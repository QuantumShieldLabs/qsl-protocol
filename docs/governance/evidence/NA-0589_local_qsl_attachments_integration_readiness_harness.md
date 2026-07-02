Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0589 Local qsl-attachments Integration Readiness Harness

## Executive Summary

NA-0589 consumed the NA-0588 local qsc/qsl-server adversarial metadata stress result and assessed qsl-attachments readiness for later local integration. qsl-attachments source was acquired, reviewed, built, tested, and classified. Initial qsl-attachments audit found one dependency advisory in the locked graph. A scratch recovery proved that updating only `Cargo.lock` was sufficient, qsl-attachments PR #38 merged that lockfile-only recovery at `767eca189ee0`, and post-merge metadata, audit, fmt, test, and build validation passed.

Result classification: `QSL_ATTACHMENTS_LOCKFILE_ONLY_RECOVERY_IMPLEMENTED`.

Selected successor: `NA-0590 -- QSL qsl-attachments Recovery Verification and Integration Readiness Harness`.

## qwork Proof Verification

Fresh NA-0589 qwork proof was verified before fetch, repository mutation, qsl-attachments clone/build/test/run, qsc command execution, qsl-server start, source-analysis publication, GitHub metadata review, or proof publication.

- qwork startup result: OK.
- lane: NA-0589.
- repo: qsl-protocol.
- proof timestamp: `2026-07-02T04:01:52Z`.
- qsl-protocol startup head, origin/main, and main: `62de62d64f5e`.
- worktree, index, and untracked startup state: clean.
- READY_COUNT: 1.
- READY item: NA-0589.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.

Disk and mount gates passed before fetch. Root usage was below the stop threshold, and the backup mount was present.

## D-1167 / D-1168 Inheritance

D-1167 and D-1168 were verified once each and Accepted. D-1167 recorded `LOCAL_QSC_QSL_SERVER_E2EE_ADVERSARIAL_METADATA_STRESS_PASS`, selected NA-0589, and deferred qsl-attachments. D-1168 closed NA-0588 and restored NA-0589 as the sole READY item. D-1169 and D-1170 were absent before the NA-0589 implementation patch.

## Authority Model Application

The D-1161-expanded issue-investigation and safe-fix authority was applied. qsl-attachments read/build/test/audit work was allowed locally. A qsl-attachments lockfile-only recovery was selected only after scratch proof showed the advisory recovery changed `Cargo.lock` only, did not require `Cargo.toml`, did not change source, and preserved validation.

## qsl-attachments Source Inventory

Source repository: `QuantumShieldLabs/qsl-attachments`.

- Initial reviewed qsl-attachments commit: `96b9352bd63e`.
- Post-recovery qsl-attachments main: `767eca189ee0`.
- Worktree state after recovery: clean.
- Package: `qsl-attachments`.
- Binary target: `qsl-attachments`.
- Library target: `qsl_attachments`.
- Tests: present.
- README/docs: present.
- Examples: none found.
- License/security docs: no dedicated security policy was required for this lane.

## qsl-attachments Build / Audit / Test

Initial qsl-attachments validation:

- `cargo metadata --locked --format-version=1`: pass.
- `cargo audit --deny warnings`: failed on a dependency advisory.
- `cargo fmt --check`: pass.
- `cargo test --locked`: pass.
- `cargo build --locked`: pass.

Recovery:

- Scratch proof showed `cargo update -p anyhow --precise 1.0.103` changed only qsl-attachments `Cargo.lock`.
- qsl-attachments PR #38 merged the lockfile-only recovery.
- Post-merge qsl-attachments validation passed metadata, audit, fmt, test, and build.

Build/audit/test classification: `QSL_ATTACHMENTS_AUDIT_LOCKFILE_ONLY_RECOVERY_PASS`.

## qsl-protocol Attachment Expectation Review

qsl-protocol attachment expectations are present in canonical attachment documents and qsc implementation/tests. The model separates qsl-server relay/control-plane behavior from qsl-attachments blob storage. qsc constructs and authenticates attachment descriptors, encrypts attachment data before upload, fetches opaque data, verifies integrity, and decrypts locally.

Attachment service expectation: store opaque ciphertext bytes and related non-secret protocol metadata classes, not plaintext or key material.

## qsc Attachment Command Surface Review

