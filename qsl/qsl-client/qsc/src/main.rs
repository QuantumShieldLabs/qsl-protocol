use clap::Parser;
use std::path::Path;
use std::process;

use qsc::attachments::{file_send_execute, FileSendExec};
use qsc::cmd::{
    Cli, Cmd, ConfigCmd, ContactsCmd, ContactsDeviceCmd, ContactsDevicePrimaryCmd,
    ContactsRequestCmd, ContactsTrustModeCmd, EnvelopeCmd, FileCmd, HandshakeCmd, IdentityCmd,
    MetaCmd, PeersCmd, RelayCmd, SendCmd, TimelineCmd, UtilCmd,
};
use qsc::contacts::{
    contacts_add, contacts_block, contacts_device_add, contacts_device_list,
    contacts_device_primary_set, contacts_device_primary_show, contacts_device_revoke,
    contacts_device_status, contacts_device_trust, contacts_device_verify, contacts_list,
    contacts_request_accept, contacts_request_block, contacts_request_ignore,
    contacts_request_list, contacts_route_set, contacts_show, contacts_trust_mode_set,
    contacts_trust_mode_show, contacts_unblock, contacts_verify, normalize_route_token,
    route_token_hash8,
};
use qsc::fs_store::set_umask_077;
use qsc::handshake::{
    handshake_init_with_suite_mode, handshake_poll_with_suite_mode, handshake_status,
};
use qsc::output::{
    CliError, CliResult,
    emit_marker, init_output_policy, install_panic_redaction_hook, print_marker, qsc_mark, qsc_sanitize_terminal_text, redact_text_for_output,
    PANIC_DEMO_SENTINEL,
};
use qsc::protocol_state::{allow_unsafe_seed_fallback_for_tests, qsp_status_tuple};
use qsc::relay::{RelayConfig, SendExecuteArgs};
use qsc::store::{
    TUI_RELAY_INBOX_TOKEN_SECRET_KEY, TUI_RELAY_TOKEN_FILE_SECRET_KEY,
    TUI_RELAY_TOKEN_SECRET_KEY,
};
use qsc::timeline::{timeline_clear, timeline_list, timeline_show};
use qsc::*;

fn bootstrap_unlock(passphrase_file: Option<&Path>, passphrase_env: Option<&str>) {
    if let Some(path) = passphrase_file {
        match vault::unlock_with_passphrase_file(path) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => exit_on(CliError::code(code)),
        }
    } else if let Some(env_name) = passphrase_env {
        if env_name.trim().is_empty() {
            exit_on(CliError::code("vault_locked"));
        }
        match vault::unlock_with_passphrase_env(Some(env_name)) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => exit_on(CliError::code(code)),
        }
    } else if allow_unsafe_seed_fallback_for_tests() {
        // Explicit unsafe fixture mode keeps deterministic test workflows isolated.
        set_vault_unlocked(true);
    }
}

fn main() {
    set_umask_077();
    install_panic_redaction_hook();
    let cli = Cli::parse();
    init_output_policy(cli.reveal);
    set_vault_unlocked(false);
    bootstrap_unlock(
        cli.unlock_passphrase_file.as_deref(),
        cli.unlock_passphrase_env.as_deref(),
    );
    if let Err(err) = run(cli) {
        exit_on(err);
    }
}

// NA-0646 (D582) PR-B: the ONE Err->emit+exit adapter. The library returns
// CliError; exit semantics live here and nowhere else (util_sanitize's usage
// exit(2) and clap's own exit(2) are the bin-local exceptions).
fn exit_on(err: CliError) -> ! {
    match err {
        CliError::Code(code) => {
            emit_marker("error", Some(&code), &[]);
            process::exit(1);
        }
        CliError::Emitted => process::exit(1),
    }
}

