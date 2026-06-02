Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0403 Director State Index Implementation Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Verify that NA-0403 implements a qsl-protocol local-ops Director State Index
helper and fixture matrix while preserving temp-output-only behavior,
stale-state rejection, live evidence authority, no durable local index, no
response archive/local history mutation, and no runtime/security/dependency or
public-surface drift.

## Protected Invariants

- READY_COUNT remains exactly 1.
- READY remains NA-0403 until closeout.
- NA-0402 is DONE.
- D-0786 exists once.
- D-0787 exists once.
- D-0788 is added once by NA-0403.
- D-0789 is absent until closeout.
- Public-safety remains required and green.
- Generated indexes are advisory only and temp-only.

## Allowed Scope

- `scripts/ci/qsl_director_state_index.py`
- `inputs/local_ops/director_state_index_fixtures/`
- `docs/governance/evidence/NA-0403_qsl_director_state_index_implementation_harness.md`
- `tests/NA-0403_qsl_director_state_index_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `.github/**`
- any script except `scripts/ci/qsl_director_state_index.py`
- any input path except `inputs/local_ops/director_state_index_fixtures/**`
- runtime, service, protocol, crypto, qshield runtime, qsl-server,
  qsl-attachments, qsc-desktop, website, README, START_HERE, docs/public,
  Cargo/dependency, workflow, backup script/timer/fstab/source-list, response
  archive, request archive, directive archive, journal archive, ops history,
  durable Director State Index files, local qstart/qresume tooling, branch
  protection, public-safety configuration, secret handling, and public technical
  paper files.

## NA-0402 Inheritance Requirements

- Consume D-0786 and D-0787.
- Preserve NA-0402's selected NA-0403 rationale.
- Preserve advisory-only status.
- Preserve future durable-storage backup-impact gating.
- Preserve qsl-server PR #56 and qsl-attachments PR #37 as bounded read-only
  evidence.

## Helper CLI Requirements

- `--help` succeeds.
- `fixture --fixtures-dir <path> --tmp-dir <path> [--json]` succeeds on the
  checked-in matrix.
- `generate --repo-root <path> --tmp-dir <path> --origin-main-sha <sha> --expected-ready <NA-id> --expected-latest-decision <D-id> --public-safety-status <success|failure|missing|unknown> [--json]` writes only under the requested temp directory.
- `validate --index-json <path> --origin-main-sha <sha> --expected-ready <NA-id> --expected-latest-decision <D-id> --public-safety-status <success|failure|missing|unknown> [--json]` validates without writing.

## Schema Requirements

Validate all required fields for `qsl.director_state_index.v1`: generation
metadata, repo SHA, queue state, decision state, PR references, public-safety,
branch protection, advisory state, sibling boundaries, backup state, local
history availability, Project Goal canon status, blockers, public claim
boundaries, evidence gaps, future candidates, stale detection, source
references, verification commands, advisory-only disclaimer, no-secret status,
and markers.

## Fixture Matrix Requirements

Run all 20 required cases:

- valid current state.
- stale origin/main rejection.
- READY mismatch rejection.
- latest decision mismatch rejection.
- duplicate decision rejection.
- multiple READY rejection.
- missing public-safety rejection.
- red public-safety rejection.
- missing branch-protection warning.
- qsl-server unavailable warning.
- qsl-attachments unavailable warning.
- backup status unavailable warning.
- secret sentinel rejection.
- public-claim overreach rejection.
- unknown schema rejection.
- malformed JSON rejection.
- durable output path rejection.
- response archive output path rejection.
- public docs output path rejection.
- advisory-only no-override success.

## Live Temp-Output Smoke Requirements

- Generate under `/srv/qbuild/tmp/NA0403_director_state_index_<timestamp>/live`.
- Validate the generated JSON.
- Use current `origin/main` SHA, READY NA-0403, latest decision D-0788, and
  public-safety status from read-only evidence.
- Record output path and SHA-256.
- Verify no output under `/home/victor/work/qsl/codex/**`.
- Verify no durable local index.

## Stale-State Rejection Requirements

Reject origin/main mismatch, READY mismatch, latest decision mismatch, duplicate
decision entries, multiple READY items, unknown schema, malformed JSON, stale
output path, and generated summary claims that override live evidence.

## Public-Claim Rejection Requirements

Reject public-claim overreach fixture input. Do not introduce public-readiness,
external-review, public technical paper, privacy, backup/restore/key, no
bug-free claim, or no perfect-crypto claim.

## Secret Rejection Requirements

Reject harmless secret sentinel fixtures and standard secret-shaped strings.
Do not copy real secrets into helper output.

## No Durable Index Requirements

- Reject any output path outside `/srv/qbuild/tmp/NA0403_director_state_index_*`.
- Reject `/home/victor/work/qsl/codex/**`.
- Reject response archive paths.
- Reject docs/public and website paths.

## No Response Archive / Local History Mutation Requirements

Do not mutate response archives, request archives, directive archives, journal
archives, ops history, or `/home/victor/work/qsl/codex/**` except the final
D223 response file required by the directive.

## Backup-Impact Requirements

No backup-plan update is required for tracked qsl-protocol helper/fixture and
governance/testplan/traceability/journal files plus temp proof output. Future
durable Director State Index storage requires separate backup-impact
authorization.

## CI Expectations

Run helper compile/help, fixture matrix, live smoke, existing helper checks,
cargo audit, rustls-webpki tree, cargo fmt, qsc send_commit, formal model
checks, qshield-cli test/build if feasible, scope guard, link-check, leak-scan,
classifier, and PR body preflight. Required PR checks must pass before merge.

## Successor Handoff

Selected successor:

`NA-0404 -- QSL Director State Index Durable Storage / Backup Impact Authorization Plan`

NA-0403 closeout must not implement NA-0404.
