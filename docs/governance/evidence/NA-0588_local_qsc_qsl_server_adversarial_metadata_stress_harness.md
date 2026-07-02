Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0588 Local qsc / qsl-server Adversarial and Metadata Stress Harness

## Executive Summary

NA-0588 consumed D-1165 and D-1166, verified fresh qwork proof, acquired and
validated qsl-server locally, reviewed qsc local relay command surfaces, and ran
a proof-root-only local stress harness against a loopback qsl-server relay.

Result classification:
`LOCAL_QSC_QSL_SERVER_E2EE_ADVERSARIAL_METADATA_STRESS_PASS`.

Selected successor:
`NA-0589 -- QSL Local qsl-attachments Integration Readiness Harness`.

## qwork Proof Verification

- Fresh qwork proof files were copied into the proof root before fetch,
  repository mutation, qsl-server acquisition, qsl-server start, qsc command
  execution, source-analysis result publication, GitHub metadata review, or
  proof publication.
- qwork proof verified lane `NA-0588`, repo `qsl-protocol`, path
  `/srv/qbuild/work/NA-0588/qsl-protocol`, clean worktree/index/untracked
  state, READY_COUNT 1, queue top READY `NA-0588`, and shared cargo target
  readiness.
- Proof timestamp `2026-07-02T02:30:51Z` was accepted.
- Pre-fetch live `HEAD` and `origin/main` matched qwork proof at
  `6d62fbc3bdd6`.
- Root disk and `/backup/qsl` were below the 95 percent stop threshold;
  `/backup/qsl` was mounted.
- Codex did not run `qwork`, `qstart`, or `qresume`.

## D-1165 / D-1166 Inheritance

- D516 response and NA-0587 proof artifacts were consumed.
- D-1165 exists once and is Accepted.
- D-1165 result classification:
  `LOCAL_CLIENT_RELAY_E2EE_INTEGRATION_PASS`.
- D-1165 selected NA-0588 as the local qsc/qsl-server adversarial and metadata
  stress harness.
- D-1166 exists once and is Accepted.
- D-1166 restored NA-0588 as the sole READY item.
- NA-0587 and NA-0586 are DONE; NA-0588 is READY.
- D-1167 was absent before this implementation patch.

## Authority Model Application

NA-0588 used the D-1161-expanded issue-investigation and safe-fix authority.
Source review, proof-root harness generation, local loopback qsl-server start,
local qsc command execution, and proof-root-only scanners were in scope. No
qsl-protocol qsc source fix and no qsl-server source fix was selected.

Recovered-failure evidence was recorded for command-shape and proof-tooling
issues that were fixed in place:

- Current-main required-check classification needed visibility recovery for
  goal-lint and CodeQL naming. Final result: PASS.
- qsl-server source search included a non-existent examples path. Final result:
  PASS after rerunning over existing paths.
- qsc source search included a non-existent examples path. Final result: PASS
  after rerunning over existing paths.
- Proof-root harness generation initially missed a generator import for the
  chmod step. Final result: PASS after proof-root-only repair.

## qsl-server Source Review

- qsl-server source was acquired under the NA-0588 workspace and reviewed at
  `6bf61d439fa2`.
- qsl-server worktree was clean before and after validation cleanup.
- Binary target: `qsl-server`.
- Route surface: canonical push and pull route shapes only.
- Route-token carriage: header-based.
- Optional bearer behavior: enabled by local configuration and fail-closed when
  missing or wrong.
- Default bind was loopback class.
- Queue semantics reviewed from source/tests: in-memory relay queue with no
  persistence guarantee.
- Body/log behavior reviewed: payloads are treated as opaque relay items; source
  review found no intentional payload, token, Authorization, or key-material
  logging.

## qsl-server Build / Audit / Test

qsl-server validation passed locally:

- `cargo metadata --locked --format-version=1`: PASS.
- `cargo audit --deny warnings`: PASS.
- `cargo fmt --check`: PASS.
- `cargo test --locked`: PASS.
- `cargo build --locked`: PASS.

Generated build output in the qsl-server checkout was cleaned after validation,
and qsl-server source remained unmodified.

## qsc Source and Command Discovery

- qsc source and help output were reviewed for local relay send/receive,
  local relay inbox setup, endpoint handling, route-token handling, bearer
  handling, diagnostic classes, and metadata surfaces.
- qsc build passed.
- Focused qsc relay tests passed.
- qsc local relay commands were usable for proof-root-only send/receive and
  E2EE validation against local qsl-server.
