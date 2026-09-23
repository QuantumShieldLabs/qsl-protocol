# QSL: a buildable successor plan

**Original review attestation — 23 September 2026.** Author: AI assistant; the exact underlying model identifier was not independently available. **Prior context: yes**—the supplied packet and earlier discussion; this was not a blind review. Packet `05_code/true-delta.diff` SHA-256: **`ca9f2b0494448f14cbe91f0fd822d04abffc6b3fdbcea6a5b5bd5ec4317e0211`**. All **60** packet manifest entries verified successfully. Initial packet reading followed brief → director state → reviews → NA-0783 escalations → suite failures → code; subsequent design work inspected the pinned repository revisions and rechecked relevant packet passages. Specialist reviews informed the synthesis. Revision 3 adds a further technical, execution and evidence review; it does not represent implementation or new product-test results.

## Model-neutral execution guide — revision 3

**Start here.** This section turns the design below into an execution contract for any authorized implementer; it prescribes no AI model, provider or runtime. It replaces the earlier milestone table as the work sequence; Q1–Q5 and the source index remain the rationale. It reduces discretion and makes drift observable. No document can guarantee that an implementer will never drift. Literal source filenames and repository instructions are retained as evidence, not as a choice of implementing system.

**Current implementation status: NOT STARTED by this planning exercise.** No card, proposal or acceptance test below is marked completed merely because it appears in this document. Historical passing results retain their original revision and scope.

### Start or resume in this order

1. Read this execution guide and the existing execution record, if one exists. On the first run, begin **F00**. Do not start by fixing the easiest red test or implementing the GUI.
2. Inspect the actual workspace and current repository instructions, including applicable `AGENTS.md`/`CLAUDE.md`, goals, charter, active directive, standing rules and authoritative work queue. The reviewed commit pins below are historical evidence, **not reset targets**. Preserve dirty work and existing branches; never overwrite them to reproduce this plan's baseline.
3. Record the actual branches/heads and differences from the reviewed pins. Identify the active implementation authorization, allowed repositories/paths and required reviews. This planning document does not grant permission to merge, deploy, administer hosts, discard work or bypass governing rules.
4. Choose the first eligible card whose prerequisites are ACCEPTED. Translate it into one bounded active assignment with exact paths, expected result and commands. Read that card's relevant design section and live source before edits. A proposed filename or API in this plan is not proof that it already exists.
5. Complete the assignment, collect evidence, obtain the review that actually applies, and update the existing record. Continue to the next authorized assignment without asking the operator to reconfirm routine work. If authorization or review is genuinely missing, identify the exact governing requirement and prepare the concrete material needed to resolve it.

Read the complete architecture once. On later sessions, reread this guide, the current card, its referenced contracts and changed source; do not repeatedly reread every historical artifact. If a required fact is missing, retrieve it rather than guessing from a previous summary.

### Authority, fixed choices and permitted judgment

Current user direction and applicable project authority govern implementation. This section controls interpretation of this plan; it cannot silently amend canonical protocol requirements or standing rules. A conflict must be named and resolved by the appropriate authority. Do not edit a goal, test expectation or instruction file just to remove the conflict.

These are the selected design directions. Reopening one requires a concrete counterexample or new requirement, an impact statement and the applicable design review—not an executor's preference:

| ID | Selected direction |
|---|---|
| D01 | Preserve the directional core and repair integration; preserve historical #1831 evidence. Use extraction only if documented reconciliation cannot produce a reviewable integration. No automatic legacy crypto fallback. |
| D02 | One fresh successor profile with authenticated version separation. Preserve/refuse incompatible old development vaults; no silent reset or migration. |
| D03 | One authoritative paired owner/session transaction; one mailbox dispatcher; existing GUI/gateway reused. No new general database, second inbox consumer or JavaScript crypto implementation. |
| D04 | Invitations and recovery precede exposed messaging. Real verified-contact text is an engineering milestone; attachment obligations remain visible until satisfied. |
| D05 | Preserve inline arbitrary content of 1..4,194,304 bytes. First file acceptance includes at least two active sessions and separately admitted incoming/outgoing maximum transfers. No silent service fallback or reduced-size substitute. |
| D06 | Security/user properties survive test migration. Obsolete wire/timing assumptions may change only with an explicit property-to-replacement mapping. |
| D07 | Queued, Prepared, relay acceptance, authenticated delivery and file publication confirmation remain different states. No promise of remote capacity from a local reservation. |
| D08 | Existing accepted obligations keep their funding. Expiry/cancellation cannot erase committed protocol debt. Full old-backup rollback remains an explicit release requirement. |

The executor may choose names, internal factoring, efficient test fixtures and other routine details within an accepted contract and declared scope. The executor must not alter a wire interpretation, trust assumption, resource promise, admission rule or user-visible meaning as an incidental refactor.

### Contract decisions that must be resolved before dependent code

The mechanisms below are **proposals needing exact contracts**, not already reviewed protocol law. Track each as OPEN, ACCEPTED, REJECTED or SUPERSEDED in the existing decision record. ACCEPTED means exact meaning is recorded and the review/authority required by live governance is satisfied. Do not create another approval ceremony where none is required.

| ID | Required decision output | Resolve before |
|---|---|---|
| C01 — versions and boundaries | Allocated profile, wire-kind and local-schema identifiers; compatibility/refusal matrix; owning modules; reserved unsupported file kinds. “v2” and “NIF1” in this plan are descriptive until checked against the registry. | F03 and all new wire/state implementation |
| C02 — invitation authentication | Exact canonical signed bytes, domain separation, algorithms from the existing suite, identity/route/attempt binding, length limits, replay rules, deterministic crossing order and first-contact trust behavior. Positive and mutation vectors specified. | F10 |
| C03 — relay authority and recovery | Exact read/deposit derivation and namespaces including invite slots; endpoint/request/response schema; request identity and secret handling; transactional first claim; retry/conflict/expiry/revoke order; fixed recovery horizon; result quotas and cleanup. | F08–F10 |
| C04 — ownership and accounting | Dimensions and owners of each reservation; maximum serializer inputs; admission versus reopen validation; global queue peak/temp/terminal rules; control-progress and retirement rules; limits derived or explicitly still hypotheses. | F04–F07, with proof completed in F06 |
| C05 — dispatcher and GUI contract | Per-item dispositions and ACK rules; pending SID/epoch and temporary admission-capacity deferral; finite request/byte/time/fairness budgets; DTO/status meanings; action-ID scope/retention; text limits, history policy, unverified-send policy and lock cancellation. | F11 and F13 |
| C06 — files | Exact binary fields, endian/length rules, NIF/file-row versions and AAD; semantic FileIntent versus final manifest; FileJob transitions; nonce ownership; full bulk-spool reservation versus two in-flight slots; durable publication intent, ownership and no-replace rules; cancellation/deadline/drain; combined resource target. | F14 before file serialization, FileJob persistence or enablement |
| C07 — rollback and durability | Covered restore domain and platform support; independent anchor provider or recorded unresolved blocker; trust/offline implications; commit/fencing/recovery contract; process-crash versus power-loss claims. | F02 decision, affected persistence implementation and F18 release |

**Resolve the earlier timing ambiguity:** F01 fixes profile identity, extension boundaries, ownership interfaces and reservation dimensions. File kinds remain reserved and unsupported. F14 settles/reviews the remaining exact file bytes and proves their bounds before any file-format implementation. If an unresolved file choice would change an earlier persisted interface, settle that choice earlier; do not invent a temporary format or silently allocate a second successor profile.

Rollback feasibility is early; implemented rollback protection is later. F02 may finish with a well-evidenced OPEN C07 and a BLOCKED release. Independent decoding, queue, invitation and GUI work may continue only where it does not depend on the missing anchor contract and current work-queue rules permit it. No dummy provider returning success; no assumption that an OS key store is monotonic. If selecting an anchor changes a persistence interface, its dependent work is blocked until that interface is settled.

### Numerical requirements versus hypotheses

| Value | Status and permitted treatment |
|---|---|
| Inline 1..4,194,304 bytes; at least two active sessions; one incoming plus one outgoing transfer | Required acceptance workload for this selected design. Do not reduce it to obtain green tests. |
| 16,384-byte chunks; at most 256 chunks; two reusable in-flight protocol/queue slots per job | Selected file design, to be validated in C06/F14. Full retained bulk content is reserved separately; reusing a protocol slot does not discard needed file bytes. Any replacement needs an explicit equivalent-bound/progress argument and review. |
| 16,777,216-byte vault aggregate | Existing candidate ceiling to preserve during proof. Raising it is not a repair for unbounded state. |
| 36 retained controls / 36,775 bytes | Unproved candidate bound. F06 must supply transition and actual-serialization evidence; a constant or passing scenario does not prove it. |
| 64 queue slots / 4-MiB queue / 60,000-byte complete file row | Candidate limits to freeze with exact accounting in C04/C06. Do not confuse raw payload, JSON expansion, encrypted row and replacement temporary. |
| Eight text sessions | Optional feasibility/optimization target, not a shipping promise or concurrent files-plus-eight-sessions guarantee. |
| 24-hour file horizon | Proposed development policy, OPEN until C06 accepts it. Never extend it on retry or use it to delete committed protocol debt. |

