Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0390 QSL Local Ops Routine Audit Cadence Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the standalone routine audit cadence helper, fixtures, temp-output
simulation, governance evidence, and public-claim boundaries without running a
full audit and without creating automation or durable audit reports.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0390` until optional closeout.
- D-0762 exists once after implementation.
- D-0763 remains absent until optional closeout.
- No scheduler, cron, workflow, timer, or background automation is created.
- No durable audit report output is created.
- No runtime, service, protocol, crypto, dependency, workflow, public-safety,
  qsl-server, qsl-attachments, qshield runtime, qsc-desktop, website, README,
  START_HERE, docs/public, backup script, timer, fstab, source-list, system
  service, or local tool path changes.
- No response, request, directive, journal, or ops-history archive mutation.
- No secret handling, off-host setup, target setup, restore, deploy, rollback,
  or public/readiness/privacy claim expansion.

## Allowed scope

- `scripts/ci/qsl_routine_audit_cadence.py`
- `inputs/local_ops/routine_audit_cadence_fixtures/`
- `docs/governance/evidence/NA-0390_qsl_local_ops_routine_audit_cadence_harness.md`
- `tests/NA-0390_qsl_local_ops_routine_audit_cadence_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`

## Forbidden scope

Forbidden paths include `.github/**`, workflows, Cargo/dependency files,
runtime/service/protocol/crypto/auth/state-machine files, qshield runtime,
qsl-server, qsl-attachments, qsc-desktop, website, docs/public, README,
START_HERE, backup scripts/timers/fstab/source lists/system services,
`scripts/ci/qsl_response_history_catalog.py`,
`scripts/ci/qsl_codex_response_writer.py`,
`scripts/ci/qsl_evidence_helper.py`,
`scripts/ci/qsl_bounded_check_poll.py`,
`scripts/ci/qsl_directive_manifest_validate.py`,
`scripts/ci/public_safety_gate.py`, `/srv/qbuild/tools/**`, durable audit
reports, response archive mutation, request file mutation, directive file
mutation, journal file mutation, ops-history mutation, and
`/home/victor/work/qsl/codex/**` except the final D209 response file required by
the directive.

## Helper CLI requirements

Required commands:

- `python3 scripts/ci/qsl_routine_audit_cadence.py --help`
- `python3 scripts/ci/qsl_routine_audit_cadence.py fixture --fixture-dir inputs/local_ops/routine_audit_cadence_fixtures --tmp-dir /srv/qbuild/tmp/NA0390_routine_audit_cadence_<timestamp>`
- `python3 scripts/ci/qsl_routine_audit_cadence.py validate --policy inputs/local_ops/routine_audit_cadence_fixtures/valid_policy.json --tmp-dir /srv/qbuild/tmp/NA0390_routine_audit_cadence_<timestamp>/validate --json`
- `python3 scripts/ci/qsl_routine_audit_cadence.py simulate --policy PATH --events PATH --tmp-dir /srv/qbuild/tmp/NA0390_routine_audit_cadence_<timestamp>/simulate/<scenario> --json`

The helper must use Python standard library only and must not import or invoke
network, GitHub, subprocess, scheduler, workflow, deletion, or overwrite
behavior.

## Fixture requirements

Fixtures must cover the full cadence policy, minimal policy, audit profiles,
trigger snippets, severity taxonomy, queue insertion policy, report-output
boundary, JSON summary, negative claim-boundary cases, malformed JSON, unknown
keys, and temp-output simulations.

## Positive validation requirements

The fixture matrix must pass valid fixtures for:

- full and minimal cadence policies;
- overall project audit profile;
- code/crypto audit profile;
- local-ops/history/backup audit profile;
- public-claim/external-review audit profile;
- targeted incident/regression audit profile;
- external standards / threat / technology watch future-gated profile;
- PR-count, NA-count, risk/event, and public-paper precondition triggers;
- severity taxonomy;
- queue insertion policy;
- temp-output report boundary;
- JSON summary fixture.

## Negative/fail-closed requirements

The fixture matrix must reject:

- missing audit profiles;
- missing code/crypto profile;
- missing public-claim boundary;
- unknown severity;
- CRITICAL finding without stop/escalation policy;
- auto-READY promotion;
- multiple READY candidates;
- durable report output;
- scheduler/cron/workflow request;
- background automation request;
- secret sentinel in report text;
- prohibited bug-free claim fixture;
- prohibited perfect-crypto claim fixture;
- prohibited production-ready claim fixture;
- prohibited public-internet-ready claim fixture;
- prohibited metadata-free/anonymity/untraceable claim fixture;
- prohibited external-review-complete claim fixture;
- public technical paper allowed without preconditions;
- external standards watch execution;
- malformed JSON;
- unknown top-level key.

## Temp-output simulation requirements

Simulations must cover:

- quick overall audit due by PR count;
- code/crypto audit due before public technical paper;
- public-claim audit due before website/public-doc changes;
- external standards / threat / technology watch recommended only as
  future-gated;
- no audit due.

All recommendations must be proposed candidates only, with no READY mutation,
no audit execution, no durable report, no scheduler, no workflow, and no web.

## No scheduler/no background requirements

Helper source and fixtures must not create scheduler, cron, timer, workflow, or
background automation behavior. Negative fixtures must reject such requests.

## No durable report requirements

Helper output must stay under `/srv/qbuild/tmp/NA0390_routine_audit_cadence_*`.
Durable report output requests must fail closed.

## No auto-READY requirements

Queue insertion policy must preserve one-READY discipline and allow
`BACKLOG_CANDIDATE` output only. READY mutations must be rejected.

## No-network/no-mutation requirements

The helper must not perform network, GitHub, subprocess, shell, branch mutation,
deletion, overwrite, or scanned-root mutation. It validates policy and writes
new temp proof files only.

## No-secret requirements

Secret sentinel text must be rejected. Generated summaries must avoid secret
material and full response body copies.

## No bug-free/perfect-crypto claim requirements

Fixture policy must reject prohibited bug-free and perfect-crypto claim
fixtures. Passing the harness is not absence-of-bugs proof and not
perfect-crypto proof.

## External standards watch future-gated requirements

The external standards / threat / technology watch profile must be represented
only as a future-gated profile. NA-0390 must not browse, fetch external sources,
or perform that watch.

## Backup-impact requirements

Tracked durable changes are qsl-protocol files. Proof logs and simulation output
remain under `/srv/qbuild/tmp`. No backup-plan update is required unless a
future directive authorizes durable audit report storage.

Same-host continuity must not be described as disaster recovery.

## Public-claim boundary requirements

The helper and evidence must not claim:

- no production readiness claim;
- no public-internet readiness claim;
- no external review completion claim;
- no metadata-free behavior claim;
- no anonymity claim;
- no untraceability claim;
- no absence-of-bugs claim;
- no perfect crypto claim.

## Successor selection requirements

If implementation succeeds and evidence shows durable report storage is not a
blocker for the next lane, selected successor is:

`NA-0391 -- QSL External Standards / Threat / Technology Watch Authorization Plan`

If durable audit report storage or backup coverage becomes a blocker, selected
successor is the blocker-resolution NA-0391 alternative. Do not implement
NA-0391 in NA-0390.

## Required local checks

- helper `--help`;
- fixture matrix;
- validate mode;
- simulation mode;
- `python3 -m py_compile scripts/ci/qsl_routine_audit_cadence.py`;
- JSON parse valid fixtures, excluding the malformed negative fixture;
- existing local-ops helper help and representative fixtures;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- `cargo fmt --check`;
- qsc `send_commit`;
- formal model checks;
- queue/decisions;
- scope guard;
- link-check;
- leak/overclaim scan;
- goal-lint and classifier proof for changed path set.

## CI expectations

The PR must merge only after required qsl-protocol checks complete green or are
accepted under existing neutral-check policy. public-safety remains required
before and after merge.

## Successor handoff

After merge and green post-merge public-safety, optional closeout may mark
NA-0390 DONE and restore NA-0391 as the sole READY successor. The closeout must
not implement NA-0391.
