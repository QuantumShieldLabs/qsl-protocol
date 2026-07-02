Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0592 qsc true triple-ratchet E2EE hardening / bug-fix authorization harness

## Executive Summary

NA-0592 consumed D-1173 and D-1174, verified fresh qwork proof from
`2026-07-02T15:58:53Z`, mapped the qsc seed fallback surface, classified it as
`SEED_FALLBACK_RUNTIME_PRODUCTION_PATH_RISK`, selected a fail-closed hardening
strategy, and selected the exact NA-0593 implementation successor.

The fallback is not test-only. It is compiled into the normal qsc binary and can
affect ordinary qsc send, receive, TUI relay, and qsc file/attachment descriptor
command paths when explicit environment variables are present. It bypasses the
authenticated handshake-bound session state by synthesizing deterministic Suite2
state. It still invokes Suite2 send/receive primitives, but that is not a valid
authenticated true triple-ratchet path.

Selected result: `SEED_FALLBACK_HARDENING_IMPLEMENTATION_READY`.

Selected successor: `NA-0593 -- QSL qsc True Triple-Ratchet Seed Fallback
Hardening Implementation Harness`.

## qwork Proof Verification

- qwork proof files were copied from `/srv/qbuild/work/NA-0592/.qwork/` into
  the proof root before fetch, source-result publication, qsc testing, GitHub
  metadata review, repository mutation, or proof publication.
- qwork startup result: OK.
- lane: NA-0592.
- repo/path: qsl-protocol at `/srv/qbuild/work/NA-0592/qsl-protocol`.
- proof timestamp: `2026-07-02T15:58:53Z`.
- startup `HEAD`, `origin/main`, and `main`: `5d79fc682098`.
- startup worktree, index, and untracked state: clean.
- READY_COUNT: 1.
- READY item: NA-0592.
- shared cargo target: ready.
- Codex did not run qwork, qstart, or qresume.
- Disk and mount gates passed; `/backup/qsl` was mounted and root usage was
  below the stop threshold.

## D-1173 / D-1174 Inheritance

- D-1173 exists once and is Accepted.
- D-1174 exists once and is Accepted.
- D-1175 and D-1176 were absent before this patch.
- NA-0591 is DONE.
- NA-0592 is READY.
- D-1173 records `TRUE_TRIPLE_RATCHET_DEMO_OR_FIXTURE_BYPASS_FOUND`.
- D-1173 selected NA-0592 as the exact hardening authorization successor.
- D-1174 restored NA-0592 as the sole READY item.
- No full qsl-attachments integration occurred in NA-0591 or closeout.
- No qsl-attachments mutation occurred.
- No qsl-server mutation occurred in closeout.
- No crypto-complete or triple-ratchet-complete claim was inherited.

## Authority Model Application

NA-0592 used Tier 0 read-only source/evidence analysis, Tier 1 proof-root
tooling, and focused local qsc tests. No qsc source/test/demo mutation was
needed under the narrow exception. No runtime crypto/protocol/wire/auth/
key-schedule/identity/transcript/state-machine mutation was performed.

## Seed Fallback Source Inventory

Primary runtime surfaces:

- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`: fallback implementation,
  environment parsing, deterministic session-store fallback key, deterministic
  `Suite2SessionState` synthesis, and protocol-active bypass gate.
- `qsl/qsl-client/qsc/src/main.rs`: startup unlock integration and qsp
  pack/unpack helpers that call `qsp_session_for_channel`.
- `qsl/qsl-client/qsc/src/transport/mod.rs`: ordinary send and receive command
  paths call the active-session gate and qsp pack/unpack.
- `qsl/qsl-client/qsc/src/attachments/mod.rs`: qsc file/attachment descriptor
  send calls the active-session gate and then sends the descriptor through the
  qsc message plane.
- `scripts/demo/qsc_remote_relay_smoke.sh`: explicitly enables and labels
  `seed_fallback_test`.
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`:
  documents the remote relay smoke lane as seed-fallback test mode and separates
  the remote handshake lane as no-seed.

The optional `qsl/qsl-client/qsc/examples` path does not exist in this checkout.
That was recorded as a recoverable discovery result.

## Seed Fallback Call Graph

Ordinary send:

`qsc send` -> main `Cmd::Send` -> `transport::send_execute` ->
`protocol_active_or_reason_for_send_peer` ->
`protocol_active_or_reason_for_peer` -> seed fallback active gate when explicitly
enabled -> `transport::relay_send_with_payload` -> `qsp_pack` ->
`qsp_session_for_channel` -> deterministic Suite2 session state if no stored
session exists -> Suite2 `send_wire`.

