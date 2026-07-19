# NA-0656 testplan — qsl-desktop satellite registration + DOC-PROG-004 v0.1.0 → v0.2.0 (D592, D-1279)

Docs-only LITE lane: the "tests" are the fail-closed verification matrix.
Every row was executed; results recorded here and in
`docs/governance/evidence/NA-0656_as_built.md` (proof root
`/srv/qbuild/tmp/NA0656_qsl_desktop_satellite_registration_20260719T002928Z/`).

| # | Check | Method | Expected | Result |
|---|---|---|---|---|
| 1 | qwork invariants | `.qwork/startup.qsl-protocol.kv` | all OK; lane NA-0656; head==origin/main==`557bb8b2`; ready_count=1 | PASS |
| 2 | D-1279 next-and-absent (pre-lane) | `decision_id_counter.py DECISIONS.md D-1279` | canonical_count=0, exit 0 | PASS |
| 3 | Sole READY = NA-0656 | anchored `^Status: READY` grep | ×1 | PASS |
| 4 | Main health on `557bb8b2` | `gh run list --commit` | green or normal windows | PASS (7 success; formal-ci + qsc-adversarial in normal windows) |
| 5 | **STOP: qsl-desktop EMPTY** | `git ls-remote` + API size | zero refs; size 0 | PASS — not fired |
| 6 | **STOP: PVR** | `gh api …/private-vulnerability-reporting` | enabled=true | PASS — not fired |
| 7 | Baked merges ancestry | `git merge-base --is-ancestor` ×4 | `b3cfd5df`/`e46cb6b3`/`0a8e0843`/`345edcd9` all ancestors | PASS |
| 8 | Baked ledger/code facts | greps | ENG-0044 open; `vault_init_with_passphrase`/`identity_ensure` present; `vault_exists` an error code; TimelineEntry no content field | PASS |
| 9 | Tracked rename | `git diff --cached -M --summary` | one `rename … (NN%)` line | PASS (51%) |
| 10 | Byte-exact landing vs Appendix A | awk extract → sed LB → `cmp` | identical; exit 0 | PASS (landed sha256 `7920fbf3…`, 266 lines) |
| 11 | LB resolution | grep `<LANDING-BASE>` | 1 in extract; 0 in landed; value `557bb8b2` | PASS |
| 12 | LN | promotion numbers vs drafted | NA-0656/D-1279 unchanged → no renumber | PASS (not exercised) |
| 13 | Fidelity hunk census | `git diff --no-index -U0` baseline vs landed | every hunk ∈ R0–R7 (+LB); zero outside | PASS (16 hunks; map in the as-built §3; hunk 10 = the anticipated R6+R7 composite, sub-mapped) |
| 14 | v0.1.0 preservation | the same diff (exhaustive) | no change outside the 16 hunks | PASS |
| 15 | Manifest re-check | greps on the landed doc | L1–L9 ×9; steps 1–9; D-A DECIDED + D-B open; Horizon ×1; 6 track rows; 7 corrections; zero NA refs > 0656 | PASS |
| 16 | B1 exactness | `git diff` DOC-CTRL-001 | the row pair + Last Updated ONLY; Version stays v1.0.1 | PASS (3 changed lines) |
| 17 | B2 exactness | `git diff` DOC-PROG-003 | the §6 bullet + Last-Updated ONLY | PASS (2 changed lines) |
| 18 | D-1279 canonical (post-append) | counter tool + grep | ×1, Accepted, dated, Goals G4/G5 | PASS |
| 19 | Scope guard | `git status` changed-path census | EXACTLY the 9-path allow-list (rename = 2 sides) | PASS (as-built §7) |
| 20 | Queue flip | STATE + anchors post-flip | `READY=NONE \| HIGHEST_NA=0656 \| HIGHEST_D=1279`; anchored READY ×0; DONE comment + prior comment | PASS |
| 21 | git diff --check | whitespace | clean | PASS |
| 22 | Validation defaults | audits, `cargo metadata --locked`, fmt --check, scans, sh -n | green / known pre-existing residue only; zero lane Rust | PASS (response §10) |
| 23 | goal-lint local | synthesized GITHUB_EVENT_PATH + the live PR body | OK, literal `Goals: G4, G5` line from creation | PASS |
| 24 | Registration completeness | D578/D-1265 term-for-term cross-check | all satellite-model terms present + the pin clause + the four-item owed list | PASS (as-built §6) |

STOP conditions never fired. NOT claimed: any GUI; the bootstrap landed
(all four owed items remain owed); any qsl-desktop mutation; any
implementation authorized. Claim boundary UNCHANGED.
