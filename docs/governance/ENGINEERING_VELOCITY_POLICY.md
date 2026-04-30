Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-04-30

# Engineering Velocity Policy

## Policy

QuantumShield governance exists to accelerate correct engineering, not replace it. Normal queue items should produce one or more concrete engineering outputs:

- executable behavior
- invariant tests
- conformance vectors
- demo acceptance behavior
- release-hardening automation

Pure governance-only PRs are exceptional. They are appropriate when they repair queue integrity, record required implementation traceability, unblock a CI deadlock, or make a release-control decision that prevents unsafe drift. They are not appropriate as routine substitutes for code, tests, vectors, or automation.

## PR shape

Prefer one invariant per PR. A PR should have a small protected property, a clear scope guard, and evidence that fails for the right reason when the property is broken.

Every PR must preserve:

- fail-closed behavior
- no silent downgrade behavior
- queue order from `NEXT_ACTIONS.md`
- public-safety required-check integrity
- branch-protection truth
- demo and release honesty

## Evidence discipline

Evidence must name exact commands, changed paths, required checks, and queue/parser results. It should use short SHAs in prose, avoid sensitive dumps, and distinguish proven bugs from recommendations or uncertain observations.

For docs-heavy work, the evidence should prove the docs do not overclaim production readiness and do not create required protocol meaning outside the canonical specs.

## Scope control

No hidden scope creep is allowed. If work requires protocol, wire, crypto, auth, state-machine, runtime, workflow, script, Cargo, demo, or service changes beyond the active item, stop and promote a successor or request explicit direction.

## Stop conditions

Stop rather than continue if any of these occur:

- more than one READY item exists
- public-safety is missing, red, or not accepted where required
- branch protection appears weakened or exception-based
- a required check is missing or failing
- code changes are needed in a docs-only lane
- decision IDs are ambiguous or duplicated
- a proposed fallback would dilute fail-closed behavior

## Branch-protection posture

Temporary branch-protection exceptions must not become normalized practice. Future work should prefer executable gate hardening, helper tests, and precise repair-admission rules over manual exceptions.
