Goals: G4, G5

Status: Supporting (product planning; subordinate to canonical specs, GOALS, the queue, and independent review)
Owner: QSL governance
Last-Updated: 2026-07-18 (NA-0656/D592: v0.2.0 — qsl-desktop registration (step 3b), gate D-A, accumulated GUI-phase design decisions)
Authority: Non-normative product planning. Does NOT override START_HERE, GOALS,
the canonical specs, NEXT_ACTIONS, the IMPROVEMENT_LEDGER, or any recorded
decision. Where this document and any of those disagree, THEY win and this
document is corrected.

# DOC-PROG-004 — QSC GUI Phase Roadmap (v1) v0.2.0 DRAFT

> Provenance: Records the operator-approved QSC GUI Phase Roadmap of 2026-07-16,
> landed per QSL-DIR-2026-07-16-586 (D586, as amended at approval: A1 filename,
> A2 lane number) by lane NA-0650 (D-1273). "Landed by this lane" in the Status
> line below refers to NA-0650. The body below is the operator-approved text
> verbatim under exactly three landing adjustments — the title form, this
> wrapper, and the landing-base resolution — proven hunk-by-hunk in
> docs/governance/evidence/NA-0650_as_built.md.
>
> v0.2.0 revision: per QSL-DIR-2026-07-18-592 (D592) by lane NA-0656 (D-1279),
> folding in the operator-approved GUI-phase decisions of 2026-07-17/18 (gate
> D-A, the post-v1 horizon, the message-history design, the ENG-0044
> refinement, the skeleton/onboarding additions, step-status updates) and the
> qsl-desktop satellite registration (step 3b). Every v0.1.0 → v0.2.0 hunk
> maps to the enumerated revision classes R0–R8, proven hunk-by-hunk in
> docs/governance/evidence/NA-0656_as_built.md.
>
> **Authorization boundary: this document authorizes NO implementation.** It is
> the PATH, not the build. Each step is its own future lane with its own
> operator-approved directive. No lane may cite this document as execution
> authority; NEXT_ACTIONS.md remains the only execution queue.


**Date:** 2026-07-16. **Status:** operator-approved 2026-07-16; landed by this lane.
**Revised:** 2026-07-18 (v0.2.0; the decisions of 2026-07-17/18, landed by lane NA-0656).
**Supersedes:** all conversational roadmap sketches. This document is the path.
**Verified against:** qsl-protocol main `fb1ef2bc` at time of writing (landing base: `6ae8e8ff`, recorded live at landing) (queue, ENG ledger, NA-0645/0646
closeouts, DOC-PROG-003), qsl-server main `8e4ea278`, the 2026-07-16 GUI-readiness
investigation, and the locked design decisions below.

**v0.2.0 verified against:** qsl-protocol main `557bb8b2` (recorded live
at landing), qsl-server main `3cc551a8` (= the qsc dev-dep pin since
ENG-0046/D-1277), and the qsl-desktop repo state recorded live 2026-07-18
(public, EMPTY — no HEAD; private vulnerability reporting ENABLED).

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
| L9 | Gate D-A (2026-07-17): **v1 platform = Linux-only**; macOS = first post-skeleton platform lane; Windows = post-Tier-1 horizon |

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
- **Step 1 / NA-0651** — the sidecar prototype retired. DONE 2026-07-17 at
  D-1274, result class QSC_DESKTOP_RETIRE_PASS, merge `b3cfd5df`.
- **Step 2 / NA-0652** — `GET /v1/server-info` shipped (cross-repo). DONE
  2026-07-17 at D-1275: qsl-server merge `3cc551a8` (DOC-SRV-006), spine
  closeout `e46cb6b3`. Pin advanced by **ENG-0046 / NA-0654**: DONE 2026-07-18
  at D-1277, merge `0a8e0843` (qsc dev-dep pin = `3cc551a8`; e2e + full suite
  green unchanged).
- **Step 3a (operator)** — `QuantumShieldLabs/qsl-desktop` created 2026-07-17
  (operator console; GitHub created_at 2026-07-18T00:00:56Z), public, EMPTY by
  design (no initial commit — bootstrap is its own lane).
