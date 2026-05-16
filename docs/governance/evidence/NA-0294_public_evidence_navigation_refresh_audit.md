Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0294 Public Evidence Navigation Refresh Audit

## Executive Summary

NA-0294 improves QSL's public first impression and evidence navigation through
README, START_HERE, and docs/public updates. The refresh applies the NA-0290A
public attention strategy while preserving evidence-bound claim discipline.

This lane changes public navigation and governance documentation only. It does
not change protocol, crypto, runtime, service, demo, website, workflow, Cargo,
dependency, branch-protection, or public-safety configuration.

## Sources Inspected

- [README.md](../../../README.md)
- [START_HERE.md](../../../START_HERE.md)
- [docs/public/INDEX.md](../../public/INDEX.md)
- [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [docs/public/EXTERNAL_REVIEW_PACKAGE.md](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md](../../public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md)
- [docs/governance/evidence/NA-0290A_public_attention_visibility_audit.md](NA-0290A_public_attention_visibility_audit.md)
- [docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md](NA-0291_metadata_phase2_identifier_padding_harness.md)
- [docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md](NA-0293_metadata_phase2_sanitized_errors_retention_harness.md)
- [docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md](NA-0287_service_production_gate_evidence_map.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [DECISIONS.md](../../../DECISIONS.md)
- [NEXT_ACTIONS.md](../../../NEXT_ACTIONS.md)
- Targeted listings under [docs/demo](../../demo/) and [docs/design](../../design/).

## README Baseline

The README already stated that QSL is research-stage and not production-ready.
It also linked to START_HERE, docs/public, the release evidence map, the
external review package, and demo acceptance criteria.

The gap was first-screen clarity. A casual reader could find the evidence, but
the most shareable hook and the fastest claim-to-evidence route were not yet
packaged as the primary landing experience.

## START_HERE Baseline

START_HERE was strong as an operational constitution and fail-closed workflow
document. It identified authoritative sources, governance priority, and
mandatory workflow.

The gap was audience routing. Reviewers, demo runners, contributors, and public
supporters had to infer their first three steps from the operational process.

## docs/public Index Baseline

docs/public/INDEX.md existed and carried the canonical public/release posture:
research-stage, public for review, not production-ready, fail-closed posture,
demo placeholders, trust/delivery semantics, receipt policy, and multi-device
policy.

The gap was landing-page depth. It did not yet group evidence by reader need,
demo evidence, service evidence, metadata phase-2 evidence, external review
package, visibility strategy, claim boundaries, and ways to help.

## Changes Made

- README now leads with the NA-0290A evidence hook, a clear research-stage and
  not-production-ready boundary, and a compact "Start With The Evidence" route.
- README now separates "What This Is", "What This Is Not", "What Is Proven
  Now", "What Is Not Proven Yet", and "How To Review Or Help".
- START_HERE now includes audience entry points for fast public overview,
  evidence inspection, demos, security/claim review, and contribution.
- docs/public/INDEX.md now functions as a public evidence landing page with
  sections for what QSL is, what it is not, evidence map, demo evidence, service
  evidence, metadata phase-2 evidence, external review package, public
  visibility strategy, claim boundaries, and how to help.
- RELEASE_READINESS_EVIDENCE_MAP.md and EXTERNAL_REVIEW_PACKAGE.md now point to
  the NA-0294 navigation refresh while preserving all `NOT_READY` boundaries.
- DECISIONS.md records D-0564 and TRACEABILITY.md links the refresh.

## Claim-Boundary Scan Summary

High-risk public phrases were reviewed in the changed content. Matches are
allowed only where they are negated, listed as prohibited, marked `NOT_READY`,
or explicitly framed as future/unproven.

The refresh does not introduce affirmative claims of production readiness,
public internet service readiness, external review completion, anonymity,
metadata-free messaging, untraceability, runtime metadata phase-2 completion,
production service deployment, or website implementation.

## What Improved

- The public hook is visible in the README first screen.
- Reviewers can move directly from public overview to evidence maps, demo
  evidence, service-boundary evidence, metadata phase-2 evidence, traceability,
  decisions, and the active queue.
- Non-production demo proof, local service-hardening evidence, and metadata
  phase-2 fixture proof are easier to find without overstating them.
- START_HERE remains operational while giving non-maintainer audiences a
  shorter safe path.

## What Did Not Change

- No protocol behavior changed.
- No wire behavior changed.
- No cryptographic state-machine behavior changed.
- No qsl-server or qsl-attachments implementation changed.
- No qsc-desktop implementation changed.
- No website or external website changed.
- No workflow, script, Cargo, dependency, branch-protection, or public-safety
  configuration changed.
- No production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, or untraceable claim was added.

## No Implementation Changes

Changed paths are public documentation, governance records, and this testplan /
audit evidence only. Runtime, protocol, service, demo, desktop, app, tool,
input, formal, Cargo, workflow, and website paths remain untouched.

## No Website Changes

NA-0294 does not edit `website/**`, an external website repository, live site
copy, repository topics, social previews, or public posting channels. Website
implementation remains future-gated.

## Remaining Public Visibility Follow-Ups

- Prepare a website landing page handoff and evidence-visual plan without
  mutating website sources.
- Build shareable evidence visuals that map claim -> evidence -> current
  boundary.
- Keep metadata phase-2, external review, service production gates, and public
  internet readiness visible until separate evidence closes them.

## Next Recommended Lane

Close out NA-0294 after this PR merges and post-merge public-safety is green,
then restore NA-0295 for website landing page handoff and evidence-visuals
planning. NA-0295 should remain planning-only and must not mutate website or
external website sources.
