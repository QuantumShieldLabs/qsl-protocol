Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-04
Replaces: n/a
Superseded-By: n/a

# NA-0250 External Review and Release-Readiness Audit

Directive: QSL-DIR-2026-05-04-030 / NA-0250

## Objective

Build a compact external-review and release-readiness evidence package that summarizes what is proven, what is not proven, how to reproduce core evidence, and which release gates remain open.

This is an evidence package only. It does not approve production release, implement website changes, or change protocol, runtime, crypto, demo, service, qsc, qsl-client, qsl-server, qsl-attachments, qsc-desktop, public-safety, workflow, script, Cargo, branch-protection, or website implementation behavior.

## Exact Commands Run

Hard-start and authority proof:

```bash
source /srv/qbuild/tools/env_qbuild.sh
df -BG /srv/qbuild
pwd
git status --porcelain=v1 --branch
git branch --show-current || true
git rev-parse HEAD || true
git diff --name-only || true
git ls-files --others --exclude-standard || true
git fetch --all --prune
git rev-parse origin/main
git log -1 --format='%H %s' origin/main
gh pr list --state open --limit 30 --json number,title,headRefName,baseRefName,isDraft,url
gh pr view 747 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 746 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 745 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 744 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 743 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 742 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 741 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 740 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 739 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 738 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 737 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 736 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 735 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 734 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 733 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 732 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 731 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 729 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 722 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh pr view 708 --json number,state,mergedAt,mergeCommit,headRefOid,baseRefName,title,url
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection
gh api "/repos/QuantumShieldLabs/qsl-protocol/commits/${head_sha}/check-runs?per_page=100"
```

Mandatory repo reads before edits:

```bash
git show origin/main:GOALS.md
git show origin/main:PROJECT_CHARTER.md
git show origin/main:DECISIONS.md
git show origin/main:TRACEABILITY.md
git show origin/main:CHECKLIST_PROTOCOL_CHANGE.md
```

