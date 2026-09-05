# NA-0779 -- AS BUILT (BORN AT THE PROMOTION): THE DEBUG LOG LANE

Goals: G4 (primary), drives G1-G3 delivery

Lane `NA-0779` (the ladder program's first lane) . `D-1422` . class: OWED (the Director declares it at close; none is invented here).

**EVERY DIGEST IN THIS FILE WAS COMPUTED FROM THE FILE AT GENERATION TIME OR IS MARKED AS CARRIED**, and where carried it names the record it is carried from. This file is BORN at the promotion PR and grows at the close. It is added with `git add -f` because `.gitignore`'s `**/evidence/` rule would otherwise silently drop it (`WF-0087`'s write half); the staging is confirmed with `git diff --cached --name-only`.

## 1. THE GOVERNING TEXTS, BY SHA (each verified BEFORE it was read; banked 444 under `state/operator/`)

| document | bytes | sha256 |
|---|---:|---|
| `KICKOFF_debug_log_lane_20260905.md` (the Director's kickoff) | 13437 | `0676fd5d29f2d8448fea11950270e490f30d6878b69da627fd9dfe60f2d33264` |
| `RULING_NA0778_017_20260905.md` (R112-R118; banked by this seat under `NA-0778/`) | 8182 | `5ffdc9d2b52d1e779adaa85984feff09744b912614d28d0e194d5b667672aeff` |
| `RBANK_cadence_amendment_and_cost_direction_20260905.md` (C2', K1; banked by this seat) | 2750 | `b886ae454a7c7988977a68b30c02ca3f1fda5e05ffef03ca22a669049e8fa117` |
| `RBANK_debug_log_first_and_no_presence_20260904.md` (D1-D3) | 3871 | `785e9e2fc5e5c80bcdb639cc72c75ae4cb9ff4881e33ecef4ab57643dd9828d7` |
| `RBANK_debug_log_extensive_audience_20260905.md` (A1-A4) | 3087 | `2f6356bc7d1aed839275cc359204c09183c18f9e2ab181b202e6e5b69215d0f5` |
| `RBANK_record_push_cadence_20260903.md` (C1-C5) | 4061 | `9747b9febbe21c47a0e3e2731fc655774f2e1d8c3996523f9b73c7d5c1746e49` |
| `method/DESIGN_delivery_ladder_metronome_v2_20260825.md` | 15775 | `aba8e2a5f8c388d1c7ac850c7b94790365cc9749e92e1a40b63ff22d056b8c59` |
| `method/AMENDMENT_1_delivery_ladder_20260827.md` | 4574 | `440b101929601704babbe356c5730746a5af2404ce51081e97544dd5f3952feb` |
| `method/AMENDMENT_2_delivery_ladder_20260827.md` | 3889 | `c41b8a59f293db58219d6c27c2585ebd1e608d0bf8cd616dbdf411a2ea2de8b8` |
| `method/ORDER_ladder_climb_sequencing_20260828.md` | 10864 | `0c9e9c46b990dc04e49749cff9e9be963ed9aaf77354c6aa6b411740e625cb58` |
| `TRIAGE_AND_PLAN_audits_2026-09-04.md` | 18838 | `e05f9401d1272782bed6bd7c1c3b1e06dadc7c8d6c7026bb2c943818b429db25` |
| `EXPECTATIONS_NA0779_001.md` (sealed BEFORE the checks; banked under `NA-0779/`) | 8164 | `3edc5bda03f595339ee9af9533a849c133816f9b592bec4207f1a7ffbab61768` |
| `NA-0775/CLOSEOUT_NA0775_20260901.md` (the class `ENG-0269`'s closure transcribes) | 5937 | `85821a72c715d0f9a0ff43280bb09f36fee06405be4eebd1801aaacab2264f27` |

## 2. THE AUDIT LANDING (kickoff act E(1)): EVERY FINDING OF `TRIAGE_AND_PLAN` SEC 1, ITS ENTRY, AND WHAT THE DIRECTOR HAD ALREADY VERIFIED

The reports themselves are NOT on this box (sec 4). Severities are the Director's PROPOSALS (sec 1); the OPERATOR ratifies them by merging the PR that lands this file (`RULING_NA0778_016` R106; `TRIAGE` sec 4 item 2). D-7 is already `ENG-0295` (NA-0778's close) and takes no second id. Where a line is given it was RE-MEASURED at `07612065` (protocol), `b4ec4693` (desktop) or `f201bb3a` (server) by this seat; "owed" means the report alone holds it.

| finding | entry | proposed severity | home | verification state (TRIAGE sec 0) | the lines, as re-measured by this seat |
|---|---|---|---|---|---|
| F-01 | `ENG-0300` | P1 LATENT | RATCHET lane A | [M] to the line; [X] mock primitives | `qsc/src/lib.rs` :1776, :1791, :2307, :2426 (equal) |
| F-02 | `ENG-0301` | P1 LATENT | RATCHET lane A | [M]; [X] | `suite2/ratchet.rs` :1712, :757 (equal); :1830/:1831 TRIAGE's |
| F-03 | `ENG-0302` | P1 LATENT | RATCHET lane B | [M]; [X] | `suite2/ratchet.rs` :10, :11, :317 (equal) |
| S-01 | `ENG-0303` | P1 | RELAY SPLIT lane, door (a) | [M] | server `src/lib.rs` :950, :1051, :1193, :1246; `store.rs` :722 (equal) |
| S-03 | `ENG-0304` | P2 (XS) | RELAY SPLIT lane | [M] | server `src/lib.rs` :313, :363, :375, :401; `main.rs` :280 (equal) |
| S-04 | `ENG-0305` | P2 | RELAY SPLIT lane | [M] | server `store.rs` :7, :48, :353, :364, :722; 0 delivery-counter tokens (equal) |
| S-02 | `ENG-0306` | P2 (design) | RELAY SPLIT lane | READ | owed |
| S-06 | `ENG-0307` | P3 (XS) | RELAY SPLIT lane | READ | owed |
| S-05 | `ENG-0308` | P3 (XS) | RELAY SPLIT lane | READ | server `src/lib.rs` :1046; DOC-SRV-003 :54 |
| S-07 | `ENG-0309` | P3 (XS) | HYGIENE (ops) | READ | `Caddyfile.example` :5; `qsl-server.service` :18; DOC-SRV-002 :20, :31 |
| S-08 | `ENG-0310` | P4 | HYGIENE (docs) | READ | owed |
| F-04 | `ENG-0311` | P2 | HANDSHAKE lane | READ | owed |
| F-07 | `ENG-0312` | P3 (XS) | HANDSHAKE lane | READ | `qsc/src/invite/mod.rs` :1126, :1380, :1552 |
| F-05 | `ENG-0313` | POSITION | recorded | READ | owed |
| D-7 | `ENG-0295` (existing) | P2 NOW | desktop half shipped at `14079140`; engine half HANDSHAKE lane; retired by the label split | [M] | `qsc/src/lib.rs` `channel_label_ok` :2635 (equal) |
| D-15 | `ENG-0314` | P3 (XS) | HANDSHAKE (engine) + HYGIENE (desktop) | READ | `invite/mod.rs` :430; desktop `index.html` :746 (no `maxlength`) |
| D-3 | `ENG-0315` | P2 | VAULT lane | READ | owed |
| F-14 | `ENG-0316` | P3 | VAULT lane | READ | a count only: 6 + 6 `derive(Debug)` |
| F-09 / D-4 | `ENG-0317` | P2-P3 | VAULT lane (format bump after key-not-passphrase) | [M] | `qsc/src/vault/mod.rs` :45-:47, :521, :596-:598, :667 (equal) |
| F-15 | `ENG-0318` | P4 | VAULT lane | READ | owed |
| D-16 | `ENG-0319` | INFO | recorded | READ | owed |
| D-1 | `ENG-0320` | P2 | ENV lane (desktop) | [M] | desktop `commands.rs` :887, :903, :904; `lib.rs` :322, :339, :448, :457 (equal) |
| D-2 / F-06 / F-18 / G-01 | `ENG-0321` | P2 | ENV lane (both) | [M] (F-06, D-2a); READ (F-18, G-01) | `protocol_state/mod.rs` :1017 (+ :173, :210, :220, :1000, :1047; `main.rs` :52); `transport/mod.rs` :2059, :2069, :2070 (equal); seam token 70 lines / 5 files |
| F-08 | `ENG-0322` | P3 | METADATA lane | READ; [X] size table | owed |
| I-01 / F-12 | `ENG-0323` | P3 | RELAY SPLIT lane | READ | owed |
| D-5 | `ENG-0324` | P3 | DIAGNOSTICS lane / DESKTOP HYGIENE | READ | desktop `ui/main.js` :1941-:1951; `settings.rs` :11; `commands.rs` :376-:382 |
| G-02 | `ENG-0325` | P4 | the checklist line; NA-0779 L3 | READ | desktop `Cargo.toml` :23; 26 commits behind |
| D-6, D-8, D-9, D-13, D-14 | `ENG-0326` (one entry) | P3/P4 | DESKTOP HYGIENE / DIAGNOSTICS | READ | owed (the report is absent) |
| D-10 | `ENG-0327` | P3 | the OPERATOR's `.github` act | READ | desktop `ci.yml`: `@master` x6, `permissions:` 0 |
| F-10, F-11, F-17, D-11 (+ R-7) | `ENG-0328` (one entry) | P3/P4 | HYGIENE bundle; the `.github` act | READ | owed; `Cargo.lock` git deps 2 |
| F-16 | `ENG-0329` | P3 (the seat's proposal; sec 1 does not grade it) | hygiene beside `ENG-0297` | [M] | `transport/mod.rs` :2183; `redirect(` 0 (equal) |

**THE RECOMMENDATIONS, RECORDED WITH SEC 1's DISPOSITIONS (not built here):** R-1 ADOPT -- the two-party interleaving simulator as the gate of both ratchet lanes, RED on main first; R-3 ADOPT door (a) -- the split (a read secret in the vault; the address by one hash; pull/ack present the preimage); R-4 ADOPT -- session reset and recovery, the LADDER DESIGN lane; R-5.1 ADOPT, S, now -- the "what we ship" claims document (the substrate of the no-warranty page); R-5.2 ADOPT -- external cryptographic review before any public release (the operator's engagement and budget); R-6.1 OPERATOR's -- un-remand `SR-17` with a tier table, its own governance act with a cold read (the Director's K1 commitment brings a lean proposal after this lane); R-6.2 ADOPT, the next records act -- `CURRENT.md` + one status convention (the "last `- Status:` bullet is the state" convention this act uses) + archive: `CURRENT.md` is OUTSIDE this kickoff's enumeration (`SR-02`) and is NOT built here, named as left out; R-6.3 OPERATOR's -- one product lane per records-only lane, which K1 now decides in favour of riders; R-7 -- the `cargo audit` job, the operator's `.github` act (`ENG-0328`); R-8 ADOPT in part -- the unverified state visible, Revoke prominent, the code never logged are in NA-0778; the copy and the default expiry are the operator's words at the contact-management lane (`D-1421`'s close block).

**"CONFIRMED SOUND" -- WHAT THE DIRECTOR VERIFIED AGAINST THE LIVE TREES (`TRIAGE` sec 0), RECORDED SO NOBODY RE-AUDITS IT.** Confirmed TO THE LINE [M]: F-01, F-02, F-03, S-01, S-03, S-04, F-06, F-09, F-16, D-1, D-2a, D-7 (every one re-measured by this seat at the current bases: equal). Confirmed BY READ, not re-executed: F-04, F-05, F-07, F-08 (its [X] half is the harness's size table), F-10 to F-15, F-17, F-18, S-02, S-05 to S-08, G-01, G-02, D-3 to D-6, D-8 to D-16, I-01 to I-03; nothing read contradicted a finding. NOT VERIFIABLE FROM THE CHAIR: the running AWS relay's `PULL_LEASE_SECS` (I-03, an operator measurement), Caddy's runtime config, the [X] results under the REAL primitives. BOUNDARIES kept: the [X] results measure the state machine under mock primitives -- the confirming rerun with StdCrypto on the pinned toolchain is OWED and is the RATCHET lane's first premise measurement, NOT this lane's; the relay's tests were not run; the desktop audit is a static read; `cargo audit` on the real lockfiles is owed. The harness is about our bytes: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` sha256 `d4c12526e754272e432a983e44e29708d1617e78c9199c7865dd3a2939dde524` at `07612065`, equal to the sha the harness copy claims (re-measured by this seat).

## 3. THE ID DERIVATION (`WF-0068`), BOTH CONTROLS, ALL THREE INPUT SETS

Sealed before the run (`EXPECTATIONS_NA0779_001.md` X1); run at 2026-09-05T06:20:38Z; instrument and output banked 444 under `state/operator/NA-0779/` (`derive_id.na0779.sh`, `DERIVATION_NA0779_20260905T062038Z.txt`). Repo truth pinned to `07612065` in the refreshed mirror (tip == bare `ls-remote`): declaring forms `^### NA-0778 —` 1 / `^### NA-0779` 0 / `^### NA-0899` 0; `^## D-1421 —` 1 / `^## D-1422` 0 / `^## D-1899` 0; `^### ENG-0299 —` 1 / `^### ENG-0300` 0 / `^### ENG-0899` 0; `^### WF-0105 —` 1 / `^### WF-0106` 0 / `^### WF-0899` 0; the widest NA needle's maximum `NA-9999` (NA-0733's heading, :37339) classified, not counted; occurrences of the four candidates in repo truth 0; desktop `NA-0779` 0 (`NA-0778` 72). Open PRs 0 on all five repositories (vacuous, said so). Operator tree: no `NA-0779` directory, 0 files; `NA-0778` 179 files; `NA-0899` 10 (control plants); `ENG-0300`/`WF-0106` 5 each (NA-0778's control statements); the frozen old root's `operator/` 0. Work tree: no `lanes/NA-0779`. Every count equal to the sealed expectation.

## 4. WHAT IS NOT ON THIS BOX (E-1), CITED BY THE SHAS `TRIAGE` SEC 0 PRINTS

| file | bytes | sha256 (as measured on the upload by the Director) |
|---|---:|---|
| `AUDIT_qsl-protocol_security_2026-09-03.md` | 34433 | `e4d91c0863e639afef6e7c5416a8c1941121b4fe0a29cdaf498864d4e7e38da3` |
| `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` | 18705 | `ea256c9990f29b18db10dd2111c86d41ec388f0cb0373a53426a47370352ae03` |
| `AUDIT_qsl-desktop_2026-09-03.md` | 20677 | `40b8bceca2f1673f441b6a66d11c97c1ab840dd1cffe677908065d177dd96e50` |
| `AUDIT_harness_exp.rs` | 14246 | `67313de62dbd826333fbb810007a4701b3d717791bd41880321ef9c02bc4cb3c` |
| `AUDIT_harness_results.txt` | 2500 | `d17ac9198e2467faadcf6e75dab7b2e5f966f0d6c4b771f178aaba874cc34b60` |
| `RECOMMENDATIONS_qsl_program_2026-09-04.md` | 13159 | `fbee4e3cb49ba04e1cb13af3e7ce869059d039dc5a5137404b1aa77715f0b85c` |

Searched by name over `state/`, the home directory, the frozen old root's `operator/`, `~/Downloads`, `~/Desktop`, `/tmp` (0) and by these six shas over every 2-60 KB file under `state/operator` (0); `state/operator/audits/` holds the July v1/v2 audits (`.docx` and their text twins) and the remediation plan only. The operator banks the six under `state/operator/audits/` at 444; the next records act fills the owed lines.

## 5. THE APPARATUS ACT (kickoff act D; `RULING_NA0778_017` R116): qsl-ops PR #1

Branch `na0779-drop-checkout-lanes` from `0680716aac6797a1a982afa3490f5224ace74519`; commits `b76e680591ef36f9fd857df84c43672df0cd3a77` (the fix and the selftest) and `e9b3f2daee4781f1fe0bf9c27c4586b5a7f8c547` (the C2' sentence); PR #1 open at head `e9b3f2da`, base `main`; identity `Tebbens4832 <238594419+Tebbens4832@users.noreply.github.com>`, 0 `Co-Authored-By`. The arms, measured 2026-09-05T06:23:41Z against the sealed X2: the tool at main on `lanes/TMP-drop-arm/repo/qsl-protocol` -> `Path escapes root ... is not under .../work`, rc=1, kept; the fixed tool -> `Removed disposable checkout ...`, rc=0, gone, the protocol mirror's tip unchanged; `--selftest` PASS rc=0 (ARM 1 under `lanes/` ACCEPTED; ARM 2 `work/` REFUSED; ARM 3 outside the root REFUSED); a scratch copy with the literal put back to `work/` -> `selftest FAIL` rc=1. `drop_checkout.sh` at the head sha256 `b7a8c8d8af7c668f5b2fe3d0a11deb6956f8b5e04954765e391ec13af7de28ed`; `CLAUDE.md` at the head sha256 `9a9db7a7da03f95ec85d1901ffe231e4b7ab1ddece0f92b79ea1fb99d2b60ab4`.

## 6. THE `WF-0086` MEASUREMENT (R118), FOR THE DIRECTOR'S TEXT

`remote-invite-roundtrip-tests` (`.github/workflows/remote-invite-roundtrip-tests.yml` at `07612065`): `on: workflow_dispatch` only; `concurrency.group: relay-remote`; one run in its history -- 32149418050, `workflow_dispatch`, success, 2026-08-18T14:36:56Z, head `726c3c8dedc8ceee2f68851b0c3a1e27e55c5ec1` (`gh run list --workflow`, `gh run view`). Its script `scripts/demo/qsc_remote_invite_roundtrip_smoke.sh` (747 lines): STEP 7 sends a message each way (`send --transport relay ... --to`), receives with `receive --transport relay ... --mailbox <route token> --from ...` (:550, :560; the comment at :517-:518 records the operator's choice of `--mailbox` over the omitted form, 2026-08-18), asserts equality on the delivered bytes and `event=qsp_unpack ok=true` per side (:554, :564), and counts `recv_commit` (:660-:663); `main-red-sentinel.yml` lists the workflow at :71. What the filing asked: a gate that FAILS when a message is sent and not received over a REMOTE relay end to end, with the operator deciding (1) required vs advisory and (2) which addressing. Measured: (2) `--mailbox`, answered; (1) undecided -- dispatch-only, n=1.

## 7. WHAT IS NOT CLAIMED

Nothing in the audit landing is repaired, re-audited or re-run; the [X] results stay mock-primitive until the ratchet lane's rerun; the reports' own coordinates are owed where marked. The census figures in `D-1422` are `git grep` counts, the SCOPE of the event model and not the model. The apparatus fix is proven on a plain directory and synthetic paths, not on a live lane checkout. Records only; nothing merged by the seat.
