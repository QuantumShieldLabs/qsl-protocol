# NA-0648 testplan — Reviewer-facing docs refresh (D584, D-1271)

Docs-only lane + the local re-run of the package's Reproducible Commands (read-only
of product). The "tests" here are the lane's verification steps and their observed
results; there is no code under test.

## T1 — The reproducible-command re-run (DOC 3, run FIRST per D584 Phase 1)

- Step: at `77576681`, run every command in the package's Reproducible Commands
  table exactly as written; capture outputs; re-stamp each "Local result" cell
  only from the observed output.
- Expected: outputs honestly recorded, pass or fail; any failure or surprise
  flagged, never fixed in-lane.
- Observed: **PASS for every executed command** (audit; tree; send_commit 3/3;
  model runner exit 0 — six models incl. handshake-auth 10,800+10,800/0 unbound;
  demo smoke `DEMO_ACCEPTANCE_OK`; metadata smoke; NA-0291 harness; NA-0293
  harness; ProVerif gate — see the as-built §2 row for the stamped result;
  refimpl 89 lib tests incl. the NA-0628 scan). The clean-host row was NOT
  re-run (dated historical proof pinned at `1e7d0a63be31`; the cell now says so).
  One benign directive-vs-reality delta recorded (six models, not five). No
  product problem surfaced; nothing new filed.

## T2 — Claim-boundary byte-preservation (the load-bearing discipline)

- Step: extract the package's `## What Is Not Proven` and `## Safe Public Wording`
  regions and the map's `## Claim Boundary Map` and `## Do Not Claim Yet` regions
  from HEAD and from the edited working tree; byte-count and sha256 both sides.
- Observed: **PASS.** All four regions byte-identical (1,176 / 994 / 1,717 / 715
  bytes; sha256 `f8ede7a1…` / `a8242ee0…` / `a17ee1d8…` / `db21db61…` identical
  both sides — full digests in the as-built §5). The boundary did not move.

## T3 — ENG-0038 wording reuse

- Step: every ENG-0038 fact stated in the package/map must reuse the
  operator-approved 2026-07-15 progress-entry sentences verbatim-or-near
  (correction record not flaw-free evidence; internal trace-analysis not a PoC;
  the pinned-identity-KEM fix; the class retirement; the bounded model with the
  FIVE named unmodeled slices and non-vacuity; the ENG-0001/NA-0609B calibration
  point; external review uncommissioned and a release gate).
- Observed: **PASS.** The Executive Summary arc paragraph, the current-handshake
  paragraph, the two Proven rows, the map's G1/G4 citations, the formal-map row,
  and the consolidated section all derive from the progress entry and
  `formal/README.md` §2/§5; no fresh claim prose where vetted prose exists; no
  "vulnerability-free / audited / formally-verified-unqualified" wording added
  anywhere (the only occurrences of such phrases are inside the byte-preserved
  boundary/negative lists and in explicit negations).

## T4 — Relative-markdown-link integrity + gate pre-check of the changed docs

- Step: run the public-safety gate's own denylist, HIGH_CONF credential-pattern,
  and markdown-relative-link logic over both changed docs (off-tree driver
  importing `scripts/ci/public_safety_gate.py`, working-tree resolution).
- Observed: **PASS.** `DENY_HITS_FILES=0`, `HC_COUNT=0`,
  `MARKDOWN_FILES_CHANGED=2`, `TOTAL_MISSING=0` — every newly added relative
  link resolves. The authoritative gate runs at the PR; the gate itself was not
  amended.

## T5 — Scope guard

- Step: `git status` / diff file list vs the D584 allowed-paths list.
- Observed: **PASS.** Changed files ⊆ {the two public docs, the NA-0648
  as-built, this testplan, `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
  the operations journal}. Zero code/script/vector/workflow/formal/website
  changes; the demo and model scripts untouched (run only); the map's line-240
  relocation is byte-identical content movement.

## T6 — Structural-fix verification (map line 240)

- Step: confirm the "Metadata / Privacy Readiness Map" heading is now
  immediately followed by its own table; confirm the relocated "What Changed
  After NA-0541" section sits after the NA-0539 section, byte-identical.
- Observed: **PASS.** Heading → table adjacency restored; the relocated section's
  text is byte-identical to the pre-lane text (pure move).
