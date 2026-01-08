# Public Release Workspace and Naming (DRAFT)

## Local workspace layout
Root: `/tmp/qsl-public-release`

Directories:
- `exports/` — allowlist-derived export trees
- `reports/` — export reports and diff logs
- `bundles/` — tar.gz bundles and sha256 files
- `staging/` — pre-export staging area

Permissions:
- Create all workspace directories with mode `0700`.

## Naming conventions
- Export run id: `export_<SHORTSHA>` (SHORTSHA = first 7 of `git rev-parse HEAD`)
- Export dir: `/tmp/qsl-public-release/exports/export_<SHORTSHA>/`
- Bundle: `/tmp/qsl-public-release/bundles/qsl_public_export_<SHORTSHA>.tar.gz`
- Report: `/tmp/qsl-public-release/reports/export_<SHORTSHA>_report.txt`
- Checksums: sha256 recorded in report and as `*.sha256` next to the bundle

## Branch and PR naming
- Branch prefix for this track: `public/<topic>`
  - Example: `public/repo-baseline-licensing`
- PR title prefix for this track: `Public release:`
