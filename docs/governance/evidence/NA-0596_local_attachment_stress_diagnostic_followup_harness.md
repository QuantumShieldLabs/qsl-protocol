Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-03

Goals: G1, G2, G3, G4, G5

# NA-0596 Local Attachment Stress Diagnostic Follow-Up Harness

## Executive Summary

NA-0596 consumed the NA-0595 exact 4 MiB diagnostic gap and classified it as `LOCAL_ATTACHMENT_EXACT_4MIB_HARNESS_TIMEOUT_BUDGET_GAP`.

The exact 4 MiB probe entered qsc legacy in-message handling, did not use qsl-attachments, and showed qsl-server relay-accepted chunk progress. It did not complete inside the bounded 180-second qsc-send phase. The just-over-4 MiB and known-good above-threshold controls used qsl-attachments as expected and completed send/receive/decrypt validation before legacy timeout probes.

## qwork Proof Verification

Fresh qwork proof from `2026-07-03T18:13:28Z` was copied and verified from the lane workspace. Live pre-fetch `HEAD` and `origin/main` matched `a9de6a9979fb`. Worktree, index, and untracked state were clean. Root disk usage was below the stop threshold and `/backup/qsl` was mounted.

Recovered proof parser issues were proof-root-only and recorded with failing command, classification, corrective action, and final result.

## D-1181 / D-1182 Inheritance

D-1181 and D-1182 were each present once and Accepted. D-1181 classified NA-0595 as `LOCAL_QSL_ATTACHMENTS_STRESS_RESOURCE_BOUNDARY_GAP` and selected NA-0596. D-1182 restored NA-0596 as READY. NA-0595 and NA-0594 were DONE before this lane.

## Authority Model Application

The lane used local diagnostic and issue-investigation authority only. qsl-server and qsl-attachments were cloned, built, audited, tested, and run locally on loopback. qsc used proof-root-only state. No remote, Tailscale, workflow, sudo, systemd, firewall, dependency, lockfile, qwork, qstart, or qresume action occurred.

## Threshold Policy Revalidation

qsc source review revalidated that the attachment threshold is strict greater-than 4 MiB for the above-threshold qsl-attachments path. Exact 4 MiB is legacy-sized. `QSC_ATTACHMENT_SERVICE` environment activation can select the validated post-w0 lane for legacy-sized files, so the exact-boundary diagnostic kept that environment unset and used explicit attachment-service arguments only for above-threshold controls.

## qsc Threshold Boundary Source Review

qsc review covered the threshold constant, comparison operator, legacy in-message path, attachment-service configured path, attachment-service absent fail-closed behavior, fixture-size handling, send/receive timeout behavior, receive/pull behavior, path-selection diagnostics, and seed fallback hardening. No qsc source mutation was selected.

## qsl-server Exact 4 MiB Relay Review

qsl-server remains a relay/control-plane surface. For the diagnostic, qsl-server ran locally on loopback with an explicit diagnostic body budget sufficient for legacy-envelope relay attempts. The exact 4 MiB probe showed relay-accepted chunk progress before qsc send exceeded the bounded diagnostic window.

## qsl-attachments Boundary Non-Use Review

qsl-attachments ran locally on loopback with proof-root storage. Exact 4 MiB did not emit attachment commit markers and did not change qsl-attachments storage counts. Just-over and above-threshold controls used qsl-attachments as expected.

## Local Build / Audit / Test Readiness

qsl-server metadata, audit, fmt, serial tests, and build passed after a recovered parallel log-capture test issue. qsl-attachments metadata, audit, fmt, tests, and build passed. qsc focused validation passed, including true triple-ratchet path, seed fallback hardening, attachment contract/streaming tests, qsp/relay tests, and receive peer-separation.

## Diagnostic Harness Design

