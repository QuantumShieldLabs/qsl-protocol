# DOC-QSC-003 Demo Packaging Runbook (Security Lens + Relay)

## Purpose
Provide a quick, safe, deterministic demo of qsc + relay hostile-network behavior that is shareable and charter-enforced.

## Prerequisites
- Rust toolchain (cargo + rustc)
- No sudo required
- Optional: isolated CARGO_HOME for cache-permission safety

## Quickstart (copy/paste)
1) Build qsc (local):
   - cargo build -p qsc --locked
2) Run relay demo (full run):
   - ./scripts/demo/qsc_demo_local.sh --seed 42 --scenario drop+reorder
3) Dry-run (offline; prints commands only):
   - ./scripts/demo/qsc_demo_local.sh --seed 42 --scenario drop+reorder --dry-run
4) Inspect marker logs (safe to share):
   - cat _demo_out/<run>/alice.markers

## Scenarios (seeded)
- happy-path
- drop
- reorder
- drop+reorder
- seeded replay

## What you should see (examples)
- QSC_MARK/1 event=send_prepare ...
- QSC_MARK/1 event=relay_event value=drop|reorder
- QSC_MARK/1 event=send_commit ... (only on success)

## Safety notes
- Do not include secrets in marker/log output.
- Logs are safe to share when redacted (default behavior).

## Troubleshooting
- Cache permissions: use isolated CARGO_HOME/CARGO_TARGET_DIR under _forensics.
- gh log retrieval: set GH_CACHE_DIR under _forensics.

## Full-run addendum
See DOC-QSC-004 Demo Full-Run Addendum for deterministic artifact details.
