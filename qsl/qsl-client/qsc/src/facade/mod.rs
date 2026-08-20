//! NA-0751 (D-1393) — THE GATEWAY SPINE: the GUI-facing typed facade.
//!
//! ONE small module that answers the four Slice-4 screens' questions as TYPES instead of as
//! `CliResult` string rows (`ENG-0206`). It CALLS; it never EDITS: every fact here is read
//! through an existing entry point, and no module body outside this file is touched. The
//! facade is IN-CRATE, so `pub(crate)` / `pub(super)`-at-root internals are reachable with
//! ZERO visibility widening.
//!
//! ⚠ WHAT THIS SURFACE DELIBERATELY DOES NOT CARRY, so an absence reads as a decision:
//!   * `send_ready`, under that or any other name (`R334.3`; NA-0705 F6 measured a responder
//!     at `send_ready=no` sending successfully, so it reports the crypto chain's state, not
//!     the user's capability).
//!   * a conversation-readiness struct. `peer_confirmed` as `recv.nr != 0` CONTRADICTS the
//!     shipped `peer_confirmed`, whose real rule is `(!send_ready) || (recv.nr != 0)`
//!     (`handshake/mod.rs:1303-1311`), and an existing green test pins the divergent state
//!     (`tests/handshake_contract_na0217i.rs:283/288/293`). Withheld at `R368` §1, not missed.
//!   * the SELF fingerprint. The desktop's existing `identity_show`/`identity_ensure`
//!     (`IdentityDto`) already carries it; the verify screens compose peer + self.
//!   * a `RevokeOutcome`. `invite_revoke` returns ONE FLAT VALUE, its commit boundary is
//!     INSIDE it (`invite/mod.rs:919-920`), and two codes are minted byte-identically on both
//!     sides of it — so position cannot carry provenance. A screen distinguishes the three
//!     outcomes by calling [`invite_list`] after an error: `state == Revoked` means the local
//!     revoke committed and only the relay call failed.

use crate::contacts::{contact_request_list, contact_state, contacts_list_entries};
use crate::identity::{identity_read_pin, identity_voice_form};
use crate::invite;
use crate::model::ErrorCode;
use crate::output::CliError;
use crate::protocol_state::qsp_status_tuple;
use crate::store::ContactRecord;

// ─────────────────────────────────────────────────────────────────────────────────────────
// ERRORS
// ─────────────────────────────────────────────────────────────────────────────────────────

/// Every failure a Slice-4 screen must be able to tell apart, as a type.
///
/// ⚠⚠ THE PARTITION RULE, and it binds at the tree's OWN boundary: `invite/mod.rs:99`
/// (*"Local, decided before any socket is opened"*) versus `:110` (*"Relay-reported"*).
/// NO VARIANT MERGES ACROSS IT — the relay is the untrusted party, and which side is
/// speaking is itself information the screen owes the user.
///
/// ⚠ FIVE DUAL-PROVENANCE EXCEPTIONS, which the facade CANNOT separate because provenance is
/// not a property of the code. Named here rather than hidden:
///   * `invite_malformed`      local `invite/mod.rs` (25 sites) · relay `transport/mod.rs:4195`
///   * `invite_not_found`      local `:915` `:1225`            · relay `transport:4184` `:4199`
///   * `invite_create_failed`  local `:868`                    · relay `transport:4194` `:4205`
///   * `invite_revoke_invalid` local `:917`                    · relay `transport:4190` `:4329`
///   * `vault_locked`          — THE WORST OF THE FIVE, and the reason [`Self::Locked`] is
///     never inferred from a string: see [`Self::VaultUnavailable`].
///
/// The first four mean the same thing on both sides ("this invite is not valid"); the fifth
/// does not. Separating them upstream is `ENG-0213`.
#[derive(Debug)]
pub enum FacadeError {
    // ── FACADE-MEASURED ONLY. Never inferred from a string. ──────────────────────────────
    /// The vault is locked, measured by THIS module before the call
    /// (`crate::vault_unlocked`, `lib.rs:196`), or reported by `require_unlocked`
    /// (`lib.rs:213`) — the only `CliError::Emitted` producer reachable from the wrapped
    /// verbs. `file_xfer_reject` (`attachments/mod.rs:1671`) and `protocol_inactive_error`
    /// (`protocol_state/mod.rs:1015`) are the crate's other two producers and are NOT
    /// reachable here: `contacts/mod.rs` references neither module.
    Locked,
    /// The vault could not be READ, and the cause is genuinely indistinguishable from here.
    ///
    /// ⚠⚠ `"vault_locked"` AS A STRING DOES NOT MEAN "the vault is locked". An AEAD tag
    /// mismatch — a corrupt, truncated or tampered vault blob — is relabelled to that exact
    /// string at `vault/mod.rs:965-973`, and passphrase-source failures at `:927`, `:1419`,
    /// `:1449` do the same. Three provenances arrive as one word: locked mid-operation,
    /// vault DAMAGE, or key-source failure.
    ///
    /// This variant is the tree's own discipline, adopted rather than invented: EIGHT modules
    /// already refuse to propagate the string as a cause and translate it to a scoped
    /// unavailability instead (`contacts` `:384`…, `handshake` `:1209`…, `transport` `:151`,
    /// `timeline` `:443`, `attachments` `:34`, `msgqueue` `:317`, `quarantine` `:219`,
    /// `owed_receipts` `:86`). Screen copy must be true in all three cases — "the vault could
    /// not be read" — and must NOT say "unlock it".
    VaultUnavailable,

    // ── LOCAL: decided before any socket (`invite/mod.rs:99`) ────────────────────────────
    /// `invite_expired` `:104` — expired by the LOCAL clock, before any network attempt.
    Expired,
    /// `invite_already_redeemed` `:106` — client-side single-use; the arm that survives a
    /// hostile relay (I2). Deliberately distinct from [`Self::AlreadyUsed`].
    AlreadyRedeemed,
    /// `invite_revoked_locally` `:107`.
    RevokedLocally,
    /// `invite_soft_cap_reached` `:108` — this client refused to mint another live invite.
    SoftCapReached,
    /// `invite_malformed` `:100`, `invite_version_newer` `:101`, `invite_type_unknown` `:102`
    /// — LOCAL parse failures only.
    Malformed,

