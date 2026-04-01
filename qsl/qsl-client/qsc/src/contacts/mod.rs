use super::*;

pub(super) fn route_token_hash8(token: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(token.as_bytes());
    hex_encode(&hash[..4])
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum TrustOnboardingMode {
    Strict,
    Balanced,
}

impl TrustOnboardingMode {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Strict => "strict",
            Self::Balanced => "balanced",
        }
    }

    pub(super) fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "strict" => Some(Self::Strict),
            "balanced" => Some(Self::Balanced),
            _ => None,
        }
    }

    pub(super) fn from_arg(value: TrustMode) -> Self {
        match value {
            TrustMode::Strict => Self::Strict,
            TrustMode::Balanced => Self::Balanced,
        }
    }
}

pub(super) fn load_trust_onboarding_mode_from_account() -> TrustOnboardingMode {
    if !vault_unlocked() {
        return TrustOnboardingMode::Balanced;
    }
    account_secret_trimmed(TUI_TRUST_MODE_SECRET_KEY)
        .as_deref()
        .and_then(TrustOnboardingMode::from_raw)
        .unwrap_or(TrustOnboardingMode::Balanced)
}

pub(super) fn normalize_route_token(raw: &str) -> Result<String, &'static str> {
    adversarial::route::normalize_route_token(raw)
}

pub(super) fn generate_route_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    hex_encode(&bytes)
}

pub(super) fn relay_self_inbox_route_token() -> Result<String, &'static str> {
    let raw = vault::secret_get(TUI_RELAY_INBOX_TOKEN_SECRET_KEY)
        .map_err(|_| QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED)?
        .ok_or(QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED)?;
    if raw.trim().is_empty() {
        return Err(QSC_ERR_RELAY_INBOX_TOKEN_REQUIRED);
    }
    normalize_route_token(raw.as_str())
}

pub(super) fn relay_peer_route_token(peer: &str) -> Result<String, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    let rec = contacts_entry_read(peer_alias).map_err(|_| QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)?;
    let token = rec
        .and_then(|v| {
            primary_device(&v)
                .and_then(|d| d.route_token.clone())
                .or(v.route_token)
        })
        .ok_or(QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)?;
    normalize_route_token(token.as_str()).map_err(|_| QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED)
}

pub(super) fn legacy_contact_status_to_device_state(status: &str) -> &'static str {
    let upper = status.trim().to_ascii_uppercase();
    match upper.as_str() {
        "PINNED" => "TRUSTED",
        "VERIFIED" | "COMPLETE" => "VERIFIED",
        "MISMATCH" | "CHANGED" => "CHANGED",
        "REVOKED" => "REVOKED",
        _ => "UNVERIFIED",
    }
}

pub(super) fn canonical_device_state(state: &str) -> &'static str {
    let upper = state.trim().to_ascii_uppercase();
    match upper.as_str() {
        "TRUSTED" => "TRUSTED",
        "VERIFIED" => "VERIFIED",
        "CHANGED" | "MISMATCH" => "CHANGED",
        "REVOKED" => "REVOKED",
        _ => "UNVERIFIED",
    }
}

pub(super) fn device_state_to_legacy_status(state: &str) -> &'static str {
    match canonical_device_state(state) {
        "TRUSTED" => "PINNED",
        "VERIFIED" => "VERIFIED",
        "CHANGED" => "CHANGED",
        "REVOKED" => "CHANGED",
        _ => "UNVERIFIED",
    }
}

pub(super) fn device_id_short(alias: &str, sig_fp: Option<&str>, fp: &str) -> String {
    let basis = sig_fp
        .filter(|v| !v.trim().is_empty())
        .unwrap_or(fp)
        .trim()
        .to_string();
    let basis = if basis.is_empty() {
        alias.trim().to_string()
    } else {
        basis
    };
    let c = StdCrypto;
    let hash = c.sha512(basis.as_bytes());
    hex_encode(&hash[..6])
}

