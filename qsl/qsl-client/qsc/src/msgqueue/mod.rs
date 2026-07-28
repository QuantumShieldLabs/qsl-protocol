//! NA-0682 (D617, MESSAGING EPIC SLICE 3): the durable per-contact message queue.
//!
//! ⚠ NAMING, stated once so it is not rediscovered (D617 census C4, operator-confirmed):
//! this module is `msgqueue`, NOT `outbox`. The name `outbox` is ALREADY TAKEN in this
//! crate by `OUTBOX_FILE_NAME = "outbox.json"` / `OutboxRecord` -- the SINGLE-SLOT in-flight
//! RATCHET JOURNAL, which holds one recipient and one ciphertext and is burned by
//! `send abort`. That is not a queue and never was. `DESIGN_outbox_delivery_v1.md` says
//! "outbox" throughout and means THIS module. Two different things called "outbox" in one
//! crate is the overloaded-name trap this epic has already paid for three times.
//!
//! ## Why a separate store rather than more vault secrets (D617 F4 / census C9)
//!
//! `vault::secret_set` decrypts the ENTIRE vault payload, inserts one key, re-serialises,
//! re-encrypts and rewrites the whole file, measured by ENG-0053 at ~18 ms release with
//! ~95-97% of it Argon2id. Slice 3 makes per-message state transitions the common case, so
//! putting records in vault secrets would pay a whole-vault re-encrypt per transition.
//! DESIGN §1's own wording is the cheaper architecture and is what is built here: "the
//! store key lives in the vault" -- the KEY is a vault secret, the RECORDS are AEAD files
//! keyed by it.
//!
//! ## Durability shape (O1)
//!
//! ONE FILE PER MESSAGE. An enqueue is a single `write_atomic` (temp -> fsync -> rename ->
//! best-effort dir fsync), so a record is either fully committed or absent -- never
//! partial, and with no read-modify-write there is no lost update when two paths touch the
//! queue. A crash at any point after `enqueue` returns leaves a QUEUED row on disk.

use super::*;
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Layout and constants
// ---------------------------------------------------------------------------

/// Root of the message store, under the config dir.
pub(crate) const MSGQUEUE_DIR: &str = "msgqueue_v1";

/// The vault secret holding the 32-byte store key (hex).
const STORE_KEY_SECRET: &str = "msgqueue_store_key_v1";

const STORE_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const MSG_ID_LEN: usize = 16;
const RECORD_VERSION: u8 = 1;

/// DESIGN §9 Q4: warn at 50 queued per contact. A SOFT warning, not a cap -- enqueue never
/// refuses, because refusing would itself be a silent loss of the user's message, which is
/// the invariant (O1) this whole module exists to protect.
pub(crate) const QUEUE_WARN_THRESHOLD: usize = 50;

/// Retry schedule (DESIGN §2, knob ruled at §9 Q1): 5s -> 15s -> 45s -> 2m -> 5m cap.
const BACKOFF_LADDER_SECS: [u64; 5] = [5, 15, 45, 120, 300];
/// Jitter is ADDED, never subtracted, so a delay can never fall below its rung.
const BACKOFF_JITTER_SECS: u64 = 3;

// Failure causes are `&'static str` constants, NOT new `ErrorCode` variants -- the D599
// pattern that `invite/mod.rs` already follows for new failure causes.
pub const MSGQUEUE_STORE_UNAVAILABLE: &str = "msgqueue_store_unavailable";
pub const MSGQUEUE_RECORD_TAMPERED: &str = "msgqueue_record_tampered";
pub const MSGQUEUE_WRITE_FAILED: &str = "msgqueue_write_failed";
pub const MSGQUEUE_NOT_FOUND: &str = "msgqueue_not_found";
pub const MSGQUEUE_VAULT_LOCKED: &str = "vault_locked";

// ---------------------------------------------------------------------------
// States (DESIGN §1 / D617 §2d)
// ---------------------------------------------------------------------------

/// ⚠ PAUSED is deliberately NOT a variant here. DESIGN §1 makes paused a SUB-STATE of
/// QUEUED that names why retries are not running; modelling it as its own state would let a
/// paused row read as terminal, which is exactly the "visibly moving or visibly stuck"
/// distinction O5 turns on.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsgState {
    Queued,
    Sent,
    Delivered,
    /// Terminal for THIS MESSAGE, but not a permanent CAUSE.
    ///
    /// ⚠ This state resolves an inconsistency between two parts of the design, and the
    /// resolution is deliberate. DESIGN §1 lists the states as "QUEUED -> SENT ->
    /// DELIVERED, plus FAILED-PERMANENT and PAUSED-cause" -- no plain FAILED. But DESIGN §2
    /// and D617 A9 both require 413/too-large to be "FAILED for that message only", while
    /// O4 and A10 reserve FAILED-PERMANENT for session-revoked and nothing else.
    ///
    /// Collapsing 413 into `FailedPermanent` would break O4 ("no retryable failure is ever
    /// surfaced as permanent", and a 413 heals against a relay with a larger limit).
    /// Retrying it forever would be dishonest -- it cannot succeed against THIS relay.
    /// So: terminal for the message, distinct from permanent, and the queue keeps draining
    /// everything behind it.
    Failed,
    /// The ONLY permanent cause in v1 (O4): the session was revoked.
    FailedPermanent,
}

impl MsgState {
    pub fn as_str(self) -> &'static str {
        match self {
            MsgState::Queued => "QUEUED",
            MsgState::Sent => "SENT",
            MsgState::Delivered => "DELIVERED",
            MsgState::Failed => "FAILED",
            MsgState::FailedPermanent => "FAILED_PERMANENT",
        }
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            MsgState::Delivered | MsgState::Failed | MsgState::FailedPermanent
        )
    }
}

/// Why retries are not running for a QUEUED row.
///
/// ⚠ None of these is permanent. O4 allows exactly ONE permanent cause (session-revoked),
/// and that is a STATE (`FailedPermanent`), not a pause. A pause always has a way out.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PausedCause {
    VaultLocked,
    TokenRejected,
    Cert,
}

impl PausedCause {
    /// Appendix-F taxonomy key. Distinct causes get distinct words (LANE_INTENT §1);
    /// nothing collapses into a generic "couldn't send".
    pub fn as_str(self) -> &'static str {
        match self {
            PausedCause::VaultLocked => "paused-vault-locked",
            PausedCause::TokenRejected => "paused-token-rejected",
            PausedCause::Cert => "paused-cert",
        }
    }

    /// The user-facing line.
    ///
    /// ⚠ §2h is CLAIMS-HONESTY, not UX: a paused queue must never read as a sending one.
    /// `VaultLocked` is structural -- the store key is in the vault -- so the wording says
    /// what to do about it rather than implying work is in progress.
    pub fn human(self) -> &'static str {
        match self {
            PausedCause::VaultLocked => "unlock to send",
            PausedCause::TokenRejected => "relay refused the access token — check Server settings",
            PausedCause::Cert => "the relay's certificate could not be trusted",
        }
    }
}

// ---------------------------------------------------------------------------
// The record (DESIGN §1)
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QueuedMessage {
    pub v: u8,
    /// 128-bit CSPRNG, lowercase hex. See `mint_msg_id` for why this does NOT reuse
    /// `invite::wire_id`.
    pub msg_id: String,
    pub peer: String,
    /// Per-session send counter. NOT the global `send.state` `send_seq`, which is
    /// per-install and so cannot express per-contact FIFO.
    pub seq: u64,
    pub state: MsgState,
    /// Sub-state of QUEUED. Always `None` unless `state == Queued`.
    pub paused_cause: Option<PausedCause>,
    /// The plaintext body, encrypted at rest under the store key.
    pub body: Vec<u8>,
    /// Per-device delivery (DESIGN F1). SINGLE entry in v1; the map SHAPE is what lets
    /// Tier-1.5 multi-device ride later without a schema change.
    pub ack_map: BTreeMap<String, u64>,
    /// DESIGN F4 disappearing-messages hook. DORMANT in v1: nothing reads it.
    pub expires_at: Option<u64>,
    pub enqueued_at: u64,
    pub attempts: u32,
    pub next_attempt_at: u64,
    pub last_error: Option<String>,

    // -----------------------------------------------------------------------
    // In-flight ratchet state (D617 §2c, operator-ruled Option 1, STOP 008)
    //
    // ⚠ THIS IS CRYPTO-STATE SAFETY, NOT QUEUE BOOKKEEPING. `qsp_pack` ADVANCES THE
    // RATCHET. If a push fails, the retry must replay the SAME packed bytes -- re-packing
    // would burn a second message key and desync the session. The shipped global
    // `outbox.json` slot exists for exactly that reason; it is ratchet-safety machinery
    // wearing a queue's name.
    //
    // Holding these PER MESSAGE is what makes contacts independent (§2c: "a stuck message
    // blocks its own contact only"). With one global slot, a stuck message for Alice
    // head-of-line blocks Bob, because the replay branch fires on any send regardless of
    // which contact is sending.
    //
    // ⚠ THE INVARIANT, and the reason these three fields move together: once `ciphertext`
    // is `Some`, the record is PACKED and every later attempt REPLAYS IT VERBATIM. Nothing
    // may re-pack a record that already has ciphertext. `next_state` and `channel` are the
    // ratchet state and route that pack produced, committed in the SAME atomic write, so a
    // crash can never leave ciphertext without the state that must follow it.
    //
    // The duplication with `outbox.json` (still used by the attachment/file-transfer paths)
    // is ACCEPTED, NAMED, and FILED for a convergence lane -- deliberately not converged
    // here.
    /// The packed envelope. `Some` => already packed; REPLAY, never re-pack.
    #[serde(default)]
    pub ciphertext: Option<Vec<u8>>,
    /// The ratchet state to commit once the relay accepts `ciphertext`.
    #[serde(default)]
    pub next_state: Option<Vec<u8>>,
    /// The routing channel `ciphertext` was packed for.
    #[serde(default)]
    pub channel: Option<String>,
}

