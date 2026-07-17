# NA-0651 — qsc-desktop sidecar retirement: testplan / checks matrix (D587, D-1274)

Lane class: DELETION+DOCS LITE-CEREMONY (single PR + single decision; DOC-OPS-006 §9,
certified in D587). Base: `bfa84b12` (the #1583 seating merge; qwork-proven).
Directive: QSL-DIR-2026-07-16-587 (approved 2026-07-16, both flags at the draft
defaults). All commands run locally in the operator-provisioned NA-0651 workspace with
the qwork shared cargo target.

| # | Check | Method | Expected | Result |
|---|-------|--------|----------|--------|
| 1 | qwork startup proof | read `.qwork/startup.qsl-protocol.kv`; all §3 invariants | startup_result=OK; lane=NA-0651; head==origin/main==main; clean; ready_count=1; queue_top=NA-0651; shared_target_ready=yes | PASS (head bfa84b12) |
| 2 | CONFIRM-LIVE counters | canonical greps on DECISIONS.md / NEXT_ACTIONS.md | D-1274 count 0 pre-landing; D-1273 count 1; STATE READY=NA-0651; anchored READY ×1 | PASS |
| 3 | Disk/mount gates | df /; mountpoint /backup/qsl | root <95%; mounted | PASS (49%; mounted) |
| 4 | Main-health at base | gh run list at bfa84b12 | push runs green or known-normal in-progress | PASS (7/9 success; formal-ci + qsc-adversarial in progress = the two known long runs) |
| 5 | Precondition (a): tree self-isolated | grep `[workspace]` in qsc-desktop/src-tauri/Cargo.toml | empty `[workspace]` declaration present | PASS (line 7) |
| 6 | Precondition (b): not a member | read root Cargo.toml members | tree absent from members list | PASS |
| 7 | Precondition (c): no workflow refs | grep -r qsc-desktop .github/ | 0 references | PASS (0) |
| 8 | Tree census | git ls-files qsl/qsl-client/qsc-desktop/ | exactly 22 tracked files | PASS (22; list in as-built §3) |
| 9 | F1 harness zero-invoker | grep -rl desktop_sidecar_stress over .github/, scripts/, Makefiles | zero invokers (self excluded) | PASS (0) |
| 10 | Baseline at base (pre-deletion) | `cargo check --all-targets`; full `cargo test -p qsc` | check 0/0; suite 412/0/1 across 108 result sets, exit 0 (= the NA-0649/D-1272 record) | PASS (0/0 in 15.62 s; 412/0/1 ×108, exit 0; e2e ok) |
| 11 | KEEP enumeration | grep -rl QSC_DESKTOP_SESSION_PASSPHRASE qsl/qsl-client/qsc/ | anchor files enumerated pre-mutation | PASS (16 files: 1 src + 15 test-side incl. tests/common/mod.rs) |
| 12 | The deletion | git rm -r the tree; git rm the F1 harness | exactly 23 staged deletions (22 + 1) | PASS (23) |
| 13 | Banners identical ×3 | sha256 of the banner block extracted from each doc | one hash, three docs; docs otherwise untouched except Last-Updated | PASS (0344b46d… ×3) |
| 14 | F2 repair minimal | python replace with count-asserts (each old string ×1) | exactly two references retargeted; zero live links to the tree remain | PASS (2 replaced; 0 remaining) |
| 15 | F2 claim-boundary byte-proof | sha256 of both regions pre/post (pinned rule: section header → end-of-section incl. trailing newline) | identical both sides | PASS (What Is Not Proven 1,176 B f8ede7a1… ==; Safe Public Wording 995 B 56fbab6f… ==) |
| 16 | Acceptance A: workspace unaffected | `cargo check --all-targets` at head | 0 errors / 0 warnings | PASS (0/0 in 0.29 s — nothing recompiled) |
| 17 | Acceptance B: suite counts unchanged | full `cargo test -p qsc` at head vs the base-derived baseline | 412/0/1 across 108 result sets, exit 0 — byte-identical totals | PASS (412/0/1 ×108, exit 0 — byte-identical totals; e2e ok; desktop_gui_contract ok) |
| 18 | Acceptance C: residual references | `git grep -n "qsc-desktop"` (literal, case-sensitive) at PR head | every hit ∈ the D587 10-class manifest; zero outside | PASS (471 files / 1,452 hits, ALL mapped classes 1–8; zero unmapped; the F1 script absent; the repaired package carries zero hyphenated references) |
| 19 | Acceptance D: untouched surfaces | `git diff --stat base..head`; cargo metadata --locked | zero changes under qsl/qsl-client/qsc/, .github/, formal/, vectors/; root Cargo.toml/Cargo.lock absent from the diff; metadata OK; KEEP-anchor count still 16 | PASS (0 changes under all four protected trees; root Cargo.toml/lock absent from diff; metadata --locked OK; KEEP anchors 16 unchanged) |
| 20 | Queue/decision proof at close | canonical greps post-edit | D-1274 ×1; STATE READY=NONE / HIGHEST_NA=0651 / HIGHEST_D=1274; anchored READY ×0 (awaiting next directive); block DONE+OUTCOME | PASS |
| 21 | Validation defaults | git diff --check; scope guard; link check on touched docs; private-material/prohibited/overclaim/claim-boundary scans; goal-lint local; root+fuzz cargo audit; cargo metadata --locked; cargo fmt --check; sh -n/bash -n qsc_adversarial.sh | all green; claim boundary UNCHANGED | PASS (diff --check clean; scope guard clean — changed paths ⊆ D587 allowed list; link check 0 broken across 6 touched docs; publication scan overclaim 0 / secrets 0, long_hex = the 4 published proof hashes; goal-lint local OK; audits 386/0 + 287/0; metadata --locked OK; fmt --check = 145 PRE-EXISTING diffs, identical at base and head, zero in lane-touched files — lane touches no Rust; sh -n/bash -n OK) |
| 22 | Public-safety + advisories | PR checks | green, no gate amendment | Verified at the PR boundary — recorded in the lane response file (the PR's own check results cannot be recorded inside the PR's own commit) |

Non-goals proven by the scope guard: NO qsc source change, NO test change, NO dep or
root-lockfile change, NO GUI code, NO IMPROVEMENT_LEDGER edit, NO public-safety-gate
edit (its `qsc_desktop_path` classifier fixture row is §9-protected and carved out in
the residual manifest).

See `docs/governance/evidence/NA-0651_as_built.md` for the full deletion census, the
zero-invokers proof, the per-class residual-reference mapping, and the byte-proof
transcripts. See D-1274 for the decision record.