### Invariants every affected card must preserve

| ID | Checkable invariant |
|---|---|
| I01 | Authenticated successor separation; no silent downgrade, old-state reinterpretation or crypto fallback. |
| I02 | Correct identity/route authentication before trusted handshake or FileJob admission; first-contact possession is not human verification. |
| I03 | The applicable durable authority commits before external output or claimed success: directional transitions use fresh paired owner/session authority; invitations, relay operations and initial queue intents use their declared capsule/transaction/intent authority. Interrupted authoritative writes recover old-complete or new-complete state. |
| I04 | One immutable operation identity/content/intent; committed retry bytes identical; no second committed seal for that operation. |
| I05 | Actual encoded costs plus remaining promises plus headroom fit; ordinary work cannot consume already owed recovery/control credit. Local funding never implies remote admission. |
| I06 | Reads, parsing, collections, queue lifetime and temporary storage bounded before unsafe allocation/growth. A valid maximum committed state remains reopenable. |
| I07 | One mailbox owner; disposition governs each ACK. Deferred prerequisite, temporary admission-capacity refusal or local failure is not permanent invalidity. No acceptance receipt precedes the corresponding durable admission. Invalid traffic allocates no mandatory durable rejection history. |
| I08 | Exclusive root-transition ownership and authenticated parent binding; invalid input does not mutate trusted state; retained epochs/windows stay bounded and finite traffic reaches conditional quiescence. |
| I09 | File manifest alone creates a job; chunks start after manifest NDR1; durable accepted bytes precede receipts; verified publication precedes FILE_COMPLETE. |
| I10 | File staging is funded before bulk I/O; unique nonce/AAD ownership; immutable snapshot; no original-file reread under an accepted operation. |
| I11 | Cancellation does not resurrect jobs, retract released bytes, erase committed flights or emit completion for an unpublished file; retirement remains bounded and funded. |
| I12 | GUI states follow durable facts; identities never silently replaced; lock hides plaintext and stale work cannot repaint or start another secret operation. |
| I13 | Shipping artifacts exclude effective test hooks; exact build/platform/service provenance and honest executed counts support claims. |
| I14 | Complete-backup rollback and cryptographic recovery claims are no stronger than the demonstrated anchor, model and compromise assumptions. |

### Work cards and dependency order

Source shorthand: **P** = qsl-protocol; **Q** = `P/qsl/qsl-client/qsc`; **D** = qsl-desktop; **S** = qsl-server. Paths below identify primary areas, not permission to edit an entire repository. Each active assignment lists actual allowed files plus the directly required tests, vectors and canonical documentation. New module/API names are proposals. Read the corresponding numbered “concrete solution” section below and cited live files.

Default order: **F00 → F01 → F02 → F03 → F04 → F05 → F06 → F07 → F08 → F09 → F10 → F11 → F12 → F13 → F14 → F15 → F16 → F17 → F18**. A completed F02 investigation is sufficient to continue independent engineering even if C07 remains OPEN, under the rules above. File execution stays off through F14.

Cards are acceptance units, not claims of two-hour completion. Split a large card into named subassignments of roughly 1–2 active hours with one outcome each; retain all parent acceptance obligations. A timebox ending means checkpoint, not PASS. Do not split an atomic invariant merely to meet a clock.

**F00 — Reconcile authority and workspace. Prerequisite: none.**

- Read actual governing files, active lane, approvals, branches, dirty work, toolchain and repo access. Compare current heads with the historical pins. Identify existing tasks/evidence that already address this plan.
- Output one intake entry in the existing work record: actual workspace, allowed scope, chosen integration base, preserved work, relevant changes, current blockers and next assignment. No product edits in this card.
- Accept when facts are verified and the next bounded work scope is authorized. Missing repo access or contradictory authority blocks its affected work; do not reconstruct or invent private paths/permissions. Review: routine/director.

**F01 — Fix the successor contracts and requirement inventory. Prerequisite: F00.**

- Record D01–D08; settle C01 and exact contracts C02–C05 needed by upcoming work. Define C06 ownership/interface boundary now; reserve its unsupported formats. Allocate IDs through the existing registry. List mandatory user/security properties and feature gates.
- Output compact canonical tables/vectors/decision references, not another parallel specification. Mark unresolved cells OPEN and name their blocked cards. Preserve existing required governance updates.
- Accept when the interface contracts needed for F03–F11 are reviewed and unambiguous; C06 detail and C07 may remain explicitly open. No production “placeholder” format. Review: independent sensitive design review where required.

**F02 — Resolve rollback feasibility early. Prerequisites: F00 and the relevant accepted F01 boundary decisions.**

- Read current persistence canon. Specify supported rollback domains and platform assumptions; inspect/test a candidate independent anchor without editing production persistence. Distinguish vault-only rollback, whole-machine restore, crash recovery and power loss.
- Output either a feasible reviewed C07 with prototype evidence, or a bounded negative finding naming what is missing and which cards/claims it blocks. Do not silently add a trusted remote service or reduce the threat model.
- Accept the investigation when its conclusion is evidenced. This is never acceptance of implemented rollback protection. Release remains blocked until F18 proves it. Review: independent security/platform review.

**F03 — Reconcile integration and repair representative fixtures. Prerequisites: F01, completed F02 investigation.**

- Scope: isolated P integration branch, exact main drift/dependencies/workflows, `Q/tests/common`, representative suite families. Preserve original candidate commits. Create explicit ordinary-vault/successor-vault/real-pair helpers without changing every fixture's meaning by stealth.
- Instrument: one real-handshake case per failure family, negative profile refusal and a meaningful crossed-send regression. Begin the complete old-test → property → replacement map; refresh failures at the actual integration head.
- These fixtures exercise the existing/reconciled directional candidate and its implemented profile, not the future envelope/relay contract. Record final-envelope/v2 cases NOT_RUN until F08–F10; do not implement a placeholder wire format to make baseline setup pass.
- Accept when implemented baseline properties are exercised, failures are classified honestly and CI can execute this head. Refresh the same property fixtures against the final successor in F10/F11. Do not use absence of CI or the 191 historical count as today's result. Review: integration; sensitive conflict resolutions get the applicable review.

**F04 — Make persisted-state decoding and reads strict. Prerequisite: F03 and C04.**

- Scope: `Q/src/vault`, `protocol_state`, `directional_delivery`, `msgqueue` readers; separate bounded assignments for required fields/duplicate keys, pre-allocation limits and semantic state validation.
- Instrument: missing field, duplicate key, inconsistent owner/session/generation, truncation, cap+1 and maximum-valid reopen. Refusal must precede external release and preserve prior trusted state.
- Accept when all real entry paths covered by the claim use the checked contract, not just a new helper. Preserve I01/I03/I06. Review: independent sensitive implementation review.

**F05 — Close commit ordering and secret lifetime gaps. Prerequisite: F04.**

- Scope: vault paired commit/projection/create writers, `protocol_state` update paths, core/epoch/runtime key ownership and actual `fs_store` persistence helpers. Map every external release to its authorizing commit.
- Instrument: injected failure/process cut at each changed commit boundary; old-or-new complete state; no wire/plaintext/receipt before commit; exact retry after restart; cleanup on error, success, clone and epoch retirement.
- Accept when actual writer semantics support the stated durability claim and I03/I04 hold. Do not label process-cut tests power-loss proof. Review: independent sensitive review.

**F06 — Prove session and control capacity. Prerequisites: F05, C04.**

- Scope: owner/reservation and directional control transitions; actual serializer calculator; bounded schedule model. First derive count/state bounds, then byte bounds, then replay counterexamples through production callers.
- Instrument: at least two peers; ordinary saturation; delayed old-epoch frames; duplicates/lost receipts; exact closure carrier; valid-capacity reopen; finite-input quiescence. Include every writer that spends shared capacity.
- Accept only with the transition argument, encoded bound and discriminating runtime evidence. A guessed 36-control limit or hard refusal at the 37th control is insufficient if a legitimate transition requires it. Preserve I05/I08. Review: independent sensitive review.

**F07 — Bound the entire queue lifecycle. Prerequisite: F06.**

- Scope: `msgqueue` enqueue, pack/projection, retry and terminal retirement; related owner accounting. Separate admission/peak-temp accounting from terminal cleanup while preserving one authority and lock discipline.
- Instrument: global slot and byte over-admission; post-pack growth; interrupted replacement; sustained drain beyond 64 lifetime messages; two-peer fairness; ordinary fullness while owed controls drain; history-full truthful refusal.
- Accept when peak/lifetime accounting holds and terminal retirement cannot remove an exact-retry obligation. Queued may wait for protocol capacity. Preserve I04–I06. Review: independent sensitive review.

**F08 — Isolate v2 relay capabilities. Prerequisites: F01/C03; default sequence follows F07.**

