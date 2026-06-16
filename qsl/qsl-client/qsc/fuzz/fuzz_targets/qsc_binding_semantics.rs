#![no_main]

use libfuzzer_sys::fuzz_target;
use qsc::adversarial::binding_fuzz::{
    category_from_input, exercise_binding_fuzz_case, BindingFuzzCategory,
    NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK, NA0487_HELPER_API_NO_SECRET_OUTPUT_OK,
    NA0487_HELPER_API_REAL_REJECT_PATHS_OK, NA0487_HELPER_API_TEST_FUZZ_ONLY_OK,
    NA0487_HELPER_API_VECTOR_TRACEABILITY_OK,
};

const _NA0487_FUZZ_A1_MUTATION_TARGET_OK: &str = "NA0487_FUZZ_A1_MUTATION_TARGET_OK";
const _NA0487_FUZZ_B1_MUTATION_TARGET_OK: &str = "NA0487_FUZZ_B1_MUTATION_TARGET_OK";
const _NA0487_FUZZ_A2_MUTATION_TARGET_OK: &str = "NA0487_FUZZ_A2_MUTATION_TARGET_OK";
const _NA0487_FUZZ_SUITE_CONFUSION_TARGET_OK: &str =
    "NA0487_FUZZ_SUITE_CONFUSION_TARGET_OK";
const _NA0487_FUZZ_REPLAY_TARGET_OK: &str = "NA0487_FUZZ_REPLAY_TARGET_OK";
const _NA0487_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK: &str =
    "NA0487_FUZZ_STALE_PUBLIC_RECORD_TARGET_OK";
const _NA0487_FUZZ_VECTOR_MANIFEST_TRACEABILITY_OK: &str =
    "NA0487_FUZZ_VECTOR_MANIFEST_TRACEABILITY_OK";
const _NA0487_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK: &str =
    "NA0487_NO_SECRET_MATERIAL_IN_FUZZ_INPUTS_OK";
const _NA0487_NO_FUZZ_COMPLETE_CLAIM_OK: &str = "NA0487_NO_FUZZ_COMPLETE_CLAIM_OK";
const _NA0487_NO_VECTOR_COMPLETE_CLAIM_OK: &str = "NA0487_NO_VECTOR_COMPLETE_CLAIM_OK";
const _NA0487_NO_REPLAY_PROOF_CLAIM_OK: &str = "NA0487_NO_REPLAY_PROOF_CLAIM_OK";
const _NA0487_NO_DOWNGRADE_PROOF_CLAIM_OK: &str = "NA0487_NO_DOWNGRADE_PROOF_CLAIM_OK";

const _NA0489_CORPUS_STRATEGY_CONSUMED_OK: &str = "NA0489_CORPUS_STRATEGY_CONSUMED_OK";
const _NA0489_EPHEMERAL_SEED_GENERATION_ONLY_OK: &str =
    "NA0489_EPHEMERAL_SEED_GENERATION_ONLY_OK";
const _NA0489_NO_CHECKED_IN_CORPUS_OK: &str = "NA0489_NO_CHECKED_IN_CORPUS_OK";
const _NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY_OK: &str =
    "NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY_OK";
const _NA0489_NO_SECRET_MATERIAL_IN_CORPUS_OK: &str =
    "NA0489_NO_SECRET_MATERIAL_IN_CORPUS_OK";
const _NA0489_NO_PUBLIC_READINESS_CLAIM_OK: &str =
    "NA0489_NO_PUBLIC_READINESS_CLAIM_OK";
const _NA0489_NO_CRYPTO_COMPLETE_CLAIM_OK: &str = "NA0489_NO_CRYPTO_COMPLETE_CLAIM_OK";
const _NA0489_NO_FUZZ_COMPLETE_CLAIM_OK: &str = "NA0489_NO_FUZZ_COMPLETE_CLAIM_OK";
const _NA0489_NO_CORPUS_COMPLETE_CLAIM_OK: &str = "NA0489_NO_CORPUS_COMPLETE_CLAIM_OK";
const _NA0489_NO_VECTOR_COMPLETE_CLAIM_OK: &str = "NA0489_NO_VECTOR_COMPLETE_CLAIM_OK";
const _NA0489_NO_REPLAY_PROOF_CLAIM_OK: &str = "NA0489_NO_REPLAY_PROOF_CLAIM_OK";
const _NA0489_NO_DOWNGRADE_PROOF_CLAIM_OK: &str = "NA0489_NO_DOWNGRADE_PROOF_CLAIM_OK";

