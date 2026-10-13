# QSL project workflow

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Adopted by the operator for the entire project.

## Authority and scope

This is the single project-wide build method for the Director, Codex and any other assistant, across protocol, desktop, relay, attachments, website, design and supporting tools. Current human instructions govern. Accepted product contracts and requirements still define what must be built. This document defines how that work is scheduled and assessed; it supersedes earlier procedural rules, autopilot policies, standing-rule ceremonies and assistant workflows. Historical decisions keep their product meaning and evidence, but their old process requirements do not become current instructions.

Use one active task record maintained by the Director. In the managed build it is the current lane TASK named by the build-root instructions. Repository queue files are compatibility views and historical indexes, never independent launch authority. The roadmap holds product order; HARDENING.md holds deferred rigor. Neither starts work by itself. Copies of this workflow in component repositories must match the canonical build copy; repository instructions may add relevant technical constraints, not a competing method.

## Current operating mode

Manual mode. At most one issued/launchable task and one executor at a time; zero is valid at a quiet boundary. The operator launches each task. The Director verifies its response and exit, then prepares the next bounded task and manual command. No automatic launches, retries, successor starts, background agent work or new schedules. Read-only observation of an existing CI run is not a retry or task launch. Existing CI and separately authorized host maintenance are not authority to run an assistant or enable another timer.

Director: GPT 6.1 Sol / High. Codex: GPT 6.1 Sol / Medium. Normal speed; no /fast. Fresh executor sessions, memories off and subagents off. Pushover is USER_PAUSED: no enqueue, send, flush or retry. Resume or mode/model changes require a later human instruction. Tool capabilities or old configuration defaults do not supply that instruction.

## Lane 1: connected working slices

Keep the accepted roadmap. Choose the smallest coherent change that advances an actual product behavior or removes a demonstrated blocker. A slice includes its real caller, necessary error/refusal behavior and affected fixtures. Avoid frameworks, general tooling, speculative future cases and repeated broad reviews unless a concrete requirement or failure justifies them.

Before issue, the Director names the roadmap card, user-visible purpose, exact source base and allowed paths, settled contracts, immediate checks, necessary independent review, deferred checks with ledger IDs, time limit, closeout reserve and exit condition. Read the full relevant current instructions, contracts and source; retrieve exact cited evidence before making a finding. Do not require the whole historical archive at every startup. Resolve structural identity, format, authentication, ownership and capacity interfaces before dependent code relies on them; a mock cannot quietly choose an unresolved production contract.

Implement in an isolated branch/worktree, preserve unrelated work, run the smallest meaningful changed-boundary checks, review the complete changed sensitive interface and record the result. For a bug, use a real caller regression that distinguishes the defect from setup failure. Add tests for new behavior and risk, not tests that merely mirror a low-impact prose edit. Reuse unchanged evidence when its exact source/configuration and claim scope remain valid. CI and branch-protection checks remain enforced; do not disable them or manufacture green results.

Put confirmed changes in the relevant source repository and push the reviewed branch after its applicable validation and publication checks. Open a reviewable PR when appropriate. Local candidates, reports and branch pushes are separate facts; a push is not a merge. The human operator alone merges, deploys and authorizes privileged host changes. Preserve published history; no force push, destructive reset or silent loss of dirty work. Use merge commits under existing repository policy.

## The safety core is due now

Never defer correctness needed by the behavior being implemented: authentication before trust, secure key/randomness handling, nonce ownership, strict bounded untrusted input, safe vault writes/recovery, commit before external release, exact retry of committed operations, fail-closed critical errors and truthful user-visible security/delivery state. A known violation blocks the affected behavior. Missing acceptance evidence is recorded as missing, never PASS.

Changed cryptography, authentication, keys, persisted security formats, ownership/durability boundaries and their security assertions need independent sensitive review before dependent work uses the changed foundation. Author self-review does not satisfy it. Keep accepted interfaces and numerical requirements; this method does not lower workloads, increase caps, introduce fallback or approve a new trust assumption.

Broader assurance can be staged: exhaustive stress/soak, whole-platform matrices, formal proof expansion, release packaging, external audit preparation and stronger optional test-apparatus guarantees. Defer only if the current slice remains safe within a stated development context and the omitted proof is not claimed. Required CI continues to run. A known reachable security defect cannot be relabelled deferred qualification to cross a safety gate.

## Lane 2: one hardening ledger

