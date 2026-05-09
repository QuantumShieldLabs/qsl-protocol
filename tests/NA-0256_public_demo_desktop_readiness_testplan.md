Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0256 Public Demo and Desktop Readiness Test Plan

## Objective

Validate that NA-0256 packages public demo and desktop touch-and-feel readiness with executable proof, artifact references, truthful host limitations, and no production-readiness overclaim.

## Protected Invariants

- Demo and GUI remain non-production.
- No production-ready desktop claim.
- No hidden protocol mutation.
- No token or secret leakage.
- Demo positive path remains inspectable.
- Demo negative/reject paths remain fail-closed.
- Native package limitations remain explicit.
- qsl-server, qsl-attachments, website/external website, `.github`, Cargo manifests/lockfiles, branch protection, public-safety configuration, and protocol/crypto state-machine paths remain untouched.

## Allowed Scope

- `qsl/qsl-client/qsc-desktop/**` only if directly required for desktop readiness.
- `scripts/ci/demo_cli_smoke.sh` only if directly required for demo evidence integration.
- `docs/demo/**`.
- `docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md`.
- `tests/NA-0256_public_demo_desktop_readiness_testplan.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- `apps/qshield-cli/**` only for a proven minimal non-protocol demo issue.
- `scripts/ci/metadata_conformance_smoke.sh` only for a proven minimal smoke label/assertion adjustment.

## Forbidden Scope

- `.github/**`.
- Cargo manifests and lockfiles.
- qsp or protocol-core / crypto state-machine files.
- qsl-server.
- qsl-attachments.
- website or external website repositories.
- production relay/service implementation.
- branch-protection settings.
- public-safety/check configuration.
- production-ready desktop claims.

## Demo Positive Path Proof

Run:

```bash
scripts/ci/demo_cli_smoke.sh
```

Expected:

- command exits zero;
- emits `DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK`;
- emits `DEMO_ACCEPTANCE_OK`;
- proves Alice/Bob init, register, establish, send, receive/decrypt, and intended sender/plaintext output.

## Demo Negative / Reject Proof

The same command must emit:

```text
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
```

Expected:

- missing relay authorization rejects;
- malformed input rejects without echoing secret-bearing material;
- invalid relay ID rejects;
- replay rejects fail-closed;
- rejects do not turn into silent downgrade or fallback success.

## No Secret / Token Leak Proof

The demo smoke must emit:

```text
DEMO_NO_SECRET_LEAK_OK
```

The final validation bundle must also run the repo leak scan in added-line mode:

```bash
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- no live relay token, passphrase, auth header, sentinel, or raw secret-like material is introduced into tracked evidence;
- command transcripts stay outside the repo under `/srv/qbuild/tmp/`.

## Metadata Conformance Proof

Run:

```bash
scripts/ci/metadata_conformance_smoke.sh
```

Expected:

- command exits zero;
- emits `metadata-conformance-smoke: OK`;
- preserves loopback default, unsafe public bind acknowledgement, required relay authorization, sanitized errors, permission checks, queue/rate/quota bounds, padding metadata checks, identity-binding rejects, bundle-consumption behavior, replay reject behavior, and no token/sentinel echo in checked paths.

## qshield CLI Readiness Proof

Capture help output for:

```bash
qshield --help
qshield relay --help
qshield establish --help
qshield send --help
qshield recv --help
```

Expected:

- help remains demo/non-production scoped;
- unsafe non-loopback relay binding remains explicit;
- token values are not printed;
- user-facing command surface does not contain active `unwrap`/`panic` paths.

## Desktop Contract / Build Proof

Run from repo root:

```bash
cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
```

Run from `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Expected:

- qsc desktop contract tests pass;
- qsp protocol gate tests pass;
- frontend build passes;
- sidecar preparation passes;
- native Tauri package build passes only on a host with required native prerequisites;
- if native package build is blocked by host prerequisites such as `pkg-config` / GLib, record the exact blocker and do not install global system packages in this lane.

## Screenshot / Transcript Proof

Expected:

- command transcripts are stored outside the repo under `/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_<timestamp>/`;
- screenshot is generated only if host display/browser/native package prerequisites are already available;
- if screenshot cannot be generated, the audit records the host limitation and does not claim screenshot or native package proof.

## Governance / CI Expectations

Run:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allow 'docs/demo/**' --allow 'docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md' --allow 'tests/NA-0256_public_demo_desktop_readiness_testplan.md' --allow 'DECISIONS.md' --allow 'TRACEABILITY.md' --allow 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

Expected:

- READY_COUNT `1`, READY `NA-0256` before closeout;
- D-0479 exists once after implementation;
- D-0480 absent before closeout;
- no duplicate decision IDs;
- no forbidden paths touched;
- public-safety remains required and green;
- required PR checks pass before merge;
- merge uses merge commit only, with no direct push, admin bypass, squash, or rebase.

## Post-Fix Hardening Review Checklist

- Correctness under stress: demo negative/reject markers and desktop protocol-inactive tests stay fail-closed.
- Minimality: patch remains docs/evidence/governance unless a proven in-scope source defect appears.
- Maintainability: readiness docs point to existing scripts/tests instead of inventing a parallel demo harness.
- Coverage quality: tests prove actual positive, negative, leak-safe, and protocol-inactive behavior.
- Cross-lane stability: host limitations are documented without weakening Linux/macOS, public-safety, or package proof boundaries.
