Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0430 Closeout and NA-0431 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan verifies the governance-only closeout that marks NA-0430 DONE
after PR #1129 merged and restores the selected NA-0431 precise-version qsc
fuzz lock retry implementation harness as the sole READY item.

## Scope

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0430_closeout_restore_na0431_testplan.md`

Forbidden changed paths:

- runtime, crypto, dependency, Cargo manifest, lockfile, workflow, script,
  executable test, fuzz target, vector, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
  qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`, and
  backup/restore paths.

## Required markers

- `NA0430_CLOSEOUT_SCOPE_OK`
- `NA0430_DONE_OK`
- `NA0431_READY_OK`
- `NA0431_PRECISE_VERSION_RETRY_SCOPE_OK`
- `NA0431_NO_RUNTIME_CRYPTO_ROOT_DEPENDENCY_WORKFLOW_TEST_VECTOR_PUBLIC_OVERCLAIM_BOUNDARY_OK`
- `NA0430_D0848_ONCE_OK`
- `NA0430_ONE_READY_INVARIANT_OK`
- `NA0430_PUBLIC_SAFETY_GREEN_OK`
- `NA0430_NO_BACKUP_RESTORE_OK`
- `NA0430_NO_QWORK_RERUN_OK`

## Validation commands

Queue and decision proof:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
import pathlib, re, collections, sys
next_actions = pathlib.Path("NEXT_ACTIONS.md").read_text()
decisions = pathlib.Path("DECISIONS.md").read_text()
ids = re.findall(r'^- \*\*ID:\*\* (D-\d{4})$', decisions, flags=re.M)
counts = collections.Counter(ids)
checks = {
    "NA0430_DONE_OK": "### NA-0430" in next_actions and "Status: DONE" in next_actions,
    "NA0431_READY_OK": "### NA-0431" in next_actions and "Status: READY" in next_actions,
    "D0847_ONCE_OK": counts.get("D-0847", 0) == 1,
    "D0848_ONCE_OK": counts.get("D-0848", 0) == 1,
    "LATEST_D0848_OK": ids[-1] == "D-0848",
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
- latest decision D-0848
- duplicate decision count zero

Scope guard:

```bash
git diff --name-only origin/main...HEAD
python3 - <<'PY'
import subprocess, sys
allowed = {
    "NEXT_ACTIONS.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
    "tests/NA-0430_closeout_restore_na0431_testplan.md",
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

- changed path count 5
- forbidden path count 0

Diff and docs checks:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- diff check passes;
- `TOTAL_MISSING 0`;
- `SECRET_FINDING_COUNT 0`.

PR body preflight:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0430_closeout_pr_body.md" --scan-overclaims
```

Expected:

- `MISSING_FIELD_COUNT 0`;
- `PROHIBITED_PHRASE_COUNT 0`.

Dependency and protocol health:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected:

- root cargo audit passes;
- root `rustls-webpki` is `v0.103.13` or newer safe version;
- root `ml-kem` remains present;
- root pqcrypto package-ID probes remain absent or zero-match;
- formatting, qsc, provider, and formal validations pass.

Public-safety:

```bash
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha <merge-sha>
```

Expected:

- PR public-safety passes before merge;
- post-merge public-safety passes before declaring closeout complete.

## Acceptance criteria

- NA-0430 is DONE.
- NA-0431 is READY and is the only READY item.
- NA-0431 scope preserves no runtime, crypto, root dependency, workflow,
  script, manifest, source, executable test, fuzz target, vector, public,
  backup, qwork, qsl-backup, or public-claim mutation unless a later exact
  directive expands scope.
- D-0848 exists once and is the latest decision.
- Changed paths are exactly the five allowed closeout paths.
- No backup or restore is run.
- No qwork/qstart/qresume command is run.
- Public-safety is green before merge and after merge.