- Scope: S route resolution, endpoints and transactional schema migration. Include invitation slots and ticket-gated A1; push, leased pull and ACK resolve one canonical authority with distinct permissions.
- Instrument: deposit-only cannot pull/ACK; all v1 representations fail to address v2 storage; alternate representation cannot bypass ticket; lease/restart/schema downgrade behavior.
- Accept when permission separation and namespace isolation are demonstrated, with no delete-on-pull/fallback. Review: independent security review.

**F09 — Make relay operations recoverable. Prerequisite: F08 and C03.**

- Scope: S redemption winning request/result, fixed recovery deadline, initial A1 receipt and bounded cleanup. Exact accepted-result lookup precedes new-admission checks.
- Instrument: lost response after commit; competing claims; changed request under same identity; exact retry after ACK, full queue, rate pressure and original invite expiry; revoke boundaries; restart and receipt expiry.
- Accept when one effect has repeatable authenticated proof and quota/retention are bounded. A second consume or fresh ticket is not recovery. Review: independent sensitive review.

**F10 — Finish client invitations and crossing. Prerequisites: F05, F09, C02/C03.**

- Scope: `Q/src/invite`, `handshake`, facade adapter and relevant #1825/#1828 tests. Natural assignments: signed envelope; durable redeem/cache-before-reserve recovery; exact reply/winner lifecycle. Reuse reviewed behavior selectively.
- Instrument: own invite rejected before network; counterfeit-before-real pinned A1; route/profile mutation; both crossing orders and delayed A2; lost redeem/A1 response; cache/reservation/reply cut points; advanced same-SID recovery.
- Accept when authentication precedes occupancy, deterministic identity ordering selects one winner and only the exact owned input is ACKed. Preserve I01–I04. Review: independent handshake/security review before downstream reliance.

**F11 — Install the single bounded dispatcher. Prerequisites: F07, F10, C05.**

- Scope: Q transport/mailbox driver, frame-class consumers, deadline/body caps and typed dispositions. Inventory and route all old receive/finish callsites through the owner; no per-contact competing pulls.
- Instrument: mixed invite/handshake/control/text batches; invalid-only batch before valid data; A2/boundary behind more than one batch of dependent data; recipient full then capacity freed then exact retry; local failure; lost ACK; 429; finite interference and continuation fairness. While new work is deferred, already owed controls must still progress; no premature ACK or acceptance NDR1.
- Accept when each ACK has the correct disposition, known pending work is deferred and untrusted traffic cannot grow mandatory durable state. Preserve I02/I06/I07. Review: sensitive integration review.

**F12 — Complete nonfile property migration and shipping-hook isolation. Prerequisite: F11; mapping began in F03.**

- Scope: Q tests, manifests/CI and actual feature/env hook paths. Replace legacy fixture assumptions by family; turn retained-state experiments into independent setup/cut/restart cases. Keep required attachment properties explicitly BLOCKED.
- Instrument: every retained nonfile engine property due through F12 has executed coverage; targeted mutation controls for high-consequence assertions; case-level census; no zero-test/filtered pass substitution; shipping build ignores legacy clock and other hook seams. Keep later GUI, model, anchor and platform obligations separately NOT_RUN/BLOCKED with their owning cards.
- Accept as the engine milestone defined here only; all deferrals must already appear in the requirement inventory, not be invented during acceptance. No blanket ignore, dropped negative assertion or marketing the one-test smoke as the whole suite. Preserve I13. Review: assertion migration review and independent review of changed security tests/guards.

**F13 — Deliver typed desktop text and verified-contact acceptance. Prerequisites: F11, F12, C05.**

- Scope in order: Q submit/query/progress/verification facade; D exact qsc pin and Tauri DTOs; existing gateway/timer adapter; Chats UI and lock generation. Each is a bounded assignment; UI contains no crypto or protocol inference.
- Instrument: real two-device invite/verify/crossed text; offline/restart exactly one visible copy; duplicate IPC action/conflicting body; truthful delivery; actual delayed receipt; slow relay plus lock and stale response. Add at least two active peer sessions for capacity/fairness acceptance.
- Accept when the actual app performs the workflow on the pinned engine/relay and required nonfile checks pass. Formal-model, rollback and remaining release obligations may remain blocked and named; the F06 control-bound/progress acceptance is already required and cannot be deferred here. Preserve I07/I12/I13. Review: integration plus actual operator GUI acceptance required by governance.

**F14 — Freeze file contracts and prove representations. Prerequisites: F06, F07, F13.**

- Scope: NIF codec, separately versioned binary file row, pure seal-selection interface, FileJob/StagePreparing schema and joint accounting model. The first, design-only subassignment may start with C06 OPEN and must finalize/review it. Implementation subassignments require C06 ACCEPTED; then implement pure codecs/accounting fixtures. File execution remains disabled.
- Instrument: 1, 16383, 16384, 16385, 4194303 and 4194304 bytes; all-zero/all-255/random chunks; maximum metadata and malformed lengths; complete encrypted row; two sessions plus maximum incoming/outgoing jobs, full retained bulk spools, text, publication temp, drain and control liabilities. Demonstrate that protocol-slot reuse preserves all still-needed chunk bytes.
- Accept when exact bytes/identities and joint peak bounds fit the accepted caps. No JSON-expansion shortcut, silent cap increase or reduced concurrency. Preserve I05/I06/I09/I10. Review: independent file/state/security design and implementation review.

**F15 — Implement funded staging and the sender. Prerequisite: F14.**

- Scope: StagePreparing reservation, encrypted spool/snapshot module, FileJob promotion, semantic FileIntent, selected one-time seal, exact-flight reuse and sender pump. Reserve before bulk I/O; promote the same ticket.
- Instrument: concurrent starts; source growth/shrink/change/delete; tampered/cross-job spool; disk/write failure; cuts before/after reservation, snapshot sync, promotion, prepare commit and queue projection; identical retry bytes after settings change.
- Accept when no bulk temporary is uncharged, original data is never reread after acceptance and every crash state has one owner/recovery action. Preserve I03–I06/I10. Review: independent sensitive review.

**F16 — Implement receiver, publication, cancellation and retirement. Prerequisite: F15.**

- Scope as separate assignments: authenticated manifest admission; durable chunk acceptance; durable publication intent and no-replace publication/READY completion; terminal cleanup and late-frame drain. Keep the complete state-transition table and ownership transfer in C06.
- Instrument: reorder/duplicate/corrupt chunks; receiver admission full then freed; publication destination collision including unrelated identical content; cuts around chunk sync, owner commit, publication intent, publish, READY and cleanup; cancellation before/after seal/publication; expiry; late valid chunks; absent peer and explicit session closure.
- Accept when completion follows publication, no late frame resurrects a job, and cleanup never deletes committed debt. Published output is not reclaimed staging space. Preserve I03/I05/I09–I11. Review: independent sensitive review.

**F17 — Accept the complete inline file workflow. Prerequisites: F13, F16.**

- Scope: production file pump/desktop adapter and targeted integration acceptance, then complete required file-property mapping. No new protocol design inside an acceptance repair.
- Instrument: small pilot first, then real 4-MiB arbitrary-content transfers; simultaneous one-in/one-out with at least two sessions and unrelated text; loss, restart, lock, ordinary saturation, cancellation and bounded retained state. Measure runtime instead of lowering the workload.
- Accept when the exact GUI/relay/engine path satisfies all required attachment properties and truthful statuses. No service requests or simulation substituted for the inline result. Review: independent evidence review and applicable operator acceptance.

**F18 — Complete security, platform and release qualification. Entry prerequisite: F17; each implementation subassignment also requires its applicable accepted contracts.**

- If C07 remains OPEN, start with a design-only resolution assignment using the F02 findings; it may proceed while C07 is OPEN. Resolve/review the provider, trust model and commit/recovery contract before anchor-dependent code. This resolution assignment may be brought forward after F02 by the authorized director if earlier persistence work needs it; do not silently reorder the work queue. If resolution is blocked, affected implementation/release stays BLOCKED while authorized independent qualification may proceed.
- Implement and review the accepted C07 anchor/fencing/recovery as bounded sensitive assignments if not already completed. F02's investigation acceptance is not anchor acceptance. Tie the required machine-checkable model/vectors to actual successor transitions and run it in CI. Qualification evidence is produced by this card, not an entry prerequisite that makes the card impossible to start.
- Instrument: complete covered-backup restore and concurrent/stale writer cases; supported-platform durability; hook-free shipping artifacts and resolved feature graph; full required-property inventory; real deployed service revisions; wrong-version refusal and end-to-end GUI workflow.
- Accept only after the required independent and operator decisions. Missing anchor/model/platform/security evidence means release BLOCKED even if text and files work. This card does not itself authorize merge or deployment. Preserve all invariants.

### Evidence, completion and bounded recovery

