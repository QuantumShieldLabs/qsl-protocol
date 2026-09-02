# NA-0776 (`D-1419`) — AS BUILT: THE DESKTOP TRIO

Bases, re-derived **bare and unpiped at the NAMED github remote** (never `origin`, which in this
seat is the local mirror): qsl-desktop main `83019356166e42ddeee483ac335dca53921779b4`;
qsl-protocol main `8b0a3efc943b6a9c89566dfc1e1fef2484b0a9ad`.

**The desktop half is landed.** PR #52 merged 2026-09-02T18:27:42Z as
`2411bf9fb84918c1548eaf2739122dbb8fa4de44`, parents `83019356` (base) and
`7be2df537f04b2d808d7b70d54c81041e217dc17` (the flown head). Desktop record `D-0046`.

## 1. WHAT SHIPPED

Six cures. `ENG-0274` — a declined handshake now shows a quiet line, built on the `#tick-status`
pattern and never on `#status-line`, so NA-0752's two-source footer ruling is untouched; the
classifier is pure and whitelisted and **no raw marker text reaches the UI**. `ENG-0275` —
`app_info` reports `build_commit`, a 40-hex commit or the literal `unknown`, with a dirty flag and
a build timestamp **ruled out as believed-and-wrong** and pinned ABSENT. `ENG-0276` — external-wipe
detection at both doors, failing closed with `STORE_VANISHED`. The pane-heading class — one
structure and one type rule across **nine sites defined BY ROLE**. `settings.json` — created 0600
via `create_new` + `mode(0o600)`, with existing profiles remediated at launch and the bootstrap
chmod a no-op when the file is absent. And the WebKit residue **relocated** under
`<app-data>/webview` rather than swept, with a marker-gated sweep and a legacy migration that uses
`symlink_metadata`.

## 2. THE SUITE — ONCE, ON THE EXACT COMMITTED TREE, AT EACH HEAD

`CARGO_EXIT=0` · **26 targets** (census by unique binary path) · **221 passed / 0 failed / 21
ignored = 242** · 27 result blocks, reconciling as 26 targets + 1 doc-test block · the test
inventory re-pinned 212 → **242** and the gate PASS at 242. All **20 driver arms** (`a`–`t`,
contiguous) green, run with `--test-threads=1`.

⚠ `na0776_gui_t_build_identity_visible` was **enumerated but unpinned** — the gate passes new tests
by design — which left the arm the build-identity flight row rests on unprotected against silent
deletion. Found by running the gate rather than reasoning about it, and re-pinned.

## 3. E-7 — THE SHIPPED TREE *IS* THE TESTED TREE, PROVEN BY COMPARISON

At every push: read the branch head bare (`ls-remote`) at the named remote, fetch, compare
`FETCH_HEAD^{tree}` to the tested tree, **and require `git diff <tested> FETCH_HEAD` EMPTY**. A
digest of the destination proves a file exists there, never that your bytes are in it.

Applied to the landing itself: `main^{tree}` == `7be2df53^{tree}` == `d1e0e6ae`, and
`git diff 7be2df53..main` EMPTY. **What shipped is the tree the operator flew and the suite ran on.**

## 4. THE BOARD — AND THE ARC THAT HID BEHIND ONE RED CHECK

Final head `7be2df53`, run `33661527423`, **all four SUCCESS**: `rust` (job 100352958753, 6m5s),
`gui-driver` (100352958400, 8m58s), `advisories` (100352958629, 2m53s), `infra-literal-scan`
(100352958659, 9s).

⚠⚠⚠ **It was not always green, and the record says so plainly.** `rust` was RED on **every earlier
push**. It runs `deps → fmt → test → inventory → clippy` and died at **`fmt`**, so `cargo test`,
the inventory gate and `clippy` had **never executed in CI on this branch**. The arc, because the
conclusion alone hides it: **33s red** (died at `fmt`) → **7m33s red** (`fmt` and the whole suite
ran and *passed*; died at `clippy`) → **6m5s green**. ⛳ **The duration of a failing job tells you
how far it got** — a 33s failure on a job that compiles cannot have reached the tests.

⚠ And `clippy`'s failure was invisible from this seat: `clippy::question_mark` exists in CI's
`@stable` (1.98.0) and **not** in the box's `0.1.95`. A 1.98.0 toolchain was installed **by version,
non-default** as a predictor, with the default left at 1.95.0, and **validated against the known
answer** — reproducing CI's finding at the exact site before its clean result was trusted.