fn run(cli: Cli) -> CliResult {
    match cli.cmd {
        None => {
            // Shell-first UX expects help by default.
            println!("QSC_MARK/1 event=help_stub");
        }
        Some(Cmd::Status) => {
            let locked = if vault_unlocked() { "false" } else { "true" };
            print_marker("status", &[("ok", "true"), ("locked", locked)]);
            let status_peer = "peer-0";
            let (status, reason) = qsp_status_tuple(status_peer);
            emit_marker(
                "qsp_status",
                None,
                &[("status", status.as_str()), ("reason", reason.as_str())],
            );
            let (peer_fp, pinned) = identity_peer_status(status_peer);
            let pinned_s = if pinned { "true" } else { "false" };
            emit_marker(
                "identity_status",
                None,
                &[
                    ("peer", status_peer),
                    ("peer_fp", peer_fp.as_str()),
                    ("pinned", pinned_s),
                ],
            );
            let policy = load_receipt_policy_from_account();
            let batch_window_s = policy.batch_window_ms.to_string();
            let jitter_s = policy.jitter_ms.to_string();
            emit_marker(
                "receipt_policy",
                None,
                &[
                    ("mode", policy.mode.as_str()),
                    ("batch_window_ms", batch_window_s.as_str()),
                    ("jitter_ms", jitter_s.as_str()),
                    ("file_confirm_mode", policy.file_confirm_mode.as_str()),
                ],
            );
        }
        Some(Cmd::Config { cmd }) => match cmd {
            ConfigCmd::Set { key, value } => config_set(&key, &value),
            ConfigCmd::Get { key } => config_get(&key),
        }?,
        Some(Cmd::Doctor {
            check_only,
            timeout_ms,
            export,
        }) => doctor_check_only(check_only, timeout_ms, export)?,
        Some(Cmd::Util { cmd }) => match cmd {
            UtilCmd::Sanitize { print } => util_sanitize(print),
            UtilCmd::Queue { len } => util_queue(len)?,
            UtilCmd::History { len } => util_history(len)?,
            UtilCmd::Retry { fail } => util_retry(fail)?,
            UtilCmd::Timeout {
                wait_ms,
                timeout_ms,
            } => util_timeout(wait_ms, timeout_ms)?,
            UtilCmd::Envelope {
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            } => util_envelope(
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                payload_lens,
            )?,
            UtilCmd::PanicDemo => util_panic_demo(),
            UtilCmd::ReceiptApply {
                peer,
                channel,
                msg_id,
                file_id,
                confirm_id,
            } => util_receipt_apply(peer.as_str(), channel.as_str(), msg_id, file_id, confirm_id)?,
        },
        Some(Cmd::Envelope { cmd }) => match cmd {
            EnvelopeCmd::PlanAck {
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            } => envelope_plan_ack(
                deterministic,
                tick_count,
                interval_ms,
                max_ticks,
                max_bundle,
                max_count,
                small_len,
            ),
        }?,
        Some(Cmd::Vault { cmd }) => vault::cmd_vault(cmd)?,
        Some(Cmd::Send {
            cmd,
            transport,
            relay,
            to,
            file,
            pad_to,
            pad_bucket,
            bucket_max,
            meta_seed,
            receipt,
        }) => match cmd {
            Some(SendCmd::Abort) => transport::send_abort(),
            None => transport::send_execute(SendExecuteArgs {
                transport,
                relay,
                to,
                file,
                pad_to,
                pad_bucket,
                bucket_max,
                meta_seed,
                receipt,
            }),
        }?,
        Some(Cmd::Receive {
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
            file,
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
        }) => {
            if let Some(path) = file {
                if transport.is_some()
                    || relay.is_some()
                    || legacy_receive_mode.is_some()
                    || ack_mode.is_some()
                    || attachment_service.is_some()
                    || from.is_some()
                    || mailbox.is_some()
                    || max.is_some()
                    || max_file_size.is_some()
                    || max_file_chunks.is_some()
                    || out.is_some()
                    || deterministic_meta
                    || interval_ms.is_some()
                    || poll_interval_ms.is_some()
                    || poll_ticks.is_some()
                    || batch_max_count.is_some()
                    || poll_max_per_tick.is_some()
                    || bucket_max.is_some()
                    || meta_seed.is_some()
                    || emit_receipts.is_some()
                    || receipt_mode.is_some()
                    || receipt_batch_window_ms.is_some()
                    || receipt_jitter_ms.is_some()
                    || file_confirm_mode.is_some()
                {
                    return Err(CliError::code("recv_file_conflict"));
                }
                receive_file(&path)?;
            } else {
                let args = ReceiveArgs {
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
                };
                transport::receive_execute(args)?;
            }
        }
        Some(Cmd::Handshake { cmd }) => match cmd {
            HandshakeCmd::Init {
                as_label,
                peer,
                relay,
                suite_mode,
            } => handshake_init_with_suite_mode(&as_label, &peer, &relay, suite_mode),
            HandshakeCmd::Poll {
                as_label,
                peer,
                relay,
                max,
                suite_mode,
            } => handshake_poll_with_suite_mode(&as_label, &peer, &relay, max, suite_mode),
            HandshakeCmd::Status { peer } => handshake_status(peer.as_deref()),
        }?,
        Some(Cmd::Identity { cmd }) => match cmd {
            IdentityCmd::Show { as_label } => identity_show(&as_label),
            IdentityCmd::Rotate {
                as_label,
                confirm,
                reset_peers,
            } => identity_rotate(&as_label, confirm, reset_peers),
        }?,
        Some(Cmd::Peers { cmd }) => match cmd {
            PeersCmd::List => peers_list(),
        }?,
        Some(Cmd::Contacts { cmd }) => match cmd {
            ContactsCmd::Add {
                label,
                fp,
                kem_pk,
                sig_pk,
                route_token,
                verify,
            } => contacts_add(
                &label,
                &fp,
                kem_pk.as_deref(),
                sig_pk.as_deref(),
                route_token.as_deref(),
                verify,
            )?,
            ContactsCmd::Show { label } => contacts_show(&label)?,
            ContactsCmd::List => contacts_list()?,
            ContactsCmd::Verify { label, fp, confirm } => contacts_verify(&label, &fp, confirm)?,
            ContactsCmd::Block { label } => contacts_block(&label)?,
            ContactsCmd::Unblock { label } => contacts_unblock(&label)?,
            ContactsCmd::RouteSet { label, route_token } => {
                contacts_route_set(&label, &route_token)?
            }
            ContactsCmd::Device { cmd } => match cmd {
                ContactsDeviceCmd::Add {
                    label,
                    fp,
                    route_token,
                } => contacts_device_add(&label, &fp, route_token.as_deref())?,
                ContactsDeviceCmd::List { label } => contacts_device_list(&label)?,
                ContactsDeviceCmd::Status { label, device } => {
                    contacts_device_status(&label, device.as_deref())?
                }
                ContactsDeviceCmd::Verify { label, device, fp } => {
                    contacts_device_verify(&label, &device, &fp)?
                }
                ContactsDeviceCmd::Trust {
                    label,
                    device,
                    confirm,
                } => contacts_device_trust(&label, &device, confirm)?,
                ContactsDeviceCmd::Revoke {
                    label,
                    device,
                    confirm,
                } => contacts_device_revoke(&label, &device, confirm)?,
                ContactsDeviceCmd::Primary { cmd } => match cmd {
                    ContactsDevicePrimaryCmd::Set {
                        label,
                        device,
                        confirm,
                    } => contacts_device_primary_set(&label, &device, confirm)?,
                    ContactsDevicePrimaryCmd::Show { label } => {
                        contacts_device_primary_show(&label)?
                    }
                },
            },
            ContactsCmd::TrustMode { cmd } => match cmd {
                ContactsTrustModeCmd::Show => contacts_trust_mode_show()?,
                ContactsTrustModeCmd::Set { mode } => contacts_trust_mode_set(mode)?,
            },
            ContactsCmd::Request { cmd } => match cmd {
                ContactsRequestCmd::List => contacts_request_list()?,
                ContactsRequestCmd::Accept { label } => contacts_request_accept(&label)?,
                ContactsRequestCmd::Ignore { label } => contacts_request_ignore(&label)?,
                ContactsRequestCmd::Block { label } => contacts_request_block(&label)?,
            },
        },
        Some(Cmd::Timeline { cmd }) => match cmd {
            TimelineCmd::List { peer, limit } => timeline_list(&peer, limit)?,
            TimelineCmd::Show { peer, id } => timeline_show(&peer, &id)?,
            TimelineCmd::Clear { peer, confirm } => timeline_clear(&peer, confirm)?,
        },
        Some(Cmd::File { cmd }) => match cmd {
            FileCmd::Send {
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
            } => file_send_execute(FileSendExec {
                transport,
                relay: relay.as_deref(),
                attachment_service: attachment_service.as_deref(),
                legacy_in_message_stage,
                to: to.as_str(),
                path: path.as_path(),
                chunk_size,
                max_file_size,
                max_chunks,
                receipt,
            })?,
        },
        Some(Cmd::Relay { cmd }) => relay_cmd(cmd)?,
        Some(Cmd::Meta { cmd }) => meta_cmd(cmd)?,
    }
    Ok(())
}

