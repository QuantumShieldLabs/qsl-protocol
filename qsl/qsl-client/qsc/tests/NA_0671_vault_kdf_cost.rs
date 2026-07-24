// NA-0671 (D-1298, directive 607): MEASURE the Argon2id (vault-KDF) derivations that a
// single relay SEND, PULL, and ACK each perform. This harness FIXES NOTHING — it counts.
//
// ============================================================================
// WHY THIS IS IN-PROCESS (the whole reason it is not a 2-line patch to an e2e test)
// ============================================================================
// `qsc::vault::perf_snapshot()` reads PROCESS-GLOBAL atomic counters. Every existing
// full-relay-op e2e (NA_0640_full_stack_e2e, same_host_client_to_client_e2e,
// receive_e2e, NA_0644_ack_client) drives the qsc CLI as SPAWNED SUBPROCESSES, so each
// `qsc send`/`receive` is a CHILD whose counters die with it and are unreadable from the
// parent. The MEASURED operation must therefore call the qsc LIBRARY functions
// (`transport::send_execute`, `transport::receive_execute`) inside THIS process. The
// SETUP (identities, trust, handshake, warm-up) is allowed to use the CLI as
// subprocesses — its derivations are not measured; only the operation is.
//
// F2 (scaffold reuse) — SETTLED AGAINST REALITY: NO existing scaffold drives a full
// in-process send/pull/ack. The one in-process passphrase-vault scaffold
// (NA_0649_gui_surface.rs) exercises only the vault + identity surface, never a relay
// operation. So this is the "standalone, none fit" case F2 anticipated: this file reuses
// `common`'s real-relay + vault-init helpers but stands up its own send/pull/ack driver.
// It is well past the ~30-line "reuse" estimate and lands in the standalone class.
//
// ============================================================================
// THE COUNTER HAZARD (OBS-3) — why the raw count is not "Argon2id cost"
// ============================================================================
// `PERF_KDF_CALLS.fetch_add(1)` sits at the TOP of `derive_runtime_key`, BEFORE the
// `match env.key_source`. It therefore counts VAULT OPENS (entries to derive_runtime_key),
// NOT Argon2id hashes: a keychain vault (key_source==2) or a locked early-return bumps it
// doing ZERO Argon2 work. The count == the Argon2id count ONLY on a key_source==1
// (passphrase) vault that is UNLOCKED so every entry reaches the hash. This harness
// measures exactly that vault, and cross-checks EVERY delta against wall-clock: a counted
// "KDF call" that cost far less than one Argon2id (~19.4 ms release) was a no-op and is
// excluded, not averaged in. IF COUNTER AND TIMING DISAGREE, THE TIMING GOVERNS.
//
// Run it LOCALLY, under --release, and read the printed table (NOT a CI green — the qsc
// full suite SKIPS on PRs per NA-0633, and a green is not the evidence; the numbers are):
//   cargo test -p qsc --release --test NA_0671_vault_kdf_cost -- --ignored --nocapture

mod common;

use qsc::cmd::{AckMode, SendTransport};
use qsc::output::{marker_queue, set_marker_routing, MarkerRouting};
use qsc::relay::SendExecuteArgs;
use qsc::transport::{receive_execute, send_execute};
use qsc::vault::{
    perf_snapshot, secret_get, secret_set, set_process_passphrase, unlock_with_passphrase,
};
use qsc::{set_vault_unlocked, ReceiveArgs};

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::Instant;

const PASS: &str = common::TEST_MOCK_VAULT_PASSPHRASE;
const ALICE_MAILBOX: &str = "na0671_alice_mailbox_token_abcd12";
const BOB_MAILBOX: &str = "na0671_bob_mailbox_token_wxyz567";
const RELAY_BEARER: &str = "na0671_relay_bearer_token_0123456789abcdef";

// Store keys that are pub in qsc::store; timeline/contacts keys are pub(crate) so we pass
// their literal string names straight to secret_get/secret_set (which take &str).
const RELAY_TOKEN_KEY: &str = "tui.relay.token"; // qsc::store::TUI_RELAY_TOKEN_SECRET_KEY
const INBOX_TOKEN_KEY: &str = "tui.relay.inbox_token"; // seeded at vault init — always present
const TIMELINE_KEY: &str = "timeline.json";

// The two per-derivation cost points the directive supplies. 41.1 is a MULTIPLY-BY
// CONSTANT for a second column only; NO KDF parameter is changed in this lane.
const MS_PER_DERIVATION_CURRENT: f64 = 19.4; // m=19456,t=2,p=1 (the shipped params)
const MS_PER_DERIVATION_OWASP: f64 = 41.1; // OWASP minimum — arithmetic only

