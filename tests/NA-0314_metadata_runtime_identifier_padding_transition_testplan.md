Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18

# NA-0314 Metadata Runtime Identifier and Padding Transition Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0314 creates a transition plan from prior metadata design/fixture evidence toward future runtime identifier/handle rotation and default padding proof without implementing runtime behavior or overstating current evidence.

## Protected Invariants

- NA-0314 remains planning/governance only.
- Existing metadata fixture evidence is not represented as runtime proof.
- Runtime identifier/handle rotation remains open until a later executable lane proves it.
- Runtime default padding remains open until a later executable lane proves it.
- Sanitized-error, retention/purge, timing/traffic-shape, deployment metadata, and public-internet metadata gaps remain visible.
- Exactly one READY queue item remains present during NA-0314: `NA-0314`.

## Allowed Scope

- Add NA-0314 evidence and transition-planning testplan.
- Add D-0607 to `DECISIONS.md`.
- Update `TRACEABILITY.md`.
- Add a concise rolling operations journal entry.

## Forbidden Scope

- Runtime metadata implementation.
- Identifier/handle rotation implementation.
- Default padding implementation.
- qsp, qsc/qsl runtime implementation, protocol-core, crypto state-machine, key schedule, qsl-server, qsl-attachments, qsc-desktop, website, workflow, Cargo, dependency, README, START_HERE, branch-protection, or public-safety changes.
- Claims of production readiness, public-internet readiness, external-review completion, anonymity, metadata-free behavior, untraceability, or complete metadata reduction.

## Metadata Evidence Review Requirements

- Inspect NA-0288, NA-0290, NA-0291, NA-0292, and NA-0293 evidence.
- Separate `PROVEN_EXECUTABLE`, `DOCS_ONLY`, `NOT_READY`, and `FUTURE_GATE` evidence.
- Identify where NA-0291/NA-0293 fixtures are design/fixture proof rather than runtime proof.
- Confirm public documents keep metadata gaps visible.

## Runtime Surface Inventory Requirements

- Inventory qshield peer/session/route/message/attachment handles.
- Inventory qshield default-padding send/receive/relay/init surfaces.
- Inventory qsl-tui padding and metadata-note surfaces.
- Inventory qsc contact/route/session/transport metadata surfaces.
- Inventory sanitized-error, retention/purge, and timing/traffic-shape relationships as later work where applicable.

## Identifier/Handle Transition Requirements

- Define session, route, peer/contact, message, attachment session, attachment object/fetch, and demo correlation identifier classes.
- Define rotation triggers for session boundary, epoch boundary, route renewal, message batch, attachment commit/fetch, and deterministic test fixture triggers.
- Require opaque handles, explicit correlation boundaries, stale/malformed reject, no accepted-state mutation on reject, and no raw handle secret logs.
- Name future markers for identifier/handle proof.

## Default Padding Transition Requirements

- Define qshield, qsl-tui, qsc, attachment-demo, and metadata-fixture padding surfaces.
- Define bucket table, max overhead, invalid config reject, deterministic test mode, default-versus-opt-in choice, and strip/verify behavior.
- Require malformed/over-limit reject, no accepted-state mutation on reject, and no plaintext/sentinel leak in public artifacts.
- Name future markers for padding proof.

## Combined Strategy Requirements

- Explain whether identifier and padding proof should be combined or split.
- Map current proof level to future artifact, expected markers, allowed files, forbidden files, risk, and recommended sequencing.
- Select an exact NA-0315 successor.

## Successor-Selection Requirements

- The successor must be one exact READY candidate.
- The successor must not implement NA-0315 during NA-0314.
- The successor must preserve stop conditions for protocol/crypto/service/dependency/workflow/claim-boundary spillover.

## Claim-Boundary Requirements

- Scan changed text for high-risk readiness/privacy/crypto phrases.
- Allow only negated, prohibited, future-gated, or explicit not-proven wording.
- Confirm no runtime metadata reduction, anonymity, metadata-free, untraceable, public-internet readiness, production readiness, or external-review-complete claim is introduced.

## Backup-Impact Requirements

- List changed paths.
- Confirm whether durable evidence remains under qsl-protocol docs/tests/governance paths in `/srv/qbuild/work`.
- Record whether backup-plan updates are required.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- NA-0310 refimpl oracle test.
- Full refimpl tests if feasible.
- qsc NA-0313, NA-0304, NA-0303, and NA-0302 harnesses if directly runnable.
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- qshield-cli build/test if feasible.
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` if feasible.
- Queue, decision, scope-guard, link-check, leak-scan, classifier, goal-lint, and overclaim checks before PR creation.

## CI Expectations

- Required checks must attach and complete successfully before merge.
- `public-safety` must remain required and green before merge and after merge.
- Docs/governance-only full-suite cost control may skip full suites only if public-safety classifies the patch accordingly.
- No admin bypass, direct push, squash, rebase, or branch deletion is allowed.

## Successor Handoff

If NA-0314 merges cleanly and public-safety is green, the closeout directive may restore:

`NA-0315 -- Metadata Runtime Identifier and Default Padding Executable Harness Plan`

NA-0315 should begin with qshield runtime identifier/default-padding proof and may include qsl-tui/qsc harness checks only where exact scope, no-mutation behavior, and no-claim boundaries remain enforceable.
