mod common;

use common::VaultFixture;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const DESKTOP_PASS_ENV: &str = "QSC_DESKTOP_SESSION_PASSPHRASE";
const DESKTOP_PASSPHRASE: &str = "desktop-passphrase";
const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

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

fn unique_test_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    safe_test_root().join(format!("{tag}_{}_{}", std::process::id(), nonce))
}

fn output_text(out: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&out.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&out.stderr));
    s
}

fn init_vault(cfg: &Path) {
    common::init_passphrase_vault(cfg, "desktop-passphrase");
}

fn qsc_plain(cfg: &Path) -> Command {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn qsc_with_unlock(cfg: &Path) -> Command {
    let mut cmd = qsc_plain(cfg);
    cmd.env(DESKTOP_PASS_ENV, "desktop-passphrase")
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .arg("--unlock-passphrase-env")
        .arg(DESKTOP_PASS_ENV);
    cmd
}

// NA-0785 PLAN F03 / S6b: the message-surface helpers below drive a `common::VaultFixture` (an S4
// successor vault of `profile::ACTIVE`, isolated HOME/XDG/TMPDIR, its own unlock) instead of a bare
// QSC_CONFIG_DIR; each assertion is the base's, unchanged.
fn qsc_fx(v: &VaultFixture) -> Command {
    let mut cmd = v.command();
    cmd.env("QSC_MARK_FORMAT", "plain");
    cmd
}

fn identity_fp(v: &VaultFixture) -> String {
    let out = qsc_fx(v)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_fp=").map(ToOwned::to_owned))
        .unwrap_or_else(|| panic!("missing identity_fp: {}", output_text(&out)))
}

fn identity_kem_pk(v: &VaultFixture) -> String {
    let out = qsc_fx(v)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_kem_pk=").map(ToOwned::to_owned))
        .unwrap_or_else(|| panic!("missing identity_kem_pk: {}", output_text(&out)))
}

fn identity_sig_pk(v: &VaultFixture) -> String {
    let out = qsc_fx(v)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find_map(|line| line.strip_prefix("identity_sig_pk=").map(ToOwned::to_owned))
        .unwrap_or_else(|| panic!("missing identity_sig_pk: {}", output_text(&out)))
}

fn device_id(v: &VaultFixture, label: &str) -> String {
    let out = qsc_fx(v)
        .args(["contacts", "device", "list", "--label", label])
        .output()
        .expect("contacts device list");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|part| part.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device output: {}", output_text(&out)))
        .to_string()
}

fn trust_device(v: &VaultFixture, label: &str) {
    let device = device_id(v, label);
    let out = qsc_fx(v)
        .args([
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device.as_str(),
            "--confirm",
        ])
        .output()
        .expect("contacts device trust");
    assert!(out.status.success(), "{}", output_text(&out));
}

fn handshake_status(v: &VaultFixture, peer: &str) -> String {
    let out = qsc_fx(v)
        .args(["handshake", "status", "--peer", peer])
        .output()
        .expect("handshake status");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
}

fn advance_handshake_to_initiator_commit(relay: &str, alice: &VaultFixture, bob: &VaultFixture) {
    let alice_init = qsc_fx(alice)
        .args([
            "handshake",
            "init",
            "--as",
            "self",
            "--peer",
            "bob",
            "--relay",
            relay,
        ])
        .output()
        .expect("alice handshake init");
    assert!(alice_init.status.success(), "{}", output_text(&alice_init));

    let bob_poll = qsc_fx(bob)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("bob handshake poll");
    assert!(bob_poll.status.success(), "{}", output_text(&bob_poll));

    let alice_poll = qsc_fx(alice)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "bob",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("alice handshake poll");
    assert!(alice_poll.status.success(), "{}", output_text(&alice_poll));
}

fn confirm_handshake_at_responder(relay: &str, bob: &VaultFixture) {
    let bob_confirm = qsc_fx(bob)
        .args([
            "handshake",
            "poll",
            "--as",
            "self",
            "--peer",
            "alice",
            "--relay",
            relay,
            "--max",
            "4",
        ])
        .output()
        .expect("bob handshake confirm");
    assert!(
        bob_confirm.status.success(),
        "{}",
        output_text(&bob_confirm)
    );
}

#[test]
fn desktop_gui_profile_surface_is_deterministic() {
    let base = unique_test_dir("na0215b_profile_surface");
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let rotate = qsc_with_unlock(&cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(rotate.status.success(), "{}", output_text(&rotate));

    let doctor = qsc_plain(&cfg)
        .args(["doctor", "--check-only"])
        .output()
        .expect("doctor");
    assert!(doctor.status.success(), "{}", output_text(&doctor));
    let doctor_text = output_text(&doctor);
    assert!(doctor_text.contains("event=doctor"), "{}", doctor_text);
    assert!(doctor_text.contains("dir_exists=true"), "{}", doctor_text);
    assert!(doctor_text.contains("symlink_safe=true"), "{}", doctor_text);
    assert!(doctor_text.contains("parent_safe=true"), "{}", doctor_text);

    let vault = qsc_plain(&cfg)
        .args(["vault", "status"])
        .output()
        .expect("vault status");
    assert!(vault.status.success(), "{}", output_text(&vault));
    let vault_text = output_text(&vault);
    assert!(vault_text.contains("event=vault_status"), "{}", vault_text);
    assert!(vault_text.contains("present=true"), "{}", vault_text);
    assert!(
        vault_text.contains("key_source=passphrase"),
        "{}",
        vault_text
    );

    let show = qsc_with_unlock(&cfg)
        .args(["identity", "show"])
        .output()
        .expect("identity show");
    assert!(show.status.success(), "{}", output_text(&show));
    let show_text = output_text(&show);
    assert!(show_text.contains("event=identity_show"), "{}", show_text);
    assert!(show_text.contains("identity_fp="), "{}", show_text);
}

#[test]
fn desktop_gui_contact_device_surface_is_deterministic() {
    let base = unique_test_dir("na0215b_contact_surface");
    create_dir_700(&base);
    let cfg = base.join("cfg");
    create_dir_700(&cfg);
    init_vault(&cfg);

    let rotate = qsc_with_unlock(&cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("identity rotate");
    assert!(rotate.status.success(), "{}", output_text(&rotate));

    let add = qsc_with_unlock(&cfg)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            "00000000000000000000000000000000000000000000000000000000000000b0",
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "{}", output_text(&add));

    let list = qsc_with_unlock(&cfg)
        .args(["contacts", "list"])
        .output()
        .expect("contacts list");
    assert!(list.status.success(), "{}", output_text(&list));
    let list_text = output_text(&list);
    assert!(
        list_text.contains("event=contacts_list count=1"),
        "{}",
        list_text
    );
    assert!(list_text.contains("label=bob"), "{}", list_text);
    assert!(list_text.contains("blocked=false"), "{}", list_text);
    assert!(list_text.contains("device_count=1"), "{}", list_text);

    let devices = qsc_with_unlock(&cfg)
        .args(["contacts", "device", "list", "--label", "bob"])
        .output()
        .expect("contacts device list");
    assert!(devices.status.success(), "{}", output_text(&devices));
    let devices_text = output_text(&devices);
    assert!(
        devices_text.contains("event=contacts_device_list"),
        "{}",
        devices_text
    );
    assert!(devices_text.contains("device="), "{}", devices_text);
    assert!(devices_text.contains("state="), "{}", devices_text);
}

/// NA-0785 PLAN F03 / S6b -- FF3: delivery and timeline TRUTH over a REAL pair, with a RELAY-SIDE
/// RECEIPT WITHHOLD (`common::start_qsl_server_withholding`) around the real in-process leasing
/// relay.
///
/// The base obtained its "SENT, not yet DELIVERED" window from NA-0688's owed-receipt hold (bob,
/// with no send chain, could not send his receipt). The head needs no send chain for a receipt:
/// bob's receive answers with an NDR1 receipt at once (src/directional_delivery.rs:30-60, the
/// receipt key bound to the RECEIVING epoch), so that window no longer exists in the client. The
/// window is now made where it can really occur -- at the relay: bob's NDR1 receipt to alice is
/// HELD, and every UI-truth claim is asserted against that real receipt: not delivered while it is
/// withheld, delivered once it is released.
///
/// Vaults: the S4 successor helper (`profile::ACTIVE`) under the desktop passphrase. The test
/// drives the REAL handshake itself, because its pre- and mid-handshake UI states are claims of the
/// surface. No seeded session, no fabricated key. A synthetic local run.
#[test]
fn desktop_gui_message_surface_reports_delivery_and_timeline_truth() {
    let (server, withhold) = common::start_qsl_server_withholding(1024 * 1024, 16, 2);
    let alice = common::init_successor_vault("f03_s6b_desktop_alice", DESKTOP_PASSPHRASE);
    let bob = common::init_successor_vault("f03_s6b_desktop_bob", DESKTOP_PASSPHRASE);
    let base = alice.iso.root.join("run");
    create_dir_700(&base);
    let alice_out = base.join("alice_out");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_out);
    create_dir_700(&bob_out);

    let alice_rotate = qsc_fx(&alice)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("alice identity rotate");
    assert!(
        alice_rotate.status.success(),
        "{}",
        output_text(&alice_rotate)
    );
    let bob_rotate = qsc_fx(&bob)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .expect("bob identity rotate");
    assert!(bob_rotate.status.success(), "{}", output_text(&bob_rotate));

    let alice_fp = identity_fp(&alice);
    let alice_kem = identity_kem_pk(&alice);
    let alice_sig = identity_sig_pk(&alice);
    let bob_fp = identity_fp(&bob);
    let bob_kem = identity_kem_pk(&bob);
    let bob_sig = identity_sig_pk(&bob);

    let alice_inbox = qsc_fx(&alice)
        .args(["relay", "inbox-set", "--token", ROUTE_TOKEN_ALICE])
        .output()
        .expect("alice inbox set");
    assert!(
        alice_inbox.status.success(),
        "{}",
        output_text(&alice_inbox)
    );
    let bob_inbox = qsc_fx(&bob)
        .args(["relay", "inbox-set", "--token", ROUTE_TOKEN_BOB])
        .output()
        .expect("bob inbox set");
    assert!(bob_inbox.status.success(), "{}", output_text(&bob_inbox));

    let add_bob = qsc_fx(&alice)
        .args([
            "contacts",
            "add",
            "--label",
            "bob",
            "--fp",
            bob_fp.as_str(),
            "--kem-pk",
            bob_kem.as_str(),
            "--sig-pk",
            bob_sig.as_str(),
            "--route-token",
            ROUTE_TOKEN_BOB,
        ])
        .output()
        .expect("alice adds bob");
    assert!(add_bob.status.success(), "{}", output_text(&add_bob));
    let add_alice = qsc_fx(&bob)
        .args([
            "contacts",
            "add",
            "--label",
            "alice",
            "--fp",
            alice_fp.as_str(),
            "--kem-pk",
            alice_kem.as_str(),
            "--sig-pk",
            alice_sig.as_str(),
            "--route-token",
            ROUTE_TOKEN_ALICE,
        ])
        .output()
        .expect("bob adds alice");
    assert!(add_alice.status.success(), "{}", output_text(&add_alice));

    trust_device(&alice, "bob");
    trust_device(&bob, "alice");

    let payload = base.join("msg.txt");
    fs::write(&payload, "desktop gui contract").expect("payload write");

    let handshake_before = handshake_status(&alice, "bob");
    assert!(
        handshake_before.contains("event=handshake_status"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("status=no_session"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("send_ready=no"),
        "{}",
        handshake_before
    );
    assert!(
        handshake_before.contains("send_ready_reason=no_session"),
        "{}",
        handshake_before
    );

    let send_blocked = qsc_fx(&alice)
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
        .expect("send blocked");
    assert!(
        !send_blocked.status.success(),
        "{}",
        output_text(&send_blocked)
    );
    let send_blocked_text = output_text(&send_blocked);
    assert!(
        send_blocked_text.contains("event=error code=protocol_inactive reason=missing_seed"),
        "{}",
        send_blocked_text
    );

    let bob_recv_blocked = qsc_fx(&bob)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "4",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ])
        .output()
        .expect("bob receive blocked");
    assert!(
        !bob_recv_blocked.status.success(),
        "{}",
        output_text(&bob_recv_blocked)
    );
    let bob_recv_blocked_text = output_text(&bob_recv_blocked);
    assert!(
        bob_recv_blocked_text.contains("event=error code=protocol_inactive reason=missing_seed"),
        "{}",
        bob_recv_blocked_text
    );

    advance_handshake_to_initiator_commit(server.base_url(), &alice, &bob);

    let alice_mid = handshake_status(&alice, "bob");
    assert!(
        alice_mid.contains("status=awaiting_peer_confirm"),
        "{}",
        alice_mid
    );
    assert!(alice_mid.contains("peer_confirmed=no"), "{}", alice_mid);
    assert!(alice_mid.contains("send_ready=yes"), "{}", alice_mid);

    confirm_handshake_at_responder(server.base_url(), &bob);

    let alice_ready = handshake_status(&alice, "bob");
    assert!(
        alice_ready.contains("status=awaiting_peer_confirm"),
        "{}",
        alice_ready
    );
    assert!(alice_ready.contains("peer_confirmed=no"), "{}", alice_ready);
    assert!(alice_ready.contains("send_ready=yes"), "{}", alice_ready);

    let bob_ready = handshake_status(&bob, "alice");
    assert!(
        bob_ready.contains("status=established_recv_only"),
        "{}",
        bob_ready
    );
    assert!(bob_ready.contains("peer_confirmed=yes"), "{}", bob_ready);
    assert!(bob_ready.contains("send_ready=no"), "{}", bob_ready);
    assert!(
        bob_ready.contains("send_ready_reason=chainkey_unset"),
        "{}",
        bob_ready
    );

    // ⚠ THE WITHHOLD: from here on, every NDR1 receipt addressed to alice is held at the relay.
    withhold.withhold(|route, body| route == ROUTE_TOKEN_ALICE && body.starts_with(b"NDR1"));

    let send = qsc_fx(&alice)
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
        .expect("send message");
    assert!(send.status.success(), "{}", output_text(&send));
    let send_text = output_text(&send);
    assert!(
        send_text.contains("QSC_DELIVERY state=accepted_by_relay"),
        "{}",
        send_text
    );

    let bob_recv = qsc_fx(&bob)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_BOB,
            "--from",
            "alice",
            "--max",
            "4",
            "--out",
            bob_out.to_str().unwrap(),
            "--emit-receipts",
            "delivered",
            "--receipt-mode",
            "immediate",
        ])
        .output()
        .expect("bob receive");
    assert!(bob_recv.status.success(), "{}", output_text(&bob_recv));
    let bob_recv_text = output_text(&bob_recv);
    assert!(
        bob_recv_text.contains("event=recv_commit"),
        "{}",
        bob_recv_text
    );

    // =====================================================================================
    // ARM 1 — ⚠ THE WINDOW, PINNED AS CORRECT BEHAVIOUR RATHER THAN LEFT AS AN ABSENCE.
    //
    // Bob HAS confirmed -- his NDR1 receipt left him -- but the relay is withholding it. The message
    // is therefore SENT and not DELIVERED, and that is **correct**, not a failure. Asserting it
    // explicitly is the point: a bare "peer_confirmed is absent" would also pass if the receipt had
    // been silently LOST, which is why ARM 2 proves the very same receipt still arrives.
    let alice_recv_pre = qsc_fx(&alice)
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
            "4",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive (pre-reply)");
    assert!(
        alice_recv_pre.status.success(),
        "{}",
        output_text(&alice_recv_pre)
    );
    assert!(
        !output_text(&alice_recv_pre).contains("QSC_DELIVERY state=peer_confirmed"),
        "a recipient who has never sent cannot confirm delivery yet: {}",
        output_text(&alice_recv_pre)
    );
    assert!(
        !output_text(&alice_recv_pre).contains("to=DELIVERED"),
        "no DELIVERED transition while the receipt is withheld: {}",
        output_text(&alice_recv_pre)
    );
    let timeline_pre = qsc_fx(&alice)
        .args(["timeline", "list", "--peer", "bob", "--limit", "8"])
        .output()
        .expect("timeline list (pre-reply)");
    assert!(
        timeline_pre.status.success(),
        "{}",
        output_text(&timeline_pre)
    );
    let timeline_pre_text = output_text(&timeline_pre);
    assert!(
        timeline_pre_text.contains("state=SENT"),
        "the surface must show the message SENT while the receipt is owed — the window is real \
         and it is correct: {timeline_pre_text}"
    );
    assert!(
        !timeline_pre_text.contains("state=peer_confirmed"),
        "and it must not claim delivery it has no evidence for: {timeline_pre_text}"
    );
    assert!(
        !timeline_pre_text.contains("state=DELIVERED"),
        "nor show DELIVERED while the receipt is withheld: {timeline_pre_text}"
    );

    // =====================================================================================
    // ARM 2 — ⚠ AND THE CONFIRMATION DOES ARRIVE.
    //
    // Bob's first real send still works; the receipt he emitted for alice's ORIGINAL message is
    // held at the relay, not lost; once released it reaches alice, who moves that message to
    // DELIVERED -- end to end at the GUI-contract layer, against a real receipt.
    let bob_reply = base.join("bob_reply.bin");
    fs::write(&bob_reply, b"bob-first-reply").expect("write bob reply");
    let bob_send = qsc_fx(&bob)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "alice",
            "--file",
            bob_reply.to_str().unwrap(),
        ])
        .output()
        .expect("bob first send");
    assert!(bob_send.status.success(), "{}", output_text(&bob_send));
    // REPLACED (the head has no owed-receipt flush, `event=receipt_flush`): bob's receipt for
    // alice's message LEFT bob and is held at the relay -- exactly one NDR1 receipt to alice.
    let held: Vec<_> = withhold
        .held()
        .into_iter()
        .filter(|h| h.route == ROUTE_TOKEN_ALICE && h.body.starts_with(b"NDR1"))
        .collect();
    assert_eq!(
        held.len(),
        1,
        "bob's receipt must have left him and be held, not lost: {held:?}"
    );
    withhold.stop();
    withhold.release(held[0].id);

    let alice_recv = qsc_fx(&alice)
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
            "4",
            "--out",
            alice_out.to_str().unwrap(),
        ])
        .output()
        .expect("alice receive");
    assert!(alice_recv.status.success(), "{}", output_text(&alice_recv));
    let alice_recv_text = output_text(&alice_recv);
    // REPLACED (the head's receive reports the confirmation as the message's state transition; the
    // QSC_DELIVERY line is printed by the timeline below): SENT -> DELIVERED on the released receipt.
    assert!(
        alice_recv_text.contains("event=message_state_transition from=SENT to=DELIVERED"),
        "{}",
        alice_recv_text
    );

    let timeline = qsc_fx(&alice)
        .args(["timeline", "list", "--peer", "bob", "--limit", "8"])
        .output()
        .expect("timeline list");
    assert!(timeline.status.success(), "{}", output_text(&timeline));
    let timeline_text = output_text(&timeline);
    assert!(
        timeline_text.contains("event=timeline_list"),
        "{}",
        timeline_text
    );
    assert!(
        timeline_text.contains("event=timeline_item"),
        "{}",
        timeline_text
    );
    assert!(
        timeline_text.contains("state=peer_confirmed"),
        "{}",
        timeline_text
    );
    assert!(
        timeline_text.contains("state=DELIVERED"),
        "{}",
        timeline_text
    );
}