// ---------------------------------------------------------------------------
// perf bracket
// ---------------------------------------------------------------------------
#[derive(Clone, Copy, Debug)]
struct Delta {
    kdf: u64,
    file_reads: u64,
    decrypts: u64,
    encrypt_writes: u64,
    wall_ms: f64,
}

fn bracket<F: FnOnce()>(f: F) -> Delta {
    let (k0, r0, d0, w0) = perf_snapshot();
    let t = Instant::now();
    f();
    let wall_ms = t.elapsed().as_secs_f64() * 1000.0;
    let (k1, r1, d1, w1) = perf_snapshot();
    Delta {
        kdf: k1 - k0,
        file_reads: r1 - r0,
        decrypts: d1 - d0,
        encrypt_writes: w1 - w0,
        wall_ms,
    }
}

impl Delta {
    // wall_ms / 19.4  — the consistency check. Should track `kdf` on a passphrase vault.
    fn implied_kdf(&self) -> f64 {
        self.wall_ms / MS_PER_DERIVATION_CURRENT
    }
    fn ms_at(&self, per: f64) -> f64 {
        self.kdf as f64 * per
    }
}

fn print_row(label: &str, d: &Delta) {
    eprintln!(
        "{label:<34} kdf={:<3} reads={:<3} decr={:<3} enc_w={:<3} wall={:>8.2}ms  implied_kdf(wall/19.4)={:>5.2}  @19.4={:>7.1}ms @41.1={:>7.1}ms",
        d.kdf,
        d.file_reads,
        d.decrypts,
        d.encrypt_writes,
        d.wall_ms,
        d.implied_kdf(),
        d.ms_at(MS_PER_DERIVATION_CURRENT),
        d.ms_at(MS_PER_DERIVATION_OWASP),
    );
}

// ---------------------------------------------------------------------------
// subprocess setup (NOT measured) — mirrors NA_0640's proven real-relay pattern
// ---------------------------------------------------------------------------
fn output_text(out: &Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn safe_test_root(tag: &str) -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp").join(tag);
    create_dir_700(&root);
    root
}

fn qsc_cmd(cfg: &Path, bearer: Option<&str>) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    if let Some(token) = bearer {
        cmd.env("QSC_RELAY_TOKEN", token);
    }
    cmd
}

fn run_ok(cfg: &Path, bearer: Option<&str>, args: &[&str]) -> String {
    let out = qsc_cmd(cfg, bearer).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed: {args:?}\n{text}");
    text
}

fn init_identity(cfg: &Path, bearer: Option<&str>) -> String {
    common::init_mock_vault(cfg);
    run_ok(cfg, bearer, &["identity", "rotate", "--confirm"]);
    let show = run_ok(cfg, bearer, &["identity", "show"]);
    show.lines()
        .find_map(|l| l.strip_prefix("identity_fp="))
        .unwrap_or_else(|| panic!("missing identity_fp: {show}"))
        .to_string()
}

fn add_contact(cfg: &Path, bearer: Option<&str>, label: &str, fp: &str, route_token: &str) {
    run_ok(
        cfg,
        bearer,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--route-token",
            route_token,
        ],
    );
    let list = run_ok(
        cfg,
        bearer,
        &["contacts", "device", "list", "--label", label],
    );
    let device = list
        .lines()
        .find_map(|l| l.split_whitespace().find_map(|t| t.strip_prefix("device=")))
        .unwrap_or_else(|| panic!("missing device: {list}"));
    run_ok(
        cfg,
        bearer,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device,
            "--confirm",
        ],
    );
}

/// Full authenticated pair against the real relay (NA_0640 aligned-label convention:
/// both sides label the peer "bob" so `receive --from bob` matches locally).
fn setup_pair(base: &Path, bearer: Option<&str>) -> (PathBuf, PathBuf, PathBuf, PathBuf) {
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    for p in [&alice_cfg, &bob_cfg, &alice_out, &bob_out] {
        create_dir_700(p);
    }
    let alice_fp = init_identity(&alice_cfg, bearer);
    let bob_fp = init_identity(&bob_cfg, bearer);
    run_ok(
        &alice_cfg,
        bearer,
        &["relay", "inbox-set", "--token", ALICE_MAILBOX],
    );
    run_ok(
        &bob_cfg,
        bearer,
        &["relay", "inbox-set", "--token", BOB_MAILBOX],
    );
    add_contact(&alice_cfg, bearer, "bob", bob_fp.as_str(), BOB_MAILBOX);
    add_contact(&bob_cfg, bearer, "bob", alice_fp.as_str(), ALICE_MAILBOX);
    (alice_cfg, bob_cfg, alice_out, bob_out)
}

fn subprocess_send(cfg: &Path, relay: &str, bearer: Option<&str>, base: &Path, tag: &str) {
    let msg = base.join(format!("{tag}.txt"));
    fs::write(
        &msg,
        format!("na0671 warmup/stock message {tag}").as_bytes(),
    )
    .expect("write msg");
    run_ok(
        cfg,
        bearer,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ],
    );
}