Use these card states: **NOT_STARTED → ELIGIBLE → ACTIVE → IMPLEMENTED → ACCEPTED**, with **BLOCKED** available from any unfinished state. ELIGIBLE means prerequisites for that assignment are satisfied, not that the project has granted a READY slot. The live one-READY/one-active-lane rules still apply. Only ACCEPTED prerequisites unlock dependent implementation. A contract-design assignment is explicitly allowed to resolve its own OPEN decision; it cannot perform dependent production edits until that decision is ACCEPTED. An investigation can be accepted as a finding while the security/product capability it investigated remains blocked; record both explicitly.

Before editing, add one short active-assignment entry: card/subassignment ID, objective, exact allowed files, invariants, expected failing behavior, planned discriminating checks, review type and next step. This is a section of the existing task record, not a new handoff packet.

A card is ACCEPTED only when:

1. Its required outcome and applicable contract are implemented or evidenced, with no unexplained out-of-scope diff.
2. Decisive checks actually ran against the recorded revision/configuration. For a defect, the regression fails for the intended reason before repair or under a targeted mutation—not a setup/import failure. New feature refusal on the baseline is not by itself proof of the whole feature contract.
3. Relevant security/user properties remain mapped, unresolved requirements retain NOT_RUN/BLOCKED status, and required reviews/CI/operator checks are recorded. A test timeout or filtered/ignored case is not a pass.
4. The record names the next eligible assignment and any remaining limitations. IMPLEMENTED alone never means accepted, merged or released.

Each evidence row records: **card + invariant/property; source SHAs; build/artifact identity where relevant; platform/features/service revisions; exact command/filter; expected case count; actual executed/pass/fail/ignored/filtered counts; outcome; retained evidence path; reviewer/disposition**. For non-test evidence, record its exact method/input/result rather than inventing test counts. Keep private raw material in the approved evidence location; publish safe classes only.

Reuse evidence only when the relevant code/build identity and property scope justify it. A documentation-only change need not trigger a full suite, but a changed serializer, build feature or shared fixture invalidates affected evidence. Do not promote synthetic or hook-enabled results into shipping/real-relay proof.

On failure, first classify it. Follow the actual repository's bounded retry policy; do not loop until a flaky run turns green. Preserve the failing evidence and minimum reproducer. An understood in-scope defect permits a narrow repair and targeted rerun. A violated invariant, unclear cause, exhausted retry budget or required out-of-scope change blocks the affected work. State: **what failed, invariant/contract affected, observed evidence, smallest proposed change, impacted cards, required decision**. Investigate safely before asking the operator; do not stop at “tests failed.”

Parallel work is optional, not the default. Use it only under live queue/scope policy, with separate worktrees and explicit file ownership; never let two executors edit shared authority/state modules concurrently. While reviews/CI run, do useful authorized current-lane work or read-only analysis. Do not start another READY lane merely because it is independent on paper. If a blocker leaves other work possible, the authorized director may reassign scope through the existing process; an executor cannot silently reorder the work queue.

### One durable restart record

Reuse the existing lane TASK/status record and required journal. Do not create a second authoritative work queue, duplicate decision log or per-session handoff collection. Add only the missing fields below. The journal remains supporting memory; canonical decisions and the actual work queue retain authority.

```text
Plan: QSL-solution-plan.md, execution revision 3, actual file SHA-256
Authority: active user/directive; applicable rule locations; exact permitted scope
Workspace: repo -> path / branch / HEAD / worktree status
Accepted decisions: Cxx -> canonical reference + review/disposition
Card states: F00..F18 -> state + evidence reference; release blockers listed separately
Active assignment: card/sub-ID / one objective / allowed files / invariants
Last verified result: exact command or method / revision / observed outcome / evidence path
Uncommitted work: owned edits; unrelated work preserved; recovery instructions
Blockers: affected contract/invariant/cards; decision needed; safe remaining work
Next action: one concrete operation, required inputs, expected result and stop condition
```

Update at meaningful boundaries, after recovered failures and before ending a session. On resume, reconcile this record against actual git/files/evidence; a stale note cannot override source. An interrupted command remains unknown until its result is recovered or rerun appropriately. Never infer acceptance from a prior assistant's prose, reset uncommitted work, or restart completed tasks merely because context was compacted.

### First response expected from the executor

After intake, report concisely: verified repositories/heads; active authorization and preserved work; first card/subassignment; exact files and decisive checks; any actual blocking decision. Then continue the authorized work. Do not generate another project-wide plan or ask the operator to approve every routine step.

Suggested instruction to give an implementing agent with this document:

> Use the revision-3 execution guide in this plan. Start with F00 against the actual workspace, preserve existing work, and proceed through eligible tasks under the project's current authority. Keep one existing execution record. Preserve the listed invariants and require evidence before marking a card ACCEPTED. Resolve open contracts before dependent implementation. Ask me only when the applicable authority genuinely requires my decision; bring the concrete issue, evidence and recommendation.

## Design rationale and Q1–Q5

The sections below retain the technical reasoning and pinned evidence behind the execution guide. Their estimates are planning ranges; their hypotheses remain hypotheses until the corresponding card establishes them.


**Yes: there is a defensible solution and a practical route to it. Preserve the directional architecture, repair its integration, and finish the product around one durable protocol engine.** A plan cannot establish that unimplemented software already works. The acceptance conditions below make that claim decidable before release.

I revise my earlier suggestion to park the branch. The pinned source supports preserving its implementation more strongly than starting over. Main records a historical shared-root transition that permanently wedged both directions and includes an ACK-specific mitigation. The candidate's exclusive root-transition ownership and independent directional chains are a coherent response to competing transitions; that is a design inference, not proof that the historical failure remains reproducible on current main or that every crossed-send case is fixed. F03 must establish the actual baseline and successor behavior. The red suite is serious integration evidence; it does not establish that this architecture is wrong. [S1, S2, P1]

The proposed first useful result is two actual desktop clients establishing verified contacts and exchanging recoverable text through a real relay. The complete product milestone adds bounded inline files through 4 MiB. Public security claims remain subject to the protocol, rollback, platform and formal-verification gates; a text demonstration is not release clearance.

## Evidence and scope

Repository access was verified by remote reads and local git checkouts. These are the revisions reviewed:

| Repository / candidate | Exact revision |
|---|---|
| qsl-protocol main | `2112bba56c2869d942a44d263cbde7c997e3d88f` |
| qsl-protocol #1831 | `ffc8fc529374e62b00ae9018726ab313efe0f036` |
| qsl-protocol #1828 | `e29a07dfba0eb24a547dd77027e54df15bd7eb73` |
| qsl-desktop main | `15818498cdd0536f08a1b9909291ee5153168cfe` |
| qsl-server main | `5ea0f9256703b97dbc77e692dfe92244efd7a9a0` |
| qsl-attachments main | `8210d7fb25fafd4ec6f5a08ab6a37a063343abbb` |

No product code was changed, built, deployed or tested for this design exercise. The existing shard-inventory checks passed on main and the candidate; that establishes target coverage only. Suite outcomes below come from the verified packet. I could not inspect live qsl-ops/qsl-record or the operator's unpublished work. Proposed names and interfaces below are designs to implement, not existing capabilities.

Packet citations P1–P10 resolve to exact packet paths and lines in the source index. Live citations S1–S17 resolve to pinned repository files. Unmeasured estimates and architectural choices are explicitly proposals or inferences.

## Q1 — What the branch is actually doing

**It is a fresh, incompatible successor profile, not an additive runtime opt-in that leaves ordinary legacy messaging operational.** Selecting the profile at fresh-vault initialization is an opt-in; the integrated send/receive path does not fall back to legacy crypto. File execution is deliberately gated. The common test initializer also now selects the successor for callers that still construct legacy sessions. These are different facts and must not be conflated. [P2; S3]

The packet reconciles **571 passed, 191 failed, 2 ignored**, plus a separate directional smoke executing one test. The raw 192 failed-name lines include duplication; 191 is the reconciled number. Panic-string counts are not causal diagnoses. [P1]

| Failure family | Assessment | Evidence that decides it / required replacement |
|---|---|---|
| `send_semantics`, `ratchet_step`, relay/timeline fixtures using seeded old sessions | Largely incompatible setup; residual failures remain unclassified | Common initializer now creates `directional-v1`. Establish a real successor pair, rerun, preserve the original durability/transport/replay assertion. A setup refusal is not proof that the tested property failed. [P2; S3] |
| `attachment_streaming_na0197c`, `file_transfer_mvp`, attachment integrity/resume tests | Deliberate missing functionality | File kinds are refused by `body_decode`; unsupported-operation tests do not replace transfer-success tests. Keep those requirements visibly blocked until files work. [P2; P3] |
| Receipt policy/cadence and `desktop_gui_contract_na0215b` | Mixed contract changes and properties still requiring evidence | Old Off/Batched policy and old chain timing may be retired. Suppress actual authenticated receipts to test SENT; wrong-peer/message receipts must never yield DELIVERED. Earlier valid confirmation can legitimately change timing. [S3] |
| Handshake suite/parameter vectors | Intended profile change, with security obligations retained | Include the critical authenticated successor parameter in positive vectors and mutation producers; retain missing/duplicate/unknown-critical, signature, KEM, downgrade and transcript-negative tests. [S3] |
| Mixed frame dispatch and invitation tests | Potential real defects; not safely dismissed as old fixtures | Candidate foreign-frame branch fails to increment the counters that drive further pulls. Run real handshake/control/text in mixed batches and a foreign-only batch before honest data. [P2] |
| Queue limits, persisted-state validation, key cleanup, uncapped reads | Real missing or incomplete implementation | Enqueue has no hard global admission check; omitted `response_pending` defaults; runtime keys lack all-exit cleanup; several reads allocate before bounds. [P4; P5] |
| Unknown residual failures | Unresolved | One row per failed test: intended property, setup, successor test, actual result, disposition. No blanket skip and no conclusion from an error substring alone. |