Ordinary receive:

`qsc receive` -> main `Cmd::Receive` -> `transport::receive_execute` ->
`protocol_active_or_reason_for_peer` -> seed fallback active gate when
explicitly enabled -> `receive_pull_and_write` -> `qsp_unpack_for_peer` ->
`qsp_unpack` -> `qsp_session_for_channel` -> deterministic Suite2 session state
if no stored session exists -> Suite2 `recv_wire`.

TUI send/receive:

TUI relay commands use the same active-session gate and the same
`relay_send_with_payload` / `receive_pull_and_write` qsp path.

Attachment descriptor send:

`qsc file send` with the attachment-service path -> `attachment_send_execute` ->
`protocol_active_or_reason_for_send_peer` -> `relay_send_with_payload` ->
`qsp_pack` -> seed-derived session state when the fallback is enabled. Full
qsl-attachments integration was not run.

## Runtime Reachability Classification

- ordinary_qsc_send_reachable: yes.
- ordinary_qsc_receive_reachable: yes.
- attachment_send_reachable: yes for the qsc attachment descriptor command path;
  full qsl-attachments integration remains deferred.
- requires_explicit_env: yes.
- requires_explicit_flag: no.
- compiled_into_normal_binary: yes.
- visible_in_help: no.
- visible_in_docs: yes.
- bypasses_handshake: yes.
- bypasses_true_triple_ratchet: yes for authenticated handshake-bound session
  assurance; it still invokes Suite2 primitives but not authenticated
  handshake-derived state.
- weakens_runtime_security_if_enabled: yes.
- logs_seed_or_key_material: yes for demo seed summaries; no key material
  logging was found in qsc source for this fallback.
- can_be_triggered_accidentally: yes through inherited environment.
- safe_to_leave_without_hardening: no.

## Primary Risk Classification

Primary class: `SEED_FALLBACK_RUNTIME_PRODUCTION_PATH_RISK`.

Reason: the fallback is compiled into the normal binary and can make ordinary
runtime command paths proceed without a stored authenticated session when
explicit environment variables are present.

## Best-Known-Method Review

Test fixture seed behavior should be unavailable in default production binaries
and retained only behind a loud test-only surface when needed. Demo behavior
must not be confused with authenticated E2EE evidence. Production CLI behavior
should fail closed when no authenticated session exists. Rollback must not
restore env-only fallback on main without a new authorization.

## Hostile Cryptographer Review

The fallback can be abused to make deterministic state too easy to invoke, to
confuse demo evidence with authenticated triple-ratchet evidence, and to bypass
identity/transcript/session binding. It risks contaminating future attachment
evidence if the qsc descriptor path inherits unsafe state.

## Red-Team Review

Risks include environment leakage into normal qsc runs, operators copying demo
scripts, hidden help-surface behavior, seed values in demo summaries, and tests
normalizing an unsafe command path as ordinary usage.

## Production SRE Review

The safe operational posture is default fail-closed behavior, explicit unsafe
test-only naming, no seed/key log output, and preserved deterministic fixtures
only through an explicit test harness or cfg surface.

## Side-Channel Caveat

NA-0592 does not evaluate or prove side-channel resistance.

## Formal Mapping Boundary

No formal proof-complete mapping exists for the seed fallback hardening. Future
implementation must not make a formal-proof-complete claim unless a later lane
adds that evidence explicitly.

## Release-Claim Boundary

This lane makes:

- no public-readiness claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-crypto claim;
- no crypto-complete claim;
- no attachment-complete claim;
- no side-channel-free claim;
- no formal-proof-complete claim;
- no triple-ratchet-complete claim;
- no external-review-complete claim.

## Hardening Option Review

Options reviewed:

- A: remove seed fallback entirely. Strongest safety; high fixture/demo rewrite
  cost.
- B: compile fallback only under an explicit unsafe test cfg. Selected.
- C: use a non-default Cargo feature. Not selected because this successor should
  avoid Cargo manifest, dependency, and lockfile changes.
- D: require explicit unsafe CLI flag plus env. Better than env-only but still
  leaves fallback in the default binary.
- E: move fallback into dedicated test helper surface. Selected with B.
- F: rename/relabel/docs only. Insufficient for a production-reachable fallback.
- G: preserve as-is. Rejected.
- H: crypto/protocol redesign. Not required for the selected hardening.

## Selected Hardening Strategy

Select a B/E hybrid:

- Gate the fallback behind an explicit unsafe test-only cfg seam, not a Cargo
  feature.
- Make default qsc builds ignore/reject the current env-only fallback and fail
  closed when no stored authenticated session exists.
