Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-18
Replaces: n/a
Superseded-By: n/a

# NA-0493 Closeout / Restore NA-0494 Test Plan

## Scope

This test plan covers only NA-0493 closeout and NA-0494 restoration. It does
not implement NA-0494 and does not authorize workflow, script, dependency,
lockfile, corpus, vector/input, runtime, crypto, source, formal, refimpl,
service, public, backup, qsl-backup, qwork/qstart/qresume/qshell, archive,
move, or delete mutation.

## Required Proofs

1. qwork proof files exist, report `startup_result=OK`, and match live
   HEAD/origin before fetch.
2. Initial queue proof shows exactly one READY item: NA-0493.
3. Initial decision proof shows D-0973, D-0974, and D-0975 once; D-0976 absent;
   duplicate decision count zero.
4. The exact D363 pointer file is recovered if present by recording stat and
   digest, copying it into the D364 proof root, removing only that file, and
   verifying absence.
5. PR #1257 is merged at `b5f140e5bd3a`.
6. D361, D362, and D363 inheritance is consumed:
   - seven checked-in binding corpus seed files exist;
   - every seed file is 8 bytes;
   - validator proof passed in D361/D362;
   - D362 diagnosed `ci-4d-evidence` as transient Cargo registry fetch failure
     for `aead`;
   - D363 stopped before CI inspection and made no repo mutation.
7. Final target check proof shows:
   - `ci-4d-evidence` success after rerun or already green;
   - public-safety success;
   - qsc-adversarial-smoke success or policy-accepted;
   - qsc-linux-full-suite success or policy-accepted;
   - macos-qsc-full-serial success or policy-accepted;
   - no terminal repo-scope red check remains.
8. Pre-closeout local validation passes:
   - `git diff --check`;
   - binding corpus validator JSON scan;
   - all qsc fuzz corpus validator JSON scan;
   - internal negative vector manifest JSON validation;
   - formal binding model;
   - formal runner;
   - qsc binding negative tests with and without `qsc_binding_fuzz_helper`;
   - refimpl signature provider-boundary test;
   - refimpl `pqkem768`;
   - root `cargo audit --deny warnings`;
   - qsc fuzz lock `cargo audit --deny warnings`;
   - `cargo fmt --check`;
   - qsc-adversarial shell syntax checks.
9. Closeout patch scope is exactly:
   - `NEXT_ACTIONS.md`;
   - `DECISIONS.md`;
   - `TRACEABILITY.md`;
   - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`;
   - `tests/NA-0493_closeout_restore_na0494_testplan.md`.
10. Final queue proof shows exactly one READY item: NA-0494.
11. Final decision proof shows D-0976 once, D-0977 absent, and duplicate
    decision count zero.
12. Added-line scan confirms no public-readiness claim, no production-readiness
    claim, no public-internet-readiness claim, no external-review-complete
    claim, no crypto-complete claim, no fuzz-complete claim, no corpus-complete
    claim, no vector-complete claim, no replay-proof claim, no downgrade-proof
    claim, no side-channel-free claim, no vulnerability-free claim, no bug-free
    claim, and no perfect-crypto claim is introduced.

## Acceptance

NA-0493 is accepted as DONE only when PR #1257 evidence, pointer recovery,
ci-4d recovery, public-safety, validator proof, local validation, exact
five-path closeout scope, and one-READY queue proof all pass. NA-0494 is
accepted as READY only as an authorization plan; no NA-0494 implementation work
is performed by this closeout.
