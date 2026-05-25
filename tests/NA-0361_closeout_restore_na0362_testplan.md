Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-05-25

# NA-0361 Closeout and NA-0362 Restoration Testplan

## Objective

Close NA-0361 after the no-secret key custody / key recovery fixture and
harness merged, and restore exactly one READY successor:
`NA-0362 -- Metadata Runtime Off-Host Encrypted Backup Target / Tool
Implementation Authorization Plan`.

## Protected Invariants

- NA-0361 is DONE.
- NA-0362 is the only READY item.
- D-0704 exists once.
- D-0705 exists once.
- D-0706 is absent.
- No NA-0362 implementation is performed by closeout.
- No real key generation, key upload, passphrase collection, private-key
  inspection, recovery-envelope content creation, or secret material handling
  occurs.
- No off-host setup occurs.
- No backup, restore, deploy, or rollback operation occurs.
- No runtime, service, dependency, workflow, website, public-doc, backup
  script/timer/fstab, local backup config, restore target, off-host target,
  key material, passphrase path, recovery envelope, branch-protection,
  public-safety configuration, or secret-handling path is changed.
- qsl-server PR #56 remains bounded end-to-end harness evidence only.
- qsl-attachments PR #37 remains service-local prerequisite evidence only.
- qshield embedded relay/demo evidence remains reference/oracle evidence only.
- No production-readiness, public-internet-readiness, external-review
  completion, anonymity, metadata-free behavior, untraceable behavior,
  hidden-size, hidden-timing, hidden-traffic-shape, complete disaster recovery,
  complete off-host backup, real restore completion, real key custody
  implementation, or real key recovery implementation claim is introduced.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0361_closeout_restore_na0362_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `scripts/**`
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`
- runtime, protocol, crypto, demo, service, deployment, restore, rollback,
  backup, branch-protection, public-safety, secret-handling, key-generation,
  key-upload, private-key-inspection, passphrase-collection,
  recovery-envelope, or off-host backup setup paths.

## Queue Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0362 Metadata Runtime Off-Host Encrypted Backup Target / Tool Implementation Authorization Plan`
- NA-0361 DONE

## Decision Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- latest decision D-0705
- duplicate count zero
- D-0704 exists once
- D-0705 exists once
- D-0706 absent

## Scope Requirements

Run scope guard with only the allowed closeout paths. The changed path set must
contain no runtime, service, workflow, dependency, website, public-doc, backup,
restore, key, recovery-envelope, or off-host paths.

## Link/Leak Requirements

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- `TOTAL_MISSING 0`
- `SECRET_FINDING_COUNT 0`

## Claim-Boundary Requirements

The closeout must state that NA-0362 remains future authorization work and must
not claim:

- production readiness
- public-internet readiness
- completed external review
- anonymous operation
- metadata-free behavior
- untraceable behavior
- hidden attachment size
- hidden timing metadata
- hidden traffic shape
- complete disaster recovery
- complete off-host backup
- real restore completion
- real key custody implementation
- real key recovery implementation

## Required Local Checks

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0361_closeout_restore_na0362_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0361_closeout_restore_na0362_testplan.md
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

## CI Expectations

- PR goal-lint passes with `Goals: G1, G2, G3, G4, G5`.
- Required checks complete normally.
- `public-safety` remains required and green.
- Merge uses a normal merge commit with `--match-head-commit`.
- No admin bypass, squash, rebase, direct push, delete-branch flag, branch
  deletion command, branch-protection mutation, or public-safety mutation is
  used.

## Successor Handoff

NA-0362 may only authorize, block, or prerequisite future off-host encrypted
backup target/tool implementation. Real off-host setup, real key generation,
key upload, passphrase collection, private-key inspection, recovery-envelope
content creation, secret handling, backup, restore, deploy, rollback, backup
script/timer/fstab mutation, source-list changes, monitoring setup, and
public-claim changes remain blocked until exact future authorization and
evidence requirements are explicit.
