# QSL PREDICTION LEDGER (SR-16) — seeded 2026-08-05
**Rule:** the Director appends predicted-vs-measured rows at every lane close. Predictions
are quoted from the directive/block as written BEFORE execution; measurements from the
stop-file of record. Purpose: after enough rows, ceremony tiering (SR-17) gets decided
with data instead of feel. Scoring: HIT (as written), MISS (direction wrong), PARTIAL.

| lane | prediction (as written) | measured | score | lesson row |
|---|---|---|---|---|
| NA-0695 | "expected calm" (D-shape, one stop at PR) | six stops, one gate surprise disposed pre-push | PARTIAL | calm ≠ stop-count; the surprise was the directive's own contradiction |
| NA-0695 | CodeQL WF-0047 class "EXPECT IT TO FIRE" on new salt consumers | ZERO alerts fired; dismissals carried | MISS (safe direction) | taint model narrower than assumed; salt→account-name not a crypto sink |
| NA-0695 | suite 596/0/2 across 129, by-name +1 binary | exact | HIT | — |
| NA-0695 | goal-lint green (§7.5) with TRACEABILITY absent from §6 | RED first run; directive internally inconsistent | MISS | became SR-07 |
| NA-0695 promo | fmt/clippy/structural base rows | all exact | HIT | — |
| NA-0696 probe | banked D1 mechanism assumptions (drop-order, panic-safety, path-key) | all held; zero contradictions | HIT | probe-before-draft pays |
| NA-0696 probe | "7 call sites" (banked) | 8 textual = 7 calls + 1 fn-value | PARTIAL | became SR-09/SR-10 |
| D630 draft | preflight GREEN at promotion (count 0 at base) | count 1 at base (:71 prose); promotion → 2 → RED mask | MISS | inference from exit code, not from the count itself; A1.1 corrected |
| D630 draft | "NO test asserts on stderr" | vault.rs:165 not-contains predicate exists (harmless to the emit) | MISS | census grep shape too narrow; A1.2 corrected |
| D630 draft | banked-block reconstruction faithful | 3 substantive divergences (D5a shape, D4 boundary, D4 comment) | MISS | became SR-14 (R-BANK) |
| NA-0696 promo | preflight RED (mask, unanchored 2 / anchored 1) | exact | HIT | corrected prediction held |
| NA-0696 promo | §7.1 base rows incl. two-needle split | all exact (12 + 1) | HIT | SR-10 applied |