const HS_MAGIC: &[u8; 4] = b"QHSM";
const HS_VERSION_V2: u16 = 2;
const HS_TYPE_INIT: u8 = 1;
const HS_TYPE_RESP: u8 = 2;
const HS_TYPE_CONFIRM: u8 = 3;
const HS_PARAM_SUITE_CONTEXT: u16 = 0x0001;
const HS_PARAM_FLAG_CRITICAL: u8 = 0x01;
const HS_SUITE2_PROTOCOL_VERSION_WIRE: u16 = 0x0500;
const HS_SUITE2_SUITE_ID_WIRE: u16 = 0x0002;
const HS_LEGACY_PROTOCOL_VERSION_WIRE: u16 = 0x0403;
const HS_LEGACY_SUITE_ID_WIRE: u16 = 0x0001;
const KEM_PUBLIC_KEY_LEN: usize = 1184;
const KEM_CIPHERTEXT_LEN: usize = 1088;
const SIG_PUBLIC_KEY_LEN: usize = 1952;
const SIG_SIGNATURE_LEN: usize = 3309;
const TRANSCRIPT_BINDING_LEN: usize = 32;
const SESSION_ID_LEN: usize = 16;

fuzz_target!(|data: &[u8]| {
    let _ = NA0487_BINDING_FUZZ_HELPER_SCOPE_CONSUMED_OK;
    let _ = NA0487_HELPER_API_TEST_FUZZ_ONLY_OK;
    let _ = NA0487_HELPER_API_NO_SECRET_OUTPUT_OK;
    let _ = NA0487_HELPER_API_REAL_REJECT_PATHS_OK;
    let _ = NA0487_HELPER_API_VECTOR_TRACEABILITY_OK;
    let _ = _NA0489_CORPUS_STRATEGY_CONSUMED_OK;
    let _ = _NA0489_EPHEMERAL_SEED_GENERATION_ONLY_OK;
    let _ = _NA0489_NO_CHECKED_IN_CORPUS_OK;
    let _ = _NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY_OK;
    let _ = _NA0489_NO_SECRET_MATERIAL_IN_CORPUS_OK;
    let _ = _NA0489_NO_PUBLIC_READINESS_CLAIM_OK;
    let _ = _NA0489_NO_CRYPTO_COMPLETE_CLAIM_OK;
    let _ = _NA0489_NO_FUZZ_COMPLETE_CLAIM_OK;
    let _ = _NA0489_NO_CORPUS_COMPLETE_CLAIM_OK;
    let _ = _NA0489_NO_VECTOR_COMPLETE_CLAIM_OK;
    let _ = _NA0489_NO_REPLAY_PROOF_CLAIM_OK;
    let _ = _NA0489_NO_DOWNGRADE_PROOF_CLAIM_OK;

    let category = if data.first().copied().unwrap_or(0) == 0xff {
        BindingFuzzCategory::VectorTraceability
    } else {
        category_from_input(data)
    };
    let _ = exercise_binding_fuzz_case(category, data.get(1..).unwrap_or_default());

    let (seed_category, seed) = ephemeral_seed_from_input(data);
    let _ = exercise_binding_fuzz_case(seed_category, seed.as_slice());
});

fn ephemeral_seed_from_input(data: &[u8]) -> (BindingFuzzCategory, Vec<u8>) {
    match data.first().copied().unwrap_or(0) % 7 {
        0 => (BindingFuzzCategory::A1Mutation, a1_mutation_seed(data)),
        1 => (BindingFuzzCategory::B1Mutation, b1_mutation_seed(data)),
        2 => (BindingFuzzCategory::A2Mutation, a2_mutation_seed(data)),
        3 => (
            BindingFuzzCategory::SuiteConfusion,
            suite_confusion_seed(data),
        ),
        4 => (BindingFuzzCategory::Replay, replay_seed(data)),
        5 => (
            BindingFuzzCategory::StalePublicRecord,
            stale_public_record_seed(data),
        ),
        _ => (
            BindingFuzzCategory::VectorTraceability,
            vector_manifest_traceability_seed(data),
        ),
    }
}

