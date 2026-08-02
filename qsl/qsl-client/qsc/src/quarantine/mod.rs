//! Quarantine — the local encrypted store that replaces client-side destruction.
//!
//! ⚠ WHY THIS EXISTS. NA-0688/C4 removed delete-on-pull as the default, so an item a command
//! merely *cannot process* now survives and REDELIVERS. What it did not fix — and D-1327 §4
//! kept explicit rather than let merge into "the pull bug was fixed" — is the point where the
//! client itself decides an item will never be processed. There it **acks the item away and
//! the bytes are destroyed with only a log marker as witness.**
//!
//! This module is the somewhere-to-put-it. Every censused discard point writes the item here
//! FIRST and acks SECOND: same wire behaviour, same loop-ending, **zero client-side data
//! destruction.**
//!
//! ## ⚠ THE SHARP BOUNDARY (D623 §1a) — quarantine replaces DESTRUCTION, never REDELIVERY
//!
//! An item that merely cannot be processed by *this* command must keep redelivering so the
//! right command can drain it. Converting a redelivery into a quarantine entry would steal
//! ordinary messages from `qsc receive`. **Only paths that would otherwise ack-and-destroy
//! write here.** The three flag-less pull callers (`invite accept`, `invite finish`,
//! `handshake poll`) ack nothing and are deliberately NOT wired to this module.
//!
//! ## The two subclasses (D-1328 Ruling 2)
//!
//! Captured items fall in two kinds, and they are **separately witnessed** so a forward-compat
//! capture is never confusable with a decrypt failure:
//!
//! - [`Subclass::Unrecoverable`] — the message key was consumed and the plaintext can never be
//!   recovered by this or any build (the NA-0644 replay backstop and the three reject arms).
//! - [`Subclass::Unsupported`] — *judged not-for-this-build*: a control payload carrying our
//!   namespace marker but of a type this build does not know. ⚠ A FUTURE BUILD WOULD
//!   UNDERSTAND IT, and redelivery cannot save it because every current build acks it away on
//!   sight. **No re-ingestion tooling is promised or built** — the store's value here is
//!   preserved evidence, nothing more.
//!
//! ## Why THIS home (D-1328 Ruling 1, ratifying the STOP-1 §5e proposal)
//!
//! The store key lives in the vault; the RECORDS are AEAD files, one file per record, written
//! by a single `write_atomic` with no read-modify-write. That is the `msgqueue` pattern, and
//! it was chosen for a measured reason rather than by resemblance:
//!
//! ⚠ `owed_receipts` puts its records INSIDE a vault secret, and its own header names the
//! load-bearing premise — that is affordable **because the writes are RARE**, since
//! `vault::secret_set` re-encrypts the WHOLE vault (~18 ms, 95–97% Argon2id, ENG-0053). It
//! also says the choice must be **reopened, not quietly kept**, if that stops being true.
//!
//! **Quarantine inherits the rarity and breaks the OTHER assumption: SIZE.** An owed receipt is
//! a `(peer, msg_id, owed_at_unix)` triple at ~10² bytes. A quarantine entry is a whole
//! envelope — ~4 KiB at `META_BUCKET_MAX_DEFAULT`, up to 64 KiB at the ceiling: **40× to 650×
//! larger.** And because `secret_set` re-encrypts the entire vault, a vault-homed quarantine
//! would tax **every future write of every other vault secret**, not merely its own. The
//! precedent transfers on frequency and fails on size.
//!
//! ## ⚠ This is a FOURTH persistence home for in-flight state, accepted deliberately
//!
//! ENG-0083 files that in-flight state lives in multiple places; `owed_receipts` recorded
//! itself as the third. This is the fourth, ruled and recorded at STOP #1. **Consolidating
//! them is ENG-0083's job and explicitly not this lane's.**

use super::*;
use std::sync::{Mutex, OnceLock};

// ---------------------------------------------------------------------------
// Layout and constants
// ---------------------------------------------------------------------------

/// Root of the quarantine store, under the config dir.
pub(crate) const QUARANTINE_DIR: &str = "quarantine_v1";

/// The vault secret holding the 32-byte store key (hex). ⚠ The KEY is a vault secret; the
/// RECORDS are files. See the module header for why.
const STORE_KEY_SECRET: &str = "quarantine_store_key_v1";

const STORE_KEY_LEN: usize = 32;
const NONCE_LEN: usize = 12;
const RECORD_VERSION: u8 = 1;

