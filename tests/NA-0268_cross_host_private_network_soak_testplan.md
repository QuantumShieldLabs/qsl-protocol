Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0268 Cross-Host Private-Network Soak Test Plan

## Scope

This test plan covers NA-0268 artifact-safe cross-host/private-network repeated
soak proof for the non-production qshield demo.

In scope:

- local build of qshield and refimpl actor;
- remote thin-client execution through SSH alias `remote`;
- local relay bind to the existing local Tailscale IP;
- three bounded positive remote receive checks;
- remote negative reject checks;
- runtime-root versus artifact-root separation;
- no-leak, no-panic, cleanup, and manifest proof.

Out of scope:

- qsl-server production hardening;
- qsl-attachments production hardening;
- qsc-desktop implementation;
- public internet exposure;
- firewall/router/Tailscale admin changes;
- remote builds or remote package installs;
- protocol/crypto state-machine changes;
- Cargo dependency changes.

## Prerequisites

- Worktree is clean and based on expected `origin/main`.
- `public-safety` is required and green before proof starts.
- Remote SSH alias `remote` resolves to `lawrence-Inspiron-3647`.
- Remote user is `qslcodex`.
- Remote Tailscale IP is `100.99.234.5` unless Tailscale has changed it.
- Remote has lightweight runtime tools already present: shell, `curl`, `tar`,
  `ldd`, and `ss`.

## Proof Command

The counted proof was run from the local qbuild worktree with a temporary
runner that kept runtime state out of artifacts:

```bash
/srv/qbuild/tmp/na0268_cross_host_soak_runner_safe.sh
```

The runner command shape is recorded in:

```text
/srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/COMMANDS_REDACTED.md
```

## Required Markers

The retained artifact marker file must include:

```text
NA0268_CROSS_HOST_SOAK_START
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

## Artifact Isolation Checks

Run:

```bash
find /srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z \
  -type f \( -name 'config.json' -o -iname '*token*' -o -iname '*secret*' \
  -o -iname '*auth*' -o -iname '*bearer*' -o -iname '*key*' \)
```

Expected result:

```text
no output
```

Confirm scan reports:

```bash
sed -n '1,80p' /srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/leak_scan.txt
sed -n '1,80p' /srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/panic_scan.txt
sed -n '1,80p' /srv/qbuild/tmp/NA-0268_cross_host_private_network_soak_artifacts_20260512T032008Z/forbidden_filename_scan.txt
```

Expected result:

- no generated token, sentinel, or plaintext payload values under artifact root;
- no panic/backtrace/unwrap markers under artifact root;
- no config/token/secret/auth/bearer/key filenames under artifact root.

## Cleanup Checks

Run:

```bash
test ! -e /srv/qbuild/tmp/NA-0268_runtime_20260512T032008Z
ssh remote 'test ! -e /home/qslcodex/qsl-na0268-soak'
```

Expected result:

```text
both commands return 0
```

The retained `cleanup_proof.txt` must also show:

```text
remote_cleanup=ok
runtime_root_cleanup=ok
```

## Validation Bundle

Run after the tracked evidence patch:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 formal/run_model_checks.py
```

Required result:

- queue remains `READY_COUNT 1`, READY `NA-0268`;
- D-0506 exists once;
- D-0507 is absent;
- no duplicate decision IDs;
- no forbidden paths are touched;
- all local validation commands pass;
- artifact root remains secret-free and panic-free.
