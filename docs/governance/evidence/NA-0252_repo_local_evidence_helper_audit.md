Goals: G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0252 Repo-Local Evidence Helper Audit

Directive: QSL-DIR-2026-05-06-038 / NA-0252

## Objective

Reduce recurring operational friction by adding a repo-local read-only evidence helper for queue/decision parsing, scope guarding, required-check summaries, public-safety diagnosis, markdown link checks, high-confidence leak scans, PR-body preflight, and CI/admission preflight.

This audit records the implementation boundary, CI/admission feasibility preflight, command evidence, limitations, and no-weakening statement.

## CodeQL Redaction Recovery

PR #754 was blocked by the aggregate required `CodeQL` status after analyzer jobs completed. The check-run annotation reported one new high-severity alert in `scripts/ci/qsl_evidence_helper.py` at line 561: clear-text logging of sensitive information from leak-scan finding output.

The fix changes leak-scan findings to retain only rule name, path, and line number. Finding output now uses a fixed redaction marker and never prints matched source text, source-line excerpts, raw credential material, authorization header values, or matched secret-like substrings.

Expected redacted finding shape:

```text
SECRET_FINDING type=<rule> path=<path> line=<line> redaction=[redacted]
```

Temporary fake-secret regression proof is recorded in `tests/NA-0252_repo_local_evidence_helper_testplan.md`. The temporary fixture is not committed.

## Helper Commands Added

`scripts/ci/qsl_evidence_helper.py` adds these read-only subcommands:

- `queue`
- `decisions`
- `scope-guard`
- `checks-summary`
- `public-safety-status`
- `link-check`
- `leak-scan`
- `pr-body-preflight`
- `ci-admission-preflight`

## CI / Admission Feasibility Preflight

Preflight findings before Packet B edits:

1. Proposed changed paths are limited to Packet B allowed paths: one helper script, governance evidence, testplan, DECISIONS, TRACEABILITY, and the rolling journal.
2. `qsc-adversarial-smoke` is expected to trigger because `scripts/ci/qsl_evidence_helper.py` is classified as `workflow_security` by `scripts/ci/classify_ci_scope.sh`.
3. The current mainline cargo-fuzz install recovery is present in `.github/workflows/qsc-adversarial.yml`, and latest main `qsc-adversarial-smoke` completed successfully on `9867d0d8ba4d`.
4. `public-safety` is required by branch protection and latest main `public-safety` completed successfully on `9867d0d8ba4d`.
5. The helper PR does not require red-main admission because main public-safety is green.
6. No circular CI dependency is likely: the helper does not change public-safety gate code, branch protection, required checks, workflows, Cargo metadata, or runtime/security paths.
7. All proposed helpers are evidence/reporting only.
8. Helpers do not mutate branch protection, merge PRs, spoof checks, or rerun workflows by default.

Conclusion: Packet B is mergeable through the normal required-check path if CI remains green.

## Commands Run

Pre-edit hard-start and health commands:

