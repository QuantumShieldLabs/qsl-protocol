use super::*;

pub(super) fn send_execute(args: SendExecuteArgs) {
    if !require_unlocked("send") {
        return;
    }
    let SendExecuteArgs {
        transport,
        relay,
        to,
        file,
        pad_to,
        pad_bucket,
        bucket_max,
        meta_seed,
        receipt,
    } = args;
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("send_transport_required"),
    };

    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("send_relay_required"),
            };
            let to = match to {
                Some(v) => v,
                None => print_error_marker("send_to_required"),
            };
            let file = match file {
                Some(v) => v,
                None => print_error_marker("send_file_required"),
            };
            let pad_cfg = match meta_pad_config_from_args(pad_to, pad_bucket, meta_seed) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            if let Err(code) = enforce_cli_send_contact_trust(to.as_str()) {
                print_error_marker(code);
            }
            if let Err(code) = enforce_peer_not_blocked(to.as_str()) {
                print_error_marker(code);
            }
            if let Err(reason) = protocol_active_or_reason_for_send_peer(to.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }
            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            if receipt.is_none() {
                emit_marker("receipt_disabled", None, &[]);
            }
            relay_send(&to, &file, &relay, pad_cfg, bucket_max, meta_seed, receipt);
        }
    }
}

pub(super) fn send_abort() {
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if let Err(e) = enforce_safe_parents(&outbox_path, source) {
        print_error(e);
    }

    if outbox_path.exists() {
        let outbox = outbox_record_load(&outbox_path).unwrap_or_else(|e| print_error_marker(e));
        if outbox.to.is_empty() {
            print_error_marker("outbox_recovery_required");
        }
        let next_state = outbox_next_state_load().unwrap_or_else(|e| print_error_marker(e));
        if qsp_session_store(
            outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
            &next_state,
        )
        .is_err()
        {
            print_error_marker("qsp_session_store_failed");
        }
        let next_seq = match read_send_state(&dir, source) {
            Ok(v) => v + 1,
            Err(()) => print_error_marker("send_state_parse_failed"),
        };
        let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
        if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
            print_error_marker("send_commit_write_failed");
        }
        if fs::remove_file(&outbox_path).is_err() {
            print_error_marker("outbox_abort_failed");
        }
        if let Err(code) = outbox_next_state_clear() {
            print_error_marker(code);
        }
        let seq_s = next_seq.to_string();
        emit_marker(
            "outbox_abort",
            None,
            &[
                ("ok", "true"),
                ("action", "burned"),
                ("send_seq", seq_s.as_str()),
            ],
        );
    } else {
        let _ = outbox_next_state_clear();
        emit_marker(
            "outbox_abort",
            None,
            &[("ok", "true"), ("action", "absent")],
        );
    }
}

fn outbox_record_load(path: &Path) -> Result<OutboxRecord, &'static str> {
    let bytes = fs::read(path).map_err(|_| "outbox_read_failed")?;
    serde_json::from_slice(&bytes).map_err(|_| "outbox_parse_failed")
}

fn outbox_next_state_store(st: &Suite2SessionState) -> Result<(), &'static str> {
    let bytes = st.snapshot_bytes();
    let secret = hex_encode(&bytes);
    match vault::secret_set(OUTBOX_NEXT_STATE_SECRET_KEY, &secret) {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("outbox_state_vault_unavailable"),
        Err(_) => Err("outbox_state_store_failed"),
    }
}

fn outbox_next_state_load() -> Result<Suite2SessionState, &'static str> {
    let Some(secret) =
        vault::secret_get(OUTBOX_NEXT_STATE_SECRET_KEY).map_err(|_| "outbox_state_read_failed")?
    else {
        return Err("outbox_state_missing");
    };
    if secret.is_empty() {
        return Err("outbox_state_missing");
    }
    let bytes = hex_decode(secret.as_str()).map_err(|_| "outbox_state_parse_failed")?;
    Suite2SessionState::restore_bytes(&bytes).map_err(|_| "outbox_state_parse_failed")
}

fn outbox_next_state_clear() -> Result<(), &'static str> {
    match vault::secret_set(OUTBOX_NEXT_STATE_SECRET_KEY, "") {
        Ok(()) => Ok(()),
        Err("vault_missing" | "vault_locked") => Err("outbox_state_vault_unavailable"),
        Err(_) => Err("outbox_state_clear_failed"),
    }
}

