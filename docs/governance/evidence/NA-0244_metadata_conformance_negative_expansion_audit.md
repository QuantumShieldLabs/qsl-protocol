Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0244 Metadata Conformance Negative Expansion Audit

## Objective

Record executable evidence that the qshield demo metadata surfaces fail closed for malformed body, content-type, auth, padding/config, and sanitized-error cases without changing protocol-core or service boundaries.

## Findings

- Malformed JSON is rejected on `/register` with HTTP `400` and a constant `invalid json` error.
- Wrong `Content-Type` is rejected on JSON POST surfaces with HTTP `415` and a constant `unsupported content type` error.
- Wrong bearer scheme is rejected with HTTP `401` and a constant token error.
- Invalid padding metadata on `/send` is rejected before queue mutation with HTTP `400`.
- Invalid padding config in `qshield init` is rejected with a sanitized constant error instead of echoing the raw bucket value.
- Error-output assertions include a relay token and `NA0244_SECRET_SENTINEL` in rejected inputs and verify neither appears in responses.

## App-Side Enforcement

The existing qshield demo relay did not enforce `Content-Type` before JSON parsing and did not validate bucket consistency when raw `/send` requests supplied padding metadata. NA-0244 adds the smallest app-side checks inside `apps/qshield-cli/src/**`:

- JSON POST bodies must declare `application/json`.
- JSON parser errors are normalized to `invalid json`.
- `/send` rejects nonzero padding metadata without a bucket, zero buckets, odd-length bucketed message bodies, bucket length mismatch, and `pad_len > bucket`.
- `qshield init --padding-buckets` no longer echoes the raw rejected bucket string.

These checks are demo metadata conformance hardening only. They do not change protocol-core, qsl-server, qsl-attachments, KT, SCKA, or cryptographic state-machine behavior.

## Executable Negatives Added

- malformed JSON / invalid body: `POST /register` with a body carrying the sentinel and relay token rejects as `400`.
- wrong Content-Type: `POST /register` with `text/plain` rejects as `415`.
- token/auth edge: `Authorization: Basic <token>` rejects as `401`.
- invalid padding metadata: `POST /send` with token-derived message data and mismatched bucket rejects as `400`.
- invalid padding config: `qshield init --padding-buckets <token>` rejects locally.
- no-secret leak: every new negative checks that neither the relay token nor sentinel appears in the error body.

## Commands

Primary local command:

```bash
scripts/ci/metadata_conformance_smoke.sh
```

Additional validation bundle:

```bash
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
scripts/ci/demo_cli_smoke.sh
```

## Residual Gaps

- This lane covers qshield demo metadata surfaces only; qsl-server is intentionally untouched.
- GET `/bundle/<id>` has no JSON body and therefore no Content-Type requirement.
- Raw `/send` message hex validation remains outside this lane unless padding metadata is supplied; future metadata work can decide whether raw demo relay send should validate ciphertext shape more broadly.

## Recommendations

- Keep metadata conformance claims tied to executable smoke checks.
- Treat any future error path that echoes bearer tokens, operator secrets, or rejected raw metadata as a release-blocking regression.
- If website claims discuss metadata minimization, tie them directly to the current executable coverage and residual gaps.