    // ── RELAY-REPORTED (`invite/mod.rs:110`) ─────────────────────────────────────────────
    /// `invite_not_found` `:111`, and `request_unknown` from the contact-request verbs.
    NotFound,
    /// `invite_revoked` `:112` — the relay says so.
    Revoked,
    /// `invite_expired_at_relay` `:117`. DELIBERATELY distinct from [`Self::Expired`]: the
    /// relay clamps expiry against ITS clock, so a local "alive" against a relay "expired" is
    /// a NORMAL outcome, and collapsing the two would blame the user's clock for a relay
    /// ceiling (`invite/mod.rs:113-116`).
    ExpiredAtRelay,
    /// `invite_already_used` `:118` — the RELAY's claim, not this client's.
    AlreadyUsed,
    /// `invite_rate_limited` `:121`.
    RateLimited,
    /// `invite_slot_cap_full` `:122` — the relay's slot table, not this client's soft cap.
    RelaySlotsFull,
    /// `invite_cap_invalid` `:119`, `invite_ticket_invalid` `:120`, `invite_too_large` `:123`,
    /// `invite_create_failed` `:124`.
    RelayRejected,
    /// `invite_revoke_invalid` `:125`. Its own arm for NAME honesty: the local producer at
    /// `invite/mod.rs:917` is "no stored revoke_token", and calling that a relay rejection
    /// names the untrusted party as the cause of a purely local condition.
    /// ⚠ Dual-provenance (see the type doc), so LOCAL STATE MAY ALREADY BE `Revoked` — call
    /// [`invite_list`] to find out.
    RevokeInvalid,
    /// `relay_unauthorized` — `transport/mod.rs:4196`, `:4198`.
    RelayUnauthorized,
    /// `invite_commitment_mismatch` `:131` — substituted KEYS. DISTINCT from
    /// [`Self::SignatureInvalid`] and from everything else, because the user needs to be told
    /// someone may be interfering (`invite/mod.rs:127-130`). Severity is accent, never red.
    CommitmentMismatch,
    /// `invite_signature_invalid` `:132` — tampered invite FIELDS. See
    /// [`Self::CommitmentMismatch`].
    SignatureInvalid,

    // ── RELAY-DELIVERED BYTES: the "different in kind" class ─────────────────────────────
    /// `handshake_envelope_malformed` `:135`, reached through `invite_accept`
    /// (`invite/mod.rs:1248`) and `invite_finish` (`:1355`) on relay-delivered bytes. This is
    /// mockup state S-F5: *bug or attack*, never a retry.
    EnvelopeMalformed,
    /// `handshake_envelope_version_newer` `:136`. See [`Self::EnvelopeMalformed`].
    EnvelopeVersionSkew,

    // ── RELAY TRANSPORT / CONFIG ─────────────────────────────────────────────────────────
    /// `relay_tls_untrusted` — `transport/mod.rs:2101`, produced at `:2230`. The D599
    /// "someone may be interfering" class; it gets its own arm for the same reason
    /// [`Self::CommitmentMismatch`] does.
    RelayTlsUntrusted,
    /// The CA-file trio — `relay_ca_file_missing` `:2103`, `_unreadable` `:2105`,
    /// `_invalid` `:2107`. LOCAL CA configuration; the specific code travels in `detail`.
    RelayCaFile,
    /// `relay_endpoint_invalid_host` / `_scheme` (`adversarial/route.rs:56`, `:67`) and
    /// `QSC_ERR_RELAY_TLS_REQUIRED` (`:5`). User-fixable endpoint configuration.
    RelayEndpointInvalid,

    // ── STORE / RESIDUAL ─────────────────────────────────────────────────────────────────
    /// `contacts_store_unavailable` — the contact store would not open (29 sites in
    /// `contacts/mod.rs`).
    StoreUnavailable,
    /// The crate's own typed store error, CARRIED rather than collapsed, so
    /// `ErrorCode::LockUpgradeRefused` stays diagnosable to the GUI — which is the whole
    /// reason `model/mod.rs:28-32` gives that variant a distinct name.
    ///
    /// ⚠ `ErrorCode` derives only `Debug, Clone, Copy` — NO `PartialEq` — so every assertion
    /// against this arm is written with `matches!`, never `assert_eq!`.
    Store(ErrorCode),
    /// The documented open-world residual, for codes no arm names. OWNED (`String`), because
    /// `CliError::Code` carries a `String` and turning one into a `&'static str` requires a
    /// leak. Its payload is shape-sealed to `^[a-z][a-z0-9_]*$` in test.
    Other(String),
}

impl FacadeError {
    /// The stable wire discriminant for one variant. EXHAUSTIVE and wildcard-free — deleting
    /// a variant makes this match non-exhaustive and the build goes RED here.
    ///
    /// ⚠ [`FacadeError::Store`] FANS OUT at the DTO boundary: its discriminant is the inner
    /// `ErrorCode::as_str()`, so the pinned set is 25 + 13 = 38, not 26. Collapsing `Store`
    /// to one code would put `lock_upgrade_refused` beyond a GUI's reach and undo the reason
    /// the variant exists.
    pub fn as_wire(&self) -> &'static str {
        match self {
            FacadeError::Locked => "locked",
            FacadeError::VaultUnavailable => "vault_unavailable",
            FacadeError::Expired => "expired",
            FacadeError::AlreadyRedeemed => "already_redeemed",
            FacadeError::RevokedLocally => "revoked_locally",
            FacadeError::SoftCapReached => "soft_cap_reached",
            FacadeError::Malformed => "malformed",
            FacadeError::NotFound => "not_found",
            FacadeError::Revoked => "revoked",
            FacadeError::ExpiredAtRelay => "expired_at_relay",
            FacadeError::AlreadyUsed => "already_used",
            FacadeError::RateLimited => "rate_limited",
            FacadeError::RelaySlotsFull => "relay_slots_full",
            FacadeError::RelayRejected => "relay_rejected",
            FacadeError::RevokeInvalid => "revoke_invalid",
            FacadeError::RelayUnauthorized => "relay_unauthorized",
            FacadeError::CommitmentMismatch => "commitment_mismatch",
            FacadeError::SignatureInvalid => "signature_invalid",
            FacadeError::EnvelopeMalformed => "envelope_malformed",
            FacadeError::EnvelopeVersionSkew => "envelope_version_skew",
            FacadeError::RelayTlsUntrusted => "relay_tls_untrusted",
            FacadeError::RelayCaFile => "relay_ca_file",
            FacadeError::RelayEndpointInvalid => "relay_endpoint_invalid",
            FacadeError::StoreUnavailable => "store_unavailable",
            FacadeError::Store(code) => code.as_str(),
            FacadeError::Other(_) => "other",
        }
    }
}

