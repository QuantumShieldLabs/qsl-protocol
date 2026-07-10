# NA-0628 Phase 7 — successor triage (WF-0003). PROPOSED, NOT PROMOTED.

The executor cannot self-promote a lane. NA-0629 becomes READY only when the operator approves its
directive. Ranked, with the reasoning that changed since D565's proposed successor block.

## What changed the ranking

D565's successor block said: *"If NA-0628 closes ENG-0034 + ENG-0019 cleanly, the post-compromise claim
language becomes decidable … the strongest next candidates are WF-0016 + WF-0012, ENG-0014, or the
NA-0627 CI-cost path-filter."*

Two of its premises are now false:
1. **ENG-0019 did not close.** It was unfolded, and it grew: it is now P2, and its retirement is a
   product decision touching branch protection and a release artifact.
2. **This lane discovered a live authority problem** (`refimpl_actor` is provenance-attested and
   shipped, its KT verifier is an empty pinned log) that did not exist in the ledger yesterday.

## Ranked candidates

### 1. ENG-0019 (P2) — the auth-unsafe reference implementation that CI blesses and releases attest
**Recommended as the successor.** It is the only P2 that this lane *raised* rather than closed, and its
(d) sub-item is the cheapest real risk reduction available anywhere in the backlog.

Menu for the directive (operator chooses; they are not mutually exclusive):
- **(d) stop shipping `refimpl_actor` in `release_artifacts/`** — one line in `release-auth.yml`.
  Release-only, so it cannot affect the required PR checks. **Independent of (a)–(c). If the operator
  wants only one thing from this list, take this one.** `.github/**` → needs authorization; a one-line
  LITE lane suffices.
- **(a) banner** — `//! NOT PRODUCTION — auth-unsafe (ENG-0019)` + module docs. Cheap, no build impact.
- **(b) type extraction** — move `RatchetError`, `HandshakeInit`, `PrekeyBundle` out of `qsp/` into a
  neutral module, so `qsp` *can* be feature-gated at all. Prerequisite for any gating.
- **(c) retire Suite-1/1B conformance** — a **product** decision: branch protection (`ci-4b`,
  `ci-4d-dur`), `tests/harness/4b/**`, `scripts/ci/durability_4d.py`, `actor_contract.md`, and the
  actor's advertised `suites`. Large. Do not fold into (a)/(b)/(d).

### 2. The refimpl CI-coverage gap (filed by this lane, D-1251)
`cargo test -p quantumshield_refimpl` runs in **no** CI job. The anti-regression scan and the four
no-mutation guard tests are lane/local only. Remedy: one line in an existing workflow job. **Pairs
naturally with ENG-0019(d)** — both are single-line `.github/**` edits, and both are about making CI
tell the truth. Batching them into one LITE lane is the efficient move.

### 3. WF-0017 + WF-0016 + WF-0012 — the process-debt batch
WF-0017 is brand new and its cost is one box in the DOC-OPS-006 directive template plus a mechanism
list in AGENTS.md. It has already cost this lane one STOP and one amendment, and it cost NA-0627's
Director turn a wrong directive. WF-0016 (handoff protocol) and WF-0012 (`ledger.py`) share the "stop
hand-maintaining structured state in markdown" theme. One docs/process + tooling LITE lane.

### 4. ENG-0014 (P2) + the constant-time family (ENG-0003/0005/0008/0015)
qsl-server non-constant-time token compare, cross-repo, cheap, with a Signal-Server precedent
(`MessageDigest.isEqual`). Unchanged in priority; still a good short lane when a slot opens.

### 5. ENG-0032 / ENG-0033 (one LITE lane)
apps hygiene + the public-safety gate's cancelled-vs-failed conflation. Unchanged.

### 6. NA-0627 CI-cost path-filter
The `formal-proverif-composition` job measures 24.1 min and is additive/non-required, so it cannot
wedge the repo. Path-filter it, or keep only the sanity pair on the PR path. Low urgency.

### 7. ENG-0035 / Tamarin (P3)
Only if the 2-epoch unrolling is judged load-bearing. **It becomes load-bearing the moment the operator
wants claim C3** (post-compromise), per the claim-boundary draft: a review of a model with a known
non-terminating query is a review of the wrong artifact. Otherwise it stays P3.

## Recommendation

**NA-0629 = ENG-0019, scoped to (d) + (a) + (b), with (c) explicitly deferred**, batched with the
refimpl CI-coverage one-liner (#2). That combination: removes the attested-binary amplifier, makes the
skeleton unmistakable, unblocks any future gating, and makes the guard's own regression scan run on
PRs — all without touching Suite-1/1B conformance or branch protection semantics.

If the operator prefers a claim-boundary push instead, the honest ordering is **ENG-0035 → independent
human review → revisit C3**, and ENG-0019 still wants its (d) line regardless.
