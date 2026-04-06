use proptest::prelude::*;
use proptest::string::string_regex;
use qsc::adversarial::payload::{
    parse_attachment_descriptor_payload, ATTACHMENT_DESCRIPTOR_TYPE, ATTACHMENT_DESCRIPTOR_VERSION,
};
use qsc::adversarial::route::{
    normalize_route_token, parse_http_request_bytes, parse_http_route_token_from_request,
    route_token_is_valid,
};
use qsc::adversarial::vault_format::parse_vault_envelope;
use qsc::envelope::{pack_bundle, BUCKET_SIZES};
use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};

const ZERO32: [u8; 32] = [0u8; 32];

fn bundle_limit_strategy() -> impl Strategy<Value = usize> {
    prop_oneof![
        Just(BUCKET_SIZES[0]),
        Just(BUCKET_SIZES[1]),
        Just(BUCKET_SIZES[2]),
        Just(BUCKET_SIZES[3]),
        Just(BUCKET_SIZES[4]),
        Just(BUCKET_SIZES[5]),
        Just(BUCKET_SIZES[6]),
    ]
}

fn suite2_establish(
    role_is_a: bool,
    session_id: [u8; 16],
    dh_init: [u8; 32],
    pq_init_ss: [u8; 32],
    dh_self_pub: [u8; 32],
    dh_peer_pub: [u8; 32],
    authenticated: bool,
) -> Result<Suite2SessionState, &'static str> {
    let crypto = StdCrypto;
    init_from_base_handshake(
        &crypto,
        role_is_a,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &session_id,
        &dh_init,
        &pq_init_ss,
        &dh_self_pub,
        &dh_peer_pub,
        authenticated,
    )
}

