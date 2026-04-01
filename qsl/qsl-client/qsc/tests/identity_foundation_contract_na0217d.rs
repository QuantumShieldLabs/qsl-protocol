use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn absolute_test_root(tag: &str) -> PathBuf {
    let root = common::unique_test_root(tag);
    if root.is_absolute() {
        root
    } else {
        env::current_dir().expect("cwd").join(root)
    }
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

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(args)
        .output()
        .expect("qsc command")
}

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn verification_code_from_fingerprint(fingerprint: &str) -> String {
    const CROCKFORD: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    let mut chars = fingerprint
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .map(|ch| ch.to_ascii_uppercase())
        .collect::<Vec<char>>();
    while chars.len() < 16 {
        chars.push('0');
    }
    let code = chars.into_iter().take(16).collect::<String>();
    let checksum_idx = code
        .bytes()
        .fold(0u32, |acc, byte| acc.saturating_add(byte as u32))
        % 32;
    let checksum = CROCKFORD[checksum_idx as usize] as char;
    format!(
        "{}-{}-{}-{}-{}",
        &code[0..4],
        &code[4..8],
        &code[8..12],
        &code[12..16],
        checksum
    )
}

#[test]
fn verification_code_pin_preserves_handshake_contract() {
    let base = absolute_test_root(&format!("na0217d_pin_code_{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    ensure_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);

    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_add_with_route(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let out_rotate = run_qsc(
        &alice_cfg,
        &["identity", "rotate", "--as", "alice", "--confirm"],
    );
    assert!(
        out_rotate.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out_rotate.stdout),
        String::from_utf8_lossy(&out_rotate.stderr)
    );

    let out_show = run_qsc(&alice_cfg, &["identity", "show", "--as", "alice"]);
    assert!(out_show.status.success());
    let show_text = String::from_utf8_lossy(&out_show.stdout).to_string()
        + &String::from_utf8_lossy(&out_show.stderr);
    let alice_fp = show_text
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("identity fp");
    let alice_code = verification_code_from_fingerprint(alice_fp);

    let out_add = run_qsc(
        &bob_cfg,
        &[
            "contacts",
            "add",
            "--label",
            "alice",
            "--fp",
            alice_code.as_str(),
            "--verify",
        ],
    );
    assert!(
        out_add.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out_add.stdout),
        String::from_utf8_lossy(&out_add.stderr)
    );
    contacts_add_with_route(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);

    let server = common::start_inbox_server(1024 * 1024, 16);
    let relay = server.base_url().to_string();

    for (cfg, args) in [
        (
            &alice_cfg,
            vec![
                "handshake",
                "init",
                "--as",
                "alice",
                "--peer",
                "bob",
                "--relay",
                relay.as_str(),
            ],
        ),
        (
            &bob_cfg,
            vec![
                "handshake",
                "poll",
                "--as",
                "bob",
                "--peer",
                "alice",
                "--relay",
                relay.as_str(),
                "--max",
                "4",
            ],
        ),
        (
            &alice_cfg,
            vec![
                "handshake",
                "poll",
                "--as",
                "alice",
                "--peer",
                "bob",
                "--relay",
                relay.as_str(),
                "--max",
                "4",
            ],
        ),
        (
            &bob_cfg,
            vec![
                "handshake",
                "poll",
                "--as",
                "bob",
                "--peer",
                "alice",
                "--relay",
                relay.as_str(),
                "--max",
                "4",
            ],
        ),
    ] {
        let out = run_qsc(cfg, args.as_slice());
        assert!(
            out.status.success(),
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let text = String::from_utf8_lossy(&out.stdout).to_string()
            + &String::from_utf8_lossy(&out.stderr);
        assert!(!text.contains("handshake_reject"), "{text}");
    }
}
