use super::*;

type FileConfirmPayload = adversarial::payload::FileConfirmPayload;

pub(super) fn parse_file_confirm_payload(plaintext: &[u8]) -> Option<FileConfirmPayload> {
    adversarial::payload::parse_file_confirm_payload(plaintext)
}

pub(super) fn parse_file_transfer_payload(plaintext: &[u8]) -> Option<FileTransferPayload> {
    adversarial::payload::parse_file_transfer_payload(plaintext)
}

pub(super) fn parse_attachment_descriptor_payload(
    plaintext: &[u8],
) -> Option<AttachmentDescriptorPayload> {
    adversarial::payload::parse_attachment_descriptor_payload(plaintext)
}

pub(super) fn parse_attachment_confirm_payload(
    plaintext: &[u8],
) -> Option<AttachmentConfirmPayload> {
    adversarial::payload::parse_attachment_confirm_payload(plaintext)
}

pub(super) fn attachment_journal_load() -> Result<AttachmentJournal, &'static str> {
    match vault::secret_get(ATTACHMENT_JOURNAL_SECRET_KEY) {
        Ok(None) => Ok(AttachmentJournal::default()),
        Ok(Some(v)) => {
            serde_json::from_str::<AttachmentJournal>(&v).map_err(|_| "attachment_journal_tampered")
        }
        Err("vault_missing" | "vault_locked") => Err("attachment_journal_unavailable"),
        Err(_) => Err("attachment_journal_unavailable"),
    }
}

pub(super) fn attachment_journal_save(store: &AttachmentJournal) -> Result<(), &'static str> {
    let json = serde_json::to_string(store).map_err(|_| "attachment_journal_unavailable")?;
    match vault::secret_set(ATTACHMENT_JOURNAL_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("attachment_journal_unavailable"),
        Err(_) => Err("attachment_journal_unavailable"),
    }
}

pub(super) fn attachment_record_key(direction: &str, peer: &str, attachment_id: &str) -> String {
    format!("{direction}:{peer}:{attachment_id}")
}

fn attachment_find_outbound_by_source(
    store: &AttachmentJournal,
    peer: &str,
    source_path: &Path,
) -> Option<(String, AttachmentTransferRecord)> {
    let needle = source_path.to_string_lossy();
    store.records.iter().find_map(|(key, rec)| {
        if rec.direction == "out"
            && rec.peer == peer
            && rec.source_path.as_deref() == Some(needle.as_ref())
            && rec.state != "PEER_CONFIRMED"
        {
            Some((key.clone(), rec.clone()))
        } else {
            None
        }
    })
}

fn attachment_stage_root(cfg_dir: &Path) -> PathBuf {
    cfg_dir.join(ATTACHMENT_STAGING_DIR)
}

fn attachment_staging_dir(cfg_dir: &Path, direction: &str) -> Result<PathBuf, &'static str> {
    let (root, source) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    if root != cfg_dir {
        return Err("attachment_stage_unavailable");
    }
    let dir = attachment_stage_root(cfg_dir).join(direction);
    ensure_dir_secure(&dir, source).map_err(|_| "attachment_stage_unavailable")?;
    Ok(dir)
}

fn attachment_outbound_rel(attachment_id: &str) -> String {
    format!("outbound/{attachment_id}.cipher")
}

fn attachment_inbound_rel(attachment_id: &str) -> String {
    format!("inbound/{attachment_id}.cipher")
}

fn attachment_path_from_rel(cfg_dir: &Path, rel: &str) -> Result<PathBuf, &'static str> {
    if rel.contains("..") {
        return Err("attachment_stage_unavailable");
    }
    let path = attachment_stage_root(cfg_dir).join(rel);
    let source = ConfigSource::EnvOverride;
    enforce_safe_parents(&path, source).map_err(|_| "attachment_stage_unavailable")?;
    Ok(path)
}

fn attachment_part_size_bytes(class: &str) -> Option<usize> {
    match class {
        "p64k" => Some(65_536),
        "p256k" => Some(262_144),
        "p1024k" => Some(1_048_576),
        _ => None,
    }
}

fn choose_attachment_part_size_class(plaintext_len: u64) -> &'static str {
    if plaintext_len <= 16 * 1024 * 1024 {
        "p64k"
    } else if plaintext_len <= 64 * 1024 * 1024 {
        "p256k"
    } else {
        "p1024k"
    }
}

fn attachment_plaintext_capacity(class: &str) -> Option<usize> {
    attachment_part_size_bytes(class)?.checked_sub(ATTACHMENT_CIPHER_TAG_LEN)
}

fn attachment_part_count_for_plaintext(plaintext_len: u64, class: &str) -> Option<u32> {
    let capacity = attachment_plaintext_capacity(class)? as u64;
    if plaintext_len == 0 || capacity == 0 {
        return None;
    }
    let count = plaintext_len.div_ceil(capacity);
    u32::try_from(count).ok()
}

fn attachment_ciphertext_len_for_plaintext(plaintext_len: u64, part_count: u32) -> Option<u64> {
    plaintext_len.checked_add(u64::from(part_count) * ATTACHMENT_CIPHER_TAG_LEN as u64)
}

fn attachment_ciphertext_part_len(
    part_index: u32,
    _plaintext_len: u64,
    part_size_class: &str,
    part_count: u32,
    ciphertext_len: u64,
) -> Option<usize> {
    let part_size = attachment_part_size_bytes(part_size_class)? as u64;
    if part_index >= part_count || part_count == 0 {
        return None;
    }
    if part_index + 1 < part_count {
        return usize::try_from(part_size).ok();
    }
    let offset = u64::from(part_index) * part_size;
    usize::try_from(ciphertext_len.checked_sub(offset)?).ok()
}

fn attachment_nonce(prefix: &[u8; 8], part_index: u32) -> [u8; 12] {
    let mut out = [0u8; 12];
    out[..8].copy_from_slice(prefix);
    out[8..].copy_from_slice(&part_index.to_be_bytes());
    out
}

fn attachment_part_aad(
    attachment_id: &str,
    enc_ctx_alg: &str,
    plaintext_len: u64,
    ciphertext_len: u64,
    part_size_class: &str,
    part_count: u32,
    part_index: u32,
) -> Vec<u8> {
    format!(
        "QATT-PART-V1|{attachment_id}|{enc_ctx_alg}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{part_index}"
    )
    .into_bytes()
}

fn attachment_merkle_leaf(part_index: u32, bytes: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.update([0x00]);
    hasher.update(part_index.to_be_bytes());
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = [0u8; 64];
    out.copy_from_slice(&digest);
    out
}

fn attachment_merkle_root(mut level: Vec<[u8; 64]>) -> Option<String> {
    if level.is_empty() {
        return None;
    }
    while level.len() > 1 {
        let mut next = Vec::with_capacity(level.len().div_ceil(2));
        let mut idx = 0usize;
        while idx < level.len() {
            let left = level[idx];
            let right = if idx + 1 < level.len() {
                level[idx + 1]
            } else {
                level[idx]
            };
            let mut hasher = Sha512::new();
            hasher.update([0x01]);
            hasher.update(left);
            hasher.update(right);
            let digest = hasher.finalize();
            let mut out = [0u8; 64];
            out.copy_from_slice(&digest);
            next.push(out);
            idx += 2;
        }
        level = next;
    }
    Some(hex_encode(&level[0]))
}

fn attachment_generate_id() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

fn attachment_build_enc_ctx() -> (String, [u8; 32], [u8; 8]) {
    let mut cek = [0u8; 32];
    let mut prefix = [0u8; 8];
    OsRng.fill_bytes(&mut cek);
    OsRng.fill_bytes(&mut prefix);
    let mut raw = [0u8; ATTACHMENT_CONTEXT_PACKAGE_LEN];
    raw[0] = 0x01;
    raw[1..33].copy_from_slice(&cek);
    raw[33..].copy_from_slice(&prefix);
    (URL_SAFE_NO_PAD.encode(raw), cek, prefix)
}

fn attachment_decode_enc_ctx(token: &str) -> Result<([u8; 32], [u8; 8]), &'static str> {
    if token.len() != ATTACHMENT_CONTEXT_PACKAGE_B64U_LEN {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let raw = URL_SAFE_NO_PAD
        .decode(token.as_bytes())
        .map_err(|_| "REJECT_ATT_DESC_ENC_CTX")?;
    if raw.len() != ATTACHMENT_CONTEXT_PACKAGE_LEN || raw[0] != 0x01 {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let mut cek = [0u8; 32];
    let mut prefix = [0u8; 8];
    cek.copy_from_slice(&raw[1..33]);
    prefix.copy_from_slice(&raw[33..41]);
    Ok((cek, prefix))
}

struct AttachmentConfirmHandleInput<'a> {
    attachment_id: &'a str,
    plaintext_len: u64,
    ciphertext_len: u64,
    part_size_class: &'a str,
    part_count: u32,
    integrity_alg: &'a str,
    integrity_root: &'a str,
    retention_class: &'a str,
    expires_at_unix_s: u64,
}