HARDENING.md is the one ledger for every shortcut as it is taken. Use a stable ID and exactly three substantive fields: **Cut/deferred**, **Risk**, **Mandatory trigger**. The first field includes the exact omitted work, affected requirement/source and responsible roadmap card. The trigger names an observable event and the checks/review that must be completed before crossing it; "later", "when time permits" and "before the end" are invalid.

The ledger has no active executor, launch command or separate queue. At issue, review and any proposed milestone, merge, data-use, external-review, deployment or release boundary, the Director checks which triggers are reached. A triggered item is brought into the same single active task queue; the triggering boundary stays blocked until its required evidence is accepted. A trigger can be earlier than release. If a deferral depends on synthetic data, enforce that restriction until its real-data trigger is closed. Revisions to a trigger record authority and reason; do not move it silently to escape a gate.

Retain closed entries with an evidence reference in their fields. Existing owed records and historical findings are source inventories, not additional active hardening ledgers. Import actual applicable debt without assuming historical "open" means a current defect or historical "closed" proves current correctness. Unresolved legacy findings must be reconciled before the affected foundation is used, its external review, or its release claim. Do not forget them or restart an entire historical audit before unrelated coding.

## Progress and acceptance stages

**IMPLEMENTED** means source exists; decisive checks may still be missing. **SLICE_VERIFIED** means the stated development behavior works through actual callers, immediate tests pass and needed sensitive review is accepted. **CORE_READY** means its safety-critical contracts and tests support the exact dependent use; remaining broader qualification is named in the ledger. **QUALIFIED** means the complete accepted card/milestone obligations, including triggered debt, are satisfied. **RELEASE_READY** additionally requires the final supported-platform, formal, external-review, service, packaging and human gates applicable to the intended release.

Dependent candidate development may proceed on a specifically reviewed CORE_READY foundation without waiting for unrelated final qualification. It may not rely on an unsettled contract, unsafe foundation or unproved bound needed for that use. Existing "ACCEPTED" card exits mean QUALIFIED unless explicitly recorded as a narrower investigation or design acceptance. A demonstration, test exit, report, PR, merge and release are never interchangeable.

## Finite tasks and honest recovery

Design supporting tools for the actual workload and stated fault model. Use an adequate bounded supervisor for potentially nonterminating administrative checks, trustworthy subject exits, source/artifact identity, raw evidence custody and privacy. Do not demand universal corruption resistance or arbitrary exception-graph accounting as a prerequisite to product coding unless that guarantee is actually required and justified. Missing stronger apparatus proof remains unclaimed and enters the ledger if deferred. Changing an approved runtime method's guarantee requires an explicit recorded assessment; this workflow is not permission to silently alter an issued probe.

An understood, reversible in-scope preparation or implementation defect may be corrected within the same task and budget, with original failure evidence retained and affected checks rerun. No automatic retry of a consumed task, probe, CI job or unknown-effect operation. A terminal stop returns to the Director: recover the result, identify the cause, propose a corrected scope or method, and let the operator launch a fresh task if one is needed. Repeated failures of the same method require a whole-method assessment, not another symptom-only assignment. Unknown effects, lost integrity, unscoped/privileged acts and genuine design decisions stop affected work before the action.

Leave time to save the report and establish process completion. Confirm the exact executor/session and owned administrative children have exited; report unknown numeric exit codes as unknown. Never clean up by a broad process-name kill. Preserve issued inputs, failures, receipts and sealed evidence; corrections are new records. Install prospective instruction changes only at a verified quiet boundary. Do not rewrite old input manifests or reseal old reports to fit new rules.

## One durable record and one explanation

At meaningful boundaries update the single TASK with authority, phase/stage, exact branch/source, last verified result/evidence, unresolved findings, ledger IDs and next manual action. The task report carries its relevant proof; references to exact immutable artifacts are adequate. Do not copy entire histories into each directive or stop, mandate duplicate journal/relay/prediction records, split routine work into promotion/implementation/closeout PRs, or require a new approval for an already authorized routine act.

When issuing or receiving work, explain in plain language: which part of the plan, what problem, what changed or will change, evidence and limits, and the next concrete step. Completion notices are reports awaiting verification. Never promise a finish date without measured support.

Keep credentials, keys, private raw logs and host topology out of public repositories/chat. Preserve existing secret/publication scanners, scope guards, hooks and meaningful CI. Use the configured GitHub remote, verify publication identity, and state where each artifact is actually saved. A local file or platter copy is not off-host durability. Recovery must restore this workflow, the single TASK and HARDENING.md together, then reconcile live source and consumed tasks before resuming.
