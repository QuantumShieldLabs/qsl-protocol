use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const ENDPOINT: &str = "https://relay.example.test:8443";

fn unique_cfg_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("NO_COLOR", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn has_long_hex(text: &str, min_len: usize) -> bool {
    let mut run = 0usize;
    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            run += 1;
            if run >= min_len {
                return true;
            }
        } else {
            run = 0;
        }
    }
    false
}

fn key_script_for(text: &str) -> String {
    let mut out = String::new();
    for ch in text.chars() {
        if ch == ' ' {
            out.push_str("/key space;");
        } else {
            out.push_str(&format!("/key {};", ch));
        }
    }
    out
}

#[derive(Debug, Clone, Copy)]
struct PerfSnapshot {
    kdf: u64,
    reads: u64,
    decrypts: u64,
}

fn parse_counter(line: &str, key: &str) -> u64 {
    let token = format!("{key}=");
    line.split_whitespace()
        .find_map(|part| part.strip_prefix(&token))
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(0)
}

fn snapshot_by_tag(out: &str, tag: &str) -> PerfSnapshot {
    let needle = format!("event=tui_perf tag={tag} ");
    let line = out
        .lines()
        .find(|line| line.contains(&needle))
        .unwrap_or_else(|| panic!("missing perf snapshot tag={tag}: {out}"));
    PerfSnapshot {
        kdf: parse_counter(line, "kdf"),
        reads: parse_counter(line, "reads"),
        decrypts: parse_counter(line, "decrypts"),
    }
}

#[test]
fn relay_show_routes_to_system_relay_and_focus_nav() {
    let cfg = unique_cfg_dir("na0148_relay_show_route");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/inspector settings;/relay show;/exit",
    );
    assert!(
        out.contains("event=tui_inspector pane=relay")
            && out.contains("event=tui_focus_home pane=nav")
            && out.contains("event=tui_nav_render")
            && out.contains("selected_label=relay"),
        "relay show should route to system relay and focus nav: {}",
        out
    );
}

#[test]
fn relay_set_persists_redacted_and_does_not_echo_endpoint() {
    let cfg = unique_cfg_dir("na0148_relay_set_redacted");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;/relay show;/exit",
    );
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=relay_set_endpoint")
            && out.contains("event=tui_relay_view")
            && out.contains("configured=true")
            && out.contains("endpoint=<redacted>"),
        "relay set should persist a redacted endpoint marker: {}",
        out
    );
    assert!(
        !out.contains(ENDPOINT),
        "raw endpoint must never appear in output/results: {}",
        out
    );

    let reload = run_headless(&cfg, "/unlock StrongPassphrase1234;/relay show;/exit");
    assert!(
        reload.contains("event=tui_relay_view")
            && reload.contains("configured=true")
            && reload.contains("endpoint=<redacted>")
            && !reload.contains(ENDPOINT),
        "relay endpoint should reload from vault in redacted form: {}",
        reload
    );
}

#[test]
fn relay_clear_resets_state() {
    let cfg = unique_cfg_dir("na0148_relay_clear");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;/relay clear;/relay show;/exit",
    );
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=relay_clear")
            && out.contains("event=tui_relay_view")
            && out.contains("configured=false")
            && out.contains("endpoint=<redacted>"),
        "relay clear should reset relay state to unset: {}",
        out
    );
}

#[test]
fn locked_relay_commands_reject_deterministically() {
    let cfg = unique_cfg_dir("na0148_relay_locked_reject");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;/lock;/relay set endpoint https://example.invalid;/relay clear;/relay test;/exit",
    );
    let rejects = out
        .lines()
        .filter(|line| line.contains("event=tui_locked_cmd_reject") && line.contains("cmd=relay"))
        .count();
    assert!(
        rejects >= 3,
        "locked relay commands should reject deterministically without mutating state: {}",
        out
    );
}

#[test]
fn account_destroy_clears_relay_config_and_resets_defaults() {
    let cfg = unique_cfg_dir("na0148_relay_destroy_reset");
    ensure_dir_700(&cfg);
    let mut script = String::from(
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;/account destroy;",
    );
    script.push_str(&key_script_for("StrongPassphrase1234"));
    script.push_str("/key enter;/key Y;/key enter;");
    script.push_str("/init NewUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay show;/exit");

    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_lock_state locked=LOCKED reason=account_destroy")
            && out.contains("event=tui_relay_view")
            && out.contains("configured=false")
            && out.contains("endpoint=<redacted>"),
        "account destroy should wipe relay config and restore unset defaults after re-init: {}",
        out
    );
}

