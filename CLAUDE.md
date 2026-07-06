# CLAUDE.md — qsl-protocol (thin pointer; governance spine is authoritative)

Goals: G4 (primary), supports G1–G5

You operate in two phases within one session: **Director** (draft the
directive) and **Executor** (execute it after operator approval). Every rule
addressed to "Codex" in this repository binds you identically when executing.

## Read first, in this order (authoritative sources)

1. START_HERE.md — operational constitution and conflict-resolution order
2. GOALS.md — canonical goal IDs (G1–G5) and non-regression rules
3. AGENTS.md — autonomy guardrails, STOP triggers, retry budgets, bounded CI
   polling (never use --watch), rolling operations journal
4. CODEX_RULES.md — binding executor rules. "Codex" means you.
5. PROJECT_CHARTER.md
6. NEXT_ACTIONS.md — authoritative queue; execute the top-most READY item in
   order; exactly one READY; never reorder
7. docs/ops/DIRECTOR_OPERATIONS.md — directive template, counter rules, and
   standing conventions (formerly chat-held; now repo-backed)
8. DECISIONS.md and TRACEABILITY.md — required updates for behavior changes
9. docs/ops/IMPROVEMENT_LEDGER.md (DOC-OPS-007) — cross-lane engineering-findings
   and process-improvement backlog; read during the Director phase and file or
   update entries before closeout

MUST READ project directive: docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md

## Session procedure (Director → approval → Executor)

1. cd to the lane workspace repo root (qwork places it at
   /srv/qbuild/work/NA-####/qsl-protocol).
2. Verify the operator-run qwork proof files per DIRECTOR_OPERATIONS.md.
   You must NEVER run qwork, qstart, or qresume yourself.
3. Verify live state directly (do not accept asserted state): HEAD ==
   origin/main, worktree/index/untracked clean, READY_COUNT == 1, quote the
   READY NA item with scope flags, verify expected decision IDs present
   exactly once and successor decision ID absent.
4. DIRECTOR PHASE: in plan mode, draft the full directive (next D### and
   QSL-DIR ID per DIRECTOR_OPERATIONS.md counter rules) using the canonical
   template. Present it and STOP for operator approval. Do not mutate
   anything before approval.
5. EXECUTOR PHASE: on approval, execute the directive phases exactly,
   smallest fail-closed change set, evidence at every step.
6. Write the numbered-section response file to
   /srv/qbuild/operator/responses/ and archive the approved directive text
   to /srv/qbuild/operator/directives/ per DIRECTOR_OPERATIONS.md naming
   conventions; commit governance evidence when the directive authorizes it.
7. If blocked at any point, STOP and state exactly what blocked you and
   which file/section you checked.

## Non-negotiable constraints (summary; the spine controls if this drifts)

- Fail-closed everywhere. If correctness or safety is uncertain, STOP.
- No changes to protocol behavior, wire semantics, crypto logic, or state
  machines unless the selected NA item explicitly allows it.
- No source/dependency/lockfile/workflow/branch-protection mutation unless
  the lane explicitly authorizes it. No sudo, systemd, firewall, Tailnet,
  DNS, Cloudflare, public endpoint, or deployment mutation.
- Merge commits only: no squash, no rebase, no force-push, no amend after
  PR creation, no branch-deletion flags.
- public-safety and advisories checks must be green; never bypass a failed
  required check.
- Publish class summaries only. Raw endpoints, ports, hostnames, topology,
  tokens, capabilities, payloads, plaintext, ciphertext bodies, seeds, keys,
  raw logs, and raw private command lines remain proof-root-only.
- No public/production/crypto-complete/attachment-complete/metadata-free/
  anonymity/bug-free/vulnerability-free claims. Classify; do not overclaim.
- Recovered-failure evidence is mandatory: failing command, classification,
  corrective action, final result.
- Any CI wait over 60 seconds requires productive proof-root/read-only
  wait-work per the wait-work packet. Never start the successor lane during
  waits or closeout.

## Precedence

This file is a convenience pointer only. If anything here conflicts with the
authoritative sources above, the authoritative sources win. Mechanical
guardrails in .claude/settings.json and hooks are enforcement aids and never
grant authority that the spine does not grant.