/// ⚠ MEASURED, NOT CHOSEN — and measured by a predecessor rather than by this lane: both
/// deployed relays report `retention.ttl_secs = 604800` from `GET /v1/server-info`, which is
/// how `owed_receipts` derived the same number. A quarantined item must not outlive the
/// retention window of the relay that delivered it; past that point the relay has dropped its
/// copy too, and holding ours only pretends to a recoverability nothing else still supports.
pub(crate) const QUARANTINE_TTL_SECS: u64 = 604_800;

/// ⚠ GLOBAL, NOT PER-PEER — D-1328 Ruling 1, and the distinction is load-bearing.
/// `owed_receipts` caps per peer so one contact's flood cannot evict another's obligation.
/// **That shape must not be copied blindly here:** a D1-class item is one whose *decryption
/// failed*, so its sender may be unattributable at the moment we store it. Bucketing by peer
/// would require an attribution we do not have, and inventing one would file items under the
/// wrong contact. The cap is therefore global, and eviction is oldest-first.
pub(crate) const QUARANTINE_MAX_ENTRIES: usize = 256;

/// The global byte ceiling, **derived from two constants already in the tree** rather than
/// invented: [`QUARANTINE_MAX_ENTRIES`] × `META_BUCKET_MAX_CEILING` (65 536) = 16 MiB. That is
/// the largest the store can be if every one of its slots held a maximum-size envelope.
///
/// ⚠ There is deliberately NO per-entry cap. An item in hand **arrived through the relay**, so
/// its size is already bounded by the relay's advertised `limits.max_body_bytes`; adding a
/// second, client-invented per-entry limit would reject items the relay had already accepted —
/// destroying exactly what this module exists to preserve.
pub(crate) const QUARANTINE_MAX_BYTES: usize = QUARANTINE_MAX_ENTRIES * 65_536;

// Failure causes are `&'static str` constants, NOT new `ErrorCode` variants — the D599
// pattern `invite/mod.rs` and `msgqueue` already follow for new failure causes.
pub const QUARANTINE_STORE_UNAVAILABLE: &str = "quarantine_store_unavailable";
pub const QUARANTINE_RECORD_TAMPERED: &str = "quarantine_record_tampered";
pub const QUARANTINE_WRITE_FAILED: &str = "quarantine_write_failed";
pub const QUARANTINE_NOT_FOUND: &str = "quarantine_not_found";
pub const QUARANTINE_VAULT_LOCKED: &str = "vault_locked";

// ---------------------------------------------------------------------------
// Subclass
// ---------------------------------------------------------------------------

/// Why an item was captured. ⚠ Ruling 2 requires these be **separately witnessed**, so this is
/// a distinct field in the record AND a distinct value on the entry marker — a forward-compat
/// capture must never read as a decrypt failure.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subclass {
    /// The message key was consumed; the plaintext is gone for every build, forever.
    Unrecoverable,
    /// Ours by namespace marker, but of a type this build does not know. A future build could
    /// read it. ⚠ No re-ingestion tooling exists; this is preserved evidence.
    Unsupported,
}

impl Subclass {
    pub fn as_str(self) -> &'static str {
        match self {
            Subclass::Unrecoverable => "unrecoverable",
            Subclass::Unsupported => "unsupported",
        }
    }
}

/// What the stored bytes ARE (D-1328 Ruling 7).
///
/// ⚠ **THIS IS NOT DERIVABLE FROM [`Subclass`], AND THAT IS THE WHOLE REASON IT EXISTS.** The two
/// rulings partition the same five sites **differently**:
///
/// | | D1 | D2 | D3 | D4 | D5 |
/// |---|---|---|---|---|---|
/// | `Subclass` (Ruling 2) | Unrecoverable | Unrecoverable | Unrecoverable | Unrecoverable | **Unsupported** |
/// | `ContentKind` (Ruling 7) | **WireEnvelope** | InnerPayload | InnerPayload | InnerPayload | InnerPayload |
///
/// The boundary falls between D1 and D2 for content and between D4 and D5 for subclass. **Neither
/// field can be inferred from the other**, so a reader holding only the subclass cannot tell
/// whether `data` is ciphertext or plaintext — and would have to guess to interpret it at all.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentKind {
    /// The item exactly as pulled from the relay. ⚠ **Permanently undecryptable by everyone** —
    /// D1 is the decrypt-failure path, so the message key was consumed in an earlier run. Stored
    /// because it is the only thing that exists, and for correlation, never for recovery.
    WireEnvelope,
    /// The decrypted inner payload, captured after `commit_unpack_state` consumed the key.
    ///
    /// ⚠ Ruling 7's grounds, recorded because they are the reason this is not the wire bytes: from
    /// the moment the key is consumed the ciphertext is undecryptable **by everyone, forever**, so
    /// storing it alone would make [`Subclass::Unsupported`]'s value **vacuous** — no future build
    /// could ever read what it was kept for — and would gut D2–D4's diagnostic value too. The
    /// plaintext rests under this store's own AEAD: the **same protection class as `msgqueue` and
    /// the timeline**, TTL'd and deletable.
    InnerPayload,
}

