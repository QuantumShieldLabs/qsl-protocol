Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-04

Goals: G1, G2, G3, G4, G5

# NA-0598 qsl-server Exact 4 MiB Relay Boundary Fix Harness

## Executive Summary

NA-0598 consumed the NA-0597 qsl-server relay queue-depth classification, implemented a bounded qsl-server fix, merged qsl-server PR #59, and retested the exact 4 MiB legacy transfer locally.

The result classification is `QSL_SERVER_EXACT_4MIB_RELAY_BOUNDARY_FIX_PASS`. Exact 4 MiB remains legacy in-message, qsl-attachments remains not used for exact 4 MiB, qsl-server accepts the required data chunks plus manifest, qsc receive/decrypt/validate completes, just-over-4 MiB and known-good greater-than-4-MiB controls use qsl-attachments, and resource/auth negatives remain bounded and fail-closed.

## qwork Proof Verification

Fresh qwork proof from `2026-07-03T22:13:30Z` was copied and verified before fetch, qsl-server acquisition/build/run, qsl-attachments acquisition/build/run, qsc execution, repository mutation, PR creation, source-analysis publication, or proof publication.

The proof matched lane NA-0598, repo qsl-protocol, path `/srv/qbuild/work/NA-0598/qsl-protocol`, branch main, upstream origin/main, clean worktree/index/untracked state, READY_COUNT 1, READY NA-0598, and shared cargo target readiness. Live pre-fetch `HEAD` and `origin/main` matched `aeb11a3d1813`. Root disk usage was below the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork, qstart, or qresume.

## D-1185 / D-1186 Inheritance

D-1185 and D-1186 were each present once and Accepted. D-1185 classified NA-0597 as `LOCAL_ATTACHMENT_EXACT_4MIB_QSL_SERVER_RELAY_BUG_FOUND` and selected NA-0598. D-1186 restored NA-0598 as READY. NA-0597 and NA-0596 were DONE before this lane.

Inherited facts were consistent: exact 4 MiB remains legacy in-message, exact 4 MiB does not use qsl-attachments, exact 4 MiB reaches the full data-chunk class, the prior failure was qsl-server relay queue-full at manifest publication, qsc threshold/path selection was not selected as the bug, qsl-attachments was not selected as the bug, and no qsl-server source mutation occurred in NA-0597.

## Authority Model Application

NA-0598 used Tier 0 read-only analysis, Tier 1 proof-root harness/classifier repair, Tier 2 qsl-server source/test fix authority, Tier 3 bounded product semantic caution, and Tier 4 local runtime proof. The qsl-server mutation was limited to `src/lib.rs`, `src/main.rs`, `README.md`, and NA-0598 tests under `tests/`.

No qsc source mutation, qsl-attachments mutation, dependency/lockfile mutation, deployment/service mutation, workflow mutation, remote action, Tailscale action, public-site mutation, Cloudflare mutation, qwork/qstart/qresume execution, sudo/systemd/firewall action, or private-material publication occurred.

## qsl-server Queue-Depth Source Review

qsl-server main before fix used a bounded per-route queue model with a `HashMap` of route state and per-route `VecDeque` relay messages. Defaults and ceilings were configured in source and CLI docs. Auth and route-token checks occur before queue mutation. Payload/body logging was not observed; qsl-server logs metadata classes rather than plaintext bodies.

The source model classification was `QSL_SERVER_QUEUE_DEPTH_MODEL_CONFIGURED`. The pre-fix default/ceiling allowed 256 queued relay items, which fits the exact 4 MiB data chunks but not the manifest message that follows them.

## qsl-server Reproduction

Clean qsl-server main at `6bf61d439fa2` passed metadata, audit, fmt, tests, and build before mutation. A qsl-server-focused reproduction then accepted 256 data-chunk-like opaque pushes and rejected the manifest-like 257th push with the queue-full class. The reproduction classification was `QSL_SERVER_EXACT_4MIB_QUEUE_FULL_REPRODUCED`.

## Fix Strategy

The selected strategy was a bounded plus-one capacity fix: increase the qsl-server default/ceiling queue depth and push burst/refill defaults from 256 to 257. This permits the exact 4 MiB legacy shape's 256 data chunks plus manifest while preserving bounded queues and beyond-bound rejection.

