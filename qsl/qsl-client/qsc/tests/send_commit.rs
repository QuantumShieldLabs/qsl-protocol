use predicates::prelude::*;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Create a writable, safe test root without relying on $HOME.
fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };

    let root = root.join("qsc-test-tmp");
    create_dir_700(&root);
    root
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

fn start_relay() -> (Child, u16, Arc<Mutex<Vec<String>>>, thread::JoinHandle<()>) {
    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .args([
            "relay",
            "serve",
            "--port",
            "0",
            "--seed",
            "7",
            "--drop-pct",
            "0",
            "--dup-pct",
            "0",
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

fn read_send_seq(path: &PathBuf) -> u64 {
    let content = fs::read_to_string(path).expect("read send.state");
    let line = content
        .lines()
        .find(|l| l.trim().starts_with("send_seq="))
        .expect("send_seq present");
    line.trim()
        .strip_prefix("send_seq=")
        .unwrap()
        .parse::<u64>()
        .expect("send_seq parse")
}

#[test]
fn send_failure_no_commit() {
    let base = safe_test_root().join(format!("na0070_send_fail_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");
    assert!(!outbox.exists());
    assert!(!send_state.exists());

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
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
        ]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("event=send_prepare"))
        .stdout(predicate::str::contains("event=send_attempt ok=false"))
        .stdout(predicate::str::contains("code=relay_connect_failed"));

    // No mutation on reject: send state must not advance.
    assert!(!send_state.exists());
    assert!(outbox.exists());
}

#[test]
fn outbox_commit_advances_once() {
    let base = safe_test_root().join(format!("na0070_send_commit_{}", std::process::id()));
    create_dir_700(&base);

    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let payload = cfg.join("msg.bin");
    fs::write(&payload, b"hello").expect("write payload");

    let outbox = cfg.join("outbox.json");
    let send_state = cfg.join("send.state");

    let (mut relay, port, _lines, handle) = start_relay();
    let relay_addr = format!("127.0.0.1:{}", port);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
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
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert!(send_state.exists());
    assert_eq!(read_send_seq(&send_state), 1);

    let mut cmd = qsc_cmd();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
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
        ]);
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("event=send_commit"));

    assert!(!outbox.exists());
    assert_eq!(read_send_seq(&send_state), 2);

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();
}
