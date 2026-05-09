Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0257 Cross-Host Demo Reproducibility Test Plan

## Objective

Validate public demo reproducibility beyond loopback-only proof using the
strongest feasible environment mode on the current host, while preserving
non-production posture, relay authorization, fail-closed reject behavior, and
truthful network assumptions.

## Protected Invariants

- Demo remains non-production.
- No production relay claim.
- No production-ready demo claim.
- Relay auth remains required.
- Positive path remains inspectable.
- Negative/reject paths remain fail-closed.
- No token or secret leakage.
- Network, firewall, Tailscale, and host assumptions are explicit.
- Simulated or LAN-style proof is not mislabeled as real two-host proof.
- Protocol/crypto state machine is unchanged.
- qsl-server and qsl-attachments are untouched.
- Website/external website, `.github`, branch protection, public-safety
  configuration, Cargo manifests, and Cargo lockfiles are untouched.

## Allowed Scope

- `scripts/ci/demo_cli_smoke.sh` only if required for cross-host mode
  parameterization.
- `docs/demo/**`.
- `apps/qshield-cli/**` only for a proven minimal demo CLI flag/runbook fix.
- `docs/governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md`.
- `tests/NA-0257_cross_host_demo_reproducibility_testplan.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` if consistent with evidence pattern.
- `scripts/ci/metadata_conformance_smoke.sh` only for a minimal smoke
  label/assertion adjustment that does not weaken checks.

## Forbidden Scope

- `.github/**`.
- Cargo manifests and lockfiles.
- qsp or qsc protocol-core / crypto state-machine files.
- qsl-server.
- qsl-attachments.
- qsc-desktop implementation changes.
- website or external website repositories.
- production relay/service implementation.
- branch-protection settings.
- public-safety/check configuration.
- firewall/router/Tailscale admin mutations.

## Proof Mode

Selected proof mode for the NA-0257 run:

```text
Mode 2 - LAN-style same-host multi-endpoint proof using the host Tailscale interface bind.
```

Mode 1 real two-host proof is acceptable only when an already authorized peer is
reachable and remote command execution is already configured safely without
credentials, host-key mutation, repo-copy ambiguity, firewall changes, or
Tailscale admin changes.

Mode 3 simulated proof is acceptable only when neither safe real two-host nor
LAN/Tailscale-interface proof is feasible.

## Positive Path Proof

Run the selected proof command set from the runbook:

```bash
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/mode2_tailscale_same_host_proof.sh
```

Expected stable marker:

```text
NA0257_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

The proof must show:

- qshield CLI and refimpl actor build;
- Alice and Bob use separate stores;
- relay binds to the documented address and port;
- relay health succeeds through the selected endpoint;
- Alice and Bob register authorized bundles;
- Alice and Bob establish demo sessions using explicit demo-only override;
- Alice sends plaintext;
- Bob receives/decrypts the expected plaintext; and
- sender/recipient output remains inspectable.

## Negative / Reject Proof

The same proof command must emit:

```text
NA0257_NEGATIVE_AUTH_REJECT_OK
NA0257_NEGATIVE_MALFORMED_REJECT_OK
NA0257_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
NA0257_NEGATIVE_REPLAY_REJECT_OK
```

Expected:

- missing relay authorization rejects;
- malformed relay input rejects with sanitized output;
- invalid relay ID rejects;
- replayed establish record rejects;
- rejected paths do not silently downgrade or fallback to success.

## No Token / Secret Leakage Proof

The same proof command must emit:

```text
NA0257_NO_SECRET_LEAK_OK
```

The validation bundle must also run:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- no live relay token, passphrase, auth header value, route token, or secret-like
  test material is committed;
- generated transcripts remain outside the repository under `/srv/qbuild/tmp/`;
- committed evidence uses descriptive summaries and short SHAs.

## Network Assumptions

Record:

- host name;
- bind interface/address and port;
- relay URL;
- client endpoint assumptions;
- proof mode label;
- whether Tailscale is installed/authenticated;
- whether visible peers were reachable;
- why stronger proof was or was not safe;
- firewall/router/Tailscale admin mutation status.

## Tailscale / Two-Host Prerequisites

Real two-host proof requires:

- both hosts are already controlled operator hosts;
- Tailscale is installed and authenticated on both hosts;
- Host A's Tailscale IP is reachable from Host B;
- qsl-protocol checkout and build prerequisites already exist on both hosts;
- command execution is already configured safely or the operator manually runs
  the Host B steps;
- relay token is transferred through an operator-approved secret channel and is
  not printed into transcripts;
- no public internet exposure, firewall/router changes, Tailscale admin/API
  changes, or branch-protection changes are needed.

## CI / Public-Safety Expectations

Required local validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow 'docs/demo/**' --allow 'docs/governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md' --allow 'tests/NA-0257_cross_host_demo_reproducibility_testplan.md' --allow 'DECISIONS.md' --allow 'TRACEABILITY.md' --allow 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- READY_COUNT `1`, READY `NA-0257`;
- D-0481 exists once after implementation;
- D-0482 absent before closeout;
- no duplicate decision IDs;
- no forbidden paths touched;
- public-safety required and green before PR;
- required PR checks pass normally before merge;
- merge uses merge commit only, no direct push, no admin bypass, no squash, and
  no rebase.

## Post-Fix Hardening Review Checklist

- Correctness under stress: positive and negative proof markers come from actual
  qshield/curl execution over the selected endpoint.
- Minimality: changes remain docs/governance/evidence unless a proven in-scope
  demo defect requires a bounded fix.
- Maintainability: runbook points to existing qshield surfaces and artifact
  command sets rather than creating a parallel production relay.
- Coverage quality: proof includes happy path, auth reject, malformed reject,
  invalid-ID reject, replay reject, and no-token-leak assertions.
- Cross-lane stability: Linux/macOS protected checks and public-safety remain
  unchanged; qsl-server, qsl-attachments, qsc-desktop implementation, and
  website lanes remain untouched.
