Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-19
Replaces: n/a
Superseded-By: n/a

# NA-0500 Closeout and NA-0501 Restoration Testplan

## Purpose

Validate that NA-0500 is closed only after PR #1271 merged and post-merge
public-safety completed success for `2a8bf9729fc9`, and that the selected
NA-0501 qsc key lifecycle / zeroization expansion scope authorization
successor is restored as the sole READY item without implementing NA-0501.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0500_closeout_restore_na0501_testplan.md`

## Preconditions

- qwork proof files exist, report `startup_result=OK`, and match live
  HEAD/origin before fetch.
- `/` usage is below the 95% stop threshold.
- PR #1271 is merged at `2a8bf9729fc9`.
- D380 response exists.
- D381 response exists.
- D382 response exists.
- D-0989 exists exactly once.
- D-0990 is absent before closeout mutation.
- READY count before closeout is exactly one.
- READY before closeout is NA-0500.
- Post-merge public-safety on `2a8bf9729fc9` is success.
- qsc-adversarial-smoke on `2a8bf9729fc9` is success.
- qsc-linux-full-suite on `2a8bf9729fc9` is success or accepted by repository
  policy.
- macos-qsc-full-serial on `2a8bf9729fc9` is success or accepted by repository
  policy.

## Required inheritance proof

- PR #1271 merged at `2a8bf9729fc9`.
- Implementation commit is `f09a029c4886`.
- D380, D381, and D382 responses exist and are consumed.
- D-0989 records the NA-0500 qsc secret-material diagnostic no-output boundary
  test implementation.
- New qsc integration test exists at
  `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`.
- Real qsc diagnostic/error surfaces exercised are invalid policy profile,
  util sanitize usage error, and handshake `identity_unknown` reject/error.
- No-secret output scanner proof exists.
- Synthetic marker fail proof exists.
- PR #1271 changed exactly the implementation test, NA-0500 evidence doc,
  NA-0500 testplan, `DECISIONS.md`, `TRACEABILITY.md`, and
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- No qsc source, qsc fuzz target/Cargo, workflow/script/helper, dependency,
  lockfile, corpus/vector/input, validator, qsc-adversarial, refimpl, formal,
  service, public, or backup mutation occurred.
- D381 and D382 stopped only because post-merge required checks remained in
  progress through bounded polls; qsc-adversarial-smoke had completed success
  and zero failures were observed.
- NA-0500 remained READY until closeout.

## Required closeout changes

- Mark NA-0500 DONE.
- Add D-0990: `NA-0500 closeout and NA-0501 restoration`.
- Restore `NA-0501 -- QSL qsc Key Lifecycle / Zeroization Expansion Scope
  Authorization Plan` as the sole READY successor.
- Update TRACEABILITY for PR #1271 implementation/evidence, D380/D381/D382
  responses, D-0989, PR #1271 post-merge public-safety proof, D-0990, the
  closeout PR, and NA-0501 successor.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden closeout changes

- Do not implement NA-0501.
- Do not mutate qsc source, tests, fuzz target code, fuzz Cargo metadata, or
  lockfiles.
- Do not mutate workflows.
- Do not mutate scripts or helpers.
- Do not mutate validator scripts.
- Do not mutate qsc-adversarial scripts.
- Do not mutate corpus, vector, input, internal manifest, dependency, lockfile,
  formal, refimpl, service, public, backup, qsl-backup, qwork, qstart, qresume,
  or qshell paths.
- Do not move, archive, or delete files.
- Do not make a public-readiness claim.
- Do not make a production-readiness claim.
- Do not make a public-internet-readiness claim.
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a KEM-complete claim.
- Do not make a signature-complete claim.
- Do not make an identity-complete claim.
- Do not make a provider-RNG-complete claim.
- Do not make a secret-material-complete claim.
- Do not make a zeroization-complete claim.
- Do not make a memory-erasure-complete claim.
- Do not make a side-channel-free claim.
- Do not make a vulnerability-free claim.
- Do not make a bug-free claim.
- Do not make a perfect-crypto claim.

## Validation commands

Run before closeout mutation:

```bash
git diff --check
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/closeout/pr_body.md"
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

## Acceptance

NA-0500 is accepted as DONE only when PR #1271 merge proof,
D380/D381/D382 inheritance, D-0989, PR #1271 post-merge public-safety success,
qsc-adversarial-smoke success, qsc-linux-full-suite success or policy
acceptance, macos-qsc-full-serial success or policy acceptance, validator
proof, local validation, exact five-path closeout scope, and one-READY queue
proof all pass. NA-0501 is accepted as READY only as the selected key
lifecycle / zeroization expansion scope authorization successor; no NA-0501
execution is performed by this closeout.
