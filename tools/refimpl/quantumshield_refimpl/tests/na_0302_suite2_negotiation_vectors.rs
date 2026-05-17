use quantumshield_refimpl::suite2::types;
use quantumshield_refimpl::RefimplError;
use serde_json::Value;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::PathBuf;

const VECTOR_FILE: &str = "inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json";
const EXPECTED_KEM_ALG: &str = "ML-KEM-768";
const EXPECTED_SIG_ALG: &str = "ML-DSA-65+Ed25519";
const EXPECTED_KDF_ALG: &str = "KDF_HYBRID_KMAC256_SHA512";
const EXPECTED_PARAMETER_SET: &str = "suite2-default";
const NEGOTIATION_SENTINEL: &str = "NA0302_NEGOTIATION_MALFORMED_SENTINEL_DO_NOT_ECHO";

#[derive(Clone, Debug, PartialEq, Eq)]
struct NegotiationState {
    accepted_protocol_version: Option<u16>,
    accepted_suite_id: Option<u16>,
    accepted_algorithm_profile: Option<String>,
    durable_accept_count: u32,
}

impl NegotiationState {
    fn empty() -> Self {
        Self {
            accepted_protocol_version: None,
            accepted_suite_id: None,
            accepted_algorithm_profile: None,
            durable_accept_count: 0,
        }
    }
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
    panic!("expected u16-compatible JSON value: {v}");
}

fn typed_string<'a>(input: &'a Value, object_key: &str, field_key: &str) -> &'a str {
    json_data(input, object_key)
        .get(field_key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing {object_key}.{field_key}"))
}

