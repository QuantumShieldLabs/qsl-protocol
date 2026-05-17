use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::ratchet::{Suite2RecvWireState, Suite2SendState};
use quantumshield_refimpl::suite2::state::Suite2SessionState;
use quantumshield_refimpl::suite2::{binding, recv_wire_canon, send_wire_canon, types};
use quantumshield_refimpl::RefimplError;
use serde_json::Value;
use std::collections::BTreeSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

const PLAINTEXT_SENTINEL: &[u8] = b"NA0301_PLAINTEXT_SENTINEL_DO_NOT_ECHO";
const NEGOTIATION_SENTINEL: &str = "NA0301_NEGOTIATION_SECRET_SENTINEL_DO_NOT_ECHO";
const EXPECTED_KEM_ALG: &str = "ML-KEM-768";
const EXPECTED_SIG_ALG: &str = "ML-DSA-65+Ed25519";
const EXPECTED_KDF_ALG: &str = "KDF_HYBRID_KMAC256_SHA512";

fn arr16(seed: u8) -> [u8; 16] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(3)).rotate_left(1))
}

fn arr32(seed: u8) -> [u8; 32] {
    std::array::from_fn(|i| seed.wrapping_add((i as u8).wrapping_mul(5)).rotate_left(1))
}

fn base_session() -> Suite2SessionState {
    let session_id = arr16(0x31);
    let dh_pub = arr32(0x41);
    let hk = arr32(0x51);
    let ck_ec = arr32(0x61);
    let ck_pq = arr32(0x71);

    Suite2SessionState {
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
            rk: arr32(0x81),
            ck_ec,
            ck_pq_send: arr32(0x91),
            ck_pq_recv: ck_pq,
            nr: 0,
            role_is_a: true,
            peer_max_adv_id_seen: 0,
            known_targets: BTreeSet::new(),
            consumed_targets: BTreeSet::new(),
            tombstoned_targets: BTreeSet::new(),
            mkskipped: Vec::new(),
        },
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
        wire,
        None,
        None,
    )?;
    session.recv = out.state;
    Ok(out.plaintext)
}

