use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use clap::Parser;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_sig_keypair, runtime_pq_sig_public_key_bytes, runtime_pq_sig_signature_bytes,
    StdCrypto,
};
use quantumshield_refimpl::crypto::traits::{
    Hash, Kmac, PqKem768, PqSigMldsa65, X25519Dh, X25519Priv, X25519Pub,
};
use quantumshield_refimpl::qse::{Envelope, EnvelopeProfile};
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::Suite2RecvWireState;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use quantumshield_refimpl::RefimplError;
use rand_core::{OsRng, RngCore};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear as TuiClear, List, ListItem, Paragraph},
    Terminal,
};
use reqwest::blocking::Client as HttpClient;
use reqwest::StatusCode as HttpStatus;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::env;
use std::fs::{self, File, OpenOptions};
use std::io::{IsTerminal, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use zeroize::Zeroize;

const CONFIG_FILE_NAME: &str = "config.txt";
const STORE_META_NAME: &str = "store.meta";
const LOCK_FILE_NAME: &str = ".qsc.lock";
const OUTBOX_FILE_NAME: &str = "outbox.json";
const SEND_STATE_NAME: &str = "send.state";
const QSE_ENV_VERSION_V1: u16 = 0x0100;
const POLICY_KEY: &str = "policy_profile";
const STORE_META_TEMPLATE: &str = "store_version=1\nvmk_status=unset\nkeyslots=0\n";
const MAX_QUEUE_LEN: usize = 64;
const MAX_HISTORY_LEN: usize = 128;
const MAX_RETRY_ATTEMPTS: u32 = 5;
const RETRY_BASE_MS: u64 = 20;
const RETRY_MAX_MS: u64 = 200;
const RETRY_JITTER_MS: u64 = 10;
const MAX_TIMEOUT_MS: u64 = 2000;
const TUI_AUTOLOCK_DEFAULT_MINUTES: u64 = 10;
const TUI_AUTOLOCK_MIN_MINUTES: u64 = 1;
const TUI_AUTOLOCK_MAX_MINUTES: u64 = 120;
const TUI_POLL_DEFAULT_INTERVAL_SECONDS: u64 = 10;
const TUI_POLL_MIN_INTERVAL_SECONDS: u64 = 2;
const TUI_POLL_MAX_INTERVAL_SECONDS: u64 = 300;
const RECEIPT_BATCH_WINDOW_MS_DEFAULT: u64 = 250;
const RECEIPT_JITTER_MS_DEFAULT: u64 = 0;
const RECEIPT_BATCH_WINDOW_MS_MAX: u64 = 60_000;
const RECEIPT_JITTER_MS_MAX: u64 = 5_000;
const ATTACHMENT_DESCRIPTOR_VERSION: u8 = 1;
const ATTACHMENT_DESCRIPTOR_TYPE: &str = "attachment_descriptor";
const ATTACHMENT_CONFIRM_KIND: &str = "attachment_confirmed";
const ATTACHMENT_LOCATOR_KIND_V1: &str = "service_ref_v1";
const ATTACHMENT_INTEGRITY_ALG_V1: &str = "sha512_merkle_v1";
const ATTACHMENT_ENC_CTX_ALG_V1: &str = "chacha20poly1305_part_v1";
const ATTACHMENT_CONTEXT_PACKAGE_LEN: usize = 41;
const ATTACHMENT_CONTEXT_PACKAGE_B64U_LEN: usize = 55;
const ATTACHMENT_CIPHER_TAG_LEN: usize = 16;
const ATTACHMENT_LEGACY_THRESHOLD_BYTES: usize = FILE_XFER_MAX_FILE_SIZE_CEILING;
const ATTACHMENT_DEFAULT_MAX_FILE_SIZE: usize = 100 * 1024 * 1024;
const ATTACHMENT_DEFAULT_MAX_PARTS: usize = 4096;
const ATTACHMENT_STAGING_DIR: &str = "attachments";
const QSC_ATTACHMENT_SERVICE_ENV: &str = "QSC_ATTACHMENT_SERVICE";
const QSC_LEGACY_IN_MESSAGE_STAGE_ENV: &str = "QSC_LEGACY_IN_MESSAGE_STAGE";

mod adversarial;
mod attachments;
mod cmd;
mod contacts;
mod envelope;
mod fs_store;
mod handshake;
mod identity;
mod model;
mod output;
mod protocol_state;
mod relay;
mod store;
mod timeline;
mod transport;
mod tui;
mod vault;

pub(crate) use timeline::{timeline_ts_default, TimelineEntry};

use attachments::*;
use cmd::*;
use contacts::*;
use fs_store::{
    check_parent_safe, check_symlink_safe, config_dir, enforce_file_perms, enforce_safe_parents,
    ensure_dir_secure, ensure_store_layout, fsync_dir_best_effort, lock_store_exclusive,
    lock_store_shared, normalize_profile, probe_dir_writable, read_policy_profile, set_umask_077,
    write_atomic, write_config_atomic,
};
use handshake::{handshake_init, handshake_poll, handshake_status, hs_kem_keypair, hs_sig_keypair};
use identity::{
    format_verification_code_from_fingerprint, identities_dir, identity_fingerprint_from_pk,
    identity_marker_display, identity_pin_matches_seen, identity_read_pin,
    identity_read_self_public, identity_read_sig_pin, identity_secret_name, identity_secret_store,
    identity_self_fingerprint, identity_self_kem_keypair, identity_self_path,
    identity_sig_secret_name, identity_sig_secret_store, identity_write_public_record,
    IdentityKeypair, IDENTITY_FP_PREFIX,
};
use model::*;
use output::{
    emit_cli_named_marker, emit_marker, emit_tui_named_marker, init_output_policy,
    install_panic_redaction_hook, marker_queue, print_error_marker, print_marker, qsc_mark,
    qsc_sanitize_terminal_text, redact_text_for_output, set_marker_routing, MarkerRouting,
    PANIC_DEMO_SENTINEL,
};
use protocol_state::{
    allow_seed_fallback_for_tests, emit_protocol_inactive, kmac_out,
    protocol_active_or_reason_for_peer, protocol_inactive_exit, qsp_send_ready_tuple,
    qsp_session_for_channel, qsp_session_load, qsp_session_store, qsp_status_parts,
    qsp_status_string, qsp_status_tuple, qsp_status_user_note, record_qsp_status, zero32,
    QSP_STATUS_FILE_NAME,
};
use relay::*;
use store::*;
use timeline::{
    apply_attachment_peer_confirmation, apply_file_peer_confirmation,
    apply_message_peer_confirmation, emit_cli_confirm_policy, emit_cli_delivery_state_with_device,
    emit_cli_file_delivery_with_device, emit_cli_receipt_ignored_wrong_device,
    emit_message_state_reject, emit_message_state_transition, emit_tui_confirm_policy,
    emit_tui_delivery_state, emit_tui_delivery_state_with_device, emit_tui_file_delivery,
    emit_tui_file_delivery_with_device, emit_tui_receipt_ignored_wrong_device,
    file_delivery_semantic_from_state, file_delivery_short_id, file_transfer_confirm_id,
    file_transfer_upsert_outbound_record, latest_outbound_file_id,
    message_delivery_semantic_from_state_str, message_state_transition_allowed,
    timeline_append_entry, timeline_append_entry_for_target, timeline_clear, timeline_list,
    timeline_show, timeline_store_load, timeline_store_save, ConfirmApplyOutcome, MessageState,
};
use tui::*;

static VAULT_UNLOCKED_THIS_RUN: AtomicBool = AtomicBool::new(false);

fn set_vault_unlocked(unlocked: bool) {
    VAULT_UNLOCKED_THIS_RUN.store(unlocked, Ordering::SeqCst);
}

fn vault_unlocked() -> bool {
    VAULT_UNLOCKED_THIS_RUN.load(Ordering::SeqCst)
}

fn bootstrap_unlock(passphrase_file: Option<&Path>, passphrase_env: Option<&str>) {
    if vault::unlock_if_mock_provider() {
        set_vault_unlocked(true);
        return;
    }
    if let Some(path) = passphrase_file {
        match vault::unlock_with_passphrase_file(path) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => print_error_marker(code),
        }
    } else if let Some(env_name) = passphrase_env {
        if env_name.trim().is_empty() {
            print_error_marker("vault_locked");
        }
        match vault::unlock_with_passphrase_env(Some(env_name)) {
            Ok(()) => set_vault_unlocked(true),
            Err(code) => print_error_marker(code),
        }
    } else if allow_seed_fallback_for_tests() {
        // Deterministic test mode keeps existing seeded test workflows intact.
        set_vault_unlocked(true);
    }
}