The proof-root harness starts qsl-server and qsl-attachments on loopback, uses proof-root storage/state, generates synthetic fixtures under the proof root, removes seed fallback environment variables, performs handshake-backed qsc setup, records per-phase timing buckets, emits JSON class summaries, and cleans up owned processes.

## Baseline Just-Over-4 MiB Revalidation

The just-over-4 MiB control classified as qsl-attachments path used expectedly and completed send/receive/decrypt validation. The known-good above-threshold control also completed.

## Threshold Matrix Diagnostic

- Known-good above-threshold: qsl-attachments, send success, receive success, decrypt/validate success.
- 4 MiB plus 1 byte: qsl-attachments, send success, receive success, decrypt/validate success.
- Exact 4 MiB: legacy in-message, qsl-attachments not used, qsc send exceeded the bounded phase window.
- 4 MiB minus 1 byte: legacy in-message in a clean prior run, qsl-attachments not used, qsc send exceeded the bounded phase window.

## Exact 4 MiB Deep Diagnostic

The final exact probe used a fresh qsc state before any legacy timeout contamination. It emitted legacy-sized policy and relay-accepted chunk progress, then exceeded the bounded qsc-send phase. It did not show qsl-attachments use, qsl-server relay timeout, fixture-generation overhead, receive/pull timeout, or decrypt/validate timeout.

## Timeout Phase Classification

Timeout phase: `qsc_send`.

## qsl-attachments Interaction Classification

Exact 4 MiB qsl-attachments interaction: `not_used`.

## Metadata / Private-Material Review

Published evidence is class-only. Raw endpoint values, private ports, route-token values, capability values, command lines, process identities, exact storage paths, raw logs, payload/body/plaintext bytes, ciphertext bodies, seed values, and key material remain proof-root-only.

## Cleanup / Rollback

The harness terminated owned qsl-server and qsl-attachments processes. External qsl-server build artifacts created during validation were removed. No unknown owned listener remained.

## Issue Investigation and Safe Fix

Issue investigation selected no product source fix. The only changes during diagnostics were proof-root harness repairs for command shape, state isolation, and case ordering. No qsc, qsl-server, or qsl-attachments source/test/docs mutation was selected.

## Result Classification

`LOCAL_ATTACHMENT_EXACT_4MIB_HARNESS_TIMEOUT_BUDGET_GAP`

## Selected Successor

NA-0597 is selected as `QSL Local Exact 4 MiB Attachment Boundary Diagnostic Completion Harness`.

## Required-Check Boundary

Current main required checks were classified before implementation: public-safety success, advisories success, suite2-vectors success/satisfied, no failed attached check-runs, and no pending attached check-runs.

## Source / Script Mutation Boundary

No qsl-protocol qsc source/test/script mutation occurred. No dependency, lockfile, workflow, public-site, formal, refimpl, qshield, or backup path was changed.

## qsc Boundary

qsc source was reviewed. qsc source was not mutated. Exact 4 MiB remains legacy-sized and not an above-threshold qsl-attachments case.

## qsl-server Boundary

qsl-server source was not mutated. Local runtime use was loopback only and diagnostic-only.

## qsl-attachments Boundary

qsl-attachments source was not mutated. Local runtime use was loopback/proof-root storage only.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, GitHub workflow dispatch, or workflow rerun occurred.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare action occurred.

## Evidence / Decision / Traceability

D-1183 records this classification. TRACEABILITY maps NA-0596 evidence to D-1183.

## Validation

Validation evidence is recorded in the proof root and testplan. Governance validation before PR includes scope guard, marker proof, link check, private-material scan, overclaim scan, cargo audits, locked metadata, cargo fmt, qsc-adversarial shell syntax, and local qsl-server/qsl-attachments validation summaries.

## Recommendation

Proceed to closeout after implementation merge and restore NA-0597 as the local exact-boundary diagnostic completion harness. The next lane should tune the exact legacy send budget/fixture strategy or add a purpose-built diagnostic that can complete or bound the remaining legacy-send cost without changing threshold semantics.