fn a1_mutation_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 kem_wrong_peer_public_key / transcript_mutation.
    let mut seed = frame_seed(
        HS_TYPE_INIT,
        suite2_context_block(),
        a1_payload(data, b"NA0489_A1_MUTATION_PUBLIC_SYNTHETIC"),
    );
    mutate_payload_byte(&mut seed, a1_payload_offset() + SESSION_ID_LEN, data, 0xA1);
    seed
}

fn b1_mutation_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 kem_wrong_ciphertext.
    let mut seed = frame_seed(
        HS_TYPE_RESP,
        suite2_context_block(),
        b1_payload(data, b"NA0489_B1_MUTATION_PUBLIC_SYNTHETIC"),
    );
    mutate_payload_byte(&mut seed, b1_payload_offset() + SESSION_ID_LEN, data, 0xB1);
    seed
}

fn a2_mutation_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 signature_wrong_identity / transcript_mutation.
    let mut seed = frame_seed(
        HS_TYPE_CONFIRM,
        suite2_context_block(),
        a2_payload(data, b"NA0489_A2_MUTATION_PUBLIC_SYNTHETIC"),
    );
    mutate_payload_byte(
        &mut seed,
        a2_payload_offset() + SESSION_ID_LEN + TRANSCRIPT_BINDING_LEN,
        data,
        0xA2,
    );
    seed
}

fn suite_confusion_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 suite_confusion_wrong_suite_token.
    frame_seed(
        HS_TYPE_INIT,
        confused_suite_context_block(data),
        a1_payload(data, b"NA0489_SUITE_CONFUSION_PUBLIC_SYNTHETIC"),
    )
}

fn replay_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 signature_cross_message_replay / replay cases.
    frame_seed(
        HS_TYPE_INIT,
        suite2_context_block(),
        a1_payload(data, b"NA0489_REPLAY_PUBLIC_SYNTHETIC"),
    )
}

fn stale_public_record_seed(data: &[u8]) -> Vec<u8> {
    // Traceability: NA-0483 kem_stale_public_record / stale_public_record_replay.
    let mut seed = synthetic_bytes(8, b"NA0489_STALE_PUBLIC_RECORD_PUBLIC_SYNTHETIC", data);
    if let Some(first) = seed.first_mut() {
        *first &= 0xfe;
    }
    seed
}

fn vector_manifest_traceability_seed(data: &[u8]) -> Vec<u8> {
    // Traceability-only static category names; the manifest JSON is not read at runtime.
    const TRACE_IDS: &[&[u8]] = &[
        b"kem_wrong_peer_public_key",
        b"kem_wrong_ciphertext",
        b"signature_wrong_identity_public_record",
        b"signature_cross_message_replay_b1_as_a2",
        b"transcript_mutation",
        b"suite_confusion_wrong_suite_token",
        b"stale_public_record_replay",
        b"formal_token_mapping",
    ];
    let mut seed = b"NA0489_VECTOR_MANIFEST_TRACEABILITY_ONLY:".to_vec();
    for id in TRACE_IDS {
        seed.extend_from_slice(id);
        seed.push(b';');
    }
    let selector = data.get(1).copied().unwrap_or(0) % 6;
    seed.push(selector);
    seed.extend_from_slice(&synthetic_bytes(
        64,
        b"NA0489_VECTOR_TRACE_PUBLIC_SYNTHETIC",
        data,
    ));
    seed
}

fn frame_seed(frame_type: u8, suite_block: [u8; 9], payload: Vec<u8>) -> Vec<u8> {
    let mut seed = Vec::with_capacity(9 + suite_block.len() + payload.len());
    seed.extend_from_slice(HS_MAGIC);
    seed.extend_from_slice(&HS_VERSION_V2.to_be_bytes());
    seed.push(frame_type);
    seed.extend_from_slice(&(suite_block.len() as u16).to_be_bytes());
    seed.extend_from_slice(&suite_block);
    seed.extend_from_slice(payload.as_slice());
    seed
}

fn suite2_context_block() -> [u8; 9] {
    suite_context_block(HS_SUITE2_PROTOCOL_VERSION_WIRE, HS_SUITE2_SUITE_ID_WIRE)
}

