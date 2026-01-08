Goals: G3, G5

# NA-0019 — Demo establish identity binding test plan

Status: DRAFT
Scope: Demo relay/CLI only. No protocol-core changes.

## Objective
- Enforce explicit identity binding in peer bundles for demo establish.
- Ensure missing/mismatched bundle.id rejects deterministically and does not consume bundles.
- Preserve normal establish behavior when binding is correct.

## CI-gated assertions
Enforced by: `scripts/ci/metadata_conformance_smoke.sh`

- Missing bundle.id causes establish (with override) to fail deterministically and does not consume bundle.
- Mismatched bundle.id causes establish (with override) to fail deterministically and does not consume bundle.
- Normal establish succeeds with override and consumes bundle (NA-0018 invariant).

## Evidence
- metadata-conformance-smoke CI job logs.
