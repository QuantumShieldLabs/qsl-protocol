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
- Authoritative source: the highest `QSL-DIR-YYYY-MM-DD-NNN` filename present in
  `/srv/qbuild/operator/directives/`, cross-checked against directives committed
  to the repo. Derive it mechanically; never trust memory.
- **⚠ CORRECTED BY D-1292 — the previous rule named
  `/srv/qbuild/operator/responses/` (highest `_D###.md` suffix) and that
  derivation is BROKEN in two independent ways.** Measured 2026-07-22 over 575
  response files: **(1)** the response-file suffix convention has DRIFTED to
  carry the four-digit **decision** ID rather than the three-digit **directive**
  ID — **34 of 575** files now do, and the newest do, so a naive maximum over
  that directory returns **`1287`**, a decision ID, not a directive number.
  **(2)** Even restricting the scan to three-digit suffixes, the directory tops
  out at **`573`** while the true directive counter is **`602`** — the response
  archive lags the directive archive, so it is stale as well as poisoned. The
  directives directory is the only source that is both complete and unambiguous.
- The drifted response-file suffix convention is recorded here as an **observed
  fact, not a defect to repair**: the 575 existing files are NOT to be renamed.
  Read a response filename's `_D####` suffix as a decision ID and `_D###` as a
  directive ID, and derive counters from the directories that own them.
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

Queue structure and conventions (D-1231):
- `NEXT_ACTIONS.md` opens with a `## LIVE QUEUE` header: a machine/human `STATE:` line, the
  single READY pointer, and an `ON DECK` priority list. Read it first. The full READY block
  (with scope flags) still lives below under section 2 with its `Status: READY` marker (the
  qwork/CI parsers key off that marker, so keep exactly one).