pub(super) fn normalize_contact_record(alias: &str, rec: &mut ContactRecord) -> bool {
    let mut mutated = false;
    if rec.devices.is_empty() {
        rec.devices.push(ContactDeviceRecord {
            device_id: device_id_short(alias, rec.sig_fp.as_deref(), rec.fp.as_str()),
            fp: rec.fp.clone(),
            sig_fp: rec.sig_fp.clone(),
            state: legacy_contact_status_to_device_state(rec.status.as_str()).to_string(),
            route_token: rec.route_token.clone(),
            seen_at: rec.seen_at,
            label: None,
        });
        mutated = true;
    }
    for dev in rec.devices.iter_mut() {
        let canonical = canonical_device_state(dev.state.as_str());
        if dev.state != canonical {
            dev.state = canonical.to_string();
            mutated = true;
        }
        if dev.device_id.trim().is_empty() || dev.device_id.len() > 12 {
            dev.device_id = device_id_short(alias, dev.sig_fp.as_deref(), dev.fp.as_str());
            mutated = true;
        }
        if dev.fp.trim().is_empty() {
            dev.fp = "UNSET".to_string();
            mutated = true;
        }
    }
    let mut normalized = rec.devices.clone();
    normalized.sort_by(|a, b| a.device_id.cmp(&b.device_id));
    if normalized.len() != rec.devices.len()
        || normalized
            .iter()
            .zip(rec.devices.iter())
            .any(|(a, b)| a.device_id != b.device_id)
    {
        rec.devices = normalized;
        mutated = true;
    }
    if let Some(primary) = rec.devices.first_mut() {
        if primary.route_token.is_none() && rec.route_token.is_some() {
            primary.route_token = rec.route_token.clone();
            mutated = true;
        }
    }
    let canonical_primary = rec
        .primary_device_id
        .as_ref()
        .and_then(|id| {
            rec.devices
                .iter()
                .find(|d| d.device_id == *id)
                .map(|d| d.device_id.clone())
        })
        .or_else(|| {
            rec.devices
                .iter()
                .find(|d| canonical_device_state(d.state.as_str()) == "TRUSTED")
                .map(|d| d.device_id.clone())
        })
        .or_else(|| rec.devices.first().map(|d| d.device_id.clone()));
    if rec.primary_device_id != canonical_primary {
        rec.primary_device_id = canonical_primary;
        mutated = true;
    }
    if let Some(primary) = rec
        .primary_device_id
        .as_ref()
        .and_then(|id| rec.devices.iter().find(|d| d.device_id == *id))
    {
        let legacy_status = device_state_to_legacy_status(primary.state.as_str()).to_string();
        if rec.status.to_ascii_uppercase() != legacy_status {
            rec.status = legacy_status;
            mutated = true;
        }
        if rec.fp != primary.fp {
            rec.fp = primary.fp.clone();
            mutated = true;
        }
        if rec.sig_fp != primary.sig_fp {
            rec.sig_fp = primary.sig_fp.clone();
            mutated = true;
        }
        if rec.route_token != primary.route_token {
            rec.route_token = primary.route_token.clone();
            mutated = true;
        }
        if rec.seen_at != primary.seen_at {
            rec.seen_at = primary.seen_at;
            mutated = true;
        }
    }
    mutated
}

pub(super) fn primary_device(rec: &ContactRecord) -> Option<&ContactDeviceRecord> {
    if let Some(primary_id) = rec.primary_device_id.as_ref() {
        if let Some(dev) = rec.devices.iter().find(|d| d.device_id == *primary_id) {
            return Some(dev);
        }
    }
    rec.devices.first()
}

pub(super) fn primary_device_mut(rec: &mut ContactRecord) -> Option<&mut ContactDeviceRecord> {
    if let Some(primary_id) = rec.primary_device_id.as_ref() {
        if let Some(idx) = rec.devices.iter().position(|d| d.device_id == *primary_id) {
            return rec.devices.get_mut(idx);
        }
    }
    rec.devices.first_mut()
}

pub(super) fn peer_alias_from_channel(peer: &str) -> &str {
    peer.split_once('#').map(|(alias, _)| alias).unwrap_or(peer)
}

pub(super) fn channel_device_id(channel: &str) -> Option<&str> {
    channel
        .split_once('#')
        .map(|(_, device)| device)
        .filter(|v| !v.is_empty())
}

pub(super) fn channel_device_marker(channel: &str) -> String {
    channel_device_id(channel)
        .map(short_device_marker)
        .unwrap_or_else(|| "unknown".to_string())
}

pub(super) fn confirm_target_matches_channel(
    target_device_id: Option<&str>,
    channel: &str,
) -> bool {
    match target_device_id {
        None => true,
        Some(expected) => match channel_device_id(channel) {
            Some(actual) => short_device_marker(actual) == short_device_marker(expected),
            // Legacy receive flows may not carry a device-qualified channel label.
            // We keep these confirmations compatible while enforcing strict matching
            // when a device-qualified channel is present.
            None => true,
        },
    }
}

pub(super) fn channel_label_for_device(peer_alias: &str, device_id: &str) -> Option<String> {
    if !channel_label_ok(peer_alias) || !channel_label_ok(device_id) {
        return None;
    }
    let label = format!("{peer_alias}#{device_id}");
    if channel_label_ok(label.as_str()) {
        Some(label)
    } else {
        None
    }
}

#[derive(Clone, Debug)]
pub(super) struct SendRoutingTarget {
    pub(super) peer_alias: String,
    pub(super) channel: String,
    pub(super) device_id: String,
    pub(super) route_token: String,
    pub(super) implicit_primary: bool,
}

pub(super) fn resolve_peer_device_target(
    peer: &str,
    require_trusted: bool,
) -> Result<SendRoutingTarget, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let mut rec = contacts_entry_read(peer_alias).map_err(|_| "contacts_store_invalid")?;
    let Some(mut rec) = rec.take() else {
        return Err("unknown_contact");
    };
    let implicit_primary = rec.primary_device_id.is_none();
    let mut mutated = normalize_contact_record(peer_alias, &mut rec);
    let Some(primary) = primary_device(&rec).cloned() else {
        return Err("no_trusted_device");
    };
    let canonical_state = canonical_device_state(primary.state.as_str());
    match canonical_state {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        "TRUSTED" => {}
        _ if require_trusted => return Err("no_trusted_device"),
        _ => {}
    }
    let route_token = primary
        .route_token
        .clone()
        .or_else(|| rec.route_token.clone())
        .ok_or("contact_route_token_missing")?;
    let route_token =
        normalize_route_token(route_token.as_str()).map_err(|_| "contact_route_token_missing")?;
    if rec.route_token != Some(route_token.clone()) {
        rec.route_token = Some(route_token.clone());
        mutated = true;
    }
    if rec.primary_device_id.as_deref() != Some(primary.device_id.as_str()) {
        rec.primary_device_id = Some(primary.device_id.clone());
        mutated = true;
    }
    let multi_device = rec.devices.len() > 1;
    if mutated {
        contacts_entry_upsert(peer_alias, rec).map_err(|_| "contacts_store_invalid")?;
    }
    let channel = if multi_device {
        channel_label_for_device(peer_alias, primary.device_id.as_str())
            .ok_or("qsp_channel_invalid")?
    } else {
        peer_alias.to_string()
    };
    Ok(SendRoutingTarget {
        peer_alias: peer_alias.to_string(),
        channel,
        device_id: primary.device_id,
        route_token,
        implicit_primary,
    })
}

