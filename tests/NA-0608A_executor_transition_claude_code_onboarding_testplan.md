# NA-0608A Executor Transition — Claude Code Onboarding Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G4 (primary), supports G1–G5

## Scope

This test plan records the Phase 0–4 verification markers for NA-0608A: the
one-time bootstrap exception, staged-file review and Tier 3 corrections, the
off-home layout migration, and the pre-commit guardrail matrix. It does not
implement NA-0608 and authorizes no source, workflow, dependency, lockfile,
Tailnet, public-site, deployment, or qscwork runtime change.

## Phase 0 — Bootstrap verification and main health

- `NA0608A_BOOTSTRAP_PWD_BRANCH_OK` — pwd `/srv/qbuild/work/NA-0608A/qsl-protocol`, branch main.
- `NA0608A_BOOTSTRAP_HEAD_EQ_ORIGIN_MAIN_OK` — HEAD == main == origin/main == baseline `ceea85ecb82f`.
- `NA0608A_BOOTSTRAP_MAIN_NOT_ADVANCED_OK` — main not advanced past the NA-0607 closeout merge.
- `NA0608A_BOOTSTRAP_CLEAN_TREE_OK` — worktree/index/untracked clean.
- `NA0608A_BOOTSTRAP_READY_COUNT_1_OK` — READY_COUNT 1; sole READY NA-0608.
- `NA0608A_BOOTSTRAP_NA0608A_ABSENT_OK` — NA-0608A heading absent pre-patch.
- `NA0608A_BOOTSTRAP_D1205_ONCE_OK`, `NA0608A_BOOTSTRAP_D1206_ONCE_OK`, `NA0608A_BOOTSTRAP_D1207_ABSENT_OK`, `NA0608A_BOOTSTRAP_DUP_DECISIONS_ZERO_OK`.
- `NA0608A_BOOTSTRAP_DISK_BELOW_95_OK`, `NA0608A_BOOTSTRAP_BACKUP_QSL_MOUNTED_OK`.
- `NA0608A_BOOTSTRAP_SEVEN_STAGED_FILES_OK` — all seven staged files present, readable, hashed.
- `NA0608A_MAIN_CHECKS_GREEN_OK` — public-safety, advisories, suite2-vectors, qsc-adversarial-smoke, CodeQL success; no failed/pending required checks.
- `NA0608A_BOOTSTRAP_EXCEPTION_RECORDED_OK` — one-time qwork-proof replacement recorded (D540 only).

## Phase 1 — Handoff inheritance

- `NA0608A_D539_FACTS_CONSUMED_OK` — NA-0607 PASS, PR #1488 merge `5637735c`, PR #1489 closeout merge `ceea85ec`.
- `NA0608A_COUNTER_RULES_CONSUMED_OK` — last D539; this D540; NA-0608 -> D541; decisions D-1207/D-1208 consumed; NA-0608 begins D-1209.
- `NA0608A_PROOF_TOOLS_SMOKE_OK` — decision_id_counter.py distinguishes canonical entries from prose (D-1205=1, D-1206=1, D-1207=0).

## Phase 2 — Staged transition file review and Tier 3 corrections

- `NA0608A_SETTINGS_VALID_JSON_OK` — settings.json parses; keys match Claude Code schema.
- `NA0608A_HOOK_BASH_N_OK`, `NA0608A_SETUP_BASH_N_OK` — scripts pass `bash -n`.
- `NA0608A_HOOK_JQ_PRESENT_OK` — jq dependency present.
- `NA0608A_CLAUDE_MD_PATHS_EXIST_OK` — all referenced spine paths exist.
- `NA0608A_TIER3_GH_VERSION_CORRECTED_OK` — DIRECTOR_OPERATIONS gh 2.93.0 -> 2.96.0.
- `NA0608A_TIER3_SETUP_REGEX_CORRECTED_OK` — highest-directive regex restricted to 3-digit form.
- `NA0608A_TIER3_DOCOPS006_ASSIGNED_OK` — DOC-OPS-006 header assigned.
- `NA0608A_NO_GUARDRAIL_WEAKENED_OK` — no deny rule or hook block removed or narrowed.

## Phase 3 — Off-home layout setup

- `NA0608A_LAYOUT_DIRS_CREATED_OK` — tools/claude, operator/responses, operator/directives exist (victor:victor).
- `NA0608A_HOOK_INSTALLED_0755_OK` — hook installed mode 0755, identical to staged.
- `NA0608A_LEGACY_COPY_COUNT_MATCH_OK` — 512 == 512.
- `NA0608A_HIGHEST_DIRECTIVE_539_OK` — reported highest directive number 539.
- `NA0608A_LEGACY_ORIGINALS_UNMODIFIED_OK` — spot-checked originals byte-identical.
- `NA0608A_DIRECTIVE_ARCHIVED_OK` — directive text archived.

## Phase 4 — Guardrail empirical verification (pre-commit)

- `NA0608A_HOOK_BLOCK_MATRIX_OK` — 12/12 required must-block classes BLOCK (exit 2).
- `NA0608A_HOOK_ALLOW_MATRIX_OK` — 4/4 required must-allow classes ALLOW (exit 0).
- `NA0608A_HOOK_EXTRA_TIER5_BLOCK_OK` — additional Tier-5 classes BLOCK.
- `NA0608A_DENY_LAYER_COVERAGE_OK` — permissions.deny second layer present for core classes.
- `NA0608A_SESSION_PICKUP_REQUIREMENT_RECORDED_OK` — live in-session hook pickup requires a session started with settings present; recorded.
- `NA0608A_HOOK_QUOTED_KEYWORD_LIMITATION_RECORDED_OK` — fail-closed false-positive on keyword-in-quoted-text recorded; no required must-allow affected.

## Phase 9 — Fresh-checkout guardrail re-verification (post-merge)

- `NA0608A_FRESH_CHECKOUT_SETTINGS_PRESENT_OK` — `.claude/settings.json` present in a fresh clone of main with no manual placement.
- `NA0608A_FRESH_CHECKOUT_SUDO_BLOCK_OK`, `NA0608A_FRESH_CHECKOUT_QWORK_BLOCK_OK`, `NA0608A_FRESH_CHECKOUT_GIT_STATUS_ALLOW_OK`.

## Boundary and claim markers

- `NA0608A_SCOPE_GUARD_OK` — only allowed implementation paths mutated.
- `NA0608A_NO_SOURCE_TEST_DEP_LOCK_WORKFLOW_MUTATION_OK` — no qsc/qsl-server/qsl-attachments source/test/dependency/lockfile/workflow change.
- `NA0608A_NO_QWORK_QSTART_QRESUME_SUDO_REMOTE_OK` — none executed.
- `NA0608A_LEGACY_RESPONSES_DIR_UNMODIFIED_OK`.
- `NA0608A_NO_PRIVATE_MATERIAL_PUBLISHED_OK`.
- `NA0608A_NO_OVERCLAIM_OK` — no public/production/security-complete/bypass-proof/vulnerability-free/bug-free claim.
- `NA0608A_ONE_READY_INVARIANT_OK` — READY_COUNT 1 throughout (NA-0608 remains READY; NA-0608A IN_PROGRESS then DONE at closeout).