Repair the fixture architecture before paying to rewrite hundreds of tests: explicit ordinary-vault, successor-vault and real successor-pair helpers; isolated child processes for environment/global-state tests. The directional target contains retained-state experiments and diagnostics as well as regressions. Convert desired crash scenarios into self-contained setup/cut/restart/assert cases and move exploratory tools out of the automatic pass count. Current CI's one-of-47 smoke remains useful, but it is not 47-case coverage. [S4]

## Q2 — Strategic choice

**Choose A with a clean integration boundary:** preserve #1831 unchanged as historical evidence, create an isolated successor integration branch, reconcile current main, and repair the candidate in reviewable changes. Preserve main's security and user properties. Replace obsolete wire-specific expectations deliberately. Do not preserve a second live crypto implementation merely to satisfy its old tests.

This matches the project's explicit pre-release preference for one mandatory strong format and versioned extensions. There is no demonstrated installed-base requirement justifying dual runtime modes. Old development vaults must be preserved and refused, with a clear fresh-profile workflow; do not reinterpret their keys or silently reset them. If an actual deployed compatibility commitment exists outside the available sources, that is a scope-changing fact to establish before implementation. [S5]

The following are **rough planning inferences**, not measured quotes. They compare reaching an accepted successor text engine, including reconciliation and test migration, but exclude new relay/GUI/file work, operator waiting and external cryptographic review.

| Option | Estimated active seat effort / reviewable PRs | Main risk | Early decision signal |
|---|---:|---|---|
| A: preserve and reconcile candidate — preferred | 40–90 hours / 8–15 | Coupled integration defects or misunderstood capacity invariants | Matched real-handshake fixtures, release-point audit and two-peer saturation tests in the first diagnostic increment |
| B: dormant core plus two active runtime paths during migration | 70–150 hours / 14–25 | Doubled policy, dispatch and state combinations; fallback/downgrade surface | Matrix expands before any new user workflow works |
| C: rewrite from design on main | 100–220 hours / 18–35 | Recreates solved retry/receipt/persistence bugs; evidence must be earned again | New code reaches the same invariant questions with less existing coverage |
| D: extract selected candidate modules into a fresh main-based integration | 50–110 hours / 10–18 | Lost integration details and misleadingly small initial diff | Use only if mechanical reconciliation cannot produce a comprehensible diff; compare exact retained behavior |

A bounded reconnaissance pass should decide A versus D. **Switch branch mechanics if needed; do not automatically discard the directional design.** An unrepairable ownership/liveness contradiction would justify abandoning that design. A stale workflow, obsolete test fixture or a large failure count does not.

For the wider solution, provision separate effort for invitation/relay/GUI integration and attachments; both are substantial workstreams. Re-estimate after the first working handshake/dispatcher and serializer-bound prototypes. A credible calendar estimate requires measured build/test time and available operator hours.

## The concrete solution

### 1. One versioned contract and one state authority

Fix the successor profile, authenticated invitation/mailbox contracts, local ownership interfaces and reserved file extensions in F01. Allocate identifiers through the existing registry. Finalize exact file bytes and persisted FileJob details in C06/F14 before those formats are written or enabled; settle earlier any field that affects an earlier persisted interface. Unsupported versions refuse before effects; no silent v1 fallback. File execution stays gated while text is accepted within the same declared successor boundary.

Keep the existing `CapacityOwner` and paired owner/session commit. Every accepted directional state transition uses fresh owner/peer generations, validates the complete invariant, commits one authenticated vault replacement, then releases its wire, plaintext projection or success status. Invitation capsules, relay transactions and initial queue intents use their separately declared authorities below. External queue/spool files have narrowly defined authority and recovery ordering; they do not become another independent protocol database. The candidate already provides the essential paired primitive. [S6]

| Durable authority | What it owns |
|---|---|
| Invitation operation / lifecycle capsule | Immutable attempt, cached verified result, exact replies, winner, recovery state |
| Paired capacity owner + peer transaction | Ratchet, exact sealed flights, received dispositions, receipts, liabilities, FileJobs |
| Encrypted queue row before preparation | Immutable user send intent and concrete padding choice |
| Encrypted file snapshot/spool | Bulk bytes referenced and authenticated by its authoritative FileJob |
| Timeline/conversation projection | User-visible history derived from committed operations; separately bounded |

At restart, a committed Flight wins over a stale queue projection. A missing projection is reconstructed from committed authority. Conflicting IDs, payload hashes or bindings refuse; retry never silently substitutes content or reseals an existing operation.

### 2. Finish invitations before exposing messaging

Keep main's existing own-invite preflight and handler enforcement. Selectively reuse #1828's exact saved replies, generation guards, authenticated winner selection, same-SID recovery and corrected ACK ownership. Use one outgoing and one authenticated responder candidate per identity generation; a canonical ordering of full identity commitments chooses the crossed-attempt winner, never aliases or arrival order. Commit winner selection and exact reply obligation before projections; recovery must not reset an advanced same-SID session. Do not treat #1828 as complete: cached redemption can still be stranded before outgoing reservation, and an unauthenticated first A1 can occupy the responder candidate. [S7]

**Authenticate before reserving handshake state.** Add a canonical, domain-separated signed outer envelope for A1/B1 using the established identity-signature suite. Bind the exact inner handshake bytes, identities, invitation/operation identity, selected profile and reply route capabilities. Verify lengths, canonical representation, signature and any existing identity pin before taking the provisional slot. Preserve the inner handshake's transcript/crypto rather than inventing another key exchange. For first contact, a valid self-presented key proves key possession, not a person's identity; the bearer invitation and later verification remain explicit trust steps.

**Make remote consumption recoverable.** Before redeeming, commit an immutable local operation containing the invite, local identity, chosen profile, fresh recovery secret and client-minted initial-push ticket. The relay atomically records the winning request digest, recovery-secret hash, ticket hash and bounded replayable result. Exact authenticated retry returns that result; changed data under the operation conflicts. Initial A1 delivery records the accepted exact-envelope digest and stable receipt, so retry after lost HTTP response—even after mailbox ACK—does not consume a second ticket or enqueue a second effect. Current consume-and-erase/burn-and-return behavior cannot supply this guarantee. [S8]

Set `recovery_until` once at first successful redemption, independent of the remaining fresh-invite lifetime and never extended by retry. Fresh expiry forbids new claimants; admitted operations can recover until their fixed deadline. Apply the same separation locally: resume a known operation before rejecting it merely because the original invitation has now expired. Revoke before first A1 acceptance prevents that effect; replaying proof of an already accepted effect does not deliver it again. After recovery expiry, expose an explicit failed/expired attempt and require a new invite.

Retained result bytes and receipts have count/byte quotas and cleanup deadlines. Authenticate and look up an already accepted operation before applying new-enqueue capacity, rate and fresh-invite-expiry checks: replaying a saved result consumes no new mailbox slot. Give result lookup its own bounded rate policy. Keeping the public bundle through recovery changes the present erasure policy; document that privacy tradeoff. The client acknowledges the result only after saving it durably. A server schema migration and downgrade refusal are part of this change.

### 3. One mailbox dispatcher; distinct deposit and read authority

The relay currently accepts the same route token for push, pull and ACK. A sender with that capability can read/delete mailbox ciphertext. Correct this at the same version boundary rather than layering GUI trust onto the old authorization model. [S9]

**Proposed capability construction for security review:** recipient holds random 32-byte secret R; deposit capability D is a domain-separated SHA-256 derivation of R. Push uses D; leased pull/ACK use R and derive D. Only D goes into authenticated route descriptors. Store v2 mailboxes in an explicit namespace unreachable by any v1 token representation. Test cross-version addressing; adding a hash domain without separating storage namespaces is insufficient. This restricts peer capability, not a malicious relay operator, who can still censor.

Apply this split to invitation slots too: the inviter retains the read secret, and the shared invitation grants only the intended redemption/deposit authority. Ticket-gated A1 and ordinary push must resolve to the same canonical slot authorization so another capability representation cannot bypass the ticket. Use lease-only v2 endpoints, explicit service capability checks and bounded response parsing/deadlines. One engine-owned dispatcher deduplicates mailbox work across invitations, handshake, controls and data. It processes all inspected frames within finite budgets and schedules continuations fairly. Remove competing per-contact pulls.

