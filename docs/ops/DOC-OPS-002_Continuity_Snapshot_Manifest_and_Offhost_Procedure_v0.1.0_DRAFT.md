Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-03

# DOC-OPS-002 — Continuity Snapshot Manifest and Off-host Procedure v0.1.0 DRAFT

## 1. Purpose

This document defines the minimum continuity snapshot package required to recover qbuild work without relying on one host or operator memory.

If the only copy of a continuity package is on `/srv/qbuild`, it does not satisfy this procedure.

## 2. Minimum snapshot cadence

Capture and store a snapshot off-host at least:

- at end of day whenever any NA branch or PR is still open;
- immediately after the first green local validation bundle for continuity-critical or governance lanes, before waiting on GitHub CI;
- immediately before host maintenance, cleanup, or migration that could affect `/srv/qbuild`; and
- immediately after a merge that changes queue/governance truth or active continuity artifacts.

## 3. Minimum snapshot contents

Every valid continuity snapshot must include:

- repo SHA inventory:
  - `qsl-protocol` active branch, `main`, `origin/main`, and `mirror/main`
  - `qsl-server` `main`, `origin/main`, and `mirror/main`
  - `qsl-attachments` `main`, `origin/main`, and `mirror/main`
- governance spine copy or checksum set:
  - `START_HERE.md`
  - `GOALS.md`
  - `AGENTS.md`
  - `PROJECT_CHARTER.md`
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
- open PR list for active repos
- branch and worktree inventory
- patch/overlay bundle inventory
- qbuild topology manifest
- secrets inventory references only, never secret values

Optional additions may be included, but missing any required item makes the snapshot incomplete.

## 4. Copyable manifest template

Use this checked-in template shape for every continuity snapshot:

```md
# Continuity Snapshot Manifest

- Snapshot UTC:
- Operator:
- Trigger:
- Off-host destination:

## Disk watermark
- filesystem:
- total GiB:
- used GiB:
- free GiB:
- used %:

## Repo SHAs
- qsl-protocol:
  - active branch:
  - HEAD:
  - main:
  - origin/main:
  - mirror/main:
- qsl-server:
  - main:
  - origin/main:
  - mirror/main:
- qsl-attachments:
  - main:
  - origin/main:
  - mirror/main:

## Governance spine
- START_HERE.md:
- GOALS.md:
- AGENTS.md:
- PROJECT_CHARTER.md:
- NEXT_ACTIONS.md:
- DECISIONS.md:
- TRACEABILITY.md:

## Open PRs
- qsl-protocol:
- qsl-server:
- qsl-attachments:

## Worktree inventory
- worktree root:
- active branches:
- changed-path summary:

## Patch / overlay bundles
- file:
- purpose:
- base SHA:
- head SHA:

## qbuild topology
- mirrors path:
- work path:
- evidence path:
- logs path:

## Secrets inventory references only
- secret manager / vault references:
- runbook references:
- values included: no
```

## 5. Off-host storage requirement

Each snapshot package must be copied to storage that is not hosted on the same qbuild machine.

Acceptable examples:

- organization-managed object storage;
- an encrypted administrative file share; or
- a second managed host dedicated to backups.

Unacceptable examples:

- `/srv/qbuild/archive` only;
- a local operator home directory on the same machine; or
- a chat transcript with no copied manifest or bundle.

## 6. Restore procedure

To restore from a continuity snapshot:

1. prepare the host and required tooling;
2. recover the latest off-host manifest and any recorded overlays;
3. recreate bare mirrors and worktrees;
4. fetch configured remotes and verify ref alignment;
5. restore active branches from pushed SHAs or PR heads;
6. compare the recovered governance spine against the snapshot;
7. re-run READY proof from live `NEXT_ACTIONS.md`; and
8. rerun the relevant local validation bundle before resuming or merging in-flight work.

If live GitHub truth disagrees with the snapshot, treat the snapshot as stale context and follow current repo truth after recording the drift.

## 7. Snapshot rejection rules

Reject a continuity snapshot if any of the following is true:

- it lacks repo SHAs or the governance spine;
- it has no off-host copy;
- it stores secret values instead of references; or
- it cannot identify active worktrees, branches, and open PRs.