impl QueuedMessage {
    /// Eligible to attempt right now: queued, not paused, and off backoff.
    pub fn is_sendable_at(&self, now: u64) -> bool {
        self.state == MsgState::Queued && self.paused_cause.is_none() && self.next_attempt_at <= now
    }

    /// DESIGN §6 / knob Q2: stuck = QUEUED (or paused) longer than 60 seconds.
    pub fn is_stuck_at(&self, now: u64, threshold_secs: u64) -> bool {
        self.state == MsgState::Queued && now.saturating_sub(self.enqueued_at) >= threshold_secs
    }

    /// Already packed: the next attempt MUST replay `ciphertext` verbatim.
    ///
    /// ⚠ The single most important predicate in this module. Every send path must consult
    /// it BEFORE packing; re-packing a record that already has ciphertext burns a second
    /// message key and desyncs the session.
    pub fn is_packed(&self) -> bool {
        self.ciphertext.is_some()
    }

    /// Record the result of a pack, so a retry replays rather than re-packs.
    ///
    /// All three move together and are committed in ONE atomic write, so a crash can never
    /// leave ciphertext without the ratchet state that must follow it.
    pub fn mark_packed(&mut self, ciphertext: Vec<u8>, next_state: Vec<u8>, channel: String) {
        self.ciphertext = Some(ciphertext);
        self.next_state = Some(next_state);
        self.channel = Some(channel);
    }

    /// Drop the in-flight bytes.
    ///
    /// ⚠ NEVER CALL THIS ON A PACKED RECORD WITHOUT FIRST COMMITTING `next_state`.
    /// `qsp_pack` already advanced the ratchet in the snapshot held here; discarding that
    /// snapshot leaves the SESSION at the old position, so the next pack reuses the same
    /// message key. If the abandoned ciphertext ever reached the relay -- which is exactly
    /// the case where the push was sent and the response was lost -- two ciphertexts then
    /// exist under one key. That is the nonce reuse `abort_burns_state_and_prevents_nonce_
    /// reuse_on_next_send` (NA-0155) exists to forbid, and it is why the shipped `send
    /// abort` "burns" forward rather than simply deleting.
    ///
    /// Use `retire_packed` for any terminal transition; this is the raw primitive.
    pub fn clear_inflight(&mut self) {
        self.ciphertext = None;
        self.next_state = None;
        self.channel = None;
    }
}

// ---------------------------------------------------------------------------
// msg_id — 128-bit CSPRNG (D617 F1)
// ---------------------------------------------------------------------------

/// Mint a message id: 16 CSPRNG bytes rendered as 32 lowercase hex characters.
///
/// ⚠ THIS DELIBERATELY DOES NOT CALL `invite::wire_id`, AND THAT IS A SECURITY PROPERTY,
/// NOT A STYLE CHOICE (D617 §4 F1, operator-confirmed 2026-07-27).
///
/// `invite::wire_id` is the ONE renderer for RELAY-VISIBLE identifiers -- the
/// `X-QSL-Route-Token` header, `invite_id`, and `cap` -- and its own doc says "there is no
/// second renderer". The inner `msg_id` is the opposite kind of value: DESIGN §4 requires it
/// to stay INDEPENDENT of the relay-visible envelope id and to NEVER appear on the wire in
/// the clear.
///
/// Reusing `wire_id` would either couple two namespaces the design requires to be
/// independent, or silently falsify that load-bearing doc comment. The few lines of separate
/// rendering here ARE the point: they keep the namespaces apart so that a future change to
/// route-token rendering cannot silently reach `msg_id` -- which is precisely the
/// correlation surface DESIGN §4 forbids.
///
/// The DISCIPLINE is shared with `wire_id` (CSPRNG bytes, lowercase hex, fixed width); only
/// the function is separate.
pub(crate) fn mint_msg_id() -> String {
    let mut b = [0u8; MSG_ID_LEN];
    OsRng.fill_bytes(&mut b);
    hex_encode(&b)
}

// ---------------------------------------------------------------------------
// Store key: held in the vault, cached per process (D617 F4)
// ---------------------------------------------------------------------------

static STORE_KEY_CACHE: OnceLock<Mutex<Option<[u8; STORE_KEY_LEN]>>> = OnceLock::new();

fn store_key_slot() -> &'static Mutex<Option<[u8; STORE_KEY_LEN]>> {
    STORE_KEY_CACHE.get_or_init(|| Mutex::new(None))
}

/// Fetch (or create) the store key.
///
/// Cached for the life of the process, mirroring `vault`'s process-passphrase slot. This is
/// the whole point of F4: ONE `secret_get` per process instead of one per state transition,
/// because every `secret_get`/`secret_set` decrypts the entire vault payload (ENG-0053).
///
/// ⚠ A LOCKED VAULT IS NOT AN ERROR HERE, IT IS A PAUSE. The caller maps `vault_locked` to
/// `PausedCause::VaultLocked` so the queue reports "unlock to send" rather than failing --
/// DESIGN §3's structural limitation, stated honestly rather than surfaced as breakage.
fn store_key() -> Result<[u8; STORE_KEY_LEN], &'static str> {
    let slot = store_key_slot();
    let mut guard = slot.lock().map_err(|_| MSGQUEUE_STORE_UNAVAILABLE)?;
    if let Some(k) = *guard {
        return Ok(k);
    }
    let existing = vault::secret_get(STORE_KEY_SECRET).map_err(|e| match e {
        "vault_missing" | "vault_locked" => MSGQUEUE_VAULT_LOCKED,
        _ => MSGQUEUE_STORE_UNAVAILABLE,
    })?;
    let key = match existing {
        Some(hex) if !hex.trim().is_empty() => {
            let bytes = hex_decode(hex.trim()).map_err(|_| MSGQUEUE_STORE_UNAVAILABLE)?;
            if bytes.len() != STORE_KEY_LEN {
                return Err(MSGQUEUE_STORE_UNAVAILABLE);
            }
            let mut k = [0u8; STORE_KEY_LEN];
            k.copy_from_slice(&bytes);
            k
        }
        _ => {
            let mut k = [0u8; STORE_KEY_LEN];
            OsRng.fill_bytes(&mut k);
            vault::secret_set(STORE_KEY_SECRET, &hex_encode(&k)).map_err(|e| match e {
                "vault_missing" | "vault_locked" => MSGQUEUE_VAULT_LOCKED,
                _ => MSGQUEUE_STORE_UNAVAILABLE,
            })?;
            k
        }
    };
    *guard = Some(key);
    Ok(key)
}

// ---------------------------------------------------------------------------
// Paths
// ---------------------------------------------------------------------------

/// A raw contact label must NEVER appear in a filename -- the same rule
/// `dedup::mailbox_store_key` states for route tokens. Filenames are visible metadata even
/// when the file contents are encrypted.
fn contact_key(peer: &str) -> String {
    let c = StdCrypto;
    let h = c.sha512(peer.as_bytes());
    hex_encode(&h[..8])
}

fn queue_root(cfg_dir: &Path) -> PathBuf {
    cfg_dir.join(MSGQUEUE_DIR)
}

fn contact_dir(cfg_dir: &Path, peer: &str) -> PathBuf {
    queue_root(cfg_dir).join(contact_key(peer))
}

/// Zero-padded seq first, so a plain lexicographic directory listing is already FIFO order.
fn record_name(seq: u64, msg_id: &str) -> String {
    format!("{:020}_{}.rec", seq, msg_id)
}

// ---------------------------------------------------------------------------
// Record encryption
// ---------------------------------------------------------------------------

/// AAD binds the record to its contact, id and sequence, so a record cannot be moved between
/// contacts or renumbered by anyone with filesystem access -- without this the file would
/// still decrypt cleanly under the store key in its new position.
fn record_aad(ck: &str, msg_id: &str, seq: u64) -> Vec<u8> {
    format!("qsc.msgqueue.v1|{}|{}|{}", ck, msg_id, seq).into_bytes()
}

fn encrypt_record(
    key: &[u8; STORE_KEY_LEN],
    aad: &[u8],
    rec: &QueuedMessage,
) -> Result<Vec<u8>, &'static str> {
    let plaintext = serde_json::to_vec(rec).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let ct = cipher
        .encrypt(
            Nonce::from_slice(&nonce_bytes),
            Payload {
                msg: &plaintext,
                aad,
            },
        )
        .map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    Ok(out)
}

fn decrypt_record(
    key: &[u8; STORE_KEY_LEN],
    aad: &[u8],
    bytes: &[u8],
) -> Result<QueuedMessage, &'static str> {
    if bytes.len() <= NONCE_LEN {
        return Err(MSGQUEUE_RECORD_TAMPERED);
    }
    let (nonce_bytes, ct) = bytes.split_at(NONCE_LEN);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let pt = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), Payload { msg: ct, aad })
        .map_err(|_| MSGQUEUE_RECORD_TAMPERED)?;
    serde_json::from_slice(&pt).map_err(|_| MSGQUEUE_RECORD_TAMPERED)
}

// ---------------------------------------------------------------------------
// Clock
// ---------------------------------------------------------------------------

/// The clock is a PARAMETER on every `_at` entry point (the NA-0681 §2k seam). Tests force
/// backoff and stuck-threshold behaviour by passing a value, never by sleeping.
pub(crate) fn now_unix_s() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Read
// ---------------------------------------------------------------------------

fn read_record(
    key: &[u8; STORE_KEY_LEN],
    ck: &str,
    path: &Path,
) -> Result<QueuedMessage, &'static str> {
    let bytes = fs::read(path).map_err(|_| MSGQUEUE_NOT_FOUND)?;
    // Parse seq+msg_id back out of the filename so the AAD can be rebuilt without trusting
    // the ciphertext first -- the AAD is what proves the file is in its rightful place.
    let name = path
        .file_name()
        .and_then(|v| v.to_str())
        .ok_or(MSGQUEUE_RECORD_TAMPERED)?;
    let stem = name.strip_suffix(".rec").ok_or(MSGQUEUE_RECORD_TAMPERED)?;
    let (seq_s, msg_id) = stem.split_once('_').ok_or(MSGQUEUE_RECORD_TAMPERED)?;
    let seq: u64 = seq_s.parse().map_err(|_| MSGQUEUE_RECORD_TAMPERED)?;
    let rec = decrypt_record(key, &record_aad(ck, msg_id, seq), &bytes)?;
    // Defence in depth: the AAD already binds these, so a mismatch means the record's own
    // fields disagree with its name -- refuse rather than pick a winner.
    if rec.msg_id != msg_id || rec.seq != seq {
        return Err(MSGQUEUE_RECORD_TAMPERED);
    }
    Ok(rec)
}

