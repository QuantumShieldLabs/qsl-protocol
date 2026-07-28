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
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
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

/// NA-0682 (D617, operator-ruled STOP 021) — FIRST-PARTY message-id acquisition.
///
/// ⚠ RETIRED FORM AND WHY. This file used to learn the message id by scraping `id=` out of the
/// `event=timeline_item` DIAGNOSTIC MARKER. The marker layer redacts any value of >= 24 chars
/// containing a digit (`should_redact_value` -> `looks_high_cardinality`,
/// `src/output/mod.rs:292`). NA-0682 widened `msg_id` to 32 hex chars precisely to stop
/// emitting the old `sha512(plaintext)[..8]` id — a fingerprint OF THE MESSAGE BODY (the C17
/// leak, closed by F1) — and the widened id crosses that threshold. The scrape therefore
/// returned the literal string `<redacted>`, and `qsc util receipt-apply --msg-id <redacted>`
/// failed with `state_unknown`.
///
/// ⚠ This is the SECOND confirmed instance of that class (ENG-0087); the first was
/// `message_state_model::replay_ack_does_not_advance_state`. It presented completely
/// differently — a `state_unknown` from a CLI verb rather than a wrong reject code — which is
/// why the class needs enumerating rather than fixing one symptom at a time.
///
/// The test IS the sender, so it reads the id IT MINTED from its OWN store: records live at
/// `msgqueue_v1/<contact>/<seq:020>_<msg_id>.rec` and persist after a successful send. No
/// marker, no redactor, no new shipped surface.
fn first_party_sent_msg_id(cfg: &Path) -> String {
    let root = cfg.join("msgqueue_v1");
    let mut found: Vec<String> = Vec::new();
    let contacts = fs::read_dir(&root).expect("msgqueue_v1 exists after a successful send");
    for contact in contacts.flatten() {
        let Ok(entries) = fs::read_dir(contact.path()) else {
            continue;
        };
        for e in entries.flatten() {
            let name = e.file_name().to_string_lossy().to_string();
            let Some(stem) = name.strip_suffix(".rec") else {
                continue;
            };
            let Some((_seq, id)) = stem.split_once('_') else {
                continue;
            };
            found.push(id.to_string());
        }
    }
    assert_eq!(
        found.len(),
        1,
        "expected exactly one message record to read the id from, got {:?}",
        found
    );
    // ⚠ Guard the migration itself: never accept the redaction sentinel as an id again.
    assert_ne!(
        found[0], "<redacted>",
        "first-party acquisition must never yield the redaction sentinel"
    );
    found.pop().expect("one record")
}

/// ⚠ STATE ONLY. The id is deliberately NOT returned: an identifier read from a diagnostic
/// marker is coupled to redaction policy (above), so this file acquires ids first-party and
/// scrapes only `state=`, which the marker layer does not redact.
fn timeline_first_item_state(cfg: &Path) -> String {
    let out = run(cfg, &["timeline", "list", "--peer", "bob", "--limit", "10"]);
    assert!(out.status.success(), "{}", output_text(&out));
    let text = output_text(&out);
    for line in text.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        for part in line.split_whitespace() {
            if let Some(value) = part.strip_prefix("state=") {
                return value.to_string();
            }
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

    let before_state = timeline_first_item_state(&cfg);
    // NA-0682: id acquired FIRST-PARTY (see `first_party_sent_msg_id`), never scraped.
    let msg_id = first_party_sent_msg_id(&cfg);
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

    let after_wrong_state = timeline_first_item_state(&cfg);
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

    let final_state = timeline_first_item_state(&cfg);
    assert_eq!(final_state, "DELIVERED", "{}", right_text);
}
