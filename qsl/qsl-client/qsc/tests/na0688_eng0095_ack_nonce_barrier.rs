// NA-0688 / D622 C0 — ENG-0095: the ack path's nonce barrier.
//
// ⚠ THE PROPERTY, not the mechanism: a delivery receipt that was PACKED must advance the
// send chain durably BEFORE its bytes go to the relay. If the advance is dropped when the
// push fails, the next send on that chain reuses the same message key — and if the
// abandoned ciphertext reached the relay (push sent, response lost — the common path), two
// ciphertexts exist under one AEAD key.
//
// This is the same property NA-0155's `abort_burns_state_and_prevents_nonce_reuse_on_next_send`
// defended, and that `msgqueue::tests::abandoning_a_packed_message_advances_the_ratchet_first`
// and `outbox_abort::discard_burns_state_and_prevents_nonce_reuse_on_next_send` defend today
// on the QUEUE path. It was never defended on the RECEIPT path, which is what ENG-0095 records.
//
// ⚠ HOW IT IS PROVEN — a SINGLE-VARIABLE two-arm experiment, so the assertion cannot pass
// vacuously. Both arms run the identical script; the only difference is whether a receipt was
// attempted. The observable is the chain index of a LATER USER SEND (`ratchet_send_advance
// msg_idx=`), which is exactly the nonce-relevant counter:
//
//   arm A (control) : no receipt requested, no ack attempted  -> bob's next send is at index k
//   arm B (subject) : receipt requested, ack PACKED then its push FORCED TO FAIL
//                     -> bob's next send must be at an index > k, because the ack consumed one
//
//   ack advance COMMITTED  (correct)  => idx_b >  idx_a   GREEN
//   ack advance DROPPED    (the bug)  => idx_b == idx_a   RED, and red for the RIGHT REASON:
//                                        the index the ack burned is handed out a second time.
//
// The failure is injected with `set_fail_pushes(1)` on the live test relay, armed AFTER
// alice's send has landed, so exactly one push fails and it is the ack's.

mod common;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN: &str = "route_token_bob_abcdefghijklmnopqr";

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    fs::create_dir_all(&root).unwrap();
    root
}

fn create_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o700));
    }
}

fn combined_output(output: &std::process::Output) -> String {
    let mut s = String::from_utf8_lossy(&output.stdout).to_string();
    s.push_str(&String::from_utf8_lossy(&output.stderr));
    s
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
            "fp-pinned-test",
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add pinned");
    assert!(out.status.success(), "{}", combined_output(&out));
}

fn send_msg(cfg: &Path, relay: &str, to: &str, file: &Path, with_receipt: bool) -> String {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            file.to_str().unwrap(),
        ]);
    if with_receipt {
        cmd.args(["--receipt", "delivered"]);
    }
    let out = cmd.output().expect("send output");
    assert!(
        out.status.success(),
        "send failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

fn recv_msg(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    emit_receipts: bool,
) -> String {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--max",
            "1",
            "--out",
            out_dir.to_str().unwrap(),
        ]);
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    } else {
        // ⚠ NA-0688 C3: EXPLICIT, because the default is no longer Off. This arm's whole job is
        // to attempt NO ack, so the two arms differ in exactly ONE variable. Inheriting a
        // default would quietly turn the control into a second subject arm, and the
        // single-variable experiment — the thing that proves the nonce barrier — would be gone.
        cmd.args(["--receipt-mode", "off"]);
    }
    let out = cmd.output().expect("receive output");
    assert!(
        out.status.success(),
        "receive failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

/// Pull `msg_idx=` out of the `ratchet_send_advance` marker a user send emits.
///
/// ⚠ ENG-0087 rule 2: a scrape may never proceed on the redaction sentinel. `msg_idx` is a
/// small integer and does not cross the redactor today, but routing through the shared
/// helper is what keeps that a fact about the CODE rather than about the value's width.
fn send_chain_index(send_output: &str) -> u64 {
    let raw = send_output
        .lines()
        .find(|line| line.contains("event=ratchet_send_advance"))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("msg_idx="))
        })
        .unwrap_or_else(|| panic!("no ratchet_send_advance marker in send output:\n{send_output}"));
    let raw = common::scraped_marker_value("msg_idx", raw);
    raw.parse::<u64>()
        .unwrap_or_else(|_| panic!("msg_idx is not a number: {raw:?}"))
}

