use crate::protocol_state::SendOrigination;
use super::*;

pub fn send_execute(args: SendExecuteArgs) -> CliResult {
    require_unlocked("send")?;
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
        None => return Err(CliError::code("send_transport_required")),
    };

    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => return Err(CliError::code("send_relay_required")),
            };
            let to = match to {
                Some(v) => v,
                None => return Err(CliError::code("send_to_required")),
            };
            let file = match file {
                Some(v) => v,
                None => return Err(CliError::code("send_file_required")),
            };
            let pad_cfg = match meta_pad_config_from_args(pad_to, pad_bucket, meta_seed) {
                Ok(v) => v,
                Err(code) => return Err(CliError::code(code)),
            };
            if let Err(code) = enforce_cli_send_contact_trust(to.as_str()) {
                return Err(CliError::code(code));
            }
            if let Err(code) = enforce_peer_not_blocked(to.as_str()) {
                return Err(CliError::code(code));
            }
            if let Err(reason) = protocol_active_or_reason_for_send_peer(to.as_str()) {
                return Err(protocol_inactive_error(reason.as_str()));
            }
            if let Some(seed) = meta_seed {
                let seed_s = seed.to_string();
                emit_marker(
                    "meta_mode",
                    None,
                    &[("deterministic", "true"), ("seed", seed_s.as_str())],
                );
            }
            // ⚠ NA-0688 C3: report the RESOLVED request, not the flag's absence. Before the
            // flip those were the same thing; now an absent flag means "follow the policy", so
            // keying the marker on `receipt.is_none()` would announce `receipt_disabled` on
            // precisely the sends that DO request one.
            let receipt = crate::resolve_sender_receipt_request(receipt);
            if receipt.is_none() {
                emit_marker("receipt_disabled", None, &[]);
            }
            relay_send(&to, &file, &relay, pad_cfg, bucket_max, meta_seed, receipt)?;
            // ⚠ NA-0688: the send just established our chain if it was unseeded, so anything we
            // owed this peer can go out now. See `flush_owed_receipts` for why this lives here.
            crate::flush_owed_receipts(&to, &relay);
            Ok(())
        }
    }
}

pub fn send_abort() -> CliResult {
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        return Err(cli_err(e));
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if let Err(e) = enforce_safe_parents(&outbox_path, source) {
        return Err(cli_err(e));
    }

    if outbox_path.exists() {
        let outbox = outbox_record_load(&outbox_path).map_err(|e| CliError::code(e))?;
        if outbox.to.is_empty() {
            return Err(CliError::code("outbox_recovery_required"));
        }
        let next_state = outbox_next_state_load().map_err(|e| CliError::code(e))?;
        if qsp_session_store(
            outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
            &next_state,
        )
        .is_err()
        {
            return Err(CliError::code("qsp_session_store_failed"));
        }
        let next_seq = match read_send_state(&dir, source)? {
            Ok(v) => v + 1,
            Err(()) => return Err(CliError::code("send_state_parse_failed")),
        };
        let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
        if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
            return Err(CliError::code("send_commit_write_failed"));
        }
        if fs::remove_file(&outbox_path).is_err() {
            return Err(CliError::code("outbox_abort_failed"));
        }
        if let Err(code) = outbox_next_state_clear() {
            return Err(CliError::code(code));
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
    Ok(())
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

pub fn receive_execute(args: ReceiveArgs) -> CliResult {
    require_unlocked("receive")?;
    let ReceiveArgs {
        transport,
        relay,
        legacy_receive_mode,
        ack_mode,
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
        None => return Err(CliError::code("recv_transport_required")),
    };
    match transport {
        SendTransport::Relay => {
            let relay = match relay {
                Some(v) => v,
                None => return Err(CliError::code("recv_relay_required")),
            };
            let attachment_service = attachment_service
                .map(|v| normalize_relay_endpoint(v.as_str()).map_err(|code| CliError::code(code)))
                .transpose()?;
            let legacy_receive_mode =
                resolve_legacy_receive_mode(legacy_receive_mode, attachment_service.as_deref())
                    .map_err(|code| CliError::code(code))?;
            // NA-0688 C4 (D622) SITE 1 of 2. ⚠ THIS LINE'S DEFAULT WAS INVERTED, not tidied.
            // It read `ack_mode.unwrap_or(AckMode::Legacy)` under NA-0644 (D580), where legacy
            // delete-on-pull was the default and lease the explicit opt-in. Both default-carrying
            // sites flipped together; a half-flip was refused, because the two of them are reached
            // by different commands and leaving one behind means the same relay behaves one way
            // under `receive` and the other under `invite`/`handshake`.
            let ack_mode = crate::resolve_ack_mode(ack_mode);
            if let Err(code) = normalize_relay_endpoint(relay.as_str()) {
                return Err(CliError::code(code));
            }
            let from = match from {
                Some(v) => v,
                None => return Err(CliError::code("recv_from_required")),
            };
            let mailbox = match mailbox {
                Some(raw) => normalize_route_token(raw.as_str())
                    .map_err(|code| CliError::code(code))?,
                None => {
                    relay_self_inbox_route_token().map_err(|code| CliError::code(code))?
                }
            };
            let max = match max {
                Some(v) if v > 0 => v,
                _ => return Err(CliError::code("recv_max_required")),
            };
            let max_file_size = match max_file_size {
                Some(v) if v > 0 && v <= ATTACHMENT_DEFAULT_MAX_FILE_SIZE => v,
                Some(_) => return Err(CliError::code("recv_file_size_bound_invalid")),
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
                Some(_) => return Err(CliError::code("recv_file_chunks_bound_invalid")),
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
                None => return Err(CliError::code("recv_out_required")),
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
                Err(code) => return Err(CliError::code(code)),
            };
            let source = ConfigSource::EnvOverride;
            if let Err(e) = ensure_dir_secure(&out, source) {
                return Err(cli_err(e));
            }
            let (cfg_dir, cfg_source) = match config_dir() {
                Ok(v) => v,
                Err(e) => return Err(cli_err(e)),
            };
            if !check_symlink_safe(&cfg_dir) {
                return Err(cli_err(ErrorCode::UnsafePathSymlink));
            }
            if !check_parent_safe(&cfg_dir, cfg_source) {
                return Err(cli_err(ErrorCode::UnsafeParentPerms));
            }
            if let Err(reason) = protocol_active_or_reason_for_peer(from.as_str()) {
                return Err(protocol_inactive_error(reason.as_str()));
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
            // Lease-only marker so the legacy stdout stays byte-identical.
            if ack_mode == AckMode::Lease {
                emit_marker("recv_ack_mode", None, &[("mode", "lease")]);
            }
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
                        ack_mode,
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
                    let stats = receive_pull_and_write(&pull, cfg.batch_max_count)?;
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
                    ack_mode,
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
                total = receive_pull_and_write(&pull, max)?.count;
            }
            if total == 0 {
                emit_marker("recv_none", None, &[]);
                return Ok(());
            }
            let count_s = total.to_string();
            emit_marker("recv_commit", None, &[("count", count_s.as_str())]);
            Ok(())
        }
    }
}

/// NA-0624: bounded re-pull rounds when a pull batch contained only SCKA control envelopes
/// (advertisements), so `receive --max N` still yields up to N application messages.
const RECV_CONTROL_ROUNDS_MAX: usize = 4;

fn receive_pull_and_write(ctx: &ReceivePullCtx<'_>, max: usize) -> CliResult<ReceivePullStats> {
    let mut stats = ReceivePullStats { count: 0, bytes: 0 };
    let mut pending_receipts: Vec<PendingReceipt> = Vec::new();
    // NA-0644 (D580): lease-mode state. Legacy mode never constructs the seen store and
    // never accumulates acks, so its behavior stays byte-identical.
    let mut pending_acks: Vec<String> = Vec::new();
    let mut seen_ids: Option<dedup::RelaySeenIds> = if ctx.ack_mode == AckMode::Lease {
        let loaded = dedup::RelaySeenIds::load(ctx.cfg_dir, ctx.mailbox, ctx.cfg_source);
        if loaded.reset {
            emit_marker("dedup_store_reset", Some("dedup_store_parse_failed"), &[]);
        }
        Some(loaded.store)
    } else {
        None
    };
    let mut rounds = 0usize;
    'pull: loop {
        let want = max.saturating_sub(stats.count).max(1);
        let items = match relay_inbox_pull_mode(ctx.relay, ctx.mailbox, want, ctx.ack_mode) {
            Ok(v) => v,
            Err(code) => return Err(CliError::code(code)),
        };
        if items.is_empty() {
            break 'pull;
        }
        let mut controls = 0usize;
        for item in items {
            // NA-0644 (D580): dedup BEFORE unpack. Lease delivery is at-least-once, so a
            // redelivered id whose item is already durably persisted must be acked and
            // skipped — reprocessing would hit the ratchet replay-reject.
            if let Some(seen) = seen_ids.as_ref() {
                if seen.contains(item.id.as_str()) {
                    emit_marker("recv_dup_skipped", None, &[("id", item.id.as_str())]);
                    pending_acks.push(item.id.clone());
                    continue;
                }
            }
            let envelope_len = item.data.len();
            match qsp_unpack_for_peer(ctx.from, &item.data) {
                Ok((outcome, channel)) => {
                    let commit_unpack_state = || {
                        record_qsp_status(
                            ctx.cfg_dir,
                            ctx.cfg_source,
                            true,
                            "unpack_ok",
                            false,
                            true,
                        );
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
                        if qsp_session_store_with_trigger(
                            channel.as_str(),
                            &outcome.next_state,
                            &outcome.trigger,
                        )
                        .is_err()
                        {
                            emit_marker("error", Some("qsp_session_store_failed"), &[]);
                            return Err(CliError::code("qsp_session_store_failed"));
                        }
                        Ok(())
                    };
                    // NA-0624: an SCKA control message (peer advertisement) carries no application
                    // payload — commit the trigger/SCKA state and move on.
                    if outcome.is_control {
                        commit_unpack_state()?;
                        record_seen_and_queue_ack(&mut seen_ids, &mut pending_acks, &item.id)?;
                        controls = controls.saturating_add(1);
                        continue;
                    }
                    let mut payload = outcome.plaintext.clone();
                    let mut request_receipt = false;
                    let mut request_msg_id = String::new();
                    // ⚠ NA-0688 C3 — TRANSPARENT FRAMING: UNWRAP BEFORE DISPATCH.
                    //
                    // The data control envelope is FRAMING, not a payload type. It used to be
                    // classified LAST, after every typed-payload sniff had already run against
                    // the still-wrapped bytes and missed -- so once receipts became the default,
                    // a `file_manifest` sent through `qsc send` was unwrapped here and then fell
                    // straight through to the generic user-message path. MEASURED, not argued:
                    // the manifest was written to `recv_1.bin`, counted in `recv_commit`, and
                    // entered the timeline as RECEIVED. It was not dropped -- it was DELIVERED
                    // TO THE USER AS MESSAGE CONTENT, which is the very failure the `ns` marker
                    // was introduced to prevent for unknown control types. The envelope and the
                    // typed dispatch had simply never been composed.
                    //
                    // So the unwrap moves to the FRONT and everything below dispatches on
                    // `payload`. For traffic that was never wrapped, `payload` IS
                    // `outcome.plaintext` and every branch behaves byte-identically to before.
                    //
                    // ⚠ ONE HOP, BY CONSTRUCTION. The unwrap happens exactly once, here; nothing
                    // below unwraps again. Our own control sends go out with `receipt: None` and
                    // are therefore never wrapped, so a wrapped body can only be one deep.
                    if let Some(ctrl) = parse_receipt_payload(&outcome.plaintext) {
                        if crate::adversarial::payload::classify_control(&ctrl)
                            == crate::adversarial::payload::ControlClass::DataEnvelope
                        {
                            if let Some(body) = ctrl.body.clone() {
                                payload = body;
                                request_receipt = true;
                                request_msg_id = ctrl.msg_id.clone();
                            }
                        }
                    }
                    if let Some(desc) = parse_attachment_descriptor_payload(&payload) {
                        let attachment_id = desc.attachment_id.clone();
                        match attachment_handle_descriptor(ctx, desc) {
                            Ok(Some((confirm_attachment_id, confirm_handle))) => {
                                commit_unpack_state()?;
                                queue_or_send_receipt(
                                    ctx,
                                    &mut pending_receipts,
                                    PendingReceipt::AttachmentComplete {
                                        attachment_id: confirm_attachment_id,
                                        confirm_handle,
                                    },
                                )?;
                            }
                            Ok(None) => {
                                commit_unpack_state()?;
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
                                return Err(CliError::code(reason));
                            }
                        }
                        queue_envelope_receipt(
                            ctx,
                            &mut pending_receipts,
                            request_receipt,
                            request_msg_id.as_str(),
                        )?;
                        record_seen_and_queue_ack(&mut seen_ids, &mut pending_acks, &item.id)?;
                        continue;
                    }
                    if let Some(file_payload) = parse_file_transfer_payload(&payload) {
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
                            return Err(CliError::code("legacy_receive_retired_post_w0"));
                        }
                        let file_res = match file_payload {
                            FileTransferPayload::Chunk(v) => {
                                file_transfer_handle_chunk(ctx, v).map(|_| None)
                            }
                            FileTransferPayload::Manifest(v) => {
                                file_transfer_handle_manifest(ctx, v)
                            }
                        };
                        match file_res {
                            Ok(Some((confirm_file_id, confirm_id))) => {
                                commit_unpack_state()?;
                                queue_or_send_receipt(
                                    ctx,
                                    &mut pending_receipts,
                                    PendingReceipt::FileComplete {
                                        file_id: confirm_file_id,
                                        confirm_id,
                                    },
                                )?;
                            }
                            Ok(None) => {
                                commit_unpack_state()?;
                            }
                            Err(reason) => {
                                if reason == "manifest_mismatch" {
                                    let _ = file_transfer_fail_clean(
                                        ctx.from,
                                        file_id.as_str(),
                                        reason,
                                    );
                                }
                                emit_marker(
                                    "file_xfer_reject",
                                    Some(reason),
                                    &[("id", file_id.as_str()), ("reason", reason)],
                                );
                                return Err(CliError::code(reason));
                            }
                        }
                        queue_envelope_receipt(
                            ctx,
                            &mut pending_receipts,
                            request_receipt,
                            request_msg_id.as_str(),
                        )?;
                        record_seen_and_queue_ack(&mut seen_ids, &mut pending_acks, &item.id)?;
                        continue;
                    }
                    if let Some(confirm) = parse_attachment_confirm_payload(&payload) {
                        commit_unpack_state()?;
                        // ⚠ The capture decision is `confirm_capture_reason`'s alone (Ruling 11.1);
                        // these arms EMIT, they do not decide. The arms below and D3's and D4's
                        // used to each carry their own copy of that decision, and one copy was
                        // wrong.
                        let outcome = apply_attachment_peer_confirmation(
                            ctx.from,
                            confirm.attachment_id.as_str(),
                            confirm.confirm_handle.as_str(),
                            channel.as_str(),
                        );
                        let discard_reason = confirm_capture_reason(&outcome);
                        match &outcome {
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
                            Err(reason) => {
                                emit_marker(
                                    "attachment_confirm_reject",
                                    Some(reason),
                                    &[("reason", reason), ("ok", "false")],
                                );
                            }
                        }
                        queue_envelope_receipt(
                            ctx,
                            &mut pending_receipts,
                            request_receipt,
                            request_msg_id.as_str(),
                        )?;
                        // NA-0689 D2. ⚠ THIS ACK IS SHARED WITH THE SUCCESS ARM, so the capture
                        // is conditional: only a rejected or ignored confirm is quarantined. A
                        // blanket capture here would store every SUCCESSFULLY applied confirm
                        // too -- turning the store into a copy of ordinary traffic.
                        match discard_reason {
                            Some(reason) => quarantine_then_ack(
                                ctx,
                                &mut seen_ids,
                                &mut pending_acks,
                                item.id.as_str(),
                                crate::quarantine::Subclass::Unrecoverable,
                                crate::quarantine::ContentKind::InnerPayload,
                                reason,
                                "transport::receive_pull_and_write/attachment_confirm",
                                &payload,
                            )?,
                            None => record_seen_and_queue_ack(
                                &mut seen_ids,
                                &mut pending_acks,
                                &item.id,
                            )?,
                        }
                        continue;
                    }
                    if let Some(file_confirm) = parse_file_confirm_payload(&payload) {
                        commit_unpack_state()?;
                        // ⚠ See D2: the decision is `confirm_capture_reason`'s; these arms emit.
                        let outcome = apply_file_peer_confirmation(
                            ctx.from,
                            file_confirm.file_id.as_str(),
                            file_confirm.confirm_id.as_str(),
                            channel.as_str(),
                        );
                        let discard_reason = confirm_capture_reason(&outcome);
                        match &outcome {
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
                            Err(reason) => {
                                emit_marker(
                                    "file_confirm_reject",
                                    Some(reason),
                                    &[("reason", reason), ("ok", "false")],
                                );
                            }
                        }
                        queue_envelope_receipt(
                            ctx,
                            &mut pending_receipts,
                            request_receipt,
                            request_msg_id.as_str(),
                        )?;
                        // NA-0689 D3 — same shared-ack shape as D2.
                        match discard_reason {
                            Some(reason) => quarantine_then_ack(
                                ctx,
                                &mut seen_ids,
                                &mut pending_acks,
                                item.id.as_str(),
                                crate::quarantine::Subclass::Unrecoverable,
                                crate::quarantine::ContentKind::InnerPayload,
                                reason,
                                "transport::receive_pull_and_write/file_confirm",
                                &payload,
                            )?,
                            None => record_seen_and_queue_ack(
                                &mut seen_ids,
                                &mut pending_acks,
                                &item.id,
                            )?,
                        }
                        continue;
                    }
                    if let Some(ctrl) = parse_receipt_payload(&payload) {
                        // NA-0682 (D617 C6): classify ONCE, here, so the "unknown control"
                        // arm exists at all. Before this, an unrecognised control payload
                        // fell through and was written to `recv_N.bin` and the timeline as
                        // a user message -- which made DESIGN F2's "a new ack type is a new
                        // type, no format break" false as built.
                        let class = crate::adversarial::payload::classify_control(&ctrl);
                        // NA-0689 D-1328 Ruling 12: the branch condition and the capture reason
                        // now come from ONE place, so they cannot drift apart.
                        if let Some(class_reason) = control_class_capture_reason(class) {
                            // Ours (it carries the namespace marker) but of a type this
                            // build does not know. IGNORE IT -- this is the seam a future
                            // read-receipt rides on, and rendering it would be the bug.
                            //
                            // ⚠ Only payloads carrying the marker reach here, so a user
                            // message that merely looks like this JSON is NOT swallowed --
                            // it classifies as NotControl and is delivered as before.
                            commit_unpack_state()?;
                            emit_marker(
                                "control_ignored",
                                None,
                                &[("reason", class_reason), ("v", "redacted")],
                            );
                            queue_envelope_receipt(
                                ctx,
                                &mut pending_receipts,
                                request_receipt,
                                request_msg_id.as_str(),
                            )?;
                            // NA-0689 D5. ⚠ SEPARATELY WITNESSED (D-1328 Ruling 2) -- this is
                            // judged NOT-FOR-THIS-BUILD, not unrecoverable, and a forward-compat
                            // capture must never read as a decrypt failure. ⚠ And it is the
                            // INNER PAYLOAD (Ruling 7): the key is already consumed above, so
                            // storing the ciphertext would make this capture VACUOUS -- no future
                            // build could ever read the thing it was kept for.
                            //
                            // ⚠ Redelivery cannot save this item: every current build acks it
                            // away on sight, so the store is the only thing that preserves it.
                            // No re-ingestion tooling is promised or built.
                            quarantine_then_ack(
                                ctx,
                                &mut seen_ids,
                                &mut pending_acks,
                                item.id.as_str(),
                                crate::quarantine::Subclass::Unsupported,
                                crate::quarantine::ContentKind::InnerPayload,
                                class_reason,
                                "transport::receive_pull_and_write/unknown_control",
                                &payload,
                            )?;
                            continue;
                        }
                        if class == crate::adversarial::payload::ControlClass::DeliveredAck {
                            commit_unpack_state()?;
                            // ⚠ See D2: the decision is `confirm_capture_reason`'s; these arms
                            // emit. Success here means `Confirmed` and nothing else -- BOTH
                            // non-success arms capture, which is the asymmetry Ruling 9 closed
                            // and Ruling 11.1 made structural.
                            let outcome = apply_message_peer_confirmation(
                                ctx.from,
                                ctrl.msg_id.as_str(),
                                channel.as_str(),
                            );
                            let discard_reason = confirm_capture_reason(&outcome);
                            match &outcome {
                                Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                                    let dev = channel_device_marker(channel.as_str());
                                    emit_cli_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                                    emit_tui_receipt_ignored_wrong_device(ctx.from, dev.as_str());
                                    // NA-0689 D-1328 RULING 9: this arm's capture was MISSING while
                                    // D2's and D3's identical arms had it. It belongs to the
                                    // destruction class by the census's own definition --
                                    // `commit_unpack_state()?` above consumed the key BEFORE this
                                    // outcome was known, nothing was applied, and the ack below
                                    // would leave a marker as the only witness. It is NOT
                                    // "already-processed": that needs a durable record proving
                                    // prior application, and there is none. The decision now lives
                                    // in `confirm_capture_reason`, so it cannot go missing at one
                                    // site again.
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
                                Err(reason) => {
                                    emit_message_state_reject(reason);
                                }
                            }
                            queue_envelope_receipt(
                                ctx,
                                &mut pending_receipts,
                                request_receipt,
                                request_msg_id.as_str(),
                            )?;
                            // NA-0689 D4 — same shared-ack shape as D2/D3.
                            match discard_reason {
                                Some(reason) => quarantine_then_ack(
                                    ctx,
                                    &mut seen_ids,
                                    &mut pending_acks,
                                    item.id.as_str(),
                                    crate::quarantine::Subclass::Unrecoverable,
                                    crate::quarantine::ContentKind::InnerPayload,
                                    reason,
                                    "transport::receive_pull_and_write/delivered_ack",
                                    &payload,
                                )?,
                                None => record_seen_and_queue_ack(
                                    &mut seen_ids,
                                    &mut pending_acks,
                                    &item.id,
                                )?,
                            }
                            continue;
                        }
                        // ⚠ NO `DataEnvelope` ARM HERE ANY MORE — the unwrap moved to the FRONT
                        // of this chain (see the transparent-framing comment above), so by the
                        // time control reaches this point `payload` is already the inner body
                        // and cannot be an envelope. Unwrapping a second time here is what
                        // would turn "one hop by construction" into an unbounded claim.
                        //
                        // The one shape that still classifies as `DataEnvelope` here is an
                        // envelope whose `body` was absent, which the front unwrap leaves
                        // untouched on purpose; it falls through to the generic path exactly as
                        // it did before this lane.
                    }
                    commit_unpack_state()?;
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
                        return Err(CliError::code("recv_write_failed"));
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
                    // NA-0682 (D617 §2f / F5): dedup by (session, msg_id) BEFORE storing.
                    //
                    // Duplicate deliveries are EXPECTED -- at-least-once delivery plus retry
                    // races -- and DESIGN §4 requires them to be invisible to the user. A
                    // duplicate is still ACKED (idempotently): the sender's ack may be what
                    // was lost, and refusing to re-ack would strand them on SENT forever.
                    if !request_msg_id.is_empty() {
                        match msgqueue::inbound_already_seen(ctx.cfg_dir, ctx.from, &request_msg_id)
                        {
                            Ok(true) => {
                                commit_unpack_state()?;
                                emit_marker(
                                    "recv_dup_msg_id_skipped",
                                    None,
                                    &[("msg_id", "<redacted>")],
                                );
                                if request_receipt {
                                    queue_or_send_receipt(
                                        ctx,
                                        &mut pending_receipts,
                                        PendingReceipt::Message {
                                            msg_id: request_msg_id.clone(),
                                        },
                                    )?;
                                }
                                record_seen_and_queue_ack(
                                    &mut seen_ids,
                                    &mut pending_acks,
                                    &item.id,
                                )?;
                                continue;
                            }
                            Ok(false) => {}
                            // Fail-closed: if we cannot tell whether it is a duplicate, do
                            // NOT guess. Leave it for redelivery rather than risk either a
                            // double-render or a silent drop.
                            Err(code) => return Err(CliError::code(code)),
                        }
                    }

                    // NA-0682 (D617 census C16): STORE DURABLY, **THEN** ACK.
                    //
                    // ⚠ Before this lane the timeline failure below was non-fatal and the
                    // ack fired regardless, so THE SENDER COULD BE TOLD "DELIVERED" WHILE
                    // THE RECIPIENT HAD NO STORED MESSAGE. That is an O3 violation by
                    // omission: "delivered" is supposed to mean the recipient's device has
                    // it, and an ack is the only evidence the sender ever gets.
                    //
                    // Fail-closed: if the row does not store, DO NOT ack. The relay still
                    // holds the message (it is not acked at the lease layer either), so it
                    // is redelivered and tried again -- visibly stuck rather than silently
                    // claimed as delivered.
                    let stored = timeline_append_entry(
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
                    );
                    if let Err(code) = stored {
                        emit_message_state_reject(code);
                        emit_marker("error", Some(code), &[("op", "timeline_receive_ingest")]);
                    }
                    // ⚠ Record the id only AFTER the row is durably stored. Recording
                    // first would let a crash in between turn a real message into permanent
                    // duplicate-suppression -- a silent loss dressed up as dedup.
                    let deduped = if stored.is_ok() && !request_msg_id.is_empty() {
                        msgqueue::record_inbound_seen(
                            ctx.cfg_dir,
                            ctx.cfg_source,
                            ctx.from,
                            &request_msg_id,
                            msgqueue::now_unix_s(),
                        )
                    } else {
                        Ok(())
                    };
                    if let Err(code) = deduped {
                        emit_marker("error", Some(code), &[("op", "msgqueue_seen_inbound")]);
                    }
                    if request_receipt && stored.is_ok() && deduped.is_ok() {
                        queue_or_send_receipt(
                            ctx,
                            &mut pending_receipts,
                            PendingReceipt::Message {
                                msg_id: request_msg_id,
                            },
                        )?;
                    } else if request_receipt {
                        // Say so, rather than letting a missing ack look like a lost one.
                        emit_marker(
                            "receipt_suppressed",
                            Some("receive_store_failed"),
                            &[("reason", "not_stored_so_not_acked")],
                        );
                    }
                    record_seen_and_queue_ack(&mut seen_ids, &mut pending_acks, &item.id)?;
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
                        // NA-0644 (D580) lease-mode backstop for the pre-existing
                        // commit-before-write seam: the ratchet consumed this envelope's
                        // key in an earlier run but its payload was never persisted (crash
                        // between commit_unpack_state and write_atomic), so the plaintext
                        // is unrecoverable no matter how often the relay redelivers it.
                        // Ack it (loudly) to end the redelivery loop instead of hard-
                        // exiting the whole batch.
                        //
                        // ⚠ NA-0688 C4 (D622): THIS BRANCH IS NOW THE DEFAULT PATH. This
                        // comment used to end "Legacy behavior is unchanged", which was true
                        // only while Legacy was the default -- it meant "the ordinary user is
                        // unaffected". C4 flipped the default to Lease, so that reassurance
                        // became false and the comment would have argued against its own code.
                        // What is actually true after C4: a replay reject no longer fails the
                        // command. It is acked, reported by the ack_replay_unrecoverable
                        // marker, and the run continues to a normal exit. The rejection and
                        // the no-state-mutation guarantee are unchanged; only the EXIT CODE
                        // moved, and only on the default path. `--ack-mode legacy` still
                        // hard-exits, and that contract is pinned explicitly.
                        if ctx.ack_mode == AckMode::Lease {
                            emit_marker(
                                "ack_replay_unrecoverable",
                                Some(code),
                                &[("id", item.id.as_str())],
                            );
                            // NA-0689 D1. ⚠ THE WIRE ENVELOPE IS ALL THERE IS HERE (D-1328
                            // Ruling 7): this is the decrypt-failure path, so the message key
                            // was consumed in an EARLIER run and the ciphertext is permanently
                            // undecryptable by everyone. It is kept for correlation and as the
                            // only surviving artefact -- never for recovery.
                            quarantine_then_ack(
                                ctx,
                                &mut seen_ids,
                                &mut pending_acks,
                                item.id.as_str(),
                                crate::quarantine::Subclass::Unrecoverable,
                                crate::quarantine::ContentKind::WireEnvelope,
                                code,
                                "transport::receive_pull_and_write/qsp_replay_reject",
                                &item.data,
                            )?;
                            continue;
                        }
                    }
                    return Err(CliError::code(code));
                }
            }
        }
        rounds = rounds.saturating_add(1);
        if controls == 0 || stats.count >= max || rounds >= RECV_CONTROL_ROUNDS_MAX {
            break 'pull;
        }
    }
    // NA-0644 (D580): flush the acks before attachment resume — a long content download
    // must not hold acks past the server's lease clock; a descriptor item is durable at
    // its pending-record commit, independent of the later content download.
    flush_pending_acks(ctx, &mut pending_acks);
    if let Some(service_url) = ctx.attachment_service {
        let resumed = attachment_resume_pending_for_peer(ctx, service_url)?;
        stats.count = stats.count.saturating_add(resumed);
    }
    flush_batched_receipts(ctx, &mut pending_receipts)?;
    Ok(stats)
}