pub(super) fn receive_execute(args: ReceiveArgs) {
    if !require_unlocked("receive") {
        return;
    }
    let ReceiveArgs {
        transport,
        relay,
        legacy_receive_mode,
        attachment_service,
        from,
        mailbox,
        max,
        max_file_size,
        max_file_chunks,
        out,
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        poll_ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
        emit_receipts,
        receipt_mode,
        receipt_batch_window_ms,
        receipt_jitter_ms,
        file_confirm_mode,
    } = args;
    let receipt_policy = resolve_receipt_policy(ReceiptPolicyOverrides {
        emit_receipts,
        receipt_mode,
        receipt_batch_window_ms,
        receipt_jitter_ms,
        file_confirm_mode,
    });
    let batch_window_s = receipt_policy.batch_window_ms.to_string();
    let jitter_s = receipt_policy.jitter_ms.to_string();
    emit_marker(
        "receipt_policy",
        None,
        &[
            ("mode", receipt_policy.mode.as_str()),
            ("batch_window_ms", batch_window_s.as_str()),
            ("jitter_ms", jitter_s.as_str()),
            (
                "file_confirm_mode",
                receipt_policy.file_confirm_mode.as_str(),
            ),
        ],
    );
    let transport = match transport {
        Some(v) => v,
        None => print_error_marker("recv_transport_required"),
    };
    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => print_error_marker("recv_relay_required"),
            };
            let attachment_service = attachment_service.map(|v| {
                normalize_relay_endpoint(v.as_str()).unwrap_or_else(|code| print_error_marker(code))
            });
            let legacy_receive_mode =
                resolve_legacy_receive_mode(legacy_receive_mode, attachment_service.as_deref())
                    .unwrap_or_else(|code| print_error_marker(code));
            if let Err(code) = normalize_relay_endpoint(relay.as_str()) {
                print_error_marker(code);
            }
            let from = match from {
                Some(v) => v,
                None => print_error_marker("recv_from_required"),
            };
            let mailbox = match mailbox {
                Some(raw) => normalize_route_token(raw.as_str())
                    .unwrap_or_else(|code| print_error_marker(code)),
                None => {
                    relay_self_inbox_route_token().unwrap_or_else(|code| print_error_marker(code))
                }
            };
            let max = match max {
                Some(v) if v > 0 => v,
                _ => print_error_marker("recv_max_required"),
            };
            let max_file_size = match max_file_size {
                Some(v) if v > 0 && v <= ATTACHMENT_DEFAULT_MAX_FILE_SIZE => v,
                Some(_) => print_error_marker("recv_file_size_bound_invalid"),
                None => {
                    if attachment_service.is_some() {
                        ATTACHMENT_DEFAULT_MAX_FILE_SIZE
                    } else {
                        FILE_XFER_DEFAULT_MAX_FILE_SIZE
                    }
                }
            };
            let max_file_chunks = match max_file_chunks {
                Some(v) if v > 0 && v <= ATTACHMENT_DEFAULT_MAX_PARTS => v,
                Some(_) => print_error_marker("recv_file_chunks_bound_invalid"),
                None => {
                    if attachment_service.is_some() {
                        ATTACHMENT_DEFAULT_MAX_PARTS
                    } else {
                        FILE_XFER_DEFAULT_MAX_CHUNKS
                    }
                }
            };
            let out = match out {
                Some(v) => v,
                None => print_error_marker("recv_out_required"),
            };
            let poll_cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta,
                interval_ms,
                poll_interval_ms,
                ticks: poll_ticks,
                batch_max_count,
                poll_max_per_tick,
                bucket_max,
                meta_seed,
            }) {
                Ok(v) => v,
                Err(code) => print_error_marker(code),
            };
            let source = ConfigSource::EnvOverride;
            if let Err(e) = ensure_dir_secure(&out, source) {
                print_error(e);
            }
            let (cfg_dir, cfg_source) = match config_dir() {
                Ok(v) => v,
                Err(e) => print_error(e),
            };
            if !check_symlink_safe(&cfg_dir) {
                print_error(ErrorCode::UnsafePathSymlink);
            }
            if !check_parent_safe(&cfg_dir, cfg_source) {
                print_error(ErrorCode::UnsafeParentPerms);
            }
            if let Err(reason) = protocol_active_or_reason_for_peer(from.as_str()) {
                protocol_inactive_exit(reason.as_str());
            }

            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            let recv_max = poll_cfg.as_ref().map(|c| c.batch_max_count).unwrap_or(max);
            let max_s = recv_max.to_string();
            let mailbox_hash = route_token_hash8(mailbox.as_str());
            emit_marker(
                "recv_start",
                None,
                &[
                    ("transport", "relay"),
                    ("mailbox", "redacted"),
                    ("mailbox_hash", mailbox_hash.as_str()),
                    ("from", from.as_str()),
                    ("max", max_s.as_str()),
                ],
            );
            let mut total = 0usize;
            if let Some(cfg) = poll_cfg {
                let interval_s = cfg.interval_ms.to_string();
                let ticks_s = cfg.ticks.to_string();
                let max_tick_s = cfg.batch_max_count.to_string();
                let bucket_max_s = cfg.bucket_max.to_string();
                emit_marker(
                    "meta_poll_config",
                    None,
                    &[
                        ("interval_ms", interval_s.as_str()),
                        ("ticks", ticks_s.as_str()),
                        ("batch_max_count", max_tick_s.as_str()),
                        ("bucket_max", bucket_max_s.as_str()),
                    ],
                );
                for tick in 0..cfg.ticks {
                    let tick_s = tick.to_string();
                    let deterministic_s = if cfg.deterministic { "true" } else { "false" };
                    emit_marker(
                        "meta_tick",
                        None,
                        &[
                            ("tick", tick_s.as_str()),
                            ("interval_ms", interval_s.as_str()),
                            ("deterministic", deterministic_s),
                        ],
                    );
                    let pull = ReceivePullCtx {
                        relay: &relay,
                        legacy_receive_mode,
                        attachment_service: attachment_service.as_deref(),
                        mailbox: mailbox.as_str(),
                        from: &from,
                        out: &out,
                        source,
                        cfg_dir: &cfg_dir,
                        cfg_source,
                        bucket_max: cfg.bucket_max,
                        file_max_size: max_file_size,
                        file_max_chunks: max_file_chunks,
                        receipt_policy,
                    };
                    let stats = receive_pull_and_write(&pull, cfg.batch_max_count);
                    total = total.saturating_add(stats.count);
                    let count_s = stats.count.to_string();
                    let bytes_s = stats.bytes.to_string();
                    emit_marker(
                        "meta_batch",
                        None,
                        &[("count", count_s.as_str()), ("bytes", bytes_s.as_str())],
                    );
                    if !cfg.deterministic && cfg.interval_ms > 0 {
                        std::thread::sleep(Duration::from_millis(cfg.interval_ms));
                    }
                }
            } else {
                let pull = ReceivePullCtx {
                    relay: &relay,
                    legacy_receive_mode,
                    attachment_service: attachment_service.as_deref(),
                    mailbox: mailbox.as_str(),
                    from: &from,
                    out: &out,
                    source,
                    cfg_dir: &cfg_dir,
                    cfg_source,
                    bucket_max: META_BUCKET_MAX_DEFAULT,
                    file_max_size: max_file_size,
                    file_max_chunks: max_file_chunks,
                    receipt_policy,
                };
                total = receive_pull_and_write(&pull, max).count;
            }
            if total == 0 {
                emit_marker("recv_none", None, &[]);
                return;
            }
            let count_s = total.to_string();
            emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
        }
    }
}

