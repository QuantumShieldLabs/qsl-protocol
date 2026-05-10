Goals: G1, G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0259 KT-Negative Demo Readiness Test Plan

## Objective

Prove that the public demo surface includes a truthful, non-production
KT-negative reject proof without fake KT evidence, protocol/crypto
state-machine changes, qsl-server/qsl-attachments changes, website changes,
workflow changes, public-safety changes, branch-protection changes, or Cargo
dependency changes.

## Protected Invariants

- KT-negative demo remains truthful.
- No fake KT evidence.
- Rejects fail closed.
- No accepted KT state mutation on the proved reject path.
- No token/secret leakage.
- Demo remains non-production.
- No production readiness claim.
- Positive qshield demo remains green.
- Existing missing-auth, malformed, invalid relay id, and replay rejects remain
  green.

## Allowed Scope

- `docs/demo/**`.
- `scripts/ci/demo_cli_smoke.sh`.
- `docs/governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md`.
- `tests/NA-0259_kt_negative_demo_readiness_testplan.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Forbidden Scope

Forbidden paths and behaviors include `.github/**`, Cargo manifests/lockfiles,
qsp/protocol-core or crypto state-machine files, qsl-server, qsl-attachments,
qsc-desktop implementation, website/external website, tools implementation
changes, inputs/vector edits, formal model edits, public-safety/check
configuration, branch-protection settings, production relay/service
implementation, fake KT evidence, and production KT claims.

## Positive Demo Proof

Run:

```bash
scripts/ci/demo_cli_smoke.sh
```

Expected qshield positive markers:

```text
DEMO_INIT_TWO_PEERS_OK
DEMO_LOOPBACK_RELAY_OK
DEMO_REGISTER_AUTHORIZED_PEERS_OK
DEMO_ESTABLISH_OK
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

## Existing Negative Demo Proof

The same smoke must emit:

```text
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
```

Expected behavior:

- missing relay auth rejects;
- malformed relay JSON rejects and does not echo supplied sensitive material;
- invalid relay id rejects through the CLI path; and
- replayed establish record rejects deterministically.

## KT-Negative Reject Proof

The same smoke must invoke the canonical KT verifier vectors:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
```

Expected marker:

```text
DEMO_NEGATIVE_KT_REJECT_OK
```

Expected KT evidence:

- positive first-seen and advanced vectors pass;
- stale STH, missing STH, inclusion mismatch, and missing consistency proof
  vectors reject with the KT fail reason code.

## No-Mutation Proof

The same smoke must invoke:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
```

Expected marker:

```text
DEMO_NEGATIVE_KT_NO_MUTATION_OK
```

Expected behavior:

- accepted KT state is primed;
- invalid consistency advancement rejects; and
- the accepted-state snapshot after reject equals the snapshot before reject.

## Non-Production Boundary Proof

The same smoke must invoke:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
```

Expected marker:

```text
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
```

Expected behavior:

- disabled KT shape rejects unless explicit non-production mode is enabled;
- the demo does not silently downgrade into disabled KT acceptance.

## No Token/Secret Leakage Proof

The smoke must emit:

```text
DEMO_NO_SECRET_LEAK_OK
```

Expected behavior:

- relay token is not printed in checked output;
- the secret sentinel is not printed in checked output; and
- reject bodies remain sanitized.

## Final Readiness Marker

The smoke must emit:

```text
NA0259_KT_NEGATIVE_DEMO_READY_OK
DEMO_ACCEPTANCE_OK
```

## Prerequisite Stop Conditions

Stop instead of merging if:

- the demo surface cannot invoke KT verifier evidence truthfully;
- KT proof would require fake evidence;
- proof would require protocol/crypto state-machine changes;
- proof would require qsl-server or qsl-attachments changes;
- proof would require Cargo dependency changes;
- rejected KT evidence mutates accepted state where no-mutation is claimed;
- token/secret leakage is detected; or
- public-safety is not required and green.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed 'docs/demo/**' \
  --allowed 'scripts/ci/demo_cli_smoke.sh' \
  --allowed 'docs/governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md' \
  --allowed 'tests/NA-0259_kt_negative_demo_readiness_testplan.md' \
  --allowed 'DECISIONS.md' \
  --allowed 'TRACEABILITY.md' \
  --allowed 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

If Rust source changes occur, also run:

```bash
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
```

No Rust source changes are expected for NA-0259.

## CI / Public-Safety Expectations

Required PR contexts must attach and pass normally:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

CodeQL may be accepted as neutral only if GitHub branch protection accepts it.
Merge must use a merge commit with `--match-head-commit`, with no direct push,
admin bypass, squash, rebase, public-safety weakening, or branch-protection
exception.
