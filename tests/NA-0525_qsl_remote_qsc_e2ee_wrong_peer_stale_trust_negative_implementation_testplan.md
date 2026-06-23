Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0525 Remote qsc E2EE Wrong-Peer / Stale-Trust Negative Implementation Testplan

## Objective

Validate NA-0525 evidence for the retained-qsc freshness gate and stale-after-D422 fail-closed result. The accepted classification is `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.

## Scope

Allowed checked-in paths:

- `docs/governance/evidence/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_harness.md`
- `tests/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo path, workflow/script/helper path, dependency/lockfile path, corpus/vector/input path, formal/refimpl/service/public/backup path, qsl-server path, or qsl-attachments path may be mutated.

## Expected evidence

- qwork proof files were read and copied without running qwork/qstart/qresume.
- Startup queue proof shows READY_COUNT 1 and READY NA-0525.
- D-1037, D-1038, and D-1039 each exist once.
- D-1040 exists once after patch.
- D-1041 is absent before optional closeout.
- Duplicate decision count is zero.
- Root and nested qsc fuzz lockfiles contain `quinn-proto 0.11.15`.
- Current local qsc was built from clean source.
- Local qsc hash differs from retained remote qsc hash.
- Retained remote qsc hash equals old known retained hash.
- Result classification is `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.
- No qsc send/receive command was run.
- No remote E2EE command was run.
- No remote E2EE root was created.
- No qsl-server or qsl-attachments path was used.
- Selected successor is `NA-0526 -- QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness`.

## Static validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_harness.md \
  --allowed tests/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  docs/governance/evidence/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_harness.md \
  tests/NA-0525_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_negative_implementation_testplan.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

## Queue and decision validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0525 --select NA-0524 --select NA-0523 --select NA-0522
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1037 --select D-1038 --select D-1039 --select D-1040 --select D-1041
```

Expected:

- READY_COUNT 1.
- READY NA-0525 before optional closeout.
- D-1040 count 1.
- D-1041 count 0 before optional closeout.
- DUPLICATE_COUNT 0.

## Marker validation

Evidence must contain:

```text
REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION
NA-0526 -- QSL Remote qsc Prebuilt Binary Restaging After quinn-proto Remediation Implementation Harness
```

Evidence must state that no remote qsc E2EE and no qsc send/receive were run.

## Private material validation

Run scans over the added evidence and governance diff to prove no private material was checked in. The private-key block marker scan should be executed from the directive proof root rather than embedded here as literal marker text:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected: zero findings.

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

1. Correctness under stress: stale retained qsc after dependency-security remediation fails closed before remote E2EE.
2. Minimality: only governance/evidence paths are changed; no runtime or dependency mutation is introduced.
3. Maintainability: the selected successor is exact and restages the binary before retrying identity/trust negatives.
4. Coverage quality: validation proves queue/decision state, stale classification, scope guard, secret scan, audits, focused qsc tests, corpus validators, formal checks, fmt, and shell syntax.
5. Cross-lane stability: macOS/Linux-sensitive qsc validation remains through the existing qsc tests and CI gates; no platform-specific runtime change is made here.

## Forbidden outcomes

- remote qsc E2EE run with stale retained qsc;
- qsc send/receive run;
- remote E2EE root creation;
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
