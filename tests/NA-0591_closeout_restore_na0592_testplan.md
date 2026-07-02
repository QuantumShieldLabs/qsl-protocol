Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-02

# NA-0591 closeout and NA-0592 restoration testplan

## Scope

This testplan records closeout-only governance for NA-0591 after D521 and PR
#1456. It marks NA-0591 DONE, records D-1174, and restores NA-0592 as the sole
READY successor. It does not implement NA-0592, does not mutate qsc source, does
not mutate qsl-server, does not mutate or integrate qsl-attachments, does not
perform remote/Tailscale/workflow action, and does not change dependencies or
lockfiles.

## Required Markers

- NA0591_CLOSEOUT_D521_CONSUMED_OK
- NA0591_CLOSEOUT_D1173_ACCEPTED_OK
- NA0591_CLOSEOUT_PR1456_POSTMERGE_GREEN_OK
- NA0591_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
- NA0591_CLOSEOUT_ADVISORIES_GREEN_OK
- NA0591_CLOSEOUT_NO_FAILED_REQUIRED_CHECKS_OK
- NA0591_CLOSEOUT_RESULT_CLASSIFICATION_ACCEPTED_OK
- NA0591_CLOSEOUT_D1174_RESTORED_NA0592_OK
- NA0591_CLOSEOUT_NA0591_DONE_OK
- NA0591_CLOSEOUT_NO_NA0592_IMPLEMENTATION_OK
- NA0591_CLOSEOUT_NO_QSC_SOURCE_MUTATION_OK
- NA0591_CLOSEOUT_NO_QSL_SERVER_MUTATION_OK
- NA0591_CLOSEOUT_NO_QSL_ATTACHMENTS_INTEGRATION_OK
- NA0591_CLOSEOUT_NO_REMOTE_TAILSCALE_WORKFLOW_ACTION_OK
- NA0591_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK
- NA0591_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK
- NA0591_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0591_CLOSEOUT_NO_CRYPTO_COMPLETE_CLAIM_OK
- NA0591_CLOSEOUT_NO_TRIPLE_RATCHET_COMPLETE_OVERCLAIM_OK
- NA0591_CLOSEOUT_ONE_READY_INVARIANT_OK

## Evidence Requirements

- Verify fresh qwork proof before fetch, repository mutation, GitHub polling,
  source-analysis result publication, or proof publication.
- Verify live pre-fetch `HEAD` and `origin/main` match qwork proof.
- Verify worktree, index, and untracked state are clean before closeout edits.
- Verify disk and mount gates before fetch.
- Verify D521 / PR #1456 implementation inheritance.
- Verify D-1173 exists once, is Accepted, and selected exact NA-0592 successor.
- Verify D-1174 and D-1175 are absent before closeout patch.
- Verify PR #1456 post-merge public-safety and advisories are green.
- Verify no failed required checks remain.
- Verify result classification
  `TRUE_TRIPLE_RATCHET_DEMO_OR_FIXTURE_BYPASS_FOUND` is accepted.
- Mark NA-0591 DONE.
- Restore exactly one READY successor:
  `NA-0592 -- QSL qsc True Triple-Ratchet E2EE Hardening / Bug Fix Authorization Harness`.
- Verify no NA-0592 implementation occurred.
- Verify no qsc source mutation occurred in closeout.
- Verify no qsl-server mutation occurred in closeout.
- Verify no qsl-attachments integration or mutation occurred in closeout.
- Verify no remote, Tailscale, or workflow action occurred.
- Verify no dependency or lockfile mutation occurred.
- Verify no private material was published.
- Verify no public-readiness, production-readiness, crypto-complete, or
  triple-ratchet-complete claim is introduced.

## Validation Commands

- `git diff --check`
- exact five-path scope guard over tracked, staged, and untracked changes
- queue/decision proof
- marker proof
- markdown link check
- added-line/new-file private-material scan
- secret/prohibited-material scan
- overclaim scan
- crypto/triple-ratchet claim-boundary scan
- docs/governance-only classifier
- PR body preflight
- goal-lint when available
- root `cargo audit`
- nested qsc fuzz `cargo audit`
- `cargo metadata --locked --format-version=1`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Focused qsc runtime tests may be skipped because this is closeout-only and no
qsc source/test/runtime/dependency/workflow mutation or NA-0592 implementation
occurs.

## Expected Result

NA-0591 is DONE, D-1174 is Accepted, and NA-0592 is the only READY item. The
seed fallback/demo-fixture shortcut remains deferred to NA-0592 for
authorization and hardening, and full qsl-attachments send/receive integration
remains deferred until that hardening decision is complete.
