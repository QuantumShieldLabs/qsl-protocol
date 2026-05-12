Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0272 qsl-server Contract Repair and Harness Prep

Directive: QSL-DIR-2026-05-12-074 / NA-0272

## Executive Summary

NA-0272 repaired qsl-server documentation/API-contract drift in the sibling
qsl-server repository, then records qsl-protocol evidence and harness prep for
the first executable qsl-server hardening suite.

The qsl-server repair was documentation/API-contract only. It did not change
qsl-server implementation code, service semantics, Cargo files, dependencies,
workflows, scripts, deployment behavior, qsl-protocol runtime behavior,
qsl-attachments implementation, protocol/wire/crypto behavior, or branch
protection. This evidence does not make a production readiness claim.

## qsl-server PR Evidence

- Repository: `QuantumShieldLabs/qsl-server`
- PR: #47, `NA-0272: repair qsl-server docs API contract`
- PR URL: https://github.com/QuantumShieldLabs/qsl-server/pull/47
- Head SHA: `4024897e80dc`
- Merge SHA: `03e3511e328b`
- Required check: `rust` completed successfully before merge.
- Merge method: normal merge commit with exact head-SHA match.
- Changed docs:
  - `README.md`
  - `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`
  - `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`
  - `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`
  - `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`
  - `tests/NA-0003_relay_inbox_contract_plan.md`
  - `tests/NA-0004_relay_auth_hardening_plan.md`
  - `tests/NA-0011_relay_compatibility_restore_evidence.md`
- Implementation changes: none. No `src/**`, Rust tests, Cargo files,
  workflows, scripts, or packaging/runtime paths changed.

## Contract Drifts Repaired

### Queue Full Error

Current qsl-server code and tests return `429 ERR_OVERLOADED` when a push is
rejected because the queue is full. PR #47 aligned stale documentation that
still used `ERR_QUEUE_FULL`.

### Pull Response Shape

Current qsl-server pull success returns JSON with an `items` array containing
message identifiers and byte arrays. PR #47 corrected README wording that
described a raw oldest-message-byte response.

### Legacy Route Behavior

Current qsl-server registers only the canonical token-free URL paths and uses a
header-carried route token. Legacy path-token routes are retired: push returns
404 without queue mutation and pull returns 404 without consuming canonical
queue items. PR #47 aligned DOC-SRV-005 and historical test/evidence wording to
that current retired-route behavior.

### Optional Relay Auth Stance

Current qsl-server supports optional relay bearer authentication through
`RELAY_TOKEN`. When configured, missing or invalid bearer auth is rejected with
`401 ERR_UNAUTHORIZED` before queue mutation. When unset or empty, relay auth is
disabled while route-token header checks still apply. PR #47 updated the stale
deployment-contract wording that said there was no relay-layer authentication.

### x-msg-id and Idempotency Wording

Current qsl-server accepts `x-msg-id` as a client-supplied identifier, but code
appends every accepted push as a new queue item. There is no duplicate
suppression or idempotent replay handling in current tests/code. PR #47 removed
idempotency wording from the current contract and marks duplicate/idempotency
semantics as a future service decision.

### Config Fallback and Future Fail-Closed Decision

Current qsl-server treats invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` values
as unset and caps values above built-in ceilings. PR #47 documented that current
fallback/capping behavior and marks fail-closed startup for invalid size/depth
config as future hardening work.

## Remaining Service Semantic Decisions

- Whether duplicate `x-msg-id` pushes should remain independent queue entries
  or become idempotent.
- Whether invalid `MAX_BODY_BYTES` and `MAX_QUEUE_DEPTH` values should fail
  startup instead of falling back to defaults.
- Whether wrong bearer auth should be proven against pre-existing queue state
  in the first hardening suite.
- Whether non-UTF-8 route-token headers and bad `max` pull queries need
  additional canonical reject taxonomy.
- Whether qsl-server should gain health/ops endpoints or keep deployment-level
  probes only.
- Whether future rate-limit, route-count cap, TTL, persistence, and global
  queue-limit behavior should be added as service semantics.

These are future implementation or policy decisions. NA-0272 does not implement
or claim them.

## First Executable Hardening Harness Prep

The first qsl-server hardening harness should be a sibling qsl-server
executable suite that starts the current Axum app locally, drives HTTP requests
against loopback, captures application logs, and asserts both HTTP behavior and
state/logging invariants.

Required harness categories:

- Auth/reject/no-mutation tests:
  - auth disabled allows canonical push/pull;
  - missing bearer auth rejects before queue mutation;
  - wrong bearer auth rejects before queue mutation;
  - rejected auth against a pre-existing queue item does not consume or alter
    that item;
  - auth reject responses do not expose bearer values, route tokens, or
    payload bytes.
- Route-token tests:
  - missing route-token header rejects without mutation;
  - empty route-token header rejects without mutation;
  - malformed/non-UTF-8 route-token header rejects deterministically;
  - route-token header remains separate from bearer auth.
- Queue/overload tests:
  - queue cap rejects with `ERR_OVERLOADED`;
  - rejected overload push does not append payload;
  - accepted item remains retrievable after rejected overload push;
  - overload logs contain bounded structured metadata only.
- Pull JSON response tests:
  - non-empty pull returns JSON `items`;
  - multiple items are bounded by `max`;
  - `max=0` rejects deterministically;
  - successful pull deletes delivered items.
- Legacy route 404/no-mutation tests:
  - legacy path push returns 404 and does not mutate canonical queue state;
  - legacy path pull returns 404 and does not consume canonical queue state.
- x-msg-id duplicate semantics tests:
  - duplicate identifiers are either proven as independent queue entries or,
    if policy changes, proven idempotent with no drift in docs;
  - generated message identifiers remain non-empty when header is absent.
- Logging/no-secret tests:
  - payload bytes absent from success and reject logs;
  - raw route tokens absent from success, overload, and reject logs;
  - bearer values absent from auth reject logs;
  - no long secret-like hex dumps in retained harness artifacts.
- Config/startup tests:
  - default limits and loopback bind are explicit;
  - invalid port env fails startup;
  - current invalid body/depth fallback is either proven as current behavior or
    replaced by a separately authorized fail-closed implementation with tests;
  - public bind remains explicit opt-in.

## Acceptance Gates for NA-0273

NA-0273 should not claim completion until:

- qsl-server executable harness exists in the qsl-server repo;
- auth/reject/no-mutation/logging tests run and pass;
- route-token and retired legacy route behavior are test-backed;
- queue overload and pull JSON behavior are test-backed;
- duplicate `x-msg-id` semantics are explicitly tested as current behavior or
  changed under a separate, test-backed semantic decision;
- config/startup behavior is tested and documented truthfully;
- qsl-server implementation changes, if any, are scoped only to NA-0273 and
  backed by same-PR tests;
- qsl-protocol evidence records the qsl-server PR head/merge and validation;
- no production readiness, deployment readiness, metadata elimination,
  anonymity, untraceability, or external-review-complete claim is introduced;
- qsl-protocol public-safety remains required and green.

## Non-Production Boundary

NA-0272 is documentation/API-contract repair and harness preparation. It does
not deploy qsl-server, expose a public service, approve production relay use,
change service semantics, change qsl-protocol runtime behavior, or certify
qsl-server for production. Known qsl-server gaps remain visible until a future
executable hardening lane proves them closed.
