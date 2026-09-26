use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub(crate) use crate::adversarial::payload::{
    AttachmentConfirmPayload, AttachmentDescriptorPayload, FileTransferChunkPayload,
    FileTransferManifestPayload,
};

#[derive(Serialize, Deserialize)]
pub(crate) struct OutboxRecord {
    pub(crate) version: u8,
    pub(crate) payload_len: usize,
    #[serde(default)]
    pub(crate) to: String,
    #[serde(default)]
    pub(crate) channel: Option<String>,
    #[serde(default)]
    pub(crate) ciphertext: Vec<u8>,
    #[serde(default)]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) message_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct QspStatusRecord {
    pub(crate) active: bool,
    pub(crate) reason: String,
    pub(crate) last_pack_ok: bool,
    pub(crate) last_unpack_ok: bool,
}

pub(crate) const QSP_SESSIONS_DIR: &str = "qsp_sessions";
pub(crate) const QSP_SESSION_LEGACY_TOMBSTONE: &[u8] = b"QSC_SESSION_MIGRATED_V1\n";
pub(crate) const QSP_SESSION_BLOB_MAGIC: &[u8; 6] = b"QSSV01";
pub(crate) const QSP_SESSION_BLOB_VERSION: u8 = 1;
pub(crate) const QSP_SESSION_STORE_KEY_SECRET: &str = "qsp_session_store_key_v1";
pub(crate) const CONTACTS_SECRET_KEY: &str = "contacts.json";
pub(crate) const TIMELINE_SECRET_KEY: &str = "timeline.json";
pub(crate) const TUI_RECEIPT_MODE_SECRET_KEY: &str = "tui.receipt.mode";
pub(crate) const TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY: &str = "tui.receipt.batch_window_ms";
pub(crate) const TUI_RECEIPT_JITTER_MS_SECRET_KEY: &str = "tui.receipt.jitter_ms";
pub(crate) const TUI_FILE_CONFIRM_MODE_SECRET_KEY: &str = "tui.file_confirm.mode";
pub(crate) const TUI_TRUST_MODE_SECRET_KEY: &str = "tui.trust.mode";
// NA-0688 C4 (D622 R7) / NA-0770 (D-1411): the per-install acknowledged-pull preference never
// lived here, and as of NA-0770 it no longer exists anywhere — the mode is retired. It was first
// added as `tui.ack.mode` beside these keys, and that was wrong twice over: the value is not a
// secret, and a vault-backed preference silently stops applying whenever the vault is locked. Its
// config key (`ACK_MODE_KEY`) survives only as a TOMBSTONE that refuses writes and announces
// reads. ⚠ Note for whoever adds the next
// preference: four keys in this namespace -- tui.receipt.mode, tui.receipt.batch_window_ms,
// tui.receipt.jitter_ms, tui.file_confirm.mode -- are READ but written by NOTHING, and the `tui.`
// prefix names a subsystem retired and stripped in NA-0645. Do not extend that pattern.
pub const TUI_RELAY_TOKEN_SECRET_KEY: &str = "tui.relay.token";
pub const TUI_RELAY_TOKEN_FILE_SECRET_KEY: &str = "tui.relay.token_file";
pub const TUI_RELAY_INBOX_TOKEN_SECRET_KEY: &str = "tui.relay.inbox_token";
// NA-0663 (D599, D-1286): the explicit CA-file path for relay TLS trust. The existing
// tui.relay namespace, no new one invented. The VALUE is a filesystem path, not a
// secret -- but it is stored here because the vault secret store is the house settings
// surface, and it is redacted in markers exactly like the token file's path.
pub const TUI_RELAY_CA_FILE_SECRET_KEY: &str = "tui.relay.ca_file";

