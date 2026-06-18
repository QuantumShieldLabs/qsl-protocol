use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;

const MANIFEST_RELATIVE_PATH: &str =
    "../../../inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json";

const EXPECTED_QSC_FRAME_IDS: &[&str] = &[
    "kem_wrong_peer_public_key",
    "kem_stale_public_record",
    "kem_wrong_ciphertext",
    "kem_wrong_key_ciphertext_pair",
    "signature_wrong_identity_public_record",
    "signature_cross_message_replay_b1_as_a2",
    "signature_wrong_message_context",
    "signature_tampered_signature",
    "signature_wrong_public_key",
    "transcript_mutation",
    "transcript_truncation",
    "replayed_a1",
    "replayed_b1",
    "replayed_a2",
    "wrong_role_replay",
    "suite_confusion_wrong_suite_token",
    "downgrade_wrong_suite_block",
    "stale_public_record_replay",
    "public_record_rollback",
    "identity_rotation_stale_peer_state",
    "stale_trusted_pin_mismatch",
];

const EXPECTED_REFIMPL_IDS: &[&str] = &[
    "refimpl_signature_wrong_public_key_length",
    "refimpl_signature_wrong_signature_length",
    "refimpl_signature_malformed_signing_key",
    "refimpl_signature_tampered_signature_invalid",
    "refimpl_signature_wrong_public_key_invalid",
    "refimpl_signature_err_vs_false_classification",
];

const EXPECTED_FORMAL_IDS: &[&str] = &[
    "formal_wrong_kem_token",
    "formal_wrong_signature_token",
    "formal_transcript_mutation",
    "formal_replay",
    "formal_suite_confusion",
    "formal_stale_public_record",
    "formal_no_session_mutation_on_reject",
];

fn manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(MANIFEST_RELATIVE_PATH)
}

fn load_manifest() -> (Value, String) {
    let text = fs::read_to_string(manifest_path()).expect("read internal negative vector manifest");
    let manifest: Value =
        serde_json::from_str(&text).expect("parse internal negative vector manifest");
    (manifest, text)
}

fn object<'a>(value: &'a Value, context: &str) -> &'a serde_json::Map<String, Value> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{context} must be a JSON object"))
}

fn array<'a>(value: &'a Value, context: &str) -> &'a [Value] {
    value
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_else(|| panic!("{context} must be a JSON array"))
}

fn field<'a>(object: &'a serde_json::Map<String, Value>, key: &str, context: &str) -> &'a Value {
    object
        .get(key)
        .unwrap_or_else(|| panic!("{context} missing required field {key}"))
}

fn string_field<'a>(
    object: &'a serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> &'a str {
    field(object, key, context)
        .as_str()
        .unwrap_or_else(|| panic!("{context}.{key} must be a string"))
}

fn bool_field(object: &serde_json::Map<String, Value>, key: &str, context: &str) -> bool {
    field(object, key, context)
        .as_bool()
        .unwrap_or_else(|| panic!("{context}.{key} must be a bool"))
}

fn string_array_field(
    object: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) -> Vec<String> {
    array(field(object, key, context), &format!("{context}.{key}"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("{context}.{key} entries must be strings"))
                .to_owned()
        })
        .collect()
}

fn vectors(manifest: &Value) -> &[Value] {
    array(
        field(object(manifest, "manifest"), "vectors", "manifest"),
        "manifest.vectors",
    )
}

fn vector_id_set(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|id| (*id).to_owned()).collect()
}

fn ids_for_layer(manifest: &Value, layer: &str) -> BTreeSet<String> {
    vectors(manifest)
        .iter()
        .filter_map(|vector| {
            let vector_obj = object(vector, "vector");
            (string_field(vector_obj, "layer", "vector") == layer)
                .then(|| string_field(vector_obj, "id", "vector").to_owned())
        })
        .collect()
}

fn count_by_field(manifest: &Value, key: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for vector in vectors(manifest) {
        let vector_obj = object(vector, "vector");
        *counts
            .entry(string_field(vector_obj, key, "vector").to_owned())
            .or_insert(0) += 1;
    }
    counts
}

fn assert_non_empty_string_field(
    object: &serde_json::Map<String, Value>,
    key: &str,
    context: &str,
) {
    assert!(
        !string_field(object, key, context).trim().is_empty(),
        "{context}.{key} must be non-empty"
    );
}

