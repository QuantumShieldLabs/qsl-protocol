# NA-0262 Demo Adversarial Stress Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Validate that the non-production qshield public demo has a bounded
adversarial stress, chaos, and abuse harness that proves selected fail-closed
and leak-safe behavior without protocol/crypto state-machine changes,
qsl-server or qsl-attachments production changes, website changes, workflow
changes, branch-protection changes, public-safety changes, or Cargo dependency
changes.

## Protected Invariants

- Stress runs are bounded and local.
- No public internet or third-party target is tested.
- Auth remains required for relay mutation paths.
- Malformed inputs reject deterministically.
- Replay attempts reject.
- Relay identity boundaries reject invalid ids.
- Queue/cap/rate boundaries remain bounded where tested.
- Rejected inputs do not mutate state where observable and claimed.
- Tokens, secret sentinels, and attachment plaintext sentinels do not leak to
  transcripts or error output.
- CLI/demo surfaces do not show panic, backtrace, or unwrap output.
- The demo remains explicitly non-production.
- No production hardening is claimed from the harness.

## Allowed Scope

- `scripts/ci/demo_adversarial_stress.sh`
- `scripts/ci/demo_cli_smoke.sh` only if integration is required
- `scripts/ci/metadata_conformance_smoke.sh` only if integration is required
- `apps/qshield-cli/**` only for minimal test-backed demo CLI hardening
- `docs/demo/**`
- `docs/governance/evidence/NA-0262_demo_adversarial_stress_audit.md`
- `tests/NA-0262_demo_adversarial_stress_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- qsc/qsl protocol-core or crypto state-machine files
- `qsl-server/**`
- `qsl-attachments/**`
- qsc-desktop implementation
- `website/**`
- external website source
- production relay/service implementation
- branch-protection settings
- public-safety/check configuration

## Positive Baseline Proof

Run:

```bash
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
```

Expected:

- exits zero;
- creates an artifact directory under
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_<timestamp>/`;
- emits `DEMO_STRESS_POSITIVE_BASELINE_OK`; and
- emits `NA0262_DEMO_ADVERSARIAL_STRESS_OK`.

## Auth Proof

The harness must prove missing, wrong-token, wrong-scheme, and empty auth
rejects on a loopback relay without printing live token values.

Expected marker:

```text
DEMO_STRESS_AUTH_REJECT_OK
```

The unauthorized send no-mutation proof must also observe an empty recipient
queue after the rejected mutation attempt.

Expected marker:

```text
DEMO_STRESS_AUTH_REJECT_NO_MUTATION_OK
```

## Malformed Proof

The harness must prove malformed JSON, wrong content type, empty body, and a
bounded oversized body reject with sanitized response bodies.

Expected marker:

```text
DEMO_STRESS_MALFORMED_REJECT_OK
```

## Replay Proof

The harness must prove establish replay rejects through the current local demo
relay replay-record endpoint.

Expected marker:

```text
DEMO_STRESS_REPLAY_REJECT_OK
```

## Relay Identity Proof

The harness must prove invalid relay id format rejects.

Expected marker:

```text
DEMO_STRESS_RELAY_ID_REJECT_OK
```

## Attachment Proof

Because NA-0260 added attachment proof surface, the baseline harness must
require the existing tampered attachment integrity reject marker from
`scripts/ci/demo_cli_smoke.sh`.

Expected marker:

```text
DEMO_STRESS_ATTACHMENT_INTEGRITY_REJECT_OK
```

## KT Proof

Because NA-0259 added KT-negative proof surface, the baseline harness must
require the existing KT-negative and KT no-mutation markers from
`scripts/ci/demo_cli_smoke.sh`.

Expected marker:

```text
DEMO_STRESS_KT_REJECT_OK
```

## Queue / Cap Proof

The harness must drive a bounded recipient queue overflow attempt and accept
only the expected bounded reject.

Expected marker:

```text
DEMO_STRESS_QUEUE_OR_RATE_BOUND_OK
```

## Chaos / Interruption Proof

The harness must kill the controlled loopback relay, verify it no longer
answers, restart it on the same loopback port, and verify health returns.

Expected marker:

```text
DEMO_STRESS_CHAOS_RECOVERY_OK
```

The baseline profile records port-in-use as unsupported because it is an
extended-profile check.

Expected unsupported declaration:

```text
UNSUPPORTED_PORT_IN_USE_BASELINE: extended profile only
```

## Leak / Panic Proof

The harness must scan the artifact bundle for generated relay tokens, injected
secret sentinels, panic text, backtraces, and unwrap-output text before
declaring success.

Expected markers:

```text
DEMO_STRESS_NO_SECRET_LEAK_OK
DEMO_STRESS_NO_PANIC_OK
```

## Validation Bundle

Run:

```bash
git diff --check
bash -n scripts/ci/demo_adversarial_stress.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Expected:

- `READY_COUNT 1`, READY `NA-0262`.
- D-0494 exists once.
- D-0495 is absent.
- No duplicate decision IDs.
- No forbidden paths are touched.
- Baseline stress harness passes.
- Demo smoke and metadata smoke still pass.
- Link check passes.
- Added-line leak scan reports zero secret findings.
- Goal-lint passes.

## CI Expectations

- Required protected checks pass normally.
- `public-safety` remains required and green.
- This script/demo lane is runtime/workflow-adjacent enough that full-suite
  waits are expected under the NA-0262A cost-control policy.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
