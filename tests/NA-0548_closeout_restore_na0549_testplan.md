Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0548 Closeout Restore NA-0549 Testplan

## Scope

This testplan covers the NA-0548 closeout that accepts D-1086, marks NA-0548
DONE, and restores NA-0549 as the exactly one READY successor. The closeout is
governance-only and does not implement NA-0549.

## Required Markers

- NA0548_CLOSEOUT_AUTHORIZATION_PR_MERGED_OK
- NA0548_CLOSEOUT_D1086_ACCEPTED_OK
- NA0548_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0548_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0548_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0548_CLOSEOUT_D1087_RESTORED_NA0549_OK
- NA0548_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0548_CLOSEOUT_NO_NA0549_IMPLEMENTATION_OK
- NA0548_CLOSEOUT_NO_RERUN_EXECUTED_OK
- NA0548_CLOSEOUT_NO_WORKFLOW_DISPATCH_OK
- NA0548_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0548_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0548_CLOSEOUT_NO_RUNTIME_MUTATION_OK
- NA0548_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0548_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0548_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK
- NA0548_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0548_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0548_CLOSEOUT_NO_RAW_LOGS_COMMITTED_OK
- NA0548_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0548_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0548_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0548_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Evidence

- Scope guard: changed files are limited to `NEXT_ACTIONS.md`,
  `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
- Queue proof: READY_COUNT 1, READY NA-0549, NA-0548 DONE, D-1086 once,
  D-1087 once, D-1088 absent, and duplicate decision count zero.
- Required checks: public-safety success, advisories success, and no failed
  required checks after bounded polling.
- Boundary proof: no NA-0549 implementation, rerun, workflow dispatch, local
  reproduction, workflow mutation, runtime mutation, qsc source mutation,
  dependency or lockfile mutation, qsl-server/qsl-attachments mutation,
  public-site mutation, Cloudflare mutation, raw-log commit, or private-material
  publication.
- Claim boundary: no public-readiness claim, production-readiness claim,
  public-internet-readiness claim, external-review-complete claim,
  reproducibility-complete claim, backup/restore-complete claim,
  vulnerability-free claim, bug-free claim, perfect-build claim, or
  perfect-crypto claim.
