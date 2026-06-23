Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0526 Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Testplan

## Objective

Validate NA-0526 evidence for restaging the retained remote `qsc` binary after D422 remediated both lockfiles to `quinn-proto 0.11.15`.

Accepted classification: `REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`.

## Scope

Allowed checked-in paths:

- `docs/governance/evidence/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_harness.md`
- `tests/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path may be mutated.

## Expected evidence

- qwork proof files were read and copied without running qwork/qstart/qresume.
- Startup queue proof shows READY_COUNT 1 and READY NA-0526.
- D-1040 and D-1041 each exist once.
- D-1042 exists once after patch.
- D-1043 is absent before optional closeout.
- Duplicate decision count is zero.
- Root and nested qsc fuzz lockfiles contain `quinn-proto 0.11.15`.
- Current local qsc was built from clean source.
- No qsc runtime/dependency diff exists since the D424 source commit.
- Local qsc hash is recorded.
- Retained remote qsc pre-state hash equals the old known stale hash.
- Stage hash equals local hash.
- Final remote hash equals local hash and differs from the old stale hash.
- Remote qsc help smoke passed.
- Final remote qsc is retained for NA-0527.
- No qsc send/receive command was run.
- No remote E2EE command was run.
- No qsl-server or qsl-attachments path was used.
- Selected successor is `NA-0527 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Harness`.

## Static validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_harness.md \
  --allowed tests/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_harness.md \
  tests/NA-0526_qsl_remote_qsc_prebuilt_binary_restaging_after_quinn_proto_remediation_implementation_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

## Queue and decision validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0526 --select NA-0525 --select NA-0524 --select NA-0523
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1040 --select D-1041 --select D-1042 --select D-1043
```

Expected:

- READY_COUNT 1.
- READY NA-0526 before optional closeout.
- D-1042 count 1.
- D-1043 count 0 before optional closeout.
- DUPLICATE_COUNT 0.

## Marker validation

Evidence must contain:

```text
REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED
NA-0527 -- QSL Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Retry After Restaging Implementation Harness
```

Evidence must state that no remote qsc E2EE and no qsc send/receive were run.

## Private material validation

Run scans over added evidence and governance diff to prove no private material was checked in:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Also run the proof-root private-key block marker scan and token-style fixture scan. Expected: zero findings.

## Required local validation

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Expected: all pass.

## Post-fix hardening review

1. Correctness under stress: stage-then-hash-then-final-move prevents replacing the retained binary until the staged hash matches local provenance.
2. Minimality: only governance/evidence paths are changed; no runtime source, dependency, workflow, corpus, formal, service, public, or backup mutation is introduced.
3. Maintainability: the old stale hash, new retained hash, cleanup command, and NA-0527 recheck requirement are recorded.
4. Coverage quality: validation proves queue/decision state, stage/final hash matching, scope guard, secret scan, audits, focused qsc tests, corpus validators, formal checks, fmt, and shell syntax.
5. Cross-lane stability: macOS/Linux-sensitive qsc validation remains through existing qsc tests and CI gates; the remote operation only restages the Linux prebuilt binary under the approved qslcodex test path.

## Forbidden outcomes

- remote qsc E2EE run;
- qsc send/receive run;
- wrong-peer or stale-trust testing;
- qsl-server use;
- qsl-attachments use;
- package installation;
- remote source checkout/build;
- qwork/qstart/qresume execution;
- qsl-backup execution;
- qsc source/test/fuzz/Cargo mutation;
- workflow/script/helper/dependency mutation;
- corpus/vector/input mutation;
- public-readiness claim;
- production-readiness claim;
- public-internet-readiness claim;
- crypto-complete claim;
- identity-complete claim;
- trust-complete claim;
- replay-proof claim;
- downgrade-proof claim;
- vulnerability-free claim;
- bug-free claim;
- perfect-crypto claim.
