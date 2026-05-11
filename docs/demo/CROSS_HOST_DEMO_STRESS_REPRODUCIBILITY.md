Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# Cross-Host Demo Stress Reproducibility

## Purpose

This document records the NA-0263 real two-host Tailscale client/relay proof
for the non-production `qshield` demo stress surface.

The proof does not claim production relay readiness, public internet exposure
safety, qsl-server readiness, qsl-attachments readiness, production desktop
readiness, release approval, or production hardening.

## Proof Mode

Proof mode achieved:

```text
real two-host Tailscale client/relay proof
```

Stable marker emitted by the artifact run:

```text
NA0263_PROOF_MODE=real-two-host-tailscale
```

Host roles:

- Host A: local qbuild Ubuntu host, relay and Alice client.
- Host B: trusted remote SSH alias `remote`, Bob client.

Private addresses used:

- Host A Tailscale IP: `100.82.111.69`.
- Host B Tailscale IP: `100.99.234.5`.
- Relay bind: `100.82.111.69:38685`.
- Relay URL: `http://100.82.111.69:38685`.

No public internet target was used. No router, firewall, Tailscale admin/API,
VPN, DNS, or branch-protection setting was changed. SSH host-key verification
was not bypassed.

## Artifact Package

Artifact directory:

```text
/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/
```

Transcript files:

- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/local_transcript.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_transcript.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_identity_recheck.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_sudo_recheck.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_resource_preflight.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_light_tool_preflight.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_binary_checksums.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_listener_cleanup.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/remote_token_secret_file_cleanup.log`
- `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/ARTIFACT_MANIFEST.txt`

Remote working directory:

```text
/home/qslcodex/qsl-na0263/
```

Remote copied binaries:

- `/home/qslcodex/qsl-na0263/bin/qshield`
- `/home/qslcodex/qsl-na0263/bin/refimpl_actor`

No remote package installation was required.

Remote thin-client resource proof:

```text
nproc: 4
memory: 15Gi total, 13Gi available
home/tmp filesystem: 916G size, 842G available, 4% used
light tools: /usr/bin/bash, /usr/bin/sh, /usr/bin/tar, /usr/bin/ldd, /usr/bin/ss
```

The remote host was used only as a bounded private-network client endpoint. It
did not run `cargo build`, `cargo test`, Node/npm installs/builds, Rust
toolchain installation, fuzzing, full suites, large package installs, package
upgrades, or parallel stress jobs.

Remote copied binary evidence:

```text
qshield: 46M, sha256 9df7b1dbb29cd43aa60cb51bc0b1a7521a8b446f621eb5b061ab3bf07694072a
refimpl_actor: 32M, sha256 6e5bb00cb8cad51cbd9487b02cef7acfb8ce3af50061548126251f1a068accd1
```

## Exact Command Shape

Build on Host A:

```bash
cargo build -p qshield-cli -p refimpl_actor --locked
```

Copy only the built demo binaries to Host B:

```bash
ssh remote 'mkdir -p /home/qslcodex/qsl-na0263/bin && chmod 700 /home/qslcodex/qsl-na0263/bin'
scp /srv/qbuild/cache/targets/qsl-protocol/debug/qshield \
  /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor \
  remote:/home/qslcodex/qsl-na0263/bin/
ssh remote 'chmod 700 /home/qslcodex/qsl-na0263/bin/qshield /home/qslcodex/qsl-na0263/bin/refimpl_actor'
```

Run the proof from Host A using an unprinted generated relay token:

```bash
export RELAY_URL="http://100.82.111.69:38685"
export QSHIELD_BIN="/srv/qbuild/cache/targets/qsl-protocol/debug/qshield"
export QSHIELD_ACTOR="/srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor"
export REMOTE_QSHIELD="/home/qslcodex/qsl-na0263/bin/qshield"
export REMOTE_ACTOR="/home/qslcodex/qsl-na0263/bin/refimpl_actor"
export QSHIELD_RELAY_TOKEN="<generated but not printed>"

"$QSHIELD_BIN" relay serve \
  --listen "100.82.111.69:38685" \
  --allow-public \
  --i-understand-this-is-unsafe
```

The relay bind uses the existing Tailscale interface only. The CLI flag is the
demo's explicit non-loopback acknowledgement; it is not a public exposure
approval.

Alice ran on Host A:

```bash
"$QSHIELD_BIN" init --store "$ALICE_STORE" --relay-url "$RELAY_URL"
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" "$QSHIELD_BIN" register --store "$ALICE_STORE" --id alice
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$QSHIELD_ACTOR" \
  "$QSHIELD_BIN" establish --store "$ALICE_STORE" --peer bob \
  --demo-unauthenticated-override --demo-identity-verified
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$QSHIELD_ACTOR" \
  "$QSHIELD_BIN" send --store "$ALICE_STORE" --peer bob \
  --text "hello-na0263-two-host-demo" --demo-unauthenticated-override
