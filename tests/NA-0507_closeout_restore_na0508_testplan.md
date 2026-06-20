Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0507 Closeout and NA-0508 Restoration Testplan

## Objective

Verify that NA-0507 closes only after PR #1286 is merged and post-merge
public-safety completes success, then restore NA-0508 as the sole READY
successor without executing NA-0508 or performing any remote setup action.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0507 is DONE.
- NA-0508 is READY.
- D-1003 exists once.
- D-1004 exists once after closeout.
- D-1005 is absent.
- No duplicate decision IDs.
- No implementation mutation is performed.
- No remote action is performed.
- No SSH execution is performed.
- No scp/sftp/rsync execution is performed.
- No remote account creation is performed.
- No SSH key generation or installation is performed.
- No local SSH config mutation is performed.
- No system SSH config mutation is performed.
- No known_hosts mutation is performed.
- No authorized_keys mutation is performed.
- No remote host mutation is performed.
- No sudo/admin action is performed.
- No qwork/qstart/qresume mutation is performed.
- No qsl-backup execution or mutation is performed.
- No qsc source/test/fuzz/Cargo mutation is performed.
- No workflow/script/helper/dependency mutation is performed.
- No corpus/vector/input mutation is performed.
- No formal/refimpl/service/public/backup mutation is performed.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no external-review-complete claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.
- no vulnerability-free, bug-free, or perfect-crypto claim is made.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0507_closeout_restore_na0508_testplan.md`

## Forbidden scope

- executing NA-0508.
- creating remote users.
- generating or installing SSH keys.
- running SSH, scp, sftp, or rsync to remote.
- mutating local SSH config.
- mutating system SSH config.
- mutating known_hosts.
- mutating authorized_keys.
- mutating remote hosts.
- sudo/admin action.
- qwork/qstart/qresume mutation.
- qsl-backup execution or mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow/script/helper/dependency mutation.
- corpus/vector/input mutation.
- formal/refimpl/service/public/backup mutation.
- no public-readiness claim and no production-readiness claim.

## Required proof

Expected before closeout mutation:

- qwork proof files verified without rerunning qwork.
- PR #1286 merged with merge commit `198a01a031c0`.
- PR #1286 post-merge public-safety completed success.
- PR #1286 changed exactly the five NA-0507 authorization paths.
- D395 response recovered or in-tree D-1003 evidence is sufficient.
- D-1003 exists once on main before closeout.
- NA-0507 remains READY before closeout.
- D-1004 is absent before closeout.
- Closeout patch touches exactly the five allowed closeout paths.

## Queue proof

Expected after patch:

- READY_COUNT 1.
- READY NA-0508.
- NA-0507 DONE.
- D-1003 once.
- D-1004 once.
- D-1005 absent.
- duplicate decision count zero.

## Validation

Run and require pass:

```bash
git diff --check
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also run:

- exact five-path closeout scope guard.
- link-check.
- leak-scan.
- added-line overclaim scan.
- docs/governance classifier.
- PR body preflight.
- goal-lint.
- queue/decision proof.

## Acceptance criteria

- NA-0507 is marked DONE.
- NA-0508 is the only READY item.
- D-1004 records NA-0507 closeout and NA-0508 restoration.
- NA-0508 remains proof-review only.
- closeout does not execute NA-0508.
- closeout does not authorize Codex remote action.
- NA-0508 must not proceed until operator-provided approved redacted proof is available.
