Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0594 local qsl-attachments send / receive integration harness

## Executive Summary

NA-0594 consumed D-1177 and D-1178, verified fresh qwork proof from
`2026-07-02T20:21:14Z`, revalidated qsc, qsl-server, and qsl-attachments
surfaces, built and tested the local service stack, and ran a proof-root-only
local attachment-bearing qsc exchange through qsl-server and qsl-attachments.

Result classification:
`LOCAL_QSL_ATTACHMENTS_SEND_RECEIVE_INTEGRATION_PASS_WITH_METADATA_LIMITS`.

The proof remains local-only. qsc owns attachment encryption/decryption and
descriptor verification; qsl-server remains relay/control-plane only; and
qsl-attachments stores opaque object bytes only. Raw local endpoints, private
ports, route tokens, Authorization material, capabilities, command lines,
fixture bytes, full hashes, logs, and qsl-attachments storage paths remain
proof-root-only and are not published here.

## qwork Proof Verification

- qwork proof files were copied from the NA-0594 workspace before fetch,
  qsl-server/qsl-attachments acquisition, builds, runtime startup, qsc
  execution, repository mutation, or proof publication.
- qwork startup result: OK.
- lane: NA-0594.
- repo/path: qsl-protocol at `/srv/qbuild/work/NA-0594/qsl-protocol`.
- proof timestamp: `2026-07-02T20:21:14Z`.
- startup `HEAD`, `origin/main`, and `main`: `d124e734f67e`.
- startup worktree, index, and untracked state: clean.
- READY_COUNT: 1.
- READY item: NA-0594.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.
- Disk and mount gates passed; `/backup/qsl` was mounted and root usage was
  below the stop threshold.

## D-1177 / D-1178 Inheritance

- D-1177 exists once and is Accepted.
- D-1178 exists once and is Accepted.
- NA-0593 is DONE.
- NA-0594 is READY.
- D-1179 and D-1180 were absent before this patch.
- D-1177 result
  `SEED_FALLBACK_HARDENING_IMPLEMENTATION_PASS_ATTACHMENT_DEFERRED` was
  consumed.
- D-1177 selected NA-0594 and D-1178 restored NA-0594.
- qsc seed fallback hardening was inherited: old env-only fallback is blocked
  in default qsc send, receive, relay, and attachment descriptor paths; retained
  deterministic fixture behavior requires the explicit unsafe diagnostic/test
  gate.
- No qsl-server or qsl-attachments mutation was inherited.

## Authority Model Application

NA-0594 used Tier 0 read-only source review, Tier 1 proof-root harness tooling,
Tier 2 validation diagnostics, and Tier 4 local build-server runtime actions.
No qsl-protocol/qsc source fix, qsl-server source fix, qsl-attachments source
fix, dependency change, lockfile change, workflow change, remote action, or
privileged action was required.

## qsc Attachment Command Surface Revalidation

- qsc exposes `file send` with relay transport, relay URL, and explicit
  attachment-service option.
- The above-threshold attachment path is selected for files larger than 4 MiB
  when an attachment service is supplied.
- Exact 4 MiB remains legacy-sized and does not silently use the attachment
  service path.
- Missing attachment service for an above-threshold file fails closed.
- qsc encrypts attachment parts locally, commits opaque object bytes through
  qsl-attachments, sends an opaque descriptor/envelope through qsl-server, and
  verifies/fetches/decrypts locally on receive.
- Seed fallback env-only behavior remains blocked for the attachment descriptor
  path.

## qsl-server Relay Boundary Revalidation

- qsl-server local startup supports loopback binding.
- Canonical relay routes are push and pull with a header-carried route token.
- qsl-server stores/relays opaque bytes only and has no attachment storage role.
- qsl-server logging reviewed for this lane is metadata-only and does not log
  payload bodies or attachment plaintext.

## qsl-attachments Runtime Surface Revalidation

- qsl-attachments starts as a local loopback service with a configurable storage
  root.
- Runtime APIs cover session creation, part upload, commit, abort/status, and
  object fetch.
- Resume tokens and fetch capabilities are resource-scoped and proof-root-only
  in this lane.
