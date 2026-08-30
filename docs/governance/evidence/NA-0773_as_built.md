# NA-0773 — AS BUILT — THE RNG-FAILURE TEST SEAM: ONE LINE, AND THE 27 RUN

**Decision:** `D-1416`. **Base:** qsl-protocol main `c6687bbdd19927e3f3ed3e0f63f8c1672b804730`,
re-derived bare and unpiped at the NAMED `github` remote. **Product change: ONE LINE.**

## ESCALATIONS (PR-17)

**THREE, ALL OPEN, NONE CLOSED BY THIS LANE.**

**`E-1` — the dark fail-safe surface.** 27 tests that claim to prove the product writes no partial
key material, identity, session or vault state under RNG failure had **not compiled for 46 days**.
**Now repaired and run.** *Severity:* compile break `P3`. *Unknown, stated as unknown:* whether the
product fails safe under a real RNG failure is **still UNKNOWN** — see `E-3`. *Filed:* `ENG-0197`,
`ENG-0256`, both amended by this lane.

**`E-2` — the seam is label-gated, so unwrapped draws are dark by construction.** Raised by this
seat as a suspicion, **answered by the SR-15 cold read as a measurement**: **13 shipped randomness
draws the seam cannot reach**, including **the vault master key** (`vault/mod.rs:1476`) and both
persisted store keys, plus 5 in `quantumshield_refimpl`, which carries the seam token **zero times**;
and **5 of 22 wrapped labels are never forced**. *Severity:* coverage gap `MAJOR`; **the product is
ungraded — no defect was found at those sites, only the absence of any test that could find one.**
*Filed:* `ENG-0266`.

**`E-3` — the seam cannot reach a real RNG failure at any site, wrapped or not.** Every draw goes
through `rand_core`'s `OsRng::fill_bytes`, which **panics**; `try_fill_bytes` occurs **0** times
tree-wide; the seam returns `Err` *instead of* drawing; and `assert_no_secret_output`, reached by
**10 of the 19**, asserts `"panicked"` is **absent**. *Severity:* **the product UNGRADED** (no defect
observed; the panic consequences are **reasoned, not observed**); **the assurance claim in our own
records `P2`** — a load-bearing false statement about a security property. *What it would take to
know:* one fallible RNG helper every draw passes through (a design lane, crypto region — it would
close `E-2` too), or an OS-level `getrandom` fault harness. *Filed:* `ENG-0265`.

⚠ **AND ONE MEASURED DEFECT THAT IS NOT AN ESCALATION BUT A FINDING:** `ENG-0269` — when the A2
signature fails, the initiator commits the session, emits `handshake_reject`, and **acks the frame
anyway**. Found by this lane's own run, on the first occasion the arm was ever able to execute.

## 1. THE CHANGE