// NA-0644 (D580): the lease-mode ordering invariant. An id becomes ack-eligible ONLY
// after (a) the item's own durable commit — the callers reach this on committed paths
// only — and (b) the seen-store entry for it is durably on disk (`record` returning Ok).
// A failed seen-write is fail-closed: the id is never acked, the lease expires, and the
// redelivery lands in the seen store or the replay-reject backstop. No-op in legacy mode.
/// NA-0689 P2: **capture, then ack.** The one place a censused discard point goes.
///
/// ⚠ **FAIL-CLOSED, AND THE CONSEQUENCE IS RATIFIED (D-1328).** If the capture fails we do
/// **NOT** ack, so the relay redelivers the item and the command reports it. That redelivery
/// loop is a **DECISION, not a regression** — a loud, witnessed availability degradation chosen
/// over silent destruction. ⚠ A future reader will feel strong pull to "fix" the loop, because
/// NA-0644's backstop exists precisely to END a poison redelivery loop and D-1327 §3a records a
/// lane that predicted a wedge from redelivery and was wrong. **This loop is the point.**
///
/// The wire behaviour is otherwise unchanged: the same ack, at the same moment, for every item
/// whose capture succeeded — which is every item, absent a filesystem or vault failure.
#[allow(clippy::too_many_arguments)]
/// NA-0689 D-1328 RULING 11.1 — **THE CAPTURE DECISION FOR THE THREE SHARED-ACK CONFIRM SITES.**
///
/// D2 (`attachment_confirm`), D3 (`file_confirm`) and D4 (`delivered_ack`) each share one ack with
/// a success arm, so each must decide per-outcome whether to capture. That decision used to be
/// written out three times — and one of the three was written **wrong**: D4's `IgnoredWrongDevice`
/// arm set no reason while its two structurally identical siblings did, so a wrong-device
/// `delivered_ack` was destroyed rather than kept (Ruling 9). ⚠ **The three cannot stay split, so
/// they no longer CAN be** — this is the only place the decision is made, and the symmetry is now
/// structural rather than three tests agreeing.
///
/// ⚠ **THE `IgnoredWrongDevice` AND `Err` CAPTURES ARE HOSTILE-PEER WITNESSES.** A stock `qsc` peer
/// cannot produce either: it would have to confirm an item it never received (the confirm must
/// arrive on a **device-qualified** session for a device that is not the item's target), or name an
/// item the receiver holds no record of. **That is not a gap — it is what these two captures are
/// FOR.** It is also why they cannot be reached from an end-to-end arm, which is why the decision
/// is pinned HERE, exhaustively, instead (D-1328 Ruling 11.5).
///
/// `None` means the confirm **APPLIED** and the item was genuinely processed — nothing to
/// quarantine. Returning `Some` for a confirm that applied would turn the store into a copy of
/// ordinary traffic.
fn confirm_capture_reason(
    outcome: &Result<(ConfirmApplyOutcome, Option<String>), &'static str>,
) -> Option<&'static str> {
    match outcome {
        Ok((ConfirmApplyOutcome::Confirmed, _)) => None,
        Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => Some("ignored_wrong_device"),
        Err(reason) => Some(reason),
    }
}

/// NA-0689 D-1328 RULING 12 — **D5's CAPTURE DECISION**, pinned where it is reachable.
///
/// D5 is the forward-compat seam: a payload that carries our namespace marker but whose type this
/// build does not know. `classify_control` decides WHAT the payload is (and is exhaustively pinned
/// by NA-0682's own tests); this decides what the SITE does about it. Splitting the two is what
/// makes the second half testable at all.
///
/// ⚠ **THE MATCH IS EXHAUSTIVE ON PURPOSE.** A new `ControlClass` variant will fail to compile here
/// rather than defaulting into "capture" or "ignore" by omission — the same protection the
/// `ConfirmApplyOutcome` table gives D2/D3/D4.
///
/// ⚠ **LIKE D2–D4's NON-SUCCESS ARMS, D5's CAPTURE IS UNREACHABLE FROM A STOCK PEER — and for a
/// reason that is the site's PURPOSE.** `UnknownControl` needs the marker plus an unknown `t`/`kind`
/// or a version above `CTRL_VERSION_MAX`, and a sender of *this* build emits neither. It is the
/// **forward-compat witness**: only a FUTURE build can trigger it. That is why the decision is
/// pinned here instead of by an end-to-end arm (D-1328 Rulings 11.5 and 12).
fn control_class_capture_reason(
    class: crate::adversarial::payload::ControlClass,
) -> Option<&'static str> {
    use crate::adversarial::payload::ControlClass;
    match class {
        ControlClass::UnknownControl => Some("unknown_control_type"),
        // Known to this build, or not ours at all: each is handled on its own path and none of
        // them is a discard. Capturing here would store ordinary traffic.
        ControlClass::DeliveredAck | ControlClass::DataEnvelope | ControlClass::NotControl => None,
    }
}

fn quarantine_then_ack(
    ctx: &ReceivePullCtx<'_>,
    seen_ids: &mut Option<dedup::RelaySeenIds>,
    pending_acks: &mut Vec<String>,
    item_id: &str,
    subclass: crate::quarantine::Subclass,
    content: crate::quarantine::ContentKind,
    reason: &str,
    site: &str,
    data: &[u8],
) -> CliResult<()> {
    match crate::quarantine::capture_at(
        ctx.cfg_dir,
        ctx.cfg_source,
        item_id,
        subclass,
        content,
        reason,
        site,
        data,
        crate::clock::now_unix_s(),
    ) {
        Ok(_) => record_seen_and_queue_ack(seen_ids, pending_acks, item_id),
        Err(code) => {
            // ⚠ LOUD. The item is NOT acked and will come back; say so rather than letting a
            // missing ack look like a lost one.
            emit_marker(
                "quarantine_capture_failed",
                Some(code),
                &[
                    ("id", item_id),
                    ("site", site),
                    ("action", "not_acked_will_redeliver"),
                ],
            );
            Ok(())
        }
    }
}

fn record_seen_and_queue_ack(
    seen: &mut Option<dedup::RelaySeenIds>,
    pending_acks: &mut Vec<String>,
    id: &str,
) -> CliResult {
    if let Some(store) = seen.as_mut() {
        if store.record(id).is_err() {
            return Err(CliError::code("dedup_store_write_failed"));
        }
        pending_acks.push(id.to_string());
    }
    Ok(())
}

