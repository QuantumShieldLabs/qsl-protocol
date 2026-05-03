Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0245 Website Truthfulness Audit Test Plan

## Objective

Validate that NA-0245 produced a docs-only website truthfulness / repo-sync audit and update plan before any website implementation changes.

## Protected Invariant

Public claims must match repo evidence. Research-stage, non-production, metadata, demo, GUI, qsl-server, and qsl-attachments boundaries must remain explicit.

## Scope Guard

Allowed changed paths:

- docs/public/WEBSITE_CLAIM_MATRIX.md
- docs/public/WEBSITE_UPDATE_PLAN.md
- docs/governance/evidence/NA-0245_website_truthfulness_audit.md
- DECISIONS.md
- TRACEABILITY.md
- tests/NA-0245_website_truthfulness_audit_testplan.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md

Forbidden proof:

- no website source files changed
- no `.github/**` changed
- no `scripts/**` changed
- no Cargo manifests or lockfiles changed
- no qsc/qsl/qsl-client/apps/tools/qsc-desktop/qsl-server/qsl-attachments implementation paths changed
- no protocol/runtime/crypto/demo/service semantics changed
- no public-safety helper/config or branch-protection settings changed

## Website Pages Checked

The audit must record exact public pages checked and retrieval timestamps for at least:

- https://quantumshieldlabs.dev/
- https://quantumshieldlabs.dev/blog/
- https://quantumshieldlabs.dev/quantum-risk-calculator/
- public linked project/resource pages sufficient to classify CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, and external evidence links

The audit must state that no login, purchase, or form submission occurred.

## Claim Matrix Validation

docs/public/WEBSITE_CLAIM_MATRIX.md must include:

- Goals line
- status/classification header
- source URL / page title / retrieval timestamp
- claim excerpt or concise paraphrase
- category
- classification from the allowed taxonomy
- repo evidence where applicable
- recommended action
- priority
- notes / uncertainty

It must cover:

- production / live / not-demo claims
- protocol / quantum / PQC claims
- Suite-2 / Triple-Ratchet-style boundary
- demo / app / GUI claims
- metadata / privacy / anonymity / minimization claims
- qsl-server / relay / attachment / service claims
- CrawDaddy / agent economy claims
- healthcare / PQC consulting claims
- NIST / deadline / compliance claims
- public roadmap / journey claims
- GitHub/code/evidence links

## Update Plan Validation

docs/public/WEBSITE_UPDATE_PLAN.md must include:

- Goals line
- executive summary
- recommended site information architecture
- homepage update plan
- projects/tools update plan
- QSL protocol status section
- demo/GUI status section
- metadata/privacy language section
- Triple-Ratchet / Suite-2 claim boundary section
- healthcare/PQC consulting claim boundary section
- CrawDaddy/SELARIX separation guidance
- evidence-links plan
- implementation backlog
- no-production-overclaim language
- next recommended implementation lane

## No Website Implementation Change Proof

Run:

```bash
git diff --name-only origin/main...HEAD
```

Expected: only allowed NA-0245 docs/governance/testplan/journal paths.

## Queue Parser Expectation

Run the canonical queue parser.

Expected:

- READY_COUNT 1
- READY NA-0245
- NA-0244 DONE
- NA-0243 DONE
- NA-0242 DONE
- NA-0241 DONE
- NA-0240 DONE
- NA-0239 DONE
- NA-0238 DONE
- NA-0237 DONE

## Decision Parser Expectation

Run the canonical decision parser.

Expected:

- D-0456 exists once after Packet A patch
- D-0457 absent during Packet A
- D-0110 exists once
- D-0439 through D-0455 exist once each
- duplicate decision count is zero

## CI Expectations

Local validation:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- goal-lint
- queue parser
- decision parser
- markdown inventory / link validation
- leak-safe scan

Required CI:

- public-safety remains required and green
- all required protected contexts attach and pass or are accepted according to repository rules
- no branch-protection exception, admin bypass, direct push, squash merge, rebase merge, or check spoofing
