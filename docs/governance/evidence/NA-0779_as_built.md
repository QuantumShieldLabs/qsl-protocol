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

## 8. THE SINK PR (STOP 003; `RULING_NA0779_002` R6) -- THE ENGINE HALF, AS BUILT

Branch `na0779-debug-log-sink` from `267657deaba92aa2c62b8a9dac7cacd2202fec66` (main after `#1818` merged). Governed by `RULING_NA0779_002_20260905.md` (sha256 `8fc89fd58cd2c8da76a515048cf67e5aa7feac4210ba4bbc38dbcd6f9e45ab4c`) and the operator's bless (`RBANK_debug_log_event_model_blessed_20260905.md`, sha256 `78f10bd501d34db9ab86937eb754f8df6b3e2e68c5a838515aa6d7382e67ac14`, the word "blessed"); mockup 18 amended to 18b under that bless (banked, sha256 `9fb1940ad6db19fe65e6ec54cc558197471221e050bb889f235c7195f569f51c`; the reference markup of STOP 004). Expectations sealed before the post-seal checks: `EXPECTATIONS_NA0779_003.md` sha256 `8f300d100f170693a742c816f88570a4e8c2247355fd001bdcf8433ed2ac82ba` -- its premise states the deviation that the first arms ran before it existed.

| file | what |
|---|---|
| `qsl/qsl-client/qsc/src/output/mod.rs` | `pub mod event;` and ONE call in `emit_marker`, `event::feed(event, code, kv)`, after `format_marker_line` and before the routing match. The line is untouched. |
| `qsl/qsl-client/qsc/src/output/event.rs` (born) | the typed `Event`; `event_from_marker` / `event_from_marker_with(&Allowlist, ..)`; `set_event_sink` / `event_sink_installed` / `feed`; `utc_rfc3339_ms`; `Event::to_line`. sha256 `c6e6e151aacd671ae1b62063e47bfda800c3614124aeb28d9652b30a7e467945`. |
| `qsl/qsl-client/qsc/src/output/event_tables.rs` (born, GENERATED + rustfmt) | `INT_KEYS` 48, `BOOL_KEYS` 23, `ENUM_KEYS` 60 with 329 members, `LEVEL_EVENTS` 105, `LEVEL_DETAILED_ONLY` 60. sha256 `872e514fdfa18d77737041b2a8d1714e4804148867eddd78b4dc71af2c61bd15`. |
| `qsl/qsl-client/qsc/src/lib.rs` | `nr` / `ns` / `pn` at the four ratchet sites (`qsp_dh_ratchet`, `qsp_pq_reseed`; send and recv). |
| `qsl/qsl-client/qsc/tests/na0779_debug_log_sink_arms.rs` (born) | t1-t6, red first. sha256 `e542cd7ce6bf07d3aee160e2e85ebf97620debe6961270e2642554b96335170e`. |
| `scripts/ci/QSC_SHARD_MANIFEST.txt`, `_MACOS.txt` | the new test file listed (exact cover kept). |
| `DECISIONS.md`, `TRACEABILITY.md`, this file | DV-9 (the impl), DV-10 (E-1), the amended claim boundary; the dated line; this section. |

THE ARMS (all after the seal): t1 RED 7 of 7 plants under copy-every-key, GREEN 0 of 7 under the real allowlist through the real `emit_marker` with a sink installed, no fragment in the debug form; t2 105 / 60 disjoint, probes and unlisted names yield nothing; t3 `?` outside a vocabulary, typed drops, `nr`/`ns`/`pn` ints, ASCII members; t4 opt-in, the slot clears; t5 the exact line and two civil-date controls; t6 the 18 wrapper-only names -> 0 events through the real wrappers, the positive control `vault_unlock` -> 1 event with `state=?`. `cargo test -p qsc --test na0779_debug_log_sink_arms`: 6 passed. `cargo test -p qsc --lib output::`: 4 passed (no regression). THE CLI-UNCHANGED ARM: base binary sha256 `6fcc9f6a193fa641999a9ee8f09be291ff42b69b7a58bf3e790f8f24a00c9c84`, head binary sha256 `58d114c3bead8298dcc771f92db61a1d6abd52a5a3ad0c367bdc2ed70ed2c399`; the five `*.markers` byte-identical (`alice` = `bob` `52ce18365d22cf6034864e30acb0b87011c92c13089c91b4dcc411f1aa47cc43`; `alice_recv` = `bob_recv` `60459e9ffd7db6de4af382d28d11eeaf588117c59b6139e95a663d5c6479165a`; `relay` empty). The richer loopback driver (14 markers per actor, ten events) identical with `fp=` masked. Clippy 1.98: rc 0, no warning in the new files.

