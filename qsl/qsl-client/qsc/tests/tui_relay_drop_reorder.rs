use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_PEER: &str = "route_token_peer_abcdefghijklmnopq";

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

fn init_mock_vault(cfg: &Path) {
    let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .args(["vault", "init", "--non-interactive", "--key-source", "mock"])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "vault init failed");
}

fn add_peer_route_token(cfg: &Path) {
    for label in ["peer", "peer-0"] {
        let out = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
            .env("QSC_CONFIG_DIR", cfg)
            .env("QSC_QSP_SEED", "1")
            .env("QSC_ALLOW_SEED_FALLBACK", "1")
            .args([
                "contacts",
                "add",
                "--label",
                label,
                "--fp",
                "fp-test",
                "--route-token",
                ROUTE_TOKEN_PEER,
            ])
            .output()
            .expect("contacts add");
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stdout)
        );
    }
}

fn run_tui(cfg_dir: &Path, seed: u64, script: &str) -> String {
    let output = Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg_dir)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .args([
            "tui",
            "--transport",
            "relay",
            "--relay",
            "http://127.0.0.1:9",
            "--seed",
            &seed.to_string(),
            "--scenario",
            "drop-reorder",
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
    init_mock_vault(&cfg);
    add_peer_route_token(&cfg);

    let script = "/send;/send abort;/send;/send abort;/send;/send abort;/send;/send abort;/send;/send abort;/send;/send abort;/exit";
    let out = run_tui(&cfg, 0, script);

    assert!(out.contains("event=tui_event kind=relay_event action=drop"));
    assert!(out.contains("event=tui_event kind=relay_event action=outbox_exists"));
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
}

#[test]
fn tui_relay_seeded_replay_deterministic() {
    let base = safe_test_root().join(format!("na0079_tui_relay_replay_{}", std::process::id()));
    create_dir_700(&base);

    let mut outputs = Vec::new();
    for run in 0..2 {
        let cfg = base.join(format!("cfg_{run}"));
        create_dir_700(&cfg);
        init_mock_vault(&cfg);
        add_peer_route_token(&cfg);
        let out = run_tui(&cfg, 9, "/send;/send;/exit");
        outputs.push(normalized_markers(&out));
    }

    assert_eq!(outputs[0], outputs[1]);
}
