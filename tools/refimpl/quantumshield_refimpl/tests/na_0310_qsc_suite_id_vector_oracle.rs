use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

const VECTOR_FILE: &str =
    "inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json";

const REQUIRED_FIELDS: &[&str] = &[
    "vector_id",
    "category",
    "purpose",
    "qhsm_version",
    "compatibility_mode",
    "suite_required_mode",
    "frame_sequence",
    "protocol_version",
    "suite_id",
    "negotiated_parameters",
    "transcript_context_label",
    "key_context_label",
    "canonical_encoding_expected",
    "expected_result",
    "expected_reason_label",
    "mutation_expected",
    "recv_commit_expected",
    "output_expected",
    "secret_leak_expected",
    "model_property_refs",
    "refimpl_oracle_expectation",
    "qsc_harness_expectation",
    "notes",
];

const REQUIRED_CATEGORIES: &[&str] = &[
    "valid_v2_suite2_parameter_block",
    "legacy_v1_compatibility_allowed",
    "legacy_v1_rejected_in_suite_required_mode",
    "unsupported_suite_id",
    "downgraded_suite_id",
    "stripped_suite_id_parameter",
    "mismatched_suite_id_A1_B1",
    "mismatched_suite_id_B1_A2",
    "duplicate_suite_id_parameter",
    "unknown_critical_parameter",
    "unknown_noncritical_parameter",
    "noncanonical_parameter_order",
    "malformed_parameter_length",
    "inconsistent_protocol_version_suite_id",
    "replayed_A1_with_suite_context",
    "replayed_A2_with_suite_context",
    "valid_suite2_with_transcript_binding",
    "transcript_binding_mismatch",
    "key_schedule_context_mismatch",
    "missing_key_context_in_required_mode",
];

const KNOWN_MODEL_REFS: &[&str] = &[
    "NA0309_MODEL_VALID_V2_SUITE2_OK",
    "NA0309_MODEL_LEGACY_COMPATIBILITY_OK",
    "NA0309_MODEL_LEGACY_REQUIRED_REJECT_OK",
    "NA0309_MODEL_UNSUPPORTED_SUITE_REJECT_OK",
    "NA0309_MODEL_DOWNGRADE_REJECT_OK",
    "NA0309_MODEL_STRIPPED_SUITE_REJECT_OK",
    "NA0309_MODEL_MISMATCH_REJECT_OK",
    "NA0309_MODEL_DUPLICATE_REJECT_OK",
    "NA0309_MODEL_UNKNOWN_CRITICAL_REJECT_OK",
    "NA0309_MODEL_NONCANONICAL_REJECT_OK",
    "NA0309_MODEL_MALFORMED_REJECT_OK",
    "NA0309_MODEL_TRANSCRIPT_BINDING_OK",
    "NA0309_MODEL_KEY_CONTEXT_OK",
    "NA0309_MODEL_NO_MUTATION_ON_REJECT_OK",
    "NA0309_MODEL_NO_OUTPUT_ON_REJECT_OK",
    "NA0309_MODEL_NO_SECRET_LEAK_OK",
    "NA0309_MODEL_NO_DOWNGRADE_PATH_OK",
    "NA0309_MODEL_REASON_LABELS_OK",
    "NA0309_QSC_HANDSHAKE_SUITE_ID_FORMAL_MODEL_OK",
    "NA-0309:P13_INCONSISTENT_TUPLE",
    "NA-0309:P12_REJECT_BOUNDARY",
];

fn repo_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .join(relative)
}

fn field<'a>(value: &'a Value, name: &str) -> &'a Value {
    value
        .get(name)
        .unwrap_or_else(|| panic!("missing field {name}"))
}

fn str_field<'a>(value: &'a Value, name: &str) -> &'a str {
    field(value, name)
        .as_str()
        .unwrap_or_else(|| panic!("field {name} must be string"))
}

fn bool_field(value: &Value, name: &str) -> bool {
    field(value, name)
        .as_bool()
        .unwrap_or_else(|| panic!("field {name} must be bool"))
}

fn array_field<'a>(value: &'a Value, name: &str) -> &'a Vec<Value> {
    field(value, name)
        .as_array()
        .unwrap_or_else(|| panic!("field {name} must be array"))
}

