use qsc::adversarial::payload::{
    parse_attachment_confirm_payload, parse_file_transfer_payload, parse_receipt_payload,
};
use qsc::adversarial::route::{
    parse_http_request_bytes, parse_http_route_token_from_request, parse_http_target,
    HttpRelayTarget,
};
use qsc::adversarial::vault_format::parse_vault_envelope;
use qsc::envelope::{pack_bundle, tick_schedule};

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