/// One arm of the experiment. Returns the chain index of bob's user send.
///
/// `attempt_ack` is the SINGLE VARIABLE: when true, alice requests a receipt, bob is told to
/// emit one, and exactly one push (the ack's) is forced to fail.
fn arm(tag: &str, attempt_ack: bool) -> u64 {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0688_eng0095_{}_{}", tag, std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN);

    let msg = base.join("msg.bin");
    fs::write(&msg, b"na0688-eng0095-probe").unwrap();

    // 1. Alice's send must LAND, so it is never a candidate for the injected failure.
    send_msg(&alice_cfg, server.base_url(), "bob", &msg, attempt_ack);

    // 2. Arm exactly one push failure. The next push on this relay is bob's ack (a receive
    //    is a PULL, and the legacy ack-mode default posts nothing), so the failure lands on
    //    the ack and on nothing else.
    if attempt_ack {
        server.set_fail_pushes(1);
    }

    // 3. Bob receives. With `attempt_ack` the ack is PACKED — consuming a chain index — and
    //    its push then fails. The failure is SOFT by design, so the receive still succeeds;
    //    that is precisely why the dropped advance is invisible without this test.
    let bob_recv = recv_msg(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN,
        "bob",
        &bob_out,
        attempt_ack,
    );
    if attempt_ack {
        assert!(
            bob_recv.contains("event=receipt_send_failed"),
            "the injected failure must actually hit the ack push, or this arm proves \
             nothing: {bob_recv}"
        );
    } else {
        assert!(
            !bob_recv.contains("event=receipt_send"),
            "the control arm must attempt no ack at all: {bob_recv}"
        );
    }

    // 4. Clear the injector so bob's user send is unaffected.
    server.set_fail_pushes(0);

    // 5. Bob's next USER send. Its chain index is the observable.
    let bob_send = send_msg(&bob_cfg, server.base_url(), "bob", &msg, false);
    let idx = send_chain_index(&bob_send);

    let _ = fs::remove_dir_all(&base);
    idx
}

#[test]
fn an_ack_whose_push_failed_still_advances_the_send_chain() {
    let idx_control = arm("control", false);
    let idx_subject = arm("subject", true);

    assert!(
        idx_subject > idx_control,
        "ENG-0095 — NONCE REUSE ON THE RECEIPT PATH.\n\
         A delivery receipt was packed (burning chain index {idx_control}) and its push then \
         failed, but the ratchet advance was never committed — so bob's next user send was \
         handed the SAME index back.\n\
         control arm (no ack attempted): next send at msg_idx={idx_control}\n\
         subject arm (ack packed, push failed): next send at msg_idx={idx_subject}\n\
         Equal indices mean two ciphertexts can exist under one AEAD key. The receipt path \
         must commit its advance BEFORE pushing, as the user send path does \
         (`outbox_next_state_store`, transport/mod.rs)."
    );
}

// ---------------------------------------------------------------------------
// D622 C0 guards. None of these is satisfied "by construction" — each is a real pin,
// because a property defended only by the current shape of the code stops being defended
// the moment someone helpfully changes that shape.
// ---------------------------------------------------------------------------