fn require_unlocked(op_name: &'static str) -> bool {
    if vault_unlocked() {
        return true;
    }
    emit_marker(
        "error",
        Some("vault_locked"),
        &[("op", op_name), ("reason", "explicit_unlock_required")],
    );
    process::exit(1);
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
        },
        Some(Cmd::Doctor {
            check_only,
            timeout_ms,
            export,
        }) => doctor_check_only(check_only, timeout_ms, export),
        Some(Cmd::Util { cmd }) => match cmd {
            UtilCmd::Sanitize { print } => util_sanitize(print),
            UtilCmd::Queue { len } => util_queue(len),
            UtilCmd::History { len } => util_history(len),
            UtilCmd::Retry { fail } => util_retry(fail),
            UtilCmd::Timeout {
                wait_ms,
                timeout_ms,
            } => util_timeout(wait_ms, timeout_ms),
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
            ),
            UtilCmd::PanicDemo => util_panic_demo(),
            UtilCmd::ReceiptApply {
                peer,
                channel,
                msg_id,
                file_id,
                confirm_id,
            } => util_receipt_apply(peer.as_str(), channel.as_str(), msg_id, file_id, confirm_id),
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
        },
        Some(Cmd::Vault { cmd }) => vault::cmd_vault(cmd),
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
        },
        Some(Cmd::Receive {
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
                    print_error_marker("recv_file_conflict");
                }
                receive_file(&path);
            } else {
                let args = ReceiveArgs {
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
                };
                transport::receive_execute(args);
            }
        }
        Some(Cmd::Handshake { cmd }) => match cmd {
            HandshakeCmd::Init {
                as_label,
                peer,
                relay,
            } => handshake_init(&as_label, &peer, &relay),
            HandshakeCmd::Poll {
                as_label,
                peer,
                relay,
                max,
            } => handshake_poll(&as_label, &peer, &relay, max),
            HandshakeCmd::Status { peer } => handshake_status(peer.as_deref()),
        },
        Some(Cmd::Identity { cmd }) => match cmd {
            IdentityCmd::Show { as_label } => identity_show(&as_label),
            IdentityCmd::Rotate {
                as_label,
                confirm,
                reset_peers,
            } => identity_rotate(&as_label, confirm, reset_peers),
        },
        Some(Cmd::Peers { cmd }) => match cmd {
            PeersCmd::List => peers_list(),
        },
        Some(Cmd::Contacts { cmd }) => match cmd {
            ContactsCmd::Add {
                label,
                fp,
                route_token,
                verify,
            } => contacts_add(&label, &fp, route_token.as_deref(), verify),
            ContactsCmd::Show { label } => contacts_show(&label),
            ContactsCmd::List => contacts_list(),
            ContactsCmd::Verify { label, fp, confirm } => contacts_verify(&label, &fp, confirm),
            ContactsCmd::Block { label } => contacts_block(&label),
            ContactsCmd::Unblock { label } => contacts_unblock(&label),
            ContactsCmd::RouteSet { label, route_token } => {
                contacts_route_set(&label, &route_token)
            }
            ContactsCmd::Device { cmd } => match cmd {
                ContactsDeviceCmd::Add {
                    label,
                    fp,
                    route_token,
                } => contacts_device_add(&label, &fp, route_token.as_deref()),
                ContactsDeviceCmd::List { label } => contacts_device_list(&label),
                ContactsDeviceCmd::Status { label, device } => {
                    contacts_device_status(&label, device.as_deref())
                }
                ContactsDeviceCmd::Verify { label, device, fp } => {
                    contacts_device_verify(&label, &device, &fp)
                }
                ContactsDeviceCmd::Trust {
                    label,
                    device,
                    confirm,
                } => contacts_device_trust(&label, &device, confirm),
                ContactsDeviceCmd::Revoke {
                    label,
                    device,
                    confirm,
                } => contacts_device_revoke(&label, &device, confirm),
                ContactsDeviceCmd::Primary { cmd } => match cmd {
                    ContactsDevicePrimaryCmd::Set {
                        label,
                        device,
                        confirm,
                    } => contacts_device_primary_set(&label, &device, confirm),
                    ContactsDevicePrimaryCmd::Show { label } => {
                        contacts_device_primary_show(&label)
                    }
                },
            },
            ContactsCmd::TrustMode { cmd } => match cmd {
                ContactsTrustModeCmd::Show => contacts_trust_mode_show(),
                ContactsTrustModeCmd::Set { mode } => contacts_trust_mode_set(mode),
            },
            ContactsCmd::Request { cmd } => match cmd {
                ContactsRequestCmd::List => contacts_request_list(),
                ContactsRequestCmd::Accept { label } => contacts_request_accept(&label),
                ContactsRequestCmd::Ignore { label } => contacts_request_ignore(&label),
                ContactsRequestCmd::Block { label } => contacts_request_block(&label),
            },
        },
        Some(Cmd::Timeline { cmd }) => match cmd {
            TimelineCmd::List { peer, limit } => timeline_list(&peer, limit),
            TimelineCmd::Show { peer, id } => timeline_show(&peer, &id),
            TimelineCmd::Clear { peer, confirm } => timeline_clear(&peer, confirm),
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
            }),
        },
        Some(Cmd::Tui {
            headless,
            transport: _transport,
            relay,
            token_file,
            seed,
            scenario,
        }) => tui_entry(
            headless,
            TuiConfig {
                relay,
                token_file,
                seed,
                scenario,
            },
        ),
        Some(Cmd::Relay { cmd }) => relay_cmd(cmd),
        Some(Cmd::Meta { cmd }) => meta_cmd(cmd),
    }
}

fn relay_cmd(cmd: RelayCmd) {
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
                print_error_marker("relay_pct_invalid");
            }
            let cfg = RelayConfig {
                seed,
                drop_pct,
                dup_pct,
                reorder_window,
                fixed_latency_ms,
                jitter_ms,
            };
            transport::relay_serve(port, cfg, max_messages);
        }
        RelayCmd::Send {
            to,
            file,
            relay,
            bucket_max,
        } => {
            if !require_unlocked("relay_send") {
                return;
            }
            transport::relay_send(&to, &file, &relay, None, bucket_max, None, None)
        }
        RelayCmd::InboxSet { token } => {
            if !require_unlocked("relay_inbox_set") {
                return;
            }
            let token = normalize_route_token(token.as_str())
                .unwrap_or_else(|code| print_error_marker(code));
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, token.as_str()).is_err() {
                print_error_marker("relay_inbox_token_store_failed");
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
            if !require_unlocked("relay_inbox_clear") {
                return;
            }
            if vault::secret_set(TUI_RELAY_INBOX_TOKEN_SECRET_KEY, "").is_err() {
                print_error_marker("relay_inbox_token_store_failed");
            }
            emit_marker(
                "relay_inbox_clear",
                None,
                &[("ok", "true"), ("token", "cleared")],
            );
            println!("relay_inbox_token=cleared");
        }
    }
}

fn meta_cmd(cmd: MetaCmd) {
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
                Ok(None) => print_error_marker("meta_poll_invalid"),
                Err(code) => print_error_marker(code),
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
}

fn compute_envelope_status(payload_len: usize) -> String {
    let plan = envelope::plan_for_payload_len(
        payload_len,
        3,
        100,
        envelope::MAX_TICKS_DEFAULT,
        envelope::MAX_BUNDLE_SIZE_DEFAULT,
        envelope::MAX_PAYLOAD_COUNT_DEFAULT,
    );
    match plan {
        Ok(p) => {
            let tick = p.ticks.first().copied().unwrap_or(0);
            format!("bucket={} tick={}", p.bundle.bucket_len, tick)
        }
        Err(e) => format!("invalid({})", e.code()),
    }
}

fn compute_local_fingerprint() -> String {
    match identity_self_fingerprint("self") {
        Ok(fp) => fp,
        Err(_) => "untrusted".to_string(),
    }
}

fn compute_peer_fingerprint(peer: &str) -> String {
    let (fp, pinned) = identity_peer_status(peer);
    if pinned {
        format!("{} (pinned)", fp)
    } else {
        "untrusted".to_string()
    }
}

fn split_cmd_result_entry(entry: &str) -> (&str, &str, &str) {
    let Some(rest) = entry.strip_prefix('[') else {
        return ("unknown", "unknown", entry);
    };
    let Some((status, after_status)) = rest.split_once("] /") else {
        return ("unknown", "unknown", entry);
    };
    let Some((command, detail)) = after_status.split_once(' ') else {
        return (status, after_status, "ok");
    };
    (status, command, detail)
}

fn relay_endpoint_hash8(endpoint: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(endpoint.as_bytes());
    hex_encode(&hash[..4])
}

fn relay_token_file_hash8(path: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(path.as_bytes());
    hex_encode(&hash[..4])
}

fn short_hash12(value: &str) -> String {
    let c = StdCrypto;
    let hash = c.sha512(value.as_bytes());
    hex_encode(&hash[..6])
}

fn short_identity_display(value: &str) -> String {
    if value.eq_ignore_ascii_case("untrusted") || value.trim().is_empty() {
        "untrusted".to_string()
    } else {
        short_hash12(value)
    }
}

fn relay_user_reason_from_code(code: &str) -> &'static str {
    match code {
        "relay_endpoint_missing" => "Relay endpoint missing: configure an endpoint first.",
        "relay_test_already_running" => "Relay test already running: wait for completion.",
        "relay_test_pending_timeout" => "Relay test did not complete in time.",
        "relay_unauthorized" => "Unauthorized (401): check token or token file.",
        "relay_overloaded" | "relay_inbox_queue_full" => "Relay overloaded (429): retry shortly.",
        "relay_network_unreachable" => "Network unreachable: check host, network, and firewall.",
        "relay_dns_failure" => "DNS failure: verify relay hostname.",
        "relay_network_timeout" => "Network timeout: relay did not respond in time.",
        "relay_inbox_push_failed" | "relay_inbox_pull_failed" | "relay_http_failure" => {
            "Relay request failed: verify endpoint and retry."
        }
        "relay_token_file_missing" => "Token file missing: set a valid token file path.",
        "relay_token_file_unreadable" => "Token file unreadable: check file ownership and perms.",
        "relay_token_file_empty" => "Token file empty: provide a valid bearer token.",
        "relay_token_file_perms_too_open" => "Token file perms too open: require 0600.",
        "relay_client_init_failed" => "Client init failed: local HTTP client unavailable.",
        "QSC_ERR_RELAY_TLS_REQUIRED" => "TLS required: use HTTPS (or loopback HTTP).",
        _ => "Relay operation failed.",
    }
}

fn read_relay_token_file(path: &str) -> Result<String, &'static str> {
    let p = Path::new(path);
    let md = fs::metadata(p).map_err(|_| "relay_token_file_missing")?;
    if !md.is_file() {
        return Err("relay_token_file_unreadable");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = md.permissions().mode() & 0o777;
        if mode != 0o600 {
            return Err("relay_token_file_perms_too_open");
        }
    }
    let raw = fs::read_to_string(p).map_err(|_| "relay_token_file_unreadable")?;
    let token = raw.trim().to_string();
    if token.is_empty() {
        return Err("relay_token_file_empty");
    }
    Ok(token)
}

