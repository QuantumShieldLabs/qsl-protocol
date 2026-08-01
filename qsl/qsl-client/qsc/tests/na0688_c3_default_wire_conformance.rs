// NA-0688 / D622 C3 — THE DEFAULT-CONFIGURATION WIRE CONFORMANCE PIN.
//
// ⚠ WHY THIS FILE EXISTS, AND WHY IT IS NOT `suite2_runtime_equivalence_na0198.rs`.
//
// That fixture pins the FROZEN CRYPTO CORE over a multi-step chain, and its worth IS its frozen,
// once-reviewed arithmetic. With delivery receipts on by default every RECEIVE also emits a SEND
// (the ack), so each receiver's send chain advances mid-fixture and all seven of its
// chain-arithmetic sites would need re-deriving. Doing that at C3 depth risks the one failure that
// matters most in a conformance pin -- a test that is GREEN AND PROVES NOTHING -- and it would
// permanently couple the crypto-core pin to control-plane behaviour, so every future receipt
// change would cascade back into it.
//
// So the concerns are SPLIT (operator ruling on STOP #020, option B). That fixture keeps its
// arithmetic untouched behind an explicit `--receipt off`; THIS file pins the default
// configuration, on a SINGLE exchange with no multi-step chain, so there is no arithmetic to
// re-derive and nothing to drift.
//
// ⚠ THE SPLIT IS NOT A COVERAGE TRADE. The default path keeps full refimpl coverage: the
// wire-equivalence assertion below runs the reference implementation over the FRAMED plaintext and
// compares bytes, which is the same property the frozen fixture asserts, asked once instead of
// four times.
//
// WHAT IS PINNED HERE, all four required by the ruling:
//   1. FRAMING ROUND-TRIP     -- the recipient writes out the ORIGINAL body, not the frame.
//   2. WIRE EQUIVALENCE       -- refimpl(framed plaintext) == the bytes the client actually sent.
//   3. THE COUPLING ASSERTION -- the client's QSP plaintext is EXACTLY the frame this file builds.
//   4. THE ACK'S PLAINTEXT    -- the determined ack frame, pinned as it sits on the wire.

mod common;

use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::crypto::traits::{Hash, Kmac};
use quantumshield_refimpl::qse::Envelope;
use quantumshield_refimpl::suite2::ratchet::{
    Suite2DhRatchetState, Suite2RecvWireState, Suite2SendState,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon};
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROUTE_TOKEN_PEER: &str = "route_token_peer_na0688_c3_conform";

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

fn kmac_out<const N: usize>(kmac: &StdCrypto, key: &[u8], label: &str, data: &[u8]) -> [u8; N] {
    let out = kmac.kmac256(key, label, data, N);
    out[..N].try_into().expect("kmac output")
}

/// The seeded (`QSC_QSP_SEED`) session the client derives, reconstructed independently.
///
/// Same shape as the copies in `qsp_qse_onwire.rs` and `suite2_runtime_equivalence_na0198.rs`;
/// the duplication is this suite's existing convention for it and is left as found rather than
/// refactored from inside a defaults lane.
fn qsp_session_for_channel(seed: u64, channel: &str) -> Suite2SessionState {
    let c = StdCrypto;
    let seed_bytes = seed.to_le_bytes();
    let seed_hash = c.sha512(&seed_bytes);
    let mut seed_key = [0u8; 32];
    seed_key.copy_from_slice(&seed_hash[..32]);

    let base = kmac_out::<32>(&c, &seed_key, "QSC.QSP.BASE", channel.as_bytes());
    let session_id = kmac_out::<16>(&c, &base, "QSC.QSP.SID", channel.as_bytes());
    let hk = kmac_out::<32>(&c, &base, "QSC.QSP.HK", b"");
    let ck_ec = kmac_out::<32>(&c, &base, "QSC.QSP.CK.EC", b"");
    let ck_pq = kmac_out::<32>(&c, &base, "QSC.QSP.CK.PQ", b"");
    let rk = kmac_out::<32>(&c, &base, "QSC.QSP.RK", b"");
    let dh_pub = kmac_out::<32>(&c, &base, "QSC.QSP.DH", b"");
    let dh_priv = kmac_out::<32>(&c, &base, "QSC.QSP.DH.PRIV", b"");

    let send = Suite2SendState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_s: hk,
        ck_ec,
        ck_pq,
        ns: 0,
        pn: 0,
    };
    let recv = Suite2RecvWireState {
        session_id,
        protocol_version: SUITE2_PROTOCOL_VERSION,
        suite_id: SUITE2_SUITE_ID,
        dh_pub,
        hk_r: hk,
        ck_ec,
        ck_pq_send: ck_pq,
        ck_pq_recv: ck_pq,
        nr: 0,
        role_is_a: true,
        peer_max_adv_id_seen: 0,
        known_targets: BTreeSet::new(),
        consumed_targets: BTreeSet::new(),
        tombstoned_targets: BTreeSet::new(),
        mkskipped: Vec::new(),
    };
    let dh = Suite2DhRatchetState {
        dhs_priv: dh_priv,
        dhs_pub: dh_pub,
        dhr: dh_pub,
    };
    Suite2SessionState { rk, send, recv, dh }
}

