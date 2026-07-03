Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-03

# NA-0595 local qsl-attachments adversarial and metadata stress harness

## Executive Summary

NA-0595 consumed D-1179 and D-1180, verified fresh qwork proof from
`2026-07-03T05:46:41Z`, revalidated the local qsc/qsl-server/qsl-attachments
surfaces, validated qsl-server, qsl-attachments, and focused qsc tests, and ran
a proof-root-only adversarial stress harness over the local attachment path.

Result classification:
`LOCAL_QSL_ATTACHMENTS_STRESS_RESOURCE_BOUNDARY_GAP`.

Baseline, ten repeated above-threshold transfers, multi-message/multi-size
stress, selected capability/auth negatives, descriptor/object corruption
negatives, route/relay negatives, qsl-attachments restart persistence, rapid
operations, metadata review, cleanup, and private-material scans completed. The
exact 4 MiB legacy boundary probe timed out inside the bounded local stress
window and is classified as a resource-boundary diagnostic gap, not a
fail-open. The selected successor is
`NA-0596 -- QSL Local Attachment Stress Diagnostic Follow-Up Harness`.

## qwork Proof Verification

- qwork proof files were copied before fetch, local service acquisition, builds,
  qsc execution, repository mutation, or proof publication.
- qwork startup result: OK.
- lane: NA-0595.
- repo/path: qsl-protocol at `/srv/qbuild/work/NA-0595/qsl-protocol`.
- proof timestamp: `2026-07-03T05:46:41Z`.
- startup `HEAD`, `origin/main`, and `main`: `d992e12ab731`.
- startup worktree, index, and untracked state: clean.
- READY_COUNT: 1.
- READY item: NA-0595.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.
- Disk and mount gates passed; `/backup/qsl` was mounted and root usage stayed
  below the stop threshold.

## D-1179 / D-1180 Inheritance

- D-1179 exists once and is Accepted.
- D-1180 exists once and is Accepted.
- NA-0594 is DONE.
- NA-0595 is READY.
- D-1181 and D-1182 were absent before this patch.
- D-1179 result
  `LOCAL_QSL_ATTACHMENTS_SEND_RECEIVE_INTEGRATION_PASS_WITH_METADATA_LIMITS`
  was consumed.
- D-1179 selected NA-0595 and D-1180 restored NA-0595.
- qsc encryption/decryption ownership, qsl-server relay/control-plane boundary,
  qsl-attachments opaque storage boundary, seed-fallback blocking, and NA-0594
  metadata limits were inherited.

## Authority Model Application

NA-0595 used Tier 0 read-only source review, Tier 1 proof-root harness tooling,
Tier 2 local diagnostic/test validation, and Tier 4 local build-server runtime
actions. No qsc, qsl-server, or qsl-attachments source fix was applied. No
dependency, lockfile, workflow, deployment, remote, Tailscale, public-site,
Cloudflare, sudo, systemd, firewall, qwork, qstart, or qresume action occurred.

## qsc Attachment Stress Surface Review

- qsc attachment send/receive, descriptor construction, fetch verification,
  local decrypt/validate, failure diagnostics, and threshold behavior were
  revalidated.
- Above-threshold files use qsl-attachments when configured.
- Just-over-4 MiB transfer classes passed through qsl-attachments.
- Exact 4 MiB legacy boundary behavior was probed and classified as a bounded
  resource-boundary gap in this local stress lane.
- Seed fallback env-only behavior remains blocked in the attachment path.

## qsl-server Relay Boundary Review

- qsl-server was validated as local loopback relay/control-plane only.
- qsl-server relays opaque qsc envelopes and route-scoped queues.
- qsl-server does not store attachment object bytes and did not see plaintext
  fixture markers in runtime scans.
- Wrong/missing route-token direct probes failed closed or returned no-item
  class as expected.

## qsl-attachments Runtime Surface Review

- qsl-attachments local startup, proof-root storage root, session upload,
  commit, fetch, capability checks, unknown-object behavior, short retention,
  restart persistence, and storage/log behavior were revalidated.
- qsl-attachments stored opaque object bytes only and did not see qsc plaintext
  fixture markers or qsc key material in runtime scans.

## Local Build / Audit / Test Readiness

- qsl-server metadata, audit, fmt, tests, and build passed at `6bf61d439fa2`.
- qsl-attachments metadata, audit, fmt, tests, and build passed at
  `767eca189ee`, and the PR #38 recovery commit was present.