fn normalize_relay_endpoint(value: &str) -> Result<String, &'static str> {
    adversarial::route::normalize_relay_endpoint(value)
}

fn relay_transport_label(endpoint: Option<&str>) -> &'static str {
    let Some(url) = endpoint else {
        return "unset";
    };
    if url.starts_with("https://") {
        "https"
    } else if url.starts_with("http://") {
        "http"
    } else {
        "unknown"
    }
}

fn relay_tls_label(endpoint: Option<&str>) -> &'static str {
    let Some(url) = endpoint else {
        return "unset";
    };
    if url.starts_with("https://") {
        "enabled"
    } else if url.starts_with("http://") {
        "disabled"
    } else {
        "unknown"
    }
}

fn relay_pinning_label(endpoint: Option<&str>) -> &'static str {
    match relay_tls_label(endpoint) {
        "enabled" => "not configured",
        _ => "n/a",
    }
}

fn validated_front_door_note() -> &'static str {
    "qbuild/local runbook is the validated front door."
}

fn compatibility_surface_note() -> &'static str {
    "remote/AWS artifacts remain compatibility evidence only."
}

fn validated_front_door_marker() -> &'static str {
    "local_qbuild_front_door"
}

fn compatibility_surface_marker() -> &'static str {
    "remote_aws_compat_only"
}

fn migration_posture_note(attachment_service_active: bool) -> &'static str {
    if attachment_service_active {
        "Validated post-w0 lane active: <= 4 MiB sends use w2 and legacy receive defaults to retired."
    } else {
        "Set QSC_ATTACHMENT_SERVICE to activate the validated post-w0 lane (w2 sends + retired legacy receive)."
    }
}

fn migration_posture_marker(attachment_service_active: bool) -> &'static str {
    if attachment_service_active {
        "attachment_service_active"
    } else {
        "attachment_service_required"
    }
}

fn vault_access_note(locked: bool) -> &'static str {
    if locked {
        "unlock required for local state"
    } else {
        "unlocked"
    }
}

fn vault_attempt_limit_note(limit: Option<u32>) -> String {
    match limit {
        Some(value) => format!("{value} failures wipe local vault + state"),
        None => "off (no automatic wipe threshold)".to_string(),
    }
}

fn relay_probe_url(endpoint: &str) -> Result<String, &'static str> {
    adversarial::route::relay_probe_url(endpoint)
}

fn account_storage_safety_status() -> String {
    let (cfg_dir, source) = match config_dir() {
        Ok(v) => v,
        Err(code) => return format!("reject ({})", code.as_str()),
    };
    if !check_symlink_safe(&cfg_dir) {
        return "reject (unsafe path symlink)".to_string();
    }
    if !check_parent_safe(&cfg_dir, source) {
        return "reject (unsafe parent perms)".to_string();
    }
    "OK".to_string()
}

fn identity_peer_status(peer: &str) -> (String, bool) {
    match identity_read_pin(peer) {
        Ok(Some(fp)) => (fp, true),
        Ok(None) => ("untrusted".to_string(), false),
        Err(_) => ("untrusted".to_string(), false),
    }
}