fn attachment_confirm_handle(input: AttachmentConfirmHandleInput<'_>) -> String {
    let AttachmentConfirmHandleInput {
        attachment_id,
        plaintext_len,
        ciphertext_len,
        part_size_class,
        part_count,
        integrity_alg,
        integrity_root,
        retention_class,
        expires_at_unix_s,
    } = input;
    let material = format!(
        "QATT-CONFIRM-V1|{attachment_id}|{plaintext_len}|{ciphertext_len}|{part_size_class}|{part_count}|{integrity_alg}|{integrity_root}|{retention_class}|{expires_at_unix_s}"
    );
    let digest = Sha512::digest(material.as_bytes());
    hex_encode(&digest[..12])
}

fn attachment_is_lower_hex_len(value: &str, len: usize) -> bool {
    value.len() == len
        && value
            .chars()
            .all(|ch| ch.is_ascii_hexdigit() && !ch.is_ascii_uppercase())
}

fn file_xfer_chunk_hash(chunk: &[u8]) -> String {
    let c = StdCrypto;
    let hash = c.sha512(chunk);
    hex_encode(&hash[..16])
}

fn file_xfer_id(peer: &str, filename: &str, payload: &[u8]) -> String {
    let c = StdCrypto;
    let mut data = Vec::new();
    data.extend_from_slice(peer.as_bytes());
    data.push(0);
    data.extend_from_slice(filename.as_bytes());
    data.push(0);
    data.extend_from_slice(payload);
    let hash = c.sha512(&data);
    hex_encode(&hash[..12])
}

fn file_xfer_manifest_hash(
    file_id: &str,
    total_size: usize,
    chunk_count: usize,
    chunk_hashes: &[String],
) -> String {
    let c = StdCrypto;
    let joined = chunk_hashes.join(",");
    let data = format!("{}|{}|{}|{}", file_id, total_size, chunk_count, joined);
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..16])
}

fn file_xfer_confirm_id(file_id: &str, manifest_hash: &str) -> String {
    let c = StdCrypto;
    let data = format!("{}|{}", file_id, manifest_hash);
    let hash = c.sha512(data.as_bytes());
    hex_encode(&hash[..12])
}

#[derive(Serialize)]
struct AttachmentServiceCreateSessionRequest {
    attachment_id: String,
    ciphertext_len: u64,
    part_size_class: String,
    part_count: u32,
    integrity_alg: String,
    integrity_root: String,
    retention_class: String,
}

#[derive(Deserialize)]
struct AttachmentServiceCreateSessionResponse {
    #[serde(rename = "session_id")]
    session_ref: String,
    resume_token: String,
}

#[derive(Deserialize)]
struct AttachmentServiceMissingRange {
    start: u32,
    end: u32,
}

#[derive(Deserialize)]
struct AttachmentServiceSessionStatusResponse {
    missing_part_ranges: Vec<AttachmentServiceMissingRange>,
}

#[derive(Serialize)]
struct AttachmentServiceCommitRequest {
    attachment_id: String,
    ciphertext_len: u64,
    part_count: u32,
    integrity_alg: String,
    integrity_root: String,
    retention_class: String,
}

#[derive(Deserialize)]
struct AttachmentServiceCommitResponse {
    locator_kind: String,
    locator_ref: String,
    fetch_capability: String,
    expires_at_unix_s: u64,
}

#[derive(Deserialize)]
struct AttachmentServiceErrorBody {
    reason_code: String,
}

fn attachment_now_unix_s() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn attachment_service_reason(
    response: reqwest::blocking::Response,
    fallback: &'static str,
) -> String {
    let status = response.status().as_u16();
    match response.json::<AttachmentServiceErrorBody>() {
        Ok(body) if !body.reason_code.trim().is_empty() => body.reason_code,
        _ => format!("{fallback}_{status}"),
    }
}

fn attachment_service_create_session(
    service_url: &str,
    record: &AttachmentTransferRecord,
) -> Result<AttachmentServiceCreateSessionResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions");
    let client = HttpClient::new();
    let request = AttachmentServiceCreateSessionRequest {
        attachment_id: record.attachment_id.clone(),
        ciphertext_len: record.ciphertext_len,
        part_size_class: record.part_size_class.clone(),
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        retention_class: record.retention_class.clone(),
    };
    let response = client
        .post(url)
        .json(&request)
        .send()
        .map_err(|_| "attachment_service_create_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_create_failed",
        ));
    }
    response
        .json::<AttachmentServiceCreateSessionResponse>()
        .map_err(|_| "attachment_service_create_parse_failed".to_string())
}

fn attachment_service_status(
    service_url: &str,
    session_ref: &str,
    resume_token: &str,
) -> Result<AttachmentServiceSessionStatusResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let response = client
        .get(url)
        .header("X-QATT-Resume-Token", token)
        .send()
        .map_err(|_| "attachment_service_status_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_status_failed",
        ));
    }
    response
        .json::<AttachmentServiceSessionStatusResponse>()
        .map_err(|_| "attachment_service_status_parse_failed".to_string())
}

fn attachment_service_upload_part(
    service_url: &str,
    session_ref: &str,
    part_index: u32,
    resume_token: &str,
    bytes: Vec<u8>,
) -> Result<(), String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}/parts/{part_index}");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let response = client
        .put(url)
        .header("X-QATT-Resume-Token", token)
        .body(bytes)
        .send()
        .map_err(|_| "attachment_service_upload_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_upload_failed",
        ));
    }
    Ok(())
}

fn attachment_service_commit(
    service_url: &str,
    session_ref: &str,
    resume_token: &str,
    record: &AttachmentTransferRecord,
) -> Result<AttachmentServiceCommitResponse, String> {
    let url = format!("{service_url}/v1/attachments/sessions/{session_ref}/commit");
    let client = HttpClient::new();
    let token = env::var("QSC_ATTACHMENT_RESUME_TOKEN_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| resume_token.to_string());
    let request = AttachmentServiceCommitRequest {
        attachment_id: record.attachment_id.clone(),
        ciphertext_len: record.ciphertext_len,
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        retention_class: record.retention_class.clone(),
    };
    let response = client
        .post(url)
        .header("X-QATT-Resume-Token", token)
        .json(&request)
        .send()
        .map_err(|_| "attachment_service_commit_failed".to_string())?;
    if !response.status().is_success() {
        return Err(attachment_service_reason(
            response,
            "attachment_service_commit_failed",
        ));
    }
    response
        .json::<AttachmentServiceCommitResponse>()
        .map_err(|_| "attachment_service_commit_parse_failed".to_string())
}

fn attachment_validate_filename_hint(raw: &str) -> Result<String, &'static str> {
    let trimmed = raw.trim();
    if trimmed.is_empty()
        || trimmed.len() > 255
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed == "."
        || trimmed == ".."
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    Ok(trimmed.to_string())
}

fn attachment_output_name(record: &AttachmentTransferRecord) -> String {
    record
        .filename_hint
        .as_deref()
        .and_then(|v| attachment_validate_filename_hint(v).ok())
        .unwrap_or_else(|| {
            format!(
                "attachment-{}.bin",
                file_delivery_short_id(&record.attachment_id)
            )
        })
}

