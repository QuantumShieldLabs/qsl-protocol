# NA-0627 — closeout test plan (D-1250)

Directive: QSL-DIR-2026-07-09-564 (D564). Implementation decision: D-1249. Closeout: D-1250.
Closes **ENG-0028**. Files **ENG-0034** (P2) and **ENG-0035** (P3).

## What the closeout mutates

`NEXT_ACTIONS.md` (LIVE QUEUE header; the NA-0627 block → `Status: DONE` + OUTCOME; the proposed
successor block), `DECISIONS.md` (D-1250), `TRACEABILITY.md` (closeout entry),
`docs/ops/IMPROVEMENT_LEDGER.md` (ENG-0028 → CLOSED with PR/merge fill-in; ENG-0034/0035 PR
fill-in), `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this file.

**No** source change; **no** vector change; **no** canonical-doc change; **no** Cargo/`.github`/
`.claude` change; no runtime/LAN action; no operator startup command; no branch-protection or
repo-settings mutation. The executor installed no package at any point in this lane.

## Implementation gates (Phase 5, PR #1533) — recorded

Derived mechanically from the workflows this lane touched (`.github/workflows/formal.yml` only):

1. `python3 formal/run_model_checks.py` — the five bounded Python explorers, **untouched** by this
   lane. GREEN locally (root-composition model: 15,032 states / 21,512 transitions / 9 regression
   shapes) and GREEN in CI as `formal-scka-model`.
2. `python3 formal/proverif/run_proverif_checks.py` — the new ProVerif gate. GREEN locally
   (`gate_exit=0`; 15 expected `RESULT` lines) and GREEN in CI as the additive
   `formal-proverif-composition` job.
3. `goal-lint` — GREEN. (`Goals: G1, G2, G4` in the PR body; `TRACEABILITY.md` + `DECISIONS.md`
   ride the diff. Note: the lane touches **no** core path under goal_lint's `CORE_PATH_PATTERNS`,
   so only the Goals-line rule binds; the governance docs were included regardless.)
4. Scope guard — `git diff --stat` shows **zero** `*.rs`, `inputs/**`, `docs/canonical/**`,
   `Cargo*`, or `.claude` paths; `.github/**` limited to the single additive formal job; the
   existing `classify` and `formal-scka-model` jobs are byte-unchanged. Because no Rust file is
   touched, the fmt/build/clippy gates do not apply to this lane.
5. PR-path checks at merge: **CLEAN** (`running == 0`, `fails == 0`), bounded REST polling, never
   `--watch`; merge commit only. The two event-filtered full suites (`qsc-linux-full-suite`,
   `macos-qsc-full-serial`) are **skipped-by-design on the PR path** and are verified at job level
   on the main-push run (Phase 6).

## Gate-integrity mutation tests (run against temporary copies; nothing committed)

*A green gate that cannot fail is not a gate.* Both were executed against the final runner:

- **A — a wrong expectation must fail the gate.** Corrupting one expected `RESULT` regex for
  `suite2_dhpq_q5_dh_healing.pv` → runner exits **1** with `[MISSING]` + `[FAIL] 1 expected RESULT
  line(s) absent.` PASS.
- **B — a LYING verifier must stop the gate before any protocol model runs.** Simulating a negative
  control that returns `is true.` (by making it not leak) → runner exits **1** at the sanity pair
  with `[STOP] The tool sanity pair failed`, and `grep -c suite2_dhpq` on the output is **0**, i.e.
  **no protocol model executed**. PASS.

Evidence: `gate_mutation_A_wrong_expectation.out`, `gate_mutation_B_lying_verifier.out` in the proof
root. A note on why the runner asserts **regexes** rather than plain substrings: ProVerif renames
bound query variables (`n_28`, …) as a model evolves, so a substring assertion keyed on a generated
name would later fail for a reason unrelated to security. The semantics are pinned exactly — which
event implies which, and `is true.` versus `is false.`

## Query results (class-only; raw outputs stay proof-root-only)

Q1 secrecy **proved** · Q2 injective agreement **proved** · Q3 PQ healing after a reseed **proved** ·
Q4 PQ healing after the combined boundary **proved** · Q5 classical healing after a DH boundary
**proved** · Q6 planted ADV never tracked **proved** · Q7 guard-form (both arms) **proved, and NOT
an attack-existence proof**.

**Q3+Q4+Q5 together are the hybrid claim.** Canaries (`m0` readable pre-heal; `m_rs`/`m_cb`
readable under full classical compromise) all returned **`is false.` as required** — the compromises
are real, and the gate asserts those reds so it cannot pass on a model that compromised nothing.

**No query disproved a security property of the shipped composition.** D564's STOP rule was not
triggered.

## Post-merge verification (Phase 6)

- `main` fast-forwarded to the merge commit; working tree clean.
- All main-push workflows on the merge commit SUCCESS, **including the two event-filtered full
  suites at JOB level** (`qsc-linux-full-suite`, `macos-qsc-full-serial`) plus `formal-ci` (both
  jobs: `formal-scka-model` and the new `formal-proverif-composition`), `public-ci`, `suite2-ci`,
  `qsc-adversarial`, `demo-packaging`, `CodeQL`.
- `python3 formal/run_model_checks.py` and `python3 formal/proverif/run_proverif_checks.py` re-run
  against the merged tree.

## Claim boundary

**UNCHANGED** (Operator Decision 4's default). A green ProVerif result is **necessary input to**,
not **sufficient grounds for**, any post-quantum / Triple-Ratchet / post-compromise claim: it is a
symbolic result over abstracted primitives against a Dolev-Yao adversary and says nothing about
computational hardness, side channels, or this implementation. **Independent human review remains an
open prerequisite.** Candidate sentences are DRAFTED in `docs/design/DOC-G4-002` §7 **for the
operator to decide**; the executor moved no claim. **ENG-0034 independently blocks post-compromise
language** until a contributory check exists.

Still **NO** public-readiness, production-readiness, security-completion, crypto-complete,
attachment-complete, bug-free, vulnerability-free, post-quantum, Triple-Ratchet, or post-compromise
claim. No endpoint, token, capability, key, seed, plaintext, ciphertext body, or raw private material
is published; raw private values remain proof-root-only.
