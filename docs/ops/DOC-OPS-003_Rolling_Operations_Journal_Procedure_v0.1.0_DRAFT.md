Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# DOC-OPS-003 — Rolling Operations Journal Procedure v0.1.0 DRAFT

## 1. Purpose and authority posture

This procedure defines the checked-in rolling operations journal for qbuild-driven work.

The rolling journal exists to capture continuous operational memory about what happened during a directive: what was proven, what failed, what was recovered, what was validated, and what still needs watching.

Authority boundary:

- the governance spine remains authoritative;
- `NEXT_ACTIONS.md` remains the execution source of truth;
- the rolling journal is supporting operational memory only; and
- stable recurring lessons must be promoted into canon under `DOC-OPS-004` rather than treated as permanent journal-only lore.

The journal must never be used to reorder the queue, widen scope, justify runtime changes, or override refreshed repo truth.

## 2. When a journal entry is required

Maintain a rolling journal entry for every directive that:

- mutates a repository;
- opens, updates, or merges a PR;
- depends on bounded recoverable-failure handling;
- captures continuity, governance, or audit evidence; or
- would be difficult to resume truthfully from memory after a host or session interruption.

The journal should be updated continuously while the directive is active. It is not a post-hoc narrative written only after completion.

## 3. Required per-directive fields

Every rolling journal entry must include, at minimum:

- directive number/title
- begin/end timestamps
- repo SHAs
- READY proof
- worktree/branch/PR
- failures/recoveries
- validation/CI notes
- disk watermark
- next-watch items

Minimum meaning of each field:

- `directive number/title`: the exact directive or NA being executed
- `begin/end timestamps`: local and UTC timestamps used to bracket the work
- `repo SHAs`: active branch `HEAD` plus the governing `main` / remote refs needed to resume truthfully
- `READY proof`: the observed READY count and the exact item that authorized the work
- `worktree/branch/PR`: the qbuild worktree path, local branch name, PR number or URL, and merge commit when applicable
- `failures/recoveries`: every recoverable failure, why it was recoverable, what changed, and the final result
- `validation/CI notes`: local validation commands, protected-check status, and any bounded retries
- `disk watermark`: the filesystem, total/used/free GiB, and used percent for the host that carried the work
- `next-watch items`: open risks, pending observations, or follow-up evidence still worth monitoring

## 4. Cadence and update responsibility

The active operator owns the correctness of the rolling journal entry.

Update the entry:

- after the initial authority proof and worktree-state capture;
- after any recoverable failure or notable warning;
- after the first green local validation bundle;
- after branch push, PR creation, or merge-state changes;
- after any CI rerun, stale-check recovery, or merge decision; and
- at directive end with the final outcome and watch items.

If a session drops or a handoff occurs, the next operator must resume the existing entry or create a continuation entry that references the same directive, branch, and governing SHAs.

## 5. Off-host publication and storage expectation

The rolling journal must not exist only on qbuild when continuity matters.

At minimum:

- after the first green local validation bundle for continuity-sensitive or governance lanes, publish the current journal state into the off-host continuity package required by `DOC-OPS-002`;
- include the journal entry or an exported equivalent alongside the repo SHA inventory and overlay inventory; and
- ensure a future operator can reconstruct branch, PR, and recovery state without relying on chat memory alone.

An off-host copy may live inside a continuity snapshot package, but it must remain subordinate to live repo truth on resume.

## 6. Secret-handling prohibition

Rolling journal entries must not contain:

- tokens, passphrases, passwords, auth headers, or route tokens;
- secret-bearing URLs;
- copied secret files or env values; or
- long hex dumps that could be confused with live secrets.

Use short SHAs, descriptive pattern counts, redacted examples, and secret-manager references only.

## 7. Relationship to canon

The rolling journal is where one-off operational memory lives first.

It is not where stable rules should stay forever.

If an operational lesson becomes recurring, load-bearing, or necessary for truthful future execution, promote it into canon under `DOC-OPS-004` so the next directive does not depend on hidden memory.
