Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-03

Goals: G1, G2, G3, G4, G5

# NA-0597 Local Exact 4 MiB Attachment Boundary Diagnostic Completion Harness

## Executive Summary

NA-0597 consumed the NA-0596 exact 4 MiB boundary gap and completed the local diagnostic classification without changing product threshold semantics.

The exact 4 MiB case remained on the qsc legacy in-message path. qsl-attachments was not used for the exact case, and no exact-case qsl-attachments storage object was created. Expanded local diagnostic budget showed the legacy send reaches the full data-chunk class and then fails at the manifest push with a qsl-server relay queue-depth boundary. The result classification is `LOCAL_ATTACHMENT_EXACT_4MIB_QSL_SERVER_RELAY_BUG_FOUND`.

The selected successor is `NA-0598 -- QSL qsl-server Exact 4 MiB Relay Boundary Fix Harness`. The successor is required because the supported fix would cross NA-0597's product-source boundary for qsl-server relay behavior.

## qwork Proof Verification

Fresh qwork proof from `2026-07-03T20:12:19Z` was copied and verified before fetch, local runtime work, repository mutation, source-analysis publication, or proof publication. The proof matched lane NA-0597, repo qsl-protocol, the expected lane workspace, branch main, upstream origin/main, clean worktree/index/untracked state, READY_COUNT 1, READY NA-0597, shared cargo target mode, and shared target readiness.

Live pre-fetch `HEAD` and `origin/main` matched `4de013ef4275`. Root disk usage was below the stop threshold and `/backup/qsl` was mounted. Codex did not run qwork, qstart, or qresume.

## D-1183 / D-1184 Inheritance

D-1183 and D-1184 were each present once and Accepted. D-1183 classified NA-0596 as `LOCAL_ATTACHMENT_EXACT_4MIB_HARNESS_TIMEOUT_BUDGET_GAP` and selected NA-0597. D-1184 restored NA-0597 as READY. NA-0596 and NA-0595 were DONE before this lane.

Inherited facts were revalidated: exact 4 MiB entered the legacy in-message path, qsl-attachments interaction was `not_used`, the NA-0596 timeout phase was qsc send, just-over-4 MiB and above-threshold controls passed through qsl-attachments, and no qsc/qsl-server/qsl-attachments source mutation was inherited.

## Authority Model Application

The lane used Tier 0 read-only analysis, Tier 1 proof-root harness repair authority, Tier 4 local runtime diagnostics, and issue investigation for classification. No Tier 3 product source change was applied.

qsl-server and qsl-attachments were acquired, built, audited, tested, and run locally only. Services bound locally for diagnostic use with proof-root state and storage. qsc state, fixtures, and raw logs remained proof-root-only. No remote action, SSH, scp, Tailscale action, workflow dispatch, workflow rerun, sudo, systemd, firewall, dependency mutation, lockfile mutation, qwork, qstart, or qresume action occurred.

## Threshold Policy Revalidation

qsc source review revalidated the attachment policy: less than 4 MiB and exactly 4 MiB are legacy in-message cases; greater than 4 MiB uses the qsl-attachments path when configured. The comparator remains strict greater-than 4 MiB for the attachment path.

The diagnostic did not treat 5 MiB as the normative threshold. The above-threshold classes were controls only.

## qsc Exact 4 MiB Source Review

qsc review covered the threshold constant, comparator, legacy in-message path, qsl-attachments path entry, attachment-service configured behavior, attachment-service absent behavior, exact 4 MiB fixture handling, send timeout handling, receive/pull behavior, path-selection diagnostics, and seed fallback hardening.

The review confirmed exact 4 MiB is legacy-sized. The exact diagnostic kept qsl-attachments activation out of the exact case. Seed fallback remained blocked by default and was not used.

## qsl-server Exact 4 MiB Relay Review

qsl-server review confirmed local relay/control-plane behavior for opaque qsc messages. The relay does not need plaintext and did not show plaintext exposure in the diagnostic.

The relevant boundary is relay queue depth for legacy in-message transfer. Exact 4 MiB legacy send maps to the full data-chunk class and then requires a manifest message. The maximum bounded diagnostic attempt reached the data-chunk boundary and then failed at the manifest push with the relay queue full class.

## qsl-attachments Exact 4 MiB Non-Use Review

qsl-attachments review confirmed it is expected for above-threshold controls and not expected for exact 4 MiB. Runtime storage deltas, audit/log classes, and qsc markers all supported exact-case non-use.

