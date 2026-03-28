Goals: G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-03-28

# DOC-G5-004 — Metadata Leakage Surface Review and Logging Contract v0.1.0 DRAFT

## 1. Purpose and result

This document records `NA-0211`.

It freezes:
- the current evidence-backed inventory of unavoidable versus avoidable metadata leakage,
- the project-wide logging and secret-hygiene contract for the current attachment-plane era, and
- the truthful next blocker implied by the current repo state.

Result:
- `MLR0` is chosen.
- Closeout path `AR1` is truthful.
- The next blocker is direct metadata / secret-hygiene enforcement, not another review/finalization item and not a prerequisite `qsl-attachments` authn/authz policy-subject lane.

## 2. Authoritative inputs reviewed

This review is grounded by the current merged state of:
- qsl-protocol `NEXT_ACTIONS.md`, `TRACEABILITY.md`, and `DECISIONS.md`, especially `NA-0199` through `NA-0209A`
- `docs/canonical/DOC-CAN-005_*`, `docs/canonical/DOC-CAN-006_*`, and `docs/canonical/DOC-CAN-007_*`
- `docs/design/DOC-ATT-002_*` through `docs/design/DOC-ATT-009_*`
- `docs/privacy/DOC-G5-001_*`, `DOC-G5-002_*`, and `DOC-G5-003_*` as the older demo-relay baseline
- qsc operator/docs/tests:
  - `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
  - `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
  - `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
  - `qsl/qsl-client/qsc/tests/tui_relay_config.rs`
  - `qsl/qsl-client/qsc/tests/lifecycle.rs`
  - `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
  - `qsl/qsl-client/qsc/tests/timeline_store.rs`
- qsl-attachments current repo state:
  - `README.md`, `TRACEABILITY.md`, `DECISIONS.md`
  - `docs/NA-0002_operational_hardening_contract.md`
  - `docs/NA-0004_reference_deployment_runbook.md`
  - `src/lib.rs`
  - `tests/service_contract.rs`
  - `tests/NA-0005_stress_soak_chaos_evidence.md`
- qsl-server current repo state:
  - `README.md`, `TRACEABILITY.md`, `DECISIONS.md`
  - `docs/server/DOC-SRV-004_*`
  - `docs/server/DOC-SRV-005_*`
  - `scripts/check_relay_compatibility.sh`
  - `scripts/verify_remote.sh`
  - `tests/NA-0011_relay_compatibility_restore_evidence.md`

## 3. Current leakage inventory

### 3.1 Message transport metadata

| Surface | Actual current exposure | Classification | Current proof | Contract consequence |
| --- | --- | --- | --- | --- |
| qsl-server / network observer on canonical relay path | request timing, envelope size, route-token presence, auth success/failure, queue/backpressure state | unavoidable | qsl-server `README.md`; `docs/server/DOC-SRV-005_*`; qsc `tests/route_header_migration_docs_na0195a.rs` | keep route tokens out of URLs and docs; keep relay transport opaque; normalize examples around header carriage only |
| Authenticated attachment descriptor seen by the intended peer | `attachment_id`, `plaintext_len`, `ciphertext_len`, `part_size_class`, `part_count`, `integrity_root`, `locator_ref`, `fetch_capability`, `enc_ctx_b64u`, retention/expiry, confirm fields, optional `filename_hint` / `media_type` | mixed: core descriptor fields are unavoidable to the intended peer; `filename_hint` / `media_type` are avoidable opt-in metadata; any further logging/evidence leakage is avoidable | `DOC-CAN-005`; `DOC-CAN-007` | descriptor is the only allowed peer-visible carriage for retrieval/decrypt context; optional hints stay opt-in and must never be echoed passively |
| Residual legacy `file_chunk` / `file_manifest` path | filename, total size, chunk count, manifest/chunk hashes, confirm id when used | avoidable compatibility residue, not the validated post-`w0` baseline | qsc `src/store/mod.rs`; `docs/design/DOC-ATT-008_*`; `docs/design/DOC-ATT-009_*`; `tests/attachment_streaming_na0197c.rs` | no future lane may widen this surface; retired-mode reject markers remain explicit and fail-closed |

### 3.2 Attachment-plane metadata