fn attachment_build_outbound_record(
    peer: &str,
    service_url: &str,
    path: &Path,
    receipt: Option<ReceiptKind>,
) -> Result<AttachmentTransferRecord, &'static str> {
    let metadata = fs::metadata(path).map_err(|_| "file_xfer_read_failed")?;
    let plaintext_len = metadata.len();
    if plaintext_len == 0 {
        return Err("file_xfer_empty");
    }
    if plaintext_len > ATTACHMENT_DEFAULT_MAX_FILE_SIZE as u64 {
        return Err("size_exceeds_max");
    }
    let filename_hint = path
        .file_name()
        .and_then(|v| v.to_str())
        .map(attachment_validate_filename_hint)
        .transpose()?;
    let attachment_id = attachment_generate_id();
    let part_size_class = choose_attachment_part_size_class(plaintext_len).to_string();
    let part_count = attachment_part_count_for_plaintext(plaintext_len, &part_size_class)
        .ok_or("attachment_shape_invalid")?;
    let ciphertext_len = attachment_ciphertext_len_for_plaintext(plaintext_len, part_count)
        .ok_or("attachment_shape_invalid")?;
    let (enc_ctx_b64u, cek, nonce_prefix) = attachment_build_enc_ctx();
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    let stage_dir = attachment_staging_dir(&cfg_dir, "outbound")?;
    let staged_rel = attachment_outbound_rel(&attachment_id);
    let staged_path = stage_dir.join(format!("{attachment_id}.cipher"));
    let mut src = File::open(path).map_err(|_| "file_xfer_read_failed")?;
    let mut dst = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&staged_path)
        .map_err(|_| "attachment_stage_unavailable")?;
    #[cfg(unix)]
    enforce_file_perms(&staged_path).map_err(|_| "attachment_stage_unavailable")?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cek));
    let capacity =
        attachment_plaintext_capacity(&part_size_class).ok_or("attachment_shape_invalid")?;
    let mut leaves = Vec::with_capacity(part_count as usize);
    let mut buf = vec![0u8; capacity];
    let mut produced = 0u32;
    loop {
        let mut read_len = 0usize;
        while read_len < capacity {
            let n = src
                .read(&mut buf[read_len..])
                .map_err(|_| "file_xfer_read_failed")?;
            if n == 0 {
                break;
            }
            read_len += n;
        }
        if read_len == 0 {
            break;
        }
        let nonce = attachment_nonce(&nonce_prefix, produced);
        let aad = attachment_part_aad(
            &attachment_id,
            ATTACHMENT_ENC_CTX_ALG_V1,
            plaintext_len,
            ciphertext_len,
            &part_size_class,
            part_count,
            produced,
        );
        let ciphertext = cipher
            .encrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &buf[..read_len],
                    aad: &aad,
                },
            )
            .map_err(|_| "attachment_encrypt_failed")?;
        dst.write_all(&ciphertext)
            .map_err(|_| "attachment_stage_unavailable")?;
        leaves.push(attachment_merkle_leaf(produced, &ciphertext));
        produced = produced.saturating_add(1);
        if read_len < capacity {
            break;
        }
    }
    dst.sync_all().map_err(|_| "attachment_stage_unavailable")?;
    if produced != part_count {
        let _ = fs::remove_file(&staged_path);
        return Err("attachment_shape_invalid");
    }
    let integrity_root = attachment_merkle_root(leaves).ok_or("attachment_shape_invalid")?;
    Ok(AttachmentTransferRecord {
        attachment_id,
        peer: peer.to_string(),
        direction: "out".to_string(),
        service_url: Some(service_url.to_string()),
        state: "STAGED".to_string(),
        plaintext_len,
        ciphertext_len,
        part_size_class,
        part_count,
        integrity_alg: ATTACHMENT_INTEGRITY_ALG_V1.to_string(),
        integrity_root,
        retention_class: "standard".to_string(),
        enc_ctx_alg: ATTACHMENT_ENC_CTX_ALG_V1.to_string(),
        enc_ctx_b64u,
        locator_kind: None,
        locator_ref: None,
        fetch_capability: None,
        expires_at_unix_s: None,
        confirm_requested: receipt.is_some(),
        confirm_handle: None,
        filename_hint,
        media_type: None,
        source_path: Some(path.to_string_lossy().to_string()),
        staged_ciphertext_rel: Some(staged_rel),
        session_ref: None,
        resume_token: None,
        timeline_id: None,
        target_device_id: None,
        uploaded_parts: Vec::new(),
        downloaded_ciphertext_bytes: 0,
        download_ciphertext_rel: None,
        download_output_name: None,
        last_error: None,
    })
}

fn attachment_read_staged_part(
    cfg_dir: &Path,
    record: &AttachmentTransferRecord,
    part_index: u32,
) -> Result<Vec<u8>, &'static str> {
    let rel = record
        .staged_ciphertext_rel
        .as_deref()
        .ok_or("attachment_stage_missing")?;
    let path = attachment_path_from_rel(cfg_dir, rel)?;
    let mut file = File::open(path).map_err(|_| "attachment_stage_missing")?;
    let part_size = attachment_part_size_bytes(&record.part_size_class)
        .ok_or("attachment_shape_invalid")? as u64;
    let offset = u64::from(part_index) * part_size;
    let len = attachment_ciphertext_part_len(
        part_index,
        record.plaintext_len,
        &record.part_size_class,
        record.part_count,
        record.ciphertext_len,
    )
    .ok_or("attachment_shape_invalid")?;
    use std::io::Seek;
    use std::io::SeekFrom;
    file.seek(SeekFrom::Start(offset))
        .map_err(|_| "attachment_stage_missing")?;
    let mut buf = vec![0u8; len];
    file.read_exact(&mut buf)
        .map_err(|_| "attachment_stage_missing")?;
    Ok(buf)
}

fn attachment_upload_missing_parts(
    service_url: &str,
    record: &AttachmentTransferRecord,
) -> Result<(), String> {
    let session_ref = record
        .session_ref
        .as_deref()
        .ok_or_else(|| "attachment_session_missing".to_string())?;
    let resume_token = record
        .resume_token
        .as_deref()
        .ok_or_else(|| "attachment_resume_missing".to_string())?;
    let status = attachment_service_status(service_url, session_ref, resume_token)?;
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable".to_string())?;
    let abort_after = env::var("QSC_ATTACHMENT_TEST_ABORT_AFTER_UPLOAD_PARTS")
        .ok()
        .and_then(|v| v.parse::<u32>().ok());
    let mut uploaded = 0u32;
    for range in status.missing_part_ranges {
        for part_index in range.start..=range.end {
            let bytes = attachment_read_staged_part(&cfg_dir, record, part_index)
                .map_err(|e| e.to_string())?;
            attachment_service_upload_part(
                service_url,
                session_ref,
                part_index,
                resume_token,
                bytes,
            )?;
            uploaded = uploaded.saturating_add(1);
            let short_id = file_delivery_short_id(&record.attachment_id);
            let part_s = part_index.to_string();
            emit_marker(
                "attachment_upload_part",
                None,
                &[
                    ("id", short_id.as_str()),
                    ("part", part_s.as_str()),
                    ("ok", "true"),
                ],
            );
            if abort_after.is_some_and(|limit| uploaded >= limit) {
                return Err("attachment_test_interrupt_upload".to_string());
            }
        }
    }
    Ok(())
}

fn attachment_build_descriptor(record: &AttachmentTransferRecord) -> Result<Vec<u8>, &'static str> {
    let expires_at = record
        .expires_at_unix_s
        .ok_or("attachment_descriptor_missing")?;
    let confirm_handle = if record.confirm_requested {
        Some(attachment_confirm_handle(AttachmentConfirmHandleInput {
            attachment_id: &record.attachment_id,
            plaintext_len: record.plaintext_len,
            ciphertext_len: record.ciphertext_len,
            part_size_class: &record.part_size_class,
            part_count: record.part_count,
            integrity_alg: &record.integrity_alg,
            integrity_root: &record.integrity_root,
            retention_class: &record.retention_class,
            expires_at_unix_s: expires_at,
        }))
    } else {
        None
    };
    let payload = AttachmentDescriptorPayload {
        v: ATTACHMENT_DESCRIPTOR_VERSION,
        t: ATTACHMENT_DESCRIPTOR_TYPE.to_string(),
        attachment_id: record.attachment_id.clone(),
        plaintext_len: record.plaintext_len,
        ciphertext_len: record.ciphertext_len,
        part_size_class: record.part_size_class.clone(),
        part_count: record.part_count,
        integrity_alg: record.integrity_alg.clone(),
        integrity_root: record.integrity_root.clone(),
        locator_kind: record
            .locator_kind
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        locator_ref: record
            .locator_ref
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        fetch_capability: record
            .fetch_capability
            .clone()
            .ok_or("attachment_descriptor_missing")?,
        enc_ctx_alg: record.enc_ctx_alg.clone(),
        enc_ctx_b64u: record.enc_ctx_b64u.clone(),
        retention_class: record.retention_class.clone(),
        expires_at_unix_s: expires_at,
        confirm_requested: record.confirm_requested,
        confirm_handle,
        filename_hint: record.filename_hint.clone(),
        media_type: record.media_type.clone(),
    };
    serde_json::to_vec(&payload).map_err(|_| "attachment_descriptor_encode_failed")
}

struct AttachmentSendExec<'a> {
    to: &'a str,
    path: &'a Path,
    relay: &'a str,
    service_url: &'a str,
    allow_legacy_sized: bool,
    max_file_size: Option<usize>,
    max_parts: Option<usize>,
    receipt: Option<ReceiptKind>,
}

