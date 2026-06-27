Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-27

# NA-0545 Closeout / Restore NA-0546 Testplan

## Scope

This closeout accepts the merged NA-0545 forward-audit authorization work and
restores the D-1080-selected Option B successor:

`NA-0546 -- QSL Remote/Relay Non-Required CI Failure Bounded Reproduction Authorization Plan`

This closeout does not implement NA-0546 and does not run remote/relay
reproduction, reruns, workflow dispatch, workflow mutation, runtime mutation,
qsc mutation, qsl-server/qsl-attachments work, public-site mutation, Cloudflare
mutation, qwork/qstart/qresume, or qsl-backup.

## Required Markers

- NA0545_CLOSEOUT_FORWARD_AUDIT_PR_MERGED_OK
- NA0545_CLOSEOUT_D1080_ACCEPTED_OK
- NA0545_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0545_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0545_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0545_CLOSEOUT_D1081_RESTORED_NA0546_OK
- NA0545_CLOSEOUT_SELECTED_SUCCESSOR_EXACT_OK
- NA0545_CLOSEOUT_NO_NA0546_IMPLEMENTATION_OK
- NA0545_CLOSEOUT_NO_WORKFLOW_MUTATION_OK
- NA0545_CLOSEOUT_NO_RUNTIME_MUTATION_OK
- NA0545_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0545_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0545_CLOSEOUT_NO_REMOTE_ACTION_OK
- NA0545_CLOSEOUT_NO_QWORK_EXECUTION_OK
- NA0545_CLOSEOUT_NO_QSL_BACKUP_EXECUTION_OK
- NA0545_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK
- NA0545_CLOSEOUT_NO_PUBLIC_SITE_MUTATION_OK
- NA0545_CLOSEOUT_NO_CLOUDFLARE_MUTATION_OK
- NA0545_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0545_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0545_CLOSEOUT_ONE_READY_INVARIANT_OK

## Validation Requirements

The closeout validation must prove:

- NA-0545 implementation PR #1363 merged at `ddca6773c563`.
- D-1080 exists once and is accepted.
- D-1081 exists once after the closeout patch.
- D-1082 remains absent.
- NA-0545 is DONE.
- READY_COUNT is 1.
- READY is NA-0546.
- The selected NA-0546 successor block exactly matches D-1080 Option B.
- Duplicate decision count is zero.
- public-safety completed success on the post-merge implementation commit.
- advisories completed success on the post-merge implementation commit.
- no failed required checks were observed.
- The closeout diff is limited to:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0545_closeout_restore_na0546_testplan.md`

## Expected Result

The closeout passes only if NA-0545 is DONE, NA-0546 is the exactly one READY
item, D-1080 and D-1081 each exist once, D-1082 is absent, all required markers
are present, and no prohibited mutation/action/claim is introduced.
