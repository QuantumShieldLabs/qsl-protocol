Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0245 Closeout and NA-0246 Restoration Test Plan

## Objective

Validate that NA-0245 is closed only from merged Packet A evidence and that NA-0246 is restored as the sole READY successor for executable one-command public demo acceptance.

## Protected Invariant

The queue must contain exactly one READY item after closeout: NA-0246. NA-0246 must remain executable demo acceptance, not website implementation, protocol wire work, qsl-server work, qsl-attachments work, qsc-desktop work, or public-safety helper/config work.

## Scope Guard

Allowed changed paths:

- NEXT_ACTIONS.md
- DECISIONS.md
- TRACEABILITY.md
- docs/ops/ROLLING_OPERATIONS_JOURNAL.md
- tests/NA-0245_closeout_restore_na0246_testplan.md

Forbidden proof:

- no website source files changed
- no `.github/**` changed
- no `scripts/**` changed
- no Cargo manifests or lockfiles changed
- no qsc/qsl/qsl-client/apps/tools/qsc-desktop/qsl-server/qsl-attachments implementation paths changed
- no protocol/runtime/crypto/demo/service semantics changed
- no public-safety helper/config or branch-protection settings changed

## Packet A Evidence Requirement

NEXT_ACTIONS.md must record:

- PR #738
- Packet A head `0eb0149456be`
- Packet A merge `ab4c7f753f1c`
- D-0456
- D-0457
- no website implementation changes
- post-merge public-safety required/green proof

## Queue Parser Expectation

Run the canonical queue parser.

Expected:

- READY_COUNT 1
- READY NA-0246
- NA-0245 DONE
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

- D-0456 exists once
- D-0457 exists once
- D-0110 exists once
- D-0439 through D-0455 exist once each
- duplicate decision count is zero

## NA-0246 Successor Validation

NEXT_ACTIONS.md must state that NA-0246:

- is titled "One-Command Public Demo Acceptance Runner"
- is READY
- has Goals G1, G3, G4, G5
- is not docs-only
- must include an executable one-command demo acceptance runner or CI target
- must protect non-production demo honesty, loopback-only default, relay authorization, positive flow proof, negative fail-closed cases, and qsl-server/qsl-attachments boundaries
- does not authorize website implementation
- does not authorize `.github`, public-safety helper/config, Cargo, qsc-desktop, qsl-server, qsl-attachments, protocol-core, KT, SCKA, or branch-protection changes

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
