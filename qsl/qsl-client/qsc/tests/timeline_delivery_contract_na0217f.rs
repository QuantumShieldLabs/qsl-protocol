mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const ROUTE_BOB_A: &str = "route_token_boba_na0217f_delivery";
const ROUTE_BOB_B: &str = "route_token_bobb_na0217f_delivery";

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod 700");
    }
}

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::temp_dir()
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn output_text(out: &Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn qsc(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_QSP_SEED", "217")
        .env("QSC_TEST_MODE", "1");
    cmd
}

fn run(cfg: &Path, args: &[&str]) -> Output {
    qsc(cfg).args(args).output().expect("run qsc")
}

fn init_cfg(cfg: &Path) {
    ensure_dir_700(cfg);
    common::init_mock_vault(cfg);
}

fn add_two_bob_devices(cfg: &Path) -> (String, String) {
    let add = run(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "ABCD-EFGH-JKMP-QRST-V",
            "--route-token",
            ROUTE_BOB_A,
            "--verify",
        ],
    );
    assert!(add.status.success(), "{}", output_text(&add));

    let add_second = run(
        cfg,
        &[
            "contacts",
            "device",
            "add",
            "--label",
            "bob",
            "--fp",
            "BBBB-CCCC-DDDD-EEEE-F",
            "--route-token",
            ROUTE_BOB_B,
        ],
    );
    assert!(add_second.status.success(), "{}", output_text(&add_second));

    let list = run(cfg, &["contacts", "device", "list", "--label", "bob"]);
    assert!(list.status.success(), "{}", output_text(&list));
    let mut devices: Vec<String> = output_text(&list)
        .lines()
        .filter_map(|line| {
            if !line.starts_with("device=") {
                return None;
            }
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device=").map(|v| v.to_string()))
        })
        .collect();
    devices.sort();
    assert_eq!(devices.len(), 2, "expected two bob devices: {devices:?}");

    for device in devices.iter() {
        let trust = run(
            cfg,
            &[
                "contacts",
                "device",
                "trust",
                "--label",
                "bob",
                "--device",
                device.as_str(),
                "--confirm",
            ],
        );
        assert!(trust.status.success(), "{}", output_text(&trust));
    }

    (devices[0].clone(), devices[1].clone())
}

fn set_primary(cfg: &Path, device: &str) {
    let out = run(
        cfg,
        &[
            "contacts",
            "device",
            "primary",
            "set",
            "--label",
            "bob",
            "--device",
            device,
            "--confirm",
        ],
    );
    assert!(out.status.success(), "{}", output_text(&out));
}

fn timeline_first_item_id_and_state(cfg: &Path) -> (String, String) {
    let out = run(cfg, &["timeline", "list", "--peer", "bob", "--limit", "10"]);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    for line in text.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        let mut id = None;
        let mut state = None;
        for part in line.split_whitespace() {
            if let Some(value) = part.strip_prefix("id=") {
                id = Some(value.to_string());
            }
            if let Some(value) = part.strip_prefix("state=") {
                state = Some(value.to_string());
            }
        }
        if let (Some(id), Some(state)) = (id, state) {
            return (id, state);
        }
    }
    panic!("missing timeline item: {text}");
}

#[test]
fn receipt_apply_ignores_wrong_device_without_mutation_then_confirms_primary_target() {
    let root = unique_test_dir("na0217f_timeline_delivery_contract");
    let cfg = root.join("alice_cfg");
    ensure_dir_700(&root);
    init_cfg(&cfg);

    let (primary_device, wrong_device) = add_two_bob_devices(&cfg);
    set_primary(&cfg, primary_device.as_str());

    let server = common::start_inbox_server(1024 * 1024, 16);
    let payload = root.join("msg.bin");
    fs::write(&payload, b"na0217f-delivery-contract").expect("write payload");

    let send = run(
        &cfg,
        &[
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().expect("utf8"),
            "--receipt",
            "delivered",
        ],
    );
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );

    let (msg_id, before_state) = timeline_first_item_id_and_state(&cfg);
    assert_eq!(before_state, "SENT", "{}", send_text);

    let wrong_channel = format!("bob#{wrong_device}");
    let wrong = run(
        &cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            wrong_channel.as_str(),
            "--msg-id",
            msg_id.as_str(),
        ],
    );
    assert!(wrong.status.success(), "{}", output_text(&wrong));
    let wrong_text = output_text(&wrong);
    assert!(
        wrong_text.contains("QSC_RECEIPT_IGNORED reason=wrong_device"),
        "{}",
        wrong_text
    );

    let (_, after_wrong_state) = timeline_first_item_id_and_state(&cfg);
    assert_eq!(after_wrong_state, "SENT");

    let right_channel = format!("bob#{primary_device}");
    let right = run(
        &cfg,
        &[
            "util",
            "receipt-apply",
            "--peer",
            "bob",
            "--channel",
            right_channel.as_str(),
            "--msg-id",
            msg_id.as_str(),
        ],
    );
    assert!(right.status.success(), "{}", output_text(&right));
    let right_text = output_text(&right);
    assert!(
        right_text.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        right_text
    );

    let (_, final_state) = timeline_first_item_id_and_state(&cfg);
    assert_eq!(final_state, "DELIVERED", "{}", right_text);
}