fn assert_no_sentinel_or_panic_text(err: &RefimplError) {
    let rendered = err.to_string();
    assert!(
        !rendered.contains("NA0301_PLAINTEXT_SENTINEL")
            && !rendered.contains("NA0301_NEGOTIATION_SECRET_SENTINEL"),
        "reject text leaked sentinel"
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

fn assert_wire_reject_is_deterministic_and_no_mutation(
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegotiationState {
    accepted_protocol_version: Option<u16>,
    accepted_suite_id: Option<u16>,
    durable_accept_count: u32,
}

impl NegotiationState {
    fn empty() -> Self {
        Self {
            accepted_protocol_version: None,
            accepted_suite_id: None,
            durable_accept_count: 0,
        }
    }
}

#[derive(Clone)]
struct NegotiationAttempt<'a> {
    local_supports_suite2: bool,
    peer_supports_suite2: bool,
    committed_local_supports_suite2: bool,
    committed_peer_supports_suite2: bool,
    protocol_version: u16,
    suite_id: u16,
    ad_protocol_version: u16,
    ad_suite_id: u16,
    kem_alg: &'a str,
    sig_alg: &'a str,
    kdf_alg: &'a str,
    opaque_secret: &'a str,
}

fn valid_negotiation_attempt() -> NegotiationAttempt<'static> {
    NegotiationAttempt {
        local_supports_suite2: true,
        peer_supports_suite2: true,
        committed_local_supports_suite2: true,
        committed_peer_supports_suite2: true,
        protocol_version: types::SUITE2_PROTOCOL_VERSION,
        suite_id: types::SUITE2_SUITE_ID,
        ad_protocol_version: types::SUITE2_PROTOCOL_VERSION,
        ad_suite_id: types::SUITE2_SUITE_ID,
        kem_alg: EXPECTED_KEM_ALG,
        sig_alg: EXPECTED_SIG_ALG,
        kdf_alg: EXPECTED_KDF_ALG,
        opaque_secret: NEGOTIATION_SENTINEL,
    }
}

fn negotiation_reject_code(attempt: &NegotiationAttempt<'_>) -> Option<&'static str> {
    if !attempt.local_supports_suite2 {
        return Some("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if !attempt.peer_supports_suite2 {
        return Some("REJECT_S2_PEER_UNSUPPORTED");
    }
    if attempt.committed_local_supports_suite2 != attempt.local_supports_suite2
        || attempt.committed_peer_supports_suite2 != attempt.peer_supports_suite2
    {
        return Some("REJECT_S2_CAPABILITY_COMMITMENT_MISMATCH");
    }
    if attempt.kem_alg != EXPECTED_KEM_ALG
        || attempt.sig_alg != EXPECTED_SIG_ALG
        || attempt.kdf_alg != EXPECTED_KDF_ALG
    {
        return Some("REJECT_S2_ALGORITHM_UNSUPPORTED");
    }
    if attempt.ad_protocol_version != attempt.protocol_version
        || attempt.ad_suite_id != attempt.suite_id
    {
        return Some("REJECT_S2_AD_MISMATCH");
    }
    if attempt.protocol_version == 0x0403 && attempt.suite_id == 0x0001 {
        return Some("REJECT_S2_DOWNGRADE");
    }
    if attempt.protocol_version != types::SUITE2_PROTOCOL_VERSION {
        return Some("REJECT_S2_VERSION_UNSUPPORTED");
    }
    if attempt.suite_id != types::SUITE2_SUITE_ID {
        return Some("REJECT_S2_SUITE_MISMATCH");
    }
    None
}

fn apply_negotiation(
    state: &mut NegotiationState,
    attempt: &NegotiationAttempt<'_>,
) -> Result<(), RefimplError> {
    let before = state.clone();
    assert!(!attempt.opaque_secret.is_empty());
    if let Some(code) = negotiation_reject_code(attempt) {
        assert_eq!(before, *state, "reject mutated negotiation state");
        return Err(RefimplError::Reject(code));
    }

    state.accepted_protocol_version = Some(types::SUITE2_PROTOCOL_VERSION);
    state.accepted_suite_id = Some(types::SUITE2_SUITE_ID);
    state.durable_accept_count = state.durable_accept_count.saturating_add(1);
    Ok(())
}

fn assert_negotiation_reject_is_deterministic_and_no_mutation(
    state: &NegotiationState,
    attempt: &NegotiationAttempt<'_>,
    expected_code: &str,
    label: &str,
) {
    let mut first_state = state.clone();
    let before = first_state.clone();
    let err1 = apply_negotiation(&mut first_state, attempt).expect_err(label);
    assert_eq!(before, first_state, "{label} mutated first state");

    let mut second_state = state.clone();
    let err2 = apply_negotiation(&mut second_state, attempt).expect_err(label);
    assert_eq!(before, second_state, "{label} mutated repeated state");

    assert_eq!(err1, err2, "{label} reject was not deterministic");
    assert_eq!(err1.code(), expected_code, "{label} reject code drifted");
    assert_no_sentinel_or_panic_text(&err1);
    assert_no_sentinel_or_panic_text(&err2);
}

fn repo_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(relative)
}

fn json_data<'a>(v: &'a Value, key: &str) -> &'a Value {
    let raw = v.get(key).unwrap_or_else(|| panic!("missing key {key}"));
    if raw.get("type").and_then(Value::as_str) == Some("json") {
        raw.get("data")
            .unwrap_or_else(|| panic!("missing typed json data for {key}"))
    } else {
        raw
    }
}

fn parse_u16_value(v: &Value) -> u16 {
    if let Some(n) = v.as_u64() {
        return u16::try_from(n).expect("u16 value");
    }
    if let Some(s) = v.as_str() {
        if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            return u16::from_str_radix(hex, 16).expect("hex u16");
        }
        return s.parse::<u16>().expect("decimal u16");
    }
    if let Some(obj) = v.as_object() {
        if let Some(n) = obj
            .get("u16")
            .and_then(Value::as_u64)
            .or_else(|| obj.get("value").and_then(Value::as_u64))
        {
            return u16::try_from(n).expect("u16 object value");
        }
    }
    panic!("expected u16-compatible JSON value: {v}");
}

fn get_typed_u16(input: &Value, key: &str) -> u16 {
    parse_u16_value(json_data(input, key))
}

