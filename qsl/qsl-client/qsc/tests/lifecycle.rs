use assert_cmd::Command;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const PANIC_SENTINEL: &str = "QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK";
const PANIC_MARKER: &str = "QSC_MARK/1 event=panic code=panic_redacted";

fn test_root() -> PathBuf {
    let base = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    base.join("qsc-test-tmp")
}

fn unique_dir(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    test_root().join(format!("na0111_{}_{}_{}", label, std::process::id(), nonce))
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn qsc_cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
}

fn combined_output(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn assert_no_secret_markers(text: &str) {
    for needle in [
        "TOKEN",
        "SECRET",
        "KEY",
        "PASS",
        "PRIVATE",
        "BEARER",
        "CREDENTIAL",
        PANIC_SENTINEL,
    ] {
        assert!(
            !text.contains(needle),
            "output leaked sensitive token: {needle}\n{text}"
        );
    }
}

#[test]
fn panic_is_redacted_no_secrets() {
    let root = unique_dir("panic");
    let cfg = root.join("cfg");
    ensure_dir_700(&cfg);

    let out = qsc_cmd()
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["util", "panic-demo"])
        .output()
        .expect("panic-demo output");

    assert!(!out.status.success(), "panic-demo must fail");
    let text = combined_output(&out);
    assert!(
        text.contains(PANIC_MARKER),
        "panic marker missing from output:\n{text}"
    );
    assert!(
        !text.contains(PANIC_SENTINEL),
        "panic sentinel leaked in output:\n{text}"
    );
    assert_no_secret_markers(&text);
}

#[test]
fn no_cwd_artifacts_for_common_commands() {
    let root = unique_dir("cwd");
    let cfg = root.join("cfg");
    let cwd = root.join("cwd");
    ensure_dir_700(&cfg);
    ensure_dir_700(&cwd);

    let commands: &[&[&str]] = &[
        &["status"],
        &["doctor", "--check-only"],
        &["util", "sanitize", "--print", "hello-world"],
    ];
    for args in commands {
        let mut cmd = qsc_cmd();
        cmd.current_dir(&cwd)
            .env("QSC_CONFIG_DIR", &cfg)
            .args(*args);
        cmd.assert().success();
        let mut entries = fs::read_dir(&cwd).unwrap();
        assert!(
            entries.next().is_none(),
            "cwd must remain empty after {:?}",
            args
        );
    }
}

#[test]
fn no_secrets_in_outputs_smoke() {
    let root = unique_dir("outputs");
    let cfg = root.join("cfg");
    ensure_dir_700(&cfg);

    let mut aggregate = String::new();

    let ok_cmds: &[&[&str]] = &[
        &["status"],
        &["doctor", "--check-only"],
        &["util", "sanitize", "--print", "hello-world"],
    ];
    for args in ok_cmds {
        let out = qsc_cmd()
            .env("QSC_CONFIG_DIR", &cfg)
            .args(*args)
            .output()
            .expect("command output");
        assert!(
            out.status.success(),
            "expected success for {:?}, got {:?}",
            args,
            out.status.code()
        );
        aggregate.push_str(&combined_output(&out));
    }

    let panic_out = qsc_cmd()
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["util", "panic-demo"])
        .output()
        .expect("panic-demo output");
    assert!(!panic_out.status.success());
    let panic_text = combined_output(&panic_out);
    assert!(panic_text.contains(PANIC_MARKER));
    aggregate.push_str(&panic_text);

    assert!(aggregate.contains("QSC_MARK/1 event=status"));
    assert_no_secret_markers(&aggregate);
}
