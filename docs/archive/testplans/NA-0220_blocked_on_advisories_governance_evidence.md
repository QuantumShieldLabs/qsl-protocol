Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-04

# NA-0220 Blocked on Advisories Governance Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Directive: `238`
- Posture: governance-only queue-truth repair
- Blocked substantive item: `NA-0220`
- Successor unblock item: `NA-0220A`
- Blocked PR: `#652`

## Why PR #652 Could Not Merge

- PR `#652` remains `OPEN` with head `1ebc856c5b5d` and `mergeStateStatus=UNSTABLE`.
- The bounded read-only handshake audit on that PR is not blocked by its changed files; it is blocked by a required protected context outside the PR's allowed write surface.
- The failing required context is `advisories`.
- Final check-run truth for the PR head was `34` total contexts, `33` green, `1` failure.

## Exact Blocker Context

- Required protected context: `advisories`
- Workflow source: `.github/workflows/public-ci.yml`
- Relevant workflow shape:
  - the job pins `toolchain: 1.84.0`
  - the install step runs `cargo binstall cargo-audit --version 0.22.0 --no-confirm`
- Failed job log proof:
  - fallback install reached `cargo install cargo-audit --version 0.22.0`
  - the job then failed because `cargo-audit 0.22.0` requires `rustc 1.85 or newer` while the runner was on `rustc 1.84.0`

## Evidence Sources

- `gh pr view 652 --repo QuantumShieldLabs/qsl-protocol --json state,headRefOid,mergeStateStatus,statusCheckRollup,url`
- `gh run view 23969882396 --repo QuantumShieldLabs/qsl-protocol --job 69916918174 --log-failed`
- `.github/workflows/public-ci.yml` lines covering the `advisories` job and `cargo-audit` install path

## Scope Classification

- This blocker is out of scope of `NA-0220` because that lane explicitly forbids `.github/**` edits and does not authorize workflow/toolchain changes.
- This governance repair does not change runtime semantics, protocol behavior, wire behavior, crypto, auth, state machines, or product priority.
- The repair only makes queue truth match live merge reality and promotes the narrow unblock lane needed to restore truthful progress.

## Queue-Truth Repair Result

- `NA-0220` moves from `READY` to `BLOCKED`.
- `NA-0220A` becomes the sole `READY` item.
- `NA-0220` remains the active substantive audit lane to resume or supersede cleanly after the unblock lands.