impl ContentKind {
    pub fn as_str(self) -> &'static str {
        match self {
            ContentKind::WireEnvelope => "wire_envelope",
            ContentKind::InnerPayload => "inner_payload",
        }
    }
}

// ---------------------------------------------------------------------------
// Record
// ---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct QuarantinedItem {
    pub v: u8,
    /// The quarantine entry id (hex). ⚠ NOT the relay item id — see [`mint_entry_id`].
    pub entry_id: String,
    /// The relay's item id, retained so an operator can correlate with relay-side logs.
    pub relay_item_id: String,
    pub subclass: Subclass,
    /// What [`Self::data`] holds. ⚠ **Orthogonal to `subclass`** — see [`ContentKind`]. Without
    /// this a reader cannot tell ciphertext from plaintext and would have to guess.
    pub content: ContentKind,
    /// The failure/ignore code that caused the capture (`qsp_replay_reject`,
    /// `unknown_control_type`, …). Metadata only; never rendered as content.
    pub reason: String,
    /// The site that captured it, so a later reader can find the branch without guessing.
    pub site: String,
    pub captured_at_unix: u64,
    /// The captured bytes — **the thing that used to be destroyed.**
    ///
    /// ⚠ **WHAT THESE BYTES ARE DEPENDS ON [`Self::content`], NOT ON `subclass`.** An earlier
    /// draft of this comment said "exactly as pulled", which Ruling 7 makes false for four of the
    /// five sites. Read `content` before interpreting this field.
    pub data: Vec<u8>,
}

// ---------------------------------------------------------------------------
// Store key: held in the vault, cached per process (the msgqueue F4 pattern)
// ---------------------------------------------------------------------------

static STORE_KEY_CACHE: OnceLock<Mutex<Option<[u8; STORE_KEY_LEN]>>> = OnceLock::new();

fn store_key_slot() -> &'static Mutex<Option<[u8; STORE_KEY_LEN]>> {
    STORE_KEY_CACHE.get_or_init(|| Mutex::new(None))
}

fn store_key() -> Result<[u8; STORE_KEY_LEN], &'static str> {
    let slot = store_key_slot();
    let mut guard = slot.lock().map_err(|_| QUARANTINE_STORE_UNAVAILABLE)?;
    if let Some(k) = *guard {
        return Ok(k);
    }
    let existing = vault::secret_get(STORE_KEY_SECRET).map_err(|e| match e {
        "vault_missing" | "vault_locked" => QUARANTINE_VAULT_LOCKED,
        _ => QUARANTINE_STORE_UNAVAILABLE,
    })?;
    let key = match existing {
        Some(hex) if !hex.trim().is_empty() => {
            let bytes = hex_decode(hex.trim()).map_err(|_| QUARANTINE_STORE_UNAVAILABLE)?;
            if bytes.len() != STORE_KEY_LEN {
                return Err(QUARANTINE_STORE_UNAVAILABLE);
            }
            let mut k = [0u8; STORE_KEY_LEN];
            k.copy_from_slice(&bytes);
            k
        }
        _ => {
            let mut k = [0u8; STORE_KEY_LEN];
            OsRng.fill_bytes(&mut k);
            vault::secret_set(STORE_KEY_SECRET, &hex_encode(&k)).map_err(|e| match e {
                "vault_missing" | "vault_locked" => QUARANTINE_VAULT_LOCKED,
                _ => QUARANTINE_STORE_UNAVAILABLE,
            })?;
            k
        }
    };
    *guard = Some(key);
    Ok(key)
}

// ---------------------------------------------------------------------------
// Paths and ids
// ---------------------------------------------------------------------------

fn quarantine_root(cfg_dir: &Path) -> PathBuf {
    cfg_dir.join(QUARANTINE_DIR)
}

/// ⚠ A RELAY ITEM ID MUST NEVER APPEAR IN A FILENAME — the rule `dedup::mailbox_store_key` and
/// `msgqueue::contact_key` both state for their own identifiers. Filenames are visible
/// metadata even when the contents are encrypted, and a relay item id is a live correlator.
fn mint_entry_id(relay_item_id: &str, captured_at: u64) -> String {
    let c = StdCrypto;
    let h = c.sha512(format!("{}|{}", relay_item_id, captured_at).as_bytes());
    hex_encode(&h[..8])
}