/// The vault-family strings that reach these paths through `invite_store_load`/`_save` ->
/// `vault::secret_get`/`secret_set`. `vault_secret_name_invalid` (`vault:231`/`:241`) is
/// deliberately ABSENT: it is guarded by a const key and unreachable from here.
const VAULT_FAMILY: &[&str] = &[
    "vault_locked",
    "vault_missing",
    "vault_parse_failed",
    "vault_config_missing",
    "vault_payload_serialize_failed",
    "encrypt_failed",
];

/// Map a `&'static str` error code from the wrapped functions onto the typed surface.
///
/// The invite consts are matched BY NAME, so a rename upstream breaks this build. The
/// one-ring-out codes are matched by value, enumerated from the read's delivery-path census.
fn map_code(code: &str) -> FacadeError {
    match code {
        // ── the taxonomy block, by const NAME (invite/mod.rs:99-136) ─────────────────────
        invite::INVITE_MALFORMED | invite::INVITE_VERSION_NEWER | invite::INVITE_TYPE_UNKNOWN => {
            FacadeError::Malformed
        }
        invite::INVITE_EXPIRED => FacadeError::Expired,
        invite::INVITE_ALREADY_REDEEMED => FacadeError::AlreadyRedeemed,
        invite::INVITE_REVOKED_LOCALLY => FacadeError::RevokedLocally,
        invite::INVITE_SOFT_CAP_REACHED => FacadeError::SoftCapReached,
        invite::INVITE_NOT_FOUND => FacadeError::NotFound,
        invite::INVITE_REVOKED => FacadeError::Revoked,
        invite::INVITE_EXPIRED_AT_RELAY => FacadeError::ExpiredAtRelay,
        invite::INVITE_ALREADY_USED => FacadeError::AlreadyUsed,
        invite::INVITE_RATE_LIMITED => FacadeError::RateLimited,
        invite::INVITE_SLOT_CAP_FULL => FacadeError::RelaySlotsFull,
        invite::INVITE_CAP_INVALID
        | invite::INVITE_TICKET_INVALID
        | invite::INVITE_TOO_LARGE
        | invite::INVITE_CREATE_FAILED => FacadeError::RelayRejected,
        invite::INVITE_REVOKE_INVALID => FacadeError::RevokeInvalid,
        invite::INVITE_COMMITMENT_MISMATCH => FacadeError::CommitmentMismatch,
        invite::INVITE_SIGNATURE_INVALID => FacadeError::SignatureInvalid,
        invite::HANDSHAKE_ENVELOPE_MALFORMED => FacadeError::EnvelopeMalformed,
        invite::HANDSHAKE_ENVELOPE_VERSION_NEWER => FacadeError::EnvelopeVersionSkew,

        // ── one ring out: transport + route ─────────────────────────────────────────────
        "relay_unauthorized" => FacadeError::RelayUnauthorized,
        "relay_tls_untrusted" => FacadeError::RelayTlsUntrusted,
        "relay_ca_file_missing" | "relay_ca_file_unreadable" | "relay_ca_file_invalid" => {
            FacadeError::RelayCaFile
        }
        "relay_endpoint_invalid_host"
        | "relay_endpoint_invalid_scheme"
        | "QSC_ERR_RELAY_TLS_REQUIRED" => FacadeError::RelayEndpointInvalid,

        // ── the contact verbs' own vocabulary ───────────────────────────────────────────
        "request_unknown" => FacadeError::NotFound,
        "contacts_store_unavailable" => FacadeError::StoreUnavailable,

        // ── the vault read, and the second half of the lock window ──────────────────────
        //
        // `identity_secret_unavailable` arrives here ONLY from the invite bridges at
        // `invite/mod.rs:847`/`:1027`, where it is a vault-read failure. The SAME string as an
        // `ErrorCode::as_str()` wire name from a typed verb is `Store(IdentitySecretUnavailable)`
        // — see `store_code_from_wire`. Same word, two provenances, told apart by the VERB.
        "identity_secret_unavailable" => FacadeError::VaultUnavailable,

        other => {
            if VAULT_FAMILY.contains(&other) {
                FacadeError::VaultUnavailable
            } else if let Some(code) = store_code_from_wire(other) {
                FacadeError::Store(code)
            } else {
                FacadeError::Other(other.to_string())
            }
        }
    }
}

/// The EXACT INVERSE of `ErrorCode::as_str` (`model/mod.rs:43-59`), so a store failure that
/// was flattened to a string by one of the crate's four bridges arrives as the SAME variant it
/// would have on a typed verb.
///
/// The four bridges: `invite/mod.rs:1222`/`:1321` (`e.as_str()`), `invite/mod.rs:847`/`:1027`,
/// `lib.rs:200-201` (`cli_err`), `vault/mod.rs:1034-1039` (`store_err_marker`).
/// ⚠ The last one RENAMES: `ErrorCode::IoWriteFailed` becomes `"vault_write_failed"`, which is
/// not any `ErrorCode` wire name, so it is mapped explicitly below.
fn store_code_from_wire(wire: &str) -> Option<ErrorCode> {
    Some(match wire {
        "missing_home" => ErrorCode::MissingHome,
        "invalid_policy_profile" => ErrorCode::InvalidPolicyProfile,
        "unsafe_path_symlink" => ErrorCode::UnsafePathSymlink,
        "unsafe_parent_perms" => ErrorCode::UnsafeParentPerms,
        "lock_open_failed" => ErrorCode::LockOpenFailed,
        "lock_contended" => ErrorCode::LockContended,
        "lock_failed" => ErrorCode::LockFailed,
        "lock_upgrade_refused" => ErrorCode::LockUpgradeRefused,
        "io_write_failed" => ErrorCode::IoWriteFailed,
        "io_read_failed" => ErrorCode::IoReadFailed,
        "parse_failed" => ErrorCode::ParseFailed,
        "identity_self_ambiguous" => ErrorCode::IdentitySelfAmbiguous,
        // `store_err_marker`'s own rename, documented at `vault/mod.rs:1034-1039`.
        "vault_write_failed" => ErrorCode::IoWriteFailed,
        _ => return None,
    })
}

impl From<ErrorCode> for FacadeError {
    fn from(code: ErrorCode) -> Self {
        FacadeError::Store(code)
    }
}

