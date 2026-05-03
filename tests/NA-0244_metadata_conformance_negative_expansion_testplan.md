Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0244 Metadata Conformance Negative Expansion Test Plan

## Objective

Expand executable qshield metadata conformance negatives for malformed request/error/auth/padding surfaces while preserving protocol-core and service boundaries.

## Scope Guard

Allowed implementation paths:

- `scripts/ci/metadata_conformance_smoke.sh`
- `apps/qshield-cli/src/**` only for bounded metadata/demo conformance enforcement
- `docs/governance/evidence/NA-0244_metadata_conformance_negative_expansion_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0244_metadata_conformance_negative_expansion_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include `.github/**`, public-safety helper/configuration, Cargo files, qsl-server, qsl-attachments, qsc-desktop, website, qsc/qsl runtime paths, tools/refimpl, tools/actors, inputs, protocol-core, KT, SCKA, and cryptographic state-machine paths.

## Protected Invariants

- Metadata minimization claims remain test-backed.
- Malformed JSON/content-type requests reject deterministically.
- Auth/token edge cases remain fail-closed.
- Padding config and bucket metadata reject deterministically where supported.
- Error outputs do not echo relay tokens or supplied sentinel secrets.
- qsl-server and qsl-attachments boundaries remain untouched.

## Executable Tests

Primary smoke command:

```bash
scripts/ci/metadata_conformance_smoke.sh
```

The smoke must prove:

- malformed JSON rejects with sanitized `invalid json`;
- wrong `Content-Type` rejects with `415`;
- malformed bearer scheme rejects with `401` or `403`;
- invalid padding metadata rejects with `400`;
- invalid padding bucket config rejects locally;
- all new negative error bodies omit the relay token and sentinel marker;
- existing queue-cap, rate-limit, padding bucket, and token quota checks remain green.

## No-Regression Commands

```bash
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
scripts/ci/demo_cli_smoke.sh
```

## Decision Parser Expectations

After the patch:

- D-0110 exists once.
- D-0439 through D-0454 exist once each.
- D-0455 is absent.
- No duplicate decision IDs exist.

## Queue Expectations

Packet A must not edit `NEXT_ACTIONS.md`; the queue parser must continue to report:

```text
READY_COUNT 1
READY NA-0244 Metadata Conformance Negative Expansion
```

## CI Expectations

- Required GitHub contexts pass normally.
- `public-safety` remains required and green.
- No qsl-server, qsl-attachments, qsc-desktop, website, Cargo, public-safety, or branch-protection drift.