pub(super) fn resolve_send_routing_target(peer: &str) -> Result<SendRoutingTarget, &'static str> {
    resolve_peer_device_target(peer, true)
}

pub(super) fn tui_resolve_peer_device_target(
    state: &TuiState,
    peer: &str,
    require_trusted: bool,
) -> Result<SendRoutingTarget, &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let mut rec = state
        .contact_record_cached(peer_alias)
        .cloned()
        .ok_or("unknown_contact")?;
    let implicit_primary = rec.primary_device_id.is_none();
    normalize_contact_record(peer_alias, &mut rec);
    let Some(primary) = primary_device(&rec).cloned() else {
        return Err("no_trusted_device");
    };
    let canonical_state = canonical_device_state(primary.state.as_str());
    match canonical_state {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        "TRUSTED" => {}
        _ if require_trusted => return Err("no_trusted_device"),
        _ => {}
    }
    let route_token = primary
        .route_token
        .clone()
        .or_else(|| rec.route_token.clone())
        .ok_or("contact_route_token_missing")?;
    let route_token =
        normalize_route_token(route_token.as_str()).map_err(|_| "contact_route_token_missing")?;
    let multi_device = rec.devices.len() > 1;
    let channel = if multi_device {
        channel_label_for_device(peer_alias, primary.device_id.as_str())
            .ok_or("qsp_channel_invalid")?
    } else {
        peer_alias.to_string()
    };
    Ok(SendRoutingTarget {
        peer_alias: peer_alias.to_string(),
        channel,
        device_id: primary.device_id,
        route_token,
        implicit_primary,
    })
}

pub(super) fn contacts_store_load() -> Result<ContactsStore, ErrorCode> {
    match vault::secret_get(CONTACTS_SECRET_KEY) {
        Ok(None) => Ok(ContactsStore::default()),
        Ok(Some(v)) => {
            let mut store =
                serde_json::from_str::<ContactsStore>(&v).map_err(|_| ErrorCode::ParseFailed)?;
            let mut migrated = false;
            for (alias, rec) in store.peers.iter_mut() {
                if normalize_contact_record(alias.as_str(), rec) {
                    migrated = true;
                }
            }
            if migrated {
                contacts_store_save(&store)?;
            }
            Ok(store)
        }
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoReadFailed),
    }
}

pub(super) fn contacts_store_save(store: &ContactsStore) -> Result<(), ErrorCode> {
    let mut normalized = store.clone();
    for (alias, rec) in normalized.peers.iter_mut() {
        normalize_contact_record(alias.as_str(), rec);
    }
    let json = serde_json::to_string(&normalized).map_err(|_| ErrorCode::ParseFailed)?;
    match vault::secret_set(CONTACTS_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

pub(super) fn contacts_entry_read(label: &str) -> Result<Option<ContactRecord>, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let store = contacts_store_load()?;
    Ok(store.peers.get(label).cloned())
}

pub(super) fn contact_requests_store_load() -> Result<ContactRequestsStore, ErrorCode> {
    match vault::secret_get(CONTACT_REQUESTS_SECRET_KEY) {
        Ok(None) => Ok(ContactRequestsStore::default()),
        Ok(Some(v)) => {
            serde_json::from_str::<ContactRequestsStore>(&v).map_err(|_| ErrorCode::ParseFailed)
        }
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoReadFailed),
    }
}

pub(super) fn contact_requests_store_save(store: &ContactRequestsStore) -> Result<(), ErrorCode> {
    let json = serde_json::to_string(store).map_err(|_| ErrorCode::ParseFailed)?;
    match vault::secret_set(CONTACT_REQUESTS_SECRET_KEY, &json) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err(ErrorCode::IdentitySecretUnavailable),
        Err(_) => Err(ErrorCode::IoWriteFailed),
    }
}

pub(super) fn contact_request_upsert(
    alias: &str,
    device_id: Option<&str>,
    reason: Option<&str>,
) -> Result<(), ErrorCode> {
    if !channel_label_ok(alias) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contact_requests_store_load()?;
    let rec = ContactRequestRecord {
        alias: alias.to_string(),
        device_id: device_id.map(short_device_marker),
        state: "PENDING".to_string(),
        reason: reason.map(|v| v.to_string()),
        seen_at: None,
    };
    store.requests.insert(alias.to_string(), rec);
    contact_requests_store_save(&store)
}

pub(super) fn contact_request_remove(alias: &str) -> Result<bool, ErrorCode> {
    if !channel_label_ok(alias) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contact_requests_store_load()?;
    let removed = store.requests.remove(alias).is_some();
    if removed {
        contact_requests_store_save(&store)?;
    }
    Ok(removed)
}

pub(super) fn contact_request_list() -> Result<Vec<ContactRequestRecord>, ErrorCode> {
    let mut items = contact_requests_store_load()?
        .requests
        .into_values()
        .collect::<Vec<_>>();
    items.sort_by(|a, b| a.alias.cmp(&b.alias));
    Ok(items)
}

pub(super) fn contacts_entry_upsert(label: &str, rec: ContactRecord) -> Result<(), ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    store.peers.insert(label.to_string(), rec);
    contacts_store_save(&store)
}