| Surface | Actual current exposure | Classification | Current proof | Contract consequence |
| --- | --- | --- | --- | --- |
| qsl-attachments HTTP surface | canonical non-secret path elements `session_id`, `part_index`, `locator_ref`; request timing; request sizes; range usage; secret-bearing `resume_token` / `fetch_capability` only in headers/body | mixed: service references and coarse request shape are unavoidable service metadata; secret leakage into URLs/logs is avoidable and forbidden | `DOC-CAN-006`; qsl-attachments `tests/service_contract.rs` | keep canonical URLs secret-free; capabilities remain header/body only |
| qsl-attachments persisted session/object journals | `attachment_id`, `session_id`, `locator_ref`, lengths, integrity, retention, expiry, stored-part map; hashed capability material only | unavoidable local service metadata; raw capability persistence is avoidable and already forbidden | qsl-attachments `src/lib.rs` (`SessionMeta`, `ObjectMeta`, capability hashes) | service journals may keep only non-secret refs plus hashed capability material |
| qsc local attachment journal | `attachment_id`, peer label, direction, `service_url`, state, shape/integrity fields, `enc_ctx_b64u`, `locator_ref`, `fetch_capability`, `resume_token`, source/output paths, uploaded parts, timeline ids, last error | unavoidable for current resume/fetch truthfulness, but highly sensitive local metadata | qsc `src/store/mod.rs`; qsc `attachment_journal_load/save` via vault in `src/main.rs` | keep vault-scoped only; never echo in passive logs, markers, docs, or evidence |

### 3.3 Operator-visible logs, markers, and evidence artifacts

| Surface | Actual current exposure | Classification | Current proof | Contract consequence |
| --- | --- | --- | --- | --- |
| qsc CLI/TUI markers and runbooks | truthful delivery states (`accepted_by_relay`, `awaiting_confirmation`, `peer_confirmed`), policy stage markers, retired-mode reject codes, redacted relay endpoint/token-file state, short device-routing markers | truthful state markers are unavoidable for current operator honesty; any raw token, path, URL, or long identifier leak is avoidable | qsc `tests/route_header_migration_docs_na0195a.rs`; `tests/tui_relay_config.rs`; `tests/lifecycle.rs`; `LOCAL_TWO_CLIENT_RUNBOOK.md` | keep state/reason/policy markers; forbid passive output of raw capabilities, long hex ids, token-file paths, or secret-bearing URLs |
| qsl-attachments audit log | `kind`, `session_id`, `locator_ref`, `attachment_id`, `reason_code`; no raw capabilities | full stable identifiers in passive logs are avoidable metadata leakage | qsl-attachments `src/lib.rs` `AuditLog::record`; qsl-attachments evidence scans in `tests/NA-0005_stress_soak_chaos_evidence.md` | enforcement must minimize passive log linkage; default logs should not emit full `attachment_id`, `session_id`, or `locator_ref` |
| qsl-server compatibility / verify evidence | canonical-vs-legacy compatibility result codes, loopback/public base, probe channel, push/pull sanity over probe-only values | unavoidable operator evidence; secret leakage is avoidable and already guarded | qsl-server `scripts/check_relay_compatibility.sh`; `scripts/verify_remote.sh`; `tests/NA-0011_relay_compatibility_restore_evidence.md` | keep probe-only channels and header-based examples; never paste real route tokens |
| qbuild / governance evidence prose | SHAs, check results, file lists, leak-safe scan counts, path-pattern summaries | unavoidable governance evidence | repo `AGENTS.md` guidance | continue short-SHA and descriptive evidence wording; no literal secret/path/token dumps |

### 3.4 Local persistence and journals

| Surface | Actual current exposure | Classification | Current proof | Contract consequence |
| --- | --- | --- | --- | --- |
| qsc contacts store | peer labels, fingerprints, trust state, route tokens, device ids | unavoidable local metadata for current routing/trust behavior | qsc `src/store/mod.rs`; qsc `contacts_store_load/save`; qsc tests across trust/handshake flows | vault-only persistence; route tokens never appear in passive output |
| qsc timeline store | peer label, direction, byte length, kind, state, timestamp, local ids | unavoidable local operator history metadata | qsc `src/store/mod.rs`; qsc `timeline_store_load/save`; qsc `tests/timeline_store.rs` | vault-only persistence; explicit timeline commands may reveal it, passive logs may not |
| qsc session / identity material | encrypted session blobs and vaulted identity secrets; no plaintext secret markers on disk | unavoidable local state, but raw secret leakage is avoidable and already rejected | qsc `tests/identity_secret_at_rest.rs`; `tests/session_state_at_rest.rs` | keep encrypted/vaulted only; no plaintext session or identity secret storage |

