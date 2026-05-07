Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# NA-0253A Relay-Auth macOS Public-Safety Recovery Testplan

## Objective

Validate the bounded qsc relay-auth public-safety recovery for the macOS
`relay_auth_without_token_fails_no_mutation` failure while preserving
unauthorized fail-closed and no-mutation behavior.

## Protected Invariants

- No-token relay send fails.
- Rejected relay send does not mutate the relay inbox.
- The test does not accept arbitrary failures.
- Token, bearer, and authorization-header text do not leak to command output.
- qsc runtime relay behavior remains fail-closed.
- No protocol, wire, crypto, state-machine, branch-protection, public-safety,
  Cargo, website, qsl-server, qsl-attachments, or qsc-desktop behavior changes.

## Scope Guard

Allowed changed paths for the recovery PR:

- `qsl/qsl-client/qsc/tests/relay_auth_header.rs`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0253A_relay_auth_macos_public_safety_recovery_audit.md`
- `tests/NA-0253A_relay_auth_macos_public_safety_recovery_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo manifests or
lockfiles, qsc runtime source unless separately proven necessary, qsl-server,
qsl-attachments, qsc-desktop, website implementation, external website
repositories, public-safety configuration, and branch-protection settings.

## Required Local Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed qsl/qsl-client/qsc/tests/relay_auth_header.rs --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/governance/evidence/NA-0253A_relay_auth_macos_public_safety_recovery_audit.md --allowed tests/NA-0253A_relay_auth_macos_public_safety_recovery_testplan.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
cargo +stable test -p qsc --locked --test relay_auth_header relay_auth_without_token_fails_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test relay_auth_header -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected result:

- all commands pass;
- queue remains `READY_COUNT 1`, sole READY `NA-0253`;
- D-0474 exists once after the branch changes;
- D-0475 remains absent;
- changed paths remain inside the allowed recovery scope.

## Required CI Validation

Required PR checks must pass normally, including `public-safety` and
`macos-qsc-full-serial`. Merge must be a merge commit with no admin bypass,
direct push, squash, or rebase.

## Post-Merge Validation

After merge:

- verify D-0474 exists once on `origin/main`;
- verify queue remains `READY_COUNT 1`, sole READY `NA-0253`;
- verify `public-safety` is required and green on the new main commit.