pub(super) fn contacts_set_blocked(label: &str, blocked: bool) -> Result<bool, ErrorCode> {
    if !channel_label_ok(label) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut store = contacts_store_load()?;
    let Some(rec) = store.peers.get_mut(label) else {
        return Ok(false);
    };
    rec.blocked = blocked;
    contacts_store_save(&store)?;
    Ok(true)
}

pub(super) fn contacts_list_entries() -> Result<Vec<(String, ContactRecord)>, ErrorCode> {
    let store = contacts_store_load()?;
    Ok(store.peers.into_iter().collect())
}

pub(super) fn contact_state(rec: Option<&ContactRecord>) -> &'static str {
    match rec {
        Some(v) => match primary_device(v).map(|d| canonical_device_state(d.state.as_str())) {
            Some("TRUSTED") => "PINNED",
            Some("VERIFIED") => "VERIFIED",
            Some("CHANGED") => "CHANGED",
            Some("REVOKED") => "CHANGED",
            _ => "UNVERIFIED",
        },
        None => "UNVERIFIED",
    }
}

pub(super) fn short_device_marker(device: &str) -> String {
    let mut out = String::new();
    for ch in device.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            out.push(ch.to_ascii_lowercase());
        }
        if out.len() >= 12 {
            break;
        }
    }
    if out.is_empty() {
        "unknown".to_string()
    } else {
        out
    }
}

pub(super) fn emit_cli_contact_flow(
    action: &str,
    state: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

pub(super) fn emit_tui_contact_flow(
    action: &str,
    state: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_FLOW",
            &[
                ("action", action),
                ("state", state),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

pub(super) fn emit_cli_contact_request(action: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_CONTACT_REQUEST",
            &[
                ("action", action),
                ("peer", safe_peer.as_str()),
                ("device", dev),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_CONTACT_REQUEST",
            &[("action", action), ("peer", safe_peer.as_str())],
        );
    }
}

pub(super) fn emit_tui_contact_request(action: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_REQUEST",
            &[
                ("action", action),
                ("peer", safe_peer.as_str()),
                ("device", dev),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_CONTACT_REQUEST",
            &[("action", action), ("peer", safe_peer.as_str())],
        );
    }
}

pub(super) fn emit_cli_trust_promotion(
    result: &str,
    reason: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_cli_named_marker(
            "QSC_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_cli_named_marker(
            "QSC_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

pub(super) fn emit_tui_trust_promotion(
    result: &str,
    reason: &str,
    peer: &str,
    device: Option<&str>,
    mode: TrustOnboardingMode,
) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    if let Some(dev) = safe_device.as_ref() {
        emit_tui_named_marker(
            "QSC_TUI_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("device", dev.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    } else {
        emit_tui_named_marker(
            "QSC_TUI_TRUST_PROMOTION",
            &[
                ("result", result),
                ("reason", reason),
                ("peer", safe_peer.as_str()),
                ("mode", mode.as_str()),
            ],
        );
    }
}

pub(super) fn trust_remediation_steps(reason: &str) -> &'static [&'static str] {
    match reason {
        "unknown_contact" => &["add_contact", "learn_more"],
        "no_trusted_device" => &[
            "list_devices",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        "device_changed_reapproval_required" => &[
            "reapprove_changed_device",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        "device_revoked" => &[
            "readd_revoked_device",
            "verify_device",
            "trust_device",
            "learn_more",
        ],
        _ => &["learn_more"],
    }
}

pub(super) fn trust_remediation_hint(reason: &str) -> &'static str {
    match reason {
        "unknown_contact" => {
            "Add contact first: /contacts add <alias> <verification_code> [route_token]"
        }
        "no_trusted_device" => {
            "No trusted device for this contact. List devices, verify one, then trust it."
        }
        "device_changed_reapproval_required" => {
            "Device changed. Re-verify and explicitly trust that device before sending."
        }
        "device_revoked" => {
            "Device revoked. Re-add or verify a replacement device before trusting it."
        }
        _ => "Send blocked by trust policy. Review contact and device trust state.",
    }
}

pub(super) fn trust_remediation_verify_vs_trusted_hint() -> &'static str {
    "VERIFIED means identity/code matched; TRUSTED means send-authorized."
}

pub(super) fn emit_cli_trust_remediation(reason: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    for step in trust_remediation_steps(reason) {
        if let Some(dev) = safe_device.as_ref() {
            emit_cli_named_marker(
                "QSC_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                    ("device", dev.as_str()),
                ],
            );
        } else {
            emit_cli_named_marker(
                "QSC_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                ],
            );
        }
    }
}

pub(super) fn emit_tui_trust_remediation(reason: &str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    let safe_device = device.map(short_device_marker);
    for step in trust_remediation_steps(reason) {
        if let Some(dev) = safe_device.as_ref() {
            emit_tui_named_marker(
                "QSC_TUI_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                    ("device", dev.as_str()),
                ],
            );
        } else {
            emit_tui_named_marker(
                "QSC_TUI_TRUST_REMEDIATION",
                &[
                    ("reason", reason),
                    ("step", step),
                    ("peer", safe_peer.as_str()),
                ],
            );
        }
    }
}

pub(super) fn trust_gate_device_hint(peer: &str, reason: &str) -> Option<String> {
    match reason {
        "no_trusted_device" | "device_changed_reapproval_required" | "device_revoked" => {}
        _ => return None,
    }
    let alias = peer_alias_from_channel(peer);
    if !channel_label_ok(alias) {
        return None;
    }
    let rec = contacts_entry_read(alias).ok().flatten()?;
    let primary = primary_device(&rec)?;
    Some(short_device_marker(primary.device_id.as_str()))
}