/// All records for one contact, in strict FIFO order (ascending seq, msg_id as tiebreak).
pub(crate) fn load_contact(cfg_dir: &Path, peer: &str) -> Result<Vec<QueuedMessage>, &'static str> {
    let key = store_key()?;
    let ck = contact_key(peer);
    let dir = contact_dir(cfg_dir, peer);
    let mut out = Vec::new();
    let entries = match fs::read_dir(&dir) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(_) => return Err(MSGQUEUE_STORE_UNAVAILABLE),
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|v| v.to_str()) == Some("rec"))
        .collect();
    paths.sort();
    for p in paths {
        out.push(read_record(&key, &ck, &p)?);
    }
    out.sort_by(|a, b| a.seq.cmp(&b.seq).then_with(|| a.msg_id.cmp(&b.msg_id)));
    Ok(out)
}

/// Every contact that has at least one record on disk.
///
/// Returned as the ON-DISK contact keys, because the raw peer label is recoverable only from
/// inside a record -- which is the point of hashing the directory name.
pub(crate) fn contact_keys(cfg_dir: &Path) -> Result<Vec<String>, &'static str> {
    let root = queue_root(cfg_dir);
    let entries = match fs::read_dir(&root) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(MSGQUEUE_STORE_UNAVAILABLE),
    };
    let mut out: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
        .collect();
    out.sort();
    Ok(out)
}

// ---------------------------------------------------------------------------
// Write
// ---------------------------------------------------------------------------

fn write_record(
    cfg_dir: &Path,
    source: ConfigSource,
    rec: &QueuedMessage,
) -> Result<(), &'static str> {
    let key = store_key()?;
    let ck = contact_key(&rec.peer);
    let dir = contact_dir(cfg_dir, &rec.peer);
    // ⚠ SECURE THE ROOT **AND** THE CONTACT DIR, in that order, every write.
    //
    // `write_atomic` does not create parents -- it only enforces permissions `if
    // dir.exists()`. Securing only the contact dir is NOT enough, and the failure is
    // beautifully misleading: the FIRST contact succeeds (its parent `msgqueue_v1` does not
    // exist yet, so the parent check is skipped and `create_dir_all` makes it with the
    // default, group-writable mode), and the SECOND contact fails, because by then the root
    // exists and is group-writable. A single-contact test passes; multi-contact FIFO fails;
    // and it reads as a queue bug when it is a directory bug.
    //
    // Both calls are idempotent, so paying them on every write costs a stat and removes a
    // whole class of first-vs-second-contact asymmetry.
    ensure_dir_secure(&queue_root(cfg_dir), source).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    ensure_dir_secure(&dir, source).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    let bytes = encrypt_record(&key, &record_aad(&ck, &rec.msg_id, rec.seq), rec)?;
    let path = dir.join(record_name(rec.seq, &rec.msg_id));
    write_atomic(&path, &bytes, source).map_err(|_| MSGQUEUE_WRITE_FAILED)
}

/// The next per-contact sequence number: one past the highest on disk.
///
/// Derived from the store rather than a counter file, so it cannot drift from reality after
/// a crash -- there is no second source of truth to reconcile.
fn next_seq(cfg_dir: &Path, peer: &str) -> Result<u64, &'static str> {
    let existing = load_contact(cfg_dir, peer)?;
    Ok(existing.iter().map(|r| r.seq).max().map_or(0, |m| m + 1))
}

/// **O1: commit a QUEUED row BEFORE any network attempt.**
///
/// Returns the committed record. When this returns `Ok`, the message is durably on disk and
/// a crash at any later point leaves it visible and re-drainable -- never lost, never
/// invisible. This is the seam Slice 2 asserted by construction and never crash-tested;
/// A1 kills the process immediately after it, inside the network call.
pub(crate) fn enqueue_at(
    cfg_dir: &Path,
    source: ConfigSource,
    peer: &str,
    body: Vec<u8>,
    now: u64,
) -> Result<QueuedMessage, &'static str> {
    let seq = next_seq(cfg_dir, peer)?;
    let rec = QueuedMessage {
        v: RECORD_VERSION,
        msg_id: mint_msg_id(),
        peer: peer.to_string(),
        seq,
        state: MsgState::Queued,
        paused_cause: None,
        body,
        ack_map: BTreeMap::new(),
        expires_at: None,
        enqueued_at: now,
        attempts: 0,
        next_attempt_at: now,
        last_error: None,
        // Not yet packed. O1 commits the ROW before anything touches the ratchet, so a
        // crash here leaves a QUEUED row with no ciphertext -- which is exactly right:
        // nothing was packed, so no message key was consumed and the retry packs fresh.
        ciphertext: None,
        next_state: None,
        channel: None,
    };
    write_record(cfg_dir, source, &rec)?;
    Ok(rec)
}

/// Persist a mutated record over its existing file.
///
/// Same path, same AAD, so a transition is one small atomic write -- not a whole-store
/// rewrite and not a whole-vault re-encrypt (F4).
pub(crate) fn save(
    cfg_dir: &Path,
    source: ConfigSource,
    rec: &QueuedMessage,
) -> Result<(), &'static str> {
    write_record(cfg_dir, source, rec)
}

/// Remove a record. Used only for an explicit, named discard of a specifically-identified
/// message (F2) -- never as a generic recovery path, because "recover" must mean drain or
/// fail visibly, never destroy.
pub(crate) fn remove(
    cfg_dir: &Path,
    peer: &str,
    seq: u64,
    msg_id: &str,
) -> Result<(), &'static str> {
    let path = contact_dir(cfg_dir, peer).join(record_name(seq, msg_id));
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Err(MSGQUEUE_NOT_FOUND),
        Err(_) => Err(MSGQUEUE_WRITE_FAILED),
    }
}

// ---------------------------------------------------------------------------
// Backoff
// ---------------------------------------------------------------------------

/// Schedule the next attempt after a retryable failure.
///
/// Jitter is derived from the msg_id rather than a fresh RNG draw so the delay is stable
/// across a reload of the same record -- a record that is re-read must not keep moving its
/// own deadline.
pub(crate) fn schedule_retry_at(rec: &mut QueuedMessage, now: u64, err: &str) {
    rec.attempts = rec.attempts.saturating_add(1);
    let rung = BACKOFF_LADDER_SECS[(rec.attempts as usize)
        .saturating_sub(1)
        .min(BACKOFF_LADDER_SECS.len() - 1)];
    let jitter = rec
        .msg_id
        .as_bytes()
        .iter()
        .fold(0u64, |a, b| a.wrapping_add(*b as u64))
        % (BACKOFF_JITTER_SECS + 1);
    rec.next_attempt_at = now.saturating_add(rung).saturating_add(jitter);
    rec.last_error = Some(err.to_string());
}

/// An immediate-retry trigger (DESIGN §2): unlock, relay settings saved, manual retry, or
/// any successful send. Clears the backoff deadline and any pause, so the next drain tries.
pub(crate) fn arm_immediate(rec: &mut QueuedMessage, now: u64) {
    if rec.state == MsgState::Queued {
        rec.paused_cause = None;
        rec.next_attempt_at = now;
    }
}

// ---------------------------------------------------------------------------
// Inbound dedup by (session, msg_id) — D617 §2f / F5
// ---------------------------------------------------------------------------

/// Per-contact record of inbound `msg_id`s already stored.
///
/// ⚠ WHY THIS EXISTS RATHER THAN REUSING `src/dedup/mod.rs` (census C8): that module keys
/// the **relay envelope id**, per mailbox, and DESIGN §4 requires the relay-visible id to be
/// INDEPENDENT of the inner `msg_id` -- so it dedups the other identifier by construction.
/// It is also built **only in lease mode**, and lease is not the default (ENG-0043), so at
/// default settings it does not run at all. `(session, msg_id)` needs its own home.
///
/// ⚠ The existing module is deliberately UNTOUCHED (F5), and the consequence is accepted
/// knowingly: relay-level at-least-once protection still does not run at default settings.
/// That gap belongs in the testplan's "what this plan cannot see", not in a silent fix here.
/// ⚠ NOT `.rec`. Message records are `<seq>_<msg_id>.rec` and the loader globs `*.rec` in
/// this same directory, so giving the dedup file that extension made the loader try to
/// decrypt it as a message and fail the AAD check -- surfacing as
/// `msgqueue_record_tampered` on the NEXT send to that contact, i.e. as corruption rather
/// than as the namespace collision it actually was. Different things, different extensions.
const SEEN_INBOUND_FILE: &str = "seen_inbound.dedup";

#[derive(Serialize, Deserialize, Default)]
struct SeenInbound {
    v: u8,
    /// `msg_id` -> first-seen unix seconds.
    ids: BTreeMap<String, u64>,
}

fn seen_inbound_path(cfg_dir: &Path, peer: &str) -> PathBuf {
    contact_dir(cfg_dir, peer).join(SEEN_INBOUND_FILE)
}

fn seen_inbound_aad(ck: &str) -> Vec<u8> {
    format!("qsc.msgqueue.seen.v1|{}", ck).into_bytes()
}