fn receive_pull_and_write(ctx: &ReceivePullCtx<'_>, max: usize) -> ReceivePullStats {
    let items = match relay_inbox_pull(ctx.relay, ctx.mailbox, max) {
        Ok(v) => v,
        Err(code) => print_error_marker(code),
    };
    let mut stats = ReceivePullStats { count: 0, bytes: 0 };
    let mut pending_receipts: Vec<PendingReceipt> = Vec::new();
    for item in items {
        let envelope_len = item.data.len();
        match qsp_unpack_for_peer(ctx.from, &item.data) {
            Ok((outcome, channel)) => {
                let commit_unpack_state = || {
                    record_qsp_status(ctx.cfg_dir, ctx.cfg_source, true, "unpack_ok", false, true);
                    emit_marker("qsp_unpack", None, &[("ok", "true"), ("version", "5.0")]);
                    let msg_idx_s = outcome.msg_idx.to_string();
                    emit_marker(
                        "ratchet_recv_advance",
                        None,
                        &[("msg_idx", msg_idx_s.as_str())],
                    );
                    if outcome.skip_delta > 0 {
                        let sd = outcome.skip_delta.to_string();
                        emit_marker("ratchet_skip_store", None, &[("count", sd.as_str())]);
                    }
                    if outcome.evicted > 0 {
                        let ev = outcome.evicted.to_string();
                        emit_marker("ratchet_skip_evict", None, &[("count", ev.as_str())]);
                    }
                    if qsp_session_store(channel.as_str(), &outcome.next_state).is_err() {
                        emit_marker("error", Some("qsp_session_store_failed"), &[]);
                        print_error_marker("qsp_session_store_failed");
                    }
                };
                let mut payload = outcome.plaintext.clone();
                let mut request_receipt = false;
                let mut request_msg_id = String::new();
                if let Some(desc) = parse_attachment_descriptor_payload(&outcome.plaintext) {
                    let attachment_id = desc.attachment_id.clone();
                    match attachment_handle_descriptor(ctx, desc) {
                        Ok(Some((confirm_attachment_id, confirm_handle))) => {
                            commit_unpack_state();
                            queue_or_send_receipt(
                                ctx,
                                &mut pending_receipts,
                                PendingReceipt::AttachmentComplete {
                                    attachment_id: confirm_attachment_id,
                                    confirm_handle,
                                },
                            );
                        }
                        Ok(None) => {
                            commit_unpack_state();
                        }
                        Err(reason) => {
                            emit_marker(
                                "attachment_desc_reject",
                                Some(reason),
                                &[
                                    (
                                        "attachment_id",
                                        file_delivery_short_id(&attachment_id).as_str(),
                                    ),
                                    ("reason", reason),
                                ],
                            );
                            print_error_marker(reason);
                        }
                    }
                    continue;
                }
                if let Some(file_payload) = parse_file_transfer_payload(&outcome.plaintext) {
                    let file_id = match &file_payload {
                        FileTransferPayload::Chunk(v) => v.file_id.clone(),
                        FileTransferPayload::Manifest(v) => v.file_id.clone(),
                    };
                    if ctx.legacy_receive_mode == LegacyReceiveMode::Retired {
                        let payload_type = match &file_payload {
                            FileTransferPayload::Chunk(_) => "file_chunk",
                            FileTransferPayload::Manifest(_) => "file_manifest",
                        };
                        emit_marker(
                            "legacy_receive_reject",
                            Some("legacy_receive_retired_post_w0"),
                            &[
                                ("id", file_id.as_str()),
                                ("mode", legacy_receive_mode_name(ctx.legacy_receive_mode)),
                                ("payload_type", payload_type),
                                ("reason", "legacy_receive_retired_post_w0"),
                            ],
                        );
                        emit_marker(
                            "file_xfer_reject",
                            Some("legacy_receive_retired_post_w0"),
                            &[
                                ("id", file_id.as_str()),
                                ("reason", "legacy_receive_retired_post_w0"),
                            ],
                        );
                        print_error_marker("legacy_receive_retired_post_w0");
                    }
                    let file_res = match file_payload {
                        FileTransferPayload::Chunk(v) => {
                            file_transfer_handle_chunk(ctx, v).map(|_| None)
                        }
                        FileTransferPayload::Manifest(v) => file_transfer_handle_manifest(ctx, v),
                    };
                    match file_res {
                        Ok(Some((confirm_file_id, confirm_id))) => {
                            commit_unpack_state();
                            queue_or_send_receipt(
                                ctx,
                                &mut pending_receipts,
                                PendingReceipt::FileComplete {
                                    file_id: confirm_file_id,
                                    confirm_id,
                                },
                            );
                        }
                        Ok(None) => {
                            commit_unpack_state();
                        }
                        Err(reason) => {
                            if reason == "manifest_mismatch" {
                                let _ =
                                    file_transfer_fail_clean(ctx.from, file_id.as_str(), reason);
                            }
                            emit_marker(
                                "file_xfer_reject",
                                Some(reason),
                                &[("id", file_id.as_str()), ("reason", reason)],
                            );
                            print_error_marker(reason);
                        }
                    }
                    continue;
                }
                if let Some(confirm) = parse_attachment_confirm_payload(&outcome.plaintext) {
                    commit_unpack_state();
                    match apply_attachment_peer_confirmation(
                        ctx.from,
                        confirm.attachment_id.as_str(),
                        confirm.confirm_handle.as_str(),
                        channel.as_str(),
                    ) {
                        Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                            let device = target
                                .as_deref()
                                .or_else(|| channel_device_id(channel.as_str()));
                            emit_marker(
                                "attachment_confirm_recv",
                                None,
                                &[("attachment_id", "redacted"), ("ok", "true")],
                            );
                            emit_cli_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                confirm.attachment_id.as_str(),
                                device,
                            );
                            emit_tui_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                confirm.attachment_id.as_str(),
                                device,
                            );
                        }
                        Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                            let dev = channel_device_marker(channel.as_str());
                            emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                        }
                        Err(reason) => emit_marker(
                            "attachment_confirm_reject",
                            Some(reason),
                            &[("reason", reason), ("ok", "false")],
                        ),
                    }
                    continue;
                }
                if let Some(file_confirm) = parse_file_confirm_payload(&outcome.plaintext) {
                    commit_unpack_state();
                    match apply_file_peer_confirmation(
                        ctx.from,
                        file_confirm.file_id.as_str(),
                        file_confirm.confirm_id.as_str(),
                        channel.as_str(),
                    ) {
                        Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                            let device = target
                                .as_deref()
                                .or_else(|| channel_device_id(channel.as_str()));
                            emit_marker(
                                "file_confirm_recv",
                                None,
                                &[
                                    ("kind", "coarse_complete"),
                                    ("file_id", "redacted"),
                                    ("ok", "true"),
                                ],
                            );
                            emit_cli_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                file_confirm.file_id.as_str(),
                                device,
                            );
                            emit_tui_file_delivery_with_device(
                                ctx.from,
                                "peer_confirmed",
                                file_confirm.file_id.as_str(),
                                device,
                            );
                        }
                        Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                            let dev = channel_device_marker(channel.as_str());
                            emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                        }
                        Err(reason) => emit_marker(
                            "file_confirm_reject",
                            Some(reason),
                            &[("reason", reason), ("ok", "false")],
                        ),
                    }
                    continue;
                }
                if let Some(ctrl) = parse_receipt_payload(&outcome.plaintext) {
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "ack" {
                        commit_unpack_state();
                        match apply_message_peer_confirmation(
                            ctx.from,
                            ctrl.msg_id.as_str(),
                            channel.as_str(),
                        ) {
                            Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                                let dev = channel_device_marker(channel.as_str());
                                emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                                emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                            }
                            Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                                let device = target
                                    .as_deref()
                                    .or_else(|| channel_device_id(channel.as_str()));
                                emit_marker(
                                    "receipt_recv",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                                emit_marker(
                                    "delivered_to_peer",
                                    None,
                                    &[("kind", "delivered"), ("msg_id", "<redacted>")],
                                );
                                emit_cli_delivery_state_with_device(
                                    ctx.from,
                                    "peer_confirmed",
                                    device,
                                );
                                emit_tui_delivery_state_with_device(
                                    ctx.from,
                                    "peer_confirmed",
                                    device,
                                );
                            }
                            Err(reason) => emit_message_state_reject(ctrl.msg_id.as_str(), reason),
                        }
                        continue;
                    }
                    if ctrl.v == 1 && ctrl.kind == "delivered" && ctrl.t == "data" {
                        if let Some(body) = ctrl.body {
                            payload = body;
                            request_receipt = true;
                            request_msg_id = ctrl.msg_id;
                        }
                    }
                }
                commit_unpack_state();
                stats.count = stats.count.saturating_add(1);
                stats.bytes = stats.bytes.saturating_add(envelope_len);
                let bucket = meta_bucket_for_len(envelope_len, ctx.bucket_max);
                let bucket_s = bucket.to_string();
                let orig_s = envelope_len.to_string();
                let capped_s = if envelope_len > ctx.bucket_max {
                    ctx.bucket_max.to_string()
                } else {
                    envelope_len.to_string()
                };
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_s.as_str()),
                        ("orig", orig_s.as_str()),
                        ("capped", capped_s.as_str()),
                        ("metric", "envelope_len"),
                    ],
                );
                let name = format!("recv_{}.bin", stats.count);
                let path = ctx.out.join(name);
                if write_atomic(&path, &payload, ctx.source).is_err() {
                    print_error_marker("recv_write_failed");
                }
                let idx_s = stats.count.to_string();
                let size_s = payload.len().to_string();
                emit_marker(
                    "recv_item",
                    None,
                    &[
                        ("idx", idx_s.as_str()),
                        ("size", size_s.as_str()),
                        ("id", item.id.as_str()),
                    ],
                );
                if let Err(code) = timeline_append_entry(
                    ctx.from,
                    "in",
                    payload.len(),
                    "msg",
                    MessageState::Received,
                    if request_msg_id.is_empty() {
                        None
                    } else {
                        Some(request_msg_id.as_str())
                    },
                ) {
                    emit_message_state_reject("<redacted>", code);
                    emit_marker("error", Some(code), &[("op", "timeline_receive_ingest")]);
                }
                if request_receipt {
                    queue_or_send_receipt(
                        ctx,
                        &mut pending_receipts,
                        PendingReceipt::Message {
                            msg_id: request_msg_id,
                        },
                    );
                }
            }
            Err(code) => {
                let from_alias = peer_alias_from_channel(ctx.from);
                if contacts_entry_read(from_alias).ok().flatten().is_none()
                    && channel_label_ok(from_alias)
                {
                    let _ = contact_request_upsert(from_alias, None, Some(code));
                    emit_cli_contact_request("created", from_alias, None);
                    emit_tui_contact_request("created", from_alias, None);
                }
                if code == "qsp_verify_failed" {
                    emit_file_integrity_fail(code, "rotate_mailbox_hint");
                }
                record_qsp_status(ctx.cfg_dir, ctx.cfg_source, false, code, false, false);
                emit_marker("qsp_unpack", Some(code), &[("ok", "false")]);
                if code == "qsp_replay_reject" {
                    let msg_idx = qsp_session_for_channel(ctx.from)
                        .map(|st| st.recv.nr.to_string())
                        .unwrap_or_else(|_| "0".to_string());
                    emit_marker("ratchet_replay_reject", None, &[("msg_idx", &msg_idx)]);
                }
                print_error_marker(code);
            }
        }
    }
    if let Some(service_url) = ctx.attachment_service {
        match attachment_resume_pending_for_peer(ctx, service_url) {
            Ok(resumed) => {
                stats.count = stats.count.saturating_add(resumed);
            }
            Err(reason) => {
                print_error_marker(reason);
            }
        }
    }
    flush_batched_receipts(ctx, &mut pending_receipts);
    stats
}

