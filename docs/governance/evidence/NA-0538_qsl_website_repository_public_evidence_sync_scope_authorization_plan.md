Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0538 QSL Website / Repository Public Evidence Sync Scope Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0538 is authorization-only. It reviews current repository, governance, evidence, and public-surface state and authorizes one later public-facing evidence-sync lane. It does not implement website, README, public-doc, qsc runtime, qsl-server, qsl-attachments, workflow, dependency, lockfile, corpus, vector, formal, service, backup, or remote changes.

Selected classification: `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_READY`.

Selected successor: `NA-0539 -- QSL Website / Repository Public Evidence Sync Implementation Harness`.

The approved future lane may update only the selected future path bundle with bounded, no-overclaim wording. The future public message should make QSL's recent engineering progress visible while preserving explicit limits: no public-readiness, no production-readiness, no crypto-complete, no identity-complete, no trust-complete, no replay-proof, no downgrade-proof, no external-review-complete, no vulnerability-free, no bug-free, and no perfect-crypto claim.

## qwork proof-file verification

- qwork was not run by Codex.
- qstart was not run by Codex.
- qresume was not run by Codex.
- qwork proof files were read from `/srv/qbuild/work/NA-0538/.qwork/` and copied into the directive proof root.
- `.kv` proof recorded `startup_result=OK`, lane `NA-0538`, repo `qsl-protocol`, path `/srv/qbuild/work/NA-0538/qsl-protocol`, clean worktree/index/untracked state, `head_equals_origin_main=yes`, `ready_count=1`, queue top READY `NA-0538`, and requested lane status READY.
- `.json` proof mirrored the `.kv` fields.
- Proof written time was `2026-06-25T15:02:49Z`, after D448 response time `2026-06-25T14:47:34Z`.
- Live pre-fetch HEAD and live pre-fetch origin/main both matched proof HEAD `14f81642a8a9`.
- Fetch occurred only after the proof/live ref match and disk-gate check.

## D448/D446 inheritance

- D448 closed NA-0537, accepted PR #1348 as merge `14f81642a8a9`, and restored NA-0538 READY without performing NA-0538 implementation.
- D446 recorded classification `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_PASS`.
- D446 run 1 valid qsc E2EE passed, wrong-peer repeat failed closed, selected state remained unchanged, and cleanup passed.
- D446 run 2 valid qsc E2EE passed, stale/replaced-peer repeat failed closed, selected state remained unchanged, and cleanup passed.
- D446 recorded no stale state reuse, retained remote qsc unchanged, no qsl-server, no qsl-attachments, and no public/production/security-completion claim.

## D441/D439/D419 inheritance

- D441 recorded classification `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`.
- D441 baseline remote qsc E2EE setup passed.
- D441 wrong-peer negative and stale/replaced-peer negative passed with selected-state no-mutation checks.
- D441 valid path after negatives and cleanup passed.
- D439 recorded the port 39176 diagnostic marker/ACK pass.
- D419 recorded replay and corrupt-delivery negatives passed and cleanup passed.
- No inherited record turns bounded lab evidence into a public-readiness, production-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, external-review-complete, vulnerability-free, bug-free, or perfect-crypto claim.
- NA-0538 remains authorization-only.

## Current public surface inventory

Existing:

- `README.md`.
- `docs/`.
- `docs/public/`.
- `docs/governance/evidence/`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Missing:

- `public/`.
- `website/`.
- `qsl-server/`.
- `qsl-attachments/`.
- `docs/public/README.md`.

Read-only public-surface scans found existing no-claim and evidence language in `README.md` and `docs/public/**`. The scans also found many current references to E2EE, remote qsc, QSL, and external review. Those hits are inventory evidence only; NA-0538 does not edit public surfaces.

## Public evidence candidate inventory

Public-safe summary topics for NA-0539:

- QSL goals G1-G5.
- Direct qsc client-to-client E2EE sprint.
- Evidence-driven governance.
- Fail-closed discipline.
- No-overclaim culture.
- Remote real-world testing while host availability permits.
- Same-host qsc E2E tests.
- Remote host setup/account boundary evidence.
- Retained remote qsc staging and restaging.
- SSH reverse-forward marker/ACK evidence.
- Build-to-Inspiron qsc E2EE success.
- Selected replay/corrupt delivery negative boundaries.
- Selected wrong-peer/stale-trust negative boundaries.
- Repeated-run cleanup/freshness.
- Public-safety and advisories gates green on current main, with no required red check.
- quinn-proto RUSTSEC-2026-0185 remediation boundary at `0.11.15` in root and nested qsc fuzz lockfiles.
- Bounded formal/model checks.
- Corpus validators and secret-material scans as bounded evidence.

Remaining limits to state plainly:

- Not public ready.
- Not production ready.
- Not public-internet ready.
- External review is not complete.
- Cryptographic design and implementation are not complete.
- Identity and trust work are not complete.
- Replay and downgrade resistance have selected bounded evidence only.
- Side-channel freedom, vulnerability freedom, bug freedom, and perfect crypto are not claimed.
- qsl-server and qsl-attachments remain deferred for this lane.
- Public/website sync is not implemented in NA-0538.

## Public claim wording policy

Permitted style for NA-0539:

- "QSL has bounded evidence for a direct remote qsc E2EE workflow using synthetic data."
- "QSL has repeated-run cleanup/freshness evidence under a controlled lab setup."
- "QSL has fail-closed evidence for selected wrong-peer, stale/replaced-peer, replay, and corrupt-delivery cases."
- "These results are engineering evidence, not production readiness."
- "We invite review of the evidence, limits, and next steps."

Forbidden style for NA-0539:

- Do not say QSL is production ready.
- Do not say QSL is public ready.
- Do not say QSL is replay proof.
- Do not say QSL is downgrade proof.
- Do not say QSL is vulnerability free.
- Do not say QSL has perfect crypto.
- Do not say QSL has complete identity, trust, or security proof.
- Do not say QSL has completed external review.

Required same-line qualifiers:

- Remote qsc E2EE statements must include bounded, synthetic, controlled, or lab wording and must say the result is not production readiness.
- Negative-test statements must name selected cases and must not imply universal replay, downgrade, identity, or trust proof.
- Formal-check statements must say bounded formal/model checks and must not imply complete formal proof.
- Cargo audit/advisory statements must say current gate status or current advisory remediation and must not imply vulnerability freedom.

## Proof/redaction rules

Future public implementation must not copy raw proof logs. It may cite decision IDs, PR numbers, evidence doc names, testplan names, bounded classification names, and no-claim boundaries.

Future public implementation must not publish private keys, passphrases, tokens, passwords, production endpoints, backup material, raw qsc runtime material, route-token/capability material, raw SSH config, `authorized_keys`, `known_hosts`, or detailed remote topology. Use approved general labels such as controlled lab setup, remote host, local client, retained qsc staging, and synthetic data.

## Option review

- Option 1, website + README + repo evidence sync implementation next: selected. The repository has `README.md` and existing `docs/public/**` surfaces, so the future lane can update the repository public front door and public evidence docs without inventing missing `public/` or `website/` paths.
- Option 2, README-only sync: rejected as too narrow because `docs/public/**` already exists and is the current public evidence front door.
- Option 3, website-only landing page sync: rejected because `public/` and `website/` are missing.
- Option 4, governance evidence index only: rejected as safe but too low visibility.
- Option 5, no public sync yet: rejected because D446/D448 created enough bounded evidence to justify truthful public synchronization.
- Option 6, qsl-server/qsl-attachments integration next: rejected as deferred and outside direct public sync.
- Option 7, public/production readiness package: rejected as premature and overclaim-prone.

## Selected future implementation design

NA-0539 should update the repository public front door and existing public evidence docs so reviewers can see current goals, evidence, limits, and invitation for review in one coherent path. It must preserve evidence-first wording, cite D-1066 and relevant inherited decisions, keep every strong statement bounded, and include explicit no-claim limits near the evidence claims.

## Selected future path bundle

NA-0539 may mutate only the following selected future paths:

- `README.md`.
- `docs/README.md`.
- `docs/public/INDEX.md`.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`.
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`.
- `docs/public/WEBSITE_CLAIM_MATRIX.md`.
- `docs/governance/evidence/NA-0539_qsl_website_repository_public_evidence_sync_implementation_harness.md`.
- `tests/NA-0539_qsl_website_repository_public_evidence_sync_implementation_testplan.md`.
- `DECISIONS.md`.
- `TRACEABILITY.md`.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

NA-0539 must not create or mutate missing `public/` or `website/` paths unless a later directive explicitly authorizes that path creation. `docs/public/README.md` is missing and is not selected for NA-0539.

## Hostile Cryptographer Review

- Public sync must not convert bounded lab evidence into crypto-complete claims.
- Negative tests are selected cases, not universal proofs.
- Remote qsc E2EE evidence is synthetic and bounded.
- Formal checks are bounded evidence only.

## Red-Team Review

- Main risk is attracting attention with overclaims.
- Public content must be accurate enough for adversarial readers.
- Public content must not leak topology, route tokens, proof-root internals, or operational secrets.
- Public content must not imply qsl-server/qsl-attachments integration while those components are deferred.

## Production SRE Review

- Public materials should communicate operational maturity honestly.
- Cleanup/freshness and public-safety gates may be stated as bounded evidence.
- Public materials must not imply service deployment, production readiness, public internet readiness, or support commitments.

## Release-Claim Boundary Review

NA-0538 and NA-0539 preserve:

- no public-ready claim.
- no production-ready claim.
- no public-internet-ready claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no identity-complete claim.
- no trust-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free, bug-free, or perfect-crypto claim.

## Successor selection

Selected successor: `NA-0539 -- QSL Website / Repository Public Evidence Sync Implementation Harness`.

Selected classification: `PUBLIC_EVIDENCE_SYNC_IMPLEMENTATION_READY`.

Exactly one READY remains mandatory.

## Future validation/marker plan

NA-0539 should prove:

- `NA0539_D1066_AUTHORIZATION_CONSUMED_OK`.
- `NA0539_SELECTED_PATH_BUNDLE_ONLY_OK`.
- `NA0539_PUBLIC_CLAIM_POLICY_APPLIED_OK`.
- `NA0539_EVIDENCE_CITATIONS_PRESENT_OK`.
- `NA0539_NO_RAW_PROOF_LOGS_OK`.
- `NA0539_NO_PRIVATE_MATERIAL_OK`.
- `NA0539_NO_QSC_SOURCE_MUTATION_OK`.
- `NA0539_NO_QSL_SERVER_ATTACHMENTS_OK`.
- `NA0539_NO_PUBLIC_READINESS_CLAIM_OK`.
- `NA0539_NO_PRODUCTION_READINESS_CLAIM_OK`.
- `NA0539_NO_CRYPTO_COMPLETE_CLAIM_OK`.
- `NA0539_ONE_READY_INVARIANT_OK`.

## No qsl-server/qsl-attachments boundary

NA-0538 did not use or mutate qsl-server or qsl-attachments. Both paths are missing in this checkout. NA-0539 must not implement or imply qsl-server/qsl-attachments integration.

## No implementation mutation in NA-0538

NA-0538 does not mutate `README.md`, `docs/public/**`, `public/`, `website/`, qsc source/test/fuzz/Cargo paths, workflow files, scripts/helpers, lockfiles, dependency manifests, corpus/vector/input paths, formal/refimpl/service/public/backup paths, qsl-server, or qsl-attachments.

## No public/production/security-completion claims

This authorization does not make a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, vulnerability-free, bug-free, or perfect-crypto claim.

## Backup-impact statement

NA-0538 is a governance authorization record only. It did not execute qsl-backup, did not mutate `/usr/local/sbin/qsl-backup`, did not mutate backup paths, and did not change backup status/plan files.

## Next recommendation

After NA-0538 merges and closes out, restore NA-0539 as the sole READY item so the repository public front door and existing public evidence docs can be synchronized with D446/D448 evidence under the D-1066 claim policy and redaction rules.
