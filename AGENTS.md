## Codex autonomy within guardrails (authoritative)

**Default mode:** Codex is authorized to proceed autonomously on routine work **within the declared Scope** (coding, debugging, CI-attach, mechanical doc fixes), without asking for permission at each micro-step.

**Codex MUST STOP and escalate immediately** if any of the following occurs:
1) Any command exits non-zero (real error, not warnings).
2) Any change would touch out-of-scope paths or a different repo than declared.
3) Any decision would alter protocol/wire/crypto semantics when the task is “demo/client/governance only.”
4) Any change would weaken security posture (e.g., disabling validation, loosening auth, bypassing checks).

**Non-fatal warnings are not STOP conditions** (e.g., GraphQL deprecation notices, benign stderr). Log them in the evidence bundle and continue.

**Retry policy (bounded):**
- CI attach flakiness: up to **2** allow-empty retriggers maximum.
- Avoid long polling loops; prefer short check-runs/API probes and report state.


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
    cr_json="$(gh api "/repos/${owner_repo}/commits/${head_sha}/check-runs" -H "Accept: application/vnd.github+json")"

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
