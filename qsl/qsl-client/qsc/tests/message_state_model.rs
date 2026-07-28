mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";
const ROUTE_TOKEN_MALLORY: &str = "route_token_mallory_abcdefghijk";

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

fn ensure_dir_700(path: &Path) {
    let _ = fs::create_dir_all(path);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn create_dir_700(path: &Path) {
    let _ = fs::remove_dir_all(path);
    ensure_dir_700(path);
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

/// NA-0682 (D617, ruled on STOP 019) — FIRST-PARTY message-id acquisition.
///
/// ⚠ THE RETIRED FORM, AND WHY IT WAS RETIRED. This test used to learn the message id by
/// scraping `id=` out of the `event=timeline_item` DIAGNOSTIC MARKER
/// (`timeline_first_item_id_and_state`). That coupled the test to REDACTION POLICY: the marker
/// layer redacts any value of >= 24 chars containing a digit
/// (`should_redact_value` -> `looks_high_cardinality`, `src/output/mod.rs:292`).
///
/// NA-0682 widened `msg_id` from 16 to 32 hex chars precisely to stop emitting the OLD id,
/// which was `sha512(plaintext)[..8]` — a fingerprint OF THE MESSAGE BODY, printed raw, that
/// slipped under the redactor only because 16 < 24 (the C17 leak, closed by F1). The widened
/// id crosses the threshold, so the scrape silently returned the literal string `<redacted>`
/// and the test built its acks against THAT.
///
/// ⚠ The failure mode is the lesson: a redaction sentinel that PARSES AS A VALID IDENTIFIER is
/// a trap, not an error. The test did not fail at the scrape — it proceeded, and failed later,
/// in a different subsystem, with a misleading code (OBS-EY, OBS-FA).
///
/// The test IS the sender, so it now reads the id IT MINTED from its OWN store: message records
/// are `msgqueue_v1/<contact>/<seq:020>_<msg_id>.rec` and persist in state SENT after a
/// successful send. No marker, no redactor, no new shipped surface.
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

fn qsc_base(cfg: &Path) -> Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    cmd
}

fn timeline_first_item_id_and_state(text: &str) -> Option<(String, String)> {
    for line in text.lines() {
        if !line.contains("event=timeline_item") {
            continue;
        }
        let mut id = None;
        let mut state = None;
        for part in line.split_whitespace() {
            if let Some(v) = part.strip_prefix("id=") {
                id = Some(v.to_string());
            }
            if let Some(v) = part.strip_prefix("state=") {
                state = Some(v.to_string());
            }
        }
        if let (Some(i), Some(s)) = (id, state) {
            return Some((i, s));
        }
    }
    None
}

fn assert_no_secrets(text: &str) {
    let upper = text.to_ascii_uppercase();
    for forbidden in ["TOKEN", "SECRET", "PASS", "PRIVATE", "BEARER", "CREDENTIAL"] {
        assert!(
            !upper.contains(forbidden),
            "found forbidden pattern {} in output: {}",
            forbidden,
            text
        );
    }
}

fn leak_counts(text: &str) -> (usize, usize) {
    let v1_count = text.matches("/v1/").count();
    let mut long_hex = 0usize;
    let mut run = 0usize;
    for ch in text.chars() {
        if ch.is_ascii_hexdigit() {
            run = run.saturating_add(1);
        } else {
            if run >= 32 {
                long_hex = long_hex.saturating_add(1);
            }
            run = 0;
        }
    }
    if run >= 32 {
        long_hex = long_hex.saturating_add(1);
    }
    (v1_count, long_hex)
}

fn write_ack_payload(path: &Path, msg_id: &str) {
    let payload = format!(
        "{{\"v\":1,\"t\":\"ack\",\"kind\":\"delivered\",\"msg_id\":\"{}\"}}",
        msg_id
    );
    fs::write(path, payload.as_bytes()).unwrap();
}

fn contacts_route_set(cfg: &Path, label: &str, token: &str) {
    let out = qsc_base(cfg)
        .args([
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            "fp-pinned-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add pinned");
    assert!(out.status.success(), "{}", output_text(&out));
    let list = qsc_base(cfg)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    let device_id = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"));
    let trust = qsc_base(cfg)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id,
            "--confirm",
        ])
        .output()
        .expect("contacts device trust");
    assert!(trust.status.success(), "{}", output_text(&trust));
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = qsc_base(cfg)
        .args(["relay", "inbox-set", "--token", token])
        .output()
        .expect("relay inbox set");
    assert!(out.status.success(), "{}", output_text(&out));
}