fn identity_show(self_label: &str) {
    let Some(rec) =
        identity_read_self_public(self_label).unwrap_or_else(|e| print_error_marker(e.as_str()))
    else {
        emit_marker(
            "identity_show",
            None,
            &[("ok", "false"), ("reason", "missing_identity")],
        );
        print_error_marker("identity_missing");
    };
    let fp = identity_fingerprint_from_pk(&rec.kem_pk);
    emit_marker(
        "identity_show",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn identity_rotate(self_label: &str, confirm: bool, reset_peers: bool) {
    if !require_unlocked("identity_rotate") {
        return;
    }
    if !confirm {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "confirm_required")],
        );
        print_error_marker("identity_rotate_confirm_required");
    }
    let (kem_pk, kem_sk) = hs_kem_keypair();
    let (sig_pk, sig_sk) = hs_sig_keypair();
    if identity_secret_store(self_label, &kem_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_sig_secret_store(self_label, &sig_sk).is_err() {
        emit_marker(
            "identity_secret_unavailable",
            None,
            &[("reason", "vault_missing_or_locked")],
        );
        print_error_marker("identity_secret_unavailable");
    }
    if identity_write_public_record(self_label, &kem_pk, &sig_pk).is_err() {
        emit_marker(
            "identity_rotate",
            None,
            &[("ok", "false"), ("reason", "write_failed")],
        );
        print_error_marker("identity_rotate_write_failed");
    }
    if reset_peers {
        let empty = ContactsStore::default();
        let _ = contacts_store_save(&empty);
        if let Ok((dir, source)) = config_dir() {
            let identities = identities_dir(&dir);
            if ensure_dir_secure(&identities, source).is_ok() {
                if let Ok(entries) = fs::read_dir(&identities) {
                    for entry in entries.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            if name.starts_with("peer_") && name.ends_with(".fp") {
                                let _ = fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }
    }
    let fp = identity_fingerprint_from_pk(&kem_pk);
    emit_marker(
        "identity_rotate",
        None,
        &[("ok", "true"), ("fp", fp.as_str())],
    );
    println!("identity_fp={}", fp);
}

fn peers_list() {
    let mut peers = contacts_list_entries()
        .unwrap_or_else(|_| print_error_marker("contacts_store_unavailable"))
        .into_iter()
        .map(|(label, rec)| (label, rec.fp))
        .collect::<Vec<_>>();
    peers.sort_by(|a, b| a.0.cmp(&b.0));
    let count_s = peers.len().to_string();
    emit_marker("peers_list", None, &[("count", count_s.as_str())]);
    for (peer, fp) in peers.iter() {
        emit_marker(
            "peer_item",
            None,
            &[
                ("peer", peer.as_str()),
                ("fp", fp.as_str()),
                ("status", "pinned"),
            ],
        );
        println!("peer={} fp={} status=pinned", peer, fp);
    }
}

fn env_bool(key: &str) -> bool {
    matches!(
        env::var(key).ok().as_deref(),
        Some("1") | Some("true") | Some("TRUE") | Some("yes") | Some("YES")
    )
}

fn tui_color_enabled() -> bool {
    if env::var_os("NO_COLOR").is_some() {
        return false;
    }
    if env::var("TERM")
        .ok()
        .map(|v| v.eq_ignore_ascii_case("dumb"))
        .unwrap_or(false)
    {
        return false;
    }
    true
}

fn config_set(key: &str, value: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let profile = match normalize_profile(value) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };

    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let _lock = match lock_store_exclusive(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    if let Err(e) = ensure_store_layout(&dir, source) {
        print_error(e);
    }
    if let Err(e) = write_config_atomic(&file, &profile, source) {
        print_error(e);
    }

    print_marker(
        "config_set",
        &[
            ("key", "policy_profile"),
            ("value", &profile),
            ("ok", "true"),
        ],
    );
}

fn config_get(key: &str) {
    if key != "policy-profile" {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    if let Err(e) = enforce_safe_parents(&file, source) {
        print_error(e);
    }
    let _lock = match lock_store_shared(&dir, source) {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    #[cfg(unix)]
    if file.exists() {
        if let Err(e) = enforce_file_perms(&file) {
            print_error(e);
        }
    }

    let value = match read_policy_profile(&file) {
        Ok(Some(v)) => v,
        Ok(None) => "unset".to_string(),
        Err(e) => print_error(e),
    };

    print_marker(
        "config_get",
        &[("key", "policy_profile"), ("value", &value), ("ok", "true")],
    );
}

fn parse_vault_attempt_limit_config(raw: &str) -> Result<Option<u32>, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("attempt_limit=") else {
            continue;
        };
        let value = value.trim();
        if value.eq_ignore_ascii_case("off") {
            return Ok(None);
        }
        let parsed = value.parse::<u32>().map_err(|_| ErrorCode::ParseFailed)?;
        if !(VAULT_ATTEMPT_LIMIT_MIN..=VAULT_ATTEMPT_LIMIT_MAX).contains(&parsed) {
            return Err(ErrorCode::ParseFailed);
        }
        return Ok(Some(parsed));
    }
    Ok(None)
}

fn parse_vault_failed_unlocks(raw: &str) -> Result<u32, ErrorCode> {
    for line in raw.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let Some(value) = trimmed.strip_prefix("failed_unlocks=") else {
            continue;
        };
        return value
            .trim()
            .parse::<u32>()
            .map_err(|_| ErrorCode::ParseFailed);
    }
    Ok(0)
}

fn vault_security_state_load() -> Result<VaultSecurityState, ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_shared(&dir, source)?;

    let attempt_limit = if config_path.exists() {
        #[cfg(unix)]
        enforce_file_perms(&config_path)?;
        let mut raw = String::new();
        File::open(&config_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        parse_vault_attempt_limit_config(raw.as_str())?
    } else {
        None
    };

    let failed_unlocks = if counter_path.exists() {
        #[cfg(unix)]
        enforce_file_perms(&counter_path)?;
        let mut raw = String::new();
        File::open(&counter_path)
            .map_err(|_| ErrorCode::IoReadFailed)?
            .read_to_string(&mut raw)
            .map_err(|_| ErrorCode::IoReadFailed)?;
        parse_vault_failed_unlocks(raw.as_str())?
    } else {
        0
    };

    Ok(VaultSecurityState {
        attempt_limit,
        failed_unlocks,
    })
}

fn vault_security_state_store(state: &VaultSecurityState) -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;

    let config_content = match state.attempt_limit {
        Some(limit) => format!("attempt_limit={limit}\n"),
        None => "attempt_limit=off\n".to_string(),
    };
    let counter_content = format!("failed_unlocks={}\n", state.failed_unlocks);
    write_atomic(&config_path, config_content.as_bytes(), source)?;
    write_atomic(&counter_path, counter_content.as_bytes(), source)?;
    Ok(())
}

fn vault_security_state_clear_files() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let config_path = dir.join(VAULT_SECURITY_CONFIG_NAME);
    let counter_path = dir.join(VAULT_UNLOCK_COUNTER_NAME);
    enforce_safe_parents(&config_path, source)?;
    enforce_safe_parents(&counter_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    let _ = fs::remove_file(config_path);
    let _ = fs::remove_file(counter_path);
    fsync_dir_best_effort(&dir);
    Ok(())
}

fn wipe_vault_file_best_effort() -> Result<(), ErrorCode> {
    let (dir, source) = config_dir()?;
    ensure_store_layout(&dir, source)?;
    let vault_path = dir.join("vault.qsv");
    enforce_safe_parents(&vault_path, source)?;
    let _lock = lock_store_exclusive(&dir, source)?;
    if !vault_path.exists() {
        return Ok(());
    }
    let tombstone = dir.join(format!("vault.qsv.tombstone.{}", process::id()));
    if fs::rename(&vault_path, &tombstone).is_ok() {
        let _ = fs::remove_file(&tombstone);
    } else {
        let _ = fs::remove_file(&vault_path);
    }
    fsync_dir_best_effort(&dir);
    Ok(())
}

#[derive(Serialize)]
struct DoctorReport {
    check_only: bool,
    ok: bool,
    dir_exists: bool,
    dir_writable: bool,
    file_parseable: bool,
    symlink_safe: bool,
    parent_safe: bool,
    config_dir: &'static str,
    redacted: bool,
}

fn doctor_check_only(check_only: bool, timeout_ms: u64, export: Option<PathBuf>) {
    if !check_only {
        print_error(ErrorCode::ParseFailed);
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    let file = dir.join(CONFIG_FILE_NAME);

    let symlink_safe = check_symlink_safe(&dir);
    let parent_safe = check_parent_safe(&dir, source);
    let dir_exists = dir.is_dir();
    let checked_dir = dir.display().to_string();
    let dir_writable_required = false;
    let dir_writable = if dir_exists && symlink_safe && parent_safe {
        probe_dir_writable(&dir, timeout_ms)
    } else {
        false
    };

    let file_parseable = file.exists()
        && matches!(read_policy_profile(&file), Ok(Some(_)) | Ok(None))
        || !file.exists();
    let receipt_policy = load_receipt_policy_from_account();
    let receipt_batch_window_s = receipt_policy.batch_window_ms.to_string();
    let receipt_jitter_s = receipt_policy.jitter_ms.to_string();

    let report = DoctorReport {
        check_only: true,
        ok: true,
        dir_exists,
        dir_writable,
        file_parseable,
        symlink_safe,
        parent_safe,
        config_dir: "<redacted>",
        redacted: true,
    };

    if let Some(path) = export {
        if let Err(e) = write_doctor_export(&path, &report) {
            print_error(e);
        }
    }

    print_marker(
        "doctor",
        &[
            ("check_only", "true"),
            ("ok", "true"),
            ("checked_dir", &checked_dir),
            (
                "dir_writable_required",
                if dir_writable_required {
                    "true"
                } else {
                    "false"
                },
            ),
            ("dir_exists", bool_str(dir_exists)),
            ("dir_writable", bool_str(dir_writable)),
            ("file_parseable", bool_str(file_parseable)),
            ("symlink_safe", bool_str(symlink_safe)),
            ("parent_safe", bool_str(parent_safe)),
            ("receipt_mode", receipt_policy.mode.as_str()),
            (
                "file_confirm_mode",
                receipt_policy.file_confirm_mode.as_str(),
            ),
            ("receipt_batch_window_ms", receipt_batch_window_s.as_str()),
            ("receipt_jitter_ms", receipt_jitter_s.as_str()),
        ],
    );
}

fn protocol_active_or_reason_for_send_peer(peer: &str) -> Result<(), String> {
    let routing = resolve_send_routing_target(peer).map_err(|code| code.to_string())?;
    protocol_active_or_reason_for_peer(routing.channel.as_str())
}

struct QspPackOutcome {
    envelope: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    ck_idx: u32,
    padded_len: usize,
    pad_label: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct QspPackError {
    code: &'static str,
    reason: Option<&'static str>,
}

struct QspUnpackOutcome {
    plaintext: Vec<u8>,
    next_state: Suite2SessionState,
    msg_idx: u32,
    skip_delta: usize,
    evicted: usize,
}

const MKSKIPPED_CAP_DEFAULT: usize = 32;
const POLL_INTERVAL_MS_MAX: u64 = 60_000;
const POLL_TICKS_MAX: u32 = 64;
const POLL_MAX_PER_TICK_MAX: u32 = 32;
const PAD_TO_MAX: usize = 65_536;
const META_TICK_COUNT_DEFAULT: u32 = 1;
const META_INTERVAL_MS_DEFAULT: u64 = 1_000;
const META_BATCH_MAX_COUNT_DEFAULT: u32 = 1;
const META_BUCKET_MAX_DEFAULT: usize = 4_096;
const META_BUCKET_MAX_CEILING: usize = 65_536;

struct MetaPollConfig {
    interval_ms: u64,
    ticks: u32,
    batch_max_count: usize,
    bucket_max: usize,
    deterministic: bool,
}

#[derive(Clone, Copy)]
struct MetaPadConfig {
    target_len: Option<usize>,
    profile: Option<EnvelopeProfile>,
    label: Option<&'static str>,
}

fn mkskipped_cap() -> usize {
    let cap = env::var("QSC_MKSKIPPED_CAP")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(MKSKIPPED_CAP_DEFAULT);
    cap.clamp(1, 1000)
}

fn bound_mkskipped(st: &mut Suite2RecvWireState) -> usize {
    let cap = mkskipped_cap();
    if st.mkskipped.len() <= cap {
        return 0;
    }
    st.mkskipped.sort_by_key(|e| e.n);
    let excess = st.mkskipped.len().saturating_sub(cap);
    if excess > 0 {
        st.mkskipped.drain(0..excess);
    }
    excess
}

fn meta_poll_config_from_args(args: MetaPollArgs) -> Result<Option<MetaPollConfig>, &'static str> {
    let MetaPollArgs {
        deterministic_meta,
        interval_ms,
        poll_interval_ms,
        ticks,
        batch_max_count,
        poll_max_per_tick,
        bucket_max,
        meta_seed,
    } = args;
    if interval_ms.is_some() && poll_interval_ms.is_some() {
        return Err("meta_poll_conflict");
    }
    if batch_max_count.is_some() && poll_max_per_tick.is_some() {
        return Err("meta_poll_conflict");
    }
    let any = deterministic_meta
        || interval_ms.is_some()
        || poll_interval_ms.is_some()
        || ticks.is_some()
        || batch_max_count.is_some()
        || poll_max_per_tick.is_some()
        || bucket_max.is_some()
        || meta_seed.is_some();
    if !any {
        return Ok(None);
    }
    let interval_ms = interval_ms
        .or(poll_interval_ms)
        .unwrap_or(META_INTERVAL_MS_DEFAULT);
    let ticks = ticks.unwrap_or(META_TICK_COUNT_DEFAULT);
    let batch_max_count = batch_max_count
        .or(poll_max_per_tick)
        .unwrap_or(META_BATCH_MAX_COUNT_DEFAULT);
    let bucket_max = bucket_max.unwrap_or(META_BUCKET_MAX_DEFAULT);
    if interval_ms == 0 || interval_ms > POLL_INTERVAL_MS_MAX {
        return Err("meta_poll_invalid");
    }
    if ticks == 0 || ticks > POLL_TICKS_MAX {
        return Err("meta_poll_invalid");
    }
    if batch_max_count == 0 || batch_max_count > POLL_MAX_PER_TICK_MAX {
        return Err("meta_poll_invalid");
    }
    if bucket_max == 0 || bucket_max > META_BUCKET_MAX_CEILING {
        return Err("meta_poll_invalid");
    }
    Ok(Some(MetaPollConfig {
        interval_ms,
        ticks,
        batch_max_count: batch_max_count as usize,
        bucket_max,
        deterministic: deterministic_meta || meta_seed.is_some(),
    }))
}

struct MetaPollArgs {
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
}

fn meta_bucket_for_len(orig_len: usize, bucket_max: usize) -> usize {
    let capped = orig_len.min(bucket_max).max(1);
    let mut bucket = 1usize;
    while bucket < capped {
        bucket = bucket.saturating_mul(2);
    }
    bucket.min(bucket_max)
}

type ReceiptControlPayload = adversarial::payload::ReceiptControlPayload;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ReceiptEmitMode {
    Off,
    Batched,
    Immediate,
}

impl ReceiptEmitMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Batched => "batched",
            Self::Immediate => "immediate",
        }
    }

    fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "batched" => Some(Self::Batched),
            "immediate" => Some(Self::Immediate),
            _ => None,
        }
    }

    fn from_arg(value: ReceiptMode) -> Self {
        match value {
            ReceiptMode::Off => Self::Off,
            ReceiptMode::Batched => Self::Batched,
            ReceiptMode::Immediate => Self::Immediate,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FileConfirmEmitMode {
    Off,
    CompleteOnly,
}

impl FileConfirmEmitMode {
    fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::CompleteOnly => "complete_only",
        }
    }

    fn from_raw(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "off" => Some(Self::Off),
            "complete_only" | "complete-only" | "completeonly" => Some(Self::CompleteOnly),
            _ => None,
        }
    }

    fn from_arg(value: FileConfirmMode) -> Self {
        match value {
            FileConfirmMode::Off => Self::Off,
            FileConfirmMode::CompleteOnly => Self::CompleteOnly,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct ReceiptPolicy {
    mode: ReceiptEmitMode,
    batch_window_ms: u64,
    jitter_ms: u64,
    file_confirm_mode: FileConfirmEmitMode,
}

impl Default for ReceiptPolicy {
    fn default() -> Self {
        Self {
            mode: ReceiptEmitMode::Off,
            batch_window_ms: RECEIPT_BATCH_WINDOW_MS_DEFAULT,
            jitter_ms: RECEIPT_JITTER_MS_DEFAULT,
            file_confirm_mode: FileConfirmEmitMode::CompleteOnly,
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct ReceiptPolicyOverrides {
    emit_receipts: Option<ReceiptKind>,
    receipt_mode: Option<ReceiptMode>,
    receipt_batch_window_ms: Option<u64>,
    receipt_jitter_ms: Option<u64>,
    file_confirm_mode: Option<FileConfirmMode>,
}

fn parse_receipt_batch_window_ms(value: &str) -> Option<u64> {
    let parsed = value.trim().parse::<u64>().ok()?;
    if (1..=RECEIPT_BATCH_WINDOW_MS_MAX).contains(&parsed) {
        Some(parsed)
    } else {
        None
    }
}

fn parse_receipt_jitter_ms(value: &str) -> Option<u64> {
    let parsed = value.trim().parse::<u64>().ok()?;
    if parsed <= RECEIPT_JITTER_MS_MAX {
        Some(parsed)
    } else {
        None
    }
}

fn account_secret_trimmed(key: &str) -> Option<String> {
    vault::secret_get(key)
        .ok()
        .flatten()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn load_receipt_policy_from_account() -> ReceiptPolicy {
    if !vault_unlocked() {
        return ReceiptPolicy::default();
    }
    let mut policy = ReceiptPolicy::default();
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_MODE_SECRET_KEY) {
        if let Some(mode) = ReceiptEmitMode::from_raw(raw.as_str()) {
            policy.mode = mode;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_BATCH_WINDOW_MS_SECRET_KEY) {
        if let Some(ms) = parse_receipt_batch_window_ms(raw.as_str()) {
            policy.batch_window_ms = ms;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_RECEIPT_JITTER_MS_SECRET_KEY) {
        if let Some(ms) = parse_receipt_jitter_ms(raw.as_str()) {
            policy.jitter_ms = ms;
        }
    }
    if let Some(raw) = account_secret_trimmed(TUI_FILE_CONFIRM_MODE_SECRET_KEY) {
        if let Some(mode) = FileConfirmEmitMode::from_raw(raw.as_str()) {
            policy.file_confirm_mode = mode;
        }
    }
    policy
}

fn resolve_receipt_policy(overrides: ReceiptPolicyOverrides) -> ReceiptPolicy {
    let mut policy = load_receipt_policy_from_account();
    if overrides.emit_receipts.is_some() {
        policy.mode = ReceiptEmitMode::Immediate;
        policy.file_confirm_mode = FileConfirmEmitMode::CompleteOnly;
    }
    if let Some(mode) = overrides.receipt_mode {
        policy.mode = ReceiptEmitMode::from_arg(mode);
    }
    if let Some(ms) = overrides.receipt_batch_window_ms {
        policy.batch_window_ms = ms.clamp(1, RECEIPT_BATCH_WINDOW_MS_MAX);
    }
    if let Some(ms) = overrides.receipt_jitter_ms {
        policy.jitter_ms = ms.min(RECEIPT_JITTER_MS_MAX);
    }
    if let Some(mode) = overrides.file_confirm_mode {
        policy.file_confirm_mode = FileConfirmEmitMode::from_arg(mode);
    }
    if policy.mode != ReceiptEmitMode::Batched {
        policy.batch_window_ms = RECEIPT_BATCH_WINDOW_MS_DEFAULT;
        policy.jitter_ms = RECEIPT_JITTER_MS_DEFAULT;
    }
    policy
}

fn receipt_kind_str(kind: ReceiptKind) -> &'static str {
    match kind {
        ReceiptKind::Delivered => "delivered",
    }
}

fn receipt_msg_id(payload: &[u8]) -> String {
    let c = StdCrypto;
    let h = c.sha512(payload);
    hex_encode(&h[..8])
}

fn encode_receipt_data_payload(
    payload: Vec<u8>,
    receipt: Option<ReceiptKind>,
) -> (Vec<u8>, Option<String>) {
    let Some(kind) = receipt else {
        return (payload, None);
    };
    let msg_id = receipt_msg_id(&payload);
    let ctrl = ReceiptControlPayload {
        v: 1,
        t: "data".to_string(),
        kind: receipt_kind_str(kind).to_string(),
        msg_id: msg_id.clone(),
        body: Some(payload),
    };
    let encoded =
        serde_json::to_vec(&ctrl).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"));
    (encoded, Some(msg_id))
}

fn emit_cli_receipt_policy_event(
    mode: ReceiptEmitMode,
    status: &'static str,
    kind: &'static str,
    peer: &str,
) {
    let safe_peer = short_peer_marker(peer);
    emit_cli_named_marker(
        "QSC_RECEIPT",
        &[
            ("mode", mode.as_str()),
            ("status", status),
            ("kind", kind),
            ("peer", safe_peer.as_str()),
        ],
    );
}

fn emit_tui_receipt_policy_event(
    mode: ReceiptEmitMode,
    status: &'static str,
    kind: &'static str,
    thread: &str,
) {
    let safe_thread = short_peer_marker(thread);
    emit_tui_named_marker(
        "QSC_TUI_RECEIPT",
        &[
            ("mode", mode.as_str()),
            ("status", status),
            ("kind", kind),
            ("thread", safe_thread.as_str()),
        ],
    );
}

fn parse_receipt_payload(plaintext: &[u8]) -> Option<ReceiptControlPayload> {
    adversarial::payload::parse_receipt_payload(plaintext)
}

fn build_delivered_ack(msg_id: &str) -> Vec<u8> {
    let ack = ReceiptControlPayload {
        v: 1,
        t: "ack".to_string(),
        kind: "delivered".to_string(),
        msg_id: msg_id.to_string(),
        body: None,
    };
    serde_json::to_vec(&ack).unwrap_or_else(|_| print_error_marker("receipt_encode_failed"))
}

#[derive(Clone, Debug)]
enum PendingReceipt {
    Message {
        msg_id: String,
    },
    FileComplete {
        file_id: String,
        confirm_id: String,
    },
    AttachmentComplete {
        attachment_id: String,
        confirm_handle: String,
    },
}

fn queue_or_send_receipt(
    ctx: &ReceivePullCtx<'_>,
    queue: &mut Vec<PendingReceipt>,
    item: PendingReceipt,
) {
    let kind = match item {
        PendingReceipt::Message { .. } => "message",
        PendingReceipt::FileComplete { .. } => "file_complete",
        PendingReceipt::AttachmentComplete { .. } => "attachment_complete",
    };
    match ctx.receipt_policy.mode {
        ReceiptEmitMode::Off => {
            emit_cli_receipt_policy_event(ctx.receipt_policy.mode, "skipped", kind, ctx.from);
            emit_tui_receipt_policy_event(ctx.receipt_policy.mode, "skipped", kind, ctx.from);
            emit_marker(
                "receipt_disabled",
                None,
                &[("mode", ctx.receipt_policy.mode.as_str()), ("kind", kind)],
            );
        }
        ReceiptEmitMode::Immediate => {
            send_pending_receipt(ctx, item);
        }
        ReceiptEmitMode::Batched => {
            queue.push(item);
            emit_cli_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
            emit_tui_receipt_policy_event(ctx.receipt_policy.mode, "queued", kind, ctx.from);
        }
    }
}

fn send_pending_receipt(ctx: &ReceivePullCtx<'_>, item: PendingReceipt) {
    match item {
        PendingReceipt::Message { msg_id } => {
            match send_delivered_receipt_ack(ctx.relay, ctx.from, &msg_id) {
                Ok(()) => {
                    emit_marker(
                        "receipt_send",
                        None,
                        &[
                            ("kind", "delivered"),
                            ("bucket", "small"),
                            ("msg_id", "<redacted>"),
                        ],
                    );
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "message",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "message",
                        ctx.from,
                    );
                }
                Err(code) => emit_marker("receipt_send_failed", Some(code), &[("code", code)]),
            }
        }
        PendingReceipt::FileComplete {
            file_id,
            confirm_id,
        } => {
            match send_file_completion_ack(
                ctx.relay,
                ctx.from,
                file_id.as_str(),
                confirm_id.as_str(),
            ) {
                Ok(()) => {
                    let safe_file_id = file_delivery_short_id(file_id.as_str());
                    emit_marker(
                        "file_confirm_send",
                        None,
                        &[
                            ("kind", "coarse_complete"),
                            ("file_id", safe_file_id.as_str()),
                            ("ok", "true"),
                        ],
                    );
                    emit_cli_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "file_complete",
                        ctx.from,
                    );
                    emit_tui_receipt_policy_event(
                        ctx.receipt_policy.mode,
                        "sent",
                        "file_complete",
                        ctx.from,
                    );
                }
                Err(code) => emit_marker("file_confirm_send_failed", Some(code), &[("code", code)]),
            }
        }
        PendingReceipt::AttachmentComplete {
            attachment_id,
            confirm_handle,
        } => {
            let payload = build_attachment_completion_ack(&attachment_id, &confirm_handle);
            let outcome = transport::relay_send_with_payload(RelaySendPayloadArgs {
                to: ctx.from,
                payload,
                relay: ctx.relay,
                injector: transport::fault_injector_from_env(),
                pad_cfg: None,
                bucket_max: None,
                meta_seed: None,
                receipt: None,
                routing_override: None,
                tui_thread: None,
            });
            if let Some(code) = outcome.error_code {
                emit_marker(
                    "attachment_confirm_send_failed",
                    Some(code),
                    &[("code", code)],
                );
                return;
            }
            let safe_attachment = file_delivery_short_id(&attachment_id);
            emit_marker(
                "attachment_confirm_send",
                None,
                &[
                    ("kind", "complete"),
                    ("attachment_id", safe_attachment.as_str()),
                    ("ok", "true"),
                ],
            );
        }
    }
}

fn flush_batched_receipts(ctx: &ReceivePullCtx<'_>, queue: &mut Vec<PendingReceipt>) {
    if ctx.receipt_policy.mode != ReceiptEmitMode::Batched || queue.is_empty() {
        return;
    }
    // Deterministic ordering; jitter only affects stable sort priority.
    queue.sort_by_key(|item| match item {
        PendingReceipt::Message { msg_id } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in msg_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (0u8, bias, msg_id.clone())
        }
        PendingReceipt::FileComplete { file_id, .. } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in file_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (1u8, bias, file_id.clone())
        }
        PendingReceipt::AttachmentComplete { attachment_id, .. } => {
            let bias = if ctx.receipt_policy.jitter_ms == 0 {
                0
            } else {
                let mut acc: u64 = 0;
                for b in attachment_id.as_bytes() {
                    acc = acc.wrapping_add(*b as u64);
                }
                acc % (ctx.receipt_policy.jitter_ms + 1)
            };
            (2u8, bias, attachment_id.clone())
        }
    });
    let pending = std::mem::take(queue);
    for item in pending {
        send_pending_receipt(ctx, item);
    }
}

fn send_delivered_receipt_ack(relay: &str, to: &str, msg_id: &str) -> Result<(), &'static str> {
    let payload = build_delivered_ack(msg_id);
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    let pack = qsp_pack(to, &payload, pad_cfg, None).map_err(|e| e.code)?;
    let route_token = relay_peer_route_token(to)?;
    transport::relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
    qsp_session_store(to, &pack.next_state).map_err(|_| "qsp_session_store_failed")?;
    Ok(())
}

fn send_file_completion_ack(
    relay: &str,
    to: &str,
    file_id: &str,
    confirm_id: &str,
) -> Result<(), &'static str> {
    let payload = build_file_completion_ack(file_id, confirm_id);
    let pad_cfg = Some(MetaPadConfig {
        target_len: None,
        profile: Some(EnvelopeProfile::Standard),
        label: Some("small"),
    });
    let pack = qsp_pack(to, &payload, pad_cfg, None).map_err(|e| e.code)?;
    let route_token = relay_peer_route_token(to)?;
    transport::relay_inbox_push(relay, route_token.as_str(), &pack.envelope)?;
    qsp_session_store(to, &pack.next_state).map_err(|_| "qsp_session_store_failed")?;
    Ok(())
}