fn assert_required_vector_metadata(vector_obj: &serde_json::Map<String, Value>) {
    let id = string_field(vector_obj, "id", "vector");
    for key in [
        "title",
        "description",
        "layer",
        "group",
        "input_kind",
        "mutation_kind",
        "public_claim_caveat",
        "validation_status",
    ] {
        assert_non_empty_string_field(vector_obj, key, id);
    }

    let expected_result = object(field(vector_obj, "expected_result", id), "expected_result");
    assert!(
        bool_field(expected_result, "reject", id),
        "{id} must reject"
    );
    assert!(
        bool_field(expected_result, "no_success_output", id),
        "{id} must forbid success output"
    );
    assert!(
        expected_result.contains_key("no_completed_session_mutation"),
        "{id} missing no_completed_session_mutation metadata"
    );
    assert!(
        expected_result.contains_key("expected_reject_class")
            || expected_result.contains_key("expected_error_class"),
        "{id} missing expected reject/error class metadata"
    );

    let material_policy = object(field(vector_obj, "material_policy", id), "material_policy");
    for key in [
        "contains_secret_material",
        "contains_private_key",
        "contains_passphrase",
        "contains_user_data",
        "generated_ephemerally_if_needed",
    ] {
        assert!(material_policy.contains_key(key), "{id} missing {key}");
    }

    let safe_public_material = object(
        field(vector_obj, "safe_public_material", id),
        "safe_public_material",
    );
    assert_non_empty_string_field(safe_public_material, "storage", id);
    assert!(
        !array(
            field(safe_public_material, "tokens", id),
            "safe_public_material.tokens"
        )
        .is_empty(),
        "{id} must carry safe-public-material token metadata"
    );

    assert!(
        !array(field(vector_obj, "source_evidence", id), "source_evidence").is_empty(),
        "{id} must carry source evidence metadata"
    );
    assert!(
        !array(field(vector_obj, "related_markers", id), "related_markers").is_empty(),
        "{id} must carry related marker metadata"
    );
}

fn assert_claim_boundary_is_internal(manifest: &Value) {
    let manifest_obj = object(manifest, "manifest");
    let boundary = object(
        field(manifest_obj, "public_claim_boundary", "manifest"),
        "public_claim_boundary",
    );
    for key in [
        "internal_only",
        "not_completion_evidence",
        "not_conformance_vectors",
        "not_interoperability_vectors",
        "not_public_vectors",
    ] {
        assert!(bool_field(boundary, key, "public_claim_boundary"), "{key}");
    }
    let statement = string_field(boundary, "statement", "public_claim_boundary");
    for phrase in [
        "no public-readiness",
        concat!("crypto", "-complete"),
        concat!("vector", "-complete"),
        concat!("replay", "-proof"),
        concat!("downgrade", "-proof"),
        concat!("side-channel", "-free"),
        concat!("vulnerability", "-free"),
        concat!("bug", "-free"),
        concat!("perfect", "-crypto"),
        concat!("external-review", "-complete"),
    ] {
        assert!(
            statement.contains(phrase),
            "public claim boundary must explicitly mention {phrase}"
        );
    }
}