#[test]
fn honest_delivery_requires_explicit_ack() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_honest_delivery_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&alice_cfg, "mallory", ROUTE_TOKEN_MALLORY);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-honest-delivery").unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );
    assert!(send_text.contains(" peer=bob "), "{}", send_text);

    let alice_list_before = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list before");
    let before_text = output_text(&alice_list_before);
    assert!(alice_list_before.status.success(), "{}", before_text);
    let (_, state_before) = timeline_first_item_id_and_state(&before_text).expect("timeline item");
    assert_eq!(state_before, "SENT", "{}", before_text);

    let bob_recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

    let alice_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_recv_text = output_text(&alice_recv);
    assert!(
        !alice_recv_text.contains("event=receipt_recv"),
        "{}",
        alice_recv_text
    );
    assert!(
        !alice_recv_text.contains("to=DELIVERED"),
        "{}",
        alice_recv_text
    );
    assert!(
        !alice_recv_text.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv_text
    );
    let mut combined = String::new();
    combined.push_str(&send_text);
    combined.push_str(&alice_recv_text);
    let (v1_count, long_hex_count) = leak_counts(&combined);
    assert_eq!(v1_count, 0, "unexpected /v1/ leak in output: {combined}");
    assert_eq!(
        long_hex_count, 0,
        "unexpected long hex token leak in output: {combined}"
    );

    let alice_list_after = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after");
    let after_text = output_text(&alice_list_after);
    assert!(alice_list_after.status.success(), "{}", after_text);
    let (_, state_after) = timeline_first_item_id_and_state(&after_text).expect("timeline item");
    assert_eq!(state_after, "SENT", "{}", after_text);
}

#[test]
fn wrong_peer_ack_rejected_no_mutation() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_wrong_peer_ack_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let mallory_cfg = base.join("mallory_cfg");
    let alice_out = base.join("alice_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&mallory_cfg);
    create_dir_700(&alice_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    common::init_mock_vault(&mallory_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    contacts_route_set(&mallory_cfg, "alice", ROUTE_TOKEN_ALICE);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);
    relay_inbox_set(&mallory_cfg, ROUTE_TOKEN_MALLORY);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-wrong-peer-ack").unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let list = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    let list_text = output_text(&list);
    let (msg_id, state_before) = timeline_first_item_id_and_state(&list_text).expect("timeline");
    assert_eq!(state_before, "SENT", "{}", list_text);

    let forged = base.join("forged_ack.json");
    write_ack_payload(&forged, &msg_id);
    let forge_send = qsc_base(&mallory_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--file",
            forged.to_str().unwrap(),
        ])
        .output()
        .expect("forged send");
    assert!(forge_send.status.success(), "{}", output_text(&forge_send));

    let recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "mallory",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive forged ack");
    assert!(!recv.status.success(), "{}", output_text(&recv));
    let recv_text = output_text(&recv);
    assert!(
        recv_text.contains("event=qsp_unpack code=qsp_hdr_auth_failed ok=false"),
        "{}",
        recv_text
    );

    let list_after = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after");
    let after_text = output_text(&list_after);
    let (_, state_after) = timeline_first_item_id_and_state(&after_text).expect("timeline after");
    assert_eq!(state_after, "SENT", "{}", after_text);
}