/// Zero-padded timestamp first, so a plain lexicographic listing is already oldest-first —
/// which is the order eviction needs, with no sort and no second source of truth.
fn record_name(captured_at: u64, entry_id: &str) -> String {
    format!("{:020}_{}.qrec", captured_at, entry_id)
}

// ---------------------------------------------------------------------------
// Record encryption
// ---------------------------------------------------------------------------

/// AAD binds the record to its entry id, so a record cannot be renamed or reordered on disk
/// and still decrypt cleanly in its new position.
fn record_aad(entry_id: &str) -> Vec<u8> {
    format!("qsc.quarantine.v1|{}", entry_id).into_bytes()
}

fn encrypt_record(
    key: &[u8; STORE_KEY_LEN],
    aad: &[u8],
    rec: &QuarantinedItem,
) -> Result<Vec<u8>, &'static str> {
    let plaintext = serde_json::to_vec(rec).map_err(|_| QUARANTINE_WRITE_FAILED)?;
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
        .map_err(|_| QUARANTINE_WRITE_FAILED)?;
    let mut out = Vec::with_capacity(NONCE_LEN + ct.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ct);
    Ok(out)
}

fn decrypt_record(
    key: &[u8; STORE_KEY_LEN],
    aad: &[u8],
    bytes: &[u8],
) -> Result<QuarantinedItem, &'static str> {
    if bytes.len() <= NONCE_LEN {
        return Err(QUARANTINE_RECORD_TAMPERED);
    }
    let (nonce_bytes, ct) = bytes.split_at(NONCE_LEN);
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key));
    let pt = cipher
        .decrypt(Nonce::from_slice(nonce_bytes), Payload { msg: ct, aad })
        .map_err(|_| QUARANTINE_RECORD_TAMPERED)?;
    serde_json::from_slice(&pt).map_err(|_| QUARANTINE_RECORD_TAMPERED)
}

// ---------------------------------------------------------------------------
// Directory scan — the one place that reads the store's shape
// ---------------------------------------------------------------------------

