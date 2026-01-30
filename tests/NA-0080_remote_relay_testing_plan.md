# NA-0080 — Remote relay testing plan

## Scope & assumptions
- Remote relay exists (AWS) and is reachable via RELAY_URL.

## Remote topology
- qsc client(s) -> remote relay (hostile conditions).

## Scenario matrix
- happy-path
- drop+reorder

## Normalized marker subset definition
- Define exact regex/fields to compare (TBD in implementation).

## Workflow strategy
- nightly + workflow_dispatch
- upload markers + normalized subset + summary

## Verification checklist
- Artifacts uploaded
- No secrets in logs
- Non-required check contexts

## Rollback
- Disable workflow and revoke relay credentials