```

Bob ran on Host B through `ssh remote`:

```bash
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$REMOTE_ACTOR" \
  "$REMOTE_QSHIELD" init --store /home/qslcodex/qsl-na0263/bob-store --relay-url "$RELAY_URL"
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$REMOTE_ACTOR" \
  "$REMOTE_QSHIELD" register --store /home/qslcodex/qsl-na0263/bob-store --id bob
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$REMOTE_ACTOR" \
  "$REMOTE_QSHIELD" establish --store /home/qslcodex/qsl-na0263/bob-store --peer alice \
  --demo-unauthenticated-override --demo-identity-verified
QSHIELD_RELAY_TOKEN="$QSHIELD_RELAY_TOKEN" QSHIELD_ACTOR="$REMOTE_ACTOR" \
  "$REMOTE_QSHIELD" recv --store /home/qslcodex/qsl-na0263/bob-store \
  --demo-unauthenticated-override
```

## Positive Proof

Host B received and decrypted the Host A message over the Tailscale-bound relay:

```text
from alice: hello-na0263-two-host-demo
```

Stable marker:

```text
NA0263_TWO_HOST_POSITIVE_OK
```

## Negative Proof

Host B also drove the selected negative paths against the Host A relay over
Tailscale:

- missing-auth register rejected with `401`;
- malformed JSON rejected with `400`;
- invalid relay id rejected with `400`;
- duplicate establish record rejected with `409`;
- tampered attachment ciphertext rejected with `attachment_integrity_reject`;
- tampered attachment reject wrote no output file.

Stable markers:

```text
NA0263_TWO_HOST_AUTH_REJECT_OK
NA0263_TWO_HOST_MALFORMED_REJECT_OK
NA0263_TWO_HOST_RELAY_ID_REJECT_OK
NA0263_TWO_HOST_REPLAY_REJECT_OK
NA0263_TWO_HOST_ATTACHMENT_INTEGRITY_REJECT_OK
```

KT-negative proof was recorded as unsupported for this remote run:

```text
UNSUPPORTED_KT_REJECT_REMOTE_CARGO_ABSENT
```

Reason: the current `qshield` CLI has no standalone KT-negative command, and
installing Rust/Cargo or a build toolchain on the remote host was out of scope
because local build plus `scp` worked.

## Leak and Panic Proof

The proof generated a relay token at runtime and did not print it. It injected
a secret sentinel into rejected remote inputs and verified sanitized response
bodies. The transcript scan completed before success markers were emitted.

Stable markers:

```text
NA0263_TWO_HOST_NO_SECRET_LEAK_OK
NA0263_TWO_HOST_NO_PANIC_OK
NA0263_CROSS_HOST_OR_PRIVATE_NETWORK_STRESS_OK
```

## Cleanup

The local relay process was stopped after proof. A local listener check on port
`38685` showed no remaining listener. A remote process check found no
unexpected `qshield` or `refimpl_actor` process under the NA-0263 work
directory.

Remote cleanup checks:

- `ssh remote 'ss -ltnp 2>/dev/null || true'` showed only pre-existing system
  listeners, not a qshield/refimpl demo listener.
- `ssh remote 'find /home/qslcodex/qsl-na0263 -maxdepth 3 -type f \( -name "*token*" -o -name "*secret*" \) 2>/dev/null || true'`
  returned no token/secret-named file.

The copied remote binaries and stores were left under
`/home/qslcodex/qsl-na0263/` for reproducibility. They are not repository
artifacts and must not be treated as production deployment material.

## Limitations

- This is a bounded non-production qshield demo proof only.
- The relay is the demo in-memory qshield relay, not qsl-server.
- Attachment proof is limited to the qshield demo descriptor/ciphertext path,
  not qsl-attachments production service hardening.
- KT-negative proof remains local demo-smoke evidence until a remote
  standalone KT command or prebuilt KT proof binary exists.
- The proof does not test public internet exposure and must not be used to
  justify it.
- The proof does not change protocol, crypto, auth, wire, or state-machine
  semantics.

## Related Evidence

- [Demo adversarial stress testing](DEMO_ADVERSARIAL_STRESS_TESTING.md)
- [Cross-host public demo reproducibility](CROSS_HOST_PUBLIC_DEMO_REPRODUCIBILITY.md)
- [NA-0263 audit](../governance/evidence/NA-0263_cross_host_demo_stress_reproducibility_audit.md)
- [NA-0263 testplan](../../tests/NA-0263_cross_host_demo_stress_reproducibility_testplan.md)
