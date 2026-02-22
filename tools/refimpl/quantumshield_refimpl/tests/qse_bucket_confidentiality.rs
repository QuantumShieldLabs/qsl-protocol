use quantumshield_refimpl::qsp::{
    QSP_PROTOCOL_VERSION, QSP_SUITE_ID, SZ_NONCE, SZ_SESSION_ID, SZ_X25519_PUB,
};
use quantumshield_refimpl::{Envelope, EnvelopeProfile, ProtocolMessage};

fn make_protocol_message(body_len: usize) -> Vec<u8> {
    let msg = ProtocolMessage {
        protocol_version: QSP_PROTOCOL_VERSION,
        suite_id: QSP_SUITE_ID,
        session_id: [0x11; SZ_SESSION_ID],
        dh_pub: [0x22; SZ_X25519_PUB],
        flags: 0,
        nonce_hdr: [0x33; SZ_NONCE],
        pq_adv_id: None,
        pq_adv_pub: None,
        pq_target_id: None,
        pq_ct: None,
        hdr_ct: vec![0x44; 24],
        body_ct: vec![0x55; body_len],
    };
    msg.encode()
}

fn make_bucketed(payload: Vec<u8>) -> Envelope {
    Envelope {
        env_version: 0x0100,
        flags: 0,
        route_token: b"route-token-fixed".to_vec(),
        timestamp_bucket: 42,
        payload,
        padding: vec![],
    }
    .pad_to_profile(EnvelopeProfile::Standard, &[0xAA; 2048])
    .expect("pad")
}

fn qse_header_prefix_len(route_token_len: usize) -> usize {
    // env_version + flags + varbytes_u16(route_token) + timestamp_bucket + pad_len + payload_len
    2 + 2 + 2 + route_token_len + 4 + 2 + 4
}

#[test]
fn bucket_mode_hides_exact_length_fields_for_same_profile() {
    let env_a = make_bucketed(make_protocol_message(32));
    let env_b = make_bucketed(make_protocol_message(176));
    let enc_a = env_a.encode();
    let enc_b = env_b.encode();

    assert_eq!(enc_a.len(), EnvelopeProfile::Standard.min_size_bytes());
    assert_eq!(enc_b.len(), EnvelopeProfile::Standard.min_size_bytes());

    let header_len = qse_header_prefix_len(env_a.route_token.len());
    assert_eq!(
        &enc_a[..header_len],
        &enc_b[..header_len],
        "header prefix must not vary with exact payload length in bucket mode"
    );
}

#[test]
fn bucket_mode_decode_recovers_payload_and_padding_split() {
    let payload = make_protocol_message(80);
    let env = make_bucketed(payload.clone());
    let encoded = env.encode();
    let decoded = Envelope::decode(&encoded).expect("decode");
    assert_eq!(decoded.payload, payload);
    assert_eq!(
        decoded.payload.len()
            + decoded.padding.len()
            + qse_header_prefix_len(decoded.route_token.len()),
        EnvelopeProfile::Standard.min_size_bytes()
    );
}

#[test]
fn bucket_mode_rejects_nonzero_cleartext_len_fields() {
    let env = make_bucketed(make_protocol_message(64));
    let mut encoded = env.encode();
    let route_len = env.route_token.len();
    let pad_len_off = 2 + 2 + 2 + route_len + 4;
    // Mutate cleartext pad_len from 0 -> 1; bucket-mode decoder must reject.
    encoded[pad_len_off] = 0;
    encoded[pad_len_off + 1] = 1;
    let err = Envelope::decode(&encoded).expect_err("expected reject");
    assert!(err.to_string().contains("bucket_len_fields"));
}