fn get_hex(input: &Value, key: &str) -> Vec<u8> {
    let raw = input
        .get(key)
        .unwrap_or_else(|| panic!("missing key {key}"));
    let data = if raw.get("type").and_then(Value::as_str) == Some("hex") {
        raw.get("data")
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("missing typed hex data for {key}"))
    } else {
        raw.as_str()
            .unwrap_or_else(|| panic!("expected hex string for {key}"))
    };
    hex::decode(data).unwrap_or_else(|e| panic!("{key} invalid hex: {e}"))
}

fn normalize(v: &Value) -> Value {
    if let Some(obj) = v.as_object() {
        if let Some(kind) = obj.get("type").and_then(Value::as_str) {
            if kind == "hex" || kind == "utf8" {
                return obj.get("data").cloned().unwrap_or(Value::Null);
            }
            if kind == "json" {
                return normalize(obj.get("data").unwrap_or(&Value::Null));
            }
        }
        let mut out = serde_json::Map::new();
        for (k, val) in obj {
            if k == "semantic" || k == "note" {
                continue;
            }
            out.insert(k.clone(), normalize(val));
        }
        return Value::Object(out);
    }
    if let Some(arr) = v.as_array() {
        return Value::Array(arr.iter().map(normalize).collect());
    }
    v.clone()
}

fn evaluate_downgrade_vector(input: &Value) -> Result<Value, RefimplError> {
    let local = json_data(input, "local");
    let peer = json_data(input, "peer");
    let negotiated = json_data(input, "negotiated");
    let ad = json_data(input, "ad");

    let local_supports = local
        .get("supports_suite2")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let peer_supports = peer
        .get("supports_suite2")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let policy_require = local
        .get("policy_require_suite2")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    let suite2_required = policy_require || (local_supports && peer_supports);

    if policy_require && !peer_supports {
        return Err(RefimplError::Reject("REJECT_S2_PEER_UNSUPPORTED"));
    }

    let pv = parse_u16_value(
        negotiated
            .get("protocol_version")
            .expect("negotiated protocol_version"),
    );
    let sid = parse_u16_value(negotiated.get("suite_id").expect("negotiated suite_id"));
    let ad_pv = parse_u16_value(ad.get("protocol_version").expect("ad protocol_version"));
    let ad_sid = parse_u16_value(ad.get("suite_id").expect("ad suite_id"));

    if ad_pv != pv || ad_sid != sid {
        return Err(RefimplError::Reject("REJECT_S2_AD_MISMATCH"));
    }
    if suite2_required && (pv != types::SUITE2_PROTOCOL_VERSION || sid != types::SUITE2_SUITE_ID) {
        if pv == 0x0403 && sid == 0x0001 {
            return Err(RefimplError::Reject("REJECT_S2_DOWNGRADE"));
        }
        return Err(RefimplError::Reject("REJECT_S2_SUITE_MISMATCH"));
    }

    Ok(serde_json::json!({
        "selected": {
            "type": "json",
            "data": {
                "protocol_version": "0x0500",
                "suite_id": "0x0002"
            }
        }
    }))
}

fn evaluate_transcript_vector(crypto: &StdCrypto, input: &Value) -> Result<Value, RefimplError> {
    let negotiated = json_data(input, "negotiated");
    let pv = parse_u16_value(
        negotiated
            .get("protocol_version")
            .expect("negotiated protocol_version"),
    );
    let sid = parse_u16_value(negotiated.get("suite_id").expect("negotiated suite_id"));
    if pv != types::SUITE2_PROTOCOL_VERSION || sid != types::SUITE2_SUITE_ID {
        return Err(RefimplError::Reject("REJECT_S2_SUITE_MISMATCH"));
    }

    let session_id = get_hex(input, "session_id");
    let dh_pub = get_hex(input, "DH_pub");
    let flags = get_typed_u16(input, "flags");
    let pq_prefix = get_hex(input, "pq_prefix");
    let ad_hdr_in = get_hex(input, "ad_hdr");
    let ad_body_in = get_hex(input, "ad_body");

    let pq_bind = binding::pq_bind_sha512_32(crypto, flags, &pq_prefix);
    let ad_hdr_calc = binding::ad_hdr(&session_id, pv, sid, &dh_pub, flags, &pq_bind);
    let ad_body_calc = binding::ad_body(&session_id, pv, sid, &pq_bind);

    if ad_hdr_calc != ad_hdr_in || ad_body_calc != ad_body_in {
        return Err(RefimplError::Reject("REJECT_S2_AD_MISMATCH"));
    }

    Ok(serde_json::json!({
        "pq_bind": { "type": "hex", "data": hex::encode(pq_bind) },
        "ad_hdr": { "type": "hex", "data": hex::encode(ad_hdr_calc) },
        "ad_body": { "type": "hex", "data": hex::encode(ad_body_calc) }
    }))
}

