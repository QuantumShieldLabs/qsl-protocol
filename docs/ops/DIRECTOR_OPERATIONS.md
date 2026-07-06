Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-06

# DOC-OPS-006 — Director Operations: Directive Template, Counters, and Standing Conventions v0.1.0 DRAFT

Purpose: This document makes the Director role repo-backed. It captures the
directive template, ID/counter rules, and standing operator conventions that
previously existed only in the Director chat context (handed off at D539).
If this document conflicts with START_HERE.md, GOALS.md, AGENTS.md, or
NEXT_ACTIONS.md, those sources win.

## 1. Canonical directive template

Section order (sections vary by lane; core order is fixed):

    identity (Directive ID, title, response file target)
    ROLE (READY lane, objective, result boundary, explicit non-goals)
    FRESH OPERATOR QWORK HANDOFF (proof files, required values, stop rules)
    EXPECTED CURRENT STATE (verified live, not asserted — see §5)
    AUTHORITY MODEL (Tier 0 read-only … Tier 5 still-forbidden)
    STRICT IMPLEMENTATION SCOPE (allowed impl/closeout/runtime paths;
      forbidden paths; cross-repo boundaries)
    LAN / REMOTE / RUNTIME MODEL (if relevant)
    PHASE 0 — proof root, qwork, repo, disk, queue, main health
    PHASE 1 — inheritance review (consume predecessor decisions)
    PHASE 2+ — lane-specific readiness/runtime/proof phases
    PHASE N — result classification
    PHASE N+1 — successor selection
    PHASE N+2 — governance patch
    PHASE N+3 — validation before PR
    PHASE N+4 — implementation PR
    PHASE N+5 — post-merge verification
    PHASE N+6 — optional closeout
    PHASE N+7 — final post-merge verification
    CONTINUOUS CI WAIT-WORK PACKET (mandatory for waits > 60 s)
    FINAL RESPONSE REQUIRED SECTIONS (numbered; ends with
      "Stop Reason, If Stopped")
    STOP CONDITIONS

Mandatory phrasing conventions:

    "Prove; do not assume."
    "Classify; do not overclaim."
    "The executor must not run qwork/qstart/qresume."
    "Exactly one READY."
    "Do not implement the successor during closeout."
    "Public-safety and advisories must be green."
    "No public/production/crypto-complete/attachment-complete/bug-free/
     vulnerability-free claims."
    "Raw private values remain proof-root-only."
    "Publish class summaries only."

## 2. Counter rules (authoritative sources)

Directive ID `QSL-DIR-YYYY-MM-DD-NNN` and short form `D###`:
- NNN/D### is a monotonic counter across all directives; never reuse.
- Authoritative source: the highest `_D###.md` suffix present in
  `/srv/qbuild/operator/responses/` (legacy history from the Codex era is
  copied into that directory so the counter is continuous), cross-checked
  against directives committed to the repo. Derive it mechanically; never
  trust memory.
- Handoff state: last issued D539; next is D540 unless D539 is explicitly
  reissued with recorded explanation.

Decision ID `D-####`:
- Increments by 1 from the highest accepted decision in DECISIONS.md
  (canonical form `- **ID:** D-####`; count canonical lines, not prose
  mentions). Never reuse; must exist exactly once, dated, accepted, mapped
  to Goals.
- Handoff state: D-1205/D-1206 accepted; D-1207 is next and must be absent
  until NA-0608 implements.

Directive IDs and decision IDs are distinct namespaces.