pub(super) fn relay_serve(port: u16, cfg: RelayConfig, max_messages: u64) {
    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let bound = listener
        .local_addr()
        .unwrap_or_else(|_| print_error_marker("relay_bind_failed"));
    let port_s = bound.port().to_string();
    let seed_s = cfg.seed.to_string();
    emit_marker(
        "relay_listen",
        None,
        &[("port", port_s.as_str()), ("seed", seed_s.as_str())],
    );

    let mut seq: u64 = 0;
    let mut inbox = RelayInboxStore::new(1024 * 1024, 1024);
    for stream in listener.incoming() {
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => continue,
        };
        seq = seq.wrapping_add(1);
        let seq_s = seq.to_string();
        let decision = relay_decide(&cfg, seq);
        if decision.delay_ms > 0 {
            let delay_s = decision.delay_ms.to_string();
            emit_marker(
                "relay_event",
                None,
                &[
                    ("action", "delay"),
                    ("ms", delay_s.as_str()),
                    ("seq", seq_s.as_str()),
                ],
            );
            std::thread::sleep(Duration::from_millis(decision.delay_ms));
        }

        if relay_try_handle_http_inbox(&mut stream, &mut inbox, &decision, seq_s.as_str()) {
            if max_messages > 0 && seq >= max_messages {
                break;
            }
            continue;
        }

        let frame: RelayFrame = match read_frame(&mut stream) {
            Ok(v) => v,
            Err(_) => {
                let resp = RelayResponse {
                    action: "reject".to_string(),
                    delivered: false,
                };
                let _ = write_frame(&mut stream, &resp);
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq_s.as_str())],
                );
                if max_messages > 0 && seq >= max_messages {
                    break;
                }
                continue;
            }
        };

        let _ = frame;
        emit_marker(
            "relay_event",
            None,
            &[("action", decision.action), ("seq", seq_s.as_str())],
        );
        let resp = RelayResponse {
            action: decision.action.to_string(),
            delivered: decision.delivered,
        };
        let _ = write_frame(&mut stream, &resp);

        if max_messages > 0 && seq >= max_messages {
            break;
        }
    }
}

