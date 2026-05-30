Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0389 QSL Local Ops Routine Audit Cadence Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0389 authorizes a future routine audit cadence implementation
harness without implementing it, without creating durable audit reports, and
without weakening queue, backup, helper, runtime, public-claim, or service
boundaries.

## Protected invariants

- READY_COUNT remains `1`.
- READY remains `NA-0389` until optional closeout.
- NA-0388 remains DONE.
- D-0758 and D-0759 exist once.
- D-0760 exists once after this authorization PR.
- D-0761 remains absent until optional closeout.
- NA-0389 is authorization/planning only.
- No audit scheduler, cron job, workflow, timer, background automation, or
  durable audit report store is created.
- No runtime, service, protocol, crypto, dependency, qsl-server,
  qsl-attachments, qshield runtime, qsc-desktop, website, docs/public, README,
  START_HERE, backup script, timer, fstab, source-list, system-service, local
  tool, or local history archive path is changed.
- No secret handling, remote/off-host setup, restore, deploy, rollback, or
  public/readiness/privacy claim expansion occurs.

## Allowed scope

- `docs/governance/evidence/NA-0389_qsl_local_ops_routine_audit_cadence_authorization.md`
- `tests/NA-0389_qsl_local_ops_routine_audit_cadence_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

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
reports, durable catalog/index files, response archive mutation, request file
mutation, directive file mutation, journal file mutation, ops-history mutation,
and `/home/victor/work/qsl/codex/**` except the final D208 response file
required by the directive.

## NA-0388 inheritance requirements

Evidence must record:

- PR #1039 merge `c928998a298f`;
- PR #1040 merge `2fda1f78b93`;
- helper path `scripts/ci/qsl_response_history_catalog.py`;
- fixture directory `inputs/local_ops/response_history_catalog_fixtures/`;
- live catalog path and SHA-256 prefix `3ab3fbec0309`;
- fixture catalog SHA-256 prefix `9422809fde32`;
- fixture log SHA-256 prefix `53a51dc1c7c3`;
- live scan counts and no durable catalog/index output;
- no archive mutation, no helper mutation, and no backup-plan update.

## Prior audit intake requirements

If present, verify NA-0380 report checksums:

- overall project audit SHA-256
  `66dd26c0b35b97113f160e4dd67fdc9992bd3be91c72452359fbef74dcef0913`;
- code/crypto audit SHA-256
  `70c21179e7a57dd168dff77e2d5bb18ac2ad1c7c285b216da7875ca712d1c099`.

If reports are absent, record absence and continue. Absence is not a stop
condition.

## Audit class requirements

The evidence must define:

- overall project audit;
- code/crypto audit;
- local-ops/history/backup audit;
- public-claim/external-review readiness audit;
- targeted incident/regression audit.

Each class must state scope, purpose, triggers, expected report format, allowed
sources, forbidden actions, CI/local checks, output storage policy, and
severity/queue-impact rules.

## Trigger/cadence requirements

The evidence must define trigger policy for:

- fixed PR-count threshold;
- fixed NA-count threshold;
- runtime/protocol/crypto changes;
- qsl-server/qsl-attachments changes;
- dependency/advisory changes;
- backup/restore/deploy/rollback changes;
- public technical paper preparation;
- website/public-doc claim changes;
- external-review package preparation;
- production/public-internet claim proposals;
- public-safety/workflow changes;
- CRITICAL/HIGH findings;
- operator-demand trigger.

No actual scheduler, cron, workflow, timer, or background automation may be
created.

## Report storage/backup requirements

The evidence must compare:

- `/srv/qbuild/tmp` temp reports;
- qsl-protocol tracked summaries;
- `/home/victor/work/qsl/codex/ops/audits`;
- final response embedding only;
- no durable report/manual summary only.

For each option, record backup impact, secret risk, durability, searchability,
restore value, authority, CI suitability, and recommendation.

Expected result:

- future first implementation uses temp-output proof and governance evidence
  only;
- durable audit report storage remains separate unless exact future scope
  authorizes backup coverage;
- NA-0389 requires no backup-plan update.

## Finding taxonomy requirements

The evidence must define:

- CRITICAL;
- HIGH;
- MEDIUM;
- LOW;
- INFO;
- EVIDENCE_INCOMPLETE;
- CLAIM_BOUNDARY;
- BACKLOG_CANDIDATE.

Required fields:

- finding ID;
- title;
- severity;
- affected area;
- evidence;
- confidence;
- risk;
- recommended action;
- proposed NEXT_ACTIONS candidate;
- goals mapping;
- scope category;
- public-claim implication;
- backup impact;
- external-review implication;
- owner/dependency;
- blocked/unblocked status.

## Queue insertion requirements

The evidence must state:

- audit findings do not automatically create READY items;
- findings may propose BACKLOG or READY candidates;
- exactly one READY remains enforced;
- CRITICAL/HIGH active-lane findings require explicit future directive triage;
- CLAIM_BOUNDARY findings may propose public-claim audit lanes;
- new NA items require decisions and traceability when promoted.

## Code/crypto audit scope requirements

The evidence must include:

- crypto API misuse review;
- nonce lifecycle review;
- key lifecycle review;
- RNG usage review;
- deterministic/test-only boundary review;
- panic/unwrap/expect/error-handling review;
- unsafe code review;
- side-channel/timing/traffic-shape limitation review;
- formal model alignment review;
- property/fuzz opportunities;
- duplicate dependency-family review;
- cargo audit/advisory review;
- broad clippy/test gaps separated from merge-critical gates;
- qsc/qshield/qsp/protocol boundary review;
- qsl-server/qsl-attachments service-local proof boundary review;
- public-claim implications.

The evidence must state that audit findings do not prove absence of bugs, do not
prove perfect crypto, do not complete external review, and do not establish
production readiness.

## Overall project audit scope requirements

The evidence must include:

- NEXT_ACTIONS queue health;
- DECISIONS sequencing and uniqueness;
- TRACEABILITY coverage;
- evidence/testplan completeness;
- CI/public-safety health;
- cargo audit/dependency health;
- backup/restore/deploy/rollback boundaries;
- local-ops helper health;
- response/history catalog coverage;
- operator input blockers;
- qsl-server/qsl-attachments production boundary;
- qshield demo/non-production boundary;
- public-claim/external-review readiness;
- website/public-doc boundary;
- D132 preservation/cleanup status;
- public technical position paper timing;
- audit finding backlog and closure status.

## Implementation option requirements

The evidence must compare:

- standalone `scripts/ci/qsl_routine_audit_cadence.py`;
- docs-only policy first;
- extension to response history catalog helper;
- extension to directive manifest validator;
- extension to qsl_evidence_helper;
- local `/srv/qbuild/tools` helper;
- GitHub workflow/cron;
- manual-only status quo.

Expected recommendation:

- standalone qsl-protocol helper with fixtures and temp-output reports is
  preferred for future NA-0390 if authorized;
- no workflow/cron for the first implementation.

## First-lane authorization requirements

The evidence must choose exactly one:

- `ROUTINE_AUDIT_CADENCE_IMPLEMENTATION_AUTHORIZATION_READY_FOR_TEMP_OUTPUT_HARNESS`;
- `ROUTINE_AUDIT_CADENCE_BLOCKED_PENDING_REPORT_STORAGE_OR_BACKUP_COVERAGE`.

If implementation is authorized, the evidence must define exact future NA-0390
allowed paths and keep durable audit reports forbidden unless separately
authorized.

## Public technical paper boundary requirements

The evidence must state:

- public technical paper remains future-gated;
- NA-0389 does not draft the paper;
- paper work requires fresh claim-boundary audit, external-review readiness,
  code/crypto audit status, service status, and backup/restore status.

## Fail-closed requirements

The evidence must require:

- no audit scheduler;
- no background work;
- no cron/workflow;
- no durable report output in NA-0389;
- no code mutation while auditing;
- no secret copying;
- no public-claim expansion;
- no audit finding auto-READY insertion;
- one READY item preserved;
- deterministic report schema for future implementation;
- bounded local checks;
- no secret material in reports;
- evidence gaps clearly marked;
- no bug-free or perfect-crypto claims.

## Public-claim boundary requirements

The evidence must not claim:

- production readiness;
- public-internet readiness;
- external review completion;
- metadata-free behavior;
- anonymity;
- untraceability;
- absence of bugs;
- perfect crypto.

No website/public docs update is authorized.

## Successor selection requirements

Expected selected successor if ready:

`NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`

Expected selected successor if blocked:

`NA-0390 -- QSL Local Ops Routine Audit Report Storage / Backup Coverage Blocker Resolution`

NA-0390 must not be implemented by NA-0389.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_response_history_catalog.py --help
python3 scripts/ci/qsl_response_history_catalog.py fixture --fixture-dir inputs/local_ops/response_history_catalog_fixtures --tmp-dir /srv/qbuild/tmp/NA0389_response_history_catalog_<timestamp> --json
python3 scripts/ci/qsl_codex_response_writer.py --help
python3 scripts/ci/qsl_codex_response_writer.py fixture --fixture-dir inputs/local_ops/response_writer_fixtures --tmp-dir /srv/qbuild/tmp/NA0389_response_writer_<timestamp>
python3 scripts/ci/qsl_bounded_check_poll.py --help
python3 scripts/ci/qsl_bounded_check_poll.py fixture --fixture inputs/local_ops/qsl_bounded_check_poll_fixtures/pr_required_success.json --policy pr-required
python3 scripts/ci/qsl_directive_manifest_validate.py --help
python3 scripts/ci/qsl_directive_manifest_validate.py fixture --fixture-dir inputs/local_ops/directive_manifest_fixtures --allow-fixture-dir inputs/local_ops/scope_allow_file_fixtures
python3 -m py_compile scripts/ci/qsl_bounded_check_poll.py scripts/ci/qsl_directive_manifest_validate.py scripts/ci/qsl_codex_response_writer.py scripts/ci/qsl_response_history_catalog.py
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed-file <allowed> --forbidden-file <forbidden>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --body-file <body>
```

## CI expectations

Required checks must attach and complete green before merge. public-safety must
remain required and green before merge and after merge. No admin bypass, direct
push, squash, rebase, force-push, amend, or branch deletion is authorized.

## Successor handoff

After NA-0389 merges and optional closeout runs, restore exactly one READY item:

`NA-0390 -- QSL Local Ops Routine Audit Cadence Implementation Harness`

The closeout may not implement NA-0390.