fn confused_suite_context_block(data: &[u8]) -> [u8; 9] {
    match data.get(1).copied().unwrap_or(0) % 3 {
        0 => suite_context_block(HS_LEGACY_PROTOCOL_VERSION_WIRE, HS_LEGACY_SUITE_ID_WIRE),
        1 => suite_context_block(HS_SUITE2_PROTOCOL_VERSION_WIRE, HS_LEGACY_SUITE_ID_WIRE),
        _ => suite_context_block(HS_LEGACY_PROTOCOL_VERSION_WIRE, HS_SUITE2_SUITE_ID_WIRE),
    }
}

fn suite_context_block(protocol_version: u16, suite_id: u16) -> [u8; 9] {
    let protocol_version = protocol_version.to_be_bytes();
    let suite_id = suite_id.to_be_bytes();
    [
        (HS_PARAM_SUITE_CONTEXT >> 8) as u8,
        HS_PARAM_SUITE_CONTEXT as u8,
        HS_PARAM_FLAG_CRITICAL,
        0,
        4,
        protocol_version[0],
        protocol_version[1],
        suite_id[0],
        suite_id[1],
    ]
}

fn a1_payload(data: &[u8], label: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(
        SESSION_ID_LEN + KEM_PUBLIC_KEY_LEN + SIG_PUBLIC_KEY_LEN + TRANSCRIPT_BINDING_LEN,
    );
    payload.extend_from_slice(session_id(data, label).as_slice());
    payload.extend_from_slice(synthetic_bytes(KEM_PUBLIC_KEY_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(SIG_PUBLIC_KEY_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(TRANSCRIPT_BINDING_LEN, label, data).as_slice());
    payload
}

fn b1_payload(data: &[u8], label: &[u8]) -> Vec<u8> {
    let mut payload = Vec::with_capacity(
        SESSION_ID_LEN
            + KEM_CIPHERTEXT_LEN
            + TRANSCRIPT_BINDING_LEN
            + SIG_PUBLIC_KEY_LEN
            + SIG_SIGNATURE_LEN
            + TRANSCRIPT_BINDING_LEN,
    );
    payload.extend_from_slice(session_id(data, label).as_slice());
    payload.extend_from_slice(synthetic_bytes(KEM_CIPHERTEXT_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(TRANSCRIPT_BINDING_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(SIG_PUBLIC_KEY_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(SIG_SIGNATURE_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(TRANSCRIPT_BINDING_LEN, label, data).as_slice());
    payload
}

fn a2_payload(data: &[u8], label: &[u8]) -> Vec<u8> {
    let mut payload =
        Vec::with_capacity(SESSION_ID_LEN + TRANSCRIPT_BINDING_LEN + SIG_SIGNATURE_LEN);
    payload.extend_from_slice(session_id(data, label).as_slice());
    payload.extend_from_slice(synthetic_bytes(TRANSCRIPT_BINDING_LEN, label, data).as_slice());
    payload.extend_from_slice(synthetic_bytes(SIG_SIGNATURE_LEN, label, data).as_slice());
    payload
}

fn session_id(data: &[u8], label: &[u8]) -> [u8; SESSION_ID_LEN] {
    let bytes = synthetic_bytes(SESSION_ID_LEN, label, data);
    let mut session = [0u8; SESSION_ID_LEN];
    session.copy_from_slice(bytes.as_slice());
    session
}

fn synthetic_bytes(len: usize, label: &[u8], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(len);
    for idx in 0..len {
        let label_byte = label[idx % label.len()];
        let data_byte = data.get(idx % data.len().max(1)).copied().unwrap_or(0);
        out.push(label_byte ^ data_byte ^ (idx as u8).wrapping_mul(31));
    }
    out
}

fn mutate_payload_byte(seed: &mut [u8], base_offset: usize, data: &[u8], mask: u8) {
    if seed.is_empty() || base_offset >= seed.len() {
        return;
    }
    let remaining = seed.len() - base_offset;
    let delta = data.get(2).copied().unwrap_or(0) as usize % remaining;
    seed[base_offset + delta] ^= mask;
}

fn a1_payload_offset() -> usize {
    9 + suite2_context_block().len()
}

fn b1_payload_offset() -> usize {
    9 + suite2_context_block().len()
}

fn a2_payload_offset() -> usize {
    9 + suite2_context_block().len()
}
