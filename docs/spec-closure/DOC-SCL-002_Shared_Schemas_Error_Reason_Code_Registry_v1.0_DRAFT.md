# QuantumShield — Shared Schemas + Error and Reason Code Registry

Doc ID: DOC-SCL-002  
Version: v1.0  
Status: DRAFT  
Last Updated: 2025-12-26  
Audience: Internal (engineering/ops), External (implementers; optional publication)  
Normative: **YES** (service error model + reason codes; harness/gate error taxonomy)  
Supersedes: Phase3 P3-14 v1.0 (Shared Schema & Error Code Registry)
**Artifact ID:** P3-14  
**Category:** Supporting (atomic)  
**Phase:** 3 (Phase 2 frozen; canonical specs frozen)  
**Canonical refs (alignment only):** QSP 4.3.2 (REVIEWED FULL), QSE 1.8.2 (REVIEWED FULL)  
**Version:** 1.0  
**Date:** 2025-12-20  
**Timezone:** America/Chicago

## 0. Purpose and scope
> Phase 4 alignment note: This registry MUST align with canonical requirements in DOC-CAN-001 (QSP) and DOC-CAN-002 (QSE),
> and with program governance in DOC-CTRL-002. Where protocol-layer rejects exist, they SHOULD map to a small, stable set of
> service-layer reason codes, while detailed diagnostics remain local (logs/evidence artifacts).
This document defines the **shared, machine-readable schemas** and **standardized error codes** used by Phase 3 services and SDK integrations:
- RSF (Relay / Store-and-Forward),
- PDS (Prekey Directory Service),
- KTL (Key Transparency Log),
- client SDKs consuming those APIs.

This registry exists to prevent divergence between:
- prose contracts (P3-06, P3-09),
- OpenAPI definitions (P3-13), and
- generated clients/servers and test harnesses.

This is a **supporting** artifact and does not modify QSP/QSE. All canonical protocol objects (QSE envelopes, QSP bundles/messages, KT objects) remain defined by the canonical specs and are transported through service APIs as **opaque bytes** encoded with base64url.

## 1. Deliverables in this artifact
This artifact consists of:
- This document (human-readable normative registry)
- `DOC-SCL-002_Shared_Schemas_v1.0.json` (machine-readable JSON Schema bundle; informational and SDK-support)
- `DOC-SCL-002_Reason_Codes_v1.0.json` (machine-readable reason code list; mirrors Section 5)

This artifact consists of the following files (packaged together in the DOC-SCL-002 ZIP):
1. `QuantumShield_Phase3_Shared_Schema_Error_Code_Registry_P3-14_v1_0.md` (this document)
2. `QuantumShield_Phase3_Shared_Schemas_P3-14_v1_0.json` (JSON Schema bundle; draft 2020-12)
3. `QuantumShield_Phase3_Reason_Codes_P3-14_v1_0.json` (reason code registry enum)

## 2. Design principles
### 2.1 Fail-closed parsing
- All schemas are written to support strict decoding:
  - reject unknown fields where `additionalProperties=false`,
  - reject invalid base64url characters,
  - enforce explicit min/max bounds.

### 2.2 Opaque canonical bytes
Fields ending in `_b64` represent raw bytes encoded as **base64url without padding** (RFC 4648 URL-safe alphabet; no trailing '=').
These bytes may represent canonical protocol objects (e.g., QSE envelope bytes), but services and schemas treat them as **opaque**.

### 2.3 Sanitized errors
Errors MUST:
- avoid raw identifiers (e.g., do not include route_token bytes),
- avoid secrets (auth tokens, internal keys),
- use **reason codes** from the registry.

## 3. base64url conventions (normative)
### 3.1 Base64url alphabet and padding
- Allowed characters: `A–Z a–z 0–9 - _`
- Padding `=` MUST NOT appear.
- Services MUST reject any input that fails validation.

### 3.2 Length constraints for fixed-size fields
The JSON Schemas define “fixed-bytes” types such as:
- `FixedBytes32B64`: exactly 32 bytes encoded in base64url
- `FixedBytes16B64`: exactly 16 bytes encoded in base64url

Note: JSON Schema cannot reliably enforce decoded-byte length using regex alone; therefore:
- the schema enforces syntactic base64url validity, and
- implementations MUST additionally enforce decoded length at runtime.

## 4. Standard error model (normative)
All non-2xx service responses MUST return a JSON body with:

- `error_code` (string): a value from the **ReasonCode** registry
- `message` (string): sanitized human-readable message, safe for logs
- `retryable` (boolean): whether a retry is expected to succeed
- `details` (object, optional): sanitized structured diagnostics (no secrets, no raw identifiers)

This structure is designed to be uniform across RSF/PDS/KTL and to simplify SDK handling.

## 5. Reason code registry (normative)
The following reason codes are the registry. Services MAY add new codes only via a new registry version.

**Service-layer (primary)**
- `noncanonical_qse` — QSE envelope not canonically encoded (trailing bytes, truncation, length-smuggling, etc.)
- `bounds_exceeded` — a size or count bound was exceeded (request too large, route_token too long, etc.)
- `invalid_request` — request schema violated or invalid parameter combination
- `rate_limited` — request rejected by rate limiter
- `queue_full` — storage/capacity policy rejection (RSF)
- `auth_failed` — missing/invalid authentication
- `forbidden` — caller authenticated but not authorized
- `not_found` — resource not found (route_token not mapped, device not found, etc.)
- `conflict` — revision mismatch, token collision, idempotency conflict, or OPK-required depletion
- `opk_unavailable` — OPK required but unavailable (PDS)
- `server_error` — internal server error (sanitized)