E-1 (DV-10): the 18 names / 31 sites / 20 keys emitted only through the two thin wrappers were outside STOP 002's census; they never enter (t6) and are NOT admitted by this PR -- the bless is on bytes; a proposed classification is offered for the operator's word in DV-10 and the stop-file.

`SR-15`: product source files 4 (2 modified, 2 born), test files 1, manifests 2, records 3. ONE read, after the flight, over both PRs at the pinned head rev (R6).

## 9. THE SECOND COMMIT (`RULING_NA0779_003` R2): E-1 ADMITTED, THE TABLES REGENERATED, THE ARM FLIPPED

On the operator's word (`RBANK_wrapper_names_admitted_20260905.md`, sha256 `6f6bb8be2d55d760d549b824f7e7f2aae6b4c4534511fd70288fe738b73b17bf`: admit as recommended -- `bucket_size`, `bundle_len`, `payload_count` stay VALUE) under `RULING_NA0779_003_20260905.md` (sha256 `f507346eb8fb608c61fe12a936a91631817fe32d895972dd9ff38267abbd7443`). The census was EXTENDED with the 31 wrapper sites (`census_sites_r2.na0779.json`, 394 sites) and the tables regenerated by the generator plus `rustfmt` (`event_tables.rs` sha256 `86aeaedf845bb9b106843339e44d1ffaccbff4c3346ab51fb336863e041570ca`): 52 int / 32 bool / 62 enum keys with 346 members; 117 events-level / 66 detailed-only names; the three empty vocabularies stay and render `?`. t6 rewritten (sha256 `7931343d1d633b5eb8fc54a09f6eb9bf27753158c715dd639ba7077b844d24d6`): the 18 names through the real wrappers -> 18 events at their levels, five planted VALUE keys absent, `qsc_mark`'s code listed, the control's `state=unlocked` rendered as itself; t2's counts 117 / 66. At the new head's binary (sha256 `02cafb2da0584c25f0b8ea81ccb564e3497ba986fb7b2226f19fe324b02d5c02`): 6 arms passed, 4 unit tests passed, the named demo's five `*.markers` identical to base, the richer loopback driver (14 markers per actor, 11 events) identical to base with the per-run `fp=` masked -- after one discarded run whose stub path the seat had set wrong (named in the arms log). The admission in diff form: `CENSUS_R2_instrument.na0779.diff` (sha256 `824c6a29788e5da88f5440a172f361f7be11994fcf5f12d6bdccd0f9eba4bb24`). The yank rider of R3 is PR #1820 (Cargo.lock only).


## 10. Closeout continuation — prepared 2026-09-09; acceptance satisfied; records and post-merge closeout pending

Goals: G4

Protocol sink #1819 merged at 4e03092fb14a, lock rider #1820 at 8d6c234f1384, and desktop #56 at 48b031574ac8 from ebd92e3ee230. The prior continuation observed 82 successful protocol-main checks and five successful final-desktop-head checks; these are implementation-head observations, not this draft's CI. The follow-up rechecked the remote mains, merged desktop head, six repositories' empty open-PR lists and both banked flight binary hashes. No merge was performed by this executor.

### Mutex warning and explicit source deferral

The sink callback is invoked while the sink mutex is held. **The sink must not emit a marker**, because that would re-enter the same sink path under the held mutex. The code already has this behavior. Per the Director's follow-up, N-14's outstanding engine source documentation sentence is DEFERRED to the next relevant source change. The exact proposed patch remains in lane evidence; only this continuation's verified one-line addition was removed, leaving event.rs byte-identical to HEAD. The desktop install-site warning has landed. No token test, executable change or linter weakening was made.

### Result

DEBUG_LOG_TYPED_ALLOWLIST_FLOWN_PASS

### Acceptance evidence boundary