pub(super) fn send_contact_trust_gate(peer: &str) -> Result<(), &'static str> {
    let peer_alias = peer_alias_from_channel(peer);
    if !channel_label_ok(peer_alias) {
        return Err("unknown_contact");
    }
    let rec = contacts_entry_read(peer_alias).map_err(|_| "contacts_store_invalid")?;
    let Some(rec) = rec else {
        return Err("unknown_contact");
    };
    let Some(primary) = primary_device(&rec) else {
        return Err("no_trusted_device");
    };
    match canonical_device_state(primary.state.as_str()) {
        "CHANGED" => return Err("device_changed_reapproval_required"),
        "REVOKED" => return Err("device_revoked"),
        _ => {}
    }
    if !contact_has_trusted_device(&rec) {
        return Err("no_trusted_device");
    }
    Ok(())
}

pub(super) fn emit_cli_send_blocked(reason: &'static str, peer: &str, device: Option<&str>) {
    let safe_peer = short_peer_marker(peer);
    emit_cli_named_marker(
        "QSC_SEND_BLOCKED",
        &[("reason", reason), ("peer", safe_peer.as_str())],
    );
    emit_marker(
        "send_blocked",
        Some(reason),
        &[("reason", reason), ("peer", safe_peer.as_str())],
    );
    emit_cli_trust_remediation(reason, peer, device);
    eprintln!("HINT: {}", trust_remediation_hint(reason));
    eprintln!("HINT: VERIFIED means identity/code matched; TRUSTED means send-authorized.");
}

pub(super) fn emit_cli_routing_marker(peer: &str, device_id: &str, implicit: bool) {
    let safe_peer = short_peer_marker(peer);
    let mut fields = vec![
        ("policy", "primary_only"),
        ("peer", safe_peer.as_str()),
        ("device", device_id),
    ];
    if implicit {
        fields.push(("selected", "implicit"));
    }
    emit_cli_named_marker("QSC_ROUTING", fields.as_slice());
}

pub(super) fn emit_tui_routing_marker(thread: &str, device_id: &str, implicit: bool) {
    let safe_thread = short_peer_marker(thread);
    let mut fields = vec![
        ("policy", "primary_only"),
        ("thread", safe_thread.as_str()),
        ("device", device_id),
    ];
    if implicit {
        fields.push(("selected", "implicit"));
    }
    emit_tui_named_marker("QSC_TUI_ROUTING", fields.as_slice());
}

pub(super) fn enforce_cli_send_contact_trust(peer: &str) -> Result<(), &'static str> {
    match send_contact_trust_gate(peer) {
        Ok(()) => Ok(()),
        Err("unknown_contact") => {
            emit_cli_send_blocked("unknown_contact", peer, None);
            Err("unknown_contact")
        }
        Err("no_trusted_device") => {
            let device = trust_gate_device_hint(peer, "no_trusted_device");
            emit_cli_send_blocked("no_trusted_device", peer, device.as_deref());
            Err("no_trusted_device")
        }
        Err("device_changed_reapproval_required") => {
            let device = trust_gate_device_hint(peer, "device_changed_reapproval_required");
            emit_cli_send_blocked(
                "device_changed_reapproval_required",
                peer,
                device.as_deref(),
            );
            Err("device_changed_reapproval_required")
        }
        Err("device_revoked") => {
            let device = trust_gate_device_hint(peer, "device_revoked");
            emit_cli_send_blocked("device_revoked", peer, device.as_deref());
            Err("device_revoked")
        }
        Err(code) => Err(code),
    }
}

pub(super) fn contact_blocked(label: &str) -> Result<bool, ErrorCode> {
    let alias = peer_alias_from_channel(label);
    Ok(contacts_entry_read(alias)?
        .map(|v| v.blocked)
        .unwrap_or(false))
}

pub(super) fn tui_contact_blocked(state: &TuiState, label: &str) -> Result<bool, &'static str> {
    let alias = peer_alias_from_channel(label);
    let rec = state
        .contact_record_cached(alias)
        .ok_or("unknown_contact")?;
    Ok(rec.blocked)
}

pub(super) fn enforce_peer_not_blocked(label: &str) -> Result<(), &'static str> {
    let alias = peer_alias_from_channel(label);
    match contact_blocked(label) {
        Ok(true) => {
            emit_marker(
                "contacts_refuse",
                None,
                &[("label", alias), ("reason", "peer_blocked")],
            );
            Err("peer_blocked")
        }
        Ok(false) => Ok(()),
        // Missing/locked contacts store means no explicit block policy is available.
        Err(ErrorCode::IdentitySecretUnavailable) => Ok(()),
        Err(_) => Err("contacts_store_invalid"),
    }
}

pub(super) fn tui_enforce_peer_not_blocked(
    state: &TuiState,
    label: &str,
) -> Result<(), &'static str> {
    let alias = peer_alias_from_channel(label);
    match tui_contact_blocked(state, label) {
        Ok(true) => {
            emit_marker(
                "contacts_refuse",
                None,
                &[("label", alias), ("reason", "peer_blocked")],
            );
            Err("peer_blocked")
        }
        Ok(false) => Ok(()),
        Err(code) => Err(code),
    }
}