| Input disposition | Relay action / local behavior |
|---|---|
| Authenticated work durably committed | ACK; project from committed state; preserve owed receipt independently |
| Duplicate of committed work | ACK; return exact saved receipt where required; no duplicate application effect |
| Permanently invalid after full dispatch | ACK/dispose; bounded lossy diagnostic only; no durable rejection row required |
| Known pending attempt/session or permissible future epoch dependency | Do not ACK; continue past it to other leased items; retry later |
| Valid authenticated new work awaiting local admission capacity | Defer without relay ACK or acceptance NDR1; leave trusted receive state uncommitted, continue other bounded work including owed controls, and retry when capacity is available |
| Local store/lock/corruption failure | Do not ACK the affected item; report local fault, not malformed input or relay offline |

Do not allocate trusted-operation reservations from unauthenticated traffic. A durable record for every rejected frame would create a storage-exhaustion attack. Unknown SID lookup must cover both committed and pending handshakes; a real early message must have a durable predecessor obligation because B1 cannot leave before that obligation commits. No general quarantine database is needed. Test A2/boundary delayed behind more than one batch of dependent data, plus finite invalid interference before honest data.

### 4. Bound resources without stopping recovery

For each authoritative commit enforce:

`actual encoded state + all remaining promised allocations + write headroom <= declared budget`.

Measure the actual nested serializer, maximum scalar widths, identifiers and mutable fields. A source comment or a finite successful schedule is not a universal byte bound. Validate stored state at its allowed limit without incorrectly charging another new-admission margin merely to reopen it.

Keep these meanings distinct:

| State shown to user | Exact meaning |
|---|---|
| Queued | Immutable local request saved; it may be waiting for local protocol capacity |
| Prepared | Exact wire and local future liabilities committed; retries reuse those bytes |
| Sent / awaiting confirmation | Relay accepted the bytes; peer receipt has not been durably applied |
| Delivered | Exact authenticated peer receipt durably applied |
| File confirmed | Authenticated confirmation of verified publication, not merely frame delivery |

**Prepared funds the sender, not the receiver's history.** Delivery also requires recipient admission space, a running cooperative peer, eventual delivery within usable retention/retry conditions and writable storage. Once a receiver accepts work, its receipt/projection/closure obligations must be funded. Without an authenticated remote reason, the UI says “awaiting peer confirmation,” not an invented “recipient full.” A receiver-credit protocol would be a larger optional change; it is not required for honest initial semantics.

Replace the proposed initial-row-only queue bound with a lifecycle bound: global outstanding slots, each row's maximum encoded packed size, one bounded replacement temporary, and bounded terminal retention. The draft explicitly omits terminal rows, temporary space and post-pack growth; it cannot establish that broader claim. Count/write under the existing exclusive lock. Refuse before creating another accepted intent when the new peak reservation does not fit. Preserve the existing 64-slot/4-MiB candidate targets only if this exact accounting proves them feasible. [P6]

Retire terminal queue rows after their application projection and required protocol references are durably complete. Never delete a live exact-retry obligation to reclaim room. Store bounded tombstone/history information only where required; make history quota visible. Do not scan an unlimited lifetime of terminal rows on each send. History does not become free when a protocol flight retires.

Ordinary sends, incoming application projections and unrelated vault writers cannot take control/recovery credit. Ordinary queue limits also cannot consume the reserved queue/output slots needed for already owed controls or file completion. Separate unaccepted-work refusal from failures processing already accepted obligations. Key cleanup must cover every exit and cloned/retired epochs. Cap file and HTTP bytes before allocation/parse, enforce collection limits while decoding, require persisted fields and reject duplicate keys/inconsistent state rather than filling defaults. [P5; S6]

The candidate currently reserves **1,699,966 bytes per established session** against a **16-MiB aggregate**, before ordinary retained material; eight reserves plus global headroom leave about 2.65 MB. This is a derived feasibility warning, not a supported-contact promise. Require **at least two simultaneously active peers** in acceptance. Treat eight as a text-only optimization target, conditional on exact encoded state. Session creation must reserve capacity before announcing completion. File capacity and active-session capacity must be budgeted together; their independent maxima cannot be advertised as simultaneously available. [S10]

Prove the retained-control bound from enforced transitions: two receiving epochs, span 16, at most two useful advertisements per epoch, persistent witnesses, one maintenance flight and useful closure effects. The proposed 36 retained-control maximum must follow inductively, with serializer-derived byte cost. Then check conditional progress and eventual quiescence: after input stops and necessary traffic is delivered, controls stop originating without demanding an infinite receipt-of-receipt chain. Use reduced-state schedule exploration to find counterexamples and replay those through production callers. Retain a bounded proof residue where required. [S10; P7]

### 5. Reuse the GUI and add a thin messaging surface

Desktop already has onboarding, invitations, contacts and diagnostics. It pins an older qsc revision and has a timer that scans invitation work; neither rebuilding those surfaces nor adding a second message poller is appropriate. Its gateway serializes calls and currently holds the gate across network work, including work ahead of a vault-lock request. [S11]

Add typed engine APIs for submit, paginated conversation read, delivery snapshot and bounded `drive_step`. A stable client-action ID makes repeated IPC submissions idempotent; conflicting content refuses. Route selection, trust, message identity and crypto stay in qsc. The existing GUI timer invokes the unified step; the engine owns dispatch and recovery.

Add the minimal Chats thread/composer/status surface and a real compare-and-mark-verified action bound to current identity fingerprints. “Connected” must never mean “Verified.” Preserve the self-invite UI guard. The initial verified-contact acceptance compares fingerprints on an independent channel; product policy for unverified sending must be explicit, never disguised by a green connection badge.

Lock immediately hides sensitive UI and increments a cancellation/session generation outside the queued engine call. Backend work checks it before new secret/network operations; bounded requests let outstanding work yield. An atomic commit already underway completes safely; stale results cannot repaint or start another operation. Test stalled relay, lock, restart and delayed IPC completion. Deliberately advance the desktop qsc pin only to the exact candidate revision being tested.

### 6. Inline files through 4 MiB

Keep the inline requirement. Do not silently route it to the attachment service. The current service also has a lost-commit-response gap: it creates a fresh locator/capability, commits the object and removes the upload session before returning that capability. Service enablement later needs recoverable commit results or an explicitly different retry contract. [S12]

The candidate cannot simply lift its file gate: a 16,384-byte chunk of all `255` values occupies 65,537 bytes as a JSON array, before metadata, exceeding the 60,000-byte application cap. The local queue also JSON-encodes byte arrays, so changing only the wire format is insufficient. [S13]

Use the proposed versioned compact NIF binary payload inside the existing file kinds: manifest, chunk and completion; preserve core cipher/KDF/nonce and receipt formats. Use 16-KiB chunks, at most 256, with exact final-length arithmetic. Bind transfer identity, session/direction, initiating operation reference, total length, chunk count, per-chunk commitments and completion policy. A bounded display name is metadata, never a path. Reject trailing bytes and inconsistent lengths/indices. Freeze an explicit metadata cap and padding policy from actual serializer maxima.

Use a separately tagged **binary file queue row** inside existing authenticated-at-rest protection, with raw bounded byte fields. Keep text's format. Include format/type/record binding in AAD; bound the entire on-disk row, including tag, nonce, packed wire and mutable status. Prove the proposed 60,000-byte complete-row limit with maximum legal fixtures before relying on it.

Bulk content lives in an encrypted snapshot/spool, not timeline JSON. A compact FileJob in the existing owner holds the authenticated identity, key/reference, progress bitmap, fixed in-flight slots and credits. Reserve the full encrypted incoming bulk spool separately: accepted bytes remain in immutable job/index-addressed storage until publication or cancellation cleanup, even after the corresponding protocol slot retires. Use unique nonces with existing local encryption primitives. A restarted uncommitted snapshot uses a new job/key; it must not overwrite different plaintext under a reused nonce.

**Sender order:** commit a compact StagePreparing ticket, private temporary identity, outgoing slot and peak disk reservation **before bulk source I/O** → stream/sync the encrypted snapshot outside the global vault lock → verify generation and promote the same ticket to Ready → save immutable semantic FileIntent → select manifest epoch/slot without sealing, construct its self-reference, seal once and atomically commit core/Flight/owner → project packed row → release exact wire. The FileIntent freezes job/content identity and padding, not a nonexistent final reference-bound body. Construct that final body once during paired preparation and retain its exact hash/wire thereafter. A cut after the core commit finds the saved Flight; it never reseals. Interrupted staging is adopted only through its owning ticket or removed before releasing credits. Changing/deleting the original source after snapshot acceptance cannot alter the accepted file.

**Receiver order:** bounded preflight → authenticate and validate manifest without durable effects → atomically commit incoming reservation, FileJob, core/disposition and exact manifest receipt → release manifest receipt → for each authenticated chunk, write and sync its already-reserved immutable bulk-spool location → atomically commit received progress/core/exact receipt → release receipt. If incoming admission lacks capacity, defer before any acceptance ACK/NDR1; do not manufacture a job or consume its trusted receive transition.

