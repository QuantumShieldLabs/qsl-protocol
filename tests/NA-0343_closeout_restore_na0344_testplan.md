# NA-0343 Closeout / NA-0344 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Close NA-0343 after the implementation authorization plan merged, and restore
the exact selected successor:

`NA-0344 -- Metadata Runtime qsl-attachments Production Size-Class Implementation Harness`

This testplan verifies governance state only. It does not authorize or implement
NA-0344.

## Protected invariants

- Exactly one READY item exists after closeout: NA-0344.
- NA-0343 is DONE.
- D-0668 and D-0669 each exist once; D-0670 is absent.
- qsl-attachments remains unmodified by this closeout.
- qsl-server remains unmodified by this closeout.
- qshield runtime, qsc/qsp/protocol/crypto/key-schedule code, dependencies,
  workflows, public-safety configuration, website/public docs, README, and
  START_HERE remain unchanged.
- qshield embedded relay/demo evidence remains reference/oracle evidence only,
  not qsl-server or qsl-attachments production proof.
- The closeout introduces no claim that attachment size, timing metadata,
  traffic shape, or all metadata is hidden.
- The closeout introduces no anonymity, metadata-free, untraceable,
  production-readiness, public-internet-readiness, or external-review-complete
  claim.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0343_closeout_restore_na0344_testplan.md`

## Forbidden scope

- qsl-attachments implementation paths.
- qsl-server implementation paths.
- qshield runtime implementation paths.
- qsc/qsp/protocol/crypto/key-schedule implementation paths.
- Cargo manifests or lockfiles.
- workflows, public-safety configuration, branch-protection configuration,
  website/public docs, README, START_HERE, formal inputs, tools/refimpl, apps,
  qsc-desktop, or service/deployment paths.

## Closeout requirements

1. NEXT_ACTIONS marks NA-0343 DONE.
2. NEXT_ACTIONS promotes NA-0344 as the sole READY successor.
3. NEXT_ACTIONS records Packet M evidence: PR #948, validated head
   `8887b42dee30`, merge `f947f8951f1d`, and post-merge `public-safety`
   success.
4. DECISIONS adds D-0669 for closeout and NA-0344 restoration.
5. TRACEABILITY links D-0669, this testplan, and the selected successor.
6. The rolling journal records Packet M merge evidence, closeout branch state,
   recoveries, validation notes, and next-watch items.

## Required local checks

Run and record:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0343_closeout_restore_na0344_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0343_closeout_restore_na0344_testplan.md
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Also run PR-body preflight and goal-lint with a synthetic event before opening
the PR.

## CI expectations

- Required qsl-protocol checks complete normally on the closeout PR.
- `public-safety` remains required by branch protection and completes success.
- Merge uses a normal merge commit with `--match-head-commit`.
- No direct push, admin bypass, squash, rebase, or branch deletion command is
  allowed.

## Successor handoff

NA-0344 remains unimplemented by this closeout. The next directive must
separately authorize any qsl-attachments mutation and must preserve the
source/authority, qsl-server boundary, qshield demo boundary, backup/deploy,
rollback, secrets/env, and public-claim boundaries recorded by NA-0343.