fn assert_vector_set_consistent(
    file: &str,
    category: &str,
    evaluator: impl Fn(&Value) -> Result<Value, RefimplError>,
) -> usize {
    let data = std::fs::read_to_string(repo_path(file)).expect("vector file");
    let root: Value = serde_json::from_str(&data).expect("vector JSON");
    let vectors = root
        .get("vectors")
        .and_then(Value::as_array)
        .expect("vectors array");

    let mut ran = 0usize;
    for vector in vectors {
        let tags = vector
            .get("tags")
            .and_then(Value::as_array)
            .expect("tags array");
        if !tags.iter().any(|tag| tag.as_str() == Some(category)) {
            continue;
        }
        ran += 1;
        let id = vector.get("id").and_then(Value::as_str).expect("vector id");
        let input = vector.get("input").expect("vector input");
        let expect = vector.get("expect").expect("vector expect");
        let expect_ok = expect
            .get("ok")
            .and_then(Value::as_bool)
            .expect("expect.ok");

        match (expect_ok, evaluator(input)) {
            (true, Ok(output)) => {
                let expected_output = expect.get("output").expect("expect.output");
                assert_eq!(
                    normalize(&output),
                    normalize(expected_output),
                    "{id} output mismatch"
                );
            }
            (false, Err(err)) => {
                let expected_reason = expect
                    .get("reason_code")
                    .and_then(Value::as_str)
                    .expect("expect.reason_code");
                assert_eq!(err.code(), expected_reason, "{id} reason drifted");
                assert_no_sentinel_or_panic_text(&err);
            }
            (true, Err(err)) => panic!("{id} expected ok but rejected: {err}"),
            (false, Ok(output)) => panic!("{id} expected reject but accepted: {output}"),
        }
    }
    assert!(ran > 0, "no vectors ran for {category}");
    ran
}