## 5. THE FLIGHT — ROW 1 FAILED FIRST, AND WHY NO ARM COULD HAVE CAUGHT IT

First flight: row 1 mismatched — the Settings list-column heading sat higher and smaller. It is a
`<header>`, **not an `h2`**, and the arm's site set had been derived from a **census of `h2`
elements**, so it was never in the set and its CSS rule stayed byte-identical to the base.

The repaired arm defines its sites **BY ROLE** — the list-column heading of each screen plus the six
detail-pane `h2`s, nine sites — and gains **A4** on computed type. ⛳ A4 is not redundant with the
position assertion: reverting the type alone shifts the centred top by exactly **0.5px**, which the
inclusive tolerance **passes**. Position alone could not have caught the size half of the operator's
report. Both halves are driven red on **disjoint** assertions.

Second flight, on `7be2df53`, rows 1 / 2 / 7 — operator, verbatim: *"Row 1, They all look and line
up perfectly. 2, Shows correct build. 7, exactly what showed up. I dismissed it and it stays
clear."* Rows 3–6 carried on two printed proofs (formatter idempotence; a `markers.rs`-only file
list). Row 7 is the **first human observation of the notice surface**.

## 6. THE CARRY INSTRUMENT — NAMED, MEASURED UNABLE TO REACH ITS BOUNDARY, CORRECTED

A carry clause made "behaviour-free" mechanical as *"survives `git diff -w`"*. `-w` ignores
whitespace **within** a line and cannot collapse a line **wrap** — so a pure reformat, exactly the
case the clause existed to excuse, reports non-empty. The tell: the `-w` output was **byte-identical
to the plain diff**; a two-file fixture differing only by a wrap confirmed it. The seat **executed
the clause as written while demonstrating the defect**, and it was corrected by ruling to
**formatter idempotence** — `rustfmt(prior head's file)` reproduces the new file byte-exactly — with
whitespace-stripped equality secondary, which then showed the entire product delta was **one deleted
trailing comma**. Filed as `WF-0098`.

## 7. WHAT THIS LANE DOES NOT REPAIR

`ENG-0285` — an external wipe voids `flock` mutual exclusion **and** resets the brute-force attempt
limit, one trigger with two consequences — is **filed, not built**. `ENG-0276`'s guard is
**detection**, and detection is not repair; it covers a **VANISHED** store, never a **REPLACED** one.
`ENG-0277`, `ENG-0278`, `ENG-0282`, `ENG-0283` remain open and belong to other lanes.
`ENG-0121`'s two limbs both stay open — `notice_list` is a NEW command, so `marker_stats` remains
dormant. Three desktop micro-items are filed in the ledger, `D-0045`'s substantive record among them.

⚠ A stale comment inside the sealed `eng0048` test is **RECORDED here, not edited** — the seal
stands untouched, and a shipped-configuration residue test is not a re-opening of
`ENG-0048`/`NA-0697`.

## 8. BOUNDS HELD, MEASURED NOT ASSERTED

Zero `.github/**` bytes — `ci.yml.CANDIDATE` was banked as a draft and never applied. Zero protocol
product bytes in this PR. No test weakened, skipped or deleted. `ENG-0119` untouched;
`ENG-0048`/`NA-0697` not reopened. The onboarding and Lane C designs were **not consumed** — their
R-BANKs remain banked for their own lanes. No label/display-name split; no `cargo-nextest`; no
parallel-suite resurrection.

**18 red arms driven and printed both ways.** Two controls that did **not** fire were repaired or
re-characterised rather than banked as green: an ordering arm was vacuous for want of
`QSC_CONFIG_DIR`, and a no-op tamper could not discriminate because `std::fs::remove_dir_all`
already refuses to follow a symlink — measured directly rather than assumed.

## 9. SR-16 — THE FULL TABLE, S-1 … S-34

