Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-30

# NA-0576 Project-Wide Bounded Codex Operational Authority Hardening

## Executive Summary

NA-0576 consumes D-1140 and D-1141, accepts the operator request for smoother
project-wide workflow, and makes the bounded Codex operational authority model
durable in tracked governance. Result classification:
`PROJECT_WIDE_BOUNDED_CODEX_OPERATIONAL_AUTHORITY_HARDENING_PASS`.

No remote action, qsl-server start, qsc send/receive, workflow dispatch/rerun,
qsl-attachments work, runtime/source/dependency mutation, or private-material
publication occurred in NA-0576.

## qwork Proof Verification

Fresh qwork proof files from `2026-06-30T16:14:11Z` or later were copied from
the NA-0576 lane workspace and verified before fetch or repository mutation.
Required values passed for lane `NA-0576`, repo `qsl-protocol`, path
`/srv/qbuild/work/NA-0576/qsl-protocol`, branch `main`, upstream `origin/main`,
clean worktree/index/untracked state, READY count 1, queue top READY NA-0576,
shared target mode, and shared target readiness.

Live pre-fetch `HEAD` and `origin/main` matched the qwork proof at
`16faab413e60`. Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1140 / D-1141 Inheritance

D-1140 exists once and is Accepted. It records the D501 bounded Codex
operational authority model and result classification
`QSL_SERVER_BIND_START_INSUFFICIENT_PROOF`.

D-1141 exists once and is Accepted. It closes NA-0575 and restores NA-0576 as
the sole READY item. D-1142 and D-1143 were absent before the NA-0576 patch.

## Operator Request and Rationale

The operator requested smoother project-wide workflow without unlimited
authority. NA-0576 therefore converts the D-1140 bounded authority model into a
durable policy and agent-facing rule set before deeper technical qsl-server
lanes continue.

## Current Main Required-Check Classification

Current main `16faab413e60` satisfied public-safety, advisories, and
suite2-vectors. Required branch-protection contexts had no failed or pending
required checks after D498 visibility recovery for PR-scoped `goal-lint` and
aggregate CodeQL visibility. Root cargo audit, nested qsc fuzz cargo audit,
locked metadata, formatting, shell syntax, and Cargo drift checks passed.

## Bounded Codex Operational Authority Model

The durable model is recorded in
`docs/ops/CODEX_BOUNDED_OPERATIONAL_AUTHORITY.md`. It is project-wide policy
for future directives and does not grant default remote action.

## Authority Tiers

The five tiers are Tier 0 governance/read-only, Tier 1 redacted diagnostics,
Tier 2 bounded test action, Tier 3 operator/admin action, and Tier 4 forbidden
action.

## Directive Opt-In Requirements

Tier 1 and Tier 2 require an active directive to name the authority tier,
host/workspace or local path, exact command family, allowed mutation paths,
raw-output quarantine path, redaction/publication policy, rollback/manifest
requirements, private-material scan, stop conditions, and final response claim
boundaries.

## Approved Test Host / Workspace Registry

The current registry contains host label `inspiron`, workspace
`/home/qslcodex/qsl-remote-test/`, purpose QSL/QSC remote relay testing, and
allowed tiers only when a directive opts in: Tier 1 and Tier 2. It grants no
blanket sudo/systemd/firewall/Tailscale/account authority.

## Redacted Evidence Policy

Repository evidence may publish safe classes and enums only. Raw outputs,
private values, endpoint values, private ports, topology, tokens,
Authorization material, process identities, payloads, response bodies,
authorized_keys content, raw private logs, key material, secret env values, and
API keys must not be published.

## Raw Output Quarantine Policy

Raw outputs, scripts, logs, parsed private JSON, and private scans remain in
the proof root or a directive-named quarantine path. Tracked files may contain
only safe summaries, classifications, markers, and validation status.

## Tier 1 Diagnostic Policy

Tier 1 permits exact read-only commands or SSH-stdin scripts on named
test hosts/workspaces only when a directive opts in. Output is reduced to safe
classes before publication.

## Tier 2 Bounded Test Action Policy

Tier 2 permits exact no-secret, non-root, non-privileged, reversible test
actions in approved test workspaces only when all preflight, postcheck,
rollback/manifest, and private-material gates are named and pass.

## Tier 3 Operator/Admin Boundary

sudo, systemd, firewall, Tailscale, account, shell, authorized_keys,
root-owned service, backup, and privileged operator actions remain
operator-owned unless a later directive explicitly authorizes a privileged
lane.

## Tier 4 Forbidden Boundary

Secret publication, destructive unbounded mutation, workflow weakening,
out-of-scope protocol/crypto/security semantic changes, and public/production
security overclaims are forbidden.

## START_HERE Update

`START_HERE.md` now points to the full authority runbook and records that
operational tiers require active directive opt-in while preserving qwork,
one-READY, evidence, public-safety, advisories, and claim gates.

## AGENTS Update