Rejected strategies included qsc threshold changes, qsl-attachments path changes, unbounded queue growth, broad storage redesign, dependency changes, and deployment/service changes.

## qsl-server Implementation / PR Evidence

qsl-server branch: `na-0598-qsl-server-exact-4mib-relay-boundary-fix`.

qsl-server implementation commit: `560379da0262`, message `NA-0598 fix qsl-server exact 4 MiB relay boundary`.

qsl-server PR: #59, title `NA-0598: fix qsl-server exact 4 MiB relay boundary`.

qsl-server merge commit: `544edfd213ea`.

Changed qsl-server paths:

- `src/lib.rs`
- `src/main.rs`
- `README.md`
- `tests/na0598_exact_4mib_relay_boundary.rs`
- `tests/na0598_exact_4mib_relay_logging.rs`

No qsl-server `Cargo.toml`, `Cargo.lock`, dependency, workflow, deployment, Docker, systemd, cloud, or service path changed.

## qsl-server Validation

qsl-server validation passed before patch on clean main, on the fix branch, and after merge on main. The recorded validation set included:

- `cargo metadata --locked --format-version=1`
- `cargo audit --deny warnings`
- `cargo fmt --check`
- `cargo test --locked`
- `cargo build --locked`
- `cargo clippy --locked --all-targets -- -D warnings`

qsl-server main was refreshed after merge and left clean at `544edfd213ea`.

## Local Exact 4 MiB Retest

The local proof-root retest used fixed qsl-server main, loopback-only qsl-server, loopback-only qsl-attachments for controls, proof-root-only qsc state/storage/logs, and seed fallback variables unset.

Exact 4 MiB classification: `EXACT_4MIB_QSL_SERVER_RELAY_FIX_LOCAL_PASS`.

Exact 4 MiB results:

- qsc policy: legacy in-message.
- qsl-attachments exact interaction: not_used.
- qsl-server accepted the data chunks plus manifest.
- qsc receive/decrypt/validate completed.
- final data chunk, manifest, and file-complete markers were observed.
- relay overload/rate-limit class was not observed.
- exact-case qsl-attachments object count remained zero.
- service logs did not show payload/plaintext sentinel leakage.

## Above-Threshold qsl-attachments Controls

Above-threshold control classification: `ABOVE_THRESHOLD_CONTROLS_AFTER_QSL_SERVER_FIX_PASS`.

The just-over-4 MiB control used the qsl-attachments path, committed through qsl-attachments, fetched the object, and completed qsc receive/decrypt/validate. The known-good 5 MiB control also used the qsl-attachments path, committed through qsl-attachments, fetched the object, and completed qsc receive/decrypt/validate.

These controls preserve the greater-than-4-MiB qsl-attachments behavior and do not move exact 4 MiB to qsl-attachments.

## Resource / Auth Regression Tests

Resource/auth regression classification: `QSL_SERVER_RESOURCE_AUTH_REGRESSION_TESTS_PASS`.

Selected negatives:

- queue beyond bound: resource_bound_preserved.
- push burst beyond bound: resource_bound_preserved.
- missing bearer: pass_fail_closed.
- wrong bearer: pass_fail_closed.
- wrong route token: pass_fail_closed.
- route isolation: pass_fail_closed.
- empty pull after drain: pass_fail_closed.
- unbounded queue attempt: resource_bound_preserved.
- attachment descriptor / unstructured body: opaque relay payload subject to auth and bounds.
- qsl-server payload/plaintext logging: not_observed.

## Metadata / Private-Material Review

Metadata review classification: `QSL_SERVER_EXACT_4MIB_METADATA_REVIEW_PASS`.

Aggregate private-material review classification: `AGGREGATE_NA0598_PRIVATE_MATERIAL_REVIEW_PASS`.

Published evidence is class-only. Endpoint values, private port values, route-token values, bearer values, Authorization values, capability values, payload/body/plaintext bytes, ciphertext bodies, seed values, key material, raw command lines, raw logs, process identities, exact storage paths, private topology, and private material remain proof-root-only.

## Cleanup / Rollback

