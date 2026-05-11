Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0263 Cross-Host Demo Stress Reproducibility Audit

Directive: QSL-DIR-2026-05-10-062 / NA-0263

## Objective

Reproduce the bounded qshield demo stress proof across a real private-network
endpoint using the already trusted SSH alias `remote`, while preserving
non-production posture, auth, fail-closed reject behavior, no-secret-leak
evidence, no public internet exposure, no Tailscale/admin/firewall mutation,
and no protocol/crypto state-machine changes.

## Starting Authority Proof

- Starting `origin/main`: `d72d4da21dda`.
- PR #778: merged as `d72d4da21dda`.
- PRs #777 through #761 and PR #708: merged.
- PR #750 and PR #722: closed and unmerged.
- Branch protection: expected protected contexts present, including
  `public-safety`; force pushes and deletions disabled; admin enforcement
  enabled.
- Latest starting-main `public-safety`: success.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0263`.
- Decision proof before edits: D-0495 existed once; D-0496 and D-0497 absent;
  duplicate decision count zero.
- NA-0262A full-suite cost-control self-test passed after the local worktree
  was fast-forwarded to `origin/main`.

## Remote Preflight

- SSH alias: `remote`.
- Hostname: `lawrence-Inspiron-3647`.
- User: `qslcodex`.
- Groups included `sudo`.
- Remote Tailscale IP: `100.99.234.5`.
- Local Tailscale IP: `100.82.111.69`.
- Sudo proof: `SUDO_OK`.
- Platform: Ubuntu 24.04.4 LTS, Linux `6.17.0-23-generic`, x86_64.
- Required basic tools present: `/usr/bin/bash`, `/usr/bin/sh`,
  `/usr/bin/tar`, `/usr/bin/curl`.
- Remote resource proof: `nproc` returned `4`; `free -h` showed `15Gi` total
  memory and `13Gi` available; `/home` and `/tmp` were on `/dev/sda1` with
  `916G` size, `842G` available, and `4%` used.
- Remote light-tool proof: `/usr/bin/bash`, `/usr/bin/sh`, `/usr/bin/tar`,
  `/usr/bin/ldd`, and `/usr/bin/ss`.
- Remote work directory prepared:
  `/home/qslcodex/qsl-na0263/`, mode `0700`.
- Bidirectional Tailscale ping succeeded between `100.82.111.69` and
  `100.99.234.5`.
- Selected proof mode: real two-host Tailscale client/relay proof.

No SSH host-key bypass, public exposure, firewall/router change, or Tailscale
admin/API mutation was performed.

The remote host was treated as a thin private-network client endpoint only.
No remote build, test, fuzzing, full-suite, Node/npm, Rust toolchain, broad
package install, package upgrade, or large parallel job was run.

## Remote Endpoint Preparation

Local build command:

```bash
cargo build -p qshield-cli -p refimpl_actor --locked
```

Copied remote binaries:

- `/home/qslcodex/qsl-na0263/bin/qshield`
- `/home/qslcodex/qsl-na0263/bin/refimpl_actor`

Remote execution proof:

```text
QuantumShield demo CLI (non-production)
Usage: qshield <COMMAND>
```

Runtime dependency proof:

```text
libgcc_s.so.1
libc.so.6
ld-linux-x86-64.so.2
```

No remote package installation was required.

Copied binary size/checksum proof:

```text
qshield: 46M, sha256 9df7b1dbb29cd43aa60cb51bc0b1a7521a8b446f621eb5b061ab3bf07694072a
refimpl_actor: 32M, sha256 6e5bb00cb8cad51cbd9487b02cef7acfb8ce3af50061548126251f1a068accd1
```

## Real Two-Host Proof

Artifact directory:

```text
/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_20260511T025100Z/
```

Artifact files:

- `local_transcript.log`
- `remote_transcript.log`
- `remote_identity_recheck.log`
- `remote_sudo_recheck.log`
- `remote_resource_preflight.log`
- `remote_light_tool_preflight.log`
- `remote_binary_checksums.log`
- `remote_listener_cleanup.log`
- `remote_token_secret_file_cleanup.log`
- `markers.log`
- `ARTIFACT_MANIFEST.txt`

Relay bind:

```text
100.82.111.69:38685
```

Proof markers emitted:

```text
NA0263_PROOF_MODE=real-two-host-tailscale
NA0263_REMOTE_HOST_OK
NA0263_REMOTE_BINARY_OK
NA0263_RELAY_TAILSCALE_BIND_OK
NA0263_REMOTE_TO_RELAY_CONNECT_OK
NA0263_TWO_HOST_POSITIVE_OK
NA0263_TWO_HOST_ATTACHMENT_INTEGRITY_REJECT_OK
NA0263_TWO_HOST_AUTH_REJECT_OK
NA0263_TWO_HOST_MALFORMED_REJECT_OK
NA0263_TWO_HOST_REPLAY_REJECT_OK
NA0263_TWO_HOST_RELAY_ID_REJECT_OK
NA0263_TWO_HOST_NO_SECRET_LEAK_OK
NA0263_TWO_HOST_NO_PANIC_OK
NA0263_CROSS_HOST_OR_PRIVATE_NETWORK_STRESS_OK
```

Unsupported marker:

```text
UNSUPPORTED_KT_REJECT_REMOTE_CARGO_ABSENT
```

## Positive Outcome

Host A ran the relay and Alice. Host B ran Bob. Bob received and decrypted:

```text
from alice: hello-na0263-two-host-demo
```

This is accepted as real two-host Tailscale client/relay proof because the
receive/decrypt action ran on `remote` against a relay bound to the build
server's local Tailscale interface. It is not production relay proof, full
distributed production stress, qsl-server proof, qsl-attachments proof, or
public internet stress proof.

## Negative Outcomes

The remote host drove these fail-closed checks against the Host A relay:

- missing auth rejected with `401`;
- malformed JSON rejected with `400`;
- invalid relay id rejected with `400`;
- duplicate establish record rejected with `409`;
- tampered attachment ciphertext rejected with `attachment_integrity_reject`;
- tampered attachment reject wrote no output file.

The proof did not fake KT-negative evidence. It records that this category is
unsupported for the remote run because the copied qshield binary has no
standalone KT command and remote toolchain installation was outside scope.

## No-Leak and No-Panic Outcome

The proof used an unprinted generated relay token and an injected secret
sentinel in rejected remote inputs. The transcript scan checked the local and
remote logs for the relay token, the sentinel, panic text, backtraces, and
unwrap-output text before success markers were emitted.

The positive plaintext was deliberately non-secret demo text. No token, secret
sentinel, panic, backtrace, or unwrap-output text was found in the transcript
logs.

## Cleanup Outcome

- The local relay was stopped.
- `ss -ltn 'sport = :38685'` showed no remaining listener.
- `ssh remote "pgrep -af '/home/qslcodex/qsl-na0263/bin/(qshield|refimpl_actor)' || true"`
  returned no unexpected process.
- `ssh remote 'ss -ltnp 2>/dev/null || true'` showed only pre-existing system
  listeners, not a qshield/refimpl demo listener.
- `ssh remote 'find /home/qslcodex/qsl-na0263 -maxdepth 3 -type f \( -name "*token*" -o -name "*secret*" \) 2>/dev/null || true'`
  returned no token/secret-named file.
- Copied remote binaries and stores remain under `/home/qslcodex/qsl-na0263/`
  as non-production reproducibility material.

## Scope Boundary

No source-code implementation changes were required. The evidence package
changes only docs/governance/testplan/journal/decision/traceability files.

No qsl-server, qsl-attachments, qsc-desktop implementation, website/external
website, `.github`, Cargo manifest/lockfile, public-safety configuration,
branch-protection setting, protocol-core, crypto, auth, wire, state-machine, or
production relay/service path was changed.

## Residual Gaps

- KT-negative remote proof needs a standalone prebuilt proof command or an
  explicitly authorized remote toolchain path.
- qsl-server and qsl-attachments production hardening remain separate.
- Desktop/sidecar stress remains separate.
- Metadata phase-2 and external cryptographic review readiness remain
  separate.
- Public internet relay safety remains unproven and out of scope.

## Recommendation

Merge the NA-0263 evidence PR only after local validation and required CI pass
normally. Keep NA-0263 READY after this proof PR until a separate closeout
directive promotes exactly one successor.