- Move or rename retained deterministic fallback behavior as a loud test-only
  helper surface.
- Update tests and demo scripts to either use handshake-backed sessions or opt
  into the explicit test-only build path.

Crypto/protocol semantic redesign is not needed for the selected successor.

## Exact Implementation Plan

Future eligible mutation paths:

- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/transport/mod.rs`
- `qsl/qsl-client/qsc/src/attachments/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs` only if a secondary explicit unsafe CLI
  guard is selected during implementation.
- `qsl/qsl-client/qsc/tests/**`
- `scripts/demo/qsc_remote_relay_smoke.sh`
- `scripts/demo/qsc_demo_local.sh`
- `scripts/demo/qsc_remote_handshake_smoke.sh` only for regression proof or
  marker wording.
- `docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md`
- `docs/qsc/DOC-QSC-004_Demo_Full_Run_Addendum_v1.0.0_DRAFT.md`

Future forbidden paths:

- `qsl/qsl-client/qsc/fuzz/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `Cargo.toml`
- `Cargo.lock`
- `.github/workflows/**`
- `docs/public/**`
- `public/**`
- `website/**`
- `formal/**`
- `refimpl/**`
- `backup/**`
- qwork/qstart/qresume implementation paths.
- `qshield/**`
- `qshield-cli/**`

Required future tests:

- Default no-cfg qsc send rejects env-only fallback with protocol inactive or
  no-session behavior.
- Default no-cfg qsc receive rejects env-only fallback.
- Default no-cfg qsc file/attachment descriptor path rejects env-only fallback
  before descriptor qsp pack/send.
- NA-0591 no-seed handshake-backed send/receive still passes.
- Retained fallback fixture, if preserved, works only through explicit unsafe
  test cfg and loud env naming.
- No seed/key material appears in qsc output or publishable demo summaries.

## Focused Validation

Focused validation passed:

- `cargo run -p qsc -- --help` help-surface check: fallback env names not
  present in top-level help.
- `cargo test -p qsc --test na_0591_true_triple_ratchet_path`: pass.
- `cargo test -p qsc --test qsp_status_truthy`: pass.
- `cargo test -p qsc --test qsp_protocol_gate`: pass.

No full qsl-attachments send/receive integration was run.

## Private-Material Review

Published evidence contains no endpoint values, private port values,
route-token/capability values, bearer or Authorization values, payload/body/
plaintext bytes, ciphertext bodies, seed values, key material, raw command
lines, raw logs, process identities, or private topology. Raw outputs and
seed-bearing details remain proof-root-only.

## Result Classification

`SEED_FALLBACK_HARDENING_IMPLEMENTATION_READY`.

## Selected Successor

`NA-0593 -- QSL qsc True Triple-Ratchet Seed Fallback Hardening Implementation
Harness`.

## Required-Check Boundary

Current main check classification passed with visibility recovery: public-safety
success, advisories success, suite2-vectors success, zero failed attached
checks, and zero pending attached checks. CodeQL was represented by successful
CodeQL workflow/analyze jobs. goal-lint is pull-request-only and remains a PR
preflight/required-check gate.

## Source / Script Mutation Boundary

NA-0592 did not mutate qsc source, qsc tests, qsc examples, demo scripts, CI
scripts, dependencies, lockfiles, or workflows.

## qsc Boundary

qsc source/tests/scripts were read and focused qsc tests were run. No qsc
runtime behavior was changed in NA-0592.

## qsl-server Boundary

qsl-server was not mutated, started, stopped, deployed, or integrated.

## qsl-attachments Boundary

qsl-attachments was not mutated and no full qsl-attachments send/receive
integration was run. qsc attachment descriptor reachability was source-reviewed
only.

## Remote / Workflow / Tailscale Boundary

No remote action, SSH, scp, Tailscale action, GitHub workflow dispatch, or
workflow rerun occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, docs/public, public, or Cloudflare mutation occurred.

## Evidence / Decision / Traceability

This evidence doc records NA-0592. D-1175 records the accepted authorization.
TRACEABILITY maps D-1175 to the selected NA-0593 successor. The rolling
operations journal records proof gates, recoveries, classification, validation,
and boundaries.

## Validation

Validation before PR includes scope guard, queue/decision proof, marker proof,
link check, private-material and overclaim scans, PR body preflight,
goal-lint when a PR body exists, cargo audits, locked cargo metadata, cargo fmt,
qsc adversarial script syntax, and the focused qsc tests above.

## Recommendation

Proceed to NA-0593 implementation. Do not start full qsl-attachments
send/receive integration until the selected seed fallback hardening is
implemented, tested fail-closed, merged, and closed out.
