# NA-0777 — AS BUILT: THE BUILD-TREE CUTOVER

Goals: G4 (primary), supports G1–G5

Lane `NA-0777` · `D-1420` · class **`TREE_CUTOVER_LIVE_BACKUP_UNATTENDED_PASS`** (declared by the Director, `RULING_NA0777_014` R113).

**EVERY DIGEST IN THIS FILE WAS COMPUTED FROM THE FILE AT GENERATION TIME, NOT TRANSCRIBED.** A retyped digest is a claim; a computed one is a measurement. This lane recorded a predicted mode string modelled from an assumed value while the real measurement sat in its own prior stop (prediction-ledger row 418), and the rule it earned applies here.

⚠ **THE RECORD OF AUTHORITY MOVED DURING THIS LANE.** Stops 001–007 were written to the old operator root; stop 008 onward to `state/operator/` in the new tree, which has been the record of authority since S4. The old root carries a `MOVED_TO.md` and a frozen relay pointer that redirect to the new one, and is frozen read-only — **not deleted**.

## 1. THE RULINGS — NAMED, NOT COUNTED

`RULING_NA0777_015` sec 0 gives the figure as **fourteen**. Measured by listing the lane directory with the numbers gap-checked `001`–`016`, it is **FIFTEEN**, `001`–`015` with no gaps. The low count came from a directory that was itself incomplete: rulings `006` (the hook read's) and `012` (on STOP 011 AMENDED) were still in the operator's home and unbanked when that figure was taken. Both are banked at 444 by this act. This is why the instruction was to NAME them rather than count them.

| ruling | bytes | sha256 |
|---|---:|---|
| `RULING_NA0777_001_20260903.md` | 8611 | `1211838dc7de440e66315b0722136eabd25986dc96bc4368ff0653d550e50499` |
| `RULING_NA0777_002_20260903.md` | 6658 | `43ee39b8641759f5aceb02da99a6edfd02edef3e24238fa7b636006fb03361e4` |
| `RULING_NA0777_003_20260903.md` | 5200 | `c28092f61c35d15d18fe9018bb97c828dc512f796f512fd368b2608df39ec7bc` |
| `RULING_NA0777_004_20260903.md` | 7363 | `cec6744915d3bb43a3cb4345fa6cb8132d249aa8ddf92105ff2437e44b2025d9` |
| `RULING_NA0777_005_20260903.md` | 7611 | `f595ae7f09ee836aaa163545c87a16b4982e65331ea33964c63703432ee0fa8e` |
| `RULING_NA0777_006_hook_read_20260903.md` | 10586 | `2a3b0cac0b1f495f5952fb157cb27c65f4f184d46fc7129ac87280d7b8e20472` |  ← the HOOK cold read's ruling
| `RULING_NA0777_007_20260903.md` | 8146 | `f32ea162bca8e23e0f193d23a158b64627babf310120cefc64d848cff2c14ed3` |
| `RULING_NA0777_008_20260903.md` | 4934 | `baf7a7eb9379fcd7b64eb9aa3aec7ed50d07bf2504b008fef44fee12b8dcccee` |
| `RULING_NA0777_009_20260903.md` | 7016 | `3558af2be2b0793637f5be99f1276f4b79ea0e28096e2f615e31fab5875ce0fd` |
| `RULING_NA0777_010_20260903.md` | 5810 | `4776745215716cdda603fa0f0ad5064e812c7d8413880068c2676f233bf221ed` |
| `RULING_NA0777_011_20260903.md` | 5018 | `5d0a385e13b4832e57e8eed424cc93e440e8e4668a8f8891775fd49d339eccac` |
| `RULING_NA0777_012_20260903.md` | 4327 | `a85b9543d56291a2ece51687117cc21f35419310b0eca6ba0f7066acf841cb74` |
| `RULING_NA0777_013_S5_read_20260903.md` | 10559 | `2588c3112defda4d959196635d271eb0a727187ef977133bea2dc05121d56e28` |  ← the S5 cold read's ruling
| `RULING_NA0777_014_20260903.md` | 5295 | `236d01ad1f5b9ba319923e05e4a169974184547d8b2552ad05f0e07b106dcdd1` |
| `RULING_NA0777_015_closeout_20260903.md` | 6305 | `4ed898d86332a29b49b7793b00c5889f3a97e0e72fcb29e38025728db789f233` |  ← the close-out

**COUNT BY LISTING: 15.** All 444 and sealed (an append is refused).

## 2. THE TWO COLD READS

Both were seated with `CLAUDE_CODE_DISABLE_AUTO_MEMORY=1`, attested first, disclosed their reading order, and changed nothing. Each found defects in changes the Director had already reviewed.

| read | bytes | sha256 |
|---|---:|---|
| `FINDINGS_SR15_NA0777_hook_read_20260903T045538Z.md` | 38937 | `9d44a872280bf50266e67f1e7ceb9d2c548cc7128270e2f39bc2dc403ab51aeb` | ← `F-1`…`F-13`: a loosening, a regression in the Director's own ordered fix, a silent fail-open, a corrosive false-positive dialect
| `FINDINGS_SR15_NA0777_S5_read_20260903T082714Z.md` | 62958 | `10b121551e0ec943a4e014c39cbe8ec609b3e1ffec8190ab5f24ee975f7f54ca` | ← `S5-1`…`S5-23`: withdrew two of the three drafts it read; `R72`'s attestation half discharged here

## 3. THE SHIPPED GUARDRAILS HOOK

`ENG-0286`. Shipped as `qsl-ops` commit `cafeab3429e8879cf32a27c0aa7664d610b124c2`. **THREE-WAY IDENTITY** was measured, not a push receipt: the bytes on disk, the bytes banked at 444, and the bytes read back off `origin/main` all hash the same.

| artifact | bytes | sha256 |
|---|---:|---|
| `S5c_qsl_guardrails_hook.na0777.FINAL.sh` | 12527 | `2fb2c286f4d9c75fb631aa583930daf11d4fb6fb3b362f947577e769c91c22fc` | ← the shipped file
| `S5c_hook_arms.na0777.FINAL.py` | 11251 | `70f044f92fc20ad859b5a0cc72a0eb2d9b3f9f412bdb6934f7852e2ff1bfca84` | ← the 89-arm harness, each arm with an expected message substring
| `S5c_reader_repros.na0777.sh` | 3056 | `885cd185c2ff94708c9c02076023dc3eff37b852d771d756f7dc99008d9bcf4c` | ← the reader's own repros, run verbatim
| `S5c_R105.na0777.diff` | 3582 | `ec14be40f5b9edeb10a396f911e44657ba32f8c891f5b55eb98a499adc381a19` | ← R105 (A) and (B)
| `S5c_rev2.na0777.diff` | 16346 | `ff55cb2fbe026113ffa156c7caea4d2629598bb1159f11b58cc9bc523db7c2ea` | ← the revision the Director read

**89 arms, 89 HIT.** And the harness was proven able to go **RED** before its green was believed: the pre-revision candidate scores **61/89** on the same arms and misses exactly the arms the two findings files predicted, no more and no fewer. A matrix that passed on both candidates would prove the arms blind.

**THE LIVE ARM ONLY THE WIRED FILE CAN ANSWER**, from a seat's own Bash after the install: a read-only unit query is ALLOWED and returns real data, where it had been DENIED for the whole lane; `sudo` is still DENIED with the new file's message. A hook allowing both would be unwired; one denying both would be the old file.

## 4. S4 — THE DURABLE-STATE COPY, AND THE CENSUSES SEALED BEFORE IT RAN

Two censuses of the source were sealed at 444 **before** the copy, so the source's state is fixed in the record independently of the copy that followed.

| census | bytes | sha256 |
|---|---:|---|
| `S4_census_sha_source_20260903.txt` | 471941 | `40e73c75c1866d7742601e1149544f5cde54107382665e99677732334c55bfb6` |
| `S4_census_mode_source_20260903.txt` | 301642 | `c25136a7cd1f608f0fc54293355733b6e6ab4fee337de7d61fef8fb5ba35c4ed` |

Then `cp -a`, then **three EMPTY diffs**: `diff -r` (0 lines), a sha256 census of every file (4147 lines each side, 0 differences), and a type/mode/owner census (4342 lines each side, 0 differences). A `du -sh` gap of 115M vs 114M was chased and ruled out — apparent size is 109394095 bytes on both sides. The one deliberate non-identity is the destination root's own mode, named in the stop.

The remaining §10.1 rows were crossed with the same instrument, **each census driven by its own source's entry list** so a later row cannot stale an earlier row's result: `evidence/` 144 files, `audits/` 3, `qwebsite/evidence/` 482, `qwebsite/logs/` 147, `artifacts/` 22 sha-named binaries, `root.crt` 1 — all EMPTY on both instruments, sources untouched and re-measured after.

## 5. THE BACKUP — R68's PATCH AND THE FIRST UNATTENDED RUN

| artifact | bytes | sha256 |
|---|---:|---|
| `qsl-backup.na0777.R68.candidate` | 11926 | `b048224af0a4f8038ddd8faed6558aef26f0adaa71e81dbd638f4af9f2384ca8` |
| `qsl-backup.na0777.R68.patch` | 284 | `031b808d42197e62b0fdded7ee8ddce9a6300dbce79e7526dee9442b10ea9f73` |

Two added lines, zero removed, one hunk; round-trip proven (`patch` rc=0, `cmp` rc=0). Pre-patch `cc325cb3fa48ddf25b582abf5e219009c385296cf69a70a1dcbc62f21786a612`; the post-patch sha was **sealed before the operator had the file** and the installed file matched it byte for byte. `WF-0101` carries the five sealed expectations, all HIT on the first unattended run.

## 6. THE WITHDRAWN DRAFTS — KEPT SO THE READER'S CITATIONS RESOLVE

| draft | bytes | sha256 |
|---|---:|---|
| `S5a_maintenance_unit.na0777.diff` | 5786 | `2cf72b57691839da5d5b21f7a640c9222b1c938f7e0a8e28fa352b52b28d1a84` | ← WITHDRAWN (R97)
| `S5b_qbuild-ssd-maintenance.na0777.candidate.sh` | 24728 | `07e599f45b69e7d242ae21a0bda889caf4bbaca961cbd4f526fee2945ba7e2b4` | ← WITHDRAWN (R98)
| `S5b_qbuild-ssd-maintenance.na0777.patch` | 1011 | `50e716b48d759a6819c2e76a483bdb65c3c3509dafc35599cc198befbde95775` | ← WITHDRAWN (R98)
| `S5_WITHDRAWN_drafts_a_and_b.md` | 3845 | `c342e2cfac3301887dd19a870f5edd44de4af2fa1cc398c8a3bafa191d174b90` | ← the marker

Withdrawn by finding number — `S5-1` (the job's subjects cannot exist in the roots rule 2 empties, so the retarget is a permanent no-op) and `S5-13` (an unstated loosening: a root-run `rm -rf` given write access inside `$HOME`) — and **not** on the gate defects `S5-3`/`S5-8`/`S5-9`/`S5-15`. Nothing was installed; the deployed script and unit remain byte-identical to their sources, re-measured after the withdrawal.

## 7. THE STOPS

Thirteen numbered stops in 21 files; eight amended. Each 444 and sealed on two arms that differ (a 644 control accepts an append, the banked 444 refuses it, `cmp` rc=0 after).

| stop | bytes | sha256 |
|---|---:|---|
| `STOP_NA0777_001_20260903T004710Z.md` | 67929 | `8dc99feda3499b6adcd37fad553e03ca70ae7d5fa6c62abef57b49a11098d0a1` |
| `STOP_NA0777_001_AMENDED_20260903T004848Z.md` | 69174 | `364d9942d64d1ca4d86298f77c1a97a2a16c7966899b3d3689babffdcad0711a` |
| `STOP_NA0777_001_AMENDED_2_20260903T014004Z.md` | 86388 | `8416959043769bf70beaab672a9ffaa3edb43a3b647ea64fcb562e6fb143a5e6` |
| `STOP_NA0777_001_AMENDED_3_20260903T014114Z.md` | 87110 | `fff05949e63193bb1878d475fc3ea59d8c3d811352133446f3fa3306d019f0ed` |
| `STOP_NA0777_002_20260903T030137Z.md` | 32015 | `7ba4c6b68c58aee8eb4ccc0d51428f904d02fc1167b4ce3e54ae6c2bc8b5c1a6` |
| `STOP_NA0777_003_20260903T033707Z.md` | 28064 | `19695ab855a37ebc7ad8d6b46c43c5998b5b806b3aa632924fea06a5345fa1eb` |
| `STOP_NA0777_003_AMENDED_20260903T033755Z.md` | 28088 | `d67176bbeba2dca0c40461f43ed8a9b9e123c5d1b497551ed51b352740b30d9f` |
| `STOP_NA0777_003_AMENDED_2_20260903T033848Z.md` | 29789 | `bf19383c5523c7949617647804ea1d6d550f3faebd315a407e96e5a2947a128a` |
| `STOP_NA0777_004_20260903T035400Z.md` | 22792 | `7fe34481f4cd2b61fbe2ace2b4708698c98a28ef9280a0820ff79d70a33c3de8` |
| `STOP_NA0777_005_20260903T041812Z.md` | 37793 | `ee525b540a7f5bfea67e741e55caef80766375e0205e724918967673dd721c80` |
| `STOP_NA0777_006_20260903T050752Z.md` | 28248 | `0830c4ed7d78d00953ae0d2fc5ad1fdf3b84e411cc1ad4f1d6943889ee38c92d` |
| `STOP_NA0777_007_20260903T053503Z.md` | 28055 | `449d4b88208ccecceefb52612304c4fba27cf49c8479cde309f8774b27693c55` |
| `STOP_NA0777_008_20260903T060631Z.md` | 21397 | `2d5f0dc9b4d6e23b60b938fa10910dca6bb6c46b07404772b2a9df00f754c3ba` |
| `STOP_NA0777_008_AMENDED_20260903T060822Z.md` | 23095 | `ec6ea6e1a352a957ef633f1408ce6ceb4da81521d41ce8cfc78edf11c5ed9304` |
| `STOP_NA0777_009_20260903T064108Z.md` | 22787 | `36c8f5395b9a7e0a1fa66dd1f8d80085f50e4356567ad9f7fb186c533b8697ed` |
| `STOP_NA0777_010_20260903T070549Z.md` | 14866 | `348479f745df6c3824462298e40f91ba3e6ab2e103a81e1c033aafe01a09366c` |
| `STOP_NA0777_010_AMENDED_20260903T070751Z.md` | 17382 | `398d7583eb0878b936d123fe2674dde253101c71a6a8d8baea81d3b66275cd8d` |
| `STOP_NA0777_011_20260903T075539Z.md` | 20402 | `4c462b50048af1c36cb6ee852585b0fed506819c0ad168068a6c87e2efa7df9e` |
| `STOP_NA0777_011_AMENDED_20260903T075732Z.md` | 22461 | `9ca1f7c4b7a74d544c6f4191d3363194a3a6661b7dd49fae1136b8e4a9513ce9` |
| `STOP_NA0777_012_20260903T090657Z.md` | 41505 | `8bba6fd52621da43bc04a3e183324496a9194bccc3ec821519750a462a04aec0` |
| `STOP_NA0777_013_20260903T151107Z.md` | 25812 | `69fdbdc23416ebe730a9a50da4d6b4e3dccee7b04c85835acb064ba11818d010` |

## 8. THE APPARATUS COMMITS

Nine commits on the private ops remote, oldest first:

```
d3112085e884c29ba26a7cf9d9fbda585b1cdc14  NA-0777 S3: adapt the apparatus to the new tree, and write the companion settings file
c758468cb16b48bf5f1775a1347e8a06e007d2f8  NA-0777: commit the rule-4 --reference clone, which STOP 005 left in the worktree
c4aec697d8392bed00a1fe4a02d4e30fe919f1a5  NA-0777: correct settings.snippet.json to the single Edit deny rule, and stop asserting why
0a5cc450c2af172fd245ab02f57757493f0639f2  NA-0777 R53: the lane-opening script stops carrying its own grammar and repo list
e482ec589043c45542797a92f540df0bfba48601  NA-0777 R54: the new tree gets its entry point, and it stops pointing at the old tree
66e7c421233d6c78ae7ca9d8e601103ca451a445  NA-0777 R63/R72: the apparatus stops naming the old tree, and the tree gets its own CLAUDE.md
cc18954c6a44f45de4d7bae1316f490eb8ba1e79  NA-0777 R63: drop the last old-tree literal, which was in the file I had just written
439b6b706f346532895abea370a450c21cf04565  NA-0777 R80: the lane-advance function moves into the versioned shell file
cafeab3429e8879cf32a27c0aa7664d610b124c2  NA-0777 R108: ship the guardrails hook, after two cold reads and 89 arms
```

**No product code changed by this lane.** These are the whole of its code footprint, and none of it is in the spine.

## 9. WHAT IS NOT CLAIMED

- The `.github/**` deny rule is **partial and non-deterministic** on this build, measured over fourteen arms. It is recorded as an aid, not a boundary, and is not claimed as protection.
- The hook's non-coverage is enumerated in its own header and in `ENG-0286`; it is not repaired.
- The nightly hygiene job is **unchanged** and still serves the old tree. Its redesign is the successor lane's (R100).
- The 19-checkout triage, the platter reclaim and the decommission itself are steps 7–8 and belong to the successor lane, on or after 2026-09-16 **and** only after the Invitations lane has opened, run and closed in the new tree.
- The old root is frozen read-only. **Nothing under it or under the backup target was deleted by this lane.**
