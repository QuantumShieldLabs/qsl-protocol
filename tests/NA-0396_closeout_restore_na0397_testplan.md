# NA-0396 Closeout and NA-0397 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0396 is closed only after the dependency/advisory watch trigger
policy plan is merged, post-merge public-safety is green, and exactly one READY
successor is restored:
`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`.

## Protected Invariants

- READY_COUNT is exactly 1 after closeout.
- READY item is NA-0397.
- NA-0396 is DONE.
- D-0774 exists once.
- D-0775 exists once.
- D-0776 is absent.
- NA-0397 is not implemented by this closeout.
- No runtime, protocol, crypto, dependency, Cargo, workflow, public-doc,
  website, backup-script, qsl-server, qsl-attachments, qshield runtime,
  response archive, local tool, or secret-bearing path is changed.
- No automatic READY promotion occurs outside the selected successor.
- No public, readiness, privacy, external-review, vulnerability-free,
  bug-free, or perfect-crypto claim is expanded.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0396_closeout_restore_na0397_testplan.md`

## Forbidden Scope

Forbidden paths include runtime/service/protocol/crypto/dependency files,
`Cargo.toml`, `Cargo.lock`, `.github/**`, `scripts/**`, `qsc/**`, `qsp/**`,
`qsl/**`, `apps/**`, `formal/**`, qsl-server, qsl-attachments, website,
README, START_HERE, docs/public, backup scripts/timers/fstab/services, response
archives, request/directive/journal history roots, `/srv/qbuild/tools/**`, and
local/off-host backup or restore configuration.

## Required Closeout Evidence

- Packet Q PR #1055 merged.
- Packet Q head SHA is recorded as `67c0a289f5f`.
- Packet Q merge SHA is recorded as `4fd4f7e31803`.
- Packet Q post-merge public-safety on `4fd4f7e31803` is success.
- D-0774 records the NA-0396 advisory trigger policy decision.
- D-0775 records closeout and selected successor restoration.
- `NEXT_ACTIONS.md` marks NA-0396 DONE and NA-0397 READY.
- `TRACEABILITY.md` links D-0775, D-0774, PR #1055, closeout testplan, selected
  successor, backup-impact classification, and sibling repo boundaries.

## Successor Handoff

The restored successor is:
`NA-0397 -- QSL Code / Crypto Research Watch and Audit Follow-Up Plan`.

Future NA-0397 may create a governance plan for code/crypto research watch and
audit follow-up candidates only if live NA-0397 scope authorizes it. This
closeout must not create research findings, update dependencies, change runtime
code, change workflows, or make public claims.

## Backup Impact

The closeout changes only tracked qsl-protocol governance and testplan files.
No backup-plan update is required. Any future durable code/crypto research
report, advisory report store, public technical paper evidence store, or
off-host backup change requires separate backup-impact review.

## Required Validation

- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard ...`
- `python3 scripts/ci/qsl_evidence_helper.py link-check --root .`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`

## CI Expectations

The closeout PR must pass required checks normally, including public-safety and
goal-lint. CodeQL neutral/skipped may be accepted only under the existing
docs-only policy. No admin bypass, squash, rebase, direct push, force-push,
amend-after-PR, or branch-deletion command is allowed.
