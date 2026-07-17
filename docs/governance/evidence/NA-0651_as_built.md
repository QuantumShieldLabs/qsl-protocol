# NA-0651 as-built — qsc-desktop sidecar prototype retirement (D587, D-1274, QSC_DESKTOP_RETIRE_PASS)

Lane: NA-0651 per QSL-DIR-2026-07-16-587 (D587, APPROVED 2026-07-16 with both flags
resolved to the draft defaults; sha256 `df4756a2…`), seated by promotion PR #1583.
Base: main `bfa84b12` (the seating merge). Executes DOC-PROG-004 step 1 — the
operator's 2026-07-16 fresh-start decision (locked decisions L1/L3). Deletion+docs
LITE-CEREMONY: single PR + single decision (D-1274). Raw command transcripts live in
the proof root (`/srv/qbuild/tmp/NA0651_retire_qsc_desktop_sidecar_20260717T040112Z/`);
this file publishes the class-level results.

## §1 Startup and CONFIRM-LIVE

qwork proof (`.qwork/startup.qsl-protocol.kv`, written 2026-07-17T03:58:23Z, operator-run):
startup_result=OK; lane=NA-0651; repo=qsl-protocol; head==origin_main==main==
`bfa84b1263521ae886c9e83dc8dc9c809016c879`; worktree/index/untracked clean;
ready_count=1; queue_top_ready=NA-0651; requested_lane_status=READY;
shared_target_ready=yes (cargo_target_mode=shared, rustc-1.95.0).

CONFIRM-LIVE at Phase 0: D-1274 canonical count 0 (D-1273 ×1 — the lane begins at
D-1274); STATE `READY=NA-0651 | HIGHEST_NA=0651 | HIGHEST_D=1273`; anchored
`^Status: READY` ×1 (this lane). Disk 49% (<95% gate); /backup/qsl mounted.
Main-health at `bfa84b12`: 7/9 push runs completed success; formal-ci and
qsc-adversarial in progress (the two known long runs; monitored — see §8).

## §2 The three deletion preconditions (re-verified live at Phase 0, pre-deletion)

- (a) `qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml` line 7 declares an EMPTY
  `[workspace]` — the tree self-isolates from the root workspace. VERIFIED.
- (b) The root `Cargo.toml` members list (refimpl, refimpl_actor_rs, apps/qshield-cli,
  apps/qsl-tui, qsl/qsl-client/qsc) EXCLUDES the tree. VERIFIED.
- (c) Zero references to `qsc-desktop` anywhere in `.github/`. VERIFIED (grep count 0).

F1 harness zero-invoker proof: `grep -rl desktop_sidecar_stress` over `.github/`,
`scripts/`, and Makefiles returns ONLY the script itself — nothing invokes it. Its
five commands are all `cargo test --manifest-path qsl/qsl-client/qsc-desktop/
src-tauri/Cargo.toml …` — it existed solely to exercise the deleted tree
(broken-if-ever-run post-deletion). sha256 of the deleted script recorded in the
proof root (`deletion_census_f1.txt`).

## §3 The deletion census (exactly 23 = 22 tree + 1 harness)

`git rm -r qsl/qsl-client/qsc-desktop/` staged exactly these 22 tracked files
(matching the draft-time census byte-for-byte):

```
qsl/qsl-client/qsc-desktop/.gitignore
qsl/qsl-client/qsc-desktop/README.md
qsl/qsl-client/qsc-desktop/index.html
qsl/qsl-client/qsc-desktop/package-lock.json
qsl/qsl-client/qsc-desktop/package.json
qsl/qsl-client/qsc-desktop/scripts/prepare-sidecar.mjs
qsl/qsl-client/qsc-desktop/src-tauri/Cargo.lock
qsl/qsl-client/qsc-desktop/src-tauri/Cargo.toml
qsl/qsl-client/qsc-desktop/src-tauri/build.rs
qsl/qsl-client/qsc-desktop/src-tauri/capabilities/default.json
qsl/qsl-client/qsc-desktop/src-tauri/icons/128x128.png
qsl/qsl-client/qsc-desktop/src-tauri/icons/128x128@2x.png
qsl/qsl-client/qsc-desktop/src-tauri/icons/32x32.png
qsl/qsl-client/qsc-desktop/src-tauri/icons/icon.png
qsl/qsl-client/qsc-desktop/src-tauri/resources/bin/qsc.stub
qsl/qsl-client/qsc-desktop/src-tauri/src/main.rs
qsl/qsl-client/qsc-desktop/src-tauri/src/model.rs
qsl/qsl-client/qsc-desktop/src-tauri/src/qsc.rs
qsl/qsl-client/qsc-desktop/src-tauri/tauri.conf.json
qsl/qsl-client/qsc-desktop/src/main.js
qsl/qsl-client/qsc-desktop/src/style.css
qsl/qsl-client/qsc-desktop/vite.config.js
```

