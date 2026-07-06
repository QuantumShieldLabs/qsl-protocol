# NA-0608A Executor Transition — Claude Code Onboarding, Mechanical Guardrails, and Director Governance Backfill

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G4 (primary), supports G1–G5

## Summary

NA-0608A is an inserted governance/tooling lane. It transitions the executor
seat from Codex to Claude Code with mechanical guardrails, migrates operational
artifacts off the home directory, and backfills the formerly chat-held Director
conventions into the repository as `docs/ops/DIRECTOR_OPERATIONS.md`
(DOC-OPS-006). It does not implement NA-0608, and it changes no qsc /
qsl-server / qsl-attachments source, test, dependency, lockfile, or workflow,
and no protocol behavior. NA-0608 remains READY throughout and is the sole
READY successor at closeout.

Result classification: `EXECUTOR_TRANSITION_CLAUDE_CODE_ONBOARDING_PASS`.

## Bootstrap exception record (D540 only)

qwork could not be run for NA-0608A because the lane did not yet exist in
NEXT_ACTIONS.md until this patch. For this directive only, qwork proof was
replaced by operator-created workspace plus executor-verified invariants. This
exception applies to D540 only; all subsequent lanes require operator-run qwork
proof per DOC-OPS-006 (DIRECTOR_OPERATIONS) §3.

Verified directly before any mutation (class-only; raw values proof-root-only):

- pwd `/srv/qbuild/work/NA-0608A/qsl-protocol`; branch `main`.
- HEAD == main == origin/main == baseline `ceea85ecb82f10f506fbacf6b6f8c6368fcfd387`
  after fetch; main not advanced.
- Worktree, index, and untracked state clean.
- READY_COUNT 1; sole READY NA-0608; NA-0608A heading absent pre-patch.
- D-1205 present once; D-1206 present once; D-1207 absent; duplicate decision
  count zero (durable `decision_id_counter.py`).
- Root filesystem usage 47% (< 95%); `/backup/qsl` mounted.
- All seven staged transition files present and readable.
- Main required checks green: public-safety, advisories, suite2-vectors,
  qsc-adversarial-smoke, CodeQL Analyze (all languages); no failed or pending
  required checks.

## Counter-shift record

- Last directive D539; this directive D540; NA-0608's directive will be D541.
- Highest accepted decision before this lane D-1206. NA-0608A consumes D-1207
  (implementation) and D-1208 (closeout).
- NA-0608 therefore begins at D-1209 (D-1209 must be absent until NA-0608
  implements). This explicitly supersedes the earlier handoff expectation of
  "D-1207 for NA-0608".
