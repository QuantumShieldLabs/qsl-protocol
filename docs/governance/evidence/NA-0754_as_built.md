# NA-0754 — AS BUILT: THE RELAY-PANE TRUTH LANE

**Lane:** NA-0754 · **Spine decision:** D-1396 · **Desktop decision:** D-0035 · **Ruling:** R379
**Bases (re-derived bare and unpiped at the NAMED `github` remotes, both mirrors measured STALE):**
qsl-protocol `c4c8e65b9af284b710beb3fd48fbf871785cc3c7` · qsl-desktop `cf0299a3b48cbd955a8d9acd9aca6cdda55dd0e8`
— and both are NA-0753's own merge commits, merged-ness read from `merged_at`, never `merge_commit_sha`.

## AUTHORITIES, EACH SHA-VERIFIED AGAINST ITS OWN BYTES BEFORE BEING READ

| artifact | sha256 | geometry |
|---|---|---|
| brief (banked SR-14) | `394b2453f1ec5d69695284cdaeb74f2013ca4ba4cf7015e9665d4a3d0f13c5a9` | 129 l / 9997 B, 444 |
| ruling `R379` | `5165debc0a1095aca1825a16be0b74907638085db274e46999b367365807f7d5` | 80 l / 6306 B, 444 |
| design bank v2 | `44f7d343c33ba4c2b382f326a18d85bfecebe9027834fdf59a0114c8e7a7cc12` | 35 l / 2607 B, 444 |
| copy bank F3 | `bdd3f083074b1f5e1642b4474cacbcfaaa1ee37a224da63f5ea1a951800f77c1` | 19 l / 1284 B, 444 |
| NA-0753 close-out | `b7a4844300d11cab2d2726071edbded3b89f4dd6ccdf6437370fbd3e163c1bb0` | 32 l / 2573 B, 444 |
| identity-display bank | `2dc9b285526d17505d48c920953303b2fcf223427049765a2ac6cc3019c3a373` | 1945 B, 444 |
| STOP 001 | `6d5a98be76e56749bf6696b20742a045353a8eda7c3518256b1b965bfa995c0e` | 792 l / 64004 B, 444 |

The brief was placed by COMPARISON, with the tamper control run FIRST and proven to differ
(ARM 1 `cmp` rc 0; ARM 2 rc 1) — a sha of the destination proves a file exists, never that the
intended bytes are in it.

## MEASUREMENTS

**Baseline, reproduced to COMPLETION before any edit:** 124 passed / 0 failed / 10 ignored, rc 0.
Inventory enumerated **134 == the pin, byte-identical** (`cmp` rc 0) with a truncation control
returning rc 1, so the comparison was proven able to fail before its result was believed.

**Post-lane:** 134 passed / 0 failed / 11 ignored, rc 0. Inventory re-pinned **134 → 145**.
`cargo fmt --check` rc 0 · `cargo clippy --all-targets -D warnings` rc 0 ·
infra-literal-scan selftest 13/13, tree clean (81 files, 25689 lines) · `test_inventory.sh` PASS.

**Nothing deleted.** Three tests pinning the SUPERSEDED model were renamed and re-aimed at the
inverse ruled behaviour, keeping their disciplines; the inventory gate caught all three as
disappearances — **the gate working** — and was re-pinned deliberately per its own instruction.

## SEALS — EIGHT COUNTERFACTUAL RED RUNS, PRESERVED 444

Every seal was proven able to FAIL before it was believed. Logs under
`/srv/qbuild/operator/NA-0754/redruns/`.

| counterfactual | arm proven able to fail |
|---|---|
| the probe persists the token | Y1 invariant — caught `TOKEN-BRAVO` vs `TOKEN-ALPHA`, **the exact drift a bare bool cannot see** |
| the probe persists the address | Y3 clobber — the working config lost, the `:844` diagnosis |
| the supplied CA path is ignored | Y5 — the missing-file arm collapses to `unreachable` |
| `EnvGuard` never restores | the env-leak arm — `QSC_RELAY_TOKEN` escapes the probe |
| persist moved ahead of the probe | the order pin |
| settings.json written first | the restored-order pin |
| a pending-removal flag reintroduced | the immediate-delete pin |
| the `~` expansion not written back | the visible-expansion pin |
| one character of copy drift (em-dash → hyphen) | the F3 claim-set pin |
| the retired one-liner survives beside the new copy | the F3 retirement pin |