fn relay_try_handle_http_inbox(
    stream: &mut TcpStream,
    store: &mut RelayInboxStore,
    decision: &RelayDecision,
    seq: &str,
) -> bool {
    let mut prefix = [0u8; 5];
    let Ok(n) = stream.peek(&mut prefix) else {
        return false;
    };
    let is_http = (n >= 4 && &prefix[..4] == b"GET ") || (n >= 5 && &prefix[..5] == b"POST ");
    if !is_http {
        return false;
    }
    let _ = stream.set_read_timeout(Some(Duration::from_millis(1500)));
    let req = match read_http_request(stream) {
        Ok(v) => v,
        Err(_) => {
            write_http_response(stream, 400, "text/plain", b"bad_request");
            emit_marker(
                "relay_event",
                None,
                &[("action", "reject"), ("seq", seq), ("proto", "http")],
            );
            return true;
        }
    };
    if decision.delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(decision.delay_ms));
    }
    match (req.method.as_str(), parse_http_target(req.target.as_str())) {
        ("POST", Some(HttpRelayTarget::Push)) => {
            let token = match parse_http_route_token(&req) {
                Ok(v) => v,
                Err(code) => {
                    write_http_response(stream, 400, "text/plain", code.as_bytes());
                    emit_marker(
                        "relay_event",
                        None,
                        &[("action", "reject"), ("seq", seq), ("proto", "http")],
                    );
                    return true;
                }
            };
            let content_len = req
                .headers
                .get("content-length")
                .and_then(|v| v.parse::<usize>().ok())
                .unwrap_or(req.body.len());
            if content_len != req.body.len() {
                write_http_response(stream, 400, "text/plain", b"content_length_mismatch");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            if req.body.len() > store.max_body {
                write_http_response(stream, 413, "text/plain", b"too_large");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            if decision.action == "drop" {
                write_http_response(stream, 503, "text/plain", b"dropped");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "drop"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            let queue = store.queues.entry(token).or_default();
            if queue.len() >= store.max_queue {
                write_http_response(stream, 429, "text/plain", b"queue_full");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "reject"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            queue.push_back((store.next_id, req.body.clone()));
            store.next_id = store.next_id.saturating_add(1);
            if decision.action == "dup" && queue.len() < store.max_queue {
                queue.push_back((store.next_id, req.body));
                store.next_id = store.next_id.saturating_add(1);
            }
            write_http_response(stream, 200, "text/plain", b"ok");
            emit_marker(
                "relay_event",
                None,
                &[("action", decision.action), ("seq", seq), ("proto", "http")],
            );
            true
        }
        ("GET", Some(HttpRelayTarget::Pull(max))) => {
            let token = match parse_http_route_token(&req) {
                Ok(v) => v,
                Err(code) => {
                    write_http_response(stream, 400, "text/plain", code.as_bytes());
                    emit_marker(
                        "relay_event",
                        None,
                        &[("action", "reject"), ("seq", seq), ("proto", "http")],
                    );
                    return true;
                }
            };
            if decision.action == "drop" {
                write_http_response(stream, 503, "text/plain", b"dropped");
                emit_marker(
                    "relay_event",
                    None,
                    &[("action", "drop"), ("seq", seq), ("proto", "http")],
                );
                return true;
            }
            let pull_max = max.clamp(1, 64);
            let queue = store.queues.entry(token).or_default();
            let mut items = Vec::new();
            for _ in 0..pull_max {
                let Some((id, data)) = queue.pop_front() else {
                    break;
                };
                items.push(InboxPullItem {
                    id: id.to_string(),
                    data,
                });
            }
            if items.is_empty() {
                write_http_response(stream, 204, "text/plain", b"");
            } else {
                let payload = serde_json::to_vec(&InboxPullResp { items })
                    .unwrap_or_else(|_| b"{\"items\":[]}".to_vec());
                write_http_response(stream, 200, "application/json", payload.as_slice());
            }
            emit_marker(
                "relay_event",
                None,
                &[("action", decision.action), ("seq", seq), ("proto", "http")],
            );
            true
        }
        _ => {
            write_http_response(stream, 404, "text/plain", b"not_found");
            emit_marker(
                "relay_event",
                None,
                &[("action", "reject"), ("seq", seq), ("proto", "http")],
            );
            true
        }
    }
}

fn parse_http_target(target: &str) -> Option<HttpRelayTarget> {
    adversarial::route::parse_http_target(target)
}

fn parse_http_route_token(req: &HttpRequestParsed) -> Result<String, &'static str> {
    adversarial::route::parse_http_route_token_from_request(req)
}

fn read_http_request(stream: &mut TcpStream) -> Result<HttpRequestParsed, ()> {
    let mut buf = Vec::with_capacity(2048);
    let mut temp = [0u8; 1024];
    let (header_end, content_len) = loop {
        if buf.len() > 64 * 1024 {
            return Err(());
        }
        let n = stream.read(&mut temp).map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        buf.extend_from_slice(&temp[..n]);
        if let Some(pos) = find_http_header_end(buf.as_slice()) {
            let header_bytes = &buf[..pos];
            let header_text = std::str::from_utf8(header_bytes).map_err(|_| ())?;
            let mut lines = header_text.split("\r\n");
            let _request_line = lines.next().ok_or(())?;
            let mut content_len = 0usize;
            for line in lines {
                if line.is_empty() {
                    continue;
                }
                let (k, v) = line.split_once(':').ok_or(())?;
                if k.trim().eq_ignore_ascii_case("content-length") {
                    content_len = v.trim().parse::<usize>().map_err(|_| ())?;
                }
            }
            break (pos, content_len);
        }
    };
    let body_start = header_end + 4;
    while buf.len() < body_start.saturating_add(content_len) {
        let n = stream.read(&mut temp).map_err(|_| ())?;
        if n == 0 {
            return Err(());
        }
        buf.extend_from_slice(&temp[..n]);
    }
    adversarial::route::parse_http_request_bytes(buf.as_slice()).map_err(|_| ())
}

fn find_http_header_end(buf: &[u8]) -> Option<usize> {
    adversarial::route::find_http_header_end(buf)
}

fn write_http_response(stream: &mut TcpStream, status: u16, content_type: &str, body: &[u8]) {
    let reason = match status {
        200 => "OK",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        413 => "Payload Too Large",
        429 => "Too Many Requests",
        503 => "Service Unavailable",
        _ => "Error",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        status,
        reason,
        content_type,
        body.len()
    );
    let _ = stream.write_all(header.as_bytes());
    if !body.is_empty() {
        let _ = stream.write_all(body);
    }
}

pub(super) fn relay_send(
    to: &str,
    file: &Path,
    relay: &str,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
) {
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        print_error_marker(code);
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        print_error_marker(code);
    }
    if let Err(reason) = protocol_active_or_reason_for_send_peer(to) {
        protocol_inactive_exit(reason.as_str());
    }
    let payload = match fs::read(file) {
        Ok(v) => v,
        Err(_) => print_error_marker("relay_payload_read_failed"),
    };
    let outcome = relay_send_with_payload(RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector: fault_injector_from_env(),
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
        routing_override: None,
        tui_thread: None,
    });
    if let Some(code) = outcome.error_code {
        print_error_marker(code);
    }
}