**Telemetry-only (secondary; sanitized)**
These are permitted for **client-side reason aggregation** and incident triage. Services SHOULD avoid returning these unless specifically defined by an endpoint contract.

- `kt_fail` — key transparency verification failure (sanitized; no raw proofs or identifiers)
- `bundle_sig_fail` — bundle signature verification failure (sanitized)
- `aead_fail` — authenticated decryption failed (e.g., wrong epoch/state; authentication failure)
- `replay` — replay detected (may include durable rollback replay detection)
- `policy_reject` — policy-based rejection (e.g., fail-closed crash recovery, invariant violation, or security posture enforcement)

## 5A. Harness and CI gate error taxonomy (normative)
This section defines **artifact-level error codes** used by automated gates (ci-4a..ci-4d-dur), harness runners,
and evidence artifacts (e.g., `artifacts/*/*.json`). These codes are **not** service-layer reason codes.
They exist to make CI enforcement deterministic and to enable machine parsing of failures.

### 5A.1 Design constraints
- Codes MUST be stable and backwards-compatible within a Phase 4 line.
- Codes MUST be **coarse** (avoid leaking secrets or internal identifiers).
- Codes MUST be actionable: each code MUST map to a clear operator/engineer response.
- Codes MUST be fail-closed: “unknown” formats are failures, not warnings.

### 5A.2 Registry (Phase 4 baseline)
**Parsing / format**
- `NEGATIVE_FORMAT_UNKNOWN` — Negative vector file format not recognized or schema invalid; execution aborted.
- `INTEROP_FORMAT_UNKNOWN` — Interop case file format not recognized or schema invalid; execution aborted.
- `VECTOR_PARSE_FAILED` — Vector parsed but validation failed (missing required fields, malformed base64url, etc.).

**Execution wiring / harness**
- `NEGATIVE_NOT_EXECUTED` — Vectors were parsed but no executable wiring existed; treated as failure in fail-closed mode.
- `CASE_NOT_EXECUTED` — Test case selected but not executed due to missing adapter/actor operation.
- `HARNESS_EXCEPTION` — Unhandled exception in harness execution path; MUST include sanitized stack context in evidence logs (not in JSON error message).

**Interop / result mismatches**
- `INTEROP_CASE_FAILED` — A case executed but failed (ciphertext mismatch, transcript mismatch, reject mismatch, etc.).
- `TRANSCRIPT_MISMATCH` — Transcript hash or transcript binding mismatch between implementations.
- `CANONICALIZATION_MISMATCH` — Canonical encoding mismatch (e.g., base64url-no-pad expectations violated).

**Durability / persistence**
- `ROLLBACK_DETECTED` — Rollback detection triggered (epoch/generation regression, inconsistent persisted state).
- `REPLAY_DETECTED` — Replay cache detected a duplicate that violates policy.
- `STATE_CORRUPT` — Persisted state failed integrity checks or could not be decoded; fail-closed.
- `CRASH_UNCERTAIN_STATE` — Crash recovery could not prove atomic commit; fail-closed.

**Evidence packaging**
- `EVIDENCE_MISSING` — Required evidence artifact not present; fail-closed.
- `ARTIFACT_WRITE_FAILED` — Harness could not write required artifacts.
- `CONFIG_INVALID` — Configuration invalid (e.g., missing required suite/profile, bounds incoherent).

### 5A.3 Severity mapping (recommended)
- Parsing/wiring/evidence failures SHOULD be treated as **fatal** in CI.
- Interop and durability failures MUST be treated as **fatal**.

## 5B. Protocol-level reject identifiers (Suite-2 establishment)
These identifiers are used by canonical Suite-2 specs (DOC-CAN-003) to indicate fail-closed establishment errors.
They are not service-layer error codes and should be surfaced only in protocol/harness contexts.

- `REJECT_S2_ESTABLISH_BAD_MSG_TYPE` — Message type is invalid for Suite-2 establishment.
- `REJECT_S2_ESTABLISH_BAD_INPUT_LEN` — Base handshake outputs for Suite-2 have invalid lengths.
- `REJECT_S2_ESTABLISH_UNAUTHENTICATED` — Base handshake cannot provide authenticated commitment to Suite-2 negotiation and session_id.

## 6. Schema bundle contents
The JSON Schema bundle includes:
- shared primitives: `Base64Url`, `IdempotencyKey`, `OpaqueHandle`, `TimestampRfc3339`
- shared error model: `ErrorResponse`, `ReasonCode`
- RSF request/response schemas
- PDS request/response schemas
- KTL request/response schemas

These schemas are aligned with P3-13’s `components/schemas` and are intended for:
- server input validation at the edge,
- generated SDK model types,
- fuzzing harness inputs,
- conformance tests.

## 7. Compatibility and versioning
- This registry is versioned as **P3-14 v1.0**.
- Backwards-incompatible changes require a new minor/major version and a corresponding update to the OpenAPI spec (P3-13).

## 8. Implementation requirements (normative)
Implementations consuming this registry MUST:
1. Strictly validate base64url syntax and then **decode**; enforce decoded-length where applicable.
2. Treat all `_b64` fields as opaque bytes; do not attempt to parse QSP/QSE/KT inside the generic schema layer.
3. Reject unknown JSON fields unless explicitly permitted.
4. Return standardized `ErrorResponse` bodies on non-2xx, using registry reason codes.
5. Ensure error messages are sanitized (no raw route_token, no auth tokens, no envelope bytes).

---
**End of document.**
