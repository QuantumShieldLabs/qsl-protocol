use assert_cmd::Command as AssertCommand;
use predicates::prelude::*;
use predicates::str::contains;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

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

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
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

fn start_relay(
    drop_pct: u8,
    dup_pct: u8,
) -> (Child, u16, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .args([
            "relay",
            "serve",
            "--port",
            "0",
            "--seed",
            "7",
            "--drop-pct",
            &drop_pct.to_string(),
            "--dup-pct",
            &dup_pct.to_string(),
            "--max-messages",
            "2",
        ])
        .env("QSC_MARK_FORMAT", "plain")
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn relay");

    let stdout = child.stdout.take().expect("relay stdout");
    let (tx, rx) = mpsc::channel();
    let lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let lines_thread = Arc::clone(&lines);
    let handle = thread::spawn(move || {
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if line.contains("event=relay_listen") {
                for part in line.split_whitespace() {
                    if let Some(v) = part.strip_prefix("port=") {
                        if let Ok(p) = v.parse::<u16>() {
                            let _ = tx.send(p);
                        }
                    }
                }
            }
            lines_thread.lock().unwrap().push(line);
        }
    });
    let port = rx.recv_timeout(Duration::from_secs(2)).expect("relay port");
    (child, port, lines, handle)
}

fn combined_output(output: &std::process::Output) -> String {
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

#[test]
fn send_refuses_without_transport() {
    let base = safe_test_root().join(format!("na0084_send_no_transport_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["send", "--to", "bob", "--file", payload.to_str().unwrap()]);
    cmd.assert().failure().stdout(predicate::eq(
        "QSC_MARK/1 event=error code=send_transport_required\n",
    ));
}

#[test]
fn send_happy_path_local_relay() {
    let base = safe_test_root().join(format!("na0084_send_happy_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let (mut relay, port, _lines, handle) = start_relay(0, 0);
    let relay_addr = format!("127.0.0.1:{}", port);

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send");

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();

    if !output.status.success() {
        panic!("send failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_attempt ok=true"));
    assert!(combined.contains("event=send_commit"));
}

#[test]
fn send_failure_no_commit() {
    let base = safe_test_root().join(format!("na0084_send_fail_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send fail");

    assert!(!output.status.success(), "send should fail");
    let combined = combined_output(&output);
    assert!(combined.contains("event=relay_event action=connect_fail"));
    assert!(combined.contains("event=send_attempt ok=false"));
    assert!(!combined.contains("event=send_commit"));
}

#[test]
fn outbox_recovery_via_send_abort() {
    let base = safe_test_root().join(format!("na0084_outbox_recover_{}", std::process::id()));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send fail");
    assert!(!output.status.success());

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send outbox exists");
    assert!(!output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    assert!(combined.contains("event=error code=outbox_exists"));

    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args(["send", "abort"]);
    cmd.assert()
        .success()
        .stdout(contains("event=outbox_abort"));

    let (mut relay, port, _lines, handle) = start_relay(0, 0);
    let relay_addr = format!("127.0.0.1:{}", port);

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_addr.as_str(),
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send after abort");

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();

    if !output.status.success() {
        panic!("send after abort failed: {}", combined_output(&output));
    }
    let combined = combined_output(&output);
    assert!(combined.contains("event=send_prepare"));
    assert!(combined.contains("event=send_attempt ok=true"));
    assert!(combined.contains("event=send_commit"));
}

#[test]
fn send_outputs_have_no_secrets() {
    let dir = safe_test_root().join(format!("na0084_send_no_secrets_{}", std::process::id()));
    create_dir_700(&dir);
    let cfg = dir.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            "127.0.0.1:9",
            "--to",
            "peer",
            "--file",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("run send no secrets");

    let combined = combined_output(&output);
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
    ] {
        assert!(
            !combined.contains(needle),
            "unexpected secret token in output"
        );
    }
}
