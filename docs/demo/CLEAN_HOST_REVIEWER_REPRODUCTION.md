Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# Clean-Host Reviewer Reproduction

This runbook records the NA-0265 reviewer reproduction procedure for the
non-production public demo evidence. It is intended to let a reviewer reproduce
the current demo proof from a fresh workdir and exact prerequisites.

This is not a production-readiness claim. It does not prove a production relay,
production desktop app, production KT deployment, production qsl-server, or
production qsl-attachments service.

## Supported Proof Modes

### Mode 1: Clean Local Source Reproduction

This is the counted NA-0265 proof mode.

The reviewer starts from a fresh clone at a named commit, uses a clean
`CARGO_TARGET_DIR`, and runs the minimum demo evidence commands. Cargo registry
and git caches may be reused; that limitation must be recorded.

### Mode 2: Remote Thin-Client Reproduction

This mode is allowed only when a trusted private-network remote endpoint is
available. The remote must be a thin client:

- no `cargo build`;
- no `cargo test`;
- no `npm install` or `npm build`;
- no Rust, Node, or npm toolchain installation;
- no fuzzing or full-suite tests;
- no broad package installation;
- no public internet exposure;
- no firewall, router, or Tailscale admin mutation.

If locally built binaries are copied to the remote, the result must be labeled
as copied-binary thin-client proof, not source-build reproduction.

### Mode 3: Prerequisite Stop

If neither Mode 1 nor Mode 2 can complete truthfully, stop and report the exact
missing prerequisite. Do not emit NA-0265 success markers for an unproven mode.

## Exact Prerequisites

Minimum local clean-source prerequisites:

- Linux host with Bash, POSIX core tools, `curl`, `git`, `python3`, and
  `timeout`.
- Rust/cargo capable of building the locked workspace.
- Network access for the initial clone and any uncached locked dependencies.
- Sufficient disk for an isolated build target under `/srv/qbuild/tmp`.
- Optional Node/npm only for evidence inventory; the counted clean-source proof
  does not build the desktop package.
- Optional `cargo-audit` for dependency validation outside the minimum
  clean-source demo proof.

Observed NA-0265 local toolchain:

```text
cargo 1.95.0
rustc 1.95.0
node v24.15.0
npm 11.12.1
cargo-audit 0.22.1
```

## Clean Source Commands

Set a timestamped artifact directory and clone the exact commit:

```bash
BASE="/srv/qbuild/tmp/NA-0265_reviewer_reproduction_<timestamp>"
mkdir -p "$BASE/clean-source" "$BASE/cargo-target"
git clone https://github.com/QuantumShieldLabs/qsl-protocol.git "$BASE/clean-source/qsl-protocol"
cd "$BASE/clean-source/qsl-protocol"
git checkout 1e7d0a63be3157bf561d58ef67f3c68e19f04091
export CARGO_TARGET_DIR="$BASE/cargo-target"
```

Record versions:

```bash
git rev-parse HEAD
cargo --version
rustc --version
node --version || true
npm --version || true
cargo audit --version || true
```

Run the reviewer command set:

```bash
cargo build -p qshield-cli --locked
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline \
  DEMO_STRESS_ARTIFACT_DIR="$BASE/demo_adversarial_stress" \
  scripts/ci/demo_adversarial_stress.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

If `formal/run_model_checks.py` is absent, record that absence instead of
claiming model-check proof.

## Expected Markers

The counted NA-0265 clean-source proof must emit:

```text
NA0265_CLEAN_SOURCE_REPRO_OK
NA0265_REVIEWER_POSITIVE_DEMO_OK
NA0265_REVIEWER_NEGATIVE_REJECT_OK
NA0265_REVIEWER_NO_SECRET_LEAK_OK
NA0265_REVIEWER_NON_PRODUCTION_BOUNDARY_OK
NA0265_REVIEWER_REPRODUCTION_OK
```

The underlying demo smoke and stress commands should also emit:

```text
DEMO_ACCEPTANCE_OK
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NO_SECRET_LEAK_OK
NA0262_DEMO_ADVERSARIAL_STRESS_OK
metadata-conformance-smoke: OK
```

If remote thin-client proof is completed, it may additionally emit:

```text
NA0265_REMOTE_THIN_CLIENT_REPRO_OK
```

Do not emit the remote marker for a copied-binary setup that did not run a
remote positive/negative client flow.

## NA-0265 Artifact Paths

Counted NA-0265 clean-source artifact directory:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/
```

Important files:

- `clean_source_transcript.log`
- `clean_source_markers.log`
- `demo_cli_smoke_clean.log`
- `metadata_conformance_clean.log`
- `send_commit_clean.log`
- `formal_model_clean.log`
- `demo_adversarial_stress/demo_adversarial_stress_transcript.log`
- `demo_adversarial_stress/demo_adversarial_stress_markers.log`
- `ARTIFACT_MANIFEST.txt`

Remote thin-client transcript path, if attempted:

- `remote_reviewer_transcript.log`

For NA-0265, the remote transcript is not counted as a completed thin-client
proof because no remote positive/negative client flow completed.

## What This Proves

- A reviewer can clone the repository at the exact NA-0265 base commit.
- The qshield demo binary can be built from the clean source workdir.
- The non-production demo positive send/receive/decrypt path passes.
- Missing auth, malformed input, replay, invalid relay id, attachment integrity,
  and KT-negative proof paths reject as expected.
- The metadata smoke and qsc `send_commit` regression pass.
- The bounded formal/model checks pass when the model runner is present.
- The transcript and artifact manifest identify what ran.
- Demo and remote boundaries remain explicitly non-production.

## What This Does Not Prove

- Production readiness.
- Production relay readiness.
- Production desktop readiness.
- Production KT deployment or live qshield KT evidence ingestion.
- Production qsl-server or qsl-attachments hardening.
- Public internet exposure safety.
- A fully cold dependency fetch without Cargo registry/git cache reuse.
- Remote source-build reproduction.

## Remote Thin-Client Limitations

NA-0265 remote preflight verified the trusted `remote` endpoint:

```text
hostname: lawrence-Inspiron-3647
user: qslcodex
Tailscale IPv4: 100.99.234.5
CPUs: 4
memory: 15Gi total
disk: 842G available on /home and /tmp
sudo proof: SUDO_OK
```

The NA-0265 remote proof did not complete. The failed attempts were command
shape errors in the SSH wrapper before any remote relay/client traffic proof was
counted. Locally built binaries were copied during the second attempt, then the
dedicated remote directory was removed. The remote was not used as a build
worker and no packages were installed.

Use the remote mode only when the exact remote command wrapper is known-good;
otherwise rely on Mode 1 or stop with prerequisites.

## Troubleshooting

- If `git checkout` cannot find the commit, fetch from the canonical GitHub
  origin and verify the requested SHA.
- If Cargo fails before compiling project crates, record whether network access
  or registry cache is missing.
- If the demo smoke leaks the relay token or a sentinel, stop immediately and do
  not emit NA-0265 markers.
- If `metadata_conformance_smoke.sh` fails on permissions, verify the host
  supports Unix file modes and rerun from a normal local filesystem.
- If the model runner is missing, record that limitation; do not claim formal
  model proof.
- If the remote host would require Rust/Node/npm installation, package upgrades,
  or use as a build worker, do not run remote mode.
