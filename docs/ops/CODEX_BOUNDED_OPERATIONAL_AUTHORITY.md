Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-06-30

# Codex Bounded Operational Authority

## 1. Purpose

This runbook defines project-wide bounded Codex operational authority. It makes
the D-1140 authority model durable without granting default remote action,
privileged action, runtime action, secret publication, or relaxed evidence
requirements.

## 2. Scope

This model governs Codex operational diagnostics and bounded test actions across
QSL lanes when the active directive explicitly opts in. It is subordinate to the
governance spine, `NEXT_ACTIONS.md`, qwork proof, one-READY queue discipline,
required checks, public-safety, advisories, private-material controls, and exact
path scope.

This model does not let Codex run `qwork`, `qstart`, or `qresume`.

This model also defines the project-wide issue-resolution authority used when a
diagnostic, test, workflow, build, runtime check, integration path, or generated
harness fails. Codex must not stop at a coarse failed, blocked, ambiguous, or
insufficient label while safe in-scope investigation remains available. Codex
must inspect relevant project-owned source, tests, workflows, scripts,
artifacts, logs, generated harnesses, and proof-root tooling; classify the
likely issue; apply authorized safe fixes when the active tier permits them;
rerun the smallest relevant validation; and report the result.

## 3. Authority tiers

The authority tiers are:

Tier 0 - governance/read-only:
- default for public, release, crypto, formal, dependency, and claim lanes
  unless exact operational authority is granted.
- includes source analysis for project-owned code, tests, workflows, scripts,
  artifacts, logs, and generated harnesses when needed to classify a failure.

Tier 1 - redacted diagnostics:
- Codex may run exact read-only commands or SSH-stdin scripts on named test
  hosts/workspaces.
- Output must be reduced to safe enums/classes.
- Raw output remains proof-root-only.
- Codex may fix generated proof-root scripts, parsers, scanners, classifiers,
  manifests, fixture generators, and local harnesses when the fix stays
  proof-root-only and safety boundaries hold.

Tier 2 - bounded test action:
- Codex may perform exact no-secret, non-root, non-privileged, reversible test
  actions in approved test workspaces.
- Action requires preflight proof and postcheck.
- Rollback/manifest required where state changes.
- When an active lane opts into issue-resolution authority, Codex may also
  patch minimal project-owned diagnostic surfaces, tests, and workflow
  diagnostics inside the lane's allowed paths to classify the active failure.
  Such fixes must preserve redaction, fail-closed behavior, checks, and claim
  boundaries.

Tier 3 - operator/admin action:
- sudo, systemd, firewall, Tailscale, account, shell, authorized_keys,
  root-owned service, backup, and privileged operator actions remain
  operator-owned unless a later directive explicitly authorizes a privileged
  lane.
- Minimal project-owned diagnostic or integration bug fixes may be made only
  when the active lane explicitly opts in, evidence supports the bug, the fix
  is narrow, tests/proof are added, paths are allowed, and no crypto, protocol,
  wire, auth, state-machine, or security semantics change unless explicitly
  authorized.

Tier 4 — forbidden:
- secret publication;
- destructive unbounded mutation;
- workflow weakening;
- protocol/crypto/security semantic changes outside exact scope;
- public/production/security overclaims.

## 4. Tier 0 - governance/read-only

Tier 0 is the default. It permits repository reads, local file inspection,
proof-root evidence assembly, GitHub metadata reads, governance edits within
the active directive scope, and validation commands that do not mutate runtime
state or weaken enforcement.

For failures, Tier 0 includes mandatory source analysis of project-owned code
and harnesses. Codex may inspect relevant source, tests, workflows, scripts,
artifacts, logs, generated scripts, classifiers, and proof-root outputs to
classify why a check or diagnostic failed. A coarse failure label is not enough
when safe evidence can still be gathered.

Tier 0 does not authorize remote commands, qsc commands, qsl-server start,
workflow dispatch/rerun, qsl-attachments work, public-site mutation,
Cloudflare mutation, dependency mutation, privileged action, or private-value
publication.

## 5. Tier 1 - redacted diagnostics

Tier 1 may be used only when the active directive names the host/workspace or
local path, exact read-only command family, raw-output quarantine path, and safe
publication format.

Tier 1 diagnostics must reduce output to coarse classes such as `present`,
`absent`, `success`, `failure`, `not_checked`, `unknown`, or
directive-defined safe enums. Raw command output, logs, process details,
private topology, endpoint values, private port values, tokens, Authorization
material, payloads, response bodies, and key material must remain
proof-root-only and must not be committed.