pub(super) fn fault_injector_from_env() -> Option<FaultInjector> {
    let scenario = env::var("QSC_SCENARIO").ok()?;
    if scenario == "happy-path" || scenario == "default" {
        return None;
    }
    let seed_str = match env::var("QSC_SEED") {
        Ok(v) => v,
        Err(_) => print_error_marker("fault_injection_seed_required"),
    };
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .unwrap_or_else(|_| print_error_marker("fault_injection_seed_invalid"));
    Some(FaultInjector { seed, scenario })
}

pub(super) fn fault_injector_from_tui(cfg: &TuiRelayConfig) -> Option<FaultInjector> {
    if cfg.scenario == "happy-path" || cfg.scenario == "default" {
        return None;
    }
    Some(FaultInjector {
        seed: cfg.seed,
        scenario: cfg.scenario.clone(),
    })
}

fn relay_auth_token() -> Option<String> {
    if let Some(token) = relay_auth_token_from_env() {
        return Some(token);
    }
    if let Some(token) = relay_auth_token_from_account_secret() {
        return Some(token);
    }
    relay_auth_token_from_token_file()
}

fn relay_auth_token_from_env() -> Option<String> {
    relay_trimmed_nonempty(env::var("QSC_RELAY_TOKEN").ok())
        .or_else(|| relay_trimmed_nonempty(env::var("RELAY_TOKEN").ok()))
}

fn relay_auth_token_from_account_secret() -> Option<String> {
    let value = match vault::secret_get(TUI_RELAY_TOKEN_SECRET_KEY) {
        Ok(Some(v)) => Some(v),
        _ => None,
    };
    relay_trimmed_nonempty(value)
}

fn relay_auth_token_from_token_file() -> Option<String> {
    let token_file = match vault::secret_get(TUI_RELAY_TOKEN_FILE_SECRET_KEY) {
        Ok(Some(v)) => relay_trimmed_nonempty(Some(v)),
        _ => None,
    }?;
    read_relay_token_file(token_file.as_str()).ok()
}

pub(super) fn relay_inbox_push(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
) -> Result<(), &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/push", base);
    let client = HttpClient::new();
    let mut req = client
        .post(url)
        .header("X-QSL-Route-Token", route_token.as_str())
        .body(payload.to_vec());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_push_failed"),
    };
    match resp.status() {
        HttpStatus::OK => Ok(()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_push_failed"),
    }
}

pub(super) fn relay_inbox_pull(
    relay_base: &str,
    route_token: &str,
    max: usize,
) -> Result<Vec<InboxPullItem>, &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/pull?max={}", base, max);
    let client = HttpClient::new();
    let mut req = client
        .get(url)
        .header("X-QSL-Route-Token", route_token.as_str());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(_) => return Err("relay_inbox_pull_failed"),
    };
    match resp.status() {
        HttpStatus::OK => {
            let body: InboxPullResp = match resp.json() {
                Ok(v) => v,
                Err(_) => return Err("relay_inbox_parse_failed"),
            };
            Ok(body.items)
        }
        HttpStatus::NO_CONTENT => Ok(Vec::new()),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        HttpStatus::BAD_REQUEST => Err("relay_inbox_bad_request"),
        HttpStatus::PAYLOAD_TOO_LARGE => Err("relay_inbox_too_large"),
        HttpStatus::TOO_MANY_REQUESTS => Err("relay_inbox_queue_full"),
        _ => Err("relay_inbox_pull_failed"),
    }
}

fn fault_action_for(fi: &FaultInjector, idx: u64) -> Option<FaultAction> {
    if fi.scenario != "drop-reorder" {
        return None;
    }
    let k = fi.seed.wrapping_add(idx);
    match k % 4 {
        0 => Some(FaultAction::Reorder),
        1 => Some(FaultAction::Drop),
        _ => None,
    }
}

