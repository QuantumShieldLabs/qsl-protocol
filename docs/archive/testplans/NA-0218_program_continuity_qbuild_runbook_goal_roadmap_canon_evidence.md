Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-03

# NA-0218 Program Continuity / qbuild Runbook / Goal-Roadmap Canon Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0218`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #647
- Implementation branch head before merge: `e6472bf631e5`
- Implementation merge SHA: `5437e0a9e0b1`
- Implementation mergedAt: `2026-04-03T23:07:31Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `5437e0a9e0b1`
- refreshed merged main contains `DECISIONS.md` `D-0365`, the `TRACEABILITY.md` `NA-0218 implementation/evidence` entry, `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md`, `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md`, `docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md`, and `tests/NA-0218_continuity_and_roadmap_testplan.md`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0218` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## Implemented Docs Inventory

- `START_HERE.md`
- `AGENTS.md`
- `docs/ops/DOC-OPS-001_qbuild_Continuity_and_Disaster_Recovery_Runbook_v0.1.0_DRAFT.md`
- `docs/ops/DOC-OPS-002_Continuity_Snapshot_Manifest_and_Offhost_Procedure_v0.1.0_DRAFT.md`
- `docs/program/DOC-PROG-001_Goal_to_Release_Roadmap_v0.1.0_DRAFT.md`
- `tests/NA-0218_continuity_and_roadmap_testplan.md`

## Acceptance-Proof Surface

- qbuild continuity / disaster-recovery procedure is checked in:
  - mirrors and worktrees
  - authority proof
  - merge refresh
  - GitHub-only recovery
  - host prep and `qstart` expectations
  - overlay / end-of-day handling
- continuity snapshot manifest and off-host procedure is checked in:
  - minimum snapshot contents
  - cadence
  - restore procedure
  - explicit off-host storage requirement
- roadmap maps `G1` through `G5` to current merged workstreams and explicitly remains subordinate to `NEXT_ACTIONS.md`
- recoverable-vs-fatal policy is codified in governance docs via the merged `AGENTS.md` update
- no secrets or sensitive values appear in the new docs:
  - `v1-path pattern count: 0`
  - `hex32plus pattern count: 0`

## Implementation / CI Nuance Summary

- the continuity canon replaced host-local practice with checked-in guidance without touching runtime or CI workflow surfaces
- the roadmap explicitly states it is strategic and cannot reorder or override the live queue
- the implementation lane completed with all 34 protected checks green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Exact Commands / Checks Run For The Merged Implementation Lane

- `git diff --name-only origin/main...HEAD`
- `BASE_SHA=$(git rev-parse --verify origin/main^{commit})`
- `HEAD_SHA=$(git rev-parse --verify HEAD^{commit})`
- `EVENT_FILE=$(mktemp /tmp/goal_lint_na0218_XXXXXX.json)`
- `python3 - <<'PY' "$EVENT_FILE" "$BASE_SHA" "$HEAD_SHA" ...`
- `GITHUB_EVENT_PATH="$EVENT_FILE" python3 tools/goal_lint.py`
- `git ls-files 'tests/*.md' | wc -l`
- `git ls-files 'tests/**/*.md' | wc -l`
- `git ls-files 'docs/*.md' | wc -l`
- `git ls-files 'docs/**/*.md' | wc -l`
- `python3 - <<'PY' ...` deterministic local-link existence check from `AGENTS.md`
- `python3 - <<'PY' ...` added-path leak-safe scan for `v1-path pattern` and `hex32plus pattern`
- `gh pr diff 647 --repo QuantumShieldLabs/qsl-protocol --name-only`
- `gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/e6472bf631e5e31315b4360096ea9181940f17f9/check-runs?per_page=100" -H "Accept: application/vnd.github+json"`
- `gh api "/repos/QuantumShieldLabs/qsl-protocol/pulls/647" -H "Accept: application/vnd.github+json"`

## Why NA-0218 Is Complete And Why NA-0219 Is Next

- `NA-0218` is complete because refreshed merged main now carries the continuity/runbook/roadmap canon, the matching governance updates, and the docs-only validation evidence required to make those artifacts durable and auditable
- `NA-0219` is next because the remaining docs/governance gap is not more continuity implementation; it is continuous operational memory plus getting the prepared director-ready audit program into checked-in repo canon so future audit/remediation work relies less on off-line handoff context

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, qsc-desktop, qsl-server, or qsl-attachments paths change in this closeout.