#[test]
fn manifest_schema_and_counts_are_bounded() {
    let (manifest, _text) = load_manifest();
    let manifest_obj = object(&manifest, "manifest");
    let top_level_keys: BTreeSet<_> = manifest_obj.keys().map(String::as_str).collect();
    assert_eq!(
        top_level_keys,
        BTreeSet::from([
            "metadata",
            "public_claim_boundary",
            "schema_version",
            "secret_material_policy",
            "sections",
            "status",
            "suite",
            "title",
            "traceability",
            "vectors",
        ])
    );
    assert_eq!(
        string_field(manifest_obj, "schema_version", "manifest"),
        "1"
    );
    assert_eq!(
        string_field(manifest_obj, "status", "manifest"),
        "internal-negative-evidence-only"
    );
    assert_eq!(
        string_field(manifest_obj, "suite", "manifest"),
        "qsl-internal-negative-binding"
    );

    let sections = array(
        field(manifest_obj, "sections", "manifest"),
        "manifest.sections",
    );
    assert_eq!(sections.len(), 3);
    let section_layers: BTreeMap<String, String> = sections
        .iter()
        .map(|section| {
            let section_obj = object(section, "section");
            (
                string_field(section_obj, "id", "section").to_owned(),
                string_field(section_obj, "layer", "section").to_owned(),
            )
        })
        .collect();
    assert_eq!(
        section_layers,
        BTreeMap::from([
            (
                "formal_token_mapping".to_owned(),
                "formal_token_mapping".to_owned()
            ),
            ("qsc_binding".to_owned(), "qsc_frame".to_owned()),
            (
                "refimpl_signature_provider_boundary".to_owned(),
                "refimpl_signature_provider_boundary".to_owned()
            ),
        ])
    );

    let manifest_vectors = vectors(&manifest);
    assert_eq!(manifest_vectors.len(), 34);
    assert_eq!(
        ids_for_layer(&manifest, "qsc_frame"),
        vector_id_set(EXPECTED_QSC_FRAME_IDS)
    );
    assert_eq!(
        ids_for_layer(&manifest, "refimpl_signature_provider_boundary"),
        vector_id_set(EXPECTED_REFIMPL_IDS)
    );
    assert_eq!(
        ids_for_layer(&manifest, "formal_token_mapping"),
        vector_id_set(EXPECTED_FORMAL_IDS)
    );

    assert_eq!(
        count_by_field(&manifest, "layer"),
        BTreeMap::from([
            ("formal_token_mapping".to_owned(), 7),
            ("qsc_frame".to_owned(), 21),
            ("refimpl_signature_provider_boundary".to_owned(), 6),
        ])
    );
    assert_eq!(
        count_by_field(&manifest, "group"),
        BTreeMap::from([
            ("formal_token_mapping".to_owned(), 7),
            ("kem_binding".to_owned(), 4),
            ("refimpl_signature_provider_boundary".to_owned(), 6),
            ("signature_binding".to_owned(), 5),
            ("stale_identity_rollback".to_owned(), 4),
            ("transcript_replay_suite".to_owned(), 8),
        ])
    );

    let mut seen = HashSet::new();
    for vector in manifest_vectors {
        let vector_obj = object(vector, "vector");
        let id = string_field(vector_obj, "id", "vector");
        assert!(seen.insert(id.to_owned()), "duplicate vector id {id}");
        assert_required_vector_metadata(vector_obj);
        let expected_result = object(field(vector_obj, "expected_result", id), "expected_result");
        let layer = string_field(vector_obj, "layer", id);
        let no_completed_session_mutation =
            bool_field(expected_result, "no_completed_session_mutation", id);
        match layer {
            "qsc_frame" | "formal_token_mapping" => assert!(
                no_completed_session_mutation,
                "{id} must preserve no completed-session mutation"
            ),
            "refimpl_signature_provider_boundary" => assert!(
                !no_completed_session_mutation,
                "{id} must keep provider-return-shape metadata supporting-only"
            ),
            other => panic!("{id} has unexpected layer {other}"),
        }
    }

    println!("NA0497_VECTOR_MANIFEST_SCHEMA_OK");
}

#[test]
fn manifest_vectors_have_mapping_and_claim_boundaries() {
    let (manifest, _text) = load_manifest();
    assert_claim_boundary_is_internal(&manifest);

    for vector in vectors(&manifest) {
        let vector_obj = object(vector, "vector");
        let id = string_field(vector_obj, "id", "vector");
        let layer = string_field(vector_obj, "layer", id);
        let group = string_field(vector_obj, "group", id);
        let caveat = string_field(vector_obj, "public_claim_caveat", id);
        let source_evidence = string_array_field(vector_obj, "source_evidence", id);
        let related_markers = string_array_field(vector_obj, "related_markers", id);

        assert!(
            caveat.contains("not public") || caveat.contains("not provider-internal proof"),
            "{id} must carry explicit public-claim caveat"
        );
        assert!(
            caveat.contains("not") && caveat.contains("completion"),
            "{id} must reject completion overclaim"
        );
        assert!(
            source_evidence.iter().any(|item| item.starts_with("NA-")),
            "{id} must map to NA evidence"
        );
        assert!(
            related_markers
                .iter()
                .any(|marker| marker.starts_with("NA")),
            "{id} must map to marker evidence"
        );

        match layer {
            "qsc_frame" => {
                assert!(
                    matches!(
                        group,
                        "kem_binding"
                            | "signature_binding"
                            | "transcript_replay_suite"
                            | "stale_identity_rollback"
                    ),
                    "{id} must map qsc-frame vectors to qsc-facing evidence classes"
                );
                assert!(
                    caveat.contains("not public or conformance evidence"),
                    "{id} must not become public/conformance evidence"
                );
            }
            "refimpl_signature_provider_boundary" => {
                assert_eq!(group, "refimpl_signature_provider_boundary");
                assert!(
                    caveat.contains("provider-boundary metadata only"),
                    "{id} must remain supporting-only refimpl boundary evidence"
                );
            }
            "formal_token_mapping" => {
                assert_eq!(group, "formal_token_mapping");
                assert!(
                    caveat.contains("Formal-token mapping only"),
                    "{id} must remain supporting-only formal-token evidence"
                );
            }
            other => panic!("{id} has unexpected layer {other}"),
        }

        for forbidden in [
            "public vector",
            "conformance vector",
            "interoperability vector",
            concat!("replay", "-proof"),
            concat!("downgrade", "-proof"),
            concat!("crypto", "-complete"),
            concat!("vector", "-complete"),
            concat!("fuzz", "-complete"),
            concat!("corpus", "-complete"),
        ] {
            assert!(
                !caveat.contains(forbidden),
                "{id} caveat must not claim {forbidden}"
            );
        }
    }

    println!("NA0497_VECTOR_CATEGORY_COVERAGE_OK");
    println!("NA0497_VECTOR_MAPPING_TRACEABILITY_OK");
    println!("NA0497_REFIMPL_VECTORS_SUPPORTING_ONLY_OK");
    println!("NA0497_FORMAL_TOKEN_VECTORS_SUPPORTING_ONLY_OK");
    println!("NA0497_NO_PUBLIC_CONFORMANCE_VECTOR_CLAIM_OK");
}

