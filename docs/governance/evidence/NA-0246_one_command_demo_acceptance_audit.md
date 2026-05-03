Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0246 One-Command Demo Acceptance Audit

Directive: QSL-DIR-2026-05-03-026 / NA-0246

## Objective

Record executable evidence for a single local public-demo acceptance command that starts only loopback services, initializes two demo peers, proves a positive establish/send/receive/decrypt flow, proves bounded negative reject flows, and keeps non-production posture explicit.

## One-Command Runner

Run from repo root:

```bash
scripts/ci/demo_cli_smoke.sh
```

The command builds `qshield-cli` and `refimpl_actor`, allocates a free `127.0.0.1` port, starts the qshield demo relay on loopback only, creates isolated temporary stores for Alice and Bob, and exits nonzero on the first failed invariant.

## Positive Flow

The runner proves:

- Alice and Bob stores initialize against `http://127.0.0.1:<ephemeral-port>`.
- The relay serves only on `127.0.0.1` by default.
- Alice and Bob register through the configured relay token.
- Alice establishes with Bob and Bob establishes with Alice using the explicit demo unauthenticated override.
- Alice sends `hello-na0246` to Bob.
- Bob receives/decrypts and the output contains both `hello-na0246` and `from alice`.

Positive proof marker:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

## Negative Flows

The runner proves these bounded current demo-surface rejects:

- missing relay authorization on `POST /register` rejects with HTTP `401` or `403`;
- malformed JSON on `POST /register` rejects with HTTP `400` and `invalid json`;
- invalid relay ID through the CLI register path rejects;
- replayed establish record rejects with HTTP `409` and `establish replay`.

Stable negative proof markers:

```text
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
```

## Stable Output Markers

Expected successful marker set:

```text
DEMO_ACCEPTANCE_START
DEMO_WARNING_NON_PRODUCTION_RESEARCH_ONLY
DEMO_LOOPBACK_ONLY_DEFAULT
DEMO_INIT_TWO_PEERS_OK
DEMO_LOOPBACK_RELAY_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_REGISTER_AUTHORIZED_PEERS_OK
DEMO_ESTABLISH_OK
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NO_SECRET_LEAK_OK
DEMO_ACCEPTANCE_OK
```

## Non-Production Posture

The runner emits `DEMO_WARNING_NON_PRODUCTION_RESEARCH_ONLY` and still uses explicit demo-only surfaces. It does not claim production readiness, anonymity, full release readiness, KT-negative proof, downgrade proof, qsl-server readiness, qsl-attachments readiness, or desktop GUI readiness.

## Leak-Safe Output

The runner sends a sentinel plus the relay token through malformed rejected input and verifies response bodies do not echo either value. It also captures command/relay output into temporary files and checks that neither the relay token nor the sentinel appears before printing `DEMO_NO_SECRET_LEAK_OK`.

NA-0246 also redacts qshield demo relay startup output so the relay no longer prints the configured/generated token.

## Commands

Primary command:

```bash
scripts/ci/demo_cli_smoke.sh
```

Validation bundle:

```bash
bash -n scripts/ci/demo_cli_smoke.sh
cargo fmt --check
cargo audit --deny warnings
cargo build --locked
cargo clippy --locked -- -D warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

## Residual Gaps

- KT-negative demo readiness is not claimed because the current qshield public-demo acceptance surface does not carry truthful KT evidence.
- Downgrade-negative demo readiness is not faked in this runner; downgrade/no-mutation proof remains tied to existing Suite-2 vector/refimpl coverage and prior demo negative lanes.
- Attachment-path demo readiness is out of scope for NA-0246 and remains tied to qsc/qsl-attachments evidence.
- qsl-server, qsl-attachments, qsc-desktop, website, protocol-core, KT, SCKA, and cryptographic state-machine paths are untouched.

## Recommendations

- Keep public-demo claims tied to the marker set above.
- Treat any future relay/demo output that exposes tokens, passphrases, auth headers, or rejected secret-bearing inputs as a release-blocking regression.
- Add a later KT-negative demo lane only when the demo surface can carry real KT evidence without faking unsupported behavior.
