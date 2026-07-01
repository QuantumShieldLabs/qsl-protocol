Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-01

# NA-0583 Remote Relay Service Reachability After Local Ready Triage Harness

## Executive Summary

NA-0583 consumed D-1155 and D-1156, verified fresh qwork proof from
`2026-07-01T07:46:34Z`, verified current main at `d96ecc074abc`, corrected
the D511 working classification, and inspected the exact NA-0582 workflow runs:

- remote-handshake run `28498817017`: completed failure.
- remote-relay run `28498817988`: completed failure.

D511's factual outcome remains valid: local recovered qsl-server postcheck
passed and both remote workflows failed. D511's service-unreachable label is
qualified as provisional before artifact review because artifacts were not then
downloaded and inspected.

Corrected working classification before artifact review:
`LOCAL_READY_REMOTE_WORKFLOW_FAILED_ARTIFACTS_REQUIRED`.

Artifact-backed failure-cause classification:
`REMOTE_RELAY_ARTIFACT_BACKED_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Result classification:
`REMOTE_RELAY_TRIAGE_ARTIFACT_BACKED_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Selected successor:
`NA-0584 -- QSL Remote Relay Runner / Service Reachability Remediation Harness`.

## qwork Proof Verification

- qwork proof files were copied and verified before fetch, repository mutation,
  GitHub artifact retrieval, workflow log retrieval, SSH, or repository proof
  publication.
- Required qwork values matched lane `NA-0583`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0583/qsl-protocol`, branch `main`, upstream
  `origin/main`, `HEAD`/`origin/main` `d96ecc074abc`, clean
  worktree/index/untracked state, READY_COUNT 1, queue top READY `NA-0583`,
  shared cargo target mode, and shared target ready.
- qwork proof timestamp was verified at or after `2026-07-01T07:46:34Z`.
- Pre-fetch live `HEAD` and `origin/main` matched the qwork proof.
- Root disk usage was below the 95 percent stop threshold.
- `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1155 / D-1156 Inheritance

- D-1155 exists once and is Accepted.
- D-1156 exists once and is Accepted.
- D-1157 was absent before this implementation patch.
- NA-0582 is DONE.
- NA-0583 is READY.
- D-1155 factual workflow outcome was local ready plus both exact workflows
  failed.
- D-1155 service-unreachable classification is treated as provisional because
  artifact contents had not been inspected in D511.
- D-1156 restored NA-0583 as the sole READY item.

## D511 Classification Correction

D511 factual result remains valid:

- local recovered qsl-server postcheck passed;
- remote-handshake workflow failed;
- remote-relay workflow failed;
- raw logs/artifacts were kept proof-root-only;
- no private material was published.

D511 limitation:
`SERVICE_UNREACHABLE_AFTER_LOCAL_READY` was over-specific before artifact
inspection because safe workflow logs did not expose enough qsc relay diagnostic
fields.

Corrected working classification before artifact inspection:
`LOCAL_READY_REMOTE_WORKFLOW_FAILED_ARTIFACTS_REQUIRED`.

This decision does not rewrite D-1155. D-1157 records the correction and the
new artifact-backed result.

## Authority Model Application

- NA-0583 applies the durable D-1142/D-1143 model.
- Tier 1 redacted diagnostics are authorized for GitHub workflow metadata,
  artifacts, logs, and proof-root-only scans.
- No remote mutation is authorized.
- No workflow file mutation is authorized.
- No qsl-server start, stop, cleanup, or mutation is authorized.
- No qsc manual send/receive is authorized.
- Optional diagnostic rerun remained gated to the exact prior runs and was not
  used because artifacts were sufficient.
- Continuous CI wait-work and automatic failure-cause investigation apply.

## Current Main Required-Check Classification

- Current main: `d96ecc074abc`.
- public-safety: completed success.
- advisories: completed success.
- suite2-vectors: completed success.
- Branch-protection required contexts were green or conclusively satisfied.
- PR #1439 supplied the PR-head/aggregate proof for `goal-lint` and `CodeQL`.
- The attached failed remote-handshake check on current main was non-required
  evidence from the prior verification lane, not a branch-protection blocker.
