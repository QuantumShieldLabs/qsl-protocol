Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0402 Director State Index Authorization Plan Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0402 creates an internal governance authorization plan for a
future Director State Index without implementing the index, creating durable
index files, mutating local history, mutating response archives, changing public
surfaces, or changing runtime/security/dependency/workflow behavior.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0402 until optional closeout.
- NA-0401 is DONE.
- D-0784 exists once.
- D-0785 exists once.
- D-0786 is added once.
- D-0787 is absent until optional closeout.
- Public-safety remains required and green.
- The Director State Index is advisory only.
- The index must not override live repo/GitHub/CI evidence.
- No Director State Index implementation is created in NA-0402.
- No durable Director State Index file is created in NA-0402.
- No public claim expansion is introduced.

## Allowed Scope

- `docs/governance/evidence/NA-0402_qsl_director_state_index_authorization_plan.md`
- `tests/NA-0402_qsl_director_state_index_authorization_plan_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- `scripts/**`
- `inputs/**`
- `formal/**`
- `qsc/**`
- `qsp/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `Cargo.toml`
- `Cargo.lock`
- workflows
- runtime, service, protocol, crypto, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, docs/public,
  public technical paper, external-review package, SECURITY.md, security.txt,
  disclosure policy, issue template, backup script/timer/fstab/source-list,
  off-host target, restore target, key, credential, passphrase, private key,
  recovery envelope, response archive, request archive, directive archive,
  journal archive, ops history, local qstart/qresume tooling, durable Director
  State Index files, branch-protection mutation, and public-safety mutation.

## NA-0401 Inheritance Requirements

- Consume the Project Goal canon created by NA-0401.
- Preserve NA-0401's selected successor rationale for NA-0402.
- Preserve no-public-overclaim, one-READY queue discipline, evidence-first
  posture, no-secret discipline, and internal-governance-only boundaries.
- Preserve qsl-server PR #56 as bounded service-local evidence only.
- Preserve qsl-attachments PR #37 as service-local prerequisite evidence only.

## Authority Model Requirements

The authorization plan must define this hierarchy:

1. Live qsl-protocol `origin/main`.
2. `NEXT_ACTIONS.md`.
3. `DECISIONS.md`.
4. `TRACEABILITY.md`.
5. GitHub PR/check/public-safety state.
6. Branch protection.
7. Cargo audit/dependency state.
8. qsl-server/qsl-attachments live read-only state.
9. Live backup status.
10. Current official sources when needed.
11. Director State Index.
12. Prior responses/history.

The plan must state that the index must not override items 1 through 10.

## Schema Design Requirements

The plan must define future schema fields for schema version, timestamps,
generator directive, generator host, qsl-protocol SHA, active READY item, last
DONE item, latest decision ID, duplicate decision count, recent PRs, public-
safety status, branch protection, dependency/advisory health, qsl-server and
qsl-attachments boundaries, backup status, local history availability, canon
status, blockers, public-claim boundaries, evidence gaps, future lane
candidates, stale-detection fields, source references, verification commands,
not-authority disclaimer, and no-secret scan status.

## Stale Detection Requirements

The plan must require stale or reject behavior for:

- origin/main SHA mismatch.
- READY mismatch.
- latest decision mismatch.
- public-safety mismatch.
- more than one READY.
- duplicate decisions.
- secret sentinel.
- public-claim overreach.
- unknown schema.
- malformed structured data when applicable.
- stale index being presented as current evidence.

Branch protection unavailability must be evidence-incomplete. qsl-server,
qsl-attachments, and backup unavailability may warn, but must not allow
overclaiming.

## Storage / Backup Requirements

The plan must compare:

- `/srv/qbuild/tmp` temp output.
- qsl-protocol tracked governance summary.
- `/home/victor/work/qsl/codex/ops` durable local state.
- final-response-only summary.
- no index.

The recommended first lane must be temp-output-only or tracked governance
helper/evidence only. Durable local ops output must require future backup-impact
review.

## Integration Requirements

The plan may define future integration with qstart/qresume guard evidence,
directive manifest validation, bounded polling, response history catalog,
response writer, routine audit cadence, queue/decision helper, public-safety,
cargo audit, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, and external
watch / audit cadence evidence.

NA-0402 must not implement integration.

## Fixture Strategy Requirements

Future NA-0403 fixture cases must include valid current state, stale
origin/main, READY mismatch, latest decision mismatch, duplicate decision, more
than one READY, missing public-safety, public-safety red, missing branch
protection, qsl-server unavailable, qsl-attachments unavailable, backup status
unavailable, secret sentinel, public claim overreach, unknown schema, malformed
JSON/YAML if applicable, stale index warning, and no live-state override proof.

## No Implementation Requirements

- No `scripts/ci/qsl_director_state_index.py` in NA-0402.
- No `inputs/local_ops/director_state_index_fixtures/` in NA-0402.
- No helper mutation in NA-0402.
- No qstart/qresume mutation in NA-0402.
- No runtime, crypto, dependency, workflow, or sibling-repo mutation in NA-0402.

## No Durable Index Requirements

- No Director State Index output file outside NA-0402 governance evidence.
- No `/home/victor/work/qsl/codex/ops` index file.
- No response archive mutation.
- No directive, request, journal, or ops history mutation.
- `/srv/qbuild/tmp` proof output is future NA-0403 scope only.

## Public Claim Boundary Requirements

The plan must state that NA-0402 is internal governance only and is not public
docs, not external review, not a public technical paper, not production
readiness, not public-internet readiness, not metadata-free proof, not anonymity
proof, not untraceable proof, not off-host backup proof, not disaster recovery
proof, not restore proof, not key custody proof, and not key recovery proof.

The plan must not update README, START_HERE, docs/public, website, public
security policy, disclosure files, or public technical paper files.

## Successor Selection Requirements

The selected normal successor should be exact:

`NA-0403 -- QSL Director State Index Implementation Harness`

The blocker successor should be exact if a storage/authority conflict is found:

`NA-0403 -- QSL Director State Index Authority / Storage Conflict Resolution`

NA-0402 must not implement NA-0403.

## Backup-Impact Requirements

No backup-plan update is required if changed paths remain limited to the allowed
qsl-protocol tracked governance/testplan/traceability/journal files.

Future durable local index output requires explicit backup-impact review before
authorization.

## Required Local Checks

Run or record:

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- helper help and representative fixture checks
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- qsc send_commit when feasible
- formal model checks when feasible
- qshield-cli test/build when feasible
- queue and decisions helper
- scope guard
- link-check
- leak-scan
- classifier
- PR body preflight / goal-lint

## CI Expectations

- Required qsl-protocol checks remain required by branch protection.
- Public-safety is required and green before merge.
- PR checks complete green, neutral, or skipped before merge.
- Post-merge public-safety is green.

## Successor Handoff

If NA-0402 merges and closeout is authorized, restore exactly one READY item:

`NA-0403 -- QSL Director State Index Implementation Harness`

The closeout must not implement NA-0403.