pub(super) fn contacts_add(label: &str, fp: &str, route_token: Option<&str>, verify: bool) {
    if !require_unlocked("contacts_add") {
        return;
    }
    let status = if verify { "verified" } else { "pinned" };
    let route_token = match route_token {
        Some(raw) => {
            Some(normalize_route_token(raw).unwrap_or_else(|code| print_error_marker(code)))
        }
        None => Some(generate_route_token()),
    };
    let rec = ContactRecord {
        fp: fp.to_string(),
        status: status.to_string(),
        blocked: false,
        seen_at: None,
        sig_fp: None,
        route_token: route_token.clone(),
        primary_device_id: None,
        devices: vec![ContactDeviceRecord {
            device_id: device_id_short(label, None, fp),
            fp: fp.to_string(),
            sig_fp: None,
            state: legacy_contact_status_to_device_state(status).to_string(),
            route_token: route_token.clone(),
            seen_at: None,
            label: None,
        }],
    };
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_add",
        None,
        &[("ok", "true"), ("label", label), ("status", status)],
    );
    let mode = load_trust_onboarding_mode_from_account();
    let state = if verify { "VERIFIED" } else { "DISCOVERED" };
    emit_cli_contact_flow("add", state, label, None, mode);
    println!("contact={} status={}", label, status);
}

pub(super) fn contact_device_find_index(rec: &ContactRecord, device_id: &str) -> Option<usize> {
    rec.devices.iter().position(|d| d.device_id == device_id)
}

pub(super) fn contact_has_trusted_device(rec: &ContactRecord) -> bool {
    rec.devices
        .iter()
        .any(|d| canonical_device_state(d.state.as_str()) == "TRUSTED")
}

pub(super) fn contacts_device_add(label: &str, fp: &str, route_token: Option<&str>) {
    if !require_unlocked("contacts_device_add") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let route_token = route_token
        .map(|raw| normalize_route_token(raw).unwrap_or_else(|code| print_error_marker(code)));
    let device_id = device_id_short(label, None, fp);
    if contact_device_find_index(&rec, device_id.as_str()).is_some() {
        emit_marker(
            "contacts_device_add",
            Some("device_exists"),
            &[
                ("ok", "false"),
                ("label", label),
                ("device", device_id.as_str()),
            ],
        );
        print_error_marker("device_exists");
    }
    rec.devices.push(ContactDeviceRecord {
        device_id: device_id.clone(),
        fp: fp.to_ascii_uppercase(),
        sig_fp: None,
        state: "UNVERIFIED".to_string(),
        route_token,
        seen_at: None,
        label: None,
    });
    normalize_contact_record(label, &mut rec);
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_add",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device_id.as_str()),
            ("state", "UNVERIFIED"),
        ],
    );
}

pub(super) fn contacts_device_list(label: &str) {
    if !require_unlocked("contacts_device_list") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let mut rec = rec;
    normalize_contact_record(label, &mut rec);
    let count_s = rec.devices.len().to_string();
    emit_marker(
        "contacts_device_list",
        None,
        &[("label", label), ("count", count_s.as_str())],
    );
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none");
    println!(
        "label={} device_count={} primary_device={}",
        label, count_s, primary
    );
    for dev in rec.devices {
        println!(
            "device={} state={}",
            dev.device_id,
            canonical_device_state(dev.state.as_str())
        );
    }
}

pub(super) fn contacts_device_status(label: &str, device: Option<&str>) {
    if !require_unlocked("contacts_device_status") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let mut rec = rec;
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none")
        .to_string();
    match device {
        Some(device_id) => {
            let Some(idx) = contact_device_find_index(&rec, device_id) else {
                print_error_marker("device_unknown");
            };
            let dev = &rec.devices[idx];
            let state = canonical_device_state(dev.state.as_str());
            emit_marker(
                "contacts_device_status",
                None,
                &[
                    ("label", label),
                    ("device", device_id),
                    ("state", state),
                    ("primary", bool_str(primary == device_id)),
                ],
            );
            println!(
                "label={} device={} state={} primary={}",
                label,
                device_id,
                state,
                bool_str(primary == device_id)
            );
        }
        None => {
            let count_s = rec.devices.len().to_string();
            emit_marker(
                "contacts_device_status",
                None,
                &[("label", label), ("count", count_s.as_str())],
            );
            println!(
                "label={} device_count={} primary_device={}",
                label, count_s, primary
            );
            for dev in rec.devices.iter() {
                println!(
                    "device={} state={} primary={}",
                    dev.device_id,
                    canonical_device_state(dev.state.as_str()),
                    bool_str(dev.device_id == primary)
                );
            }
        }
    }
}

pub(super) fn contacts_device_verify(label: &str, device: &str, fp: &str) {
    if !require_unlocked("contacts_device_verify") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    let expected = rec.devices[idx].fp.to_ascii_uppercase();
    let provided = fp.to_ascii_uppercase();
    if expected == provided {
        let mode = load_trust_onboarding_mode_from_account();
        rec.devices[idx].state = "VERIFIED".to_string();
        rec.status = "VERIFIED".to_string();
        if mode == TrustOnboardingMode::Balanced {
            rec.devices[idx].state = "TRUSTED".to_string();
            rec.status = "PINNED".to_string();
        }
        if contacts_entry_upsert(label, rec).is_err() {
            print_error_marker("contacts_store_unavailable");
        }
        emit_marker(
            "contacts_device_verify",
            None,
            &[
                ("ok", "true"),
                ("label", label),
                ("device", device),
                (
                    "state",
                    if mode == TrustOnboardingMode::Balanced {
                        "TRUSTED"
                    } else {
                        "VERIFIED"
                    },
                ),
            ],
        );
        emit_cli_contact_flow(
            "verify",
            if mode == TrustOnboardingMode::Balanced {
                "TRUSTED"
            } else {
                "VERIFIED"
            },
            label,
            Some(device),
            mode,
        );
        if mode == TrustOnboardingMode::Balanced {
            emit_cli_trust_promotion("trusted", "verified_match", label, Some(device), mode);
        } else {
            emit_cli_trust_promotion("verified_only", "strict_mode", label, Some(device), mode);
        }
        return;
    }
    rec.devices[idx].state = "CHANGED".to_string();
    rec.status = "CHANGED".to_string();
    let _ = contacts_entry_upsert(label, rec);
    emit_marker(
        "contacts_device_verify",
        Some("verification_mismatch"),
        &[
            ("ok", "false"),
            ("label", label),
            ("device", device),
            ("state", "CHANGED"),
        ],
    );
    emit_cli_contact_flow(
        "verify",
        "CHANGED",
        label,
        Some(device),
        load_trust_onboarding_mode_from_account(),
    );
    print_error_marker("verification_mismatch");
}