Plus `scripts/ci/desktop_sidecar_stress_na0264.sh` (F1). The tree's OWN
Cargo.toml/Cargo.lock leave with the tree — a lockfile cargo-audit never audited
(the 2026-07-16 investigation's noted gap), so a strict audit-surface reduction.
The ROOT Cargo.toml/Cargo.lock are byte-identical (§7, acceptance D). History
retained: the tree remains fully recoverable from git history.

## §4 The supersession banners (identical ×3)

DOC-QSC-008/009/010 each carry the IDENTICAL banner blockquote immediately after
the H1 (the NA-0645 TUI-retirement house form; verbatim text from D587 with
NA-0651/D-1274 filled), plus a Last-Updated bump
(`2026-07-16 (NA-0651: superseded — sidecar prototype retired)`). Nothing else in
those documents changed. Identity proof: the extracted banner block hashes
`0344b46d96afb6015886506e1d88a7da0946188f877f390d7c1a8b7d1219ca36` in all three
documents.

## §5 The F2 reviewer-package repair and the claim-boundary byte-proof

`docs/public/EXTERNAL_REVIEW_PACKAGE.md`, EXACTLY two references (each old string
asserted unique before replacement):

1. The "Desktop GUI guided demo readiness" evidence row: the live markdown link
   labeled "qsc desktop README", targeting `../../qsl/qsl-client/qsc-desktop/README.md`
   (link syntax not reproduced here — a deleted target must not appear in link form) →
   `the retired prototype's README (tree removed at NA-0651, D-1274, 2026-07-16 —
   DOC-PROG-004 step 1; retained in git history), and`. The row's other citations
   (NA-0247/NA-0258 evidence, DOC-QSC-010) and its claim-bounding column are
   untouched.
2. The references-list bullet: the same live link form (label "qsc desktop
   prototype README", same target) →
   `- qsc desktop prototype README — retired at NA-0651 (D-1274, 2026-07-16); see
   git history and DOC-QSC-009/010 (superseded, retained as history)`.

Zero live links to the deleted tree remain anywhere in the package.

THE BYTE-PROOF (extraction rule pinned: from the `## <section>` header line through
the end of the section, inclusive of the trailing newline; "Safe Public Wording" is
the final section and runs to EOF):

| Region | Before | After |
|---|---|---|
| What Is Not Proven | 1,176 B `f8ede7a186d748d3c34d6522f2726e33ad225ee02a847b0b94f64c051c7abae1` | IDENTICAL |
| Safe Public Wording | 995 B `56fbab6f4fb1374ebe4f8e3eaf6a2d4c115d79f991f74ffab2de38880599aedb` | IDENTICAL |

Note: "What Is Not Proven" matches the NA-0648 record byte-for-byte (1,176 B
`f8ede7a1…`). "Safe Public Wording" measures 995 B here vs the NA-0648 record's
994 B — an extraction-convention difference (this rule includes the file's final
newline), not a content change; the pre/post identity under one pinned rule is the
load-bearing fact. The claim boundary did not move.

## §6 Acceptance C — the residual-reference proof (case-sensitive `git grep -n "qsc-desktop"` at PR head)

The full per-file mapping is `residual_mapping_head.txt` in the proof root; the
class-level census at PR head (file counts are the stable fact; the grep pattern is
the literal hyphenated tree name, which the underscore KEEP anchor can never match):

| Manifest class | Files | Hits |
|---|---|---|
| 1 — queue/decision/traceability records (NEXT_ACTIONS, DECISIONS, TRACEABILITY) | 3 | 636 |
| 2 — governance records (docs/governance/** incl. this file; the journal) | 102 | 256 |
| 3 — archived/historical lane testplans (docs/archive/testplans/**; tests/NA-*_testplan.md) | 347 | 506 |
| 4 — frozen inputs/ fixtures | 3 | 3 |
| 5 — the bannered DOC-QSC-008/009/010 | 3 | 18 |
| 6 — program-doc history (DOC-PROG-004 step 1; DOC-PROG-001 past-tense wave prose) | 2 | 2 |
| 7 — historical planning/audit/demo records (DOC-QSC-011; DOC-AUD-001/002; five docs/demo; two docs/public scope-lists) | 10 | 30 |
| 8 — the public-safety gate's own classifier fixture (§9-protected, untouched) | 1 | 1 |
| 9 — EXTERNAL_REVIEW_PACKAGE.md | 0 | 0 (the F2 history-prose uses the unhyphenated name — the package carries ZERO matches) |
| 10 — the F1 script | ABSENT | deleted, as required |
| **Total** | **471** | **1,452** |

ZERO hits outside the manifest. The deleted tree itself: zero matches (gone).

## §7 Acceptance A/B/D

- **A (workspace unaffected):** `cargo check --all-targets` at head: **0 errors /
  0 warnings in 0.29 s** — nothing recompiled; the workspace build graph never
  contained the tree. (At base: 0/0 in 15.62 s.)
- **B (suite counts unchanged):** full local `cargo test -p qsc`:
  - At BASE `bfa84b12` (Phase 0, pre-deletion): **412 passed / 0 failed /
    1 pre-existing-ignored across all 108 result sets, exit 0** — exactly the
    NA-0649/D-1272 record; the NA-0640 e2e
    (`two_client_local_relay_message_and_file_flow_is_honest`) ok within the run.
  - At HEAD (post-deletion): **412 passed / 0 failed / 1 pre-existing-ignored
    across all 108 result sets, exit 0** — BYTE-IDENTICAL totals and set count.
  - The `desktop_gui_contract_na0215b` tests pass in both runs — the KEPT env-ingress
    is exercised as the suite's unlock vehicle and works.
- **D (untouched surfaces):** `git diff --cached --stat` over qsl/qsl-client/qsc/,
  .github/, formal/, vectors/ = ZERO changes in each; root Cargo.toml and Cargo.lock
  ABSENT from the changed-path list; `cargo metadata --locked` OK; the KEEP-anchor
  file count unchanged at 16 (Phase-1 enumeration == post-change enumeration).

## §8 Validation defaults and gates

- `git diff --cached --check`: clean. Scope guard: every changed path ⊆ the D587
  allowed list (11 non-tree paths + the 22 tree deletions; list in the proof root).
- Link check over all 6 touched docs: 0 broken. TWO self-inflicted findings fixed
  in-flight, same root cause: the as-built's §5 quotes of the OLD reference strings
  parsed as live markdown links to the deleted target — first caught by the local
  offline checker (fixed to inline code spans), then by the public-safety gate's
  stricter first PR run (its parser strips fenced blocks but not inline spans;
  TOTAL_MISSING=2 with DENY_HITS_FILES=0 and HC_COUNT=0 — a link-integrity finding,
  not a content violation). Final form DESCRIBES the old links without reproducing
  link syntax; the gate re-run is recorded in the lane response. No gate amendment.
- Publication scan (house tool, staged added lines + new files): overclaim hits 0;
  secret/ssh/token patterns 0; `long_hex` = 4 — the published proof hashes (the
  banner block sha256, the two boundary-region sha256s, the full head SHA), the
  class the house publishes by design.
- goal-lint local (synthesized event payload): OK.
- Root `cargo audit`: 386 dependencies, 0 advisories, exit 0. Nested fuzz
  `cargo audit`: 287 dependencies, 0 advisories, exit 0.
- `cargo metadata --locked`: OK. `sh -n`/`bash -n` on scripts/ci/qsc_adversarial.sh: OK.
- `cargo fmt --check`: 145 diffs — ALL PRE-EXISTING (identical count on a clean
  `bfa84b12` base checkout; this lane touches zero Rust files, so the lane delta is
  zero). Recorded as a pre-existing observation for Director triage, not fixed
  in-lane and not a ledger edit (the D587 boundary).
- Main-push at `bfa84b12` and this PR's checks: recorded at the PR boundary (the
  final response carries the live check states).

## §9 Boundaries held / NOT claimed

KEEP boundary (anchor-defined): ZERO changes under `qsl/qsl-client/qsc/**` — the
`QSC_DESKTOP_SESSION_PASSPHRASE` env-ingress machinery (the hidden
`Cli.unlock_passphrase_env` flag, src/cmd/mod.rs; the `bootstrap_unlock` env branch,
src/main.rs; `DESKTOP_PASS_ENV_KEY` + the one-name allowlist +
`unlock_with_passphrase_env` + `VaultUnlockArgs.passphrase_env`, src/vault/mod.rs)
and ALL 16 anchor-carrying files (1 src + 15 test-side incl. tests/common/mod.rs,
live-enumerated at Phase 1 — the D587 draft parenthetical "15 test files +
tests/common/mod.rs" counted the same set with common/mod.rs listed separately; the
anchor-defined boundary is unaffected) are byte-untouched. Env-ingress retirement
remains the separately-tracked future decision (DOC-PROG-004 parallel-tracks row).
The public-safety gate and its lists untouched (its `qsc_desktop_path` classifier
fixture row at scripts/ci/public_safety_gate.py ~:2218 is §9-protected — a synthetic
fixture naming `qsc-desktop/src/main.ts`, a file that never existed in the tree).
The IMPROVEMENT_LEDGER untouched (the one DECISIONS entry D-1274 is the lane's only
ledger-adjacent motion). NO dep change; NO GUI code; NO qsl-server or website-repo
change.

NOT claimed: any GUI exists; the satellite repo is created; the env-ingress is
retired; the external-review gate moved (it remains THE release gate). Claim
boundary UNCHANGED, byte-proven (§5).
