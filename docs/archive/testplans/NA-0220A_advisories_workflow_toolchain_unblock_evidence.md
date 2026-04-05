Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-04

# NA-0220A Advisories Workflow Toolchain Unblock Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0220A`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #654
- Implementation branch head before merge: `eaf04d30bc4e`
- Implementation merge SHA: `b0f4fa27cd31`
- Implementation mergedAt: `2026-04-04T22:17:06Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `b0f4fa27cd31`
- refreshed merged main contains `.github/workflows/public-ci.yml` with the repaired `advisories` job, `DECISIONS.md` `D-0370`, the `TRACEABILITY.md` `NA-0220A implementation/evidence` entry, and `tests/NA-0220A_advisories_unblock_testplan.md`
- refreshed live queue still showed `NA-0220A` as the sole `READY` item and `NA-0220` as `BLOCKED` before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## Implemented Workflow Surface

- Workflow source: `.github/workflows/public-ci.yml`
- Exact repaired `advisories` surface now on merged `main`:
  - `toolchain: 1.85.1`
  - `cargo install --locked cargo-audit --version 0.22.0`
  - `cargo audit --deny warnings`
  - `Upload advisories audit output`

## Acceptance-Proof Surface

- PR `#654` is merged with merge commit `b0f4fa27cd31`
- PR `#654` changed only:
  - `.github/workflows/public-ci.yml`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `tests/NA-0220A_advisories_unblock_testplan.md`
- the required `advisories` context on PR `#654` head `eaf04d30bc4e` completed `success`
  - run URL: `https://github.com/QuantumShieldLabs/qsl-protocol/actions/runs/23988414839/job/69964047237`
  - startedAt: `2026-04-04T21:57:49Z`
  - completedAt: `2026-04-04T22:00:37Z`
- no runtime surfaces changed in the merged implementation lane
- queue repair in this closeout lane therefore restores `NA-0220` as the sole `READY` item
- PR `#652` itself was not altered in the unblock implementation lane and is not altered in this closeout lane

## Implementation / CI Nuance Summary

- the required `advisories` context was repaired without weakening fail-closed behavior: the job still runs `cargo audit --deny warnings` and still uploads the advisories artifact
- the unblock removed the brittle `cargo-binstall` path rather than weakening or skipping the dependency-audit gate
- the implementation lane completed with all 34 protected checks green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Exact Commands / Checks Run For The Merged Implementation Lane

- `gh pr view 652 --repo QuantumShieldLabs/qsl-protocol --json state,headRefOid,mergeStateStatus,statusCheckRollup,url`
- `gh run view 23969882396 --repo QuantumShieldLabs/qsl-protocol --job 69916918174 --log-failed`
- `gh pr diff 654 --repo QuantumShieldLabs/qsl-protocol --name-only`
- `gh pr view 654 --repo QuantumShieldLabs/qsl-protocol --json state,headRefOid,mergeCommit,mergedAt,url`
- `gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/${HEAD_SHA}/check-runs?per_page=100" --jq '.check_runs[] | select(.name=="advisories") | {name,status,conclusion,html_url,started_at,completed_at}'`
- local `tools/goal_lint.py` via synthesized `GITHUB_EVENT_PATH`
- manual markdown link-integrity runbook from `AGENTS.md`
- added-line leak-safe scan for `v1-path pattern` and `hex32plus pattern`

## Why NA-0220A Is Complete And Why NA-0220 Is Restored

- `NA-0220A` is complete because refreshed merged main now carries the repaired `advisories` workflow/toolchain surface, the matching governance updates, and protected-check proof that the unblock PR itself went green without changing runtime paths
- `NA-0220` is restored because the only blocker was the required `advisories` context, and PR `#654` repaired that blocker on `main` without changing the underlying handshake audit scope

## Closeout Note

- This closeout PR is governance-only.
- No runtime, `.github`, protocol, relay, qsc-desktop, qsl-server, or qsl-attachments paths change in this closeout.
- PR `#652` remains open and untouched in this lane.