fn relay_cmd(cmd: RelayCmd) -> CliResult {
    match cmd {
        RelayCmd::Serve {
            port,
            seed,
            drop_pct,
            dup_pct,
            reorder_window,
            fixed_latency_ms,
            jitter_ms,
            max_messages,
        } => {
            if drop_pct > 100 || dup_pct > 100 {
                return Err(CliError::code("relay_pct_invalid"));
            }
            let cfg = RelayConfig {
                seed,
                drop_pct,
                dup_pct,
                reorder_window,
                fixed_latency_ms,
                jitter_ms,
            };
            transport::relay_serve(port, cfg, max_messages)?;
        }
        RelayCmd::Send {
            to,
            file,
            relay,
            bucket_max,
        } => {
            require_unlocked("relay_send")?;
            transport::relay_send(&to, &file, &relay, None, bucket_max, None, None)?;
        }
        RelayCmd::InboxSet { token } => {
            require_unlocked("relay_inbox_set")?;
            let token = normalize_route_token(token.as_str())
                .map_err(|code| CliError::code(code))?;
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str()).is_err() {
                return Err(CliError::code("relay_inbox_token_store_failed"));
            }
            let hash = route_token_hash8(token.as_str());
            emit_marker(
                "relay_inbox_set",
                None,
                &[
                    ("ok", "true"),
                    ("token", "redacted"),
                    ("token_hash", hash.as_str()),
                ],
            );
            println!("relay_inbox_token=set hash={}", hash);
        }
        RelayCmd::InboxClear => {
            require_unlocked("relay_inbox_clear")?;
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "").is_err() {
                return Err(CliError::code("relay_inbox_token_store_failed"));
            }
            emit_marker(
                "relay_inbox_clear",
                None,
                &[("ok", "true"), ("token", "cleared")],
            );
            println!("relay_inbox_token=cleared");
        }
        RelayCmd::TokenSet { token } => {
            require_unlocked("relay_token_set")?;
            let token = token.trim();
            if token.is_empty() {
                return Err(CliError::code("relay_token_missing"));
            }
            if vault::secret_set(TUI_RELAY_TOKEN_SECRET_KEY, token).is_err() {
                return Err(CliError::code("relay_token_store_failed"));
            }
            emit_marker(
                "relay_token_set",
                None,
                &[("ok", "true"), ("token", "redacted")],
            );
            println!("relay_token=set");
        }
        RelayCmd::TokenFileSet { path } => {
            require_unlocked("relay_token_file_set")?;
            if path.as_os_str().is_empty() {
                return Err(CliError::code("relay_token_file_missing"));
            }
            let canonical = path
                .canonicalize()
                .unwrap_or(path)
                .to_string_lossy()
                .to_string();
            if vault::secret_set(TUI_RELAY_TOKEN_FILE_SECRET_KEY, canonical.as_str()).is_err() {
                return Err(CliError::code("relay_token_file_store_failed"));
            }
            emit_marker(
                "relay_token_file_set",
                None,
                &[("ok", "true"), ("path", "redacted")],
            );
            println!("relay_token_file=set");
        }
    }
    Ok(())
}

