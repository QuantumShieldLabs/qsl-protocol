Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-04-30

# Workday and Overnight Autopilot Policy

## Level 1: unattended inside active scope

Codex may perform these actions unattended when they remain inside the active directive scope and preserve the sole READY item:

- fetch current refs
- preserve local state
- refresh a clean branch from `origin/main`
- validate locally
- poll CI with bounded REST/API loops
- perform a normal merge with a validated head SHA
- verify post-merge state

## Level 2: only with directive pre-authorization

Codex may do these only when the directive explicitly authorizes them:

- combined closeout
- successor restoration
- PR body update
- close a superseded PR

## Level 3: hard stops

Codex must stop for Director action if any of these appear:

- branch protection changes
- admin bypass
- public-safety red
- forbidden paths
- queue ambiguity
- more than one READY item
- code changes outside active scope
- required checks missing, failing, or not accepted

## Overnight mode

Overnight mode is bounded automation, not open-ended implementation. It must not:

- start unapproved new implementation lanes
- change branch protection
- create public-safety exceptions
- spoof checks
- direct-push
- squash or rebase merge when merge commits are required
- modify code outside the active scope

If the next step would require a new implementation lane, the run must stop after recording evidence and preserving the queue.

## Wait budgets

For public-safety / full-suite post-merge completion, the wait budget is 120 minutes. The default polling interval is 5 minutes. Stop immediately on failure. Continue on queued or in-progress checks until the 120-minute budget is exhausted.

For PR required checks, use the same 120-minute budget and 5-minute polling interval unless a directive sets a narrower bound. Do not use watch modes.

## Morning report

The morning report must include:

- directive ID and timestamps
- branch, PR, head, and merge commit if any
- queue proof before and after
- public-safety required/green proof
- poll tables for pending checks
- changed-path scope proof
- commands run and pass/fail summary
- recovered failures with classification and final result
- explicit statement whether code changed
- next recommended READY item
