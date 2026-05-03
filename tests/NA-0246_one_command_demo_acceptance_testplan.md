Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0246 One-Command Demo Acceptance Test Plan

## Objective

Validate that NA-0246 provides a single local public-demo acceptance command with a positive establish/send/receive/decrypt flow, bounded negative reject flows, stable output markers, leak-safe output, and explicit non-production posture.

## Protected Invariant

The demo acceptance runner must remain honest, loopback-only by default, relay-authenticated for state-changing relay surfaces, fail-closed for supported negative cases, and non-production in operator-visible output. It must not change protocol wire semantics, cryptographic state machines, qsl-server, qsl-attachments, qsc-desktop, website, public-safety helpers, branch protection, Cargo manifests, or lockfiles.

## Scope Guard

Allowed changed paths:

- `scripts/ci/demo_cli_smoke.sh`
- `apps/qshield-cli/src/**` only for directly required demo acceptance behavior or stable output markers
- `docs/governance/evidence/NA-0246_one_command_demo_acceptance_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0246_one_command_demo_acceptance_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden proof:

- no `.github/**` changes
- no `scripts/ci/public_safety_gate.py` changes
- no Cargo manifest or lockfile changes
- no qsp/qsc/qsl/qsl-client/tools/inputs/protocol-core/KT/SCKA changes
- no qsc-desktop/qsl-server/qsl-attachments/website changes
- no branch-protection or public-safety/check configuration changes

## Runner Expectations

Run:

```bash
scripts/ci/demo_cli_smoke.sh
```

Expected:

- exits zero only when every invariant passes;
- starts only `127.0.0.1:<ephemeral-port>` relay service by default;
- initializes two temporary demo stores;
- registers Alice and Bob using relay authorization;
- establishes Alice/Bob demo sessions;
- sends `hello-na0246` from Alice to Bob;
- receives/decrypts on Bob and verifies intended sender/plaintext;
- emits `DEMO_WARNING_NON_PRODUCTION_RESEARCH_ONLY`;
- emits `DEMO_ACCEPTANCE_OK`;
- emits no relay token, sentinel, passphrase, auth header, or raw credential.

## Negative Flow Expectations

Required markers:

```text
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
```

Expected negative behavior:

- missing relay authorization rejects;
- malformed JSON rejects and does not echo token/sentinel input;
- invalid relay ID rejects through the CLI path;
- replayed establish record rejects deterministically.

## Decision Parser Expectation

Run the canonical decision parser.

Expected:

- D-0110 exists once;
- D-0439 through D-0458 exist once each;
- D-0459 is absent during Packet A;
- duplicate decision count is zero.

## Queue Parser Expectation

Run the canonical queue parser.

Expected during Packet A:

- READY_COUNT 1;
- READY NA-0246;
- NA-0245 through NA-0237 remain DONE.

## CI Expectations

Local validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
bash -n scripts/ci/demo_cli_smoke.sh
cargo fmt --check
cargo audit --deny warnings
cargo build --locked
cargo clippy --locked -- -D warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
```

Also run goal-lint, queue parser, decision parser, markdown inventory/link validation, changed-path scope guard, forbidden-path guard, and leak-safe scan using established repository patterns.

Required CI:

- public-safety remains required and green;
- all required protected contexts attach and pass or are accepted according to repository rules;
- no branch-protection exception, admin bypass, direct push, squash merge, rebase merge, or check spoofing.