- No failed required checks were classified before implementation.
- No required pending checks were classified before implementation.
- Root cargo audit: success.
- Nested qsc fuzz cargo audit: success after wrapper-status recovery.
- `cargo metadata --locked --format-version=1`: success.
- Cargo manifest/lock drift: absent.

## Workflow Run Metadata Review

remote-handshake:

- run ID: `28498817017`.
- workflow file: `.github/workflows/remote-handshake-tests.yml`.
- repository: `QuantumShieldLabs/qsl-protocol`.
- branch/event: `main` / `workflow_dispatch`.
- conclusion: failure.
- artifact count: present.
- job: `remote-handshake`.

remote-relay:

- run ID: `28498817988`.
- workflow file: `.github/workflows/remote-relay-tests.yml`.
- repository: `QuantumShieldLabs/qsl-protocol`.
- branch/event: `main` / `workflow_dispatch`.
- conclusion: failure.
- artifact count: present.
- job: `remote-relay`.

Run identity was unambiguous for both runs.

## Workflow Artifact Inspection

Artifacts were listed, downloaded proof-root-only, and scanned before
summarization.

remote-handshake:

- artifact availability: `ARTIFACTS_PRESENT_SCANNED_SAFE`.
- artifact download: succeeded.
- qsc relay diagnostic class: redacted relay push diagnostic present.
- timeout class: present.
- HTTP status class: present as unknown class only; no status value published.
- auth/route presence class: present, but no auth/route-token failure evidence.
- endpoint/secret mismatch evidence: absent.
- qsc runtime failure evidence: absent.
- workflow harness failure evidence: absent.
- service-unreachable evidence: present.
- raw artifact private-material class: no private material class observed.
- redaction review: pass.

remote-relay:

- artifact availability: `ARTIFACTS_PRESENT_SCANNED_SAFE`.
- artifact download: succeeded.
- qsc relay diagnostic class: redacted relay push diagnostic present.
- timeout class: present.
- HTTP status class: present as unknown class only; no status value published.
- auth/route presence class: present, but no auth/route-token failure evidence.
- endpoint/secret mismatch evidence: absent.
- qsc runtime failure evidence: absent.
- workflow harness failure evidence: absent.
- service-unreachable evidence: present.
- raw artifact publish-forbidden classes were quarantined proof-root-only and
  not copied into repository evidence.
- redaction review: pass.

## Workflow Log Re-Review

Raw logs were fetched proof-root-only and scanned before summarization.

remote-handshake:

- smoke step exit class: nonzero.
- happy-path class: present.
- qsc relay diagnostic in logs: absent.
- endpoint value disclosed: no.
- private port value disclosed: no.
- token value disclosed: no.
- unmasked Authorization value disclosed: no.
- masked GitHub auth header present: yes.
- payload/body disclosed: no.
- timeout class present: yes.
- HTTP status class present in logs: no.
- qsc runtime error class present: no.
- workflow harness failure class present: no.
- redaction review: pass.

remote-relay:

- smoke step exit class: nonzero.
- happy-path class: present.
- qsc relay diagnostic in logs: absent.
- endpoint value disclosed: no.
- private port value disclosed: no.
- token value disclosed: no.
- unmasked Authorization value disclosed: no.
- masked GitHub auth header present: yes.
- payload/body disclosed: no.
- timeout class present: yes.
- HTTP status class present in logs: no.
- qsc runtime error class present: no.
- workflow harness failure class present: no.
- redaction review: pass.

## Masked Authorization Header Scanner Correction

Scanner correction classification:
`MASKED_GITHUB_AUTH_HEADER_PRESENT`.

Observed masked GitHub checkout metadata was classified as masked:
`AUTHORIZATION: basic ***`.

No unmasked Authorization value was observed. The corrected scanner does not
classify masked GitHub checkout metadata as `AUTHORIZATION_HEADER_UNMASKED`.

## Optional Remote Postcheck

Classification:
`REMOTE_RELAY_TRIAGE_QSL_SERVER_POSTCHECK_NOT_RUN`.

Skip reason: downloaded artifacts supplied sufficient redacted relay timeout
diagnostics for classification. No SSH command was run.

## Optional Diagnostic Rerun

Classification:
`DIAGNOSTIC_RERUN_NOT_RUN_ARTIFACTS_SUFFICIENT`.

