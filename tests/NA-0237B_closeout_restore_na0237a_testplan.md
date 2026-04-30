Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237B Closeout and NA-0237A Restoration Testplan

Goals: G3, G4

## Objective

Close `NA-0237B` from already-merged PR `#713`, repair the known historical duplicate decision IDs `D-0240` and `D-0241`, restore `NA-0237A` as the sole READY item, keep `NA-0237` / PR `#708` blocked, and capture `NA-0238` as BACKLOG only.

## Protected Invariant

- `cargo audit --deny warnings` remains green on refreshed `main`.
- `public-safety` and fail-closed advisory behavior are not weakened.
- PR `#708` and the preserved `NA-0237A` WIP are not modified.
- Exactly one READY item exists after the patch: `NA-0237A`.
- No runtime, protocol, crypto, demo, dependency, workflow, service, KT, SCKA, client, relay, or state-machine files are changed.

## Scope Guard

Allowed changed paths for this directive:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0237B_closeout_restore_na0237a_testplan.md`

Forbidden changed paths include `.github/**`, `scripts/**`, Cargo manifests and lockfiles, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, PR `#708`, and the preserved `NA-0237A` worktree.

## Known Historical Duplicate-ID Repair

Pre-patch parser expectation:

- `D-0240` count: 2
- `D-0241` count: 2
- no other duplicate decision IDs
- `D-0435` and `D-0436` unused

Post-patch parser expectation:

- `D-0240` count: 1
- `D-0241` count: 1
- `D-0435` count: 1
- `D-0436` count: 1
- no duplicate decision IDs

Canonical preserved entries:

- `D-0240`: 2026-02-16, `NA-0141` / PR `#375` visual polish follow-up
- `D-0241`: 2026-02-17, `NA-0142` / PR `#379` System Account + Results + destroy

Repaired entries:

- `NA-0214`: duplicate `D-0240` repaired to `D-0435`
- `NA-0214A`: duplicate `D-0241` repaired to `D-0436`

The repair changes identifiers and references only; it changes no runtime or security semantics.

## Queue Parser Proof

Pre-patch expected state:

- `READY_COUNT 1`
- `READY NA-0237B`
- `NA-0237A BLOCKED`
- `NA-0237 BLOCKED`
- `NA-0237C DONE`
- `NA-0237D DONE`

Post-patch expected state:

- `READY_COUNT 1`
- `READY NA-0237A`
- `NA-0237B DONE`
- `NA-0237 BLOCKED`
- `NA-0237C DONE`
- `NA-0237D DONE`
- `NA-0238 BACKLOG` only
- no other READY items

## PR #713 Merge Proof

- PR `#713`: merged
- final validated head: `cef21a4c4d8199ea847217f5299b8500a8b278c9`
- merge commit: `81f6523e26651c621a71b2cd97d57cfa48ced6c6`
- merged remediation paths included `Cargo.lock`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, `tests/NA-0237B_dependency_advisory_remediation_testplan.md`, and `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`

## Cargo Audit Proof

Run on refreshed `main` before and after this governance patch:

```bash
cargo audit --deny warnings
```

Required result: pass.

## rustls-webpki Patched-Floor Proof

Run:

```bash
cargo tree -i rustls-webpki --locked
```

Required result: `rustls-webpki v0.103.13` or higher, and no `0.103.12` lockfile path.

## PR #708 Untouched Proof

Run:

```bash
gh pr view 708 --json number,state,headRefName,headRefOid,title,url
```

Required result: PR `#708` remains `OPEN` at `7f54ea7ab4ae7347af4655183dfb24188cf1a8ce`.

## NA-0237A WIP Untouched Proof

If `/srv/qbuild/work/NA-0237A/qsl-protocol` exists, inspect read-only:

```bash
git -C /srv/qbuild/work/NA-0237A/qsl-protocol status --porcelain=v1 --branch
```

Do not clean, reset, checkout, stash, or modify that worktree. Report the observed status, including dirty state if preserved.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --check
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
python3 tools/goal_lint.py
git ls-files 'tests/*.md' | wc -l
git ls-files 'tests/**/*.md' | wc -l
git ls-files 'docs/*.md' | wc -l
git ls-files 'docs/**/*.md' | wc -l
```

Run the repository markdown link-integrity check and any established leak/secret scan for governance evidence PRs without modifying validators.

## Required CI Context Expectations

Required contexts must satisfy branch protection on the exact PR head:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

All listed contexts must succeed except `CodeQL`, which may be accepted as neutral only if GitHub treats it as satisfied on the exact PR head.

## Roadmap Deferral

`NA-0238` captures the engineering-velocity roadmap request as BACKLOG only. This directive intentionally does not create `ROADMAP.md` or any full engineering policy artifact.