/// The msg_id THIS CLIENT MINTED, read first-party from its own msgqueue record.
///
/// The frame carries a 128-bit CSPRNG `msg_id`, so the framed plaintext cannot be predicted
/// before the send — it has to be read back. Records are
/// `msgqueue_v1/<contact>/<seq:020>_<msg_id>.rec`. First-party per ENG-0087, never scraped.
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
        "expected exactly one queued record to read the id from, got {found:?}"
    );
    found.pop().expect("one record")
}

/// The EXACT data-control frame a default `qsc send` puts inside the QSP plaintext.
///
/// ⚠ HAND-ROLLED AS LITERAL JSON ON PURPOSE, and this is the point of the whole file. Building it
/// from the crate's own `ReceiptControlPayload` would make the test agree with the code BY
/// CONSTRUCTION and catch no format drift whatsoever — the opposite of what a conformance pin is
/// for. Compact JSON, fields in declaration order, `body` as a byte list. This must match
/// `encode_data_payload_with_id` byte for byte, and if either side moves, this file says so.
fn framed_data_plaintext(msg_id: &str, body: &[u8]) -> Vec<u8> {
    let body_list = body
        .iter()
        .map(|b| b.to_string())
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"v\":2,\"t\":\"data\",\"kind\":\"delivered\",\"msg_id\":\"{msg_id}\",\"body\":[{body_list}],\"ns\":\"qsc.ctrl\"}}"
    )
    .into_bytes()
}

/// The EXACT delivery-ack frame, which is fully determined by the message it acknowledges.
///
/// ⚠ `body` is absent, not empty — `skip_serializing_if = "Option::is_none"` omits the field
/// entirely. Hand-rolled for the same reason as above.
fn framed_ack_plaintext(msg_id: &str) -> Vec<u8> {
    format!("{{\"v\":2,\"t\":\"ack\",\"kind\":\"delivered\",\"msg_id\":\"{msg_id}\",\"ns\":\"qsc.ctrl\"}}")
        .into_bytes()
}

fn qsc_base(cfg: &Path) -> std::process::Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    cmd
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