fn expected_reason_for(category: &str) -> &'static str {
    match category {
        "valid_v2_suite2_parameter_block" | "valid_suite2_with_transcript_binding" => {
            "ACCEPT_QSC_HS_SUITE2"
        }
        "legacy_v1_compatibility_allowed" => "ACCEPT_QSC_HS_LEGACY_COMPATIBILITY",
        "legacy_v1_rejected_in_suite_required_mode" => "REJECT_QSC_HS_LEGACY_REQUIRED",
        "unsupported_suite_id" => "REJECT_QSC_HS_SUITE_UNSUPPORTED",
        "downgraded_suite_id" => "REJECT_QSC_HS_DOWNGRADE",
        "stripped_suite_id_parameter" => "REJECT_QSC_HS_SUITE_MISSING",
        "mismatched_suite_id_A1_B1" | "mismatched_suite_id_B1_A2" => {
            "REJECT_QSC_HS_CONTEXT_MISMATCH"
        }
        "duplicate_suite_id_parameter" => "REJECT_QSC_HS_DUPLICATE_PARAMETER",
        "unknown_critical_parameter" => "REJECT_QSC_HS_UNKNOWN_CRITICAL",
        "unknown_noncritical_parameter" => "REJECT_QSC_HS_UNKNOWN_PARAMETER",
        "noncanonical_parameter_order" => "REJECT_QSC_HS_NONCANONICAL_ORDER",
        "malformed_parameter_length" => "REJECT_QSC_HS_MALFORMED_LENGTH",
        "inconsistent_protocol_version_suite_id" => "REJECT_QSC_HS_INCONSISTENT_TUPLE",
        "replayed_A1_with_suite_context" | "replayed_A2_with_suite_context" => {
            "REJECT_QSC_HS_REPLAY"
        }
        "transcript_binding_mismatch" => "REJECT_QSC_HS_TRANSCRIPT_CONTEXT",
        "key_schedule_context_mismatch" | "missing_key_context_in_required_mode" => {
            "REJECT_QSC_HS_KEY_CONTEXT"
        }
        _ => panic!("unexpected category {category}"),
    }
}

