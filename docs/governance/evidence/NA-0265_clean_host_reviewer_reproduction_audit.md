Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0265 Clean-Host Reviewer Reproduction Audit

Directive: QSL-DIR-2026-05-11-065 / NA-0265

## Objective

Prove that a reviewer can reproduce the public demo evidence from clean
instructions on a fresh workdir and exact commit, with transcripts, artifacts,
prerequisites, no-secret-leak proof, and explicit non-production boundaries.

## Starting Authority Proof

- Starting `origin/main`: `1e7d0a63be31`.
- PR #782: merged as `1e7d0a63be31`.
- PRs #781 through #761 and PR #708: verified merged.
- PR #750 and PR #722: verified closed and unmerged.
- Branch protection required the expected contexts, including `public-safety`;
  force pushes and deletions were disabled; admin enforcement was enabled.
- Latest starting-main `public-safety`: success on `1e7d0a63be31`.
- Queue proof: `READY_COUNT 1`, sole READY `NA-0265`.
- Decision proof before patching: latest D-0499, D-0500 absent, duplicate
  decision count zero.
- NA-0262A cost-control behavior: docs/demo paths classified docs-only;
  `scripts/ci` paths classified workflow-security/full-suite-required; empty
  path classification remained runtime-critical.

## Proof Mode Selection

Selected counted proof:

```text
Mode 1 - clean local source reproduction on the build host
```

Reason:

- The build host had the needed Rust toolchain, Git, Python, Bash, curl, and
  disk capacity.
- A fresh clone outside the active worktree could build and run the public demo
  evidence with a clean `CARGO_TARGET_DIR`.
- Cargo registry and git caches were reused and are recorded as a limitation.

Remote status:

- The trusted `remote` endpoint was preflighted successfully.
- Remote thin-client proof was attempted but not counted because command-shape
  errors in the SSH wrapper stopped the proof before relay/client traffic ran.
- The remote was not used as a build worker, no packages were installed, and
  the dedicated remote directory was cleaned up.

## Clean Source Reproduction

Artifact directory:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/
```

Fresh clone:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/clean-source/qsl-protocol
```

Commit:

```text
1e7d0a63be3157bf561d58ef67f3c68e19f04091
```

Clean target directory:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/cargo-target
```

Toolchain and host versions recorded in the transcript:

```text
cargo 1.95.0
rustc 1.95.0
node v24.15.0
npm 11.12.1
cargo-audit 0.22.1
```

Cache limitation:

- `CARGO_TARGET_DIR` was clean and scoped under the artifact directory.
- Cargo registry cache was reused.
- Cargo git cache was reused.

Commands run from the clean clone:

```bash
cargo build -p qshield-cli --locked
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline \
  DEMO_STRESS_ARTIFACT_DIR=/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/demo_adversarial_stress \
  scripts/ci/demo_adversarial_stress.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Transcript:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/clean_source_transcript.log
```

Clean-source markers:

```text
NA0265_REVIEWER_POSITIVE_DEMO_OK
NA0265_REVIEWER_NEGATIVE_REJECT_OK
NA0265_REVIEWER_NO_SECRET_LEAK_OK
NA0265_REVIEWER_NON_PRODUCTION_BOUNDARY_OK
NA0265_CLEAN_SOURCE_REPRO_OK
NA0265_REVIEWER_REPRODUCTION_OK
```

Underlying positive/negative proof included:

- qshield demo initialization, loopback relay, authorized registration,
  explicit demo unauthenticated establish, send, receive, and decrypt;
- missing-auth reject;
- malformed-input reject;
- invalid relay-id reject;
- establish replay reject;
- attachment descriptor/fetch/decrypt positive proof;
- tampered attachment ciphertext reject with no output file;
- KT verifier negative and accepted-state no-mutation proof;
- metadata conformance negative smoke;
- qsc `send_commit` regression; and
- bounded SCKA plus Suite-2 negotiation model checks.

## Remote Preflight

Remote host:

```text
hostname: lawrence-Inspiron-3647
user: qslcodex
Tailscale IPv4: 100.99.234.5
sudo proof: SUDO_OK
```

Remote resources:

```text
CPUs: 4
memory: 15Gi total, 13Gi available
disk: 842G available on /home and /tmp
tools: bash, sh, tar, ldd, ss, curl
```

Remote attempted work directory:

```text
/home/qslcodex/qsl-na0265-reviewer/
```

Local binary checksums for the files copied during the second remote attempt:

```text
qshield sha256 f94faa090ad6009875e72180b979399cb725a756316ab32a3d15618040e9def4
refimpl_actor sha256 b11e4b67c2c20a0feb5349cdb8851261665b60412f1b40302c340fcbbbcd1e9f
```

Remote result:

- No remote build work.
- No remote package installation.
- No remote Rust, Node, npm, fuzzing, or full-suite work.
- Remote thin-client positive/negative proof did not complete and
  `NA0265_REMOTE_THIN_CLIENT_REPRO_OK` was not emitted.
- Cleanup command removed `/home/qslcodex/qsl-na0265-reviewer/` and returned
  `REMOTE_CLEANUP_OK`.

Remote transcript:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/remote_reviewer_transcript.log
```

## Artifact Manifest

Manifest:

```text
/srv/qbuild/tmp/NA-0265_reviewer_reproduction_20260511T133410Z/ARTIFACT_MANIFEST.txt
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
- `remote_reviewer_transcript.log`
- `remote_identity_binary.log`
- `local_binary_checksums.log`
- `remote_cleanup.log`

## No-Leak Proof

The counted clean-source proof emitted:

```text
DEMO_NO_SECRET_LEAK_OK
DEMO_STRESS_NO_SECRET_LEAK_OK
NA0265_REVIEWER_NO_SECRET_LEAK_OK
```

The demo scripts check the generated relay token and secret sentinels against
operator-visible output and artifact files. The NA-0265 runbook does not embed
tokens, auth headers, passphrases, or long secret-like values.

## Limitations

- Cargo registry and git caches were reused.
- Remote thin-client proof was preflighted but not achieved.
- qsc desktop package/screenshot proof is cited from NA-0264 and was not rerun
  as part of the NA-0265 minimum proof.
- This is not production relay, qsl-server, qsl-attachments, KT deployment,
  desktop release, public internet, or production-hardening proof.

## Recommendations

- Keep Mode 1 as the minimum external reviewer command path.
- Treat remote copied-binary proof as an optional add-on only when the remote
  command wrapper is already proven.
- Move repeated-run/soak stability into the successor lane instead of expanding
  NA-0265.
