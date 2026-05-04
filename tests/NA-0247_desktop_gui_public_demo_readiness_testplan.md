Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0247 Desktop GUI Public Demo Readiness Test Plan

Goals: G1, G4, G5

## Objective

Validate that NA-0247 records bounded desktop GUI prototype readiness evidence without widening active operations, claiming production readiness, or changing forbidden runtime/service/protocol surfaces.

## Scope

Allowed changed paths for Packet A:

- `docs/governance/evidence/NA-0247_desktop_gui_public_demo_readiness_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0247_desktop_gui_public_demo_readiness_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `qsl/qsl-client/qsc-desktop/**` or `qsl/qsl-client/qsc/tests/**` only for directly required bounded fixes

Forbidden changed paths include `.github/**`, public-safety helpers, Cargo metadata, qsl-server, qsl-attachments, website, protocol-core, KT, SCKA, cryptographic state-machine paths, and qsc runtime/core paths outside existing desktop contract validation.

## Required Local Commands

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
```

Expected result: all commands pass.

## Desktop Package Commands

Run from `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Expected result:

- `npm ci`, `npm run build`, and `npm run prepare:sidecar` pass where Node/npm and Rust are available.
- `npm run tauri:build` passes only on a host with the required native Tauri Linux/macOS toolchain.
- If native package build is host-limited, record the exact failing command, root cause, corrective action, and closest bounded evidence in the audit.
- Do not install global tools, update dependencies, or change lockfiles in this lane.

## Governance Checks

```bash
git diff --name-only origin/main...HEAD
git diff --check
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
python3 tools/goal_lint.py
```

Goal-lint may be run with the repo's established synthetic pull-request event payload when local execution requires PR-body context.

## Queue and Decision Checks

Run the canonical queue parser and decision parser from the directive.

Expected Packet A result:

- `READY_COUNT 1`
- sole READY item remains `NA-0247`
- D-0460 exists once
- D-0461 is absent
- no duplicate decision IDs

## Evidence Acceptance

The audit must state:

- desktop GUI contract test proof;
- qsp/protocol-inactive proof;
- frontend/package/sidecar readiness proof or host-limited package gap;
- guided init/unlock/contact/readiness/send/receive walkthrough;
- keychain active operations deferred;
- handshake/session-establish UI out of scope;
- attachments UI, transcript history, and multiprofile out of scope;
- not production-ready;
- no qsl-server, qsl-attachments, website, Cargo, public-safety, branch-protection, protocol-core, KT, SCKA, or cryptographic state-machine changes.

## Post-Fix Hardening Review Checklist

- Correctness under stress: validation must include deterministic contract tests and protocol-inactive negatives.
- Minimality: Packet A should be docs/evidence/governance only unless a directly required bounded desktop validation fix appears.
- Maintainability: evidence should point to existing tests and scripts rather than inventing a parallel validation path.
- Coverage quality: tests must prove fail-closed inactive behavior and sidecar contract truth, not just frontend build success.
- Cross-lane stability: host-limited package gaps must be documented without changing Linux/macOS scope or CI requirements.