The Director's final-build disposition satisfies NA-0779 acceptance on desktop ebd92e3ee230, merged in #56 as 48b031574ac8. Operator observations: two Copies produced different nonzero labels; lock/wrong-passphrase/successful-unlock was performed; invalid-directory treatment passed; the real A-to-B exchange left both connections green. The Director verified the lock export's contiguous gw.lock then successful gw.unlock boundary without retained failed-attempt events; A's 115 contiguous events with handshake_complete and nothing/offered, both out=ok; B's 77 contiguous events, sequences 53–129, with handshake_complete and nothing/finished, both out=ok. All three had valid footer digests and the correct build. Event-line scans reported no URLs, IPv4 addresses, sensitive-value keys or long hexadecimal values. This is bounded export evidence, not a general security certification. Legacy compatibility admission and B's peer_confirmed=false remain inherited findings, not fixes. Bank: RBANK_NA0779_final_build_acceptance_20260909.md (sha256 fc8e9b8526d4).

Local custody is complete and remains separate from the Director's acceptance verification. All three exact exports were independently matched to the Director's complete whole-file SHA-256 values and banked mode 444: ACCEPTANCE_EXPORT_lock-unlock_7885c08ec67a.txt, ACCEPTANCE_EXPORT_machine-A_6bc19fb7bd18.txt and ACCEPTANCE_EXPORT_machine-B_4138846e5ceb.txt. The two later transfers were located in Downloads. Independent local checks confirmed ASCII, the expected desktop build, valid footer digests and contiguous event sequences: lock/unlock has 21 events starting with gw.lock then successful gw.unlock and no failed-attempt events; A has 115 events; B has 77 events, sequences 53–129. B's invite_finish result words are finished and nothing. This local file verification closes the acquisition gap; it is not a new operator flight or a new security certification and does not alter the Director's bounded acceptance disposition.

The log remains an intentional millisecond activity timeline. Screenshots and typed-input verdict capture are separate raw channels; census drift and Q7 lock edges remain as recorded.

N-19 corrections: the original sequence assertion covered the unit path; F-02 later added the intake latch and app-path proof. The old pill became the main-window footer's right end, with no footer on the unlock window. debug_log_control alone writes the debug_log field, not the entire settings file. The earlier flight's destination-bearing success text predates the shipped short line. N-12's intermediate eight desktop reasons became seven at the final head when not_finished was retired. Final invite_finish reads names from the upstream qsc queue before gateway drain; it does not consume the richer MarkerBuffer store.

### Three-axis retrospective

1. Product/correctness: the typed log produced useful two-device evidence; the cold read found four defects missed by earlier flights/arms. Its utility is not a handshake repair or final-build acceptance result.
2. Evidence: historical discriminating redaction arms and one shared formatter support the bounded claim. The richer CLI comparison is stronger than the older vault-locked smoke. The read's true nine-commit subject differed from the Director's two-commit commission. Preserve distinctions between reported findings, source inspection, mock results and operator observation.
3. Cost/process: five principal stops was a target. Four principal stops and multiple returns/supplements are banked; STOP 005 is pending. Count principal stops, supplements, returned commits and operator decisions separately. This follow-up resolves the missing section and source/test conflict without inventing tests; successor approval and publication scope are now resolved; final-build acceptance is now satisfied; records merge and post-merge gates remain.

### Post-fix hardening review

- Stress correctness: this is a verified source-comment rollback; no concurrency fix or new stress result is claimed. The held-mutex warning matches feed's callback path.
- Minimality: event.rs remains byte-identical to the merged implementation. The original 292 ledger additions are preserved in the pre-edit evidence snapshot; this records PR incorporates their reviewed corrections and additional filings. The proposed revision changes the 14 authorized records/audit paths plus the Director-authorized test-only census tables in ratchet.rs. Production bytes and scanner logic remain unchanged.
- Maintainability: the exact N-14 patch is preserved, the as-built carries the warning, and source placement is explicitly owed with the next relevant source change.
- Coverage: the prior continuation ran six existing sink tests. Required 4A was rerun successfully for this closeout; committed-tree goal-lint, infrastructure/secret scans and link checks passed. No artificial test was added to satisfy metadata. Historical runtime tests remain distinct from these current documentation checks and PR-head CI.
- Cross-platform: no runtime/platform change remains. Both cached flight profiles match their banked hashes; this is binary identity, not Linux or macOS acceptance. No new macOS run occurred.

