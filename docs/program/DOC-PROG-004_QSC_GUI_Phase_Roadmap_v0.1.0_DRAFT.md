Goals: G4, G5

Status: Supporting (product planning; subordinate to canonical specs, GOALS, the queue, and independent review)
Owner: QSL governance
Last-Updated: 2026-07-16 (NA-0650/D586: initial landing)
Authority: Non-normative product planning. Does NOT override START_HERE, GOALS,
the canonical specs, NEXT_ACTIONS, the IMPROVEMENT_LEDGER, or any recorded
decision. Where this document and any of those disagree, THEY win and this
document is corrected.

# DOC-PROG-004 — QSC GUI Phase Roadmap (v1) v0.1.0 DRAFT

> Provenance: Records the operator-approved QSC GUI Phase Roadmap of 2026-07-16,
> landed per QSL-DIR-2026-07-16-586 (D586, as amended at approval: A1 filename,
> A2 lane number) by lane NA-0650 (D-1273). "Landed by this lane" in the Status
> line below refers to NA-0650. The body below is the operator-approved text
> verbatim under exactly three landing adjustments — the title form, this
> wrapper, and the landing-base resolution — proven hunk-by-hunk in
> docs/governance/evidence/NA-0650_as_built.md.
>
> **Authorization boundary: this document authorizes NO implementation.** It is
> the PATH, not the build. Each step is its own future lane with its own
> operator-approved directive. No lane may cite this document as execution
> authority; NEXT_ACTIONS.md remains the only execution queue.


**Date:** 2026-07-16. **Status:** operator-approved 2026-07-16; landed by this lane.
**Supersedes:** all conversational roadmap sketches. This document is the path.
**Verified against:** qsl-protocol main `fb1ef2bc` at time of writing (landing base: `6ae8e8ff`, recorded live at landing) (queue, ENG ledger, NA-0645/0646
closeouts, DOC-PROG-003), qsl-server main `8e4ea278`, the 2026-07-16 GUI-readiness
investigation, and the locked design decisions below.

---

## Locked decisions (operator-approved, 2026-07-16)

| # | Decision |
|---|----------|
| L1 | GUI = Tauri v2 desktop app, **in-process** qsc library consumer (no sidecar, no subprocess) |
| L2 | GUI lives in a **separate satellite repo** (qsl-server pattern); qsc pinned by git rev |
| L3 | The NA-0215-era sidecar prototype is **retired** (fresh start) |
| L4 | Capability model: **`GET /v1/server-info`**, additive-only, auth-gated, features-never-security; `directory.mode` defaults `none` |
| L5 | Onboarding order: **vault → identity → server**, with "configure later" escape; single URL field; conditional token field |
| L6 | v1 slice = skeleton + onboarding + identity display + Settings (Server, Vault/Security) — **no** messaging UI in slice 1 |
| L7 | Signal's three-pane layout + settings architecture is the UX reference |
| L8 | Onboarding "This is you" must explain what the fingerprint is FOR (operator note 2026-07-16) |

## Standing constraints (from repo record — not optional)

- **ENG-0044**: the GUI phase MUST NOT close without restoring vault attempt-limit,
  idle autolock, and account-destroy — or a recorded per-feature operator drop
  decision. The implementations are core vault machinery in qsl-protocol history →
  restoration is a **qsc spine lane** (L2 makes satellite-side restoration impossible).
- **External review remains THE release gate** (unchanged by anything here).
- **R8**: v1 documents "do not run the CLI and the GUI against the same profile."
- Lane discipline: one concern per lane; small; directive → approval → execute → PR.

---

## The path

### Completed
- **NA-0649** — qsc GUI-surface (B1 vault_init_with_passphrase, B2 identity data
  as data, B3 identity_ensure) + the approved ErrorCode scope amendment.
  DONE 2026-07-16 at D-1272, result class QSC_GUI_SURFACE_PASS, merge `6e4f7a93`.

### Step 1 — retire the sidecar prototype (LITE)
Delete `qsl/qsl-client/qsc-desktop/`, supersession banners on DOC-QSC-008/009/010,
one decision entry. Env-ingress machinery explicitly KEPT (its own future decision).
*Can slot anywhere; no dependency.*

### Step 2 — server-info lane (cross-repo: qsl-server code + spine governance)
`GET /v1/server-info` per the locked contract: minimal unauthenticated probe body
("QSL relay, auth required" — distinguishable from not-a-relay), full document when
authorized (`name`, `api[]`, `auth.mode`, `limits`, `retention.ttl_secs`,
`directory.mode`, `attachments.service_url`, `kt.mode`, `min_client_version`).
Additive only; push/pull semantics untouched.
*Before the GUI skeleton so onboarding's "test connection" exercises a real contract.*