No exact-case qsl-attachments storage object was created. The just-over-4 MiB and known-good greater-than-4-MiB controls still used qsl-attachments as expected.

## Local Build / Audit / Test Readiness

qsl-server metadata, audit, fmt, serial tests, and build passed after a recovered parallel log-capture validation issue. The recovery was diagnostic-only and did not mutate qsl-server.

qsl-attachments metadata, audit, fmt, tests, and build passed on main with the required PR #38 recovery commit present.

qsc focused validation passed for true triple-ratchet path, seed fallback hardening, attachment contract/streaming coverage, qsp/relay coverage, receive peer separation, and same-host send/receive coverage. An initially overbroad qsc attachment test invocation was terminated after the relevant coverage passed and replaced with narrower focused invocations.

## Completion Diagnostic Harness Design

The proof-root harness used local-only qsl-server and qsl-attachments processes, proof-root storage/state, synthetic exact-size fixtures, class-only summaries, bounded budget escalation for exact 4 MiB, and cleanup of owned processes.

The static review verified no public bind requirement, no secret environment dependency, no raw command-line publication, no seed fallback use, no deletion outside proof root, no qwork/qstart/qresume invocation, no sudo/systemctl/firewall/Tailscale operation, and no qsl-attachments use for exact 4 MiB except as an unexpected-use detector.

## Control Baseline

Control baseline classification: `CONTROL_BASELINE_PASS`.

The just-over-4 MiB control used the qsl-attachments path and completed send/receive/decrypt validation. The known-good greater-than-4-MiB control also used qsl-attachments and completed send/receive/decrypt validation. The under-4-MiB control entered the legacy in-message path, did not use qsl-attachments, and remained a bounded legacy-send resource-control case rather than an attachment-path regression.

## Exact 4 MiB Budget Completion Probe

Exact 4 MiB completion classification: `EXACT_4MIB_FAILS_WITH_PRODUCT_ERROR`.

The prior-budget-equivalent attempt stayed on the legacy path, did not use qsl-attachments, and timed out during qsc send after partial chunk progress. The expanded bounded attempt stayed on the same path and timed out after greater partial chunk progress. The maximum bounded local diagnostic attempt stayed on the legacy path, did not use qsl-attachments, reached the full data-chunk class, and then failed at the manifest push with relay queue full classification.

This sequence clarifies that the prior timeout was not a threshold comparator issue, not a qsl-attachments path-selection issue, and not fixture-generation overhead.

## qsc Send Phase Classification

qsc send phase classification: `QSC_SEND_PHASE_RELAY_QUEUE_FULL_AFTER_ALL_CHUNKS`.

The qsc command shape was valid. Exact 4 MiB entered legacy send. The maximum bounded attempt progressed through the complete data-chunk class and failed before qsc send completion because the qsl-server relay queue was full when the manifest message was pushed.

## qsl-server Phase Classification

qsl-server phase classification: `QSL_SERVER_PHASE_QUEUE_DEPTH_BOUNDARY_FOUND`.

qsl-server accepted opaque relay data up to the queue-depth boundary. The failure is supported as a local relay resource-boundary issue for exact 4 MiB legacy transfer shape. qsl-server plaintext exposure was not observed.

## qsl-attachments Non-Use Classification

qsl-attachments exact-case interaction classification: `not_used`.

The exact case showed no qsl-attachments storage object creation and no qsl-attachments use markers. Above-threshold controls used qsl-attachments as expected, which preserves the working greater-than-4 MiB attachment path.

## Legacy Path Completion / Receive Validation

Legacy completion classification: `EXACT_4MIB_LEGACY_SEND_QUEUE_FULL_BEFORE_RECEIVE`.

The exact 4 MiB path was legacy in-message and qsl-attachments remained not used. Receive/pull and decrypt/validate were not reached because qsc send failed before publishing the manifest.

## Failure-Cause Completion Review

Failure-cause classification: `EXACT_4MIB_CAUSE_PRODUCT_BUG_SUPPORTED`.

Required cause fields:

- qsc command-shape issue: no.
- legacy in-message path entered: yes.
- attachment path entered unexpectedly: no.
- qsl-attachments used unexpectedly: no.
- fixture-generation overhead: no.
- qsc pack or envelope overhead: yes, as part of normal legacy message volume rather than a command-shape error.
- qsc send process timeout: yes for prior and expanded budgets; no for the maximum bounded attempt.
- qsl-server push timeout: no.
- qsl-server response timeout: no.
- qsc receive/pull timeout: not reached.
- decrypt/validate timeout: not reached.
- harness wait budget too low: yes for the first two budget classes; no for the maximum bounded diagnostic.
- resource pressure: none.
- diagnostic visibility gap: no.
- product bug supported: yes, localized to qsl-server relay queue-depth behavior for exact legacy manifest publication.