fn meta_pad_config_from_args(
    pad_to: Option<usize>,
    pad_bucket: Option<MetaPadBucket>,
    meta_seed: Option<u64>,
) -> Result<Option<MetaPadConfig>, &'static str> {
    if pad_to.is_none() && pad_bucket.is_none() {
        return Ok(None);
    }
    if pad_to.is_some() && pad_bucket.is_some() {
        return Err("meta_pad_conflict");
    }
    if let Some(len) = pad_to {
        if len == 0 || len > PAD_TO_MAX {
            return Err("meta_pad_invalid");
        }
        return Ok(Some(MetaPadConfig {
            target_len: Some(len),
            profile: None,
            label: Some("pad_to"),
        }));
    }
    let bucket = pad_bucket.unwrap_or(MetaPadBucket::Standard);
    let profile = match bucket {
        MetaPadBucket::Standard => EnvelopeProfile::Standard,
        MetaPadBucket::Enhanced => EnvelopeProfile::Enhanced,
        MetaPadBucket::Private => EnvelopeProfile::Private,
        MetaPadBucket::Auto => {
            let seed = meta_seed.ok_or("meta_seed_required")?;
            let mut rng = RelayRng::new(seed ^ 0x51d2a9f1);
            match rng.next_u32() % 3 {
                0 => EnvelopeProfile::Standard,
                1 => EnvelopeProfile::Enhanced,
                _ => EnvelopeProfile::Private,
            }
        }
    };
    let label = match bucket {
        MetaPadBucket::Standard => "standard",
        MetaPadBucket::Enhanced => "enhanced",
        MetaPadBucket::Private => "private",
        MetaPadBucket::Auto => "auto",
    };
    Ok(Some(MetaPadConfig {
        target_len: None,
        profile: Some(profile),
        label: Some(label),
    }))
}