fn attachment_send_execute(args: AttachmentSendExec<'_>) -> Result<(), String> {
    let AttachmentSendExec {
        to,
        path,
        relay,
        service_url,
        allow_legacy_sized,
        max_file_size,
        max_parts,
        receipt,
    } = args;
    if let Err(code) = enforce_peer_not_blocked(to) {
        return Err(code.to_string());
    }
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        return Err(code.to_string());
    }
    if let Err(reason) = protocol_active_or_reason_for_send_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let routing = resolve_send_routing_target(to).map_err(|e| e.to_string())?;
    let effective_limit = max_file_size.unwrap_or(ATTACHMENT_DEFAULT_MAX_FILE_SIZE);
    let effective_max_parts = max_parts.unwrap_or(ATTACHMENT_DEFAULT_MAX_PARTS);
    let payload_len = fs::metadata(path)
        .map_err(|_| "file_xfer_read_failed".to_string())?
        .len() as usize;
    if payload_len <= ATTACHMENT_LEGACY_THRESHOLD_BYTES && !allow_legacy_sized {
        return Err("attachment_path_requires_large_file".to_string());
    }
    if payload_len > effective_limit {
        return Err("size_exceeds_max".to_string());
    }
    let mut journal = attachment_journal_load().map_err(|e| e.to_string())?;
    let (record_key, mut record) = match attachment_find_outbound_by_source(&journal, to, path) {
        Some((key, existing)) => (key, existing),
        None => {
            let fresh = attachment_build_outbound_record(to, service_url, path, receipt)
                .map_err(|e| e.to_string())?;
            let key = attachment_record_key("out", to, &fresh.attachment_id);
            (key, fresh)
        }
    };
    if record.part_count as usize > effective_max_parts {
        return Err("chunk_count_exceeds_max".to_string());
    }
    if record.state == "PEER_CONFIRMED" || record.state == "AWAITING_CONFIRMATION" {
        return Err("attachment_send_inflight".to_string());
    }
    record.service_url = Some(service_url.to_string());
    record.target_device_id = Some(short_device_marker(&routing.device_id));
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;

    if record.session_ref.is_none() {
        let created = attachment_service_create_session(service_url, &record)?;
        record.session_ref = Some(created.session_ref);
        record.resume_token = Some(created.resume_token);
        record.state = "SESSION_CREATED".to_string();
        journal.records.insert(record_key.clone(), record.clone());
        attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    }

    record.state = "UPLOADING".to_string();
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    attachment_upload_missing_parts(service_url, &record)?;

    let session_ref = record
        .session_ref
        .clone()
        .ok_or_else(|| "attachment_session_missing".to_string())?;
    let resume_token = record
        .resume_token
        .clone()
        .ok_or_else(|| "attachment_resume_missing".to_string())?;
    let committed = attachment_service_commit(service_url, &session_ref, &resume_token, &record)?;
    record.locator_kind = Some(committed.locator_kind);
    record.locator_ref = Some(committed.locator_ref);
    record.fetch_capability = Some(committed.fetch_capability);
    record.expires_at_unix_s = Some(committed.expires_at_unix_s);
    record.confirm_handle = if record.confirm_requested {
        Some(attachment_confirm_handle(AttachmentConfirmHandleInput {
            attachment_id: &record.attachment_id,
            plaintext_len: record.plaintext_len,
            ciphertext_len: record.ciphertext_len,
            part_size_class: &record.part_size_class,
            part_count: record.part_count,
            integrity_alg: &record.integrity_alg,
            integrity_root: &record.integrity_root,
            retention_class: &record.retention_class,
            expires_at_unix_s: committed.expires_at_unix_s,
        }))
    } else {
        None
    };
    record.state = "COMMITTED".to_string();
    journal.records.insert(record_key.clone(), record.clone());
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    let short_id = file_delivery_short_id(&record.attachment_id);
    emit_marker(
        "attachment_service_commit",
        None,
        &[("id", short_id.as_str()), ("ok", "true")],
    );

    let descriptor = attachment_build_descriptor(&record).map_err(|e| e.to_string())?;
    let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload: descriptor,
        relay,
        injector: transport::fault_injector_from_env(),
        pad_cfg: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
        routing_override: None,
        tui_thread: None,
    });
    if let Some(code) = outcome.error_code {
        return Err(code.to_string());
    }
    record.timeline_id = latest_outbound_file_id(to).ok();
    record.state = if record.confirm_requested {
        "AWAITING_CONFIRMATION".to_string()
    } else {
        "ACCEPTED_BY_RELAY".to_string()
    };
    record.last_error = None;
    journal.records.insert(record_key, record);
    attachment_journal_save(&journal).map_err(|e| e.to_string())?;
    emit_cli_file_delivery_with_device(
        to,
        "accepted_by_relay",
        path.file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("attachment.bin"),
        Some(routing.device_id.as_str()),
    );
    if receipt.is_some() {
        emit_cli_file_delivery_with_device(
            to,
            "awaiting_confirmation",
            path.file_name()
                .and_then(|v| v.to_str())
                .unwrap_or("attachment.bin"),
            Some(routing.device_id.as_str()),
        );
    }
    Ok(())
}

fn attachment_record_matches_descriptor(
    record: &AttachmentTransferRecord,
    desc: &AttachmentDescriptorPayload,
) -> bool {
    record.attachment_id == desc.attachment_id
        && record.plaintext_len == desc.plaintext_len
        && record.ciphertext_len == desc.ciphertext_len
        && record.part_size_class == desc.part_size_class
        && record.part_count == desc.part_count
        && record.integrity_alg == desc.integrity_alg
        && record.integrity_root == desc.integrity_root
        && record.enc_ctx_alg == desc.enc_ctx_alg
        && record.enc_ctx_b64u == desc.enc_ctx_b64u
        && record.retention_class == desc.retention_class
        && record.locator_kind.as_deref() == Some(desc.locator_kind.as_str())
        && record.locator_ref.as_deref() == Some(desc.locator_ref.as_str())
        && record.fetch_capability.as_deref() == Some(desc.fetch_capability.as_str())
        && record.expires_at_unix_s == Some(desc.expires_at_unix_s)
        && record.confirm_requested == desc.confirm_requested
        && record.confirm_handle == desc.confirm_handle
}

fn attachment_validate_descriptor(
    desc: &AttachmentDescriptorPayload,
    max_file_size: usize,
    max_parts: usize,
) -> Result<(), &'static str> {
    if !attachment_is_lower_hex_len(&desc.attachment_id, 64) {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.plaintext_len == 0 || desc.ciphertext_len == 0 || desc.part_count == 0 {
        return Err("REJECT_ATT_DESC_MISSING_REQUIRED_FIELD");
    }
    if desc.plaintext_len as usize > max_file_size || desc.part_count as usize > max_parts {
        return Err("REJECT_ATT_DESC_POLICY");
    }
    if attachment_part_size_bytes(&desc.part_size_class).is_none() {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.integrity_alg != ATTACHMENT_INTEGRITY_ALG_V1
        || desc.locator_kind != ATTACHMENT_LOCATOR_KIND_V1
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.locator_ref.trim().is_empty() || desc.locator_ref.len() > 128 {
        return Err("REJECT_ATT_DESC_LOCATOR_PLACEMENT");
    }
    if desc.fetch_capability.len() < 32 || desc.fetch_capability.len() > 255 {
        return Err("REJECT_ATT_DESC_LOCATOR_PLACEMENT");
    }
    if desc.enc_ctx_alg != ATTACHMENT_ENC_CTX_ALG_V1 {
        return Err("REJECT_ATT_DESC_ENC_CTX");
    }
    let _ = attachment_decode_enc_ctx(&desc.enc_ctx_b64u)?;
    if desc.retention_class != "short"
        && desc.retention_class != "standard"
        && desc.retention_class != "extended"
    {
        return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
    }
    if desc.expires_at_unix_s <= attachment_now_unix_s() {
        return Err("REJECT_ATT_DESC_EXPIRED");
    }
    if desc.confirm_requested != desc.confirm_handle.is_some() {
        return Err("REJECT_ATT_DESC_MISSING_REQUIRED_FIELD");
    }
    if let Some(handle) = desc.confirm_handle.as_deref() {
        if !attachment_is_lower_hex_len(handle, 24) {
            return Err("REJECT_ATT_DESC_FIELD_DOMAIN");
        }
    }
    if let Some(name) = desc.filename_hint.as_deref() {
        let _ = attachment_validate_filename_hint(name)?;
    }
    let expected_part_count =
        attachment_part_count_for_plaintext(desc.plaintext_len, &desc.part_size_class)
            .ok_or("REJECT_ATT_DESC_INCONSISTENT_SHAPE")?;
    let expected_ciphertext_len =
        attachment_ciphertext_len_for_plaintext(desc.plaintext_len, expected_part_count)
            .ok_or("REJECT_ATT_DESC_INCONSISTENT_SHAPE")?;
    if expected_part_count != desc.part_count || expected_ciphertext_len != desc.ciphertext_len {
        return Err("REJECT_ATT_DESC_INCONSISTENT_SHAPE");
    }
    Ok(())
}

fn attachment_inbound_record_from_descriptor(
    peer: &str,
    service_url: Option<&str>,
    desc: &AttachmentDescriptorPayload,
) -> AttachmentTransferRecord {
    AttachmentTransferRecord {
        attachment_id: desc.attachment_id.clone(),
        peer: peer.to_string(),
        direction: "in".to_string(),
        service_url: service_url.map(|v| v.to_string()),
        state: "PENDING_FETCH".to_string(),
        plaintext_len: desc.plaintext_len,
        ciphertext_len: desc.ciphertext_len,
        part_size_class: desc.part_size_class.clone(),
        part_count: desc.part_count,
        integrity_alg: desc.integrity_alg.clone(),
        integrity_root: desc.integrity_root.clone(),
        retention_class: desc.retention_class.clone(),
        enc_ctx_alg: desc.enc_ctx_alg.clone(),
        enc_ctx_b64u: desc.enc_ctx_b64u.clone(),
        locator_kind: Some(desc.locator_kind.clone()),
        locator_ref: Some(desc.locator_ref.clone()),
        fetch_capability: Some(desc.fetch_capability.clone()),
        expires_at_unix_s: Some(desc.expires_at_unix_s),
        confirm_requested: desc.confirm_requested,
        confirm_handle: desc.confirm_handle.clone(),
        filename_hint: desc.filename_hint.clone(),
        media_type: desc.media_type.clone(),
        source_path: None,
        staged_ciphertext_rel: None,
        session_ref: None,
        resume_token: None,
        timeline_id: None,
        target_device_id: None,
        uploaded_parts: Vec::new(),
        downloaded_ciphertext_bytes: 0,
        download_ciphertext_rel: Some(attachment_inbound_rel(&desc.attachment_id)),
        download_output_name: Some(
            desc.filename_hint
                .as_deref()
                .and_then(|v| attachment_validate_filename_hint(v).ok())
                .unwrap_or_else(|| {
                    format!(
                        "attachment-{}.bin",
                        file_delivery_short_id(&desc.attachment_id)
                    )
                }),
        ),
        last_error: None,
    }
}

enum AttachmentFetchOutcome {
    Complete(u64),
    Interrupted(u64),
}

fn attachment_fetch_ciphertext(
    service_url: &str,
    record: &AttachmentTransferRecord,
    cfg_dir: &Path,
) -> Result<AttachmentFetchOutcome, String> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or_else(|| "attachment_stage_missing".to_string())?;
    let path = attachment_path_from_rel(cfg_dir, rel).map_err(|e| e.to_string())?;
    if let Some(parent) = path.parent() {
        ensure_dir_secure(parent, ConfigSource::EnvOverride)
            .map_err(|_| "attachment_stage_unavailable".to_string())?;
    }
    let mut existing_len = fs::metadata(&path).map(|v| v.len()).unwrap_or(0);
    let locator_ref = record
        .locator_ref
        .as_deref()
        .ok_or_else(|| "REJECT_ATT_DESC_LOCATOR_PLACEMENT".to_string())?;
    let fetch_capability = env::var("QSC_ATTACHMENT_FETCH_CAPABILITY_OVERRIDE")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .or_else(|| record.fetch_capability.clone())
        .ok_or_else(|| "REJECT_ATT_DESC_LOCATOR_PLACEMENT".to_string())?;
    let url = format!("{service_url}/v1/attachments/objects/{locator_ref}");
    let client = HttpClient::new();
    let mut req = client
        .get(url)
        .header("X-QATT-Fetch-Capability", fetch_capability);
    if existing_len > 0 && existing_len < record.ciphertext_len {
        req = req.header(
            "Range",
            format!("bytes={existing_len}-{}", record.ciphertext_len - 1),
        );
    }
    let mut response = req
        .send()
        .map_err(|_| "attachment_fetch_failed".to_string())?;
    if !(response.status().is_success() || response.status() == HttpStatus::PARTIAL_CONTENT) {
        return Err(attachment_service_reason(
            response,
            "attachment_fetch_failed",
        ));
    }
    let restart = existing_len > 0 && response.status() == HttpStatus::OK;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(!restart && existing_len > 0)
        .truncate(restart || existing_len == 0)
        .open(&path)
        .map_err(|_| "attachment_stage_unavailable".to_string())?;
    #[cfg(unix)]
    enforce_file_perms(&path).map_err(|_| "attachment_stage_unavailable".to_string())?;
    if restart {
        existing_len = 0;
    }
    let abort_after = env::var("QSC_ATTACHMENT_TEST_ABORT_AFTER_FETCH_BYTES")
        .ok()
        .and_then(|v| v.parse::<u64>().ok());
    let mut downloaded = existing_len;
    let mut buf = [0u8; 8192];
    loop {
        let n = response
            .read(&mut buf)
            .map_err(|_| "attachment_fetch_failed".to_string())?;
        if n == 0 {
            break;
        }
        file.write_all(&buf[..n])
            .map_err(|_| "attachment_stage_unavailable".to_string())?;
        downloaded = downloaded.saturating_add(n as u64);
        if abort_after.is_some_and(|limit| downloaded >= limit) {
            file.sync_all()
                .map_err(|_| "attachment_stage_unavailable".to_string())?;
            return Ok(AttachmentFetchOutcome::Interrupted(downloaded));
        }
    }
    file.sync_all()
        .map_err(|_| "attachment_stage_unavailable".to_string())?;
    Ok(AttachmentFetchOutcome::Complete(downloaded))
}

