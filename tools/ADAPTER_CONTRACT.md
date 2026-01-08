# Phase 4 Canonical Adapter Contract (Phase 3 → Phase 4)

**Purpose:** Normalize Phase 3 artifacts into **Phase 4 canonical** formats so CI can validate and execute conformance artifacts deterministically and fail-closed.

**Canonical outputs (only):**
- Vector Set: `QSHIELD-P4-VECTOR-SET-1` (see `schemas/qshield.phase4.vector_set.schema.v1.json`)
- Interop Set: `QSHIELD-P4-INTEROP-SET-1` (see `schemas/qshield.phase4.interop_set.schema.v1.json`)

---

## 1. Hard requirements
Adapters MUST:
1) Be **deterministic** (same input bytes → same canonical output bytes under stable JSON serialization).
2) Be **fail-closed** on ambiguity (missing required fields; unknown kind; ambiguous semantics).
3) Be **lossless where feasible** by preserving unmapped fields under `ext` (audit traceability).
4) Populate `protocol.protocol_version` and `protocol.suite_id` when present; otherwise use project defaults:
   - `protocol_version`: `0x0403`
   - `suite_id`: `0x0002` (Suite-1B)

---

## 2. Canonical JSON serialization
To ensure deterministic outputs, adapters SHOULD:
- Sort object keys lexicographically.
- Avoid floating-point values.
- Emit arrays in source order.

---

## 3. P3-23 Negative Vectors → P4 Vector Set
### 3.1 Accepted inputs
- Top-level object with `cases` array.

### 3.2 Required mapping
For each P3 case `c`:
- `vector.id`        ← `c.id`
- `vector.op`        ← `c.op`
- `vector.input`     ← map `c.input` into `input` (typed data objects). If `c.input` is not already typed, wrap it under a single field `data`.
- `vector.expect`    ← `c.expect` (`ok` required; `reason_code` required when `ok=false`)
- `vector.maps_to`   ← `c.maps_to` (optional)
- `vector.notes`     ← `c.notes` (optional)
- `vector.ext.p3`    ← full original case (recommended)

Populate:
- `source.member` / `source.artifact_id` / `source.format` / `source.version` from the P3 top-level metadata when available.

### 3.3 Fail-closed conditions
Adapter MUST error if:
- `cases` is missing or not an array.
- Any case lacks `id`, `op`, `expect.ok`.
- Any rejecting case lacks `expect.reason_code`.

---

## 4. P3-08 Vectors → P4 Vector Set
### 4.1 Accepted inputs
- Top-level object with `cases` array and a `format`/`version` marker.

### 4.2 Kind-to-op mapping (strict)
Adapter MUST use a **closed** mapping. Unknown kinds MUST error.

Recommended initial mapping:
- `qse_envelope_roundtrip` → `qse_parse`
- `qse_trailing_bytes`     → `qse_parse`
- `qse_bad_lengths`        → `qse_parse`

### 4.3 Required mapping
- `vector.id` ← `case.id`
- `vector.op` ← mapped op
- `vector.input.data` ← `{ "type": "b64u", "data": case.input_qse_envelope_b64, "semantic": "qse_envelope" }`
- `vector.expect.ok` ← `case.expected == "accept"`
- If reject: `vector.expect.reason_code` ← `case.expected_error_code` (required)
- Preserve original `kind` under `vector.ext.p3_kind`.

---

## 5. P3-04 Interop → P4 Interop Set (forward-compatible)
### 5.1 Accepted inputs
Adapters MUST accept both common shapes:
- Variant A: top-level object with `cases` array.
- Variant B: top-level array of cases.

### 5.2 Required mapping
- If input is array: wrap into a P4 Interop Set object.
- For each case, create:
  - `case.id`: prefer source `id`; else derive deterministically from title + ordered steps.
  - `case.participants`: map source actor names into stable IDs (e.g., `A`, `B`, `PDS`).
  - `case.steps`: preserve original step objects under `step.ext.p3_step` and promote any wire payloads into typed fields.

### 5.3 Fail-closed conditions
Adapter MUST error if:
- Steps cannot be ordered deterministically.
- Actor identities are inconsistent (e.g., same name refers to different roles) without an explicit mapping.

---

## 6. Transitional harness compatibility
If the current harness only accepts a raw list:
- Adapters MAY emit a secondary file view (`vectors_only.json` / `cases_only.json`) containing the bare list,
- BUT CI MUST validate and archive the canonical wrapper object as the source of truth.