After all chunks verify, first commit a publication intent containing the authorized destination, owned temporary/publication identities, expected content and recovery rule. Assemble to that private temporary on the publication filesystem, verify exact length/digest, sync and atomically publish with no replacement of an existing destination. Then commit PUBLISHED and make reserved completion READY. On restart, adopt only an output whose ownership and content are established by the committed intent and its reviewed recovery mechanism; matching bytes alone are insufficient. An unrelated destination collision, including identical content, must not be overwritten or adopted. If ownership is ambiguous, pause without confirming. User authorization of a destination is separate from wire metadata. No publication confirmation precedes successful publication.

Cleanup is resumable: mark retiring while keeping credits, remove only owned artifacts when protocol references permit, then release credits. A cut between deletion and credit release causes conservative overaccounting, not over-admission. Published user output remains disk usage; freeing transient job credit does not make that output's bytes free. Process-crash recovery and power-loss durability are different claims; strict directory-sync failures must propagate if power-loss durability is claimed. [S14]

Reserve **one incoming and one outgoing admitted transfer independently**, with two reusable in-flight protocol/queue slots per job. Those two slots limit outstanding work; they do not replace storage for the entire file. A single shared transfer slot deadlocks symmetric simultaneous sends. Reuse protocol slots only after exact receipt/closure conditions permit retirement, with new operation identities; retain the bulk bytes still required for publication. Fair scheduling must leave text and controls able to progress. Admission may wait for a peer's incoming slot; acceptance does not conjure remote capacity.

Define explicit Cancelled/Expired states and a fixed recovery horizon; **proposed development default: 24 hours from each endpoint's first durable admission**, persisted once and never extended by retries. Only a self-referencing manifest creates a FileJob, atomically with its accepted core/disposition; its initiating reference must equal its own authenticated Wire. File identity includes session, direction, initiating epoch/slot/operation and manifest hash. Sender waits for manifest NDR1 before sending chunks. Chunk/completion frames never create jobs. After receiver cancellation, valid authenticated late chunks without an active matching job are durably disposed through normal protocol processing and ordinary NDR1, with no publication or FILE_COMPLETE. Rejecting them forever would strand the sender's ratchet flights. Exact old manifest replay is handled by its disposition or closed replay floor, not a new job.

Sender cancellation stops new seals and removes unsealed intents; its at-most-two committed chunk flights, plus any committed manifest/completion, retain bounded session retirement credit until receipt/closure. Reserve a bounded discard/receipt drain allowance before file admission, and admit no new outbound file on that peer until its old committed file flights retire. Timeout cannot delete those obligations. Release the active file slot and transient bulk-storage credits only after cleanup and explicit transfer of remaining protocol liabilities to the session owner. A permanently absent peer can leave bounded protocol debt; explicit session close/re-establishment is the unilateral escape and must fail outstanding work honestly. This needs no unlimited per-file tombstone graveyard. A published job cannot be relabelled unpublished or have an owed completion silently withdrawn. Cancellation means local work stopped; it cannot retract bytes already released or prove the peer has not published them.

The combined bound must include both jobs, at least two peer sessions, unrelated text, output temporary space, queue replacement, retained controls and cleanup. Do not reuse the old single-transfer 8.5-MB estimate as that proof. If the combined profile does not fit, correct representation/reservation overhead or define a separately reviewed arbitration design before enabling it. Do not raise caps or silently impose a deadlocking restriction to make a demo pass. [P7]

### 7. State the remaining security boundary honestly

The candidate itself disclaims complete-old-backup rollback protection. Atomic writes and generation counters inside the same restorable vault do not detect restoring that entire vault. GOALS G2 requires rollback detection, and G4 requires machine-checkable verification for release. These remain explicit release work, not properties obtained from this plan's queue fix. [S15]

A normal restore workflow should establish a fresh device/session rather than silently resume old sending state. That reduces accidental reuse; it does not solve adversarial full-state rollback. A complete solution needs a freshness anchor outside the rollback domain, with a specified trust/availability model and recovery protocol. Make a narrow RollbackAnchor provider feasibility check an **early architecture gate**, before committing the full implementation budget. Evaluate platform-backed monotonic storage first; an ordinary OS key store is not automatically monotonic or excluded from backup. Demonstrate Linux/macOS behavior against the supported restore model. Where unavailable, a remote freshness witness changes offline, trust, metadata and per-commit fencing requirements; review it separately, not as an incidental relay field. If neither is acceptable, the release claim/goal requires an explicit decision. Do not conceal this behind “crash safe.” A separate local marker detects vault-only restoration only if the marker survives; it does not detect a whole-machine snapshot restoration. [S17]

Similarly, PQ recovery claims must identify compromise scope and the next uncompromised authenticated reseed assumptions. This exercise has not certified the cryptographic construction. Preserve the project's per-message hybrid and transcript-bound requirements, with model/vectors tied to the actual successor transitions. [S5]

## Q3 — Ordered implementation lanes and decisive acceptance

Use the ordered **F00–F18 work cards in the revision-3 execution guide above**. They replace the earlier broad milestone table with explicit prerequisites, source scope, decisive instruments, reviews and acceptance conditions. The guide also separates early rollback feasibility from implemented release acceptance and early profile/interface allocation from later file-format finalization. Each card may be split into bounded assignments without deleting its parent obligations.

The first live flight should be simple: two fresh clients, one relay, invite/verify, crossed text, offline recipient, restart, exact retry and receipt confirmation. Then fly simultaneous invitation crossing, mixed inbox, local saturation and lock races. Only after those pass should the operator spend time on maximum files. The final full-file flight uses production cryptography and the exact shipping path, not synthetic injected ciphertext.

Keep one current evidence matrix: property → exact source/artifact → test/instrument → observed result → remaining limit. Preserve old evidence without repeatedly copying its narrative. A green gate needs the executed test count, not just a process exit code. Feature-disabled release builds must also ignore old environment seams such as `QSC_UNSAFE_TEST_CLOCK_UNIX_S`; disabling only `na0780-test-hooks` does not accomplish that today. [S16]

## Q4 — What the director's plan misses

1. **The current recommendation is too narrow.** Queue admission plus three hygiene fixes does not restore the integration contract, prove full lifecycle bytes, finish invitation recovery or expose a usable app. The NA-0783 measurements themselves name the omitted growth and terminal-row problems. They are evidence for revising scope, not a reason to press on with an inadequate aggregate claim. [P4; P6]
2. **The original R-02 was a Stage-B-readiness finding.** It explicitly was not a Stage-A implementation defect, and its clearance requested a banked design/review/ruling. Conflating that with completed Stage-B machinery obscures what was actually required. Today the paired owner is valuable; finish and test its real contract instead of retroactively rewriting the review's demand. [P8]
3. **Seven scenarios were used too broadly.** The ruling correctly says universal-bound, malformed-state, production, macOS and actual-relay gates remained NOT-RUN. Keep those distinctions. One selected CI test is not the full directional target, and target inventory is not assertion coverage. [P9; S4]
4. **“Opt-in” and “main green” do not settle strategy.** Main documents a historical shared-root wedge and an ACK-specific mitigation; F03 must establish which crossed-send failures, if any, remain on the actual baseline. The successor's changed common helper contaminates many old fixtures. Neither fact justifies keeping old runtime behavior forever, nor dismissing every successor failure. [S1; P2]
5. **Crossed invitations and crossed messaging are separate bugs.** Different branches do not make their combined product independent. They share handshake, route, mailbox and persistence boundaries; the final acceptance must exercise both together. The ruling already identifies overlapping files. [P9]
6. **The product boundary was underspecified.** Desktop already implements much of the stated roadmap, but lacks messaging and complete verification actions. Relay response-loss recovery and read/delete capabilities must be addressed at the engine/service boundary. More protocol-only scenarios will not discover every GUI lock or mailbox ownership defect. [S7–S11]
7. **Some governance work costs more than it returns.** Re-copying authority text, serial document promotions with no changed decision, and rerunning the entire expensive suite after every small edit consume the operator. Preserve source pins, independent sensitive review, red-capable assertions and final exact-build acceptance. Propose one compact governing contract and delta evidence; apply any process simplification through the actual standing rules. [P10]
8. **Universal two-hour lanes are not credible.** The packet's directional smoke alone took 552 seconds near a 600-second limit. Bound active authoring/review; let deterministic validation run without a seat continually watching it. Do not make timeboxing a reason to omit a proof obligation. [P1]

## Q5 — Stage A and Stage B

**Keep the dependency, change the framing.** Define attachment ownership, reservation dimensions and persisted interface boundaries in F01. Finalize exact codec, FileJob and publication details in C06/F14, bringing forward any decision that affects earlier persistence. Implement and accept text before enabling file execution. Do invitation/relay recovery first in product order, with directional proof and contract work in parallel where independent and authorized.

The milestones are:

