# DOC-DEV-004 — Public Demo Runbook (Metadata Visibility + Minimization) v0.1.0 DRAFT

Goals: G4, G5

Status: DRAFT
Date: 2026-01-22
Owner: QSL governance
Audience: INTERNAL (Engineering / Demo)
Scope: Public demo/client track (qsl-tui + scripts/demo) — governance and execution discipline only

## 1. Purpose (authoritative)
This runbook is the single authoritative checklist for executing and extending the public demo/client track without drift:
- deterministic operating procedure each session
- strict scope discipline
- slow-machine build/CI constraints
- bounded CI wait behavior (no `--watch`)
- “claims discipline” for metadata minimization statements

## 2. Non-negotiable invariants
- **Protocol/wire/crypto unchanged** for demo/client work unless an explicit protocol NA says otherwise.
- **Fail-closed** behavior preserved: no silent downgrades; no “best effort” acceptance on ambiguous states.
- **Uniform rejects** at probeable boundaries: rejection behavior must not encode “why” via variable strings, timing, or partial state.
- **No mutation on reject**: rejected inputs must not advance state or consume one-time material.
- **Scope guard required**: stop immediately on out-of-scope file changes.
- **Bounded waits**: no indefinite polling; stop with evidence when time bound is hit.

## 3. Intended code/document surfaces (normal in-scope)
Typical demo/client work should be confined to:
- `apps/qsl-tui/**`
- `scripts/demo/**`
- `docs/test/**` (demo test plan + expected markers)
- governance artifacts when required: `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`
- `Cargo.lock` only when explicitly approved by the NA scope

## 4. Product posture (security-by-default)
- **Secure-by-default** is the expected user posture.
- Any “baseline” behavior is **internal comparison only** and must be clearly labeled as non-default and non-recommended.

## 5. Metadata surfaces (what we can and cannot control)
We treat metadata as multiple surfaces:
- Message size / attachment shape (client can mitigate via bucketing/padding)
- Timing and traffic analysis (hardest; relay/network layer dominates)
- IP exposure (client can route via proxy/Tor-friendly connectivity; still not a full anonymity system)
- Identifiers and linkability (often service/relay dependent)

For the public demo, near-term measurable surfaces:
- **Size**: plaintext_len vs ciphertext_len, bucket/padding outcome
- **Connectivity framing**: proxy/Tor-friendly path documented
- **Timing**: bounded waits; do not exaggerate “timing privacy” claims

## 6. Privacy envelopes (recommended mitigation pattern)
A “privacy envelope” is the combination of:
- **Tick schedule**: traffic is emitted/processed on a fixed cadence where feasible (reduces timing leakage but costs latency/energy).
- **Size buckets**: payloads are padded to a bounded set of sizes (reduces length leakage).
- **Bundle packing**: multiple logical units may be packed into a single envelope when policy allows (reduces per-message metadata).

Operational guidance:
- Default demo posture should live inside a defined envelope.
- Always disclose tradeoffs (latency/throughput/battery/UX) in internal notes and public claims.

## 7. Receipt/ACK camouflage (roadmap item; do not over-claim)
Receipts/ACKs are a common metadata leak surface (linkability and timing).
Roadmap direction (not necessarily implemented at any moment):
- Pad/camouflage ACKs to match envelope sizes (or co-pack with scheduled ticks).
- Randomized delay within a bounded window (must remain bounded and testable).
- Avoid explicit “delivery vs read” semantics leaking via traffic shape.

Claims discipline:
- Do not claim ACK camouflage is active unless the demo output and test plan explicitly prove it.

## 8. Logging/metrics privacy budget (required discipline)
Default posture: **minimal logs** and **bounded analytics**.
- No stable identifiers in logs unless explicitly bucketed/anonymized.
- No raw payload sizes; report only bucket IDs or bounded categories.
- Retention must be short and explicit; aggregation should be coarse by default.
- New metrics require an explicit “budget” statement: what is collected, why, retention, and expected risk.

## 9. Definition of Done (demo/client PR)
A demo/client PR is DONE only when:
- Scope guard passes (no out-of-scope paths).
- CI checks attach and are green (goal-lint + required pipelines).
- Demo scripts run and emit stable, machine-readable markers (when applicable).
- **Uniform reject** behavior is preserved/extended (where relevant).
- **No-mutation-on-reject** regression tests exist for any probeable state boundary touched.
- Governance evidence updated when required (DECISIONS + TRACEABILITY + test plan).

## 10. Session execution checklist (copy/paste discipline)
### 10.1 Start from clean main
- `cd /home/victor/work/qsl/qsl-protocol`
- `git checkout main && git pull --ff-only`
- `git status --porcelain=v1` must be empty
- Create a feature branch.

### 10.2 Scope guard (mandatory)
Declare an allowlist of paths up front and stop immediately if anything else changes.

### 10.3 Slow-machine notes (operational reality)
- Prefer package-scoped builds/tests where possible.
- Use large cargo timeouts and sparse registry when building.
- Avoid long-running `gh ... --watch` calls.

### 10.4 Bounded CI wait (required)
Poll check-runs every 20 seconds up to 60 minutes:
- Stop if failures appear.
- Stop (no merge) if still running after the bound.
- Merge only on explicit instruction.

## 11. Claims discipline (website alignment)
Do not claim “metadata eliminated.”
Do claim “metadata minimized” with explicit scope:
- Now: size bucketing/padding + proxy/Tor-friendly connectivity framing
- Next: privacy envelopes hardened; receipts/ACK camouflage; relay batching/jitter experiments (design-first)
- Later: stronger traffic-analysis resistance primitives (mixing/mixnet class)
## Quoting-safe directive template

Date: 2026-01-25

This project has repeatedly encountered avoidable failures caused by shell quoting pitfalls.
To prevent recurrence, all Codex-run directives MUST follow these rules.

### Mandatory rules
- Do NOT nest heredocs.
- Do NOT pipe a heredoc-fed python3 block into tee.
- Do NOT embed backticks in shell output or strings.
- Prefer file-based JSON parsing over piping.
- Write artifacts only under /home/victor/work/qsl/_forensics/...
- Use exactly ONE top-level logger (exec > >(tee ...)) per directive.

### Reference template (required; indented code block)

    set -euo pipefail

    OUT_DIR="/home/victor/work/qsl/_forensics/<task>_$(date -u +%Y%m%dT%H%M%SZ)"
    mkdir -p "$OUT_DIR"
    LOG="$OUT_DIR/run.log"
    exec > >(tee -a "$LOG") 2>&1

    stop_bundle () {
      rc="$1"
      echo "FAILED_COMMAND: ${BASH_COMMAND:-<unknown>}"
      tail -n 200 "$LOG" || true
      git status --porcelain=v1 || true
      echo "BRANCH=$(git rev-parse --abbrev-ref HEAD || true)"
      echo "HEAD=$(git rev-parse HEAD || true)"
      exit "$rc"
    }
    trap 'stop_bundle $?' ERR

### Python usage rule
All Python blocks MUST:
- Use python3 <<'PY' (single-quoted heredoc).
- Receive inputs via environment variables.
- Write outputs to files (never rely on stdout piping).

Example:

~~~bash
export SOME_PATH="$OUT_DIR/input.json"
python3 - <<'PY'
import os, json, pathlib
p = pathlib.Path(os.environ["SOME_PATH"]) 
data = json.loads(p.read_text("utf-8"))
print("ok")
PY
~~~

Violations of these rules MUST STOP and be corrected before proceeding.