// NA-0681 (D616 §2h): invite state, in the vault, as a JSON blob under one key -- exactly
// the shape contacts already use (`contacts_store_load`/`_save`). Two keys, because the two
// sides have different lifetimes: what Alice CREATED, and what Bob REDEEMED.
pub const INVITES_SECRET_KEY: &str = "invite.created";
pub const REDEMPTIONS_SECRET_KEY: &str = "invite.redeemed";
pub(crate) const OUTBOX_NEXT_STATE_SECRET_KEY: &str = "outbox.next_state.v1";
pub(crate) const CONTACT_REQUESTS_SECRET_KEY: &str = "contact_requests.json";
pub(crate) const ATTACHMENT_JOURNAL_SECRET_KEY: &str = "attachments.json";
// NA-0658 (D594, D-1281): the ENG-0044 vault-protection consts restored to where the
// originals lived (deleted with the TUI at NA-0645/86c0858d). The bounds and the wipe
// marker are pub — the GUI reads the bounds and compares the marker value; the state
// file names stay crate-internal. TUI_AUTOLOCK_SECRET_KEY is NOT restored.
pub(crate) const VAULT_SECURITY_CONFIG_NAME: &str = "vault_security.txt";
pub(crate) const VAULT_UNLOCK_COUNTER_NAME: &str = "vault_unlock_failures.txt";
pub const VAULT_ATTEMPT_LIMIT_MIN: u32 = 1;
pub const VAULT_ATTEMPT_LIMIT_MAX: u32 = 100;
pub const QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS: &str =
    "QSC_ERR_VAULT_WIPED_AFTER_FAILED_UNLOCKS";
pub(crate) const QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED: &str = "QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED";
pub(crate) const QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED: &str =
    "QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED";