1. **Shared foundation:** one profile, invitation recovery, one dispatcher, paired authority, actual capacity bounds and honest status.
2. **Text engineering acceptance:** two real verified clients exchange recoverable text; retained nonfile properties pass. Attachment-success obligations remain explicitly unfinished.
3. **File engineering acceptance:** inline 1..4 MiB content, bounded one-in/one-out concurrency, exact retry, verified publication and separate confirmation through the real GUI.
4. **Release qualification:** complete required product/security inventory, hook-free platform artifacts, deployed-service acceptance, model and rollback commitments.

This preserves the inline review's useful ordering—publish before completion, bind references before release, reserve progress capacity—while correcting its unproved control inventory and concurrency scope. It also avoids spending months calling a text-only engine a completed messenger. [P7]

## Conditions for continuing, changing course or stopping

Continue with the candidate while ownership, exact retry, bounded accepted-work progress and meaningful property migration hold. Change branch mechanics if reviewability demands extraction. Redesign the relevant subsystem if a counterexample disproves a capacity/liveness invariant. Abandon the directional construction only if the ownership/recovery mechanism cannot meet the canonical goals without unacceptable complexity, or independent cryptographic review rejects it.

Do not release on the basis of this document. Release is justified by the implemented invariants and the exact-build evidence. The plan's practical advantage is that the high-risk contradictions are tested before GUI polish and maximum-file marathons, while preserving the considerable code that already solves the right problems.

## Source index

Packet paths below are relative to `second-opinion-packet/`; line numbers refer to the verified supplied files.

- **P1:** `04_suite_failures/reconcile_shards.log`, lines 231–245; `04_suite_failures/shard12.log`, lines 13–20. Reconciled counts and smoke timing.
- **P2:** `05_code/after/tests/common_mod.rs`, lines 158–176; `05_code/after/src/transport_mod.rs`, lines 488–538 and 3632–3652; `05_code/after/src/directional_delivery.rs`, lines 418–421. Fixture selection, successor-only paths and foreign-frame progression defect.
- **P3:** `04_suite_failures/shard0.log`, lines 17–31; `04_suite_failures/shard1.log`, lines 43–51. Attachment refusal failures.
- **P4:** `03_na0783_formalization/REPORT.md`, lines 21–38, 72–96 and 104–115. Conflicts, dependency audit result, queue premises and red-first admission evidence.
- **P5:** `02_reviews/ASTRA_read_1831_delta_2026-09-22.txt`, lines 18–48, 66–75 and 97–115. Persisted decoding, bounds, cleanup, partial R-02 implementation and remaining findings.
- **P6:** `03_na0783_formalization/DIRECTIVE_DRAFT.md`, lines 248–279. Lifetime scan cost, locking, measured encoded rows and explicitly excluded post-admission costs.
- **P7:** `02_reviews/NA0780-inline4m-design-review.md`, lines 15–34, 40–67, 73–106 and 112–185. Correctable design; single-transfer arithmetic; correct publication/reference ordering; concurrency and control-bound gaps.
- **P8:** `02_reviews/NA0780-Stage-A-independent-review.md`, lines 233–240 and 329–359; `02_reviews/ASTRA_read_1831_delta_2026-09-22.txt`, lines 91–105. Original R-02 scope and clearance distinction.
- **P9:** `01_state_and_rulings/RULING_NA0780_rederivation_2026-09-22.md`, lines 10–18 and 45–61. Separate branches, overlapping files, unrun gates and work order.
- **P10:** `00_BRIEF.md`, lines 14–21 and 72–88. Operator/process constraints and requested evidence/review shape.

Pinned live sources (line ranges given as text; each link opens the cited file):

- **S1 — main root collision:** [qsl-protocol lib.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/qsl/qsl-client/qsc/src/lib.rs#L1963), lines 1781–1798 and 1963–1992.
- **S2 — directional ownership:** [candidate directional_core.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/directional_core.rs#L559), lines 403–473, 559–575 and 639–680.
- **S3 — successor contract and fixtures:** [candidate README](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/README.md#L92), lines 92–143 and 163–200; [send_semantics.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/tests/send_semantics.rs#L57), lines 57–76 and 103–142; [desktop_gui_contract_na0215b.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs#L605), lines 605–650; [handshake/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/handshake/mod.rs#L336), lines 336–347.
- **S4 — actual directional test shape:** [qsc-sharded-suite.yml](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/.github/workflows/qsc-sharded-suite.yml#L139), lines 139–145; [na0780_directional_integration.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/tests/na0780_directional_integration.rs#L501), lines 501–534, 585–587 and 937–954.
- **S5 — canonical goals and pre-release strategy:** [PROJECT_CHARTER.md](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/PROJECT_CHARTER.md#L18), lines 18–55; [GOALS.md](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/GOALS.md#L5), lines 5–34 and 55–70.
- **S6 — paired authority:** [candidate vault/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/vault/mod.rs#L2054), lines 1961–2028 and 2054–2182; [directional_delivery.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/directional_delivery.rs#L580), lines 580–591, 689–744 and 1212–1244.
- **S7 — invitation state:** [main invite/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/qsl/qsl-client/qsc/src/invite/mod.rs#L1149), lines 1149–1178 and 1212–1240; [#1828 invite/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/e29a07dfba0eb24a547dd77027e54df15bd7eb73/qsl/qsl-client/qsc/src/invite/mod.rs#L1255), lines 1255–1274 and 1303–1356; [#1828 handshake/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/e29a07dfba0eb24a547dd77027e54df15bd7eb73/qsl/qsl-client/qsc/src/handshake/mod.rs#L930), lines 578–640, 712–736, 808–843, 930–985 and 2546–2552; [counterfeit-first regression](https://github.com/QuantumShieldLabs/qsl-protocol/blob/e29a07dfba0eb24a547dd77027e54df15bd7eb73/qsl/qsl-client/qsc/tests/na0768_invite_finish_mixed_role.rs#L1555), lines 1555–1613.
- **S8 — destructive remote one-shot operations:** [server store.rs](https://github.com/QuantumShieldLabs/qsl-server/blob/5ea0f9256703b97dbc77e692dfe92244efd7a9a0/src/store.rs#L514), lines 514–543 and 620–650.
- **S9 — relay capability and lease contract:** [server lib.rs](https://github.com/QuantumShieldLabs/qsl-server/blob/5ea0f9256703b97dbc77e692dfe92244efd7a9a0/src/lib.rs#L1025), lines 1025–1038, 1171–1201 and 1228–1256.
- **S10 — resource constants and controls:** [candidate directional_delivery.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/directional_delivery.rs#L1340), lines 1118–1135 and 1282–1368; [protocol_state/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/protocol_state/mod.rs#L1408), lines 1408–1499 and 1572–1643.
- **S11 — desktop integration:** [Cargo.toml](https://github.com/QuantumShieldLabs/qsl-desktop/blob/15818498cdd0536f08a1b9909291ee5153168cfe/src-tauri/Cargo.toml#L19), lines 19–23; [lib.rs](https://github.com/QuantumShieldLabs/qsl-desktop/blob/15818498cdd0536f08a1b9909291ee5153168cfe/src-tauri/src/lib.rs#L492), lines 492–555; [gateway.rs](https://github.com/QuantumShieldLabs/qsl-desktop/blob/15818498cdd0536f08a1b9909291ee5153168cfe/src-tauri/src/gateway.rs#L113), lines 113–128; [main.js](https://github.com/QuantumShieldLabs/qsl-desktop/blob/15818498cdd0536f08a1b9909291ee5153168cfe/ui/main.js#L3445), lines 2045–2088, 3445–3501, 3714–3727 and 3960–4056.
- **S12 — attachment service commit gap:** [qsl-attachments lib.rs](https://github.com/QuantumShieldLabs/qsl-attachments/blob/8210d7fb25fafd4ec6f5a08ab6a37a063343abbb/src/lib.rs#L952), lines 952–997; [main client attachments/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/qsl/qsl-client/qsc/src/attachments/mod.rs#L1057), lines 1057–1081.
- **S13 — file representation:** [candidate store/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/store/mod.rs#L311), lines 311–361; [msgqueue/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/msgqueue/mod.rs#L154), lines 154–216 and 390–411.
- **S14 — current filesystem durability:** [main fs_store/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/qsl/qsl-client/qsc/src/fs_store/mod.rs#L223), lines 223–253 and 493–494.
- **S15 — acknowledged rollback limits:** [candidate README](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/README.md#L160), lines 160–161 and 198; canonical release requirements in S5.
- **S16 — shipping test seam:** [candidate clock/mod.rs](https://github.com/QuantumShieldLabs/qsl-protocol/blob/ffc8fc529374e62b00ae9018726ab313efe0f036/qsl/qsl-client/qsc/src/clock/mod.rs#L40), lines 40–75.
- **S17 — normative persistence failure model:** [DOC-SCL-004](https://github.com/QuantumShieldLabs/qsl-protocol/blob/2112bba56c2869d942a44d263cbde7c997e3d88f/docs/spec-closure/DOC-SCL-004_State_Persistence_Crash_Safety_v1.0_DRAFT.md#L25), lines 25, 35–44, 69–82 and 110–140.
