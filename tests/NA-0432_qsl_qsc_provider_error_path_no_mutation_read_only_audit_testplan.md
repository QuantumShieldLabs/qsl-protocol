Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0432 qsc Provider Error Path / No-Mutation Read-Only Audit Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan records how NA-0432 validates the read-only qsc provider-error
path and no-mutation evidence audit without mutating runtime code, crypto code,
dependencies, Cargo manifests, lockfiles, workflows, executable tests, fuzz
targets, vectors, public surfaces, qwork tooling, or backup/local-ops state.

## Scope

Allowed changed paths:

- `docs/governance/evidence/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_plan.md`
- `tests/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, `/backup/qsl`, and backup/restore paths.

## Required markers

- `NA0432_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0432_QUEUE_READY_OK`
- `NA0432_D0851_INHERITANCE_OK`
- `NA0432_D0852_DECISION_OK`
- `NA0432_PROVIDER_ERROR_MARKERS_INVENTORIED_OK`
- `NA0432_QSC_CALL_PATH_REVIEW_OK`
- `NA0432_NO_MUTATION_CLASSIFIED_OK`
- `NA0432_TEST_FUZZ_FORMAL_REVIEW_OK`
- `NA0432_FINDINGS_MATRIX_OK`
- `NA0432_SUCCESSOR_SELECTED_OK`
- `NA0432_NO_RUNTIME_CHANGE_OK`
- `NA0432_NO_CRYPTO_CHANGE_OK`
- `NA0432_NO_DEPENDENCY_CHANGE_OK`
- `NA0432_NO_CARGO_CHANGE_OK`
- `NA0432_NO_LOCKFILE_CHANGE_OK`
- `NA0432_NO_WORKFLOW_CHANGE_OK`
- `NA0432_NO_EXECUTABLE_TEST_OR_VECTOR_MUTATION_OK`
- `NA0432_NO_PUBLIC_OVERCLAIM_OK`
- `NA0432_NO_BACKUP_RESTORE_OK`
- `NA0432_ONE_READY_INVARIANT_OK`

## Queue and decision proof

Commands:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0432 --select NA-0431 --select NA-0430 --select NA-0429
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0850 --select D-0851 --select D-0852 --select D-0853
```

Expected:

- `READY_COUNT 1`
- `READY NA-0432`
- NA-0431 DONE
- NA-0430 DONE
- NA-0429 BLOCKED
- D-0850 once
- D-0851 once
- D-0852 once after NA-0432 patch
- D-0853 absent before optional closeout
- duplicate decision count zero

## qwork proof verification

Commands:

```bash
test -f /srv/qbuild/work/NA-0432/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0432/.qwork/startup.qsl-protocol.json
python3 -m json.tool /srv/qbuild/work/NA-0432/.qwork/startup.qsl-protocol.json >/dev/null
```

Expected:

- qwork proof files exist.
- `.kv` reports startup OK, lane NA-0432, repo qsl-protocol, expected path,
  clean worktree/index/untracked, READY count 1, queue top READY NA-0432, and
  requested lane status READY.
- JSON proof is valid and mirrors the required `.kv` values.
- Codex does not run qwork, qstart, or qresume.

## Provider-error inventory commands

Commands:

```bash
rg -n -i -e 'pq_encap_failed' -e 'pq_decap_failed' -e 'encap_failed' -e 'decap_failed' -e 'provider' -e 'PqKem768' -e 'StdCrypto' -e 'pqkem' -e 'kem' -e 'ml-kem' -e 'ML-KEM' -e 'error' -e 'reject' -e 'fail' -e 'abort' -e 'commit' -e 'transcript' -e 'send_commit' -e 'recv' -e 'handshake' -e 'state' -e 'mutate' -e 'pending' -e 'session' -e 'rollback' \
  qsl/qsl-client/qsc/ \
  tools/refimpl/quantumshield_refimpl/src/crypto/ \
  tools/refimpl/quantumshield_refimpl/tests/ \
  qsl/qsl-client/qsc/fuzz/ \
  formal/ \
  inputs/ \
  tests/ \
  docs/governance/evidence/ \
  --glob '*.rs' --glob '*.toml' --glob '*.lock' --glob '*.md' --glob '*.json' \
  --count-matches

rg -n -i 'pq_encap_failed|pq_decap_failed|encap_failed|decap_failed' \
  qsl/qsl-client/qsc/ \
  tools/refimpl/quantumshield_refimpl/src/crypto/ \
  tools/refimpl/quantumshield_refimpl/tests/ \
  qsl/qsl-client/qsc/fuzz/ \
  formal/ inputs/ tests/ docs/governance/evidence/
```

Expected:

- `pq_encap_failed` and `pq_decap_failed` appear in qsc runtime handshake code.
- Direct executable qsc test references to those exact marker names are absent
  or explicitly classified.
- Counts are recorded as planning signals only.

## Scope guard

Worktree-aware command before commit:

```bash
python3 - <<'PY'
import subprocess

allowed = {
    "docs/governance/evidence/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_plan.md",
    "tests/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_testplan.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}
tracked = set(subprocess.check_output(["git", "diff", "--name-only"], text=True).splitlines())
untracked = set(subprocess.check_output(["git", "ls-files", "--others", "--exclude-standard"], text=True).splitlines())
paths = tracked | untracked
extra = sorted(paths - allowed)
print(f"CHANGED_PATH_COUNT {len(paths)}")
print(f"FORBIDDEN_PATH_COUNT {len(extra)}")
for path in extra:
    print(f"FORBIDDEN_PATH {path}")
raise SystemExit(1 if extra else 0)
PY
```

Expected:

- changed path count 5
- forbidden path count 0

## Local docs and evidence checks

Commands:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0432_pr_body.md" --scan-overclaims
```

Expected:

- diff check passes
- `TOTAL_MISSING 0`
- `SECRET_FINDING_COUNT 0`
- `MISSING_FIELD_COUNT 0`
- `PROHIBITED_PHRASE_COUNT 0`

## Dependency and qsc validation

Commands:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected:

- root cargo audit passes
- nested qsc fuzz lock audit passes
- root `rustls-webpki` is `v0.103.13` or newer safe version
- root `ml-kem` remains present
- root pqcrypto package-ID probes report absence
- nested qsc fuzz lock pqcrypto residual scan returns zero matches
- formatting, qsc, provider, and formal validations pass

## qsc adversarial smoke

Command:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

Expected:

- pass if local cargo-fuzz is installed;
- if local cargo-fuzz is unavailable, record exact output and rely on PR CI
  `qsc-adversarial-smoke` while requiring root audit, nested audit, qsc
  `send_commit`, provider `pqkem768`, and formal checks to pass.

Observed local result for NA-0432:

- `adversarial_properties`: 8 passed, 0 failed.
- `adversarial_miri`: 6 passed, 0 failed.
- cargo-fuzz stage stopped with `error: no such command: fuzz`.
- classification: recoverable local cargo-fuzz availability caveat.
- corrective action: no local install or toolchain mutation; require PR CI
  `qsc-adversarial-smoke`.

## Acceptance criteria

- qsc provider-error path audit is read-only.
- `NO_MUTATION_PROOF_QSC_LEVEL_PARTIAL` or another exact classification is
  recorded with rationale.
- Findings matrix is created.
- NA-0433 successor is selected.
- D-0852 exists once.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, qsl-server, qsl-attachments, qshield runtime, website,
  public-doc, README, START_HERE, qwork, qsl-backup, backup status, backup
  plan, rollback, or `/backup/qsl` mutation occurs.
- No public readiness, production readiness, external-review completion,
  crypto-complete, side-channel-free, bug-free, vulnerability-free, or
  perfect-crypto claim is introduced.
- Exactly one READY item remains.
