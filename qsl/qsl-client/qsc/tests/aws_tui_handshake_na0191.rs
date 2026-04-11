use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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
    let out = common::qsc_assert_command()
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

fn run_cli(cfg: &Path, args: &[&str]) -> String {
    let out = common::qsc_assert_command()
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("NO_COLOR", "1")
        .args(args)
        .output()
        .expect("run qsc");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success(), "qsc {:?} failed: {}", args, combined);
    combined
}

fn format_verification_code_from_fingerprint(fingerprint: &str) -> String {
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

#[test]
fn tui_handshake_completes_after_restart_na0191() {
    let alice_cfg = unique_cfg_dir("na0191_tui_handshake_restart_alice");
    let bob_cfg = unique_cfg_dir("na0191_tui_handshake_restart_bob");
    ensure_dir_700(&alice_cfg);
    ensure_dir_700(&bob_cfg);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);

    run_cli(
        &alice_cfg,
        &["identity", "rotate", "--as", "self", "--confirm"],
    );
    run_cli(
        &bob_cfg,
        &["identity", "rotate", "--as", "self", "--confirm"],
    );
    let alice_show = run_cli(&alice_cfg, &["identity", "show", "--as", "self"]);
    let bob_show = run_cli(&bob_cfg, &["identity", "show", "--as", "self"]);
    let alice_fp = alice_show
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("alice identity fp");
    let bob_fp = bob_show
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .expect("bob identity fp");
    let alice_code = format_verification_code_from_fingerprint(alice_fp);
    let bob_code = format_verification_code_from_fingerprint(bob_fp);

    run_cli(
        &alice_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE],
    );
    run_cli(
        &bob_cfg,
        &["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB],
    );
    run_cli(
        &alice_cfg,
        &["contacts", "trust-mode", "set", "--mode", "balanced"],
    );
    run_cli(
        &bob_cfg,
        &["contacts", "trust-mode", "set", "--mode", "strict"],
    );
    run_cli(
        &alice_cfg,
        &[
            "contacts",
            "add",
            "--label",
            "Bob",
            "--fp",
            &bob_code,
            "--route-token",
            ROUTE_TOKEN_BOB,
            "--verify",
        ],
    );
    run_cli(
        &bob_cfg,
        &[
            "contacts",
            "add",
            "--label",
            "Alice",
            "--fp",
            &alice_code,
            "--route-token",
            ROUTE_TOKEN_ALICE,
        ],
    );
    run_cli(
        &bob_cfg,
        &[
            "contacts",
            "verify",
            "--label",
            "Alice",
            "--fp",
            &alice_code,
            "--confirm",
        ],
    );
    let alice_device = run_cli(
        &bob_cfg,
        &["contacts", "device", "primary", "show", "--label", "Alice"],
    )
    .lines()
    .find_map(|line| line.split("device=").nth(1))
    .and_then(|tail| tail.split_whitespace().next())
    .expect("alice device id")
    .to_string();
    run_cli(
        &bob_cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            "Alice",
            "--device",
            alice_device.as_str(),
            "--confirm",
        ],
    );

    let relay = common::start_inbox_server(1024 * 1024, 16);
    let alice_token_file = alice_cfg.join("relay_token.txt");
    let bob_token_file = bob_cfg.join("relay_token.txt");
    std::fs::write(&alice_token_file, "persisted-token\n").expect("write alice token");
    std::fs::write(&bob_token_file, "persisted-token\n").expect("write bob token");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&alice_token_file, std::fs::Permissions::from_mode(0o600))
            .expect("chmod 600 alice token");
        std::fs::set_permissions(&bob_token_file, std::fs::Permissions::from_mode(0o600))
            .expect("chmod 600 bob token");
    }

    let alice_setup = run_headless(
        &alice_cfg,
        format!(
            "/relay set endpoint {};/relay set token-file {};/exit",
            relay.base_url(),
            alice_token_file.to_string_lossy()
        )
        .as_str(),
    );
    let bob_setup = run_headless(
        &bob_cfg,
        format!(
            "/relay set endpoint {};/relay set token-file {};/exit",
            relay.base_url(),
            bob_token_file.to_string_lossy()
        )
        .as_str(),
    );
    assert!(
        alice_setup.contains("event=tui_cmd_result kind=ok command=relay_set_endpoint")
            && bob_setup.contains("event=tui_cmd_result kind=ok command=relay_set_endpoint"),
        "tui relay setup should persist endpoint on both sides: {alice_setup}\n---\n{bob_setup}"
    );

    let bob_init = run_headless(&bob_cfg, "/messages select Alice;/handshake init;/exit");
    let alice_poll = run_headless(&alice_cfg, "/messages select Bob;/handshake poll;/exit");
    let bob_poll = run_headless(&bob_cfg, "/messages select Alice;/handshake poll;/exit");
    let alice_confirm = run_headless(&alice_cfg, "/messages select Bob;/handshake poll;/exit");
    let combined = format!("{bob_init}{alice_poll}{bob_poll}{alice_confirm}");

    assert!(
        combined.contains("event=handshake_send msg=B1"),
        "tui handshake should produce B1 on responder poll: {combined}"
    );
    assert!(
        combined.contains("event=handshake_send msg=A2")
            && combined.contains("event=handshake_complete peer=Alice role=initiator")
            && combined.contains("event=handshake_complete peer=Bob role=responder"),
        "tui handshake should complete A1/B1/A2 across restarted headless sessions: {combined}"
    );
    assert!(
        !combined.contains("reason=decode_failed")
            && !combined.contains("reason=session_id_mismatch")
            && !combined.contains("reason=peer_mismatch"),
        "tui handshake rerun must not fail with decode/session/peer mismatch: {combined}"
    );
    assert!(
        !combined.contains("/v1/"),
        "must not leak relay URI path: {combined}"
    );
    assert!(
        !combined.contains("Authorization") && !combined.contains("Bearer"),
        "must not leak auth markers: {combined}"
    );
    assert!(
        !has_long_hex(&combined, 32),
        "must not leak long hex secrets: {combined}"
    );
}