<!-- Append below at each lane close. NA-0696 execution rows DISCHARGED — entered
directly below at that lane's close. NA-0697 + NA-0698 close rows appended at NA-0699
(D-1339), byte-verbatim from the Director's ruling banked as RBANK_NA0699_003. -->
| NA-0696 exec | §2d.5: "NO test pins the old collapse" | FALSIFIED — na0694:155 pinned it; 1-red suite | MISS | became SR-18; third census-class instance |
| NA-0696 exec | "expects its R16 stops" (the lane framing) | exactly one R16 (STOP 006), fully diagnosed | HIT | the framing prediction held |
| NA-0696 exec | suite 606/0/2 across 130, by name | exact, on the amended commit (mandatory re-run) | HIT | identity-carry correctly refused across a test edit |
| NA-0696 exec | six E-C control red sets, committed exact | all EXACT incl. two pre-named sketch deviations | HIT | E-C discipline at full scale |
| NA-0696 exec | sweep: zero unclassified; 7 retired sites in held locks | exact, bidirectional | HIT | — |
| NA-0696 exec | CodeQL "may re-anchor" (hedged) | 0 open alerts on the ref | HIT (hedged) | second consecutive non-fire; WF-0047 set still 4 |
| NA-0696 exec | preflight A1.1 corrected trajectory (RED window → green post-flip) | exact | HIT | the corrected prediction, not the drafted one |
| NA-0696 exec | ProVerif pass (no formal/ touch) | pass | HIT | — |
<!-- NA-0697 close rows entered 2026-08-06 by the incoming Director (SR-16; STOP 006
ruled ACCEPTED, RBANK 006 sha 611a6085…0a36; desktop #24 merged bcb1dc1a, spine #1706
merged 5fafce42 — merge-commit shape verified, second parents = the ruled heads). -->
| NA-0697 draft | Control 1 as ruled: assert-red at base, inline edit shape (D631 §7/§8) | FALSIFIED pre-edit — Probe F: E0425 compile error at base, not assert-red; the §7/§8 triangle proven JOINTLY unsatisfiable (static lemma + Probes L2/F) | MISS | became SR-19 (delta-symbol reachability, verified at drafting); A2 replaced the pair with Controls 1′/2′ |
| NA-0697 draft | base residue listing ["qsc","settings.json"] | exact — Probe L2 and both controls each measured the predicted listing verbatim | HIT | the prediction survived the instrument redesign intact |
| NA-0697 exec | suite 104+N by name (N=1 → the 105 pinned names) | exact — 104/0/1 across 12, exit 0; by-name delta exactly destroy_residue_set_enumerated_by_name | HIT | figures re-attached to the amended commit per the NA-0695 identity rule |
| NA-0697 exec | Controls 1′/2′: EXACTLY the one-test red, same listing, all others green | exact both — 103/1/1, same test, same listing; restores cmp-identical (2′ also byte-identical to committed HEAD) | HIT | mechanism-identical replacement pair held at full suite scale |
| NA-0697 promo | SR-11 mask: unanchored 2 / anchored 1; a count of 3 = STOP; post-flip green | exact at promotion AND re-observed at the executor seat; post-flip exit 0, clean_tree=yes | HIT | pre-named, never chased — third consecutive validation of the mask model |
| NA-0697 | operator-paste fidelity (transport) | RBANK 002 arrived TRUNCATED; caught by the block's one-anchored-READY invariant BEFORE consumption; RBANK 003 superseded it. At STOP 006 the chat paste was mechanically diffed against the sealed artifact: content-identical | MISS (safe direction) | invariants as transport tripwires validated; Director now diffs paste-vs-artifact as routine |
<!-- NA-0698 close rows entered 2026-08-07 by the Director (SR-16; lane COMPLETE per
STOP 008 / R59; class QSL_CI_PHASE1_HERMETIC_SHARD_PASS; impl PR #1708 merged 3fcda47d). -->
| NA-0698 draft | Control-1-analog: shard args as drafted would run (I3/E8: emit non-empty) | FALSIFIED — cargo REJECTS shard 6's args outright (`can't mix --doc`, exit 101, pre-build); found by SR-15, corroborated upstream | MISS | became SR-20 (consumer-validated emission); the F1 blocker; 11/12 shards WERE accepted first attempt |
| NA-0698 draft | `--no-run` = universal acceptance probe (the first SR-20 control's own design) | FALSIFIED — `can't skip running doc tests with --no-run`, exit 101 on the doc shard | MISS | SR-20 caught its own control; probe now matches the workflow's real invocation per shard |
| NA-0698 draft | the `--assert-workflow` gate reads the matrix it claims to read | FALSIFIED run 1 — bare `shard:` needle matched the JOB named shard (:96); 6 controls died; fixed to inside-`matrix:` scope; full series re-run | MISS (caught by its own controls) | needle-wider-than-claim INSIDE the gate built to catch that class; red run preserved |
| NA-0698 exec | drafting fixture represents the production log (E24 vs the real workflow) | FALSIFIED — Control G attempt 1 red 12/12: `CARGO_TERM_COLOR=always` from dtolnay; fixture came from the ONE uncoloured ci.yml job | MISS | fixture-from-the-wrong-producer → SR-20 extension (environment is part of artifact identity); FIRST defect this lane's own CI caught that drafting controls could not (R52) |
| NA-0698 exec | re-running the sealed series is side-effect-free | FALSIFIED — output root not re-pointed; run 2 `.out` files overwritten (bounded ×7 arithmetic; red run + verdict log intact) | MISS | a re-run narrower than its side effects → output-root-first practice (R43c), applied to the very next run |
| NA-0698 close | repo-wide cache total = evidence of our workflow's write behaviour (R27-specified, R53-elevated) | FALSIFIED — total moved on an UNRELATED workflow's write; valid instrument (keyed entries) shows 0 across 4 PR runs, then exactly 1 from main | MISS — caught by LUCK, not an honest instrument; DUAL AUTHORSHIP, Director first (R27 spec → seat offered → R53 elevated) | → SR-21 (instrument scope = claim scope) + the pre-offer check; property itself STRONGER on the valid instrument |
| NA-0698 drafting-evidence | expectations-before-checks kept for all controls | E9/E10 expectations existed only inside the run log carrying their results; SR-15 did not catch it | MISS ×2 (half-kept rule + adversarial-read coverage gap) | cured in v2 forward; SR-15 coverage data recorded per R28 |
| NA-0698 exec | worst shard ~36.5 CI-min (derived 1.373× ratio + build); 60-min STOP threshold | measured 30.9 / 31.1 / 30.3 / 30.4 min across four runs — conservative ~15%, threshold never approached; shard-level variance real (shard 4: 14.7↔30.4 for identical targets) | HIT | derivation method TRANSFERRED from the monolith; tune against measured worst case, never the threshold (R54) |
| NA-0698 exec | Control P red set EXACTLY {qsc-shard-4, qsc-sharded-suite}; 13 shard-mates reconcile clean under --no-fail-fast | EXACT — 2 contexts red, nothing else; `expected 14 / observed 14 / missing 0 / extra 0` WITH the mid-shard failure | HIT | first gate in this program proven red-capable end-to-end on real infrastructure before being trusted (R48) |
| NA-0698 close | cache: PRs restore, never write; one writer, main-only | PROVEN both sides on the valid instrument: 0 keyed entries across 4 PR runs (48 shard-jobs); exactly 1 from refs/heads/main (shard 0 post-step, 301 MB) | HIT | ~3.3 GB saved vs per-shard keys; writer-move pre-authorization correctly NOT exercised |
| NA-0698 promo | SR-11 mask: unanchored 2 / anchored 1; 3 = STOP; post-flip 1 | EXACT at promotion AND at the flip | HIT | fourth consecutive validation |
| NA-0698 outage | (operational) `rerun` throttled ⇒ Actions down | FALSE — `workflow_dispatch` unthrottled the whole time; recovered on the OPERATOR's try-anyway; status page lagged capacity in both directions; 4 runs wedged self-contradictory | MISS (Director/seat shared) | measure more than one door before concluding the building is locked (→ recorded practice, SR-23 candidate) |
| NA-0698 Director | D1 finding-of-fact from a chat rendering (R15); "three names" adopted without remeasurement (R16); R27 instrument spec (R56b) | three Director misses, owned in the Director's name | MISS ×3 | → SR-01 extension (findings require the sealed file); census-as-wide-as-claim; SR-21 |
| NA-0698 ceremony-payoff (SR-17 substrate) | Director's successive sums: "7 pre-commit / execution caught 1" (R61e), then "7 honest / 1 fortune; 5 pre / 3 after" (first packet correction) | BOTH SUPERSEDED by the SR-15 D633 read (findings sha 570e546e…d3d3, B2 — canonical): "the five instrument defects" names TWO unreconciled sets sharing 2 members; on Set A at most 2 pre-commit defects in the deliverable; EXECUTION caught ≥5 (X1 ANSI via the lane's own CI, X2–X4 the watcher trio, X5 the cache confound by fortune) plus 2 more via execution-side pre-write instruments; "one required-check hazard" unnamed anywhere and NO required check was ever degraded; the catching instrument was the 37-control series (E32 postdates the commits) | MISS ×2, DIRECTOR'S, and the direction matters: both wrong sums FAVORED the Director's own tier-down preference — the exact conflict the operator's mandatory red-team existed to test, and it fired | ratio ≈ parity, not 7:1; the per-stop ritual produced the post-commit catches; the "keep drafting rigor, tier the ritual" inference RAN BACKWARDS from the evidence (B2's words) — SR-17-as-drafted falsified by its own mandated read |
| NA-0717 draft | A7: each failing assert prints (-1,35) vs (-1,11); :393 shows probe_rc=-1 probe_errno=35 | byte-exact in the fresh pull AND identical in all four historical failure logs | HIT | seat-blind seal (68 s mtime margin); predicted bytes pre-observed in-program via NA-0715's banked copy of the same job log; corroborative, not discovery-grade. C-2 rests on the five-run logs + source + one-sha two-platform pair |
| NA-0717 draft | id sweep: max NA anywhere across main+3 PRs = 0716 (pr1727); nothing ≥ 0717 | exact; only NA-9999 fixture placeholders above, classified | HIT | WF-0068 discipline, 4 refs × 6 spaces |
| NA-0717 draft | pr1725 max NA heading = 0714 | measured 0712 — NA-0714 closed with no block ever born | MISS (benign) | blocks are born at promotion; prose/STATE carry the max |
| NA-0717 draft | Linux lib baseline at 5b43eefe: 122 passed / 0 failed, four tests ok by name | exact | HIT | both platforms measured at one sha |
| NA-0717 draft | 653 Phase 3 sealed "goal-lint PASS" for the single-PR shape; the R269/R270 packet sealed it again for PR-B | goal_lint.py:90-93 fails BOTH shapes — core path, no tests/-dir path; unsatisfiable as sealed | MISS (three chairs: 653's author, SR-15 T4, the Director's ruling packet) | caught by the amending seat re-verifying every consumer against the new shape; a sealed expectation is not sealed until its consumer has been executed or read against the exact shape it seals |
| NA-0717 draft | 653 Phase 1 sealed "cargo fmt --check clean on the file" jointly with §4(b)'s exact one-line site forms | rustfmt rewraps 4 of 6 sites (symbol +13 chars; only one rewrapped site crosses max_width=100 — the operative trigger is an inner call-width heuristic, unmeasured; governing evidence = measured rc chain + the four-hunk diff); the seal pair is unsatisfiable as written — base file clean, instrument valid | MISS (653's author; SR-15 and the ruling packet did not execute the post-edit fmt consumer either) | row 5's lesson, second instance in one lane: a sealed expectation is not sealed until its consumer has been executed against the exact shape it seals |
<!-- LEDGER DEBT AUDIT 2026-08-13 (NA-0718, D-1354): close rows exist through NA-0698 and
drafting rows for NA-0717 ONLY. For NA-0699..NA-0716 (18 lanes) NO close rows exist here
AND no on-box source contains them (instrument and per-file classification in
docs/ops/DIRECTOR_OWED_REAUDIT_2026-08-13.md): they are STILL-OWED at each lane's
retrospective close — recorded as debt, deliberately NOT reconstructed from memory.
NA-0717's execution rows enter at its close per the standing deferral. -->
| NA-0699 draft | SR-17 tiering rule as drafted (§6.3) is adoptable | FALSIFIED by the operator-mandated SR-15 read (findings 570e546e…d3d3): 2 BLOCKER / 11 MAJOR — three tiers with the content of none defined; "blast radius" announced, domain-keyed enumeration delivered; all six hard cases ambiguous or unclassifiable; the old text's any-surprise-auto-upgrades clause silently dropped | MISS, DIRECTOR'S — the design was written on the Director's recommendation | REMANDED; 26 findings become binding design constraints; the predecessor entry survives BYTE-IDENTICAL (verified independently: base entry present verbatim inside the amended entry) |
| NA-0699 draft | ceremony-payoff substrate: "drafting caught 7 pre-commit, execution caught 1" | FALSIFIED (B2): "the five instrument defects" named TWO unreconciled sets sharing 2 members; execution caught ≥5; "one required-check hazard" unnamed anywhere and no required check ever degraded | MISS ×2, DIRECTOR'S — **both wrong sums favored the Director's own tier-down preference** | the conflict of interest the operator named when ordering the read; the read fired on exactly it. Ratio ≈ parity, not 7:1 |
| NA-0699 draft | "SR-17 restraint" is a real citation (used as operative ground in D-1338 twice) | FALSIFIED (M7): SR-17's text contains zero occurrences of restraint/mint/one-per; the practice was real, the citation was not; the convention was breached at its own birth (WF-0048 activated SR-14/15/16 in one merge) | MISS (program-wide, inherited) | → the unnumbered RESTRAINT convention at §B head; D-1339 records every prior citation's referent |
| NA-0699 exec | R97's premise: the unsealed draft contains the remanded SR-17 tiering text | FALSIFIED by the executor seat BEFORE writing: ten tiering needles = 0 in the draft and 1 each in the canonical directive (red-capability proven); draft mtime 14:46:38Z predates STOP 001's 15:12:54Z by 26 min — the text could not have been there | MISS, DIRECTOR'S — contents inferred from the file's ROLE rather than read from its bytes; the SR-01 extension violated by the Director who ruled it three stops earlier | the false clause would have been written PERMANENTLY into the artifact whose purpose is preventing a future seat from believing a false thing. Retirement stands on the real defect: the draft carries the SR-17-restraint MIS-CITATION verbatim → **an authorized act whose premise measures false is an R16 STOP, not a judgment call** |
| NA-0699 exec | seat needle discipline (STOP 005 §7.2/§7.3, STOP 002 §4.1) | four expectations wrong: a line-wrapped clause missed by a single-line literal; SR-20 occurrence needle counting cross-references; `minimum gate set` counting the annotation's own quote; a port needle matching `DECISIONS.md:37370` as host.tld:port | MISS ×4, SEAT'S — all free (expectation written first), all self-reported | SR-21 caught its own author within hours of landing; cure adopted: **measure the target's bytes (cat -A) first, then build the needle**; and when a needle keeps mismatching, ENUMERATE AND CLASSIFY rather than refine |
| NA-0699 exec | Director's own needle while verifying R66/R70 | reproduced the seat's wrap defect exactly — single-line literal returned 0 against line-wrapped text; corrected instrument returned 1 | MISS, DIRECTOR'S | the wrap-safe needle is now house instrument for any sentence-presence claim; the family has now claimed every chair at the table |
| NA-0699 exec | G6 preflight at base: exit 0, ready_count=1 | measured exit 1, READY_COUNT=2 — the figure was copied from STOP 002's PRE-PROMOTION base where the NA-0699 block did not yet exist | MISS, SEAT'S — a figure carried across a base change | → **an expectation is measured at the base it will be checked at; a carried figure is re-derived, not copied.** The measured result was SR-11's pre-named mask behaving exactly as pre-named |
| NA-0699 exec | R86 packet-mirror inheritance provable | PROVEN both directions on an independent target: pre-merge cmp identical (m8's trap), post-merge cmp differs at byte 935/2256, post-copy sha matches the Director's independently-measured targets EXACTLY (6a22b172… / a4fd2b83…), cmp exit 0, mode re-asserted 444 | HIT | the m8 gap (two ruled appends untaken for two days) is closed and its own reassurance-instrument became the evidence |
| NA-0699 exec | method/ directory fully sealed | `find method/ -type f ! -perm 444` = 0 — true for the first time; the retired draft carries its defects named on its face, content otherwise unaltered (diff of body: empty) | HIT | a mutable draft beside canonical mirrors was a future seat's wrong source; the SR-17-restraint mis-citation would have been re-inherited verbatim |
| NA-0699 ceremony-payoff (SR-17 substrate, second datapoint) | the mandated adversarial read on a Director-recommended rule is worth its cost | the read killed the rule and corrected the Director's arithmetic in the Director's own name; every one of the lane's other catches came from expectations-written-first at seat level | — (data row) | **the two mechanisms that produced every catch this lane: the adversarial read on the Director, and expectations-before-checks on the seat.** Any successor tiering design must not tier either away |
| NA-0700 draft | R119(a): `receipt_policy_mvp_na0177` is the byte-identity gate; "any red is a STOP" | FALSIFIED BY EXECUTION — the SR-15 seat ran two deliberate regressions (probe A: TUI emit fd-swapped to stderr; probe B: gross `QSC_MARK/1` prefix) and the gate stayed GREEN BOTH TIMES: it asserts substrings over stdout+stderr MERGED, so it can see neither which stream bytes land on nor what precedes them | MISS, DIRECTOR'S, BLOCKER-class | the Director named a gate without requiring it be proven red-capable — violating the gate-liveness principle the Director himself ruled at NA-0698 R7/R48. The lane could have shipped non-identical output with every gate green and recorded "byte-identical: measured green" — a false record |
| NA-0700 draft | R131: the emitted-line surface is "7 TUI labels" | FALSIFIED — measured 6 TUI labels across 7 SITES; the findings carried a site count into a label count and the Director repeated it verbatim | MISS, DIRECTOR'S | figure adopted from a findings document rather than measured; same class as R8's "three names" and R118's comma. The seat refused to write it and re-measured |
| NA-0700 draft | R144's criterion (`should_redact_value`) spans the Director's stated concern (contact aliases in a shareable debug artifact) | FALSIFIED — the criterion is SHAPE-based (length, digits, URL-form, timestamp-form); `should_redact_value("peer","mom")` = false on BOTH paths. It catches a subset; the semantic residue survives | MISS, DIRECTOR'S | an instrument narrower than the claim it was offered for → filed as the shape-vs-semantics entry (R151): a contact alias is semantically sensitive in this threat model and shape-innocuous; the two do not meet |
| NA-0700 draft | R123's manual build-root needle rows: "expected 0" over added lines | FALSIFIED-BY-CONSTRUCTION — R124 separately ordered a filing that must NAME the exposure class's tokens; the two orders cannot both be satisfied literally. Measured 3/2/2, every hit enumerated: exactly three lines, all the WF-0049 filing quoting its own needles; zero in source/tests/executable text | MISS, DIRECTOR'S (instrument design) | → the rows re-specified as MEASURE · ENUMERATE · CLASSIFY, zero-unclassified as the gate (R159(b)) — R90's escalation applied to the Director's own rows. Obfuscating the filing was rejected: adjusting content to satisfy a needle is how instruments stop measuring what they claim |
| NA-0700 draft | STOP 001 §5.5's desktop required-context census: {advisories, public-safety, rust} | FALSIFIED at execution — measured live at the server: **{rust, advisories, infra-literal-scan}**. The census recorded ci.yml's own COMMENT — a FILE claim — as if it were a server measurement | MISS (census, Director-approved) | safe direction (infra-literal-scan required where prose said advisory); `public-safety` no longer required is UNCONFIRMED and filed as WF-0050 for the operator's audit-log check — only that instrument can date it |
| NA-0700 exec | the census of 23 print statements | EXACT — confirmed by FOUR independent seats with independent needles (macro variants, cfg proximity, fd-level routes, the linked refimpl path at zero) | HIT | the lane's foundation was sound; what failed was the instruments around it |
| NA-0700 exec | golden control: probe A reds separate-capture and is INVISIBLE merged; probe B reds stdout and merged, stderr identical | EXACT on every stream — and the base-vs-edited comparison measured IDENTICAL across all 78 separate files and all 52 merged files | HIT | **byte-identity PROVEN by an instrument itself proven able to fail**, replacing a gate that could not. The three-stream design's reason demonstrated: merged capture is structurally blind to an fd swap, separate capture structurally blind to interleaving |
| NA-0700 exec | the two probe binaries are distinct artifacts | FALSIFIED — they were the SAME FILE (identical sha): a shared cargo target dir marked treeB "fresh" on an older mtime, so the second copy took treeA's artifact. Visible ONLY because probe B's stderr was PREDICTED identical and measured different | MISS, SEAT'S — self-caught, free | SR-20's family one level down: not the wrong producer's fixture, but ONE producer's output mistaken for two → practice adopted: verify the distinct shas of every artifact under comparison BEFORE trusting any capture |
| NA-0700 exec | control C4's red set = {cell-4, I-1} | first run reded the committed pair PLUS a pre-existing test — a LATENT PARALLEL-TOGGLE RACE between tests mutating process-global routing | MISS (pre-existing defect, exposed) | **a control that finds a bug outside the code it was aimed at is the control working.** Cured by design (mod-wide ROUTING_LOCK incl. the pre-existing test), not by retry; re-run hit exactly the committed pair. By-name count unchanged — nothing added or removed, only serialized |
| NA-0700 exec | worst shard ~30–36 CI-min against the 60-min STOP threshold | measured 1852s ≈ **30m52s** on run 31234531796, and ~19–31 min on the second round — HALF the threshold, twice, on different trees | HIT | union across shard logs reconciled BY NAME to the local suite: 610 passed identical, 2 ignored identical, 130 result lines. **Cloud and box agree name for name.** Promotion count 0 → 1 of 3 |
| NA-0700 exec | R162 uniformity: `emit_raw_payload_line(&format!(` = 21 | EXACT — contacts 14 + lib 7, no other file; Director re-measured independently on the PR head | HIT | closes the gap between the byte-identity CLAIM and its evidence: the driven surface is measured, the 4 undriven sites carry by PROVEN uniformity stated as a structural argument — not an implied measurement |
| NA-0700 close | the on-box method/ PREDICTION_LEDGER mirror differs from repo truth (the Director's close rows) | FALSIFIED — measured IDENTICAL. The Director conflated THREE artifacts sharing a name: repo truth (40 data rows) · the on-box MIRROR (copied from repo truth at NA-0699's close, never touched since) · the Director's PACKET copy (which alone carries the composed close rows) | MISS, DIRECTOR'S | contents predicted from a model of what "the Director's copy" should hold rather than from what that specific file IS. **Durability consequence, named: the composed NA-0699/NA-0700 rows live ONLY in the packet until a governance touch lands them** |
| NA-0700 ceremony-payoff (SR-17 substrate, third datapoint) | drafting-side spend: 1 adversarial read + 3 amendments + a 4-seat census | caught PRE-COMMIT: 1 BLOCKER (a gate blind to what it guarded), 3 MAJOR (the corpus insufficient as byte-identity evidence; the edit set unable to deliver its own claim; the consumer census short on both flanks), and the redaction census that found `peer` verbatim at 10 of 14 labels. EXECUTION caught 3 more by expectations-written-first (probe-B same-binary, the C4 race, the piped fmt check) | — (data row) | **every Director miss this lane (5) was caught by a seat measuring instead of adopting; every seat miss (4) was caught by an expectation written before its check.** The two mechanisms hold at a third lane |
| NA-0717 exec | STOP_005 §2 wrote "squash/merge per house habit" | the standing rule is MERGE COMMITS ONLY; the Director caught it in the merged-resume packet | MISS (seat) | a merge-form stated as habit without measuring the rule; every later ask names the form explicitly |
| NA-0717 exec | sealed "all required checks settle green" on PR-A #1728 | `infra-literal-scan` (required) RED on ONE tier1:personal_email hit — the seat's own GH007 note wrote the operator's address into the as-built | MISS (seat); the NA-0684/0685 scan caught it on first exposure | the record duty needed the CLASS, never the literal (NA-0703); cured by the R275 amend; the catching instrument credited |
| NA-0717 exec | sealed needle "`Status:\s*READY` count = 0 in NEXT_ACTIONS.md" | count is 1 on main AND 1 after the edit (a historical comment at :71); the guard fails only >1 | MISS (wrong instrument — anchored scan vs unanchored guard) | suspect the instrument before the tree; the tree diverged nowhere (R276) |
| NA-0717 exec | first literal sweep needle `tebbens\|proton\|users.noreply` over the six records files | hits were ONLY sanctioned GH007-noreply literals in deep pre-existing history; the scan's own class read 0 | MISS (SR-21 — instrument wider than the claim) | narrow to the consumer's own needle before concluding |
| NA-0717 exec | 4b runtime "~105-110 min" (653's estimate; SR-15 MINOR-3 flagged it stale) | killed at 180 with 117/~130 green; the main-push repeat 127/~130, same kill; full suite later measured 206.5m | MISS — MINOR-3's "fail direction is safe" held EXACTLY | ceilings re-fitted from the kills (NA-0719); the ratchet filed (WF-0076) |
| NA-0717 arc | R268 adopted 653 §6's "NO PR admissible by ANY path while main is red" | FALSE at mechanism level — the docs door; BLOCKER-1, verified 3×; R268 RESCINDED | MISS (Director premise-adoption) | an exhaustiveness claim must include the conditional that decides whether the gate is INVOKED |
| NA-0717 arc | the SR-15 seating premise "the test module is Unix-gated (lib.rs:6)" | no crate-level Unix cfg gate exists; the boundary is de facto (NOTE-1); the directive never claimed it | MISS (seating premise) | a seating premise cannot propagate unmeasured |
| NA-0717 arc | STOP_001's ⭐ "A7 sealed before any log byte was read" | seat-blind only — NA-0715's banked copy of the same log was on-box (MAJOR-2) | MISS (evidence-weight framing) | C-2 never needed the garnish |
| NA-0717 arc | 653 Phase 3 sealed "goal-lint PASS"; the ruling packet resealed it for PR-B | goal_lint.py fails BOTH shapes (NEW-1); unsatisfiable as sealed | MISS ×3 chairs | the sealed-consumer lesson, instance 1; cured R272; WF-0075 |
| NA-0717 arc | 653 Phase 1 sealed fmt-clean JOINTLY with §4(b)'s exact bytes | rustfmt rewraps 4 of 6 sites (NEW-2); pair unsatisfiable | MISS (653's author; SR-15 + packet did not execute the consumer) | instance 2; cured R273 |
| NA-0717 arc | RECO §2 modeled the watchdog fit as "ceiling+20 ≤ 360"; R278 sealed that arithmetic (Director, authorship named); STOP_009's gate executed the sealed arithmetic after running only the consumer's EXTRACTION | the consumer's sizing adds a 60m queue margin (coverage 390) AND the platform kills hosted jobs at 360 regardless of declared timeout — the Director's R281 measurement that also refused the seat's 420/420 cure | MISS ×3 chairs (RECO's model · R278's arithmetic · STOP_009's half-consumer gate), instance 3 in ONE arc | **the rider (R281): THE CONSUMER INCLUDES THE PLATFORM CONTRACT — runner-class limits are part of any CI consumer, and no local execution reveals them** |
<!-- NA-0731 (D-1367) — THE RECORDS SWEEP, 2026-08-15. The rows below are the Director's own
SR-16 error rows for the 2026-08-14/15 working session, landed BYTE-VERBATIM from
`QSL_CARRY_FORWARD_PRE_SWEEP_2026-08-15.md` (whole-file sha256
`7dd6de8009adb1b7a7550ca31baec1dda9e8658627e26512ad81c71365de65ff`, 151 lines), lines 119-135,
extract sha256 `b42a23b2b9617e202c62801da763d8570275976d155ed3158664e8e0b74d301c`, 17 lines.
⚠ SHAPE DIVERGENCE, reported not tidied: these are a FOUR-column table (`# | the assertion |
the measurement | the instrument that caught it`) while the running table above is FIVE-column.
Reshaping them to fit would have broken the byte-verbatim requirement, so they land as their own
table with their own header. The bytes are unaltered; only the placement is a decision.
⚠⚠ THE CLOSING PARAGRAPH IS LANDED WITH THEM AND IS THE TRANSFERABLE PART. A successor who
inherits ten rulings without it inherits confidence where the record meant to leave caution.
⚠ Ten instances in ~30 hours, NONE of which reached main — the rows are evidence for a structural
property (seats measuring, the operator challenging), not a tally against a person. -->

