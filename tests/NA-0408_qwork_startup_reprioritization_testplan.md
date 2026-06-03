Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-03

# NA-0408 qwork Startup Reprioritization Testplan

## Purpose

Prove that the governance reroute explicitly defers the previous backup
manifest/status lane, promotes qwork startup hardening as the sole active READY
lane, and does not implement qwork or mutate runtime, dependency, workflow,
backup, public, or sibling-repo surfaces.

## Scope

Allowed mutation paths for this testplan:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0408_qwork_startup_reprioritization_testplan.md`

Forbidden effects:

- qwork implementation.
- backup execution or restore execution.
- qsl-backup, backup source-list, backup status, or backup plan mutation.
- durable Director State Index output.
- runtime, protocol, crypto, dependency, workflow, public-doc, website,
  README, START_HERE, qsl-server, or qsl-attachments mutation.
- public-readiness, public-technical-paper, backup-complete, restore-proof, or
  off-host-backup claims.

## Deterministic Checks

Run from the qsl-protocol repo root.

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 - <<'PY'
from pathlib import Path

checks = {
    "D-0797": Path("DECISIONS.md").read_text().count("- **ID:** D-0797"),
    "D-0798": Path("DECISIONS.md").read_text().count("- **ID:** D-0798"),
    "D-0799": Path("DECISIONS.md").read_text().count("- **ID:** D-0799"),
    "D-0800": Path("DECISIONS.md").read_text().count("- **ID:** D-0800"),
}
for key, count in checks.items():
    print(f"{key} {count}")
PY
git diff --name-only origin/main...HEAD
git diff --check
```

Expected results:

- `READY_COUNT 1`.
- `READY NA-0408 QSL Local Ops qwork Unified Startup Command Implementation Harness`.
- `NA-0407 DONE`.
- `D-0797 1`.
- `D-0798 1`.
- `D-0799 1`.
- `D-0800 0`.
- `NA-0409 -- QSL Codex Ops Backup Coverage Manifest Verification / Status Update Plan`
  exists and is not READY.
- Changed paths are exactly the allowed reroute paths.
- `git diff --check` exits zero.

## Link and Leak Checks

```bash
python3 - <<'PY'
import pathlib, re

repo = pathlib.Path(".").resolve()
md_files = []
for pattern in ("*.md", "**/*.md"):
    for p in repo.glob(pattern):
        if ".git/" in p.as_posix():
            continue
        if p.is_file():
            md_files.append(p)
md_files = sorted(set(md_files))

link_re = re.compile(r'\[[^\]]+\]\(([^)#]+)(?:#[^)]+)?\)')
missing = []

for md in md_files:
    text = md.read_text(encoding="utf-8", errors="replace")
    for raw in link_re.findall(text):
        target = raw.strip()
        if not target or "://" in target or target.startswith("mailto:"):
            continue
        if target.startswith("<") and target.endswith(">"):
            target = target[1:-1]
        abs_target = (md.parent / target).resolve()
        if not abs_target.exists():
            missing.append((md.relative_to(repo).as_posix(), target))

for src, target in missing:
    print(f"MISSING_LINK {src} -> {target}")
print(f"TOTAL_MISSING {len(missing)}")
PY

git diff --cached --unified=0 | rg -n "BEGIN .*PRIVATE|PRIVATE KEY|AWS_ACCESS_KEY|SECRET_ACCESS_KEY|TOKEN=|PASSWORD=|PASSWD=|Authorization:" || true
```

Expected results:

- `TOTAL_MISSING 0`.
- No high-confidence secret findings in staged added lines.

## Dependency and Required Safety Checks

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
```

Expected results:

- `cargo audit --deny warnings` exits zero.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13`
  or a newer safe version.
- GitHub branch protection still requires `public-safety`.
- `public-safety` is green on the PR head before merge and on `origin/main`
  after merge.

## Acceptance

This reroute passes only if it preserves the old backup manifest/status lane as
NA-0409, promotes qwork startup hardening as the sole READY NA-0408 lane, adds
D-0799 exactly once, keeps D-0800 absent, and leaves all forbidden surfaces
unchanged.
