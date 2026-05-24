Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-24
Replaces: n/a
Superseded-By: n/a

# NA-0349 Closeout and NA-0350 Restoration Test Plan

## Objective

Verify that NA-0349 closes only after the bounded end-to-end qsl-server /
qsl-attachments integration implementation harness and qsl-protocol governance
companion have merged, and that the exact selected NA-0350 successor is
restored as the sole READY item without implementing NA-0350.

## Protected invariants

- Exactly one READY item exists after closeout.
- NA-0349 is DONE.
- NA-0350 is READY.
- D-0680 and D-0681 each exist exactly once.
- D-0682 is absent.
- qsl-server PR #56 remains bounded integration harness evidence.
- qsl-attachments PR #37 remains service-local prerequisite evidence.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No production/public-internet/external-review/anonymity claim is introduced.
- No metadata-free or untraceable claim is introduced.
- No claim states that attachment size, timing metadata, or traffic shape is
  hidden.
- No NA-0350 implementation is included by closeout.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0349_closeout_restore_na0350_testplan.md`

## Forbidden scope

- qsl-server source or governance files.
- qsl-attachments source or governance files.
- qshield runtime, qsc, qsp, qsl, protocol, crypto, or key schedule paths.
- `Cargo.toml`, `Cargo.lock`, dependency manifests, and workflow files.
- `README.md`, `START_HERE.md`, `docs/public/**`, website paths, and external
  public claim surfaces.
- Branch-protection, public-safety, deployment, secret, backup, or monitoring
  configuration.
- Any NA-0350 implementation.

## Queue requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0350 Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`
- NA-0349 is marked DONE in `NEXT_ACTIONS.md`.

## Decision requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- D-0680 exists once.
- D-0681 exists once.
- D-0682 is absent.
- No duplicate decision IDs are reported.

## Scope guard requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0349_closeout_restore_na0350_testplan.md
```

Expected:

- All changed paths are allowed.
- `FORBIDDEN_COUNT 0`.

## Link and leak requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.

## Required validation

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected:

- Cargo audit passes.
- `rustls-webpki` remains at `v0.103.13` or newer safe version.
- qsc send-commit and formal/model checks pass.

## Public-safety and PR requirements

- PR body includes standalone `Goals: G1, G2, G3, G4, G5`.
- Goal-lint passes.
- Required PR checks pass normally.
- Merge uses normal merge with `--match-head-commit`.
- No squash, rebase, direct push, admin bypass, or delete-branch flag is used.
- Post-merge `public-safety` completes success.

## Successor handoff

The restored successor is exact:

`NA-0350 -- Metadata Runtime Production Backup / Deploy / Rollback Hardening Plan`

This closeout authorizes no NA-0350 implementation.