Main health, parser, and command proof:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/demo_cli_smoke.sh
scripts/ci/metadata_conformance_smoke.sh
python3 <canonical queue parser>
python3 <canonical decision parser>
python3 <NA-0250 queue-entry quote helper>
```

Evidence discovery:

```bash
rg --files docs/public docs/governance/evidence tests docs/ops
sed -n '1,220p' docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
sed -n '1,220p' docs/public/WEBSITE_CLAIM_MATRIX.md
sed -n '1,220p' docs/public/WEBSITE_UPDATE_PLAN.md
sed -n '1,220p' docs/demo/DEMO_ACCEPTANCE_CRITERIA.md
sed -n '1,220p' docs/conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md
sed -n '1,220p' docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md
sed -n '1,220p' docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md
sed -n '1,220p' qsl/qsl-client/qsc-desktop/README.md
sed -n '1,220p' docs/design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md
sed -n '1,220p' formal/README.md
sed -n '1,220p' docs/governance/ENGINEERING_VELOCITY_POLICY.md
sed -n '1,220p' docs/governance/WORKDAY_AUTOPILOT_POLICY.md
sed -n '6370,7170p' DECISIONS.md
tail -n 220 TRACEABILITY.md
```

## Pass / Fail Summary

| Command or proof | Result | Notes |
| --- | --- | --- |
| Disk watermark | PASS | `/srv/qbuild` had 468G total, 33G used, 411G available, 8% used. |
| Worktree cleanliness | PASS | No tracked or untracked local content changes before edits. |
| `origin/main` authority | PASS | `3408b306666`. |
| PR state proof | PASS | PRs #747, #746, #745, #744, #743, #742, #741, #740, #739, #738, #737, #736, #735, #734, #733, #732, #731, #729, and #708 were merged; #722 was closed and unmerged. |
| Branch protection proof | PASS | Required contexts included `public-safety` and the expected required CI contexts. |
| Latest main public-safety | PASS | `public-safety` for `3408b306666` completed successfully. |
| `cargo audit --deny warnings` | PASS | Scanned 381 locked crate dependencies. |
| `cargo tree -i rustls-webpki --locked` | PASS | `rustls-webpki v0.103.13` is reached through `rustls v0.23.36`. |
| `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` | PASS | 3 tests passed. |
| `python3 formal/run_model_checks.py` | PASS | SCKA model: 926 states; negotiation model: 108 attempts, 214 rejects, 428 no-mutation assertions. |
| `scripts/ci/demo_cli_smoke.sh` | PASS | Ended with `DEMO_ACCEPTANCE_OK` and `demo-cli-smoke: OK`. |
| `scripts/ci/metadata_conformance_smoke.sh` | PASS | Ended with `metadata-conformance-smoke: OK`. |
| Queue parser before edit | PASS | `READY_COUNT 1`; sole READY `NA-0250`. |
| Decision parser before edit | PASS | D-0110 and D-0439 through D-0465 existed once; D-0466 and D-0467 absent; no duplicate IDs. |

Non-fatal warnings:

- The initial demo and metadata smoke proof commands briefly waited on normal Cargo package/artifact locks because they were started concurrently. Both commands completed successfully without rerun or repo mutation.
- The post-edit `cargo audit --deny warnings` validation observed a temporary advisory database lock wait and completed successfully.

## Evidence Consulted

- [GOALS.md](../../../GOALS.md)
- [ROADMAP.md](../../../ROADMAP.md)
- [Suite-2 claim boundary](../../public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md)
- [Website claim matrix](../../public/WEBSITE_CLAIM_MATRIX.md)
- [Website update plan](../../public/WEBSITE_UPDATE_PLAN.md)
- [Demo acceptance criteria](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [Conformance vector prioritization](../../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [Engineering velocity policy](../ENGINEERING_VELOCITY_POLICY.md)
- [Workday autopilot policy](../WORKDAY_AUTOPILOT_POLICY.md)
- [Metadata threat model](../../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [Envelope transport profile](../../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- qsc desktop README — retired at NA-0651 (D-1274, 2026-07-16); see git history and DOC-QSC-009/010 (superseded, retained as history)
- [Desktop GUI active-ops boundary](../../design/DOC-QSC-010_Desktop_GUI_Prototype_Active_Ops_Boundary_v0.1.0_DRAFT.md)
- [Formal README](../../../formal/README.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [DECISIONS.md](../../../DECISIONS.md), especially D-0440 through D-0465
- [NA-0240 evidence](NA-0240_scka_persistence_monotonicity_audit.md)
- [NA-0241 evidence](NA-0241_demo_downgrade_no_mutation_audit.md)
- [NA-0242 evidence](NA-0242_kt_consistency_no_mutation_audit.md)
- [NA-0243 evidence](NA-0243_skipped_key_decrypt_no_mutation_audit.md)
- [NA-0244 evidence](NA-0244_metadata_conformance_negative_expansion_audit.md)
- [NA-0245 evidence](NA-0245_website_truthfulness_audit.md)
- [NA-0246 evidence](NA-0246_one_command_demo_acceptance_audit.md)
- [NA-0247 evidence](NA-0247_desktop_gui_public_demo_readiness_audit.md)
- [NA-0248 evidence](NA-0248_suite2_triple_ratchet_evidence_audit.md)
- [NA-0249 evidence](NA-0249_formal_downgrade_no_mutation_audit.md)

## Reviewer Package Summary

NA-0250 adds:

- [External review package](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [Release-readiness evidence map](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- this audit report
- [NA-0250 testplan](../../../tests/NA-0250_external_review_release_readiness_testplan.md)
- D-0466 in [DECISIONS.md](../../../DECISIONS.md)
- TRACEABILITY evidence for NA-0250

The public package gives reviewers a single place to find the current posture, proven evidence, unproven claims, reproducible commands, artifact index, recent PR table, review questions, gaps, and safe public wording.

## Release-Readiness Matrix Summary

- G1 is PARTIAL: always-hybrid evidence exists, but release readiness and external review are not complete.
- G2 is PARTIAL: SCKA persistence/monotonicity evidence exists, but broader release reproduction remains open.
- G3 is PARTIAL: downgrade/no-mutation evidence exists, but bounded model abstractions and demo limits remain explicit.
- G4 is PARTIAL: formal/model checks and CI gates exist, but external review and cross-host reproducibility are incomplete.
- G5 is PARTIAL: metadata conformance baseline exists, but anonymity is not claimed and phase-2 work remains open.

## Known Limitations

- This package is not external cryptographic review completion.
- This package is not production release approval.
- Formal/model evidence is bounded and does not prove AEAD/KDF/authentication/secrecy security.
- Demo evidence is loopback/non-production and does not prove KT-negative or attachment readiness.
- Desktop GUI evidence is guided-prototype readiness and does not prove fully provisioned native packaging on this host.
- Metadata evidence does not prove anonymity, metadata-free messaging, or full traffic-analysis resistance.
- Website evidence is audit and planning evidence only; no website implementation changes are included.

## No Implementation Changes

NA-0250 makes no implementation changes. It does not edit `.github`, `scripts`, `Cargo.toml`, `Cargo.lock`, `qsp`, `qsc`, `qsl`, `qsl-client`, `apps`, `tools`, `inputs`, `formal`, `qsc-desktop`, `qsl-server`, `qsl-attachments`, `website`, public-safety helpers/configuration, branch-protection settings, runtime code, protocol code, crypto code, demo implementation, or service implementation.

## Future Work

1. Close out NA-0250 only after the Packet A package merges and post-merge gates pass.
2. Restore NA-0251 as a handoff package for website evidence-boundary implementation, not website implementation itself.
3. Add KT-negative demo evidence only when the demo surface truthfully carries KT evidence.
4. Add attachment demo readiness proof before public attachment demo claims.
5. Define metadata phase-2 work for identifier rotation, padding defaults, retention/purge, and broader sanitized-error behavior.
6. Record external reviewer findings in a separate evidence lane.