```bash
df -BG /srv/qbuild
git fetch --all --prune
git rev-parse origin/main
gh pr view 753 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 752 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 751 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 750 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 749 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 748 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 747 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 746 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

Helper validation commands:

```bash
python3 -m py_compile scripts/ci/qsl_evidence_helper.py
python3 scripts/ci/qsl_evidence_helper.py --help
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths DECISIONS.md TRACEABILITY.md NEXT_ACTIONS.md
# temporary fake-secret regression: leak-scan reports metadata plus redaction marker,
# exits nonzero for a finding, and does not print the fake token or a large substring
python3 scripts/ci/qsl_evidence_helper.py checks-summary --pr 752 --report-only --allow-codeql-neutral
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --report-only
python3 scripts/ci/qsl_evidence_helper.py ci-admission-preflight --pr 752 --report-only
```

Additional validation is recorded in `tests/NA-0252_repo_local_evidence_helper_testplan.md` and the final PR evidence.

## Current Proof Summary

- `origin/main`: `9867d0d8ba4d`
- PR #753: merged with merge commit `9867d0d8ba4d`
- PR #752: merged
- PR #750: closed and unmerged
- PR #722: closed and unmerged
- PR #708: merged
- Branch protection requires `public-safety` plus the expected required contexts.
- Force pushes and deletions are disabled.
- Admin enforcement is enabled.
- Latest main `public-safety`, `qsc-linux-full-suite`, `macos-qsc-full-serial`, and `qsc-adversarial-smoke` are green.
- Queue parser reports `READY_COUNT 1`, sole READY `NA-0252`.
- Decision parser reports D-0110 and D-0439 through D-0470 once each, D-0471 absent before Packet B, D-0472 absent, and duplicate count zero.

## Recovered Failure Evidence

- Failing command: `git show origin/main:NEXT_ACTIONS.md | python3 - <<'PY' ...`
- Classification: recoverable command-shape / stdin-wiring mistake during read-only queue proof.
- Cause: the heredoc fed Python source through stdin and discarded the piped `git show` content, producing an invalid `READY_COUNT 0` result.
- Corrective action: reran the canonical queue parser with Python source passed via `-c` and `git show` content on stdin.
- Final result: corrected parser reported `READY_COUNT 1`, sole READY `NA-0252`; corrected decision parser reported D-0110 and D-0439 through D-0470 once each, D-0471 absent on `origin/main`, D-0472 absent, and duplicate count zero.

- Failing command: `cargo audit --deny warnings`
- Classification: recoverable command-context issue, not a main-health failure.
- Cause: the existing clean local worktree was still on an older `main` checkout (`2abcee236e23`) while live `origin/main` was the required `9867d0d8ba4d`.
- Corrective action: switched the clean worktree to `na-0252-repo-local-evidence-helper` from `origin/main`.
- Final result: `cargo audit --deny warnings` passed on the corrected checkout; `cargo tree -i rustls-webpki --locked` reported `rustls-webpki v0.103.13`.

- Failing command: `git add scripts/ci/qsl_evidence_helper.py tests/NA-0252_repo_local_evidence_helper_testplan.md docs/governance/evidence/NA-0252_repo_local_evidence_helper_audit.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- Classification: recoverable staging command-shape issue for an explicitly allowed evidence path.
- Cause: local ignore rules skip new files under `docs/governance/evidence`.
- Corrective action: reran staging for only the intended audit path with `git add -f docs/governance/evidence/NA-0252_repo_local_evidence_helper_audit.md`.
- Final result: staged changed paths include exactly the six Packet B allowed files.

## No-Weakening Statement

NA-0252 does not change `.github` workflows, `scripts/ci/public_safety_gate.py`, `scripts/ci/qsc_adversarial.sh`, Cargo files, qsc/qsl apps, tools, inputs, formal models, qsc-desktop, qsl-server, qsl-attachments, website implementation, protocol/runtime/crypto/demo/service code, public-safety configuration, branch-protection settings, or required check settings.

The helper is read-only: it parses files, reads Git diffs, checks markdown links, scans text for high-confidence secret-like markers, validates PR body fields, and calls `gh` only for read-only API summaries. It does not write files, mutate branch protection, merge PRs, spoof checks, or rerun workflows.

## Limitations

- GitHub-backed commands require `gh` and authentication unless `--report-only` is acceptable for diagnostics.
- `checks-summary` reads the latest visible check runs for the target SHA; it does not replace GitHub branch protection.
- `ci-admission-preflight` is a conservative summary, not an admission authority.
- `leak-scan` targets high-confidence secret-like markers and intentionally avoids broad long-hex matching to reduce false positives in governance evidence.
- `link-check` validates relative markdown targets and ignores generated/build/vendor directories.

## Recommended Future Improvements

1. Add small fixture-driven helper self-tests under a future explicit testing lane if the helper begins to grow.
2. Add optional JSON output for automation consumers while keeping the current line-oriented output stable.
3. Add a documented runbook showing how Codex Automations can call `queue`, `public-safety-status`, and `checks-summary` in report-only monitoring mode.
4. Consider a later public-safety diagnostic lane only if repeated evidence shows current public-safety output remains hard to interpret.

## Post-Fix Hardening Review

1. Correctness under stress: parser and diagnostic commands fail closed on wrong READY count, duplicate decision IDs, forbidden scope paths, missing/red required contexts, missing markdown links, high-confidence secret findings, and incomplete PR body metadata.
2. Minimality: the implementation is a single standard-library helper plus governance evidence and does not touch CI workflows, public-safety gate code, Cargo files, runtime, protocol, crypto, demo, service, or website implementation.
3. Maintainability: required contexts and default governance selections are centralized, GitHub API calls are isolated, and command output is stable line-oriented evidence.
4. Coverage quality: validation exercises every required subcommand and uses temporary PR-body fixtures for pass/fail behavior.
5. Cross-lane stability: the helper uses Python standard library and read-only Git/GitHub commands, so it is portable across Linux/macOS runners that already support the repository validation tooling.