fn role_shape_matches_status_truth(role_is_a: bool, st: &Suite2SessionState) -> bool {
    if st.send.protocol_version != SUITE2_PROTOCOL_VERSION
        || st.send.suite_id != SUITE2_SUITE_ID
        || st.recv.protocol_version != SUITE2_PROTOCOL_VERSION
        || st.recv.suite_id != SUITE2_SUITE_ID
        || st.recv.nr != 0
    {
        return false;
    }

    if role_is_a {
        st.send.ck_ec != ZERO32
            && st.send.ck_pq != ZERO32
            && st.recv.ck_ec == ZERO32
            && st.recv.ck_pq_recv == ZERO32
    } else {
        st.send.ck_ec == ZERO32
            && st.send.ck_pq == ZERO32
            && st.recv.ck_ec != ZERO32
            && st.recv.ck_pq_recv != ZERO32
    }
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(24))]

    #[test]
    fn valid_route_tokens_round_trip(token in string_regex("[A-Za-z0-9_-]{22,128}").expect("regex")) {
        prop_assert!(route_token_is_valid(token.as_str()));
        prop_assert_eq!(normalize_route_token(token.as_str()).expect("route token"), token);
    }

    #[test]
    fn invalid_route_tokens_are_rejected(token in any::<String>()) {
        prop_assume!(!route_token_is_valid(token.as_str()));
        prop_assert!(normalize_route_token(token.as_str()).is_err());
    }

    #[test]
    fn bundle_invariants_hold(
        payloads in prop::collection::vec(0usize..2048, 0..24),
        max_bundle in bundle_limit_strategy(),
        max_count in 1usize..17
    ) {
        if let Ok(bundle) = pack_bundle(payloads.as_slice(), max_bundle, max_count) {
            let expected: Vec<usize> = payloads
                .iter()
                .copied()
                .take(max_count)
                .scan(0usize, |total, len| {
                    if len > max_bundle || total.saturating_add(len) > max_bundle {
                        None
                    } else {
                        *total += len;
                        Some(len)
                    }
                })
                .collect();
            let total: usize = expected.iter().sum();
            prop_assert_eq!(bundle.payload_lens, expected);
            prop_assert_eq!(bundle.total_len, total);
            prop_assert!(bundle.bucket_len >= bundle.total_len);
            prop_assert!(bundle.bucket_len <= max_bundle);
        }
    }

    #[test]
    fn query_parameters_never_supply_route_tokens(token in string_regex("[A-Za-z0-9_-]{22,128}").expect("regex")) {
        let request = format!(
            "GET /v1/pull?max=1&route_token={token} HTTP/1.1\r\nHost: localhost\r\n\r\n"
        );
        let parsed = parse_http_request_bytes(request.as_bytes()).expect("request parses");
        prop_assert_eq!(
            parse_http_route_token_from_request(&parsed).unwrap_err(),
            "missing_route_token"
        );
    }

    #[test]
    fn suite2_establish_rejects_unauthenticated_commitment(
        role_is_a in any::<bool>(),
        session_id in any::<[u8; 16]>(),
        dh_init in any::<[u8; 32]>(),
        pq_init_ss in any::<[u8; 32]>(),
        dh_self_pub in any::<[u8; 32]>(),
        dh_peer_pub in any::<[u8; 32]>()
    ) {
        let result = suite2_establish(
            role_is_a,
            session_id,
            dh_init,
            pq_init_ss,
            dh_self_pub,
            dh_peer_pub,
            false,
        );
        match result {
            Err(err) => prop_assert_eq!(err, "REJECT_S2_ESTABLISH_UNAUTHENTICATED"),
            Ok(_) => prop_assert!(false, "unauthenticated establish must reject"),
        }
    }

    #[test]
    fn suite2_establish_role_shapes_survive_snapshot_roundtrip(
        session_id in any::<[u8; 16]>(),
        dh_init in any::<[u8; 32]>(),
        pq_init_ss in any::<[u8; 32]>(),
        alice_dh_pub in any::<[u8; 32]>(),
        bob_dh_pub in any::<[u8; 32]>()
    ) {
        let initiator = suite2_establish(
            true,
            session_id,
            dh_init,
            pq_init_ss,
            alice_dh_pub,
            bob_dh_pub,
            true,
        )
        .expect("initiator establish");
        prop_assert!(role_shape_matches_status_truth(true, &initiator));

        let initiator_restored =
            Suite2SessionState::restore_bytes(&initiator.snapshot_bytes()).expect("restore initiator");
        prop_assert!(role_shape_matches_status_truth(true, &initiator_restored));

        let responder = suite2_establish(
            false,
            session_id,
            dh_init,
            pq_init_ss,
            bob_dh_pub,
            alice_dh_pub,
            true,
        )
        .expect("responder establish");
        prop_assert!(role_shape_matches_status_truth(false, &responder));

        let responder_restored =
            Suite2SessionState::restore_bytes(&responder.snapshot_bytes()).expect("restore responder");
        prop_assert!(role_shape_matches_status_truth(false, &responder_restored));
    }
}

#[test]
fn attachment_descriptor_accepts_canonical_shape() {
    let raw = format!(
        "{{\"v\":{ATTACHMENT_DESCRIPTOR_VERSION},\"t\":\"{ATTACHMENT_DESCRIPTOR_TYPE}\",\"attachment_id\":\"att1\",\"plaintext_len\":1,\"ciphertext_len\":2,\"part_size_class\":\"small\",\"part_count\":1,\"integrity_alg\":\"sha512_merkle_v1\",\"integrity_root\":\"root\",\"locator_kind\":\"service_ref_v1\",\"locator_ref\":\"loc\",\"fetch_capability\":\"cap\",\"enc_ctx_alg\":\"ctx\",\"enc_ctx_b64u\":\"ctxb64\",\"retention_class\":\"default\",\"expires_at_unix_s\":1,\"confirm_requested\":false}}"
    );
    let parsed = parse_attachment_descriptor_payload(raw.as_bytes()).expect("descriptor parses");
    assert_eq!(parsed.attachment_id, "att1");
}

#[test]
fn malformed_vault_envelope_rejects_without_panic() {
    assert!(parse_vault_envelope(b"QSCV01not-a-real-envelope").is_err());
}
