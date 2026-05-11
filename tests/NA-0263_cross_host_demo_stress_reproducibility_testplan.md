# NA-0263 Cross-Host Demo Stress Reproducibility Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Validate real two-host private-network qshield demo stress proof using the
trusted SSH alias `remote`, without public internet exposure, firewall/router
changes, Tailscale admin/API mutation, SSH host-key bypass, protocol/crypto
state-machine changes, production hardening claims, or qsl-server /
qsl-attachments production changes.

## Protected Invariants

- Proof mode is truthfully labeled.
- Remote host identity is verified before use.
- Remote command execution uses strict SSH host-key behavior.
- Relay binds only to the local Tailscale IP and bounded high port.
- Remote reaches the relay only over the private Tailscale address.
- Auth remains required for relay mutation paths.
- Malformed, replay, invalid-id, and attachment-integrity rejects fail closed.
- Tokens and injected secret sentinels do not leak to transcript output.
- Demo commands do not emit panic, backtrace, or unwrap-output text.
- The demo remains explicitly non-production.
- No production hardening is claimed from this proof.

## Allowed Scope

- `docs/demo/**`
- `docs/governance/evidence/NA-0263_cross_host_demo_stress_reproducibility_audit.md`
- `tests/NA-0263_cross_host_demo_stress_reproducibility_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `scripts/ci/demo_adversarial_stress.sh` only if parameterization is needed
- `apps/qshield-cli/**` only for minimal test-backed demo CLI hardening

## Forbidden Scope

- `.github/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- qsc/qsl protocol-core or crypto state-machine files
- `qsl-server/**`
- `qsl-attachments/**`
- qsc-desktop implementation
- `website/**`
- external website source
- production relay/service implementation
- branch-protection settings
- public-safety/check configuration

## Preflight Proof

Run:

```bash
ssh remote 'hostname; whoami; id; tailscale ip -4'
ssh remote 'nproc; free -h; df -h /home /tmp 2>/dev/null || df -h'
ssh remote 'sudo -n true && echo SUDO_OK'
ssh remote 'uname -a; cat /etc/os-release || true; command -v bash; command -v sh; command -v tar || true; command -v curl || true; command -v git || true; command -v cargo || true; command -v node || true'
ssh remote 'command -v bash; command -v sh; command -v tar || true; command -v ldd || true; command -v ss || true'
ssh remote 'mkdir -p /home/qslcodex/qsl-na0263 && chmod 700 /home/qslcodex/qsl-na0263 && ls -ld /home/qslcodex/qsl-na0263'
tailscale ip -4 || true
ssh remote 'tailscale ip -4'
ping -c 1 -W 2 100.99.234.5
ssh remote 'ping -c 1 -W 2 100.82.111.69'
```

Expected:

- hostname `lawrence-Inspiron-3647`;
- user `qslcodex`;
- sudo group present;
- remote Tailscale IP `100.99.234.5`;
- resource proof is captured for CPU, memory, and `/home` / `/tmp` disk;
- lightweight shell/runtime tools are present;
- `ss` is present for cleanup/listener proof;
- `SUDO_OK`;
- private reachability succeeds in both directions.

Remote must remain a thin client endpoint. Do not run remote build/test/fuzzing,
full-suite, Rust toolchain, Node/npm, broad package install, package upgrade, or
large parallel job commands for this test.

## Remote Binary Proof

Run:

```bash
cargo build -p qshield-cli -p refimpl_actor --locked
scp /srv/qbuild/cache/targets/qsl-protocol/debug/qshield \
  /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor \
  remote:/home/qslcodex/qsl-na0263/bin/
ssh remote '/home/qslcodex/qsl-na0263/bin/qshield --help'
ssh remote 'ldd /home/qslcodex/qsl-na0263/bin/qshield'
ssh remote 'ls -lh /home/qslcodex/qsl-na0263/bin && sha256sum /home/qslcodex/qsl-na0263/bin/*'
```

Expected:

- qshield help prints successfully on the remote host;
- copied binary has all runtime libraries available;
- copied file sizes and checksums are recorded;
- no remote package installation is needed unless a minimal runtime dependency
  is proven by command output or `ldd`.

## Real Two-Host Proof

Run the bounded proof with:

- relay on Host A bound to `100.82.111.69:38685`;
- Alice on Host A;
- Bob on Host B via `ssh remote`;
- relay token generated but not printed;
- transcript directory under
  `/srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_<timestamp>/`.

Expected required markers:

```text
NA0263_PROOF_MODE=real-two-host-tailscale
NA0263_REMOTE_HOST_OK
NA0263_REMOTE_BINARY_OK
NA0263_RELAY_TAILSCALE_BIND_OK
NA0263_REMOTE_TO_RELAY_CONNECT_OK
NA0263_TWO_HOST_POSITIVE_OK
NA0263_TWO_HOST_AUTH_REJECT_OK
NA0263_TWO_HOST_MALFORMED_REJECT_OK
NA0263_TWO_HOST_REPLAY_REJECT_OK
NA0263_TWO_HOST_RELAY_ID_REJECT_OK
NA0263_TWO_HOST_NO_SECRET_LEAK_OK
NA0263_TWO_HOST_NO_PANIC_OK
NA0263_CROSS_HOST_OR_PRIVATE_NETWORK_STRESS_OK
```

Expected supported optional marker:

```text
NA0263_TWO_HOST_ATTACHMENT_INTEGRITY_REJECT_OK
```

Expected unsupported declaration for the current remote setup:

```text
UNSUPPORTED_KT_REJECT_REMOTE_CARGO_ABSENT
```

## Positive Proof

Expected remote receive transcript:

```text
from alice: hello-na0263-two-host-demo
```

This must appear in the remote transcript, not only in a same-host local log.

## Negative Proof

Expected remote-driven negative outcomes:

- missing auth register: `401` or `403`;
- malformed register: `400` with sanitized `invalid json`;
- invalid relay id: `400` with sanitized `invalid id format`;
- replayed establish record: `409` with sanitized `establish replay`;
- tampered attachment receive: nonzero with `attachment_integrity_reject`;
- tampered attachment reject writes no output file.

## Leak / Panic Proof

Scan local and remote transcripts before declaring success:

```bash
grep -R -E "NA0263_SECRET_SENTINEL|thread '.*' panicked|panicked at|stack backtrace|RUST_BACKTRACE|called \`.*unwrap" \
  /srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_<timestamp>/*.log \
  /srv/qbuild/tmp/NA-0263_cross_host_demo_stress_artifacts_<timestamp>/ARTIFACT_MANIFEST.txt || true
```

Expected:

- no injected secret sentinel appears in transcripts;
- no relay token appears in transcripts;
- no panic/backtrace/unwrap-output text appears.

## Cleanup Proof

Run:

```bash
ssh remote 'pgrep -af "/home/qslcodex/qsl-na0263/bin/(qshield|refimpl_actor)" || true'
ssh remote 'ss -ltnp 2>/dev/null || true'
ssh remote 'find /home/qslcodex/qsl-na0263 -maxdepth 3 -type f \( -name "*token*" -o -name "*secret*" \) 2>/dev/null || true'
```

Expected:

- no unexpected copied-binary demo process remains;
- no qshield/refimpl demo listener remains;
- no token/secret-named file remains under the NA-0263 remote work directory.

## Validation Bundle

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
bash -n scripts/ci/demo_adversarial_stress.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  docs/demo/CROSS_HOST_DEMO_STRESS_REPRODUCIBILITY.md \
  docs/governance/evidence/NA-0263_cross_host_demo_stress_reproducibility_audit.md \
  tests/NA-0263_cross_host_demo_stress_reproducibility_testplan.md \
  DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Expected:

- `READY_COUNT 1`, READY `NA-0263`;
- D-0496 exists once;
- D-0497 absent;
- no duplicate decision IDs;
- no forbidden paths touched;
- baseline stress still passes;
- real two-host proof passes;
- link check passes;
- added-line leak scan reports zero secret findings;
- goal-lint passes.

## CI Expectations

- Required protected checks pass normally.
- `public-safety` remains required and green.
- This evidence PR is docs/governance/testplan/journal only, so NA-0262A
  docs-only cost control may skip full-suite waits as designed.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
