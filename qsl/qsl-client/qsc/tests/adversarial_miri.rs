use qsc::adversarial::payload::{
    parse_attachment_confirm_payload, parse_file_transfer_payload, parse_receipt_payload,
};
use qsc::adversarial::route::{
    parse_http_request_bytes, parse_http_route_token_from_request, parse_http_target,
    HttpRelayTarget,
};
use qsc::adversarial::vault_format::parse_vault_envelope;
use qsc::envelope::{pack_bundle, tick_schedule};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};

const ZERO32: [u8; 32] = [0u8; 32];

fn assert_role_shape_matches_status_truth(role_is_a: bool, st: &Suite2SessionState) {
    assert_eq!(st.send.protocol_version, SUITE2_PROTOCOL_VERSION);
    assert_eq!(st.send.suite_id, SUITE2_SUITE_ID);
    assert_eq!(st.recv.protocol_version, SUITE2_PROTOCOL_VERSION);
    assert_eq!(st.recv.suite_id, SUITE2_SUITE_ID);
    assert_eq!(st.recv.nr, 0);

    if role_is_a {
        assert_ne!(st.send.ck_ec, ZERO32);
        assert_ne!(st.send.ck_pq, ZERO32);
        assert_eq!(st.recv.ck_ec, ZERO32);
        assert_eq!(st.recv.ck_pq_recv, ZERO32);
    } else {
        assert_eq!(st.send.ck_ec, ZERO32);
        assert_eq!(st.send.ck_pq, ZERO32);
        assert_ne!(st.recv.ck_ec, ZERO32);
        assert_ne!(st.recv.ck_pq_recv, ZERO32);
    }
}

#[test]
fn miri_http_parser_stays_header_authoritative() {
    let request =
        b"GET /v1/pull?max=2&route_token=ignored HTTP/1.1\r\nHost: localhost\r\nX-QSL-Route-Token: valid_route_token_value_1234\r\n\r\n";
    let parsed = parse_http_request_bytes(request).expect("request parses");
    assert_eq!(
        parse_http_target(parsed.target.as_str()),
        Some(HttpRelayTarget::Pull(2))
    );
    assert_eq!(
        parse_http_route_token_from_request(&parsed).expect("route token"),
        "valid_route_token_value_1234"
    );
}

#[test]
fn miri_payload_parsers_accept_expected_shapes_only() {
    let receipt = br#"{"v":1,"t":"data","kind":"delivered","msg_id":"m1","body":[1,2,3]}"#;
    assert_eq!(
        parse_receipt_payload(receipt).expect("receipt").msg_id,
        "m1"
    );

    let confirm =
        br#"{"v":1,"t":"ack","kind":"attachment_confirmed","attachment_id":"att1","confirm_handle":"h1"}"#;
    assert_eq!(
        parse_attachment_confirm_payload(confirm)
            .expect("attachment confirm")
            .attachment_id,
        "att1"
    );

    let manifest = br#"{"v":1,"t":"file_manifest","file_id":"f1","filename":"x","total_size":1,"chunk_count":1,"chunk_hashes":["h1"],"manifest_hash":"m1"}"#;
    assert!(parse_file_transfer_payload(manifest).is_some());
}

#[test]
fn miri_vault_parser_rejects_short_input() {
    assert_eq!(
        parse_vault_envelope(b"QSCV01").unwrap_err(),
        "vault_parse_failed"
    );
}

#[test]
fn miri_envelope_helpers_remain_deterministic() {
    let ticks = tick_schedule(3, 100, 4).expect("ticks");
    assert_eq!(ticks, vec![0, 100, 200]);
    let bundle = pack_bundle(&[8, 16, 32], 64, 3).expect("bundle");
    assert_eq!(bundle.total_len, 56);
    assert_eq!(bundle.bucket_len, 64);
}

#[test]
fn miri_establish_rejects_unauthenticated_commitment() {
    let crypto = StdCrypto;
    let err = match init_from_base_handshake(
        &crypto,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &[0x11; 16],
        &[0x22; 32],
        &[0x33; 32],
        &[0x44; 32],
        &[0x55; 32],
        false,
    ) {
        Err(err) => err,
        Ok(_) => panic!("unauthenticated establish must reject"),
    };
    assert_eq!(err, "REJECT_S2_ESTABLISH_UNAUTHENTICATED");
}

#[test]
fn miri_establish_role_shapes_survive_snapshot_roundtrip() {
    let crypto = StdCrypto;

    let initiator = init_from_base_handshake(
        &crypto,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &[0x11; 16],
        &[0x22; 32],
        &[0x33; 32],
        &[0x44; 32],
        &[0x55; 32],
        true,
    )
    .expect("initiator establish");
    assert_role_shape_matches_status_truth(true, &initiator);
    let initiator_restored =
        Suite2SessionState::restore_bytes(&initiator.snapshot_bytes()).expect("restore initiator");
    assert_role_shape_matches_status_truth(true, &initiator_restored);

    let responder = init_from_base_handshake(
        &crypto,
        false,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &[0x11; 16],
        &[0x22; 32],
        &[0x33; 32],
        &[0x55; 32],
        &[0x44; 32],
        true,
    )
    .expect("responder establish");
    assert_role_shape_matches_status_truth(false, &responder);
    let responder_restored =
        Suite2SessionState::restore_bytes(&responder.snapshot_bytes()).expect("restore responder");
    assert_role_shape_matches_status_truth(false, &responder_restored);
}
