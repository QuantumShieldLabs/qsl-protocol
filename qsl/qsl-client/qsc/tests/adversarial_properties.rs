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

proptest! {
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