fn attachment_verify_ciphertext_root(
    cfg_dir: &Path,
    record: &AttachmentTransferRecord,
) -> Result<(), &'static str> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    let path = attachment_path_from_rel(cfg_dir, rel)?;
    let actual_len = fs::metadata(&path)
        .map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?
        .len();
    if actual_len != record.ciphertext_len {
        return Err("REJECT_ATT_CIPHERTEXT_PRECHECK");
    }
    let mut file = File::open(path).map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    let mut leaves = Vec::with_capacity(record.part_count as usize);
    for part_index in 0..record.part_count {
        let expected_len = attachment_ciphertext_part_len(
            part_index,
            record.plaintext_len,
            &record.part_size_class,
            record.part_count,
            record.ciphertext_len,
        )
        .ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
        let mut buf = vec![0u8; expected_len];
        file.read_exact(&mut buf)
            .map_err(|_| "REJECT_ATT_CIPHERTEXT_PRECHECK")?;
        leaves.push(attachment_merkle_leaf(part_index, &buf));
    }
    let root = attachment_merkle_root(leaves).ok_or("REJECT_ATT_CIPHERTEXT_PRECHECK")?;
    if root != record.integrity_root {
        return Err("REJECT_ATT_CIPHERTEXT_PRECHECK");
    }
    Ok(())
}

fn attachment_decrypt_to_output(
    cfg_dir: &Path,
    out_dir: &Path,
    source: ConfigSource,
    record: &AttachmentTransferRecord,
) -> Result<(), &'static str> {
    let rel = record
        .download_ciphertext_rel
        .as_deref()
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    let ciphertext_path = attachment_path_from_rel(cfg_dir, rel)?;
    let mut src = File::open(ciphertext_path).map_err(|_| "REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    let (mut cek, nonce_prefix) = attachment_decode_enc_ctx(&record.enc_ctx_b64u)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(&cek));
    let output_name = record
        .download_output_name
        .clone()
        .unwrap_or_else(|| attachment_output_name(record));
    let final_path = out_dir.join(output_name);
    enforce_safe_parents(&final_path, source).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    let tmp_path = out_dir.join(format!(
        ".{}.tmp.{}",
        file_delivery_short_id(&record.attachment_id),
        process::id()
    ));
    let _ = fs::remove_file(&tmp_path);
    let mut dst = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp_path)
        .map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    #[cfg(unix)]
    enforce_file_perms(&tmp_path).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    let mut plaintext_len = 0u64;
    for part_index in 0..record.part_count {
        let ct_len = attachment_ciphertext_part_len(
            part_index,
            record.plaintext_len,
            &record.part_size_class,
            record.part_count,
            record.ciphertext_len,
        )
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
        let mut ciphertext = vec![0u8; ct_len];
        src.read_exact(&mut ciphertext)
            .map_err(|_| "REJECT_ATT_DECRYPT_AUTH")?;
        let nonce = attachment_nonce(&nonce_prefix, part_index);
        let aad = attachment_part_aad(
            &record.attachment_id,
            &record.enc_ctx_alg,
            record.plaintext_len,
            record.ciphertext_len,
            &record.part_size_class,
            record.part_count,
            part_index,
        );
        let plaintext = cipher
            .decrypt(
                Nonce::from_slice(&nonce),
                Payload {
                    msg: &ciphertext,
                    aad: &aad,
                },
            )
            .map_err(|_| "REJECT_ATT_DECRYPT_AUTH")?;
        plaintext_len = plaintext_len.saturating_add(plaintext.len() as u64);
        dst.write_all(&plaintext)
            .map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    }
    dst.sync_all().map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    if plaintext_len != record.plaintext_len {
        let _ = fs::remove_file(&tmp_path);
        return Err("REJECT_ATT_PLAINTEXT_SHAPE");
    }
    fs::rename(&tmp_path, &final_path).map_err(|_| "REJECT_ATT_PLAINTEXT_SHAPE")?;
    cek.zeroize();
    Ok(())
}

fn attachment_process_inbound_record(
    ctx: &ReceivePullCtx<'_>,
    record_key: &str,
) -> Result<Option<(String, String)>, &'static str> {
    let Some(service_url) = ctx.attachment_service else {
        return Ok(None);
    };
    let mut journal = attachment_journal_load()?;
    let mut record = journal
        .records
        .get(record_key)
        .cloned()
        .ok_or("REJECT_ATT_DECRYPT_CTX_MISMATCH")?;
    record.service_url = Some(service_url.to_string());
    let (cfg_dir, _) = config_dir().map_err(|_| "attachment_stage_unavailable")?;
    match attachment_fetch_ciphertext(service_url, &record, &cfg_dir) {
        Ok(AttachmentFetchOutcome::Interrupted(downloaded)) => {
            record.downloaded_ciphertext_bytes = downloaded;
            record.state = "DOWNLOADING".to_string();
            journal.records.insert(record_key.to_string(), record);
            attachment_journal_save(&journal)?;
            return Err("attachment_test_interrupt_download");
        }
        Ok(AttachmentFetchOutcome::Complete(downloaded)) => {
            record.downloaded_ciphertext_bytes = downloaded;
            record.state = "FETCHED".to_string();
        }
        Err(code) => {
            record.last_error = Some(code.clone());
            journal.records.insert(record_key.to_string(), record);
            attachment_journal_save(&journal)?;
            return Err(Box::leak(code.into_boxed_str()));
        }
    }
    journal
        .records
        .insert(record_key.to_string(), record.clone());
    attachment_journal_save(&journal)?;
    attachment_verify_ciphertext_root(&cfg_dir, &record)?;
    attachment_decrypt_to_output(&cfg_dir, ctx.out, ctx.source, &record)?;
    record.state = "VERIFIED".to_string();
    record.last_error = None;
    journal
        .records
        .insert(record_key.to_string(), record.clone());
    attachment_journal_save(&journal)?;
    timeline_append_entry(
        ctx.from,
        "in",
        record.plaintext_len as usize,
        "file",
        MessageState::Received,
        None,
    )?;
    if record.confirm_requested
        && ctx.receipt_policy.file_confirm_mode == FileConfirmEmitMode::CompleteOnly
    {
        return Ok(Some((
            record.attachment_id.clone(),
            record
                .confirm_handle
                .clone()
                .ok_or("REJECT_ATT_CONFIRM_EARLY")?,
        )));
    }
    Ok(None)
}

