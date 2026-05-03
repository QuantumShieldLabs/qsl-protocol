Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03
Replaces: n/a
Superseded-By: n/a

# NA-0246 Closeout and NA-0247 Restoration Test Plan

## Objective

Validate that NA-0246 is closed only from merged one-command demo acceptance evidence and that NA-0247 is restored as the sole READY successor for bounded desktop GUI prototype validation and public demo readiness.

## Protected Invariant

The queue must contain exactly one READY item after closeout: NA-0247. NA-0247 must remain desktop GUI validation/readiness, not production release, website implementation, protocol wire work, qsl-server work, qsl-attachments work, public-safety helper/config work, Cargo work, KT work, or SCKA work.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0246_closeout_restore_na0247_testplan.md`

Forbidden proof:

- no `.github/**` changed
- no `scripts/**` changed
- no Cargo manifests or lockfiles changed
- no qsp/qsc/qsl/qsl-client/apps/tools/inputs implementation paths changed
- no qsc-desktop/qsl-server/qsl-attachments/website files changed
- no protocol/runtime/crypto/demo/service semantics changed
- no public-safety helper/config or branch-protection settings changed

## Packet A Evidence Requirement

NEXT_ACTIONS.md must record:

- PR #740
- Packet A head `9ae30e5373c5`
- Packet A merge `94f17b99a180`
- D-0458
- D-0459
- `DEMO_ACCEPTANCE_OK`
- `DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK`
- negative reject markers
- no-token leak proof marker
- non-production posture marker
- post-merge public-safety required/green proof

## Queue Parser Expectation

Run the canonical queue parser.

Expected:

- READY_COUNT 1
- READY NA-0247
- NA-0246 DONE
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

- D-0110 exists once
- D-0439 through D-0459 exist once each
- duplicate decision count is zero

## NA-0247 Successor Validation

NEXT_ACTIONS.md must state that NA-0247:

- is titled "Desktop GUI Prototype Validation and Public Demo Readiness";
- is READY;
- has Goals G1, G4, G5;
- allows no wire/behavior change;
- allows no crypto/state-machine change;
- is not docs-only;
- requires executable validation/build evidence or screenshot/transcript artifacts generated from the current GUI surface;
- protects qsc sidecar-shell boundaries, memory-only child-scoped passphrase handling, deferred keychain active ops, truthful `protocol_inactive`, handshake/session-establish out-of-scope status, and no production-readiness overclaim;
- does not authorize `.github`, public-safety helper/config, Cargo, qsl-server, qsl-attachments, website, protocol-core, KT, SCKA, or branch-protection changes.

## CI Expectations

Local validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Also run goal-lint, queue parser, decision parser, markdown inventory/link validation, changed-path scope guard, forbidden-path guard, and leak-safe scan using established repository patterns.

Required CI:

- public-safety remains required and green;
- all required protected contexts attach and pass or are accepted according to repository rules;
- no branch-protection exception, admin bypass, direct push, squash merge, rebase merge, or check spoofing.