fn map_qsp_recv_err(err: &RefimplError) -> &'static str {
    let s = err.to_string();
    if s.contains("REJECT_S2_REPLAY") {
        "qsp_replay_reject"
    } else if s.contains("REJECT_S2_OOO_BOUNDS") {
        "qsp_ooo_reject"
    } else if s.contains("REJECT_S2_BODY_AUTH_FAIL") {
        "qsp_auth_failed"
    } else if s.contains("REJECT_S2_HDR_AUTH_FAIL") {
        "qsp_hdr_auth_failed"
    } else {
        "qsp_verify_failed"
    }
}

fn map_qsp_pack_reason(err: &RefimplError) -> &'static str {
    let s = err.to_string();
    if s.contains("REJECT_S2_CHAINKEY_UNSET") {
        "chainkey_unset"
    } else if s.contains("REJECT_S2_LOCAL_UNSUPPORTED") {
        "local_unsupported"
    } else if s.contains("REJECT_S2_LOCAL_AEAD_FAIL") {
        "local_aead_fail"
    } else {
        "pack_internal"
    }
}

fn qsp_activate_responder_send_chain_if_needed(st: &mut Suite2SessionState) {
    if st.recv.role_is_a {
        return;
    }
    if !(zero32(&st.send.ck_ec) || zero32(&st.send.ck_pq)) {
        return;
    }
    if zero32(&st.recv.rk) || zero32(&st.send.hk_s) {
        return;
    }
    let c = StdCrypto;
    st.send.ck_ec = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/CK0/B->A", &[0x01]);
    st.send.ck_pq = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/PQ0/B->A", &[0x01]);
    emit_marker(
        "qsp_send_chain",
        None,
        &[
            ("activated", "true"),
            ("reason", "responder_recv_bootstrap"),
        ],
    );
}

fn qsp_activate_initiator_recv_chain_if_needed(st: &mut Suite2SessionState) {
    if !st.recv.role_is_a {
        return;
    }
    if !(zero32(&st.recv.ck_ec) || zero32(&st.recv.ck_pq_recv)) {
        return;
    }
    if zero32(&st.recv.rk) || zero32(&st.recv.hk_r) {
        return;
    }
    let c = StdCrypto;
    st.recv.ck_ec = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/CK0/B->A", &[0x01]);
    st.recv.ck_pq_recv = kmac_out::<32>(&c, &st.recv.rk, "QSP5.0/PQ0/B->A", &[0x01]);
    emit_marker(
        "qsp_recv_chain",
        None,
        &[
            ("activated", "true"),
            ("reason", "initiator_send_bootstrap"),
        ],
    );
}

fn qsp_pack(
    channel: &str,
    plaintext: &[u8],
    pad_cfg: Option<MetaPadConfig>,
    meta_seed: Option<u64>,
) -> Result<QspPackOutcome, QspPackError> {
    let st =
        qsp_session_for_channel(channel).map_err(|code| QspPackError { code, reason: None })?;
    let c = StdCrypto;
    let outcome =
        send_wire_canon(&c, &c, &c, st.send.clone(), 0, plaintext).map_err(|e| QspPackError {
            code: "qsp_pack_failed",
            reason: Some(map_qsp_pack_reason(&e)),
        })?;
    let mut env = Envelope {
        env_version: QSE_ENV_VERSION_V1,
        flags: 0,
        route_token: Vec::new(),
        timestamp_bucket: 0,
        payload: outcome.wire,
        padding: Vec::new(),
    };
    let mut pad_label = None;
    let mut encoded_len = env.encode().len();
    let min_len = EnvelopeProfile::Standard.min_size_bytes();
    if encoded_len < min_len {
        let need = min_len - encoded_len;
        let mut seed_bytes = Vec::new();
        if let Some(seed) = meta_seed {
            seed_bytes.extend_from_slice(&seed.to_le_bytes());
        }
        let pad = c.kmac256(&env.payload, "QSC.QSP.PAD", &seed_bytes, need);
        env = env
            .pad_to_profile(EnvelopeProfile::Standard, &pad)
            .map_err(|_| QspPackError {
                code: "qsp_pack_failed",
                reason: Some("QSP_PACK_INTERNAL"),
            })?;
        encoded_len = env.encode().len();
    }
    if let Some(cfg) = pad_cfg {
        if let Some(target) = cfg.target_len {
            if target < encoded_len {
                return Err(QspPackError {
                    code: "meta_pad_too_small",
                    reason: None,
                });
            }
            let need = target - encoded_len;
            if need > 0 {
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env.padding.extend_from_slice(&pad);
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        } else if let Some(profile) = cfg.profile {
            let min_len = profile.min_size_bytes();
            if encoded_len < min_len {
                let need = min_len - encoded_len;
                let mut seed_bytes = Vec::new();
                if let Some(seed) = meta_seed {
                    seed_bytes.extend_from_slice(&seed.to_le_bytes());
                }
                let pad = c.kmac256(&env.payload, "QSC.META.PAD", &seed_bytes, need);
                env = env
                    .pad_to_profile(profile, &pad)
                    .map_err(|_| QspPackError {
                        code: "qsp_pack_failed",
                        reason: Some("QSP_PACK_INTERNAL"),
                    })?;
                encoded_len = env.encode().len();
            }
            pad_label = cfg.label;
        }
    }
    let mut next_state = st.clone();
    next_state.send = outcome.state;
    qsp_activate_initiator_recv_chain_if_needed(&mut next_state);
    Ok(QspPackOutcome {
        envelope: env.encode(),
        next_state,
        msg_idx: outcome.n,
        ck_idx: outcome.n,
        padded_len: encoded_len,
        pad_label,
    })
}

fn qsp_unpack_channels_for_peer(peer: &str) -> Vec<String> {
    let mut channels = Vec::new();
    channels.push(peer.to_string());
    let peer_alias = peer_alias_from_channel(peer);
    if peer_alias != peer {
        channels.push(peer_alias.to_string());
    }
    if let Ok(Some(mut rec)) = contacts_entry_read(peer_alias) {
        normalize_contact_record(peer_alias, &mut rec);
        for dev in rec.devices.iter() {
            if let Some(channel) = channel_label_for_device(peer_alias, dev.device_id.as_str()) {
                if !channels.iter().any(|v| v == &channel) {
                    channels.push(channel);
                }
            }
        }
    }
    channels
}

fn qsp_unpack_for_peer(
    peer: &str,
    envelope_bytes: &[u8],
) -> Result<(QspUnpackOutcome, String), &'static str> {
    let mut first_err: Option<&'static str> = None;
    for channel in qsp_unpack_channels_for_peer(peer).into_iter() {
        match qsp_unpack(channel.as_str(), envelope_bytes) {
            Ok(outcome) => return Ok((outcome, channel)),
            Err(code) => {
                if first_err.is_none() {
                    first_err = Some(code);
                }
            }
        }
    }
    Err(first_err.unwrap_or("qsp_channel_invalid"))
}