fn subprocess_receive(cfg: &Path, relay: &str, bearer: Option<&str>, mailbox: &str, out: &Path) {
    // Warm-up receive; tolerate an empty inbox (nothing to warm) without failing the run.
    let _ = qsc_cmd(cfg, bearer)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .expect("warmup receive");
}

// ---------------------------------------------------------------------------
// in-process operation drivers (MEASURED)
// ---------------------------------------------------------------------------
fn drain_markers() -> Vec<String> {
    marker_queue()
        .lock()
        .unwrap_or_else(|p| p.into_inner())
        .drain(..)
        .collect()
}

fn switch_and_unlock(cfg: &Path) {
    std::env::set_var("QSC_CONFIG_DIR", cfg);
    set_process_passphrase(None);
    set_vault_unlocked(false);
    unlock_with_passphrase(PASS).expect("in-process unlock");
    set_vault_unlocked(true);
    drain_markers();
}

fn send_once(relay: &str, base: &Path, tag: &str) {
    let msg = base.join(format!("{tag}.bin"));
    fs::write(&msg, format!("na0671 measured send {tag}").as_bytes()).expect("write msg");
    let args = SendExecuteArgs {
        transport: Some(SendTransport::Relay),
        relay: Some(relay.to_string()),
        to: Some("bob".to_string()),
        file: Some(msg),
        pad_to: None,
        pad_bucket: None,
        bucket_max: None,
        meta_seed: None,
        receipt: None,
    };
    match send_execute(args) {
        Ok(()) => {}
        Err(e) => panic!(
            "in-process send failed: {e:?}\nmarkers={:?}",
            drain_markers()
        ),
    }
}

fn receive_once(relay: &str, mailbox: &str, out: &Path, ack: AckMode) {
    let args = ReceiveArgs {
        transport: Some(SendTransport::Relay),
        relay: Some(relay.to_string()),
        legacy_receive_mode: None,
        ack_mode: Some(ack),
        attachment_service: None,
        from: Some("bob".to_string()),
        mailbox: Some(mailbox.to_string()),
        max: Some(1),
        max_file_size: None,
        max_file_chunks: None,
        out: Some(out.to_path_buf()),
        deterministic_meta: false,
        interval_ms: None,
        poll_interval_ms: None,
        poll_ticks: None,
        batch_max_count: None,
        poll_max_per_tick: None,
        bucket_max: None,
        meta_seed: None,
        emit_receipts: None,
        receipt_mode: None,
        receipt_batch_window_ms: None,
        receipt_jitter_ms: None,
        file_confirm_mode: None,
    };
    match receive_execute(args) {
        Ok(()) => {}
        Err(e) => panic!(
            "in-process receive failed: {e:?}\nmarkers={:?}",
            drain_markers()
        ),
    }
    drain_markers();
}

// ---------------------------------------------------------------------------
// the measurement
// ---------------------------------------------------------------------------
#[derive(Clone, Copy)]
enum AuthMode {
    Open,
    Token,
}

impl AuthMode {
    fn label(&self) -> &'static str {
        match self {
            AuthMode::Open => "open",
            AuthMode::Token => "token",
        }
    }
    fn bearer(&self) -> Option<&'static str> {
        match self {
            AuthMode::Open => None,
            AuthMode::Token => Some(RELAY_BEARER),
        }
    }
}