No workflow rerun was executed because artifact inspection supplied sufficient
diagnostic classes for both failed runs. No workflow dispatch, rerun, or
workflow file mutation occurred.

## Failure-Cause Investigation

Classification:
`REMOTE_RELAY_ARTIFACT_BACKED_FAILURE_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

Artifact-backed basis:

- D511/D-1155 local recovered qsl-server postcheck ready fact is preserved.
- Both exact remote workflows failed.
- Both workflow artifact sets contain redacted relay push diagnostics.
- Both diagnostics expose timeout class with unknown HTTP status/body classes.
- Route header and auth presence classes are true, but no auth/route-token
  failure evidence is present.
- Endpoint/secret mismatch evidence is absent.
- qsc runtime failure evidence is absent.
- Workflow harness failure evidence is absent.

## Private-Material Review

- Workflow artifact private-material scan: pass for publishable summaries; raw
  publish-forbidden classes remain proof-root-only.
- Workflow log private-material scan: pass.
- Metadata scan: pass.
- Optional remote postcheck scan: not run, no private material.
- Optional rerun scan: not run, no private material.
- No endpoint values are published.
- No private port values are published.
- No route-token/capability values are published.
- No bearer values or unmasked Authorization values are published.
- No private topology is published.
- No process identity is published.
- No command lines are published.
- No payloads or response bodies are published.
- No authorized_keys content or key material is published.
- No secret values are published.

## Result Classification

`REMOTE_RELAY_TRIAGE_ARTIFACT_BACKED_SERVICE_UNREACHABLE_AFTER_LOCAL_READY`.

## Selected Successor

`NA-0584 -- QSL Remote Relay Runner / Service Reachability Remediation Harness`

Status: READY

Goals: G1, G2, G3, G4, G5

Objective: remediate the artifact-backed runner/service reachability gap after
local qsl-server readiness. Codex may use redacted network/reachability
diagnostics only if explicitly authorized. Codex must not publish endpoint
values, private ports, topology, tokens, payloads, response bodies, process
identities, or key material.

## Required-Check Boundary

NA-0583 does not weaken required checks, public-safety, advisories, goal-lint,
CodeQL, branch protection, or public-safety policy. Failed remote workflows are
evidence, not treated as success.

## Source / Script Mutation Boundary

No qsl-protocol source, repository script, workflow, dependency, lockfile, qsc
runtime, qsc source, qsc tests, qsc fuzz, qsl-server source, or qsl-attachments
source was mutated.

## Workflow Mutation Boundary

No workflow file was mutated. No workflow was dispatched or rerun in NA-0583.

## Runtime / qsc Boundary

No manual qsc send/receive was run. qsc runtime/source was not mutated.
Evidence is limited to GitHub workflow artifacts/logs and safe classes.

## qsl-server / qsl-attachments Boundary

No qsl-server start, stop, cleanup, deployment, source mutation, or PR occurred.
No qsl-attachments command, clone, build, run, or mutation occurred.

## Remote-Action Boundary

No remote command was run. No SSH, scp, sudo, systemctl, service, firewall,
Tailscale, account, shell, authorized_keys, ps, ss, netstat, lsof, qsc, or
qsl-attachments command was run.

## Public-Site / Cloudflare Boundary

No public-site or Cloudflare action occurred.

## Raw Output Boundary

Raw workflow artifacts and logs remain proof-root-only. Repository evidence
publishes only coarse classes, workflow names, run IDs, check/job names, and
redacted summaries.

## Claim Boundary

NA-0583 makes no public-readiness, production-readiness, public-internet-
readiness, external-review-complete, vulnerability-free, bug-free,
perfect-build, or perfect-crypto claim.

## Validation

Required validation is recorded in the proof root. It includes scope guard,
queue/decision proof, marker proof, link-check, private-material scans,
overclaim scan, docs/governance-only classifier, PR body preflight, goal-lint,
cargo audits, locked metadata, formatting, and qsc-adversarial shell syntax.

## Recommendation

Proceed to the selected NA-0584 runner/service reachability remediation
successor after NA-0583 closeout. The successor should use only redacted
diagnostics unless a later directive authorizes a narrower operational action.
