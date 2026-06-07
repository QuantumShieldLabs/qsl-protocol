Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0433 qsc Provider Error Path / No-Mutation Findings Triage Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan validates the NA-0433 governance-only findings triage
authorization. NA-0433 consumes NA-0432 findings and selects the exact NA-0434
successor without implementing tests or mutating runtime, crypto, dependency,
Cargo, lockfile, workflow, fuzz target, vector, service, public, backup, or
qwork-tool paths.

## Scope

Allowed changed paths:

- `docs/governance/evidence/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_plan.md`
- `tests/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include runtime, crypto, dependency, Cargo manifest,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, `/backup/qsl`, and backup/restore paths.

## Required markers

- `NA0433_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0433_QUEUE_READY_OK`
- `NA0433_NA0432_FINDINGS_CONSUMED_OK`
- `NA0433_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0433_FINDINGS_MATRIX_TRIAGED_OK`
- `NA0433_TEST_IMPLEMENTATION_AUTHORIZED_OK`
- `NA0433_EXACT_TEST_PATH_SELECTED_OK`
- `NA0433_NO_RUNTIME_HOOK_AUTHORIZED_OK`
- `NA0433_FORMAL_SUPPORTING_ONLY_OK`
- `NA0433_FUZZ_COVERAGE_BACKLOG_OK`
- `NA0433_PUBLIC_CLAIM_BOUNDARY_OK`
- `NA0433_DEPENDENCY_HEALTH_GREEN_OK`
- `NA0433_D0854_DECISION_OK`
- `NA0433_TRACEABILITY_UPDATED_OK`
- `NA0433_NO_RUNTIME_CHANGE_OK`
- `NA0433_NO_CRYPTO_CHANGE_OK`
- `NA0433_NO_DEPENDENCY_CHANGE_OK`
- `NA0433_NO_CARGO_CHANGE_OK`
- `NA0433_NO_LOCKFILE_CHANGE_OK`
- `NA0433_NO_WORKFLOW_CHANGE_OK`
- `NA0433_NO_EXECUTABLE_TEST_OR_VECTOR_MUTATION_OK`
- `NA0433_NO_PUBLIC_OVERCLAIM_OK`
- `NA0433_NO_BACKUP_RESTORE_OK`
- `NA0433_ONE_READY_INVARIANT_OK`

## Queue and decision proof

Commands:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue \
  --select NA-0433 --select NA-0432 --select NA-0431 \
  --select NA-0430 --select NA-0429

python3 scripts/ci/qsl_evidence_helper.py decisions \
  --select D-0852 --select D-0853 --select D-0854 --select D-0855
```

Expected:

- `READY_COUNT 1`
- `READY NA-0433`
- NA-0432 DONE
- NA-0431 DONE
- NA-0430 DONE
- NA-0429 BLOCKED
- D-0852 once
- D-0853 once
- D-0854 once after NA-0433 patch
- D-0855 absent before optional closeout
- duplicate decision count zero

## qwork proof verification

Commands:

```bash
test -f /srv/qbuild/work/NA-0433/.qwork/startup.qsl-protocol.kv
test -f /srv/qbuild/work/NA-0433/.qwork/startup.qsl-protocol.json
python3 -m json.tool /srv/qbuild/work/NA-0433/.qwork/startup.qsl-protocol.json >/dev/null
```

Expected:

- qwork proof files exist.
- `.kv` reports startup OK, lane NA-0433, repo qsl-protocol, expected path,
  clean worktree/index/untracked, READY count 1, queue top READY NA-0433, and
  requested lane status READY.
- JSON proof is valid and mirrors the required `.kv` values.
- Codex does not run qwork, qstart, or qresume.

## Findings triage proof

Read-only evidence roots:

- `docs/governance/evidence/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_plan.md`
- `tests/NA-0432_qsl_qsc_provider_error_path_no_mutation_read_only_audit_testplan.md`
- `/home/victor/work/qsl/codex/responses/NA0432_20260607T000358Z_D276.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/`
- `qsl/qsl-client/qsc/fuzz/`
- `tools/refimpl/quantumshield_refimpl/src/crypto/`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- `formal/`

Expected classifications:

- F-0432-01: `ACCEPTED_NO_ACTION`
- F-0432-02: `NEXT_CANDIDATE`, `EVIDENCE_GAP`,
  `IMPLEMENTATION_AUTHORIZED`
- F-0432-03: `NEXT_CANDIDATE`, `EVIDENCE_GAP`,
  `IMPLEMENTATION_AUTHORIZED`
- F-0432-04: `BACKLOG_CANDIDATE`, `WATCH_ONLY`
- F-0432-05: `BACKLOG_CANDIDATE`, `WATCH_ONLY`
- F-0432-06: `CLAIM_BOUNDARY_ONLY`
- F-0432-07: `CLAIM_BOUNDARY_ONLY`, `WATCH_ONLY`
- F-0432-08: `ACCEPTED_NO_ACTION`, `WATCH_ONLY`

Expected primary decision:

`PROVIDER_ERROR_NO_MUTATION_TEST_IMPLEMENTATION_AUTHORIZED`

Expected exact future test path:

`qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

## Scope guard

Worktree-aware command before commit:

```bash
python3 - <<'PY'
import subprocess

allowed = {
    "docs/governance/evidence/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_plan.md",
    "tests/NA-0433_qsl_qsc_provider_error_path_no_mutation_findings_triage_authorization_testplan.md",
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
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PROOF_DIR/validation/na0433_pr_body.md" --scan-overclaims
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
  `qsc-adversarial-smoke`;
- local Rust phases are still expected to pass.

## Public claim boundary

NA-0433 is internal governance only. It is not production readiness. It is not
public-internet readiness. It is not external-review completion. It is not
crypto completeness. It is not side-channel freedom. It is not bug-free status.
It is not vulnerability-free status. It is not perfect-crypto proof. Cargo
audit green remains dependency-health evidence only.

## Acceptance criteria

- NA-0432 findings are consumed.
- Stewardship template summaries are recorded.
- Exact future qsc test path is selected.
- D-0854 exists once.
- D-0855 remains absent before optional closeout.
- NA-0434 implementation successor is selected, but not implemented.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, qsl-server, qsl-attachments, qshield runtime, website,
  public-doc, README, START_HERE, qwork, qsl-backup, backup status, backup
  plan, rollback, `/backup/qsl`, backup, or restore mutation occurs.
- No public readiness, production readiness, external-review completion,
  crypto-complete, side-channel-free, bug-free, vulnerability-free, or
  perfect-crypto claim is introduced.
- Public-safety is green before merge and after merge.
- Exactly one READY item remains.