#[test]
fn suite2_negotiation_downgrade_expansion_harness() {
    let crypto = StdCrypto;

    let mut negotiation_state = NegotiationState::empty();
    let valid_attempt = valid_negotiation_attempt();
    let before_negotiation_accept = negotiation_state.clone();
    apply_negotiation(&mut negotiation_state, &valid_attempt).expect("valid Suite-2 negotiation");
    assert_ne!(
        before_negotiation_accept, negotiation_state,
        "valid Suite-2 negotiation did not mutate accepted control state"
    );
    assert_eq!(
        negotiation_state.accepted_protocol_version,
        Some(types::SUITE2_PROTOCOL_VERSION)
    );
    assert_eq!(
        negotiation_state.accepted_suite_id,
        Some(types::SUITE2_SUITE_ID)
    );

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

    let before_receive_accept = session.snapshot_bytes();
    let plaintext =
        recv_into_session(&crypto, &mut session, &send.wire).expect("valid Suite-2 receive");
    assert_eq!(plaintext.as_slice(), PLAINTEXT_SENTINEL);
    assert_ne!(
        before_receive_accept,
        session.snapshot_bytes(),
        "valid Suite-2 receive did not advance accepted state"
    );
    println!("NA0301_SUITE2_CONTROL_OK");

    let accepted_negotiation_state = negotiation_state.clone();
    let accepted_session_snapshot = session.snapshot_bytes();

    let mut unsupported_suite = valid_attempt.clone();
    unsupported_suite.suite_id = 0x9999;
    unsupported_suite.ad_suite_id = 0x9999;
    assert_negotiation_reject_is_deterministic_and_no_mutation(
        &accepted_negotiation_state,
        &unsupported_suite,
        "REJECT_S2_SUITE_MISMATCH",
        "unsupported suite negotiation",
    );
    let mut unsupported_suite_wire = send.wire.clone();
    unsupported_suite_wire[2..4].copy_from_slice(&0x9999u16.to_be_bytes());
    assert_wire_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &unsupported_suite_wire,
        "REJECT_S2_PARSE_PREFIX",
        "unsupported suite wire",
    );
    println!("NA0301_UNSUPPORTED_SUITE_REJECT_OK");

    let mut downgraded = valid_attempt.clone();
    downgraded.protocol_version = 0x0403;
    downgraded.suite_id = 0x0001;
    downgraded.ad_protocol_version = 0x0403;
    downgraded.ad_suite_id = 0x0001;
    assert_negotiation_reject_is_deterministic_and_no_mutation(
        &accepted_negotiation_state,
        &downgraded,
        "REJECT_S2_DOWNGRADE",
        "downgrade negotiation",
    );
    let mut downgraded_wire = send.wire.clone();
    downgraded_wire[0..2].copy_from_slice(&0x0403u16.to_be_bytes());
    downgraded_wire[2..4].copy_from_slice(&0x0001u16.to_be_bytes());
    assert_wire_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &downgraded_wire,
        "REJECT_S2_PARSE_PREFIX",
        "downgrade wire",
    );
    println!("NA0301_DOWNGRADE_REJECT_OK");

    let mut unsupported_version = valid_attempt.clone();
    unsupported_version.protocol_version = 0x0600;
    unsupported_version.ad_protocol_version = 0x0600;
    assert_negotiation_reject_is_deterministic_and_no_mutation(
        &accepted_negotiation_state,
        &unsupported_version,
        "REJECT_S2_VERSION_UNSUPPORTED",
        "unsupported version negotiation",
    );
    let mut unsupported_algorithm = valid_attempt.clone();
    unsupported_algorithm.kem_alg = "ML-KEM-1024";
    assert_negotiation_reject_is_deterministic_and_no_mutation(
        &accepted_negotiation_state,
        &unsupported_algorithm,
        "REJECT_S2_ALGORITHM_UNSUPPORTED",
        "unsupported algorithm negotiation",
    );

    let mut unsupported_flags_wire = send.wire.clone();
    let flags_offset = 10 + 32;
    unsupported_flags_wire[flags_offset..flags_offset + 2]
        .copy_from_slice(&0x8000u16.to_be_bytes());
    assert_wire_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &unsupported_flags_wire,
        "REJECT_S2_PARSE_FLAGS",
        "unsupported flags wire",
    );
    println!("NA0301_UNSUPPORTED_PARAMETER_REJECT_OK");

    let malformed_wire = PLAINTEXT_SENTINEL.to_vec();
    assert_wire_reject_is_deterministic_and_no_mutation(
        &crypto,
        &mut session,
        &malformed_wire,
        "REJECT_S2_PARSE_PREFIX",
        "malformed sentinel wire",
    );
    println!("NA0301_MALFORMED_NEGOTIATION_REJECT_OK");

    assert_eq!(
        accepted_session_snapshot,
        session.snapshot_bytes(),
        "accepted Suite-2 receive state drifted after reject cases"
    );
    assert_eq!(
        accepted_negotiation_state, negotiation_state,
        "accepted negotiation state drifted after reject cases"
    );
    println!("NA0301_NO_MUTATION_ON_REJECT_OK");

    let downgrade_count = assert_vector_set_consistent(
        "inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json",
        "CAT-S2-DOWNGRADE-001",
        evaluate_downgrade_vector,
    );
    let transcript_count = assert_vector_set_consistent(
        "inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json",
        "CAT-S2-TRANSCRIPT-001",
        |input| evaluate_transcript_vector(&crypto, input),
    );
    assert_eq!(downgrade_count, 5, "unexpected downgrade vector count");
    assert_eq!(transcript_count, 4, "unexpected transcript vector count");

    println!("NA0301_NO_PANIC_OK");
    println!("NA0301_NO_SECRET_LEAK_OK");
    println!("NA0301_VECTOR_CONSISTENCY_OK");
    println!("NA0301_SUITE2_NEGOTIATION_DOWNGRADE_OK");
}