- qsl-protocol root metadata, root cargo audit, nested qsc fuzz cargo audit,
  cargo fmt, and Cargo drift checks passed before the stress run.
- Focused qsc validation passed for:
  `na_0591_true_triple_ratchet_path`,
  `na_0593_seed_fallback_hardening`,
  `attachments_contract_na0217h`,
  `attachment_streaming_na0197c`,
  `qsp_protocol_gate`,
  `relay_auth_header`, and
  `receive_e2e receive_mailbox_peer_separation_fail_closed`.

## Stress Harness Design

- The harness used proof-root-only qsc state, qsl-server logs, qsl-attachments
  storage/logs, and synthetic non-secret fixture classes.
- qsl-server and qsl-attachments were started on loopback/local proof-root
  storage only.
- Raw local endpoints, private ports, route tokens, Authorization material,
  capabilities, command lines, payloads, plaintext, ciphertext bodies, key
  material, process identities, and storage paths remain proof-root-only.
- Static review found no public bind, secret env dependency, deletion outside
  proof root, qwork/qstart/qresume call, sudo/systemctl/firewall/Tailscale call,
  or remote/workflow path.

## Baseline Revalidation

- One synthetic attachment larger than 4 MiB was sent through qsc, qsl-server,
  and qsl-attachments, fetched by qsc, decrypted locally, and byte-validated.
- Classification: `BASELINE_LOCAL_ATTACHMENT_SEND_RECEIVE_PASS`.

## Repetition Stress

- Ten sequential above-threshold attachment send/receive cycles passed.
- Each cycle used a fresh proof-root fixture identity and validated the received
  object.
- Classification: `ATTACHMENT_REPETITION_STRESS_PASS`.

## Multi-Attachment / Multi-Size Stress

- One-message multi-attachment is not exposed by the selected qsc command
  surface and was classified unsupported.
- Multiple attachment messages with just-over-4 MiB and larger bounded size
  classes passed with no cross-object mismatch.
- Classification: `MULTI_ATTACHMENT_STRESS_PASS`.

## Attachment Threshold Boundary

- Just-over-4 MiB boundary behavior passed through qsl-attachments.
- The corrected exact 4 MiB legacy boundary probe timed out inside the bounded
  local stress window and was isolated from later negative tests.
- Classification: `MULTI_ATTACHMENT_SIZE_BOUNDARY_GAP`.

## Capability / Auth Negatives

- Wrong qsl-attachments fetch capability failed closed through qsc and recovered
  with the correct capability.
- Missing/wrong direct fetch capability failed closed.
- Unknown object failed closed.
- Short-retention expired object failed closed.
- Classification: `CAPABILITY_AUTH_NEGATIVES_PASS`.

## Descriptor / Object Corruption Negatives

- Corrupted descriptor failed closed.
- Corrupted fetched ciphertext failed closed.
- Missing object after descriptor failed closed.
- Wrong object for descriptor failed closed for both swapped descriptors.
- Classification: descriptor `DESCRIPTOR_NEGATIVES_PASS`; object
  `OBJECT_CORRUPTION_NEGATIVES_PASS`.

## Route / Peer / Relay Negatives

- Wrong-route direct pull returned no-item class.
- Correct-route recovery after wrong-route probe succeeded.
- Missing route-token direct pull failed closed.
- Empty push body failed closed.
- Empty pull after drain returned no-item class.
- Classification: `ROUTE_PEER_RELAY_NEGATIVES_PASS`.

## Restart / Retention / Cleanup Stress

- qsl-attachments restart after upload before fetch preserved retained object
  fetch.
- qsl-server descriptor persistence across restart is not supported because the
  selected local qsl-server runtime is memory-only.
- Classification: `RESTART_RETENTION_STRESS_PASS`.

## Concurrency / Rapid Operations

- Direct qsl-attachments parallel upload/fetch workers passed.
- Three rapid sequential qsc attachment send/receive operations passed.
- Classification: `ATTACHMENT_CONCURRENCY_STRESS_PASS`.

## Metadata Minimization Matrix

- Filename, exact size, chunk/object count, object ID, capability, route,
  sender/recipient, timing, command-line, process identity, and storage path
  details remain proof-root-only.
- Size bucket is class-only.
- Payload/body exposure: no.
- Plaintext exposure: no.
- Key material exposure: no.
- Seed fallback exposure: no.
- Ciphertext body exposure: proof-root-only.
- Residual metadata classification: `acceptable_for_next_remote_readiness`, with
  the exact-boundary resource gap carried to the selected diagnostic successor.