/// ONE default-configuration exchange, pinned end to end against the reference implementation.
///
/// No flags are passed anywhere: this is what two users who touch nothing actually put on the wire.
#[test]
fn a_default_exchange_frames_its_body_and_matches_the_refimpl_wire() {
    let server = common::start_inbox_server(1024 * 1024, 64);
    let base = safe_test_root().join(format!("na0688_c3_conform_{}", std::process::id()));
    create_dir_700(&base);
    let alice_cfg = base.join("alice");
    let bob_cfg = base.join("bob");
    let bob_out = base.join("bob_out");
    for d in [&alice_cfg, &bob_cfg, &bob_out] {
        create_dir_700(d);
    }
    common::init_mock_vault(&alice_cfg);
    common::init_mock_vault(&bob_cfg);
    contacts_route_set(&alice_cfg, "peer", ROUTE_TOKEN_PEER);
    contacts_route_set(&bob_cfg, "peer", ROUTE_TOKEN_PEER);
    relay_inbox_set(&alice_cfg, ROUTE_TOKEN_PEER);
    relay_inbox_set(&bob_cfg, ROUTE_TOKEN_PEER);

    let body = b"hello-default-configuration".to_vec();
    let body_path = base.join("body.bin");
    fs::write(&body_path, &body).expect("write body");

    // ---- alice sends, with NO flags at all -------------------------------------------------
    let send = qsc_base(&alice_cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--to",
            "peer",
            "--file",
            body_path.to_str().unwrap(),
        ])
        .output()
        .expect("alice send");
    assert!(send.status.success(), "{}", output_text(&send));

    let msg_id = first_party_sent_msg_id(&alice_cfg);
    let framed = framed_data_plaintext(&msg_id, &body);

    let queued = server.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(queued.len(), 1, "expected exactly one queued envelope");
    let env = Envelope::decode(&queued[0]).expect("decode alice envelope");

    // ---- 3. THE COUPLING ASSERTION ---------------------------------------------------------
    // Decrypt what the client REALLY sent, with the reference implementation, and require it to
    // be exactly the frame this file constructed from literal JSON. Without this, the equivalence
    // check below could agree with itself while both drifted from what the client emits.
    let decrypted = recv_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        qsp_session_for_channel(1, "peer").recv,
        &qsp_session_for_channel(1, "peer").rk,
        &env.payload,
        None,
        None,
    )
    .expect("refimpl decrypt of alice's envelope");
    assert_eq!(
        decrypted.plaintext, framed,
        "a default send's QSP plaintext must be exactly the data-control frame"
    );

    // ---- 2. WIRE EQUIVALENCE, on the FRAMED plaintext ---------------------------------------
    // Same property the frozen fixture asserts -- same plaintext in, same wire out, qsc ==
    // refimpl -- asked once, in the DEFAULT configuration. This is what keeps the split from
    // being a coverage trade.
    let expected = send_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        qsp_session_for_channel(1, "peer").send,
        0,
        &framed,
    )
    .expect("refimpl send");
    assert_eq!(
        env.payload, expected.wire,
        "a default send emitted a non-canonical Suite-2 wire"
    );

    // ---- bob receives, also with NO flags ---------------------------------------------------
    server.replace_channel(ROUTE_TOKEN_PEER, queued);
    let recv = qsc_base(&bob_cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            server.base_url(),
            "--mailbox",
            ROUTE_TOKEN_PEER,
            "--from",
            "peer",
            "--max",
            "1",
            "--out",
            bob_out.to_str().unwrap(),
        ])
        .output()
        .expect("bob receive");
    assert!(recv.status.success(), "{}", output_text(&recv));

    // ---- 1. FRAMING ROUND-TRIP --------------------------------------------------------------
    // The recipient writes out the ORIGINAL body. The frame is transport, never user content --
    // this is the assertion that would have caught a typed payload being delivered as a message.
    assert_eq!(
        fs::read(bob_out.join("recv_1.bin")).expect("bob output file"),
        body,
        "the recipient must write the original body, never the frame"
    );

    // ---- 4. THE ACK'S DETERMINED PLAINTEXT, ON THE WIRE -------------------------------------
    let ack_text = output_text(&recv);
    assert!(
        ack_text.contains("event=receipt_send"),
        "a default receive must ack, or the rest of this pin is vacuous:\n{ack_text}"
    );
    let acked = server.drain_channel(ROUTE_TOKEN_PEER);
    assert_eq!(acked.len(), 1, "expected exactly one ack envelope");
    let ack_env = Envelope::decode(&acked[0]).expect("decode ack envelope");
    let ack_decrypted = recv_wire_canon(
        &StdCrypto,
        &StdCrypto,
        &StdCrypto,
        qsp_session_for_channel(1, "peer").recv,
        &qsp_session_for_channel(1, "peer").rk,
        &ack_env.payload,
        None,
        None,
    )
    .expect("refimpl decrypt of the ack");
    assert_eq!(
        ack_decrypted.plaintext,
        framed_ack_plaintext(&msg_id),
        "the ack must be the determined frame naming the message it acknowledges, and nothing else"
    );
}