- qsl-attachments stores opaque ciphertext/object bytes and JSON metadata; it
  does not own qsc plaintext or qsc key material.
- Size, retention, TTL, cleanup, recovery, invalid-secret, invalid-range, and
  missing-object behavior were revalidated from source/tests and the local
  negative harness.

## Local Build / Audit / Test Readiness

- qsl-server at `6bf61d439fa2`: metadata, audit, fmt, test, and build passed.
- qsl-attachments at `767eca189ee`: metadata, audit, fmt, test, and build
  passed; PR #38 recovery commit was present.
- qsc focused validation passed:
  - `cargo test -p qsc --test na_0591_true_triple_ratchet_path`
  - `cargo test -p qsc --test na_0593_seed_fallback_hardening`
  - `cargo test -p qsc --test attachments_contract_na0217h`
  - `cargo test -p qsc --test attachment_streaming_na0197c`
  - `cargo test -p qsc --test qsp_protocol_gate`
  - `cargo test -p qsc --test relay_auth_header`
  - `cargo test -p qsc --test receive_e2e_peer_separation`

## Integration Harness Design

The generated harness is proof-root-only. It uses subprocess argument arrays,
not shell execution; binds qsl-server and qsl-attachments to loopback; stores
qsc state, qsl-server logs, qsl-attachments logs, and qsl-attachments storage
under the proof root; removes qsc seed-fallback env variables for qsc
send/receive; emits only class-level terminal output; and writes raw values only
to proof-root private artifacts.

Static review passed: no public bind, no secret env dependency for publication,
no endpoint/private-port publication in class summaries, no payload/body/plaintext
publication, no seed/key publication, no qwork/qstart/qresume, no sudo/systemd/
firewall/Tailscale, and no deletion outside proof-root-owned runtime subtrees.

## Local Runtime Startup

qsl-server and qsl-attachments started locally on loopback with proof-root logs
and proof-root qsl-attachments storage. Readiness checks classified:

- `LOCAL_RUNTIME_QSL_SERVER_READY`
- `LOCAL_RUNTIME_QSL_ATTACHMENTS_READY`

No public bind was detected. Owned processes were tracked by the harness.

## Local qsl-attachments Send / Receive Integration

The harness created a non-secret synthetic attachment fixture larger than 4 MiB
and ran a qsc sender/receiver exchange through the local qsl-server relay and
local qsl-attachments service.

Classification:
`LOCAL_QSL_ATTACHMENTS_SEND_RECEIVE_PASS`.

Observed class-level markers:

- qsc send committed through the attachment service and was accepted by the
  relay.
- qsc receive unpacked the qsp envelope, advanced the receive ratchet, fetched
  the attachment object, verified/decrypted locally, and validated the received
  fixture class.
- Empty pull after drain was classified fail-closed.
- Seed fallback markers were absent from send/receive output.

## Boundary Verification

- `BOUNDARY_QSC_OWNS_ENCRYPTION_CONFIRMED`
- `BOUNDARY_QSL_SERVER_CONTROL_PLANE_ONLY_CONFIRMED`
- `BOUNDARY_QSL_ATTACHMENTS_OPAQUE_STORAGE_CONFIRMED`
- `BOUNDARY_SEED_FALLBACK_BLOCKED_IN_ATTACHMENT_PATH`

The harness scanned qsl-server logs and qsl-attachments storage for the private
fixture marker and found zero hits. Key-material exposure was not found.

## Selected Negative Tests

Selected negatives were classified as follows:

- above-threshold send without attachment service: pass_fail_closed.
- wrong route token receive: pass_fail_closed.
- wrong fetch capability: pass_fail_closed.
- corrupted descriptor/envelope: pass_fail_closed.
- corrupted fetched ciphertext: pass_fail_closed.
- missing object: pass_fail_closed.
- exact 4 MiB boundary: pass_fail_closed; it remained legacy-sized and did not
  use the attachment-service commit path.
- missing capability descriptor: not_supported by the current command surface.
- wrong bearer: not_configured for this local run.
- wrong peer: not_supported by the selected harness surface.
- expired/deleted object: covered by missing-object behavior.