- Directive IDs (`D###`) and decision IDs (`D-####`) are distinct namespaces.
- Legacy anomaly reconciled: one legacy response file used decision id D-1199
  in the directive-number filename slot (NA-0604 implementation response, PR
  #1482). It is not a directive numbered 1199; the highest legacy directive
  number is 539.

## Staged transition file inventory (SHA-256; as reviewed)

- CLAUDE.md (qsl-protocol root pointer):
  `759586f9bfcc2c27dbd1daf87ac76400d3f2453068f6be7f54c12e3f514c44a0`
- CLAUDE_qsl-server.md (satellite pointer):
  `96183e0bfadc91c7d60548b57cacec0dbe8afd3fa809fb523972c518d97f278d`
- CLAUDE_qsl-attachments.md (satellite pointer):
  `a7422adec2072d6dede64caa968b8f3f85810a7c8e135336af5f62ebe98e8516`
- DIRECTOR_OPERATIONS.md (pre-correction):
  `d9f82ad9ec5ef2d7c585c7538e445f2b63d57f867aade1b51dea1d674318ec1c`
- settings.json:
  `d7a4f733dea9c66a5c373a94d50ae98ed80b7cf41062ff4cc2506ec7dfe3f913`
- qsl_guardrails_hook.sh:
  `4167513dc4440442ec2285f57f869010c17998f9d75b9d2aaf34333a308f8476`
- setup_claude_layout.sh (pre-correction):
  `595003ab4216b2f9da5e816b463049bc014226bc82975abdc8d5d3fc15df69ce`

## Tier 3 corrections (bounded; no guardrail weakened)

1. `DIRECTOR_OPERATIONS.md` §6 environment facts — `gh 2.93.0` corrected to
   `gh 2.96.0` to match live `gh --version` (2.96.0, 2026-07-02) at
   `/usr/bin/gh`. Single-token factual-drift correction.
2. `setup_claude_layout.sh` highest-directive report — regex `_D[0-9]+\.md$`
   restricted to `_D[0-9]{3}\.md$` so the true highest DIRECTIVE number (539)
   is reported rather than the decision-id-in-slot anomaly (D-1199). Reporting
   fidelity only; the anomaly is documented, not hidden.
3. `docs/ops/DIRECTOR_OPERATIONS.md` header — DOC-OPS-006 assigned with the
   standard classification header (Status/Owner/Last-Updated) per Phase 5
   ("assign DOC number and Goals line"), mirroring DOC-OPS-001…005. Filename
   remains the directive-allowed `docs/ops/DIRECTOR_OPERATIONS.md`; a future
   docs-only lane may rename to the full DOC-OPS-006 filename form.

No deny rule or hook block was removed or narrowed. Two spurious
`Write(inputs/phase2/**)` / `Write(inputs/phase3/**)` deny entries (leftover
from the authoring environment; they protect non-existent repo paths) were left
intact rather than narrowed.

## Off-home layout setup (Phase 3)

- Created (victor:victor): `/srv/qbuild/tools/claude/`,
  `/srv/qbuild/operator/responses/`, `/srv/qbuild/operator/directives/`.
- Installed `/srv/qbuild/tools/claude/qsl_guardrails_hook.sh` mode 0755,
  byte-identical to the reviewed staged hook.
- Legacy Codex-era responses copied non-destructively (`cp -an`) from
  `/home/victor/work/qsl/codex/responses/` (retained read-only as backup):
  512 source files == 512 destination files. Three spot-checked originals were
  byte-identical (mtime + SHA-256 unchanged) after the copy.
- Highest directive number reported: 539.
- Archived directive text to
  `/srv/qbuild/operator/directives/QSL-DIR-2026-07-06-540_executor_transition_claude_code_onboarding.md`.

## Guardrail empirical verification (Phase 4, pre-commit; class-only)

The PreToolUse hook is the primary enforcement layer; the `permissions.deny`
rules in `.claude/settings.json` are the second layer. Verified by feeding the
installed hook the exact PreToolUse JSON payload interface Claude Code uses
(stdin JSON; exit 2 blocks, exit 0 allows):

- Must-block (all BLOCK): sudo, qwork, qstart, qresume, tailscale,
  systemctl, gh workflow run, git push --force, git rebase, gh pr checks
  --watch, prune_evidence.sh --apply (12/12). Ten additional Tier-5 classes
  (amend, hard reset on main, squash/rebase merge, delete-branch, gh run
  rerun/watch, apt/apt-get, firewall) also BLOCK.
- Must-allow (all ALLOW): git status, cargo fmt --check, gh pr list, the
  durable decision_id_counter.py --help (4/4). PR-create/merge (merge form),
  push (non-force), gh api reads, and cargo audit also ALLOW.

Session-context note: this onboarding session started before
`.claude/settings.json` existed. Claude Code snapshots PreToolUse hooks at
session start, so newly added hooks require a session restart / hook review to
engage live — a step an autonomous session cannot self-perform. Dangerous
must-block commands were therefore not executed live; enforcement was proven at
the hook interface and re-verified from a fresh checkout post-merge (Phase 9).
Live in-session enforcement engages automatically for any session that starts
with the committed `.claude/settings.json` present — including every disposable
qwork lane checkout.

Known limitation (recorded): the hook matches command-class keywords even
inside quoted argument text (e.g. a commit message containing the word
"sudo"), which is fail-closed (errs toward blocking) and affects none of the
required must-allow commands. A future hardening lane may tokenize more
precisely; the merged hook is not modified in this lane.

## Scope and boundary

Mutations are limited to the directive's allowed implementation paths: root
`CLAUDE.md`, `.claude/settings.json`, `docs/ops/DIRECTOR_OPERATIONS.md`, this
evidence doc, the NA-0608A testplan, `NEXT_ACTIONS.md`, `DECISIONS.md`,
`TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. Satellite
repos receive only a new `CLAUDE.md` each. No qsc / qsl-server /
qsl-attachments source, test, dependency, lockfile, or workflow changed; no
protocol behavior changed; no qwork/qstart/qresume, sudo, systemd, firewall,
Tailnet, or qscwork/laptop access occurred; no workflow dispatch/rerun; no
branch-protection or repo-settings mutation; nothing under the legacy responses
directory was modified. No token, key, SSH-config, LAN hostname/IP, or personal
material is published; only the operational path constants (`/srv/qbuild/...`
and the legacy `/home/victor/work/qsl/codex/responses/` reference) appear.

## Claim boundary

No public-readiness, production-readiness, remote-ready, Tailnet-ready,
LAN-ready, crypto-complete, attachment-complete, security-complete,
bypass-proof, vulnerability-free, or bug-free claim is made. This lane proves
only that the reviewed executor guardrails block the tested Tier-5 command
classes and allow the tested legitimate commands, that the off-home layout was
migrated non-destructively, and that the Director conventions are now
repo-backed. The guardrail set is not claimed complete.