fn next_fault_index() -> u64 {
    FAULT_IDX.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

pub(super) fn relay_send_with_payload(args: RelaySendPayloadArgs<'_>) -> RelaySendOutcome {
    let RelaySendPayloadArgs {
        to,
        payload,
        relay,
        injector,
        pad_cfg,
        bucket_max,
        meta_seed,
        receipt,
        routing_override,
        tui_thread,
    } = args;
    if let Err(code) = normalize_relay_endpoint(relay) {
        return RelaySendOutcome {
            action: "endpoint_reject".to_string(),
            delivered: false,
            error_code: Some(code),
        };
    }
    let routing = match routing_override {
        Some(v) => v,
        None => match resolve_send_routing_target(to) {
            Ok(v) => v,
            Err(code) => {
                return RelaySendOutcome {
                    action: "route_token_reject".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        },
    };
    emit_cli_routing_marker(
        routing.peer_alias.as_str(),
        routing.device_id.as_str(),
        routing.implicit_primary,
    );
    emit_cli_confirm_policy();
    if let Some(thread) = tui_thread {
        emit_tui_routing_marker(thread, routing.device_id.as_str(), routing.implicit_primary);
        emit_tui_confirm_policy();
    }
    let push_route_token = routing.route_token.clone();
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if outbox_path.exists() {
        let outbox = match outbox_record_load(&outbox_path) {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return RelaySendOutcome {
                    action: "outbox_load_failed".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        };
        if outbox.to.is_empty() || outbox.ciphertext.is_empty() {
            emit_marker("error", Some("outbox_recovery_required"), &[]);
            return RelaySendOutcome {
                action: "outbox_recovery_required".to_string(),
                delivered: false,
                error_code: Some("outbox_recovery_required"),
            };
        }
        let next_state = match outbox_next_state_load() {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return RelaySendOutcome {
                    action: "outbox_state_missing".to_string(),
                    delivered: false,
                    error_code: Some(code),
                };
            }
        };
        let replay_route_token = if outbox.to == routing.peer_alias {
            routing.route_token.clone()
        } else {
            match relay_peer_route_token(outbox.to.as_str()) {
                Ok(v) => v,
                Err(code) => {
                    return RelaySendOutcome {
                        action: "route_token_reject".to_string(),
                        delivered: false,
                        error_code: Some(code),
                    };
                }
            }
        };
        print_marker("send_retry", &[("mode", "outbox_replay")]);
        return match relay_inbox_push(relay, replay_route_token.as_str(), &outbox.ciphertext) {
            Ok(()) => finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "replay_deliver".to_string(),
                Some((
                    outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
                    next_state,
                )),
                Some(TimelineSendIngest {
                    peer: outbox.to.as_str(),
                    byte_len: outbox.payload_len,
                    kind: outbox.kind.as_str(),
                    message_id: outbox.message_id.as_deref(),
                    target_device_id: outbox.channel.as_deref().and_then(channel_device_id),
                }),
            ),
            Err(code) => {
                print_marker("send_attempt", &[("ok", "false")]);
                RelaySendOutcome {
                    action: "push_fail".to_string(),
                    delivered: false,
                    error_code: Some(code),
                }
            }
        };
    }

    let (payload, receipt_msg_id) = encode_receipt_data_payload(payload, receipt);
    let pack = match qsp_pack(routing.channel.as_str(), &payload, pad_cfg, meta_seed) {
        Ok(v) => {
            record_qsp_status(&dir, source, true, "pack_ok", true, false);
            emit_marker("qsp_pack", None, &[("ok", "true"), ("version", "5.0")]);
            if let Some(label) = v.pad_label {
                let len_s = v.padded_len.to_string();
                emit_marker(
                    "meta_pad",
                    None,
                    &[("bucket", label), ("padded_len", len_s.as_str())],
                );
            }
            let msg_idx_s = v.msg_idx.to_string();
            let ck_idx_s = v.ck_idx.to_string();
            emit_marker(
                "ratchet_send_advance",
                None,
                &[
                    ("msg_idx", msg_idx_s.as_str()),
                    ("ck_idx", ck_idx_s.as_str()),
                ],
            );
            v
        }
        Err(err) => {
            record_qsp_status(&dir, source, false, err.code, false, false);
            if let Some(reason) = err.reason {
                emit_marker(
                    "qsp_pack",
                    Some(err.code),
                    &[("ok", "false"), ("reason", reason)],
                );
            } else {
                emit_marker("qsp_pack", Some(err.code), &[("ok", "false")]);
            }
            return RelaySendOutcome {
                action: err.code.to_string(),
                delivered: false,
                error_code: Some(err.code),
            };
        }
    };
    let ciphertext = pack.envelope.clone();
    if receipt_msg_id.is_some() {
        emit_marker(
            "receipt_request",
            None,
            &[("kind", "delivered"), ("msg_id", "<redacted>")],
        );
    }
    if let Some(max_bucket) = bucket_max {
        if max_bucket == 0 || max_bucket > META_BUCKET_MAX_CEILING {
            return RelaySendOutcome {
                action: "meta_bucket_invalid".to_string(),
                delivered: false,
                error_code: Some("meta_bucket_invalid"),
            };
        }
        let bucket = meta_bucket_for_len(ciphertext.len(), max_bucket);
        let bucket_s = bucket.to_string();
        let orig_s = ciphertext.len().to_string();
        let capped_s = ciphertext.len().min(max_bucket).to_string();
        emit_marker(
            "meta_bucket",
            None,
            &[
                ("bucket", bucket_s.as_str()),
                ("orig", orig_s.as_str()),
                ("capped", capped_s.as_str()),
                ("metric", "envelope_len"),
            ],
        );
    }
    let outbox = OutboxRecord {
        version: 1,
        payload_len: payload.len(),
        to: to.to_string(),
        channel: Some(routing.channel.clone()),
        ciphertext: ciphertext.clone(),
        kind: "file".to_string(),
        message_id: receipt_msg_id.clone(),
    };
    let outbox_bytes = match serde_json::to_vec(&outbox) {
        Ok(v) => v,
        Err(_) => {
            emit_marker("error", Some("outbox_serialize_failed"), &[]);
            return RelaySendOutcome {
                action: "outbox_serialize_failed".to_string(),
                delivered: false,
                error_code: Some("outbox_serialize_failed"),
            };
        }
    };
    if write_atomic(&outbox_path, &outbox_bytes, source).is_err() {
        emit_marker("error", Some("outbox_write_failed"), &[]);
        return RelaySendOutcome {
            action: "outbox_write_failed".to_string(),
            delivered: false,
            error_code: Some("outbox_write_failed"),
        };
    }
    if let Err(code) = outbox_next_state_store(&pack.next_state) {
        let _ = fs::remove_file(&outbox_path);
        emit_marker("error", Some(code), &[]);
        return RelaySendOutcome {
            action: "outbox_state_store_failed".to_string(),
            delivered: false,
            error_code: Some(code),
        };
    }

    if let Some(fi) = injector.as_ref() {
        let idx = next_fault_index();
        let idx_s = idx.to_string();
        let seed_s = fi.seed.to_string();
        if let Some(action) = fault_action_for(fi, idx) {
            match action {
                FaultAction::Drop => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "drop"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                    print_marker("send_attempt", &[("ok", "false")]);
                    return RelaySendOutcome {
                        action: "drop".to_string(),
                        delivered: false,
                        error_code: Some("relay_drop_injected"),
                    };
                }
                FaultAction::Reorder => {
                    emit_marker(
                        "relay_event",
                        None,
                        &[
                            ("action", "reorder"),
                            ("idx", idx_s.as_str()),
                            ("seed", seed_s.as_str()),
                            ("scenario", fi.scenario.as_str()),
                        ],
                    );
                }
            }
        }
    }

    let len_s = payload.len().to_string();
    print_marker("send_prepare", &[("payload_len", len_s.as_str())]);

    match relay_inbox_push(relay, push_route_token.as_str(), &ciphertext) {
        Ok(()) => {
            emit_marker("relay_event", None, &[("action", "deliver")]);
            emit_cli_delivery_state_with_device(
                to,
                "accepted_by_relay",
                Some(routing.device_id.as_str()),
            );
            if let Some(thread) = tui_thread {
                emit_tui_delivery_state_with_device(
                    thread,
                    "accepted_by_relay",
                    Some(routing.device_id.as_str()),
                );
            }
            finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "deliver".to_string(),
                Some((routing.channel.as_str(), pack.next_state.clone())),
                Some(TimelineSendIngest {
                    peer: to,
                    byte_len: payload.len(),
                    kind: "file",
                    message_id: receipt_msg_id.as_deref(),
                    target_device_id: Some(routing.device_id.as_str()),
                }),
            )
        }
        Err(code) => {
            emit_marker("relay_event", None, &[("action", "push_fail")]);
            print_marker("send_attempt", &[("ok", "false")]);
            RelaySendOutcome {
                action: "push_fail".to_string(),
                delivered: false,
                error_code: Some(code),
            }
        }
    }
}