/// `(path, captured_at, size_on_disk)` for every record, **oldest first** by filename.
fn scan(cfg_dir: &Path) -> Vec<(PathBuf, u64, usize)> {
    let root = quarantine_root(cfg_dir);
    let entries = match fs::read_dir(&root) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    let mut out: Vec<(PathBuf, u64, usize)> = Vec::new();
    for e in entries.flatten() {
        let path = e.path();
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if !name.ends_with(".qrec") {
            continue;
        }
        let ts = name
            .split('_')
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let size = e.metadata().map(|m| m.len() as usize).unwrap_or(0);
        out.push((path, ts, size));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

/// Drop expired entries, emitting a witness for each.
///
/// ⚠ THE MARKER IS NOT DECORATION. An expired quarantine entry is a real loss — it is the very
/// data this module exists to keep — so it leaves a witness for the same reason an eviction
/// does: **a drop with no witness is the defect this lane was created to remove.**
fn prune_expired(cfg_dir: &Path, now: u64) {
    let mut expired = 0usize;
    for (path, ts, _) in scan(cfg_dir) {
        if now.saturating_sub(ts) > QUARANTINE_TTL_SECS && fs::remove_file(&path).is_ok() {
            expired = expired.saturating_add(1);
        }
    }
    if expired > 0 {
        let n = expired.to_string();
        emit_marker(
            "quarantine_expired",
            None,
            &[("count", n.as_str()), ("ttl_secs", "604800")],
        );
    }
}

/// Enforce the GLOBAL caps, oldest-evicted, **every eviction witnessed**.
fn enforce_caps(cfg_dir: &Path) {
    let mut items = scan(cfg_dir);
    let mut total: usize = items.iter().map(|(_, _, s)| *s).sum();
    let mut evicted = 0usize;
    let mut idx = 0usize;
    while (items.len() - idx > QUARANTINE_MAX_ENTRIES || total > QUARANTINE_MAX_BYTES)
        && idx < items.len()
    {
        let (path, _, size) = &items[idx];
        if fs::remove_file(path).is_ok() {
            total = total.saturating_sub(*size);
            evicted = evicted.saturating_add(1);
        }
        idx += 1;
    }
    items.truncate(0);
    if evicted > 0 {
        let n = evicted.to_string();
        emit_marker(
            "quarantine_evicted",
            None,
            &[("count", n.as_str()), ("reason", "store_full")],
        );
    }
}

// ---------------------------------------------------------------------------
// Write
// ---------------------------------------------------------------------------

/// Capture an item that the caller is about to ack away.
///
/// ⚠ **FAIL-CLOSED, AND THE CONSEQUENCE IS RATIFIED (D-1328, STOP #002 concurrence 1).** If
/// this returns `Err`, the caller **MUST NOT ack**. The item then keeps redelivering — a loud,
/// witnessed availability degradation, deliberately chosen over silent destruction. **That
/// redelivery loop is a DECISION, not a regression**, and a future reader must not "fix" it:
/// NA-0644's backstop exists to end a poison loop and D-1327 §3a records a lane that predicted
/// a wedge from redelivery and was wrong, so the pull toward killing the loop is strong and
/// wrong here.
// ⚠ ARGUED, NOT SILENT (D-1328 Ruling 10's standard, settled by Ruling 13). These nine arguments
// ARE the capture record's own fields -- cfg dir and source, the relay item id, the two independent
// discriminators (subclass and content kind, which by Rulings 2 and 7 neither implies), the reason,
// the site, the bytes, and the clock. A params struct here would add a type whose only purpose is
// to satisfy a lint: it would remove no decision, no argument, and no call site, and would put a
// second name on the same nine fields. Revisit if a TENTH is ever wanted -- that would be evidence
// the function is accreting responsibilities rather than fields.
//
// ⚠ The params-struct form is DEFERRED, NOT REJECTED (Ruling 13 rider i), and the counter-argument
// is kept rather than buried: positional same-typed discriminators are a standing TRANSPOSITION
// hazard that named-field construction would remove. Today that line is held by the Ruling 11.2 and
// 11.3 pins instead; the refactor is natural to the ENG-0083 consolidation context.
#[allow(clippy::too_many_arguments)]
pub(crate) fn capture_at(
    cfg_dir: &Path,
    source: ConfigSource,
    relay_item_id: &str,
    subclass: Subclass,
    content: ContentKind,
    reason: &str,
    site: &str,
    data: &[u8],
    now: u64,
) -> Result<String, &'static str> {
    let key = store_key()?;
    capture_with_key_at(
        &key,
        cfg_dir,
        source,
        relay_item_id,
        subclass,
        content,
        reason,
        site,
        data,
        now,
    )
}

/// The key-injectable inner. ⚠ Split out for the same reason `msgqueue` keeps a `test_key()`
/// helper and the clock keeps `_at` seams: the file-level behaviour — write failure, capping,
/// eviction — must be provable **without standing up a vault**, or the guards that matter most
/// would be the ones hardest to test and therefore the ones left untested.
#[allow(clippy::too_many_arguments)]
fn capture_with_key_at(
    key: &[u8; STORE_KEY_LEN],
    cfg_dir: &Path,
    source: ConfigSource,
    relay_item_id: &str,
    subclass: Subclass,
    content: ContentKind,
    reason: &str,
    site: &str,
    data: &[u8],
    now: u64,
) -> Result<String, &'static str> {
    let entry_id = mint_entry_id(relay_item_id, now);
    let rec = QuarantinedItem {
        v: RECORD_VERSION,
        entry_id: entry_id.clone(),
        relay_item_id: relay_item_id.to_string(),
        subclass,
        content,
        reason: reason.to_string(),
        site: site.to_string(),
        captured_at_unix: now,
        data: data.to_vec(),
    };
    let aad = record_aad(&entry_id);
    let blob = encrypt_record(key, &aad, &rec)?;

    let root = quarantine_root(cfg_dir);
    fs_store::ensure_dir_secure(&root, source).map_err(|_| QUARANTINE_WRITE_FAILED)?;
    let path = root.join(record_name(now, &entry_id));
    // ONE write_atomic, no read-modify-write: a record is either fully committed or absent.
    fs_store::write_atomic(&path, &blob, source).map_err(|_| QUARANTINE_WRITE_FAILED)?;

    let size_s = data.len().to_string();
    emit_marker(
        "quarantine_stored",
        None,
        &[
            ("id", entry_id.as_str()),
            // ⚠ Ruling 2: the subclass is on the marker, so a forward-compat capture is
            // distinguishable from a decrypt failure in the log alone.
            ("subclass", subclass.as_str()),
            // ⚠ Ruling 7: the content kind is orthogonal to the subclass, so it must be on the
            // marker too — the log alone must say whether these bytes are ciphertext or plaintext.
            ("content", content.as_str()),
            ("reason", reason),
            ("site", site),
            ("bytes", size_s.as_str()),
        ],
    );

    // Housekeeping AFTER the entry is durable, so a full store never costs us the new item.
    prune_expired(cfg_dir, now);
    enforce_caps(cfg_dir);
    Ok(entry_id)
}