// qsl-server MAX_ACK_IDS: the ack route rejects larger id lists.
const RELAY_ACK_MAX_IDS: usize = 4096;

enum AckFlushOutcome {
    Acked(usize),
    LegacyComplete,
}

fn flush_pending_acks(ctx: &ReceivePullCtx<'_>, pending_acks: &mut Vec<String>) {
    if pending_acks.is_empty() {
        return;
    }
    let sent = pending_acks.len();
    let mut acked = 0usize;
    let mut legacy_complete = false;
    for chunk in pending_acks.chunks(RELAY_ACK_MAX_IDS) {
        match relay_inbox_ack(ctx.relay, ctx.mailbox, chunk) {
            Ok(AckFlushOutcome::Acked(n)) => acked = acked.saturating_add(n),
            Ok(AckFlushOutcome::LegacyComplete) => {
                legacy_complete = true;
                break;
            }
            Err(code) => {
                // Never fail the receive on a lost ack: every queued id is already
                // durably persisted locally; the lease expires server-side and the
                // redelivery is deduped on the next receive.
                let pending_s = sent.to_string();
                emit_marker("ack_failed", Some(code), &[("pending", pending_s.as_str())]);
                pending_acks.clear();
                return;
            }
        }
    }
    let sent_s = sent.to_string();
    if legacy_complete {
        // Old-server tolerance: a pre-durability relay ignores ?ack=lease and has no ack
        // route (404). It already delivered legacy-style (delete-on-deliver), so nothing
        // is lost and nothing will redeliver — "legacy-complete", not an error, no retry.
        emit_marker("ack_legacy_complete", None, &[("count", sent_s.as_str())]);
    } else {
        let acked_s = acked.to_string();
        emit_marker(
            "relay_ack",
            None,
            &[("sent", sent_s.as_str()), ("acked", acked_s.as_str())],
        );
    }
    pending_acks.clear();
}

pub fn relay_serve(port: u16, cfg: RelayConfig, max_messages: u64) -> CliResult {
    let addr = format!("127.0.0.1:{}", port);
    let listener =
        TcpListener::bind(&addr).map_err(|_| CliError::code("relay_bind_failed"))?;
    let bound = listener
        .local_addr()
        .map_err(|_| CliError::code("relay_bind_failed"))?;
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
    Ok(())
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

pub fn relay_send(
    to: &str,
    file: &Path,
    relay: &str,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
) -> CliResult {
    if let Err(code) = enforce_cli_send_contact_trust(to) {
        return Err(CliError::code(code));
    }
    if let Err(code) = enforce_peer_not_blocked(to) {
        return Err(CliError::code(code));
    }
    if let Err(reason) = protocol_active_or_reason_for_send_peer(to) {
        return Err(protocol_inactive_error(reason.as_str()));
    }
    let payload = match fs::read(file) {
        Ok(v) => v,
        Err(_) => return Err(CliError::code("relay_payload_read_failed")),
    };

    // NA-0682 (D617 §2b/§2c, operator-ruled Option A): ENQUEUE FIRST, THEN DRAIN.
    //
    // ⚠ THIS IS O1, AND IT IS WHY IT LIVES ON THE DEFAULT PATH. The message is committed
    // durably to the message queue BEFORE anything is packed or pushed, so a crash at any
    // later point leaves a QUEUED row -- never nothing. Putting this behind an opt-in verb
    // would have made "no silent loss" true only of a path nobody uses, which is the
    // claims-honesty failure this project exists to prevent.
    //
    // It also closes census C4: the old path replayed a single GLOBAL in-flight slot and
    // RETURNED, so a second send while one was stuck was never even packed -- silently
    // dropped, with `send_semantics::outbox_recovery_via_send_abort` asserting the drop as
    // correct. Now the second message is durably queued before any of that can happen.
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let now = msgqueue::now_unix_s();
    let rec = msgqueue::enqueue_at(&dir, source, to, payload, now).map_err(CliError::code)?;
    let queued_len = rec.body.len().to_string();
    emit_marker(
        "msgqueue_enqueued",
        None,
        &[
            ("state", rec.state.as_str()),
            ("payload_len", queued_len.as_str()),
            // ⚠ never the raw msg_id: it is inner, and DESIGN §4 keeps it off the wire and
            // out of anything a log might carry.
            ("msg_id", "<redacted>"),
        ],
    );

    // DESIGN §9 Q4: a SOFT warning at 50 queued for this contact. Never a cap -- refusing
    // to enqueue would itself be the silent loss O1 forbids.
    if let Ok(all) = msgqueue::load_contact(&dir, to) {
        let queued = all
            .iter()
            .filter(|r| r.state == msgqueue::MsgState::Queued)
            .count();
        if queued >= msgqueue::QUEUE_WARN_THRESHOLD {
            let n = queued.to_string();
            emit_marker("msgqueue_backlog_warning", None, &[("queued", n.as_str())]);
        }
    }

    let mut sender =
        RelayMessageSender::new(relay).with_meta(pad_cfg, bucket_max, meta_seed, receipt);
    let outcome = msgqueue::drain_at(
        &dir,
        source,
        msgqueue::DrainTrigger::Scheduled,
        now,
        &mut sender,
    )
    .map_err(CliError::code)?;

    // ⚠ Report honestly. The message is SAFE (durably queued) but that is not the same as
    // SENT, and saying "sent" when it is queued would be exactly the false claim §2h
    // forbids. So a message that did not reach the relay still exits non-zero -- and says
    // why, and says it will be retried.
    if outcome.sent == 0 {
        let state = msgqueue::load_contact(&dir, to)
            .ok()
            .and_then(|v| v.into_iter().find(|r| r.msg_id == rec.msg_id));
        // A9: when a 413 named a limit, say it. "too large" alone is not actionable.
        let too_large_line = match sender.last_limit() {
            Some(n) => format!("too large — this relay accepts up to {} bytes", n),
            None => "too large for this relay".to_string(),
        };
        // ⚠ NA-0682 (operator-ruled, STOP 020). A LOCAL relay-config fault -- a CA file that
        // is missing, unreadable or not a certificate -- is MESSAGE-INDEPENDENT and needs no
        // network to detect. It must be NAMED even when this particular message was never
        // attempted, because per-contact FIFO can hold it behind an earlier one.
        //
        // ⚠ WHY THIS IS A SAFETY FIX, NOT TIDINESS: reporting "will send when the relay is
        // reachable" for a broken CA file is a FALSE DIAGNOSIS. It makes a TRUST failure
        // indistinguishable from a flaky network, so an operator facing an untrusted
        // certificate -- possibly an active interception -- is told to wait. That is exactly
        // the confusion `NA_0663_relay_tls_trust` exists to prevent.
        //
        // The message is already durably enqueued (O1) and NOTHING is transmitted, so
        // fail-closed is untouched: zero bytes leave while trust is broken.
        let local_relay_cfg_fault = match relay_http_client() {
            Err(RelayHttpClientError::CaFile(code)) => Some(code),
            _ => None,
        };
        // DESIGN: PAUSED is a sub-state of QUEUED that names WHY retries are not running.
        // A trust fault pauses rather than fails -- saving relay settings resumes it.
        if local_relay_cfg_fault.is_some() {
            if let Some(mut r) = state.clone() {
                if r.state == msgqueue::MsgState::Queued && r.paused_cause.is_none() {
                    r.paused_cause = Some(msgqueue::PausedCause::Cert);
                    let _ = msgqueue::save(&dir, source, &r);
                }
            }
        }
        // If THIS message was not attempted, the reason lives at the HEAD of the contact's
        // FIFO. Report that reason rather than a generic line about this message.
        let head_pause = msgqueue::load_contact(&dir, to).ok().and_then(|v| {
            v.into_iter()
                .filter(|r| r.state == msgqueue::MsgState::Queued)
                .min_by_key(|r| r.seq)
                .and_then(|r| r.paused_cause)
        });
        let (code, cause) = if let Some(c) = local_relay_cfg_fault {
            (c, msgqueue::PausedCause::Cert.human())
        } else {
            match state.as_ref() {
                Some(r) if r.state == msgqueue::MsgState::FailedPermanent => {
                    ("msgqueue_failed_permanent", "session revoked")
                }
                Some(r) if r.state == msgqueue::MsgState::Failed => {
                    ("msgqueue_failed", too_large_line.as_str())
                }
                Some(r) => match r.paused_cause {
                    Some(c) => ("msgqueue_paused", c.human()),
                    // ⚠ "will send when the relay is reachable" is RESERVED for the
                    // TRANSIENT class. A paused head is not transient, so say what is
                    // actually holding the queue (O5: visibly moving or visibly stuck).
                    None => match head_pause {
                        Some(c) => ("msgqueue_paused", c.human()),
                        None => ("msgqueue_queued", "will send when the relay is reachable"),
                    },
                },
                None => ("msgqueue_queued", "will send when the relay is reachable"),
            }
        };
        emit_marker("msgqueue_not_sent", Some(code), &[("cause", cause)]);
        // Report the SPECIFIC transport cause when there is one; fall back to the queue's
        // own state otherwise. A queued message still exits non-zero -- safe is not sent.
        return Err(CliError::code(sender.last_code().unwrap_or(code)));
    }
    Ok(())
}

pub(super) fn fault_injector_from_env() -> CliResult<Option<FaultInjector>> {
    let scenario = match env::var("QSC_SCENARIO") {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };
    if scenario == "happy-path" || scenario == "default" {
        return Ok(None);
    }
    let seed_str = match env::var("QSC_SEED") {
        Ok(v) => v,
        Err(_) => return Err(CliError::code("fault_injection_seed_required")),
    };
    let seed = seed_str
        .trim()
        .parse::<u64>()
        .map_err(|_| CliError::code("fault_injection_seed_invalid"))?;
    Ok(Some(FaultInjector { seed, scenario }))
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

// ===========================================================================
// NA-0663 (D599, D-1286): client TLS trust.
//
// Part 1 — the OS trust store is honored via the reqwest feature
// "rustls-tls-native-roots", held in UNION with the baked-in webpki roots
// ("rustls-tls"). Part 2 — an operator CA file is ADDITIVE on top of both.
// Part 3 — a certificate-verification failure is a DISTINGUISHABLE typed
// outcome instead of the opaque per-op failure value.
//
// THE HARD BOUNDARY: this crate exposes NO certificate-verification bypass of
// any kind — no option to skip verification, to trust every certificate, or to
// tolerate one that fails to verify — in any form, for any reason, including
// tests. An explicit CA file is the sanctioned escape; a blanket bypass is not.
// The bypass-needle scan in tests/NA_0663_relay_tls_trust.rs is fail-closed and
// admits no exemptions, so the needles are deliberately not spelled here.
// ===========================================================================

/// The one new caller-visible relay outcome: the peer's certificate did not
/// verify. Distinct from unreachable, DNS failure, timeout, and
/// `relay_unauthorized`.
pub const RELAY_TLS_UNTRUSTED: &str = "relay_tls_untrusted";
/// A CA file was configured but is not present.
pub const RELAY_CA_FILE_MISSING: &str = "relay_ca_file_missing";
/// A CA file was configured and exists but could not be read.
pub const RELAY_CA_FILE_UNREADABLE: &str = "relay_ca_file_unreadable";
/// A CA file was configured and read but holds no parsable PEM certificate.
pub const RELAY_CA_FILE_INVALID: &str = "relay_ca_file_invalid";

const RELAY_CA_FILE_ENV: &str = "QSC_RELAY_CA_FILE";
const RELAY_CA_FILE_ENV_FALLBACK: &str = "RELAY_CA_FILE";

/// Why a house relay client could not be built.
///
/// This is an internal Rust type, NOT a caller-visible outcome: `CaFile`
/// carries one of the enumerated values above, which the call site returns
/// verbatim, and `Build` carries nothing so the call site reports its OWN
/// pre-existing failure value. No new vocabulary reaches a caller through it.
pub(crate) enum RelayHttpClientError {
    CaFile(&'static str),
    Build,
}

/// The configured CA-file path, resolved exactly like the auth token:
/// env `QSC_RELAY_CA_FILE` -> env `RELAY_CA_FILE` -> vault secret
/// `tui.relay.ca_file`. `None` means no CA file is configured, which is the
/// ordinary case and NOT an error.
fn relay_ca_file() -> Option<String> {
    if let Some(path) = relay_ca_file_from_env() {
        return Some(path);
    }
    relay_ca_file_from_account_secret()
}

fn relay_ca_file_from_env() -> Option<String> {
    relay_trimmed_nonempty(env::var(RELAY_CA_FILE_ENV).ok())
        .or_else(|| relay_trimmed_nonempty(env::var(RELAY_CA_FILE_ENV_FALLBACK).ok()))
}

fn relay_ca_file_from_account_secret() -> Option<String> {
    let value = match vault::secret_get(TUI_RELAY_CA_FILE_SECRET_KEY) {
        Ok(Some(v)) => Some(v),
        _ => None,
    };
    relay_trimmed_nonempty(value)
}

/// Read a configured CA bundle from disk, FAIL-CLOSED.
///
/// Deliberate, recorded asymmetry vs `read_relay_token_file`: there is NO 0600
/// permission gate here. A CA certificate is PUBLIC material and a
/// world-readable CA file is correct; readability is the only requirement.
fn read_relay_ca_file(path: &str) -> Result<Vec<u8>, &'static str> {
    let p = Path::new(path);
    let md = fs::metadata(p).map_err(|_| RELAY_CA_FILE_MISSING)?;
    if !md.is_file() {
        return Err(RELAY_CA_FILE_UNREADABLE);
    }
    fs::read(p).map_err(|_| RELAY_CA_FILE_UNREADABLE)
}

/// The ONE house relay HTTP client. Replaces every `HttpClient::new()` site.
///
/// Built through `ClientBuilder` rather than `Client::new()`, which panics on a
/// TLS backend failure; this form must not panic. Root composition:
///
///   * webpki/Mozilla roots  — from the "rustls-tls" feature (UNCHANGED)
///   * OS trust store        — from the "rustls-tls-native-roots" feature (NEW)
///   * the operator CA file  — added here when configured (NEW, ADDITIVE)
///
/// The built-in roots are NEVER disabled: this function does not call
/// `tls_built_in_root_certs(false)`, and that absence is the webpki-continuity
/// pin (an in-suite public-endpoint probe would breach the zero-external-
/// networking discipline, so the source pin is the honest substitute).
pub(crate) fn relay_http_client() -> Result<HttpClient, RelayHttpClientError> {
    let mut builder = reqwest::blocking::Client::builder();
    if let Some(path) = relay_ca_file() {
        // Fail closed: a configured CA that cannot be loaded is an error. We
        // never silently proceed without the operator's CA and never fall back
        // to ignoring the option.
        let pem = read_relay_ca_file(path.as_str()).map_err(RelayHttpClientError::CaFile)?;
        let certs = reqwest::Certificate::from_pem_bundle(pem.as_slice())
            .map_err(|_| RelayHttpClientError::CaFile(RELAY_CA_FILE_INVALID))?;
        if certs.is_empty() {
            return Err(RelayHttpClientError::CaFile(RELAY_CA_FILE_INVALID));
        }
        for cert in certs {
            builder = builder.add_root_certificate(cert);
        }
    }
    builder.build().map_err(|_| RelayHttpClientError::Build)
}

/// Does this error chain report a certificate-verification refusal?
///
/// Typed, not string-matched: walks the std source chain and matches the
/// `rustls::Error::InvalidCertificate` class by VALUE, so it is robust to
/// upstream message wording. rustls reports the whole verification class here
/// (unknown issuer, name mismatch, expiry) — that class IS "certificate not
/// trusted" for taxonomy purposes.
fn relay_error_is_tls_untrusted(err: &(dyn std::error::Error + 'static)) -> bool {
    let mut cursor: Option<&(dyn std::error::Error + 'static)> = Some(err);
    while let Some(current) = cursor {
        if let Some(tls_err) = current.downcast_ref::<rustls::Error>() {
            return matches!(tls_err, rustls::Error::InvalidCertificate(_));
        }
        // std::io::Error is the load-bearing hop: tokio-rustls reports a handshake
        // refusal as io::Error::new(InvalidData, rustls::Error), and io::Error's
        // `source()` delegates to the INNER error's source rather than yielding the
        // inner error itself. Without `get_ref()` the walk steps straight past the
        // rustls value and the trust failure collapses back into the opaque outcome.
        if let Some(io_err) = current.downcast_ref::<std::io::Error>() {
            if let Some(inner) = io_err.get_ref() {
                if let Some(tls_err) = inner.downcast_ref::<rustls::Error>() {
                    return matches!(tls_err, rustls::Error::InvalidCertificate(_));
                }
                cursor = Some(inner);
                continue;
            }
        }
        cursor = current.source();
    }
    false
}

/// Pure classifier, socket-free and testable — the house `_from_parts` shape.
/// Returns the trust outcome when the failure was a certificate refusal, and
/// otherwise the caller's OWN pre-existing failure value, unchanged.
fn relay_send_outcome_from_parts(tls_untrusted: bool, fallback: &'static str) -> &'static str {
    if tls_untrusted {
        RELAY_TLS_UNTRUSTED
    } else {
        fallback
    }
}

/// Classify a live send error against a call site's existing failure value.
fn relay_send_outcome_for_error(err: &reqwest::Error, fallback: &'static str) -> &'static str {
    relay_send_outcome_from_parts(relay_error_is_tls_untrusted(err), fallback)
}

/// Presence class + redacted hash of the configured CA path. The raw path is
/// never published — the `relay_token_set` redaction precedent.
pub struct RelayCaFileStatus {
    pub configured: bool,
    pub path_hash: Option<String>,
}

/// Set the explicit relay CA-file path (vault-backed; env still takes
/// precedence at resolution time). Pub for the GUI Server pane (slice B).
pub fn relay_ca_file_set(path: &str) -> Result<(), &'static str> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(RELAY_CA_FILE_MISSING);
    }
    vault::secret_set(TUI_RELAY_CA_FILE_SECRET_KEY, trimmed)
}

/// Clear the explicit relay CA-file path.
pub fn relay_ca_file_clear() -> Result<(), &'static str> {
    vault::secret_set(TUI_RELAY_CA_FILE_SECRET_KEY, "")
}

/// Inspect the configured CA-file path: presence + hash only, never the path.
pub fn relay_ca_file_show() -> RelayCaFileStatus {
    match relay_ca_file_from_account_secret() {
        Some(path) => RelayCaFileStatus {
            configured: true,
            path_hash: Some(route_token_hash8(path.as_str())),
        },
        None => RelayCaFileStatus {
            configured: false,
            path_hash: None,
        },
    }
}

// ===========================================================================
// NA-0672 (D608, D-1300): server-info consumer + relay token trio.
//
// GUI slice B's Server pane and the `relay server-info` CLI verb both call
// `relay_server_info`, which returns a PRE-CLASSIFIED typed outcome. An open
// relay answers push/pull with a byte-identical 200, so ONLY `auth.mode`
// separates connected/open from connected/bearer: the taxonomy MUST live here,
// tested against the real DOC-SRV-006 / NA-0652 contract, and NOT be re-derived
// in slice-B JS.
//
// This adds a NEW probe. It does NOT touch the send/pull/ack `Authorization`
// attach points in `relay_inbox_push` / `relay_inbox_pull_mode` /
// `relay_inbox_ack` -- that is ENG-0051, a later messaging-slice lane. Zero
// dependency motion: it reuses `relay_http_client()` (native-roots ∪ webpki ∪
// operator-CA, fail-closed) and `resp.json()` (the reqwest `json` feature is
// already on).
// ===========================================================================

/// The relay's advertised authentication posture (`auth.mode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayAuthMode {
    Open,
    Bearer,
}

/// The per-request ceiling on the server-info probe. FLAG-4: it lives on the
/// PROBE request only, NEVER on `relay_http_client()`, which the send/pull/ack
/// path shares -- a client-level timeout would reach those callers.
const RELAY_SERVER_INFO_TIMEOUT_SECS: u64 = 10;

fn relay_auth_mode_from_str(mode: &str) -> Option<RelayAuthMode> {
    match mode {
        "open" => Some(RelayAuthMode::Open),
        "bearer" => Some(RelayAuthMode::Bearer),
        _ => None,
    }
}

/// The qsl-relay contract boundary (FLAG-2): a body IS a QSL relay challenge iff
/// it carries `auth.mode ∈ {open, bearer}`. Read straight off the JSON `Value`,
/// so a cosmetic type quirk in any OTHER field never changes the classification.
///
/// TOLERANT BY DESIGN: deliberately does NOT require `server == "qsl-server"`. A
/// protocol-compatible AGPL fork self-hosting a relay is a legitimate outcome; a
/// strict identifier check would reject it for no security gain.
fn relay_auth_mode_from_body(body: &serde_json::Value) -> Option<RelayAuthMode> {
    let mode = body.get("auth")?.get("mode")?.as_str()?;
    relay_auth_mode_from_str(mode)
}

// The nested wire shape of GET /v1/server-info exactly as the server emits it
// (DOC-SRV-006 / NA-0652 as-built). Every field `serde(default)` and unknown
// fields are tolerated (serde ignores them) -- the additive-only rule (OBS-H):
// a newer relay may add fields and an older client must still parse.
//
// `auth` is deliberately ABSENT here: the auth-mode challenge is read straight
// off the JSON `Value` by `relay_auth_mode_from_body`, so the classification
// never depends on the full-document deserialize succeeding (FLAG-2).
#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoLimitsWire {
    #[serde(default)]
    max_body_bytes: u64,
    #[serde(default)]
    max_queue_depth: u64,
    // NA-0681 (D616 C9): additive. Slice 1 advertises this; the client did not read it.
    #[serde(default)]
    max_invite_bundle_bytes: u64,
}

/// NA-0681 (D616 C9): the `invite` object Slice 1 added to `/v1/server-info`.
/// Additive per DOC-SRV-006 rule 1 -- nothing removed, renamed or repurposed. Read so the
/// client can pre-clamp its own expiry against the relay's advertised ceiling (F3) instead
/// of discovering the clamp only when a redeemer is told the invite expired.
#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoInviteWire {
    #[serde(default)]
    max_expiry_secs: u64,
    #[serde(default)]
    max_slots: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoRetentionWire {
    #[serde(default)]
    ttl_secs: u64,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoModeWire {
    #[serde(default)]
    mode: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoAttachmentsWire {
    #[serde(default)]
    service_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct ServerInfoWire {
    #[serde(default)]
    version: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    api: Vec<String>,
    #[serde(default)]
    limits: ServerInfoLimitsWire,
    #[serde(default)]
    retention: ServerInfoRetentionWire,
    #[serde(default)]
    directory: ServerInfoModeWire,
    #[serde(default)]
    attachments: ServerInfoAttachmentsWire,
    #[serde(default)]
    kt: ServerInfoModeWire,
    // NA-0681 (D616 C9): additive.
    #[serde(default)]
    invite: ServerInfoInviteWire,
    #[serde(default)]
    min_client_version: Option<String>,
}

/// The parsed server-info document (the FULL documented contract, DOC-SRV-006).
/// Flat and ergonomic for the GUI/CLI; built from the nested wire shape. Present
/// only inside `Reachable`, i.e. only when `auth.mode` was a valid QSL challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServerInfoDoc {
    pub name: String,
    pub version: String,
    pub api: Vec<String>,
    pub auth_mode: RelayAuthMode,
    pub max_body_bytes: u64,
    pub max_queue_depth: u64,
    pub retention_ttl_secs: u64,
    pub directory_mode: String,
    pub attachments_service_url: Option<String>,
    pub kt_mode: String,
    pub min_client_version: Option<String>,
    /// NA-0681 (D616 C9/F3). Zero means "not advertised" -- an older relay, or one that
    /// does not offer invites. The caller must treat zero as UNKNOWN and not as a ceiling
    /// of zero seconds.
    pub invite_max_expiry_secs: u64,
    pub invite_max_slots: u64,
    pub max_invite_bundle_bytes: u64,
}

impl ServerInfoDoc {
    fn from_wire(wire: ServerInfoWire, auth_mode: RelayAuthMode) -> Self {
        ServerInfoDoc {
            name: wire.name,
            version: wire.version,
            api: wire.api,
            auth_mode,
            max_body_bytes: wire.limits.max_body_bytes,
            max_queue_depth: wire.limits.max_queue_depth,
            retention_ttl_secs: wire.retention.ttl_secs,
            directory_mode: wire.directory.mode,
            attachments_service_url: wire.attachments.service_url,
            kt_mode: wire.kt.mode,
            min_client_version: wire.min_client_version,
            invite_max_expiry_secs: wire.invite.max_expiry_secs,
            invite_max_slots: wire.invite.max_slots,
            max_invite_bundle_bytes: wire.limits.max_invite_bundle_bytes,
        }
    }
}

/// The pre-classified server-info probe outcome. Five variants; two carry an
/// inner field, together yielding the seven observable network states the GUI
/// mockup enumerates: Reachable{Open}, Reachable{Bearer},
/// AuthRequired{token_was_sent:true} (token rejected),
/// AuthRequired{token_was_sent:false} (token required, none configured),
/// CertNotTrusted, Unreachable, NotAQslRelay.
// NA-0681 (D616 C9): the three additive invite fields on `ServerInfoDoc` push `Reachable`
// past clippy's variant-size threshold — 225 bytes against a 1-byte second-largest. The
// lint's remedy is to box the payload, but `RelayServerInfoOutcome` is PUBLIC and already
// consumed by the GUI slice-B path and `tests/NA_0672_relay_server_info.rs`, so boxing
// would be a public API change with a wider blast radius than the lint it silences. The
// enum is returned once per probe and never held in a collection, so the size difference
// costs nothing measurable. Allowed deliberately, with the reason, rather than reshaped.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelayServerInfoOutcome {
    /// 200 + a valid QSL contract. `doc.auth_mode` equals `auth_mode`.
    Reachable {
        auth_mode: RelayAuthMode,
        doc: ServerInfoDoc,
    },
    /// 401 + a parseable QSL challenge (`auth.mode` present). `token_was_sent`
    /// distinguishes "token rejected" (true) from "token required, none sent"
    /// (false).
    AuthRequired { token_was_sent: bool },
    /// The host answered, but the body is not a QSL relay contract: a 200 whose
    /// body lacks `auth.mode`, a generic reverse-proxy 401 with no QSL challenge,
    /// or any other status. This is the claim-discipline boundary (FLAG-2 RULE):
    /// the app NEVER tells a user "this relay requires a token" about a 401 that
    /// is not a relay.
    NotAQslRelay,
    /// TLS certificate verification refused (unknown issuer, name mismatch,
    /// expiry). Reuses the NA-0663 typed rustls downcast.
    CertNotTrusted,
    /// Connection/DNS failure or timeout -- nothing answered.
    Unreachable,
}

/// Pure, socket-free classifier -- the house `_from_parts` shape and the unit the
/// suite exercises. `status` is the HTTP status; `body` is the response body
/// parsed as JSON (`None` when the body was absent or not JSON); `token_was_sent`
/// records whether the probe attached an Authorization header.
///
/// PUB, not the private `fn` sketched in the directive: the directive's own named
/// integration test (`tests/NA_0672_relay_server_info.rs`) drives this classifier
/// socket-free, and an integration test can reach only `pub` items. Exposing the
/// pure taxonomy also serves the slice-B rationale directly -- a future GUI that
/// fetches server-info on its own runtime reuses this exact classification rather
/// than re-deriving it. `relay_server_info` remains the ordinary entry point.
///
/// FLAG-2 RULE: `AuthRequired` requires BOTH a 401 AND a parseable QSL challenge
/// (`auth.mode` present). A generic reverse-proxy 401 with no QSL body therefore
/// classifies `NotAQslRelay`, so the app never mislabels a non-relay as "requires
/// a token" (the §7.4 wrong-error-mapping bug that mocks would hide).
pub fn relay_server_info_from_parts(
    status: u16,
    body: Option<&serde_json::Value>,
    token_was_sent: bool,
) -> RelayServerInfoOutcome {
    let auth_mode = body.and_then(relay_auth_mode_from_body);
    match status {
        200 => match auth_mode {
            Some(mode) => {
                // auth.mode is present (a valid QSL contract), so this IS a
                // reachable relay. Parse the rest best-effort: a field-type
                // quirk falls back to defaults rather than losing the Reachable
                // classification, which keys ONLY on auth.mode.
                let doc = body
                    .and_then(|v| serde_json::from_value::<ServerInfoWire>(v.clone()).ok())
                    .unwrap_or_default();
                RelayServerInfoOutcome::Reachable {
                    auth_mode: mode,
                    doc: ServerInfoDoc::from_wire(doc, mode),
                }
            }
            None => RelayServerInfoOutcome::NotAQslRelay,
        },
        401 => match auth_mode {
            Some(_) => RelayServerInfoOutcome::AuthRequired { token_was_sent },
            None => RelayServerInfoOutcome::NotAQslRelay,
        },
        _ => RelayServerInfoOutcome::NotAQslRelay,
    }
}

/// Probe `GET {relay_base}/v1/server-info` and return a PRE-CLASSIFIED outcome.
///
/// `Err` carries ONLY a LOCAL-CONFIGURATION code, never a network result: the
/// `normalize_relay_endpoint` codes for a malformed/inadmissible typed address,
/// or -- like `relay_inbox_push` -- a CA-file code when a configured operator CA
/// cannot be loaded. Both mean "we could not even form the request." Every
/// reachable-network result -- success, auth challenge, non-relay answer, TLS
/// refusal, timeout -- is `Ok(variant)`.
///
/// The probe SENDS the resolved bearer token (env -> vault -> token-file) when
/// one is configured, so `AuthRequired` can distinguish token-rejected from
/// token-absent. Per-request 10s timeout (FLAG-4); never mutates the shared
/// `relay_http_client()`.
pub fn relay_server_info(relay_base: &str) -> Result<RelayServerInfoOutcome, &'static str> {
    let base = normalize_relay_endpoint(relay_base)?;
    let url = format!("{}/v1/server-info", base);
    let client = match relay_http_client() {
        Ok(client) => client,
        Err(RelayHttpClientError::CaFile(code)) => return Err(code),
        Err(RelayHttpClientError::Build) => return Err("relay_server_info_failed"),
    };
    let bearer_token = relay_auth_token();
    let token_was_sent = bearer_token.is_some();
    let mut req = client.get(url).timeout(std::time::Duration::from_secs(
        RELAY_SERVER_INFO_TIMEOUT_SECS,
    ));
    if let Some(token) = bearer_token {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(resp) => resp,
        Err(err) => {
            // Same typed rustls downcast the send path uses (NA-0663); a
            // certificate refusal is distinguishable from a plain unreachable.
            if relay_error_is_tls_untrusted(&err) {
                return Ok(RelayServerInfoOutcome::CertNotTrusted);
            }
            return Ok(RelayServerInfoOutcome::Unreachable);
        }
    };
    let status = resp.status().as_u16();
    // `None` when the body was absent or not JSON -- which, at 401, is exactly a
    // generic reverse-proxy challenge and classifies NotAQslRelay (FLAG-2 RULE).
    let body = resp.json::<serde_json::Value>().ok();
    Ok(relay_server_info_from_parts(
        status,
        body.as_ref(),
        token_was_sent,
    ))
}

/// Presence class ONLY -- no hash, no bytes. See the RULE on `relay_token_show`.
pub struct RelayTokenStatus {
    pub configured: bool,
}

/// Set the relay bearer token (account secret; the env token still takes
/// precedence at resolution time). Trims; empty is rejected. This is the ONE
/// writer -- the CLI `relay token-set` verb now routes through it, so there is a
/// single code path and no behaviour change.
pub fn relay_token_set(token: &str) -> Result<(), &'static str> {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return Err("relay_token_missing");
    }
    vault::secret_set(TUI_RELAY_TOKEN_SECRET_KEY, trimmed)
}