impl From<CliError> for FacadeError {
    /// `CliError::Emitted` reaching us AFTER the facade's own `vault_unlocked()` pre-check can
    /// only have come from `require_unlocked` (`lib.rs:213`) inside the check-to-call window,
    /// and that site carries exactly one meaning — so it is [`FacadeError::Locked`], measured
    /// rather than inferred.
    fn from(err: CliError) -> Self {
        match err {
            CliError::Emitted => FacadeError::Locked,
            CliError::Code(code) => map_code(&code),
        }
    }
}

/// The facade's own lock pre-measurement, run before EVERY verb.
///
/// ⚠ It is POINT-IN-TIME. The wrapped call happens after it, so a lock landing in between is
/// reported by whatever the wrapped path returns — for the invite verbs that is the
/// `"vault_locked"` literal, which maps to [`FacadeError::VaultUnavailable`], and for the
/// contact verbs `CliError::Emitted`, which maps to [`FacadeError::Locked`]. In the shipped
/// desktop the window is CLOSED by the `CoreGateway`'s single-flight gate, which is also why a
/// caller outside that gate reopens it.
fn require_unlocked_here() -> Result<(), FacadeError> {
    if crate::vault_unlocked() {
        Ok(())
    } else {
        Err(FacadeError::Locked)
    }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// CONNECTION STATUS
// ─────────────────────────────────────────────────────────────────────────────────────────

/// The banner's coarse state — the first element of `qsp_status_tuple`
/// (`protocol_state/mod.rs:79`). `Active` is produced by exactly one arm, `:91`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectState {
    Active,
    Inactive,
}

/// Why the connection is in that state.
///
/// SEVEN variants mirror `qsp_status_tuple`'s own vocabulary — eight match arms producing
/// seven distinct strings, `session_invalid` at both `:99` and `:100`. Two more are the
/// facade's own.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectReason {
    /// `"handshake"` `protocol_state/mod.rs:91` — the only [`ConnectState::Active`] arm.
    Handshake,
    /// `"no_session"` `:94`.
    NoSession,
    /// `"missing_seed"` `:96`.
    MissingSeed,
    /// `"session_invalid"` `:99` AND `:100`.
    SessionInvalid,
    /// `"channel_invalid"` `:88`.
    ChannelInvalid,
    /// `"unsafe_parent"` `:85`.
    UnsafeParent,
    /// `"missing_home"` `:82`.
    MissingHome,
    /// FACADE-MEASURED, never produced upstream: the vault is locked AND the tuple says
    /// `session_invalid`. While locked, the session blob's validity is UNKNOWABLE — the key
    /// load fails and `qsp_session_decrypt_blob` short-circuits at `protocol_state:870-874`
    /// BEFORE any structural or integrity check — so "locked" is the operative fact and
    /// "invalid" would be a guess presented as a finding.
    VaultLocked,
    /// The honest runtime surface for an EIGHTH upstream reason string. Any appearance is a
    /// DEFECT TO FILE, and a test asserts it is unreached across the seven driven states.
    /// ⚠ An eighth upstream string cannot red the COMPILER — the upstream reasons are
    /// `String` literals returned from match arms, not an enum — so the suite reds instead,
    /// via the totality test. A test is not a compiler; this line says so.
    Unrecognized,
}

impl ConnectReason {
    /// EXHAUSTIVE, wildcard-free. **W1's delta symbol**: delete any variant and the build goes
    /// RED here.
    pub fn as_wire(self) -> &'static str {
        match self {
            ConnectReason::Handshake => "handshake",
            ConnectReason::NoSession => "no_session",
            ConnectReason::MissingSeed => "missing_seed",
            ConnectReason::SessionInvalid => "session_invalid",
            ConnectReason::ChannelInvalid => "channel_invalid",
            ConnectReason::UnsafeParent => "unsafe_parent",
            ConnectReason::MissingHome => "missing_home",
            ConnectReason::VaultLocked => "vault_locked",
            ConnectReason::Unrecognized => "unrecognized",
        }
    }

    fn from_wire(reason: &str) -> ConnectReason {
        match reason {
            "handshake" => ConnectReason::Handshake,
            "no_session" => ConnectReason::NoSession,
            "missing_seed" => ConnectReason::MissingSeed,
            "session_invalid" => ConnectReason::SessionInvalid,
            "channel_invalid" => ConnectReason::ChannelInvalid,
            "unsafe_parent" => ConnectReason::UnsafeParent,
            "missing_home" => ConnectReason::MissingHome,
            _ => ConnectReason::Unrecognized,
        }
    }
}

/// What the connection banner renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectStatus {
    pub state: ConnectState,
    pub reason: ConnectReason,
}

/// Wraps `qsp_status_tuple` (`protocol_state/mod.rs:79`).
///
/// ⚠ ORDER IS LOAD-BEARING, and it is NOT "check the lock first". `missing_home`,
/// `unsafe_parent`, `channel_invalid`, `no_session` and `missing_seed` are all decided before
/// any vault secret is touched, so unlocking cures none of them and reporting `VaultLocked`
/// would SHADOW the operative fact. The tuple runs first; the override applies to exactly ONE
/// arm — locked AND `session_invalid`.
pub fn connect_status(peer: &str) -> ConnectStatus {
    let (state, reason) = qsp_status_tuple(peer);
    let reason = ConnectReason::from_wire(&reason);
    let reason = if reason == ConnectReason::SessionInvalid && !crate::vault_unlocked() {
        ConnectReason::VaultLocked
    } else {
        reason
    };
    let state = if state == "ACTIVE" {
        ConnectState::Active
    } else {
        ConnectState::Inactive
    };
    ConnectStatus { state, reason }
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// CONTACTS
// ─────────────────────────────────────────────────────────────────────────────────────────

/// The two ratified fingerprint tiers of ONE identity, together, or not at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FingerprintPair {
    /// 64-hex, `qsl-fp-v1`.
    pub full: String,
    /// The 30-digit read-aloud form — `identity_voice_form` (`identity/mod.rs:679`), whose own
    /// doc warns that its `""` sentinel "is only safe if the CALLER refuses it". The 64-hex
    /// guard on [`ContactSummary::fingerprint`] is that refusal, and it also refuses the
    /// `"untrusted"` sentinel `identity_peer_status` would have produced.
    pub voice: String,
}

/// The trust state the CLI renders and the badge means — `contact_state`
/// (`contacts/mod.rs:498`), which reads the PRIMARY DEVICE's canonical state and ignores the
/// legacy `ContactRecord.status` field entirely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactState {
    /// `"PINNED"` — the primary device is `TRUSTED`.
    Pinned,
    /// `"VERIFIED"`.
    Verified,
    /// `"CHANGED"` — the peer-key-changed signal, the MITM tell. Upstream maps BOTH `CHANGED`
    /// and `REVOKED` here.
    Changed,
    /// `"UNVERIFIED"`, including the no-record case.
    Unverified,
}

