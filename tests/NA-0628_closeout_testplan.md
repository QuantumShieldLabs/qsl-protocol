# NA-0628 — closeout test plan (D-1252)

Goals: G1, G2, G4. Directive: QSL-DIR-2026-07-10-565 (D565) as amended by D565-A1. Impl decision D-1251.

## Merge record
- Impl PR: **#1536**, merged as **`e9439df7`** (merge commit; branch `na0628-eng0034-contributory-dh`).
- Base at open: `main == 1fdd5b9b`. Fast-forwarded local `main` to `e9439df7` after merge.

## PR-path gate (at merge; bounded REST polling, never `--watch`; merge commit only)
- **35 pass, 0 fail, 2 skipping by design** (`qsc-linux-full-suite`, `macos-qsc-full-serial` — the
  event-filtered full suites that run only on main-push). `mergeStateStatus: CLEAN`.
- All **14 required contexts** green, verified explicitly, including `suite2-vectors`, `goal-lint`,
  `formal-proverif-composition` (CI independently reproduced the NA-0627 gate — 17 assertions, sanity
  pair first), `public-safety`, and `CodeQL`.

## Post-merge verification — AT JOB LEVEL (a workflow can report success while a filtered job never ran)
Verified on `e9439df7`:
- `suite2-ci` → the `suite2-vectors` job ran and passed (the two new negative vectors executed on the
  required path).
- `formal-ci` → BOTH `formal-scka-model` and `formal-proverif-composition` jobs ran and passed.
- `qshield-ci` → the event-filtered full suites `qsc-linux-full-suite` and `macos-qsc-full-serial` ran
  at JOB level (not merely skipped), plus `ci-4b` / `ci-4d-dur`.
- `public-ci` → `public-safety` green (it WAITS on the push-only full suites, so its wall-clock exceeds
  their 60–105 min envelope — not a hang).
- `macos-build`, `qsc-adversarial`, `demo-packaging` → success.
- [FILL AT CLOSEOUT: paste the job-level conclusions once the three long runs settle.]

## Local gates (recorded at Phase 5)
- `cargo fmt --all -- --check` — PASS
- `cargo build --workspace --all-targets --locked` (WF-0013) — PASS
- `cargo clippy -p quantumshield_refimpl --all-targets -- -D warnings` — PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` — PASS
- `cargo test -p quantumshield_refimpl --locked` — PASS (89 lib incl. the anti-regression scan; all
  integration targets)
- `cargo test -p qsc --locked` — **591 passed, 0 failed, 3 ignored** (incl. 4 new establishment tests)
- `python3 formal/run_model_checks.py` — PASS (15,032 states / 9 shapes)
- `python3 formal/proverif/run_proverif_checks.py` — GREEN, 17 assertions, sanity pair first, UNCHANGED
- `scripts/ci/validate_suite2_vectors.py` — PASS
- vectors vs actor: pqreseed 12/12, scka-logic 21/21

## Mutation proofs performed (evidence in /srv/qbuild/logs/NA-0628/)
- Guards removed → all 4 refimpl no-mutation tests FAIL; both negative vectors FAIL (the send vector
  with "expects failure but actor returned ok" — the unguarded sender ratchets on a degenerate secret).
- Anti-regression scan → FAILS on (i) synthetic unguarded site, (ii) empty allowlist reason,
  (iii) count drift in an already-allowlisted file.
- WF-0014 byte-scan → FAILS on a flipped byte in an untouched file, an altered pre-existing vector
  inside an appended file, and a smuggled id. PASSES on the real tree (162 pre-existing byte-identical,
  cross-set `460f97e3…`).

## DoD closure
1. Every LIVE DH output checked for all-zero, fail-closed, no state mutation — **DONE** (6 sites).
2. Distinct reason code `REJECT_S2_DH_NONCONTRIBUTORY` registered via the bounded DOC-CAN-003 unfreeze
   — **DONE**.
3. Additive negative vectors + WF-0014 byte-scan — **DONE**.
4. ~~ENG-0019 retirement~~ — **STRUCK by D565-A1** (premise falsified); ENG-0019 unfolded, re-rated P2.
5. Q7 — **cannot be honestly strengthened; `formal/proverif/**` unchanged** (abstraction A4).
6. Claim-boundary draft — **DONE** (`NA-0628_claim_boundary_draft.md`); default NO CHANGE; operator
   decides.
7. D-1251 recorded; merged green; **D-1252 = this closeout.**

## Scope adherence
- Result boundary respected: no `apps/**`, `tools/actors/**`, `.github/**`, Cargo/lockfile, DOC-CAN-004,
  or KDF/AEAD/KEM change. Verified: the staged set contained zero forbidden paths.
- DOC-CAN-003: exactly two additions, zero deletions.
- One recorded deviation (not silent): `recv_dh_boundary` / `send_boundary` unreachable from any actor
  op → co-located Rust tests instead of vectors; a vector would need a forbidden `tools/actors/**` op.
- One recorded limitation: `cargo test -p quantumshield_refimpl` runs in no CI job → the anti-regression
  scan is a lane/local gate, not a PR gate; filed for the successor.

## Out of scope / not done (by design)
ENG-0019 (P2, unfolded); ENG-0035/Tamarin; ENG-0014; ENG-0032/0033; the NA-0627 CI-cost path-filter;
WF-0016/WF-0012. No claim moved.