Safe proof-root fixes are authorized by default. Codex may repair generated
proof-root scripts, parsers, scanners, classifiers, manifests, fixture
generators, and local harnesses without a new Director turn when the repair
stays proof-root-only, preserves private-material quarantine, and does not cross
the stop boundaries in this runbook or the active directive.

## 6. Tier 2 - bounded test action

Tier 2 may be used only when the active directive proves all action gates before
execution. The action must be no-secret, non-root, non-privileged, reversible,
bounded to an approved test workspace, and limited to the exact command family
named by the directive.

Tier 2 requires preflight proof, a state-change manifest when state changes,
postcheck proof, cleanup/rollback proof when applicable, and a private-material
scan before any evidence is published.

When the active lane opts into the issue-resolution authority model, Tier 2 also
permits minimal project-owned diagnostic fixes inside declared allowed paths.
Examples include safe error-classification summaries, redacted diagnostic
fields, workflow diagnostic summaries, CI helper diagnostics, and tests proving
redaction and no-private-material publication. These fixes must not weaken auth,
validation, fail-closed behavior, checks, or claim boundaries.

Minimal project-owned bug fixes are allowed only when evidence supports the
bug, the fix is narrow, the active scope allows the path, tests/proof are added,
and the fix does not alter crypto, protocol, wire, auth, state-machine, or
security semantics unless the active lane explicitly authorizes those
semantics.

## 7. Tier 3 — operator/admin action

Tier 3 action remains operator/admin-owned unless a later directive explicitly
authorizes a privileged lane with exact boundaries. Codex must stop before
sudo, systemd, firewall, Tailscale, account, shell, authorized_keys,
root-owned service, backup, service-owner, tunnel, or other privileged action
unless the active directive grants that authority.

## 8. Tier 4 — forbidden

Tier 4 action is never authorized by this model. It includes secret
publication, destructive unbounded mutation, weakening workflow/check behavior,
protocol/crypto/security semantic changes outside exact scope, and forbidden
public-readiness, forbidden production-readiness, forbidden vulnerability-free,
forbidden bug-free, forbidden perfect-build, forbidden perfect-crypto, or
similar overclaims.

## 9. Directive opt-in requirements

An active directive must name all of the following before Tier 1 or Tier 2 may
be used:

- named authority tier;
- host/workspace or local path;
- exact command family;
- allowed mutation paths;
- raw-output quarantine path;
- redaction/publication policy;
- rollback/manifest requirements;
- private-material scan;
- stop conditions;
- final response claim boundaries.

Missing, ambiguous, contradictory, or convenience-inferred authority is no
authority. Codex must stop rather than infer.

## 10. Approved test-host/workspace registry

Current approved test registry:

- host label: `inspiron`
- workspace: `/home/qslcodex/qsl-remote-test/`
- purpose: QSL/QSC remote relay testing
- allowed tiers only when an active directive opts in: Tier 1 and Tier 2
- no blanket sudo/systemd/firewall/Tailscale/account authority

This registry is not a standing grant. A directive must still opt in and name
exact boundaries for each lane.

## 11. Redacted evidence format

Repository evidence should publish only safe labels, counts, and enums. Use
coarse values such as `present`, `absent`, `ready`, `not_ready`,
`insufficient`, `not_checked`, `unknown`, `pass`, and `fail` where those labels
are enough to support the decision.

Do not publish raw private values. Do not publish endpoint values, private port
values, topology, token values, Authorization headers, process identities,
payloads, response bodies, authorized_keys content, raw private logs, key
material, secret env values, Cloudflare tokens, or API keys.

## 12. Private inspection vs private publication

Private inspection may occur only when the active directive authorizes it and
the raw output is quarantined in the proof root. Private publication is not
permitted. A value inspected privately must be converted into a safe class
before it appears in tracked files, PR bodies, final responses, or public docs.

## 13. Raw output quarantine

Raw outputs, generated diagnostic scripts, generated start/postcheck scripts,
logs, parsed private JSON, and intermediate scans must remain in the proof root
or another directive-named quarantine path. Tracked repository files may include
only summaries, classifications, marker names, validation status, and safe
evidence references.

## 14. Bounded start/action gates

A bounded start or action requires all of the following:

- active directive Tier 2 opt-in;
- exact command family;
- exact workspace;
- proof that no secrets are required or emitted;
- proof that action is non-root and non-privileged;
- proof that no sudo/systemd/firewall/Tailscale/account/shell/authorized_keys
  mutation is required;
- proof that state changes are reversible or manifest-recorded;
- preflight success;
- postcheck plan;
- cleanup/rollback plan where applicable;
- private-material scan plan.

Any unknown gate fails closed.

## 15. Rollback and manifest requirements

When Tier 2 changes state, Codex must write a manifest before or immediately
after the action that records the state boundary, generated artifacts, cleanup
owner, and rollback path. If cleanup is not needed, the evidence must state why.
If rollback cannot be proven, Codex must stop before action.

## 16. Required stop conditions

Codex must stop when:

- the qwork proof is missing, stale, malformed, inconsistent, or unavailable;
- the queue has zero or more than one READY item;
- the READY item differs from the active directive;
- required checks are failed, pending beyond cap, or ambiguous;
- public-safety or advisories are not green when required;
- the requested action touches out-of-scope paths;
- the requested action changes protocol, wire, crypto, auth, state-machine, or
  security semantics outside exact scope;
- the requested action weakens validation, auth, evidence, checks, or claims;
- private material would be published;
- source analysis or a fix would require secrets, private key access,
  dependency or lockfile changes outside scope, branch-protection/settings
  mutation, workflow weakening, out-of-scope source paths, or unquarantined raw
  private material;
- Tier 3 or Tier 4 action is required without exact authorization;
- root cause is unclear enough that continuing would risk untruthful evidence
  or behavior drift.

## 17. Required validation gates

Each lane must run the validation gates named by its directive. When Tier 1 or
Tier 2 is used, validation must include scope guard proof, private-material
scan, overclaim scan, raw-output quarantine proof, safe-publication proof, and
any required postcheck/rollback proof.

## 18. Required final response boundaries

Final responses must report only safe classes and public metadata. They must
not include raw private outputs, endpoint values, private ports, topology,
tokens, Authorization values, process identities, payloads, response bodies,
authorized_keys content, key material, secret values, or private logs. They
must not make public-readiness claims, must not make production-readiness
claims, must not make vulnerability-free claims, must not make bug-free claims,
must not make perfect-build claims, must not make perfect-crypto claims, and
must not make similar overclaims.

## 19. qwork and queue invariants

Codex must not run `qwork`, `qstart`, or `qresume`. The operator-provided qwork
proof must be verified from files before fetch or mutation when a directive
requires it. Exactly one READY item remains mandatory unless the active
directive explicitly records a stop state before mutation.

## 20. Required-check and CI invariants

Required-check handling must be read-only unless the active directive explicitly
authorizes workflow dispatch/rerun. Watch modes are not permitted. Use bounded
REST polling and stop at cap. Do not weaken required-check, goal-lint,
public-safety, advisories, CodeQL, or branch-protection intent.

## 20A. Productive CI wait-work and read-only forward audits

CI/check polling must not be idle when safe current-lane or read-only
forward-audit work exists. During long waits, Codex must perform productive
same-lane proof work, validation work, response drafting, scanner/parser
hardening, or read-only forward audits. Codex must not begin the next READY
lane or mutate outside scope during waits. Final responses must report what
useful work was performed during waits; if no useful work was possible, Codex
must justify that specifically. Passive polling alone is not productive work.

This rule is triggered by any of the following:

- an expected CI/check wait over about 2-3 minutes;
- any wait already over 60 seconds;
- known long public-safety, full-suite, macOS, or CodeQL waits.

Required continuous work categories, when safely available inside the active
directive boundary, are:

1. capture PR, check, status, and branch-protection metadata;
2. draft response sections that are already knowable;
3. prepare if-green and if-red plans;
4. prepare post-merge and closeout proof;
5. run remaining allowed validation;
6. harden proof-root parsers, scanners, and classifiers;
7. perform read-only forward audits;
8. triage findings;
9. record work in the proof root;
10. report wait-work in the final response.

Allowed read-only audit categories are:

- crypto/protocol invariants;
- qsc client/runtime risks;
- qsl-server relay risks;
- qsl-attachments boundary risks;
- dependency/advisory watch;
- workflow/CI reliability;
- public-claim safety;
- private-material/redaction safety;
- test coverage gaps;
- runbook/operator friction;
- governance/queue consistency;
- build/provenance/reproducibility risks.

