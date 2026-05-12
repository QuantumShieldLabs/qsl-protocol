Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0271 qsl-attachments Read-Only Audit and Test-Harness Design

Directive: QSL-DIR-2026-05-12-073 / NA-0271

## Executive Summary

NA-0271 performed a read-only audit of the local `qsl-attachments` sibling
worktree and designed the first executable hardening harness for a future
implementation lane. No `qsl-attachments` files were changed, no service
behavior was changed, and this document makes no production readiness claim.

The current attachment service already has useful security and operations
groundwork: opaque ciphertext-only storage, per-session resume tokens,
per-object fetch capabilities, local JSON metadata, single-node local-disk
startup reconciliation, disk-headroom rejects, expiry sweeps, range retrieval,
secret-redacted audit handles, and contract tests for many positive and
negative paths. The audit also found one concrete service-contract mismatch and
several evidence gaps that must remain visible before any stronger service
claim or implementation hardening lane.

## Repo Identity and Status

- Primary inspected path: `/srv/qbuild/work/NA-0237D/qsl-attachments`.
- Alternate `/home/victor/work/qsl/qsl-attachments`: absent.
- Other local sibling copies: `/srv/qbuild/work/NA-0237*/qsl-attachments`.
- Worktree status: clean before and after the read-only audit.
- Branch/status: `main...mirror/main`.
- HEAD: `1e1ae272a4cb` (`Merge pull request #31 from QuantumShieldLabs/na-0235a-qsl-attachments-macos-width-fix`).
- Remotes: `origin=https://github.com/QuantumShieldLabs/qsl-attachments.git`, `mirror=/srv/qbuild/mirrors/qsl-attachments.git`.
- Build system/language: Rust 2021 / Cargo, Axum/Tokio HTTP service.
- Audit mode: read-only; no fetch, build, test, branch creation, staging, or
  file mutation in the sibling repo.

## Service Role

`qsl-attachments` is the opaque encrypted attachment plane. It stores
ciphertext parts during upload, commits ciphertext objects, and returns
ciphertext bytes by object reference plus fetch capability. It must not parse
plaintext attachment content, must not require or store the descriptor
decrypt-context material from `DOC-CAN-007`, and must not become a qsl-server
transport relay.

Current role boundaries observed:

- The message-plane descriptor is defined in qsl-protocol `DOC-CAN-005`.
- The service-plane contract is defined in qsl-protocol `DOC-CAN-006`.
- The attachment decrypt context and part-cipher rules are defined in
  qsl-protocol `DOC-CAN-007`.
- The service consumes only ciphertext-shape and service fields:
  `attachment_id`, `ciphertext_len`, `part_size_class`, `part_count`,
  `integrity_alg`, `integrity_root`, `retention_class`, `locator_ref`, and
  resource capabilities.
- The service does not consume `plaintext_len`, `enc_ctx_alg`, `enc_ctx_b64u`,
  decoded content keys, nonce prefixes, filenames, or media types.

## Current Evidence Baseline

Observed evidence in the sibling repo:

- `README.md` states opaque encrypted attachment handling only, no plaintext
  attachment handling on service surfaces, no capability-like secrets in
  canonical URLs, and single-node local-disk posture.
- `src/lib.rs` implements create/upload/status/commit/abort/fetch lifecycle,
  disk headroom checks, expiry sweeps, range retrieval, local storage
  reconciliation, and redacted audit handles.
- `src/main.rs` defaults to loopback bind, reads `QATT_*` configuration, emits
  operator-safe startup summaries, and starts the Axum service.
- `docs/NA-0007_authn_authz_policy_subject_contract.md` freezes the current
  operator-scoped deployment policy subject and per-resource capability model.
- `docs/NA-0009_durability_recovery_contract.md` freezes the current
  single-node local-disk durability boundary and explicitly excludes hot backup,
  partial restore, multi-node storage, and cross-file transaction claims.
- `tests/service_contract.rs` contains deterministic in-process service tests
  for positive lifecycle, wrong capability, quota, disk pressure, range
  retrieval, audit redaction, expiry, and startup reconciliation.
- `tests/NA-0003_*`, `tests/NA-0004_*`, and `tests/NA-0005_*` record
  constrained-host, reference-deployment, stress, soak, and chaos evidence.

This evidence is service-hardening groundwork, not proof of production service
operation.

## API/Service Contract Inventory

Implemented routes in `src/lib.rs`:

- `POST /v1/attachments/sessions`
  - Body: `attachment_id`, `ciphertext_len`, `part_size_class`, `part_count`,
    `integrity_alg`, `integrity_root`, `retention_class`.
  - Success: `201` JSON with `session_id`, `resume_token`, shape, and
    `session_expires_at_unix_s`.

- `PUT /v1/attachments/sessions/:session_id/parts/:part_index`
  - Required header: `X-QATT-Resume-Token`.
  - Body: raw ciphertext part bytes.
  - Success: JSON with `session_state`, received part index, stored part count,
    and deterministic missing ranges.

- `GET /v1/attachments/sessions/:session_id`
  - Required header: `X-QATT-Resume-Token`.
  - Success: JSON resume/status shape with no returned secret material.

- `POST /v1/attachments/sessions/:session_id/commit`
  - Required header: `X-QATT-Resume-Token`.
  - Body: shape commitment matching the session.
  - Success: JSON with `locator_kind=service_ref_v1`, `locator_ref`,
    `fetch_capability`, object shape, and expiry.

- `DELETE /v1/attachments/sessions/:session_id`
  - Required header: `X-QATT-Resume-Token`.
  - Success: JSON with `session_state=aborted_session`.

- `GET /v1/attachments/objects/:locator_ref`
  - Required header: `X-QATT-Fetch-Capability`.
  - Optional header: single `Range: bytes=start-end`.
  - Success: raw `application/octet-stream` ciphertext bytes with content
    length and range metadata.

Implemented canonical reason codes include the `REJECT_QATTSVC_*` taxonomy from
`DOC-CAN-006`, plus `INTERNAL` for internal failures.

## Capability/Auth Model

Observed model:

- Create-session currently has no service-level Authorization gate.
- `Authorization` is documented as reserved/undefined in the current contract.
- `resume_token` is generated with `OsRng`, returned only in the create-session
  JSON response, stored only as a SHA-512 hash, and authorizes one session.
- `fetch_capability` is generated with `OsRng`, returned only in the commit
  JSON response, stored only as a SHA-512 hash, and authorizes one object.
- Required capabilities are carried in dedicated headers, not canonical URLs.
- Query-string carriage is rejected before state mutation.
- Invalid resume/fetch attempts are counted by resource reference, with bounded
  abuse rejection.
- Current tests prove wrong-session and wrong-object capability rejects and
  continued success with the correct capability.

The current model is intentionally operator-scoped plus resource capability. It
is not a multi-tenant end-user service identity model.

## Descriptor/Opaque-Ciphertext Model

The service stores opaque ciphertext bytes and service metadata only. It does
not parse the peer-visible descriptor payload from `DOC-CAN-005`, and it does
not consume the decrypt-context fields from `DOC-CAN-007`.

Observed service fields:

- Session metadata: `session_id`, `attachment_id`, ciphertext shape,
  `integrity_alg`, `integrity_root`, retention class, expiry, state,
  `resume_token_hash`, and stored part lengths.
- Object metadata: `attachment_id`, `locator_kind`, `locator_ref`,
  `fetch_capability_hash`, ciphertext shape, `integrity_alg`, `integrity_root`,
  retention class, expiry, and state.
- Stored bytes: staged `*.part` files and committed `ciphertext.bin`.

This is consistent with the opaque-ciphertext boundary: descriptor parsing,
decrypt-context validation, plaintext release, and final decrypt/integrity
decisions remain client/message-plane responsibilities.

## Fetch/Decrypt/Integrity Model

Service-side integrity checks:

- Create validates `attachment_id`, positive `ciphertext_len`, part count,
  `integrity_alg=sha512_merkle_v1`, lower-case hex `integrity_root`, and
  ciphertext length/count consistency.
- Upload validates part index and exact expected part length.
- Re-upload of an existing part succeeds only if bytes are identical.
- Commit validates body shape equals session shape, all parts are present, part
  lengths are correct, and the SHA-512 Merkle root over staged ciphertext parts
  equals the session `integrity_root`.
- Fetch returns the committed ciphertext bytes only after matching
  `fetch_capability` and unexpired object state.

Client-side or future harness responsibilities:

- Descriptor authentication and field comparison.
- `enc_ctx_b64u` decode and decrypt-context checks.
- Full fetched ciphertext re-integrity check before decrypt.
- Per-part ChaCha20-Poly1305 authentication and plaintext shape validation.
- No plaintext release or `peer_confirmed` advancement before all checks pass.

## Retention/Cleanup Model

Observed model:

- Incomplete upload sessions have `session_expires_at_unix_s`.
- Committed objects have `expires_at_unix_s` derived from `retention_class`.
- `sweep_expired` runs on request paths before mutating or fetching state.
- Expired sessions have `resume_token_hash` cleared, parts removed, and
  `state=expired_session`.
- Expired objects have `fetch_capability_hash` cleared, `ciphertext.bin`
  removed, and `object_state=expired_object`.
- Startup reconciliation removes orphan/incoherent sessions and objects, and
  preserves only coherent open sessions and committed objects.

There is no standalone background cleanup worker or operator command observed
in this audit. Cleanup is request-path and startup-reconciliation driven.

## Quota/Disk-Pressure Model

Observed model:

- Default maximum ciphertext size is 101 MiB.
- `QATT_MAX_CIPHERTEXT_BYTES` sets the deployment-global object-size ceiling.
- `QATT_MAX_OPEN_SESSIONS` sets deployment-global open-session ceiling.
- One active session or committed object per `attachment_id` is enforced.
- `QATT_STORAGE_RESERVE_BYTES` is added to staged/commit headroom checks.
- Create checks for a two-copy headroom model.
- Upload checks expected part length plus reserve.
- Commit checks committed ciphertext length plus reserve.
- Tests cover too-large create, global open-session quota, create disk pressure,
  upload disk pressure no-mutation, and commit disk pressure no-mutation.

No per-end-user quota exists in the current model, and none should be inferred.

## Logging/No-Secret Model

Observed log points:

- `src/main.rs` startup configuration summary logs bind address, storage root,
  limits, policy-surface wording, and recovery counts.
- `AuditLog::record` logs kind, short hashed handles for session/locator/
  attachment, and optional `reason_code`.
- Internal server errors log `reason_code` and message.

Observed tests:

- `audit_log_redacts_secrets_plaintext_and_full_identifiers` asserts the in-memory
  audit snapshot excludes `resume_token`, `fetch_capability`, sample ciphertext
  bytes, full `session_id`, full `attachment_id`, and full `locator_ref`, while
  preserving redacted handles.

Evidence still needed:

- capture of real tracing output across success and reject paths;
- proof for reverse proxy/access logs and retained artifacts;
- proof that malformed extractor errors and internal I/O errors cannot echo
  secret-bearing values.

## Storage/Recovery Model

Observed storage layout:

- `sessions/<session_id>/session.json`
- `sessions/<session_id>/parts/<part_index>.part`
- `objects/<locator_ref>/object.json`
- `objects/<locator_ref>/ciphertext.bin`

Observed mechanics:

- Session metadata, object metadata, and staged part writes use temp-file then
  rename helpers.
- Commit writes `ciphertext.bin`, flushes the file handle, saves `object.json`,
  then removes the session directory.
- Startup reconciliation discards invalid directory names, missing/corrupt
  metadata, incoherent staged parts, orphan staged parts, and incoherent object
  records.
- Tests cover coherent session restart, missing journaled part discard, and
  committed-object recovery requiring both `object.json` and `ciphertext.bin`.

Known boundary:

- The durability contract explicitly states no fsync/power-loss durability,
  no cross-file transaction, no hot backup, no partial restore, no multi-node
  replication, and fail-closed recovery for incoherent state.

## Deployment/Network Assumptions

Observed model:

- Default bind address is `127.0.0.1:3000`.
- `QATT_BIND_ADDR` can set the listener address.
- The service is HTTP-only; TLS/ingress is operator-managed outside the app.
- Reference docs use loopback bind behind Caddy TLS.
- Current repo CI requires only the sibling repo `rust` check.
- No in-app health, readiness, metrics, or alerting endpoint was observed.
- Operational docs require stronger observability for reference deployments,
  but this audit did not find executable in-app monitoring support.

## Existing Tests

Executable tests observed in `tests/service_contract.rs`:

- `operator_policy_surface_is_explicit_and_truthful`
- `create_session_success`
- `upload_part_success`
- `status_resume_state_visibility`
- `commit_success_after_complete_parts`
- `abort_success_and_post_abort_rejects`
- `retrieval_success_only_after_commit`
- `missing_invalid_resume_token_rejects_without_mutation`
- `missing_invalid_fetch_capability_rejects_without_mutation`
- `mismatched_part_index_and_shape_reject_without_mutation`
- `expired_session_and_object_behavior`
- `quota_limit_rejects`
- `deployment_global_open_session_quota_is_shared_across_attachments`
- `deployment_policy_allows_many_transfers_when_quota_allows_them`
- `hundred_mib_target_class_create_session_succeeds`
- `create_session_rejects_when_two_copy_disk_headroom_is_missing`
- `upload_part_disk_pressure_rejects_without_mutation`
- `commit_disk_pressure_rejects_without_mutation`
- `valid_single_range_retrieval`
- `audit_log_redacts_secrets_plaintext_and_full_identifiers`
- `repeated_invalid_fetch_capability_becomes_abuse_reject`
- `canonical_urls_reject_query_string_secret_carriage`
- `resume_token_is_scoped_to_one_session`
- `fetch_capability_is_scoped_to_one_object`
- `graceful_same_root_restart_recovers_coherent_session_and_discards_orphan_parts`
- `restart_discards_incoherent_session_when_journaled_part_is_missing`
- `committed_object_recovery_requires_object_json_and_ciphertext_bin`
- `durability_docs_and_validation_evidence_state_restart_backup_and_unsupported_cases_truthfully`

The audit did not run these tests because NA-0271 required read-only sibling
repo inspection only.

## Proven Bugs

1. Malformed JSON and extractor rejects appear to bypass the canonical
   `reason_code` taxonomy.
   - `DOC-CAN-006` says all non-successful operations MUST return a
     deterministic canonical `reason_code` in an operator-safe error body.
   - The route handlers accept `Json<CreateSessionRequest>` and
     `Json<CommitRequest>` directly in the Axum extractor position
     (`src/lib.rs` handler block around lines 958-1000).
   - Axum JSON extraction failures occur before the handler returns
     `ServiceError`, so malformed JSON, wrong JSON types, or missing required
     JSON fields are handled by Axum's default rejection surface rather than
     the service's `REJECT_QATTSVC_SESSION_SHAPE` or
     `REJECT_QATTSVC_COMMIT_MISMATCH` body.
   - The audit did not find an executable malformed-JSON test in
     `tests/service_contract.rs`.
   - Severity: medium service-contract mismatch. A future lane should add the
     failing test first, then map extractor failures into the canonical reject
     taxonomy without weakening fail-closed behavior.

## Evidence Gaps

- The sibling repo tests were not executed in this lane.
- No executable malformed JSON/body-shape test proves canonical `reason_code`
  behavior.
- No executable test covers malformed or non-ASCII capability headers.
- No executable test covers repeated invalid resume-token abuse escalation.
- No executable test covers repeated invalid range abuse escalation.
- No executable test proves wrong capability for abort and commit specifically.
- No executable descriptor parser test exists in `qsl-attachments`; descriptor
  validation is correctly client/message-plane owned, but the first integrated
  harness still needs to prove descriptor and service fields cannot drift.
- No executable proof shows fetched ciphertext is re-integrity-checked and
  decrypted only by the client harness before plaintext release.
- No real tracing-output capture proves every success and reject path is free
  of capabilities, descriptors, ciphertext bytes, plaintext, or full stable
  identifiers.
- No reverse-proxy/access-log proof is present in the sibling repo tests.
- No startup binary/config test proves invalid root path, invalid bind address,
  invalid quota, invalid TTL, or impossible retention configuration produces a
  structured operator-safe failure instead of an uncontrolled panic.
- No power-loss/fsync proof exists, consistent with the durability contract.
- No hot-backup, partial-restore, multi-node, external object-store, or
  replicated-storage proof exists.
- No in-app health, readiness, metrics, or alerting endpoint exists.
- No long-running executable soak in this test suite covers cleanup during
  load, repeated expiry sweeps, or bounded burst behavior after restart.
- No external review completion evidence exists.

## Recommendations

- Build the first executable qsl-attachments hardening harness before any
  stronger service claim.
- Start the harness with malformed JSON/body-shape rejects because the current
  code evidence indicates a concrete taxonomy mismatch.
- Add log-capture tests using `tracing` subscriber capture, not only
  `AuditLog::snapshot`.
- Add startup/config tests around `Config::from_env`, invalid bind/root/limit
  values, and binary startup error shape.
- Add integrated client-harness tests that fetch opaque ciphertext, verify
  `integrity_root`, decrypt locally, and prove no plaintext release on tamper.
- Keep resource-capability auth explicit until a separate operator-scoped
  `Authorization` layer is designed and authorized.
- Add request-path abuse tests for resume tokens, fetch capabilities, and range
  headers.
- Add cleanup/restart tests that combine expiry, orphan state, and load.
- Add a minimal ops probe story: either an in-app health/readiness endpoint or
  a documented external probe harness that does not expose secrets.