fn finalize_send_commit(
    dir: &Path,
    source: ConfigSource,
    outbox_path: &Path,
    action: String,
    session_update: Option<(&str, Suite2SessionState)>,
    timeline_ingest: Option<TimelineSendIngest<'_>>,
) -> RelaySendOutcome {
    let next_seq = match read_send_state(dir, source) {
        Ok(v) => v + 1,
        Err(()) => {
            emit_marker("error", Some("send_state_parse_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("send_state_parse_failed"),
            };
        }
    };
    if let Some((peer, st)) = session_update {
        if qsp_session_store(peer, &st).is_err() {
            emit_marker("error", Some("qsp_session_store_failed"), &[]);
            return RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("qsp_session_store_failed"),
            };
        }
    }
    if let Some(ingest) = timeline_ingest {
        if let Err(code) = timeline_append_entry_for_target(
            ingest.peer,
            "out",
            ingest.byte_len,
            ingest.kind,
            MessageState::Sent,
            ingest.message_id,
            ingest.target_device_id,
        ) {
            emit_message_state_reject("<redacted>", code);
            emit_marker("error", Some(code), &[("op", "timeline_send_ingest")]);
        }
    }
    let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
    if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
        emit_marker("error", Some("send_commit_write_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("send_commit_write_failed"),
        };
    }
    if fs::remove_file(outbox_path).is_err() {
        emit_marker("error", Some("outbox_remove_failed"), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("outbox_remove_failed"),
        };
    }
    if let Err(code) = outbox_next_state_clear() {
        emit_marker("error", Some(code), &[]);
        return RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some(code),
        };
    }
    print_marker("send_attempt", &[("ok", "true")]);
    let seq_s = next_seq.to_string();
    print_marker("send_commit", &[("send_seq", seq_s.as_str())]);
    RelaySendOutcome {
        action,
        delivered: true,
        error_code: None,
    }
}

fn read_frame<T: for<'de> Deserialize<'de>>(stream: &mut TcpStream) -> Result<T, ()> {
    let mut len_buf = [0u8; 4];
    stream.read_exact(&mut len_buf).map_err(|_| ())?;
    let len = u32::from_be_bytes(len_buf) as usize;
    if len == 0 || len > 1_048_576 {
        return Err(());
    }
    let mut buf = vec![0u8; len];
    stream.read_exact(&mut buf).map_err(|_| ())?;
    serde_json::from_slice(&buf).map_err(|_| ())
}

fn write_frame<T: Serialize>(stream: &mut TcpStream, value: &T) -> Result<(), ()> {
    let bytes = serde_json::to_vec(value).map_err(|_| ())?;
    let len = bytes.len();
    if len > u32::MAX as usize {
        return Err(());
    }
    let len_buf = (len as u32).to_be_bytes();
    stream.write_all(&len_buf).map_err(|_| ())?;
    stream.write_all(&bytes).map_err(|_| ())?;
    Ok(())
}

fn read_send_state(dir: &Path, source: ConfigSource) -> Result<u64, ()> {
    let path = dir.join(SEND_STATE_NAME);
    if let Err(e) = enforce_safe_parents(&path, source) {
        print_error(e);
    }
    if !path.exists() {
        return Ok(0);
    }
    let mut f = File::open(&path).map_err(|_| ())?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).map_err(|_| ())?;
    for line in buf.lines() {
        if let Some(rest) = line.trim().strip_prefix("send_seq=") {
            let v = rest.trim().parse::<u64>().map_err(|_| ())?;
            return Ok(v);
        }
    }
    Err(())
}
