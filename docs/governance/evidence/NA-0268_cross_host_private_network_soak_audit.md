Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0268 Cross-Host Private-Network Soak Audit

## Result

NA-0268 completed an artifact-safe real two-host private-network repeated soak
for the non-production `qshield` demo.

Proof mode:

```text
real-two-host-tailscale-repeated-soak
```

Counted artifact directory:

```text
/srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/
```

Runtime root:

```text
/srv/qbuild/tmp/NA-0268_runtime_20260512T032008Z/
```

Remote runtime directory:

```text
/home/qslcodex/qsl-na0268-soak/
```

Both runtime locations were cleaned before success.

## Prior Stop Root Cause

The prior stopped proof correctly failed because token-bearing runtime state was
stored under the retained artifact tree:

```text
/srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T025638Z/run_01/alice-store/config.json
```

That `config.json` belonged to the local qshield store and carried the generated
relay token. The failure was a real artifact hygiene failure, not a false
positive. The no-leak gate was preserved and the recovery changed the proof
layout instead of weakening leak detection.

## Corrected Layout

The corrected proof keeps secret-bearing runtime state outside retained
artifacts:

- local qshield stores, relay logs, generated relay token, generated plaintext,
  and per-run runtime logs stayed under the runtime root;
- remote Bob stores, remote env files, raw receive output, and rejected response
  bodies stayed under `/home/qslcodex/qsl-na0268-soak/`;
- the retained artifact root received only redacted local/remote transcripts,
  marker files, matrices, cleanup proof, scan reports, binary proof, command
  shape, environment summary, and manifest;
- the artifact root was scanned for forbidden filenames and generated secret
  values before success markers were emitted.

The runtime root is not inside the artifact root.

## Counted Proof Markers

The counted proof emitted:

```text
NA0268_CROSS_HOST_SOAK_START
NA0268_REMOTE_BINARY_OK
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

Run matrix:

```text
run 1: remote positive ok, remote negative ok, clean stores
run 2: remote positive ok, remote negative ok, clean stores
run 3: remote positive ok, remote negative ok, clean stores
```

## Leak Scan

Artifact-safe checks passed:

- no `config.json` under the artifact root;
- no artifact filenames matching token/secret/auth/bearer/key patterns;
- generated relay token absent from retained artifacts;
- generated sentinel absent from retained artifacts;
- generated plaintext payload absent from retained artifacts;
- known demo sentinel strings absent from retained artifacts.

Scan report:

```text
leak_scan.txt: no generated token, sentinel, or plaintext payload values under artifact root
forbidden_filename_scan.txt: no config/token/secret/auth/bearer/key filenames under artifact root
```

Full checksum values are retained in `binary_checksums.log`; this audit does
not paste long hex values.

## Panic Scan

The artifact panic scan passed:

```text
panic_scan.txt: no panic/backtrace/unwrap markers under artifact root
```

No panic, backtrace, or unwrap-panic marker was found in the retained local or
remote transcripts.

## Cleanup Proof

Cleanup proof:

```text
remote_cleanup=ok
runtime_root_cleanup=ok
```

Independent checks after proof:

```text
LOCAL_RUNTIME_ROOT_ABSENT
REMOTE_RUNTIME_ROOT_ABSENT
```

The local runtime root and remote workdir are not retained as proof artifacts.

## Recoveries During This Directive

Two temporary proof-runner issues were recovered without weakening checks:

- Failing command: `/srv/qbuild/tmp/na0268_cross_host_soak_runner_safe.sh`.
  Classification: recoverable command-shape issue. The remote env file was
  sourced without exporting variables, so remote `qshield register` did not see
  `QSHIELD_RELAY_TOKEN`. Corrective action: export sourced remote env values and
  clean failed runtime state. Final result: next runner attempt reached all
  three remote cycles.
- Failing command: `/srv/qbuild/tmp/na0268_cross_host_soak_runner_safe.sh`.
  Classification: recoverable proof-runner scratch-path issue. The artifact
  filename scan tried to write a temporary hit list inside the runtime root
  after cleanup. Corrective action: move scan scratch to `/tmp` and rerun from a
  fresh runtime/artifact timestamp. Final result: three-run proof completed with
  all success markers and clean scans.

Neither recovered failure emitted final NA-0268 success markers. Failed runtime
state was cleaned before the counted proof.

## Boundary Preservation

This lane did not change:

- protocol, wire, crypto, auth, or state-machine semantics;
- qsp protocol-core;
- qsl-server production behavior;
- qsl-attachments production behavior;
- qsc-desktop implementation;
- website or external website code;
- workflows, branch protection, public-safety configuration, Cargo manifests,
  or Cargo lockfile.

The proof does not claim production readiness. It is bounded non-production demo
evidence only.

## Limitations

- The remote host is a trusted thin client endpoint, not a production client.
- The relay is the non-production qshield demo relay, not qsl-server.
- Attachment service production hardening is not exercised.
- Public internet exposure is not tested or authorized.
- No firewall/router/Tailscale admin behavior is validated.
- Remote builds, remote package installs, fuzzing, and unbounded soak are out of
  scope.
