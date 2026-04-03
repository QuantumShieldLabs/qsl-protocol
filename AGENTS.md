## Codex autonomy within guardrails (authoritative)

**Default mode:** Codex is authorized to proceed autonomously on routine work **within the declared Scope** (coding, debugging, CI-attach, mechanical doc fixes), without asking for permission at each micro-step.

**Codex MUST STOP and escalate immediately** if any of the following occurs:
1) Any change would touch out-of-scope paths or a different repo than declared.
2) Any decision would alter protocol, wire, crypto, auth, state-machine, or security semantics outside the declared scope.
3) Any change would weaken security posture (e.g., disabling validation, loosening auth, bypassing checks).
4) Any destructive or history-rewriting action would be required.
5) Live repo, ref, governance, or directive contradictions block truthful continuation.
6) Required CI conclusively fails after the bounded retry budget.
7) Root cause is unclear enough that continuing would risk untruthful evidence or behavior drift.

**Non-zero command policy:** a non-zero exit is **not** an automatic hard stop. Classify it first, then either recover in place within budget or STOP.

**Recoverable in place (bounded):**
- command-shape / CLI usage mistake: **1** immediate self-correction
- valid zero-match discovery/proof outcome: recover with zero-failure-safe tooling and record the zero result
- in-scope local build/test/lint/docs validation failure with understood cause: up to **3** fix/rerun cycles for the same root cause
- transient `gh`, network, API, or tool invocation issue: up to **2** retries
- stale/flaky CI jobs: up to **2** reruns total

**Never recover past these boundaries:**
- scope, security, authority, destructive, or contradiction triggers listed above
- any failure whose corrective action would require out-of-scope edits or queue reordering
- any failure whose corrective action would dilute fail-closed behavior

**Recovered-failure evidence is mandatory.** Record:
- failing command
- why it was classified as recoverable
- corrective action taken
- final result

**Non-fatal warnings are not STOP conditions** (e.g., GraphQL deprecation notices, benign stderr). Log them in the evidence bundle and continue.


**Waiting + long-running operations (authoritative):**
- **One goal per directive.** Do not combine “merge” and “post-merge evidence capture” if it risks timeouts.
- **Never use `--watch`.** Do not use `gh pr checks --watch` (or other watch modes) in directives.
- **Use bounded REST polling for checks.** Sleep 20s; cap at 180 iterations (60 minutes). Report state and STOP if not complete.
- **Make “post-merge evidence” a separate directive** when it may require large API payloads (PR file lists, logs, etc.).

**Canonical helper snippet (copy/paste into directives):**
~~~bash
wait_for_pr_checks() {
  # Usage: wait_for_pr_checks <OWNER_REPO> <PR_NUM> [interval_s=20] [max_iters=180]
  local owner_repo="$1"
  local pr_num="$2"
  local interval="${3:-20}"
  local max_iters="${4:-180}"

  local head_sha
  head_sha="$(gh api "/repos/${owner_repo}/pulls/${pr_num}" --jq '.head.sha')"
  echo "wait_for_pr_checks: owner_repo=${owner_repo} pr=${pr_num} head_sha=${head_sha}"

  for i in $(seq 1 "${max_iters}"); do
    local cr_json total succ inprog fails
    cr_json="$(gh api "/repos/${owner_repo}/commits/${head_sha}/check-runs?per_page=100" -H "Accept: application/vnd.github+json")"

    total="$(python3 -c 'import json,sys; d=json.load(sys.stdin); print(d.get("total_count",0))' <<<"${cr_json}")"
    succ="$(python3 -c 'import json,sys; d=json.load(sys.stdin); print(sum(1 for r in d.get("check_runs",[]) if r.get("conclusion")=="success"))' <<<"${cr_json}")"
    inprog="$(python3 -c 'import json,sys; d=json.load(sys.stdin); print(sum(1 for r in d.get("check_runs",[]) if r.get("status")!="completed"))' <<<"${cr_json}")"
    fails="$(python3 -c 'import json,sys; d=json.load(sys.stdin); print(sum(1 for r in d.get("check_runs",[]) if r.get("status")=="completed" and r.get("conclusion") not in ("success","neutral","skipped"))) ' <<<"${cr_json}")"

    echo "ITER=${i}/${max_iters} TOTAL=${total} SUCCESS=${succ} INPROG=${inprog} FAILS=${fails}"

    # “Attached” means total>0. If none attached yet, keep waiting.
    if [ "${total}" -gt 0 ] && [ "${inprog}" -eq 0 ] && [ "${fails}" -eq 0 ]; then
      echo "DONE: checks attached and all green."
      return 0
    fi

    sleep "${interval}"
  done

  echo "STOP: checks not complete/green after bounded wait."
  return 2
}
~~~

**Perfection directive:**
If Codex notices an improvement that increases correctness, safety, clarity, or testability **within scope**, it should implement it; if unsure whether it is acceptable, STOP and ask.


Goals: G4

# AGENTS.md (Repository Agent Policy)


## Working directory (mandatory)