```diff
 #[cfg(qsc_rng_failure_test_seam)]
-fn generate_default_route_token() -> CliResult<Result<String, &'static str>> {
+fn generate_default_route_token() -> Result<String, &'static str> {
     let mut bytes = [0u8; 16];
     vault_rng_fill("QSC.VAULT.INIT.DEFAULT_ROUTE_TOKEN", &mut bytes)?;
```
`qsl/qsl-client/qsc/src/vault/mod.rs:726` — **one file, one insertion, one deletion.** The exact
byte-inverse of the hunk `ba141149` (NA-0646 PR-B, PR #1574, 2026-07-15) introduced. **Strictly
inside the seam arm:** gate `:725` < target `:726` < `not()` twin `:736`, each asserted from the
file's own bytes before the edit, with the target line proven **unique in the file**.

## 2. WHY — ONE CAUSE, FOUR ERRORS

`pub type CliResult<T=()> = Result<T, CliError>`, so the declared type was
`Result<Result<String,&str>, CliError>` and all four compiler errors fall out of that single fact:
`:733` body returns `Ok(out)` with `out: String` · `:728` `vault_rng_fill(..)?` yields `&'static str`
and no `From<&str> for CliError` exists · `:578` the call site binds `token: Result<String,&str>`
where a `String` is required · `:570` it binds `code: CliError` where `fail_core_buffers` takes
`&'static str`. `vault_init_core` returns `Result<(), &'static str>`, and **the seam arm was the only
thing in the region mentioning `CliError`.**

## 3. WHEN IT BROKE — BISECT

`ba14114919a4dea56b323dee9592720f2aa0b60a`, **2026-07-15**, NA-0646 PR-B, **PR #1574**. `git bisect`
over **1498 commits**: **11 steps, ZERO skips**, every arm either 0 errors or exactly the four —
monotonic, which also rules out an earlier break that was fixed and re-broken. Bracket exact
(`ba141149` has one parent). **Gap: 46 days.** ⚠ The hypothesis on record — NA-0649 / `6a93012c` —
is **REFUTED**; it reasoned from a *last-touched* line, and the break is at the **signature**.

## 4. THE ARMS, EXPECTATION WRITTEN BEFORE EACH RUN

| arm | before | after |
|---|---|---|
| `RUSTFLAGS="--cfg qsc_rng_failure_test_seam" cargo check -p qsc --lib` | **101**, 4 errors | **0** |
| `RUSTFLAGS="--cfg qsc_rng_failure_test_seam" cargo check -p qsc --tests` | **101** (halts on the lib) | **0**, 2 unrelated warnings |
| `cargo check -p qsc --lib` (ordinary) | **0** | **0** |

## 5. THE 27 — RESULT BY NAME

Full table at `tests/NA-0773_rng_failure_seam_repair_testplan.md` sec 4.
**27 specified · 27 executed · 26 passed · 1 FAILED · 0 ignored · 0 not observed**, keyed by unique
target path and census-reconciled (`PR-7`). The failure is
`t8_the_a2_sig_failure_exit_emits_no_producer_ack`, **both antecedents passing**; re-run alone and
single-threaded under a rule written before the re-run (*fails alone = real*), it **failed again**.
⇒ **`ENG-0269`, argued `P2`, FILED and NOT REPAIRED** per `RULING_NA0773_002`.

## 6. THE FINISH LINE, AND WHAT A GREEN RUN DOES NOT PROVE

As ruled: **"the 19 have run and the Err-path cleanup behaviour is recorded."** A green run proves
the seam-arm cleanup executes when reached by an `Err` return. It does **not** prove fail-safe
behaviour under a real RNG failure (`ENG-0265`), does **not** cover the 13 unwrapped draws
(`ENG-0266`), and **the coverage figure is 16**, not 27 — 27 declared, 8 assert nothing, 1 is not an
RNG test, 1 pins a surviving partial write, 1 pins a different property by design.

## 7. THE ORDINARY FULL SUITE — INSTRUMENT AND EXPECTATION (result recorded separately, SR-22)

`cargo test --workspace --no-fail-fast`, ordinary build (**no cfg**), **once**, on a **frozen
worktree at this branch's final commit**, every target's own exit read unpiped, the executed count
**reconciled against the census of test binaries** enumerated by `cargo test --workspace --no-run`.
**Expectation, written before the run:** the suite is unaffected by this change, because the edit is
unreachable from a default build (sec 1) — any nonzero target is re-run **alone** under the rule
*passes alone = contention; fails alone = real*.
⚠ **The figure is deliberately NOT written in this pass** (`SR-22`: never write a figure about an
artifact in the same pass that creates it). It is recorded in the lane's stop and in the PR.
⚠⚠ **AN EARLIER LAUNCH OF THIS SUITE WAS DISCARDED BY THIS SEAT** — it was started and then had its
tree moved under it by two branch switches, so its results were not attributable to any commit. The
delta was records-only and no compiled artifact differed, **but it is discarded on principle, not on
effect**, and its output is preserved rather than deleted. See `D-1416` `DV-15`.