- Keep production-service wording blocked until auth, logging, cleanup,
  recovery, deployment, observability, dependency, and external review gates
  have executable proof.

## Non-Issues

- The absence of service-visible `enc_ctx_alg`, `enc_ctx_b64u`, decoded keys,
  or nonce prefixes is correct; those belong to `DOC-CAN-007` and client-side
  decrypt handling, not the attachment service.
- `Authorization` being reserved/undefined is not a bug in the current
  contract; it is a documented limitation and future auth layer boundary.
- `resume_token` and `fetch_capability` are not logged by the in-memory audit
  snapshot test.
- Wrong-session resume token and wrong-object fetch capability are covered by
  executable tests.
- Identical part re-upload idempotence is implemented; mismatched replay
  rejects before overwriting the staged part.
- Expiry is request-path and startup-reconciliation driven; absence of a
  background worker is a documented operational limitation, not hidden proof.
- Startup panics from `expect(...)` are startup/config/bind/serve paths, not
  request-path panics. They still need structured failure tests before stronger
  operations claims.

## Recommended Test Harness

The first harness should live in the `qsl-attachments` sibling repo and should
be executable against the Axum router and, for selected cases, a spawned
loopback service with captured tracing output. It should assert HTTP behavior,
canonical error bodies, storage state before/after rejects, log hygiene, and
client-owned decrypt/integrity boundaries.

### Capability/Auth Tests

- Missing `X-QATT-Resume-Token` rejects with
  `REJECT_QATTSVC_RESUME_TOKEN` and no state mutation.
- Wrong `X-QATT-Resume-Token` rejects and cannot mutate a different session.
- Wrong resource: a valid resume token for session A rejects on session B.
- Replayed capability: already aborted/expired session resume token rejects.
- Missing `X-QATT-Fetch-Capability` rejects with
  `REJECT_QATTSVC_FETCH_CAPABILITY`.
- Wrong `X-QATT-Fetch-Capability` rejects and cannot mutate object state.
- Wrong resource: a valid fetch capability for object A rejects on object B.
- Repeated invalid resume/fetch attempts escalate to the abuse reject at the
  configured bound.
- Capability values are absent from captured application logs and retained test
  artifacts.

### Descriptor Tests

- Malformed descriptor rejects before any service fetch in the integrated
  client harness.
- Tampered descriptor field rejects before fetch/decrypt.
- Wrong resource descriptor, such as mismatched `attachment_id`,
  `locator_ref`, or `integrity_root`, rejects before plaintext release.
- Descriptor metadata boundary: `filename_hint` and `media_type` remain
  peer-visible/client-side only and never reach qsl-attachments requests, logs,
  or storage.

### Opaque Ciphertext Tests

- Store arbitrary opaque bytes without plaintext parsing.
- Fetch returns the exact committed opaque bytes and preserves byte order.
- The service does not decrypt, inspect, or log plaintext.
- The service does not require or store `enc_ctx_*`, content keys, or nonce
  prefixes.
- Tampered ciphertext is handled by the client/integrity layer and cannot be
  silently promoted to plaintext completion.

### Fetch/Decrypt/Integrity Tests

- Valid fetch/decrypt path through the client harness when descriptor,
  integrity, and decrypt context are correct.
- Tamper reject: modified fetched bytes fail integrity/decrypt before plaintext
  release.
- Missing object returns `REJECT_QATTSVC_LOCATOR_UNKNOWN`.
- Duplicate fetch with the same valid capability has explicitly chosen
  semantics and no hidden state mutation.
- Unauthorized fetch never returns bytes.
- Range fetch returns exact byte span and content-range metadata.
- Invalid and repeated invalid range headers reject deterministically and
  escalate to abuse when configured.

### Retention/Cleanup Tests

- Incomplete-session expiry clears resume capability and staged bytes.
- Committed-object expiry clears fetch capability and removes committed bytes.
- Abort/delete clears staged bytes and invalidates resume token.
- Cleanup leaves no stale object bytes retrievable after expiry.
- Cleanup during concurrent load remains bounded and does not panic.
- Startup reconciliation does not re-expose expired or incoherent artifacts.

### Quota/Disk Tests

- Max ciphertext size reject returns `REJECT_QATTSVC_QUOTA`.
- Deployment-global open-session quota reject is shared across attachments.
- Per-resource duplicate active `attachment_id` reject remains deterministic.
- Disk-full simulation on create/upload/commit rejects without state mutation.
- Cleanup frees expected staged/object space in controlled fixtures.
- Bounded upload/fetch bursts do not create unbounded growth.