// ---------------------------------------------------------------------------
// Read / delete — the P4 inspection surface
// ---------------------------------------------------------------------------

/// Redacted metadata for one entry. ⚠ **THE FIELD NAMES, NEVER THE VALUES** — there is no
/// accessor for `data` here and none is offered; `show` and `export` are out of scope.
pub struct QuarantineSummary {
    pub entry_id: String,
    pub subclass: &'static str,
    pub content: &'static str,
    pub reason: String,
    pub site: String,
    pub captured_at_unix: u64,
    pub bytes: usize,
}

pub(crate) fn list(cfg_dir: &Path) -> Result<Vec<QuarantineSummary>, &'static str> {
    let key = store_key()?;
    let mut out = Vec::new();
    for (path, _, _) in scan(cfg_dir) {
        let blob = match fs::read(&path) {
            Ok(b) => b,
            Err(_) => continue,
        };
        let entry_id = path
            .file_name()
            .and_then(|n| n.to_str())
            .and_then(|n| n.strip_suffix(".qrec"))
            .and_then(|n| n.split_once('_').map(|(_, id)| id.to_string()))
            .unwrap_or_default();
        let aad = record_aad(&entry_id);
        let rec = decrypt_record(&key, &aad, &blob)?;
        out.push(QuarantineSummary {
            entry_id: rec.entry_id,
            subclass: rec.subclass.as_str(),
            content: rec.content.as_str(),
            reason: rec.reason,
            site: rec.site,
            captured_at_unix: rec.captured_at_unix,
            bytes: rec.data.len(),
        });
    }
    Ok(out)
}

/// Delete one entry by id. **A stored item must always be deletable** (D623 P4).
///
/// ⚠ A PLAIN DELETE IS CORRECT HERE, AND THAT WAS CHECKED RATHER THAN ASSUMED. `outbox
/// discard` routes through `msgqueue::retire_packed` because discarding a PACKED OUTGOING
/// message without committing its ratchet advance is nonce reuse (ENG-0095). A quarantined
/// item is an INCOMING item whose message key was **already consumed at `commit_unpack_state`
/// before it ever reached us**, so no advance is owed and there is no barrier to route
/// through. Copying the outbox shape here would add a ratchet operation with nothing to
/// ratchet.
pub(crate) fn drop_entry(cfg_dir: &Path, entry_id: &str) -> Result<(), &'static str> {
    for (path, _, _) in scan(cfg_dir) {
        let matches = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.contains(entry_id))
            .unwrap_or(false);
        if matches {
            fs::remove_file(&path).map_err(|_| QUARANTINE_STORE_UNAVAILABLE)?;
            emit_marker("quarantine_dropped", None, &[("id", entry_id)]);
            return Ok(());
        }
    }
    Err(QUARANTINE_NOT_FOUND)
}

