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

## Executed evidence
- Script: scripts/demo/qsc_remote_relay_smoke.sh
- Workflow: .github/workflows/remote-relay-tests.yml (workflow_dispatch + nightly)
- Artifacts: remote.markers, normalized_subset.txt, summary.txt
- Normalized subset: fields from QSC_MARK lines (event/seed/scenario).

## workflow_dispatch inputs
- scenario (happy-path|drop-reorder)
- seed (u64 string)
