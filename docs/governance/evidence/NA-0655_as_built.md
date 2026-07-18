# NA-0655 as built — satellite community-health docs (D591, D-1278)

Result class: SATELLITE_COMMUNITY_DOCS_PASS. Stopped at the four open PRs per
the directive; the operator merges (satellites first, any order, merge commits;
spine closeout last).

Directive: QSL-DIR-2026-07-18-591 (D591), APPROVED 2026-07-18 as amended
(sha256 `7b17303c…`, 334 lines) — F1 resolved: website repo INCLUDED; F2
resolved: re-based rationale accepted, the private-vulnerability-reporting
toggles are the operator's console companion step, outside the lane's
docs-only boundary.

## 1. Phase 0 census (all verified live, 2026-07-18)

- qwork proof: startup_result=OK; lane=NA-0655; repo=qsl-protocol;
  head==origin/main==`35bd5fa8` (the #1591 seating merge); worktree/index/
  untracked clean; ready_count=1; queue_top_ready=NA-0655;
  requested_lane_status=READY; shared_target_ready=yes.
- Spine: D-1278 next-and-absent (canonical count 0); anchored `Status: READY`
  ×1 (NA-0655); disk 52% (< 95%); /backup/qsl mounted. Main health on
  `35bd5fa8`: 8/9 SUCCESS including public-ci (the D589 exit condition holds);
  formal-ci in_progress inside its normal ~31–35 min window (queue-only diff).
- Satellites (FRESH clones per house rail, never the mirror; proof root
  `/srv/qbuild/tmp/NA0655_satellite_community_health_docs_*/clones/`):
  - qsl-server HEAD `3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944` — UNMOVED
  - qsl-attachments HEAD `a3ebad2fd19ae50b0f764fd44b7fc47fd5ca8723` — UNMOVED
  - website HEAD `31558fa4930ddf24a80824536acbfaa4a745d1cd` — UNMOVED
  The Phase-0 STOP did not trigger.
- File gap re-confirmed LIVE ×3: case-insensitive find (maxdepth 2, incl.
  `.github/`) for SECURITY.md / CODE_OF_CONDUCT.md / CONTRIBUTING.md → zero
  hits in every satellite.
- Satellite decision counters next-and-absent: canonical tops D-0012
  (qsl-server), D-0010 (qsl-attachments), D-0017 (website); D-0013 / D-0011 /
  D-0018 each 0 hits pre-lane.
- PVR state AS FOUND (API-verified): qsl-server ENABLED, qsl-attachments
  ENABLED, qsl-protocol ENABLED, qsl-desktop ENABLED — the operator's F2
  console step confirmed live (at drafting the two code satellites were
  disabled). Website endpoint: HTTP 404 = feature unavailable on the private
  repo, as expected; recorded as found.

## 2. The nine files (per satellite, branch `na0655-community-health-docs`)

Delta per satellite: EXACTLY four files — three NEW + one `DECISIONS.md`
append. README/LICENSE/NOTICE absent from every diff.

- `SECURITY.md` ×3: the spine's "Reporting a Vulnerability" section VERBATIM
  (extracted mechanically from the spine file at `35bd5fa8`; `cmp` proof of
  the first 7 lines ×3) + the D591-prescribed per-repo `## Scope` section:
  - qsl-server: "a research-stage, transport-only relay server for the QSL
    protocol project (see README.md)"; protocol-level reports → qsl-protocol.
  - qsl-attachments: "the research-stage attachment service/runtime for the
    QSL opaque encrypted attachment plane, as defined by qsl-protocol (see
    README.md)"; protocol-level reports → qsl-protocol.
  - website: "the source of the QuantumShield Labs website"; protocol- and
    implementation-level reports → qsl-protocol.
- `CODE_OF_CONDUCT.md` ×3: BYTE-IDENTICAL to the spine's — sha256
  `2cbf021e2c84858b76c8a83c5f73297be7e6034218b097abb79fa392542f01b3` for the
  spine source and all three copies.
- `CONTRIBUTING.md` ×3: spine-adapted; each states the satellite-of-
  qsl-protocol governance reality and the repo's REAL gate in its CI's exact
  order — qsl-server: the required `rust` check (`test_aws_update_and_verify.sh`,
  `test_update_checksum.sh`, `cargo fmt --all -- --check`, `cargo test -q`,
  `cargo clippy -q -- -D warnings`); qsl-attachments: the required `rust`
  check (`fmt --check`, `clippy --all-targets -- -D warnings`,
  `build --locked`, `test --locked`); website: `website-validation`
  (`npm ci --no-audit --no-fund`, `npm run scan:claims`,
  `test_technical_claims_integrity.sh`, `npm run build`), claim-governed
  content vs the spine's `docs/public/WEBSITE_CLAIM_MATRIX.md`,
  merge-to-main auto-deploys production, currently-private status mirrored
  from its README. No spine-only tool references anywhere.
- `DECISIONS.md` appends, each in its repo's own canonical form:
  qsl-server **D-0013** (Rationale form, Goals: G4); qsl-attachments
  **D-0011** (Invariants form, Goals: G4); website **D-0018** (Invariants
  form, no Goals field — matching that repo's precedent). Each records
  provenance D591/NA-0655/D-1278.

## 3. The four PRs

- qsl-server PR #63 — branch @ `d0b69a1c88b0fc07f56f1821a890b1f2d18ffaa1`
  off `3cc551a8`; required `rust` check: PASS.
- qsl-attachments PR #40 — branch @ `a99623efd7ef916bc39a07693b0b9056279ccfaa`
  off `a3ebad2f`; required `rust` check: PASS.
- website PR #32 — branch @ `3e2fe4993d2bdeda970d3e091023142e1fcdb8d6`
  off `31558fa4`; `website-validation`: PASS (observed; recorded as NOT a
  required check — branch protection unavailable on the private plan);
  Cloudflare Pages preview: pass.
- Spine closeout: THIS PR (branch `na0655-governance-closeout` off
  `35bd5fa8`) — D-1278, TRACEABILITY, journal, this evidence, the testplan,
  and the queue flip to `READY=NONE | HIGHEST_NA=0655 | HIGHEST_D=1278`.

## 4. Validation

- Assurance-language census over ALL NINE files: 0 hits
  (`\baudited\b|\bsecure\b|\bsecured\b|\bhardened\b|\bproduction-ready\b|\bguarantee`,
  case-insensitive, word-bounded).
- `git diff --check` clean ×3 (satellites) and on the spine closeout.
- Delta census: exactly the four files per satellite (`git diff --cached
  --name-only`); README/LICENSE/NOTICE byte-untouched (absent from every
  diff); zero workflow/code/test/dependency/lockfile hunks anywhere.
- Added-line private-material scan: 0 hits ×3 (token/secret/password/key
  patterns over every added line).
- Website guards run locally WITH the three files present, UNCHANGED:
  `scan-overclaims: files=40 / allowlisted=0 / PASS` (scan set unchanged —
  the new root files are outside its roots, exactly as the drafting analysis
  proved) and `NA-0006 technical claims integrity guardrail: PASS` (static
  tracked-file list untouched). Zero guard edits. The three files are outside
  the Vite build: zero rendered-content change.
- Spine validation defaults on this closeout PR: goal-lint local OK (literal
  `Goals: G4` body line from creation); `cargo metadata --locked` OK; root +
  nested-fuzz cargo audits green; `cargo fmt --check` = the known
  pre-existing residue only (zero lane Rust); results recorded in the
  response file.

## 5. Boundaries held / not claimed

- NO README/LICENSE/NOTICE edit in any repo; NO code, test, dependency,
  lockfile, or workflow change anywhere; NO repo-settings action in-lane
  (the PVR flips were the operator's console step, verified as found at
  Phase 0); NO qsl-desktop touch (empty by design — its set lands in the
  bootstrap lane); NO ledger edit (no ledger item existed; operator-sourced
  lane per D591 §8 triage record); org `.github` defaults untouched.
- NOT claimed: any security assurance about any repo's contents (the files
  describe REPORTING); website visibility (still private; PVR unavailable
  there); any rendered website content change. Claim boundary UNCHANGED.

See D-1278; tests/NA-0655_satellite_community_health_docs_testplan.md; the
response file `operator/responses/satellite_community_health_docs_<UTC>_D1278.md`.