// NA-0689 D-1328 Ruling 10: `count()` was built here for a `doctor` one-line rider that did not
// land, and it was dead code. REMOVED rather than given a caller, and never silenced with an
// `#[allow]`. Two reasons, both measured:
//
//   1. The rider is NOT free. `doctor` emits ONE `print_marker` line, and `tests/cli.rs`
//      (`doctor_check_only_no_dir`) pins that line with `predicate::eq` — an EXACT stdout equality.
//      Adding a key means editing a pinned CLI contract in a file this lane otherwise does not
//      touch. D623 §P4: a rider that grows is not a rider.
//   2. It was redundant anyway. `quarantine_list` already derives and emits `count=N` from
//      `list()`, so the count a user can ask for is not lost with this function.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ttl_matches_the_measured_relay_retention() {
        // ⚠ Derived, not chosen: `retention.ttl_secs` as both deployed relays report it —
        // the same provenance `owed_receipts` used. If a relay's retention changes, this
        // must be re-derived rather than left as a number someone once liked.
        assert_eq!(QUARANTINE_TTL_SECS, 604_800);
    }

    #[test]
    fn the_byte_cap_is_derived_from_the_entry_cap_and_the_envelope_ceiling() {
        // 256 slots × META_BUCKET_MAX_CEILING (65 536) = 16 MiB. Pinned so the derivation
        // cannot be quietly replaced by a round number.
        assert_eq!(QUARANTINE_MAX_BYTES, QUARANTINE_MAX_ENTRIES * 65_536);
        assert_eq!(QUARANTINE_MAX_BYTES, 16 * 1024 * 1024);
    }

    #[test]
    fn the_entry_cap_is_global_not_per_peer() {
        // ⚠ A GUARD ON A DECISION, not on a value. `owed_receipts` caps PER PEER; this caps
        // globally, because a D1-class item's sender may be unattributable at capture time.
        // The constant carries no peer dimension and this pins that it never grows one.
        let one_global_cap: usize = QUARANTINE_MAX_ENTRIES;
        assert_eq!(one_global_cap, 256);
    }

    #[test]
    fn subclasses_are_distinguishable_in_the_witness() {
        // ⚠ Ruling 2: a forward-compat capture must never read as a decrypt failure.
        assert_ne!(
            Subclass::Unrecoverable.as_str(),
            Subclass::Unsupported.as_str()
        );
        assert_eq!(Subclass::Unrecoverable.as_str(), "unrecoverable");
        assert_eq!(Subclass::Unsupported.as_str(), "unsupported");
    }

    fn test_key() -> [u8; STORE_KEY_LEN] {
        [7u8; STORE_KEY_LEN]
    }

    fn tmp_cfg(tag: &str) -> PathBuf {
        // ⚠ `set_umask_077` is what the BINARY does at startup; without it `create_dir_all`
        // yields 0755 and `fs_store::enforce_dir_perms` correctly refuses the store. The first
        // draft of these probes omitted it and BOTH failed on the harness rather than on the
        // property -- and one of them "passed" for the wrong reason as a result (see
        // `a_failed_quarantine_write_is_loud_and_returns_err`'s positive control).
        crate::fs_store::set_umask_077();
        let base = std::env::temp_dir()
            .join(format!("qsc_quarantine_p1_{}_{}", tag, std::process::id()));
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("cfg dir");
        base
    }

    #[allow(clippy::too_many_arguments)]
    fn capture_probe(cfg: &Path, id: &str, now: u64) -> Result<String, &'static str> {
        capture_with_key_at(
            &test_key(),
            cfg,
            ConfigSource::EnvOverride,
            id,
            Subclass::Unrecoverable,
            ContentKind::InnerPayload,
            "attachment_confirm_reject",
            "transport::receive_pull_and_write",
            b"payload",
            now,
        )
    }

    fn drain_markers() -> Vec<String> {
        let q = crate::output::marker_queue();
        let mut g = q.lock().expect("marker queue");
        let out: Vec<String> = g.iter().cloned().collect();
        g.clear();
        out
    }

    /// ⚠ P1's FIRST OWED PROBE — **a quarantine write failure is LOUD, and the caller is told.**
    ///
    /// The store's half of the fail-closed contract is that it returns `Err` rather than
    /// pretending. The caller's half — **do not ack** — is pinned per-site in P2; this proves the
    /// signal exists for those sites to honour.
    #[test]
    fn a_failed_quarantine_write_is_loud_and_returns_err() {
        let cfg = tmp_cfg("writefail");
        // Occupy the store root with a FILE, so the directory can never be created. This is a
        // real filesystem failure, not a mocked one.
        let root = cfg.join(QUARANTINE_DIR);
        fs::write(&root, b"not a directory").expect("plant blocker");

        let res = capture_probe(&cfg, "relay-item-1", 1_000);

        // ⚠ SILENCE WOULD BE THE DEFECT. A store that swallowed this and returned Ok would let
        // the caller ack -- destroying the item while reporting success.
        assert!(res.is_err(), "a failed write must not report success");
        assert_eq!(res.unwrap_err(), QUARANTINE_WRITE_FAILED);
        // And nothing was left behind pretending to be a record.
        assert!(scan(&cfg).is_empty(), "no record may exist after a failed write");

        // ⚠ THE POSITIVE CONTROL, AND IT IS NOT OPTIONAL. Without it this test passes whenever
        // capture fails for ANY reason -- including a broken harness, which is exactly what
        // happened on the first run: the umask was wrong and the store refused the directory,
        // so the assertion above was satisfied by the wrong cause. A negative result is only
        // evidence if the instrument could have returned positive.
        fs::remove_file(&root).expect("lift the blocker");
        let ok = capture_probe(&cfg, "relay-item-1", 1_000);
        assert!(
            ok.is_ok(),
            "with the blocker lifted the SAME call must succeed, else the red above proves nothing: {:?}",
            ok
        );
        assert_eq!(scan(&cfg).len(), 1, "the control write must land");
        let _ = fs::remove_dir_all(&cfg);
    }

    /// ⚠ P1's SECOND OWED PROBE — **eviction is WITNESSED, and it is oldest-first.**
    ///
    /// A drop with no witness is the defect this lane exists to remove, so the marker is the
    /// assertion, not the file count.
    #[test]
    fn store_full_eviction_is_witnessed_and_oldest_first() {
        crate::output::set_marker_routing(crate::output::MarkerRouting::InApp);
        let cfg = tmp_cfg("evict");
        let over = 3usize;
        for i in 0..(QUARANTINE_MAX_ENTRIES + over) {
            capture_probe(&cfg, &format!("relay-item-{}", i), 1_000 + i as u64).expect("capture");
        }

        let markers = drain_markers();
        crate::output::set_marker_routing(crate::output::MarkerRouting::Stdout);

        // The cap holds.
        assert_eq!(
            scan(&cfg).len(),
            QUARANTINE_MAX_ENTRIES,
            "the global cap must hold"
        );
        // ⚠ THE WITNESS IS THE POINT.
        let evictions: Vec<&String> = markers
            .iter()
            .filter(|m| m.contains("quarantine_evicted"))
            .collect();
        assert!(
            !evictions.is_empty(),
            "eviction must emit a witness; a silent drop is the defect"
        );
        assert!(
            evictions.iter().any(|m| m.contains("store_full")),
            "the witness must name its cause"
        );
        // Oldest-first: the earliest timestamps are the ones gone.
        let remaining = scan(&cfg);
        let oldest_kept = remaining.first().map(|(_, ts, _)| *ts).unwrap_or(0);
        assert!(
            oldest_kept > 1_000,
            "the oldest entries must be the evicted ones, got {}",
            oldest_kept
        );
        let _ = fs::remove_dir_all(&cfg);
    }

    #[test]
    fn subclass_and_content_kind_are_orthogonal() {
        // ⚠ THE GUARD THAT STOPS THE TWO RULINGS BEING CONFLATED. Ruling 2 splits the five
        // sites {D1..D4} | {D5}; Ruling 7 splits them {D1} | {D2..D5}. The boundaries fall in
        // DIFFERENT PLACES, so neither field can ever be derived from the other.
        //
        // D1 is Unrecoverable AND WireEnvelope; D2 is Unrecoverable AND InnerPayload. If some
        // later change tried to infer content from subclass, these two would collide -- this
        // pins that the pairing is free in exactly that spot.
        let d1 = (Subclass::Unrecoverable, ContentKind::WireEnvelope);
        let d2 = (Subclass::Unrecoverable, ContentKind::InnerPayload);
        assert_eq!(d1.0, d2.0, "D1 and D2 share a subclass");
        assert_ne!(d1.1, d2.1, "but they do NOT share a content kind");

        // And the other direction: D4 and D5 differ in subclass while sharing content.
        let d4 = (Subclass::Unrecoverable, ContentKind::InnerPayload);
        let d5 = (Subclass::Unsupported, ContentKind::InnerPayload);
        assert_ne!(d4.0, d5.0, "D4 and D5 differ in subclass");
        assert_eq!(d4.1, d5.1, "but share a content kind");
    }

    #[test]
    fn content_kinds_are_distinguishable_in_the_witness() {
        assert_ne!(
            ContentKind::WireEnvelope.as_str(),
            ContentKind::InnerPayload.as_str()
        );
        assert_eq!(ContentKind::WireEnvelope.as_str(), "wire_envelope");
        assert_eq!(ContentKind::InnerPayload.as_str(), "inner_payload");
    }

    #[test]
    fn a_record_name_sorts_oldest_first_lexicographically() {
        // Eviction relies on this: the scan sorts by filename and evicts from the front, so
        // the zero-padding is load-bearing rather than cosmetic.
        let early = record_name(1_000, "aaaa");
        let late = record_name(2_000, "bbbb");
        assert!(early < late);
        // ⚠ And it must survive a digit-count change, which is what zero-padding buys.
        let nine = record_name(9, "zzzz");
        let ten = record_name(10, "aaaa");
        assert!(nine < ten, "unpadded names would sort 10 before 9");
    }

    #[test]
    fn the_entry_id_does_not_leak_the_relay_item_id() {
        // Filenames are visible metadata even when contents are encrypted.
        let relay_id = "relay-item-0123456789abcdef";
        let entry = mint_entry_id(relay_id, 1_700_000_000);
        assert!(!entry.contains("relay-item"));
        assert!(!relay_id.contains(&entry));
        assert_eq!(entry.len(), 16, "8 bytes hex-encoded");
    }
}