### Complete rider disposition and audit provenance


The Director supplied section 9(a) in the current follow-up. It is no longer a missing input. “Prepared” means an evidence draft, not a landed record; “implemented” refers to the cited merged implementation, not final-build operator acceptance.

| Requirement | Disposition and evidence |
| --- | --- |
| SR-16 S-31, NA-0778 | Completed filing: existing row 485 in PREDICTION_LEDGER, accepted at R115; no duplicate row. |
| Executor rows STOPs 002–004 and Director corrections | Prepared in SR16-ROWS.md and the prediction draft, including STOP 001's tamper-value row once, the Director's fifth/sixth correction, the shell-cause correction, commission two/nine correction, and final bool-result correction. Numbered rows 486–505 are filed after the final declaring-row census. |
| E-38 under WF-0104 | Completed filing on protocol main; retain the existing entry. |
| C2' beside C1–C5 | Completed filing at D-1422 DV-4 beside the D-1421 cadence reference; current tools card contains it. The tools update is recorded completed in qsl-ops #2; do not reopen it. |
| Relay lease beside ENG-0142 and Amendment 1 A4 | Prepared ledger/as-built cross-reference to MEASUREMENT_relay_pull_lease_20260905.md: operator-read file specifies 60 seconds and retention 604800 seconds. This is not a new measurement of the running process. |
| Thirty audit filings and harness ratchet SHA | AUDIT-FILING-MAP.md names all thirty ENG-0300–0329 rows and exact source filenames. Protocol report header supplies the ratchet SHA; matched to ratchet.rs at the pre-classification source revision during this continuation. Archive digests in the reports are not Git commit IDs. |
| ENG-0326/0328 split | Prepared: ENG-0330–0334 split the former; ENG-0335–0338 split the latter, which retains the audit-job remainder. Findings remain open; a split is not a repair. |
| One name per person | Implemented in desktop #56 via exact immutable-alias display lookup; operator I12 confirmation is banked. Current contact name for accepted invitations, mint label for pending; stored invite/contact link still open (CLI alias mismatch/same-label limitations). |
| Status bar / rail icon banks | Implemented in #56; proposed filings ENG-0353/0354. Main-window-only footer; surface-derived vault words remain a limitation. Rail square 34px rather than reference 36px is explicitly recorded. |
| Public roadmap | Full evidence draft follows ROADMAP_of_record_20260905.md, retaining its order and marking Invitation reliability placement as a proposal. Not published. |
| Six audit files under docs, reproduction README | Prepared byte-identical public drafts under publication/docs/audits/2026-09-03/ with README; the same reviewed bytes are now placed in the repository. Private-material scan found zero hits; no redactions. The exact seven paths were added through automatic review and drafts placed locally; publication awaits acceptance and PR landing. |
| Claims document only if drafted | No current Director-drafted claims artifact located in the available lane/operator filename discovery. No claims document invented; conditional item pending a supplied draft if one exists. Historical claim-correction directives are not that artifact. |

