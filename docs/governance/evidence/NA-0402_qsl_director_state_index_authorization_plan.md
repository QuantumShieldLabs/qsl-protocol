Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-01

# NA-0402 QSL Director State Index Authorization Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0402 authorizes planning for a future Director State Index that may produce
bounded internal governance/local-ops summaries of QSL queue state, evidence
state, blockers, caveats, and next-action context.

The index is advisory only. It must not override live qsl-protocol
`origin/main`, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, GitHub PR
state, required CI, branch protection, dependency/advisory health,
qsl-server/qsl-attachments read-only state, live backup status, or current
official sources when a directive requires source verification.

NA-0402 does not create a Director State Index, does not implement a helper, and
does not create durable local state. The selected normal successor is:

`NA-0403 -- QSL Director State Index Implementation Harness`

## Live NA-0402 Scope

The live `NEXT_ACTIONS.md` item is:

`NA-0402 -- QSL Director State Index Authorization Plan`

Allowed NA-0402 scope is qsl-protocol internal governance planning for a future
Director State Index. The plan may define purpose, schema, authority hierarchy,
refresh rules, stale-data rejection rules, validation cases, storage options,
backup-impact policy, no-secret policy, update policy, and future successor
scope.

Forbidden NA-0402 scope includes runtime, service, protocol, crypto, dependency,
workflow, public-doc, website, README, START_HERE, docs/public, sibling-repo,
qshield runtime, backup script/timer/fstab/source-list, local history, response
archive, helper-script, durable index, public technical paper, public security
policy, secret, credential, passphrase, private-key, recovery-envelope, branch
protection, and public-safety configuration mutation.

Acceptance requires exactly one READY item, READY NA-0402 during this packet,
NA-0401 DONE, D-0784 once, D-0785 once, D-0786 added once by this packet,
D-0787 absent until closeout, no Director State Index implementation, no durable
index output, no public claim expansion, and public-safety remaining required
and green.

Stop conditions include any live-scope conflict, multiple READY items, duplicate
decision IDs, stale or red required CI that cannot be recovered within policy,
attempted implementation, attempted durable index creation, attempted local
history mutation, attempted public-surface mutation, attempted runtime/security
semantic change, or any change that would weaken fail-closed evidence behavior.

## Inherited NA-0401 Rationale

NA-0401 created
`docs/governance/PROJECT_GOAL_AND_OPERATING_PRINCIPLES.md` as internal
governance canon and selected NA-0402 as the next READY successor. The canon
requires evidence-first work, one-READY queue discipline, conservative public
claim boundaries, scope control, no-secret discipline, and live evidence
authority.

The Director State Index follows from that canon: long directive chains and
large response bundles now carry useful state, but future Directors, Codex, and
human operators need a compact way to find current state without trusting stale
summaries as current evidence.

## State-Load and Handoff Problem Statement

QSL directives and responses now contain large evidence bundles. Queue state,
decision state, CI requirements, branch protection, sibling-repo boundaries,
backup caveats, public-claim caveats, and residual blockers span many prior
lanes.

A future index can reduce cognitive load by summarizing the current state in a
bounded structure. It must not shorten verification below the project safety
bar. It must expose uncertainty, source references, and stale-data rules. It
must require revalidation against live repo/GitHub/CI evidence before a future
directive relies on it.

The index is useful as a map. It is not evidence when live state disagrees.

## Director State Index Authority Model

Authority hierarchy:

1. Live qsl-protocol `origin/main`.
2. `NEXT_ACTIONS.md`.
3. `DECISIONS.md`.
4. `TRACEABILITY.md`.
5. GitHub PR, check-run, and required public-safety state.
6. Branch protection.
7. Cargo audit and dependency/advisory state.
8. Live qsl-server/qsl-attachments read-only source and CI state.
9. Live backup status.
10. Current official sources when a directive requires source verification.
11. Director State Index.
12. Prior responses/history.

The Director State Index must remain advisory. It must not be used as the
current authority over items 1 through 10. Prior responses and history remain
lower than the index and lower than all live evidence.

The index is internal governance/local-ops evidence only. It is not public docs,
not a public claim, not external review, not a public technical paper, not
production readiness, and not public-internet readiness.

## Director State Index Schema Design

Future schema should be explicit and versioned. Recommended top-level fields:

- `schema_version`: exact supported version.
- `status`: generated, stale, rejected, or validation-only.
- `generated_at_utc`: generation timestamp.
- `generated_at_local`: local timestamp and timezone.
- `generator_directive_id`: directive that generated the summary.
- `generator_host`: host identity string safe for internal evidence.
- `repo`: qsl-protocol owner/repo and `origin/main` SHA.
- `active_ready_item`: READY NA number, title, and source reference.
- `last_done_item`: latest completed NA number and title.
- `latest_decision_id`: latest decision ID and source reference.
- `duplicate_decision_count`: duplicate decision count.
- `recent_prs`: PR numbers, titles, head SHAs, merge SHAs, and state.
- `public_safety`: required status and latest relevant conclusion.
- `branch_protection`: required checks, admin enforcement, force-push setting,
  and deletion setting.
