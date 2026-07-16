# NA-0648 as-built — Reviewer-facing docs refresh (D584, D-1271)

Lane: NA-0648 per QSL-DIR-2026-07-16-584 (D584, APPROVED), seated by promotion PR #1577.
Base: qsl-protocol `main` `77576681` (the #1577 seating merge; within D584's "f71f3739 or newer").
Docs-only + the local re-run of the package's Reproducible Commands (read-only of product).
No code, scripts, vectors, workflows, or formal models changed.

## §1 Phase 0 (CONFIRM-LIVE)

- Checkout `/srv/qbuild/work/NA-0648/qsl-protocol` at `77576681` == `origin/main`, clean.
- Queue: exactly one anchored `^Status: READY` (NA-0648); STATE
  `READY=NA-0648 | HIGHEST_NA=0648 | HIGHEST_D=1270`.
- Highest DECISIONS entry: D-1270 → this lane begins at **D-1271**.
- ENG-0045 present on the ledger (filed by NA-0647, unchanged by this lane).
- Link targets confirmed current: `formal/README.md` (the §2 fifth-model
  handshake-authentication section and §5 limits present),
  `docs/public/progress/2026-07-15.md`, `docs/public/INDEX.md`, and the
  NA-0632/0633/0634/0636/0640/0642/0644/0645/0646 evidence files.

## §2 DOC 3 — the reproducible-command re-run (run FIRST, per D584 Phase 1)

All commands run from the repository root at `77576681` on 2026-07-16, exactly as
written in the package table. Raw logs preserved off-tree (session scratchpad
`na0648_reruns/`); the observed lines below are what the re-stamped cells cite.

| Command (package row) | Result | Observed basis for the re-stamped cell |
| --- | --- | --- |
| `cargo audit --deny warnings` | PASS, exit 0 | 386 crate dependencies scanned; zero advisories reported. |
| `cargo tree -i rustls-webpki --locked` | PASS, exit 0 | `rustls-webpki v0.103.13` via `rustls v0.23.36` — unchanged from the recorded cell. |
| `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1` | PASS, exit 0 | `3 passed; 0 failed` (24.20s test wall-clock). |
| `python3 formal/run_model_checks.py` | PASS, exit 0, 1.8 s | **Six** models, all `OK`: SCKA (926 explored states), Suite-2 negotiation (428 no-mutation assertions), qsc handshake suite-id, qsc KEM/signature/transcript binding, Suite-2 root composition (15,032 states / 9 regression shapes), and `QSC.HS.*` handshake authentication (10,800 responder + 10,800 initiator configurations, 0 unbound commits, faithfulness anchor 54 pre-fix traces reproduced, non-vacuity 128 counterfactual unbound commits detected). |
| `scripts/ci/demo_cli_smoke.sh` | PASS, exit 0 | `DEMO_ACCEPTANCE_OK` and `demo-cli-smoke: OK` both emitted. |
| Clean-source command set (CLEAN_HOST_REVIEWER_REPRODUCTION.md) | NOT RE-RUN | Dated historical proof pinned at commit `1e7d0a63be31` (2026-05-11); a multi-hour fresh-workdir reproduction, outside this refresh's re-run set. The cell now states the pin and that it was not re-run — no staleness is hidden. |
| `scripts/ci/metadata_conformance_smoke.sh` | PASS, exit 0 | `metadata-conformance-smoke: OK`. |
| `scripts/ci/metadata_phase2_identifier_padding_harness.sh` | PASS, exit 0 | `NA0291_METADATA_PHASE2_HARNESS_OK`. |
| NA-0292 design row | N/A | DOCS_ONLY row; nothing to run. |
| `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh` | PASS, exit 0 | `NA0293_METADATA_PHASE2_SANITIZED_RETENTION_OK`. |
| `python3 formal/proverif/run_proverif_checks.py` | PASS, exit 0, 28m11s wall-clock | Tool sanity pair first (proves AND refutes), then all four model files green: 17 assertions total (2 sanity + 15 model), `All expected ProVerif RESULT lines present (fail-closed gate green)`. Same assertion count as the recorded cell; the wall-clock note updated ~24 → ~28 min (re-run host). |
| `cargo test -p quantumshield_refimpl --locked` | PASS, exit 0 | 89 lib tests incl. `na0628_every_dh_call_site_is_guarded_or_allowlisted`, plus the integration targets; all result sets `ok`, 0 failed. |

**Surprises/deltas (recorded, not fixed):**

- **Directive-vs-reality delta (benign, favors coverage):** D584 describes
  `run_model_checks.py` as "now executes FIVE models"; the runner executes **six**
  (the qsc handshake **suite-id** model is the one the directive's count missed).
  The re-stamped cell records six. No product problem; nothing filed.
- No command failed and no product problem surfaced; ENG-0045 (the separate public
  demo script, out of this table) remains filed and untouched.

Freshness basis for the two main-run cells: `origin/main` `77576681` `public-ci`
run completed with jobs `public-safety` and `advisories` both `success`
(2026-07-16, checked via the GitHub API during this lane).

## §3 DOC 1 — EXTERNAL_REVIEW_PACKAGE.md (what changed)

Additions + framing + freshness; blind-by-OMISSION confirmed (the pre-edit doc
contained no handshake/QSC.HS/ENG-0038 text, so nothing was excised):

- (a) The ENG-0038 arc: one Executive Summary paragraph (found → fixed → class
  retired → bounded model, with the ENG-0001/NA-0609B contradiction as a
  calibration point and the five unmodeled slices named) + two What-Is-Currently-
  Proven rows (the found-and-fixed row; the bounded-model row). Wording reused
  verbatim-or-near from the operator-approved 2026-07-15 progress entry.
- (b) One bounded current-handshake paragraph ("The shipped handshake, as
  authenticated today") mirroring the progress-entry/formal-README wording;
  explicitly marked as the bounded-model description, not new spec prose.
- (c) Formal-spine update: "How to read this package" item 3 now routes to TWO
  artifacts (DOC-G4-002 + the formal/README handshake-auth model); the paragraph
  beneath it rewritten to state both bounded results; the line-157 model-runner
  cell re-stamped from the actual six-model re-run; the ProVerif cell re-stamped
  from the actual re-run; the Evidence Artifact Index gains the progress entry,
  the four ENG-0038 arc evidence files, and the five product-arc as-builts.
- (d) Product context: five new Proven rows (NA-0640 e2e dev-harness; NA-0642
  durable queue, repo evidence, ENG-0039 open; NA-0644 ack client, bounded
  ENG-0042 seam, not default; NA-0645 TUI retirement, ENG-0044 owed; NA-0646
  core extraction, architecture not product/SDK) + one compact Executive Summary
  pointer paragraph.
- (e) Freshness: Last-Updated → 2026-07-16; the line-92 main-run cell →
  `77576681` (2026-07-16); the Recent-PR-Evidence table extended #1545..#1576
  with the stated decision ("review-relevant merges since #1541; queue-promotion
  and closeout-only PRs omitted"); Progress routing → the 2026-07-15 entry; the
  Known-Gaps website row rewritten against WEB-0006/WCM-101..115 reality (the
  stale "prepare a handoff before editing the website" framing retired).
- (f) The one-sentence ENG-0045 accuracy note appended to the one-command-demo
  Proven row's boundary cell, mirroring the progress entry's publication-time
  note (recorded + flagged, not fixed; the in-repo smoke re-verified green).

Boundary sections untouched — see §5.

## §4 DOC 2 — RELEASE_READINESS_EVIDENCE_MAP.md (what changed)

- (a) Formal Verification Readiness Map: +2 rows — the ProVerif composition model
  (DOC-G4-002 §2 abstractions, ENG-0035 bound stated) and the `QSC.HS.*`
  handshake-authentication bounded model (10,800+10,800 configurations, five
  unmodeled slices named, "not cryptographic security / not an unqualified formal
  verification").
- (b) Citations: G1 and G4 evidence cells gain the ENG-0038 arc + NA-0640 e2e
  (G4 also the handshake-auth model) in the vetted bounded phrasing; the
  dependency-advisory and latest-main gate rows re-stamped at `77576681`
  (2026-07-16); the service-hardening gate row STAYS `NOT_READY` and gains the
  bounded NA-0642/NA-0644 repo-evidence citation with ENG-0039 stated open; the
  package-refreshed row updated to cite this NA-0648 refresh; the Local
  Reproduction Map purpose cell for the model runner updated to the six models;
  the Bounded-qsc public-safety row re-stamped at `77576681`.
- (c) Website rows: the gate row and the Demo/GUI/Website row no longer say
  "external website changes remain future work" (false since WEB-0005B/WEB-0006);
  both now record the executed WEB-0006 accuracy pass as live, cite the
  WCM-101..115 audit, and keep the consolidated content pass (and the OUTDATED
  WCM-110/WCM-112 rows) as the stated remaining work. Status NOT_READY →
  "PARTIAL for the executed accuracy pass" with an explicit no-public-readiness
  boundary — a website-implementation status corrected to fact, not a protocol
  claim change.
- (d) Structural fix (the line-240 mis-merge fossil): the "What Changed After
  NA-0541" section moved out from between the "Metadata / Privacy Readiness Map"
  header and that map's own table, to its chronological place after the NA-0539
  section. Content byte-identical; pure relocation.
- (e) Consolidation: ONE new "What Changed After NA-0629 Through NA-0647"
  section (ENG-0038 arc / product path / public docs, each with the vetted
  bounded phrasing and a closing boundary-unchanged bullet) instead of
  extending the per-lane chain. Last-Updated → 2026-07-16.

Boundary sections untouched — see §5.

## §5 The claim-boundary byte-proof (machine-proven)

Method: extract each boundary region from `HEAD` (pre-lane) and from the edited
working tree by exact heading anchors; byte-count and sha256 both sides
(NA-0631/NA-0647 precedent). All four regions are **byte-identical** — the
boundary did not move and was not even added to.

| Region (anchors) | Bytes | sha256 (identical both sides) |
| --- | --- | --- |
| Package `## What Is Not Proven` → before `## Reproducible Commands` | 1,176 | `f8ede7a186d748d3c34d6522f2726e33ad225ee02a847b0b94f64c051c7abae1` |
| Package `## Safe Public Wording` → EOF | 994 | `a8242ee01f028e85bd520f2d2757e81abd3404bca1329ce806dfb51f47c4133d` |
| Map `## Claim Boundary Map` → before `## Demo / GUI / Website Readiness Map` | 1,717 | `a17ee1d8a62a7c8f6052bd2e663e35b201a370efec240893d0ffd502c233fc0c` |
| Map `## Do Not Claim Yet` → EOF | 715 | `db21db610356168ee0338c071aa47c9eeb11377e3f3113ac7655d37c404de088` |

"Not vulnerability-free" remains present in the package's What-Is-Not-Proven list,
byte-unchanged.

## §6 Validation

- **Scope guard:** `git status` shows exactly the two docs (plus this lane's
  governance/closeout files); no code, script, vector, workflow, formal-model,
  or website-repo change; `qsc_demo_local.sh` and `run_model_checks.py` and
  `run_proverif_checks.py` untouched (run only).
- **Public-safety pre-check (local):** the gate's own denylist, HIGH_CONF
  credential pattern, and markdown-relative-link logic executed over both
  changed docs via an off-tree driver importing `scripts/ci/public_safety_gate.py`
  — `DENY_HITS_FILES=0`, `HC_COUNT=0`, `TOTAL_MISSING=0` (this also verifies
  every newly added relative link resolves). The authoritative gate runs at the
  PR; **no gate amendment made or needed**.
- **Goal-lint:** run locally against the lane PR (synthesized event payload) —
  see the lane response file for the recorded result.
- **Claim-boundary byte-proof:** §5, all four regions byte-identical.