#[test]
fn na_0310_qsc_suite_id_vector_oracle() {
    let path = repo_path(VECTOR_FILE);
    let raw = fs::read_to_string(&path).expect("read NA-0310 vector file");
    let doc: Value = serde_json::from_str(&raw).expect("parse NA-0310 vector JSON");

    assert_eq!(str_field(&doc, "schema_version"), "1.0.0");
    assert_eq!(
        str_field(&doc, "artifact_id"),
        "NA-0310-qsc-handshake-suite-id-vector-schema-refimpl-oracle"
    );
    assert_eq!(str_field(&doc, "source_na"), "NA-0310");
    assert!(
        str_field(&doc, "statement").contains("not runtime implementation"),
        "statement must not overclaim runtime implementation"
    );
    assert!(
        str_field(&doc, "statement").contains("not QHSM/QSP wire-format implementation"),
        "statement must not overclaim wire-format implementation"
    );
    assert!(!array_field(&doc, "design_refs").is_empty());
    assert!(!array_field(&doc, "model_refs").is_empty());

    let known_refs: BTreeSet<&str> = KNOWN_MODEL_REFS.iter().copied().collect();
    for model_ref in array_field(&doc, "model_refs") {
        let model_ref = model_ref.as_str().expect("top-level model ref string");
        assert!(
            known_refs.contains(model_ref),
            "unknown top-level model ref {model_ref}"
        );
    }

    let vectors = array_field(&doc, "vectors");
    assert_eq!(
        vectors.len(),
        REQUIRED_CATEGORIES.len(),
        "NA-0310 must keep one vector per required category"
    );

    let mut ids = BTreeSet::new();
    let mut categories = BTreeSet::new();
    let mut reject_count = 0usize;
    let mut accept_count = 0usize;
    let mut compatibility_accept_count = 0usize;

    for vector in vectors {
        for required in REQUIRED_FIELDS {
            assert!(
                vector.get(*required).is_some(),
                "missing required field {required}"
            );
        }

        let vector_id = str_field(vector, "vector_id");
        assert!(
            ids.insert(vector_id.to_string()),
            "duplicate vector id {vector_id}"
        );

        let category = str_field(vector, "category");
        assert!(
            REQUIRED_CATEGORIES.contains(&category),
            "unexpected vector category {category}"
        );
        assert!(
            categories.insert(category.to_string()),
            "duplicate vector category {category}"
        );
        assert_eq!(
            str_field(vector, "expected_reason_label"),
            expected_reason_for(category),
            "reason label drift for {category}"
        );
        assert!(
            !array_field(vector, "frame_sequence").is_empty(),
            "{category} must name at least one frame"
        );
        assert!(
            !str_field(vector, "purpose").is_empty(),
            "{category} must state purpose"
        );

        for model_ref in array_field(vector, "model_property_refs") {
            let model_ref = model_ref.as_str().expect("model property ref string");
            assert!(
                known_refs.contains(model_ref),
                "{category} references unknown model property {model_ref}"
            );
        }

        let refimpl = field(vector, "refimpl_oracle_expectation");
        assert_eq!(
            str_field(refimpl, "status"),
            "proven_refimpl_oracle",
            "{category} must be asserted by this oracle"
        );
        assert!(
            !array_field(refimpl, "assertions").is_empty(),
            "{category} must name oracle assertions"
        );

        let qsc = field(vector, "qsc_harness_expectation");
        assert_eq!(
            str_field(qsc, "status"),
            "future_gate",
            "{category} must not claim qsc runtime implementation"
        );
        let qsc_description = str_field(qsc, "description");
        assert!(
            qsc_description.contains("Future") && qsc_description.contains("qsc"),
            "{category} must keep qsc harness expectation future-gated"
        );
        assert!(
            !qsc_description.contains("implemented"),
            "{category} must not claim implemented qsc suite-id behavior"
        );

        assert!(
            !bool_field(vector, "secret_leak_expected"),
            "{category} must never expect secret leakage"
        );

        match str_field(vector, "expected_result") {
            "accept" => {
                accept_count += 1;
                assert_eq!(str_field(vector, "qhsm_version"), "v2");
                assert!(!bool_field(vector, "compatibility_mode"));
                assert!(bool_field(vector, "suite_required_mode"));
                assert_eq!(str_field(vector, "protocol_version"), "0x0500");
                assert_eq!(str_field(vector, "suite_id"), "0x0002");
                assert!(bool_field(vector, "canonical_encoding_expected"));
                assert!(bool_field(vector, "mutation_expected"));
                assert!(bool_field(vector, "recv_commit_expected"));
                assert!(bool_field(vector, "output_expected"));
            }
            "compatibility_accept" => {
                compatibility_accept_count += 1;
                assert_eq!(category, "legacy_v1_compatibility_allowed");
                assert_eq!(str_field(vector, "qhsm_version"), "v1");
                assert!(bool_field(vector, "compatibility_mode"));
                assert!(!bool_field(vector, "suite_required_mode"));
                assert!(
                    str_field(vector, "transcript_context_label")
                        .contains("no-explicit-suite-context"),
                    "legacy compatibility must not claim explicit suite context"
                );
                assert!(bool_field(vector, "mutation_expected"));
                assert!(bool_field(vector, "recv_commit_expected"));
                assert!(bool_field(vector, "output_expected"));
            }
            "reject" => {
                reject_count += 1;
                assert!(
                    str_field(vector, "expected_reason_label").starts_with("REJECT_QSC_HS_"),
                    "{category} reject reason must use deterministic qsc label"
                );
                assert!(
                    !bool_field(vector, "mutation_expected"),
                    "{category} reject must expect no mutation"
                );
                assert!(
                    !bool_field(vector, "recv_commit_expected"),
                    "{category} reject must expect no recv_commit"
                );
                assert!(
                    !bool_field(vector, "output_expected"),
                    "{category} reject must expect no output"
                );
            }
            other => panic!("{category} has unsupported expected_result {other}"),
        }

        if category.contains("transcript") {
            assert!(
                str_field(vector, "transcript_context_label").contains("transcript"),
                "{category} must carry an explicit transcript label"
            );
        }
        if category.contains("key") {
            assert!(
                str_field(vector, "key_context_label").contains("key")
                    || str_field(vector, "key_context_label") == "missing",
                "{category} must carry an explicit key-context expectation"
            );
        }
    }

    for required in REQUIRED_CATEGORIES {
        assert!(
            categories.contains(*required),
            "missing category {required}"
        );
    }
    assert_eq!(
        accept_count, 2,
        "two explicit Suite-2 accept vectors expected"
    );
    assert_eq!(
        compatibility_accept_count, 1,
        "one explicit legacy compatibility vector expected"
    );
    assert_eq!(reject_count, 17, "seventeen reject vectors expected");

    println!("NA0310_VECTOR_SCHEMA_OK");
    println!("NA0310_VECTOR_CATEGORIES_OK");
    println!("NA0310_VALID_SUITE2_VECTOR_OK");
    println!("NA0310_LEGACY_COMPAT_VECTOR_OK");
    println!("NA0310_REQUIRED_MODE_REJECT_VECTOR_OK");
    println!("NA0310_UNSUPPORTED_SUITE_VECTOR_OK");
    println!("NA0310_DOWNGRADE_VECTOR_OK");
    println!("NA0310_STRIPPED_SUITE_VECTOR_OK");
    println!("NA0310_MISMATCH_VECTOR_OK");
    println!("NA0310_DUPLICATE_VECTOR_OK");
    println!("NA0310_UNKNOWN_CRITICAL_VECTOR_OK");
    println!("NA0310_NONCANONICAL_VECTOR_OK");
    println!("NA0310_MALFORMED_VECTOR_OK");
    println!("NA0310_TRANSCRIPT_VECTOR_OK");
    println!("NA0310_KEY_CONTEXT_VECTOR_OK");
    println!("NA0310_NO_MUTATION_EXPECTATIONS_OK");
    println!("NA0310_NO_OUTPUT_EXPECTATIONS_OK");
    println!("NA0310_NO_SECRET_LEAK_EXPECTATIONS_OK");
    println!("NA0310_REFIMPL_ORACLE_OK");
    println!("NA0310_QSC_SUITE_ID_VECTOR_REFIMPL_ORACLE_OK");
}
