Goals: G5
Status: DRAFT

Scope:
- Governance-only allowlist/denylist inventory for public scrub.

Objective:
- Ensure allowlist/denylist are documented and authoritative for export.

CI-gated assertions:
- Allowlist/denylist documented.
- Denylist patterns return zero matches inside allowlisted paths (manual/commanded verification).
- No secrets found in allowlisted content (manual/commanded verification).

Verification commands (from inventory doc):
- rg -n "token|secret|apikey|api_key|password|passwd|bearer|Authorization|PRIVATE_KEY|BEGIN .* PRIVATE" -S .
- rg -n "ssh-rsa|ed25519|BEGIN RSA|BEGIN OPENSSH" -S .
- rg -n "http(s)?://|@" -S docs scripts .github || true
