Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-16

# NA-0235 Blocked on Dependency Advisories Evidence

## Summary

- Refreshed live proof shows PR `#695` remains open and blocked on head `68a3a8081889`.
- The repaired `public-safety` context is now attached truthfully to that PR head.
- `public-safety` fails for the correct reason: the nested `advisories` job fails at `Cargo audit (fail-closed) on relevant PR/workflow lanes`.
- The rest of the protected required set is green on the same PR head.
- This evidence PR is governance-only and introduces no runtime changes.

## Live blocker proof

- PR: `#695` https://github.com/QuantumShieldLabs/qsl-protocol/pull/695
- State: `OPEN`
- Merge state: `BLOCKED`
- Base branch: `main`
- PR head SHA: `68a3a8081889`
- Refreshed `main` SHA: `fd4400406d80`

## Current-main truth

- Refreshed `main` still lacks the `NA-0235` repair because PR `#695` is unmerged.
- Read-only proof: refreshed `main` still carries the older `public-ci` trigger shape with `pull_request`, not the repaired `pull_request_target` plus sanctioned bootstrap support from PR `#695`.

## Required-context proof on PR #695 head

- `public-safety`: `failure`
- `ci-4a`: `success`
- `ci-4b`: `success`
- `ci-4c`: `success`
- `ci-4d`: `success`
- `ci-4d-dur`: `success`
- `demo-cli-build`: `success`
- `demo-cli-smoke`: `success`
- `formal-scka-model`: `success`
- `goal-lint`: `success`
- `metadata-conformance-smoke`: `success`
- `suite2-vectors`: `success`
- `CodeQL`: `success`
- `macos-qsc-qshield-build`: `success`

## Why public-safety is failing

- Workflow run `24487753581` attached the real `public-safety` context to PR head `68a3a8081889`.
- In that run, job `advisories` failed at step `Cargo audit (fail-closed) on relevant PR/workflow lanes`.
- The job log shows live RustSec findings including `RUSTSEC-2026-0099`, `RUSTSEC-2026-0098`, and `RUSTSEC-2026-0097`.
- The downstream `public-safety` job then failed at step `Require advisories success`.
- This proves the remaining blocker is dependency health, not missing context attachment, fake status wiring, or branch-protection ambiguity.

## Queue-repair rationale

- `NA-0235` is no longer the truthful sole READY item because its implementation PR is blocked by live dependency findings outside the lane’s approved governance/workflow scope.
- The next truthful successor is `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`.

