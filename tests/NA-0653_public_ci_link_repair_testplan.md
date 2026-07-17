# NA-0653 Testplan — main-push public-ci link repair (D589, D-1276)

Docs-only LITE lane: no code, no test-code, no dependency, no workflow change.
Every check is a docs/gate-replica/diff proof. Proof root:
`/srv/qbuild/tmp/NA-0653_public_ci_link_repair_20260717T223023Z/`.

| # | Check | Method | Expected | Result |
|---|-------|--------|----------|--------|
| 1 | qwork proof invariants | read `.qwork/startup.qsl-protocol.kv` | all §3 invariants OK; head==origin/main==`a2d7c1c1` | PASS |
| 2 | Disk/mount gates | df + mountpoint | root <95%; /backup/qsl mounted | PASS (50%; mounted) |
| 3 | D-1276 absent pre-landing | canonical-line grep count | 0 | PASS |
| 4 | Exactly one READY = NA-0653 | anchored `Status: READY` grep count + STATE line | 1; READY=NA-0653 | PASS |
| 5 | Live CI finding | run 29617565812 log on `a2d7c1c1` | DENY=0, HC=0, TOTAL_MISSING 7 | PASS |
| 6 | Replica byte-copy | extract workflow lines 397–429, dedent, sha256 | sha256 `fa5ec033…` recorded | PASS |
| 7 | Replica-vs-CI agreement at base | run replica at base | same 7 files, TOTAL_MISSING 7, exit 2 | PASS |
| 8 | Fix set = the only missing links repo-wide | replica scan-set census | 721 files scanned; no 8th missing link | PASS |
| 9 | F1 premise: no fix-set doc in docs/public/ | path inspection | 0 members | PASS |
| 10 | F1 premise: every flagged line outside boundary-styled sections | enclosing-heading derivation per link | 7/7 in Related Evidence / Evidence Consulted | PASS |
| 11 | Canonical suffix byte-copy | extract from the reviewer-package precedent line | 114-byte suffix captured | PASS |
| 12 | The seven repairs | one Edit per file, whole flagged line replaced | display text preserved; suffix byte-identical ×7 | PASS |
| 13 | Minimality | `git diff --numstat` over the seven docs | exactly `1 1` per file (+7/−7) | PASS |
| 14 | Boundary sections byte-identical | sha256 pre/post under the pinned extraction rule | 4/4 identical | PASS |
| 15 | NA-0250 fenced transcript untouched | line 96 verbatim + zero diff hits | untouched | PASS |
| 16 | Suffix census | fixed-string grep under docs/ | 7 repaired files + the precedent line only | PASS (8 file-hits, no strays) |
| 17 | Zero link syntax introduced | bracket-paren regex over added diff lines | 0 | PASS |
| 18 | Gate replica at head | run replica post-repair | TOTAL_MISSING 0, exit 0, repo-wide | PASS |
| 19 | Untouched surfaces | `git diff --stat` census at PR | only the 7 docs + governance/closeout set; no .github/scripts/formal/vectors/source/Cargo/docs-public diff | PASS |
| 20 | git diff --check | whitespace-error scan | clean | PASS |
| 21 | cargo metadata --locked | run | OK | PASS |
| 22 | cargo fmt --check | run; compare to known base residue | exactly the 145 pre-existing diffs; zero lane delta | PASS (recorded, not fixed) |
| 23 | cargo audit (root + nested fuzz) | run both | exit 0 both | PASS (386 deps / 287 deps) |
| 24 | sh -n / bash -n qsc_adversarial.sh | run | OK | PASS |
| 25 | Added-line scans | private-material/prohibited/overclaim patterns over + lines | 0 hits | PASS |
| 26 | goal-lint local | synthesized event payload; literal `Goals: G4` body line | OK | PASS |
| 27 | Phase-5 exit condition | main-push public-ci on the lane merge commit | GREEN with TOTAL_MISSING 0 | PENDING (operator merge; verified post-merge) |

The lane PR never runs the link check (push-only step) — row 18 is the
pre-merge proof and row 27 is the exit condition, per D589.
