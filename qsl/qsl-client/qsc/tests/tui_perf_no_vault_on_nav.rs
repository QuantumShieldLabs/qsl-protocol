use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

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
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_DISABLE_KEYCHAIN", "1")
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

#[derive(Debug, Clone, Copy)]
struct PerfSnapshot {
    kdf: u64,
    reads: u64,
    decrypts: u64,
    writes: u64,
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
        writes: parse_counter(line, "writes"),
    }
}

#[test]
fn nav_and_idle_do_not_trigger_vault_kdf_or_decrypt_work() {
    let cfg = unique_cfg_dir("na0142_perf_nav_idle");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 Y;\
/unlock StrongPassphrase1234;\
/contacts add peer-1 F00DBABE;\
perf snapshot baseline;\
/inspector account;/inspector settings;/inspector contacts;\
/key down;/key up;/key down;\
wait 200;wait 200;wait 200;\
perf snapshot after_nav;/exit",
    );
    let baseline = snapshot_by_tag(&out, "baseline");
    let after_nav = snapshot_by_tag(&out, "after_nav");
    assert_eq!(
        after_nav.kdf, baseline.kdf,
        "navigation/idle must not invoke KDF: {out}"
    );
    assert_eq!(
        after_nav.reads, baseline.reads,
        "navigation/idle must not read vault files: {out}"
    );
    assert_eq!(
        after_nav.decrypts, baseline.decrypts,
        "navigation/idle must not decrypt vault payload: {out}"
    );
}

#[test]
fn explicit_mutation_updates_writes_without_kdf_or_decrypt_regression() {
    let cfg = unique_cfg_dir("na0142_perf_mutation");
    ensure_dir_700(&cfg);
    let out = run_headless(
        &cfg,
        "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 Y;\
/unlock StrongPassphrase1234;\
perf snapshot baseline;\
/autolock set 11;\
perf snapshot after_set;/exit",
    );
    let baseline = snapshot_by_tag(&out, "baseline");
    let after_set = snapshot_by_tag(&out, "after_set");
    assert_eq!(
        after_set.kdf, baseline.kdf,
        "explicit settings mutation must not invoke additional KDF: {out}"
    );
    assert_eq!(
        after_set.reads, baseline.reads,
        "explicit settings mutation should use in-memory session cache: {out}"
    );
    assert_eq!(
        after_set.decrypts, baseline.decrypts,
        "explicit settings mutation must not require extra decrypts: {out}"
    );
    assert!(
        after_set.writes > baseline.writes,
        "explicit settings mutation should persist at least one write: {out}"
    );
}
