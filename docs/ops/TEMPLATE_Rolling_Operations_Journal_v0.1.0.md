Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-04

# TEMPLATE — Rolling Operations Journal v0.1.0

Use this template for one active directive entry. Replace placeholders with real values. Do not store live secrets in any field.

## Blank template

```md
# Rolling Operations Journal Entry

- Directive:
- Begin timestamp (America/Chicago):
- Begin timestamp (UTC):
- End timestamp (America/Chicago):
- End timestamp (UTC):

## Repo SHAs
- qsl-protocol branch:
- qsl-protocol HEAD:
- qsl-protocol main:
- qsl-protocol origin/main:
- qsl-protocol mirror/main:
- qsl-server main:
- qsl-server origin/main:
- qsl-server mirror/main:
- qsl-attachments main:
- qsl-attachments origin/main:
- qsl-attachments mirror/main:

## READY proof
- READY_COUNT:
- Sole READY item:
- Proof source:

## Worktree / branch / PR
- Worktree path:
- Branch:
- PR:
- Merge commit:

## Failures / recoveries
- None yet

## Validation / CI notes
- Local validation:
- Protected checks:
- Retry notes:

## Disk watermark
- Filesystem:
- Total GiB:
- Used GiB:
- Free GiB:
- Used %:

## Next-watch items
- None yet
```

## Minimal worked example using placeholders only

```md
# Rolling Operations Journal Entry

- Directive: <NA-xxxx — Example Title>
- Begin timestamp (America/Chicago): <YYYY-MM-DDTHH:MM:SS-05:00>
- Begin timestamp (UTC): <YYYY-MM-DDTHH:MM:SS+00:00>
- End timestamp (America/Chicago): <YYYY-MM-DDTHH:MM:SS-05:00>
- End timestamp (UTC): <YYYY-MM-DDTHH:MM:SS+00:00>

## Repo SHAs
- qsl-protocol branch: <example-branch>
- qsl-protocol HEAD: <sha12>
- qsl-protocol main: <sha12>
- qsl-protocol origin/main: <sha12>
- qsl-protocol mirror/main: <sha12>
- qsl-server main: <sha12>
- qsl-server origin/main: <sha12>
- qsl-server mirror/main: <sha12>
- qsl-attachments main: <sha12>
- qsl-attachments origin/main: <sha12>
- qsl-attachments mirror/main: <sha12>

## READY proof
- READY_COUNT: <1>
- Sole READY item: <NA-xxxx — Example Title>
- Proof source: <NEXT_ACTIONS.md on refreshed main>

## Worktree / branch / PR
- Worktree path: </srv/qbuild/work/example/qsl-protocol>
- Branch: <example-branch>
- PR: <PR# or URL>
- Merge commit: <sha12 or n/a>

## Failures / recoveries
- <command> -> recoverable because <reason>; corrected by <action>; final result <pass>

## Validation / CI notes
- Local validation: <goal-lint, link-check, leak-safe scan>
- Protected checks: <attached and green>
- Retry notes: <none>

## Disk watermark
- Filesystem: </srv/qbuild>
- Total GiB: <000>
- Used GiB: <000>
- Free GiB: <000>
- Used %: <00%>

## Next-watch items
- <watch item or none>
```
