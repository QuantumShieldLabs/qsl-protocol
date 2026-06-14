Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0475 Closeout and NA-0476 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

This testplan validates the governance-only closeout of NA-0475 and restoration
of the exact NA-0476 successor selected by D-0938.

## Scope

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0475_closeout_restore_na0476_testplan.md`

Forbidden closeout mutation paths:

- qsc runtime/source paths.
- crypto code.
- dependencies, Cargo manifests, and lockfiles.
- workflows.
- executable tests other than this governance testplan.
- fuzz targets, vectors, and formal models.
- refimpl.
- qsl-server, qsl-attachments, qshield runtime, and qshield-cli runtime.
- website, public docs, README, and START_HERE.
- qwork/qstart/qresume/qshell.
- qsl-backup, backup status, backup plan, rollback subtree, and backup tree
  paths.

## Required checks

Run:

- `git diff --check`
- exact closeout scope guard over the five allowed closeout paths
- queue proof
- decision proof
- link check
- leak scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- root `cargo audit --deny warnings`
- nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`

## Expected queue result

- READY_COUNT 1.
- READY NA-0476.
- NA-0475 DONE.
- NA-0474 DONE.
- NA-0434 BLOCKED.
- NA-0429 BLOCKED.

## Expected decision result

- D-0938 exists once.
- D-0939 exists once.
- D-0940 absent.
- duplicate decision count zero.

## Expected successor

NA-0476 is restored as:

`NA-0476 -- QSL qsc KEM / Signature / Transcript Binding Negative Test Implementation Harness`

The exact selected executable test path remains:

`qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`

## Acceptance criteria

- NA-0475 is marked DONE.
- NA-0476 is the only READY item.
- D-0939 records closeout and successor restoration.
- TRACEABILITY records closeout evidence.
- No NA-0476 implementation is performed.
- No runtime/source mutation is introduced.
- No crypto mutation is introduced.
- No dependency, Cargo, lockfile, or workflow mutation is introduced.
- No refimpl, fuzz, vector, or formal model mutation is introduced.
- No public overclaim is introduced.
- No external-review-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free or perfect-crypto claim is introduced.
- No backup or restore is run.
- qsl-backup is not mutated.