- `docs/ops/IMPROVEMENT_LEDGER.md` is the single authoritative prioritized backlog
  (ENG-####/WF-####, by severity). The DOC-G5-005 §9 table is superseded. `ON DECK` is a
  view of the top ledger items; the Director promotes the top item to READY at each closeout
  (WF-0003 triage).
- NA-#### is a permanent creation-order ID and does NOT imply run order; run order is the
  `ON DECK` list. Inserting a lane never renumbers existing lanes — assign the next free NA
  number and place it in `ON DECK`. Governance/housekeeping edits (e.g. filing ledger
  findings) need not consume an NA number.
- Physically splitting the `DONE` archive out of `NEXT_ACTIONS.md` is deferred to WF-0011
  (it must first update the CI scripts that read `DONE` blocks: `scripts/ci/post_merge_verify.sh`,
  `scripts/ci/qsl_director_state_index.py`, `scripts/ci/public_safety_gate.py`).

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

### 4a. Reporting and queue-verification conventions (recorded by D-1292)

**These four rules were operator conventions in force across every lane before
they were written down anywhere findable.** Rules 1 and 2 existed only in a
`NEXT_ACTIONS.md` archive block and in each session's hand-pasted prompt; rules
3 and 4 existed nowhere in the tree. They are recorded here, in the
authoritative source, rather than in `CLAUDE.md`, which declares itself a
"convenience pointer only" that loses to the authoritative sources — a rule
placed only there has the weakest standing in the tree.

**1. OPERATOR RELAY FILE.** Write `/srv/qbuild/operator/relay/LATEST.md` at the
**END OF EVERY TURN** — overwritten at the same path, plus a timestamped
`RELAY_<UTC>.md` copy in the same directory so history survives the overwrite.
It carries: phase and lane state; what was done this turn; full results and
evidence summaries; any question or flag awaiting a ruling, CLEARLY MARKED;
exact paths and SHAs; and the stop reason. **It MUST be SELF-CONTAINED —
written on the assumption that the reader never saw the terminal.** **THE RELAY
FILE IS THE RESPONSE; terminal output is a preview.** Source: operator rule,
2026-07-21, binding on all lanes and all future directives.

> **⚠ The convention's own burial is an instance of the defect this subsection
> fixes.** Its only prior home was `NEXT_ACTIONS.md:36054` — **inside a DONE
> lane's archive block**, where nothing indexes it and no procedure points at
> it. It survived only by being re-pasted into each session's prompt by hand.
> That is the mechanism by which **a rule everyone follows can still be
> unfindable to a fresh reader**: compliance is no evidence that a rule is
> written down anywhere. The archive block is historical record and is left
> exactly as it is; this entry is additive.

**2. PROACTIVE OBSERVATIONS.** Material, decision-bearing engineering
observations go into a marked `OBSERVATIONS` section of that turn's relay file
**the moment they form** — not held back for a closeout, a summary, or a
convenient stopping point. **Inference is labelled as inference.** **Report;
never act:** an observation is surfaced for a ruling, and the lane does not
widen its own scope to address what it noticed. This was recorded **nowhere** in
the tree before D-1292. It is a different rule from `DOC-DEV-003:140`
("Proactive Improvement & Tooling Defaults"), which governs drive-by
improvements becoming new NA items and does not cover in-flight reporting.

**3. QUEUE VERIFICATION.** Queue state is proven with
`python3 scripts/ci/qsl_evidence_helper.py queue` — **the way `qwork` reads it —
never with a `STATE:` grep.** The `STATE:` line is a human summary; the helper
is the parser that actually gates, and the two can disagree (that disagreement
broke `qwork` on 2026-07-21 and required the #1617 correction merge).

> **⚠ THE RULE CARRIES ITS OWN KNOWN LIMITATION, so that it stays satisfiable
> and honest.** Per **WF-0025**, the subcommand returns `0` **only** at
> `READY_COUNT == 1`; every other count returns **2**. **At a correct
> `READY=NONE` closeout it therefore exits 2 while being entirely correct.**
> Until WF-0025 and WF-0026 land: **a closeout proves the queue by recording
> `READY_COUNT 0` together with the `Status: DONE` section agreeing — the two
> layers agreeing IS the proof — and a closeout instruction MUST NOT be written
> to require exit 0.** **Do not pass `--allow-nonready-count` to obtain a green
> exit code: it suppresses the very check being invoked**, reporting the flag's
> behaviour instead of the queue's. **A gate that cannot pass teaches bypass**
> (established at NA-0665, confirmed by the operator).
>
> Per **WF-0026**, the parser is additionally blind to two-letter-suffix
> headings (`### NA-0215BA` and six others). **A `READY_COUNT 0` on a lane that
> was genuinely promoted is that bug, not an empty queue** — check the heading
> before concluding the queue is clear.

**4. RELAY FILE VERSUS RESPONSE FILE — the two are DISTINCT, and the response
file is STILL OWED.** Determined from evidence at D-1292, not assumed:

| | operator relay file | numbered-section response file |
|---|---|---|
| cadence | **every turn** (58 files in ~2 days) | **per lane milestone** (NA-0663 produced 7) |
| key | UTC timestamp only; `LATEST.md` is mutable | lane + UTC + decision ID; immutable |
| scope | all activity, **including Director-only turns with no lane executing** | lane execution under a decision |
| retrieval | by time | **by lane and decision** |
| shape | headings vary per turn | fixed numbered sections, incl. response-safety and response-file proofs |
| mandate | operator standing rule, 2026-07-21 | `CLAUDE.md` step 6, **and every directive's `Response file target:` line** |

**The relay file did NOT supersede the response file.** The relay rule's own
words contrast it with *terminal output* — "the relay file IS the response;
terminal output is a preview" — and what it displaced is the **terminal/chat
response**, which the operator's advisor cannot see. Nothing retired the
response file: it remains mandated by `CLAUDE.md` step 6, and **every directive
drafted after the relay convention was adopted — D600, D601, and D602 itself —
still names a `Response file target:`.**

**The measured discrepancy, recorded rather than explained away:** the newest
response file is NA-0663's, `2026-07-21T04:04Z`; the relay convention began
`2026-07-21T15:32Z`; **NA-0664 and NA-0665 wrote no response file at all.**
That is displacement **in practice** and an **unpaid obligation** — it is not a
retirement, and it must not be recorded as one. **Both lanes still owe a
response file.** A supersession invented here to make the discrepancy disappear
would be the exact failure this subsection exists to prevent, committed while
preventing it.

> **⚠ SPLIT AUTHORITY, STATED SO IT IS NOT MISTAKEN FOR A SECOND
> CONTRADICTION.** The reconciliation above is **authoritative here and settled
> now** — this document is an authoritative source and wins over `CLAUDE.md`,
> which declares itself a convenience pointer. **`CLAUDE.md` step 6 (`:47-50`)
> is deliberately NOT edited by D-1292** (`CLAUDE.md` is not a docs path, so
> touching it fires both full suites — see **WF-0032**), and its text is
> therefore **known-stale in one respect only**: it presents the response file
> as the reporting artifact without mentioning the relay file. **A reader who
> finds `CLAUDE.md:47-50` still mandating a response file is reading a
> requirement that is still LIVE — it is the relay convention that is missing
> there, not the response file that is dead.** The pointer to this subsection
> and any correction to that text ride **WF-0032**'s sequencing: free once
> `CLAUDE.md` is a docs path, or carried by the first future lane already
> paying `docs_only=false`.

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

### 9a. LITE read-only-audit fast-path (certification checklist)

A read-only audit qualifies for the LITE-CEREMONY single-PR/single-decision path when
ALL of the following hold; the Director certifies them in the directive:

1. Output is docs/evidence/ledger/governance ONLY — no source/test/Cargo/workflow/
   `.claude`/hook/spec change and no fix (every fix is deferred to its own lane).
2. No runtime/LAN/qscwork action; reads code and contracts only (existing suites MAY be
   run unmodified to observe behavior).
3. The successor is either clear from the findings or the directive names a default plus
   a stop-and-request-direction fork; a P0/P1 finding escalates before merge.
4. All normal gates still apply: operator startup proof, one decision ID, the
   exactly-one-READY invariant, validation defaults, bounded CI polling, and the claim
   boundaries. Precedents: NA-0611, NA-0612, NA-0613.

If any item is uncertain, use the full ritual. This fast-path reduces ceremony only; it
never reduces evidence or fail-closed safety.

### 9b. Batch-audit convention (related read-only audits over a shared surface)

Several related read-only audits that read the SAME code/contract surface MAY be carried
by one directive as a WAVE lane (§9) instead of a chain of separate lanes, to avoid
repeated plan/startup/PR cycles. Each sub-audit keeps its own decision ID, evidence,
findings, and claim boundary; the batch shares a single closeout and successor
selection. The batch remains subject to the §9 hard boundary and the §9a checklist: it
is read-only, docs-only output, no fix, no runtime action. Do not batch audits that span
unrelated surfaces (keep findings attributable) or any item that would touch protocol/
crypto/security semantics, dependencies, workflows, or branch protection.