| # | the assertion | the measurement | the instrument that caught it |
|---|---|---|---|
| 1 | "No PR is admissible by any path while main is red" (R268) | FALSE — the docs door; the claim missed the conditional deciding whether the gate is INVOKED | SR-15 cold read |
| 2 | "Press Update branch on #1725" | It had been CONFLICTING for two days and five merges; the button does not exist on a conflicted PR | the seat, at the edit |
| 3 | "This lane will not touch the gate file" (R289) | It must — the parser reads job keys, the wait takes check names | the drafting seat, first hour |
| 4 | "Prove the macOS arrangement by workflow_dispatch" | Impossible — dispatch only fires workflows present on the DEFAULT branch | SR-15 cold read |
| 5 | "Repointing `failure_check` is behaviour-preserving" | FALSE — the gate reads that check's JOB LOG, and an aggregate's log can never carry a test name | SR-15 cold read |
| 6 | "`qsc` trusts ONLY pinned CAs" | FALSE — roots compose ADDITIVELY; the comment naming `tls_built_in_root_certs(false)` exists to record that it is NOT called. ⚠ **`NEXT_ACTIONS.md:36204` already documented this correctly and was not consulted** | **the operator's challenge** — no automated gate covers a prose mechanism claim |
| 7 | "120 failures / 51 days" | 392 / 187. A 60-run API PAGE read as a DURATION | the drafting seat, by bisection |
| 8 | "Ten lines fewer" bound as a STOP constraint (R318) | Unreachable while executing the same ruling's other orders; the figure came from a read that measured only two of five changes | the executing seat |
| 9 | "The `ci-red` label" ordered without checking it existed | It did not; `gh issue create --label` fails the ENTIRE create on a missing label — literal compliance would have made the first real red record NOTHING | the executing seat |
| 10 | "Discretion is an explicit SR-15 trigger" (R288) | SR-15 lists three mechanical triggers; discretion is not among them. The read was validly ordered — the disposition power is the Director's — but "explicit" was belief | SR-15 cold read |
**THE PATTERN, which is the point: every one is the same failure — a claim adopted from a
fragment, a summary, another document, or memory, instead of measured at the moment of assertion.
Ten instances in ~30 hours by the chair whose recorded failure mode is exactly this. NONE reached
main. What stopped them was seats measuring and the operator challenging — a structural property,
not a personal one, and the single most valuable thing in this program.**