pub(super) fn attachment_handle_descriptor(
    ctx: &ReceivePullCtx<'_>,
    desc: AttachmentDescriptorPayload,
) -> Result<Option<(String, String)>, &'static str> {
    attachment_validate_descriptor(&desc, ctx.file_max_size, ctx.file_max_chunks)?;
    let mut journal = attachment_journal_load()?;
    let key = attachment_record_key("in", ctx.from, &desc.attachment_id);
    let mut record = match journal.records.get(&key).cloned() {
        Some(existing) => {
            if !attachment_record_matches_descriptor(&existing, &desc) {
                return Err("REJECT_ATT_DECRYPT_CTX_MISMATCH");
            }
            existing
        }
        None => attachment_inbound_record_from_descriptor(ctx.from, ctx.attachment_service, &desc),
    };
    record.service_url = ctx.attachment_service.map(|v| v.to_string());
    record.state = "PENDING_FETCH".to_string();
    journal.records.insert(key.clone(), record);
    attachment_journal_save(&journal)?;
    if ctx.attachment_service.is_none() {
        emit_marker(
            "attachment_pending_service",
            None,
            &[
                ("id", file_delivery_short_id(&desc.attachment_id).as_str()),
                ("ok", "true"),
            ],
        );
        return Ok(None);
    }
    attachment_process_inbound_record(ctx, &key)
}

pub(super) fn attachment_resume_pending_for_peer(
    ctx: &ReceivePullCtx<'_>,
    service_url: &str,
) -> Result<usize, &'static str> {
    let journal = attachment_journal_load()?;
    let pending: Vec<String> = journal
        .records
        .iter()
        .filter(|(_, rec)| {
            rec.direction == "in"
                && rec.peer == ctx.from
                && rec.service_url.as_deref().unwrap_or(service_url) == service_url
                && matches!(
                    rec.state.as_str(),
                    "PENDING_FETCH" | "DOWNLOADING" | "FETCHED"
                )
        })
        .map(|(key, _)| key.clone())
        .collect();
    drop(journal);
    let mut resumed = 0usize;
    for key in pending {
        if let Some((attachment_id, confirm_handle)) = attachment_process_inbound_record(ctx, &key)?
        {
            resumed = resumed.saturating_add(1);
            let item = PendingReceipt::AttachmentComplete {
                attachment_id,
                confirm_handle,
            };
            match ctx.receipt_policy.mode {
                ReceiptEmitMode::Off => {
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "skipped",
                        "attachment_complete",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "skipped",
                        "attachment_complete",
                        ctx.from,
                    );
                }
                ReceiptEmitMode::Immediate | ReceiptEmitMode::Batched => {
                    send_pending_receipt(ctx, item);
                }
            }
        }
    }
    Ok(resumed)
}

pub(super) fn file_xfer_reject(id: &str, reason: &str) -> ! {
    emit_marker(
        "file_xfer_reject",
        Some(reason),
        &[("id", id), ("reason", reason)],
    );
    print_error_marker(reason);
}

pub(super) fn file_xfer_store_key(peer: &str, file_id: &str) -> String {
    format!("{}:{}", peer, file_id)
}

const FILE_PUSH_MAX_ATTEMPTS: usize = 3;
const FILE_PUSH_RETRY_BASE_BACKOFF_MS: u64 = 50;

fn file_push_retryable(code: &str) -> bool {
    matches!(
        code,
        "relay_inbox_push_failed"
            | "relay_inbox_queue_full"
            | "relay_network_timeout"
            | "relay_network_unreachable"
            | "relay_http_failure"
    )
}

fn emit_file_push_retry(attempt: usize, backoff_ms: u64, reason: &str) {
    let attempt_s = attempt.to_string();
    let backoff_s = backoff_ms.to_string();
    emit_marker(
        "file_push_retry",
        None,
        &[
            ("attempt", attempt_s.as_str()),
            ("backoff_ms", backoff_s.as_str()),
            ("reason", reason),
        ],
    );
    emit_cli_named_marker(
        "QSC_FILE_PUSH_RETRY",
        &[
            ("attempt", attempt_s.as_str()),
            ("backoff_ms", backoff_s.as_str()),
            ("reason", reason),
        ],
    );
}

pub(super) fn emit_file_integrity_fail(reason: &str, action: &str) {
    emit_cli_named_marker(
        "QSC_FILE_INTEGRITY_FAIL",
        &[("reason", reason), ("action", action)],
    );
    emit_tui_named_marker(
        "QSC_TUI_FILE_INTEGRITY_FAIL",
        &[("reason", reason), ("action", action)],
    );
}

pub(super) fn file_transfer_fail_clean(
    peer: &str,
    file_id: &str,
    reason: &str,
) -> Result<(), &'static str> {
    let key = file_xfer_store_key(peer, file_id);
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    if let Some(rec) = store.file_transfers.get_mut(&key) {
        rec.state = "FAILED".to_string();
        rec.chunk_hashes.clear();
        rec.chunks_hex.clear();
        rec.confirm_requested = false;
        rec.confirm_id = None;
        timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
        emit_file_integrity_fail(reason, "purge_partials");
        emit_marker(
            "file_xfer_fail_clean",
            None,
            &[
                ("id", file_id),
                ("reason", reason),
                ("action", "purge_partials"),
            ],
        );
        return Ok(());
    }
    emit_file_integrity_fail(reason, "rotate_mailbox_hint");
    Ok(())
}

fn relay_send_file_payload_with_retry(to: &str, payload: Vec<u8>, relay: &str) -> RelaySendOutcome {
    let mut attempt = 1usize;
    loop {
        let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
            to,
            payload: payload.clone(),
            relay,
            injector: transport::fault_injector_from_env(),
            pad_cfg: None,
            bucket_max: None,
            meta_seed: None,
            receipt: None,
            routing_override: None,
            tui_thread: None,
        });
        let Some(code) = outcome.error_code else {
            return outcome;
        };
        if !file_push_retryable(code) || attempt >= FILE_PUSH_MAX_ATTEMPTS {
            return outcome;
        }
        let backoff_ms = FILE_PUSH_RETRY_BASE_BACKOFF_MS * (1u64 << (attempt - 1));
        emit_file_push_retry(attempt, backoff_ms, code);
        std::thread::sleep(Duration::from_millis(backoff_ms));
        attempt += 1;
    }
}

pub(super) struct FileSendExec<'a> {
    pub(super) transport: Option<SendTransport>,
    pub(super) relay: Option<&'a str>,
    pub(super) attachment_service: Option<&'a str>,
    pub(super) legacy_in_message_stage: Option<LegacyInMessageStage>,
    pub(super) to: &'a str,
    pub(super) path: &'a Path,
    pub(super) chunk_size: usize,
    pub(super) max_file_size: Option<usize>,
    pub(super) max_chunks: Option<usize>,
    pub(super) receipt: Option<ReceiptKind>,
}