/// Clear the relay bearer token (account secret).
pub fn relay_token_clear() -> Result<(), &'static str> {
    vault::secret_set(TUI_RELAY_TOKEN_SECRET_KEY, "")
}

/// Inspect the configured relay bearer token: PRESENCE ONLY.
///
/// ⚠ RULE (FLAG-3), not an implementation note. `relay_ca_file_show` returns a
/// `path_hash` because a CA path is PUBLIC material. A bearer token is SECRET, so
/// `relay_token_show` returns a BARE BOOL -- even a truncated hash of a secret is
/// a needless oracle. The two SHOULD look inconsistent. Do NOT "fix" the
/// asymmetry by adding a hash here: doing so reintroduces the oracle.
pub fn relay_token_show() -> RelayTokenStatus {
    let configured = relay_auth_token_from_account_secret().is_some();
    RelayTokenStatus { configured }
}


const RELAY_PUSH_DIAGNOSTIC_ENV: &str = "QSC_RELAY_PUSH_DIAGNOSTIC";
const RELAY_PUSH_DIAGNOSTIC_MODE_REDACTED: &str = "redacted";

#[derive(Clone, Copy)]
struct RelayPushDiagnostic {
    status: Option<HttpStatus>,
    body_len: Option<u64>,
    error_class: &'static str,
    diagnostic_class: &'static str,
    timeout_phase_class: &'static str,
    qsc_error: &'static str,
    route_header_present: bool,
    auth_present: bool,
}

fn relay_push_diagnostic_enabled() -> bool {
    env::var(RELAY_PUSH_DIAGNOSTIC_ENV)
        .ok()
        .map(|v| v == RELAY_PUSH_DIAGNOSTIC_MODE_REDACTED)
        .unwrap_or(false)
}

fn relay_push_status_class(status: Option<HttpStatus>) -> &'static str {
    match status.map(|s| s.as_u16() / 100) {
        Some(2) => "2xx",
        Some(3) => "3xx",
        Some(4) => "4xx",
        Some(5) => "5xx",
        _ => "unknown",
    }
}

fn relay_push_body_presence(body_len: Option<u64>) -> &'static str {
    match body_len {
        Some(0) => "false",
        Some(_) => "true",
        None => "unknown",
    }
}

/// NA-0686 / D-1325 (ENG-0082) — the 401/403 collapse ENDS HERE, at the marker layer.
///
/// ⚠ **401 IS DELIBERATELY UNCHANGED.** `relay_unauthorized` still means exactly
/// what it meant, and `NA_0663_relay_tls_trust.rs` — whose assertion exists to
/// stop a 401 being misreported as a TLS trust failure — is NOT touched by this
/// split and must pass byte-identical afterwards. That is the measurement which
/// proves the split kept its promise. ENG-0082 was filed believing this fix
/// "requires rewriting NA_0663's assertion"; measurement showed otherwise —
/// NA_0663 contains no 403 case at all, so splitting only the 403 arm leaves
/// every existing assertion true. Rewriting a guard to match new behaviour is
/// the dangerous edit class, and it turned out to be unnecessary.
///
/// The two causes are genuinely different and now say so: **401** is a fixable
/// token rejection that PAUSES and resumes on a settings save; **403** on the
/// invite path is a ticketless push to a consumed slot, which is not a pause at
/// all. `PushFailClass` has carried that distinction internally since NA-0682
/// (Option B) — this makes it legible in a raw log, which is the whole of what
/// ENG-0082 asked for.
///
/// ⚠ DIAGNOSTIC ONLY: these strings feed `emit_relay_push_diagnostic`. Pause
/// cause, retry policy and the C11 classification all derive from
/// `push_fail_class_for_status`, which is untouched. No behaviour moves.
fn relay_push_qsc_error_for_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::OK => "none",
        HttpStatus::UNAUTHORIZED => "relay_unauthorized",
        HttpStatus::FORBIDDEN => "relay_forbidden",
        HttpStatus::PAYLOAD_TOO_LARGE => "relay_inbox_too_large",
        HttpStatus::TOO_MANY_REQUESTS => "relay_inbox_queue_full",
        _ => "relay_inbox_push_failed",
    }
}

