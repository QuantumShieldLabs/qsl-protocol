use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn self_identity_path(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{}.json", label))
}

fn add_contact(cfg: &Path, peer: &str, fp: &str) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(["contacts", "add", "--label", peer, "--fp", fp, "--verify"])
        .output()
        .expect("contacts add");
    assert!(
        out.status.success(),
        "contacts add failed: {}",
        output_str(&out)
    );
}

fn output_str(out: &std::process::Output) -> String {
    let mut s = String::new();
    s.push_str(&String::from_utf8_lossy(&out.stdout));
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn init_mock_vault(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(["vault", "init", "--non-interactive", "--key-source", "mock"])
        .output()
        .expect("vault init");
    assert!(
        out.status.success(),
        "vault init failed: {}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn assert_no_secrets(s: &str) {
    let needle = [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ];
    for n in needle.iter() {
        assert!(!s.contains(n), "unexpected secret marker: {n}");
    }
}

#[test]
fn identity_rotate_requires_confirm() {
    let base = safe_test_root().join(format!("na0102_identity_rotate_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_mock_vault(&cfg);

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "rotate", "--as", "self"])
        .output()
        .expect("identity rotate");
    assert!(!out.status.success());
    let s = output_str(&out);
    assert!(s.contains("identity_rotate"));
    assert!(s.contains("confirm_required"));
    assert!(!self_identity_path(&cfg, "self").exists());
    assert_no_secrets(&s);
}

#[test]
fn identity_show_and_rotate_confirm() {
    let base = safe_test_root().join(format!("na0102_identity_show_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_mock_vault(&cfg);

    let out_rotate = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "rotate", "--as", "self", "--confirm"])
        .output()
        .expect("identity rotate confirm");
    assert!(out_rotate.status.success());
    let s_rotate = output_str(&out_rotate);
    assert!(s_rotate.contains("identity_rotate"));
    assert!(s_rotate.contains("ok=true"));
    assert!(self_identity_path(&cfg, "self").exists());
    assert_no_secrets(&s_rotate);

    let out_show = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "show", "--as", "self"])
        .output()
        .expect("identity show");
    assert!(out_show.status.success());
    let s_show = output_str(&out_show);
    assert!(s_show.contains("identity_show"));
    assert!(s_show.contains("identity_fp=QSCFP-"));
    assert_no_secrets(&s_show);
}

#[test]
fn peers_list_deterministic_order() {
    let base = safe_test_root().join(format!("na0102_peers_list_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    init_mock_vault(&cfg);

    add_contact(&cfg, "bob", "QSCFP-bbbbbbbbbbbbbbbb");
    add_contact(&cfg, "alice", "QSCFP-aaaaaaaaaaaaaaaa");

    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["peers", "list"])
        .output()
        .expect("peers list");
    assert!(out.status.success());
    let s = output_str(&out);
    assert!(s.contains("peers_list"));
    assert!(s.contains("peer_item"));
    let alice_idx = s.find("peer=alice").unwrap();
    let bob_idx = s.find("peer=bob").unwrap();
    assert!(alice_idx < bob_idx, "expected alice before bob");
    assert_no_secrets(&s);
}
