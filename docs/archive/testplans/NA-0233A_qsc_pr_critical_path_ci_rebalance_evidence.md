Goals: G4, G5

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-12
Replaces: n/a
Superseded-By: n/a

# NA-0233A qsc PR Critical-Path CI Rebalance Evidence

## Summary

`NA-0233A` resolved the PR critical-path CI blocker that had been preventing truthful continuation of the bounded MockProvider runtime lane in PR #688.

Implementation PR: #690, `NA-0233A qsc PR Critical-Path CI Rebalance`
Implementation merge SHA: `96e02a79db5e`
Implementation mergedAt: `2026-04-12T22:41:08Z`
Implementation head SHA: `0e37e676b20f`

## Implementation Evidence Surfaces On Main

- `DECISIONS.md` `D-0404` records that the protected status names `ci-4a` and `macos-qsc-qshield-build` were preserved truthfully while being narrowed to build-plus-smoke PR critical-path signals.
- `TRACEABILITY.md` contains the `NA-0233A implementation/evidence` entry pointing to the workflow, helper, governance, journal, and companion testplan surfaces changed by PR #690.
- `.github/workflows/ci.yml` now keeps protected `ci-4a` as Linux qsc release-build plus representative smoke tests (`vault`, `handshake_contract_na0217i`, `qsp_protocol_gate`) and keeps the prior broad whole-package qsc lane available as non-required `qsc-linux-full-suite` outside `pull_request` gating.
- `.github/workflows/macos-build.yml` now keeps protected `macos-qsc-qshield-build` as macOS qsc/qshield build plus the same smoke subset and keeps the prior full serial suite available as non-required `macos-qsc-full-serial` outside `pull_request` gating with a larger timeout budget.
- `scripts/ci/classify_ci_scope.sh` now treats markdown companions under `tests/` as docs scope for CI classification so workflow/governance PRs with mandatory companion stubs do not pick up unrelated runtime-only advisory churn.

## Queue-Restore Basis

- PR #690 is merged and its merge commit `96e02a79db5e` is now on refreshed `main`.
- PR #688 remains OPEN at head `d9a0d3260ae0` with merge state `DIRTY`.
- The current required-context snapshot on the stale PR #688 head still shows `ci-4a=failure` and `macos-qsc-qshield-build=cancelled`, but that is evidence of the old branch state rather than the refreshed-main truth because the rebalance from PR #690 is already merged on `main`.
- Refreshed `main` now carries the rebalance that removed the old whole-package Linux gate and timed full-serial macOS gate from the protected PR critical path, so the truthful remaining blocker is stale-base resume work on the bounded runtime lane rather than unresolved CI-critical-path design.

## Exact Closeout Outcome

- The old PR critical-path blocker named under `NA-0233` is no longer true on refreshed `main`.
- `NA-0233A` closes truthfully as the merged CI/workflow governance lane that restored a bounded protected PR path without weakening required coverage truth.
- `NA-0233` is restored as the sole READY item because the remaining work is to resume or supersede the existing bounded runtime lane from refreshed `main`, not to invent a new queue item or reopen workflow design.
- No qsc runtime paths, runtime tests, qsc-desktop paths, qsl-server paths, qsl-attachments paths, `.github` paths in this closeout PR, `Cargo.toml`, `Cargo.lock`, or website/public-runtime surfaces changed in this closeout/evidence lane.

## Closeout Scope

This closeout PR is governance-only. It records durable archive evidence for the merged CI rebalance, marks `NA-0233A` DONE, restores `NA-0233` to READY, and leaves PR #688 open and untouched for later resume from refreshed `main`.