fn measure_mode(mode: AuthMode) {
    let bearer = mode.bearer();
    let relay = common::start_qsl_server_with_store(2 * 1024 * 1024, 512, bearer, 60);
    let relay_url = relay.base_url().to_string();
    let base = safe_test_root(&format!("na0671_{}", mode.label()));
    let (alice_cfg, bob_cfg, alice_out, bob_out) = setup_pair(&base, bearer);

    // Warm-up to steady state (subprocess): one full round-trip, then STOCK Bob's mailbox
    // so every in-process pull/ack below finds a real message to unpack.
    subprocess_send(&alice_cfg, &relay_url, bearer, &base, "warmup");
    subprocess_receive(&bob_cfg, &relay_url, bearer, BOB_MAILBOX, &bob_out);
    subprocess_receive(&alice_cfg, &relay_url, bearer, ALICE_MAILBOX, &alice_out);
    for i in 0..8 {
        subprocess_send(&alice_cfg, &relay_url, bearer, &base, &format!("stock{i}"));
    }

    eprintln!(
        "\n================ AUTH MODE: {} (relay {}) ================",
        mode.label(),
        relay_url
    );

    // ---- as ALICE: positive control, amortisation, timeline slice, SEND ----
    // For a clean in-process auth story, the token is resolved from the VAULT (the
    // persistent `relay token-set` config), so ensure no QSC_RELAY_TOKEN env leaks in.
    std::env::remove_var("QSC_RELAY_TOKEN");
    switch_and_unlock(&alice_cfg);
    if let AuthMode::Token = mode {
        // vault-stored bearer: exercises the account-secret probe (returns Some).
        secret_set(RELAY_TOKEN_KEY, RELAY_BEARER).expect("store relay token in vault");
    }

    eprintln!("-- positive control + amortisation (lone secret_get on a PRESENT key) --");
    let pc1 = bracket(|| {
        let _ = secret_get(INBOX_TOKEN_KEY).expect("pc get");
    });
    let pc2 = bracket(|| {
        let _ = secret_get(INBOX_TOKEN_KEY).expect("pc get 2");
    });
    print_row("positive_control_get#1", &pc1);
    print_row("positive_control_get#2(warm)", &pc2);

    eprintln!("-- timeline slice (secret_get + secret_set on timeline.json) --");
    let tl = bracket(|| {
        let cur = secret_get(TIMELINE_KEY).ok().flatten().unwrap_or_default();
        let _ = secret_set(TIMELINE_KEY, &cur).expect("tl set");
    });
    print_row("timeline_append_slice(get+set)", &tl);

    eprintln!("-- SEND (in-process send_execute) --");
    let send_cold = bracket(|| send_once(&relay_url, &base, "measured_send_cold"));
    let send_warm = bracket(|| send_once(&relay_url, &base, "measured_send_warm"));
    print_row(&format!("SEND cold [{}]", mode.label()), &send_cold);
    print_row(&format!("SEND warm [{}]", mode.label()), &send_warm);

    if let AuthMode::Token = mode {
        // ENV-token variant: QSC_RELAY_TOKEN present -> the token vault probe is
        // SHORT-CIRCUITED (env wins before any vault access). Expect one FEWER derivation
        // than the vault-token SEND above.
        std::env::set_var("QSC_RELAY_TOKEN", RELAY_BEARER);
        let send_env = bracket(|| send_once(&relay_url, &base, "measured_send_env"));
        print_row("SEND [token via ENV, vault probe skipped]", &send_env);
        std::env::remove_var("QSC_RELAY_TOKEN");
    }

    // ---- as BOB: PULL (legacy) and ACK (lease) ----
    switch_and_unlock(&bob_cfg);
    if let AuthMode::Token = mode {
        secret_set(RELAY_TOKEN_KEY, RELAY_BEARER).expect("store relay token in bob vault");
    }

    eprintln!("-- PULL (in-process receive_execute, ack_mode=Legacy) --");
    let pull_cold = bracket(|| receive_once(&relay_url, BOB_MAILBOX, &bob_out, AckMode::Legacy));
    let pull_warm = bracket(|| receive_once(&relay_url, BOB_MAILBOX, &bob_out, AckMode::Legacy));
    print_row(&format!("PULL cold [{}]", mode.label()), &pull_cold);
    print_row(&format!("PULL warm [{}]", mode.label()), &pull_warm);

    eprintln!("-- ACK (in-process receive_execute, ack_mode=Lease = pull + ack) --");
    let ack_cold = bracket(|| receive_once(&relay_url, BOB_MAILBOX, &bob_out, AckMode::Lease));
    let ack_warm = bracket(|| receive_once(&relay_url, BOB_MAILBOX, &bob_out, AckMode::Lease));
    print_row(&format!("ACK(lease) cold [{}]", mode.label()), &ack_cold);
    print_row(&format!("ACK(lease) warm [{}]", mode.label()), &ack_warm);
    eprintln!(
        "   marginal ACK vs PULL (lease - legacy): {} derivations",
        ack_cold.kdf as i64 - pull_cold.kdf as i64
    );

    // keep the relay alive until here
    drop(relay);
}

#[test]
#[ignore = "measurement harness: run locally under --release with --ignored --nocapture"]
fn na0671_measure_vault_kdf_cost() {
    std::env::set_var("QSC_DISABLE_KEYCHAIN", "1");
    std::env::set_var("QSC_QSP_SEED", "1");
    std::env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    std::env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    set_marker_routing(MarkerRouting::InApp);

    eprintln!("\n########## NA-0671 vault-KDF derivations per relay op ##########");
    eprintln!(
        "profile: {} (cargo cfg)",
        if cfg!(debug_assertions) {
            "DEBUG"
        } else {
            "RELEASE"
        }
    );
    eprintln!("vault: key_source==1 (passphrase), unlocked. Counter = vault OPENS, cross-checked vs wall-clock.");

    measure_mode(AuthMode::Open);
    measure_mode(AuthMode::Token);

    eprintln!("\n########## end NA-0671 ##########\n");
}