/// Receive with an explicit `--max` and `--receipt-mode`, for the cadence pin.
fn recv_msg_full(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
    max: &str,
    emit_receipts: bool,
    receipt_mode: Option<&str>,
) -> String {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--max",
            max,
            "--out",
            out_dir.to_str().unwrap(),
        ]);
    if emit_receipts {
        cmd.args(["--emit-receipts", "delivered"]);
    }
    if let Some(m) = receipt_mode {
        cmd.args(["--receipt-mode", m]);
    }
    let out = cmd.output().expect("receive output");
    assert!(
        out.status.success(),
        "receive failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

fn timeline_list(cfg: &Path, peer: &str) -> String {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["timeline", "list", "--peer", peer, "--limit", "20"])
        .output()
        .expect("timeline list");
    assert!(
        out.status.success(),
        "timeline list failed: {}",
        combined_output(&out)
    );
    combined_output(&out)
}

/// A two-party fixture: alice sends `count` messages requesting receipts; bob receives them.
/// Returns (base dir, alice_cfg, bob_cfg, bob's receive output).
/// ⚠ The server is RETURNED, not dropped. `InboxTestServer` shuts the listener down in its
/// `Drop`, so a fixture that keeps it locally leaves every later pull talking to a dead port
/// — which is exactly how the first draft of `an_ack_never_provokes_an_ack_in_reply` failed,
/// with `relay_inbox_pull_failed` masquerading as a property violation.
fn ack_fixture(
    tag: &str,
    count: usize,
    max: &str,
    emit_receipts: bool,
    receipt_mode: Option<&str>,
) -> (
    PathBuf,
    PathBuf,
    PathBuf,
    String,
    String,
    common::InboxTestServer,
) {
    let server = common::start_inbox_server(1024 * 1024, 32);
    let base = safe_test_root().join(format!("na0688_{}_{}", tag, std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice_cfg");
    let bob_cfg = base.join("bob_cfg");
    let bob_out = base.join("bob_out");
    create_dir_700(&alice_cfg);
    create_dir_700(&bob_cfg);
    create_dir_700(&bob_out);
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "bob", ROUTE_TOKEN);
    contacts_route_set(&bob_cfg, "bob", ROUTE_TOKEN);

    for i in 0..count {
        let msg = base.join(format!("msg{i}.bin"));
        fs::write(&msg, format!("na0688-body-{i}").as_bytes()).unwrap();
        send_msg(&alice_cfg, server.base_url(), "bob", &msg, true);
    }
    let bob_recv = recv_msg_full(
        &bob_cfg,
        server.base_url(),
        ROUTE_TOKEN,
        "bob",
        &bob_out,
        max,
        emit_receipts,
        receipt_mode,
    );
    let url = server.base_url().to_string();
    (base, alice_cfg, bob_cfg, bob_recv, url, server)
}

/// GUARD 3 (D622 C0 requirement 3) — a receipt is UI-INVISIBLE: it creates no timeline entry.
///
/// ⚠ This pin is NOT redundant with the current implementation. It exists because the ruled
/// alternative — routing acks through `transport::relay_send_with_payload` — passes a
/// `TimelineSendIngest` unconditionally on success (`transport/mod.rs`), so a future
/// "let's use one send path" refactor would silently start writing `kind="file"` rows for
/// every ack. `DESIGN_outbox_delivery_v1` §5 requires the ack be "invisible in their UI".
/// This test is what makes that requirement fail loudly instead of drifting.
#[test]
fn an_ack_creates_no_timeline_entry_on_the_sender_of_the_ack() {
    let (base, _alice_cfg, bob_cfg, bob_recv, _url, _server) =
        ack_fixture("guard3", 1, "1", true, None);
    assert!(
        bob_recv.contains("event=receipt_send"),
        "the fixture must actually send an ack, or this guard proves nothing: {bob_recv}"
    );

    let tl = timeline_list(&bob_cfg, "bob");
    let outbound: Vec<&str> = tl
        .lines()
        .filter(|l| l.contains("event=timeline_item") && l.contains("dir=out"))
        .collect();
    assert!(
        outbound.is_empty(),
        "an ack must create NO timeline entry — DESIGN_outbox_delivery_v1 §5 requires it be \
         invisible in the UI, but bob's timeline gained {} outbound row(s):\n{}",
        outbound.len(),
        tl
    );
    let _ = fs::remove_dir_all(&base);
}

/// GUARD 4 (D622 C0 requirement 4) — an ack is NEVER itself acked. BEHAVIOURAL, not structural.
///
/// ⚠ The ruling is explicit that "the ack path takes no receipt parameter" is NOT a test.
/// That protection is an accident of the current signature and evaporates the moment the ack
/// path gains one (which the Path-1 alternative would have done). So this drives the real
/// scenario: alice receives bob's ack while she herself is configured to emit receipts, and
/// the assertion is that she sends nothing back. Without the no-recursion property this is an
/// infinite ack loop between two peers.
#[test]
fn an_ack_never_provokes_an_ack_in_reply() {
    let (base, alice_cfg, _bob_cfg, bob_recv, url, _server) =
        ack_fixture("guard4", 1, "1", true, None);
    assert!(
        bob_recv.contains("event=receipt_send"),
        "the fixture must actually send an ack: {bob_recv}"
    );

    let alice_out = base.join("alice_out");
    create_dir_700(&alice_out);
    // ⚠ alice is told to emit receipts. If an ack could provoke an ack, this is where it fires.
    let alice_recv = recv_msg_full(
        &alice_cfg,
        &url,
        ROUTE_TOKEN,
        "bob",
        &alice_out,
        "1",
        true,
        None,
    );
    assert!(
        alice_recv.contains("event=receipt_recv"),
        "alice must actually have processed the ack, or this guard is vacuous: {alice_recv}"
    );
    assert!(
        !alice_recv.contains("event=receipt_send"),
        "NO RECURSION: processing an ack must never enqueue or send an ack in reply, even \
         with receipts enabled — DESIGN_outbox_delivery_v1 §5 (\"never itself acked\"). \
         Alice answered bob's ack with one of her own:\n{alice_recv}"
    );
    let _ = fs::remove_dir_all(&base);
}

/// GUARD 6-REPLACEMENT (D622 C0 requirement 5, as re-ruled) — THE CADENCE TRUTH PIN.
///
/// ⚠ The guard this replaces ("receipt enqueue does not trigger its own immediate drain")
/// was STRUCK as vacuous: on this send path there is no enqueue, so it would have asserted
/// nothing. What IS true, and what R2a's third amendment now says, is that receipt sends are
/// COALESCED PER PULL — they happen in the end-of-pull flush, never inline with each message.
/// This pin asserts the property that is actually true, which is the point.
///
/// It is also the pin that keeps the honest-limit wording honest: if receipts ever started
/// going out inline, the per-message timing signal would be strictly worse than the docs say.
#[test]
fn receipt_sends_are_coalesced_into_the_end_of_pull_flush() {
    let (base, _alice_cfg, _bob_cfg, bob_recv, _url, _server) =
        ack_fixture("guard6", 2, "2", false, Some("batched"));

    let lines: Vec<&str> = bob_recv.lines().collect();
    let last_message_ingest = lines
        .iter()
        .rposition(|l| l.contains("event=recv_item"))
        .unwrap_or_else(|| panic!("no message was ingested: {bob_recv}"));
    let first_receipt_send = lines
        .iter()
        .position(|l| l.contains("event=receipt_send"))
        .unwrap_or_else(|| panic!("no receipt was sent: {bob_recv}"));
    let receipt_sends = lines
        .iter()
        .filter(|l| l.contains("event=receipt_send"))
        .count();

    assert_eq!(
        receipt_sends, 2,
        "both received messages must be acked: {bob_recv}"
    );
    assert!(
        first_receipt_send > last_message_ingest,
        "CADENCE: every receipt send must come AFTER the last message of the pull was \
         ingested — they are coalesced into the end-of-pull flush, not sent inline. \
         first receipt_send at line {first_receipt_send}, last ingest at line \
         {last_message_ingest}:\n{bob_recv}"
    );
    let _ = fs::remove_dir_all(&base);
}