/// NA-0686 / D-1325 (ENG-0082) — the second of the two named collapse sites.
///
/// ⚠ The token is `access_forbidden`, NOT `relay_forbidden`, and the difference
/// is deliberate: this function's vocabulary is `<subject>_<disposition>`
/// (`auth_rejected`, `route_rejected`, `endpoint_not_found`,
/// `payload_rejected`), while the marker-code function's is `relay_*`. D-1324's
/// standing rule is that a lane **adopts the vocabulary the tree already uses**
/// and derives its mapping from that usage, so each site takes the form its own
/// neighbours take rather than importing the other's.
fn relay_push_error_class_for_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::OK => "unknown",
        HttpStatus::UNAUTHORIZED => "auth_rejected",
        HttpStatus::FORBIDDEN => "access_forbidden",
        HttpStatus::BAD_REQUEST => "route_rejected",
        HttpStatus::NOT_FOUND => "endpoint_not_found",
        HttpStatus::PAYLOAD_TOO_LARGE => "payload_rejected",
        HttpStatus::TOO_MANY_REQUESTS => "route_rejected",
        _ => "unexpected_status",
    }
}

/// NA-0686 / D-1325 (ENG-0082) — the THIRD collapse site, ruled in mid-lane.
///
/// ⚠ Why it could not be left: ENG-0082 cannot close while one collapse still
/// stands, or the ledger's closure claim would be false. This lane found it while
/// fixing the two named sites and reported it; the operator ruled it in.
///
/// ⚠ **`bearer_auth_failed` was not merely imprecise for a 403 — it was WRONG.**
/// The neighbours in this function name WHICH CREDENTIAL failed
/// (`bearer_auth_failed`, `route_token_auth_failed`). A 403 is the case where the
/// bearer token was ACCEPTED and the request was refused anyway — on the invite
/// path, a ticketless push to a consumed slot. Reporting that as a bearer failure
/// sends an operator to re-check a token that was never the problem.
///
/// So the token here is **`access_refused`**, which keeps this function's
/// `<subject>_<outcome>` shape while deliberately NOT saying `auth_failed` —
/// because "auth failed" is precisely the false statement the collapse was
/// making. This is D-1324's rule applied per layer: the marker-code site took
/// `relay_forbidden` and the error-class site took `access_forbidden`, each from
/// its own neighbours; this site takes its own too rather than importing either.
///
/// ⚠ 401 IS UNCHANGED, again. Both existing consumers of this field
/// (`relay_push_diagnostics.rs`, `secret_material_diagnostic_boundary.rs`) are
/// driven by a 401 and assert `diagnostic_class=bearer_auth_failed`; splitting
/// only the 403 arm leaves both true and byte-identical, exactly as with NA_0663.
///
/// DIAGNOSTIC ONLY: this string feeds `emit_relay_push_diagnostic` and nothing
/// else. No pause cause, retry policy or classification reads it.
fn relay_push_diagnostic_class_for_status(status: HttpStatus) -> &'static str {
    match status {
        HttpStatus::UNAUTHORIZED => "bearer_auth_failed",
        HttpStatus::FORBIDDEN => "access_refused",
        HttpStatus::BAD_REQUEST => "route_token_auth_failed",
        _ => "http_status_received",
    }
}

fn relay_push_timeout_phase_class_from_parts(
    is_timeout: bool,
    is_connect: bool,
    error_text: &str,
) -> &'static str {
    if !is_timeout {
        return "not_timeout";
    }

    let lower = error_text.to_ascii_lowercase();
    if lower.contains("dns")
        || lower.contains("name or service")
        || lower.contains("failed to lookup")
    {
        return "dns_timeout";
    }
    if lower.contains("tls") || lower.contains("certificate") {
        return "tls_handshake_timeout";
    }
    if is_connect || lower.contains("connect") {
        return "tcp_connect_timeout";
    }
    if lower.contains("request") || lower.contains("body") || lower.contains("response") {
        return "http_request_timeout";
    }
    "unknown_timeout"
}

fn relay_push_diagnostic_class_from_error_parts(
    is_timeout: bool,
    is_connect: bool,
    error_text: &str,
) -> &'static str {
    let lower = error_text.to_ascii_lowercase();
    if lower.contains("connection refused") {
        return "connection_refused";
    }
    if lower.contains("connection reset") {
        return "connection_reset";
    }
    if is_timeout {
        return relay_push_timeout_phase_class_from_parts(true, is_connect, error_text);
    }
    "not_timeout"
}

fn relay_push_timeout_phase_class_for_send_error(err: &reqwest::Error) -> &'static str {
    relay_push_timeout_phase_class_from_parts(err.is_timeout(), err.is_connect(), &err.to_string())
}

fn relay_push_diagnostic_class_for_send_error(err: &reqwest::Error) -> &'static str {
    relay_push_diagnostic_class_from_error_parts(
        err.is_timeout(),
        err.is_connect(),
        &err.to_string(),
    )
}

fn relay_push_error_class_for_send_error(err: &reqwest::Error) -> &'static str {
    if err.is_timeout() {
        return "timeout";
    }
    let lower = err.to_string().to_ascii_lowercase();
    if lower.contains("tls") || lower.contains("certificate") {
        return "tls_error";
    }
    if err.is_connect() {
        return "network_error";
    }
    "transport_error"
}

fn emit_relay_push_diagnostic(diag: RelayPushDiagnostic) {
    if !relay_push_diagnostic_enabled() {
        return;
    }

    let status_code = diag
        .status
        .map(|status| status.as_u16().to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let body_len = diag
        .body_len
        .map(|len| len.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let route_header_present = if diag.route_header_present {
        "true"
    } else {
        "false"
    };
    let auth_present = if diag.auth_present { "true" } else { "false" };

    emit_marker(
        "relay_push_diagnostic",
        None,
        &[
            ("diagnostic", RELAY_PUSH_DIAGNOSTIC_ENV),
            ("mode", RELAY_PUSH_DIAGNOSTIC_MODE_REDACTED),
            ("api", "relay_push_v1"),
            ("status_class", relay_push_status_class(diag.status)),
            ("status_code", status_code.as_str()),
            ("error_class", diag.error_class),
            ("diagnostic_class", diag.diagnostic_class),
            ("timeout_phase_class", diag.timeout_phase_class),
            (
                "response_body_present",
                relay_push_body_presence(diag.body_len),
            ),
            ("response_body_len", body_len.as_str()),
            ("route_header_present", route_header_present),
            ("auth_present", auth_present),
            ("qsc_error", diag.qsc_error),
            ("attempt", "1"),
        ],
    );
}

/// D617 census C11, resolved by **Option B** (operator-ruled 2026-07-28, STOP 007).
///
/// The message queue must tell a **401 token rejection** (a fixable PAUSE that resumes on a
/// settings save) from a **403** (which is not a pause at all -- on the invite path it is a
/// ticketless push to a consumed slot). The shipped DIAGNOSTIC code cannot carry that
/// distinction: `relay_push_qsc_error_for_status` collapses `UNAUTHORIZED | FORBIDDEN` into
/// the single marker `relay_unauthorized`, and four test files depend on that string --
/// including `NA_0663_relay_tls_trust.rs`, whose assertion exists precisely to stop a 401
/// being misreported as a TLS trust failure.
///
/// **Option A -- renaming the 401 code -- was REJECTED**: it would have required rewriting
/// that guard, and rewriting a guard to match new behaviour silently weakens a safety
/// assertion. The tree already establishes the safer pattern (NA-0681 refined this same
/// match *conditionally* and kept the 403 default "byte-identical to before").
///
/// So the classification rides **beside** the code rather than replacing it: every existing
/// caller still sees exactly the `&'static str` it saw before, no marker string moves, and
/// the queue derives its PAUSE cause from the class. The remaining marker-layer collapse
/// (401 and 403 indistinguishable in raw logs) is a narrow diagnostic-observability gap and
/// is FILED, not silenced.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum PushFailClass {
    /// 401 — the relay refused the access token. Fixable, so it PAUSES rather than fails.
    TokenRejected,
    /// 403 — refused for a reason that is not the bearer token.
    Forbidden,
    /// 413 — larger than this relay accepts. Fails THAT MESSAGE ONLY.
    TooLarge,
    /// 429 — the mailbox is full. Retryable.
    QueueFull,
    /// The certificate could not be trusted. Fail-closed, PAUSES.
    CertUntrusted,
    /// Unreachable, timeout, 5xx — retryable.
    Transient,
    /// Anything else. Retryable, because O4 forbids surfacing a retryable failure as
    /// permanent and "unknown" is not evidence of permanence.
    Other,
}

pub(crate) struct PushFailure {
    /// The UNCHANGED shipped code. Existing callers see only this.
    pub(crate) code: &'static str,
    pub(crate) class: PushFailClass,
}

fn push_fail_class_for_status(status: HttpStatus) -> PushFailClass {
    match status {
        HttpStatus::UNAUTHORIZED => PushFailClass::TokenRejected,
        HttpStatus::FORBIDDEN => PushFailClass::Forbidden,
        HttpStatus::PAYLOAD_TOO_LARGE => PushFailClass::TooLarge,
        HttpStatus::TOO_MANY_REQUESTS => PushFailClass::QueueFull,
        s if s.is_server_error() => PushFailClass::Transient,
        _ => PushFailClass::Other,
    }
}

pub(super) fn relay_inbox_push(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
) -> Result<(), &'static str> {
    relay_inbox_push_with_ticket(relay_base, route_token, payload, None)
}

/// The classified form the message queue uses. Same request, same diagnostics, same codes --
/// it just also reports WHICH failure it was, so a PAUSE cause can be derived from the
/// status instead of from a string that deliberately collapses two causes.
pub(crate) fn relay_inbox_push_classified(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
) -> Result<(), PushFailure> {
    relay_inbox_push_inner(relay_base, route_token, payload, None)
}

/// NA-0681 (D616 §2e): the same push, optionally presenting the one-shot invite ticket.
///
/// The ticket is a HEADER, not a body field, because `/v1/push`'s body IS the opaque
/// handshake payload and cannot be repurposed -- the relay's own contract says so, matching
/// the `X-QSL-Route-Token` precedent. `relay_inbox_push` delegates here with `None`, so
/// every existing call site keeps byte-identical behaviour and no ticket header is ever
/// sent on an ordinary push.
pub(super) fn relay_inbox_push_with_ticket(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
    ticket: Option<&str>,
) -> Result<(), &'static str> {
    // Every pre-existing caller sees exactly the code it saw before; the class is dropped.
    relay_inbox_push_inner(relay_base, route_token, payload, ticket).map_err(|f| f.code)
}

fn relay_inbox_push_inner(
    relay_base: &str,
    route_token: &str,
    payload: &[u8],
    ticket: Option<&str>,
) -> Result<(), PushFailure> {
    let fail = |code: &'static str, class: PushFailClass| PushFailure { code, class };
    let route_token =
        normalize_route_token(route_token).map_err(|c| fail(c, PushFailClass::Other))?;
    let base = normalize_relay_endpoint(relay_base).map_err(|c| fail(c, PushFailClass::Other))?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/push", base);
    let client = match relay_http_client() {
        Ok(v) => v,
        // A configured CA that cannot be read is a LOCAL file problem, not a trust failure --
        // the distinction NA-0674 states for states 12 vs 3. Either way it pauses.
        Err(RelayHttpClientError::CaFile(code)) => {
            return Err(fail(code, PushFailClass::CertUntrusted))
        }
        Err(RelayHttpClientError::Build) => {
            return Err(fail("relay_inbox_push_failed", PushFailClass::Other))
        }
    };
    let mut req = client
        .post(url)
        .header("X-QSL-Route-Token", route_token.as_str())
        .body(payload.to_vec());
    if let Some(t) = ticket {
        req = req.header("X-QSL-Invite-Ticket", t);
    }
    let bearer_token = relay_auth_token();
    let auth_present = bearer_token.is_some();
    if let Some(token) = bearer_token {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(err) => {
            // NA-0663: the certificate-verification class becomes distinguishable.
            // error_class / diagnostic_class / timeout_phase_class are UNCHANGED --
            // the new outcome flows through the EXISTING qsc_error field only.
            let outcome = relay_send_outcome_for_error(&err, "relay_inbox_push_failed");
            emit_relay_push_diagnostic(RelayPushDiagnostic {
                status: None,
                body_len: None,
                error_class: relay_push_error_class_for_send_error(&err),
                diagnostic_class: relay_push_diagnostic_class_for_send_error(&err),
                timeout_phase_class: relay_push_timeout_phase_class_for_send_error(&err),
                qsc_error: outcome,
                route_header_present: true,
                auth_present,
            });
            // The cert class is already distinguishable here (NA-0663); everything else on
            // this branch is a transport failure, which is retryable by definition.
            let class = if outcome == "relay_tls_untrusted" {
                PushFailClass::CertUntrusted
            } else {
                PushFailClass::Transient
            };
            return Err(fail(outcome, class));
        }
    };
    let status = resp.status();
    emit_relay_push_diagnostic(RelayPushDiagnostic {
        status: Some(status),
        body_len: resp.content_length(),
        error_class: relay_push_error_class_for_status(status),
        diagnostic_class: relay_push_diagnostic_class_for_status(status),
        timeout_phase_class: "not_timeout",
        qsc_error: relay_push_qsc_error_for_status(status),
        route_header_present: true,
        auth_present,
    });
    // ⚠ The CODES below are unchanged from before NA-0682. Only the CLASS is added, and it
    // is derived from the status rather than from the code -- which is the whole point of
    // Option B, because `relay_unauthorized` deliberately covers two different causes.
    let class = push_fail_class_for_status(status);
    match status {
        HttpStatus::OK => Ok(()),
        // NA-0681: an invite slot refuses a push that presents no live ticket, and says so
        // with its own code. Only reachable when a ticket was offered, i.e. on the invite
        // handshake path -- an ordinary push never sends the header and a 403 there stays
        // `relay_unauthorized`, byte-identical to before.
        HttpStatus::FORBIDDEN if ticket.is_some() => {
            Err(fail(crate::invite::INVITE_TICKET_INVALID, class))
        }
        HttpStatus::GONE if ticket.is_some() => Err(fail(
            crate::invite::INVITE_EXPIRED_AT_RELAY,
            PushFailClass::Other,
        )),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err(fail("relay_unauthorized", class)),
        HttpStatus::PAYLOAD_TOO_LARGE => Err(fail("relay_inbox_too_large", class)),
        HttpStatus::TOO_MANY_REQUESTS => Err(fail("relay_inbox_queue_full", class)),
        _ => Err(fail("relay_inbox_push_failed", class)),
    }
}

/// NA-0688 C4 (D622) SITE 2 of 2 — the flag-less pull.
///
/// ⚠ **THIS ONE HARDCODED `AckMode::Legacy` AND NOW RESOLVES LIKE EVERY OTHER PULL.** It takes no
/// `AckMode` parameter because none of its callers has a flag to pass, which is exactly why the
/// hardcode was dangerous: it was unreachable by `--ack-mode` and so could not be escaped.
///
/// ⚠ **THREE PRODUCTION COMMANDS MOVE TOGETHER HERE, and that is intended.** Ratified membership:
///   1. `invite accept`  — `invite_accept_at`, pulls the invite's OWN mailbox (`invite_id_wire`), `--max 1`
///   2. `invite finish`  — `invite_finish`, ⚠ pulls the user's **ORDINARY inbox**, `--max 1`
///   3. `handshake poll` — `perform_handshake_poll*`, `--max 4`
///
/// ⚠ **`invite redeem` IS NOT AMONG THEM.** `invite_redeem_at` reaches the relay only through
/// `invite_redeem_call` (`POST /v1/invite/redeem`) — a different route, no inbox pull, no
/// `AckMode` — so it is untouched by this default. It is named here **because C4's own census
/// wrongly listed it**, having identified the two `invite/mod.rs` call sites by line number
/// instead of bracketing them to their enclosing functions. **A line number identifies a
/// location, never a function.**
///
/// Only `handshake poll` had been named before the census. ⚠ **`invite finish` matters most**:
/// it pulls the mailbox where a peer's ordinary messages sit, under a command the user is
/// required to run — and it processes only `.next()`, so under the old delete-on-pull default
/// anything else it collected was destroyed with no witness, at exit 0.
pub(super) fn relay_inbox_pull(
    relay_base: &str,
    route_token: &str,
    max: usize,
) -> Result<Vec<InboxPullItem>, &'static str> {
    relay_inbox_pull_mode(relay_base, route_token, max, crate::resolve_ack_mode(None))
}

fn relay_inbox_pull_mode(
    relay_base: &str,
    route_token: &str,
    max: usize,
    ack_mode: AckMode,
) -> Result<Vec<InboxPullItem>, &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    // Legacy keeps the exact pre-NA-0644 URL. Lease adds the opt-in ack param, which a
    // pre-durability relay silently ignores (it then behaves legacy end-to-end).
    let url = match ack_mode {
        AckMode::Legacy => format!("{}/v1/pull?max={}", base, max),
        AckMode::Lease => format!("{}/v1/pull?max={}&ack=lease", base, max),
    };
    let client = match relay_http_client() {
        Ok(v) => v,
        Err(RelayHttpClientError::CaFile(code)) => return Err(code),
        Err(RelayHttpClientError::Build) => return Err("relay_inbox_pull_failed"),
    };
    let mut req = client
        .get(url)
        .header("X-QSL-Route-Token", route_token.as_str());
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(err) => return Err(relay_send_outcome_for_error(&err, "relay_inbox_pull_failed")),
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