pub(super) fn contacts_device_trust(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_trust") {
        return;
    }
    if !confirm {
        print_error_marker("trust_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.devices[idx].state = "TRUSTED".to_string();
    rec.status = "PINNED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_trust",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("state", "TRUSTED"),
        ],
    );
    let mode = load_trust_onboarding_mode_from_account();
    emit_cli_contact_flow("trust", "TRUSTED", label, Some(device), mode);
    emit_cli_trust_promotion(
        "trusted",
        "explicit_operator_action",
        label,
        Some(device),
        mode,
    );
}

pub(super) fn contacts_device_revoke(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_revoke") {
        return;
    }
    if !confirm {
        print_error_marker("revoke_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(idx) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.devices[idx].state = "REVOKED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_revoke",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("state", "REVOKED"),
        ],
    );
}

pub(super) fn contacts_device_primary_set(label: &str, device: &str, confirm: bool) {
    if !require_unlocked("contacts_device_primary_set") {
        return;
    }
    if !confirm {
        print_error_marker("primary_set_requires_confirm");
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    normalize_contact_record(label, &mut rec);
    let Some(_) = contact_device_find_index(&rec, device) else {
        print_error_marker("device_unknown");
    };
    rec.primary_device_id = Some(device.to_string());
    normalize_contact_record(label, &mut rec);
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_device_primary_set",
        None,
        &[
            ("ok", "true"),
            ("label", label),
            ("device", device),
            ("selected", "explicit"),
            ("policy", "primary_only"),
        ],
    );
}

pub(super) fn contacts_device_primary_show(label: &str) {
    if !require_unlocked("contacts_device_primary_show") {
        return;
    }
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or_else(|| print_error_marker("peer_unknown"));
    let implicit = rec.primary_device_id.is_none();
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec)
        .map(|d| d.device_id.as_str())
        .unwrap_or("none");
    let selected = if implicit { "implicit" } else { "explicit" };
    emit_marker(
        "contacts_device_primary_show",
        None,
        &[
            ("label", label),
            ("device", primary),
            ("selected", selected),
            ("policy", "primary_only"),
        ],
    );
    println!(
        "label={} primary_device={} selected={} policy=primary_only",
        label, primary, selected
    );
}

pub(super) fn contacts_route_set(label: &str, route_token: &str) {
    if !require_unlocked("contacts_route_set") {
        return;
    }
    let token = normalize_route_token(route_token).unwrap_or_else(|code| print_error_marker(code));
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: "UNSET".to_string(),
            status: "UNVERIFIED".to_string(),
            blocked: false,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, "UNSET"),
                fp: "UNSET".to_string(),
                sig_fp: None,
                state: "UNVERIFIED".to_string(),
                route_token: None,
                seen_at: None,
                label: None,
            }],
        });
    rec.route_token = Some(token);
    normalize_contact_record(label, &mut rec);
    let primary_route_token = rec.route_token.clone();
    if let Some(primary) = primary_device_mut(&mut rec) {
        primary.route_token = primary_route_token;
    }
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_marker(
        "contacts_route_set",
        None,
        &[("ok", "true"), ("label", label)],
    );
}

pub(super) fn contacts_show(label: &str) {
    if !require_unlocked("contacts_show") {
        return;
    }
    let rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    let state = contact_state(rec.as_ref());
    let blocked = bool_str(rec.as_ref().map(|v| v.blocked).unwrap_or(false));
    let device_count = rec.as_ref().map(|v| v.devices.len()).unwrap_or(0);
    let device_count_s = device_count.to_string();
    emit_marker(
        "contacts_show",
        None,
        &[
            ("label", label),
            ("state", state),
            ("blocked", blocked),
            ("device_count", device_count_s.as_str()),
        ],
    );
    if let Some(v) = rec {
        let primary_id = primary_device(&v)
            .map(|d| d.device_id.as_str())
            .unwrap_or("none");
        println!(
            "label={} state={} blocked={} device_count={} primary_device={}",
            label, state, blocked, device_count, primary_id
        );
        for dev in v.devices.iter() {
            let state = canonical_device_state(dev.state.as_str());
            println!("device={} state={}", dev.device_id, state);
        }
    } else {
        println!("label={} state=unknown blocked=false", label);
    }
}

pub(super) fn contacts_list() {
    if !require_unlocked("contacts_list") {
        return;
    }
    let mut entries = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = entries.len().to_string();
    emit_marker("contacts_list", None, &[("count", count_s.as_str())]);
    for (label, rec) in entries {
        let state = contact_state(Some(&rec));
        let blocked = bool_str(rec.blocked);
        let device_count = rec.devices.len();
        let primary_id = primary_device(&rec)
            .map(|d| d.device_id.as_str())
            .unwrap_or("none");
        println!(
            "label={} state={} blocked={} device_count={} primary_device={}",
            label, state, blocked, device_count, primary_id
        );
    }
}

