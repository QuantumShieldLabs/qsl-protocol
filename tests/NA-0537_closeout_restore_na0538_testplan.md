Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0537 Closeout / NA-0538 Restoration Test Plan

Goals: G1, G2, G3, G4, G5

## Scope

This closeout verifies that NA-0537 implementation evidence is merged and accepted, records D447 disk-stop inheritance, marks NA-0537 DONE, and restores exactly one READY successor: `NA-0538 -- QSL Website / Repository Public Evidence Sync Scope Authorization Plan`.

This test plan is closeout-only. It does not authorize NA-0538 implementation, website mutation, README mutation, public-doc mutation, remote action, SSH execution, qsc E2EE, qsc send/receive, qsc protocol commands, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, dependency/lockfile mutation, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation.

## Required Proofs

1. qwork proof files exist at `/srv/qbuild/work/NA-0537/.qwork/startup.qsl-protocol.kv` and `/srv/qbuild/work/NA-0537/.qwork/startup.qsl-protocol.json`.
2. qwork proof is fresh at or after 2026-06-25T14:07:41Z, records `startup_result=OK`, lane `NA-0537`, repo `qsl-protocol`, clean worktree/index/untracked state, HEAD/origin-main `cda8b3090337`, READY_COUNT 1, and queue top READY `NA-0537`.
3. D447 disk-stop inheritance is recorded: D447 stopped before fetch and mutation at `/` usage 95%, and the operator later cleaned old per-lane target directories and reran qwork.
4. Disk gate proves `/` usage is below 95% before fetch and remains below 95% after closeout.
5. PR #1347 is merged at `cda8b3090337`.
6. D-1064 exists once, D-1065 is absent before patch, and duplicate decision count is zero.
7. Pre-closeout public-safety and advisories are completed success for `cda8b3090337`.
8. Current main has no red branch-protection required checks.
9. Root and nested cargo audits, cargo fmt, and qsc-adversarial shell syntax pass before and after patch.
10. Exact closeout scope guard allows only:
    - `NEXT_ACTIONS.md`
    - `DECISIONS.md`
    - `TRACEABILITY.md`
    - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
    - `tests/NA-0537_closeout_restore_na0538_testplan.md`

## Acceptance Markers

- `NA0537_CLOSEOUT_QWORK_PROOF_FRESH_OK`
- `NA0537_CLOSEOUT_D447_DISK_STOP_CONSUMED_OK`
- `NA0537_CLOSEOUT_DISK_GATE_OK`
- `NA0537_CLOSEOUT_PR1347_MERGED_OK`
- `NA0537_CLOSEOUT_D1064_ACCEPTED_OK`
- `NA0537_CLOSEOUT_D1065_ADDED_OK`
- `NA0537_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0537_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0537_CLOSEOUT_REQUIRED_CHECKS_GREEN_OK`
- `NA0537_CLOSEOUT_NA0537_DONE_OK`
- `NA0537_CLOSEOUT_NA0538_READY_OK`
- `NA0537_CLOSEOUT_NO_NA0538_IMPLEMENTATION_OK`
- `NA0537_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0537_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0537_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0537_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0537_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0537_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Stop Conditions

Stop if qwork proof is missing, stale, not OK, or mismatched from live pre-fetch HEAD/origin-main; if qwork/qstart/qresume is executed by Codex; if `/` usage is at or above 95%; if more than one READY item exists; if D-1064 is absent or duplicated; if D-1065 already exists before patch; if PR #1347 is not merged at `cda8b3090337`; if public-safety or advisories are red; if any branch-protection required check is red; if the closeout touches any path outside the five allowed paths; if private material is exposed; if qsl-backup is executed; if remote action, SSH, qsc send/receive, qsc E2EE, qsl-server, or qsl-attachments are used; or if no-claim boundaries are violated by forbidden readiness, completeness, proof, vulnerability-free, bug-free, or perfect-crypto wording.

## Post-Fix Hardening Review

- Correctness under stress: queue and decision parsers must prove exactly one READY item and no duplicate decision IDs before and after patch.
- Minimality: the scope guard must prove the closeout changed only the five allowed governance/testplan paths.
- Maintainability: D-1065, traceability, and the rolling journal must point to D446/D447 evidence and the restored NA-0538 successor without duplicating implementation evidence.
- Coverage quality: checks must fail on stale qwork proof, duplicate READY items, missing D-1064, pre-existing D-1065, out-of-scope paths, red required checks, or forbidden claim wording.
- Cross-lane stability: closeout validation is docs/governance-only and must preserve macOS/Linux runtime paths by avoiding qsc source/test/fuzz/Cargo, dependency/lockfile, workflow/script/helper, and runtime mutations.
