Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-21

# NA-0237A Scope Repair — qsp/state Clippy Evidence

Goals: G4

## Purpose

This archive evidence records why the first local `NA-0237A` implementation attempt stopped truthfully and why the live `NA-0237A` queue block on `main` only needs one additional bounded scope line:

- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` only if directly touched by the bounded clippy-only fix required to pass the lane's already-mandated `cargo clippy --locked -- -D warnings` validation

This PR is governance-only. It does not change runtime semantics, wire semantics, queue ordering, or protected-check semantics.

## First Attempt Stop Proof

- The dirty implementation worktree under `/srv/qbuild/work/NA-0237A/qsl-protocol` remained intact on head `27d4ec48b48f` and was preserved off-repo under `/srv/qbuild/tmp/na0237a_scope_repair_preservation/`.
- The preservation bundle proves the attempted `NA-0237A` change set is dirty and resumable:
  - `status.txt` records the stopped branch `na-0237a-send-commit-fallout-repair`
  - `changed_paths.txt` records the local changed paths
  - `tracked.patch` captures the tracked diff
  - `head_sha.txt` records the base SHA `27d4ec48b48f`
- The first local attempt stopped on the lane's required `cargo clippy --locked -- -D warnings` gate because current main still trips `clippy::unnecessary_sort_by` in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` at the sorted `hk_entries` and `pq_entries` blocks.
- The dirty implementation attempt itself did not touch `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`; the stopped code/test diff remains outside that file.

## Bounded send_commit Repair Proof

- The local implementation diff in the dirty worktree is bounded to `qsl/qsl-client/qsc/tests/send_commit.rs` among code/test paths.
- `qsl/qsl-client/qsc/tests/common/mod.rs` did not require modification for the first attempt because the needed passphrase-backed helper already existed there.
- The bounded local fix replaces the stale `qsc vault init --non-interactive --key-source mock` setup with the existing passphrase-backed helper path and switches the test invocations onto the explicit unlock-bearing helper command.
- No broader qsc runtime-source change is required by the current contradiction proof; the stale dependency sits in the test seam, while `vault_mock_provider_retired` remains the truthful runtime behavior on refreshed `main`.

## Scope Repair Conclusion

Refreshed contradiction proof from the stopped local attempt therefore supports exactly one minimal queue repair for `NA-0237A`:

1. authorize `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` only for the bounded clippy-only fix required by the lane's mandatory validation gate

No broader qsc runtime, refimpl runtime, manifest, workflow, sibling-repo, or queue-order widening is required by this governance lane.
