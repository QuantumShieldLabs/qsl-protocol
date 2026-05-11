Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0267 CI Advisories Fetch Resilience Audit

Directive: QSL-DIR-2026-05-11-067 / NA-0267

## Objective

Harden the `advisories` / `public-safety` evidence path so transient external
RustSec advisory database fetch failures are retried and reported distinctly
without treating unavailable audit results as a vulnerability-free pass.

## Starting Authority Proof

- Starting `origin/main`: `4c455548f7df`.
- PR #786 was merged as `4c455548f7df`.
- PRs #785 through #761 and PR #708 were verified merged.
- PR #750 and PR #722 were verified closed and unmerged.
- Branch protection required the expected protected contexts, including
  `public-safety`; force pushes and deletions were disabled; admin enforcement
  was enabled.
- Starting `public-safety` on `origin/main` was success.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0267`.
- Decision proof before edits: D-0503 existed once, D-0504 absent, duplicate
  decision count zero.

## Failure-Mode Audit

Current invocation:

- `.github/workflows/public-ci.yml` job `advisories` installs pinned
  `cargo-audit 0.22.0`.
- The push path ran `cargo audit --deny warnings`.
- The relevant PR/workflow path materialized the PR `Cargo.lock`, then ran
  `cargo audit --deny warnings --file /tmp/pr-Cargo.lock`.
- `public-safety` depended on `advisories` and failed at `Require advisories
  success` when `needs.advisories.result` was not `success`.

Prior failure evidence:

- Run `25675241453` attempt 1 on PR #784 merge `a7dbfb2f9e13` failed in
  `advisories`.
- The log showed a RustSec advisory database fetch IO/network failure:
  `couldn't fetch advisory database`, `git operation failed`, and `error
  sending request` for the RustSec advisory database URL.
- The failed log did not report a `RUSTSEC-*` advisory finding or vulnerability
  summary.
- A later failed-job rerun on the same run succeeded, and `public-safety`
  succeeded.

Failure modes identified:

- Real vulnerability/advisory or cargo-audit warning: must fail closed.
- External RustSec advisory database fetch/network failure: may be retried
  boundedly; if still unavailable, the run remains red.
- Local cargo-audit binary, lockfile, parse, or tool failure: unknown failure,
  fail closed.
- GitHub Actions infrastructure failure: remains a job failure; no local
  classifier bypass is introduced.

## Chosen Fix Point

The fix is a combination of helper-level classification and workflow-level
bounded retry:

- `scripts/ci/public_safety_gate.py` now classifies cargo-audit output as
  `clean_success`, `transient_fetch`, `real_finding`, or `unknown_failure`.
- `.github/workflows/public-ci.yml` calls the helper for both push and relevant
  PR/workflow audit paths.
- Transient external fetch failures are retried at most two times.
- Real findings and unknown failures stop immediately and remain red.
- If all attempts are transient fetch failures, the job remains red with a
  distinct transient marker.

This is safer than classifying only in `public-safety` because it gives the
advisories job a chance to recover before downstream gating sees a red check,
while preserving `public-safety` as a required fail-closed dependent check.

## Log Markers

The helper emits deterministic markers:

```text
ADVISORIES_AUDIT_ATTEMPT
ADVISORIES_RETRY_TRANSIENT_FETCH
ADVISORIES_TRANSIENT_FETCH_RETRY_OK
ADVISORIES_REAL_FINDING_FAIL_CLOSED
ADVISORIES_UNKNOWN_FAIL_CLOSED
ADVISORIES_TRANSIENT_FETCH_FAIL_CLOSED
```

## What Is Proven

- Clean cargo-audit success passes.
- Transient advisory database fetch failure is classified narrowly.
- A transient fetch retry that later succeeds emits retry-ok proof.
- Real advisory findings, including `RUSTSEC-*` output, fail closed.
- cargo-audit warnings fail closed.
- Unknown local/tool/lockfile failures fail closed.
- Mixed logs containing both fetch text and real advisory text classify as real
  findings, not transient.

## What Is Not Claimed

- No advisory is ignored.
- No dependency is updated.
- No `Cargo.toml` or `Cargo.lock` file is changed.
- No branch-protection setting is changed.
- No protocol/runtime/crypto/demo behavior is changed.
- A transient fetch classification is not a vulnerability-free pass unless a
  later bounded retry obtains a successful cargo-audit result.
