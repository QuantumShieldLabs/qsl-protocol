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

Project directive (must follow):
- Continuous improvement: implement safe/in-scope improvements immediately; ask approval if unsure.
- ALL Codex directives must be fully contained in a single code block (or split into multiple code-block directives).
- See docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md.
- For public demo/client work: read docs/dev/DOC-DEV-004_Public_Demo_Runbook_v0.1.0_DRAFT.md first and follow it.


# DEPRECATED — Superseded by NEXT_ACTIONS.md (New Chat Starter)


## Working directory (mandatory)

If you started Codex in `~/work/qsl`, first change into the **qsl-protocol repo root**:

    cd ~/work/qsl/qsl-protocol

All relative paths in directives assume you are running from the repo root above.


This file is retained only for backwards compatibility with older chat workflows.

**Do not paste this file into new chats.**

Where to look now:
- The authoritative “New Chat Starter” block lives at the top of **NEXT_ACTIONS.md**.
- **START_HERE.md** is the operational constitution and the first mandatory read.
- **GOALS.md / AGENTS.md / PROJECT_CHARTER.md / DECISIONS.md / TRACEABILITY.md** are the authoritative governance spine.

If any tool, script, or prior note instructs you to use this file as the chat starter, treat that instruction as stale.
- Start every new chat with a read-only state reset: sync main, prove READY_NA, and print the READY block before issuing any implementation directive.
