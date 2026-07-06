# NA-0608A Closeout — Restore NA-0608 Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G4 (primary), supports G1–G5

## Scope

This closeout records qsl-protocol implementation PR #1490 merge evidence and
the two satellite pointer PR merges, marks NA-0608A DONE, adds D-1208, and keeps
NA-0608 the sole READY successor. It does not implement NA-0608 and authorizes
no source, workflow, dependency, lockfile, Tailnet, public-site, deployment, or
qscwork runtime change.

## Closeout markers

- `NA0608A_CLOSEOUT_IMPLEMENTATION_PR1490_MERGED_OK` — PR #1490 merged at `0fed64e82e19`.
- `NA0608A_CLOSEOUT_D1207_ACCEPTED_OK` — D-1207 exists once and is Accepted.
- `NA0608A_CLOSEOUT_PR1490_POSTMERGE_GREEN_OK` — public-safety, advisories, suite2-vectors, qsc-adversarial-smoke, goal-lint, CodeQL success; no failed required checks.
- `NA0608A_CLOSEOUT_SATELLITE_QSL_SERVER_MERGED_OK` — qsl-server PR #60 merged at `19b9b02dbe1f` (CLAUDE.md only).
- `NA0608A_CLOSEOUT_SATELLITE_QSL_ATTACHMENTS_MERGED_OK` — qsl-attachments PR #39 merged at `a3ebad2fd19a` (CLAUDE.md only).
- `NA0608A_CLOSEOUT_SATELLITE_FLAKY_RECOVERY_OK` — qsl-server flaky na0347 check recovered via operator-approved no-op re-trigger; no workflow rerun/dispatch by executor.
- `NA0608A_CLOSEOUT_FRESH_CHECKOUT_PROPAGATION_OK` — .claude/settings.json present in a fresh clone of main; sudo/qwork blocked, git status allowed.
- `NA0608A_CLOSEOUT_D1208_RECORDED_OK` — D-1208 recorded once; D-1209 absent.
- `NA0608A_CLOSEOUT_NA0608A_DONE_OK` — NA-0608A marked DONE.
- `NA0608A_CLOSEOUT_NA0608_READY_OK` — NA-0608 restored/retained as the sole READY successor.
- `NA0608A_CLOSEOUT_ONE_READY_INVARIANT_OK` — READY_COUNT 1.
- `NA0608A_CLOSEOUT_COUNTER_SHIFT_OK` — NA-0608 begins at D-1209; its directive is D541.
- `NA0608A_CLOSEOUT_SCOPE_GUARD_OK` — closeout mutates only the allowed closeout paths.
- `NA0608A_CLOSEOUT_NO_IMPLEMENTATION_OK` — no NA-0608 implementation; no source/dependency/lockfile/workflow/protocol change.
- `NA0608A_CLOSEOUT_NO_PRIVATE_MATERIAL_OK`, `NA0608A_CLOSEOUT_NO_OVERCLAIM_OK`.
- `NA0608A_CLOSEOUT_LEGACY_RESPONSES_UNMODIFIED_OK`.
