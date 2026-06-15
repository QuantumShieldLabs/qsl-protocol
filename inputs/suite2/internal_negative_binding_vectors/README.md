# QSL Internal Negative Binding Vectors

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

## Scope

This directory contains internal negative binding vector evidence for QSL
Suite-2 binding review. These files are internal negative evidence only. They
are not public vectors, conformance vectors, interoperability vectors, or
completion evidence.

The manifest in this directory contains no private keys, KEM secret keys,
signing keys, passphrases, runtime keys, backup keys, operator data, user data,
or live service data. Any secret material needed for future validation must be
generated ephemerally by tests and must not be checked in.

The manifest covers metadata, public or mutated public message descriptions
where safe, formal tokens, expected reject outcomes, and no-mutation
expectations. It is intended to be read by later authorization or
implementation lanes and consumed by tests only after exact directive
authorization. It is supporting evidence for external-review readiness, not
external-review-complete evidence.

## Manifest Schema

The JSON manifest is a top-level object with:

- `schema_version`: manifest schema version.
- `suite`: internal suite identifier.
- `status`: internal evidence status.
- `public_claim_boundary`: public and release claim caveats.
- `secret_material_policy`: checked-in material boundary.
- `traceability`: links to prior NA evidence.
- `sections`: logical sections for qsc binding, refimpl signature
  provider-boundary metadata, and formal-token mapping.
- `vectors`: vector metadata entries.

Every vector entry includes:

- `id`
- `group`
- `layer`
- `title`
- `description`
- `source_evidence`
- `material_policy`
- `input_kind`
- `mutation_kind`
- `expected_result`
- `validation_status`
- `public_claim_caveat`
- `related_markers`

The allowed `layer` values are:

- `qsc_frame`
- `refimpl_signature_provider_boundary`
- `formal_token_mapping`

## Sections

### qsc_binding

Negative qsc binding metadata for KEM, signature, transcript, replay, suite,
stale identity, and rollback cases. These entries describe expected fail-closed
reject behavior and no completed-session mutation expectations. They do not
store qsc runtime secrets.

### refimpl_signature_provider_boundary

Refimpl ML-DSA provider-boundary metadata for malformed length inputs,
well-shaped invalid inputs, and Err versus `Ok(false)` classification. These
entries describe provider-boundary expectations without storing signing keys.

### formal_token_mapping

Opaque formal-token mapping entries corresponding to NA-0478 bounded model
cases. These entries use abstract tokens such as wrong KEM token, wrong
signature token, replay token, suite-confusion token, and stale public-record
token.

## Validation

Validate the manifest JSON from the qsl-protocol repo root:

```bash
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
```

## Claim Boundary

This directory makes no public-readiness, crypto-complete, vector-complete,
KEM-complete, signature-complete, identity-complete, transcript-complete,
replay-proof, downgrade-proof, side-channel-free, vulnerability-free,
bug-free, perfect-crypto, external-review-complete, backup-complete, or
restore-proof claim. Cargo audit output remains dependency-health evidence
only.

Required NA-0483 markers:

- `NA0483_VECTOR_SCOPE_CONSUMED_OK`
- `NA0483_NO_SECRET_MATERIAL_IN_VECTORS_OK`
- `NA0483_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0483_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0483_NO_VECTOR_COMPLETE_CLAIM_OK`