<!-- NA-0733 (D-1369) — SR-16 ROW 11, 2026-08-15, ORDERED AT R331.1.
⚠ LANDED BESIDE THE TEN ROWS ABOVE AND DELIBERATELY NOT INSIDE THEM. Lines 118-134 of this file
are the Director's byte-verbatim block (extract sha256
b42a23b2b9617e202c62801da763d8570275976d155ed3158664e8e0b74d301c, 17 lines, landed by D-1367);
they were re-hashed UNCHANGED as a gate of this lane. Inserting a row into that table would have
edited a landed byte-verbatim block and broken the digest D-1367's provenance sentence cites
(R327.2: corrections sit BESIDE a fence, never inside it).
⚠⚠ AND THE PLACEMENT IS NOT MERELY MECHANICAL, WHICH IS THE PART WORTH READING. The closing
paragraph above names the pattern of the ten: "a claim adopted from a fragment, a summary, another
document, or memory, instead of measured at the moment of assertion." THIS ROW IS NOT THAT. The
claim was never wrong -- it was never DELIVERED. Folding it into a table whose own closing
paragraph does not cover it would have made that paragraph false about its own contents, and would
have quietly widened a pattern statement that is the transferable half of the block.
⚠ Its sibling is R329.2's CORRECT FIGURE THAT EXPIRED, landed as B12 in ## D-1369. Together they
are the two shapes this arc produced that the original ten do not contain: a TRANSPORT failure and
a FRESHNESS failure. NEITHER IS A MEASUREMENT ERROR, and measuring at the moment of assertion
prevents neither -- which is exactly why they are recorded as distinct shapes rather than as an
eleventh instance of the same one.
Composed at STOP_NA0732_004 §2, on the ground that a row about a ruling which existed only in a
conversation must not itself be left only in a conversation. -->