| v7.7 item | Completed, pending or deferred |
| --- | --- |
| N-05 | Copy/export distinct actions implemented; correction that neither event nor file contains destination path prepared. |
| N-06 | Timeline/privacy and relay-policy-triple claim boundary prepared in DECISIONS/as-built. |
| N-07 | relayProven reset implemented; surface-derived vault state limitation deferred. |
| N-08 | Generation-counter/reset-detection debt prepared as ENG-0339; deferred implementation. |
| N-09 | Third settings writer / launch discriminator prepared as ENG-0340; deferred. |
| N-10 | Client viewer buffer may outlive ring entries, ENG-0341; deferred. |
| N-11 | Off does not clear, and export header lacks off state, ENG-0342; operator design decision deferred. |
| N-12 | Seven final desktop reasons implemented, intermediate eight-member description corrected; remaining reason vocabulary debt ENG-0344 deferred. |
| N-13 | Screenshot raw channel and pre-existing typed-input verdict boundary prepared; neither covered by typed-log redaction claim. |
| N-14 | Desktop warning landed; engine source sentence explicitly deferred by this follow-up, exact patch preserved. As-built mutex warning included now. Source equals HEAD. |
| N-15 | Richer MarkerBuffer debt ENG-0343 deferred. Final invite_finish peeks the upstream qsc queue before drain, not the richer MarkerBuffer store. |
| N-16 | Live-event/spec unit arm implemented in #56. |
| N-17 | Miskeyed off_surface measurement removed in #56. |
| N-18 | Accent failure treatment implemented; operator invalid-directory observation PASS in the final disposition. |
| N-19(a–e) | All five corrections prepared: original unit-only lock claim and later app latch; main-window footer; sole debug_log-field writer versus other settings-file writers; earlier flight text versus shipped short success line. |
| Q7 | Panic before on_lock and erase/destroy Err before wipe remain deferred lock edges; no repair or new safety proof claimed. |
| H1 / H7 | ENG-0345 simultaneous invitations and ENG-0346 refused-frame lease recurrence remain open, attributed to the Director's exports. Proposed successor includes reproduction, instrumentation and code-based design. |
| H2 | ENG-0347 remains a hypothesis. Source correction: each invite_accept pulls the slot selected by its own invite ID; distinct IDs are distinct slots. Two calls alone do not prove two pulls of the same mailbox. Route-aware runtime measurement remains owed. |
| H3 / H4 | ENG-0348 stuck-contact recovery included in proposal; ENG-0349 LegacyCompat suite admission stays deferred to separately approved hardening. |
| G1–G3 | ENG-0352 tracks absent invitation relay/disposal events, vocabulary gaps and proposed random correlator. Necessary instrumentation is a successor requirement; exact census, privacy review and scope are still required. Correlator design is not selected. |
| Later Director corrections | RULING 005 R3/R6 and D-0048 004e corrections included in the record drafts and SR16 rows. Final Finished/Offered/Nothing replaces the erroneous false→not_finished interpretation. The later Director disposition verifies final-build cases on ebd92e3ee230; earlier 962f01ab verification remains historical. |

H5/H6 remain separate notes ENG-0350/0351; they are not silently included in the successor's repair scope. No handshake fix, StdCrypto rerun, final stop, record push or freeze occurred in this closeout continuation. Final acceptance is declared from the new Director disposition.

### Audit filing-to-source inventory


All filenames below are proposed under `docs/audits/2026-09-03/`. The thirty filings already exist; this adds source provenance without asserting publication or current-source reproduction. D/F/S identifiers follow the original triage. Archive digests are not commit IDs.

