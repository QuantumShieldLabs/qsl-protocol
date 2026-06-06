Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0430 qsc Adversarial Fuzz Validation Blocker Triage Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan records how NA-0430 proves the failed PR #1127 qsc adversarial
fuzz blocker was triaged without mutating runtime code, crypto code,
dependencies, manifests, lockfiles, workflows, executable tests, fuzz targets,
vectors, public surfaces, backup/local-ops state, or qwork tooling.

## Scope

Allowed changed paths for the NA-0430 evidence PR:

- `docs/governance/evidence/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_plan.md`
- `tests/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths:

- runtime, crypto, dependency, Cargo manifest, lockfile, workflow, script,
  executable test, fuzz target, vector, qsl-server, qsl-attachments, qshield
  runtime, website, public docs, README, START_HERE, qwork/qstart/qresume/qshell,
  qsl-backup, backup status, backup plan, rollback subtree, `/backup/qsl`, and
  backup/restore paths.

## Required evidence markers

- `NA0430_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0430_PR1127_FAILURE_CONSUMED_OK`
- `NA0430_QSC_ADVERSARIAL_SMOKE_FAILURE_CLASSIFIED_OK`
- `NA0430_LOCKFILE_DIFF_ANALYZED_OK`
- `NA0430_PROOF_ROOT_PRECISE_RETRY_SIMULATION_OK`
- `NA0430_AUTHORIZATION_DECISION_OK`
- `NA0430_SUCCESSOR_SELECTED_OK`
- `NA0430_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0430_NO_RUNTIME_CRYPTO_DEPENDENCY_MUTATION_OK`
- `NA0430_NO_WORKFLOW_TEST_VECTOR_MUTATION_OK`
- `NA0430_NO_PUBLIC_OVERCLAIM_OK`
- `NA0430_NO_BACKUP_RESTORE_OK`
- `NA0430_ONE_READY_INVARIANT_OK`

## Validation commands

Queue and decision proof:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
rg -n "\\*\\*ID:\\*\\* D-0847|\\*\\*ID:\\*\\* D-0848" DECISIONS.md
```

Expected:

- `READY_COUNT 1`
- `READY NA-0430`
- NA-0429 BLOCKED
- NA-0428 DONE
- latest decision D-0847 after this PR
- D-0844 once
- D-0845 once
- D-0846 once
- D-0847 once
- D-0848 absent
- duplicate decision count zero

Scope guard:

```bash
git diff --name-only origin/main...HEAD
python3 - <<'PY'
import subprocess, sys
allowed = {
    "docs/governance/evidence/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_plan.md",
    "tests/NA-0430_qsl_qsc_adversarial_fuzz_validation_blocker_triage_testplan.md",
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
sys.exit(1 if extra or missing else 0)
PY
```

Expected:

- changed path count 5
- forbidden path count 0

Diff hygiene:

```bash
git diff --check
```

Expected: pass.

Manual link-integrity check:

```bash
python3 - <<'PY'
import pathlib, re

repo = pathlib.Path(".").resolve()
md_files = []
for pattern in ("*.md", "**/*.md"):
    for p in repo.glob(pattern):
        if ".git/" in p.as_posix():
            continue
        if p.is_file():
            md_files.append(p)
md_files = sorted(set(md_files))

link_re = re.compile(r'\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)')
missing = []

for md in md_files:
    text = md.read_text(encoding="utf-8", errors="replace")
    for raw in link_re.findall(text):
        target = raw.strip()
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        abs_target = (md.parent / target).resolve()
        if not abs_target.exists():
            missing.append((md.relative_to(repo).as_posix(), target))

for src, target in missing:
    print(f"MISSING_LINK {src} -> {target}")
print(f"TOTAL_MISSING {len(missing)}")
raise SystemExit(1 if missing else 0)
PY
```

Expected:

- `TOTAL_MISSING 0`

Leak and overclaim scans:

```bash
python3 - <<'PY'
import re, subprocess, sys
diff = subprocess.check_output(["git", "diff", "--cached"], text=True, errors="replace")
secret_patterns = [
    r"BEGIN [A-Z ]*PRIVATE KEY",
    r"ghp_[A-Za-z0-9_]{20,}",
    r"github_pat_[A-Za-z0-9_]{20,}",
    r"AKIA[0-9A-Z]{16}",
]
findings = []
for pat in secret_patterns:
    if re.search(pat, diff):
        findings.append(pat)
print(f"SECRET_FINDING_COUNT {len(findings)}")
sys.exit(1 if findings else 0)
PY

python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0430_pr_body.md" --scan-overclaims
```

Expected:

- `SECRET_FINDING_COUNT 0`
- `PROHIBITED_PHRASE_COUNT 0`

Dependency and implementation health:

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
- root `ml-kem` is present;
- root pqcrypto package-ID probes are absent or zero-match;
- formatting, qsc, provider, and formal validations pass.

Proof-root reproduction evidence:

```bash
# PR #1127 lock in proof copy: expected to fail with ml-dsa/pkcs8 E0277.
cargo +nightly build \
  --manifest-path "$PROOF_DIR/simulations/pr1127_lock_build_repro/qsl/qsl-client/qsc/fuzz/Cargo.toml" \
  --locked --bins

# Precise retry proof copy: expected to pass.
cargo +nightly build \
  --manifest-path "$PROOF_DIR/simulations/selective_qsc_refimpl_plus_precise/qsl/qsl-client/qsc/fuzz/Cargo.toml" \
  --locked --bins
```

Expected:

- PR #1127 proof copy fails before fuzz execution with `ml-dsa 0.1.0-rc.7`,
  `pkcs8 0.11.0`, and Rust `E0277`;
- precise retry proof copy passes and keeps `pkcs8 0.11.0-rc.11`;
- both proof runs write only under `/srv/qbuild/tmp/...`.

Public-safety and PR body:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0430_pr_body.md --scan-overclaims
```

Expected:

- PR body includes `Goals: G1, G2, G3, G4, G5` near the top;
- required metadata fields are present;
- no prohibited affirmative public-claim phrase is introduced.

## Acceptance criteria

- PR #1127 failure is consumed and cited.
- Lockfile diff explains why the broad refresh failed the fuzz build.
- Proof-root precise retry evidence supports lockfile-only authorization.
- D-0847 exists once and D-0848 remains absent.
- Selected successor is
  `NA-0431 -- QSL qsc Fuzz Lock Precise-Version pqcrypto Cleanup Retry Implementation Harness`.
- PR #1127 branch retention recommendation is recorded.
- Changed paths are exactly the five allowed NA-0430 paths.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, service, public, backup, restore, qwork, or qsl-backup
  mutation occurs.
- Public-safety passes before merge and after merge.
- Exactly one READY item remains.