// NA-0644 (D580): POST /v1/pull/ack — acknowledge durably persisted ids so the relay
// deletes its leased copies. A 404 is the pre-durability relay (no ack route): it
// already delivered legacy-style, so the caller must treat it as legacy-complete.
fn relay_inbox_ack(
    relay_base: &str,
    route_token: &str,
    ids: &[String],
) -> Result<AckFlushOutcome, &'static str> {
    let route_token = normalize_route_token(route_token)?;
    let base = normalize_relay_endpoint(relay_base)?;
    let base = base.trim_end_matches('/');
    let url = format!("{}/v1/pull/ack", base);
    let body = match serde_json::to_vec(&AckReq { ids: ids.to_vec() }) {
        Ok(v) => v,
        Err(_) => return Err("relay_ack_failed"),
    };
    let client = match relay_http_client() {
        Ok(v) => v,
        Err(RelayHttpClientError::CaFile(code)) => return Err(code),
        Err(RelayHttpClientError::Build) => return Err("relay_ack_failed"),
    };
    let mut req = client
        .post(url)
        .header("X-QSL-Route-Token", route_token.as_str())
        .header("Content-Type", "application/json")
        .body(body);
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(err) => return Err(relay_send_outcome_for_error(&err, "relay_ack_failed")),
    };
    match resp.status() {
        HttpStatus::OK => {
            let body: AckResp = match resp.json() {
                Ok(v) => v,
                Err(_) => return Err("relay_ack_parse_failed"),
            };
            Ok(AckFlushOutcome::Acked(body.acked))
        }
        HttpStatus::NOT_FOUND => Ok(AckFlushOutcome::LegacyComplete),
        HttpStatus::UNAUTHORIZED | HttpStatus::FORBIDDEN => Err("relay_unauthorized"),
        _ => Err("relay_ack_failed"),
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

pub(super) fn relay_send_with_payload(args: RelaySendPayloadArgs<'_>) -> CliResult<RelaySendOutcome> {
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
        origination,
    } = args;
    if let Err(code) = normalize_relay_endpoint(relay) {
        return Ok(RelaySendOutcome {
            action: "endpoint_reject".to_string(),
            delivered: false,
            error_code: Some(code),
        });
    }
    let routing = match routing_override {
        Some(v) => v,
        None => match resolve_send_routing_target(to) {
            Ok(v) => v,
            Err(code) => {
                return Ok(RelaySendOutcome {
                    action: "route_token_reject".to_string(),
                    delivered: false,
                    error_code: Some(code),
                });
            }
        },
    };
    emit_cli_routing_marker(
        routing.peer_alias.as_str(),
        routing.device_id.as_str(),
        routing.implicit_primary,
    );
    emit_cli_confirm_policy();
    let push_route_token = routing.route_token.clone();
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => return Err(cli_err(e)),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        return Err(cli_err(e));
    }

    let outbox_path = dir.join(OUTBOX_FILE_NAME);
    if outbox_path.exists() {
        let outbox = match outbox_record_load(&outbox_path) {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return Ok(RelaySendOutcome {
                    action: "outbox_load_failed".to_string(),
                    delivered: false,
                    error_code: Some(code),
                });
            }
        };
        if outbox.to.is_empty() || outbox.ciphertext.is_empty() {
            emit_marker("error", Some("outbox_recovery_required"), &[]);
            return Ok(RelaySendOutcome {
                action: "outbox_recovery_required".to_string(),
                delivered: false,
                error_code: Some("outbox_recovery_required"),
            });
        }
        let next_state = match outbox_next_state_load() {
            Ok(v) => v,
            Err(code) => {
                emit_marker("error", Some(code), &[]);
                return Ok(RelaySendOutcome {
                    action: "outbox_state_missing".to_string(),
                    delivered: false,
                    error_code: Some(code),
                });
            }
        };
        let replay_route_token = if outbox.to == routing.peer_alias {
            routing.route_token.clone()
        } else {
            match relay_peer_route_token(outbox.to.as_str()) {
                Ok(v) => v,
                Err(code) => {
                    return Ok(RelaySendOutcome {
                        action: "route_token_reject".to_string(),
                        delivered: false,
                        error_code: Some(code),
                    });
                }
            }
        };
        print_marker("send_retry", &[("mode", "outbox_replay")]);
        return Ok(match relay_inbox_push(relay, replay_route_token.as_str(), &outbox.ciphertext) {
            Ok(()) => finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "replay_deliver".to_string(),
                Some((
                    outbox.channel.as_deref().unwrap_or(outbox.to.as_str()),
                    next_state,
                )),
                None,
                Some(TimelineSendIngest {
                    peer: outbox.to.as_str(),
                    byte_len: outbox.payload_len,
                    kind: outbox.kind.as_str(),
                    message_id: outbox.message_id.as_deref(),
                    target_device_id: outbox.channel.as_deref().and_then(channel_device_id),
                }),
            )?,
            Err(code) => {
                print_marker("send_attempt", &[("ok", "false")]);
                RelaySendOutcome {
                    action: "push_fail".to_string(),
                    delivered: false,
                    error_code: Some(code),
                }
            }
        });
    }

    let (payload, receipt_msg_id) = encode_receipt_data_payload(payload, receipt)?;
    let pack = match qsp_pack(routing.channel.as_str(), &payload, pad_cfg, meta_seed, origination) {
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
            return Ok(RelaySendOutcome {
                action: err.code.to_string(),
                delivered: false,
                error_code: Some(err.code),
            });
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
            return Ok(RelaySendOutcome {
                action: "meta_bucket_invalid".to_string(),
                delivered: false,
                error_code: Some("meta_bucket_invalid"),
            });
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
            return Ok(RelaySendOutcome {
                action: "outbox_serialize_failed".to_string(),
                delivered: false,
                error_code: Some("outbox_serialize_failed"),
            });
        }
    };
    if write_atomic(&outbox_path, &outbox_bytes, source).is_err() {
        emit_marker("error", Some("outbox_write_failed"), &[]);
        return Ok(RelaySendOutcome {
            action: "outbox_write_failed".to_string(),
            delivered: false,
            error_code: Some("outbox_write_failed"),
        });
    }
    if let Err(code) = outbox_next_state_store(&pack.next_state) {
        let _ = fs::remove_file(&outbox_path);
        emit_marker("error", Some(code), &[]);
        return Ok(RelaySendOutcome {
            action: "outbox_state_store_failed".to_string(),
            delivered: false,
            error_code: Some(code),
        });
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
                    return Ok(RelaySendOutcome {
                        action: "drop".to_string(),
                        delivered: false,
                        error_code: Some("relay_drop_injected"),
                    });
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

    // NA-0624: deliver any SCKA control envelopes (advertisements) before the main message.
    // Their secret material is already durable (qsp_pack persists the SCKA store fail-closed
    // before returning an advertisement) and the chain advance is carried by the outbox
    // next-state, so a crash or push failure here is recovered by the normal outbox replay.
    for pre in pack.pre_envelopes.iter() {
        if let Err(code) = relay_inbox_push(relay, push_route_token.as_str(), pre) {
            emit_marker("relay_event", None, &[("action", "push_fail")]);
            print_marker("send_attempt", &[("ok", "false")]);
            return Ok(RelaySendOutcome {
                action: "push_fail".to_string(),
                delivered: false,
                error_code: Some(code),
            });
        }
        emit_marker("relay_event", None, &[("action", "deliver_control")]);
    }

    Ok(match relay_inbox_push(relay, push_route_token.as_str(), &ciphertext) {
        Ok(()) => {
            emit_marker("relay_event", None, &[("action", "deliver")]);
            emit_cli_delivery_state_with_device(
                to,
                "accepted_by_relay",
                Some(routing.device_id.as_str()),
            );
            finalize_send_commit(
                &dir,
                source,
                &outbox_path,
                "deliver".to_string(),
                Some((routing.channel.as_str(), pack.next_state.clone())),
                Some(&pack.trigger),
                Some(TimelineSendIngest {
                    peer: to,
                    byte_len: payload.len(),
                    kind: "file",
                    message_id: receipt_msg_id.as_deref(),
                    target_device_id: Some(routing.device_id.as_str()),
                }),
            )?
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
    })
}

fn finalize_send_commit(
    dir: &Path,
    source: ConfigSource,
    outbox_path: &Path,
    action: String,
    session_update: Option<(&str, Suite2SessionState)>,
    // NA-0624: the DH-ratchet trigger from qsp_pack. The deliver path MUST persist it —
    // without it the cleared ratchet-on-reply flag and the N/T fallback counters never land
    // on the main send path, every post-receive send ratchets, and the co-scheduled PQ-reseed
    // cadence (a non-boundary send) can never fire. The outbox-replay path has no pack
    // outcome and preserves the stored trigger (None).
    send_trigger: Option<&QspTriggerState>,
    timeline_ingest: Option<TimelineSendIngest<'_>>,
) -> CliResult<RelaySendOutcome> {
    let next_seq = match read_send_state(dir, source)? {
        Ok(v) => v + 1,
        Err(()) => {
            emit_marker("error", Some("send_state_parse_failed"), &[]);
            return Ok(RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("send_state_parse_failed"),
            });
        }
    };
    if let Some((peer, st)) = session_update {
        let stored = match send_trigger {
            Some(trig) => qsp_session_store_with_trigger(peer, &st, trig),
            None => qsp_session_store(peer, &st),
        };
        if stored.is_err() {
            emit_marker("error", Some("qsp_session_store_failed"), &[]);
            return Ok(RelaySendOutcome {
                action,
                delivered: true,
                error_code: Some("qsp_session_store_failed"),
            });
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
            emit_message_state_reject(code);
            emit_marker("error", Some(code), &[("op", "timeline_send_ingest")]);
        }
    }
    let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
    if write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source).is_err() {
        emit_marker("error", Some("send_commit_write_failed"), &[]);
        return Ok(RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("send_commit_write_failed"),
        });
    }
    if fs::remove_file(outbox_path).is_err() {
        emit_marker("error", Some("outbox_remove_failed"), &[]);
        return Ok(RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some("outbox_remove_failed"),
        });
    }
    if let Err(code) = outbox_next_state_clear() {
        emit_marker("error", Some(code), &[]);
        return Ok(RelaySendOutcome {
            action,
            delivered: true,
            error_code: Some(code),
        });
    }
    print_marker("send_attempt", &[("ok", "true")]);
    let seq_s = next_seq.to_string();
    print_marker("send_commit", &[("send_seq", seq_s.as_str())]);
    Ok(RelaySendOutcome {
        action,
        delivered: true,
        error_code: None,
    })
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

fn read_send_state(dir: &Path, source: ConfigSource) -> CliResult<Result<u64, ()>> {
    let path = dir.join(SEND_STATE_NAME);
    if let Err(e) = enforce_safe_parents(&path, source) {
        return Err(cli_err(e));
    }
    if !path.exists() {
        return Ok(Ok(0));
    }
    let mut f = match File::open(&path) {
        Ok(v) => v,
        Err(_) => return Ok(Err(())),
    };
    let mut buf = String::new();
    if f.read_to_string(&mut buf).is_err() {
        return Ok(Err(()));
    }
    for line in buf.lines() {
        if let Some(rest) = line.trim().strip_prefix("send_seq=") {
            let v = match rest.trim().parse::<u64>() {
                Ok(v) => v,
                Err(_) => return Ok(Err(())),
            };
            return Ok(Ok(v));
        }
    }
    Ok(Err(()))
}

// ---------------------------------------------------------------------------
// NA-0681 (D616) — the three invite-slot calls.
//
// They live HERE, not in `invite/`, so every socket in qsc stays in one file and these
// reuse `relay_http_client()`, the operator-CA handling and the NA-0663 TLS taxonomy for
// free (D616 F4). What they do NOT do is interpret anything: the bundle and signature are
// opaque bytes to this layer, exactly as they are to the relay.
//
// ⚠ Failure mapping keys on the relay's BODY CODE, not on the status alone. Two statuses
// are ambiguous by design -- 429 is both `ERR_INVITE_CAP_FULL` and `ERR_RATE_LIMITED`, and
// 410 is both `ERR_INVITE_REVOKED` and `ERR_INVITE_EXPIRED` -- and the failure taxonomy
// requires those causes to stay distinct. Mapping on status alone would collapse exactly
// the distinctions DESIGN §6 exists to preserve.
// ---------------------------------------------------------------------------

/// Map a relay error body to this client's taxonomy. Unknown codes fall back to the status
/// class rather than to a generic failure, so a newer relay does not become "something went
/// wrong".
fn invite_err_for(status: HttpStatus, body: &str) -> &'static str {
    use crate::invite as inv;
    match body.trim() {
        "ERR_INVITE_NOT_FOUND" => inv::INVITE_NOT_FOUND,
        "ERR_INVITE_REVOKED" => inv::INVITE_REVOKED,
        "ERR_INVITE_EXPIRED" => inv::INVITE_EXPIRED_AT_RELAY,
        "ERR_INVITE_ALREADY_USED" => inv::INVITE_ALREADY_USED,
        "ERR_INVITE_CAP_INVALID" => inv::INVITE_CAP_INVALID,
        "ERR_INVITE_TICKET_INVALID" => inv::INVITE_TICKET_INVALID,
        "ERR_INVITE_REVOKE_INVALID" => inv::INVITE_REVOKE_INVALID,
        "ERR_INVITE_CAP_FULL" => inv::INVITE_SLOT_CAP_FULL,
        "ERR_RATE_LIMITED" => inv::INVITE_RATE_LIMITED,
        "ERR_INVITE_TOO_LARGE" => inv::INVITE_TOO_LARGE,
        "ERR_INVITE_DUPLICATE" => inv::INVITE_CREATE_FAILED,
        "ERR_INVITE_BAD_BODY" => inv::INVITE_MALFORMED,
        "ERR_UNAUTHORIZED" => "relay_unauthorized",
        _ => match status {
            HttpStatus::UNAUTHORIZED => "relay_unauthorized",
            HttpStatus::NOT_FOUND => inv::INVITE_NOT_FOUND,
            HttpStatus::GONE => inv::INVITE_EXPIRED_AT_RELAY,
            HttpStatus::CONFLICT => inv::INVITE_ALREADY_USED,
            HttpStatus::FORBIDDEN => inv::INVITE_CAP_INVALID,
            HttpStatus::PAYLOAD_TOO_LARGE => inv::INVITE_TOO_LARGE,
            HttpStatus::TOO_MANY_REQUESTS => inv::INVITE_RATE_LIMITED,
            _ => inv::INVITE_CREATE_FAILED,
        },
    }
}

fn invite_post(
    relay_base: &str,
    path: &str,
    body: serde_json::Value,
    fallback: &'static str,
) -> Result<serde_json::Value, &'static str> {
    let base = normalize_relay_endpoint(relay_base)?;
    let url = format!("{}{}", base.trim_end_matches('/'), path);
    let client = match relay_http_client() {
        Ok(v) => v,
        Err(RelayHttpClientError::CaFile(code)) => return Err(code),
        Err(RelayHttpClientError::Build) => return Err(fallback),
    };
    let mut req = client.post(url).json(&body);
    // ⚠ ENG-0051 (D616 F5, operator-confirmed REPLICATE): the bearer token is attached
    // whenever one is configured, with no consultation of the relay's advertised auth mode
    // -- exactly as the three existing relay operations do. This is the SHIPPED behaviour
    // replicated, not a new decision, and it grows the ENG-0051 surface. Recorded there.
    if let Some(token) = relay_auth_token() {
        req = req.header("Authorization", format!("Bearer {}", token));
    }
    let resp = match req.send() {
        Ok(v) => v,
        Err(err) => return Err(relay_send_outcome_for_error(&err, fallback)),
    };
    let status = resp.status();
    if status != HttpStatus::OK {
        let text = resp.text().unwrap_or_default();
        return Err(invite_err_for(status, &text));
    }
    resp.json::<serde_json::Value>().map_err(|_| fallback)
}

/// `POST /v1/invite/create` -> the relay's one-time `revoke_token`.
///
/// `expiry` is sent as the client computed it and the relay CLAMPS rather than rejects; the
/// response cannot report that it did. That is why the caller pre-clamps against the
/// advertised ceiling and why a clamp is never an error (D616 F3/C7).
pub(super) fn invite_create_call(
    relay_base: &str,
    invite_id_wire: &str,
    cap_hash_hex: &str,
    expiry: u64,
    bundle: &[u8],
    invite_sig: &[u8],
) -> Result<String, &'static str> {
    let body = serde_json::json!({
        "invite_id": invite_id_wire,
        "cap_hash": cap_hash_hex,
        "expiry": expiry,
        // URL_SAFE_NO_PAD -- the engine the crate already uses and the one the relay's
        // hand-rolled codec emits. Measured byte-identical at Phase 0.
        "bundle_b64": URL_SAFE_NO_PAD.encode(bundle),
        "invite_sig_b64": URL_SAFE_NO_PAD.encode(invite_sig),
    });
    let v = invite_post(
        relay_base,
        "/v1/invite/create",
        body,
        crate::invite::INVITE_CREATE_FAILED,
    )?;
    v.get("revoke_token")
        .and_then(|t| t.as_str())
        .map(|t| t.to_string())
        .ok_or(crate::invite::INVITE_CREATE_FAILED)
}