qsc has attachment-aware file send/receive command support and an attachment client module. The reviewed surface supports a local attachment service option for file send/receive, creates sessions, uploads encrypted parts, commits objects, fetches objects, and verifies/decrypts locally. Existing qsc tests include local qsl-attachments helper coverage and negative checks for resume/fetch capability behavior.

No qsc source, tests, examples, or helper code changed in NA-0589.

## Runtime / Service Model

Runtime classification: `RUNTIME_MODEL_MIXED`.

qsl-attachments is an HTTP service with a Rust library and local filesystem storage service behavior. The reviewed runtime supports local bind configuration, configurable storage root, upload-session lifecycle, part upload, status, commit, abort, object fetch, retention cleanup, and resource bounds. The service can be configured for loopback-only local use and proof-root-contained storage in a later lane.

## Crypto / Opaque-Data Boundary

Crypto/opaque-data classification:

- `ATTACHMENT_OPAQUE_CIPHERTEXT_BOUNDARY_CONFIRMED`.
- `ATTACHMENT_ENCRYPTION_OWNER_QSC`.
- `ATTACHMENT_KEY_MATERIAL_BOUNDARY_CLEAR`.

qsc encrypts and authenticates attachment content before storage. qsl-attachments stores opaque ciphertext bytes and metadata needed for storage and retrieval. qsl-attachments does not require plaintext body access or qsc key material for the reviewed local integration path.

## Auth / Access Control Boundary

Auth classification:

- `AUTH_MODEL_CAPABILITY_ID`.
- Authorization header model is not the selected qsl-attachments local access model.

qsl-attachments uses capability-style values for resume/fetch access and stores verifier material rather than raw capability publication. Missing or wrong capability evidence is fail-closed in source/tests. Attachment IDs and locator references are not treated as sufficient authorization.

No token, Authorization value, raw capability value, or capability-like object value is published here.

## Storage / Retention / Resource Boundary

Storage classification:

- Backend: local filesystem.
- Storage path class: configurable, proof-root-suitable.
- Cleanup/delete support: present for session lifecycle and retention cleanup.
- TTL/retention: present.
- Maximum size/resource bounds: present.
- Restart persistence: committed local objects and coherent open sessions are persistent; incoherent or expired state is rejected or cleaned.
- Local smoke resource risk: low when proof-root-contained.

## Metadata Exposure Review

Metadata classes:

- Filename exposure: avoidable in qsl-attachments; qsc descriptor policy must control any user-facing filename metadata.
- Size exposure: required/observable by storage and transfer bounds.
- MIME/content-type exposure: avoidable for the storage service in the reviewed path.
- Object ID / locator reference exposure: required as reference class; not authorization by itself.
- Capability exposure: proof-root-only and never publishable as a value.
- Route/recipient exposure: qsl-server/qsc descriptor boundary, not qsl-attachments storage requirement.
- Timing exposure: operationally observable, publish class only.
- Storage path exposure: proof-root-only.
- Payload/body/plaintext/key exposure: not required and not published.
- Logs/artifacts exposure: proof-root-only raw logs; repository evidence uses classes only.

Metadata review classification: acceptable for recovery-verification successor with class-only publication.

## Failure Behavior Review

Reviewed source/tests support fail-closed classes for missing/wrong capabilities, unknown objects, malformed requests, oversized requests, invalid or conflicting part upload, expired sessions, bad ranges, quota pressure, and malformed state. Duplicate identical part upload is idempotent by design. Storage failure and cleanup failure behavior is bounded by local filesystem error handling and cleanup paths, but should remain a focus in the next recovery-verification readiness lane.

## Optional Local No-Secret Smoke

Classification: `QSL_ATTACHMENTS_LOCAL_SMOKE_NOT_RUN_NOT_NEEDED`.

The lane deferred a fresh runtime smoke because the source/test review and qsl-attachments validation were sufficient for boundary classification, and NA-0589 had to complete a qsl-attachments lockfile-only recovery first. A later recovery-verification lane can run proof-root-contained loopback service smoke before any send/receive integration.

## First Local Integration Plan

The first local integration should be gated by NA-0590 recovery verification. The plan is:

- start qsl-attachments as a loopback-only local service using proof-root-contained storage;
- use non-secret fixture content only;
- start/use the already proven local qsc/qsl-server foundation only after recovery verification passes;
- run qsc attachment send through local qsl-server descriptor exchange and local qsl-attachments opaque blob storage;
- run qsc receive/fetch/decrypt/validate locally;
- verify cleanup and retention behavior;
- run fail-closed negatives for missing/wrong capability, wrong object, wrong recipient/descriptor where supported, and malformed or oversized input;
- scan logs/artifacts and publish only safe classes;
- stop on any plaintext, token, Authorization, capability value, key material, raw payload/body, private port, endpoint value, or public/production/security claim risk.

## Issue Investigation and Safe Fix

Issue investigation executed for the initial audit failure. Cause classification: `QSL_ATTACHMENTS_FAILURE_DEPENDENCY_ADVISORY`.

Safe fix selected: qsl-attachments `Cargo.lock` only. qsl-attachments PR #38 merged the recovery from head `be54a80a870b` to main merge `767eca189ee0`. No qsl-attachments source, `Cargo.toml`, deployment/service, Docker, systemd, cloud, protocol, crypto, auth, wire, storage-model semantic, or qsl-protocol source change was made.

Recovered operational issues were proof-root-only command-shape/parser/polling issues and were corrected within the bounded policy.

## Private-Material Review

Private-material review passed for qsl-attachments logs/artifacts, qsl-attachments diffs, qsl-protocol diffs, smoke decision, metadata review, integration plan, evidence docs, PR text, and response draft classes.

Repository evidence does not publish endpoint values, private port values, token values, bearer values, Authorization values, raw capability values, payloads, response bodies, plaintext attachment contents, sensitive filenames, key material, secret env values, raw logs, or raw storage paths.

## Result Classification

Selected classification: `QSL_ATTACHMENTS_LOCKFILE_ONLY_RECOVERY_IMPLEMENTED`.

The recovered post-merge state is favorable for readiness, but the directive requires a recovery-verification successor after a qsl-attachments recovery PR.

## Selected Successor

`NA-0590 -- QSL qsl-attachments Recovery Verification and Integration Readiness Harness`

Status: READY.

Goals: G1, G2, G3, G4, G5.

Objective: verify the qsl-attachments recovery PR and rerun readiness review before starting local send/receive integration.

## Required-Check Boundary

Current qsl-protocol main checks were classified before implementation: public-safety success, advisories success, suite2 success, no failed required checks, and no required pending checks. qsl-attachments PR #38 completed its required check successfully before merge.

## Source / Script Mutation Boundary

qsl-protocol implementation mutation is limited to governance/evidence/testplan/traceability/journal files. qsl-attachments mutation was limited to `Cargo.lock` in qsl-attachments PR #38. No source or script behavior changed in qsl-protocol.

## qsc Boundary

qsc was reviewed for attachment command and source surfaces. qsc was not mutated. qsc remains the encryption/decryption owner for the reviewed attachment flow.

## qsl-server Boundary

qsl-server remains a local relay/control-plane component from NA-0587/NA-0588 evidence. qsl-server was not started or mutated in NA-0589.

## qsl-attachments Boundary

qsl-attachments was cloned, reviewed, validated, and recovered through a lockfile-only audit fix. qsl-attachments was not integrated with qsc/qsl-server in this lane.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale, public network exposure, GitHub workflow dispatch, or GitHub workflow rerun occurred. GitHub usage was limited to normal qsl-attachments PR creation/check/merge and qsl-protocol PR handling.

## Public-Site / Cloudflare Boundary

No public-site, website, public docs, Cloudflare, DNS, or public deployment mutation occurred.

## Evidence / Decision / Traceability

This evidence document, the NA-0589 testplan, D-1169, TRACEABILITY.md, and the rolling operations journal record the readiness classification, qsl-attachments PR #38 recovery, private-material boundary, and selected successor.

## Validation

Validation before PR includes scope guard, queue/decision proof, marker proof, link check, added-line/new-file private-material scan, qsl-attachments artifact/log private-material scan, overclaim scan, PR body preflight, goal-lint when available, root cargo audit, nested qsc fuzz cargo audit, locked metadata, cargo fmt, shell syntax checks for qsc adversarial script, and qsl-attachments post-recovery metadata/audit/fmt/test/build.

## Recommendation

Proceed to NA-0590 recovery verification before first local qsc/qsl-server/qsl-attachments send/receive integration. Keep raw service/log/artifact values proof-root-only, preserve loopback-only execution, and continue class-only publication.
