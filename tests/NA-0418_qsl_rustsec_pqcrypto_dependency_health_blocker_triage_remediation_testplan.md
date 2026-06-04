Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-04

# NA-0418 RustSec pqcrypto Dependency Health Blocker Triage Remediation Testplan

Goals: G4

## Purpose

Validate that NA-0418 truthfully triages and remediates the pqcrypto RustSec
dependency-health blocker that stopped D256, without weakening crypto/security
boundaries, without using an ineligible audit waiver, and without resuming the
operator-packet verification lane.

## Scope

Allowed qsl-protocol changes:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `docs/governance/evidence/NA-0418_qsl_rustsec_pqcrypto_dependency_health_blocker_triage_remediation.md`
- `tests/NA-0418_qsl_rustsec_pqcrypto_dependency_health_blocker_triage_remediation_testplan.md`
- `Cargo.lock`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- `qsl/qsl-client/qsc/src/tui/controller/render.rs`

Forbidden scope includes qwork/qstart/qresume execution by Codex, sudo,
generated packet script execution, backup execution, restore execution,
qsl-backup mutation, `/backup/qsl` mutation, rollback subtree mutation,
backup status/plan mutation, workflow mutation, qsl-server mutation,
qsl-attachments mutation, qshield runtime mutation, website/public-doc/README/
START_HERE mutation, public technical paper work, and public-claim expansion.

## Required Assertions

- qwork proof files exist, parse, and match live repo state.
- Codex does not run qwork, qstart, or qresume.
- PR #1104 is MERGED at `4f6cc35fec89`.
- Start queue proof is READY_COUNT `1`, READY `NA-0418`.
- NA-0417 is DONE.
- D-0821, D-0822, and D-0823 each exist once.
- D-0824 is absent before the patch and exists once after the patch.
- D-0825 remains absent before optional closeout.
- duplicate decision count remains `0`.
- Initial `cargo audit --deny warnings` fails only on the three pqcrypto
  RustSec warnings or on directly related findings.
- Reachability is classified before remediation.
- Runtime/security-critical reachability does not receive an audit waiver.
- Selected remediation removes or replaces the pqcrypto crates.
- `cargo audit --deny warnings` passes after remediation.
- inverse cargo trees for `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals` prove package removal.
- `cargo tree -i rustls-webpki --locked` remains on `v0.103.13` or newer.
- qsl-backup SHA remains unchanged.
- Codex ops source inclusion count in qsl-backup remains `1`.
- operator-packet state is inspected read-only and preserved for NA-0419.
- NA-0419 is not implemented by this remediation PR.
- no public-readiness, backup-complete, complete-vulnerability-absence, or
  perfect-crypto claim is introduced.

## Dependency / Provider Validation Commands

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo tree -p quantumshield_refimpl --features pqcrypto --locked
cargo tree -p qsc --locked
cargo test -p quantumshield_refimpl --features pqcrypto --locked
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

## qsl-protocol Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file /tmp/na0418_pr_body.md --scan-overclaims
cargo audit --deny warnings
cargo fmt --check
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If qshield is feasible and impacted, also run:

```bash
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
```

## Scope Guard

The final qsl-protocol changed path set must be limited to the allowed paths in
this testplan. No qsl-server, qsl-attachments, qshield runtime, website, public
docs, README, START_HERE, workflow, backup status/plan, qwork/qstart/qresume/
qshell, qsl-backup, `/backup/qsl`, or rollback subtree path may be mutated.

## Public-Safety and CI

Before merge, required PR checks must pass, including `public-safety`. After
merge, public-safety must complete success on the merge commit. Use bounded REST
polling only; do not use watch modes.

## Acceptance Criteria

- RustSec pqcrypto blocker is confirmed and remediated.
- Affected crates are removed from the root cargo tree without audit waiver.
- Provider boundary tests pass, including wrong-length fail-closed behavior.
- qsc and formal checks pass.
- qsl-backup and operator-packet state are preserved read-only.
- NA-0418 remains READY until separate closeout.
- NA-0419 remains preserved but not implemented.