### Step 3 — GUI satellite bootstrap (two parts)
- **3a (operator):** create the repo (name TBD, e.g. `qsl-desktop`); thin CI
  (single Rust check gate, qsl-server pattern).
- **3b (spine governance lane, small):** register the satellite in the repo-authority
  model — no independent directive authority, cross-repo lanes spine-governed, qsc
  consumed as a rev-pinned git dependency (the ENG-0041 bump-lane pattern applies).

**Decision gate D-A (operator, before step 4): platform target for v1** —
Linux-only, or Linux + macOS? (Build machine is Ubuntu; the spine already runs
macOS CI; the answer shapes the skeleton lane's CI and acceptance.)

### Step 4 — ENG-0044 restoration (qsc spine lane)
Restore attempt-limit + idle autolock + account-destroy from git history as
library surface (the GUI consumes; the CLI need not re-expose). Includes the
one-call lock()/unlock() helper (investigation residue R3) — autolock needs it.
*Before GUI lane 1 closes; ideally before it starts, so the Settings Vault/Security
pane binds to real functions.*

### Step 5 — GUI lane 1: skeleton + onboarding slice
Tauri shell; the four startup rules from the investigation verdict (env+routing
once before threads, drain queue per call, two-global lock pair, strictly serial
core calls); three-pane empty main window; onboarding wizard end-to-end against a
real qsl-server via server-info; identity display (fingerprint + purpose line per
L8 + PQ-status); Settings: Server pane + Vault/Security pane (ENG-0044 controls);
R8 constraint documented. **Acceptance: a fresh machine onboards to a live relay.**

**Decision gate D-B (design conversation, before step 6): the contact-add /
rendezvous problem.** Resolve or explicitly adopt the baseline (out-of-band
verification codes + route token) with the fancier scheme as a later upgrade.
The parked triangle: easy-to-share vs. relay-uncorrelatable vs. spam-resistant.

### Step 6 — GUI lane 2: add-contact + handshake UX
The add-contact flow per D-B's outcome; handshake init/poll surfaced honestly
(pending / verified states); contact list pane goes live.

### Step 7 — GUI lane 3: send/receive + delivery states
Message composition, receive polling (serialized per the v1 concurrency rule),
delivery-state display from the marker vocabulary. Includes routing the
`emit_cli_named_marker` family through the InApp sink (investigation R4 — the
message views need those markers; today they bypass routing).

### Step 8 — GUI lanes 4+: remaining Tier 1
Disappearing messages; basic search. One lane each.

### Step 9 — Tier 2 differentiator UIs (post-Tier-1)
PQ-status indicator (partly ships in onboarding already); KT verifier UI; guided
admission-token UX (pairs with ENG-0036 server work).

---

## Parallel tracks (not on the critical path, with triggers)

| Track | Trigger |
|---|---|
| **Reviewer outreach** (operator; external review is THE gate) | Start NOW — lead time is months; NA-0648 made the package current |
| **ENG-0039** server hardening bundle | MUST land before first real external operator OR public release, whichever first |
| **ENG-0036** per-client admission | Before Tier-2 guided-admission UX; slots into `auth.mode` |
| **ENG-0042/0043** (commit seam, ack default) | Opportunistic; ENG-0043 irrelevant to GUI (it passes ack-mode explicitly) |
| **ENG-0045** demo script fix | Own lane, any time (public-facing accuracy) |
| Env-ingress retirement (post-prototype-retirement residue) | Own future decision; touches 7 pinning test files |

## Corrections this document encodes (vs. the earlier sketch)

1. ENG-0044 moved from "inside GUI lane 1" to its own qsc spine lane (step 4) —
   forced by satellite placement, not just preferred.
2. Old "GUI lane 2" split into steps 6 and 7 (house lane-sizing norm).
3. Reviewer outreach moved from "after Tier 1" to "start now, parallel."
4. ENG-0039 given a hard trigger instead of "rides alongside."
5. Satellite bootstrap split into repo creation + spine governance lane (3a/3b).
6. Platform-target decision (D-A) surfaced explicitly before the skeleton lane.
7. R8 same-profile constraint made a named v1 documentation requirement.

---

*End DOC-PROG-004 v0.1.0 DRAFT. Subordinate at all times to the canonical
specs, GOALS, the queue, the ledger, and independent review. This document
records a plan; it authorizes no implementation and makes no comparative
security claim.*
