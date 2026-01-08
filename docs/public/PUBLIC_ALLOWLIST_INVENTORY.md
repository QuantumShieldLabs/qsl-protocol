# Public Allowlist Inventory (DRAFT)

## Proposed allowlist
- docs/canonical/**
- docs/privacy/**
- docs/public/**
- docs/INDEX.md
- inputs/suite2/vectors/**
- DECISIONS.md
- TRACEABILITY.md
- NEXT_ACTIONS.md
- README.md
- LICENSE
- SECURITY.md
- CONTRIBUTING.md
- THIRD_PARTY_NOTICES.md
- CODE_OF_CONDUCT.md
- SUPPORT.md
- .github/CODEOWNERS
- .github/ISSUE_TEMPLATE/**
- .github/PULL_REQUEST_TEMPLATE.md
- .github/workflows/public-ci.yml
- scripts/ci/** (pending secret review)

## Proposed denylist
- Any file matching: .env, *.pem, *.key, *.p12, *secrets*, *credentials*
- Operational configs, tokens, internal endpoints, or private infrastructure
- Anything not required for protocol/spec/vectors/governance/public CI

## Scan commands executed
- ls -la
- find docs -maxdepth 3 -type d -print | sort
- find inputs -maxdepth 4 -type d -print | sort
- find scripts -maxdepth 3 -type d -print | sort
- find .github -maxdepth 3 -type f -print | sort
- rg -n "token|secret|apikey|api_key|password|passwd|bearer|Authorization|PRIVATE_KEY|BEGIN .* PRIVATE" -S .
- rg -n "ssh-rsa|ed25519|BEGIN RSA|BEGIN OPENSSH" -S .
- rg -n "http(s)?://|@" -S docs scripts .github || true

## Findings summary (file:line)
- docs/privacy/DOC-G5-002_Metadata_Leakage_Inventory_v1.0.0_DRAFT.md: token/bearer references in metadata posture.
- docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md: token handling guidance.
- scripts/ci/metadata_conformance_smoke.sh: Authorization header usage with QSHIELD_RELAY_TOKEN.
- scripts/ci/demo_cli_smoke.sh: relay token usage in demo CLI setup.
- apps/qshield-cli/README.md: relay token usage in demo examples.
- docs/review/DOC-REV-001_Signal_Comparative_Review_v1.0.0_DRAFT.md: external URLs cited.

## Go/No-Go criteria for scrub PR
- Allowlist matches actual public export paths and is documented.
- Denylist patterns return zero matches in allowlisted paths.
- No secrets or credentials found in allowlisted content.
- CI guardrails for allowlist/denylist checks pass.