### Logging Tests

- `resume_token` absent from success and reject logs.
- `fetch_capability` absent from success and reject logs.
- Descriptor secrets, especially `enc_ctx_b64u`, absent from service logs.
- Ciphertext bytes and plaintext bytes absent from service logs.
- Full `session_id`, `locator_ref`, and `attachment_id` are replaced with
  short handles where passive logs need correlation.
- Error output is sanitized and contains only canonical reason codes plus
  operator-safe messages.

### Restart/Recovery Tests

- Coherent open-session journal replay preserves resumable state.
- Partial/corrupt session journal fails closed and removes incoherent state.
- Missing staged file fails closed for that session.
- Committed object survives restart only with both `object.json` and
  `ciphertext.bin`.
- Missing object bytes or metadata rejects after restart.
- Restart after expiry sweep preserves expired-state non-retrievability.
- Recovery summary counts remain operator-safe and contain no secrets.

### Config/Startup Tests

- Missing env uses loopback and bounded defaults.
- Invalid bind address fails closed with operator-safe error.
- Invalid storage root fails closed with operator-safe error.
- Invalid quota, retention, and TTL values fail closed.
- Zero or overflow-prone limits are rejected or documented with executable
  behavior.
- Safe bind config keeps loopback default unless explicitly changed.

### Health/Ops Tests

- If a health/readiness endpoint is added, it must expose only non-secret
  readiness state and must not authorize data operations.
- If no endpoint is added, an explicit external probe harness should be
  documented and tested.
- Metrics/observability absence remains an explicit gap until implemented.
- Deployment/proxy log capture must prove no capability or plaintext leakage.

### Soak/Stress Tests

- Bounded upload/fetch bursts across multiple sessions and objects.
- Cleanup/expiry during load.
- Restart during bounded upload/fetch sequences.
- Repeated malformed request bursts.
- No panic, no unbounded growth, no false successful retrieval, and no
  plaintext or capability leakage in retained artifacts.

## Proposed Future Implementation Lanes

1. qsl-attachments API reject-taxonomy harness and malformed JSON repair.
2. qsl-attachments capability/auth abuse and no-mutation harness.
3. qsl-attachments log/no-secret capture harness.
4. qsl-attachments opaque ciphertext fetch/decrypt/integrity client harness.
5. qsl-attachments retention, cleanup, and restart/recovery hardening harness.
6. qsl-attachments startup/config and ops probe hardening.
7. qsl-server docs/API contract repair and harness prep, if the queue promotes
   that qsl-server follow-up first.
8. External review package refresh only after executable service evidence
   exists.

## Non-Production / No Production-Readiness Boundary

NA-0271 is a read-only audit and design lane. It does not:

- implement qsl-attachments changes;
- implement qsl-server changes;
- change protocol, wire, crypto, auth, or state-machine semantics;
- change qsp protocol-core, qsc/qsl runtime, qsc-desktop, website, workflows,
  scripts, Cargo files, dependencies, branch protection, or public-safety;
- authorize deployment or public internet exposure;
- claim production service operation;
- claim external review completion; or
- hide known capability, logging, cleanup, recovery, deployment, or
  observability gaps.

Current demo evidence remains non-production. The opaque-ciphertext boundary
remains explicit: qsl-attachments stores and returns ciphertext bytes; clients
own descriptor authentication, decrypt-context handling, integrity verification,
decryption, plaintext release, and final delivery confirmation.

## Stop Conditions for Future Implementation

Future qsl-attachments implementation lanes must stop if:

- the change would weaken fail-closed behavior;
- a reject path mutates protected state without explicit contract authority;
- service logs or retained artifacts expose capabilities, plaintext, decrypt
  context, ciphertext bytes, route tokens, or secret-bearing URL material;
- qsl-attachments starts parsing plaintext attachment content or message-plane
  transcript semantics;
- descriptor/decrypt-context boundaries are blurred or moved into service
  storage/logging;
- known gaps are hidden or recast as completed proof;
- production service wording is introduced before executable evidence and
  review exist;
- protocol, wire, crypto, auth, or state-machine semantics would change outside
  the declared lane;
- qsl-server, qsc/qsl runtime, qsc-desktop, website, workflows, scripts, Cargo
  files, branch protection, public-safety, dependencies, or deployment settings
  would need to change without a separate directive; or
- required local validation or CI fails without a bounded, truthful recovery.