#[test]
fn relay_nav_does_not_trigger_vault_work_on_idle() {
    let cfg = unique_cfg_dir("na0148_relay_perf_guard");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint https://relay.example.test:8443;perf snapshot baseline;/inspector relay;wait 200;wait 200;wait 200;perf snapshot after_nav;/exit",
    );
    let baseline = snapshot_by_tag(&out, "baseline");
    let after = snapshot_by_tag(&out, "after_nav");
    assert_eq!(
        after.kdf, baseline.kdf,
        "relay nav/idle must not invoke KDF in render loop: {}",
        out
    );
    assert_eq!(
        after.reads, baseline.reads,
        "relay nav/idle must not read vault files in render loop: {}",
        out
    );
    assert_eq!(
        after.decrypts, baseline.decrypts,
        "relay nav/idle must not decrypt vault payload in render loop: {}",
        out
    );
}

#[test]
fn relay_token_file_is_redacted_and_status_is_deterministic() {
    let cfg = unique_cfg_dir("na0177_relay_token_file_redacted");
    ensure_dir_700(&cfg);
    let token_file = cfg.join("relay_token.txt");
    std::fs::write(&token_file, "demo-token-value\n").expect("write token file");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&token_file, std::fs::Permissions::from_mode(0o600))
            .expect("chmod 600");
    }
    let token_file_s = token_file.to_string_lossy().to_string();
    let script = format!(
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;/unlock StrongPassphrase1234;/relay set endpoint {ENDPOINT};/relay set token-file {token_file_s};/relay test;/inspector relay;/inspector status;/exit"
    );
    let out = run_headless(&cfg, script.as_str());
    assert!(
        out.contains("event=tui_cmd_result kind=ok command=relay_set_token-file")
            && out.contains("event=tui_relay_view")
            && out.contains("token_file=<redacted>")
            && out.contains("token_file_state=<redacted>")
            && out.contains("token_file_perms=<redacted>")
            && out.contains("event=tui_status_view")
            && out.contains("setup_token_file=<redacted>"),
        "expected token-file setup markers with redaction and deterministic state: {out}"
    );
    assert!(
        !out.contains(token_file_s.as_str()),
        "token file path must be redacted from markers/output: {out}"
    );
    assert!(!out.contains("/v1/"), "must not leak relay URI path: {out}");
    assert!(
        !has_long_hex(&out, 32),
        "must not leak long hex secrets: {out}"
    );
}

#[test]
fn trust_pin_flow_blocks_mismatch_then_allows_send_after_confirm() {
    let cfg = unique_cfg_dir("na0177_trust_pin_flow");
    ensure_dir_700(&cfg);
    let script = "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I UNDERSTAND;\
/unlock StrongPassphrase1234;\
/contacts add peer-0 ABCD-EFGH-JKMP-QRST-V;\
/verify peer-0 ABCD-EFGH-JKMP-QRST-W;\
/send;\
/trust pin peer-0 confirm;\
/send;\
/exit";
    let out = run_headless(&cfg, script);
    assert!(
        out.contains("event=tui_contacts_verify code=verification_mismatch")
            && out.contains("label=peer-0")
            && out.contains("status=CHANGED")
            && out.contains("QSC_TUI_SEND_BLOCKED reason=trust_not_pinned peer=peer-0")
            && out.contains("event=tui_send_blocked code=trust_not_pinned reason=trust_not_pinned")
            && out.contains("event=tui_trust_pin")
            && out.contains("label=peer-0")
            && out.contains("status=PINNED")
            && out.contains("event=tui_send_blocked reason=explicit_only_no_transport"),
        "expected mismatch block and post-pin transition out of mismatch block: {out}"
    );
    assert!(!out.contains("/v1/"), "must not leak relay URI path: {out}");
    assert!(
        !has_long_hex(&out, 32),
        "must not leak long hex secrets: {out}"
    );
}
