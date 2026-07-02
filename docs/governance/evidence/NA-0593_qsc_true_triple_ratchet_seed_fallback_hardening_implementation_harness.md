Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0593 qsc true triple-ratchet seed fallback hardening implementation harness

## Executive Summary

NA-0593 consumed D-1175 and D-1176, verified fresh qwork proof from
`2026-07-02T17:18:57Z`, refreshed the qsc seed fallback source map, implemented
the selected hardening, and classified the result as
`SEED_FALLBACK_HARDENING_IMPLEMENTATION_PASS_ATTACHMENT_DEFERRED`.

The old env-only fallback is blocked from ordinary qsc send, receive, relay, and
attachment descriptor paths. Retained deterministic fixture behavior now
requires the explicit unsafe diagnostic/test gate
`QSC_UNSAFE_TEST_SEED_FALLBACK=1` in addition to the legacy fallback env. Full
qsl-attachments send/receive integration remains deferred to NA-0594.

## qwork Proof Verification

- qwork proof files were copied from `/srv/qbuild/work/NA-0593/.qwork/` before
  fetch, source inspection publication, qsc tests, repository mutation, GitHub
  metadata review, or proof publication.
- qwork startup result: OK.
- lane: NA-0593.
- repo/path: qsl-protocol at `/srv/qbuild/work/NA-0593/qsl-protocol`.
- proof timestamp: `2026-07-02T17:18:57Z`.
- startup `HEAD`, `origin/main`, and `main`: `e1f9b3eb2204`.
- startup worktree, index, and untracked state: clean.
- READY_COUNT: 1.
- READY item: NA-0593.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.
- Disk and mount gates passed; `/backup/qsl` was mounted and root usage was
  below the stop threshold.

## D-1175 / D-1176 Inheritance

- D-1175 exists once and is Accepted.
- D-1176 exists once and is Accepted.
- NA-0592 is DONE.
- NA-0593 is READY.
- D-1177 and D-1178 were absent before this patch.
- D-1175 classified the fallback as
  `SEED_FALLBACK_RUNTIME_PRODUCTION_PATH_RISK`.
- D-1175 selected the B/E hybrid strategy: explicit unsafe test/diagnostic
  gating, helper renaming/isolation, default fail-closed behavior, fixture/docs
  updates, and no-seed path preservation.
- Full qsl-attachments integration remained deferred.

## Authority Model Application

NA-0593 used Tier 0 read-only review, Tier 1 proof-root tooling, Tier 2 qsc
hardening implementation, and Tier 4 focused local qsc tests. It did not use
Tier 3 semantic hardening authority.

## Seed Fallback Source Map Refresh

- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`: fallback gate, deterministic
  session-store fallback key, deterministic `Suite2SessionState` synthesis, and
  protocol-active gate.
- `qsl/qsl-client/qsc/src/main.rs`: startup unlock path and qsp pack/unpack
  helpers that call `qsp_session_for_channel`.
- `qsl/qsl-client/qsc/src/transport/mod.rs`: ordinary relay send/receive and
  TUI relay paths.
- `qsl/qsl-client/qsc/src/attachments/mod.rs`: qsc attachment descriptor send
  path before qsp pack.
- `scripts/demo/qsc_remote_relay_smoke.sh`: retained diagnostic relay smoke
  mode.
- `scripts/demo/qsc_remote_handshake_smoke.sh`: no-seed handshake smoke lane.
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`: qsc
  remote relay/handshake contract.

## Selected Mutation Paths

Selected qsc implementation paths were recorded in
`selected_paths/selected_mutation_paths.json` and
`safe_fix/selected_fix_paths.json` before editing. The selected paths are qsc
source, qsc tests, qsc demo scripts, and the qsc remote relay testing contract.
No selected path is outside NA-0593 scope.

## Hardening Implementation

- Renamed the retained gate to `allow_unsafe_seed_fallback_for_tests`.
- Made `QSC_ALLOW_SEED_FALLBACK=1` insufficient by default.
- Required `QSC_UNSAFE_TEST_SEED_FALLBACK=1` before deterministic fixture
  fallback can satisfy protocol-active checks or synthesize deterministic qsp
  state.
- Updated existing qsc fixture tests to opt into the unsafe gate where they
  intentionally use deterministic fallback.