/// `POST /v1/invite/redeem` -> the bundle, its signature, and the one-shot handshake ticket.
///
/// ⚠ Returns DECODED BYTES. The base64 strings are never handed upward and are never
/// compared against what was sent: the relay accepts padded input and emits unpadded, so a
/// string comparison would pass every same-implementation test and break against a relay
/// that re-encodes. The commitment is computed over these bytes.
pub(super) fn invite_redeem_call(
    relay_base: &str,
    invite_id_wire: &str,
    cap_wire: &str,
) -> Result<(Vec<u8>, Vec<u8>, String), &'static str> {
    let body = serde_json::json!({ "invite_id": invite_id_wire, "cap": cap_wire });
    let v = invite_post(
        relay_base,
        "/v1/invite/redeem",
        body,
        crate::invite::INVITE_CREATE_FAILED,
    )?;
    let get_b64 = |k: &str| -> Result<Vec<u8>, &'static str> {
        let s = v.get(k).and_then(|x| x.as_str()).ok_or(crate::invite::INVITE_MALFORMED)?;
        URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|_| crate::invite::INVITE_MALFORMED)
    };
    let bundle = get_b64("bundle_b64")?;
    let sig = get_b64("invite_sig_b64")?;
    let ticket = v
        .get("ticket")
        .and_then(|x| x.as_str())
        .filter(|t| !t.trim().is_empty())
        .ok_or(crate::invite::INVITE_MALFORMED)?
        .to_string();
    Ok((bundle, sig, ticket))
}

/// `POST /v1/invite/revoke`. Idempotent at the relay: a second revoke succeeds.
pub(super) fn invite_revoke_call(
    relay_base: &str,
    invite_id_wire: &str,
    revoke_token: &str,
) -> Result<(), &'static str> {
    let body =
        serde_json::json!({ "invite_id": invite_id_wire, "revoke_token": revoke_token });
    let v = invite_post(
        relay_base,
        "/v1/invite/revoke",
        body,
        crate::invite::INVITE_CREATE_FAILED,
    )?;
    if v.get("revoked").and_then(|x| x.as_bool()) == Some(true) {
        Ok(())
    } else {
        Err(crate::invite::INVITE_REVOKE_INVALID)
    }
}




// ---------------------------------------------------------------------------
// NA-0682 (D617 §2e): the crypto + network half of the drain.
//
// `msgqueue` owns the FIFO/backoff POLICY and knows nothing about ratchets or HTTP; this
// type is the other half. Keeping them apart is what lets the policy be unit-tested with a
// fake sender -- which is how "packed exactly once across four attempts" became a one-line
// assertion instead of an argument about ratchet internals.
// ---------------------------------------------------------------------------

pub(crate) struct RelayMessageSender<'a> {
    relay: &'a str,
    /// Captured at pack time and replayed at commit. On a REPLAY there is no pack outcome,
    /// so this stays `None` and the stored trigger is preserved -- the same rule
    /// `finalize_send_commit` already follows for the outbox-replay path.
    trigger: Option<QspTriggerState>,
    /// SCKA control envelopes that must be pushed BEFORE the message envelope, in order.
    pre_envelopes: Vec<Vec<u8>>,
    /// The routed device, captured at pack time for the timeline entry at commit.
    device_id: Option<String>,
    /// ⚠ METADATA-PRIVACY CONFIG, threaded through to `qsp_pack`.
    ///
    /// These four were silently DROPPED when `qsc send` was rewired around
    /// `relay_send_with_payload` — and the whole test suite stayed green, because no test
    /// asserts padding. Clippy's unused-variable warning was the only thing that noticed.
    /// `pad_cfg` and `bucket_max` are metadata padding/bucketing, `meta_seed` is
    /// deterministic-meta mode, and `receipt` is the requested receipt kind: dropping them
    /// is a privacy regression, not a tidiness one.
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt_kind: Option<ReceiptKind>,
    /// The relay's advertised body limit, fetched only when a 413 actually happens.
    last_limit: Option<u64>,
    /// ⚠ The PRECISE failure code from the last attempt.
    ///
    /// `AttemptResult` deliberately carries only the queue's coarse vocabulary (retry /
    /// pause / fail), because that is all the FIFO policy needs. But the CLI must still
    /// report the specific cause -- `relay_drop_injected` is not `relay_inbox_push_failed`,
    /// and collapsing them would be exactly the "distinct causes, distinct words" failure
    /// this project keeps finding. So the code rides alongside, not inside, the result.
    last_code: Option<&'static str>,
}

impl<'a> RelayMessageSender<'a> {
    /// The precise code from the most recent failed attempt, if any.
    pub(crate) fn last_code(&self) -> Option<&'static str> {
        self.last_code
    }

    /// The relay's advertised `max_body_bytes`, if a 413 caused us to look it up.
    pub(crate) fn last_limit(&self) -> Option<u64> {
        self.last_limit
    }

    pub(crate) fn new(relay: &'a str) -> Self {
        Self {
            relay,
            trigger: None,
            pre_envelopes: Vec::new(),
            device_id: None,
            pad_cfg: None,
            bucket_max: None,
            meta_seed: None,
            // ⚠ NA-0688 C3 — THE FLIP, AND IT CHANGES THE VALUE ASSIGNED HERE, NEVER THE
            // MEANING OF `None`. `receipt_kind: None` still means "request no receipt, put the
            // body on the wire raw" everywhere it appears — which is exactly what keeps a
            // control send from asking for a receipt of its own. Reinterpreting `None`
            // downstream instead would arm unbounded ack recursion, since the receipt paths
            // pass `receipt: None` explicitly; `an_ack_never_provokes_an_ack_in_reply` pins it.
            // ⚠ NA-0688 C3 — RESOLVED, NOT HARD-CODED, and that is the whole correction.
            //
            // Writing `Some(ReceiptKind::Delivered)` here looked like the flip and was not: the
            // `qsc send` path immediately overwrites this field via `with_meta`, so the value
            // only ever reached the wire on the paths that DON'T call `with_meta` — `outbox
            // retry` and `outbox discard`. The same queued row therefore went out differently
            // depending on which command drained it. Going through the shared resolver instead
            // gives all three production construction sites one rule, so a row queued by a
            // default `qsc send` drains identically via send, retry and discard.
            receipt_kind: crate::resolve_sender_receipt_request(None),
            last_limit: None,
            last_code: None,
        }
    }

    /// Carry the caller's metadata-privacy settings into the pack.
    pub(crate) fn with_meta(
        mut self,
        pad_cfg: Option<MetaPadConfig>,
        bucket_max: Option<usize>,
        meta_seed: Option<u64>,
        receipt: Option<ReceiptKind>,
    ) -> Self {
        self.pad_cfg = pad_cfg;
        self.bucket_max = bucket_max;
        self.meta_seed = meta_seed;
        // ⚠ VERBATIM, IN BOTH DIRECTIONS — AND DELIBERATELY NOT CONDITIONAL.
        //
        // Swept at NA-0688 C3: this comment used to say the sender half defaults OFF, which
        // stopped being true when the default flipped and was the only description of this
        // assignment in the file.
        //
        // The obvious-looking repair for the flip was to make this assignment skip a `None`
        // caller so the constructor's default survived. That would be WRONG: `receipt: None`
        // is how a caller says "no receipt — put the body on the wire RAW, no data control
        // envelope, no `msg_id`, nothing an ack can be provoked by", and the receipt paths
        // depend on exactly that to avoid asking for receipts of their own (unbounded ack
        // recursion, which `an_ack_never_provokes_an_ack_in_reply` pins). So this stays a
        // straight assignment, and the CALLER's absent-vs-explicit distinction is resolved
        // BEFORE it gets here, by `resolve_sender_receipt_request`.
        self.receipt_kind = receipt;
        self
    }

    /// Map a push failure class to the queue's vocabulary.
    ///
    /// ⚠ This is where C11/Option B pays off: the PAUSE cause is derived from the HTTP
    /// STATUS CLASS, not from the marker string -- which deliberately collapses 401 and 403
    /// into one code and therefore cannot express this distinction.
    ///
    /// ⚠ O4 governs the default: anything unrecognised is RETRYABLE. "Unknown" is not
    /// evidence of permanence, and surfacing a retryable failure as permanent is the exact
    /// thing O4 forbids.
    fn classify(class: PushFailClass) -> msgqueue::AttemptResult {
        use msgqueue::{AttemptResult, PausedCause};
        match class {
            PushFailClass::TokenRejected => AttemptResult::Pause(PausedCause::TokenRejected),
            PushFailClass::CertUntrusted => AttemptResult::Pause(PausedCause::Cert),
            // 413: terminal for THIS message (it cannot succeed against this relay), but NOT
            // permanent -- it heals against a relay with a larger limit.
            PushFailClass::TooLarge => AttemptResult::Fail,
            PushFailClass::QueueFull | PushFailClass::Transient | PushFailClass::Forbidden => {
                AttemptResult::Retry
            }
            PushFailClass::Other => AttemptResult::Retry,
        }
    }
}

impl<'a> msgqueue::MessageSender for RelayMessageSender<'a> {
    fn pack(
        &mut self,
        rec: &msgqueue::QueuedMessage,
    ) -> Result<(Vec<u8>, Vec<u8>, String), msgqueue::AttemptResult> {
        // ⚠ A REVOKED session is the ONLY permanent cause in v1 (O4/A10), and it is detected
        // LOCALLY -- the relay has no session-revoked signal on the push path. The contact
        // record's device state carries it, and `resolve_send_routing_target` already
        // refuses with `device_revoked`. That makes the one permanent state deterministic
        // and testable without a hostile relay.
        let routing = match resolve_send_routing_target(rec.peer.as_str()) {
            Ok(v) => v,
            Err("device_revoked") => return Err(msgqueue::AttemptResult::FailPermanent),
            // Anything else about routing is a local configuration problem that can heal.
            Err(_) => return Err(msgqueue::AttemptResult::Retry),
        };
        emit_cli_routing_marker(
            routing.peer_alias.as_str(),
            routing.device_id.as_str(),
            routing.implicit_primary,
        );
        emit_cli_confirm_policy();
        // ⚠ Wrap the body in the data control envelope carrying THIS RECORD'S msg_id.
        //
        // This is what makes the delivery-ack correlate to the queued row: the peer echoes
        // the id back and the sender flips exactly that record SENT -> DELIVERED. The old
        // path minted an id here that the queue knew nothing about; unifying them is what
        // lets A4 work against the store rather than against the timeline alone.
        // Wrap in the data control envelope ONLY when a receipt was explicitly requested.
        // The envelope is what carries the `msg_id` an ack echoes back, so no request means
        // no envelope, no ack, and a byte-for-byte pre-NA-0682 wire.
        let wire_body = match self.receipt_kind {
            Some(kind) => match crate::encode_data_payload_with_id(
                rec.body.clone(),
                kind,
                rec.msg_id.as_str(),
            ) {
                Ok(v) => v,
                Err(_) => return Err(msgqueue::AttemptResult::Retry),
            },
            None => rec.body.clone(),
        };
        match qsp_pack(
            routing.channel.as_str(),
            &wire_body,
            self.pad_cfg,
            self.meta_seed,
            // The msgqueue drains USER messages; receipts never enter this queue.
            SendOrigination::User,
        ) {
            Ok(v) => {
                // Same markers, same points, AND the same disk writes as the pre-NA-0682
                // path. Enumerated from the side-effect inventory rather than discovered
                // one failing test at a time.
                if let Ok((dir, source)) = config_dir() {
                    // ⚠ A PERSISTENT WRITE, not a marker. The QSP status record on disk is
                    // what anything asking "is the protocol healthy" reads; skipping it left
                    // that record stale on every send. Found by the inventory, not a test.
                    record_qsp_status(&dir, source, true, "pack_ok", true, false);
                }
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
                let len_s = rec.body.len().to_string();
                print_marker("send_prepare", &[("payload_len", len_s.as_str())]);
                if self.receipt_kind.is_some() {
                    emit_marker(
                        "receipt_request",
                        None,
                        &[("kind", "delivered"), ("msg_id", "<redacted>")],
                    );
                }
                if let Some(max_bucket) = self.bucket_max {
                    let bucket = meta_bucket_for_len(v.envelope.len(), max_bucket);
                    let bucket_s = bucket.to_string();
                    let orig_s = v.envelope.len().to_string();
                    let capped_s = v.envelope.len().min(max_bucket).to_string();
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
                self.trigger = Some(v.trigger);
                self.pre_envelopes = v.pre_envelopes.clone();
                self.device_id = Some(routing.device_id.clone());
                Ok((v.envelope, v.next_state.snapshot_bytes(), routing.channel))
            }
            Err(err) => {
                if let Ok((dir, source)) = config_dir() {
                    record_qsp_status(&dir, source, false, err.code, false, false);
                }
                if let Some(reason) = err.reason {
                    emit_marker(
                        "qsp_pack",
                        Some(err.code),
                        &[("ok", "false"), ("reason", reason)],
                    );
                } else {
                    emit_marker("qsp_pack", Some(err.code), &[("ok", "false")]);
                }
                Err(msgqueue::AttemptResult::Retry)
            }
        }
    }

    fn push(&mut self, rec: &msgqueue::QueuedMessage) -> Result<(), msgqueue::AttemptResult> {
        let Some(ciphertext) = rec.ciphertext.as_ref() else {
            // Unreachable by construction: the drain only pushes what it packed.
            return Err(msgqueue::AttemptResult::Retry);
        };
        let routing = match resolve_send_routing_target(rec.peer.as_str()) {
            Ok(v) => v,
            Err("device_revoked") => return Err(msgqueue::AttemptResult::FailPermanent),
            Err(_) => return Err(msgqueue::AttemptResult::Retry),
        };
        // ⚠ Fault injection must stay reachable on the DEFAULT send path. It lives at the
        // push boundary, and the `relay_{drop,dup,reorder}_no_mutation` guards drive the
        // adversarial surface through it -- bypassing it would quietly delete the project's
        // ability to simulate a lossy relay at all.
        if let Ok(Some(fi)) = fault_injector_from_env() {
            let idx = next_fault_index();
            let idx_s = idx.to_string();
            let seed_s = fi.seed.to_string();
            if let Some(action) = fault_action_for(&fi, idx) {
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
                        self.last_code = Some("relay_drop_injected");
                        return Err(msgqueue::AttemptResult::Retry);
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

        // NA-0624: SCKA advertisements go first, in order. Their secret material is already
        // durable and the chain advance rides in the record's next_state, so a failure here
        // is recovered by the ordinary retry.
        for pre in self.pre_envelopes.iter() {
            if let Err(f) =
                relay_inbox_push_classified(self.relay, routing.route_token.as_str(), pre)
            {
                emit_marker("relay_event", None, &[("action", "push_fail")]);
                print_marker("send_attempt", &[("ok", "false")]);
                self.last_code = Some(f.code);
                return Err(Self::classify(f.class));
            }
            emit_marker("relay_event", None, &[("action", "deliver_control")]);
        }
        match relay_inbox_push_classified(self.relay, routing.route_token.as_str(), ciphertext) {
            Ok(()) => {
                emit_marker("relay_event", None, &[("action", "deliver")]);
                emit_cli_delivery_state_with_device(
                    rec.peer.as_str(),
                    "accepted_by_relay",
                    Some(routing.device_id.as_str()),
                );
                Ok(())
            }
            Err(f) => {
                emit_marker("relay_event", None, &[("action", "push_fail")]);
                print_marker("send_attempt", &[("ok", "false")]);
                self.last_code = Some(f.code);
                // A9 / DESIGN §2: a too-large failure must NAME THE RELAY'S LIMIT, not just
                // say "too large" -- the user cannot act on the latter. Looked up only when
                // a 413 actually happens, so the ordinary path pays nothing, and treated as
                // best-effort: an unavailable server-info must not turn a clear failure into
                // a confusing one.
                if f.class == PushFailClass::TooLarge {
                    if let Ok(RelayServerInfoOutcome::Reachable { doc, .. }) =
                        relay_server_info(self.relay)
                    {
                        self.last_limit = Some(doc.max_body_bytes);
                    }
                }
                Err(Self::classify(f.class))
            }
        }
    }

    fn commit(&mut self, rec: &msgqueue::QueuedMessage) -> Result<(), &'static str> {
        // O2: only now -- the relay durably accepted the bytes (200 == fsynced, NA-0644
        // lineage). Committing the ratchet earlier would advance state for a message the
        // relay never took.
        let (Some(next_state), Some(channel)) = (rec.next_state.as_ref(), rec.channel.as_ref())
        else {
            return Err("msgqueue_inflight_incomplete");
        };
        let st = Suite2SessionState::restore_bytes(next_state)
            .map_err(|_| "outbox_state_parse_failed")?;
        let stored = match self.trigger.as_ref() {
            Some(trig) => qsp_session_store_with_trigger(channel.as_str(), &st, trig),
            None => qsp_session_store(channel.as_str(), &st),
        };
        stored.map_err(|_| "qsp_session_store_failed")?;

        // The send counter is maintained exactly as before: advanced once per accepted
        // message, on the commit path only. Keeping it means `send.state` stays meaningful
        // for every observer that already reads it, and the exactly-once property is now
        // ALSO guarded directly at the ratchet
        // (`a_successful_send_commits_the_ratchet_exactly_once`).
        let (dir, source) = config_dir().map_err(|_| "send_commit_write_failed")?;
        let next_seq = match read_send_state(&dir, source) {
            Ok(Ok(v)) => v + 1,
            _ => return Err("send_state_parse_failed"),
        };
        let state_bytes = format!("send_seq={}\n", next_seq).into_bytes();
        write_atomic(&dir.join(SEND_STATE_NAME), &state_bytes, source)
            .map_err(|_| "send_commit_write_failed")?;
        // The timeline entry is still written HERE, at commit -- so
        // `timeline_written_on_send_commit_only` keeps holding. The O1 row lives in the
        // message queue (a separate store, per F4); the timeline remains the record of what
        // was actually SENT. Two stores, two meanings, neither pretending to be the other.
        if let Err(code) = timeline_append_entry_for_target(
            rec.peer.as_str(),
            "out",
            rec.body.len(),
            "file",
            MessageState::Sent,
            // Only carry the id when an ack could actually reference it (pre-NA-0682 shape).
            self.receipt_kind.map(|_| rec.msg_id.as_str()),
            self.device_id.as_deref(),
        ) {
            emit_message_state_reject(code);
            emit_marker("error", Some(code), &[("op", "timeline_send_ingest")]);
        }
        print_marker("send_attempt", &[("ok", "true")]);
        let seq_s = next_seq.to_string();
        print_marker("send_commit", &[("send_seq", seq_s.as_str())]);
        Ok(())
    }
}

#[cfg(test)]
mod receipt_sender_default_tests {
    use super::{ReceiptKind, RelayMessageSender};

    /// ⚠ PIN THE DEFAULT, SENDER HALF — MIGRATED at NA-0688 C3 (R1b), not rewritten down.
    ///
    /// This pin was `sender_requests_no_receipt_by_default` and asserted `is_none()`. It was
    /// DESIGNED to go red when the flip landed, and it did. It is still pinned separately from
    /// the recipient half ON PURPOSE: F6 has two independent switches, and flipping only one
    /// would leave the wire noisy while the feature looked disabled.
    ///
    /// ⚠ WHAT MOVED IS THE VALUE ASSIGNED AT CONSTRUCTION, NOT THE MEANING OF `None`.
    /// `receipt_kind: None` still means "request no receipt; body goes out RAW — no data
    /// control envelope, no `msg_id` on the wire, nothing an ack can be provoked by", and the
    /// receipt paths rely on exactly that when they pass `receipt: None` to avoid asking for a
    /// receipt of their own. `an_ack_never_provokes_an_ack_in_reply` pins that consequence.
    #[test]
    fn sender_requests_a_delivered_receipt_by_default() {
        let s = RelayMessageSender::new("https://relay.invalid");
        assert_eq!(
            s.receipt_kind,
            Some(ReceiptKind::Delivered),
            "the sender must request a DELIVERED receipt by default as of NA-0688 C3"
        );
    }

    /// `with_meta` takes the caller's choice VERBATIM — in both directions.
    ///
    /// ⚠ MIGRATED at NA-0688 C3, and this pin is named in no prior record: ENG-0086 and R1b
    /// both say "the two default pins", but there are THREE assertions that a
    /// default-constructed sender requests no receipt, and this was the third. The census
    /// found it; it went red with the other two.
    ///
    /// The property is unchanged — `with_meta` must not OVERRIDE the caller's choice — but
    /// with the default now ON, proving "does not ENABLE" requires a sender that has been
    /// EXPLICITLY DISABLED first. Passing a default-constructed sender would assert nothing,
    /// because it now arrives with receipts already on.
    #[test]
    fn with_meta_takes_the_callers_receipt_choice_verbatim() {
        let mut disabled = RelayMessageSender::new("https://relay.invalid");
        disabled.receipt_kind = None;
        let s = disabled.with_meta(None, None, None, None);
        assert!(
            s.receipt_kind.is_none(),
            "with_meta must not re-enable receipts on a sender the caller disabled"
        );
        let on = RelayMessageSender::new("https://relay.invalid").with_meta(
            None,
            None,
            None,
            Some(ReceiptKind::Delivered),
        );
        assert!(
            on.receipt_kind.is_some(),
            "and an EXPLICIT request must still be honoured -- the mechanism ships, only the \
             default waits"
        );
    }
}

#[cfg(test)]
mod relay_push_diagnostic_tests {
    use super::*;

    #[test]
    fn relay_push_status_and_error_mapping_is_bounded() {
        assert_eq!(relay_push_status_class(Some(HttpStatus::OK)), "2xx");
        assert_eq!(relay_push_status_class(Some(HttpStatus::FOUND)), "3xx");
        assert_eq!(
            relay_push_status_class(Some(HttpStatus::UNAUTHORIZED)),
            "4xx"
        );
        assert_eq!(
            relay_push_status_class(Some(HttpStatus::INTERNAL_SERVER_ERROR)),
            "5xx"
        );
        assert_eq!(relay_push_status_class(None), "unknown");

        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::UNAUTHORIZED),
            "auth_rejected"
        );
        // NA-0686 (ENG-0082): 403 no longer collapses into the 401 class.
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::FORBIDDEN),
            "access_forbidden"
        );
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::BAD_REQUEST),
            "route_rejected"
        );
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::NOT_FOUND),
            "endpoint_not_found"
        );
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::PAYLOAD_TOO_LARGE),
            "payload_rejected"
        );
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::TOO_MANY_REQUESTS),
            "route_rejected"
        );
        assert_eq!(
            relay_push_error_class_for_status(HttpStatus::INTERNAL_SERVER_ERROR),
            "unexpected_status"
        );

        assert_eq!(
            relay_push_diagnostic_class_for_status(HttpStatus::UNAUTHORIZED),
            "bearer_auth_failed"
        );
        // NA-0686 (ENG-0082): the THIRD collapse site. A 403 is not a bearer
        // failure -- the bearer was accepted and the request refused anyway.
        assert_eq!(
            relay_push_diagnostic_class_for_status(HttpStatus::FORBIDDEN),
            "access_refused"
        );
        assert_eq!(
            relay_push_diagnostic_class_for_status(HttpStatus::BAD_REQUEST),
            "route_token_auth_failed"
        );
        assert_eq!(
            relay_push_diagnostic_class_for_status(HttpStatus::PAYLOAD_TOO_LARGE),
            "http_status_received"
        );
    }

    /// NA-0686 / D-1325 (ENG-0082) — THE GUARD FOR THE SPLIT ITSELF.
    ///
    /// ⚠ Asserting the two new strings would not be enough, because the defect
    /// ENG-0082 names is not "403 has the wrong word" — it is **"403 and 401 are
    /// the SAME word"**. A test that pins each value independently would still
    /// pass if some later edit collapsed them back onto a shared constant. So
    /// the property asserted here is DISTINCTNESS, stated directly.
    ///
    /// The second clause is the one NA_0663 cares about and is why it is here
    /// rather than left implicit: an auth rejection of either kind must never be
    /// reported as a TLS TRUST failure. NA_0663 asserts that for the 401; the
    /// 403 arm is new, so it gets the same protection at birth rather than after
    /// an incident.
    #[test]
    fn forbidden_is_distinct_from_unauthorized_and_from_every_trust_code() {
        let unauthorized = relay_push_qsc_error_for_status(HttpStatus::UNAUTHORIZED);
        let forbidden = relay_push_qsc_error_for_status(HttpStatus::FORBIDDEN);

        // (a) 401 is byte-identical to what it has always been. NA_0663 depends
        //     on this string and is deliberately NOT touched by the split.
        assert_eq!(
            unauthorized, "relay_unauthorized",
            "the 401 code must not move: NA_0663_relay_tls_trust asserts it verbatim"
        );
        assert_eq!(forbidden, "relay_forbidden");

        // (b) THE PROPERTY: the two causes are distinct words.
        assert_ne!(
            unauthorized, forbidden,
            "a 401 token rejection and a 403 ticketless push are different causes \
             and must not collapse to one marker code (ENG-0082)"
        );

        // (c) neither is a trust failure — the NA_0663 invariant, extended to 403.
        assert_ne!(forbidden, RELAY_TLS_UNTRUSTED);
        assert_ne!(unauthorized, RELAY_TLS_UNTRUSTED);

        // (d) the same distinctness at BOTH remaining layers. ENG-0082 cannot be
        //     closed while any one of the three still collapses -- a split that
        //     stops at two layers leaves the ledger's closure claim false, which
        //     is why this asserts all three rather than the one it started with.
        assert_ne!(
            relay_push_error_class_for_status(HttpStatus::UNAUTHORIZED),
            relay_push_error_class_for_status(HttpStatus::FORBIDDEN),
            "the error class must carry the distinction too, or the split is half-made"
        );
        assert_ne!(
            relay_push_diagnostic_class_for_status(HttpStatus::UNAUTHORIZED),
            relay_push_diagnostic_class_for_status(HttpStatus::FORBIDDEN),
            "the diagnostic class must carry the distinction too (the third site)"
        );

        // (e) the 403 diagnostic class must not CLAIM a credential failed, which
        //     is the specific falsehood the collapse was asserting.
        assert!(
            !relay_push_diagnostic_class_for_status(HttpStatus::FORBIDDEN)
                .contains("auth_failed"),
            "a 403 accepted the bearer token; reporting an auth failure sends an \
             operator to re-check a credential that was never the problem"
        );
    }

    #[test]
    fn relay_push_body_presence_is_length_only() {
        assert_eq!(relay_push_body_presence(Some(0)), "false");
        assert_eq!(relay_push_body_presence(Some(17)), "true");
        assert_eq!(relay_push_body_presence(None), "unknown");
    }

    #[test]
    fn relay_push_timeout_phase_mapping_is_bounded() {
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(true, false, "dns lookup timed out"),
            "dns_timeout"
        );
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(true, true, "connect timed out"),
            "tcp_connect_timeout"
        );
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(true, false, "tls handshake timed out"),
            "tls_handshake_timeout"
        );
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(true, false, "request timed out"),
            "http_request_timeout"
        );
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(true, false, "elapsed timeout"),
            "unknown_timeout"
        );
        assert_eq!(
            relay_push_timeout_phase_class_from_parts(false, true, "connect failed"),
            "not_timeout"
        );
    }

    #[test]
    fn relay_push_diagnostic_error_mapping_is_bounded() {
        assert_eq!(
            relay_push_diagnostic_class_from_error_parts(false, true, "connection refused"),
            "connection_refused"
        );
        assert_eq!(
            relay_push_diagnostic_class_from_error_parts(false, true, "connection reset by peer"),
            "connection_reset"
        );
        assert_eq!(
            relay_push_diagnostic_class_from_error_parts(true, true, "connect timed out"),
            "tcp_connect_timeout"
        );
        assert_eq!(
            relay_push_diagnostic_class_from_error_parts(false, false, "other error"),
            "not_timeout"
        );
    }
}