If you started Codex in `~/work/qsl`, first change into the **qsl-protocol repo root**:

    cd ~/work/qsl/qsl-protocol

All relative paths in directives assume you are running from the repo root above.


This file governs automated assistants (including Codex) operating on this repository.

## Mandatory reads (before any changes)
1. GOALS.md
2. PROJECT_CHARTER.md
3. DECISIONS.md
4. TRACEABILITY.md
5. CHECKLIST_PROTOCOL_CHANGE.md (when relevant)

## Non-negotiable rules
- Every change MUST advance at least one Goal ID (G1–G5) and MUST NOT regress others.
- Prefer fail-closed semantics: if uncertain, abort/reject rather than accept/continue.
- No silent downgrades: any fallback must be explicit, negotiated, and documented.
- Protocol behavior changes MUST include tests/vectors in the same PR.
- Any change to state machines, key schedules, negotiation, or wire semantics MUST add a DECISIONS.md entry and update TRACEABILITY.md.
- Keep specs self-contained: do not introduce required meaning via external references.

## Required PR / task metadata
Every task/PR description MUST include:
- **Goals:** G1, G2, ... (at least one)
- **Impact:** short description of how goals are advanced
- **No-regression:** statement of invariants preserved
- **Tests/Vectors:** what was added/updated

## Goal-lint PR body requirements (exact)
- Include a standalone line in the PR body: `Goals: G1, G2, ...`
- Use ASCII commas, no ranges (e.g., avoid `G1–G5` or `G1-G5`).
- Keep the Goals line near the top so goal-lint can detect it reliably.

## If repository layout differs
If the repository does not contain the referenced files, create them at repo root.
If CI/workflow integration requires path tuning, do so without weakening enforcement intent.

## Assistant operating rules (project directive)
- All assistants and Codex sessions MUST follow docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md.
- In particular: every Codex directive must be fully contained in a code block (or split into multiple code-block directives).

## Post-fix hardening review (mandatory)
After any issue fix, the assistant MUST complete and report a post-fix hardening review before declaring the work complete. The review MUST include:
1. Correctness under stress.
2. Minimality (no unintended behavior changes).
3. Maintainability (clear, reusable, low-complexity fix shape).
4. Coverage quality (tests fail for the right reasons; no superficial pass).
5. Cross-lane stability (macOS/Linux consistency for affected areas).

## Queue successor requirement (mandatory)
For any governance close-out directive, if the current READY item is the last NA block with no successor, the directive MUST either:
1. Include an approved successor NA block to append in the same close-out, or
2. STOP before editing governance files and request explicit successor direction.

Do not invent new NAs without explicit block text provided in the directive.

## CI dependency policy for scripts/ci (mandatory)
CI shell scripts must use POSIX shell plus coreutils and `grep`/`awk`/`sed` only, unless the workflow explicitly installs additional tools.

## Documentation hygiene guardrails (mandatory)

### Doc placement rules
- Governance spine documents belong at repository root:
  `START_HERE.md`, `GOALS.md`, `AGENTS.md`, `PROJECT_CHARTER.md`, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`.
- Canonical specs belong under `docs/canonical/**`.
- Active docs navigation belongs in `docs/INDEX.md` (single docs front door).
- Active supporting guidance and runbooks belong under `docs/**` (prefer existing folders; avoid creating new top-level doc buckets without a directive).
- Historical/superseded plans and audits belong under `docs/archive/**`, including `docs/archive/testplans/**`.
- `tests/` may contain executable tests and minimal harness docs that are directly consumed by tests; planning markdowns do not belong in `tests/` and must live in `docs/archive/testplans/**` or `docs/**`.

### Doc classification header template (for new docs)
Add this near the top of new standalone docs unless the file is a tiny subordinate README already linked from `docs/INDEX.md`:

```md
Status: Authoritative | Supporting | Archive
Owner: <team-or-role>
Last-Updated: YYYY-MM-DD
Replaces: <optional-path-or-id>
Superseded-By: <optional-path-or-id>
```

### Evidence and scan conventions (safe wording)
- In governance/docs evidence text, use short SHAs (12 hex) unless a tool explicitly requires full SHAs.
- Do not embed literal sensitive path tokens in evidence prose; use descriptive wording such as `v1-path pattern`.
- For long-hex scan references, use descriptive wording such as `hex32plus pattern`; do not paste regexes or long hex sequences.
- Leak scans primarily target runtime/test-visible output and deterministic markers; docs should still avoid unnecessary secret-like identifiers.

### Correct inventory examples
Use both commands when inventorying markdown under `tests/`:

```bash
git ls-files 'tests/*.md'
git ls-files 'tests/**/*.md'
```

Relying only on the nested pattern can miss markdown files directly under `tests/`.

### Manual docs link-integrity check (runbook)
Run this sequence after doc moves/archives and before opening a docs PR.

1) Inventory markdown files (root-level and recursive counts):

```bash
git ls-files 'tests/*.md' | wc -l
git ls-files 'tests/**/*.md' | wc -l
git ls-files 'docs/*.md' | wc -l
git ls-files 'docs/**/*.md' | wc -l
```

2) Deterministic local-link existence check (relative markdown links only):

```bash
python3 - <<'PY'
import pathlib, re