## Cleanup / Rollback

- All NA-0595-owned local qsl-server and qsl-attachments processes were stopped.
- No owned process remained listening after cleanup.
- Proof-root storage was retained only as local artifacts.
- Classification: `ATTACHMENT_STRESS_CLEANUP_DONE`.

## Issue Investigation and Safe Fix

- No product source fix was applied.
- No qsl-server or qsl-attachments PR was created.
- Proof-root and validation recoveries were limited to parser/classifier/
  isolation, goal-lint preflight setup, and scanner-scope fixes. They did not
  weaken validation, redaction, or fail-closed behavior.
- Failure-cause classification:
  `ATTACHMENT_STRESS_FAILURE_RESOURCE_BOUNDARY_GAP`.
- Safe-fix classification: `SAFE_FIX_SKIPPED`.

## Private-Material Review

- Aggregate plaintext marker scan: PASS.
- qsl-server runtime logs: zero plaintext fixture-marker hits.
- qsl-attachments proof-root storage: zero plaintext fixture-marker hits.
- No endpoint value, private port value, token or Authorization value,
  capability value, payload/body/plaintext bytes, seed value, or key material is
  published.

## Result Classification

Selected classification:
`LOCAL_QSL_ATTACHMENTS_STRESS_RESOURCE_BOUNDARY_GAP`.

The boundary gap is local and diagnostic: it concerns exact 4 MiB legacy
boundary runtime/resource behavior under this bounded harness. It is not a
capability/auth fail-open, descriptor/object acceptance, plaintext exposure,
seed fallback regression, qsl-server boundary failure, or qsl-attachments
opaque-storage failure.

## Selected Successor

Selected successor:
`NA-0596 -- QSL Local Attachment Stress Diagnostic Follow-Up Harness`.

The successor should complete the exact stress diagnostic/resource-boundary gap
without expanding to remote/Tailscale/workflow.

## Required-Check Boundary

Startup main health classified public-safety success, advisories success,
suite2-vectors success, no failed required checks, no pending required checks,
root cargo audit success, nested qsc fuzz cargo audit success, locked metadata
success, and no Cargo drift. Non-required remote check failures attached to
main were classified separately and did not block local NA-0595 work.

## Source / Script Mutation Boundary

No qsc source/test/script mutation, qsl-server mutation, qsl-attachments
mutation, dependency mutation, lockfile mutation, workflow mutation, or selected
source fix occurred in NA-0595. This implementation PR mutates only governance
evidence/testplan/decision/trace/journal files.

## qsc Boundary

qsc remained the owner of attachment encryption, descriptor construction,
descriptor verification, fetch verification, local decrypt, and validation.
Seed fallback env-only behavior remains blocked.

## qsl-server Boundary

qsl-server remained relay/control-plane only. It relayed opaque qsc envelopes and
did not store attachment object bytes or plaintext fixture markers.

## qsl-attachments Boundary

qsl-attachments remained opaque object storage. It received opaque ciphertext
object bytes and did not receive qsc plaintext or qsc key material.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, workflow dispatch, workflow rerun,
GitHub-runner test, public-network exposure, or remote readiness action
occurred.

## Public-Site / Cloudflare Boundary

No public-site, docs/public, website, Cloudflare, DNS, deployment, service,
systemd, firewall, or public exposure mutation occurred.

## Evidence / Decision / Traceability

- D-1181 records NA-0595 stress evidence and selected successor.
- This evidence file records the local proof classification.
- `tests/NA-0595_local_qsl_attachments_adversarial_metadata_stress_testplan.md`
  records required validation markers.
- `TRACEABILITY.md` maps NA-0595 to D-1181.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records qwork proof, recoveries,
  validation, classification, selected successor, and boundaries.

## Validation

Validation includes startup qwork proof, queue/decision proof, main required
check classification, qsl-server validation, qsl-attachments validation, focused
qsc validation, proof-root stress harness completion, private-material scans,
cleanup proof, and the pre-PR validation suite recorded in the PR evidence.

## Recommendation

Merge NA-0595 as a resource-boundary diagnostic result, then close out NA-0595
to restore `NA-0596 -- QSL Local Attachment Stress Diagnostic Follow-Up
Harness` as the sole READY successor. Do not advance to remote/Tailnet readiness
until the exact-boundary diagnostic/resource gap is handled or explicitly
accepted by a later decision.