// NA-0689 D-1328 RULING 11.2 — THE CAPTURE DECISION, PINNED EXHAUSTIVELY AT THE DECISION LAYER.
//
// ⚠ WHY THESE ARE UNIT TESTS AND NOT END-TO-END ARMS, STATED SO NOBODY "FIXES" IT LATER.
// The two non-success outcomes CANNOT be produced by a stock `qsc` peer over the wire:
// `IgnoredWrongDevice` needs a confirm arriving on a DEVICE-QUALIFIED session for a device that is
// not the item's target -- i.e. the peer's second device confirming an item it never received --
// and `Err` needs the peer to name an item the receiver holds no record of. Both are HOSTILE-PEER
// behaviours, which is exactly what those captures exist to witness. An integration arm therefore
// cannot reach them without a test seam inside the receive path, which was refused: if such a seam
// is ever built it belongs on the SENDER side, so a crafted hostile frame feeds an UNMODIFIED
// receive path. Filed as its own ENG against the negative-control audit track.
//
// So the decision is pinned HERE, where every arm IS reachable and each is trivially red-capable:
// delete any one line of `confirm_capture_reason` and exactly one of these goes red.
//
// ⚠ THE TABLE IS EXHAUSTIVE OVER `ConfirmApplyOutcome`, which has exactly two variants, plus the
// `Err` case -- three rows, closed. A new variant makes the helper's `match` fail to compile, so
// this table cannot silently fall behind the enum.
#[cfg(test)]
mod confirm_capture_reason_tests {
    use super::confirm_capture_reason;
    use crate::timeline::ConfirmApplyOutcome;

    /// SUCCESS CAPTURES NOTHING. This is the whole point of the shared-ack split: D2/D3/D4 ack the
    /// same way whether or not the confirm applied, so a blanket capture would store every
    /// successfully applied confirm and turn the quarantine into a copy of ordinary traffic.
    #[test]
    fn an_applied_confirm_is_never_captured() {
        let applied = Ok((ConfirmApplyOutcome::Confirmed, Some("device-1".to_string())));
        assert_eq!(
            confirm_capture_reason(&applied),
            None,
            "a confirm that APPLIED must not be quarantined"
        );
        // The target device is carried for the emit arms, never for the decision.
        let applied_no_target = Ok((ConfirmApplyOutcome::Confirmed, None));
        assert_eq!(
            confirm_capture_reason(&applied_no_target),
            None,
            "the decision must not depend on whether a target device was resolved"
        );
    }

    /// POSITIVE 1 — the wrong-device ignore. ⚠ This is the arm that was MISSING at D4 while D2 and
    /// D3 had it (Ruling 9); nothing was applied and the message key was already consumed, so the
    /// item is unrecoverable at the moment it is acked.
    #[test]
    fn a_wrong_device_confirm_is_captured_with_its_own_reason() {
        let ignored = Ok((
            ConfirmApplyOutcome::IgnoredWrongDevice,
            Some("device-2".to_string()),
        ));
        assert_eq!(
            confirm_capture_reason(&ignored),
            Some("ignored_wrong_device"),
            "a confirm from the wrong device must be captured, not destroyed"
        );
    }

    /// POSITIVE 2 — the apply reject. The reason is the callee's own, passed through UNCHANGED, so
    /// the quarantine record says why rather than flattening every failure to one word.
    #[test]
    fn a_rejected_confirm_is_captured_under_the_callees_reason() {
        let rejected: Result<(ConfirmApplyOutcome, Option<String>), &'static str> =
            Err("timeline_entry_not_found");
        assert_eq!(
            confirm_capture_reason(&rejected),
            Some("timeline_entry_not_found"),
            "the reject reason must reach the quarantine record unflattened"
        );
        let other: Result<(ConfirmApplyOutcome, Option<String>), &'static str> =
            Err("invalid_state_transition");
        assert_eq!(
            confirm_capture_reason(&other),
            Some("invalid_state_transition"),
            "a DIFFERENT reject must carry a DIFFERENT reason -- otherwise this pin would pass \
             against a helper that returned one hardcoded string for every failure"
        );
    }

    /// ⚠ THE SEPARATION ITSELF. Success and non-success must not merely differ in the reason they
    /// carry -- one captures and one does not. Asserted as the partition, because that is the
    /// property the three sites share and the one Ruling 9 found broken at a single site.
    #[test]
    fn success_and_non_success_land_on_opposite_sides_of_the_capture_boundary() {
        let applied = Ok((ConfirmApplyOutcome::Confirmed, None));
        let ignored = Ok((ConfirmApplyOutcome::IgnoredWrongDevice, None));
        let rejected: Result<(ConfirmApplyOutcome, Option<String>), &'static str> = Err("nope");
        assert!(confirm_capture_reason(&applied).is_none());
        assert!(confirm_capture_reason(&ignored).is_some());
        assert!(confirm_capture_reason(&rejected).is_some());
    }
}

// NA-0689 D-1328 RULING 12 — D5's CAPTURE DECISION, PINNED EXHAUSTIVELY AT THE DECISION LAYER.
//
// ⚠ WHY A UNIT TABLE AND NOT AN END-TO-END ARM -- THE SAME REASON AS D2/D3/D4, AND IT IS THE
// SITE'S PURPOSE RATHER THAN A GAP. `UnknownControl` needs our namespace marker PLUS either an
// unknown `t`/`kind` pair or a version above CTRL_VERSION_MAX. A sender of THIS build emits
// neither: same binary, same version ceiling, same three known shapes. There is no env override,
// and all three ReceiptControlPayload builders are crate-private, so no integration test can craft
// one either. D5's capture is the FORWARD-COMPAT WITNESS -- only a FUTURE build can trigger it.
//
// ⚠ THE HONEST SHAPE OF THE WHOLE CAPTURE SURFACE, recorded above the test names because it is the
// thing a later reader most needs and least expects: FOUR OF THE FIVE SITES' POSITIVES ARE
// UNREACHABLE FROM A STOCK PEER, EACH FOR A REASON THAT IS THE SITE'S PURPOSE -- hostile-peer
// witnesses at D2-D4, the forward-compat witness at D5. D1 alone is stock-reachable, because D1's
// trigger is OUR OWN CRASH rather than the peer's behaviour.
//
// What a payload IS is `classify_control`'s call and is exhaustively pinned by NA-0682's own tests
// (all four classes, both UnknownControl routes, and the silent-loss guard). What the SITE DOES
// about it is pinned here.
#[cfg(test)]
mod control_class_capture_tests {
    use super::control_class_capture_reason;
    use crate::adversarial::payload::ControlClass;

    /// THE ONE CLASS THAT IS CAPTURED. Judged not-for-this-build, NOT unrecoverable: a future build
    /// could read it, but redelivery cannot save it because every current build acks it away on
    /// sight -- so the store is the only thing that preserves it.
    #[test]
    fn an_unknown_control_is_captured_under_its_own_reason() {
        assert_eq!(
            control_class_capture_reason(ControlClass::UnknownControl),
            Some("unknown_control_type"),
            "the forward-compat seam must be KEPT, not acked away with a marker as the only witness"
        );
    }

    /// ⚠ THE ZEROS, EXHAUSTIVE OVER EVERY OTHER VARIANT. Each of these has its own handling path and
    /// none is a discard; capturing any of them would store ordinary traffic. `NotControl` matters
    /// most: it is the silent-loss guard's class -- a user message that merely LOOKS like a control
    /// payload -- and capturing it would swallow real user mail into the quarantine.
    #[test]
    fn every_class_this_build_understands_captures_nothing() {
        for class in [
            ControlClass::DeliveredAck,
            ControlClass::DataEnvelope,
            ControlClass::NotControl,
        ] {
            assert_eq!(
                control_class_capture_reason(class),
                None,
                "{class:?} is handled on its own path and must never be quarantined"
            );
        }
    }

    /// ⚠ THE PARTITION ITSELF, asserted as a partition rather than as four independent facts:
    /// exactly one class captures. A helper that captured two, or none, would satisfy neither.
    #[test]
    fn exactly_one_control_class_reaches_the_capture() {
        let all = [
            ControlClass::DeliveredAck,
            ControlClass::DataEnvelope,
            ControlClass::UnknownControl,
            ControlClass::NotControl,
        ];
        let captured = all
            .iter()
            .filter(|c| control_class_capture_reason(**c).is_some())
            .count();
        assert_eq!(
            captured, 1,
            "exactly one of the four classes may reach the D5 capture"
        );
    }
}
