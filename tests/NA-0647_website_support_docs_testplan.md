# NA-0647 testplan — Website-support public-docs refresh (D583, D-1270)

Docs-only lane + ONE verification run. The "tests" here are the lane's verification
steps and their observed results; there is no code under test.

## T1 — Demo smoke-run (ITEM 3, run FIRST per D583 Phase 2)

- Step: from a clean qsl-protocol checkout at `ac7e850c`, run exactly what the site's
  RunDemos page instructs: `./scripts/demo/qsc_demo_local.sh` (bare; happy-path, seed 1).
- Expected (if the instructions are current): deliveries occur; the received payloads
  match; the script's summary shows nonzero delivery activity.
- Observed: **FAIL (surprise failure, recorded + flagged, not fixed).** Script exit 0 +
  `DEMO DONE`, but `deliver_count=0`, both receive directories empty, and all four qsc
  invocations emitted exactly `event=error code=vault_locked reason=explicit_unlock_required`.
  Root cause: the script pre-dates the qsc explicit vault-unlock requirement and passes
  no `--unlock-passphrase-file`/`--unlock-passphrase-env`; the script also masks failure
  (`|| true` + unconditional `DEMO DONE`). Full record: as-built §2. Filed: ENG-0045;
  matrix row WCM-110 (OUTDATED, MUST_FIX).

## T2 — Policy-half byte-preservation (ITEM 1's hard requirement)

- Step: extract the region from `## NA-0539 Repository Claim Policy Addendum` through
  the end of the NA-0541 policy list (everything before `## Pages Checked`) from HEAD
  and from the edited working tree; `diff` them; hash them.
- Observed: **PASS.** diff EMPTY; 5,220 bytes / 57 lines; sha256
  `3566f215b961112e51b8db4af949ec29eb54a927b5e5dcb306f56d9b154e46a1` identical on both
  sides. The lane diff contains zero hunks inside the policy region.

## T3 — Website deep-link target existence (WCM-104 basis)

- Step: for each of the 15 qsl-protocol targets in the website's `src/links.js`
  (at website commit `21a908a4`), check `git cat-file -e HEAD:<path>` at `ac7e850c`;
  confirm `.github/workflows/ci.yml` carries `name: qshield-ci`.
- Observed: **PASS.** 15/15 present; workflow name confirmed.

## T4 — Relative-markdown-link integrity of the changed public docs

- Step: replicate the public-safety gate's markdown link check offline (same regex,
  same code-block stripping, same relative resolution) over the four changed
  docs/public files.
- Observed: **PASS.** 0 missing targets.

## T5 — Coupled-touch consistency (ITEM 2)

- Step: confirm INDEX.md's latest-entry sentence, its entry list, and its inline
  summary all reference 2026-07-15; confirm PROGRESS.md lists the new entry first and
  its Last-Updated is 2026-07-15; confirm the new entry file exists at
  `docs/public/progress/2026-07-15.md`.
- Observed: **PASS.** No remaining "latest ... July 10" statement in INDEX.md.

## T6 — Scope guard

- Step: `git status` / the lane diff vs the D583 allowed list.
- Observed: **PASS.** Changed paths are exactly the four docs/public files + the
  governance/closeout set. Zero code/script/vector/workflow/formal-model edits; the
  demo script untouched; EXTERNAL_REVIEW_PACKAGE.md and
  RELEASE_READINESS_EVIDENCE_MAP.md untouched; the public-safety gate untouched.

## T7 — Public-safety gate + goal-lint (PR-level)

- Step: the public-safety gate runs as the required PR check (and its scan is run
  locally against the PR via `scan-pr-changes`); goal-lint run locally via
  `scripts/audit/run_goal_lint_pr.sh <PR>`.
- Observed: recorded in the lane response at PR time. No gate amendment was made or
  needed.
