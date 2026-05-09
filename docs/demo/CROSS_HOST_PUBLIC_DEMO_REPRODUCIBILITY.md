Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# Cross-Host Public Demo Reproducibility

## Posture

This document is for the non-production qshield public demo only. It does not
claim production relay readiness, production authentication UX, qsl-server
readiness, qsl-attachments readiness, production desktop readiness, anonymity,
metadata-free messaging, or release approval.

The demo relay is allowed only inside controlled local, LAN, or Tailscale scope
for this runbook. Do not open router or firewall ports. Do not publish the relay
on the public internet. Do not change Tailscale admin/API state.

## Proof Mode Used

NA-0257 used Mode 2: LAN-style same-host multi-endpoint proof.

The relay was bound to the host's already configured Tailscale interface address
instead of loopback, and the Alice/Bob clients used independent stores and
independent qshield command invocations against that Tailscale URL. This moves
the proof beyond loopback-only execution, but it is not labeled as real
two-host proof.

Real two-host/Tailscale proof was not used because visible Linux Tailscale peers
were reachable, but SSH command execution was not already configured under
strict host-key checking. Adding host keys, requesting credentials, copying the
repo to a peer, or changing peer policy was outside this directive.

## NA-0257 Artifact Package

Artifact directory:

```text
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/
```

Manifest:

```text
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/ARTIFACT_MANIFEST.txt
```

Transcript:

```text
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/mode2_tailscale_same_host_proof.log
```

Exact executed proof script:

```text
/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_20260509T130756Z/mode2_tailscale_same_host_proof.sh
```

The transcript ended with:

```text
NA0257_POSITIVE_SEND_RECEIVE_DECRYPT_OK
NA0257_NO_SECRET_LEAK_OK
NA0257_MODE2_TAILSCALE_SAME_HOST_PROOF_OK
```

## Network Assumptions

- Hostname: `ideacentre`.
- Tailscale command: available at `/usr/bin/tailscale`.
- Local Tailscale address used for the relay bind: `100.82.111.69`.
- Proof relay bind: `100.82.111.69:33821`.
- Client relay URL: `http://100.82.111.69:33821`.
- Client endpoints: separate local Alice and Bob stores and separate qshield
  process invocations.
- Firewall/router changes: none.
- Tailscale admin/API changes: none.
- Public internet exposure: none intended or required.
- Tailscale DNS health warning was present during preflight, but direct
  Tailscale-IP reachability for the local bind proof succeeded.

## Exact Mode 2 Repeat Commands

Run from the qsl-protocol repository root on a host that already has a
Tailscale interface address:

```bash
export TAILSCALE_IP="$(ip -4 addr show tailscale0 | awk '/inet / {print $2}' | cut -d/ -f1 | head -n 1)"
export ARTIFACT_DIR="/srv/qbuild/tmp/NA-0257_cross_host_demo_artifacts_$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$ARTIFACT_DIR"
cargo build -p qshield-cli --locked
cargo build -p refimpl_actor --locked
```

Then run the checked artifact script shape from the NA-0257 transcript package,
or create an equivalent local script that:

- generates `QSHIELD_RELAY_TOKEN` without printing it;
- allocates an ephemeral port on `$TAILSCALE_IP`;
- initializes separate Alice and Bob stores with `--relay-url
  http://$TAILSCALE_IP:<port>`;
- starts `qshield relay serve --listen "$TAILSCALE_IP:<port>"
  --allow-public --i-understand-this-is-unsafe`;
- verifies `/health` through `curl --noproxy '*'`;
- verifies missing-auth, malformed-input, invalid-id, and replay rejects;
- registers Alice and Bob;
- establishes demo sessions with `--demo-unauthenticated-override`;
- sends a plaintext from Alice to Bob;
- receives/decrypts on Bob and verifies sender plus plaintext; and
- checks transcript and relay output for no live token/sentinel leakage.

The exact executed command file for this NA-0257 run is outside the repository
at the artifact path above so the repo does not accumulate generated transcript
or token-adjacent test material.

## Two-Host / Tailscale Operator Runbook

Use this only when both hosts are already authorized operator hosts and no new
credentials, Tailscale admin changes, firewall changes, or public exposure are
required.

Host A, relay:

```bash
cd /path/to/qsl-protocol
cargo build -p qshield-cli --locked
export QSHIELD_RELAY_TOKEN="$(python3 - <<'PY'
import os
print(os.urandom(16).hex())
PY
)"
export HOST_A_TAILSCALE_IP="<host-a-tailscale-ip>"
export QSHIELD_BIN="${CARGO_TARGET_DIR:-target}/debug/qshield"
"$QSHIELD_BIN" relay serve \
  --listen "${HOST_A_TAILSCALE_IP}:18080" \
  --allow-public \
  --i-understand-this-is-unsafe
```