pub(super) fn file_send_execute(args: FileSendExec<'_>) {
    let FileSendExec {
        transport,
        relay,
        attachment_service,
        legacy_in_message_stage,
        to,
        path,
        chunk_size,
        max_file_size,
        max_chunks,
        receipt,
    } = args;
    if !require_unlocked("file_send") {
        return;
    }
    let legacy_in_message_stage = resolve_legacy_in_message_stage(legacy_in_message_stage)
        .unwrap_or_else(|code| file_xfer_reject("unknown", code));
    let path_len_hint = fs::metadata(path)
        .map(|v| v.len() as usize)
        .unwrap_or_else(|_| file_xfer_reject("unknown", "file_xfer_read_failed"));
    let size_class = if path_len_hint > ATTACHMENT_LEGACY_THRESHOLD_BYTES {
        "above_threshold"
    } else {
        "legacy_sized"
    };
    let use_attachment_path = path_len_hint > ATTACHMENT_LEGACY_THRESHOLD_BYTES
        || matches!(
            legacy_in_message_stage,
            LegacyInMessageStage::W1 | LegacyInMessageStage::W2
        );
    let path_kind = if use_attachment_path {
        "attachment"
    } else {
        "legacy_in_message"
    };
    emit_marker(
        "file_send_policy",
        None,
        &[
            (
                "stage",
                legacy_in_message_stage_name(legacy_in_message_stage),
            ),
            ("size_class", size_class),
            ("path", path_kind),
        ],
    );
    if use_attachment_path {
        let service_url = resolve_large_file_attachment_service(attachment_service)
            .unwrap_or_else(|code| file_xfer_reject("unknown", code));
        if chunk_size != FILE_XFER_DEFAULT_CHUNK_SIZE {
            file_xfer_reject("unknown", "attachment_chunk_flag_invalid");
        }
        let transport = match transport {
            Some(v) => v,
            None => file_xfer_reject("unknown", "file_xfer_transport_required"),
        };
        match transport {
            SendTransport::Relay => {}
        }
        let relay = match relay {
            Some(v) => v,
            None => file_xfer_reject("unknown", "file_xfer_relay_required"),
        };
        if let Err(code) = attachment_send_execute(AttachmentSendExec {
            to,
            path,
            relay,
            service_url: service_url.as_str(),
            allow_legacy_sized: path_len_hint <= ATTACHMENT_LEGACY_THRESHOLD_BYTES,
            max_file_size,
            max_parts: max_chunks,
            receipt,
        }) {
            file_xfer_reject("unknown", code.as_str());
        }
        return;
    }
    let transport = match transport {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_transport_required"),
    };
    match transport {
        SendTransport::Relay => {}
    }
    let relay = match relay {
        Some(v) => v,
        None => file_xfer_reject("unknown", "file_xfer_relay_required"),
    };
    if !channel_label_ok(to) {
        file_xfer_reject("unknown", "file_xfer_peer_invalid");
    }
    let max_file_size = max_file_size.unwrap_or(FILE_XFER_DEFAULT_MAX_FILE_SIZE);
    let max_chunks = max_chunks.unwrap_or(FILE_XFER_DEFAULT_MAX_CHUNKS);
    if max_file_size == 0 || max_file_size > FILE_XFER_MAX_FILE_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_size_bound_invalid");
    }
    if chunk_size == 0 || chunk_size > FILE_XFER_MAX_CHUNK_SIZE_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunk_bound_invalid");
    }
    if max_chunks == 0 || max_chunks > FILE_XFER_MAX_CHUNKS_CEILING {
        file_xfer_reject("unknown", "file_xfer_chunks_bound_invalid");
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        file_xfer_reject("unknown", code);
    }
    let payload =
        fs::read(path).unwrap_or_else(|_| file_xfer_reject("unknown", "file_xfer_read_failed"));
    if payload.is_empty() {
        file_xfer_reject("unknown", "file_xfer_empty");
    }
    if payload.len() > max_file_size {
        file_xfer_reject("unknown", "size_exceeds_max");
    }
    let chunk_count = payload.len().div_ceil(chunk_size);
    if chunk_count > max_chunks {
        file_xfer_reject("unknown", "chunk_count_exceeds_max");
    }
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        file_xfer_reject("unknown", code);
    }
    let routing = match resolve_send_routing_target(to) {
        Ok(v) => v,
        Err(code) => file_xfer_reject("unknown", code),
    };
    if let Err(reason) = protocol_active_or_reason_for_peer(routing.channel.as_str()) {
        emit_marker(
            "file_xfer_reject",
            Some("protocol_inactive"),
            &[
                ("id", "unknown"),
                ("reason", "protocol_inactive"),
                ("detail", reason.as_str()),
            ],
        );
        protocol_inactive_exit(reason.as_str());
    }
    let filename = path
        .file_name()
        .and_then(|v| v.to_str())
        .unwrap_or("file.bin")
        .to_string();
    let file_id = file_xfer_id(to, filename.as_str(), &payload);
    let size_s = payload.len().to_string();
    emit_marker(
        "file_xfer_prepare",
        None,
        &[
            ("id", file_id.as_str()),
            ("size", size_s.as_str()),
            ("ok", "true"),
        ],
    );
    let mut chunk_hashes = Vec::with_capacity(chunk_count);
    for idx in 0..chunk_count {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = &payload[start..end];
        chunk_hashes.push(file_xfer_chunk_hash(chunk));
    }
    let manifest_hash = file_xfer_manifest_hash(
        file_id.as_str(),
        payload.len(),
        chunk_count,
        chunk_hashes.as_slice(),
    );
    let confirm_requested = receipt.is_some();
    let confirm_id = file_xfer_confirm_id(file_id.as_str(), manifest_hash.as_str());

    for (idx, chunk_hash) in chunk_hashes.iter().enumerate() {
        let start = idx * chunk_size;
        let end = (start + chunk_size).min(payload.len());
        let chunk = payload[start..end].to_vec();
        let chunk_payload = FileTransferChunkPayload {
            v: FILE_XFER_VERSION,
            t: "file_chunk".to_string(),
            file_id: file_id.clone(),
            filename: filename.clone(),
            total_size: payload.len(),
            chunk_index: idx,
            chunk_count,
            chunk_hash: chunk_hash.clone(),
            manifest_hash: manifest_hash.clone(),
            chunk,
        };
        let body = serde_json::to_vec(&chunk_payload)
            .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
        let outcome = relay_send_file_payload_with_retry(to, body, relay);
        if let Some(code) = outcome.error_code {
            file_xfer_reject(file_id.as_str(), code);
        }
        let idx_s = idx.to_string();
        emit_marker(
            "file_xfer_chunk",
            None,
            &[
                ("id", file_id.as_str()),
                ("idx", idx_s.as_str()),
                ("ok", "true"),
            ],
        );
    }

    let manifest = FileTransferManifestPayload {
        v: FILE_XFER_VERSION,
        t: "file_manifest".to_string(),
        file_id: file_id.clone(),
        filename,
        total_size: payload.len(),
        chunk_count,
        chunk_hashes,
        manifest_hash,
        confirm_requested,
        confirm_id: confirm_id.clone(),
    };
    let manifest_body = serde_json::to_vec(&manifest)
        .unwrap_or_else(|_| file_xfer_reject(file_id.as_str(), "file_xfer_encode_failed"));
    let outcome = relay_send_file_payload_with_retry(to, manifest_body, relay);
    if let Some(code) = outcome.error_code {
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
    if let Err(code) = timeline_append_entry_for_target(
        to,
        "out",
        payload.len(),
        "file",
        MessageState::Sent,
        Some(file_id.as_str()),
        Some(routing.device_id.as_str()),
    ) {
        emit_message_state_reject(file_id.as_str(), code);
        file_xfer_reject(file_id.as_str(), code);
    }
    let outbound = FileTransferRecord {
        id: file_id.clone(),
        peer: to.to_string(),
        filename: manifest.filename.clone(),
        total_size: payload.len(),
        chunk_count,
        manifest_hash: manifest.manifest_hash.clone(),
        chunk_hashes: Vec::new(),
        chunks_hex: Vec::new(),
        confirm_requested,
        confirm_id: if confirm_requested {
            Some(confirm_id.clone())
        } else {
            None
        },
        target_device_id: Some(short_device_marker(routing.device_id.as_str())),
        state: if confirm_requested {
            "AWAITING_CONFIRMATION".to_string()
        } else {
            "ACCEPTED_BY_RELAY".to_string()
        },
    };
    if let Err(code) = file_transfer_upsert_outbound_record(to, file_id.as_str(), outbound) {
        file_xfer_reject(file_id.as_str(), code);
    }
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", file_id.as_str()), ("ok", "true")],
    );
    emit_cli_confirm_policy();
    emit_cli_file_delivery_with_device(
        to,
        "accepted_by_relay",
        file_id.as_str(),
        Some(routing.device_id.as_str()),
    );
    if confirm_requested {
        emit_cli_file_delivery_with_device(
            to,
            "awaiting_confirmation",
            file_id.as_str(),
            Some(routing.device_id.as_str()),
        );
    }
}

