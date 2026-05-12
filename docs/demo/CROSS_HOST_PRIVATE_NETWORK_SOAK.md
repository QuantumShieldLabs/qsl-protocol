Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# Cross-Host Private-Network Soak

## Purpose

NA-0268 extends the bounded demo soak evidence into a real two-host private
network setting. The proof repeats the non-production `qshield` demo flow
across the existing Tailscale private network and keeps token-bearing runtime
state out of the retained artifact package.

This is not production-hardening evidence. It does not prove public internet
exposure safety, qsl-server production relay readiness, qsl-attachments
production service readiness, desktop release readiness, KT deployment
readiness, or release approval.

## Proof Mode

Proof mode achieved:

```text
real-two-host-tailscale-repeated-soak
```

Host roles:

- Host A: local qbuild Ubuntu host, local demo relay, Alice client.
- Host B: trusted SSH alias `remote`, Bob thin client.

Private addresses used:

- Host A Tailscale IP: `100.82.111.69`.
- Host B Tailscale IP: `100.99.234.5`.
- Relay binds used across the counted run set:
  - `100.82.111.69:34431`
  - `100.82.111.69:46239`
  - `100.82.111.69:57183`

No public internet target was used. No router, firewall, Tailscale admin/API,
VPN, DNS, branch-protection, or public-safety setting was changed. SSH host-key
verification was not bypassed.

The demo relay requires `--allow-public --i-understand-this-is-unsafe` for a
non-loopback bind. In this proof that bind is limited to the already configured
Tailscale address above and is not a public exposure approval.

## Runtime and Artifact Separation

The proof uses two separate roots:

```text
runtime root:  /srv/qbuild/tmp/NA-0268_runtime_20260512T032008Z/
artifact root: /srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/
```

The runtime root held local qshield stores, relay logs, generated relay tokens,
and per-run temporary state. The remote runtime directory was:

```text
/home/qslcodex/qsl-na0268-soak/
```

The retained artifact root contains only redacted transcripts, marker files,
summary tables, cleanup proof, scan reports, a manifest, and binary
checksum/size proof. Runtime stores were not copied into the artifact root.
`config.json` files with relay-token material were kept only in runtime state
and were deleted before success.

## Counted Run Set

Run count:

```text
3
```

Each counted run used a distinct local Alice store and a distinct remote Bob
store:

```text
run 1: local runtime run_01, remote runtime run_01
run 2: local runtime run_02, remote runtime run_02
run 3: local runtime run_03, remote runtime run_03
```

The proof emitted:

```text
NA0268_RUN_1_REMOTE_OK
NA0268_RUN_2_REMOTE_OK
NA0268_RUN_3_REMOTE_OK
NA0268_REMOTE_POSITIVE_OK
NA0268_REMOTE_NEGATIVE_REJECT_OK
NA0268_NO_STATE_BLEED_OK
NA0268_NO_SECRET_LEAK_OK
NA0268_NO_PANIC_OK
NA0268_ARTIFACT_MANIFEST_OK
NA0268_CROSS_HOST_PRIVATE_NETWORK_SOAK_OK
```

## Exact Command Shape

Local build:

```bash
cargo build -p qshield-cli -p refimpl_actor --locked
```

Remote preparation:

```bash
ssh remote 'rm -rf /home/qslcodex/qsl-na0268-soak && mkdir -p /home/qslcodex/qsl-na0268-soak/bin'
scp /srv/qbuild/cache/targets/qsl-protocol/debug/qshield \
  /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor \
  remote:/home/qslcodex/qsl-na0268-soak/bin/
ssh remote 'chmod 700 /home/qslcodex/qsl-na0268-soak/bin/qshield /home/qslcodex/qsl-na0268-soak/bin/refimpl_actor'
```

Per counted run, the local host generated an unprinted relay token and an
unprinted demo payload, started the relay on the local Tailscale address, and
kept local store logs under the runtime root:

```bash
QSHIELD_RELAY_TOKEN="<generated but not printed>" \
  qshield relay serve \
  --listen "100.82.111.69:<run-port>" \
  --allow-public \
  --i-understand-this-is-unsafe

QSHIELD_RELAY_TOKEN="<generated but not printed>" \
QSHIELD_ACTOR="/srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor" \
  qshield init --store "<runtime-root>/run_<n>/alice-store" \
  --relay-url "http://100.82.111.69:<run-port>"

QSHIELD_RELAY_TOKEN="<generated but not printed>" \
QSHIELD_ACTOR="/srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor" \
  qshield register --store "<runtime-root>/run_<n>/alice-store" --id "alice<n>"

QSHIELD_RELAY_TOKEN="<generated but not printed>" \
QSHIELD_ACTOR="/srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor" \
  qshield establish --store "<runtime-root>/run_<n>/alice-store" \
  --peer "bob<n>" --demo-unauthenticated-override \
  --demo-identity-verified

QSHIELD_RELAY_TOKEN="<generated but not printed>" \
QSHIELD_ACTOR="/srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor" \
  qshield send --store "<runtime-root>/run_<n>/alice-store" \
  --peer "bob<n>" --text "<redacted demo payload>" \
  --demo-unauthenticated-override
```

The remote host sourced a runtime-only env file and ran Bob init, register,
establish, receive, and negative reject probes. Raw receive output was checked
on the remote side and only a redacted success line was copied to the retained
remote transcript.

Remote negative probes covered missing credentials, malformed JSON, and replayed
establish-record rejection. Rejected response bodies were checked for absence of
the generated relay token and sentinel before the run was accepted.

## Artifact Package

Artifact directory:

```text
/srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/
```

Important files:

- `local_transcript.log`
- `remote_transcript.log`
- `markers.log`
- `summary_matrix.tsv`
- `run_state.tsv`
- `leak_scan.txt`
- `panic_scan.txt`
- `forbidden_filename_scan.txt`
- `cleanup_proof.txt`
- `binary_checksums.log`
- `environment_summary.txt`
- `COMMANDS_REDACTED.md`
- `ARTIFACT_MANIFEST.txt`

The artifact manifest lists only non-secret proof files. It does not include
qshield store directories, relay token files, raw remote runtime files, or
store `config.json` files.

## What Is Proven

- Remote host `remote` was reachable over the existing private Tailscale
  network.
- The remote copied `qshield` binary executed without a remote build.
- The local demo relay bound to the local Tailscale IP.
- The remote host reached the Tailscale-bound relay in three bounded runs.
- Each run completed a positive remote receive/decrypt check with plaintext
  redacted from retained artifacts.
- Each run completed remote negative rejects for missing credentials, malformed
  JSON, and establish replay.
- Each run used clean local and remote runtime stores.
- The retained artifact root contains no qshield store config, no forbidden
  token/secret/auth/bearer/key filenames, no generated token, no generated
  sentinel, no plaintext payload, and no panic/backtrace markers.
- Local and remote runtime roots were cleaned before success.

## What Is Not Proven

- Production readiness.
- Public internet exposure safety.
- Production qsl-server relay readiness.
- Production qsl-attachments service readiness.
- Production desktop package or release readiness.
- Production KT service operation.
- Firewall, router, DNS, or Tailscale control-plane behavior.
- Remote build reproducibility.
- Unbounded soak behavior.

## Known Gaps

- This remains a non-production `qshield` demo proof.
- The remote host is a trusted private endpoint, not an untrusted public client.
- The proof does not exercise qsl-server or qsl-attachments production
  deployments.
- The proof does not replace external cryptographic review, metadata phase-2
  work, or production-boundary hardening plans.
