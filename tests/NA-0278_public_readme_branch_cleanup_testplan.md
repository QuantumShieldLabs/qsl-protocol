Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0278 Public README and Branch Cleanup Test Plan

## Objective

Validate that NA-0278 improves the public README reviewer path, records a
read-only stale-branch cleanup audit, and preserves all public-safety,
claim-boundary, and no-branch-deletion invariants.

## Protected invariants

- README remains research-stage and evidence-bound.
- No production readiness or deployment readiness is claimed.
- No quantum-proof, metadata-free, anonymity, anonymous messaging, or
  untraceable claim is introduced.
- No external review completion claim is introduced.
- No proven true Triple Ratchet claim is introduced.
- No branch deletion, branch mutation, branch-protection mutation, or PR state
  mutation is performed outside normal NA-0278 PR creation/merge.
- qsl-protocol implementation, qsl-server implementation, qsl-attachments
  implementation, qsc-desktop, website/external website, workflows, scripts,
  Cargo files, dependencies, public-safety configuration, protocol, wire,
  crypto, and state machines remain untouched.

## Allowed and forbidden scope

Allowed paths:

- `README.md`
- `docs/governance/evidence/NA-0278_public_readme_branch_cleanup_audit.md`
- `tests/NA-0278_public_readme_branch_cleanup_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include `.github/**`, `scripts/**`, `Cargo.toml`,
`Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`,
`tools/**`, `inputs/**`, `formal/**`, `qsc-desktop/**`, `qsl-server/**`,
`qsl-attachments/**`, `website/**`, and any external website repository.

## README claim-safety checks

Review changed README lines for:

- research-stage/no-production posture.
- direct evidence links.
- no stronger release or production claim than current evidence supports.
- no implied completed external review.
- no metadata-elimination, anonymity, untraceable, or quantum-proof language.

## Branch audit read-only proof

Required branch-audit commands:

- `gh repo view QuantumShieldLabs/qsl-protocol --json defaultBranchRef,nameWithOwner`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches --paginate`
- `gh pr list --repo QuantumShieldLabs/qsl-protocol --state open --json number,title,headRefName,headRefOid,baseRefName,url --limit 200`
- `gh pr list --repo QuantumShieldLabs/qsl-protocol --state closed --json number,title,headRefName,headRefOid,baseRefName,mergedAt,mergeCommit,url --limit 200`
- targeted PR mapping checks for the stale branch names.

Expected result:

- `main` is the default branch and remains protected.
- stale non-main branches map to closed/unmerged PRs.
- no open PR depends on the stale branches.
- branch recommendations are recorded as future approval candidates only.

## No branch deletion proof

Verify:

- no `git push origin --delete` was run.
- no `gh api` DELETE branch-ref mutation was run.
- no branch-protection mutation was run.
- no stale branch was deleted from GitHub.
- audit text states no branch deletion was performed.

## Overclaim scan

Scan changed lines for:

- `production-ready`
- `deployment-ready`
- `production relay ready`
- `qsl-server production ready`
- `production attachment ready`
- `quantum-proof`
- `metadata-free`
- `anonymity`
- `anonymous messaging`
- `untraceable`
- `external review complete`
- `proven true Triple Ratchet`

Matches are acceptable only when explicitly negated, listed as prohibited
wording, or described as not yet proven.

## Link, leak, and goal-lint expectations

Expected:

- README links resolve.
- repo link-check reports zero missing links.
- added-line leak scan reports zero secret findings.
- docs and testplan carry a `Goals:` line near the top.
- PR body includes a standalone `Goals: G1, G3, G4, G5` line.

## CI expectations

Expected local validation:

- `git diff --check`
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py` if present
- goal-lint with a synthetic PR event or helper-supported equivalent.

Expected protected CI:

- required checks attach and pass normally.
- docs/governance-only cost-control may skip heavy full-suite jobs while
  required public-safety remains green.

## Successor handoff

NA-0278 remains READY after the README/audit PR merges. If post-merge
public-safety is green and the queue remains exactly one READY item, a separate
closeout may restore:

`NA-0279 — qsl-server Rate-Limit / Global Route-Cap Design and Harness Plan`