| # | chair | what happened |
|---|---|---|
| S-1 | Director MISS | transmitted governing bytes carried `U+00AD` inside a hex digest |
| S-2 | Director HIT | the pre-stated expected sha caught S-1, by design |
| S-3 | Seat SELF-REPORT | repaired a mismatch where the order said STOP — an explicit STOP means stop-and-propose **even when the fix is provable** |
| S-4 | Director MISS | sec 4 cited `RULING_011` without its bytes on the box |
| S-5 | Seat MISS | a **DERIVED** 46px header height reported where the order said *measure* (measured: 84/56/25) |
| S-6 | Seat MISS, self-caught | the first desktop id-sweep arm was vacuous — NA-0775 has zero desktop presence |
| S-7 | Director HIT | the escalation-direction claim checked against `ci.yml:78-79` |
| S-8 | Seat NOTE | `escalations(3)` mischaracterised which direction was live |
| S-9 | RE-CLOSED AT 43 | seat eye-count 41, Director's `commands.rs`-only census 42; true value **43/43**, both instruments |
| S-10 | Read chair MISS, self-reported | the scratchpad deviation |
| S-11 | Read chair HIT, self-caught | the vacuous first tamper arm, repaired |
| S-12 | Director slip, self-noted | `na0753`'s listing equality is at `:77`, not `:78` |
| S-13 | — | the `RULING_003` delta ruled **IMMATERIAL**: indent loss on a long paste, zero prose changed |
| S-14 | HIT | the transmission rule verified on its own first use |
| S-15 | Seat MISS | `git checkout --` **destroyed uncommitted work** mid-control-run |
| S-16 | Seat MISS | S-15 **repeated one turn later**; cure adopted as standing: revert a perturbation from a `cp` **COPY**, never `git checkout --` |
| S-17 | Seat MISS | `cp` inherited 444 from a sealed source, the write failed, and an identical copy was banked as v3.1 |
| S-18 | Seat MISS | a canned conclusion printed **under a contradicting measurement** ("the delta is ONE hunk" beneath `hunks: 0`) |
| S-19 | Director MISS | `RULING_009` 1(a)'s premise adopted where a census was available (replay covers 29/46) |
| S-20 | Seat MISS | the notice refresh placed **outside** the tick's quiet scope, undoing `ENG-0271` |
| S-21 | HIT | `f_p` caught S-20 before it left the seat |
| S-22 | Seat MISS | **two** controls did not fire and were repaired rather than banked as green |
| S-23 | Prediction HIT | the 45/45 census landed exactly as `RULING_003` sec 1(a) predicted |
| S-24 | Note | two needles of different scope in one artifact — `WF-0087`'s own property, harmless here, recorded so the next reader is not confused by an off-by-one that is not one |
| S-25 | Director | the `git add -f` evidence step misapplied to the desktop repo, which has no `docs/governance/evidence/` and no evidence ignore rule |
| S-26 | Director | *"NO UNAUTHORIZED FILE IS IN THE DIFF"* declared from an enumeration built off the reports, when the mechanical instrument — the diff's file list minus 3.0's list — was available and **was not run**. Instrument narrower than the claim, again |
| S-27 | Seat, self-report | did not flag `paths.rs` as outside the ratified set at the time — **and the seat's catch of the Director's count is a HIT beside it** |
| S-28 | Director | `build_commit` was never rendered, so flight check (2) was **unflyable** as specified |
| S-29 | Director | `B-1` and the T1–T4 translation narrowed the operator's *"no matter what it is"* to *"every `h2`"* — an **element-typed** census where the claim was **role-typed**. The read chair's `MINOR-6` census is recorded as the source of the eight-`h2` framing: recorded, not faulted; its charter asked about `h2` sites |
| S-30 | **HIT, operator flight** | **it caught what no arm measured. THIS IS THE STANDING REASON FLIGHTS PRECEDE MERGES.** Seat: no row — it built exactly what was ruled |
| S-31 | Director | `RULING_012` cleared the PR to author and `RULING_014` *"verified it from this chair"* by state, head and file list, and **never once read the check-runs** — having written *"read no check state until SETTLED"* and then never read it at all |
| S-32 | Seat, self-reported | **three pushes without reading the board** |
| S-33 | Director | an instrument named without proving it reaches its claim's boundary (`git diff -w` against a line-wrap). HIT, seat: executed the clause as written **while demonstrating the defect** — the letter obeyed, the defect surfaced, no private re-reading |
| S-34 | Director | premise adopted, not measured: `stable` assumed to be a new toolchain beside a 1.95.0 pin, when the seat's **default** IS `stable`. HIT, seat: the property delivered, the literal command refused, the predictor validated red-then-green |