#[test]
fn replay_ack_does_not_advance_state() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0118_replay_ack_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0118-replay-ack").unwrap();

    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    // NA-0682: id acquired FIRST-PARTY (see `first_party_sent_msg_id`), not scraped.
    let msg_id = first_party_sent_msg_id(&alice_cfg);

    let bob_recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
        ])
        .output()
        .expect("bob recv ack");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

    let first_ack = base.join("first_ack.json");
    write_ack_payload(&first_ack, &msg_id);
    let first_ack_send = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            first_ack.to_str().unwrap(),
        ])
        .output()
        .expect("first ack send");
    assert!(
        first_ack_send.status.success(),
        "{}",
        output_text(&first_ack_send)
    );

    let alice_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice recv ack");
    let alice_recv_text = output_text(&alice_recv);
    assert!(alice_recv.status.success(), "{}", alice_recv_text);
    assert!(
        alice_recv_text.contains("event=receipt_recv"),
        "{}",
        alice_recv_text
    );
    assert!(
        alice_recv_text.contains("QSC_DELIVERY state=peer_confirmed"),
        "{}",
        alice_recv_text
    );
    assert!(
        alice_recv_text.contains(" peer=bob "),
        "{}",
        alice_recv_text
    );

    let list2 = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list delivered");
    let list2_text = output_text(&list2);
    let (_, delivered_state) =
        timeline_first_item_id_and_state(&list2_text).expect("timeline item delivered");
    assert_eq!(delivered_state, "DELIVERED", "{}", list2_text);

    let replay = base.join("replay_ack.json");
    write_ack_payload(&replay, &msg_id);
    let replay_send = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            replay.to_str().unwrap(),
        ])
        .output()
        .expect("replay send");
    assert!(
        replay_send.status.success(),
        "{}",
        output_text(&replay_send)
    );

    let replay_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("replay recv");
    let replay_recv_text = output_text(&replay_recv);
    assert!(replay_recv.status.success(), "{}", replay_recv_text);
    // NA-0682 (D617, ruled on STOP 019) — second half of the same migration, recorded
    // rather than made silently. The RETIRED assertion embedded the raw id into the
    // expected MARKER text (`... reason=state_duplicate id={msg_id}`). That could only
    // ever pass while the marker layer PRINTED the id — i.e. it asserted the C17 leak
    // (`sha512(plaintext)[..8]`, raw in the logs) that F1 closed. With a properly
    // redacted id it is unsatisfiable by construction.
    //
    // The PROPERTY is unchanged and is asserted in three parts, none of which depend on
    // an identifier reaching the diagnostic surface:
    //   (a) the replay is rejected, and rejected AS A DUPLICATE (not as unknown);
    //   (b) ⚠ the marker does NOT echo the raw id — the leak must stay closed, so this
    //       is now asserted ON PURPOSE instead of depended upon;
    //   (c) the entry does not move: still exactly one item, still DELIVERED (below).
    assert!(
        replay_recv_text
            .contains("event=message_state_reject code=state_duplicate reason=state_duplicate"),
        "{}",
        replay_recv_text
    );
    assert!(
        !replay_recv_text.contains(msg_id.as_str()),
        "the reject marker must not print the raw message id (C17/F1): {}",
        replay_recv_text
    );

    let list3 = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list after replay");
    let list3_text = output_text(&list3);
    let (_, final_state) =
        timeline_first_item_id_and_state(&list3_text).expect("timeline item final");
    assert_eq!(final_state, "DELIVERED", "{}", list3_text);
}