## THE CLAIM BOUNDARY, STATED RATHER THAN IMPLIED

**No relay is reachable from this harness and none was made reachable.** No fixture relay exists in
qsl-desktop — every scenario address is the reserved non-resolving `.test` TLD and the Rust IPC
tests use the discard port — so the GREEN half of the model (a Connected test persisting the tested
triple; a working configuration surviving a later failed test) is **not a CI gate**. Its ENGINE half
is sealed relay-free in `na0754_persist_boundary.rs`, whose differ-control proves each of the three
observables CAN move before proving the probe leaves them alone. Its LIVE half is the operator's
acceptance flight, recorded [O]. The fixture is filed as `ENG-0226`.

⚠ **`R379` §Q4 corrected the seat on two of these.** The seat reported Y5's accepting arm and Y1's
differ-control as undrivable; both were drivable relay-free, and the seat's own §4 measurement (the
CA check runs BEFORE any socket) is what licensed the first. Recorded as prediction row 180, a MISS.

## PREMISES THAT MEASURED FALSE, RECORDED RATHER THAN QUIETLY ABSORBED

1. **S2(d)'s degrade contingency was never reached.** The brief allowed the CA check to degrade to
   an exists+PEM-header sniff "if cert-parsing needs a new dep". It does not: a full PEM parse
   already runs in `relay_http_client()` via `reqwest`, already a qsc dependency. Row 179.
2. **The desktop's own doc comment about qsc was false** (`commands.rs:628-629`), and so was
   `main.js:1270-1272`. `relay_ca_file_set` never touches the filesystem. **No test asserted
   either claim.** `ENG-0222`.
3. **The carried-forward PREDICTION_LEDGER maximum (169) was stale by eight rows** — measured 177.
   A base-scoped note re-measured rather than inherited.
4. **`all_27_registered_commands_…` is a stale claim at base** — 38 commands are registered and the
   test invokes 20. Pre-existing (NA-0751 added twelve gateway wrappers), outside this lane's
   enumeration, and REPORTED rather than touched.

## THE LESSON THIS LANE PAID FOR THREE TIMES

**A source-text pin cannot tell a comment from code, so documenting a removal re-plants it.**
A comment enumerating the four retired helper sentences put all four back into `main.js` and would
have turned their absence seal green-when-it-should-be-red; a comment explaining the vault write
mechanism spelled the very construct `no_secret_is_written_outside_the_qsc_vault_trios` forbids; and
the R-space sweep's own classification sentences are why that space's raw content maximum reads
`R391` while its declaring maximum is `R378`. ⇒ **Describe a retired construct, never spell it; the
retired wording belongs in the records, which no seal reads.**

## DELTAS BEYOND THE RULED ENUMERATION — REPORTED, NOT SLIPPED IN

- `src-tauri/src/lib.rs` — the two ruled command-layer functions cannot EXIST as commands without
  `generate_handler!` registration. A mechanical consequence of the ruled requirement.
- `src-tauri/tests/fixtures/na0754_ca.pem` — a real certificate the admitted engine test needs for
  the CA rung's ACCEPTING arm. Certificate only; the private key was generated, used and discarded,
  never written into the repo.
- The 5+1 polish note was recorded at the END of `ENG-0205`'s block, that being where mockup-07's
  layout authority lives; if the Director meant a different home, it moves without loss.

## OBSERVATION, UNPROMPTED AND NOT ACTED ON

`.gitignore` carries no `__pycache__` / `*.pyc` rule, so running the infra-literal scan locally
leaves `scripts/ci/__pycache__/*.pyc` STAGEABLE — it was caught in this lane's `git add -A` and
removed by hand. Any lane that runs that gate locally can commit a build artifact without noticing.
Not filed: outside this lane's enumeration, and it is the operator's call whether it is worth a rule.