impl ContactState {
    fn from_wire(state: &str) -> ContactState {
        match state {
            "PINNED" => ContactState::Pinned,
            "VERIFIED" => ContactState::Verified,
            "CHANGED" => ContactState::Changed,
            _ => ContactState::Unverified,
        }
    }
}

/// One row of the contact list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactSummary {
    pub alias: String,
    /// `Some` ONLY when the resolved fingerprint is exactly 64 ASCII-hex. `UNSET` — written
    /// into `ContactRecord.fp` by `contacts_request_accept` (`contacts/mod.rs:1572`) and
    /// `contacts_request_block` (`:1630`), both of which THIS facade exposes — yields `None`,
    /// typed absence, never `""` and never the literal word.
    pub fingerprint: Option<FingerprintPair>,
    /// Whether this contact is pinned.
    ///
    /// ⛳ `fingerprint` and `pinned` come from ONE resolution — `identity_read_pin`
    /// (`identity/mod.rs:744-752`), the same expression the pin comparison itself consumes:
    /// primary-device fp else record fp, with empty/`UNSET` refused. So a trust screen can
    /// never show a fingerprint that is not the one that was pinned.
    pub pinned: bool,
    pub blocked: bool,
    pub state: ContactState,
}

fn hex64(fp: &str) -> bool {
    fp.len() == 64 && fp.bytes().all(|b| b.is_ascii_hexdigit())
}

fn summarize_contact(alias: String, rec: &ContactRecord) -> ContactSummary {
    // ONE resolution, feeding BOTH fields. `identity_peer_status` (`lib.rs:242`) is exactly
    // `identity_read_pin` plus an `"untrusted"` placeholder; calling the inner function keeps
    // the placeholder off this surface entirely.
    let resolved = identity_read_pin(&alias).ok().flatten();
    let pinned = resolved.is_some();
    let fingerprint = resolved.as_deref().filter(|fp| hex64(fp)).map(|fp| FingerprintPair {
        full: fp.to_string(),
        voice: identity_voice_form(fp),
    });
    ContactSummary {
        alias,
        fingerprint,
        pinned,
        blocked: rec.blocked,
        state: ContactState::from_wire(contact_state(Some(rec))),
    }
}

/// Wraps `contacts_list_entries` (`contacts/mod.rs:493`) — the TYPED source.
///
/// ⚠ NOT `contacts_list` (`:1440`), which returns `CliResult` = `Result<(), CliError>`: it
/// emits its rows as printed lines and returns `()`, so wrapping it would yield a unit and
/// force the caller to scrape stdout — the exact defect `ENG-0206` exists to remove.
pub fn contact_list() -> Result<Vec<ContactSummary>, FacadeError> {
    require_unlocked_here()?;
    let entries = contacts_list_entries()?;
    Ok(entries
        .into_iter()
        .map(|(alias, rec)| summarize_contact(alias, &rec))
        .collect())
}

/// The only state an inbound contact request can be in. `ContactRequestRecord` has exactly ONE
/// constructor in the crate (`contacts/mod.rs:439-444`), which writes `"PENDING"` at `:442`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactRequestState {
    Pending,
}

impl ContactRequestState {
    /// EXHAUSTIVE, wildcard-free — its own delta symbol.
    pub fn as_wire(self) -> &'static str {
        match self {
            ContactRequestState::Pending => "PENDING",
        }
    }
}

/// One inbound contact request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContactRequestSummary {
    pub alias: String,
    pub state: ContactRequestState,
    pub device_id: Option<String>,
    pub seen_at: Option<u64>,
}

/// Wraps `contact_request_list` (`contacts/mod.rs:462`) — the TYPED source, not the
/// `CliResult` emitter at `:1545`. Named `contact_requests` so the wrapper does not name
/// itself.
pub fn contact_requests() -> Result<Vec<ContactRequestSummary>, FacadeError> {
    require_unlocked_here()?;
    let items = contact_request_list()?;
    Ok(items
        .into_iter()
        .map(|r| ContactRequestSummary {
            alias: r.alias,
            state: ContactRequestState::Pending,
            device_id: r.device_id,
            seen_at: r.seen_at,
        })
        .collect())
}

/// Wraps `contacts_request_accept` (`contacts/mod.rs:1565`). Unlike the LIST verbs, the ACTION
/// verbs are correctly wrapped at the `pub` layer: their result genuinely IS unit-or-error.
pub fn contact_request_accept(alias: &str) -> Result<(), FacadeError> {
    require_unlocked_here()?;
    Ok(crate::contacts::contacts_request_accept(alias)?)
}

/// Wraps `contacts_request_ignore` (`contacts/mod.rs:1613`).
pub fn contact_request_ignore(alias: &str) -> Result<(), FacadeError> {
    require_unlocked_here()?;
    Ok(crate::contacts::contacts_request_ignore(alias)?)
}

/// Wraps `contacts_request_block` (`contacts/mod.rs:1624`).
pub fn contact_request_block(alias: &str) -> Result<(), FacadeError> {
    require_unlocked_here()?;
    Ok(crate::contacts::contacts_request_block(alias)?)
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// INVITES
// ─────────────────────────────────────────────────────────────────────────────────────────

/// The lifecycle state of one invite. Wraps `InviteState` (`invite/mod.rs:588`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InviteStateKind {
    Creating,
    Active,
    Redeemed,
    Expired,
    Revoked,
}

impl InviteStateKind {
    /// EXHAUSTIVE and wildcard-free over the upstream enum — a SIXTH `InviteState` variant is
    /// a COMPILE red here. That is this type's delta symbol.
    fn from_upstream(state: &invite::InviteState) -> InviteStateKind {
        match state {
            invite::InviteState::Creating => InviteStateKind::Creating,
            invite::InviteState::Active => InviteStateKind::Active,
            invite::InviteState::Redeemed => InviteStateKind::Redeemed,
            invite::InviteState::Expired => InviteStateKind::Expired,
            invite::InviteState::Revoked => InviteStateKind::Revoked,
        }
    }

    pub fn as_wire(self) -> &'static str {
        match self {
            InviteStateKind::Creating => "creating",
            InviteStateKind::Active => "active",
            InviteStateKind::Redeemed => "redeemed",
            InviteStateKind::Expired => "expired",
            InviteStateKind::Revoked => "revoked",
        }
    }
}

