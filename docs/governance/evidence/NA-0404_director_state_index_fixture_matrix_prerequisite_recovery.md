Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-02

# NA-0404 Director State Index Fixture Matrix Prerequisite Recovery

Goals: G1, G2, G3, G4, G5

## Executive Summary

This recovery repairs a prerequisite defect inherited from NA-0403: the
Director State Index helper required a `secret_sentinel_reject.json` fixture,
but that file was absent from tracked `origin/main`.

The recovery adds only the missing fixture and records governance evidence for
that prerequisite repair. It does not execute NA-0404 durable storage planning,
does not close NA-0404, and does not restore or implement NA-0405.

## Live Recovery Scope

Allowed tracked changes are limited to the missing fixture, this evidence file,
the NA-0404 prerequisite-recovery testplan, D-0790, TRACEABILITY, and the
rolling operations journal.

The recovery leaves `scripts/ci/qsl_director_state_index.py` unchanged. It also
leaves runtime, protocol, crypto, dependencies, workflows, public docs, website,
backup tooling, local history, durable index storage, qsl-server,
qsl-attachments, and qsc-desktop unchanged.

## D225 Recovery Note

D225 stopped on dependency health from a stale checkout. D226 later proved true
`origin/main` was dependency-healthy after proper fast-forward: `cargo audit
--deny warnings` passed and `cargo tree -i rustls-webpki --locked` reported
`rustls-webpki v0.103.13`.

That stale-checkout stop did not identify a dependency defect on current main.

## D226 Root-Cause Note

D226 reproduced a real fixture-contract defect on true `origin/main`. The
helper's required fixture list had 20 entries, but only 19 JSON fixtures were
tracked. The absent required file was:

`inputs/local_ops/director_state_index_fixtures/secret_sentinel_reject.json`

The pre-patch matrix failed with:

`ERROR: required fixtures missing: secret_sentinel_reject.json`

## Fixture Schema Discovery

Read-only inspection found that fixture cases use schema
`qsl.director_state_index.fixture_case.v1`. A fixture may provide a `set`
object to deep-merge values into the helper's base fixture index, and may
provide `remove` entries for negative cases.

The helper expects `secret_sentinel_reject.json` to produce result `fail` with
zero required warnings. The helper's secret scan includes a harmless test
sentinel pattern named `na0403_secret_sentinel`, matched by the literal string
`NA0403_SECRET_SENTINEL`.

## Missing Fixture Implementation

The added fixture is minimal:

`inputs/local_ops/director_state_index_fixtures/secret_sentinel_reject.json`

It sets only one harmless test probe string, `NA0403_SECRET_SENTINEL`, so the
helper reaches the intended secret-sentinel rejection path. The fixture contains
no real secret, credential, key material, passphrase, password, token, or
recovery-envelope content.

The broad repository ignore rule for `*secret*` also matched this exact file.
The path is explicitly authorized by this recovery, so staging must force-add
only this one fixture path and must not alter ignore rules.

## Fixture Matrix Proof

Pre-patch reproduction:

`python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_NA0404_prerecovery_fixture_check --json`

Result: failed because `secret_sentinel_reject.json` was missing.

Post-patch proof:

`python3 scripts/ci/qsl_director_state_index.py fixture --fixtures-dir inputs/local_ops/director_state_index_fixtures --tmp-dir /srv/qbuild/tmp/NA0403_director_state_index_NA0404_recovered_fixture_check --json`

Result: fixture matrix passed with `fixture_count=20`, `pass_count=20`, and
`fail_count=0`. The recovered fixture case produced:

`secret sentinel/material rejected: na0403_secret_sentinel`

Output was confined to:

`/srv/qbuild/tmp/NA0403_director_state_index_NA0404_recovered_fixture_check`

No durable Director State Index file was created.

## No-Secret / Leak / CodeQL Safety Proof

Added-line leak scan:

`python3 scripts/ci/qsl_evidence_helper.py leak-scan --paths inputs/local_ops/director_state_index_fixtures/secret_sentinel_reject.json`

Result: `SECRET_FINDING_COUNT 0`.

Targeted high-confidence credential-pattern scan over the new fixture produced
zero matches. The fixture uses only the helper-recognized harmless test sentinel
and does not contain token-shaped or private-key-shaped content.

CodeQL and public-safety remain required PR checks. This recovery does not
bypass, weaken, or configure those checks.

## Scope Boundary

This recovery changes no helper logic and no existing fixture file. The only
fixture addition is the missing required case.

The recovery does not touch `.github/**`, Cargo files, runtime code, protocol
state machines, key schedules, negotiation, crypto implementation, qshield
runtime, public docs, website, README, START_HERE, backup scripts, backup
timers, fstab, source lists, local history, durable Director State Index output,
qsl-server, qsl-attachments, or qsc-desktop.

## Backup Impact

No backup-plan update is required for this recovery because the durable changes
are tracked qsl-protocol files and the proof output is temp-only under
`/srv/qbuild/tmp`.

Durable Director State Index storage and backup-impact authorization remain
unresolved and belong to the future NA-0404 retry.

## Queue Impact

NA-0404 remains the sole READY item. NA-0403 remains DONE. D-0790 records only
the prerequisite repair. D-0791 remains absent. NA-0405 is not restored.

## Next Recommendation

After this recovery merges and required checks remain green, retry NA-0404's
durable Director State Index storage and backup-impact authorization plan as a
separate directive.

## Rejected Alternatives

- Modify `scripts/ci/qsl_director_state_index.py`.
- Modify existing fixtures.
- Use real secret-like material to trigger the rejection path.
- Change ignore rules instead of force-adding the one authorized ignored path.
- Treat fixture recovery as NA-0404 durable storage completion.
- Restore or implement NA-0405.
- Create durable Director State Index output.
- Change runtime, crypto, dependencies, workflows, public docs, website, backup
  configuration, qsl-server, qsl-attachments, or qsc-desktop.