fn meta_cmd(cmd: MetaCmd) -> CliResult {
    match cmd {
        MetaCmd::Plan {
            deterministic,
            tick_count,
            interval_ms,
            bucket_max,
            batch_max_count,
            cover_enabled,
        } => {
            let cfg = match meta_poll_config_from_args(MetaPollArgs {
                deterministic_meta: deterministic,
                interval_ms: Some(interval_ms),
                poll_interval_ms: None,
                ticks: Some(tick_count),
                batch_max_count: Some(batch_max_count),
                poll_max_per_tick: None,
                bucket_max: Some(bucket_max),
                meta_seed: None,
            }) {
                Ok(Some(v)) => v,
                Ok(None) => return Err(CliError::code("meta_poll_invalid")),
                Err(code) => return Err(CliError::code(code)),
            };
            let deterministic_s = if cfg.deterministic { "true" } else { "false" };
            let ticks_s = cfg.ticks.to_string();
            let interval_s = cfg.interval_ms.to_string();
            let bucket_s = cfg.bucket_max.to_string();
            let batch_s = cfg.batch_max_count.to_string();
            emit_marker(
                "meta_plan",
                None,
                &[
                    ("deterministic", deterministic_s),
                    ("ticks", ticks_s.as_str()),
                    ("interval_ms", interval_s.as_str()),
                    ("bucket_max", bucket_s.as_str()),
                    ("batch_max_count", batch_s.as_str()),
                ],
            );
            for tick in 0..cfg.ticks {
                let tick_s = tick.to_string();
                let bucket = meta_bucket_for_len(1, cfg.bucket_max);
                let bucket_out_s = bucket.to_string();
                let planned_count_s = cfg.batch_max_count.to_string();
                emit_marker(
                    "meta_tick",
                    None,
                    &[
                        ("tick", tick_s.as_str()),
                        ("interval_ms", interval_s.as_str()),
                        ("deterministic", deterministic_s),
                    ],
                );
                emit_marker(
                    "meta_bucket",
                    None,
                    &[
                        ("bucket", bucket_out_s.as_str()),
                        ("orig", "1"),
                        ("capped", "1"),
                        ("metric", "planned_envelope_len"),
                    ],
                );
                emit_marker(
                    "meta_batch",
                    None,
                    &[
                        ("count", planned_count_s.as_str()),
                        ("bytes", "0"),
                        ("planned", "true"),
                    ],
                );
                if cover_enabled {
                    emit_marker(
                        "meta_cover",
                        None,
                        &[("enabled", "true"), ("tick", tick_s.as_str())],
                    );
                }
            }
        }
    }
    Ok(())
}