fn load_seen_inbound(cfg_dir: &Path, peer: &str) -> Result<SeenInbound, &'static str> {
    let key = store_key()?;
    let path = seen_inbound_path(cfg_dir, peer);
    let bytes = match fs::read(&path) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Ok(SeenInbound {
                v: RECORD_VERSION,
                ids: BTreeMap::new(),
            })
        }
        Err(_) => return Err(MSGQUEUE_STORE_UNAVAILABLE),
    };
    if bytes.len() <= NONCE_LEN {
        return Err(MSGQUEUE_RECORD_TAMPERED);
    }
    let (nonce, ct) = bytes.split_at(NONCE_LEN);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let pt = cipher
        .decrypt(
            Nonce::from_slice(nonce),
            Payload {
                msg: ct,
                aad: &seen_inbound_aad(&contact_key(peer)),
            },
        )
        .map_err(|_| MSGQUEUE_RECORD_TAMPERED)?;
    serde_json::from_slice(&pt).map_err(|_| MSGQUEUE_RECORD_TAMPERED)
}

/// Has this `(peer, msg_id)` already been stored?
pub(crate) fn inbound_already_seen(
    cfg_dir: &Path,
    peer: &str,
    msg_id: &str,
) -> Result<bool, &'static str> {
    Ok(load_seen_inbound(cfg_dir, peer)?.ids.contains_key(msg_id))
}

/// Durably record an inbound `msg_id` as stored.
///
/// ⚠ ORDER MATTERS, and it is the same fail-closed rule as C16: the caller records the id
/// only AFTER the message itself is durably stored. Recording first would let a crash in
/// between turn a real message into a permanent duplicate-suppression -- a silent loss
/// dressed as dedup.
pub(crate) fn record_inbound_seen(
    cfg_dir: &Path,
    source: ConfigSource,
    peer: &str,
    msg_id: &str,
    now: u64,
) -> Result<(), &'static str> {
    let key = store_key()?;
    let mut seen = load_seen_inbound(cfg_dir, peer)?;
    if seen.ids.contains_key(msg_id) {
        return Ok(());
    }
    seen.v = RECORD_VERSION;
    seen.ids.insert(msg_id.to_string(), now);
    let plaintext = serde_json::to_vec(&seen).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&key));
    let mut nonce_bytes = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce_bytes);
    let ct = cipher
        .encrypt(
            Nonce::from_slice(&nonce_bytes),
            Payload {
                msg: &plaintext,
                aad: &seen_inbound_aad(&contact_key(peer)),
            },
        )
        .map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    let dir = contact_dir(cfg_dir, peer);
    ensure_dir_secure(&queue_root(cfg_dir), source).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    ensure_dir_secure(&dir, source).map_err(|_| MSGQUEUE_WRITE_FAILED)?;
    write_atomic(&seen_inbound_path(cfg_dir, peer), &out, source).map_err(|_| MSGQUEUE_WRITE_FAILED)
}

// ---------------------------------------------------------------------------
// The drain (D617 §2e, F3)
// ---------------------------------------------------------------------------

/// What made a drain run. DESIGN §2's immediate-retry triggers.
///
/// ⚠ F3: Slice 3 ships this ENTRY POINT and the trigger vocabulary; it does NOT ship a
/// thread or a timer. Slice 4 owns the loop that calls `drain` on unlock, settings-save,
/// manual retry, any successful send, and on the backoff schedule. That boundary is what
/// keeps Slice 3 = logic and Slice 4 = app runtime.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DrainTrigger {
    Unlock,
    RelaySettingsSaved,
    ManualRetry,
    SendSucceeded,
    Scheduled,
}

impl DrainTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            DrainTrigger::Unlock => "unlock",
            DrainTrigger::RelaySettingsSaved => "relay_settings_saved",
            DrainTrigger::ManualRetry => "manual_retry",
            DrainTrigger::SendSucceeded => "send_succeeded",
            DrainTrigger::Scheduled => "scheduled",
        }
    }

    /// Every trigger except the scheduled tick is a USER-VISIBLE event that should clear
    /// backoff immediately -- DESIGN §2 names them exactly.
    pub fn is_immediate(self) -> bool {
        !matches!(self, DrainTrigger::Scheduled)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DrainOutcome {
    pub attempted: usize,
    pub sent: usize,
    pub paused: usize,
    pub failed: usize,
    pub still_queued: usize,
}

/// How a send attempt ended, in the queue's own vocabulary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttemptResult {
    Sent,
    /// Retryable: stay QUEUED, climb the backoff ladder.
    Retry,
    /// Stop trying and name why, but keep a way out (PAUSED sub-state of QUEUED).
    Pause(PausedCause),
    /// Terminal for this message only (413).
    Fail,
    /// The one permanent cause (O4): the session is gone.
    FailPermanent,
}

/// The crypto + network half of a send, injected so the FIFO/backoff POLICY in this module
/// is testable without a relay, a vault, or a ratchet.
///
/// ⚠ `pack` and `push` are separate on purpose. `pack` ADVANCES THE RATCHET, so the drain
/// calls it only when `!rec.is_packed()`, commits the result atomically, and from then on
/// calls `push` alone -- replaying identical bytes forever. Fusing them would reintroduce
/// exactly the re-pack that burns a message key.
pub trait MessageSender {
    /// Pack a not-yet-packed record: returns (ciphertext, next_ratchet_state, channel).
    fn pack(&mut self, rec: &QueuedMessage) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult>;
    /// Push already-packed bytes. MUST NOT pack.
    fn push(&mut self, rec: &QueuedMessage) -> Result<(), AttemptResult>;
    /// Commit the ratchet state after the relay durably accepted the bytes (O2).
    fn commit(&mut self, rec: &QueuedMessage) -> Result<(), &'static str>;
}

/// Drain every contact.
///
/// **Per contact: strict FIFO, and STOP at the first non-terminal failure** -- that stop is
/// what makes FIFO strict, because continuing past a stuck message would let msg2 overtake
/// msg1. **Across contacts: fully independent** -- a stuck contact is skipped, never allowed
/// to block another (§2c, the property Option 1 exists to deliver).
pub(crate) fn drain_at(
    cfg_dir: &Path,
    source: ConfigSource,
    trigger: DrainTrigger,
    now: u64,
    sender: &mut dyn MessageSender,
) -> Result<DrainOutcome, &'static str> {
    let mut out = DrainOutcome::default();
    for ck in contact_keys(cfg_dir)? {
        let dir = queue_root(cfg_dir).join(&ck);
        let mut recs = load_dir(cfg_dir, &ck, &dir)?;
        recs.sort_by(|a, b| a.seq.cmp(&b.seq).then_with(|| a.msg_id.cmp(&b.msg_id)));
        for rec in recs.iter_mut() {
            if rec.state.is_terminal() || rec.state == MsgState::Sent {
                continue;
            }
            if trigger.is_immediate() {
                arm_immediate(rec, now);
                save(cfg_dir, source, rec)?;
            }
            if !rec.is_sendable_at(now) {
                if rec.state == MsgState::Queued {
                    out.still_queued += 1;
                    if rec.paused_cause.is_some() {
                        out.paused += 1;
                    }
                }
                // ⚠ FIFO: a not-yet-due or paused message BLOCKS ITS OWN CONTACT, so msg2
                // can never overtake msg1. The outer loop still moves on to other contacts.
                break;
            }
            out.attempted += 1;
            match attempt_one(cfg_dir, source, rec, now, sender) {
                Ok(AttemptResult::Sent) => out.sent += 1,
                Ok(AttemptResult::Retry) => {
                    out.still_queued += 1;
                    break;
                }
                Ok(AttemptResult::Pause(_)) => {
                    out.paused += 1;
                    out.still_queued += 1;
                    break;
                }
                Ok(AttemptResult::Fail) | Ok(AttemptResult::FailPermanent) => {
                    // Terminal for THIS message: the queue keeps draining behind it, which
                    // is what "failed for that message only" means.
                    out.failed += 1;
                }
                Err(e) => return Err(e),
            }
        }
    }
    Ok(out)
}

/// One attempt at one record. Pack if needed (once), push, then commit.
fn attempt_one(
    cfg_dir: &Path,
    source: ConfigSource,
    rec: &mut QueuedMessage,
    now: u64,
    sender: &mut dyn MessageSender,
) -> Result<AttemptResult, &'static str> {
    // --- pack, at most once in this record's life -------------------------------
    if !rec.is_packed() {
        match sender.pack(rec) {
            Ok((ct, next_state, channel)) => {
                rec.mark_packed(ct, next_state, channel);
                // ⚠ COMMIT THE PACK BEFORE THE NETWORK. The ratchet has advanced in memory;
                // if the process dies after the push but before this write, the retry would
                // re-pack and burn a second key. Persisting first makes the crash safe.
                save(cfg_dir, source, rec)?;
            }
            Err(res) => return apply_result(cfg_dir, source, rec, now, res, "pack_failed", sender),
        }
    }

    // --- push: always the SAME bytes ------------------------------------------
    match sender.push(rec) {
        Ok(()) => {}
        Err(res) => return apply_result(cfg_dir, source, rec, now, res, "push_failed", sender),
    }

    // --- accepted (O2: the relay durably took it) -----------------------------
    sender.commit(rec)?;
    rec.state = MsgState::Sent;
    rec.paused_cause = None;
    rec.last_error = None;
    // Safe here, and ONLY here, because `sender.commit` above already advanced the session
    // past this message. The bytes are with the relay and the key is spent.
    rec.clear_inflight();
    save(cfg_dir, source, rec)?;
    Ok(AttemptResult::Sent)
}

fn apply_result(
    cfg_dir: &Path,
    source: ConfigSource,
    rec: &mut QueuedMessage,
    now: u64,
    res: AttemptResult,
    stage: &'static str,
    sender: &mut dyn MessageSender,
) -> Result<AttemptResult, &'static str> {
    match res {
        AttemptResult::Retry => schedule_retry_at(rec, now, stage),
        AttemptResult::Pause(cause) => {
            // ⚠ PAUSED is a SUB-STATE of QUEUED: the state does not move, so the row can
            // never read as terminal, and any immediate trigger clears it.
            rec.paused_cause = Some(cause);
            rec.last_error = Some(cause.as_str().to_string());
        }
        // ⚠ NONCE REUSE. A packed record that is abandoned MUST advance the session first.
        // See `clear_inflight`. `retire_packed` is fail-closed: if the ratchet cannot be
        // committed, the record stays QUEUED and retries rather than silently dropping a
        // key's worth of state.
        AttemptResult::Fail => {
            retire_packed(rec, sender)?;
            rec.state = MsgState::Failed;
        }
        AttemptResult::FailPermanent => {
            retire_packed(rec, sender)?;
            rec.state = MsgState::FailedPermanent;
        }
        AttemptResult::Sent => {}
    }
    save(cfg_dir, source, rec)?;
    Ok(res)
}

