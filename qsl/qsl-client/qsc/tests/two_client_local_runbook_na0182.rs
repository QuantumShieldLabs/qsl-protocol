mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

const ALICE_MAILBOX: &str = "alice_mailbox_token_abcd1234efgh";
const BOB_MAILBOX: &str = "bob_mailbox_token_wxyz5678qrst";

fn output_text(out: &Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn leak_counts(text: &str) -> (usize, usize) {
    let v1 = text.matches("/v1/").count();
    let mut long_hex = 0usize;
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i].is_ascii_hexdigit() {
            let start = i;
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                i += 1;
            }
            if i - start >= 32 {
                long_hex += 1;
            }
        } else {
            i += 1;
        }
    }
    (v1, long_hex)
}

fn assert_no_leaks(text: &str) {
    let (v1, hex32) = leak_counts(text);
    assert_eq!(v1, 0, "found v1 path marker in output: {text}");
    assert_eq!(hex32, 0, "found long hex marker in output: {text}");
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

fn qsc_cmd(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn run_ok(cfg: &Path, args: &[&str]) -> String {
    let out = qsc_cmd(cfg).args(args).output().expect("run qsc");
    let text = output_text(&out);
    assert!(out.status.success(), "command failed: {:?}\n{}", args, text);
    text
}

fn init_identity(cfg: &Path) -> String {
    run_ok(
        cfg,
        &["vault", "init", "--non-interactive", "--key-source", "mock"],
    );
    run_ok(cfg, &["identity", "rotate", "--confirm"]);
    let show = run_ok(cfg, &["identity", "show"]);
    show.lines()
        .find_map(|line| line.strip_prefix("identity_fp="))
        .unwrap_or_else(|| panic!("missing identity_fp output: {show}"))
        .to_string()
}

fn set_inbox(cfg: &Path, token: &str) {
    run_ok(cfg, &["relay", "inbox-set", "--token", token]);
}

fn add_contact(cfg: &Path, label: &str, fp: &str, route_token: &str) {
    run_ok(
        cfg,
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
}

fn trust_primary_device(cfg: &Path, label: &str) {
    let list = run_ok(cfg, &["contacts", "device", "list", "--label", label]);
    let device = list
        .lines()
        .find_map(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {list}"));
    run_ok(
        cfg,
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

struct LocalRelay {
    child: Child,
}

impl LocalRelay {
    fn start(root: &Path) -> (Self, String) {
        let log_path = root.join("relay.log");
        let log = fs::File::create(&log_path).expect("relay log");
        let child = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
            .args([
                "relay",
                "serve",
                "--port",
                "0",
                "--seed",
                "0",
                "--drop-pct",
                "0",
                "--dup-pct",
                "0",
                "--reorder-window",
                "0",
                "--fixed-latency-ms",
                "0",
                "--jitter-ms",
                "0",
            ])
            .stdout(Stdio::from(log))
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn relay");
        let deadline = Instant::now() + Duration::from_secs(4);
        let port = loop {
            let text = fs::read_to_string(&log_path).unwrap_or_default();
            if let Some(port) = text
                .lines()
                .find_map(|line| line.split("event=relay_listen port=").nth(1))
                .and_then(|tail| tail.split_whitespace().next())
            {
                break port.to_string();
            }
            assert!(Instant::now() < deadline, "relay did not become ready");
            thread::sleep(Duration::from_millis(20));
        };
        (Self { child }, format!("http://127.0.0.1:{port}"))
    }
}

impl Drop for LocalRelay {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn two_client_isolation_state_dirs_do_not_collide() {
    let base = safe_test_root("na0182_isolation");
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);

    let bob_fp = init_identity(&bob_cfg);
    init_identity(&alice_cfg);
    add_contact(&alice_cfg, "peerz", bob_fp.as_str(), BOB_MAILBOX);

    let alice_list = run_ok(&alice_cfg, &["contacts", "list"]);
    let bob_list = run_ok(&bob_cfg, &["contacts", "list"]);
    assert!(alice_list.contains("label=peerz"), "{}", alice_list);
    assert!(!bob_list.contains("label=peerz"), "{}", bob_list);
    assert_no_leaks(&alice_list);
    assert_no_leaks(&bob_list);
}

#[test]
fn two_client_local_relay_message_and_file_flow_is_honest() {
    let base = safe_test_root("na0182_relay_flow");
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);

    let alice_fp = init_identity(&alice_cfg);
    let bob_fp = init_identity(&bob_cfg);
    let (_relay, relay_url) = LocalRelay::start(&base);

    set_inbox(&alice_cfg, ALICE_MAILBOX);
    set_inbox(&bob_cfg, BOB_MAILBOX);

    add_contact(&alice_cfg, "bob", bob_fp.as_str(), BOB_MAILBOX);
    // Keep contact labels aligned so receive --from matches local peer label on both clients.
    add_contact(&bob_cfg, "bob", alice_fp.as_str(), ALICE_MAILBOX);
    trust_primary_device(&alice_cfg, "bob");
    trust_primary_device(&bob_cfg, "bob");

    let msg = base.join("msg.txt");
    fs::write(&msg, b"na0182 message").expect("write msg");
    let send_msg = run_ok(
        &alice_cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--to",
            "bob",
            "--file",
            msg.to_str().expect("msg path"),
            "--receipt",
            "delivered",
        ],
    );
    assert!(
        send_msg.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_msg
    );

    let bob_recv_msg = run_ok(
        &bob_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--mailbox",
            BOB_MAILBOX,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            bob_out.to_str().expect("bob out"),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ],
    );
    assert!(
        bob_recv_msg.contains("QSC_RECEIPT mode=immediate status=sent kind=message peer=bob"),
        "{}",
        bob_recv_msg
    );
    let alice_recv_msg = run_ok(
        &alice_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--mailbox",
            ALICE_MAILBOX,
            "--from",
            "bob",
            "--max",
            "4",
            "--out",
            alice_out.to_str().expect("alice out"),
        ],
    );
    assert!(
        alice_recv_msg.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv_msg
    );

    let payload = base.join("payload.bin");
    fs::write(&payload, b"na0182-file").expect("write payload");
    let send_file = run_ok(
        &alice_cfg,
        &[
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--to",
            "bob",
            "--path",
            payload.to_str().expect("payload path"),
            "--receipt",
            "delivered",
        ],
    );
    assert!(
        send_file.contains("QSC_FILE_DELIVERY state=accepted_by_relay"),
        "{}",
        send_file
    );
    assert!(
        send_file.contains("QSC_FILE_DELIVERY state=awaiting_confirmation"),
        "{}",
        send_file
    );
    run_ok(
        &bob_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--mailbox",
            BOB_MAILBOX,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            bob_out.to_str().expect("bob out"),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
            "--file-confirm-mode",
            "complete-only",
        ],
    );
    let alice_recv_file = run_ok(
        &alice_cfg,
        &[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--mailbox",
            ALICE_MAILBOX,
            "--from",
            "bob",
            "--max",
            "8",
            "--out",
            alice_out.to_str().expect("alice out"),
        ],
    );
    assert!(
        alice_recv_file.contains("QSC_FILE_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv_file
    );

    assert_no_leaks(&send_msg);
    assert_no_leaks(&bob_recv_msg);
    assert_no_leaks(&alice_recv_msg);
    assert_no_leaks(&send_file);
    assert_no_leaks(&alice_recv_file);
}

#[test]
fn blocked_paths_emit_remediation_and_do_not_mutate() {
    let base = safe_test_root("na0182_blocked_paths");
    let alice_cfg = base.join("alice_cfg");
    create_dir_700(&alice_cfg);
    init_identity(&alice_cfg);

    let msg = base.join("blocked.txt");
    fs::write(&msg, b"blocked").expect("write blocked");

    let unknown = qsc_cmd(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--to",
            "unknown",
            "--file",
            msg.to_str().expect("msg path"),
        ])
        .output()
        .expect("unknown send");
    let unknown_text = output_text(&unknown);
    assert!(!unknown.status.success(), "{}", unknown_text);
    assert!(
        unknown_text.contains("QSC_SEND_BLOCKED reason=unknown_contact peer=unknown"),
        "{}",
        unknown_text
    );
    assert!(
        unknown_text
            .contains("QSC_TRUST_REMEDIATION reason=unknown_contact step=add_contact peer=unknown"),
        "{}",
        unknown_text
    );
    assert!(
        !unknown_text.contains("event=send_prepare") && !unknown_text.contains("QSC_ROUTING"),
        "{}",
        unknown_text
    );

    assert_no_leaks(&unknown_text);
}