## Metadata / Private-Material Review

Metadata review classification: `EXACT_4MIB_METADATA_PRIVATE_MATERIAL_REVIEW_PASS`.

Published evidence is class-only. Endpoint values, private port values, route-token values, Authorization values, capability values, raw command lines, payload/body/plaintext bytes, ciphertext bodies, seed values, key material, process identities, and exact storage paths remain proof-root-only.

## Cleanup / Rollback

Cleanup classification: `EXACT_4MIB_COMPLETION_CLEANUP_DONE`.

Owned qsl-server and qsl-attachments diagnostic processes were stopped. No unknown owned listener remained. Proof-root artifacts were retained for evidence; no repository rollback was required because no product source mutation was applied.

## Issue Investigation and Safe Fix

Issue investigation classification: `EXACT_4MIB_COMPLETION_FAILURE_QSL_SERVER_RELAY_QUEUE_BOUNDARY`.

No safe fix was applied in NA-0597. The supported corrective area is qsl-server relay behavior, which is outside NA-0597 implementation authority because product source and relay semantics were not authorized for mutation. The selected successor carries the exact qsl-server fix scope.

## Result Classification

`LOCAL_ATTACHMENT_EXACT_4MIB_QSL_SERVER_RELAY_BUG_FOUND`

## Selected Successor

NA-0598 is selected as `QSL qsl-server Exact 4 MiB Relay Boundary Fix Harness`.

Objective:

Fix the artifact-backed qsl-server local relay behavior affecting the exact 4 MiB legacy boundary. Codex may mutate exact D-1185-selected qsl-server source/test paths and create a qsl-server PR. Stop before route/auth/storage/protocol semantic changes or dependency/lockfile mutation unless explicitly authorized.

## Required-Check Boundary

Current main was classified before implementation with public-safety success, advisories success, suite2-vectors success, no failed attached check-runs, and no pending attached check-runs. Root cargo audit, nested qsc fuzz cargo audit, locked cargo metadata, and cargo drift checks passed before mutation.

## Source / Script Mutation Boundary

No qsc source, qsc test, qsc example, qsl-protocol script, dependency, lockfile, workflow, public-site, formal, refimpl, qshield, qshield-cli, or backup path was mutated for the diagnostic.

## qsc Boundary

qsc source was reviewed and not mutated. qsc threshold/path-selection semantics remain unchanged: exact 4 MiB is legacy in-message, and greater than 4 MiB uses qsl-attachments when configured.

## qsl-server Boundary

qsl-server source was reviewed, built, audited, tested, and run locally only. qsl-server source was not mutated. The supported fix is deferred to NA-0598 because NA-0597 did not authorize qsl-server product source or relay semantic changes.

## qsl-attachments Boundary

qsl-attachments source was reviewed, built, audited, tested, and run locally only. qsl-attachments source was not mutated. The working above-threshold qsl-attachments path was not disturbed.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, GitHub workflow dispatch, workflow rerun, public-network action, deployment action, or GitHub runner action occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, docs/public, Cloudflare, public DNS, or public deployment path was mutated.

## Evidence / Decision / Traceability

D-1185 records the NA-0597 result classification and selected successor. `TRACEABILITY.md` maps the diagnostic to this evidence document and testplan. The rolling operations journal records qwork proof, local validation, recoveries, diagnostic classification, selected successor, and boundaries.

## Validation

Validation covered diff hygiene, scope guard, queue/decision proof, marker proof, markdown link check, private-material scans, overclaim scans, dependency/main-health checks, shell syntax checks, qsl-server validation, qsl-attachments validation, focused qsc validation, and PR body preflight.

Recovered failures were limited to proof parser shape, optional missing example path discovery, qsl-server parallel log-capture validation, overbroad qsc attachment test invocation, static-review classifier shape, and a self-matching progress-probe cleanup command. Each recovery stayed inside the allowed diagnostic/proof boundary and ended with a passing replacement or a safe classification.

## Recommendation

Merge the NA-0597 governance PR after required checks pass, then close out NA-0597 by restoring NA-0598 with the selected qsl-server relay boundary fix scope. Do not implement NA-0598 in the closeout PR.
