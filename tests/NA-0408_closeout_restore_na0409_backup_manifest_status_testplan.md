Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0408 Closeout and NA-0409 Restoration Testplan

## Purpose

Prove that NA-0408 qwork startup hardening is closed and the preserved backup
manifest/status lane is restored as the sole READY successor without
implementing NA-0409 or mutating backup, runtime, dependency, workflow, public,
or sibling-repo surfaces.

## Scope

Allowed mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0408_closeout_restore_na0409_backup_manifest_status_testplan.md`

Forbidden effects:

- NA-0409 implementation.
- backup execution or restore execution.
- qsl-backup, backup source-list, backup status, or backup plan mutation.
- durable Director State Index output.
- runtime, protocol, crypto, dependency, workflow, public-doc, website,
  README, START_HERE, qsl-server, or qsl-attachments mutation.
- public-readiness, public-technical-paper, backup-complete, restore-proof, or
  off-host-backup claims.

## Deterministic Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
from pathlib import Path
text = Path("DECISIONS.md").read_text()
for token in ["D-0799", "D-0800", "D-0801"]:
    print(token, text.count(f"- **ID:** {token}"))
PY
git diff --name-only origin/main...HEAD
git diff --check
```

Expected results:

- `READY_COUNT 1`.
- `READY NA-0409 QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`.
- `NA-0408 DONE`.
- `D-0799 1`.
- `D-0800 1`.
- `D-0801 1`.
- Changed paths are exactly the allowed closeout paths.
- `git diff --check` exits zero.

## Link and Leak Checks

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main --paths \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0408_closeout_restore_na0409_backup_manifest_status_testplan.md
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body-file> --scan-overclaims
```

Expected results:

- `TOTAL_MISSING 0`.
- `SECRET_FINDING_COUNT 0`.
- PR body has required fields and no prohibited public/backup/restore claims.

## Dependency and Safety Checks

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Expected results:

- Dependency checks pass and `rustls-webpki` remains `v0.103.13` or newer safe
  version.
- Formatting, qsc send_commit, and formal model checks pass.
- Required PR checks, including `public-safety`, pass before merge.
- Post-merge `public-safety` passes before declaring final completion.

## Acceptance

Closeout passes only if NA-0408 is DONE, NA-0409 is the sole READY item,
D-0801 exists exactly once, NA-0409 is not implemented, and all forbidden
surfaces remain unchanged.
