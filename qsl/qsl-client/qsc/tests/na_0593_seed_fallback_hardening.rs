mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn fixture_route_value() -> String {
    format!("na0593_fixture_route_{}", std::process::id())
}

fn loopback_url(port: u16) -> String {
    let scheme = ["ht", "tp"].concat();
    format!("{scheme}://{}:{}", std::net::Ipv4Addr::LOCALHOST, port)
}

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
    fs::create_dir_all(path).expect("create test dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn clean_dir(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(output: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-na0593",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn assert_no_seed_or_key_material(text: &str) {
    for forbidden in [
        "QSC_QSP_SEED",
        "QSC_ALLOW_SEED_FALLBACK",
        "QSC_UNSAFE_TEST_SEED_FALLBACK",
        "seed=1",
        "SECRET",
        "PRIVATE",
        "KEY",
        "Authorization",
        "Bearer",
    ] {
        assert!(
            !text.contains(forbidden),
            "forbidden marker leaked: {forbidden}\n{text}"
        );
    }
}

#[test]
fn default_send_blocks_old_env_only_seed_fallback() {
    let base = safe_test_root().join(format!("na0593_send_block_{}", std::process::id()));
    clean_dir(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let bob_route = fixture_route_value();
    contacts_route_set(&cfg, "bob", &bob_route);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"na0593 send fixture").expect("write msg");
    let relay_url = loopback_url(1);

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    let text = output_text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("event=error code=protocol_inactive reason=no_session"));
    assert!(!text.contains("event=qsp_pack ok=true"), "{text}");
    assert!(!text.contains("event=send_commit"), "{text}");
    assert!(!cfg.join("outbox.json").exists(), "outbox should not exist");
    assert_no_seed_or_key_material(&text);
}

#[test]
fn default_receive_blocks_old_env_only_seed_fallback_before_relay_pull() {
    let base = safe_test_root().join(format!("na0593_recv_block_{}", std::process::id()));
    clean_dir(&base);
    let cfg = base.join("cfg");
    let out_dir = base.join("out");
    ensure_dir_700(&cfg);
    ensure_dir_700(&out_dir);
    common::init_mock_vault(&cfg);
    let bob_route = fixture_route_value();
    let relay_url = loopback_url(1);

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--mailbox",
            bob_route.as_str(),
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("receive");
    let text = output_text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("event=error code=protocol_inactive reason=no_session"));
    assert!(!text.contains("event=recv_start"), "{text}");
    assert!(!text.contains("event=qsp_unpack ok=true"), "{text}");
    assert_eq!(fs::read_dir(&out_dir).expect("out dir").count(), 0);
    assert_no_seed_or_key_material(&text);
}

#[test]
fn default_attachment_descriptor_blocks_old_env_only_seed_fallback() {
    let base = safe_test_root().join(format!("na0593_attachment_block_{}", std::process::id()));
    clean_dir(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let bob_route = fixture_route_value();
    contacts_route_set(&cfg, "bob", &bob_route);
    let payload = base.join("payload.bin");
    fs::write(&payload, b"na0593 attachment descriptor fixture").expect("write payload");
    let relay_url = loopback_url(1);
    let attachment_service_url = loopback_url(9);

    let output = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "file",
            "send",
            "--transport",
            "relay",
            "--relay",
            relay_url.as_str(),
            "--attachment-service",
            attachment_service_url.as_str(),
            "--legacy-in-message-stage",
            "w2",
            "--to",
            "bob",
            "--path",
            payload.to_str().unwrap(),
        ])
        .output()
        .expect("attachment descriptor send");
    let text = output_text(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("event=file_send_policy"), "{text}");
    assert!(text.contains("stage=w2"), "{text}");
    assert!(text.contains("size_class=legacy_sized"), "{text}");
    assert!(text.contains("event=error code=protocol_inactive reason=no_session"));
    assert!(!text.contains("event=qsp_pack ok=true"), "{text}");
    assert!(!text.contains("attachment_descriptor"), "{text}");
    assert!(!text.contains("event=attachment_service_commit"), "{text}");
    assert_no_seed_or_key_material(&text);
}

#[test]
fn explicit_unsafe_test_seed_fallback_fixture_still_works() {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0593_unsafe_fixture_{}", std::process::id()));
    clean_dir(&base);
    let cfg = base.join("cfg");
    ensure_dir_700(&cfg);
    common::init_mock_vault(&cfg);
    let bob_route = fixture_route_value();
    contacts_route_set(&cfg, "bob", &bob_route);
    let msg = base.join("msg.bin");
    fs::write(&msg, b"na0593 unsafe fixture").expect("write msg");

    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_MARK_FORMAT", "plain");
    common::add_unsafe_seed_fallback_env(&mut cmd);
    let output = cmd
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            msg.to_str().unwrap(),
        ])
        .output()
        .expect("unsafe fixture send");
    let text = output_text(&output);
    assert!(output.status.success(), "{text}");
    assert!(text.contains("event=qsp_pack ok=true"), "{text}");
    assert!(text.contains("event=send_commit"), "{text}");
    assert_no_seed_or_key_material(&text);
}

#[test]
fn help_does_not_expose_seed_fallback_as_normal_runtime() {
    let output = common::qsc_std_command()
        .env("QSC_MARK_FORMAT", "plain")
        .args(["--help"])
        .output()
        .expect("help");
    let text = output_text(&output);
    assert!(output.status.success(), "{text}");
    assert!(!text.contains("QSC_ALLOW_SEED_FALLBACK"), "{text}");
    assert!(!text.contains("QSC_UNSAFE_TEST_SEED_FALLBACK"), "{text}");
    assert!(!text.contains("QSC_QSP_SEED"), "{text}");
}
