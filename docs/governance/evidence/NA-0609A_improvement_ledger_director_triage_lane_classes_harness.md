Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609A — Cross-Lane Improvement Ledger, Director Triage Discipline, and Lane-Class Definitions

## Summary

NA-0609A is an inserted governance/tooling lane executed under directive
QSL-DIR-2026-07-06-542 (D542). It does not implement NA-0609. It makes cross-lane
continuity and queue-to-roadmap alignment durable and repo-backed, and reduces
per-lane ceremony for low-risk work without weakening fail-closed rails.

Motivation: every lane runs as a fresh assistant in a different qwork workspace
path, so per-session assistant memory does not propagate; the committed repo is
the only reliable continuity channel. Findings and process recommendations
therefore needed a committed, triageable home.

Result classification: `IMPROVEMENT_LEDGER_AND_LANE_DISCIPLINE_ESTABLISHED`.

This is a governance/docs lane only. It is not a protocol, wire, crypto,
state-machine, or security-behavior change. It makes no public-readiness,
production-readiness, security-completion, crypto-complete, attachment-complete,
vulnerability-free, or bug-free claim.

## Required Markers

- NA0609A_D1209_CONSUMED_OK
- NA0609A_D1210_CONSUMED_OK
- NA0609A_FRESH_QWORK_PROOF_OK
- NA0609A_CURRENT_MAIN_HEALTH_OK
- NA0609A_D1211_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0609A_LEDGER_CREATED_DOC_OPS_007_OK
- NA0609A_LEDGER_SEEDED_CLASS_ONLY_OK
- NA0609A_MANDATORY_READ_WIRING_OK
- NA0609A_DIRECTOR_TRIAGE_DISCIPLINE_OK
- NA0609A_LANE_CLASSES_DEFINED_OK
- NA0609A_FAIL_CLOSED_BOUNDARY_PRESERVED_OK
- NA0609A_NEXT_ACTIONS_INSERTED_IN_PROGRESS_OK
- NA0609A_ONE_READY_INVARIANT_OK
- NA0609A_NO_PROTOCOL_OR_SOURCE_MUTATION_OK
- NA0609A_NO_SETTINGS_OR_HOOK_MUTATION_OK
- NA0609A_BOUNDARY_MUTATION_OK
- NA0609A_PRIVATE_MATERIAL_SCAN_OK
- NA0609A_RESULT_CLASSIFICATION_SELECTED_OK
- NA0609A_SUCCESSOR_NA0609_SOLE_READY_OK

## Qwork, Queue, And Main Gates

Operator-run qwork proof for lane NA-0609 from `2026-07-06T21:50:33Z` was
verified before any mutation, GitHub metadata, PR creation, or proof publication.
Live pre-fetch HEAD and origin/main matched at `586f4c6c7272`; startup proof
classified worktree/index/untracked clean; root disk usage below the stop
threshold and `/backup/qsl` mounted; READY_COUNT 1 with READY NA-0609. Decision
proof classified D-1209 once, D-1210 once, D-1211 absent. NA-0609A was absent
from `NEXT_ACTIONS.md` before this patch.

## Inheritance

D-1209 (NA-0608 implementation) and D-1210 (NA-0608 closeout) were each consumed
once and Accepted. This lane preserves all prior fail-closed rails and claim
boundaries; it adds guidance and does not remove any rule or gate.

## Changes

- `docs/ops/IMPROVEMENT_LEDGER.md` (new; assigned DOC-OPS-007): the committed,
  cross-lane backlog. Defines the entry schema (engineering findings in the
  DOC-AUD-001 §6 shape; workflow items), the status lifecycle, the class-only
  rule, and how every lane reads/files entries. Seeded with ENG-0001 and ENG-0002
  (found while driving NA-0608, framed as audit-needed, not confirmed defects) and
  WF-0001..WF-0003 (the continuity, ceremony, and triage recommendations, marked
  done by this lane). No secrets, endpoints, tokens, capabilities, keys,
  plaintext, or ciphertext appear.
- `CLAUDE.md`, `START_HERE.md`, `AGENTS.md`: the ledger is added to the read-first
  lists, and `AGENTS.md` gains a binding rule that every lane reads the ledger and
  files/updates an entry before closeout when it discovers or advances a finding
  or process issue. Additive only.
- `docs/ops/DIRECTOR_OPERATIONS.md` (DOC-OPS-006): adds §8 Director triage
  discipline (each Director turn reads the ledger and the DOC-PROG-001 release
  gates and justifies successor selection against them, without overriding
  `NEXT_ACTIONS.md`) and §9 Lane classes (WAVE and LITE-CEREMONY), each with a
  hard fail-closed boundary excluding anything touching protocol/wire/crypto/auth/
  state-machine/security, dependencies, lockfiles, workflows, branch protection,
  the public-safety/advisories gates, or runtime/LAN action.
- `NEXT_ACTIONS.md`: NA-0609A inserted as IN_PROGRESS above NA-0609; NA-0609
  remains the sole READY item (READY_COUNT stays 1).

## Boundary Review

Implementation mutates only `docs/ops/IMPROVEMENT_LEDGER.md`,
`docs/ops/DIRECTOR_OPERATIONS.md`, `START_HERE.md`, `AGENTS.md`, `CLAUDE.md`,
`NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, this evidence doc, and the NA-0609A
testplan(s). No qsc/qsl-server/qsl-attachments source, test, dependency,
lockfile, or workflow mutation occurred. No protocol/wire/crypto/state-machine
behavior change occurred. No `.claude/settings.json` or guardrail-hook edit
occurred. No qwork/qstart/qresume execution, sudo, systemd, firewall, package
install, Tailnet/Tailscale, workflow dispatch/rerun, branch-protection or
repo-settings mutation, or runtime/LAN action occurred. No fail-closed gate,
one-READY invariant, evidence requirement, or claim boundary was removed or
narrowed; all changes are additive guidance.

## Private-Material Review

Private-material review classified pass for repository publication. No endpoint
value, private port value, hostname, topology value, token value, capability
value, payload/body/plaintext, ciphertext body, seed, key material, personal
path, raw command line, or raw log is published; only class summaries and
governance path constants appear.

## Result And Successor

Selected result: `IMPROVEMENT_LEDGER_AND_LANE_DISCIPLINE_ESTABLISHED`.
NA-0609A is closed out under D-1212; NA-0609 remains the sole READY successor and
begins at D-1213.

## Claim Boundary

No public-readiness, production-readiness, remote-ready, Tailnet-ready,
LAN-ready, security-completion, crypto-complete, attachment-complete,
metadata-free, anonymity, vulnerability-free, or bug-free claim is introduced.
This lane adds process discipline only; it does not itself establish any security
or release property.