fn qsp_unpack(channel: &str, envelope_bytes: &[u8]) -> Result<QspUnpackOutcome, &'static str> {
    let env = Envelope::decode(envelope_bytes).map_err(|_| "qsp_env_decode_failed")?;
    let st = qsp_session_for_channel(channel)?;
    let c = StdCrypto;
    let outcome = recv_wire_canon(&c, &c, &c, st.recv.clone(), &env.payload, None, None)
        .map_err(|e| map_qsp_recv_err(&e))?;
    let mut next_state = st.clone();
    let prev_len = next_state.recv.mkskipped.len();
    next_state.recv = outcome.state;
    qsp_activate_responder_send_chain_if_needed(&mut next_state);
    let skip_delta = next_state.recv.mkskipped.len().saturating_sub(prev_len);
    let evicted = bound_mkskipped(&mut next_state.recv);
    Ok(QspUnpackOutcome {
        plaintext: outcome.plaintext,
        next_state,
        msg_idx: outcome.n,
        skip_delta,
        evicted,
    })
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        out.push(HEX[(b >> 4) as usize] as char);
        out.push(HEX[(b & 0x0f) as usize] as char);
    }
    out
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ErrorCode> {
    if !s.len().is_multiple_of(2) {
        return Err(ErrorCode::ParseFailed);
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        let hi = hex_nibble(bytes[i]).ok_or(ErrorCode::ParseFailed)?;
        let lo = hex_nibble(bytes[i + 1]).ok_or(ErrorCode::ParseFailed)?;
        out.push((hi << 4) | lo);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn short_peer_marker(peer: &str) -> String {
    let all_hex = peer.chars().all(|ch| ch.is_ascii_hexdigit());
    if all_hex && peer.len() >= 32 {
        peer.chars().take(12).collect()
    } else {
        peer.to_string()
    }
}

struct ReceiveArgs {
    transport: Option<SendTransport>,
    relay: Option<String>,
    legacy_receive_mode: Option<LegacyReceiveMode>,
    attachment_service: Option<String>,
    from: Option<String>,
    mailbox: Option<String>,
    max: Option<usize>,
    max_file_size: Option<usize>,
    max_file_chunks: Option<usize>,
    out: Option<PathBuf>,
    deterministic_meta: bool,
    interval_ms: Option<u64>,
    poll_interval_ms: Option<u64>,
    poll_ticks: Option<u32>,
    batch_max_count: Option<u32>,
    poll_max_per_tick: Option<u32>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    emit_receipts: Option<ReceiptKind>,
    receipt_mode: Option<ReceiptMode>,
    receipt_batch_window_ms: Option<u64>,
    receipt_jitter_ms: Option<u64>,
    file_confirm_mode: Option<FileConfirmMode>,
}

struct ReceivePullCtx<'a> {
    relay: &'a str,
    legacy_receive_mode: LegacyReceiveMode,
    attachment_service: Option<&'a str>,
    mailbox: &'a str,
    from: &'a str,
    out: &'a Path,
    source: ConfigSource,
    cfg_dir: &'a Path,
    cfg_source: ConfigSource,
    bucket_max: usize,
    file_max_size: usize,
    file_max_chunks: usize,
    receipt_policy: ReceiptPolicy,
}

struct ReceivePullStats {
    count: usize,
    bytes: usize,
}

fn receive_file(path: &Path) {
    if !require_unlocked("receive_file") {
        return;
    }
    let (dir, source) = match config_dir() {
        Ok(v) => v,
        Err(e) => print_error(e),
    };
    // Fail-closed: reject if config dir parents or symlinks are unsafe.
    if !check_symlink_safe(&dir) {
        print_error(ErrorCode::UnsafePathSymlink);
    }
    if !check_parent_safe(&dir, source) {
        print_error(ErrorCode::UnsafeParentPerms);
    }

    let bytes = match fs::read(path) {
        Ok(v) => v,
        Err(_) => print_error(ErrorCode::IoReadFailed),
    };
    if bytes.is_empty() {
        emit_marker("recv_reject", None, &[("reason", "empty")]);
        print_error_marker("recv_reject_parse");
    }
    if bytes.len() > envelope::MAX_BUNDLE_SIZE_DEFAULT {
        emit_marker("recv_reject", None, &[("reason", "oversize")]);
        print_error_marker("recv_reject_size");
    }

    emit_marker("recv_reject", None, &[("reason", "malformed")]);
    print_error_marker("recv_reject_parse");
}

struct RelayInboxStore {
    queues: BTreeMap<String, VecDeque<(u64, Vec<u8>)>>,
    next_id: u64,
    max_body: usize,
    max_queue: usize,
}

impl RelayInboxStore {
    fn new(max_body: usize, max_queue: usize) -> Self {
        Self {
            queues: BTreeMap::new(),
            next_id: 1,
            max_body,
            max_queue,
        }
    }
}

type HttpRelayTarget = adversarial::route::HttpRelayTarget;
type HttpRequestParsed = adversarial::route::HttpRequestParsed;

struct RelaySendOutcome {
    action: String,
    delivered: bool,
    error_code: Option<&'static str>,
}

#[derive(Clone, Copy)]
struct TimelineSendIngest<'a> {
    peer: &'a str,
    byte_len: usize,
    kind: &'a str,
    message_id: Option<&'a str>,
    target_device_id: Option<&'a str>,
}

#[derive(Deserialize, Serialize)]
struct InboxPullItem {
    id: String,
    data: Vec<u8>,
}

#[derive(Deserialize, Serialize)]
struct InboxPullResp {
    items: Vec<InboxPullItem>,
}

#[derive(Clone)]
struct FaultInjector {
    seed: u64,
    scenario: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FaultAction {
    Drop,
    Reorder,
}

fn channel_label_ok(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '#')
}

fn relay_trimmed_nonempty(value: Option<String>) -> Option<String> {
    let value = value?;
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

static FAULT_IDX: AtomicU64 = AtomicU64::new(0);

struct RelaySendPayloadArgs<'a> {
    to: &'a str,
    payload: Vec<u8>,
    relay: &'a str,
    injector: Option<FaultInjector>,
    pad_cfg: Option<MetaPadConfig>,
    bucket_max: Option<usize>,
    meta_seed: Option<u64>,
    receipt: Option<ReceiptKind>,
    routing_override: Option<SendRoutingTarget>,
    tui_thread: Option<&'a str>,
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

fn util_receipt_apply(
    peer: &str,
    channel: &str,
    msg_id: Option<String>,
    file_id: Option<String>,
    confirm_id: Option<String>,
) {
    if !env_bool("QSC_TEST_MODE") {
        print_error_marker("test_mode_required");
    }
    if !channel_label_ok(peer) || !channel_label_ok(channel) {
        print_error_marker("qsp_channel_invalid");
    }
    emit_cli_confirm_policy();
    match (msg_id.as_deref(), file_id.as_deref(), confirm_id.as_deref()) {
        (Some(msg), None, None) => match apply_message_peer_confirmation(peer, msg, channel) {
            Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                let dev = channel_device_marker(channel);
                emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
            }
            Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                let device = target.as_deref().or_else(|| channel_device_id(channel));
                emit_cli_delivery_state_with_device(peer, "peer_confirmed", device);
            }
            Err(code) => print_error_marker(code),
        },
        (None, Some(file), Some(confirm)) => {
            let file_id = if file == "latest" {
                latest_outbound_file_id(peer).unwrap_or_else(|code| print_error_marker(code))
            } else {
                file.to_string()
            };
            let confirm_id = if confirm == "auto" {
                file_transfer_confirm_id(peer, file_id.as_str())
                    .unwrap_or_else(|code| print_error_marker(code))
            } else {
                confirm.to_string()
            };
            match apply_file_peer_confirmation(peer, file_id.as_str(), confirm_id.as_str(), channel)
            {
                Ok((ConfirmApplyOutcome::IgnoredWrongDevice, _)) => {
                    let dev = channel_device_marker(channel);
                    emit_cli_receipt_ignored_wrong_device(peer, dev.as_str());
                }
                Ok((ConfirmApplyOutcome::Confirmed, target)) => {
                    let device = target.as_deref().or_else(|| channel_device_id(channel));
                    emit_cli_file_delivery_with_device(
                        peer,
                        "peer_confirmed",
                        file_id.as_str(),
                        device,
                    );
                }
                Err(code) => print_error_marker(code),
            }
        }
        _ => print_error_marker("receipt_apply_invalid_args"),
    }
}

struct BoundedQueue<T> {
    max: usize,
    items: VecDeque<T>,
}

impl<T> BoundedQueue<T> {
    fn new(max: usize) -> Self {
        Self {
            max,
            items: VecDeque::new(),
        }
    }

    fn push(&mut self, item: T) -> Result<(), ()> {
        if self.items.len() >= self.max {
            return Err(());
        }
        self.items.push_back(item);
        Ok(())
    }
}

