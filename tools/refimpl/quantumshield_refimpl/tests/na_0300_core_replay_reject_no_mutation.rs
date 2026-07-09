use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::ratchet::{
    Suite2DhRatchetState, Suite2RecvWireState, Suite2SendState,
};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::{recv_wire_canon, send_wire_canon, types};
use quantumshield_refimpl::RefimplError;
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};

const PLAINTEXT_SENTINEL: &[u8] = b"NA0300_PLAINTEXT_SENTINEL_DO_NOT_ECHO";

fn arr16(seed: u8) -> [u8; 16] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(3)).rotate_left(1))
}

fn arr32(seed: u8) -> [u8; 32] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(5)).rotate_left(1))
}

fn base_session() -> Suite2SessionState {
    let session_id = arr16(0x30);
    let dh_pub = arr32(0x40);
    let hk = arr32(0x50);
    let ck_ec = arr32(0x60);
    let ck_pq = arr32(0x70);

    Suite2SessionState {
        rk: arr32(0x80),
        send: Suite2SendState {
            session_id,
            protocol_version: types::SUITE2_PROTOCOL_VERSION,
            suite_id: types::SUITE2_SUITE_ID,
            dh_pub,
            hk_s: hk,
            ck_ec,
            ck_pq,
            ns: 0,
            pn: 0,
        },
        recv: Suite2RecvWireState {
            session_id,
            protocol_version: types::SUITE2_PROTOCOL_VERSION,
            suite_id: types::SUITE2_SUITE_ID,
            dh_pub,
            hk_r: hk,
            ck_ec,
            ck_pq_send: arr32(0x90),
            ck_pq_recv: ck_pq,
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        },
        dh: Suite2DhRatchetState::default(),
    }
}

fn recv_into_session(
    crypto: &StdCrypto,
    session: &mut Suite2SessionState,
    wire: &[u8],
) -> Result<Vec<u8>, RefimplError> {
    let out = recv_wire_canon(
        crypto,
        crypto,
        crypto,
        session.recv.clone(),
        &session.rk,
        wire,
        None,
        None,
    )?;
    session.recv = out.state;
    session.rk = out.rk;
    Ok(out.plaintext)
}

fn assert_no_sentinel_or_panic_text(err: &RefimplError) {
    let rendered = err.to_string();
    assert!(
        !rendered.contains("NA0300_PLAINTEXT_SENTINEL"),
        "reject text leaked plaintext sentinel"
    );
    assert!(
        !rendered.contains("panicked") && !rendered.contains("stack backtrace"),
        "reject text included panic/backtrace wording"
    );
}

fn reject_once_without_panic(
    crypto: &StdCrypto,
    session: &mut Suite2SessionState,
    wire: &[u8],
    label: &str,
) -> RefimplError {
    match catch_unwind(AssertUnwindSafe(|| {
        recv_into_session(crypto, session, wire)
    })) {
        Ok(Err(err)) => err,
        Ok(Ok(_)) => panic!("{label} unexpectedly accepted"),
        Err(_) => panic!("{label} panicked"),
    }
}

fn assert_reject_is_deterministic_and_no_mutation(
    crypto: &StdCrypto,
    session: &mut Suite2SessionState,
    wire: &[u8],
    expected_code: &str,
    label: &str,
) {
    let before = session.snapshot_bytes();
    let err1 = reject_once_without_panic(crypto, session, wire, label);
    assert_eq!(
        before,
        session.snapshot_bytes(),
        "{label} mutated accepted state on first reject"
    );

    let err2 = reject_once_without_panic(crypto, session, wire, label);
    assert_eq!(
        before,
        session.snapshot_bytes(),
        "{label} mutated accepted state on repeated reject"
    );

    assert_eq!(err1, err2, "{label} reject was not deterministic");
    assert_eq!(err1.code(), expected_code, "{label} reject code drifted");
    assert_no_sentinel_or_panic_text(&err1);
    assert_no_sentinel_or_panic_text(&err2);
}

#[test]
fn suite2_wire_replay_malformed_and_unsupported_rejects_are_no_mutation() {
    let crypto = StdCrypto;
    let mut session = base_session();

    let send = send_wire_canon(
        &crypto,
        &crypto,
        &crypto,
        session.send.clone(),
        0,
        PLAINTEXT_SENTINEL,
    )
    .expect("valid send wire");
    session.send = send.state.clone();

    let before_accept = session.snapshot_bytes();
    let plaintext =
        recv_into_session(&crypto, &mut session, &send.wire).expect("valid receive path");
    assert_eq!(plaintext.as_slice(), PLAINTEXT_SENTINEL);
    assert_ne!(
        before_accept,
        session.snapshot_bytes(),
        "accepted control path did not advance receive state"
    );

    let accepted_snapshot = session.snapshot_bytes();
    assert_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &send.wire,
        "REJECT_S2_REPLAY",
        "replayed Suite-2 wire",
    );
    assert_eq!(accepted_snapshot, session.snapshot_bytes());
    println!("NA0300_REPLAY_REJECT_OK");

    let malformed_wire = PLAINTEXT_SENTINEL.to_vec();
    assert_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &malformed_wire,
        "REJECT_S2_PARSE_PREFIX",
        "malformed sentinel wire",
    );
    println!("NA0300_MALFORMED_REJECT_OK");

    let mut downgrade_like_wire = send.wire.clone();
    downgrade_like_wire[0..2].copy_from_slice(&0x0400u16.to_be_bytes());
    assert_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &downgrade_like_wire,
        "REJECT_S2_PARSE_PREFIX",
        "downgrade-like protocol version wire",
    );

    let mut unsupported_flags_wire = send.wire.clone();
    let flags_offset = 10 + 32;
    unsupported_flags_wire[flags_offset..flags_offset + 2]
        .copy_from_slice(&0x8000u16.to_be_bytes());
    assert_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &unsupported_flags_wire,
        "REJECT_S2_PARSE_FLAGS",
        "unsupported flags wire",
    );

    println!("NA0300_NO_MUTATION_ON_REJECT_OK");
    println!("NA0300_NO_PANIC_OK");
    println!("NA0300_NO_SECRET_LEAK_OK");
    println!("NA0300_CORE_REPLAY_REJECT_NO_MUTATION_OK");
}
