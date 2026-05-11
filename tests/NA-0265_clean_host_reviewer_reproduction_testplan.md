Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0265 Clean-Host Reviewer Reproduction Testplan

## Objective

Prove the public demo evidence can be reproduced from clean instructions on a
fresh source workdir, or stop with exact prerequisites. Remote thin-client proof
is optional and must be truthfully labeled if used.

## Protected Invariants

- Reviewer instructions are executable.
- Proof mode is truthfully labeled.
- Demo remains non-production.
- No production readiness is claimed.
- No token, secret, passphrase, auth header, or plaintext leak is embedded in
  docs or transcripts.
- No protocol, wire, crypto, auth, or state-machine semantics change.
- No qsl-server or qsl-attachments production-hardening change.
- No qsc-desktop implementation change.
- No website or external website change.
- `public-safety` remains required and green.

## Allowed Scope

- `docs/demo/**`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/governance/evidence/NA-0265_clean_host_reviewer_reproduction_audit.md`
- `tests/NA-0265_clean_host_reviewer_reproduction_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- protocol, wire, crypto, auth, or state-machine code
- qsl-server production hardening
- qsl-attachments production hardening
- qsc-desktop implementation
- website or external website source
- branch-protection or public-safety configuration

## Clean Source Proof

Required commands from a fresh clone:

```bash
git clone https://github.com/QuantumShieldLabs/qsl-protocol.git "$BASE/clean-source/qsl-protocol"
cd "$BASE/clean-source/qsl-protocol"
git checkout 1e7d0a63be3157bf561d58ef67f3c68e19f04091
export CARGO_TARGET_DIR="$BASE/cargo-target"
cargo build -p qshield-cli --locked
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline \
  DEMO_STRESS_ARTIFACT_DIR="$BASE/demo_adversarial_stress" \
  scripts/ci/demo_adversarial_stress.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Required NA-0265 markers:

```text
NA0265_CLEAN_SOURCE_REPRO_OK
NA0265_REVIEWER_POSITIVE_DEMO_OK
NA0265_REVIEWER_NEGATIVE_REJECT_OK
NA0265_REVIEWER_NO_SECRET_LEAK_OK
NA0265_REVIEWER_NON_PRODUCTION_BOUNDARY_OK
NA0265_REVIEWER_REPRODUCTION_OK
```

## Remote Thin-Client Proof

If run, remote proof must:

- use `/home/qslcodex/qsl-na0265-reviewer/`;
- copy only minimal local binaries/scripts;
- run no build, test, fuzzing, full-suite, package install, or toolchain
  installation on the remote;
- record copied file sizes/checksums;
- prove at least one remote positive client flow and one fail-closed negative
  flow;
- scan transcript/reject bodies for generated token and sentinel leakage; and
- record cleanup.

If any of those conditions cannot be met, do not emit
`NA0265_REMOTE_THIN_CLIENT_REPRO_OK`.

## Positive / Negative Proof

Positive proof must include successful qshield send/receive/decrypt.

Negative proof must include at least one fail-closed reject and, for NA-0265,
the clean-source run includes missing auth, malformed input, replay, invalid id,
tampered attachment integrity, and KT-negative reject proof.

## No-Leak Proof

The proof must record:

- `DEMO_NO_SECRET_LEAK_OK`;
- `DEMO_STRESS_NO_SECRET_LEAK_OK`; and
- `NA0265_REVIEWER_NO_SECRET_LEAK_OK`.

Added docs must avoid literal tokens, passphrases, auth headers, route tokens,
and long secret-like values.

## Non-Production Posture

The runbook and audit must state that the proof is non-production demo evidence
only. It must not claim production relay, qsl-server, qsl-attachments, desktop,
KT deployment, public internet, or external review completion.

## CI / Public-Safety Expectations

- `public-safety` remains required in branch protection.
- Starting main `public-safety` is green.
- Local validation must include demo smoke, stress, metadata smoke,
  `send_commit`, formal/model checks if present, cargo audit, rustls-webpki
  dependency path proof, queue/decision helpers, scope guard, link-check,
  leak-scan, and goal-lint.
- Required PR checks must pass normally before merge.