| # | the assertion | the measurement | the instrument that caught it |
|---|---|---|---|
| 11 | "R329 was issued, therefore R329 is ruled" | FALSE as a claim about repo truth — measured at `6f6b72e9`, `R329` appears **ZERO** times tree-wide, and `### NA-0731`'s DONE line still read `no class declared` two hours after issuance; the ruling reached no seat and no file | the Director's own re-measurement at R331.1 — **second instance for this chair**, after R287 was found by the SR-24 cold read as a dangling citation |

**⚠ THE ROW ABOVE IS INDEPENDENTLY CONFIRMED, and the confirmation carries a distinction a sweep
would otherwise get wrong.** Measured at `6f6b72e9` by the executing seat as well as by the
Director, `R329`, `R330` AND `R331` each appear **ZERO** times tree-wide — but **only R329 is a
defect.** R330 and R331 were issued *after* #1748 merged and were routed forward by R331.5, so
their absence is by design; R329 was issued *before* #1748 existed and declared a result class it
should have carried. ⇒ ***absence is not by itself evidence of a records failure; the discriminator
is whether the ruling existed before an act that could have carried it.*** All three are landed by
`## D-1369`, and `### NA-0731`'s DONE line now carries the class R329.1 ruled.

<!-- NA-0736 (D-1371) — SR-16 ROWS 12-15, 2026-08-15, ORDERED AT R335 §1 and §4.
⚠ LANDED BESIDE THE TEN BYTE-VERBATIM ROWS AND THE NA-0733 ROW, AND DELIBERATELY NOT INSIDE EITHER.
Lines 118-134 of this file are the Director's byte-verbatim block (extract sha256
b42a23b2b9617e202c62801da763d8570275976d155ed3158664e8e0b74d301c, 17 lines, landed by D-1367); they
were RE-HASHED UNCHANGED as a gate of this lane, exactly as NA-0733 did. Inserting a row into that
table would break the digest D-1367's provenance sentence cites (R327.2: corrections sit BESIDE a
fence, never inside it).
⚠⚠ ROW 12 IS THE DIRECTOR'S OWN AND IS THE REASON THIS BLOCK EXISTS. It is not a measurement error
of the kind the ten rows describe -- nothing was measured wrongly. It is an INSTRUMENT error one
level up: the option set a ruling offers is itself an instrument, and an enumerated candidate list
that omits "neither" is narrower than the claim it invites. R335 §1 names it the SECOND instance of
that shape in two consecutive briefs, which is why the property is carried here rather than left as
an anecdote. ⚠ NO RULE IS MINTED (R305/D-2: a rule without an executable consumer decays).
⚠ ROW 15 IS RECORDED AS A HIT, NOT A MISS, ON THE DIRECTOR'S EXPLICIT ORDER (R335 §4) -- "that is
the house method working, and it is worth having on the record as a hit rather than only recording
misses." A ledger that records only failures teaches that measuring is a tax rather than a tool. -->

