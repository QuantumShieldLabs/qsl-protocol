# NA-0089 — Demo evidence counts plan

## Scope & assumptions

## Artifact format changes (summary/subset)

## Marker-to-count mapping (what counts as drop/reorder/deliver)

## Determinism rules (seed/scenario)

## Local demo verification steps

## Remote relay verification steps (workflow artifact)

## Rollback

## Executed evidence
- Count mapping (deterministic, marker-based):
  - deliver_count: action=deliver
  - drop_count: action=drop
  - reorder_count: action=reorder
  - dup_count: action=dup (optional; emitted as 0 if absent)
- Local script updates:
  - scripts/demo/qsc_demo_local.sh writes summary.txt + normalized_counts.txt
- Remote script updates:
  - scripts/demo/qsc_remote_relay_smoke.sh writes summary.txt + normalized_counts.txt
- Sanity commands (local):
  - bash -n scripts/demo/qsc_demo_local.sh
  - bash -n scripts/demo/qsc_remote_relay_smoke.sh
  - ./scripts/demo/qsc_demo_local.sh --help
  - ./scripts/demo/qsc_remote_relay_smoke.sh --help
