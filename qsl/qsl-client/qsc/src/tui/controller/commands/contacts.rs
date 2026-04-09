use super::*;

pub(super) fn dispatch_contacts_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    match cmd.cmd.as_str() {
        "contacts" => handle_contacts_command(cmd, state),
        "trust" => handle_trust_command(cmd, state),
        "requests" => handle_requests_command(cmd, state),
        "verify" => handle_verify_command(cmd, state),
        _ => {
            state.set_command_error(format!("unknown command: {}", cmd.cmd));
            emit_marker("tui_cmd", None, &[("cmd", cmd.cmd.as_str())]);
            false
        }
    }
}

fn handle_contacts_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "contacts")]);
    let sub = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
    match sub {
        "list" => {
            state.refresh_contacts();
            let count_s = state.contacts.len().to_string();
            emit_marker("tui_contacts_list", None, &[("count", count_s.as_str())]);
        }
        "device" => {
            let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("contacts device: missing action");
                emit_marker(
                    "tui_contacts_invalid",
                    None,
                    &[("reason", "missing_device_action")],
                );
                return false;
            };
            match action {
                "list" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device list: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(rec) = state.contacts_records.get(label) else {
                        state.set_command_error("contacts device list: unknown alias");
                        emit_marker(
                            "tui_contacts_device_list",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    let mut rec = rec.clone();
                    normalize_contact_record(label, &mut rec);
                    let count_s = rec.devices.len().to_string();
                    emit_marker(
                        "tui_contacts_device_list",
                        None,
                        &[("label", label), ("count", count_s.as_str())],
                    );
                    for dev in &rec.devices {
                        emit_marker(
                            "tui_contacts_device",
                            None,
                            &[
                                ("label", label),
                                ("device", dev.device_id.as_str()),
                                ("state", canonical_device_state(dev.state.as_str())),
                            ],
                        );
                    }
                }
                "status" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device status: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(rec) = state.contacts_records.get(label) else {
                        state.set_command_error("contacts device status: unknown alias");
                        emit_marker(
                            "tui_contacts_device_status",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    let mut rec = rec.clone();
                    normalize_contact_record(label, &mut rec);
                    if let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) {
                        let Some(idx) = contact_device_find_index(&rec, device_id) else {
                            state.set_command_error("contacts device status: unknown device id");
                            emit_marker(
                                "tui_contacts_device_status",
                                Some("device_unknown"),
                                &[("label", label), ("device", device_id), ("ok", "false")],
                            );
                            return false;
                        };
                        let dev = &rec.devices[idx];
                        emit_marker(
                            "tui_contacts_device_status",
                            None,
                            &[
                                ("label", label),
                                ("device", device_id),
                                ("state", canonical_device_state(dev.state.as_str())),
                            ],
                        );
                    } else {
                        let count_s = rec.devices.len().to_string();
                        emit_marker(
                            "tui_contacts_device_status",
                            None,
                            &[("label", label), ("count", count_s.as_str())],
                        );
                    }
                }
                "add" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device add: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(code) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device add: missing verification code");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                        return false;
                    };
                    if !tui_verification_code_is_valid(code) {
                        state.set_command_error(
                            "contacts device add: invalid verification code format",
                        );
                        emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                        return false;
                    }
                    let route_token = match cmd.args.get(4).map(|s| s.as_str()) {
                        Some(raw) => match normalize_route_token(raw) {
                            Ok(token) => Some(token),
                            Err(code) => {
                                state.set_command_error(format!("contacts device add: {}", code));
                                emit_marker("tui_contacts_invalid", Some(code), &[]);
                                return false;
                            }
                        },
                        None => None,
                    };
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("contacts device add: unknown alias");
                        emit_marker(
                            "tui_contacts_device_add",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    let device_id = device_id_short(label, None, code);
                    if contact_device_find_index(rec, device_id.as_str()).is_some() {
                        state.set_command_error("contacts device add: device already exists");
                        emit_marker(
                            "tui_contacts_device_add",
                            Some("device_exists"),
                            &[
                                ("label", label),
                                ("device", device_id.as_str()),
                                ("ok", "false"),
                            ],
                        );
                        return false;
                    }
                    rec.devices.push(ContactDeviceRecord {
                        device_id: device_id.clone(),
                        fp: code.to_ascii_uppercase(),
                        sig_fp: None,
                        state: "UNVERIFIED".to_string(),
                        route_token,
                        seen_at: None,
                        label: None,
                    });
                    normalize_contact_record(label, rec);
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("contacts device add: store unavailable");
                        emit_marker(
                            "tui_contacts_device_add",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    emit_marker(
                        "tui_contacts_device_add",
                        None,
                        &[
                            ("label", label),
                            ("device", device_id.as_str()),
                            ("state", "UNVERIFIED"),
                            ("ok", "true"),
                        ],
                    );
                    state.refresh_contacts();
                }
                "verify" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device verify: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device verify: missing device id");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_device")],
                        );
                        return false;
                    };
                    let Some(code) = cmd.args.get(4).map(|s| s.as_str()) else {
                        state
                            .set_command_error("contacts device verify: missing verification code");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                        return false;
                    };
                    if !tui_verification_code_is_valid(code) {
                        state.set_command_error(
                            "contacts device verify: invalid verification code format",
                        );
                        emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                        return false;
                    }
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("contacts device verify: unknown alias");
                        emit_marker(
                            "tui_contacts_device_verify",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    let Some(idx) = contact_device_find_index(rec, device_id) else {
                        state.set_command_error("contacts device verify: unknown device id");
                        emit_marker(
                            "tui_contacts_device_verify",
                            Some("device_unknown"),
                            &[("label", label), ("device", device_id), ("ok", "false")],
                        );
                        return false;
                    };
                    if rec.devices[idx].fp.eq_ignore_ascii_case(code) {
                        let mode = state.trust_onboarding_mode;
                        rec.devices[idx].state = "VERIFIED".to_string();
                        rec.status = "VERIFIED".to_string();
                        if mode == TrustOnboardingMode::Balanced {
                            rec.devices[idx].state = "TRUSTED".to_string();
                            rec.status = "PINNED".to_string();
                        }
                        if state.persist_contacts_cache().is_err() {
                            state.set_command_error("contacts device verify: store unavailable");
                            emit_marker(
                                "tui_contacts_device_verify",
                                Some("contacts_store_unavailable"),
                                &[("label", label), ("ok", "false")],
                            );
                            return false;
                        }
                        emit_marker(
                            "tui_contacts_device_verify",
                            None,
                            &[
                                ("label", label),
                                ("device", device_id),
                                (
                                    "state",
                                    if mode == TrustOnboardingMode::Balanced {
                                        "TRUSTED"
                                    } else {
                                        "VERIFIED"
                                    },
                                ),
                                ("ok", "true"),
                            ],
                        );
                        emit_tui_contact_flow(
                            "verify",
                            if mode == TrustOnboardingMode::Balanced {
                                "TRUSTED"
                            } else {
                                "VERIFIED"
                            },
                            label,
                            Some(device_id),
                            mode,
                        );
                        if mode == TrustOnboardingMode::Balanced {
                            emit_tui_trust_promotion(
                                "trusted",
                                "verified_match",
                                label,
                                Some(device_id),
                                mode,
                            );
                            state.set_command_feedback(
                                "ok: verification matched and device auto-trusted (balanced mode)",
                            );
                        } else {
                            emit_tui_trust_promotion(
                                "verified_only",
                                "strict_mode",
                                label,
                                Some(device_id),
                                mode,
                            );
                            state.set_command_feedback(
                                "ok: verification code matched identity (strict mode requires trust)",
                            );
                        }
                    } else {
                        rec.devices[idx].state = "CHANGED".to_string();
                        rec.status = "CHANGED".to_string();
                        let _ = state.persist_contacts_cache();
                        state.set_command_error(
                            "contacts device verify: verification code mismatch",
                        );
                        emit_marker(
                            "tui_contacts_device_verify",
                            Some("verification_mismatch"),
                            &[
                                ("label", label),
                                ("device", device_id),
                                ("state", "CHANGED"),
                                ("ok", "false"),
                            ],
                        );
                        emit_tui_contact_flow(
                            "verify",
                            "CHANGED",
                            label,
                            Some(device_id),
                            state.trust_onboarding_mode,
                        );
                        return false;
                    }
                    state.refresh_contacts();
                }
                "trust" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device trust: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device trust: missing device id");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_device")],
                        );
                        return false;
                    };
                    let confirmed = cmd
                        .args
                        .get(4)
                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                        .unwrap_or(false);
                    if !confirmed {
                        state.set_command_error("contacts device trust: confirmation required");
                        emit_marker(
                            "tui_contacts_device_trust",
                            Some("confirm_required"),
                            &[("label", label), ("device", device_id), ("ok", "false")],
                        );
                        return false;
                    }
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("contacts device trust: unknown alias");
                        emit_marker(
                            "tui_contacts_device_trust",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    let Some(idx) = contact_device_find_index(rec, device_id) else {
                        state.set_command_error("contacts device trust: unknown device id");
                        emit_marker(
                            "tui_contacts_device_trust",
                            Some("device_unknown"),
                            &[("label", label), ("device", device_id), ("ok", "false")],
                        );
                        return false;
                    };
                    rec.devices[idx].state = "TRUSTED".to_string();
                    rec.status = "PINNED".to_string();
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("contacts device trust: store unavailable");
                        emit_marker(
                            "tui_contacts_device_trust",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    emit_marker(
                        "tui_contacts_device_trust",
                        None,
                        &[
                            ("label", label),
                            ("device", device_id),
                            ("state", "TRUSTED"),
                            ("ok", "true"),
                        ],
                    );
                    emit_tui_contact_flow(
                        "trust",
                        "TRUSTED",
                        label,
                        Some(device_id),
                        state.trust_onboarding_mode,
                    );
                    emit_tui_trust_promotion(
                        "trusted",
                        "explicit_operator_action",
                        label,
                        Some(device_id),
                        state.trust_onboarding_mode,
                    );
                    state.set_command_feedback("ok: device trusted (allowed to send)");
                    state.refresh_contacts();
                }
                "revoke" => {
                    let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device revoke: missing alias");
                        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                        return false;
                    };
                    let Some(device_id) = cmd.args.get(3).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device revoke: missing device id");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_device")],
                        );
                        return false;
                    };
                    let confirmed = cmd
                        .args
                        .get(4)
                        .map(|s| s.eq_ignore_ascii_case("confirm"))
                        .unwrap_or(false);
                    if !confirmed {
                        state.set_command_error("contacts device revoke: confirmation required");
                        emit_marker(
                            "tui_contacts_device_revoke",
                            Some("confirm_required"),
                            &[("label", label), ("device", device_id), ("ok", "false")],
                        );
                        return false;
                    }
                    let Some(rec) = state.contacts_records.get_mut(label) else {
                        state.set_command_error("contacts device revoke: unknown alias");
                        emit_marker(
                            "tui_contacts_device_revoke",
                            Some("peer_unknown"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    };
                    normalize_contact_record(label, rec);
                    let Some(idx) = contact_device_find_index(rec, device_id) else {
                        state.set_command_error("contacts device revoke: unknown device id");
                        emit_marker(
                            "tui_contacts_device_revoke",
                            Some("device_unknown"),
                            &[("label", label), ("device", device_id), ("ok", "false")],
                        );
                        return false;
                    };
                    rec.devices[idx].state = "REVOKED".to_string();
                    if state.persist_contacts_cache().is_err() {
                        state.set_command_error("contacts device revoke: store unavailable");
                        emit_marker(
                            "tui_contacts_device_revoke",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                    emit_marker(
                        "tui_contacts_device_revoke",
                        None,
                        &[
                            ("label", label),
                            ("device", device_id),
                            ("state", "REVOKED"),
                            ("ok", "true"),
                        ],
                    );
                    state.refresh_contacts();
                }
                "primary" => {
                    let Some(primary_action) = cmd.args.get(2).map(|s| s.as_str()) else {
                        state.set_command_error("contacts device primary: missing action");
                        emit_marker(
                            "tui_contacts_invalid",
                            None,
                            &[("reason", "missing_primary_action")],
                        );
                        return false;
                    };
                    match primary_action {
                        "set" => {
                            let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device primary set: missing alias",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(device_id) = cmd.args.get(4).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device primary set: missing device id",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_device")],
                                );
                                return false;
                            };
                            let confirmed = cmd
                                .args
                                .get(5)
                                .map(|s| s.eq_ignore_ascii_case("confirm"))
                                .unwrap_or(false);
                            if !confirmed {
                                state.set_command_error(
                                    "contacts device primary set: confirmation required",
                                );
                                emit_marker(
                                    "tui_contacts_device_primary_set",
                                    Some("confirm_required"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            }
                            let Some(rec) = state.contacts_records.get_mut(label) else {
                                state.set_command_error(
                                    "contacts device primary set: unknown alias",
                                );
                                emit_marker(
                                    "tui_contacts_device_primary_set",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            normalize_contact_record(label, rec);
                            let Some(_) = contact_device_find_index(rec, device_id) else {
                                state.set_command_error(
                                    "contacts device primary set: unknown device id",
                                );
                                emit_marker(
                                    "tui_contacts_device_primary_set",
                                    Some("device_unknown"),
                                    &[("label", label), ("device", device_id), ("ok", "false")],
                                );
                                return false;
                            };
                            rec.primary_device_id = Some(device_id.to_string());
                            normalize_contact_record(label, rec);
                            if state.persist_contacts_cache().is_err() {
                                state.set_command_error(
                                    "contacts device primary set: store unavailable",
                                );
                                emit_marker(
                                    "tui_contacts_device_primary_set",
                                    Some("contacts_store_unavailable"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            }
                            emit_marker(
                                "tui_contacts_device_primary_set",
                                None,
                                &[
                                    ("label", label),
                                    ("device", device_id),
                                    ("selected", "explicit"),
                                    ("policy", "primary_only"),
                                    ("ok", "true"),
                                ],
                            );
                            state.refresh_contacts();
                        }
                        "show" => {
                            let Some(label) = cmd.args.get(3).map(|s| s.as_str()) else {
                                state.set_command_error(
                                    "contacts device primary show: missing alias",
                                );
                                emit_marker(
                                    "tui_contacts_invalid",
                                    None,
                                    &[("reason", "missing_label")],
                                );
                                return false;
                            };
                            let Some(rec) = state.contacts_records.get(label) else {
                                state.set_command_error(
                                    "contacts device primary show: unknown alias",
                                );
                                emit_marker(
                                    "tui_contacts_device_primary_show",
                                    Some("peer_unknown"),
                                    &[("label", label), ("ok", "false")],
                                );
                                return false;
                            };
                            let mut rec = rec.clone();
                            let implicit = rec.primary_device_id.is_none();
                            normalize_contact_record(label, &mut rec);
                            let primary = primary_device(&rec)
                                .map(|d| d.device_id.as_str())
                                .unwrap_or("none")
                                .to_string();
                            emit_marker(
                                "tui_contacts_device_primary_show",
                                None,
                                &[
                                    ("label", label),
                                    ("device", primary.as_str()),
                                    ("selected", if implicit { "implicit" } else { "explicit" }),
                                    ("policy", "primary_only"),
                                    ("ok", "true"),
                                ],
                            );
                        }
                        _ => {
                            state.set_command_error("contacts device primary: unknown action");
                            emit_marker(
                                "tui_contacts_invalid",
                                None,
                                &[("reason", "unknown_primary_action")],
                            );
                            return false;
                        }
                    }
                }
                _ => {
                    state.set_command_error("contacts device: unknown action");
                    emit_marker(
                        "tui_contacts_invalid",
                        None,
                        &[("reason", "unknown_device_action")],
                    );
                    return false;
                }
            }
        }
        "block" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing label");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            if let Some(rec) = state.contacts_records.get_mut(label) {
                rec.blocked = true;
                match state.persist_contacts_cache() {
                    Ok(()) => emit_marker(
                        "tui_contacts_block",
                        None,
                        &[("label", label), ("ok", "true")],
                    ),
                    Err(_) => {
                        state.set_command_error("contacts: store unavailable");
                        emit_marker(
                            "tui_contacts_block",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                }
            } else {
                state.set_command_error("contacts: unknown alias");
                emit_marker(
                    "tui_contacts_block",
                    Some("peer_unknown"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            }
            state.refresh_contacts();
        }
        "unblock" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing label");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            if let Some(rec) = state.contacts_records.get_mut(label) {
                rec.blocked = false;
                match state.persist_contacts_cache() {
                    Ok(()) => emit_marker(
                        "tui_contacts_unblock",
                        None,
                        &[("label", label), ("ok", "true")],
                    ),
                    Err(_) => {
                        state.set_command_error("contacts: store unavailable");
                        emit_marker(
                            "tui_contacts_unblock",
                            Some("contacts_store_unavailable"),
                            &[("label", label), ("ok", "false")],
                        );
                        return false;
                    }
                }
            } else {
                state.set_command_error("contacts: unknown alias");
                emit_marker(
                    "tui_contacts_unblock",
                    Some("peer_unknown"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            }
            state.refresh_contacts();
        }
        "add" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing label");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            if !tui_alias_is_valid(label) {
                state.set_command_error("contacts: alias must be 2-32 chars [A-Za-z0-9._-]");
                emit_marker("tui_contacts_invalid", None, &[("reason", "alias_invalid")]);
                return false;
            }
            let Some(code) = cmd.args.get(2).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing verification code");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
                return false;
            };
            if !tui_verification_code_is_valid(code) {
                state.set_command_error("contacts: invalid verification code format");
                emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
                return false;
            }
            let route_token = match cmd.args.get(3).map(|s| s.as_str()) {
                Some(raw) => match normalize_route_token(raw) {
                    Ok(token) => Some(token),
                    Err(code) => {
                        state.set_command_error(format!("contacts: {}", code));
                        emit_marker(
                            "tui_contacts_invalid",
                            Some(code),
                            &[("reason", "invalid_route_token")],
                        );
                        return false;
                    }
                },
                None => Some(generate_route_token()),
            };
            let rec = ContactRecord {
                fp: code.to_ascii_uppercase(),
                status: "UNVERIFIED".to_string(),
                blocked: false,
                seen_at: None,
                sig_fp: None,
                route_token: route_token.clone(),
                primary_device_id: None,
                devices: vec![ContactDeviceRecord {
                    device_id: device_id_short(label, None, code),
                    fp: code.to_ascii_uppercase(),
                    sig_fp: None,
                    state: "UNVERIFIED".to_string(),
                    route_token: route_token.clone(),
                    seen_at: None,
                    label: None,
                }],
            };
            state.contacts_records.insert(label.to_string(), rec);
            match state.persist_contacts_cache() {
                Ok(()) => emit_marker(
                    "tui_contacts_add",
                    None,
                    &[("label", label), ("ok", "true"), ("status", "UNVERIFIED")],
                ),
                Err(_) => {
                    state.set_command_error("contacts: store unavailable");
                    emit_marker(
                        "tui_contacts_add",
                        Some("contacts_store_unavailable"),
                        &[("label", label), ("ok", "false")],
                    );
                    return false;
                }
            }
            emit_tui_contact_flow(
                "add",
                "DISCOVERED",
                label,
                None,
                state.trust_onboarding_mode,
            );
            state.refresh_contacts();
        }
        "route" => {
            let Some(action) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing route subcommand");
                emit_marker(
                    "tui_contacts_invalid",
                    None,
                    &[("reason", "missing_route_subcmd")],
                );
                return false;
            };
            if action != "set" {
                state.set_command_error("contacts: unknown route subcommand");
                emit_marker(
                    "tui_contacts_invalid",
                    None,
                    &[("reason", "unknown_route_subcmd")],
                );
                return false;
            }
            let Some(label) = cmd.args.get(2).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing alias");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            let Some(raw_token) = cmd.args.get(3).map(|s| s.as_str()) else {
                state.set_command_error("contacts: missing route token");
                emit_marker("tui_contacts_invalid", None, &[("reason", "missing_token")]);
                return false;
            };
            let token = match normalize_route_token(raw_token) {
                Ok(v) => v,
                Err(code) => {
                    state.set_command_error(format!("contacts: {}", code));
                    emit_marker("tui_contacts_invalid", Some(code), &[]);
                    return false;
                }
            };
            let rec = state
                .contacts_records
                .entry(label.to_string())
                .or_insert(ContactRecord {
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
            let primary_route_token = rec.route_token.clone();
            if let Some(primary) = primary_device_mut(rec) {
                primary.route_token = primary_route_token;
            }
            if state.persist_contacts_cache().is_err() {
                state.set_command_error("contacts: store unavailable");
                emit_marker(
                    "tui_contacts_route",
                    Some("contacts_store_unavailable"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            }
            emit_marker(
                "tui_contacts_route",
                None,
                &[("label", label), ("ok", "true"), ("action", "set")],
            );
            state.push_cmd_result(
                "contacts route set",
                true,
                "contact route token stored (redacted)",
            );
            state.set_status_last_command_result(format!("contact route set {}", label));
            state.set_command_feedback("ok: contact route token set");
            state.refresh_contacts();
        }
        _ => {
            state.set_command_error("contacts: unknown subcommand");
            emit_marker(
                "tui_contacts_invalid",
                None,
                &[("reason", "unknown_subcmd")],
            );
        }
    }
    false
}

fn handle_trust_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "trust")]);
    let Some(action) = cmd.args.first().map(|s| s.as_str()) else {
        state.set_command_error("trust: missing action (use pin)");
        emit_marker("tui_trust_invalid", None, &[("reason", "missing_action")]);
        return false;
    };
    match action {
        "pin" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("trust: missing alias");
                emit_marker("tui_trust_invalid", None, &[("reason", "missing_label")]);
                return false;
            };
            let confirmed = cmd
                .args
                .get(2)
                .map(|s| s.eq_ignore_ascii_case("confirm"))
                .unwrap_or(false);
            if !confirmed {
                state.set_command_error(
                    "trust: confirmation required (use '/trust pin <alias> confirm')",
                );
                emit_marker(
                    "tui_trust_pin",
                    Some("confirm_required"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            }
            let Some(rec) = state.contacts_records.get_mut(label) else {
                state.set_command_error("trust: unknown alias");
                emit_marker(
                    "tui_trust_pin",
                    Some("peer_unknown"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            };
            normalize_contact_record(label, rec);
            rec.status = "PINNED".to_string();
            if let Some(primary) = primary_device_mut(rec) {
                primary.state = "TRUSTED".to_string();
            }
            if state.persist_contacts_cache().is_err() {
                state.set_command_error("trust: store unavailable");
                emit_marker(
                    "tui_trust_pin",
                    Some("contacts_store_unavailable"),
                    &[("label", label), ("ok", "false")],
                );
                return false;
            }
            state.push_cmd_result("trust pin", true, "contact pinned");
            state.set_status_last_command_result(format!("trust pinned {}", label));
            state.set_command_feedback("ok: contact trusted (allowed to send)");
            emit_marker(
                "tui_trust_pin",
                None,
                &[("label", label), ("ok", "true"), ("status", "PINNED")],
            );
            state.refresh_contacts();
        }
        "mode" => {
            let mode_arg = cmd.args.get(1).map(|s| s.as_str());
            match mode_arg {
                None | Some("show") => {
                    let mode = state.trust_onboarding_mode.as_str();
                    emit_tui_named_marker("QSC_TUI_TRUST_MODE", &[("mode", mode)]);
                    state.push_cmd_result("trust mode", true, format!("mode={mode}"));
                }
                Some(raw) => {
                    let Some(mode) = TrustOnboardingMode::from_raw(raw) else {
                        state.set_command_error("trust mode: expected strict|balanced");
                        emit_tui_named_marker(
                            "QSC_TUI_TRUST_MODE",
                            &[("ok", "false"), ("reason", "invalid_mode")],
                        );
                        return false;
                    };
                    if state
                        .persist_account_secret(TUI_TRUST_MODE_SECRET_KEY, mode.as_str())
                        .is_err()
                    {
                        state.set_command_error("trust mode: store unavailable");
                        emit_tui_named_marker(
                            "QSC_TUI_TRUST_MODE",
                            &[("ok", "false"), ("reason", "contacts_store_unavailable")],
                        );
                        return false;
                    }
                    state.trust_onboarding_mode = mode;
                    emit_tui_named_marker(
                        "QSC_TUI_TRUST_MODE",
                        &[("mode", mode.as_str()), ("ok", "true")],
                    );
                    state.push_cmd_result("trust mode", true, format!("mode={}", mode.as_str()));
                }
            }
        }
        _ => {
            state.set_command_error("trust: unknown action");
            emit_marker("tui_trust_invalid", None, &[("reason", "unknown_action")]);
        }
    }
    false
}

fn handle_requests_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "requests")]);
    let action = cmd.args.first().map(|s| s.as_str()).unwrap_or("list");
    match action {
        "list" => {
            let items = contact_request_list().unwrap_or_default();
            let count_s = items.len().to_string();
            emit_tui_contact_request("list", "all", None);
            state.push_cmd_result("requests list", true, format!("count={count_s}"));
        }
        "accept" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("requests accept: missing alias");
                return false;
            };
            let removed = contact_request_remove(label).unwrap_or(false);
            if !removed {
                state.set_command_error("requests accept: unknown request");
                return false;
            }
            let mut rec = state
                .contacts_records
                .get(label)
                .cloned()
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
                        label: Some("request".to_string()),
                    }],
                });
            normalize_contact_record(label, &mut rec);
            rec.status = "UNVERIFIED".to_string();
            if state.persist_contacts_cache_with(label, rec).is_err() {
                state.set_command_error("requests accept: store unavailable");
                return false;
            }
            state.refresh_contacts();
            emit_tui_contact_request("accept", label, None);
            emit_tui_contact_flow(
                "add",
                "DISCOVERED",
                label,
                None,
                state.trust_onboarding_mode,
            );
            state.push_cmd_result("requests accept", true, label.to_string());
        }
        "ignore" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("requests ignore: missing alias");
                return false;
            };
            let removed = contact_request_remove(label).unwrap_or(false);
            if !removed {
                state.set_command_error("requests ignore: unknown request");
                return false;
            }
            emit_tui_contact_request("ignore", label, None);
            state.push_cmd_result("requests ignore", true, label.to_string());
        }
        "block" => {
            let Some(label) = cmd.args.get(1).map(|s| s.as_str()) else {
                state.set_command_error("requests block: missing alias");
                return false;
            };
            let _ = contact_request_remove(label);
            let mut rec = state
                .contacts_records
                .get(label)
                .cloned()
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
            if state.persist_contacts_cache_with(label, rec).is_err() {
                state.set_command_error("requests block: store unavailable");
                return false;
            }
            state.refresh_contacts();
            emit_tui_contact_request("block", label, None);
            state.push_cmd_result("requests block", true, label.to_string());
        }
        _ => {
            state.set_command_error("requests: unknown action");
            return false;
        }
    }
    false
}

