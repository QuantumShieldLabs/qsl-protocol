use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
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

fn start_relay(
    seed: u64,
    drop_pct: u8,
    reorder_window: usize,
    max_messages: u64,
) -> (std::process::Child, u16, thread::JoinHandle<()>) {
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
            "--reorder-window",
            &reorder_window.to_string(),
            "--max-messages",
            &max_messages.to_string(),
        ])
        .env("QSC_MARK_FORMAT", "plain")
        .stdout(Stdio::piped())
        .spawn()
        .expect("spawn relay");

    let stdout = child.stdout.take().expect("relay stdout");
    let (tx, rx) = mpsc::channel();
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
        }
    });
    let port = rx.recv_timeout(Duration::from_secs(2)).expect("relay port");
    (child, port, handle)
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

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.state = x;
        x.wrapping_mul(0x2545f4914f6cdd1d)
    }

    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }
}

fn relay_action(seed: u64, seq: u64, drop_pct: u8, reorder_window: usize) -> &'static str {
    let mut rng = RelayRng::new(seed ^ seq);
    let roll = (rng.next_u32() % 100) as u8;
    if drop_pct > 0 && roll < drop_pct {
        return "drop";
    }
    if reorder_window > 1 && (seq % (reorder_window as u64)) == 1 {
        return "reorder";
    }
    "deliver"
}

fn find_seed_for_drop_and_reorder(drop_pct: u8, reorder_window: usize, max_seq: u64) -> u64 {
    for seed in 1u64..500u64 {
        let mut has_drop = false;
        let mut has_reorder = false;
        for seq in 1..=max_seq {
            match relay_action(seed, seq, drop_pct, reorder_window) {
                "drop" => has_drop = true,
                "reorder" => has_reorder = true,
                _ => {}
            }
        }
        if has_drop && has_reorder {
            return seed;
        }
    }
    panic!("no seed found for drop+reorder");
}

fn run_tui(cfg_dir: &Path, relay_addr: &str, seed: u64, scenario: &str, script: &str) -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg_dir)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .args([
            "tui",
            "--transport",
            "relay",
            "--relay",
            relay_addr,
            "--seed",
            &seed.to_string(),
            "--scenario",
            scenario,
        ])
        .output()
        .expect("run tui");
    assert!(output.status.success());
    let mut combined = String::from_utf8_lossy(&output.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&output.stderr));
    combined
}

fn count_markers(out: &str, event: &str) -> usize {
    out.lines().filter(|l| l.contains(event)).count()
}

fn read_send_state(path: &Path) -> Option<u64> {
    let text = fs::read_to_string(path).ok()?;
    for line in text.lines() {
        if let Some(rest) = line.trim().strip_prefix("send_seq=") {
            return rest.trim().parse::<u64>().ok();
        }
    }
    None
}

fn normalized_markers(out: &str) -> Vec<String> {
    out.lines()
        .filter(|l| l.contains("QSC_MARK/1"))
        .filter(|l| {
            l.contains("event=tui_event")
                || l.contains("event=send_")
                || l.contains("event=tui_cmd")
        })
        .map(|l| l.to_string())
        .collect()
}

#[test]
fn tui_relay_drop_reorder_event_stream() {
    let base = safe_test_root().join(format!(
        "na0079_tui_relay_drop_reorder_{}",
        std::process::id()
    ));
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);

    let seed = find_seed_for_drop_and_reorder(35, 2, 8);
    let (mut relay, port, handle) = start_relay(seed, 35, 2, 10);
    let relay_addr = format!("127.0.0.1:{}", port);

    let script = "/send\n/send\n/send\n/send\n/send\n/send\n/exit\n";
    let out = run_tui(&cfg, &relay_addr, seed, "drop-reorder", script);

    assert!(out.contains("event=tui_event kind=relay_event action=drop"));
    assert!(out.contains("event=tui_event kind=relay_event action=reorder"));
    assert!(!out.contains("retry"));
    assert!(!out.contains("recover"));

    let commit_count = count_markers(&out, "event=send_commit");
    let send_state = cfg.join("send.state");
    if send_state.exists() {
        let seq = read_send_state(&send_state).unwrap_or(0);
        assert_eq!(seq as usize, commit_count);
    } else {
        assert_eq!(commit_count, 0);
    }

    let _ = relay.kill();
    let _ = relay.wait();
    let _ = handle.join();
}

#[test]
fn tui_relay_seeded_replay_deterministic() {
    let base = safe_test_root().join(format!("na0079_tui_relay_replay_{}", std::process::id()));
    create_dir_700(&base);

    let mut outputs = Vec::new();
    for run in 0..2 {
        let cfg = base.join(format!("cfg_{run}"));
        create_dir_700(&cfg);
        let (mut relay, port, handle) = start_relay(9, 0, 2, 6);
        let relay_addr = format!("127.0.0.1:{}", port);
        let out = run_tui(&cfg, &relay_addr, 9, "reorder", "/send\n/send\n/exit\n");
        outputs.push(normalized_markers(&out));
        let _ = relay.kill();
        let _ = relay.wait();
        let _ = handle.join();
    }

    assert_eq!(outputs[0], outputs[1]);
}
