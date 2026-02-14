# Worktree Hygiene Policy

This repository uses multiple worktrees for implementation and verification. To avoid branch collisions and stale registry entries:

- Verification/forensics worktrees must be detached (`git worktree add <path> origin/main`).
- Do not run verification from `/tmp` worktrees.
- Do not pin `main` in a secondary worktree.

## Why This Matters

- Pinning `main` outside the canonical repo path causes merge/checkout failures (`main is already used by worktree ...`).
- `/tmp` worktrees often leave stale/prunable entries after directory cleanup.
- Scripts that assume `git checkout main` are unsafe in multi-worktree repos.

## Standard Cleanup

1. Review registered worktrees:
   - `git worktree list --porcelain`
2. Remove stale temporary worktrees explicitly:
   - `git worktree remove <path>`
3. Prune stale registrations:
   - `git worktree prune`

## Scripted Sentinel

- Run `scripts/ci/hygiene_sentinel.sh` in repo root.
- Optional strict flags:
  - `--require-clean`
  - `--fail-on-tmp`
  - `--fail-on-main-pin`
