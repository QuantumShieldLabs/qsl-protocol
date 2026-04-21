Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-21

# NA-0237 Scope Repair — qsp/state Clippy and Refimpl Test Evidence

Goals: G4

## Purpose

This archive evidence records why the first local `NA-0237` KT verifier implementation attempt stopped truthfully and why the minimal queue repair on `main` only needs two additional bounded surfaces:

- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` for the clippy-only validation fix required by the lane's already-mandated `cargo clippy --locked -- -D warnings` gate
- `tools/refimpl/quantumshield_refimpl/tests/**` for direct KT verifier vectors/regressions

This PR is governance-only. It does not change runtime semantics, wire semantics, KT semantics, or queue ordering.

## First Attempt Stop Proof

- Refreshed `main` for qsl-protocol, `origin/main`, and `mirror/main` matched at `905c32f4e325` when the first implementation lane resumed.
- The dirty implementation worktree preserved under `/srv/qbuild/tmp/na0237_scope_repair_preservation` shows the attempted KT change set touched the bounded refimpl/actor KT seam plus governance companions, while `git diff --name-only -- tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` returned zero lines.
- The lane's required validation bundle included `cargo clippy --locked -- -D warnings`.
- That command failed only on `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs:273` and `:296` with `clippy::unnecessary_sort_by`, where clippy suggested the behavior-preserving replacement of `sort_by(|a, b| a.0.cmp(&b.0))` with `sort_by_key(|a| a.0)`.
- Because `qsp/state.rs` was outside the then-live `NA-0237` scope block, the implementation lane stopped before branch creation or PR creation instead of making an out-of-scope edit.

## Bounded Refimpl Regression Surface Proof

- The first local implementation attempt added `tools/refimpl/quantumshield_refimpl/tests/kt_verifier_vectors.rs` as a direct refimpl KT regression/vector file.
- That file is bounded to the KT verifier seam: it constructs canonical KT bundles and proofs, exercises STH signature verification, inclusion proof verification, consistency-proof handling, bundle-signature validation, and fail-closed mutation cases against the refimpl verifier interface.
- The same attempt also added the matching vector data file `inputs/suite2/vectors/qshield_suite2_kt_verifier_vectors_v1.json`, which already fits the existing `NA-0237` vector allowance. The missing surface was the refimpl test file itself.
- No broader test widening was discovered. The direct refimpl regression file is the only additional test path needed for the KT verifier lane.

## Scope Repair Conclusion

Refreshed contradiction proof from the stopped local attempt therefore supports exactly one minimal queue repair:

1. authorize `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` only for the bounded clippy-only fix required by the lane's mandatory validation gate
2. authorize `tools/refimpl/quantumshield_refimpl/tests/**` only for direct KT verifier vectors/regressions

No broader runtime, manifest, workflow, sibling-repo, or queue-order widening is required.