pub(crate) const FILE_XFER_VERSION: u8 = crate::adversarial::payload::FILE_XFER_VERSION;
pub(crate) const FILE_XFER_DEFAULT_MAX_FILE_SIZE: usize = 256 * 1024;
pub(crate) const FILE_XFER_MAX_FILE_SIZE_CEILING: usize = 4 * 1024 * 1024;
pub(crate) const FILE_XFER_DEFAULT_CHUNK_SIZE: usize = 16 * 1024;
// Sender and receiver must share the same supported chunk ceiling. Larger
// chunks can overflow the current Suite-2 wire body-length field once file
// metadata is serialized, so fail closed before any relay send occurs.
pub(crate) const FILE_XFER_MAX_CHUNK_SIZE_CEILING: usize = FILE_XFER_DEFAULT_CHUNK_SIZE;
pub(crate) const FILE_XFER_DEFAULT_MAX_CHUNKS: usize = 64;
pub(crate) const FILE_XFER_MAX_CHUNKS_CEILING: usize = 256;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct FileTransferRecord {
    pub(crate) id: String,
    pub(crate) peer: String,
    pub(crate) filename: String,
    pub(crate) total_size: usize,
    pub(crate) chunk_count: usize,
    pub(crate) manifest_hash: String,
    #[serde(default)]
    pub(crate) chunk_hashes: Vec<String>,
    #[serde(default)]
    pub(crate) chunks_hex: Vec<String>,
    #[serde(default)]
    pub(crate) confirm_requested: bool,
    #[serde(default)]
    pub(crate) confirm_id: Option<String>,
    #[serde(default)]
    pub(crate) target_device_id: Option<String>,
    pub(crate) state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct AttachmentJournal {
    #[serde(default)]
    pub(crate) records: BTreeMap<String, AttachmentTransferRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct AttachmentTransferRecord {
    pub(crate) attachment_id: String,
    pub(crate) peer: String,
    pub(crate) direction: String,
    pub(crate) service_url: Option<String>,
    pub(crate) state: String,
    // NA-0614: true delivered length. `plaintext_len` is the padded/encrypted length
    // (a size-ladder bucket); `content_len` is the true file length the receiver
    // truncates to. Invariant 0 < content_len <= plaintext_len. #[serde(default)] keeps
    // pre-release persisted records loadable; new transfers always set it explicitly.
    #[serde(default)]
    pub(crate) content_len: u64,
    pub(crate) plaintext_len: u64,
    pub(crate) ciphertext_len: u64,
    pub(crate) part_size_class: String,
    pub(crate) part_count: u32,
    pub(crate) integrity_alg: String,
    pub(crate) integrity_root: String,
    pub(crate) retention_class: String,
    pub(crate) enc_ctx_alg: String,
    pub(crate) enc_ctx_b64u: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) locator_kind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) locator_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) fetch_capability: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) expires_at_unix_s: Option<u64>,
    #[serde(default)]
    pub(crate) confirm_requested: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) confirm_handle: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) filename_hint: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) media_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) source_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) staged_ciphertext_rel: Option<String>,
    #[serde(
        default,
        rename = "session_id",
        alias = "session_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub(crate) session_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) resume_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) timeline_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) target_device_id: Option<String>,
    #[serde(default)]
    pub(crate) uploaded_parts: Vec<u32>,
    #[serde(default)]
    pub(crate) downloaded_ciphertext_bytes: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) download_ciphertext_rel: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) download_output_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) last_error: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct ContactsStore {
    pub(crate) peers: BTreeMap<String, ContactRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct ContactRequestsStore {
    pub(crate) requests: BTreeMap<String, ContactRequestRecord>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactRequestRecord {
    pub(crate) alias: String,
    #[serde(default)]
    pub(crate) device_id: Option<String>,
    #[serde(default)]
    pub(crate) state: String,
    #[serde(default)]
    pub(crate) reason: Option<String>,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
}

#[derive(Default, Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactRecord {
    pub(crate) fp: String,
    pub(crate) status: String,
    pub(crate) blocked: bool,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
    #[serde(default)]
    pub(crate) sig_fp: Option<String>,
    /// NA-0633 (ENG-0038): the peer's full identity KEM public key (hex), verified at add-time
    /// against `fp`. Load-bearing: the initiator encapsulates to it so the responder must prove
    /// KEM-secret possession (DOC-CAN handshake C1). Absent on legacy contacts => the initiator
    /// fails closed rather than fall back to the unauthenticated path.
    #[serde(default)]
    pub(crate) kem_pk: Option<String>,
    #[serde(default)]
    pub(crate) route_token: Option<String>,
    #[serde(default)]
    pub(crate) primary_device_id: Option<String>,
    #[serde(default)]
    pub(crate) devices: Vec<ContactDeviceRecord>,
    // ---- NA-0681 (D616 §2f / DESIGN P3): laid from contact #1, additive, `serde(default)`
    // so every existing record loads unchanged.
    /// PLURAL from day one. Relay migration later happens by authenticated in-session
    /// announcement, never by re-invite -- so the field that will hold the second endpoint
    /// exists before there is a second endpoint.
    #[serde(default)]
    pub(crate) relay_endpoints: Vec<String>,
    /// Captured at redemption, DORMANT. Relay-pinning is explicitly not in this epic; the
    /// hook exists so pinning is later a feature rather than a migration.
    #[serde(default)]
    pub(crate) pinned_cert_fp: Option<String>,
    /// Which invite produced this contact. Provenance, and the key the client-side
    /// single-use check reads.
    #[serde(default)]
    pub(crate) invite_id: Option<String>,
    // ---- NA-0764 (`D-1405`) — the LOCAL display name, additive, `serde(default)` so every
    // existing record loads unchanged.
    /// What YOU call this contact. **Never sent anywhere**, and never supplied by the network.
    ///
    /// ⚠⚠ **NOT THE KEY, AND THAT IS THE WHOLE POINT.** The alias is this record's key in
    /// `ContactsStore.peers`, and the SAME string keys `identity_read_pin(peer)` and
    /// `qsp_session_for_channel(channel)`. A rename that re-keyed the map would therefore
    /// reach identity pins and live sessions. This field sits BESIDE the key so a rename
    /// touches neither: the UI renders `display_name` and passes `alias`, always.
    ///
    /// ⚠ **`None`, never `Some("")`.** The setter normalises at the boundary so no consumer
    /// has to special-case an empty string — one of them would forget. `None` means "no local
    /// name; show the alias".
    ///
    /// ⚠ Duplicates are PERMITTED and that is deliberate (`R6`): identity is the 30-digit
    /// code, never the name. Two contacts may carry the same display name; neither becomes
    /// the other, because nothing keyed on this field.
    #[serde(default)]
    pub(crate) display_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub(crate) struct ContactDeviceRecord {
    pub(crate) device_id: String,
    pub(crate) fp: String,
    #[serde(default)]
    pub(crate) sig_fp: Option<String>,
    /// NA-0633 (ENG-0038): the device's full identity KEM public key (hex); see ContactRecord::kem_pk.
    #[serde(default)]
    pub(crate) kem_pk: Option<String>,
    pub(crate) state: String,
    #[serde(default)]
    pub(crate) route_token: Option<String>,
    #[serde(default)]
    pub(crate) seen_at: Option<u64>,
    #[serde(default)]
    pub(crate) label: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub(crate) struct TimelineStore {
    #[serde(default = "crate::timeline_ts_default")]
    pub(crate) next_ts: u64,
    #[serde(default)]
    pub(crate) peers: BTreeMap<String, Vec<crate::TimelineEntry>>,
    #[serde(default)]
    pub(crate) file_transfers: BTreeMap<String, FileTransferRecord>,
}


// NDI2 file payloads have a fixed-width initiating reference outside the canonical
// JSON descriptor. Stage A validates the complete shape but never executes files.
// SID[16], direction:u8, initiating-id-length:u8, ASCII initiating-id,
// initiating-epoch:u64, initiating-slot:u32, request:u8, digest[32],
// descriptor-length:u32, canonical typed descriptor JSON. Integers are big-endian.
// The 12 reference bytes are reserved unresolved at enqueue and filled once at seal.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DirectionalConfirmation {
    handle: String,
    content_len: u64,
}

pub(crate) fn directional_file_shape(kind: u8, raw: &[u8]) -> Result<(), &'static str> {
    const ERROR: &str = "INTEGRATION_FILE_SHAPE";
    if !(1..=4).contains(&kind) || raw.len() > 60000 || raw.len() < 18 { return Err(ERROR); }
    let id_len = raw[17] as usize;
    let header = 18usize.checked_add(id_len).and_then(|n| n.checked_add(12 + 1 + 32 + 4)).ok_or(ERROR)?;
    if raw[16] > 1 || id_len == 0 || id_len > 64 || raw.len() < header || !raw[18..18 + id_len].is_ascii() { return Err(ERROR); }
    let request = raw[18 + id_len + 12];
    if request > 1 || (kind == 4 && request != 1) { return Err("INTEGRATION_FILE_REQUEST"); }
    let length = u32::from_be_bytes(raw[header - 4..header].try_into().unwrap()) as usize;
    if length != raw.len() - header { return Err(ERROR); }
    let data = &raw[header..];
    // Exact canonical reserialization rejects duplicate/unknown fields, omitted
    // defaulted fields and permissive legacy parser behavior without changing it.
    fn strict<T: for<'a> Deserialize<'a> + Serialize>(raw: &[u8]) -> Result<T, &'static str> {
        let value: T = serde_json::from_slice(raw).map_err(|_| "INTEGRATION_FILE_SHAPE")?;
        if serde_json::to_vec(&value).map_err(|_| "INTEGRATION_FILE_SHAPE")? != raw { return Err("INTEGRATION_FILE_SHAPE"); }
        Ok(value)
    }
    fn filename(name: &str) -> bool {
        !name.is_empty() && name != "." && name != ".." && !name.contains(['/', '\\', '\0'])
    }
    fn digest(value: &str) -> bool { value.len() == 32 && value.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) }
    match kind {
        1 => {
            let v: AttachmentDescriptorPayload = strict(data)?;
            use base64::Engine;
            let context = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&v.enc_ctx_b64u).map_err(|_| ERROR)?;
            let part_size = match v.part_size_class.as_str() { "p64k"=>65536u64, "p256k"=>262144, "p1024k"=>1048576, _=>return Err(ERROR) };
            let expected_parts = v.plaintext_len.checked_add(part_size - 17).ok_or(ERROR)? / (part_size - 16);
            if v.integrity_alg != crate::ATTACHMENT_INTEGRITY_ALG_V1 || v.locator_kind != crate::ATTACHMENT_LOCATOR_KIND_V1
                || v.enc_ctx_alg != crate::ATTACHMENT_ENC_CTX_ALG_V1 || context.len() != 41 || context[0] != 1
                || v.enc_ctx_b64u.len() != 55 || expected_parts != u64::from(v.part_count)
                || v.locator_ref.trim().is_empty() || v.locator_ref.len()>128 || !(32..=255).contains(&v.fetch_capability.len())
                || !matches!(v.retention_class.as_str(), "short"|"standard"|"extended")
                || v.expires_at_unix_s == 0 || v.confirm_requested != v.confirm_handle.is_some()
                || v.confirm_handle.as_ref().is_some_and(|h| h.len()!=24 || !h.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()))
                || v.v != 1 || v.t != "attachment_descriptor" || v.attachment_id.len()!=64
                || !v.attachment_id.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
                || v.content_len == 0 || v.content_len > v.plaintext_len || v.plaintext_len > 100 * 1024 * 1024
                || v.part_count == 0 || v.part_count > 4096
                || v.ciphertext_len != v.plaintext_len.checked_add(16 * u64::from(v.part_count)).ok_or(ERROR)?
                || v.confirm_requested != (request == 1) || (request == 1 && v.confirm_handle.as_ref().is_none_or(|h| h.is_empty()))
                || v.filename_hint.as_ref().is_some_and(|n| !filename(n)) { return Err(ERROR); }
        }
        2 => {
            let v: FileTransferChunkPayload = strict(data)?;
            if v.v != FILE_XFER_VERSION || v.t != "file_chunk" || !filename(&v.filename) || v.file_id.is_empty()
                || v.total_size == 0 || v.total_size > 4 * 1024 * 1024 || v.chunk_count == 0 || v.chunk_count > 256
                || v.chunk_index >= v.chunk_count || v.chunk.is_empty() || v.chunk.len() > v.total_size
                || !digest(&v.chunk_hash) || !digest(&v.manifest_hash) { return Err(ERROR); }
        }
        3 => {
            let v: FileTransferManifestPayload = strict(data)?;
            if v.v != FILE_XFER_VERSION || v.t != "file_manifest" || !filename(&v.filename) || v.file_id.is_empty()
                || v.total_size == 0 || v.total_size > 4 * 1024 * 1024 || v.chunk_count == 0 || v.chunk_count > 256
                || v.chunk_hashes.len() != v.chunk_count || !v.chunk_hashes.iter().all(|h| digest(h))
                || !digest(&v.manifest_hash) || v.confirm_requested != (request == 1)
                || (request == 1 && v.confirm_id.is_empty()) { return Err(ERROR); }
        }
        4 => {
            let v: DirectionalConfirmation = strict(data)?;
            if v.handle.is_empty() || v.content_len == 0 || v.content_len > 100 * 1024 * 1024 { return Err(ERROR); }
        }
        _ => return Err(ERROR),
    }
    Ok(())
}