- `dependency_advisory`: cargo audit result and key dependency summary.
- `sibling_boundaries`: qsl-server and qsl-attachments PR/CI caveats.
- `backup_status`: live mount/log/manifest summary and continuity caveat.
- `history_availability`: read-only response/request/directive/journal/ops
  availability summary.
- `canon_status`: Project Goal canon presence and marker status.
- `active_blockers`: unresolved blockers or explicit none.
- `public_claim_boundaries`: current public-claim limits and caveats.
- `evidence_gaps`: missing or unavailable evidence.
- `future_candidate_lanes`: queue candidates that are not auto-promoted.
- `d132_status`: D132 cleanup authorization state.
- `audit_cadence`: routine audit cadence status.
- `external_watch`: external watch status and source-verification caveat.
- `recurring_caveats`: caveats that recur across directives.
- `stale_detection`: generation SHA, READY, latest decision, public-safety,
  and optional source checksums/references.
- `source_references`: exact files, PRs, commands, and safe short SHAs.
- `last_verification_commands`: commands that produced the summary.
- `disclaimer`: explicit not-authoritative wording.
- `no_secret_scan`: secret-sentinel and scan result.

The schema must state what the index may contain: bounded internal state
summaries, source references, short SHAs, statuses, caveats, blockers, and
future lane candidates. It must state what the index must not contain: secrets,
credentials, private keys, passphrases, tokens, raw sensitive paths, long secret-
like dumps, public claims, external-review claims, public technical paper
content, runtime source copies, or durable local-history copies.

## Staleness / Revalidation / Fail-Closed Rules

The index is stale if live qsl-protocol `origin/main` differs from the
generation SHA.

The index is stale if live READY item differs from the recorded READY item.

The index is stale if live latest decision differs from the recorded latest
decision.

The index is stale if required public-safety state differs from the recorded
public-safety state.

The index is evidence-incomplete if branch protection cannot be read.

The index may warn, but not fail the full summary, if qsl-server,
qsl-attachments, or backup status is unavailable. The warning must prevent
overclaiming and must require live revalidation before relying on those areas.

The index must reject if a secret sentinel is found, if more than one READY item
exists, if duplicate decisions exist, if public-claim overreach appears, if the
schema is unknown, if JSON/YAML is malformed when structured output is used, or
if a stale index is presented as current evidence.

A stale index may be read only as a historical hint. Future directives must run
live revalidation before citing index data as current.

Future directive citations should cite the index as:

`Director State Index generated by <directive> at <timestamp> for <sha12>;
revalidated against live <sha12> by <command/evidence>.`

If revalidation is not performed, the directive must cite it only as historical
context.

## Storage / Backup / Retention Options

Option 1: `/srv/qbuild/tmp` temp output.

- Best first implementation option.
- Low retention assumption.
- No durable local index.
- Requires no backup-plan update if temp-only.
- Suitable for NA-0403 proof harness output.

Option 2: qsl-protocol tracked governance summary.

- Durable through repo history.
- Easy to review in PRs.
- Higher risk of stale summaries being mistaken for current state.
- Must carry strong stale-data and not-authority disclaimers.
- Acceptable only as governance/evidence helper output, not live authority.

Option 3: `/home/victor/work/qsl/codex/ops` durable local state.

- Useful for local operations.
- Requires explicit backup-impact review before use.
- Must not be introduced by NA-0402.
- Must not become a hidden source of current truth.

Option 4: final-response-only summary.

- Already permitted as directive evidence.
- Does not create a separate index artifact.
- Does not solve future discovery as well as a bounded helper.

Option 5: no index.

- Avoids stale-index risk.
- Leaves state-load and handoff cost high.

Recommendation: NA-0403 should implement a qsl-protocol helper and fixtures with
temp output under `/srv/qbuild/tmp/NA0403_director_state_index_*`. Durable local
ops output must wait for backup-impact review and explicit future scope.

## Integration Plan

Future NA-0403 may integrate read-only references from:

- qstart/qresume guard evidence, without mutating `/srv/qbuild/tools/**`.
- directive manifest / allow-file validation.
- bounded polling helper.
- response history catalog helper, read-only and metadata-only.
- response writer helper, without writing response archives.
- routine audit cadence helper.
- `qsl_evidence_helper.py` queue and decisions commands.
- public-safety check-run evidence.
- cargo audit and dependency tree evidence.
- `NEXT_ACTIONS.md`, `DECISIONS.md`, and `TRACEABILITY.md`.
- external watch and routine audit cadence evidence, when already present.

NA-0402 implements none of this integration.

## Fixture / Negative Test Strategy

Future validation cases:

- valid current-state fixture.
- stale origin/main reject.
- READY mismatch reject.
- latest decision mismatch reject.
- duplicate decision reject.
- more than one READY reject.
- missing public-safety status reject.
- public-safety red reject.
- missing branch protection evidence-incomplete result.
- qsl-server unavailable warning.
- qsl-attachments unavailable warning.
- backup status unavailable warning.
- secret sentinel reject.
- public claim overreach reject.
- unknown schema reject.
- malformed JSON/YAML reject when applicable.
- stale index warning.
- no live-state override proof.

Fixtures should prove both positive generation and fail-closed rejection. Tests
must fail for the protected invariant, not merely for formatting.

## Selected Successor

Selected normal successor:

`NA-0403 -- QSL Director State Index Implementation Harness`

Rationale: the Project Goal canon is merged, state load is high,
response/history/catalog helpers exist, and a temp-output harness can prove the
index behavior without durable local-state mutation.

Rejected blocker successor:

`NA-0403 -- QSL Director State Index Authority / Storage Conflict Resolution`

Reason rejected: no storage/authority conflict was found that blocks a
temp-output-only harness. Durable local ops storage remains future-gated by
backup-impact review.

## Future Path / Scope Bundle

Future NA-0403 allowed paths if the normal successor is accepted:

- `scripts/ci/qsl_director_state_index.py`
- `inputs/local_ops/director_state_index_fixtures/`
- `docs/governance/evidence/NA-0403_qsl_director_state_index_implementation_harness.md`
- `tests/NA-0403_qsl_director_state_index_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future temp output:

- `/srv/qbuild/tmp/NA0403_director_state_index_*`

Future forbidden unless explicitly authorized:

- runtime, service, protocol, crypto, qsc, qsp, qsl, qshield runtime,
  qsl-server, qsl-attachments, workflow, dependency, Cargo, public docs,
  website, README, START_HERE, backup scripts/timers/fstab/source-list,
  response archives, local history mutation, durable ops index, public
  technical paper, public claims, secret handling, and branch protection or
  public-safety configuration mutation.

## Public Claim / External Review / Website Boundary

Director State Index authorization is internal governance only.

It is not public docs, not external review, not a public technical paper, not
production readiness, not public-internet readiness, not metadata-free proof,
not anonymity proof, not untraceable proof, not off-host backup proof, not
disaster recovery proof, not restore proof, not key custody proof, and not key
recovery proof.

NA-0402 does not update README, START_HERE, docs/public, website, security.txt,
SECURITY.md, disclosure policy, issue templates, or public technical paper
files.

## Future Validation / Marker Plan

Future NA-0403 markers:

- `NA0403_DIRECTOR_STATE_INDEX_HELPER_OK`
- `NA0403_TEMP_OUTPUT_ONLY_OK`
- `NA0403_LIVE_REPO_AUTHORITY_OK`
- `NA0403_QUEUE_STATE_REFERENCE_OK`
- `NA0403_DECISION_STATE_REFERENCE_OK`
- `NA0403_PUBLIC_SAFETY_REFERENCE_OK`
- `NA0403_STALE_ORIGIN_REJECT_OK`
- `NA0403_READY_MISMATCH_REJECT_OK`
- `NA0403_DUPLICATE_DECISION_REJECT_OK`
- `NA0403_PUBLIC_CLAIM_OVERREACH_REJECT_OK`
- `NA0403_SECRET_SENTINEL_REJECT_OK`
- `NA0403_NO_BACKGROUND_WORK_OK`
- `NA0403_NO_DURABLE_LOCAL_INDEX_OK`
- `NA0403_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0403_NO_RUNTIME_CHANGE_OK`
- `NA0403_NO_CRYPTO_IMPLEMENTATION_CHANGE_OK`
- `NA0403_NO_DEPENDENCY_CHANGE_OK`
- `NA0403_NO_WORKFLOW_CHANGE_OK`
- `NA0403_NO_SECRET_MATERIAL_OK`

## Rejected Alternatives

- Creating the Director State Index in NA-0402.
- Creating durable local ops index output in NA-0402.
- Treating prior responses/history as current evidence.
- Treating an index as current authority when live evidence disagrees.
- Updating public docs, README, START_HERE, docs/public, website, public
  security policy, or public technical paper files.
- Mutating qsl-server, qsl-attachments, qshield runtime, runtime/protocol/crypto
  implementation, dependencies, workflows, branch protection, backup
  configuration, or local history.
- Selecting the storage-conflict blocker successor without an actual conflict.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0402 because changed paths are limited
to tracked qsl-protocol governance evidence, testplan, decision, traceability,
and rolling journal files.

Future durable local index output under `/home/victor/work/qsl/codex/ops` or
any other non-temp local path requires explicit backup-impact review before it
is authorized.

## Next Recommendation

Proceed to NA-0403 with a temp-output-only implementation harness if Packet P
and optional closeout finish with required CI green and exactly one READY
successor.

Do not create durable local state, public claims, public documents, public
technical paper artifacts, runtime changes, crypto changes, dependency changes,
workflow changes, sibling-repo changes, or backup configuration changes in the
next lane unless exact future scope explicitly authorizes them.
