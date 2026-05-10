Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# NA-0260 Attachment Demo Readiness Audit

Directive: QSL-DIR-2026-05-10-057 / NA-0260

## Objective

Add truthful public demo readiness for attachment descriptor/fetch/decrypt and
integrity behavior while preserving opaque-ciphertext boundaries and avoiding
protocol/crypto state-machine changes, qsp protocol-core changes, qsl-server
production hardening, qsl-attachments production hardening, website changes,
workflow changes, branch-protection changes, public-safety changes, and Cargo
dependency changes.

## Baseline Proof

- Starting `origin/main`: `3947352f359a`.
- PR #769: merged as `3947352f359a`.
- PRs #768 through #757 and PR #708: merged.
- PR #750 and PR #722: closed and unmerged.
- Branch protection: expected protected contexts present, including
  `public-safety`; force pushes and deletions disabled; admin enforcement
  enabled.
- Latest starting-main `public-safety`: success.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0260`.
- Decision proof before edits: D-0486 existed once; D-0487 and D-0488 absent;
  duplicate decision count zero.

## Surfaces Inspected

- `NEXT_ACTIONS.md` NA-0260 entry.
- `docs/demo/**`.
- `docs/governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md`.
- `docs/governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md`.
- `docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md`.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`.
- `apps/qshield-cli/**`.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- qsc attachment tests:
  `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs` and
  `qsl/qsl-client/qsc/tests/attachments_contract_na0217h.rs`.
- qsc attachment implementation references under `qsl/qsl-client/qsc/src/**`
  for read-only boundary understanding.
- qsl-server and qsl-attachments path searches. Those sibling repo directories
  are not present in this qsl-protocol checkout; this lane did not create or
  modify them.

## Packet A Classification

Selected path: Path 2, minimal demo-only attachment evidence surface.

Reason:

- Existing qshield public demo already has a local relay, bearer-token auth,
  Suite-2 actor-backed send/receive/decrypt, replay rejection, malformed-input
  rejection, invalid-id rejection, and secret-leak checks.
- Existing qshield public demo did not have an attachment descriptor/fetch/
  decrypt/integrity command.
- Existing qsc attachment tests prove production-client attachment behavior but
  are not the public qshield demo surface.
- A bounded qshield attachment proof can carry descriptor, fetch/decrypt,
  integrity, missing-auth, and no-leak evidence without changing protocol-core,
  crypto state machines, qsl-server, qsl-attachments production code, website,
  workflows, public-safety, branch protection, or Cargo dependencies.

## Implementation Summary

- Added `qshield attachment send` and `qshield attachment recv` under
  `apps/qshield-cli/src/commands/attachment.rs`.
- The send command creates a demo-only descriptor and encrypts both descriptor
  and payload through the existing refimpl actor Suite-2 demo e2e path.
- The local demo relay queues only opaque wire hex values.
- The receive command fetches the pair, decrypts the descriptor, validates
  descriptor fields, validates ciphertext length/hash, decrypts the payload,
  and writes the output only after validation.
- Added a test-only `--tamper-ciphertext` option for deterministic integrity
  rejection.
- Updated `scripts/ci/demo_cli_smoke.sh` to run the positive attachment
  roundtrip, compare input/output bytes, run the tampered reject, verify no
  output file is written on reject, and emit stable NA-0260 markers.
- Added public-safe demo readiness documentation, this audit, the NA-0260
  testplan, D-0487, traceability, and a rolling journal entry.

## Commands Run

Hard-start and preflight proof included:

```bash
df -BG /srv/qbuild
git status --porcelain=v1 --branch
git fetch --all --prune
git rev-parse origin/main
gh pr view 769 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 768 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 767 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 766 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 765 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 764 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 763 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 762 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 761 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 760 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 759 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 758 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 757 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 750 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Attachment transcript proof:

```bash
scripts/ci/demo_cli_smoke.sh
```

Focused build proof during implementation:

```bash
cargo build -p qshield-cli --locked
```

## Transcript Artifact

Artifact directory:

```text
/srv/qbuild/tmp/NA-0260_attachment_demo_artifacts_20260510T041841Z/
```

Transcript:

```text
/srv/qbuild/tmp/NA-0260_attachment_demo_artifacts_20260510T041841Z/demo_cli_smoke.log
```

The transcript includes:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_ATTACHMENT_DESCRIPTOR_OK
DEMO_ATTACHMENT_FETCH_DECRYPT_OK
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_ATTACHMENT_NO_SECRET_LEAK_OK
NA0260_ATTACHMENT_DEMO_READY_OK
DEMO_NO_SECRET_LEAK_OK
DEMO_ACCEPTANCE_OK
```

## Positive Outcome

The qshield positive path still proves two-store initialization, local relay
startup, authorized peer registration, explicit demo unauthenticated establish,
send, receive, decrypt, and expected plaintext output for the normal message
path.

The attachment path additionally proves:

- descriptor creation;
- descriptor encryption as an opaque Suite-2 demo wire;
- attachment payload encryption as an opaque Suite-2 demo wire;
- authenticated relay fetch through poll;
- descriptor decrypt and validation;
- descriptor-bound ciphertext length/hash validation;
- attachment decrypt; and
- output byte equality with the sender payload.

## Negative Outcome

The existing demo smoke still proves missing-auth, malformed input, invalid
relay id, and replay rejects against the current local demo relay.

The attachment-specific negative proof uses `--tamper-ciphertext`. It creates a
descriptor over the original ciphertext, queues modified ciphertext, and proves
the receiver rejects with `attachment_integrity_reject`.

## No-Mutation / Fail-Closed Outcome

The attachment reject path writes no receiver output file before emitting
`DEMO_ATTACHMENT_INTEGRITY_REJECT_OK`. The proof does not claim relay queue
no-mutation because the existing demo relay poll API is consuming by design.
The no-mutation claim is therefore limited to receiver output state where it is
applicable.

## Opaque-Boundary Guarantee

The qshield demo relay receives encrypted wire hex values for both descriptor
and payload messages. The attachment plaintext is not sent to the relay and is
not printed in the transcript or relay startup log. The receiver writes
plaintext only to the requested output directory after descriptor and
ciphertext integrity validation pass.

## No-Fake-Evidence Guarantee

- Positive readiness is emitted only after qshield attachment send/recv pass and
  the receiver output matches the sender payload.
- Integrity readiness is emitted only after the tampered path fails with the
  specific `attachment_integrity_reject` reason and writes no output file.
- The proof does not treat arbitrary command failure as attachment integrity
  success.
- The proof mode is explicitly labeled demo-only and non-production.

## No-Production-Overclaim

This audit supports only non-production public demo readiness for bounded
attachment descriptor/fetch/decrypt/integrity behavior. It does not support
production qsl-server readiness, production qsl-attachments readiness,
production relay readiness, full retention/resume/quota/durability readiness,
or a release approval claim.

## Residual Gaps

- Cross-host/private-network attachment proof remains future work.
- `docs/public/**` stale summaries remain for NA-0261 or another explicitly
  authorized public-docs lane.
- qsl-server and qsl-attachments production hardening remain separate.
- qsc production attachment test coverage remains independent of this qshield
  public demo proof.

## Recommendations

1. Merge NA-0260 evidence only after required checks pass normally with
   `public-safety` still required.
2. Keep NA-0260 READY after this implementation/evidence PR until a separate
   closeout promotes exactly one successor.
3. Do not claim production attachment readiness, production relay readiness, or
   qsl-attachments hardening from this proof.
4. Use NA-0261 to refresh stale public/demo evidence summaries after merge.