### 3.5 Docs and posture statements

| Surface | Actual current exposure | Classification | Current proof | Contract consequence |
| --- | --- | --- | --- | --- |
| Current runbooks and route-token docs | canonical header-based examples, placeholders, redacted relay state, explicit secret-safe wording | acceptable and intentional | qsc `LOCAL_TWO_CLIENT_RUNBOOK.md`; qsc `tests/route_header_migration_docs_na0195a.rs`; qsl-server `README.md` | keep placeholder/header discipline |
| Older demo privacy docs | accurate for the older demo relay baseline, but they do not fully inventory current attachment-plane journals, qsl-attachments audit surfaces, or the post-`w0` attachment descriptor era | avoidable documentation drift if treated as exhaustive current policy | `docs/privacy/DOC-G5-001_*`; `DOC-G5-002_*`; `DOC-G5-003_*` | this document becomes the current authoritative metadata/logging contract for the attachment-plane era |

### 3.6 Ambiguous surfaces

The review found one real future boundary but not a current policy blocker:
- qsl-attachments repo-local authn/authz and policy-subject semantics remain undefined by current v1 service docs because `DOC-CAN-006` intentionally reserves `Authorization` for a future repo-local layer.

Current judgment:
- this is a future product/policy lane, not a prerequisite for freezing the current metadata/logging contract;
- the current v1 surface is still explicit enough to enforce secret-hygiene and log-minimization now; and
- the authn/authz lane would add new operator-visible identities later, but it does not excuse current token/correlation leakage today.

## 4. Frozen logging and secret-hygiene contract

The following rules are now non-negotiable.

### 4.1 Secret-bearing values never appear outside their canonical carriage surface

The following values are secret-bearing and must never appear in passive logs, markers, docs, copied commands, screenshots, evidence bundles, canonical URLs, or query strings:
- route tokens
- relay bearer tokens
- `resume_token`
- `fetch_capability`
- `enc_ctx_b64u`
- decoded attachment keys or nonce prefixes
- vault passphrases and token-file contents

They may appear only in the canonical protocol/application carriage already frozen by current docs:
- route tokens in `X-QSL-Route-Token`
- relay auth in `Authorization: Bearer ...`
- qsl-attachments capabilities in `X-QATT-Resume-Token`, `X-QATT-Fetch-Capability`, or the designated JSON response field
- attachment decrypt context only in the authenticated message-plane descriptor

### 4.2 Canonical URLs remain secret-free

Canonical URLs, path segments, and query strings must never carry:
- route tokens
- `resume_token`
- `fetch_capability`
- `enc_ctx_*`
- plaintext filenames or media hints
- any future capability-like secret

Header/body-only secret carriage remains mandatory for both relay and attachment-service surfaces.

### 4.3 Passive logs and evidence must be correlation-minimized by default

Default passive logs, metrics, traces, and evidence may include only what is needed for truthful operator state:
- event kind
- coarse state/result
- deterministic reason code
- bounded counters and coarse policy/status fields

They must not emit:
- full route tokens or bearer tokens
- full `attachment_id`, `session_id`, or `locator_ref`
- attachment decrypt context
- token-file paths
- local source/output paths
- full service URLs when redacted state is sufficient
- long hex dumps or copied protocol payloads

When correlation is required, enforcement work must prefer short, non-secret, operator-scoped handles over full stable identifiers.

### 4.4 Local state may persist only where needed for truthful behavior

Current local persistence is permitted only in these shapes:
- qsl-attachments session/object journals with non-secret refs plus hashed capability material
- qsc vault-scoped contacts, timeline, relay config, and attachment journals
- encrypted qsc session blobs

Local state needed for resume, replay protection, truthful delivery state, or operator history is allowed.
What is not allowed:
- plaintext secret storage
- passive re-echo of vaulted/journal secrets
- copying vault/journal internals into docs or evidence

### 4.5 Truthful operator markers remain allowed

The project may keep explicit operator-visible markers for truthful semantics, including:
- `accepted_by_relay`
- `awaiting_confirmation`
- `peer_confirmed`
- explicit retired-mode reject codes
- compatibility-guard result codes
- coarse policy stage / routing markers that stay secret-safe