No critical fail-open was found.

## Metadata Minimization Review

Residual metadata is classified and acceptable for the next stress lane:

- filename, exact size, object/capability values, route/recipient values,
  timing, command lines, and storage paths: proof_root_only.
- size bucket: class_only.
- MIME/content-type: not_applicable.
- payload/body, plaintext, key material, and seed fallback exposure: no.

## Cleanup / Rollback

Cleanup classification:
`LOCAL_ATTACHMENT_CLEANUP_DONE`.

Owned qsl-server and qsl-attachments processes were stopped. Proof-root storage
is retained only as evidence. No unknown owned listener remained.

## Issue Investigation and Safe Fix

Issue investigation executed for the first harness run. The exact 4 MiB
negative correctly failed closed as a legacy-sized size-limit rejection, but the
first proof-root harness classifier treated the nonzero command as ambiguous.
This was classified as a proof-root harness classifier issue. The proof-root
classifier was repaired, and the full harness was rerun successfully.

No repository source fix, qsc source fix, qsl-server PR, qsl-attachments PR,
dependency change, lockfile change, or semantic change was required.

## Private-Material Review

Private-material scan classification:
`PRIVATE_MATERIAL_SCAN_PASS_FOR_PUBLISHABLE_SUMMARIES`.

No endpoint value, private port value, route token, bearer value,
Authorization value, capability value, payload/body/plaintext, seed value, key
material, raw command line, raw log, process identity, qsl-attachments storage
path, or private topology is published in this evidence record.

## Result Classification

`LOCAL_QSL_ATTACHMENTS_SEND_RECEIVE_INTEGRATION_PASS_WITH_METADATA_LIMITS`.

## Selected Successor

Selected successor:
`NA-0595 -- QSL Local qsl-attachments Adversarial and Metadata Stress Harness`.

## Required-Check Boundary

Startup main check classification passed before implementation work:
public-safety success, advisories success, suite2-vectors success or satisfied,
no failed required checks, and no pending required checks in the attached
check-run set.

## Source / Script Mutation Boundary

This implementation patch mutates governance/evidence/testplan/trace/journal
files only. Proof-root harness scripts and raw artifacts remain outside the
repository. No qsc source/test/script, qsl-server source, qsl-attachments
source, Cargo manifest, lockfile, workflow, public-site, formal, refimpl,
qshield, qshield-cli, qwork, qstart, or qresume path is changed.

## qsc Boundary

qsc was executed locally with proof-root state and with seed-fallback env
variables removed. qsc retained encryption, descriptor construction, descriptor
verification, fetch verification, local decrypt, and fixture validation
ownership.

## qsl-server Boundary

qsl-server was built, started locally, used as a relay/control-plane boundary,
and stopped. It was not mutated and did not store attachment bytes or expose
plaintext.

## qsl-attachments Boundary

qsl-attachments was built, started locally with proof-root storage, used for
opaque object storage/fetch, and stopped. It was not mutated and did not receive
qsc plaintext or qsc key material.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, GitHub workflow dispatch,
workflow rerun, deployment, service/cloud action, sudo, systemd, or firewall
action occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, docs/public, public, or Cloudflare mutation occurred.

## Evidence / Decision / Traceability

This evidence doc records NA-0594. D-1179 records the accepted local integration
decision. TRACEABILITY maps NA-0594 to this evidence, the testplan, validation,
runtime proof, boundaries, and selected NA-0595 successor. The rolling
operations journal records proof gates, recoveries, validation, classification,
and boundaries.

## Validation

Validation includes qwork proof verification, queue/decision proof, current-main
check classification, D-1177/D-1178 inheritance, qsc/qsl-server/qsl-attachments
source-surface reviews, qsl-server validation, qsl-attachments validation, qsc
focused validation, proof-root harness static review, local runtime startup,
local send/receive integration, boundary scans, selected negative tests,
metadata review, cleanup verification, private-material scans, and result
classification.

## Recommendation

Merge NA-0594 after required PR checks pass. After merge and post-merge check
proof, close out NA-0594 and restore the selected NA-0595 adversarial and
metadata stress successor only if the post-merge gates remain healthy.