fn handle_verify_command(cmd: &TuiParsedCmd, state: &mut TuiState) -> bool {
    emit_marker("tui_cmd", None, &[("cmd", "verify")]);
    let Some(label) = cmd.args.first().map(|s| s.as_str()) else {
        state.set_command_error("verify: missing alias");
        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_label")]);
        return false;
    };
    let Some(code) = cmd.args.get(1).map(|s| s.as_str()) else {
        state.set_command_error("verify: missing verification code");
        emit_marker("tui_contacts_invalid", None, &[("reason", "missing_fp")]);
        return false;
    };
    if !tui_verification_code_is_valid(code) {
        state.set_command_error("verify: invalid verification code format");
        emit_marker("tui_contacts_invalid", None, &[("reason", "invalid_code")]);
        return false;
    }
    let Some(rec) = state.contacts_records.get_mut(label) else {
        state.set_command_error("verify: unknown alias");
        emit_marker(
            "tui_contacts_verify",
            Some("peer_unknown"),
            &[("label", label), ("ok", "false")],
        );
        return false;
    };
    normalize_contact_record(label, rec);
    let expected = primary_device(rec)
        .map(|d| d.fp.to_ascii_uppercase())
        .unwrap_or_else(|| rec.fp.to_ascii_uppercase());
    let provided = code.to_ascii_uppercase();
    if expected == provided {
        let mode = state.trust_onboarding_mode;
        rec.status = "VERIFIED".to_string();
        if let Some(primary) = primary_device_mut(rec) {
            primary.state = "VERIFIED".to_string();
            if mode == TrustOnboardingMode::Balanced {
                primary.state = "TRUSTED".to_string();
                rec.status = "PINNED".to_string();
            }
        }
        if state.persist_contacts_cache().is_err() {
            state.set_command_error("verify: store unavailable");
            emit_marker(
                "tui_contacts_verify",
                Some("contacts_store_unavailable"),
                &[("label", label), ("ok", "false")],
            );
            return false;
        }
        emit_marker(
            "tui_contacts_verify",
            None,
            &[
                ("label", label),
                ("ok", "true"),
                (
                    "status",
                    if mode == TrustOnboardingMode::Balanced {
                        "TRUSTED"
                    } else {
                        "VERIFIED"
                    },
                ),
            ],
        );
        emit_tui_contact_flow(
            "verify",
            if mode == TrustOnboardingMode::Balanced {
                "TRUSTED"
            } else {
                "VERIFIED"
            },
            label,
            None,
            mode,
        );
        if mode == TrustOnboardingMode::Balanced {
            emit_tui_trust_promotion("trusted", "verified_match", label, None, mode);
            state.set_command_feedback("ok: verification matched and contact auto-trusted");
        } else {
            emit_tui_trust_promotion("verified_only", "strict_mode", label, None, mode);
            state.set_command_feedback(
                "ok: verification matched; strict mode requires explicit trust",
            );
        }
    } else {
        rec.status = "CHANGED".to_string();
        if let Some(primary) = primary_device_mut(rec) {
            primary.state = "CHANGED".to_string();
        }
        let _ = state.persist_contacts_cache();
        emit_marker(
            "tui_contacts_verify",
            Some("verification_mismatch"),
            &[("label", label), ("ok", "false"), ("status", "CHANGED")],
        );
        emit_tui_contact_flow(
            "verify",
            "CHANGED",
            label,
            None,
            state.trust_onboarding_mode,
        );
        state.set_command_error("verify: verification code mismatch");
        return false;
    }
    state.refresh_contacts();
    false
}