## 3. File and path conventions

    Response files:
      /srv/qbuild/operator/responses/
      NA####_<lane_slug>_<timestamp>_D###.md
      (legacy Codex-era responses are copied here from
      /home/victor/work/qsl/codex/responses/, which is retained read-only
      as backup)

    Approved directive texts:
      /srv/qbuild/operator/directives/
      QSL-DIR-YYYY-MM-DD-NNN_<lane_slug>.md

    Proof roots:
      /srv/qbuild/tmp/NA####_<lane_slug>_<timestamp>/

    qwork proof files (operator-run only):
      /srv/qbuild/work/NA-####/.qwork/startup.qsl-protocol.kv
      /srv/qbuild/work/NA-####/.qwork/startup.qsl-protocol.json
      /srv/qbuild/work/NA-####/.qwork/cargo-target.qsl-protocol.env

    qwork invariants (verify all before any mutation, fetch, service work,
    remote access, GitHub metadata, PR creation, or proof publication):
      startup_result=OK; lane matches; repo=qsl-protocol;
      head==origin/main==main and matches live pre-fetch state;
      worktree/index/untracked clean; ready_count=1;
      queue_top_ready=requested lane; requested_lane_status=READY;
      shared_target_ready=yes

    Durable proof tooling (promoted from NA-0607 recovery work):
      /srv/qbuild/tools/na0607-proof-tools/
        decision_id_counter.py
        added_line_publication_scan.py
        qwork_env_parser.py
        qsl_attachments_readiness_json_check.py

## 4. Standing conventions

Disk/mount gates: root usage below 95%; /backup/qsl must be mounted; STOP
otherwise.

Queue discipline: exactly one READY; STOP if zero or more than one; no
successor implementation during closeout or CI waits.

PR rules: small atomic PRs; merge commits only; no squash/rebase/force-push;
no amend after PR creation; no branch-deletion flags (repo settings may
auto-delete after merge). PR body requires Goals / Impact / No-regression /
Tests-Vectors sections.

Required checks: public-safety green; advisories green; no failed or pending
required checks at merge/closeout boundaries; visibility-recovery procedures
apply only to context-name visibility, never to bypass a failed check.

Validation defaults (run before every PR): git diff --check; scope guard;
queue/decision proof; marker proof; link check; added-line private-material
scan; prohibited-material scan; overclaim scan; claim-boundary scan;
docs/governance classifier; PR body preflight; goal-lint; root cargo audit;
nested qsc fuzz cargo audit; cargo metadata --locked --format-version=1;
cargo fmt --check; sh -n and bash -n on scripts/ci/qsc_adversarial.sh.

Runtime evidence: raw logs proof-root-only; published responses class-only.

LAN/qscwork model (only when a directive explicitly authorizes it): SSH alias
qscremote; account qscwork; verify whoami == qscwork; operate only inside
qscwork-owned test workspace; no sudo, system install, personal file access,
broad scans, or second executor on the laptop.

Terminology discipline: QSL/qsc is the cryptographic/content protocol;
HTTP/HTTPS/SSH/Tailnet are transport/carriage layers; SSH loopback-forward
is a LAN test transport, not the QSL security protocol.

## 5. Verified state replaces asserted state

Because the Director and Executor now share one seat with live repo access,
directives MUST NOT hard-code expected SHAs or decision counts from memory.
The EXPECTED CURRENT STATE section instead instructs: verify live HEAD ==
origin/main, verify queue/decision invariants with the durable proof tools,
record the observed values in the proof root and response, and STOP on any
inconsistency. The fail-closed checks are unchanged; only their source of
truth moves from human transcription to direct verification.

## 5a. Operational layout (post-transition)

    /srv/qbuild/tools/claude/qsl_guardrails_hook.sh   PreToolUse guardrail hook
    /srv/qbuild/operator/responses/                    response files (see §3)
    /srv/qbuild/operator/directives/                   approved directives
    <repo>/.claude/settings.json                       committed; propagates to
                                                       every qwork lane checkout
    <repo>/CLAUDE.md                                   committed; ditto

Claude Code's own install, auth, and user config remain per-user in the home
directory by design (like ~/.ssh and ~/.config/gh); project state must not
live there.

## 6. Environment facts (build machine, recorded at handoff)

- gh 2.96.0 at /usr/bin/gh; PR creation via local gh CLI auth (OAuth token,
  scopes gist/read:org/repo/workflow). The GitHub App integration cannot
  create PRs (403); do not use it for that. `gh pr create --json` is
  unsupported on this version: create first, then `gh pr view --json`.
- rustup-managed stable rustc 1.95.0 x86_64-unknown-linux-gnu; qbuild uses
  /srv/qbuild/cache/{cargo,rustup}; shared targets keyed
  /srv/qbuild/cache/targets/<repo>/rustc-1.95.0-x86_64-unknown-linux-gnu/default.
  qwork preserves a preexisting CARGO_TARGET_DIR; unset it to get shared mode.