#[cfg(test)]
mod directional_file_tests {
    use super::*;
    fn wrap(json: Vec<u8>, request:u8)->Vec<u8> {
        let mut out=vec![0;16];out.extend([0,1,b'i']);out.extend(0u64.to_be_bytes());out.extend(0u32.to_be_bytes());
        out.push(request);out.extend([0;32]);out.extend((json.len() as u32).to_be_bytes());out.extend(json);out
    }
    pub(crate) fn fixture(kind:u8)->Vec<u8> {
        let data=match kind {
            1=>serde_json::to_vec(&AttachmentDescriptorPayload {
                v:1,t:"attachment_descriptor".into(),attachment_id:"a".repeat(64),content_len:1,plaintext_len:4096,
                ciphertext_len:4112,part_size_class:"p64k".into(),part_count:1,integrity_alg:"sha512_merkle_v1".into(),
                integrity_root:"0".repeat(128),locator_kind:"service_ref_v1".into(),locator_ref:"synthetic".into(),
                fetch_capability:"x".repeat(32),enc_ctx_alg:"chacha20poly1305_part_v1".into(),enc_ctx_b64u:{ use base64::Engine; let mut raw=[0u8;41];raw[0]=1;base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw) },
                retention_class:"standard".into(),expires_at_unix_s:u64::MAX,confirm_requested:true,
                confirm_handle:Some("a".repeat(24)),filename_hint:Some("file.bin".into()),media_type:None,
            }).unwrap(),
            2=>serde_json::to_vec(&FileTransferChunkPayload {
                v:1,t:"file_chunk".into(),file_id:"file".into(),filename:"file.bin".into(),total_size:1,
                chunk_index:0,chunk_count:1,chunk_hash:"0".repeat(32),manifest_hash:"0".repeat(32),chunk:vec![1],
            }).unwrap(),
            3=>serde_json::to_vec(&FileTransferManifestPayload {
                v:1,t:"file_manifest".into(),file_id:"file".into(),filename:"file.bin".into(),total_size:1,
                chunk_count:1,chunk_hashes:vec!["0".repeat(32)],manifest_hash:"0".repeat(32),confirm_requested:true,confirm_id:"confirm".into(),
            }).unwrap(),
            4=>serde_json::to_vec(&DirectionalConfirmation {handle:"confirm".into(),content_len:1}).unwrap(),
            _=>panic!("invalid test kind"),
        };
        wrap(data,1)
    }
    #[test]
    fn strict_file_kinds_requests_and_lengths() {
        for kind in 1..=4 {
            let raw=fixture(kind);assert_eq!(directional_file_shape(kind,&raw),Ok(()));
            let mut wrong=raw.clone();wrong[31]=2;
            assert_eq!(directional_file_shape(kind,&wrong),Err("INTEGRATION_FILE_REQUEST"));
            let mut trailing=raw.clone();trailing.push(0);assert!(directional_file_shape(kind,&trailing).is_err());
            assert!(directional_file_shape(kind,&raw[..raw.len()-1]).is_err());
            let mut wrong_dir=raw.clone();wrong_dir[16]=2;assert!(directional_file_shape(kind,&wrong_dir).is_err());
            // All file codecs are typed: another file kind cannot be guessed.
            assert!(directional_file_shape(if kind==4{1}else{kind+1},&raw).is_err());
        }
        assert!(directional_file_shape(0,&fixture(1)).is_err());
        let mut bad=fixture(4);bad[31]=0;assert!(directional_file_shape(4,&bad).is_err());
        let missing=wrap(br#"{"handle":"confirm"}"#.to_vec(),1);assert!(directional_file_shape(4,&missing).is_err());
        let unknown=wrap(br#"{"handle":"confirm","content_len":1,"unknown":0}"#.to_vec(),1);assert!(directional_file_shape(4,&unknown).is_err());
    }
    #[test]
    fn file_execution_stays_gated_after_codec_validation() {
        for kind in 1..=4 { crate::directional_delivery::test_file_body(kind,&fixture(kind)); }
    }
}
