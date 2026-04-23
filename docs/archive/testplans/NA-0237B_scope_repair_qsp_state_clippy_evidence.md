Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-23

# NA-0237B Scope Repair Evidence — qsp/state Clippy Gate

Goals: G4

## Purpose

This archive evidence records why `NA-0237B` required a governance-only scope repair after the first local dependency-remediation implementation attempt stopped.

## Stopped local implementation proof

- Dirty implementation worktree: `/srv/qbuild/work/NA-0237B/qsl-protocol`
- Dirty implementation branch: `na-0237b-rustls-webpki-remediation`
- Dirty implementation base/head at preservation time: `385c99fcb52a`
- Preserved off-repo bundle: `/srv/qbuild/tmp/na0237b_scope_repair_preservation/`

The stopped local implementation attempt ran the lane's required validation command:

```bash
cargo clippy --locked -- -D warnings
```

That command failed in untouched out-of-scope file:

- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs:273`
- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs:296`

The failure class was `clippy::unnecessary_sort_by`, with the bounded fix shape being `sort_by_key(...)` in `qsp/state.rs`.

## Proof the dependency remediation itself is bounded

The preserved dirty worktree changed-path set is:

- `Cargo.lock`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0237B_dependency_advisory_remediation_testplan.md`

That changed-path proof shows the dependency remediation itself remained bounded to:

- one lockfile update (`Cargo.lock`)
- allowed governance/evidence companions

No runtime source, test source, Cargo manifest, workflow, sibling-repo, or website/public-runtime path was needed for the dependency update itself.

## Why scope repair was required

The local implementation attempt already proved:

- `RUSTSEC-2026-0104` is the live blocker on refreshed main
- a lockfile-only `rustls-webpki` update to the patched floor is workable
- `cargo audit --deny warnings` turns green on the local dependency-remediation branch head

The same attempt also proved the lane cannot finish truthfully under its current scope because the required `cargo clippy --locked -- -D warnings` gate depends on one untouched out-of-scope file:

- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`

No broader scope widening is supported by the local proof.

## Governance-only scope-repair statement

This scope-repair PR is governance-only.

It does not:

- change runtime semantics
- change protocol, wire, crypto, auth, or state-machine behavior
- change any Cargo manifest or lockfile
- change `.github/**`
- touch qsl-server, qsl-attachments, qsc-desktop, or website/public-runtime surfaces

It only repairs the live `NA-0237B` block text on `main` so the next implementation directive may truthfully apply the bounded clippy-only fix in `qsp/state.rs` if needed.
