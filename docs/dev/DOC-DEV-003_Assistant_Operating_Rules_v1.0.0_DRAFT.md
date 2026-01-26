# DOC-DEV-003 — Assistant Operating Rules (Project Directive) v1.0.0 DRAFT


## Working directory (mandatory)

If you started Codex in `~/work/qsl`, first change into the **qsl-protocol repo root**:

    cd ~/work/qsl/qsl-protocol

All relative paths in directives assume you are running from the repo root above.


Status: DRAFT
Date: 2026-01-19
Owner: QSL governance
Applies to: All assistants and Codex sessions in this project

## 1. Non-negotiable operating directive

### 1.1 Continuous improvement (“perfection” standard)
- If an improvement is clearly safe and in-scope, implement it immediately.
- If an improvement could change scope, security posture, protocol behavior, or create churn: STOP and request explicit approval with clear options.

### 1.2 Codex directives must be code-block complete
- Every directive sent to Codex MUST be fully contained in a single code block.
- If a directive is long, split it into multiple directives; each directive must be in its own code block.
- No directive content may appear outside a code block.

### 1.3 Mandatory directive structure (minimum)
Each Codex directive MUST include:
- Objective (what “done” means)
- Allowed scope (paths allowed to change)
- Stop conditions (when to halt and report)
- Verification bundle (what evidence must be pasted back)

## 2. Security invariants (project baseline)
- Fail-closed on rejects; deterministic error behavior where specified by policy.
- “No mutation on reject” for any stateful reject path.
- CodeQL: CI is authoritative; local targeted queries are used as fast regression checks.

## 3. Process invariants
- Clean-tree preflight for any verification directive.
- If unexpected diffs appear: stop, salvage if needed, and resume from a clean base.

---

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
## Mandatory State Ledger

To prevent acting on the wrong Next Action item or wrong PR, every Director response that issues a Codex directive MUST include a
State Ledger, proven from `main` in the same session (read-only).

Required fields (all must be present):
- MAIN_HEAD (git rev-parse HEAD on main)
- READY_COUNT and READY_NA (derived from NEXT_ACTIONS.md on main)
- READY block path or excerpt (verbatim)
- Active PR(s) being acted on (if any)
- Next directive number + purpose (one-line)
- Confirmation: “No other NA is READY.”

Hard rule:
- If there is any ambiguity (user confusion, repeated messages, uncertain PR/NA), the Director MUST issue a read-only “state reset”
  directive before any further work.
## Codex diagnosis rule (blocked/unclear situations)

When the workflow becomes blocked or ambiguous (examples: mergeStateStatus=BLOCKED, missing required check contexts, unexpected CI failure,
stale PR checks, unexplained tool errors), the Director MUST issue a Codex read-only diagnostic directive before proposing a fix.

Minimum required diagnostic outputs (as applicable):
- PR head SHA, mergeStateStatus, reviewDecision
- Required vs actual check contexts
- Failing job log URLs and extracted error lines
- Scope guard evidence (name-only diff)
- Current READY_NA and READY block

Rationale: this prevents guessing and accelerates root-cause analysis by using repo-local evidence.

