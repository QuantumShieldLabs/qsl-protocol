Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-08

# NA-0255 Closeout Restore NA-0256 Test Plan

## Objective

Validate that NA-0255 closes only after the external website PR #19 merge and automatic deployment evidence are recorded, then restore exactly one successor: `NA-0256 — Public Demo and Desktop Touch-and-Feel Readiness Hardening`.

## Protected Invariants

- `public-safety` remains a required, green protected check.
- Exactly one READY item exists after closeout.
- NA-0255 records external website PR #19 head, merge, production URL, Cloudflare deployment evidence, and claim-boundary validation.
- qsl-protocol remained read-only during D046-D050 external website work.
- NA-0256 remains non-production demo/desktop readiness work and does not claim production desktop readiness.
- NA-0256 does not authorize protocol/crypto state-machine changes.
- Known gaps remain explicit: KT-negative demo, attachment demo, native package proof, keychain active ops, and production relay hardening.

## Scope Guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0255_closeout_restore_na0256_testplan.md`

Forbidden path classes include `.github/**`, `scripts/**`, Cargo manifests/locks, qsc/qsl app/runtime/protocol/crypto/demo/service implementation paths, qsc-desktop implementation, qsl-server, qsl-attachments, website, external website files, apps, tools, inputs, formal, branch-protection settings, and public-safety/check configuration.

Expected guard command:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed "NEXT_ACTIONS.md" --allowed "DECISIONS.md" --allowed "TRACEABILITY.md" --allowed "docs/ops/ROLLING_OPERATIONS_JOURNAL.md" --allowed "tests/NA-0255_closeout_restore_na0256_testplan.md"
```

If the helper is unavailable, use `git diff --name-only origin/main...HEAD` and fail unless every changed path is in the allowed list above.

## Pre-Closeout Proof

Required preconditions:

- qsl-protocol `origin/main` is `ebac12077d1fab9734af38c33d926c402d979c87` unless a newer expected closeout state has already landed.
- READY_COUNT is `1` with READY `NA-0255`.
- `D-0477` exists once.
- `D-0478` is absent before closeout edits.
- no duplicate decision IDs exist on `origin/main`.
- external website PR #19 is merged.
- PR #19 head is `624fb5e0dcad39dcade0e77a044574a2967ca19b`.
- PR #19 merge commit is `b72cca5e81307436d749fd9df1ddef14c07efcef`.
- D050 records automatic Cloudflare Pages deployment success and production render evidence for `https://quantumshieldlabs.org`.

## Queue Parser Expectation

After closeout:

```text
READY_COUNT 1
READY NA-0256 Public Demo and Desktop Touch-and-Feel Readiness Hardening
NA-0255 DONE External Website Evidence-Boundary Implementation Execution
```

## Decision Parser Expectation

After closeout:

- `D-0477` exists once.
- `D-0478` exists once.
- no duplicate decision IDs exist.

## NA-0256 Boundary Proof

The restored NA-0256 entry must state:

- demo/desktop readiness hardening only.
- no production-ready desktop claim.
- no protocol/crypto state-machine changes.
- no hidden protocol mutation.
- no token/secret leakage.
- demo positive path remains inspectable.
- demo negative/reject paths remain fail-closed.
- native package proof limitations remain explicit if host prerequisites block packaging.
- local checks must pass or host limitations must be documented.

## Validation Commands

```bash
git diff --check origin/main...HEAD
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed "NEXT_ACTIONS.md" --allowed "DECISIONS.md" --allowed "TRACEABILITY.md" --allowed "docs/ops/ROLLING_OPERATIONS_JOURNAL.md" --allowed "tests/NA-0255_closeout_restore_na0256_testplan.md"
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

If the helper is unavailable, run the directive-provided canonical queue/decision parsers, deterministic markdown link check, and added-line leak scan fallback.

Run local goal-lint with a synthetic PR event whose body includes `Goals: G1, G4, G5`.

## CI Expectations

- Required PR checks pass normally.
- CodeQL may be accepted as neutral only if GitHub accepts the aggregate required context.
- `public-safety` succeeds on the PR head and post-merge main.
- Merge uses a normal merge commit with a validated head SHA.
- No branch-protection exception, admin bypass, check spoofing, direct push, squash merge, or rebase merge occurs.
