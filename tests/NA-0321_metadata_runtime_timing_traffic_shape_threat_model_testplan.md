Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0321 Metadata Runtime Timing and Traffic-Shape Threat Model Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0321 produces a truthful timing and traffic-shape threat model
plus future executable evidence plan for current qshield/demo/qsc and
service-referenced metadata-runtime surfaces, without implementing runtime
timing or traffic-shape behavior.

## Protected Invariants

- Timing and traffic-shape gaps must remain visible.
- Current qshield executable proof remains bounded to qshield embedded
  relay/demo behavior.
- qsl-server and qsl-attachments production behavior remains explicitly
  unproven.
- No claim of anonymity, metadata-free behavior, untraceability, production
  readiness, public internet readiness, external review completion, timing
  hiding, or traffic-shape hiding is introduced.
- Future measurement must be distinguished from mitigation.
- Padding proof must not be presented as proof that traffic shape is hidden.

## Allowed Scope

- `docs/governance/evidence/NA-0321_metadata_runtime_timing_traffic_shape_threat_model.md`
- `tests/NA-0321_metadata_runtime_timing_traffic_shape_threat_model_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime timing, jitter, batching, cover traffic, send scheduling, receive
  scheduling, transport padding, or production traffic-shape implementation.
- qshield runtime implementation changes.
- qsl-server or qsl-attachments implementation changes.
- qsc/qsp/protocol/crypto/key-schedule implementation changes.
- Cargo manifest or lockfile changes.
- Workflow, branch-protection, public-safety, website, external website,
  qsc-desktop, README, START_HERE, or dependency changes.
- NA-0322 implementation.

## Prior Metadata-Runtime Review Requirements

Review and preserve:

- NA-0318 qshield candidate-fetch plus ack/commit proof.
- NA-0319 qshield identifier/default-padding proof.
- NA-0320 qshield sanitized-error and retention/purge proof.
- NA-0291 and NA-0293 policy-fixture evidence boundaries.
- Current demo smoke/stress/soak evidence.
- qsc local fixed-polling/meta-plan/receipt-policy evidence as adjacent qsc
  evidence only.
- qsl-server and qsl-attachments production-gate boundaries.

## Surface-Inventory Requirements

The evidence must classify:

- sender cadence;
- receiver cadence;
- queue shape;
- message size and padding;
- ordering and correlation;
- retry and invalid-message cadence;
- stress/soak behavior;
- qsl-server and qsl-attachments production boundaries.

Use classifications: `PROVEN_EXECUTABLE`, `PARTIAL_EXECUTABLE`, `DOCS_ONLY`,
`NOT_READY`, `FUTURE_GATE`, and `OUT_OF_SCOPE`.

## Threat-Model Requirements

The threat model must include:

- message size and bucket observations;
- send timing and receive timing;
- burst frequency;
- queue drain cadence;
- retry cadence;
- ack/commit timing;
- invalid-message retry timing;
- polling interval;
- attachment descriptor/ciphertext size or timing if visible;
- contact/route/session/candidate correlation;
- ordering correlation;
- failed/valid error timing;
- local demo artifact observations;
- production-service observer boundaries marked future-gated.

Attacker categories must include passive local observer, passive network
observer, relay observer, malicious queued-message injector in the bounded demo
context, log/artifact observer, public website/docs observer, and production
service observer.

## Existing-Evidence Mapping Requirements

Map each relevant existing evidence item to:

- what it proves;
- what it does not prove;
- whether it can feed future timing/traffic measurement;
- whether it needs deterministic clocking, artificial jitter, batching, or trace
  logging in a future lane.

Required entries include NA-0318, NA-0319, NA-0320, NA-0291, NA-0293, demo
smoke, demo stress, demo soak, metadata conformance, qsc timing-adjacent tests,
and qsl-server/qsl-attachments production-boundary evidence.

## Future Evidence-Plan Requirements

The plan must describe future harnesses for:

- qshield demo timing measurement;
- padding/size distribution;
- queue cadence;
- invalid retry cadence;
- service timing boundary/cross-repo authorization if needed.

The plan must state that measurement artifacts measure observable timing; they
do not hide timing and do not implement mitigation.

## Successor-Selection Requirements

If a bounded qshield/demo measurement harness is feasible without runtime
changes, select:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

If measurement first requires runtime instrumentation, select an
instrumentation-plan successor. If evidence is insufficient, select a blocker
resolution successor. If qsl-server/qsl-attachments production timing is the
top dependency, select a cross-repo service timing authorization successor.

Do not implement NA-0322.

## Claim-Boundary Requirements

Evidence and PR text must state:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production/public-internet readiness claim;
- no external-review-complete claim;
- no claim that timing metadata is hidden;
- no claim that padding alone hides traffic shape;
- qshield embedded relay/demo proof remains distinct from qsl-server and
  qsl-attachments production behavior.

## Backup-Impact Requirements

Record whether changes create durable artifacts outside the qsl-protocol
worktree or existing Codex response path. Expected result: no backup-plan
update required.

## Required Local Checks

Run or record exact blocker:

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- qshield NA-0320, NA-0319, and NA-0318 harnesses if directly runnable
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh` when feasible
- metadata runtime plan and phase-2 harness scripts
- `scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- formal/model checks
- NA-0310 vector JSON parse and refimpl oracle where available
- queue/decision/scope/link/leak/goal-lint checks
- classifier proof for the changed paths

## CI Expectations

The PR must keep required checks green, including `public-safety`. This is a
docs/governance/testplan lane; no docs-only CI skip may hide claim-boundary,
goal-lint, link, leak, or public-safety failures.

## Successor Handoff

After the NA-0321 PR merges and post-merge public-safety is green, a separate
closeout may mark NA-0321 DONE and restore:

`NA-0322 -- Metadata Runtime Timing and Traffic-Shape Measurement Harness`

The closeout must not implement NA-0322.