- **Parallel / NA-0655** — community-health docs (SECURITY / CODE_OF_CONDUCT /
  CONTRIBUTING) landed on the three existing satellites; four merges
  2026-07-18 (spine `345edcd9`); private vulnerability reporting ENABLED on
  all four org repos. D-1278.
- **Step 3b / NA-0656** — this v0.2.0 revision (D-1279): the registration
  record in step 3 below.

### Step 1 — retire the sidecar prototype (LITE)
*DONE 2026-07-17 — NA-0651 (D-1274), merge `b3cfd5df`; see Completed.*
Delete `qsl/qsl-client/qsc-desktop/`, supersession banners on DOC-QSC-008/009/010,
one decision entry. Env-ingress machinery explicitly KEPT (its own future decision).
*Can slot anywhere; no dependency.*

### Step 2 — server-info lane (cross-repo: qsl-server code + spine governance)
*DONE 2026-07-17 — NA-0652 (D-1275); pin bump NA-0654 (D-1277) 2026-07-18; see
Completed.*
`GET /v1/server-info` per the locked contract: minimal unauthenticated probe body
("QSL relay, auth required" — distinguishable from not-a-relay), full document when
authorized (`name`, `api[]`, `auth.mode`, `limits`, `retention.ttl_secs`,
`directory.mode`, `attachments.service_url`, `kt.mode`, `min_client_version`).
Additive only; push/pull semantics untouched.
*Before the GUI skeleton so onboarding's "test connection" exercises a real contract.*

### Step 3 — GUI satellite bootstrap (two parts)
- **3a (operator): DONE 2026-07-17** — repo `qsl-desktop` created (operator
  console), public, EMPTY by design; the thin CI (single Rust check gate,
  qsl-server pattern) is OWED at the bootstrap lane, not yet landed.
- **3b (spine governance lane, small): DONE at this revision — NA-0656
  (D-1279)** — the registration record:

**Registration record (v0.2.0, 2026-07-18).** `QuantumShieldLabs/qsl-desktop`
is a SATELLITE repo, NOT a peer (the qsl-server model, D578/D-1265):
- ALL directive/queue/decision authority lives in the qsl-protocol spine;
  qsl-desktop has no qwork, no directives directory, no queue-as-authority,
  no guardrail hooks.
- Cross-repo lanes are SPINE-GOVERNED: GUI code lands as qsl-desktop PRs; the
  spine carries the directive, the decision, and the governance closeout.
- qsc is consumed as a REV-PINNED git dependency; pin advances are deliberate
  spine-governed bump lanes with the suite/e2e re-proof (the ENG-0041/D-1266
  and ENG-0046/D-1277 pattern).
- OWED at the bootstrap lane (recorded as owed; explicitly NOT landed by this
  revision): the thin single-check CI gate (the qsl-server required-`rust`
  pattern) and the branch protection carrying it; the community-health file
  set (SECURITY.md / CODE_OF_CONDUCT.md / CONTRIBUTING.md — the NA-0655/
  D-1278 set); the repo-local pointer CLAUDE.md (the NA-0608A satellite
  pattern); the repo-local DECISIONS.md log (the satellite convention).
- Recorded live at registration (2026-07-18): repo public, EMPTY (no HEAD);
  private vulnerability reporting ENABLED.

**Decision gate D-A — DECIDED 2026-07-17 (operator): v1 platform target =
Linux-only.** macOS is the FIRST post-skeleton platform lane (and the trigger
for the multi-device horizon below); Windows sits at the post-Tier-1 horizon.
The skeleton lane's CI and acceptance are Linux-only. (Recorded as locked
decision L9.)

### Step 4 — ENG-0044 restoration (qsc spine lane)
Restore attempt-limit + idle autolock + account-destroy from git history as
library surface (the GUI consumes; the CLI need not re-expose). Includes the
one-call lock()/unlock() helper (investigation residue R3) — autolock needs it.
*Before GUI lane 1 closes; ideally before it starts, so the Settings Vault/Security
pane binds to real functions.*

