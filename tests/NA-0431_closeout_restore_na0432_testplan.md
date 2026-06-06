Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0431 Closeout and NA-0432 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify the governance-only closeout that marks NA-0431 DONE after PR #1132
merged and post-merge public-safety completed success, then restores
`NA-0432 -- QSL qsc Provider Error Path / No-Mutation Read-Only Audit Plan` as
the sole READY item.

## Protected invariants

- Exactly one READY item remains.
- NA-0431 is DONE only after PR #1132 merge, public-safety success, qsc
  adversarial success, root cargo audit green, and nested fuzz lock audit green.
- NA-0432 is restored READY but not implemented by this closeout.
- NA-0429 remains BLOCKED and NA-0430 remains DONE.
- D-0851 exists once and D-0852 remains absent.
- Cargo audit output remains dependency-health evidence only. It is not
  public-readiness, production-readiness, external-review, crypto-complete,
  vulnerability-free, bug-free, perfect-crypto, or side-channel-free proof.

## Allowed scope

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0431_closeout_restore_na0432_testplan.md`

## Forbidden scope

Forbidden changed paths:

- runtime, crypto, dependency, Cargo manifest, lockfile, workflow, script,
  executable test, fuzz target, vector, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
  qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`, and
  backup/restore paths.

Forbidden actions:

- running qwork, qstart, qresume, backup, restore, sudo, cargo update, or cargo
  generate-lockfile.
- creating or implying public-readiness, production-readiness,
  external-review, crypto-complete, vulnerability-free, bug-free,
  perfect-crypto, or side-channel-free claims.

## PR #1132 merge/public-safety checks

Commands:

```bash
gh pr view 1132 --repo QuantumShieldLabs/qsl-protocol --json number,state,mergedAt,mergeCommit,headRefOid,title,url,mergeStateStatus
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --repo QuantumShieldLabs/qsl-protocol --sha 77df962590e4457d18ec04732b09413798f13043 --report-only
```

Expected:

- PR #1132 state is MERGED.
- PR #1132 merge commit begins with `77df962590e4`.
- `public-safety`, `qsc-linux-full-suite`, and `macos-qsc-full-serial` are
  completed success on `77df962590e4`.

## qsc-adversarial success check

Commands:

```bash
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/<pr-head-sha>/check-runs?per_page=100"
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/77df962590e4457d18ec04732b09413798f13043/check-runs?per_page=100"
```

Expected:

- PR-head `qsc-adversarial-smoke` is completed success.
- Merge-commit `qsc-adversarial-smoke` is completed success, or an accepted
  check-shape note explains why the PR-head success plus green public-safety is
  sufficient.

## nested fuzz lock audit green check

Commands:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Expected:

- Nested qsc fuzz lock audit passes.
- Nested pqcrypto residual package IDs are absent or explicitly explained by
  accepted D274/D275 evidence.

## root cargo audit green check

Commands:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Expected:

- Root cargo audit passes.
- Root `rustls-webpki` is `v0.103.13` or newer safe version.
- Root `ml-kem` remains present.
- Root pqcrypto package-ID probes remain absent or zero-match.

## NA-0431 DONE / NA-0432 READY check

Commands:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
import collections
import pathlib
import re

next_actions = pathlib.Path("NEXT_ACTIONS.md").read_text()
decisions = pathlib.Path("DECISIONS.md").read_text()
ids = re.findall(r"^- \*\*ID:\*\* (D-\d{4})$", decisions, flags=re.M)
counts = collections.Counter(ids)
checks = {
    "NA0431_DONE_OK": "### NA-0431" in next_actions and "Status: DONE" in next_actions,
    "NA0432_READY_OK": "### NA-0432" in next_actions and "Status: READY" in next_actions,
    "NA0430_DONE_OK": "### NA-0430" in next_actions and "Status: DONE" in next_actions,
    "NA0429_BLOCKED_OK": "### NA-0429" in next_actions and "Status: BLOCKED" in next_actions,
    "D0850_ONCE_OK": counts.get("D-0850", 0) == 1,
    "D0851_ONCE_OK": counts.get("D-0851", 0) == 1,
    "D0852_ABSENT_OK": counts.get("D-0852", 0) == 0,
    "DUPLICATES_ABSENT_OK": not any(n > 1 for n in counts.values()),
}
for key, value in checks.items():
    print(f"{key} {'OK' if value else 'FAIL'}")
raise SystemExit(0 if all(checks.values()) else 1)
PY
```

Expected:

- `READY_COUNT 1`
- `READY NA-0432`
- NA-0431 DONE
- NA-0430 DONE
- NA-0429 BLOCKED
- D-0851 exists once
- D-0852 absent
- duplicate decision count zero

## qsl-protocol closeout scope guard

Commands:

```bash
git diff --name-only origin/main...HEAD
python3 - <<'PY'
import subprocess

allowed = {
    "NEXT_ACTIONS.md",
    "DECISIONS.md",
    "TRACEABILITY.md",
    "docs/ops/ROLLING_OPERATIONS_JOURNAL.md",
    "tests/NA-0431_closeout_restore_na0432_testplan.md",
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

- changed path count 5.
- forbidden path count 0.
- no required closeout path is missing.

## no runtime/dependency/workflow/test/vector mutation

Commands:

```bash
git diff --name-only origin/main...HEAD | rg -n '^(Cargo\\.lock|Cargo\\.toml|qsl/qsl-client/qsc/|qsl-server/|qsl-attachments/|qshield/|website/|\\.github/|scripts/|formal/|inputs/|tests/.*\\.(rs|py|json|toml|lock))' || true
```

Expected:

- zero forbidden runtime, dependency, workflow, executable test, fuzz target, or
  vector changed paths.

## no public overclaim

Commands:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0431_closeout_pr_body.md" --scan-overclaims
```

Expected:

- diff check passes.
- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.
- `MISSING_FIELD_COUNT 0`.
- `PROHIBITED_PHRASE_COUNT 0`.
