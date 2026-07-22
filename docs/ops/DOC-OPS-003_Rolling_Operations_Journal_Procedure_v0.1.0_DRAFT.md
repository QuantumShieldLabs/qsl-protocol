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
- `failures/recoveries`: every recoverable failure, why it was recoverable, what changed, and the final result — **each recorded with the controlled marker defined in §3a**
- `validation/CI notes`: local validation commands, protected-check status, and any bounded retries
- `disk watermark`: the filesystem, total/used/free GiB, and used percent for the host that carried the work
- `next-watch items`: open risks, pending observations, or follow-up evidence still worth monitoring

### 3a. Recovery-event marker (FORWARD-ONLY, added by D-1292)

**Why this exists.** The `failures/recoveries` field has always been REQUIRED,
but the **marker was never specified** — so thirteen distinct phrasings bloomed,
and four reasonable framings applied to the same corpus returned **41 / 42 /
175 / 35** while counting one recurring hazard. **The full analysis is already
in the repository at `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` (the section
"⚑ A GOVERNANCE-OBSERVABILITY GAP, FOUND BY ACCIDENT: THIS JOURNAL CANNOT ANSWER
'HOW OFTEN HAS X HAPPENED'", recorded by NA-0664) and is NOT restated here.**
Read it there; this section is the rule that answers it.

**Shape.** Modelled on the `RF-###` precedent — the only stable identifier the
existing vocabulary produced (37 references across 8 ids, in the form "Recovered
proof issue RF-###"). Every recovery event gets one line:

```
- REC-<NNN> · <CLASS> · <class-key>: <what failed> — recoverable because
  <reason>; corrected by <action>; final result <result>.
```

- `REC-<NNN>` — monotonic **within the entry**, starting at `001`, as `RF-###`
  numbers already are. It makes an event citable ("REC-003 of the NA-0666
  entry") without implying a global registry that nothing maintains.
- `<CLASS>` — **exactly one of three**, which is the distinction the current
  vocabulary cannot express:
  - **`DEFECT`** — *a defect that bit.* Something known-broken, latent, or
    previously filed produced this failure. **A `DEFECT` marker should name its
    ledger ID where one exists**, and its absence from the ledger is itself
    worth noticing.
  - **`HAZARD`** — *a hazard correctly anticipated.* A known trap the lane
    expected and paid; the recovery worked as designed. **This class is the one
    that matters most for prioritisation and the one the old vocabulary hid
    best** — a hazard whose recovery always succeeds generates no pressure to
    fix itself, and counting these is how that becomes visible.
  - **`ONE-OFF`** — *unrelated and non-recurring.* Transient or external, with
    no standing defect implied. **Claiming `ONE-OFF` is a claim**; if the same
    `class-key` appears as `ONE-OFF` repeatedly, it was never one.
- `<class-key>` — a short stable kebab-case slug naming **the class of thing
  that failed**, reused verbatim across entries for the same underlying thing
  (for example `evidence-gitignore`, `commit-identity`, `gh-5xx`,
  `queue-parse`). **This is the part that makes counting possible:** the class
  is what recurs; the individual failure is not.

**What it buys.** `grep -c 'REC-.* · HAZARD ·'` counts anticipated hazards;
`grep -o '· [a-z0-9-]*:' | sort | uniq -c` ranks failure classes by frequency.
**"How often has X happened" becomes answerable from the record instead of from
the phrasing of the search.**

**⚠ LIMITATION, STATED PLAINLY: this makes the journal countable FORWARD ONLY.**
The ~44,000 lines of existing entries are **NOT retro-labelled** — that is a
separate and much larger job, and **mislabelling history is worse than leaving
it uncounted.** Any count derived from this marker is a count *since D-1292*,
and must be reported as such. The historical corpus remains exactly what
NA-0664 found it to be: excellent as narrative, unreliable as arithmetic.

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