**ENG-0044 design refinement (operator-approved 2026-07-17):** the
attempt-limit DEFAULT is lock-with-escalating-delay, not wipe;
wipe-after-N-tries is an explicit OPT-IN; panic/account-destroy stays a
deliberate, instant wipe. All three surface as GUI Settings → Vault/Security
switches with conservative defaults. Honest scope note (v1 documents it):
try-limits defend the device-in-hand path THROUGH the app; an attacker who
copies the vault file offline is defended only by passphrase strength +
Argon2id.

### Step 5 — GUI lane 1: skeleton + onboarding slice
Tauri shell; the four startup rules from the investigation verdict (env+routing
once before threads, drain queue per call, two-global lock pair, strictly serial
core calls); three-pane empty main window; onboarding wizard end-to-end against a
real qsl-server via server-info; identity display (fingerprint + purpose line per
L8 + PQ-status); Settings: Server pane + Vault/Security pane (ENG-0044 controls);
R8 constraint documented. **Acceptance: a fresh machine onboards to a live relay.**

**Skeleton/onboarding spec additions (operator-approved 2026-07-17/18):**
- The UNLOCK screen is a designed surface: wrong-passphrase feedback, the
  escalating-delay display, and idle-autolock re-entry.
- No-recovery honesty: before the passphrase is committed the wizard states
  plainly that there is NO recovery; passphrase confirm + strength feedback.
- Interrupted-onboarding resume: safe re-entry via the `vault_exists` init
  error contract (`vault_init_with_passphrase` refuses when a vault is already
  present) + `identity_ensure` idempotency; "start over" is safe strictly
  PRE-contacts.
- The GUI gets its OWN app-scoped data directory in v1; an existing CLI vault
  is detected and reported honestly ("found; v1 doesn't import") — composes
  with R8.
- The test-connection error taxonomy includes "certificate not trusted"
  (private-CA self-hosters); an operator runbook note is OWED (recorded here,
  not landed by this revision).
- Identity export is Settings-only in v1 and NEVER part of onboarding; off by
  default, loud warnings; revisit after tester feedback.
- Idle auto-lock ON by default (~15 minutes, adjustable) — binds to the
  step-4 autolock restoration.
- Onboarding order CONFIRMED: vault → identity → server (L5 stands).

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

**Message-history design (operator-approved 2026-07-17):** message CONTENT is
not persisted today — the timeline store is encrypted METADATA only (id /
peer / direction / byte-length / kind / timestamps / delivery-state; no
content field). The agreed design: a SEPARATE encrypted message-history store
whose key lives in the vault (unlock vault → unlock history); disappearing
messages (Tier 1) are the retention control; an optional no-history mode;
Tier-1 basic search (step 8) depends on this stored history.

### Step 8 — GUI lanes 4+: remaining Tier 1
Disappearing messages; basic search. One lane each.

### Step 9 — Tier 2 differentiator UIs (post-Tier-1)
PQ-status indicator (partly ships in onboarding already); KT verifier UI; guided
admission-token UX (pairs with ENG-0036 server work).

---

## Horizon (named, post-v1) — recorded 2026-07-17

Named so they shape v1 design; nothing here authorizes any build.

- **Multi-device sync (Tier 1.5).** Trigger: the second platform lane (macOS).
  Design sketch TO CONSIDER (recorded, not locked): identity-signed per-device
  keys; per-device inboxes; a QR linking ceremony; a signed device manifest
  distributed peer-to-peer through existing encrypted sessions (the relay
  never learns the device map); fan-out send; self-sync via device-to-device
  sessions; signed revocation. OPEN QUESTIONS (recorded, undecided): linking
  authority — primary-only vs any-device (lean: primary-only); trust-mode
  handling of identity-signed new devices.
- **WebSocket real-time push.** Paired with mobile, or earlier if instant-feel
  becomes a priority.
- **Mobile clients.** Post-Tier-1.

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

*End DOC-PROG-004 v0.2.0 DRAFT. Subordinate at all times to the canonical
specs, GOALS, the queue, the ledger, and independent review. This document
records a plan; it authorizes no implementation and makes no comparative
security claim.*
