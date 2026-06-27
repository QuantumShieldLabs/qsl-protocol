Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0546 Closeout / Restore NA-0547 Testplan

## Objective

Validate that the merged NA-0546 reproduction authorization is accepted,
D-1083 records closeout, NA-0546 is marked DONE, and NA-0547 is restored as the
sole READY successor without implementing NA-0547 or executing any reproduction.

## Required Markers

- `NA0546_CLOSEOUT_REPRO_AUTH_PR_MERGED_OK`
- `NA0546_CLOSEOUT_D1082_ACCEPTED_OK`
- `NA0546_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0546_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0546_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK`
- `NA0546_CLOSEOUT_D1083_RESTORED_NA0547_OK`
- `NA0546_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK`
- `NA0546_CLOSEOUT_NO_NA0547_IMPLEMENTATION_OK`
- `NA0546_CLOSEOUT_NO_RERUN_EXECUTED_OK`
- `NA0546_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK`
- `NA0546_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK`
- `NA0546_CLOSEOUT_NO_WORKFLOW_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_RUNTIME_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0546_CLOSEOUT_NO_QWORK_EXECUTION_OK`
- `NA0546_CLOSEOUT_NO_QSL_BACKUP_EXECUTION_OK`
- `NA0546_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK`
- `NA0546_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0546_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0546_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Expected Final Queue

- READY_COUNT 1
- READY `NA-0547`
- `NA-0546` DONE
- D-1082 once
- D-1083 once
- D-1084 absent
- duplicate decision count zero

## Scope Guard

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0546_closeout_restore_na0547_testplan.md`

Any workflow, runtime/source, qsc source/test/fuzz/Cargo, dependency/lockfile,
qsl-server, qsl-attachments, public-site, Cloudflare, backup, or operator/local
system mutation fails this testplan.

## Validation

Closeout validation must include diff check, exact five-path closeout scope
guard, queue/decision proof, marker proof, link-check, private-material scan,
overclaim scan, docs/governance-only classifier, PR body preflight, goal-lint
if available, cargo audits, cargo fmt, and qsc-adversarial shell syntax checks.
