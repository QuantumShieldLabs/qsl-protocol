use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
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
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn start_relay(
    seed: u64,
    drop_pct: u8,
    dup_pct: u8,
    reorder_window: usize,
    max_messages: u64,
) -> (
    std::process::Child,
    u16,
    Arc<Mutex<Vec<String>>>,
    thread::JoinHandle<()>,
) {
    let mut child = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .args([
            "relay",
            "serve",
            "--port",
            "0",
            "--seed",
            &seed.to_string(),
            "--drop-pct",
            &drop_pct.to_string(),
            "--dup-pct",
            &dup_pct.to_string(),
            "--reorder-window",
            &reorder_window.to_string(),
            "--fixed-latency-ms",
            "0",
            "--jitter-ms",
            "0",
            "--max-messages",
            &max_messages.to_string(),
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

fn send_once(cfg_dir: &Path, relay_addr: &str, payload: &[u8], idx: usize) -> String {
    let payload_path = cfg_dir.join(format!("msg_{idx}.bin"));
    fs::write(&payload_path, payload).expect("write payload");

    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg_dir)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "relay",
            "send",
            "--to",
            "peer",
            "--file",
            payload_path.to_str().unwrap(),
            "--relay",
            relay_addr,
        ])
        .output()
        .expect("relay send");

    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn assert_no_implicit_recovery(markers: &str) {
    assert!(!markers.contains("recover"));
    assert!(!markers.contains("resync"));
    assert!(!markers.contains("retry"));
}

fn normalize_marker(line: &str) -> String {
    if line.contains("event=relay_listen") {
        let mut seed = "unknown";
        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix("seed=") {
                seed = v;
            }
        }
        return format!("QSC_MARK/1 event=relay_listen seed={seed}");
    }
    line.trim().to_string()
}

fn collect_markers(output: &str) -> Vec<String> {
    output
        .lines()
        .filter(|l| l.contains("QSC_MARK/1"))
        .map(normalize_marker)
        .collect()
}

fn relay_action(
    seed: u64,
    seq: u64,
    drop_pct: u8,
    dup_pct: u8,
    reorder_window: usize,
) -> &'static str {
    let mut rng = RelayRng::new(seed ^ seq);
    let roll = (rng.next_u32() % 100) as u8;
    if drop_pct > 0 && roll < drop_pct {
        return "drop";
    }
    let roll_dup = (rng.next_u32() % 100) as u8;
    if dup_pct > 0 && roll_dup < dup_pct {
        return "dup";
    }
    if reorder_window > 1 && (seq % (reorder_window as u64)) == 1 {
        return "reorder";
    }
    "deliver"
}

struct RelayRng {
    state: u64,
}

impl RelayRng {
    fn new(seed: u64) -> Self {
        Self {
            state: seed ^ 0x9e3779b97f4a7c15,
        }
    }

    fn next_u32(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        (x & 0xffff_ffff) as u32
    }
}

fn find_seed_for_drop_and_reorder(drop_pct: u8, reorder_window: usize) -> u64 {
    for seed in 1u64..=2000u64 {
        let mut saw_drop = false;
        let mut saw_reorder = false;
        for seq in 1u64..=6u64 {
            match relay_action(seed, seq, drop_pct, 0, reorder_window) {
                "drop" => saw_drop = true,
                "reorder" => saw_reorder = true,
                _ => {}
            }
        }
        if saw_drop && saw_reorder {
            return seed;
        }
    }
    panic!("no seed produced drop+reorder in search range");
}

#[test]
fn relay_reorder_no_implicit_recovery() {
    let base = safe_test_root().join(format!("na0075_relay_reorder_{}", std::process::id()));
    create_dir_700(&base);

    let (mut relay, port, lines, handle) = start_relay(42, 0, 0, 2, 3);
    let relay_addr = format!("127.0.0.1:{}", port);

    let mut outputs = Vec::new();
    for i in 0..3 {
        let cfg = base.join(format!("cfg_{i}"));
        create_dir_700(&cfg);
        let out = send_once(&cfg, &relay_addr, b"reorder", i);
        outputs.push(out);
    }

    let relay_lines = lines.lock().unwrap().join("\n");
    assert!(relay_lines.contains("event=relay_event action=reorder"));

    for out in &outputs {
        assert!(out.contains("event=send_prepare"));
        assert!(out.contains("event=send_commit"));
        assert_no_implicit_recovery(out);
    }

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();
}

#[test]
fn relay_drop_plus_reorder_no_mutation() {
    let base = safe_test_root().join(format!("na0075_relay_drop_reorder_{}", std::process::id()));
    create_dir_700(&base);

    let seed = find_seed_for_drop_and_reorder(35, 2);
    let (mut relay, port, lines, handle) = start_relay(seed, 35, 0, 2, 6);
    let relay_addr = format!("127.0.0.1:{}", port);

    let mut outputs = Vec::new();
    let mut failed = 0;
    for i in 0..6 {
        let cfg = base.join(format!("cfg_{i}"));
        create_dir_700(&cfg);
        let out = send_once(&cfg, &relay_addr, b"drop_reorder", i);
        if out.contains("code=relay_delivery_failed") {
            failed += 1;
            let send_state = cfg.join("send.state");
            let outbox = cfg.join("outbox.json");
            assert!(!send_state.exists());
            assert!(outbox.exists());
            assert!(out.contains("event=send_attempt ok=false"));
        }
        assert_no_implicit_recovery(&out);
        outputs.push(out);
    }

    assert!(failed > 0);
    let relay_lines = lines.lock().unwrap().join("\n");
    assert!(relay_lines.contains("event=relay_event action=drop"));
    assert!(relay_lines.contains("event=relay_event action=reorder"));

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();
}

#[test]
fn relay_seeded_replay_deterministic() {
    let base = safe_test_root().join(format!("na0075_relay_replay_{}", std::process::id()));
    create_dir_700(&base);

    let run_sequence = |seed: u64, run_id: usize| -> Vec<String> {
        let (mut relay, port, lines, handle) = start_relay(seed, 0, 0, 2, 3);
        let relay_addr = format!("127.0.0.1:{}", port);
        let mut markers = Vec::new();
        for i in 0..3 {
            let cfg = base.join(format!("cfg_{run_id}_{i}"));
            create_dir_700(&cfg);
            let out = send_once(&cfg, &relay_addr, b"replay", i);
            markers.extend(collect_markers(&out));
        }
        let relay_lines = lines.lock().unwrap().join("\n");
        markers.extend(collect_markers(&relay_lines));
        let _ = relay.kill();
        let _ = relay.wait();
        let _ = handle.join();
        markers
    };

    let a = run_sequence(123, 1);
    let b = run_sequence(123, 2);
    assert_eq!(a, b);
}
