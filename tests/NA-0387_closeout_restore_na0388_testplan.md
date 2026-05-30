Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-30

# NA-0387 Closeout Restore NA-0388 Test Plan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0387 after the response archive index/history catalog authorization PR
merged and restore `NA-0388 -- QSL Local Ops Response Archive Index and History
Catalog Implementation Harness` as the sole READY successor without
implementing NA-0388.

## Protected invariants

- READY_COUNT is `1`.
- NA-0387 is DONE and NA-0388 is READY.
- D-0756 and D-0757 each exist once; D-0758 is absent.
- NA-0388 is not implemented by this closeout.
- No response, request, directive, journal, ops-history, or durable local
  catalog output is created.
- Existing archives are not overwritten, deleted, truncated, moved, copied into
  a durable catalog, or rewritten.
- No runtime, workflow, dependency, qsl-server, qsl-attachments, qshield runtime,
  website, docs/public, README, START_HERE, backup script, timer, fstab, target,
  key, restore, deploy, rollback, or off-host setup mutation occurs.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0387_closeout_restore_na0388_testplan.md`

## Forbidden scope

Forbidden paths include README, START_HERE, docs/public, `.github/**`, Cargo
files, qsp, qsc, qsl, qsl-client, apps, tools, inputs, formal, scripts,
qsc-desktop, qsl-server, qsl-attachments, website, runtime/protocol/crypto/demo
or service implementation paths, branch-protection or public-safety
configuration, backup scripts/timers/fstab/local system paths, branch deletion,
`/home/victor/work/qsl/codex/**`, and `/srv/qbuild/tools/**`.

## Closeout evidence requirements

Evidence must record:

- qsl-protocol PR #1037 head `d74ea7ccaae`;
- qsl-protocol PR #1037 merge `f8165a6626fa`;
- post-merge public-safety success on `f8165a6626fa`;
- D-0756 as the authorization decision;
- D-0757 as the closeout decision;
- selected successor exactly:
  `NA-0388 -- QSL Local Ops Response Archive Index and History Catalog Implementation Harness`.

## Successor requirements

The NA-0388 READY block must state the temp-output-only objective:

- scan authorized history roots read-only;
- emit metadata-only catalog output under `/srv/qbuild/tmp`;
- prove no archive mutation;
- prove no secret content copy;
- prove no durable index output;
- prove no runtime, workflow, dependency, backup script, qsl-server, or
  qsl-attachments drift.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed-file <allowed> --forbidden-file <forbidden>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run classifier proof and PR-body/goal-lint preflight for the closeout PR.

## CI expectations

Required checks must attach and complete green before merge. public-safety must
remain required and green before merge and after merge. No admin bypass, direct
push, squash, rebase, force-push, amend, or branch deletion is authorized.

## Successor handoff

After merge, `NEXT_ACTIONS.md` must show READY_COUNT `1`, READY NA-0388,
NA-0387 DONE, D-0757 once, and D-0758 absent. Closeout must not implement
NA-0388.