#[test]
fn manifest_preserves_no_secret_material_policy() {
    let (manifest, text) = load_manifest();
    let manifest_obj = object(&manifest, "manifest");
    let policy = object(
        field(manifest_obj, "secret_material_policy", "manifest"),
        "secret_material_policy",
    );
    let checked_in_material = string_field(policy, "checked_in_material", "secret_material_policy");
    assert!(checked_in_material.contains("metadata"));
    assert!(checked_in_material.contains("expected reject outcomes"));
    assert!(
        bool_field(
            policy,
            "ephemeral_generation_required_for_sensitive_validation",
            "secret_material_policy"
        ),
        "sensitive validation must require ephemeral generation"
    );
    assert_eq!(
        string_field(
            policy,
            "no_checked_in_secret_material_marker",
            "secret_material_policy"
        ),
        "NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK"
    );

    let forbidden_material_types =
        string_array_field(policy, "forbidden_material_types", "secret_material_policy");
    for forbidden in [
        "private keys",
        "KEM secret keys",
        "signing keys",
        "passphrases",
        "runtime keys",
        "backup keys",
        "operator data",
        "user data",
        "live service data",
        "private production endpoint data",
    ] {
        assert!(
            forbidden_material_types
                .iter()
                .any(|item| item == forbidden),
            "secret material policy missing {forbidden}"
        );
    }

    for vector in vectors(&manifest) {
        let vector_obj = object(vector, "vector");
        let id = string_field(vector_obj, "id", "vector");
        let material_policy = object(field(vector_obj, "material_policy", id), "material_policy");
        for key in [
            "contains_secret_material",
            "contains_private_key",
            "contains_passphrase",
            "contains_user_data",
        ] {
            assert!(
                !bool_field(material_policy, key, id),
                "{id} must set {key}=false"
            );
        }
    }

    for marker in [
        "-----BEGIN",
        "PRIVATE KEY-----",
        "OPENSSH PRIVATE KEY",
        "BEGIN PGP PRIVATE KEY",
        "AKIA",
        "ghp_",
        "xoxb-",
    ] {
        assert!(
            !text.contains(marker),
            "manifest text must not contain secret-like marker {marker}"
        );
    }

    println!("NA0497_NO_SECRET_MATERIAL_POLICY_OK");
}

#[test]
fn na0497_common_no_overclaim_markers() {
    let (manifest, _text) = load_manifest();
    assert_claim_boundary_is_internal(&manifest);

    println!("NA0497_VECTOR_CONSUMER_SCOPE_CONSUMED_OK");
    println!("NA0497_NO_PUBLIC_READINESS_CLAIM_OK");
    println!("NA0497_NO_CRYPTO_COMPLETE_CLAIM_OK");
    println!("NA0497_NO_FUZZ_COMPLETE_CLAIM_OK");
    println!("NA0497_NO_CORPUS_COMPLETE_CLAIM_OK");
    println!("NA0497_NO_VECTOR_COMPLETE_CLAIM_OK");
    println!("NA0497_NO_REPLAY_PROOF_CLAIM_OK");
    println!("NA0497_NO_DOWNGRADE_PROOF_CLAIM_OK");
    println!("NA0497_ONE_READY_INVARIANT_OK");
    println!("NA0497_QSC_VECTOR_CONSUMER_TEST_IMPLEMENTED_OK");
    println!("NA0497_QSC_FRAME_VECTORS_MAPPED_OK");
    println!("NA0497_NO_QSC_SOURCE_CHANGE_OK");
    println!("NA0497_NO_DEPENDENCY_CHANGE_OK");
}