pub(super) fn contacts_verify(label: &str, fp: &str, confirm: bool) {
    if !require_unlocked("contacts_verify") {
        return;
    }
    let Some(mut rec) = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
    else {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "peer_unknown"),
            ],
        );
        print_error_marker("peer_unknown");
    };
    normalize_contact_record(label, &mut rec);
    let primary = primary_device(&rec).map(|d| d.device_id.clone());
    let Some(primary) = primary else {
        print_error_marker("device_unknown");
    };
    if !confirm {
        emit_marker(
            "contacts_verify",
            None,
            &[
                ("ok", "false"),
                ("label", label),
                ("result", "refused"),
                ("reason", "confirm_required"),
            ],
        );
        print_error_marker("verify_requires_confirm");
    }
    contacts_device_verify(label, primary.as_str(), fp);
}

pub(super) fn contacts_block(label: &str) {
    if !require_unlocked("contacts_block") {
        return;
    }
    match contacts_set_blocked(label, true) {
        Ok(true) => emit_marker("contacts_block", None, &[("label", label), ("ok", "true")]),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

pub(super) fn contacts_unblock(label: &str) {
    if !require_unlocked("contacts_unblock") {
        return;
    }
    match contacts_set_blocked(label, false) {
        Ok(true) => emit_marker(
            "contacts_unblock",
            None,
            &[("label", label), ("ok", "true")],
        ),
        Ok(false) => print_error_marker("peer_unknown"),
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

pub(super) fn contacts_trust_mode_show() {
    if !require_unlocked("contacts_trust_mode_show") {
        return;
    }
    let mode = load_trust_onboarding_mode_from_account();
    emit_cli_named_marker("QSC_TRUST_MODE", &[("mode", mode.as_str())]);
    println!("trust_mode={}", mode.as_str());
}

pub(super) fn contacts_trust_mode_set(mode: TrustMode) {
    if !require_unlocked("contacts_trust_mode_set") {
        return;
    }
    let mode = TrustOnboardingMode::from_arg(mode);
    match vault::secret_set(TUI_TRUST_MODE_SECRET_KEY, mode.as_str()) {
        Ok(()) => {
            emit_cli_named_marker("QSC_TRUST_MODE", &[("mode", mode.as_str()), ("ok", "true")]);
            println!("trust_mode={}", mode.as_str());
        }
        Err(_) => print_error_marker("contacts_store_unavailable"),
    }
}

pub(super) fn contacts_request_list() {
    if !require_unlocked("contacts_request_list") {
        return;
    }
    let items =
        contact_request_list().unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    let count_s = items.len().to_string();
    emit_cli_named_marker(
        "QSC_CONTACT_REQUEST",
        &[("action", "list"), ("count", count_s.as_str())],
    );
    for item in items {
        println!(
            "request alias={} state={} device={}",
            item.alias,
            item.state,
            item.device_id.unwrap_or_else(|| "unknown".to_string())
        );
    }
}

pub(super) fn contacts_request_accept(label: &str) {
    if !require_unlocked("contacts_request_accept") {
        return;
    }
    let removed = contact_request_remove(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    if !removed {
        print_error_marker("request_unknown");
    }
    let fp = "UNSET".to_string();
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: fp.clone(),
            status: "UNVERIFIED".to_string(),
            blocked: false,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, fp.as_str()),
                fp: fp.clone(),
                sig_fp: None,
                state: "UNVERIFIED".to_string(),
                route_token: None,
                seen_at: None,
                label: Some("request".to_string()),
            }],
        });
    normalize_contact_record(label, &mut rec);
    rec.status = "UNVERIFIED".to_string();
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_cli_contact_request("accept", label, None);
    emit_cli_contact_flow(
        "add",
        "DISCOVERED",
        label,
        None,
        load_trust_onboarding_mode_from_account(),
    );
}

pub(super) fn contacts_request_ignore(label: &str) {
    if !require_unlocked("contacts_request_ignore") {
        return;
    }
    let removed = contact_request_remove(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"));
    if !removed {
        print_error_marker("request_unknown");
    }
    emit_cli_contact_request("ignore", label, None);
}

pub(super) fn contacts_request_block(label: &str) {
    if !require_unlocked("contacts_request_block") {
        return;
    }
    let _ = contact_request_remove(label);
    let mut rec = contacts_entry_read(label)
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .unwrap_or(ContactRecord {
            fp: "UNSET".to_string(),
            status: "REVOKED".to_string(),
            blocked: true,
            seen_at: None,
            sig_fp: None,
            route_token: None,
            primary_device_id: None,
            devices: vec![ContactDeviceRecord {
                device_id: device_id_short(label, None, "UNSET"),
                fp: "UNSET".to_string(),
                sig_fp: None,
                state: "REVOKED".to_string(),
                route_token: None,
                seen_at: None,
                label: Some("blocked_request".to_string()),
            }],
        });
    normalize_contact_record(label, &mut rec);
    rec.blocked = true;
    if let Some(primary) = primary_device_mut(&mut rec) {
        primary.state = "REVOKED".to_string();
    }
    if contacts_entry_upsert(label, rec).is_err() {
        print_error_marker("contacts_store_unavailable");
    }
    emit_cli_contact_request("block", label, None);
}
