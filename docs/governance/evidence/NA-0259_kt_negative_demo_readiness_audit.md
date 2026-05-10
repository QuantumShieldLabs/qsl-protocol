Goals: G1, G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0259 KT-Negative Demo Readiness Audit

Directive: QSL-DIR-2026-05-09-056 / NA-0259

## Objective

Add truthful public demo readiness for KT-negative reject behavior without fake
KT evidence, protocol or crypto state-machine changes, qsl-server changes,
qsl-attachments changes, website changes, workflow changes, branch-protection
changes, public-safety changes, or Cargo dependency changes.

## Baseline Proof

- Starting `origin/main`: `d4ce1959a45f`.
- PR #767: merged as `d4ce1959a45f`.
- PRs #766, #765, #764, #763, #762, #761, #760, #759, #758, #757, and #708:
  merged.
- PR #750 and PR #722: closed and unmerged.
- Branch protection: expected protected contexts present, including
  `public-safety`; force pushes and deletions disabled; admin enforcement
  enabled.
- Latest starting-main `public-safety`: success.
- Queue proof before edits: `READY_COUNT 1`, sole READY `NA-0259`.
- Decision proof before edits: D-0484 existed once; D-0485 and D-0486 absent;
  duplicate decision count zero.

## Surfaces Inspected

- `NEXT_ACTIONS.md` NA-0259 entry.
- `docs/demo/**`.
- `docs/governance/evidence/NA-0257_cross_host_demo_reproducibility_audit.md`.
- `docs/governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md`.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`.
- `tools/refimpl/quantumshield_refimpl/src/kt/**`.
- `tools/refimpl/quantumshield_refimpl/tests/kt_verifier_vectors.rs`.
- `inputs/suite2/vectors/qshield_suite2_kt_verifier_vectors_v1.json`.
- `apps/qshield-cli/**`.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/metadata_conformance_smoke.sh`.
- `tests/**` and read-only qsc KT/demo evidence searches.

## Packet A Classification

Selected path: Path 2, minimal demo-only KT evidence surface.

Reason:

- Existing qshield CLI demo commands do not carry live KT evidence through
  `qshield establish`.
- The repository already contains canonical KT verifier implementation, vector
  evidence, and no-mutation tests from PR #708 and NA-0242.
- The existing one-command demo smoke is a public demo surface and can
  truthfully invoke those verifier proofs as a bounded, non-production KT
  negative proof.
- No protocol, wire, key schedule, crypto state-machine, qsl-server,
  qsl-attachments, qsc-desktop, website, public-safety, branch-protection, or
  Cargo dependency change is required.

## Implementation Summary

- Updated `scripts/ci/demo_cli_smoke.sh` to run:
  - canonical KT verifier vectors;
  - rejected consistency advancement accepted-state no-mutation proof; and
  - explicit disabled-shape non-production boundary proof.
- Added stable KT demo markers:
  - `DEMO_NEGATIVE_KT_REJECT_OK`;
  - `DEMO_NEGATIVE_KT_NO_MUTATION_OK`;
  - `DEMO_KT_NON_PRODUCTION_BOUNDARY_OK`;
  - `NA0259_KT_NEGATIVE_DEMO_READY_OK`.
- Added public-safe demo readiness documentation.
- Added this audit and the NA-0259 testplan.
- Added D-0485 and traceability links.

## Commands Run

Packet A and hard-start proof:

```bash
df -BG /srv/qbuild
git status --porcelain=v1 --branch
git fetch --all --prune
git rev-parse origin/main
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
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/d4ce1959a45f85fcffdd68e869012c9a26a34f63/check-runs?per_page=100"
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
```

KT/demo transcript proof:

```bash
scripts/ci/demo_cli_smoke.sh
```

Targeted KT commands now carried by the smoke:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
```

## Transcript Artifact

Artifact directory:

```text
/srv/qbuild/tmp/NA-0259_kt_negative_demo_artifacts_20260510T002546Z/
```

Transcript:

```text
/srv/qbuild/tmp/NA-0259_kt_negative_demo_artifacts_20260510T002546Z/demo_cli_smoke_kt_negative_transcript.log
```

The transcript includes:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NEGATIVE_KT_NO_MUTATION_OK
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
DEMO_NO_SECRET_LEAK_OK
NA0259_KT_NEGATIVE_DEMO_READY_OK
DEMO_ACCEPTANCE_OK
```

## Positive Outcome

The qshield demo still proves the established local positive path:

- two demo stores initialize;
- loopback relay starts;
- authenticated peer registration succeeds;
- explicit demo unauthenticated establish override is visible;
- send succeeds; and
- receive/decrypt shows the expected plaintext from the expected peer.

The KT vector proof also includes positive first-seen and advanced KT cases.

## Negative Outcome

The qshield demo still proves missing-auth, malformed input, invalid relay id,
and replay rejects. The KT vector proof proves selected invalid KT evidence
rejects with `kt_fail`, including stale STH, missing STH, inclusion mismatch,
and missing consistency proof.

## No-Mutation Outcome

The no-mutation marker is tied to
`kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state`.
That test compares the accepted KT state snapshot before and after a rejected
consistency advancement. The qshield demo stores are not KT trust stores in this
proof, so no broader qshield KT-store mutation claim is made.

## No-Fake-KT Guarantee

- No arbitrary failure is accepted as KT rejection proof.
- No KT success is claimed for malformed evidence.
- Disabled KT shape remains bounded to an explicit non-production test mode.
- The proof uses existing canonical verifier tests and vector-defined
  mutations rather than fabricated transcripts.

## No-Production-Overclaim

This audit supports only non-production public demo readiness for selected
KT-negative reject behavior. It does not support production KT readiness, live
KT log operation, externally verified KT service readiness, or qshield live KT
evidence ingestion.

## Residual Gaps

- Live qshield KT evidence input remains absent.
- Cross-host KT-negative proof remains separate.
- Production KT service/log readiness remains separate.
- Public release-readiness summary docs under `docs/public/**` remain outside
  this packet's allowed changed paths and should be refreshed only under an
  explicitly authorized public-docs scope.

## Recommendations

1. Merge NA-0259 evidence only after required checks pass normally with
   `public-safety` still required.
2. Keep NA-0259 READY after the implementation/evidence PR until a separate
   closeout promotes one successor.
3. Do not claim production KT readiness or live qshield KT ingestion from this
   proof.
4. Consider a future reviewer-facing qshield KT evidence command only if it can
   stay demo-only and avoid protocol/crypto state-machine changes.