Cleanup classification: `QSL_SERVER_EXACT_4MIB_FIX_CLEANUP_DONE`.

Owned qsc, qsl-server, and qsl-attachments process counts were zero after the local retest. Proof-root artifacts were retained. No qsl-server rollback was required because PR #59 merged and post-merge validation passed.

## Result Classification

`QSL_SERVER_EXACT_4MIB_RELAY_BOUNDARY_FIX_PASS`

## Selected Successor

NA-0599 is selected as `QSL Remote / Tailnet Full-Stack Reintroduction Readiness Harness`.

Status: READY

Goals: G1, G2, G3, G4, G5

Objective:
Plan and authorize the controlled reintroduction of remote/Tailnet full-stack testing after the local qsc/qsl-server/qsl-attachments path passed integration, adversarial/metadata stress, seed-fallback hardening, exact 4 MiB boundary diagnostics, and qsl-server exact 4 MiB relay-boundary fix. Codex must separate local correctness evidence from remote reachability, identify exact remote/Tailnet/GitHub-runner constraints, define whether operator Tailscale/OAuth/secret setup is required, design redacted DNS/TCP/TLS/HTTP/relay/attachment diagnostics, preserve no private endpoint/token/topology publication, and select the first bounded remote verification successor. Codex must not mutate Tailscale, Cloudflare, public DNS, GitHub secrets, workflows, remote hosts, qsl-server deployment, or qsl-attachments deployment unless a later exact directive authorizes that action.

## Required-Check Boundary

Current qsl-protocol main health passed before mutation: public-safety success, advisories success, suite2-vectors success/satisfied, no failed or pending attached required checks, root cargo audit success, nested qsc fuzz cargo audit success, locked metadata success, and Cargo drift absent.

qsl-server PR #59 checks attached and completed green before merge. qsl-server post-merge validation passed.

## Source / Script Mutation Boundary

qsl-protocol mutation in this PR is governance-only: this evidence document, the NA-0598 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling operations journal.

No qsl-protocol qsc source/test/example/fuzz/script/workflow/dependency/lockfile path was mutated.

## qsc Boundary

qsc source was reviewed read-only. The strict greater-than-4-MiB attachment threshold remains unchanged. Exact 4 MiB remains legacy in-message. No qsc threshold/path-selection mutation occurred.

## qsl-server Boundary

qsl-server was the only product repo mutated. The qsl-server fix was narrow and bounded: queue depth and push burst/refill defaults/ceilings were changed from 256 to 257 with targeted tests. Auth, route isolation, fail-closed behavior, metadata-only logging, and beyond-bound rejection were preserved.

## qsl-attachments Boundary

qsl-attachments was read/build/run-only for controls. No qsl-attachments mutation occurred. The above-threshold controls passed after the qsl-server fix.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, workflow dispatch, workflow rerun, public-network action, deployment action, GitHub runner action, sudo, systemd, or firewall action occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, docs/public, Cloudflare, public DNS, or public deployment path was mutated.

## Evidence / Decision / Traceability

D-1187 records the NA-0598 result classification, qsl-server PR/merge evidence, local exact 4 MiB retest, above-threshold controls, resource/auth regression tests, metadata/private-material review, cleanup, and selected successor. `TRACEABILITY.md` maps NA-0598 to this evidence document and testplan. The rolling operations journal records proof gates, recoveries, validation, qsl-server PR evidence, local runtime proof, and boundaries.

## Validation

Validation covered qsl-server metadata/audit/fmt/tests/build/clippy, qsl-attachments metadata/audit/fmt/tests/build, qsl-protocol main-health checks, local exact 4 MiB retest, above-threshold controls, scope guards, marker proof, link-check, private-material scans, overclaim scans, goal-lint/PR-body preflight, cargo audits, cargo metadata, cargo fmt, and qsc-adversarial shell syntax.

Recovered failures were proof-root or command-shape issues and are recorded in the proof root. Each recovery stayed within the directive's bounded recovery policy and ended with a passing replacement or safe classification.

## Recommendation

Merge the NA-0598 governance PR after required checks pass. If post-merge checks remain green, close out NA-0598 by marking it DONE and restoring the selected NA-0599 readiness successor. Do not implement NA-0599 in the closeout PR.