/// One invite, as a screen needs it.
///
/// ⚠ NO `cap` AND NO `relay_ep`. `cap` is a live BEARER CREDENTIAL — holding `invite_id` +
/// `cap` is what redeeming a slot requires — and the desktop already refuses to hand its front
/// end even a HASH of the relay token (`src-tauri/src/commands.rs:500`, `:600`, FLAG-3). The
/// CLI's own `invite list` renders neither field. `invite_create` still returns the full code
/// ONCE at mint: that is the shareable artefact and it is meant to leave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InviteSummary {
    pub invite_id: String,
    pub state: InviteStateKind,
    pub expiry: u64,
    /// `revoke_token.is_some()` — presence, never the one-shot secret itself.
    pub revocable: bool,
}

/// Wraps `invite_list` (`invite/mod.rs:924`) at the module's own clock, `invite::now_unix_s`
/// (`:149`), which delegates to the ONE clock, `crate::clock`.
pub fn invite_list() -> Result<Vec<InviteSummary>, FacadeError> {
    invite_list_at(invite::now_unix_s())
}

/// The `_at` seam, per `invite/mod.rs:142-144`: "every decision that depends on it takes the
/// value as a parameter through an `_at` entry point, so tests force the outcome
/// deterministically instead of sleeping."
///
/// The HONEST name: it returns ALL invites, not only pending ones — that is what the wrapped
/// function returns, and a screen filters. State is COMPUTED, because `InviteState::Expired`
/// has ZERO constructors in the crate and expiry is evaluated only at read time. The overlay
/// is the exact complement of the soft cap's own live-door test at `invite/mod.rs:829`
/// (`state == Active && expiry > now`): here, `Active && expiry <= now` becomes `Expired`.
pub fn invite_list_at(now: u64) -> Result<Vec<InviteSummary>, FacadeError> {
    require_unlocked_here()?;
    let records = invite::invite_list().map_err(map_code)?;
    Ok(records
        .into_iter()
        .map(|r| {
            let kind = InviteStateKind::from_upstream(&r.state);
            let kind = if kind == InviteStateKind::Active && r.expiry <= now {
                InviteStateKind::Expired
            } else {
                kind
            };
            InviteSummary {
                invite_id: r.invite_id,
                state: kind,
                expiry: r.expiry,
                revocable: r.revoke_token.is_some(),
            }
        })
        .collect())
}

/// Wraps `invite_create` (`invite/mod.rs:800`). Returns the full invite code, ONCE, at mint —
/// the existing deliberate emission the CLI already prints (`main.rs:339-343`).
pub fn invite_create(
    self_label: Option<&str>,
    relay: &str,
    ttl_secs: u64,
) -> Result<String, FacadeError> {
    require_unlocked_here()?;
    invite::invite_create(self_label, relay, ttl_secs).map_err(map_code)
}

/// Wraps `invite_redeem` (`invite/mod.rs:932`).
pub fn invite_redeem(
    code: &str,
    alias: &str,
    self_label: Option<&str>,
) -> Result<String, FacadeError> {
    require_unlocked_here()?;
    invite::invite_redeem(code, alias, self_label).map_err(map_code)
}

/// Wraps `invite_accept` (`invite/mod.rs:1195`).
pub fn invite_accept(
    self_label: Option<&str>,
    invite_id: &str,
    alias: &str,
    max: usize,
) -> Result<Option<String>, FacadeError> {
    require_unlocked_here()?;
    invite::invite_accept(self_label, invite_id, alias, max).map_err(map_code)
}

/// Wraps `invite_finish` (`invite/mod.rs:1310`).
pub fn invite_finish(
    self_label: Option<&str>,
    alias: &str,
    relay: &str,
    max: usize,
) -> Result<bool, FacadeError> {
    require_unlocked_here()?;
    invite::invite_finish(self_label, alias, relay, max).map_err(map_code)
}

/// Wraps `invite_revoke` (`invite/mod.rs:907`).
///
/// ⚠ ON ERROR, THE LOCAL REVOKE MAY ALREADY HAVE COMMITTED. The wrapped function commits
/// locally at `:919-920` and only then calls the relay at `:921`, and two of its codes are
/// minted byte-identically on both sides of that boundary — so this return type CANNOT say
/// which happened, and no return type available to this module could.
/// A screen tells the three outcomes apart by calling [`invite_list`] after an error:
/// `state == Revoked` means "revoked here, relay not told"; anything else means "nothing
/// happened". Two calls the surface already carries, serialized by the desktop gateway's
/// single flight.
pub fn invite_revoke(invite_id: &str) -> Result<(), FacadeError> {
    require_unlocked_here()?;
    invite::invite_revoke(invite_id).map_err(map_code)
}