Forward-audit findings should be classified as `CRITICAL_BLOCKER`,
`HIGH_VALUE_FIX`, `MEDIUM_RISK`, `LOW_RISK`, `FALSE_POSITIVE`, or
`NO_ACTION`.

During CI waits, Codex must not:

- implement the next READY lane;
- open speculative PRs;
- mutate outside current scope;
- close out before required gates are green;
- run `qwork`, `qstart`, or `qresume`;
- weaken checks;
- treat pending or failed CI as success;
- publish private material;
- make forbidden public-readiness, production-readiness, security,
  forbidden vulnerability-free, forbidden bug-free, forbidden perfect-build, or
  forbidden perfect-crypto overclaims.

## 21. Public-safety/advisories invariants

This model does not weaken public-safety or advisories. A lane that requires
those gates must verify them as green or stop under the directive's recovery
budget. Non-fatal warnings may be logged, but failed or ambiguous required
security gates must not be ignored.

## 22. qsc/protocol/crypto/security mutation boundary

Codex must not change qsc runtime behavior, protocol behavior, wire semantics,
crypto, auth, negotiation, key schedules, state machines, or security semantics
unless the active directive explicitly scopes that change and the required
decision, traceability, tests, and vectors are included.

## 23. qsl-server/qsl-attachments boundary

This model does not authorize qsl-server start/run/deployment, qsl-server
source mutation, qsl-server PRs, qsl-attachments command/build/run/clone,
qsl-attachments mutation, or qsc send/receive by default. Those actions require
a later exact directive and must preserve private-material boundaries.

## 24. Public-site/Cloudflare boundary

This model does not authorize public-site, website, public docs publication
changes outside exact scope, Cloudflare mutation, DNS mutation, tunnel mutation,
or production/public availability claims.

## 25. Operator-owned privileged action boundary

Operator/admin tasks remain operator-owned unless a later directive explicitly
authorizes a privileged lane. Codex must stop before privileged action rather
than converting Tier 1 or Tier 2 into sudo/systemd/firewall/Tailscale/account
authority.

## 26. Example: inspiron / qslcodex test workspace

The `inspiron` host label and `/home/qslcodex/qsl-remote-test/` workspace may
be used for QSL/QSC remote relay testing only when the active directive opts in
to Tier 1 or Tier 2 and names exact boundaries. The registry entry does not
authorize blanket remote action, privileged action, qsl-server start, qsc
send/receive, qsl-attachments work, or private-value publication.

## 27. Example directive snippet for Tier 1

```text
Authority tier: Tier 1 redacted diagnostics.
Host/workspace: inspiron, /home/qslcodex/qsl-remote-test/.
Command family: exact read-only diagnostic script through SSH stdin.
Raw-output quarantine: proof root only.
Publication policy: publish safe enums/classes only.
Stop conditions: stop on missing qwork proof, private material, command drift,
or any need for Tier 2/Tier 3 action.
Final response boundary: no raw output or private values.
```

## 28. Example directive snippet for Tier 2

```text
Authority tier: Tier 2 bounded test action.
Host/workspace: inspiron, /home/qslcodex/qsl-remote-test/.
Command family: exact non-root reversible test start command named here.
Allowed mutation paths: only generated files under the named test workspace.
Preflight: no-secret, no-sudo, no-systemd, no-firewall, no-Tailscale,
no-account-mutation, no-private-publication proof.
Rollback/manifest: write action manifest, postcheck, cleanup proof, and
private-material scan into proof root.
Stop conditions: stop on any unknown preflight gate or privilege requirement.
Final response boundary: publish only classes and pass/fail markers.
```

## 29. Examples of forbidden overclaims

Forbidden overclaims include:

- claiming public readiness from a private or loopback-only test;
- claiming production readiness from a bounded test workspace;
- claiming external review completion without actual external review evidence;
- claiming vulnerability-free, bug-free, perfect-build, or perfect-crypto
  status;
- claiming a service is deployed, reachable, or safe when evidence only proves
  a coarse diagnostic class.

## 30. Relationship to existing governance

This runbook is part of the operational governance layer. It does not override
`START_HERE.md`, `AGENTS.md`, `NEXT_ACTIONS.md`, `DECISIONS.md`,
`TRACEABILITY.md`, qwork proof requirements, public-safety, advisories,
required checks, protocol-change rules, or exact directive scope. It formalizes
how future directives may opt into bounded operational authority while
preserving fail-closed evidence and claim boundaries.