`AGENTS.md` now contains agent-facing rules: do not infer authority, keep raw
outputs proof-root-only, publish safe classes only, stop before unauthorized
Tier 3/Tier 4 actions, do not run `qwork`/`qstart`/`qresume`, do not publish
private material, and do not weaken checks or claims.

## qsl-server Technical Thread Preservation

The qsl-server dependency audit recovered. qsl-server was staged on
`inspiron`. qsl-server start was skipped. D501 inspection remained
insufficient. The next technical work must complete start/bind proof through
the newly formalized authority model.

## Selected Successor

Selected successor after hardening:
`NA-0577 -- QSL Remote qsl-server Start / Bind Proof Completion Harness`.

## Required-Check Boundary

NA-0576 only read GitHub metadata and local validation output. No workflow
dispatch or rerun occurred. Required-check handling used bounded visibility
recovery only for known PR-scoped or aggregate contexts.

## Continuous CI Wait-Work Amendment

D-1142 is consumed by this amendment. D502 implementation PR #1425 merged at
`c6f4cacde1d6`. D502 post-merge public-safety, advisories, suite2-vectors,
and visible check-runs were verified green before repository mutation. D503
stopped before mutation because D502 public-safety was still nonterminal, and
the operator rejected idle CI waits.

The authority runbook now requires productive current-lane/proof-root-safe work
during nontrivial CI/check waits when such work exists. Examples include
metadata capture, response drafting, if-green/if-red planning, post-merge proof
preparation, remaining allowed validation, proof-root parser/scanner/classifier
hardening, read-only forward audits, finding triage, proof-root logging, and
final-response wait-work reporting. Passive polling alone is not productive
work.

Read-only forward audits are allowed during waits when they remain inside the
active directive boundary. The allowed categories cover crypto/protocol
invariants, qsc client/runtime risks, qsl-server relay risks, qsl-attachments
boundary risks, dependency/advisory watch, workflow/CI reliability,
public-claim safety, private-material/redaction safety, test coverage gaps,
runbook/operator friction, governance/queue consistency, and
build/provenance/reproducibility risks. Findings are classified as
`CRITICAL_BLOCKER`, `HIGH_VALUE_FIX`, `MEDIUM_RISK`, `LOW_RISK`,
`FALSE_POSITIVE`, or `NO_ACTION`.

The amendment preserves the prohibition on next-lane implementation,
speculative PRs, mutation outside current scope, closeout before green gates,
`qwork`/`qstart`/`qresume`, check weakening, treating pending/failed CI as
success, private-material publication, and public/production/security
overclaims during waits.

## Source / Script Mutation Boundary

No qsl-protocol runtime source, repository script, workflow, dependency, or
lockfile mutation occurred.

## Workflow Mutation Boundary

No workflow files changed. No workflow dispatch or rerun occurred.

## Runtime / qsc Boundary

No qsc command, qsc send/receive, or E2EE action occurred.

## qsl-server / qsl-attachments Boundary

No qsl-server start, run, deployment, source mutation, or PR occurred. No
qsl-attachments command, clone, build, run, or mutation occurred.

## Remote-Action Boundary

No SSH, scp, Tailscale, remote command, qsl-server start, qsl-server
deployment, remote probe, or remote write occurred in NA-0576.

## Public-Site / Cloudflare Boundary

No public-site, website, public docs publication outside exact scope, or
Cloudflare mutation occurred.

## Private-Material Review

Added tracked text was reviewed for private-material publication. It contains
only approved host/workspace labels, public PR/decision references, safe
classes, and policy boundaries.

## Claim Boundary

NA-0576 makes:

- no public-readiness claim;
- no production-readiness claim;
- no vulnerability-free claim;
- no bug-free claim;
- no perfect-build claim;
- no perfect-crypto claim.

## Validation

Validation covers qwork proof, queue/decision proof, D-1140/D-1141 inheritance,
required-check classification, scope guard, markers, link-check,
private-material scan, overclaim scan, docs/governance-only classification, PR
body preflight, goal-lint when available, cargo audits, locked metadata,
formatting, and shell syntax.

Post-fix hardening review:

1. Correctness under stress: the policy fails closed on missing opt-in,
   ambiguous authority, private material, and unknown Tier 2 gates.
2. Minimality: changes are limited to governance, runbook, evidence, testplan,
   traceability, and rolling journal paths.
3. Maintainability: the authority model is centralized in one runbook with
   short pointers from START_HERE and AGENTS.
4. Coverage quality: markers and scans cover the authority tiers, opt-in
   requirements, continuous CI wait-work, read-only forward audits, qsl-server
   successor preservation, and forbidden-action boundaries.
5. Cross-lane stability: no macOS/Linux runtime, qsc, qsl-server,
   qsl-attachments, workflow, dependency, or lockfile behavior changed.

## Recommendation

Merge NA-0576 hardening, then close out to NA-0577 so qsl-server start/bind
proof completion can resume under the D-1142 authority model without widening
remote authority or publishing private values.