| # | the assertion | the measurement | the instrument that caught it |
|---|---|---|---|
| 12 | "Either ENG-0134 is BROADER than its filing states, or this is a sibling" (Director's brief §5) | **NEITHER** — measured, the pull did not abort and nothing was destroyed, so ENG-0134's mechanism does not fit; and ENG-0142's region sits inside a loop zero items never entered. The true answer was excluded by the option set | the seat, refusing the frame and saying so |
| 13 | "The user's message is sitting in bob's mailbox" (Director, repeated to the operator in chat) | FALSE — the retained items measure **4279 B** and **6436 B**, matching `handshake_send` **A1** and **B1** exactly; the user payloads were **17 B** and **15 B**. The cited 15250/23043 are the **JSON response-body file sizes**, not item sizes | the seat, parsing the artifact instead of reading its `ls -l` |
| 14 | "The `--mailbox` override has ZERO green coverage anywhere in the tree" (this lane, STOP 001 §6) | FALSE — **40 test files, 99 call sites**, with `recv_commit` and `qsp_unpack ok=true` assertions through it. The claim was quantified over *the tree* from a sweep of `scripts/demo/` and `.github/workflows/` alone — **an instrument narrower than its claim (SR-21)** | the seat, asking whether the Rust suite already covered it before letting "zero" stand |
| 15 | ⛳ **A HIT.** "`send_ab_1` is absent from the committed script, so the brief's `:372` is wrong" (this lane, first pass) | The brief was **RIGHT**: the script writes `"send_ab_${i}"` inside a `while` loop, so a literal needle returns 0. Re-measured from bytes, **all four** of the brief's line numbers are correct | the seat's own re-measurement, before the claim reached any record |

**⚠ THE SHAPE THESE FOUR ADD, which the ten above do not contain.** The ten are all *"a claim adopted
from a fragment, a summary, another document, or memory, instead of measured at the moment of
assertion."* **Rows 12 and 14 are a different failure: a claim whose INSTRUMENT was narrower than its
SCOPE** — an option set that excluded the true answer, and a two-directory sweep quantified over a
whole tree. Measuring at the moment of assertion does not prevent either; **checking that the
instrument's scope equals the claim's scope does.** ⚠ And row 12 is the Director's, row 14 the
seat's — **the same defect from both chairs in one lane**, which is the argument for treating it as
structural rather than as a lapse. Row 13 is a transport failure of the NA-0733 row-11 kind: the
figure was never measured, only repeated. **Row 15 is here to keep the ledger honest in the other
direction** — the method caught a *correct* record before a lane could "correct" it.

<!-- NA-0736 (D-1371) — SR-16 ROWS 16-18, 2026-08-15, ORDERED AT R336 §A, §B and §D.
⚠ LANDED BESIDE the sealed ten (lines 118-134, b42a23b2…d301c, re-hashed UNCHANGED again as a gate
of this amendment) and beside rows 11-15. Nothing above is edited.
⚠⚠ ROWS 16 AND 17 ARE BOTH THIS SEAT'S, AND ROW 16 IS THE FIRST ERROR IN THIS LANE THAT THE SEAT DID
NOT CATCH ITSELF. Rows 12 and 14 were caught in-lane; row 16 reached a PUSHED PR and was caught by
the Director. That distinction is the point of recording it: the in-lane catch rate is not 100%, and
a ledger that only shows caught errors would imply it is.
⚠ ROW 18 IS A HIT, ORDERED AT R336 §D. It is not decoration. The three failures it names were each
caught by the SAME mechanism -- an expectation written before the run -- and a ledger that records
only failures teaches that measuring is a tax rather than a tool. -->

| # | the assertion | the measurement | the instrument that caught it |
|---|---|---|---|
| 16 | "`ROADMAP.md:13`'s *No open P1 remains* is NOT SUPPORTED" (this lane, D-1371 PART 3 as first pushed) | **TOO WIDE.** That sentence reads "…and there is no known correctness gap **in the crypto core**", inside a paragraph opening "The cryptographic core is now correctness-complete:". ENG-0134/ENG-0142 are **transport**-layer ⇒ the finding does **not** falsify it. What is unsupported is the **ledger-wide** "zero open P1s" claim, and **only** that | ⚠ **the Director, at R336 §B — NOT caught in-lane, and it had already been pushed** |
| 17 | "Replacing §0's preimages with their shapes keeps the argument fully checkable, since the hashes carry it" (this lane, D-1371 PART 8, recommending option (a)) | **FALSE.** A reader given `bob-${run_tag}` **cannot** recompute `f4c89d20` — **a hash is not checkable without its preimage.** The option would have bought the APPEARANCE of a byte-verbatim landing while destroying the property that makes one worth doing | the Director, at R336 §A, applying R335 §1's own option-set property to the seat's option set |
| 18 | ⛳ **A HIT — three instrument failures, none of which reached a record.** (i) a uniqueness assert counting the bare token `NA-0736`, which the evidence-doc **path** also contains; (ii) a `- Severity: **P1**` count taken over the **whole file** where the claim was a **delta**; (iii) `classify_ci_scope.sh` invoked with two **SHAs** where it takes **paths** | Each was wrong in a way that would have shipped a false figure or a false CI-scope claim. **All three were caught by the same mechanism: the expected result was written down before the command ran** | the seat, in-lane, before any record — the method working |

**⚠ WHAT ROWS 16–18 ADD, taken together.** Rows 12 and 14 established that *an instrument narrower or
wider than its claim* is this program's live failure shape, from both chairs. **Row 16 shows the shape
surviving an entire lane's discipline and reaching a pushed PR** — caught only because the ruling
chair re-measured a claim it had not itself made. **Row 17 shows it one level up again:** the seat
supplied the Director an option set whose recommended branch rested on a false property, and it was
caught only because R335 §1's own rule — *a supplied option set is an INSTRUMENT* — was turned on the
seat that supplied it. ⚠⚠ **The rule generalises in both directions, and this lane is the proof: it
was minted against the Director's option set and it next caught the seat's.** Row 18 is the
counterweight — **the same discipline caught three of the seat's own errors before any of them
reached a record**, which is what the ledger exists to make visible.
