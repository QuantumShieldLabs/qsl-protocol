Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-03

# DOC-OPS-001 — qbuild Continuity and Disaster Recovery Runbook v0.1.0 DRAFT

## 1. Purpose and posture

This runbook defines the checked-in continuity procedure for the qbuild control plane.

- qbuild remains the control plane for active work, evidence capture, and branch/worktree management.
- GitHub and configured remotes are the recovery source for git objects and merged truth.
- The governance spine remains authoritative for what work is allowed and what item executes next.
- `NEXT_ACTIONS.md` remains the execution source of truth.

This document is operational guidance only. It does not authorize queue reordering, runtime changes, or host-local overrides of live repo truth.

## 2. Expected qbuild topology

Minimum expected layout under `/srv/qbuild`:

- bare mirrors:
  - `/srv/qbuild/mirrors/qsl-protocol.git`
  - `/srv/qbuild/mirrors/qsl-server.git`
  - `/srv/qbuild/mirrors/qsl-attachments.git`
- active worktrees:
  - `/srv/qbuild/work/<directive-or-date>/qsl-protocol`
  - `/srv/qbuild/work/<directive-or-date>/qsl-server`
  - `/srv/qbuild/work/<directive-or-date>/qsl-attachments`
- supporting state:
  - `/srv/qbuild/evidence`
  - `/srv/qbuild/logs`
  - `/srv/qbuild/tmp`
  - `/srv/qbuild/cache`

Expected remote posture for each active worktree:

- `origin` points at the GitHub repository and is the push remote.
- `mirror` points at the local bare mirror and is fetch-only.
- `mirror` push remains disabled.

## 3. Authority proof before any write

Before mutating a repo, the operator must prove live truth in this order:

1. confirm disk watermark and host path for `/srv/qbuild`;
2. refresh each bare mirror from its configured `origin`;
3. refresh each worktree from its configured `origin` and `mirror`;
4. enumerate refs with `git for-each-ref --format='%(refname) %(objectname)' refs/heads refs/remotes`;
5. read the governance spine in order:
   - `START_HERE.md`
   - `GOALS.md`
   - `AGENTS.md`
   - `PROJECT_CHARTER.md`
   - `NEXT_ACTIONS.md`
   - `DECISIONS.md`
   - `TRACEABILITY.md`
6. prove the exact live READY item and scope from `NEXT_ACTIONS.md` using zero-failure-safe parsing; and
7. prove sibling-repo queue assumptions before relying on them.

For the active `qsl-protocol` worktree, `HEAD`, local `main`, `origin/main`, `mirror/main`, and the bare mirror `main` must align before branching for a new lane.

## 4. Merge-refresh procedure

Use this sequence after every merge and before any new implementation lane:

1. fetch the bare mirror from `origin`;
2. fetch the active worktree from `origin`;
3. fetch the active worktree from `mirror`;
4. switch the worktree to `main`;
5. fast-forward with `git merge --ff-only origin/main`;
6. re-run ref enumeration and READY proof; and
7. confirm `git status --short --branch` is clean.

Never use `--watch` modes for CI checks. Poll required check-runs with bounded REST calls and `per_page=100`.

## 5. GitHub-only recovery starting point

If qbuild is lost and only GitHub remains available:

1. recreate the `/srv/qbuild` directory skeleton;
2. recreate the bare mirrors with `git clone --mirror` from GitHub;
3. recreate worktrees from the local mirrors;
4. in each worktree:
   - rename the default mirror-sourced remote to `mirror` if needed;
   - add `origin` pointing at GitHub;
   - disable pushes on `mirror`;
   - fetch `origin` and `mirror`;
5. authenticate `gh` and verify access to the three repositories;
6. restore the latest off-host continuity snapshot and overlay inventory;
7. run the authority proof from Section 3 before resuming work; and
8. restore in-flight branches only from pushed branch SHAs, PR heads, or recorded overlays after diff review.

Recovery is not complete until the live queue, governing refs, and active worktree status all match the recorded snapshot and current GitHub truth.

## 6. Operator host prep and qstart expectations

The host bootstrap layer is expected to provide:

- `git`
- `gh`
- `python3`
- `zip`
- coreutils and POSIX shell tooling
- enough free disk for mirrors, worktrees, and evidence capture

If a site-local `qstart` helper exists, it may:

- create the `/srv/qbuild` directory skeleton;
- install or verify the required toolchain; and
- prepare authentication/session prerequisites.

`qstart` must not:

- invent queue order;
- override `origin/main` or the governance spine;
- rewrite branch history; or
- become the source of truth for READY state.

If `qstart` output conflicts with live repo truth, stop and trust the governance spine plus refreshed refs instead.

## 7. In-flight branches, patch bundles, overlays, and end-of-day state

Branch and handoff rules:

- one active branch should map to one directive or NA lane;
- record `git status --short --branch` and the changed-path list before and after every work session;
- push in-scope branches after the first green local validation bundle when continuity matters; and
- keep merge-commit PR history intact; do not use destructive rewrites.

Overlay and patch-bundle rules:

- overlays must contain only changed files that unzip at repo root;
- record overlay filename, purpose, base SHA, and head SHA in the snapshot manifest;
- never include build outputs, caches, or `.git/` content; and
- store overlay bundles off-host if they are needed for recovery.

End-of-day minimum record:

- active branch names and SHAs;
- open PR list;
- worktree inventory and current status;
- pending local overlays/patch bundles;
- current READY proof; and
- the off-host snapshot location for the latest continuity package.

## 8. Secret-handling rule for continuity artifacts

Continuity artifacts must never contain raw secret values.

Allowed:

- references to secret managers, vault item names, or operator runbooks;
- redacted examples;
- environment variable names without values; and
- short SHAs or descriptive counts.

Forbidden:

- tokens, passphrases, passwords, auth headers, or route tokens;
- secret-bearing URLs;
- long hex dumps that look like live secrets; and
- copied local secret files.

Use descriptive wording such as `v1-path pattern` and `hex32plus pattern` in narrative evidence when secret-like material must be discussed.