repo = pathlib.Path(".").resolve()
md_files = []
for pattern in ("*.md", "**/*.md"):
    for p in repo.glob(pattern):
        if ".git/" in p.as_posix():
            continue
        if p.is_file():
            md_files.append(p)
md_files = sorted(set(md_files))

link_re = re.compile(r'\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)')
missing = []

for md in md_files:
    text = md.read_text(encoding="utf-8", errors="replace")
    for raw in link_re.findall(text):
        target = raw.strip()
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        abs_target = (md.parent / target).resolve()
        if not abs_target.exists():
            missing.append((md.relative_to(repo).as_posix(), target))

for src, target in missing:
    print(f"MISSING_LINK {src} -> {target}")
print(f"TOTAL_MISSING {len(missing)}")
PY
```

3) Redirect index update reminder:
- If paths are moved to archive, update the relevant archive index in the same PR with old-path -> new-path mapping entries.

4) Evidence wording reminder:
- Avoid literal endpoint fragments in governance evidence text; use descriptive wording such as `v1-path pattern`.
- Use short SHAs in narrative evidence unless tooling explicitly requires full SHAs.

### Docs PR checklist (copy/paste into PR body)
Use this checklist for docs-only and docs-heavy PRs:

```md
- [ ] Scope proof included (`gh pr diff <PR#> --name-only`) and scope is docs/policy only.
- [ ] Placement rule check complete (root vs `docs/` vs `docs/archive/` vs `tests/`).
- [ ] Inventory evidence includes both root and recursive markdown patterns:
      - `git ls-files 'tests/*.md'`
      - `git ls-files 'tests/**/*.md'`
- [ ] Manual link-integrity runbook executed; reported PASS/FAIL with summary counts only.
- [ ] Redirect discipline applied for every moved/renamed doc (archive index mapping updated in same PR).
- [ ] Leak-safe evidence wording used:
      - `v1-path pattern count: <n>`
      - `hex32plus pattern count: <n>`
- [ ] No sensitive endpoints, tokens, auth headers, route tokens, or long-hex dumps pasted in PR evidence.
```

### Docs hygiene audit cadence
- Cadence: run monthly.
- Also run before major release milestones and after bulk doc move/archive PRs.

### Audit procedure (deterministic)
1) Inventory markdown counts using both root and recursive patterns:

```bash
git ls-files 'tests/*.md' | wc -l
git ls-files 'tests/**/*.md' | wc -l
git ls-files 'docs/*.md' | wc -l
git ls-files 'docs/**/*.md' | wc -l
```

2) Run the manual link-integrity runbook in this file:
- See `Manual docs link-integrity check (runbook)` and record PASS/FAIL plus summary counts.

3) If any docs were moved/renamed:
- Update the relevant archive redirect index in the same PR with old-path -> new-path mapping.

### Docs hygiene audit evidence template (copy/paste)
```md
Docs hygiene audit evidence
- Base commit: <sha12>
- tests root md count: <n>
- tests nested md count: <n>
- docs root md count: <n>
- docs nested md count: <n>
- link-check result: PASS|FAIL
- missing-link count: <n>
- redirect index updated: yes|no|n/a
- v1-path pattern count: <n>
- hex32plus pattern count: <n>
- notes: <short summary>
```

### Docs Move Protocol (Example: Move/Archive a Markdown Doc Safely)

#### Why counts can lie (common pitfall)
- `tests/**/*.md` can miss markdown files directly under `tests/`.
- Always report both counts: `tests/*.md` and `tests/**/*.md`.

#### Step-by-step protocol (copy/paste)
1) Classify the doc (`Authoritative` | `Supporting` | `Archive`) and add the header if missing.
2) Move with `git mv` (prefer move over delete when uncertain).
3) If moving to archive, update or extend the relevant redirect index in the same PR.
4) Rewrite markdown links using deterministic search/update passes (`rg` + targeted edits) without changing document meaning.
5) Run `Manual docs link-integrity check (runbook)` from this file.
6) Produce PR evidence using the template below.

#### Docs Move PR Evidence Template (copy/paste into PR body)
```md
Docs move evidence
- Scope proof command: `gh pr diff <PR#> --name-only`
- Scope proof output location: <paste concise name-only output>
- tests root md count (`tests/*.md`): <n>
- tests nested md count (`tests/**/*.md`): <n>
- docs root md count (`docs/*.md`): <n>
- docs nested md count (`docs/**/*.md`): <n>
- link-integrity runbook result: PASS|FAIL
- redirect index updated: yes|no (+ file path)
- v1-path pattern count: <n>
- hex32plus pattern count: <n>
- SHA policy: short=12 only (no full SHAs)
```

#### Quarterly spot-audit norm
- Once per quarter, reviewers should require one docs-hygiene evidence snippet on any PR that moves or renames docs.
- This is process-only enforcement; no CI/tooling changes required.