- qsc emitted actionable safe failure classes in the tested relay negatives.
- No qsc source/test/helper mutation was needed.

## Local Stress Harness Design

The local stress harness was generated under the proof root only. Static review
passed:

- qsl-server bound to loopback class only.
- Runtime, qsc state, qsl-server logs, fixtures, and raw outputs were
  proof-root-only.
- Route-token and optional bearer values were non-secret local fixtures and were
  not published.
- subprocess execution used argument lists rather than shell expansion.
- qsl-server process ownership and cleanup were explicit.
- No public bind, remote/Tailscale/SSH/scp action, workflow dispatch/rerun,
  qsl-attachments invocation, qsl-backup, sudo, systemd, firewall mutation,
  `qwork`, `qstart`, or `qresume` appeared in the harness.

## Baseline Revalidation

Baseline local route/E2EE revalidation passed:

- qsl-server local readiness: `BASELINE_LOCAL_QSL_SERVER_READY`.
- qsl-server route shape and auth fail-closed baseline passed.
- qsc local relay push/pull passed.
- qsc E2EE send/receive over the local relay passed.
- Empty pull after drain returned the no-item class.

## Repetition Stress

Repetition stress passed:

- Ten sequential E2EE send/receive cycles ran with fresh local state per cycle.
- Each cycle produced the expected receive class.
- No stale-state reuse or misleading success after failed push/pull was
  detected.
- Classification: `REPETITION_STRESS_PASS`.

## Multi-Message / Ordering / Empty Queue

Multi-message and empty-queue stress passed:

- Multiple direct relay items were pushed and pulled.
- Multiple qsc relay messages were sent and received.
- Ordering was preserved in the tested local queue.
- Empty pull after drain returned the no-item class.
- Classification: `MULTI_MESSAGE_ORDERING_PASS` and
  `EMPTY_PULL_AFTER_DRAIN_PASS`.

## Route Isolation

Route isolation passed:

- Wrong-route pull did not cross-deliver.
- Route A and Route B remained separated.
- Wrong route token failed closed for mutation paths.
- Classification: `ROUTE_ISOLATION_PASS`.

## Auth Negative Tests

Auth negative tests passed:

- Missing route token failed closed.
- Wrong route token failed closed.
- Wrong bearer failed closed when bearer was configured.
- Missing bearer failed closed when bearer was configured.
- qsc relay wrong-bearer negative failed closed.
- Classification: `AUTH_NEGATIVES_FAIL_CLOSED_PASS`.

## Malformed / Bounded Input Tests

Malformed and bounded input tests passed:

- Empty push body failed closed.
- Malformed pull query failed closed.
- Unsupported method/path failed closed.
- Oversized local payload was rejected within the bounded local test limit.
- Bounded-large local payload was classified without resource pressure.
- Invalid content type was classified as not a semantic discriminator because
  qsl-server treats nonempty relay bodies as opaque payloads.
- Classification: `MALFORMED_INPUTS_FAIL_CLOSED_PASS`.

## Restart / Volatility Boundary

Restart/volatility boundary was executed with an NA-0588-owned qsl-server
process:

- A relay item was pushed.
- The owned qsl-server process was stopped and restarted.
- Pull after restart returned the no-item class.
- Source review found in-memory queue semantics and no persistence guarantee.
- Classification: `RESTART_BOUNDARY_VOLATILE_QUEUE_EXPECTED`.

## Concurrency / Rapid Operations

Bounded rapid/concurrency testing passed:

- A small concurrent push batch completed.
- Pull results contained all expected local test items.
- Ordering was classified as not relied on for the concurrent case.
- No race, count mismatch, cleanup failure, or resource-risk stop was detected.
- Classification: `CONCURRENCY_RAPID_OPS_PASS`.

## Metadata Minimization Review

Metadata minimization review passed:

- Raw qsc stdout/stderr, qsc local state, qsl-server logs, response bodies,
  payload fixtures, endpoint values beyond loopback class, private port values,
  route-token values, bearer values, Authorization values, plaintext, envelope
  bodies, key material, process identity, and private command lines remained
  proof-root-only.
- Repository evidence, PR body, and final response are limited to safe classes,
  counts, commit identifiers, and no-value boundary statements.
- No avoidable metadata overexposure was selected for a source fix.

## Diagnostics Quality Review

Diagnostics quality review passed:

- Required and supported stress categories returned actionable safe classes.
- Unsupported or non-discriminating semantics were explicitly classified.
- No generic or ambiguous stress failure remained after harness execution.
- Classification: `DIAGNOSTICS_ACTIONABLE_PASS`.

## Issue Investigation and Safe Fix

Automatic issue investigation found no product bug requiring qsc or qsl-server
mutation. Proof-root-only harness repairs were completed and recorded. No
project-owned source/test/helper fix was applied.

## Private-Material Review

Aggregate and category private-material scans passed with zero findings in the
publishable evidence set. The repository patch does not publish endpoint
values beyond loopback class, private port values, route-token values, bearer
values, Authorization values, payloads, response bodies, plaintext fixture
content, envelope bodies, qsc key material, topology, process identities, or
private command lines.

## Result Classification

`LOCAL_QSC_QSL_SERVER_E2EE_ADVERSARIAL_METADATA_STRESS_PASS`.

Basis:

- Baseline qsc E2EE over local qsl-server passed.
- Ten-cycle fresh-state repetition passed.
- Multi-message ordering and empty queue behavior passed.
- Route isolation passed.
- Auth negatives failed closed.
- Malformed and bounded input behavior was classified safely.
- Restart boundary matched expected volatile in-memory queue semantics.
- Bounded rapid/concurrency operations passed.
- Metadata review and private-material scans passed.

## Selected Successor

Option A was selected:

`NA-0589 -- QSL Local qsl-attachments Integration Readiness Harness`.

The successor may begin local qsl-attachments integration readiness after the
local qsc/qsl-server client/relay/E2EE behavior and selected adversarial and
metadata stress passed. NA-0589 must not be implemented by this PR.

## Required-Check Boundary

Current-main required checks were classified before implementation. Public
safety and advisories were green, suite2-vectors was satisfied, attached checks
had no failed or pending required run, and local dependency/metadata health
passed. Required-check visibility recovery was recorded for goal-lint/CodeQL
context naming without weakening check requirements.

## Source / Script Mutation Boundary

No qsc source, qsc test, qsc example, demo script, CI script, qsl-server source,
qsl-server test, qsl-server example, qsl-server doc, dependency, lockfile, or
workflow file was mutated by the implementation harness. The implementation PR
changes only governance, evidence, traceability, journal, and testplan files.

## Runtime / qsc Boundary

qsc was built and focused relay tests were run locally. qsc send/receive was
executed only against an NA-0588-owned loopback qsl-server and proof-root-only
state. Raw output and local state remain proof-root-only.

## qsl-server Boundary

qsl-server was cloned, reviewed, validated, started on loopback only, and cleaned
up. qsl-server source was not mutated. No qsl-server deployment, service,
Docker, systemd, cloud, dependency, or lockfile change occurred.

## qsl-attachments Boundary

No qsl-attachments command, runtime, integration, direct build lane, source
mutation, or attachment proof was executed. Any incidental dependency graph
compilation during qsc validation did not enter the qsl-attachments integration
lane. qsl-attachments remains deferred to the selected NA-0589 successor.

## Remote / Workflow / Tailscale Boundary

No remote SSH/scp action, Tailscale action, public network exposure, GitHub
workflow dispatch, GitHub workflow rerun, or GitHub runner work occurred.

## Public-Site / Cloudflare Boundary

No public-site, website, public docs, Cloudflare, DNS, public endpoint, or
production deployment mutation occurred.

## Evidence / Decision / Traceability

- D-1167 records the NA-0588 implementation decision.
- This evidence file records the local stress harness results.
- `tests/NA-0588_local_qsc_qsl_server_adversarial_metadata_stress_testplan.md`
  records required markers and boundary proof.
- `TRACEABILITY.md` maps NA-0588 to D-1167 and the selected successor.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` records proof gates, recovered
  failures, validation, classification, and successor selection.

## Validation

Validation for the PR includes diff/scope/queue/marker proof, link check,
private-material scan, overclaim scan, PR body preflight, goal-lint where
available, root cargo audit, nested qsc fuzz cargo audit, locked cargo metadata,
cargo fmt, qsc adversarial shell syntax checks, focused qsc relay tests, and
qsl-server metadata/audit/fmt/test/build validation.

## Recommendation

Merge NA-0588 after required checks pass, then close out NA-0588 and restore the
selected NA-0589 qsl-attachments readiness harness as the sole READY successor.
