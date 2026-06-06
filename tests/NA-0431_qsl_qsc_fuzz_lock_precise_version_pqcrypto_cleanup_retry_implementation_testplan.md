Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0431 qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan records how NA-0431 verifies the precise-version nested qsc fuzz
lock cleanup without mutating runtime code, crypto code, manifests, root Cargo
files, workflows, scripts, executable tests, fuzz targets, vectors, public
surfaces, backup/local-ops state, or qwork tooling.

## Scope

Allowed changed paths:

- `qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `docs/governance/evidence/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_harness.md`
- `tests/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include runtime, crypto, root Cargo files, qsc
`Cargo.toml`, fuzz `Cargo.toml`, workflow, script, executable test, fuzz target,
vector, qsl-server, qsl-attachments, qshield runtime, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status,
backup plan, rollback subtree, `/backup/qsl`, and backup/restore paths.

## Required markers

- `NA0431_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0431_D272_D273_INHERITANCE_OK`
- `NA0431_ROLLBACK_COPY_SHA_MATCH_OK`
- `NA0431_PRECISE_VERSION_COMMANDS_OK`
- `NA0431_LOCKFILE_ONLY_SCOPE_OK`
- `NA0431_NESTED_AUDIT_GREEN_OK`
- `NA0431_ROOT_AUDIT_GREEN_OK`
- `NA0431_PQCRYPTO_RESIDUAL_REMOVED_OK`
- `NA0431_ML_DSA_PKCS8_COMPATIBILITY_OK`
- `NA0431_QSC_FUZZ_BINS_BUILD_OK`
- `NA0431_QSC_SEND_COMMIT_OK`
- `NA0431_PROVIDER_PQKEM768_OK`
- `NA0431_FORMAL_CHECKS_OK`
- `NA0431_QSC_ADVERSARIAL_STATUS_CLASSIFIED_OK`
- `NA0431_NO_RUNTIME_CRYPTO_WORKFLOW_TEST_VECTOR_MUTATION_OK`
- `NA0431_NO_PUBLIC_OVERCLAIM_OK`
- `NA0431_NO_BACKUP_RESTORE_OK`
- `NA0431_SUCCESSOR_SELECTED_OK`
- `NA0431_ONE_READY_INVARIANT_OK`

## Validation commands

Queue and decision proof:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
import pathlib, re, collections, sys
decisions = pathlib.Path("DECISIONS.md").read_text()
ids = re.findall(r'^- \*\*ID:\*\* (D-\d{4})$', decisions, flags=re.M)
counts = collections.Counter(ids)
checks = {
    "D0847_ONCE_OK": counts.get("D-0847", 0) == 1,
    "D0848_ONCE_OK": counts.get("D-0848", 0) == 1,
    "D0849_ONCE_OK": counts.get("D-0849", 0) == 1,
    "D0850_ONCE_OK": counts.get("D-0850", 0) == 1,
    "D0851_ABSENT_OK": counts.get("D-0851", 0) == 0,
    "LATEST_D0850_OK": ids[-1] == "D-0850",
    "DUPLICATES_ABSENT_OK": not any(n > 1 for n in counts.values()),
}
for key, value in checks.items():
    print(f"{key} {'OK' if value else 'FAIL'}")
raise SystemExit(0 if all(checks.values()) else 1)
PY
```

Expected:

- `READY_COUNT 1`
- `READY NA-0431`
- NA-0430 DONE
- NA-0429 BLOCKED
- D-0847 once
- D-0848 once
- D-0849 once
- D-0850 once
- D-0851 absent
- duplicate decision count zero

Scope guard:

```bash
git diff --name-only origin/main...HEAD
python3 - <<'PY'
import subprocess, sys
allowed = {
    "qsl/qsl-client/qsc/fuzz/Cargo.lock",
    "docs/governance/evidence/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_harness.md",
    "tests/NA-0431_qsl_qsc_fuzz_lock_precise_version_pqcrypto_cleanup_retry_implementation_testplan.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
}
paths = set(subprocess.check_output(
    ["git", "diff", "--name-only", "origin/main...HEAD"], text=True
).splitlines())
extra = sorted(paths - allowed)
missing = sorted(allowed - paths)
print(f"CHANGED_PATH_COUNT {len(paths)}")
print(f"FORBIDDEN_PATH_COUNT {len(extra)}")
for path in extra:
    print(f"FORBIDDEN_PATH {path}")
for path in missing:
    print(f"MISSING_EXPECTED_PATH {path}")
raise SystemExit(1 if extra or missing else 0)
PY
```

Expected:

- changed path count 6
- forbidden path count 0

Diff and docs checks:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0431_pr_body.md" --scan-overclaims
```

Expected:

- diff check passes;
- `TOTAL_MISSING 0`;
- `SECRET_FINDING_COUNT 0`;
- `MISSING_FIELD_COUNT 0`;
- `PROHIBITED_PHRASE_COUNT 0`.

Dependency and qsc validation:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
CARGO_TARGET_DIR="$PROOF_DIR/cargo_target/qsc_fuzz_bins" \
  cargo +nightly build --locked \
  --manifest-path qsl/qsl-client/qsc/fuzz/Cargo.toml --bins
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected:

- root cargo audit passes;
- nested qsc fuzz lock audit passes;
- root `rustls-webpki` remains `v0.103.13` or newer safe version;
- root `ml-kem` remains present;
- root pqcrypto package-ID probes report absence;
- nested qsc fuzz lock pqcrypto residual search returns zero matches;
- qsc fuzz-bin build passes from the proof-root target directory;
- formatting, qsc, provider, and formal validations pass.

Local qsc adversarial script:

```bash
if [ -x scripts/ci/qsc_adversarial.sh ]; then
  scripts/ci/qsc_adversarial.sh
else
  sh scripts/ci/qsc_adversarial.sh
fi
```

Expected:

- pass if local cargo-fuzz is installed;
- if local cargo-fuzz is not installed, record exact output and proceed only
  when root audit, nested audit, qsc fuzz-bin build, qsc `send_commit`,
  provider `pqkem768`, and formal checks passed;
- PR CI `qsc-adversarial-smoke` must pass before merge.

## Acceptance criteria

- Only the nested qsc fuzz lock changed as implementation.
- Governance/evidence changes are limited to the five allowed NA-0431 paths.
- Nested qsc fuzz lock audit is green.
- Root cargo audit is green.
- pqcrypto residual package IDs are absent from the nested fuzz lock.
- The `ml-dsa 0.1.0-rc.7` release-candidate compatibility chain remains in
  place.
- qsc fuzz-bin build passes.
- qsc/refimpl/formal validations pass.
- qsc adversarial status is classified truthfully, and PR CI
  `qsc-adversarial-smoke` passes before merge.
- No runtime, crypto, workflow, script, test, vector, public, backup, qwork,
  qsl-backup, qsl-server, or qsl-attachments mutation occurs.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No side-channel-free claim is introduced.
- No bug-free claim is introduced.
- No vulnerability-free claim is introduced.
- No perfect-crypto claim is introduced.
- Exactly one READY item remains.
