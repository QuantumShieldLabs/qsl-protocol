Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0403 QSL Director State Index Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0403 adds a qsl-protocol local-ops Director State Index helper and fixture
matrix. The helper summarizes queue, decision, caveat, and blocker state into
temporary proof output only. It validates stale-state inputs and rejects
conflicting state before writing an index.

The helper is advisory only. Live qsl-protocol source, GitHub PR/check data,
required CI, branch protection, dependency/advisory health, and current
read-only operational evidence remain above any generated summary.

## Live NA-0403 Scope

The live `NEXT_ACTIONS.md` item is:

`NA-0403 -- QSL Director State Index Implementation Harness`

The live objective is a temp-output-only harness that summarizes current
queue/evidence/blocker/caveat state while preserving live evidence authority,
rejecting stale or conflicting state, creating no durable local index, making
no public claims, and mutating no runtime, dependency, workflow, history, or
public surface.

## Inherited NA-0402 Authorization

NA-0402 authorized a future helper at
`scripts/ci/qsl_director_state_index.py`, fixture inputs under
`inputs/local_ops/director_state_index_fixtures/`, and temp output under
`/srv/qbuild/tmp/NA0403_director_state_index_*`.

NA-0402 also required that durable local index output wait for a later
backup-impact authorization. NA-0403 follows that boundary.

## Helper Design

The helper exposes only:

- `fixture --fixtures-dir <path> --tmp-dir <path> [--json]`
- `generate --repo-root <path> --tmp-dir <path> --origin-main-sha <sha> --expected-ready <NA-id> --expected-latest-decision <D-id> --public-safety-status <success|failure|missing|unknown> [--json]`
- `validate --index-json <path> --origin-main-sha <sha> --expected-ready <NA-id> --expected-latest-decision <D-id> --public-safety-status <success|failure|missing|unknown> [--json]`

It uses Python standard library only. It does not call GitHub, git, network
APIs, workflow tooling, schedulers, or mutation commands. The helper writes only
under the explicit temp output directory after validating the NA-0403 temp
prefix.

## Helper Implementation

Helper path:

`scripts/ci/qsl_director_state_index.py`

The generated index schema is `qsl.director_state_index.v1` and contains the
required fields for generation time, generator, directive ID, repo root,
origin/main SHA, READY state, latest DONE item, latest decision, duplicate
decision count, recent PR references, public-safety status, branch protection,
dependency/advisory summary, qsl-server and qsl-attachments boundaries, backup
status, local history availability, Project Goal canon status, blockers, public
claim boundaries, evidence gaps, future candidate lanes, stale detection,
source references, verification commands, advisory-only disclaimer,
no-secret-scan status, and markers.

## Fixture Matrix

Fixture directory:

`inputs/local_ops/director_state_index_fixtures/`

The fixture matrix covers valid current state, stale origin/main, READY
mismatch, latest decision mismatch, duplicate decisions, multiple READY items,
missing and red public-safety, branch-protection warning, qsl-server warning,
qsl-attachments warning, backup-status warning, secret sentinel rejection,
public-claim overreach rejection, unknown schema rejection, malformed JSON
rejection, durable output path rejection, response archive output path
rejection, public docs output path rejection, and advisory-only no-override
success.

Final fixture proof:

- Command: `python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_final_fixtures --json`
- Result: 20/20 fixture cases matched expected pass/fail/warn behavior.

## Live Temp-Output Smoke

Live smoke was run after D-0788 was present locally. The command shape was:

`python3 scripts/ci/qsl_director_state_index.py generate --repo-root . --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_final2_live/live --origin-main-sha 779e1cf2edbb2f942ff940235c695d02e5b2beae --expected-ready NA-0403 --expected-latest-decision D-0788 --public-safety-status success --json`

The generated JSON was then validated with the same expected live-state inputs.
The output is temp-only. Live index SHA-256:

`68440bcd6b4beee95547ea8c015fca4c044d23b5ea379554277a7a046a065b0f`

## No-Mutation / No-Secret / No-Public-Claim Proof

The helper contains no network imports or mutation tool calls. It does not write
outside the validated temp output directory. It rejects sentinel-shaped secret
fixtures and public-claim overreach fixtures.

Negative fixture strings are harmless test sentinels only. The helper does not
copy secret-bearing material into output.

## Backup / Storage Impact

Tracked durable changes are limited to qsl-protocol helper, fixture,
governance, testplan, traceability, and journal files. Generated index proof is
only under `/srv/qbuild/tmp/NA0403_director_state_index_*`.

No durable Director State Index file is created. No output is written under
`/home/victor/work/qsl/codex/ops`. No backup-plan update is required for
NA-0403.

Future durable index storage requires backup-impact authorization.

## Public Claim / External Review / Website Boundary

The Director State Index is internal local-ops/governance evidence only. It is
not public docs, not external review, not a public technical paper, not
production readiness, and not public-internet readiness.

NA-0403 does not mutate README, START_HERE, docs/public, website, security
policy, disclosure files, or public technical paper files.

## Successor Selection

Selected successor:

`NA-0404 -- QSL Director State Index Durable Storage / Backup Impact Authorization Plan`

Rationale: NA-0403 proves temp-only helper behavior. The next decision is
whether any durable Director State Index storage should exist, where it would
live, how backup coverage would protect it, and how stale summaries stay below
live evidence.

## Future Path / Scope Bundle

Expected NA-0404 allowed paths:

- `docs/governance/evidence/NA-0404_qsl_director_state_index_durable_storage_backup_impact_authorization_plan.md`
- `tests/NA-0404_qsl_director_state_index_durable_storage_backup_impact_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden unless future exact scope authorizes: helper implementation changes,
durable local ops files, response archive mutation, local history mutation,
backup script/timer/fstab/source-list mutation, runtime, crypto, dependency,
workflow, public docs, website, and public-claim changes.

## Future Validation / Marker Plan

Future NA-0404 markers:

- `NA0404_DIRECTOR_STATE_INDEX_STORAGE_AUTHORIZATION_OK`
- `NA0404_BACKUP_IMPACT_REVIEW_OK`
- `NA0404_DURABLE_INDEX_LOCATION_DECISION_OK`
- `NA0404_STALENESS_POLICY_PRESERVED_OK`
- `NA0404_LIVE_REPO_AUTHORITY_PRESERVED_OK`
- `NA0404_NO_RUNTIME_CHANGE_OK`
- `NA0404_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0404_NO_DEPENDENCY_CHANGE_OK`
- `NA0404_NO_WORKFLOW_CHANGE_OK`
- `NA0404_NO_SECRET_MATERIAL_OK`
- `NA0404_NO_PUBLIC_READINESS_CLAIM_OK`

## Rejected Alternatives

- Durable local Director State Index output in NA-0403.
- Output under `/home/victor/work/qsl/codex/ops`.
- Public docs or website publication.
- Any helper behavior that treats generated summaries as authority above live
  evidence.
- Runtime, crypto, dependency, workflow, qsl-server, or qsl-attachments changes.

## Next Recommendation

Merge NA-0403 only after local validation and required PR checks pass. If
post-merge public-safety remains green, close out NA-0403 and restore the exact
selected NA-0404 successor without implementing NA-0404.