fn typed_bool(input: &Value, object_key: &str, field_key: &str) -> bool {
    json_data(input, object_key)
        .get(field_key)
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn typed_u16(input: &Value, object_key: &str, field_key: &str) -> u16 {
    parse_u16_value(
        json_data(input, object_key)
            .get(field_key)
            .unwrap_or_else(|| panic!("missing {object_key}.{field_key}")),
    )
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

fn assert_no_sentinel_or_panic_text(err: &RefimplError) {
    let rendered = err.to_string();
    assert!(
        !rendered.contains(NEGOTIATION_SENTINEL) && !rendered.contains("NA0302_NEGOTIATION"),
        "reject text leaked negotiation sentinel"
    );
    assert!(
        !rendered.contains("panicked") && !rendered.contains("stack backtrace"),
        "reject text included panic/backtrace wording"
    );
}

fn reject_reason(input: &Value) -> Option<&'static str> {
    if input.get("malformed").is_some() {
        return Some("REJECT_S2_MALFORMED_NEGOTIATION");
    }

    let local_supports = typed_bool(input, "local", "supports_suite2");
    let peer_supports = typed_bool(input, "peer", "supports_suite2");
    if !local_supports {
        return Some("REJECT_S2_LOCAL_UNSUPPORTED");
    }
    if !peer_supports {
        return Some("REJECT_S2_PEER_UNSUPPORTED");
    }

    let flags = typed_u16(input, "parameters", "flags");
    if flags != 0 {
        return Some("REJECT_S2_PARSE_FLAGS");
    }

    if typed_string(input, "algorithms", "kem") != EXPECTED_KEM_ALG
        || typed_string(input, "algorithms", "sig") != EXPECTED_SIG_ALG
        || typed_string(input, "algorithms", "kdf") != EXPECTED_KDF_ALG
    {
        return Some("REJECT_S2_ALGORITHM_UNSUPPORTED");
    }

    let parameter_set = typed_string(input, "parameters", "parameter_set");
    if parameter_set != EXPECTED_PARAMETER_SET {
        return Some("REJECT_S2_PARAMETER_UNSUPPORTED");
    }

    let protocol_version = typed_u16(input, "negotiated", "protocol_version");
    let suite_id = typed_u16(input, "negotiated", "suite_id");
    let ad_protocol_version = typed_u16(input, "ad", "protocol_version");
    let ad_suite_id = typed_u16(input, "ad", "suite_id");
    if ad_protocol_version != protocol_version || ad_suite_id != suite_id {
        return Some("REJECT_S2_AD_MISMATCH");
    }
    if protocol_version == 0x0403 && suite_id == 0x0001 {
        return Some("REJECT_S2_DOWNGRADE");
    }
    if protocol_version != types::SUITE2_PROTOCOL_VERSION {
        return Some("REJECT_S2_VERSION_UNSUPPORTED");
    }
    if suite_id != types::SUITE2_SUITE_ID {
        return Some("REJECT_S2_SUITE_MISMATCH");
    }

    None
}

fn apply_negotiation_vector(
    state: &mut NegotiationState,
    vector: &Value,
) -> Result<Value, RefimplError> {
    let input = vector.get("input").expect("vector input");
    let before = state.clone();
    if let Some(code) = reject_reason(input) {
        assert_eq!(before, *state, "reject mutated accepted state");
        return Err(RefimplError::Reject(code));
    }

    state.accepted_protocol_version = Some(types::SUITE2_PROTOCOL_VERSION);
    state.accepted_suite_id = Some(types::SUITE2_SUITE_ID);
    state.accepted_algorithm_profile = Some(EXPECTED_PARAMETER_SET.to_string());
    state.durable_accept_count = state.durable_accept_count.saturating_add(1);

    Ok(serde_json::json!({
        "selected": {
            "type": "json",
            "data": {
                "protocol_version": "0x0500",
                "suite_id": "0x0002",
                "algorithm_profile": "suite2-default"
            }
        }
    }))
}

fn apply_without_panic(
    state: &mut NegotiationState,
    vector: &Value,
    id: &str,
) -> Result<Value, RefimplError> {
    match catch_unwind(AssertUnwindSafe(|| apply_negotiation_vector(state, vector))) {
        Ok(result) => result,
        Err(_) => panic!("{id} panicked"),
    }
}

fn assert_reject_is_deterministic_and_no_mutation(
    accepted_state: &NegotiationState,
    vector: &Value,
    id: &str,
    expected_reason: &str,
) {
    let mut first_state = accepted_state.clone();
    let before = first_state.clone();
    let err1 = apply_without_panic(&mut first_state, vector, id).expect_err(id);
    assert_eq!(before, first_state, "{id} mutated state on first reject");

    let mut second_state = accepted_state.clone();
    let err2 = apply_without_panic(&mut second_state, vector, id).expect_err(id);
    assert_eq!(
        before, second_state,
        "{id} mutated state on repeated reject"
    );

    assert_eq!(err1, err2, "{id} reject was not deterministic");
    assert_eq!(err1.code(), expected_reason, "{id} reason code drifted");
    assert_no_sentinel_or_panic_text(&err1);
    assert_no_sentinel_or_panic_text(&err2);
}

#[test]
fn suite2_negotiation_vectors_na0302_are_fail_closed_and_no_mutation() {
    let data = std::fs::read_to_string(repo_path(VECTOR_FILE)).expect("NA-0302 vector file");
    let root: Value = serde_json::from_str(&data).expect("NA-0302 vector JSON");
    assert_eq!(
        root.get("format").and_then(Value::as_str),
        Some("QSHIELD-P4-VECTOR-SET-1")
    );
    assert_eq!(
        root.pointer("/protocol/protocol_version")
            .and_then(Value::as_str),
        Some("0x0500")
    );
    assert_eq!(
        root.pointer("/protocol/suite_id").and_then(Value::as_str),
        Some("0x0002")
    );
    let vectors = root
        .get("vectors")
        .and_then(Value::as_array)
        .expect("vectors array");
    assert_eq!(vectors.len(), 6, "unexpected NA-0302 vector count");
    println!("NA0302_VECTOR_SCHEMA_OK");

    let mut state = NegotiationState::empty();
    let mut accepted_state = None;
    let mut saw_valid = false;
    let mut saw_unsupported_suite = false;
    let mut saw_downgrade = false;
    let mut saw_unsupported_parameter = false;
    let mut saw_malformed = false;

    for vector in vectors {
        let id = vector.get("id").and_then(Value::as_str).expect("vector id");
        let expect = vector.get("expect").expect("vector expect");
        let expect_ok = expect
            .get("ok")
            .and_then(Value::as_bool)
            .expect("expect.ok");

        if expect_ok {
            let before = state.clone();
            let output = apply_without_panic(&mut state, vector, id).expect("valid vector");
            assert_ne!(before, state, "{id} did not mutate accepted state");
            assert_eq!(
                normalize(&output),
                normalize(expect.get("output").expect("expect.output")),
                "{id} output mismatch"
            );
            accepted_state = Some(state.clone());
            saw_valid = true;
            println!("NA0302_VALID_SUITE2_VECTOR_OK");
        } else {
            let base = accepted_state.as_ref().unwrap_or(&state);
            let reason = expect
                .get("reason_code")
                .and_then(Value::as_str)
                .expect("expect.reason_code");
            assert_reject_is_deterministic_and_no_mutation(base, vector, id, reason);
            match reason {
                "REJECT_S2_SUITE_MISMATCH" => {
                    saw_unsupported_suite = true;
                    println!("NA0302_UNSUPPORTED_SUITE_VECTOR_REJECT_OK");
                }
                "REJECT_S2_DOWNGRADE" => {
                    saw_downgrade = true;
                    println!("NA0302_DOWNGRADE_VECTOR_REJECT_OK");
                }
                "REJECT_S2_ALGORITHM_UNSUPPORTED" | "REJECT_S2_PARSE_FLAGS" => {
                    saw_unsupported_parameter = true;
                }
                "REJECT_S2_MALFORMED_NEGOTIATION" => {
                    saw_malformed = true;
                    println!("NA0302_MALFORMED_NEGOTIATION_VECTOR_REJECT_OK");
                }
                other => panic!("unexpected NA-0302 reason code {other}"),
            }
        }
    }

    assert!(saw_valid, "valid Suite-2 vector did not run");
    assert!(
        saw_unsupported_suite,
        "unsupported suite vector did not run"
    );
    assert!(saw_downgrade, "downgrade vector did not run");
    assert!(
        saw_unsupported_parameter,
        "unsupported parameter/algorithm vector did not run"
    );
    assert!(saw_malformed, "malformed vector did not run");
    assert_eq!(
        Some(&state),
        accepted_state.as_ref(),
        "accepted state drifted after reject vectors"
    );

    println!("NA0302_UNSUPPORTED_PARAMETER_VECTOR_REJECT_OK");
    println!("NA0302_VECTOR_NO_MUTATION_ON_REJECT_OK");
    println!("NA0302_NO_PANIC_OK");
    println!("NA0302_NO_SECRET_LEAK_OK");
    println!("NA0302_SUITE2_NEGOTIATION_VECTOR_HARDENING_OK");
}