fn util_queue(len: usize) {
    let mut q = BoundedQueue::new(MAX_QUEUE_LEN);
    for i in 0..len {
        if q.push(i).is_err() {
            print_error_marker("queue_limit_exceeded");
        }
    }
    print_marker("queue_limit", &[("ok", "true")]);
}

fn util_history(len: usize) {
    let mut h = BoundedQueue::new(MAX_HISTORY_LEN);
    for i in 0..len {
        if h.push(i).is_err() {
            print_error_marker("history_limit_exceeded");
        }
    }
    print_marker("history_limit", &[("ok", "true")]);
}

fn bounded_retry<F>(mut attempts: u32, mut op: F) -> Result<u32, ()>
where
    F: FnMut() -> Result<(), ()>,
{
    let mut tried = 0;
    let mut backoff = RETRY_BASE_MS;
    while attempts > 0 {
        tried += 1;
        match op() {
            Ok(()) => return Ok(tried),
            Err(()) => {
                attempts -= 1;
                if attempts == 0 {
                    return Err(());
                }
                let jitter = (tried as u64 % (RETRY_JITTER_MS + 1)).min(RETRY_JITTER_MS);
                let sleep_ms = (backoff + jitter).min(RETRY_MAX_MS);
                std::thread::sleep(Duration::from_millis(sleep_ms));
                backoff = (backoff * 2).min(RETRY_MAX_MS);
            }
        }
    }
    Err(())
}

fn util_retry(fail: u32) {
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
        Err(()) => print_error_marker("retry_limit_exceeded"),
    }
}

fn util_timeout(wait_ms: u64, timeout_ms: u64) {
    let limit = timeout_ms.clamp(1, MAX_TIMEOUT_MS);
    if wait_ms > limit {
        print_error_marker("timeout_exceeded");
    }
    let elapsed_s = wait_ms.to_string();
    print_marker("timeout_ok", &[("elapsed_ms", elapsed_s.as_str())]);
}

fn util_envelope(
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    payload_lens: Vec<usize>,
) {
    let ticks = match envelope::tick_schedule(tick_count, interval_ms, max_ticks) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let bundle = match envelope::pack_bundle(&payload_lens, max_bundle, max_count) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let ticks_s = ticks.len().to_string();
    let interval_s = interval_ms.to_string();
    let bucket_s = bundle.bucket_len.to_string();
    let total_s = bundle.total_len.to_string();
    let count_s = bundle.payload_lens.len().to_string();
    print_marker(
        "envelope_plan",
        &[
            ("ticks", ticks_s.as_str()),
            ("interval_ms", interval_s.as_str()),
            ("bucket_size", bucket_s.as_str()),
            ("bundle_len", total_s.as_str()),
            ("payload_count", count_s.as_str()),
        ],
    );
}

fn envelope_plan_ack(
    deterministic: bool,
    tick_count: usize,
    interval_ms: u64,
    max_ticks: usize,
    max_bundle: usize,
    max_count: usize,
    small_len: usize,
) {
    if !deterministic {
        print_error_marker("ack_plan_requires_deterministic");
    }
    let plan = match envelope::plan_ack(
        small_len,
        tick_count,
        interval_ms,
        max_ticks,
        max_bundle,
        max_count,
    ) {
        Ok(v) => v,
        Err(e) => print_error_marker(e.code()),
    };
    let tick = plan.ticks.first().copied().unwrap_or(0);
    let tick_s = tick.to_string();
    let bucket_s = plan.bundle.bucket_len.to_string();
    print_marker(
        "ack_plan",
        &[("size_class", bucket_s.as_str()), ("tick", tick_s.as_str())],
    );
}

fn normalize_tui_autolock_minutes(value: &str) -> Result<u64, ErrorCode> {
    let minutes = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_AUTOLOCK_MIN_MINUTES..=TUI_AUTOLOCK_MAX_MINUTES).contains(&minutes) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(minutes)
}

fn normalize_tui_poll_interval_seconds(value: &str) -> Result<u64, ErrorCode> {
    let seconds = value
        .trim()
        .parse::<u64>()
        .map_err(|_| ErrorCode::ParseFailed)?;
    if !(TUI_POLL_MIN_INTERVAL_SECONDS..=TUI_POLL_MAX_INTERVAL_SECONDS).contains(&seconds) {
        return Err(ErrorCode::ParseFailed);
    }
    Ok(seconds)
}

fn normalize_tui_poll_mode(value: &str) -> Result<TuiPollMode, ErrorCode> {
    match value.trim() {
        "adaptive" => Ok(TuiPollMode::Adaptive),
        "fixed" => Ok(TuiPollMode::Fixed),
        _ => Err(ErrorCode::ParseFailed),
    }
}

fn print_error(code: ErrorCode) -> ! {
    emit_marker("error", Some(code.as_str()), &[]);
    process::exit(1);
}

fn bool_str(v: bool) -> &'static str {
    if v {
        "true"
    } else {
        "false"
    }
}

#[cfg(unix)]
extern "C" {
    fn umask(mask: u32) -> u32;
    fn flock(fd: i32, operation: i32) -> i32;
}

fn write_doctor_export(path: &Path, report: &DoctorReport) -> Result<(), ErrorCode> {
    let dir = path.parent().ok_or(ErrorCode::IoWriteFailed)?;
    let payload = serde_json::to_vec(report).map_err(|_| ErrorCode::IoWriteFailed)?;
    let tmp = dir.join(format!(
        "{}.tmp.{}",
        path.file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("doctor"),
        process::id()
    ));
    let _ = fs::remove_file(&tmp);
    fs::create_dir_all(dir).map_err(|_| ErrorCode::IoWriteFailed)?;

    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&tmp)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.write_all(&payload)
        .map_err(|_| ErrorCode::IoWriteFailed)?;
    f.sync_all().map_err(|_| ErrorCode::IoWriteFailed)?;
    fs::rename(&tmp, path).map_err(|_| ErrorCode::IoWriteFailed)?;
    fsync_dir_best_effort(dir);
    Ok(())
}

#[cfg(test)]
mod message_state_tests {
    use super::{message_state_transition_allowed, MessageState};

    #[test]
    fn failed_state_is_terminal() {
        let err =
            message_state_transition_allowed(MessageState::Failed, MessageState::Delivered, "out")
                .expect_err("FAILED must be terminal");
        assert_eq!(err, "failed_terminal");
    }

    #[test]
    fn out_state_cannot_skip_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Created, MessageState::Delivered, "out")
                .expect_err("CREATED -> DELIVERED must reject");
        assert_eq!(err, "state_invalid_transition");
    }

    #[test]
    fn in_state_cannot_transition_to_delivered() {
        let err =
            message_state_transition_allowed(MessageState::Received, MessageState::Delivered, "in")
                .expect_err("RECEIVED -> DELIVERED must reject for inbound timeline");
        assert_eq!(err, "state_invalid_transition");
    }
}

#[cfg(test)]
mod tui_perf_tests {
    use super::{
        tui_next_poll_timeout_ms, TuiConfig, TuiPollMode, TuiState, TUI_POLL_MIN_INTERVAL_SECONDS,
    };

    fn test_tui_config(relay: bool) -> TuiConfig {
        TuiConfig {
            relay: if relay {
                Some("http://127.0.0.1:9".to_string())
            } else {
                None
            },
            token_file: None,
            seed: 0,
            scenario: "direct".to_string(),
        }
    }

    #[test]
    fn interactive_poll_timeout_is_never_zero() {
        assert!(
            tui_next_poll_timeout_ms() >= 50,
            "interactive poll timeout must be clamped to prevent busy loops"
        );
    }

    #[test]
    fn idle_clock_advance_without_state_change_does_not_request_redraw() {
        let mut state = TuiState::new(test_tui_config(false));
        state.vault_locked = false;
        state.status.locked = "UNLOCKED";
        state.needs_redraw = false;
        for _ in 0..32 {
            state.headless_advance_clock(100);
            assert!(
                !state.needs_redraw,
                "idle clock advancement must not schedule redraws without state changes"
            );
        }
    }

    #[test]
    fn fixed_poll_due_seed_respects_minimum_interval_clamp() {
        let mut state = TuiState::new(test_tui_config(true));
        state.vault_locked = false;
        state.status.locked = "UNLOCKED";
        state.poll_mode = TuiPollMode::Fixed;
        state.poll_interval_seconds = TUI_POLL_MIN_INTERVAL_SECONDS.saturating_sub(1);
        state.poll_next_due_ms = None;
        assert!(
            !state.maybe_run_fixed_poll(0),
            "initial scheduling should seed next due timestamp without immediate tick"
        );
        assert_eq!(
            state.poll_next_due_ms,
            Some(TUI_POLL_MIN_INTERVAL_SECONDS.saturating_mul(1_000)),
            "fixed polling due timestamp must respect min interval clamp"
        );
    }
}

#[cfg(test)]
mod relay_url_policy_tests {
    use super::normalize_relay_endpoint;
    use crate::adversarial::route::QSC_ERR_RELAY_TLS_REQUIRED;

    #[test]
    fn relay_url_policy_allow_deny_matrix() {
        assert!(normalize_relay_endpoint("http://localhost:8080").is_ok());
        assert!(normalize_relay_endpoint("http://127.0.0.1:8080").is_ok());
        assert!(normalize_relay_endpoint("http://[::1]:8080").is_ok());
        assert!(normalize_relay_endpoint("https://example.com").is_ok());

        assert_eq!(
            normalize_relay_endpoint("http://example.com")
                .expect_err("non-loopback http must reject"),
            QSC_ERR_RELAY_TLS_REQUIRED
        );
        assert_eq!(
            normalize_relay_endpoint("http://192.168.1.10")
                .expect_err("non-loopback LAN http must reject"),
            QSC_ERR_RELAY_TLS_REQUIRED
        );
        assert_eq!(
            normalize_relay_endpoint("tcp://example.com")
                .expect_err("non-http scheme must reject deterministically"),
            "relay_endpoint_invalid_scheme"
        );
    }
}