- Added `qsl/qsl-client/qsc/tests/na_0593_seed_fallback_hardening.rs`.
- Updated remote relay smoke markers to
  `protocol_mode=unsafe_seed_fallback_diagnostic`.
- Redacted the deterministic fixture seed in remote relay marker/summary output.
- Updated remote handshake smoke to clear both fallback envs.

## Default Runtime Fallback Behavior

Default qsc send/receive behavior now fails closed when no authenticated stored
session exists, even if `QSC_QSP_SEED` and `QSC_ALLOW_SEED_FALLBACK=1` are
present. No qsp pack/unpack success marker is emitted for the old env-only path.

## qsc Send / Receive Boundary

Focused NA-0593 tests prove ordinary send and receive reject old env-only
fallback before qsp pack, relay pull, qsp unpack, outbox creation, or receive
commit.

## qsc Attachment Descriptor Boundary

Focused NA-0593 tests prove the qsc attachment descriptor path reaches the
attachment policy surface and then rejects old env-only fallback with
`protocol_inactive reason=no_session` before descriptor construction, qsp pack,
or attachment-service commit.

## No-Seed Dynamic Path Preservation

Classification:
`NO_SEED_DYNAMIC_PATH_NOT_RUN_SOURCE_TESTS_SUFFICIENT`.

`cargo test -p qsc --test na_0591_true_triple_ratchet_path` passed after the
hardening. The implementation only restricts deterministic fallback after stored
session loading fails; the no-seed qsp pack/unpack and Suite2 send/receive path
remains unchanged.

## Focused Test Results

- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`: PASS.
- `cargo test -p qsc --test na_0593_seed_fallback_hardening`: PASS.
- `cargo test -p qsc --test qsp_protocol_gate`: PASS.
- `cargo test -p qsc --test qsp_status_truthy`: PASS.
- `cargo test -p qsc --test relay_auth_header`: PASS.
- `cargo test -p qsc --test receive_e2e receive_mailbox_peer_separation_fail_closed`: PASS.
- `cargo test -p qsc --tests --no-run`: PASS.

## Metadata / Key / Logging Review

- qsc runtime output reviewed by the focused tests does not expose fallback env
  names, seed values, key material, Authorization material, or bearer material.
- Remote relay diagnostic output now redacts the deterministic fixture seed.
- Attachment descriptor output keeps redaction of sensitive path-like fields.

## Private-Material Review

No endpoint values, private ports, token values, Authorization values,
payload/body/plaintext bytes, seed values, key material, raw logs, command
lines, process identities, or private topology are published in this evidence
record.

## Result Classification

`SEED_FALLBACK_HARDENING_IMPLEMENTATION_PASS_ATTACHMENT_DEFERRED`.

## Selected Successor

Selected successor:
`NA-0594 -- QSL Local qsl-attachments Send / Receive Integration Harness`.

## Required-Check Boundary

Startup main check classification passed: public-safety success, advisories
success, suite2-vectors success or satisfied, no failed checks, and no pending
checks in the attached check-run set.

## Source / Script Mutation Boundary

Source/script mutations are limited to selected qsc source, qsc tests, qsc demo
scripts, and qsc docs. No workflow, dependency, lockfile, qwork/qstart/qresume,
public-site, docs/public, formal, refimpl, qshield, or qshield-cli mutation is
included.

## qsc Boundary

qsc source and tests were changed only to harden the seed fallback fixture
surface and preserve focused coverage.

## qsl-server Boundary

qsl-server was not mutated, started, stopped, deployed, or integrated.

## qsl-attachments Boundary

qsl-attachments was not mutated and no full qsl-attachments send/receive
integration was run.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, GitHub workflow dispatch, or
workflow rerun occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, docs/public, public, or Cloudflare mutation occurred.

## Evidence / Decision / Traceability

This evidence doc records NA-0593. D-1177 records the accepted implementation.
TRACEABILITY maps NA-0593 to the qsc source/tests and selected successor. The
rolling operations journal records proof gates, recoveries, validation, and
boundaries.

## Validation

Validation includes qwork proof verification, startup queue/decision proof,
current-main check classification, selected path proof, source-map refresh,
focused qsc tests, qsc integration-test compile check, fixture pairing check,
private-material review, and claim-boundary review.

## Recommendation

Merge NA-0593 after PR checks pass, then close out NA-0593 and restore NA-0594
only after post-merge public-safety/advisories and required-check gates are
healthy.