Those markers must stay deterministic and secret-safe.

### 4.6 Docs, runbooks, and evidence use placeholder and redaction discipline

Docs, runbooks, screenshots, and evidence must:
- use placeholders for secret values
- use header-based route-token examples only
- avoid token-bearing URL examples
- avoid literal long-hex dumps
- use short SHAs in narrative evidence unless tooling requires more
- use descriptive leak-scan wording instead of pasting sensitive path/token fragments

### 4.7 The current contract is implementation-ready

More review evidence is not required before enforcement begins.
The remaining work is runtime/test/docs/evidence enforcement against this contract.

## 5. Logging-contract option set

| Option | Summary | Evidence sufficiency | Result |
| --- | --- | --- | --- |
| `LC0` | Direct metadata / secret-hygiene enforcement is now the next blocker | Sufficient: canonical docs already freeze URL/header/context rules; qsc tests already prove output redaction and vault-scoped local state; qsl-attachments proof already shows hashed capability persistence plus no raw token leakage; qsl-server guard already freezes canonical header carriage | chosen |
| `LC1` | qsl-attachments authn/authz / policy-subject maturity is the more load-bearing blocker | Insufficient: current v1 capability-based service semantics already define the present metadata surface strongly enough to enforce logging and secret-hygiene now | rejected |
| `LC2` | One smaller metadata-review finalization item still blocks a truthful next lane | Insufficient: this document closes the remaining review gap without semantic invention; another docs-only round would duplicate evidence instead of moving enforcement | rejected |

Why `LC0` wins:
- the real unresolved work is enforcement against current surfaces, especially passive-log minimization and cross-surface secret-safe evidence discipline;
- the review did find avoidable current leakage targets, but they are enforcement targets rather than proof gaps; and
- the current service contract does not need a prior authn/authz redesign before the project can stop leaking capability-adjacent material and stable service identifiers.

## 6. Decision

Chosen result:
- `MLR0`

Exact reason:
- current repo state is already decision-grade and unambiguous enough to separate unavoidable transport/local-state metadata from avoidable logging/evidence leakage;
- the remaining gap is direct enforcement of this contract on qsc, qsl-attachments, qsl-server-adjacent evidence scripts, and operator-facing documentation/evidence surfaces; and
- no truthful part of that next step requires inventing new protocol, relay, or attachment-service semantics first.

Exact remaining blocker:
- enforce this contract across runtime behavior, tests, logs, journals, docs, and evidence generation surfaces.

Smallest truthful successor lane:
- `NA-0211A — Metadata / Secret-Hygiene Enforcement`

## 7. References

- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-003_Default_Attachment_Path_Promotion_and_Legacy_In_Message_Policy_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-004_Legacy_In_Message_Deprecation_Readiness_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-005_Legacy_In_Message_Final_Removal_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-006_Legacy_Receive_Compatibility_Retirement_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-007_Legacy_Receive_Compatibility_Retirement_Gate_Finalization_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-008_Post_W0_Receive_Compatibility_Boundary_Decision_v0.1.0_DRAFT.md`
- `docs/design/DOC-ATT-009_Post_W0_Activation_and_Legacy_Mode_Retirement_Decision_v0.1.0_DRAFT.md`
- `docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md`
- `docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md`
- `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`
- `qsl/qsl-client/qsc/LOCAL_TWO_CLIENT_RUNBOOK.md`
- `qsl/qsl-client/qsc/tests/route_header_migration_docs_na0195a.rs`
- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
- `qsl/qsl-client/qsc/tests/tui_relay_config.rs`
- `qsl/qsl-client/qsc/tests/lifecycle.rs`
- `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/timeline_store.rs`
- qsl-attachments `README.md`, `TRACEABILITY.md`, `DECISIONS.md`, `docs/NA-0002_operational_hardening_contract.md`, `docs/NA-0004_reference_deployment_runbook.md`, `src/lib.rs`, `tests/service_contract.rs`, `tests/NA-0005_stress_soak_chaos_evidence.md`
- qsl-server `README.md`, `TRACEABILITY.md`, `DECISIONS.md`, `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`, `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`, `scripts/check_relay_compatibility.sh`, `scripts/verify_remote.sh`, `tests/NA-0011_relay_compatibility_restore_evidence.md`
