Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0547 Closeout and NA-0548 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record the closeout validation markers for accepting NA-0547 reproduction and
log-capture evidence, marking NA-0547 DONE, and restoring the exact D-1084
selected NA-0548 follow-up evidence authorization successor as the sole READY
item.

## Required Markers

- NA0547_CLOSEOUT_REPRO_LOG_PR_MERGED_OK
- NA0547_CLOSEOUT_D1084_ACCEPTED_OK
- NA0547_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0547_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0547_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0547_CLOSEOUT_D1085_RESTORED_NA0548_OK
- NA0547_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0547_CLOSEOUT_NO_NA0548_IMPLEMENTATION_OK
- NA0547_CLOSEOUT_NO_LOCAL_REPRODUCTION_OK
- NA0547_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0547_CLOSEOUT_NO_RUNTIME_MUTATION_OK
- NA0547_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0547_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0547_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_MUTATION_OK
- NA0547_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0547_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0547_CLOSEOUT_NO_RAW_LOGS_COMMITTED_OK
- NA0547_CLOSEOUT_NO_PRIVATE_MATERIAL_PUBLISHED_OK
- NA0547_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0547_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0547_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation

Closeout validation must prove:

- PR #1367 merged at `ce781ad33c27`.
- D-1084 exists once and D-1085 exists once.
- D-1086 is absent.
- NA-0547 is DONE.
- NA-0548 is the exactly one READY item.
- public-safety completed success.
- advisories completed success.
- no failed required checks are present.
- the selected successor block exactly matches the D-1084 Option C successor.
- no NA-0548 implementation occurred.
- no local reproduction, workflow mutation, runtime mutation, qsc source
  mutation, dependency/lockfile mutation, qsl-server/qsl-attachments mutation,
  public-site mutation, or Cloudflare mutation occurred.
- no raw logs were committed and no private material was published.
- no public-readiness, production-readiness, vulnerability-free, bug-free,
  perfect-build, or perfect-crypto claim was introduced.
- the closeout patch is limited to:
  `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan.
