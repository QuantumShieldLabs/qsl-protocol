Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-21

# NA-0237 Blocked on Main — Send Commit Fallout Evidence

Goals: G4

## Summary

- PR `#708` contains the bounded `NA-0237` KT verifier implementation work and remains open on head `7f54ea7ab4ae`.
- That PR is not blocked by remaining KT ambiguity or an in-scope KT failure.
- The current blocker is latest `main` commit `9643c566b485`, which is red in the out-of-scope qsc `send_commit` seam after MockProvider retirement.
- The preserved KT work bundle at `/srv/qbuild/tmp/na0237_scope_repair_preservation/` remains present and usable.
- This PR is governance-only and does not change runtime semantics, wire semantics, or branch-protection semantics.

## PR #708 Truth

- State: `OPEN`
- Mergeable: `true`
- Merge state status from REST: `unstable`
- Base SHA: `9643c566b485`
- Head SHA: `7f54ea7ab4ae`
- Branch: `na-0237-kt-verifier-fail-closed-v2`

The PR head's direct implementation checks are otherwise green or accepted-neutral, but `public-safety` fails because it requires latest `main` to be green first.

## Current Main Blocker Proof

- Latest refreshed `main` SHA: `9643c566b485`
- Blocking required check path:
  - workflow run `24722275799`
  - job `72314416392`
  - required check `public-safety`
- Downstream red-main test path:
  - workflow run `24722275802`
  - job `72313939942`
  - check `macos-qsc-full-serial`

The failed `macos-qsc-full-serial` log shows the current `send_commit` fallout directly:

- failing test file: `tests/send_commit.rs`
- failing tests:
  - `outbox_commit_advances_once`
  - `send_failure_no_commit`
- failing command:
  - `qsc vault init --non-interactive --key-source mock`
- fail-closed marker emitted:
  - `QSC_MARK/1 event=error code=vault_mock_provider_retired`
- terminal test runner result:
  - `error: test failed, to rerun pass '-p qsc --test send_commit'`

This proves the current required-main blocker is the qsc `send_commit` seam's stale dependency on pre-retirement MockProvider behavior.

## Why This Is Outside NA-0237 Scope

The live `NA-0237` implementation scope is bounded to KT/refimpl/actor surfaces plus governance evidence. It does not authorize changes under:

- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**` except only if directly touched by bounded KT verifier vectors/regressions and justified by contradiction proof

The live blocker sits in qsc `send_commit` tests and any associated compatibility seam, so it is outside the KT verifier lane that PR `#708` implements.

## KT Preservation / Resume Proof

The preserved KT bundle under `/srv/qbuild/tmp/na0237_scope_repair_preservation/` still contains the required artifacts:

- `status.txt`
- `changed_paths.txt`
- `diffstat.txt`
- `tracked.patch`
- `untracked.zlist`
- `untracked.tgz`
- `head_sha.txt`

This means the local KT implementation WIP remains resumable even though the queue now truthfully records the unrelated main blocker first.
