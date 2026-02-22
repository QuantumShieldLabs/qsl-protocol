use quantumshield_refimpl::{Envelope, EnvelopeProfile};

fn make_suite2_wire(body_len: usize) -> Vec<u8> {
    let header_len = 58usize; // 32 dh_pub + 2 flags + 24 hdr_ct
    let body_len = body_len.max(16); // suite2 minimum body ciphertext length
    let mut out = Vec::with_capacity(10 + header_len + body_len);
    out.extend_from_slice(&0x0500u16.to_be_bytes()); // protocol_version
    out.extend_from_slice(&0x0002u16.to_be_bytes()); // suite_id
    out.push(0x02); // msg_type ratchet
    out.push(0x00); // envelope flags
    out.extend_from_slice(&(header_len as u16).to_be_bytes());
    out.extend_from_slice(&(body_len as u16).to_be_bytes());
    out.extend(std::iter::repeat_n(0x44, header_len));
    out.extend(std::iter::repeat_n(0x55, body_len));
    out
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
    let env_a = make_bucketed(make_suite2_wire(32));
    let env_b = make_bucketed(make_suite2_wire(176));
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
    let payload = make_suite2_wire(80);
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
    let env = make_bucketed(make_suite2_wire(64));
    let mut encoded = env.encode();
    let route_len = env.route_token.len();
    let pad_len_off = 2 + 2 + 2 + route_len + 4;
    // Mutate cleartext pad_len from 0 -> 1; bucket-mode decoder must reject.
    encoded[pad_len_off] = 0;
    encoded[pad_len_off + 1] = 1;
    let err = Envelope::decode(&encoded).expect_err("expected reject");
    assert!(err.to_string().contains("bucket_len_fields"));
}