/// Retire a record's in-flight state on the way to a TERMINAL state.
///
/// ⚠ THE NONCE-REUSE GUARD. If the record was packed, `qsp_pack` already advanced the
/// ratchet and that advance lives only in `next_state`. Abandoning the message without
/// committing it leaves the session at the old position, so the next pack reuses the same
/// message key -- and if the abandoned ciphertext reached the relay (push sent, response
/// lost), two ciphertexts exist under one key.
///
/// This is the same forward-burn the shipped `send abort` performs, and NA-0155's
/// `abort_burns_state_and_prevents_nonce_reuse_on_next_send` is its guard.
///
/// FAIL-CLOSED: if the commit fails, the caller keeps the record QUEUED and retries. Never
/// drop the bytes on a failed commit -- that is the bug this function exists to prevent.
fn retire_packed(
    rec: &mut QueuedMessage,
    sender: &mut dyn MessageSender,
) -> Result<(), &'static str> {
    if rec.is_packed() {
        sender.commit(rec)?;
    }
    rec.clear_inflight();
    Ok(())
}

/// The named discard (D617 §4 F2): drop ONE specifically-identified stuck message.
///
/// ⚠ THIS IS NOT `remove()`. A packed record carries a ratchet advance that lives only in
/// `next_state`; deleting it without committing that advance is NONCE REUSE -- the next
/// pack reuses the same message key, and if the abandoned ciphertext reached the relay
/// (push sent, response lost) two ciphertexts exist under one key. `retire_packed` is the
/// barrier, and it is fail-closed: a failed commit aborts the discard and leaves the
/// message QUEUED rather than dropping the state.
///
/// This is the same forward-burn the shipped `send abort` performs, which is why NA-0155's
/// `abort_burns_state_and_prevents_nonce_reuse_on_next_send` is its guard.
///
/// ⚠ It is deliberately NOT reachable from a generic recovery path (F2: "recover means
/// drain or fail visibly, never destroy"). The caller must name a specific `msg_id`, so
/// destroying a user's message is always an explicit act on an identified message.
pub(crate) fn discard_at(
    cfg_dir: &Path,
    peer: &str,
    msg_id: &str,
    sender: &mut dyn MessageSender,
) -> Result<(), &'static str> {
    let mut rec = load_contact(cfg_dir, peer)?
        .into_iter()
        .find(|r| r.msg_id == msg_id)
        .ok_or(MSGQUEUE_NOT_FOUND)?;
    // Commit the advance BEFORE the record goes away. Fail-closed: on error the record is
    // untouched on disk and still drainable.
    retire_packed(&mut rec, sender)?;
    remove(cfg_dir, peer, rec.seq, &rec.msg_id)
}

/// A contact's queue, summarised as DATA (D617 §2i).
///
/// ⚠ Structured, not marker strings. Slice 4 must not have to parse stdout -- today's
/// `timeline_list` prints markers and `TimelineEntry`'s fields are private, so a GUI can
/// call it but cannot read it. This is the shape that does not repeat that.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContactQueueSummary {
    pub peer_key: String,
    pub queued: usize,
    pub sent: usize,
    pub delivered: usize,
    pub failed: usize,
    pub paused: Option<PausedCause>,
    /// Oldest QUEUED row's age in seconds, for the stuck threshold (DESIGN §9 Q2 = 60s).
    pub oldest_queued_age_s: Option<u64>,
}

/// Summarise every contact's queue.
pub fn summarize_at(cfg_dir: &Path, now: u64) -> Result<Vec<ContactQueueSummary>, &'static str> {
    let mut out = Vec::new();
    for ck in contact_keys(cfg_dir)? {
        let dir = queue_root(cfg_dir).join(&ck);
        let recs = load_dir(cfg_dir, &ck, &dir)?;
        if recs.is_empty() {
            continue;
        }
        let mut s = ContactQueueSummary {
            peer_key: ck,
            queued: 0,
            sent: 0,
            delivered: 0,
            failed: 0,
            paused: None,
            oldest_queued_age_s: None,
        };
        for r in recs.iter() {
            match r.state {
                MsgState::Queued => {
                    s.queued += 1;
                    if s.paused.is_none() {
                        s.paused = r.paused_cause;
                    }
                    let age = now.saturating_sub(r.enqueued_at);
                    s.oldest_queued_age_s = Some(s.oldest_queued_age_s.map_or(age, |o| o.max(age)));
                }
                MsgState::Sent => s.sent += 1,
                MsgState::Delivered => s.delivered += 1,
                MsgState::Failed | MsgState::FailedPermanent => s.failed += 1,
            }
        }
        out.push(s);
    }
    Ok(out)
}

impl ContactQueueSummary {
    /// The honest one-line status (DESIGN §3 / D617 §2h).
    ///
    /// ⚠ CLAIMS-HONESTY, NOT UX. v1 has no background daemon: messages move only while the
    /// app is open and the vault unlocked, and a locked vault means the outbox is PAUSED
    /// because the store key is in the vault. **A paused queue must never read as a sending
    /// one**, so a pause names its cause and what to do about it, and an unreachable relay
    /// says it will send later rather than implying it is trying right now.
    pub fn honest_line(&self) -> Option<String> {
        if self.queued == 0 {
            return None;
        }
        Some(match self.paused {
            Some(c) => format!("{} queued — {}", self.queued, c.human()),
            None => format!(
                "{} queued — will send when the relay is reachable",
                self.queued
            ),
        })
    }
}

