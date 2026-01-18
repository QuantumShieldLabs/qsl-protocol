# DOC-DEV-002 — CodeQL Operating Procedure v1.0.0 (DRAFT)

Status: DRAFT
Date: 2026-01-18 (America/Chicago)
Owner: QSL governance
Goals: G2, G3

## 1. Purpose
CodeQL is treated as a continuous security regression test for QSL:
- CI CodeQL is the authoritative gate on PRs.
- Local CodeQL is a fast targeted regression check for high-risk diffs (crypto/ratchet/handshake/state paths).

This is not a one-time scan; it is a persistent control to prevent quiet security regressions.

## 2. CI Policy (Authoritative)
- GitHub Actions CodeQL remains required on PRs touching security-sensitive code.
- CI CodeQL is the source of truth for merge readiness.
- If CodeQL flags an issue, PR must not merge until triaged and addressed per Section 4.

## 3. Local Fast CodeQL (Targeted Query Only)
Local CodeQL can be slow if you run full suites. The intended local use is a single high-signal query:
- rust/hard-coded-cryptographic-value

### 3.1 Artifact hygiene (must not dirty the repo)
Do not keep CodeQL DBs or SARIF files in the repo working tree.

Store outputs under:
- /home/victor/work/qsl/_forensics/codeql_fast_<RUN_ID>/

Optional local-only excludes (NOT committed):
- Add to .git/info/exclude:
  - _codeql_db/
  - *.sarif

This prevents “dirty tree” stops without changing repo policy.

### 3.2 Avoiding pack download/auth failures
`codeql pack download github/codeql` can fail with 403 (GHCR auth). The fast local check must not require downloads.
Instead, use the locally installed query packs under:
- $HOME/.codeql/packages/codeql/rust-queries/<version>/

### 3.3 Fast local script (copy/paste)
This runs only the hard-coded crypto query and summarizes SARIF.

```bash
#!/usr/bin/env bash
set -euo pipefail

REPO="/home/victor/work/qsl/qsl-protocol"
RUN_ID="$(date -u +%Y%m%dT%H%M%SZ)"
OUT="/home/victor/work/qsl/_forensics/codeql_fast_${RUN_ID}"
DB="$OUT/db"
SARIF="$OUT/hardcoded_crypto.sarif"

mkdir -p "$OUT"
cd "$REPO"

# Must be clean
git status --porcelain=v1

# Locate installed Rust query pack locally (no downloads)
RUST_PACK_DIR="$(ls -d "$HOME/.codeql/packages/codeql/rust-queries/"* 2>/dev/null | sort -V | tail -n 1)"
QUERY_QL="$(rg -l --no-messages "@id\\s+rust/hard-coded-cryptographic-value" "$RUST_PACK_DIR/queries" | head -n 1)"
test -f "$QUERY_QL"

rm -rf "$DB"
codeql database create "$DB" \
  --language=rust \
  --source-root="$REPO" \
  --command="cargo build -q -p quantumshield_refimpl"

rm -f "$SARIF"
codeql database analyze "$DB" "path:$QUERY_QL" \
  --format=sarifv2.1.0 \
  --output="$SARIF" \
  --threads=0

python3 - <<'PY' "$SARIF"
import json, collections, sys, pathlib
p = pathlib.Path(sys.argv[1])
data = json.loads(p.read_text(encoding="utf-8"))
ctr = collections.Counter()
for run in data.get("runs", []):
    for res in run.get("results", []):
        ctr[res.get("ruleId","<no-ruleId>")] += 1
print("SARIF:", p)
print("Total results:", sum(ctr.values()))
for rid, n in ctr.most_common(50):
    print(f"{n:5d}  {rid}")
PY

echo "OUT=$OUT"
```

## 4. Triage Rules (Mandatory)
### 4.1 “Hard-coded cryptographic value”
Rule intent: hard-coded passwords/keys/IVs/salts used for crypto operations are unsafe.

Triage decision:

- Real bug: value can reach crypto ops as a key/nonce/salt (or equivalent).
  Required response:
  1) Fix the issue.
  2) Add a targeted regression test if applicable:
     - deterministic reject behavior
     - no mutation on reject for stateful rejects
  3) Record evidence in DECISIONS + TRACEABILITY when required by repo policy.

- Safe sentinel / test helper (e.g., all-zero used as “unset”):
  Allowed only if ALL are true:
  1) The value is clearly documented as a sentinel (not key material).
  2) There are guardrails: fail-closed at point-of-use so it can never be consumed as key material.
  3) There is a regression test proving the guardrails (deterministic reject; no mutation on reject where stateful).
     Only after (1)-(3) may suppression be considered, and any suppression must reference the governing DECISIONS entry (and spec/audit anchors if applicable).

## 5. API visibility for alert listing (gh api)
Listing code scanning alerts commonly fails with 404/403 without the correct scope.

Requirement:
- Ensure your auth token has security_events scope.

Examples:
- gh auth refresh -h github.com -s security_events
- gh api /repos/QuantumShieldLabs/qsl-protocol/code-scanning/alerts

If you get 404/403, treat it as a permissions/config issue unless independently confirmed otherwise.

## 6. When to run the local fast check
Run the targeted local query before pushing changes that touch:
- tools/refimpl/** (crypto, ratchet, handshake, state)
- tools/actors/**
- any parsing/encoding boundary that could become attacker-controlled

Otherwise rely on CI CodeQL.
