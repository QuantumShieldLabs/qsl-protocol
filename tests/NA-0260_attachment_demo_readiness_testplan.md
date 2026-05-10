Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0260 Attachment Demo Readiness Test Plan

## Objective

Prove that the public qshield demo includes a truthful, non-production
attachment descriptor/fetch/decrypt/integrity proof while preserving
opaque-ciphertext boundaries and avoiding protocol/crypto state-machine,
qsp protocol-core, qsl-server production, qsl-attachments production, website,
workflow, public-safety, branch-protection, and Cargo dependency changes.

## Protected Invariants

- Attachment demo remains non-production.
- Opaque ciphertext boundary remains intact.
- Descriptor/fetch/decrypt/integrity proof is truthful.
- Rejects fail closed.
- No receiver output mutation occurs on the proved tamper reject path.
- No token, secret, or attachment plaintext leakage occurs in checked
  transcript/relay output.
- No production readiness claim is made.
- Existing positive qshield demo remains green.
- Existing missing-auth, malformed, invalid relay id, replay, and KT-negative
  markers remain green.

## Allowed Scope

- `docs/demo/**`.
- `apps/qshield-cli/**` for minimal demo CLI support.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/metadata_conformance_smoke.sh` only if needed without weakening
  checks.
- `docs/governance/evidence/NA-0260_attachment_demo_readiness_audit.md`.
- `tests/NA-0260_attachment_demo_readiness_testplan.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Forbidden Scope

Forbidden paths and behaviors include `.github/**`, Cargo manifests/lockfiles,
qsp/protocol-core or crypto state-machine files, qsl-server, qsl-attachments,
qsc-desktop implementation, website/external website, formal model changes,
branch-protection settings, public-safety/check configuration, production
relay/service implementation, fake attachment evidence, plaintext relay storage
where opaque boundary is claimed, and production attachment readiness claims.

## Positive Descriptor / Fetch / Decrypt Proof

Run:

```bash
scripts/ci/demo_cli_smoke.sh
```

Expected attachment markers:

```text
DEMO_ATTACHMENT_DESCRIPTOR_OK
DEMO_ATTACHMENT_FETCH_DECRYPT_OK
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
NA0260_ATTACHMENT_DEMO_READY_OK
```

Expected behavior:

- `qshield attachment send` creates a demo-only descriptor.
- Descriptor and attachment payload are encrypted as separate Suite-2 demo wires
  before relay submission.
- The local relay stores and returns only opaque wire hex.
- `qshield attachment recv` fetches the pair, decrypts the descriptor, validates
  descriptor fields and ciphertext hash/length, decrypts the payload, and writes
  the output file.
- The smoke compares the output file to the sender payload.

## Integrity / Reject Proof

The same smoke must run:

```bash
qshield attachment send --tamper-ciphertext ...
qshield attachment recv ...
```

Expected marker:

```text
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
```

Expected behavior:

- the descriptor is created over the original ciphertext;
- the queued ciphertext is modified after descriptor creation;
- receive fails with `attachment_integrity_reject`;
- no receiver output file is written for the tampered payload; and
- arbitrary command failures are not accepted as integrity proof.

## Opaque-Ciphertext Boundary Proof

Expected marker:

```text
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
```

Expected behavior:

- relay queue submissions contain qshield Suite-2 demo wire hex values;
- attachment plaintext is not sent to the relay;
- descriptor plaintext is encrypted before relay storage;
- attachment plaintext appears only in the receiver output file after
  validation; and
- docs and markers label this as demo-only, not production service readiness.

## No Token / Secret / Plaintext Leakage Proof

Expected marker:

```text
DEMO_ATTACHMENT_NO_SECRET_LEAK_OK
DEMO_NO_SECRET_LEAK_OK
```

Expected behavior:

- the relay token is not printed in checked output;
- the attachment sentinel plaintext is not printed in checked command output or
  relay startup output;
- reject bodies remain sanitized; and
- the intentional receiver output file is excluded from log-leak claims because
  it is the delivered plaintext artifact.

## Existing Positive And Negative Demo Proof

The same smoke must continue to emit:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NEGATIVE_KT_NO_MUTATION_OK
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
DEMO_ACCEPTANCE_OK
```

## Non-Production Posture

The qshield attachment commands are a public demo proof surface. They do not
claim qsl-server readiness, qsl-attachments readiness, production attachment
retention/resume/quota/durability behavior, production relay exposure, or full
metadata privacy.

## Prerequisite Stop Conditions

Stop instead of merging if:

- the demo cannot prove descriptor/fetch/decrypt/integrity truthfully;
- opaque-ciphertext boundary cannot be preserved;
- proof would require protocol/crypto state-machine changes;
- proof would require qsl-server or qsl-attachments production changes;
- proof would require website, workflow, branch-protection, public-safety, or
  Cargo dependency changes;
- tampered ciphertext is accepted;
- the tampered reject writes receiver output;
- token/secret/plaintext leakage is detected in checked outputs; or
- public-safety is not required and green.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed 'docs/demo/**' \
  --allowed 'apps/qshield-cli/**' \
  --allowed 'scripts/ci/demo_cli_smoke.sh' \
  --allowed 'scripts/ci/metadata_conformance_smoke.sh' \
  --allowed 'docs/governance/evidence/NA-0260_attachment_demo_readiness_audit.md' \
  --allowed 'tests/NA-0260_attachment_demo_readiness_testplan.md' \
  --allowed 'DECISIONS.md' \
  --allowed 'TRACEABILITY.md' \
  --allowed 'docs/ops/ROLLING_OPERATIONS_JOURNAL.md'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI / Public-Safety Expectations

Required PR contexts must attach and pass normally:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

CodeQL may be accepted as neutral only if GitHub branch protection accepts it.
Merge must use a merge commit with `--match-head-commit`, with no direct push,
admin bypass, squash, rebase, public-safety weakening, or branch-protection
exception.