| Filing | Report finding | Evidence filename(s) |
| --- | --- | --- |
| ENG-0300 | F-01 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0301 | F-02 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0302 | F-03 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0303 | S-01 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0304 | S-03 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0305 | S-04 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0306 | S-02 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0307 | S-06 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0308 | S-05 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0309 | S-07 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0310 | S-08 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md` |
| ENG-0311 | F-04 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0312 | F-07 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0313 | F-05 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0314 | D-15 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0315 | D-3 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0316 | F-14 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0317 | F-09 / D-4 | `AUDIT_qsl-protocol_security_2026-09-03.md`; `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0318 | F-15 | `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0319 | D-16 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0320 | D-1 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0321 | D-2 / F-06 / F-18 / G-01 | `AUDIT_qsl-desktop_2026-09-03.md`; `AUDIT_qsl-protocol_security_2026-09-03.md`; `RECOMMENDATIONS_qsl_program_2026-09-04.md` |
| ENG-0322 | F-08 / experiment D | `AUDIT_qsl-protocol_security_2026-09-03.md`; `AUDIT_harness_results.txt` |
| ENG-0323 | I-01 / F-12 | `AUDIT_qsl-server_desktop_interaction_2026-09-03.md`; `AUDIT_qsl-protocol_security_2026-09-03.md` |
| ENG-0324 | D-5 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0325 | G-02 | `RECOMMENDATIONS_qsl_program_2026-09-04.md` |
| ENG-0326 | D-6 / D-8 / D-9 / D-13 / D-14 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0327 | D-10 | `AUDIT_qsl-desktop_2026-09-03.md` |
| ENG-0328 | F-10 / F-11 / F-17 / D-11 / R-7 | `AUDIT_qsl-protocol_security_2026-09-03.md`; `AUDIT_qsl-desktop_2026-09-03.md`; `RECOMMENDATIONS_qsl_program_2026-09-04.md` |
| ENG-0329 | F-16 | `AUDIT_qsl-protocol_security_2026-09-03.md` |

The original protocol report header names the ratchet SHA; README.md in the publication draft preserves the complete verification value and reproduction recipe. `AUDIT_harness_exp.rs` and `AUDIT_harness_results.txt` supply the mock experiments for ENG-0300–0302 and the size evidence for ENG-0322. The pre-classification ratchet source digest at 4e03092fb14a was matched, not its behavior revalidated under StdCrypto. The later test-only table additions change the whole-file digest; they do not change that historical reproduction evidence.

### Remaining gates

The successor and its placement are approved and banked; READY remains NA-0779 until closeout. The seven publication files are covered by the updated selected scope and placed locally. Final-build observations and Director verification satisfy acceptance and authorize this PR; all three exact exports are now locally hash-verified and banked. Claims documentation is conditional on a supplied Director draft. SR-16 rows 486–505 were derived at this filing. The final record sequence still requires its existing narrow grants: classification/baseline, final stop naming merge commits and local record tip, installed-hook push, bare remote-tip proof, then freeze. New needle hits invoke C2': classification/supplement and Director direction before retry. No final stop, push, remote-tip change or freeze is claimed.

Approval follow-up, 2026-09-09: the exact NA-0780 Invitation reliability block and placement immediately after NA-0779 closeout are approved in RBANK_NA0780_invitation_reliability_approved_20260909.md (sha256 53852629b28b). The seven reviewed audit publication files are now placed locally under the approved scope; original audit bytes and README limitations are preserved. Final-build operator acceptance is satisfied by the Director disposition. No successor implementation, READY advancement, record push, merge or completed post-merge closeout is claimed.

### Publication-census blocker found during custody completion

The two transferred exports now satisfy local custody, but required GitHub ci-4a has a separate blocking failure. Its reference-crate DH census scans the byte-preserved historical AUDIT_harness_exp.rs and rejects the two ToyDh calls in establish at lines 113–114. This was observed on PR #1822 head 981e1eebafc8 and reproduced by one exact local test on 136081efc711. The source test requires reasoned file/function classification plus pinned counts; editing that source is outside the closeout scope. An approved classification or revised publication form is needed before a green-board claim. No original audit bytes, production DH behavior or guard were changed. The previously passing run_4a.sh validates bundles/OpenAPI/schemas/bounds and is not the complete GitHub ci-4a Rust job. ENG-0034 records this publication-gate finding separately from its closed product fix.

### Director-authorized census correction and remaining test-path gate

The preceding classification blocker is superseded by the Director's exact authorization and automatically approved scope extension. The archive's establish function now has one explicit ToyDh reason in ALLOWED_UNGUARDED_DH and its exact file is pinned to 2 sites. No production code, scanner logic, directory exclusions or publication bytes changed. This revision is records plus a test-only census correction.

Observed local proof in closeout-dh-census-20260909: real census FAIL before (the two archive sites), PASS afterward; full reference-crate suite PASS before commit. The reusable check_archive_census.py copied all 2427 tracked files into a disposable fixture without hard links. Classification present: PASS. Classification removed: FAIL for unguarded archive calls. Extra mock call in establish: FAIL at pinned-count comparison, measured 3 versus expected 2. Fixture restored: PASS. Whole-file hashes verified all original tracked files unchanged during mutations. Original publication files remain byte-identical, including the source harness; the StdCrypto rerun remains owed.

Post-fix hardening review: count drift within the already classified function fails closed, and removal still fails; the exact inverse of the two table additions reproduces the original source bytes, proving minimality. The existing reasoned-table mechanism remains maintainable without new scanner branches. The mutation checks execute the actual Rust census and distinguish its two failure reasons, rather than asserting text presence alone. Linux execution is observed; no new macOS execution or remote corrected-head CI is inferred.

Required goal-lint independently requires a changed tests/vectors/harness path for this source location. The authorized two table additions and records do not supply such a path. A substantive reusable fixture test is prepared in lane evidence at check_archive_census.py; proposed publication destination tests/na0779_dh_census_classification.py requires explicit scope authority. It runs the real census and the requested disposable mutations, preserving originals. No token test or gate modification is made. The correction push remains held at this gate; committed-tree validation and the precise local/remote tips belong in the continuation's evidence and PR update. Operator merge, final durable-record push, freeze and NA-0780 remain gated.