- cargo-audit 0.22.1 installed; cargo-deny/cargo-nextest/sccache not installed
  (do not assume them; installing requires explicit authorization).
- qwork wrapper at /home/victor/.local/bin/qwork wrapping
  /srv/qbuild/tools/qwork.sh; qwork_version_or_sha is the SHA-256 of that
  script. qstart/qresume are not on PATH. None of these are executor-runnable.
- qwork cargo env files are shell `export KEY=value` format, not plain KV.
- Cleanup: drop_checkout.sh removes disposable /srv/qbuild/work checkouts
  only; prune_evidence.sh is dry-run by default (--apply to act,
  --allow-live-evidence for live evidence); prune manifests under
  /srv/qbuild/logs/prune.

## 7. Held roadmap intentions (recorded from D539 Director handoff)

- Continue LAN baby steps before any Tailnet/GitHub-runner work.
- NA-0608 stresses the LAN qsl-attachments path (wrong/missing capability,
  corrupted descriptor/object, wrong route/peer, missing/deleted object,
  replay-like duplicate, log/storage/state reviews, seed-fallback regression,
  metadata matrix, cleanup, private-material scan).
- Future lane family to record: "QSL Hostile Analyst / Metadata Minimization
  and Implementation Attack Hardening Plan" covering traffic-analysis
  metadata, implementation attacks, relay and qsl-attachments compromise
  models, malformed envelope/descriptor/object tests, padding/bucketing
  feasibility, error/retry normalization, and external/formal review
  readiness.
- Direct LAN HTTPS is a later transport-hardening track; it protects
  carriage-layer metadata only and never replaces the QSL protocol.
- Deferred until explicitly authorized: Tailnet/GitHub-runner, public
  endpoint/DNS/Cloudflare, self-hosted runner, laptop SSH server, second
  executor on laptop (not selected).

## 8. Director triage discipline (mandatory)

Every Director turn must, before drafting the directive:

- read `docs/ops/IMPROVEMENT_LEDGER.md` (DOC-OPS-007) and the release gates in
  `docs/program/DOC-PROG-001`; and
- justify successor selection against them: either advance the highest actionable
  ledger item (or the item that best closes a DOC-PROG-001 release gate), or
  record in the directive/evidence why the current READY item takes precedence.

This triage informs successor choice only. It never overrides `NEXT_ACTIONS.md`
order authority, never reorders or promotes queue items implicitly, and never
weakens a fail-closed gate. If the ledger and the queue disagree about priority,
`NEXT_ACTIONS.md` wins and the ledger/roadmap is updated later. When a lane
discovers a finding or process issue, the Director ensures a ledger entry is
filed or updated before closeout (see `AGENTS.md`).

## 9. Lane classes (optional, Director-declared)

A directive MAY declare one of two reduced-ceremony classes. If none is declared,
the full ritual applies (separate implementation and closeout PRs/decisions).

- WAVE lane: one directive carries several bounded, related sub-items with shared
  evidence and a single closeout (precedent: the NA-0217A–J modularization wave
  and the NA-0235A paired set). All normal gates apply to each sub-item: scope
  guard, decision IDs, validation defaults, claim boundaries, and evidence.

- LITE-CEREMONY lane: for genuinely low-risk docs/process/read-only-audit work
  ONLY, the implementation and closeout MAY be a single PR and a single decision
  instead of two. It STILL requires operator-run qwork proof, a decision ID,
  evidence + testplan, the exactly-one-READY invariant, the validation defaults,
  bounded CI polling, and the claim boundaries.

Hard boundary (fail-closed): neither class may be used for anything that touches
protocol/wire/crypto/auth/state-machine or other security semantics,
dependencies, lockfiles, workflows, branch protection, the public-safety or
advisories gates, or any runtime/LAN/qscwork action. Those always use the full
two-PR ritual. The Director must certify in the directive that the lane qualifies
before using a reduced class; if qualification is uncertain, use the full ritual.
These classes reduce ceremony only; they never reduce truthfulness, evidence, or
fail-closed safety.