fn util_sanitize(print: Option<Vec<String>>) {
    let Some(parts) = print else {
        qsc_mark("util_sanitize", "usage");
        eprintln!("usage: qsc util sanitize --print <text>");
        process::exit(2);
    };
    let raw = parts.join(" ");
    let sanitized = qsc_sanitize_terminal_text(&raw);
    println!("{}", redact_text_for_output(&sanitized));
    qsc_mark("util_sanitize", "ok");
}

fn util_panic_demo() {
    panic!("panic_demo {}", PANIC_DEMO_SENTINEL);
}

fn util_queue(len: usize) -> CliResult {
    let mut q = BoundedQueue::new(MAX_QUEUE_LEN);
    for i in 0..len {
        if q.push(i).is_err() {
            return Err(CliError::code("queue_limit_exceeded"));
        }
    }
    print_marker("queue_limit", &[("ok", "true")]);
    Ok(())
}

fn util_history(len: usize) -> CliResult {
    let mut h = BoundedQueue::new(MAX_HISTORY_LEN);
    for i in 0..len {
        if h.push(i).is_err() {
            return Err(CliError::code("history_limit_exceeded"));
        }
    }
    print_marker("history_limit", &[("ok", "true")]);
    Ok(())
}

fn util_retry(fail: u32) -> CliResult {
    let mut remaining = fail;
    let res = bounded_retry(MAX_RETRY_ATTEMPTS, || {
        if remaining > 0 {
            remaining -= 1;
            Err(())
        } else {
            Ok(())
        }
    });
    match res {
        Ok(attempts) => {
            let attempts_s = attempts.to_string();
            print_marker("retry_bound", &[("attempts", attempts_s.as_str())]);
        }
        Err(()) => return Err(CliError::code("retry_limit_exceeded")),
    }
    Ok(())
}

fn util_timeout(wait_ms: u64, timeout_ms: u64) -> CliResult {
    let limit = timeout_ms.clamp(1, MAX_TIMEOUT_MS);
    if wait_ms > limit {
        return Err(CliError::code("timeout_exceeded"));
    }
    let elapsed_s = wait_ms.to_string();
    print_marker("timeout_ok", &[("elapsed_ms", elapsed_s.as_str())]);
    Ok(())
}