// ─────────────────────────────────────────────────────────────────────────────────────────
// W7 — THE ERROR-MAPPING SEALS
//
// These live in-src because `map_code` and `store_code_from_wire` are PRIVATE. The honest
// instrument for a private function is a same-file `#[cfg(test)]` module (the tree's own
// `confirm_capture_reason_tests` precedent); reaching them from `tests/` would have meant
// widening them to `pub`, i.e. adding public surface the sealed type surface does not carry.
// An in-src module also adds +0 test binaries, so the shard-manifest arithmetic stays +4.
// ─────────────────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod na0751_facade_mapping_tests {
    use super::*;

    /// The taxonomy's own bytes — the FOUND side of the seal.
    const INVITE_SRC: &str = include_str!("../invite/mod.rs");

    /// Scrape `invite/mod.rs` for error-code consts by VALUE SHAPE.
    ///
    /// ⚠ NOT by name: `^pub const [A-Z_]+: &str` returns TWENTY-FIVE, over-capturing
    /// `DS_COMMIT`, `DS_SIG` and `INVITE_CODE_PREFIX`, none of which is an error code. And a
    /// LINE RANGE would let a future error const added outside it escape the seal entirely.
    /// Pure lowercase snake_case separates them with no range to drift.
    fn scrape_error_code_values(src: &str) -> Vec<String> {
        let mut out = Vec::new();
        for line in src.lines() {
            let Some(rest) = line.trim_end().strip_prefix("pub const ") else {
                continue;
            };
            let Some((_name, tail)) = rest.split_once(": &str = ") else {
                continue;
            };
            let Some(value) = tail.strip_suffix(';') else {
                continue;
            };
            let value = value.trim_matches('"');
            if is_code_shaped(value) {
                out.push(value.to_string());
            }
        }
        out.sort();
        out.dedup();
        out
    }

    fn is_code_shaped(s: &str) -> bool {
        !s.is_empty()
            && s.starts_with(|c: char| c.is_ascii_lowercase())
            && s.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }

    /// The DECLARED side, taken from the tree's own consts so a rename upstream breaks this
    /// test's compile as well as the mapping's.
    fn declared_codes() -> Vec<String> {
        let mut v: Vec<String> = [
            invite::INVITE_MALFORMED,
            invite::INVITE_VERSION_NEWER,
            invite::INVITE_TYPE_UNKNOWN,
            invite::INVITE_EXPIRED,
            invite::INVITE_ALREADY_REDEEMED,
            invite::INVITE_REVOKED_LOCALLY,
            invite::INVITE_SOFT_CAP_REACHED,
            invite::INVITE_NOT_FOUND,
            invite::INVITE_REVOKED,
            invite::INVITE_EXPIRED_AT_RELAY,
            invite::INVITE_ALREADY_USED,
            invite::INVITE_CAP_INVALID,
            invite::INVITE_TICKET_INVALID,
            invite::INVITE_RATE_LIMITED,
            invite::INVITE_SLOT_CAP_FULL,
            invite::INVITE_TOO_LARGE,
            invite::INVITE_CREATE_FAILED,
            invite::INVITE_REVOKE_INVALID,
            invite::INVITE_COMMITMENT_MISMATCH,
            invite::INVITE_SIGNATURE_INVALID,
            invite::HANDSHAKE_ENVELOPE_MALFORMED,
            invite::HANDSHAKE_ENVELOPE_VERSION_NEWER,
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        v.sort();
        v.dedup();
        v
    }

    #[test]
    fn na0751_w7b_mapping_covers_every_error_const_the_taxonomy_declares() {
        let found = scrape_error_code_values(INVITE_SRC);
        let declared = declared_codes();
        // NON-VACUOUS: an empty or truncated scrape cannot reach the equality below.
        assert_eq!(found.len(), 22, "taxonomy declares 22 error consts; found {found:?}");
        assert_eq!(declared.len(), 22, "the mapping declares 22");
        let uncovered: Vec<&String> = found.iter().filter(|f| !declared.contains(f)).collect();
        assert!(uncovered.is_empty(), "declared upstream but not mapped: {uncovered:?}");
        assert_eq!(found, declared, "found side and declared side agree exactly");
    }

    #[test]
    fn na0751_w7b_the_scraper_discriminates_in_both_directions() {
        // FOUND-SIDE POSITIVE CONTROL: an upstream addition IS seen, so the seal above reds
        // rather than passing silently. This is the control v3's name-needle lacked.
        let plus = format!("{INVITE_SRC}\npub const SYNTHETIC_CONTROL: &str = \"synthetic_code\";\n");
        let found_plus = scrape_error_code_values(&plus);
        assert_eq!(found_plus.len(), 23, "a new lowercase const is FOUND");
        assert!(found_plus.iter().any(|v| v == "synthetic_code"));

        // NEGATIVE CONTROL: the three non-code consts are excluded BY THEIR OWN VALUES.
        let found = scrape_error_code_values(INVITE_SRC);
        for excluded in [
            "QSL.invite.identity-commitment.v1",
            "QSL.invite.payload.v1",
            "QSLI-1-",
        ] {
            assert!(!found.iter().any(|v| v == excluded), "{excluded} must not be scraped");
        }
        // And the NAME needle over-captures by exactly three — the reason it was abandoned.
        let by_name = INVITE_SRC
            .lines()
            .filter(|l| l.starts_with("pub const ") && l.contains(": &str = "))
            .count();
        assert_eq!(by_name, 25, "the name needle over-captures by exactly three");
    }

    #[test]
    fn na0751_w7a_no_known_invite_const_lands_in_other_or_store() {
        for code in declared_codes() {
            let mapped = map_code(&code);
            assert!(
                !matches!(mapped, FacadeError::Other(_)),
                "{code} fell through to Other"
            );
            assert!(
                !matches!(mapped, FacadeError::Store(_)),
                "{code} is an invite code, not a store code"
            );
        }
    }

    #[test]
    fn na0751_w7a_the_partition_is_respected_by_every_arm() {
        // No VARIANT merges across `invite/mod.rs:99` (local) / `:110` (relay-reported).
        let local = [
            (invite::INVITE_MALFORMED, "malformed"),
            (invite::INVITE_VERSION_NEWER, "malformed"),
            (invite::INVITE_TYPE_UNKNOWN, "malformed"),
            (invite::INVITE_EXPIRED, "expired"),
            (invite::INVITE_ALREADY_REDEEMED, "already_redeemed"),
            (invite::INVITE_REVOKED_LOCALLY, "revoked_locally"),
            (invite::INVITE_SOFT_CAP_REACHED, "soft_cap_reached"),
        ];
        let relay = [
            (invite::INVITE_NOT_FOUND, "not_found"),
            (invite::INVITE_REVOKED, "revoked"),
            (invite::INVITE_EXPIRED_AT_RELAY, "expired_at_relay"),
            (invite::INVITE_ALREADY_USED, "already_used"),
            (invite::INVITE_CAP_INVALID, "relay_rejected"),
            (invite::INVITE_TICKET_INVALID, "relay_rejected"),
            (invite::INVITE_RATE_LIMITED, "rate_limited"),
            (invite::INVITE_SLOT_CAP_FULL, "relay_slots_full"),
            (invite::INVITE_TOO_LARGE, "relay_rejected"),
            (invite::INVITE_CREATE_FAILED, "relay_rejected"),
            (invite::INVITE_REVOKE_INVALID, "revoke_invalid"),
            (invite::INVITE_COMMITMENT_MISMATCH, "commitment_mismatch"),
            (invite::INVITE_SIGNATURE_INVALID, "signature_invalid"),
        ];
        let mut local_wires = Vec::new();
        for (code, wire) in local {
            let m = map_code(code);
            assert_eq!(m.as_wire(), wire, "{code}");
            local_wires.push(m.as_wire());
        }
        let mut relay_wires = Vec::new();
        for (code, wire) in relay {
            let m = map_code(code);
            assert_eq!(m.as_wire(), wire, "{code}");
            relay_wires.push(m.as_wire());
        }
        // The two sides share NO variant.
        for l in &local_wires {
            assert!(
                !relay_wires.contains(l),
                "{l} carries both a LOCAL and a RELAY-reported code"
            );
        }
        // The distinctness pairs the consts' own docs demand.
        assert_ne!(map_code(invite::INVITE_EXPIRED).as_wire(), map_code(invite::INVITE_EXPIRED_AT_RELAY).as_wire());
        assert_ne!(map_code(invite::INVITE_ALREADY_REDEEMED).as_wire(), map_code(invite::INVITE_ALREADY_USED).as_wire());
        assert_ne!(map_code(invite::INVITE_COMMITMENT_MISMATCH).as_wire(), map_code(invite::INVITE_SIGNATURE_INVALID).as_wire());
        assert_ne!(map_code(invite::INVITE_REVOKED_LOCALLY).as_wire(), map_code(invite::INVITE_REVOKED).as_wire());
    }

    #[test]
    fn na0751_w7c_other_carries_only_code_shaped_payloads() {
        // The door this closes: `Other(format!("{e:?}"))` would satisfy every other seal
        // while leaking a rendered value into the error channel.
        match map_code("some_unmapped_code") {
            FacadeError::Other(p) => assert!(is_code_shaped(&p), "payload must be code-shaped"),
            other => panic!("expected Other, got {other:?}"),
        }
        assert!(!is_code_shaped("Io(Custom { kind: NotFound })"), "a Debug rendering is refused");
        assert!(!is_code_shaped("/home/victor/.config/qsc"), "a path is refused");
        assert!(!is_code_shaped("QSC_ERR_RELAY_TLS_REQUIRED"), "an uppercase code is refused");
    }

    #[test]
    fn na0751_w7_string_bridges_arrive_as_the_typed_verbs_variant() {
        // M3's cure: the same fact must arrive as the same variant on every path.
        assert!(matches!(
            map_code(ErrorCode::LockUpgradeRefused.as_str()),
            FacadeError::Store(ErrorCode::LockUpgradeRefused)
        ));
        assert!(matches!(
            map_code(ErrorCode::IoWriteFailed.as_str()),
            FacadeError::Store(ErrorCode::IoWriteFailed)
        ));
        // `store_err_marker`'s own RENAME (`vault/mod.rs:1034-1039`).
        assert!(matches!(
            map_code("vault_write_failed"),
            FacadeError::Store(ErrorCode::IoWriteFailed)
        ));
    }

    #[test]
    fn na0751_b2_the_vault_family_is_never_locked() {
        // `"vault_locked"` as a STRING does not carry the claim `Locked` makes: an AEAD tag
        // mismatch is relabelled to it at `vault/mod.rs:965-973`.
        for v in [
            "vault_locked",
            "vault_missing",
            "vault_parse_failed",
            "vault_config_missing",
            "vault_payload_serialize_failed",
            "encrypt_failed",
            "identity_secret_unavailable",
        ] {
            assert!(
                matches!(map_code(v), FacadeError::VaultUnavailable),
                "{v} must be VaultUnavailable, never Locked"
            );
        }
        // `Locked` is produced ONLY by the facade's own measurements.
        assert!(matches!(FacadeError::from(CliError::Emitted), FacadeError::Locked));
    }

    #[test]
    fn na0751_the_one_ring_out_codes_have_named_arms() {
        assert_eq!(map_code("relay_unauthorized").as_wire(), "relay_unauthorized");
        assert_eq!(map_code("relay_tls_untrusted").as_wire(), "relay_tls_untrusted");
        for c in ["relay_ca_file_missing", "relay_ca_file_unreadable", "relay_ca_file_invalid"] {
            assert_eq!(map_code(c).as_wire(), "relay_ca_file", "{c}");
        }
        for c in [
            "relay_endpoint_invalid_host",
            "relay_endpoint_invalid_scheme",
            "QSC_ERR_RELAY_TLS_REQUIRED",
        ] {
            assert_eq!(map_code(c).as_wire(), "relay_endpoint_invalid", "{c}");
        }
        assert_eq!(map_code("request_unknown").as_wire(), "not_found");
        assert_eq!(map_code("contacts_store_unavailable").as_wire(), "store_unavailable");
    }

    #[test]
    fn na0751_as_wire_discriminants_are_distinct_and_store_fans_out() {
        // W4's pinned set is 25 + 13 = 38, not 26: `Store` fans out over `ErrorCode::as_str`.
        let singles = [
            FacadeError::Locked, FacadeError::VaultUnavailable, FacadeError::Expired,
            FacadeError::AlreadyRedeemed, FacadeError::RevokedLocally, FacadeError::SoftCapReached,
            FacadeError::Malformed, FacadeError::NotFound, FacadeError::Revoked,
            FacadeError::ExpiredAtRelay, FacadeError::AlreadyUsed, FacadeError::RateLimited,
            FacadeError::RelaySlotsFull, FacadeError::RelayRejected, FacadeError::RevokeInvalid,
            FacadeError::RelayUnauthorized, FacadeError::CommitmentMismatch,
            FacadeError::SignatureInvalid, FacadeError::EnvelopeMalformed,
            FacadeError::EnvelopeVersionSkew, FacadeError::RelayTlsUntrusted,
            FacadeError::RelayCaFile, FacadeError::RelayEndpointInvalid,
            FacadeError::StoreUnavailable, FacadeError::Other(String::new()),
        ];
        assert_eq!(singles.len(), 25, "25 non-Store variants");
        let mut wires: Vec<&str> = singles.iter().map(|e| e.as_wire()).collect();
        let store_codes = [
            ErrorCode::MissingHome, ErrorCode::InvalidPolicyProfile, ErrorCode::UnsafePathSymlink,
            ErrorCode::UnsafeParentPerms, ErrorCode::LockOpenFailed, ErrorCode::LockContended,
            ErrorCode::LockFailed, ErrorCode::LockUpgradeRefused, ErrorCode::IoWriteFailed,
            ErrorCode::IoReadFailed, ErrorCode::ParseFailed, ErrorCode::IdentitySecretUnavailable,
            ErrorCode::IdentitySelfAmbiguous,
        ];
        assert_eq!(store_codes.len(), 13);
        for c in store_codes {
            wires.push(FacadeError::Store(c).as_wire());
        }
        assert_eq!(wires.len(), 38, "the pinned discriminant set is 38");
        let mut sorted = wires.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(sorted.len(), 38, "all 38 discriminants are DISTINCT");
        // The reason `Store` exists: `lock_upgrade_refused` survives to the boundary.
        assert!(wires.contains(&"lock_upgrade_refused"));
    }
}
