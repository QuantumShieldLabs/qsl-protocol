Goals: G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0252 Repo-Local Evidence Helper Test Plan

## Objective

Validate that NA-0252 adds read-only repo-local evidence helpers for recurring governance and CI diagnostics without changing protocol, runtime, crypto, demo, service, branch-protection, public-safety, workflow, or Cargo behavior.

## Protected Invariants

- helper commands are evidence/reporting only
- helper commands do not edit files
- helper commands do not mutate branch protection
- helper commands do not spoof checks
- helper commands do not merge PRs
- helper commands do not rerun workflows by default
- public-safety remains required and green
- NA-0252 remains READY until a later closeout directive
- leak-scan findings never print matched sensitive text, source-line excerpts, raw credential values, Authorization header values, matched secret-like substrings, source-line-derived metadata, or `SCAN_LINE_COUNT`

## Scope Guard

Allowed changed paths for Packet B:

- `scripts/ci/qsl_evidence_helper.py`
- `tests/NA-0252_repo_local_evidence_helper_testplan.md`
- `docs/governance/evidence/NA-0252_repo_local_evidence_helper_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, `scripts/ci/public_safety_gate.py`, `scripts/ci/qsc_adversarial.sh`, Cargo files, qsp/qsc/qsl/qsl-client/apps/tools/inputs/formal paths, qsc-desktop, qsl-server, qsl-attachments, website source, branch-protection settings, public-safety/check configuration, and protocol/runtime/crypto/demo/service implementation code.

Validation commands:

```bash
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed scripts/ci/qsl_evidence_helper.py \
  --allowed tests/NA-0252_repo_local_evidence_helper_testplan.md \
  --allowed docs/governance/evidence/NA-0252_repo_local_evidence_helper_audit.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --forbidden .github/** \
  --forbidden scripts/ci/public_safety_gate.py \
  --forbidden scripts/ci/qsc_adversarial.sh \
  --forbidden Cargo.toml \
  --forbidden Cargo.lock \
  --forbidden qsp/** \
  --forbidden qsc/** \
  --forbidden qsl/** \
  --forbidden qsl-client/** \
  --forbidden apps/** \
  --forbidden tools/** \
  --forbidden inputs/** \
  --forbidden formal/** \
  --forbidden qsc-desktop/** \
  --forbidden qsl-server/** \
  --forbidden qsl-attachments/** \
  --forbidden website/**
```

Expected result: changed paths stay inside the allowed Packet B set, and forbidden count is zero.

## Helper Command Validation

Required local commands:

```bash
python3 -m py_compile scripts/ci/qsl_evidence_helper.py
python3 scripts/ci/qsl_evidence_helper.py --help
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths DECISIONS.md TRACEABILITY.md NEXT_ACTIONS.md
python3 scripts/ci/qsl_evidence_helper.py checks-summary --pr 752 --report-only --allow-codeql-neutral
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --report-only
python3 scripts/ci/qsl_evidence_helper.py ci-admission-preflight --pr 752 --report-only
```

Expected result:

- top-level help lists every required subcommand
- queue parser reports `READY_COUNT 1` and `READY NA-0252`
- decision parser reports D-0471 once after Packet B and no duplicates
- link-check reports `TOTAL_MISSING 0`
- leak-scan reports zero secret findings
- checks-summary reports all required contexts for PR #752 without failing in report-only mode
- public-safety-status reports latest main public-safety success
- ci-admission-preflight reports no circular dependency risk for PR #752

## Leak-Scan Redaction Regression

Use a temporary file only; generate the fake marker at runtime and do not commit it:

```bash
secret_fixture="$(mktemp)"
fake_suffix="$(python3 - <<'PY'
import secrets
alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
print("".join(secrets.choice(alphabet) for _ in range(36)))
PY
)"
fake_token="ghp_${fake_suffix}"
printf '%s\n' "temporary fake token for redaction regression: ${fake_token}" > "${secret_fixture}"
set +e
leak_output="$(python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode full --paths "${secret_fixture}" 2>&1)"
leak_rc="$?"
set -e
printf '%s\n' "${leak_output}"
test "${leak_rc}" -eq 2
printf '%s\n' "${leak_output}" | grep 'SECRET_FINDING type=github_token path='
printf '%s\n' "${leak_output}" | grep 'line=1'
printf '%s\n' "${leak_output}" | grep 'redaction=\[redacted\]'
! printf '%s\n' "${leak_output}" | grep "${fake_token}"
! printf '%s\n' "${leak_output}" | grep "${fake_suffix}"
! printf '%s\n' "${leak_output}" | grep 'SCAN_LINE_COUNT'
rm -f "${secret_fixture}"
```

Expected result: leak-scan exits nonzero for the finding, reports only finding metadata plus `[redacted]`, and does not print the generated fake token, its large distinguishing substring, or `SCAN_LINE_COUNT`.

## PR Body Preflight Fixtures

Use temporary files only:

```bash
valid_body="$(mktemp)"
invalid_body="$(mktemp)"
printf '%s\n' \
  'Goals: G3, G4, G5' \
  'Impact: Adds read-only evidence helpers.' \
  'No-regression: No public-safety or branch-protection weakening.' \
  'Tests/Vectors: Helper command validation.' > "${valid_body}"
printf '%s\n' \
  'Goals: G3' \
  'Impact: incomplete body' > "${invalid_body}"
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "${valid_body}"
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "${invalid_body}"; test "$?" -ne 0
rm -f "${valid_body}" "${invalid_body}"
```

Expected result: the valid body passes and the invalid body fails.

## Queue And Decision Expectations

Run the canonical queue parser and helper queue parser.

Expected result:

- `READY_COUNT 1`
- `READY NA-0252 Repo-Local Evidence and CI Recovery Helper Toolkit`
- NA-0251 through NA-0237 remain `DONE`
- NA-0252 remains `READY`

Run the canonical decision parser and helper decision parser.

Expected result after Packet B:

- D-0110 exists once
- D-0439 through D-0471 exist once each
- D-0472 is absent
- duplicate count is zero

## Main Health And CI Expectations

Local validation bundle:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

Expected result: all pass.

Required PR checks:

- ci-4a
- ci-4b
- ci-4c
- ci-4d
- ci-4d-dur
- demo-cli-build
- demo-cli-smoke
- formal-scka-model
- goal-lint
- metadata-conformance-smoke
- suite2-vectors
- CodeQL
- macos-qsc-qshield-build
- public-safety

Expected result: required checks pass normally before merge. CodeQL neutral is acceptable only under the repository's existing neutral/skipped policy.

## Post-Fix Hardening Review Checklist

1. Correctness under stress: helper commands fail nonzero for ambiguous READY count, duplicate decisions, missing/red checks, missing links, secret findings, and invalid PR bodies unless report-only is explicitly requested for GitHub diagnostics.
2. Minimality: helper is one standard-library Python file plus governance evidence; no workflow, public-safety helper, Cargo, runtime, protocol, crypto, demo, service, or website implementation paths change.
3. Maintainability: parsing and GitHub helpers are factored by command, required contexts are centralized, and file writes are absent.
4. Coverage quality: command validation exercises every required subcommand, valid/invalid PR body cases, and a temporary fake-secret regression proving redacted leak-scan output.
5. Cross-lane stability: helper uses Python standard library and POSIX-compatible Git/GitHub CLI calls, with no platform-specific filesystem mutation.
