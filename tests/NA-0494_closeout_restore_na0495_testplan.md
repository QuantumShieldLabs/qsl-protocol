Status: Supporting
Owner: QSL Director
Last-Updated: 2026-06-18

# NA-0494 Closeout and NA-0495 Restoration Testplan

## Purpose

Validate that NA-0494 is closed only after its evidence PR merged and
post-merge public-safety completed success, and that the selected NA-0495
successor is restored as the sole READY item without implementing NA-0495.

## Allowed closeout paths

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0494_closeout_restore_na0495_testplan.md`

## Preconditions

- NA-0494 evidence PR #1259 merged at `8221f6cceb2a`.
- D-0977 exists exactly once on main.
- Post-merge public-safety on `8221f6cceb2a` is green.
- READY count before closeout is exactly one.
- READY before closeout is NA-0494.
- D-0978 is absent before closeout mutation.

## Required closeout changes

- Mark NA-0494 DONE.
- Add D-0978: `NA-0494 closeout and NA-0495 restoration`.
- Restore `NA-0495 -- QSL Binding Fuzz Corpus Validator qsc-Adversarial
  Integration Implementation Harness` as the sole READY successor.
- Update TRACEABILITY for closeout.
- Update the rolling operations journal.
- Add this closeout testplan.

## Forbidden closeout changes

- Do not implement NA-0495.
- Do not mutate `scripts/ci/qsc_adversarial.sh`.
- Do not mutate workflows.
- Do not mutate `qsl_evidence_helper.py`.
- Do not mutate validator scripts.
- Do not mutate corpus, vector, input, qsc source, qsc fuzz target, Cargo,
  lockfile, dependency, formal, refimpl, service, public, backup, qsl-backup,
  qwork, qstart, qresume, or qshell paths.
- Do not move, archive, or delete files.
- Do not make a public-readiness claim.
- Do not make a production-readiness claim.
- Do not make a public-internet-readiness claim.
- Do not make an external-review-complete claim.
- Do not make a crypto-complete claim.
- Do not make a fuzz-complete claim.
- Do not make a corpus-complete claim.
- Do not make a vector-complete claim.
- Do not make a replay-proof claim.
- Do not make a downgrade-proof claim.
- Do not make a side-channel-free claim.
- Do not make a vulnerability-free claim.
- Do not make a bug-free claim.
- Do not make a perfect-crypto claim.

## Validation commands

Run before PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/closeout/pr_body.md"
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Run exact five-path closeout scope guard and require only the closeout allowed
paths.

Run queue/decision proof:

- READY_COUNT is 1.
- READY is NA-0495.
- NA-0494 is DONE.
- D-0977 exists once.
- D-0978 exists once.
- D-0979 is absent.
- duplicate decision count is 0.

Run goal-lint after the closeout PR is opened.

## Acceptance criteria

- Closeout PR checks pass.
- Closeout PR merges with a merge commit.
- Post-merge `origin/main` contains D-0978 exactly once.
- Post-merge READY count is exactly one.
- Post-merge READY is NA-0495.
- Post-merge public-safety is green.