/// NA-0682 (D617, ruled on STOP 019, item 2) — an ack that identifies NO message must
/// transition NOTHING.
///
/// ⚠ WHY THIS TEST EXISTS, recorded because the reason is the point. The property is real and
/// the shipped code already held it — but it was evidenced **only by an accident**: a test
/// whose id-scrape had silently degraded to the literal string `<redacted>` fed that garbage id
/// into ack-apply, and the correct refusal (`state_unknown`, zero mutation) appeared as an
/// incidental line inside a FAILING run. **Sole-evidence-by-accident is exactly what the audit
/// discipline exists to eliminate**, so the property is now asserted on purpose.
///
/// This is the honest-delivery-claim guard for the ack path: an ack naming an id this client
/// never sent must not be able to mark ANY message DELIVERED. `state_unknown` (no such id) and
/// `state_duplicate` (known id, already delivered) are DISTINCT causes and stay distinct words —
/// `replay_ack_does_not_advance_state` covers the other one.
#[test]
fn ack_for_unknown_msg_id_transitions_nothing() {
    let server = common::start_inbox_server(1024 * 1024, 16);
    let base = safe_test_root().join(format!("na0682_unknown_ack_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
    contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

    let payload = base.join("msg.bin");
    fs::write(&payload, b"na0682-unknown-ack").unwrap();
    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            payload.to_str().unwrap(),
            "--receipt",
            "delivered",
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "{}", output_text(&send));

    let real_msg_id = first_party_sent_msg_id(&alice_cfg);

    // Bob consumes the message WITHOUT `--emit-receipts`, so no genuine ack is ever produced.
    // The ONLY ack in the mailbox is the forged one below — that is what makes this test
    // non-vacuous rather than a race between two acks.
    let bob_recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            // ⚠ `"bob"`, not `"alice"`: in this fixture the peer label `bob` resolves to
            // ROUTE_TOKEN_BOB — the same mailbox both sides use — so the session is keyed on
            // that label at both ends. Same shape the sibling tests in this file use.
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob recv");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

    // An id of the SAME SHAPE as a real one (32 lowercase hex) that this client never minted.
    // Same shape matters: the refusal must be because the id is UNKNOWN, not because it is
    // malformed or because it tripped a length check.
    let unknown_id = "0123456789abcdef0123456789abcdef";
    assert_eq!(
        unknown_id.len(),
        real_msg_id.len(),
        "same shape as a real id"
    );
    assert_ne!(
        unknown_id,
        real_msg_id.as_str(),
        "and genuinely not the real one"
    );

    let forged = base.join("unknown_ack.json");
    write_ack_payload(&forged, unknown_id);
    let forged_send = qsc_base(&bob_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "bob",
            "--file",
            forged.to_str().unwrap(),
        ])
        .output()
        .expect("forged ack send");
    assert!(
        forged_send.status.success(),
        "{}",
        output_text(&forged_send)
    );

    let alice_recv = qsc_base(&alice_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "bob",
            "--max",
            "1",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice recv forged");
    let recv_text = output_text(&alice_recv);
    assert!(alice_recv.status.success(), "{}", recv_text);

    // (a) it is REFUSED, and refused as UNKNOWN -- not as a duplicate.
    assert!(
        recv_text.contains("event=message_state_reject code=state_unknown"),
        "{}",
        recv_text
    );
    assert!(
        !recv_text.contains("code=state_duplicate"),
        "an unknown id must not be reported as a duplicate: {}",
        recv_text
    );
    // (b) NOTHING is claimed delivered -- no receipt is recognised, no peer confirmation.
    assert!(
        !recv_text.contains("event=receipt_recv"),
        "a forged ack must not register as a receipt: {}",
        recv_text
    );
    assert!(
        !recv_text.contains("event=delivered_to_peer"),
        "a forged ack must not claim delivery: {}",
        recv_text
    );
    assert!(
        !recv_text.contains("QSC_DELIVERY state=peer_confirmed"),
        "a forged ack must not confirm the peer: {}",
        recv_text
    );

    // (c) ZERO MUTATION: the real message is still SENT, never advanced to DELIVERED.
    let list = qsc_base(&alice_cfg)
        .args(["timeline", "list", "--peer", "bob", "--limit", "10"])
        .output()
        .expect("timeline list");
    let list_text = output_text(&list);
    let (_, state) = timeline_first_item_id_and_state(&list_text).expect("timeline item");
    assert_eq!(
        state, "SENT",
        "a forged ack moved a real message's state: {}",
        list_text
    );
}

#[test]
fn state_markers_are_deterministic_and_secret_safe() {
    fn run_once(tag: &str) -> String {
        let server = common::start_inbox_server(1024 * 1024, 16);
        let base =
            safe_test_root().join(format!("na0118_determinism_{}_{}", tag, std::process::id()));
        create_dir_700(&base);
        let alice_cfg = base.join("alice_cfg");
        let bob_cfg = base.join("bob_cfg");
        let alice_out = base.join("alice_out");
        let bob_out = base.join("bob_out");
        create_dir_700(&alice_cfg);
        create_dir_700(&bob_cfg);
        create_dir_700(&alice_out);
        create_dir_700(&bob_out);
        common::init_mock_vault(&alice_cfg);
        common::init_mock_vault(&bob_cfg);
        contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN_BOB);
        contacts_route_set(&bob_cfg, "alice", ROUTE_TOKEN_ALICE);
        contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN_BOB);
        relay_inbox_set(&alice_cfg, ROUTE_TOKEN_ALICE);
        relay_inbox_set(&bob_cfg, ROUTE_TOKEN_BOB);

        let payload = base.join("msg.bin");
        fs::write(&payload, b"na0118-determinism").unwrap();

        let send = qsc_base(&alice_cfg)
            .args([
                "send",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--to",
                "bob",
                "--file",
                payload.to_str().unwrap(),
                "--receipt",
                "delivered",
            ])
            .output()
            .expect("send");
        assert!(send.status.success(), "{}", output_text(&send));

        let bob_recv = qsc_base(&bob_cfg)
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--mailbox",
                ROUTE_TOKEN_BOB,
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                bob_out.to_str().unwrap(),
                "--emit-receipts",
                "delivered",
            ])
            .output()
            .expect("bob receive");
        assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));

        let alice_recv = qsc_base(&alice_cfg)
            .args([
                "receive",
                "--transport",
                "relay",
                "--relay",
                server.base_url(),
                "--mailbox",
                ROUTE_TOKEN_ALICE,
                "--from",
                "bob",
                "--max",
                "1",
                "--out",
                alice_out.to_str().unwrap(),
            ])
            .output()
            .expect("alice receive");
        assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));

        let mut all = String::new();
        all.push_str(&output_text(&send));
        all.push_str(&output_text(&bob_recv));
        all.push_str(&output_text(&alice_recv));
        assert_no_secrets(&all);

        all.lines()
            .filter(|line| line.contains("event=message_state_"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    let a = run_once("a");
    let b = run_once("b");
    assert_eq!(
        a, b,
        "state markers not deterministic\nA:\n{}\nB:\n{}",
        a, b
    );
}