pub(super) fn file_transfer_handle_chunk(
    ctx: &ReceivePullCtx<'_>,
    chunk: FileTransferChunkPayload,
) -> Result<(), &'static str> {
    if chunk.total_size == 0 || chunk.total_size > ctx.file_max_size {
        return Err("size_exceeds_max");
    }
    if chunk.chunk_count == 0 || chunk.chunk_count > ctx.file_max_chunks {
        return Err("chunk_count_exceeds_max");
    }
    if chunk.chunk.len() > FILE_XFER_DEFAULT_CHUNK_SIZE {
        return Err("chunk_size_exceeds_max");
    }
    if chunk.chunk_index >= chunk.chunk_count {
        return Err("chunk_index_invalid");
    }
    if chunk.chunk_hash != file_xfer_chunk_hash(&chunk.chunk) {
        return Err("chunk_hash_invalid");
    }
    let key = file_xfer_store_key(ctx.from, chunk.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .entry(key)
        .or_insert_with(|| FileTransferRecord {
            id: chunk.file_id.clone(),
            peer: ctx.from.to_string(),
            filename: chunk.filename.clone(),
            total_size: chunk.total_size,
            chunk_count: chunk.chunk_count,
            manifest_hash: chunk.manifest_hash.clone(),
            chunk_hashes: Vec::new(),
            chunks_hex: Vec::new(),
            confirm_requested: false,
            confirm_id: None,
            target_device_id: None,
            state: "RECEIVING".to_string(),
        });
    if rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if chunk.chunk_index == 0 {
        if rec.state == "FAILED" {
            rec.filename = chunk.filename.clone();
            rec.total_size = chunk.total_size;
            rec.chunk_count = chunk.chunk_count;
            rec.manifest_hash = chunk.manifest_hash.clone();
            rec.chunk_hashes.clear();
            rec.chunks_hex.clear();
            rec.confirm_requested = false;
            rec.confirm_id = None;
            rec.state = "RECEIVING".to_string();
            emit_marker(
                "file_xfer_reset",
                None,
                &[("id", chunk.file_id.as_str()), ("reason", "rerun_detected")],
            );
        }
    } else if rec.state == "FAILED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != chunk.total_size
        || rec.chunk_count != chunk.chunk_count
        || rec.manifest_hash != chunk.manifest_hash
    {
        return Err("chunk_meta_mismatch");
    }
    let expected = rec.chunks_hex.len();
    if chunk.chunk_index != expected {
        return Err("chunk_order_invalid");
    }
    rec.chunk_hashes.push(chunk.chunk_hash.clone());
    rec.chunks_hex.push(hex_encode(&chunk.chunk));
    rec.state = "RECEIVING".to_string();
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    let idx_s = chunk.chunk_index.to_string();
    emit_marker(
        "file_xfer_chunk",
        None,
        &[
            ("id", chunk.file_id.as_str()),
            ("idx", idx_s.as_str()),
            ("ok", "true"),
        ],
    );
    Ok(())
}

pub(super) fn file_transfer_handle_manifest(
    ctx: &ReceivePullCtx<'_>,
    manifest: FileTransferManifestPayload,
) -> Result<Option<(String, String)>, &'static str> {
    if manifest.total_size == 0 || manifest.total_size > ctx.file_max_size {
        return Err("size_exceeds_max");
    }
    if manifest.chunk_count == 0 || manifest.chunk_count > ctx.file_max_chunks {
        return Err("chunk_count_exceeds_max");
    }
    let key = file_xfer_store_key(ctx.from, manifest.file_id.as_str());
    let mut store = timeline_store_load().map_err(|_| "timeline_unavailable")?;
    let rec = store
        .file_transfers
        .get_mut(&key)
        .ok_or("manifest_missing_chunks")?;
    if rec.state == "FAILED" || rec.state == "VERIFIED" {
        return Err("state_invalid_transition");
    }
    if rec.total_size != manifest.total_size
        || rec.chunk_count != manifest.chunk_count
        || rec.filename != manifest.filename
    {
        return Err("manifest_meta_mismatch");
    }
    if rec.chunks_hex.len() != rec.chunk_count {
        return Err("manifest_missing_chunks");
    }
    if manifest.chunk_hashes.len() != rec.chunk_count {
        return Err("manifest_chunk_count_mismatch");
    }
    let expected_manifest = file_xfer_manifest_hash(
        manifest.file_id.as_str(),
        manifest.total_size,
        manifest.chunk_count,
        manifest.chunk_hashes.as_slice(),
    );
    if expected_manifest != manifest.manifest_hash || rec.manifest_hash != manifest.manifest_hash {
        return Err("manifest_mismatch");
    }
    if rec.chunk_hashes != manifest.chunk_hashes {
        return Err("manifest_mismatch");
    }
    let mut reconstructed = Vec::new();
    for (idx, chunk_hex) in rec.chunks_hex.iter().enumerate() {
        let chunk = hex_decode(chunk_hex).map_err(|_| "chunk_decode_failed")?;
        if file_xfer_chunk_hash(&chunk) != manifest.chunk_hashes[idx] {
            return Err("chunk_hash_invalid");
        }
        reconstructed.extend_from_slice(&chunk);
    }
    if reconstructed.len() != manifest.total_size {
        return Err("manifest_size_mismatch");
    }
    rec.state = "VERIFIED".to_string();
    rec.confirm_requested = manifest.confirm_requested;
    rec.confirm_id = if manifest.confirm_requested {
        Some(manifest.confirm_id.clone())
    } else {
        None
    };
    timeline_store_save(&store).map_err(|_| "timeline_unavailable")?;
    timeline_append_entry(
        ctx.from,
        "in",
        reconstructed.len(),
        "file",
        MessageState::Received,
        Some(manifest.file_id.as_str()),
    )?;
    emit_marker(
        "file_xfer_manifest",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    emit_marker(
        "file_xfer_complete",
        None,
        &[("id", manifest.file_id.as_str()), ("ok", "true")],
    );
    if manifest.confirm_requested {
        if ctx.receipt_policy.file_confirm_mode == FileConfirmEmitMode::CompleteOnly {
            return Ok(Some((manifest.file_id, manifest.confirm_id)));
        }
        emit_cli_receipt_policy_event(
            ctx.receipt_policy.mode,
            "skipped",
            "file_complete",
            ctx.from,
        );
        emit_tui_receipt_policy_event(
            ctx.receipt_policy.mode,
            "skipped",
            "file_complete",
            ctx.from,
        );
    }
    Ok(None)
}

pub(super) fn build_file_completion_ack(file_id: &str, confirm_id: &str) -> Vec<u8> {
    let ack = FileConfirmPayload {
        v: 1,
        t: "ack".to_string(),
        kind: "file_confirmed".to_string(),
        file_id: file_id.to_string(),
        confirm_id: confirm_id.to_string(),
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

pub(super) fn build_attachment_completion_ack(
    attachment_id: &str,
    confirm_handle: &str,
) -> Vec<u8> {
    let ack = AttachmentConfirmPayload {
        v: 1,
        t: "ack".to_string(),
        kind: ATTACHMENT_CONFIRM_KIND.to_string(),
        attachment_id: attachment_id.to_string(),
        confirm_handle: confirm_handle.to_string(),
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

pub(super) fn validated_attachment_service_from_env() -> Option<String> {
    relay_trimmed_nonempty(env::var(QSC_ATTACHMENT_SERVICE_ENV).ok())
}

pub(super) fn legacy_in_message_stage_name(stage: LegacyInMessageStage) -> &'static str {
    match stage {
        LegacyInMessageStage::W0 => "w0",
        LegacyInMessageStage::W1 | LegacyInMessageStage::W2 => "w2",
    }
}

pub(super) fn legacy_receive_mode_name(mode: LegacyReceiveMode) -> &'static str {
    match mode {
        LegacyReceiveMode::Coexistence => "coexistence",
        LegacyReceiveMode::Retired => "retired",
    }
}

fn validated_legacy_in_message_stage_from_env() -> Result<Option<LegacyInMessageStage>, &'static str>
{
    let Some(raw) = relay_trimmed_nonempty(env::var(QSC_LEGACY_IN_MESSAGE_STAGE_ENV).ok()) else {
        return Ok(None);
    };
    match raw.to_ascii_lowercase().as_str() {
        "w0" => Ok(Some(LegacyInMessageStage::W0)),
        "w1" => Ok(Some(LegacyInMessageStage::W1)),
        "w2" => Ok(Some(LegacyInMessageStage::W2)),
        _ => Err("legacy_in_message_stage_invalid"),
    }
}

pub(super) fn resolve_legacy_in_message_stage(
    explicit_stage: Option<LegacyInMessageStage>,
) -> Result<LegacyInMessageStage, &'static str> {
    let env_stage = validated_legacy_in_message_stage_from_env()?;
    if validated_attachment_service_from_env().is_some() {
        let selected = explicit_stage.or(env_stage);
        return match selected {
            Some(LegacyInMessageStage::W0 | LegacyInMessageStage::W1) => {
                Err("legacy_in_message_stage_retired_post_w0")
            }
            Some(LegacyInMessageStage::W2) | None => Ok(LegacyInMessageStage::W2),
        };
    }
    if let Some(stage) = explicit_stage {
        return Ok(stage);
    }
    if let Some(stage) = env_stage {
        return Ok(stage);
    }
    Ok(LegacyInMessageStage::W0)
}

pub(super) fn resolve_legacy_receive_mode(
    explicit_mode: Option<LegacyReceiveMode>,
    attachment_service: Option<&str>,
) -> Result<LegacyReceiveMode, &'static str> {
    if attachment_service.is_some() {
        return match explicit_mode {
            Some(LegacyReceiveMode::Coexistence) => Err("legacy_receive_mode_retired_post_w0"),
            Some(LegacyReceiveMode::Retired) | None => Ok(LegacyReceiveMode::Retired),
        };
    }
    Ok(explicit_mode.unwrap_or(LegacyReceiveMode::Coexistence))
}

pub(super) fn resolve_large_file_attachment_service(
    explicit_attachment_service: Option<&str>,
) -> Result<String, &'static str> {
    let raw = explicit_attachment_service
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .or_else(validated_attachment_service_from_env)
        .ok_or("attachment_service_required")?;
    normalize_relay_endpoint(raw.as_str())
}