Host A, Alice store in a second terminal:

```bash
export RELAY_URL="http://<host-a-tailscale-ip>:18080"
export QSHIELD_BIN="${CARGO_TARGET_DIR:-target}/debug/qshield"
"$QSHIELD_BIN" init --store /tmp/qshield-alice --relay-url "$RELAY_URL" --relay-token "$QSHIELD_RELAY_TOKEN"
"$QSHIELD_BIN" register --store /tmp/qshield-alice --id alice
"$QSHIELD_BIN" establish --store /tmp/qshield-alice --peer bob --demo-unauthenticated-override
```

Host B, Bob store:

```bash
cd /path/to/qsl-protocol
cargo build -p qshield-cli --locked
cargo build -p refimpl_actor --locked
export QSHIELD_ACTOR="${CARGO_TARGET_DIR:-target}/debug/refimpl_actor"
export QSHIELD_BIN="${CARGO_TARGET_DIR:-target}/debug/qshield"
export RELAY_URL="http://<host-a-tailscale-ip>:18080"
export QSHIELD_RELAY_TOKEN="<operator-secret-token-from-host-a-secret-channel>"
"$QSHIELD_BIN" init --store /tmp/qshield-bob --relay-url "$RELAY_URL" --relay-token "$QSHIELD_RELAY_TOKEN"
"$QSHIELD_BIN" register --store /tmp/qshield-bob --id bob
"$QSHIELD_BIN" establish --store /tmp/qshield-bob --peer alice --demo-unauthenticated-override
"$QSHIELD_BIN" send --store /tmp/qshield-bob --peer alice --text "hello-two-host-demo" --demo-unauthenticated-override
```

Host A, receive:

```bash
"$QSHIELD_BIN" recv --store /tmp/qshield-alice --demo-unauthenticated-override
```

Expected positive marker from the operator transcript:

```text
from bob: hello-two-host-demo
```

Expected negative checks to repeat on the relay URL:

- missing bearer token rejects with `401` or `403`;
- malformed JSON rejects with `400` and sanitized `invalid json`;
- invalid relay ID rejects through the CLI path;
- duplicate establish record rejects with `409` and sanitized replay wording;
- no output prints the live relay token.

## Positive Path Proof

The NA-0257 Mode 2 transcript proves:

- qshield CLI and refimpl actor built successfully;
- Alice and Bob stores initialized independently;
- the relay served health on the Tailscale interface bind;
- Alice and Bob registered authorized bundles;
- Alice and Bob established demo sessions with explicit demo override;
- Alice sent `hello-na0257-tailscale-mode2`;
- Bob received/decrypted that plaintext; and
- Bob's output included `from alice`.

Stable marker:

```text
NA0257_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

## Negative / Fail-Closed Proof

The NA-0257 Mode 2 transcript proves:

- missing relay auth rejected;
- malformed relay input rejected without echoing supplied token/sentinel material;
- invalid relay ID rejected through the CLI path;
- replayed establish record rejected; and
- no reject path silently downgraded into success.

Stable markers:

```text
NA0257_NEGATIVE_AUTH_REJECT_OK
NA0257_NEGATIVE_MALFORMED_REJECT_OK
NA0257_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
NA0257_NEGATIVE_REPLAY_REJECT_OK
```

## No Secret Leakage Statement

The proof generated a relay token at runtime, used it only through process
environment / command arguments needed by the demo, and did not print the live
token into the transcript. The proof also injected sentinel material into
rejected malformed input and verified command output, reject bodies, and relay
startup output before printing:

```text
NA0257_NO_SECRET_LEAK_OK
```

## Known Gaps

- Real two-host execution remains unproven in this run because no peer had
  already safe SSH command execution under strict host-key checking.
- The relay is still a non-production in-memory qshield demo relay.
- This proof does not harden qsl-server or qsl-attachments.
- This proof does not prove production relay authentication UX.
- KT-negative demo readiness remains open.
- Attachment demo readiness remains open.
- Native desktop package and screenshot proof remain separate NA-0258 work.
- Metadata phase-2 and external review readiness remain separate work.

## Related Evidence

- [Public demo touch-and-feel readiness](PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [Demo acceptance criteria](DEMO_ACCEPTANCE_CRITERIA.md)
- [NA-0257 audit](../governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md)
- [NA-0257 testplan](../../tests/NA-0257_cross_host_demo_reproducibility_testplan.md)