/// Load one contact's records from an already-known directory (the drain has the hashed
/// key, not the peer label -- that is the point of hashing the directory name).
fn load_dir(_cfg_dir: &Path, ck: &str, dir: &Path) -> Result<Vec<QueuedMessage>, &'static str> {
    let key = store_key()?;
    let mut out = Vec::new();
    let entries = match fs::read_dir(dir) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(_) => return Err(MSGQUEUE_STORE_UNAVAILABLE),
    };
    let mut paths: Vec<PathBuf> = entries
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        // Belt and braces after the `.rec` collision: only message records, never sidecars.
        .filter(|p| p.extension().and_then(|v| v.to_str()) == Some("rec"))
        .collect();
    paths.sort();
    for p in paths {
        out.push(read_record(&key, ck, &p)?);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Unit tests — the pure parts, testable without a vault.
//
// §3b discipline: the HAPPY PATH is written and seen green FIRST; only then the
// negatives, each of which is a property that must FAIL on wrong code. Every negative
// below sits beside a positive exercising the same function, so "it rejected everything"
// cannot masquerade as "it rejected the right thing".
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn test_key() -> [u8; STORE_KEY_LEN] {
        [7u8; STORE_KEY_LEN]
    }

    fn sample(peer: &str, seq: u64, msg_id: &str) -> QueuedMessage {
        QueuedMessage {
            v: RECORD_VERSION,
            msg_id: msg_id.to_string(),
            peer: peer.to_string(),
            seq,
            state: MsgState::Queued,
            paused_cause: None,
            body: b"hello".to_vec(),
            ack_map: BTreeMap::new(),
            expires_at: None,
            enqueued_at: 100,
            attempts: 0,
            next_attempt_at: 100,
            last_error: None,
            ciphertext: None,
            next_state: None,
            channel: None,
        }
    }

    // --- HAPPY PATH FIRST ---------------------------------------------------

    #[test]
    fn a_record_round_trips_under_its_own_aad() {
        let k = test_key();
        let rec = sample("alice", 3, "0123456789abcdef0123456789abcdef");
        let aad = record_aad(&contact_key("alice"), &rec.msg_id, rec.seq);
        let ct = encrypt_record(&k, &aad, &rec).expect("encrypt");
        let back = decrypt_record(&k, &aad, &ct).expect("decrypt");
        assert_eq!(back.msg_id, rec.msg_id);
        assert_eq!(back.seq, rec.seq);
        assert_eq!(back.body, b"hello".to_vec());
        assert_eq!(back.state, MsgState::Queued);
    }

    #[test]
    fn record_names_sort_in_fifo_order() {
        // FIFO is carried by the FILENAME, so lexicographic order must equal seq order --
        // including across a power-of-ten boundary, which is the case naive padding breaks.
        let mut names = [
            record_name(10, "aa"),
            record_name(2, "bb"),
            record_name(1, "cc"),
            record_name(100, "dd"),
        ];
        names.sort();
        let seqs: Vec<&str> = names.iter().map(|n| n.split('_').next().unwrap()).collect();
        assert_eq!(
            seqs,
            vec![
                "00000000000000000001",
                "00000000000000000002",
                "00000000000000000010",
                "00000000000000000100"
            ]
        );
    }

    #[test]
    fn a_minted_msg_id_is_128_bits_of_lowercase_hex() {
        let id = mint_msg_id();
        assert_eq!(id.len(), MSG_ID_LEN * 2, "128 bits rendered as hex");
        assert!(
            id.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "lowercase hex only: {id}"
        );
        assert_ne!(mint_msg_id(), mint_msg_id(), "CSPRNG, not a counter");
    }

    #[test]
    fn a_contact_label_never_appears_in_its_directory_name() {
        let peer = "alice-very-distinctive-label";
        let ck = contact_key(peer);
        assert!(
            !ck.contains("alice"),
            "raw label leaked into a filename: {ck}"
        );
        assert_eq!(ck.len(), 16, "8 bytes of hash, hex-rendered");
        assert_eq!(ck, contact_key(peer), "stable for the same peer");
        assert_ne!(ck, contact_key("bob"), "distinct peers, distinct dirs");
    }

    // --- NEGATIVES, each beside the positive above ---------------------------

    #[test]
    fn a_record_moved_to_another_contact_fails_to_decrypt() {
        // The AAD binds contact_key. Without it, the file would decrypt cleanly in its new
        // home -- an attacker with filesystem access could reattribute a message.
        let k = test_key();
        let rec = sample("alice", 3, "0123456789abcdef0123456789abcdef");
        let ct = encrypt_record(
            &k,
            &record_aad(&contact_key("alice"), &rec.msg_id, rec.seq),
            &rec,
        )
        .expect("encrypt");
        let wrong = record_aad(&contact_key("bob"), &rec.msg_id, rec.seq);
        assert_eq!(
            decrypt_record(&k, &wrong, &ct).unwrap_err(),
            MSGQUEUE_RECORD_TAMPERED
        );
    }

    #[test]
    fn a_renumbered_record_fails_to_decrypt() {
        // The AAD binds seq, so a record cannot be reordered in the FIFO by renaming it.
        let k = test_key();
        let rec = sample("alice", 3, "0123456789abcdef0123456789abcdef");
        let ct = encrypt_record(
            &k,
            &record_aad(&contact_key("alice"), &rec.msg_id, rec.seq),
            &rec,
        )
        .expect("encrypt");
        let wrong = record_aad(&contact_key("alice"), &rec.msg_id, 99);
        assert_eq!(
            decrypt_record(&k, &wrong, &ct).unwrap_err(),
            MSGQUEUE_RECORD_TAMPERED
        );
    }

    #[test]
    fn a_record_under_a_different_store_key_fails_to_decrypt() {
        let rec = sample("alice", 3, "0123456789abcdef0123456789abcdef");
        let aad = record_aad(&contact_key("alice"), &rec.msg_id, rec.seq);
        let ct = encrypt_record(&test_key(), &aad, &rec).expect("encrypt");
        assert_eq!(
            decrypt_record(&[9u8; STORE_KEY_LEN], &aad, &ct).unwrap_err(),
            MSGQUEUE_RECORD_TAMPERED
        );
    }

    #[test]
    fn a_truncated_record_is_refused_not_panicked() {
        let k = test_key();
        let rec = sample("alice", 3, "0123456789abcdef0123456789abcdef");
        let aad = record_aad(&contact_key("alice"), &rec.msg_id, rec.seq);
        let ct = encrypt_record(&k, &aad, &rec).expect("encrypt");
        for cut in [0usize, 1, NONCE_LEN, NONCE_LEN + 1, ct.len() - 1] {
            assert!(decrypt_record(&k, &aad, &ct[..cut]).is_err(), "cut={cut}");
        }
    }

    // --- inbound dedup by (session, msg_id) (§2f / F5 / A5) ------------------

    #[test]
    fn an_unseen_inbound_msg_id_is_not_a_duplicate_and_becomes_one_once_recorded() {
        // Happy path first: unseen -> record -> seen.
        install_test_store_key();
        let cfg = temp_cfg("dedup_happy");
        let src = ConfigSource::EnvOverride;
        assert!(!inbound_already_seen(&cfg, "alice", "aa11").expect("q1"));
        record_inbound_seen(&cfg, src, "alice", "aa11", 100).expect("rec");
        assert!(inbound_already_seen(&cfg, "alice", "aa11").expect("q2"));
        // Idempotent: recording twice is not an error and does not change the answer.
        record_inbound_seen(&cfg, src, "alice", "aa11", 200).expect("rec again");
        assert!(inbound_already_seen(&cfg, "alice", "aa11").expect("q3"));
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn dedup_is_scoped_per_contact_so_two_peers_can_use_the_same_msg_id() {
        // (session, msg_id) -- NOT msg_id alone. Two peers minting the same 128-bit id is
        // vanishingly unlikely, but scoping by contact means it could never matter, and it
        // is what DESIGN §4 actually specifies.
        install_test_store_key();
        let cfg = temp_cfg("dedup_scope");
        let src = ConfigSource::EnvOverride;
        record_inbound_seen(&cfg, src, "alice", "shared", 100).expect("rec");
        assert!(inbound_already_seen(&cfg, "alice", "shared").expect("a"));
        assert!(
            !inbound_already_seen(&cfg, "bob", "shared").expect("b"),
            "bob's queue must not inherit alice's dedup state"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn the_seen_record_survives_a_reload_and_is_encrypted_at_rest() {
        install_test_store_key();
        let cfg = temp_cfg("dedup_at_rest");
        let src = ConfigSource::EnvOverride;
        record_inbound_seen(&cfg, src, "alice", "deadbeef", 100).expect("rec");
        // Durable across a fresh read.
        assert!(inbound_already_seen(&cfg, "alice", "deadbeef").expect("q"));
        // And the id is not sitting in the clear on disk.
        let raw = fs::read(seen_inbound_path(&cfg, "alice")).expect("read raw");
        let hay = String::from_utf8_lossy(&raw);
        assert!(
            !hay.contains("deadbeef"),
            "msg_id must not be readable at rest"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_seen_record_from_another_contact_fails_to_decrypt() {
        // Same AAD discipline as the message records: the file is bound to its contact.
        install_test_store_key();
        let cfg = temp_cfg("dedup_aad");
        let src = ConfigSource::EnvOverride;
        record_inbound_seen(&cfg, src, "alice", "aa11", 100).expect("rec");
        let stolen = fs::read(seen_inbound_path(&cfg, "alice")).expect("read");
        ensure_dir_secure(&contact_dir(&cfg, "bob"), src).expect("mk bob");
        write_atomic(&seen_inbound_path(&cfg, "bob"), &stolen, src).expect("plant");
        assert_eq!(
            inbound_already_seen(&cfg, "bob", "aa11").unwrap_err(),
            MSGQUEUE_RECORD_TAMPERED,
            "a seen-record moved between contacts must not decrypt"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    // --- the drain: FIFO and contact independence (§2c / A3) -----------------

    /// Set the store key directly so the POLICY can be tested without a vault. The key is
    /// process-cached anyway, so this exercises exactly the production read path afterwards.
    fn install_test_store_key() {
        *store_key_slot().lock().unwrap() = Some(test_key());
    }

    /// ⚠ NOT `/tmp`. `ensure_dir_secure` -> `enforce_safe_parents` refuses a world-writable
    /// ancestor, and `/tmp` is 1777 -- so a store under it fails with `UnsafeParentPerms`.
    /// That is the production guard working correctly, not a test inconvenience, and it is
    /// why the integration suite roots its fixtures under the target dir too.
    fn temp_cfg(tag: &str) -> PathBuf {
        let root = std::env::var("QSC_TEST_ROOT")
            .or_else(|_| std::env::var("CARGO_TARGET_DIR"))
            .unwrap_or_else(|_| "target".to_string());
        let d = PathBuf::from(root).join("qsc-msgq-tests").join(format!(
            "{}_{}_{}",
            tag,
            std::process::id(),
            now_unix_s()
        ));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).expect("mk cfg");
        // ⚠ And then chmod 0700 EXPLICITLY. The qbuild tree is setgid group-writable
        // (`/srv/qbuild` is 2775), so a directory created under it INHERITS group-write and
        // `enforce_safe_parents` rejects it with `UnsafeParentPerms` -- before
        // `enforce_dir_perms` ever gets to correct the mode. The integration suite's
        // `create_dir_700` helper exists for exactly this; the same rule applies here.
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            for dir in [d.parent().unwrap(), d.as_path()] {
                fs::set_permissions(dir, fs::Permissions::from_mode(0o700)).expect("chmod 700");
            }
        }
        ensure_dir_secure(&d, ConfigSource::EnvOverride).expect("secure cfg");
        d
    }

    /// A sender that records what it was asked to do and fails on demand.
    struct FakeSender {
        packed: Vec<String>,
        pushed: Vec<String>,
        fail_push_for: Option<String>,
        result: AttemptResult,
    }

    impl FakeSender {
        fn new() -> Self {
            Self {
                packed: Vec::new(),
                pushed: Vec::new(),
                fail_push_for: None,
                result: AttemptResult::Retry,
            }
        }
    }

    impl MessageSender for FakeSender {
        fn pack(
            &mut self,
            rec: &QueuedMessage,
        ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
            self.packed.push(rec.msg_id.clone());
            Ok((
                format!("CT:{}", rec.msg_id).into_bytes(),
                b"NEXT".to_vec(),
                "chan".to_string(),
            ))
        }
        fn push(&mut self, rec: &QueuedMessage) -> Result<(), AttemptResult> {
            self.pushed.push(rec.msg_id.clone());
            if self.fail_push_for.as_deref() == Some(rec.peer.as_str()) {
                return Err(self.result);
            }
            Ok(())
        }
        fn commit(&mut self, _rec: &QueuedMessage) -> Result<(), &'static str> {
            Ok(())
        }
    }

    #[test]
    fn the_happy_path_drains_a_contact_in_fifo_order() {
        // §3b: the happy path runs and is seen green BEFORE any negative below it.
        install_test_store_key();
        let cfg = temp_cfg("fifo_happy");
        let src = ConfigSource::EnvOverride;
        for body in [b"one".to_vec(), b"two".to_vec(), b"three".to_vec()] {
            enqueue_at(&cfg, src, "alice", body, 100).expect("enqueue");
        }
        let mut s = FakeSender::new();
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");
        assert_eq!(out.sent, 3);
        assert_eq!(s.pushed.len(), 3);
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs.len(), 3);
        assert!(recs.iter().all(|r| r.state == MsgState::Sent));
        assert_eq!(recs[0].seq, 0, "seq ascends and drives FIFO");
        assert_eq!(recs[2].seq, 2);
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn msg2_never_overtakes_msg1_for_the_same_contact() {
        // Strict FIFO: when msg1 cannot go, NOTHING behind it goes either.
        install_test_store_key();
        let cfg = temp_cfg("fifo_strict");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"first".to_vec(), 100).expect("e1");
        enqueue_at(&cfg, src, "alice", b"second".to_vec(), 100).expect("e2");

        let mut s = FakeSender::new();
        s.fail_push_for = Some("alice".to_string());
        s.result = AttemptResult::Retry;
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");

        assert_eq!(out.sent, 0);
        assert_eq!(
            s.pushed.len(),
            1,
            "only msg1 was attempted; msg2 must not overtake"
        );
        let recs = load_contact(&cfg, "alice").expect("load");
        assert!(recs.iter().all(|r| r.state == MsgState::Queued));
        assert_eq!(recs[0].attempts, 1, "msg1 climbed the ladder");
        assert_eq!(recs[1].attempts, 0, "msg2 was never touched");
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_stuck_contact_does_not_stall_another_contact() {
        // ⚠ THE §2c PROPERTY, and the entire reason in-flight state moved into the record
        // (Option 1). With the shipped global slot, Alice being stuck head-of-line blocks
        // Bob across every conversation.
        install_test_store_key();
        let cfg = temp_cfg("independence");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"stuck".to_vec(), 100).expect("e1");
        enqueue_at(&cfg, src, "bob", b"should still go".to_vec(), 100).expect("e2");

        let mut s = FakeSender::new();
        s.fail_push_for = Some("alice".to_string());
        s.result = AttemptResult::Retry;
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");

        assert_eq!(out.sent, 1, "bob's message went while alice was stuck");
        let alice = load_contact(&cfg, "alice").expect("a");
        let bob = load_contact(&cfg, "bob").expect("b");
        assert_eq!(alice[0].state, MsgState::Queued, "alice is stuck, visibly");
        assert_eq!(bob[0].state, MsgState::Sent, "bob is unaffected");
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_packed_record_is_never_repacked_across_retries() {
        // ⚠ The ratchet invariant at drain level: pack ONCE, replay forever. A second pack
        // would burn a second message key and desync the session.
        install_test_store_key();
        let cfg = temp_cfg("repack");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"body".to_vec(), 100).expect("e1");

        let mut s = FakeSender::new();
        s.fail_push_for = Some("alice".to_string());
        s.result = AttemptResult::Retry;
        for t in [100u64, 1_000, 2_000, 3_000] {
            let _ = drain_at(&cfg, src, DrainTrigger::ManualRetry, t, &mut s).expect("drain");
        }
        assert_eq!(
            s.packed.len(),
            1,
            "packed exactly once across four attempts"
        );
        assert!(s.pushed.len() >= 4, "but pushed every time");
        let first = &s.pushed[0];
        assert!(
            s.pushed.iter().all(|m| m == first),
            "same record every retry"
        );
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(
            recs[0].ciphertext.as_deref(),
            Some(format!("CT:{}", recs[0].msg_id).as_bytes()),
            "the SAME bytes are still on disk for the next replay"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_successful_send_commits_the_ratchet_exactly_once() {
        // ⚠ MIGRATION EQUIVALENT for `send_commit::outbox_commit_advances_once`, added
        // BEFORE that guard is retired (the binding condition). The old test counted the
        // derived `send.state` counter; this counts the RATCHET COMMIT itself, which is the
        // property that counter stood for -- exactly once per accepted message, never twice
        // (double-advance) and never zero (skipped).
        install_test_store_key();
        let cfg = temp_cfg("commit_once");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"one".to_vec(), 100).expect("e1");
        enqueue_at(&cfg, src, "alice", b"two".to_vec(), 100).expect("e2");

        struct CountCommits {
            commits: usize,
            pushes: usize,
        }
        impl MessageSender for CountCommits {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                Ok((b"CT".to_vec(), b"N".to_vec(), "c".to_string()))
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                self.pushes += 1;
                Ok(())
            }
            fn commit(&mut self, _r: &QueuedMessage) -> Result<(), &'static str> {
                self.commits += 1;
                Ok(())
            }
        }
        let mut s = CountCommits {
            commits: 0,
            pushes: 0,
        };
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");
        assert_eq!(out.sent, 2);
        assert_eq!(s.pushes, 2, "one push per message");
        assert_eq!(s.commits, 2, "EXACTLY one commit per accepted message");

        // And a second drain over the same store commits NOTHING further -- already-SENT
        // records are skipped, so a re-drain cannot double-advance the ratchet.
        let out2 = drain_at(&cfg, src, DrainTrigger::Scheduled, 200, &mut s).expect("drain2");
        assert_eq!(out2.attempted, 0);
        assert_eq!(s.commits, 2, "a re-drain must not advance anything again");
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_retryable_push_failure_mutates_no_session_state() {
        // ⚠ MIGRATION EQUIVALENT for the `relay_{drop,dup,reorder}_no_mutation` trio's
        // `send_attempt ok=false` assertions, added BEFORE they are retired.
        //
        // Their property is that a transport failure leaves protocol state untouched. Here
        // that is exact and observable: on a retryable failure the ratchet MUST NOT be
        // committed, because the message was never accepted -- committing would advance the
        // session past a message the peer will never see.
        install_test_store_key();
        let cfg = temp_cfg("no_mutation");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"x".to_vec(), 100).expect("e1");

        struct DropWithCommitWatch {
            commits: usize,
        }
        impl MessageSender for DropWithCommitWatch {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                Ok((b"CT".to_vec(), b"N".to_vec(), "c".to_string()))
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                Err(AttemptResult::Retry)
            }
            fn commit(&mut self, _r: &QueuedMessage) -> Result<(), &'static str> {
                self.commits += 1;
                Ok(())
            }
        }
        let mut s = DropWithCommitWatch { commits: 0 };
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");
        assert_eq!(out.sent, 0);
        assert_eq!(
            s.commits, 0,
            "a dropped push must NOT advance the session -- no mutation on failure"
        );
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs[0].state, MsgState::Queued, "still queued, visibly");
        assert!(
            recs[0].is_packed(),
            "and the same bytes await the next retry"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_413_fails_only_its_own_message_and_the_queue_keeps_draining() {
        // A9 + A10 together: terminal for THIS message, and NOT permanent.
        install_test_store_key();
        let cfg = temp_cfg("too_large");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"huge".to_vec(), 100).expect("e1");
        enqueue_at(&cfg, src, "alice", b"small".to_vec(), 100).expect("e2");

        struct FailFirst {
            n: usize,
        }
        impl MessageSender for FailFirst {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                Ok((b"CT".to_vec(), b"N".to_vec(), "c".to_string()))
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                self.n += 1;
                if self.n == 1 {
                    return Err(AttemptResult::Fail);
                }
                Ok(())
            }
            fn commit(&mut self, _r: &QueuedMessage) -> Result<(), &'static str> {
                Ok(())
            }
        }
        let mut s = FailFirst { n: 0 };
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");

        assert_eq!(out.failed, 1);
        assert_eq!(
            out.sent, 1,
            "the queue kept draining behind the failed message"
        );
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs[0].state, MsgState::Failed);
        assert_ne!(
            recs[0].state,
            MsgState::FailedPermanent,
            "O4: 413 is terminal for the message, NOT a permanent cause"
        );
        assert_eq!(recs[1].state, MsgState::Sent);
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn abandoning_a_packed_message_advances_the_ratchet_first() {
        // ⚠ NONCE REUSE GUARD, the msgqueue counterpart of NA-0155's
        // `abort_burns_state_and_prevents_nonce_reuse_on_next_send`.
        //
        // A packed record carries a ratchet advance that exists ONLY in `next_state`. If a
        // terminal transition drops it without committing, the session stays at the old
        // position and the next pack reuses the same message key -- catastrophic if the
        // abandoned ciphertext reached the relay (push sent, response lost).
        install_test_store_key();
        let cfg = temp_cfg("nonce_reuse");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"too big".to_vec(), 100).expect("e1");

        /// Records whether the ratchet was committed before the bytes were dropped.
        struct FailWithCommitWatch {
            committed: usize,
        }
        impl MessageSender for FailWithCommitWatch {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                Ok((b"CT".to_vec(), b"ADVANCED".to_vec(), "chan".to_string()))
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                // 413: terminal for this message, and it WAS packed.
                Err(AttemptResult::Fail)
            }
            fn commit(&mut self, rec: &QueuedMessage) -> Result<(), &'static str> {
                assert!(
                    rec.is_packed(),
                    "commit must see the ratchet state, not an already-cleared record"
                );
                assert_eq!(rec.next_state.as_deref(), Some(&b"ADVANCED"[..]));
                self.committed += 1;
                Ok(())
            }
        }
        let mut s = FailWithCommitWatch { committed: 0 };
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");

        assert_eq!(out.failed, 1);
        assert_eq!(
            s.committed, 1,
            "the ratchet advance MUST be committed before the bytes are dropped"
        );
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs[0].state, MsgState::Failed);
        assert!(!recs[0].is_packed(), "and only then are the bytes cleared");
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn a_failed_ratchet_commit_keeps_the_message_queued_rather_than_dropping_state() {
        // Fail-closed: if the advance cannot be committed, DO NOT drop the bytes. Keeping
        // the record queued is recoverable; dropping the state is not.
        install_test_store_key();
        let cfg = temp_cfg("nonce_failclosed");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"x".to_vec(), 100).expect("e1");

        struct CommitFails;
        impl MessageSender for CommitFails {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                Ok((b"CT".to_vec(), b"ADV".to_vec(), "chan".to_string()))
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                Err(AttemptResult::Fail)
            }
            fn commit(&mut self, _r: &QueuedMessage) -> Result<(), &'static str> {
                Err("qsp_session_store_failed")
            }
        }
        let mut s = CommitFails;
        let err = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).unwrap_err();
        assert_eq!(err, "qsp_session_store_failed");
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs[0].state, MsgState::Queued, "not marked terminal");
        assert!(
            recs[0].is_packed(),
            "and the ratchet state is STILL on disk"
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    // --- A7 / A8 / A10: the cause taxonomy ----------------------------------

    #[test]
    fn a7_every_pause_cause_is_distinct_in_both_vocabularies() {
        // A7: "each PAUSE cause distinct". LANE_INTENT §1: distinct causes get distinct
        // words and NOTHING collapses into a generic "couldn't send".
        //
        // ⚠ Both vocabularies matter and they are different audiences: `as_str` is the
        // Appendix-F taxonomy key an operator sees, `human` is the line a USER reads. A7 is
        // about the user-facing layer (OBS-EC), so both are checked for collisions.
        let all = [
            PausedCause::VaultLocked,
            PausedCause::TokenRejected,
            PausedCause::Cert,
        ];
        let keys: Vec<&str> = all.iter().map(|c| c.as_str()).collect();
        let humans: Vec<&str> = all.iter().map(|c| c.human()).collect();
        for i in 0..all.len() {
            for j in (i + 1)..all.len() {
                assert_ne!(keys[i], keys[j], "taxonomy keys collide");
                assert_ne!(humans[i], humans[j], "user-facing lines collide");
            }
            assert!(!keys[i].is_empty() && !humans[i].is_empty());
            assert!(
                !humans[i].contains("couldn't send"),
                "a cause must not collapse into the generic line: {}",
                humans[i]
            );
        }
    }

    #[test]
    fn a8_a_locked_vault_pauses_honestly_and_says_what_to_do() {
        // A8: "locked vault pauses honestly". §2h is claims-honesty: the pause is STRUCTURAL
        // (the store key lives in the vault), so the line must tell the user what to do
        // rather than imply the queue is working.
        let mut rec = sample("alice", 1, "aa");
        rec.paused_cause = Some(PausedCause::VaultLocked);
        assert_eq!(
            rec.state,
            MsgState::Queued,
            "paused is a SUB-STATE of queued"
        );
        assert!(!rec.state.is_terminal(), "a locked vault is never terminal");
        assert!(
            !rec.is_sendable_at(9_999),
            "and nothing is attempted while paused"
        );
        assert_eq!(PausedCause::VaultLocked.human(), "unlock to send");

        let s = ContactQueueSummary {
            peer_key: "k".into(),
            queued: 2,
            sent: 0,
            delivered: 0,
            failed: 0,
            paused: Some(PausedCause::VaultLocked),
            oldest_queued_age_s: Some(120),
        };
        let line = s.honest_line().expect("a queued contact has a line");
        assert_eq!(line, "2 queued — unlock to send");
        // ⚠ The words that would be a FALSE CLAIM if they appeared on a paused queue.
        for forbidden in ["sending", "in progress", "retrying"] {
            assert!(
                !line.contains(forbidden),
                "a paused queue must not read as active: {line}"
            );
        }
    }

    #[test]
    fn a8_an_unreachable_relay_says_it_will_send_later_not_that_it_is_sending() {
        let s = ContactQueueSummary {
            peer_key: "k".into(),
            queued: 1,
            sent: 0,
            delivered: 0,
            failed: 0,
            paused: None,
            oldest_queued_age_s: Some(5),
        };
        assert_eq!(
            s.honest_line().expect("line"),
            "1 queued — will send when the relay is reachable"
        );
    }

    #[test]
    fn a10_only_a_revoked_session_is_ever_permanent() {
        // A10, and the negative half is the point: O4 says no retryable failure is EVER
        // surfaced as permanent. So exactly one state is terminal-and-permanent, and every
        // other outcome must be recoverable or terminal-but-not-permanent.
        assert!(MsgState::FailedPermanent.is_terminal());
        assert!(
            MsgState::Failed.is_terminal(),
            "413 is terminal for its own message"
        );
        assert_ne!(
            MsgState::Failed,
            MsgState::FailedPermanent,
            "413 must NOT be permanent -- it heals against a relay with a larger limit"
        );
        assert!(!MsgState::Queued.is_terminal());
        assert!(!MsgState::Sent.is_terminal());
        // A paused row -- whatever the cause -- is never terminal, so it always has a way out.
        for c in [
            PausedCause::VaultLocked,
            PausedCause::TokenRejected,
            PausedCause::Cert,
        ] {
            let mut rec = sample("alice", 1, "aa");
            rec.paused_cause = Some(c);
            assert!(!rec.state.is_terminal(), "{c:?} must not be terminal");
        }
    }

    #[test]
    fn a10_a_revoked_session_reaches_failed_permanent_and_nothing_else_does() {
        install_test_store_key();
        let cfg = temp_cfg("a10_revoked");
        let src = ConfigSource::EnvOverride;
        enqueue_at(&cfg, src, "alice", b"x".to_vec(), 100).expect("e1");

        struct Revoked;
        impl MessageSender for Revoked {
            fn pack(
                &mut self,
                _r: &QueuedMessage,
            ) -> Result<(Vec<u8>, Vec<u8>, String), AttemptResult> {
                // The routing layer refuses: the contact's device state is REVOKED. That is
                // the ONLY trigger for the permanent state, and it is detected LOCALLY --
                // the relay has no session-revoked signal on the push path.
                Err(AttemptResult::FailPermanent)
            }
            fn push(&mut self, _r: &QueuedMessage) -> Result<(), AttemptResult> {
                unreachable!("must not push a message whose session is revoked")
            }
            fn commit(&mut self, _r: &QueuedMessage) -> Result<(), &'static str> {
                Ok(())
            }
        }
        let mut s = Revoked;
        let out = drain_at(&cfg, src, DrainTrigger::Scheduled, 100, &mut s).expect("drain");
        assert_eq!(out.failed, 1);
        let recs = load_contact(&cfg, "alice").expect("load");
        assert_eq!(recs[0].state, MsgState::FailedPermanent);
        let _ = fs::remove_dir_all(&cfg);
    }

    // --- state and backoff ---------------------------------------------------

    #[test]
    fn paused_is_a_sub_state_of_queued_and_never_terminal() {
        // O5 turns on this distinction: a paused row must not read as finished.
        let mut rec = sample("alice", 1, "aa");
        rec.paused_cause = Some(PausedCause::VaultLocked);
        assert_eq!(rec.state, MsgState::Queued);
        assert!(!rec.state.is_terminal());
        assert!(!rec.is_sendable_at(1_000), "a paused row is not sendable");
        // ...and the way out exists.
        arm_immediate(&mut rec, 1_000);
        assert_eq!(rec.paused_cause, None);
        assert!(rec.is_sendable_at(1_000));
    }

    #[test]
    fn backoff_climbs_the_ruled_ladder_and_caps_at_five_minutes() {
        // DESIGN §9 Q1: 5s -> 15s -> 45s -> 2m -> 5m, forever at the cap.
        let mut rec = sample("alice", 1, "aa");
        let mut seen = Vec::new();
        for _ in 0..7 {
            schedule_retry_at(&mut rec, 0, "relay_unreachable");
            seen.push(rec.next_attempt_at);
        }
        // Jitter is ADDED, never subtracted, so each delay is >= its rung.
        for (i, rung) in BACKOFF_LADDER_SECS.iter().enumerate() {
            assert!(seen[i] >= *rung, "rung {i}: {} < {rung}", seen[i]);
            assert!(
                seen[i] <= rung + BACKOFF_JITTER_SECS,
                "rung {i} over-jittered"
            );
        }
        assert!(
            seen[5] >= 300 && seen[6] >= 300,
            "caps at 5 minutes, forever"
        );
    }

    // --- in-flight ratchet state (§2c Option 1) -----------------------------

    #[test]
    fn a_fresh_record_is_not_packed_and_a_packed_one_replays() {
        // ⚠ The invariant the operator called non-negotiable: once packed, a retry replays
        // the SAME bytes. Re-packing would burn a second message key and desync the
        // session, so `is_packed` is the predicate every send path must consult BEFORE
        // deciding to pack.
        let mut rec = sample("alice", 1, "aa");
        assert!(!rec.is_packed(), "a fresh row has consumed no message key");

        rec.mark_packed(
            b"CIPHERTEXT".to_vec(),
            b"NEXTSTATE".to_vec(),
            "chan-a".to_string(),
        );
        assert!(rec.is_packed());
        assert_eq!(rec.ciphertext.as_deref(), Some(&b"CIPHERTEXT"[..]));
        assert_eq!(rec.next_state.as_deref(), Some(&b"NEXTSTATE"[..]));
        assert_eq!(rec.channel.as_deref(), Some("chan-a"));
    }

    #[test]
    fn in_flight_state_survives_a_round_trip_so_a_retry_replays_identical_bytes() {
        // The bytes must come back BYTE-IDENTICAL across a store round trip, because a
        // retry after a crash replays exactly what is on disk.
        let k = test_key();
        let mut rec = sample("alice", 4, "0123456789abcdef0123456789abcdef");
        rec.mark_packed(
            vec![0, 1, 2, 250, 251, 255],
            vec![9, 8, 7],
            "chan-a".to_string(),
        );
        let aad = record_aad(&contact_key("alice"), &rec.msg_id, rec.seq);
        let ct = encrypt_record(&k, &aad, &rec).expect("encrypt");
        let back = decrypt_record(&k, &aad, &ct).expect("decrypt");
        assert_eq!(
            back.ciphertext, rec.ciphertext,
            "replayed bytes must be identical"
        );
        assert_eq!(
            back.next_state, rec.next_state,
            "ratchet state rides with them"
        );
        assert_eq!(back.channel, rec.channel);
        assert!(back.is_packed());
    }

    #[test]
    fn clearing_in_flight_state_drops_all_three_together() {
        // They move together or the record can end up with ciphertext and no ratchet state
        // to commit after it -- a shape no crash should be able to produce.
        let mut rec = sample("alice", 1, "aa");
        rec.mark_packed(b"C".to_vec(), b"N".to_vec(), "chan".to_string());
        rec.clear_inflight();
        assert!(!rec.is_packed());
        assert!(rec.ciphertext.is_none() && rec.next_state.is_none() && rec.channel.is_none());
    }

    #[test]
    fn jitter_is_stable_for_the_same_record() {
        // A record re-read from disk must not keep moving its own deadline.
        let mut a = sample("alice", 1, "0123456789abcdef0123456789abcdef");
        let mut b = a.clone();
        schedule_retry_at(&mut a, 500, "x");
        schedule_retry_at(&mut b, 500, "x");
        assert_eq!(a.next_attempt_at, b.next_attempt_at);
    }
}
