# Provenance Guide

This document explains how to verify that a checkout and related proof evidence are official for this repository.

## What "official" means

- Source of truth is the `main` branch in:
  - https://github.com/QuantumShieldLabs/qsl-protocol
- Governance state and work evidence are tracked in:
  - `NEXT_ACTIONS.md`
  - `TRACEABILITY.md`
  - `DECISIONS.md`
- CI lanes must be green for merged changes.

## Verify a local checkout

From repository root:

```bash
git remote -v
git rev-parse HEAD
git log -1 --oneline
```

Confirm the commit SHA appears in merged PR evidence in `TRACEABILITY.md`.

## CI proof lanes

- Handshake proof lane:
  - Workflow: https://github.com/QuantumShieldLabs/qsl-protocol/actions/workflows/remote-handshake-tests.yml
  - Purpose: proves handshake-established `ACTIVE(reason=handshake)` and bidirectional pack/unpack behavior without seed fallback.

- Transport health lane (`seed_fallback_test`):
  - Workflow: https://github.com/QuantumShieldLabs/qsl-protocol/actions/workflows/remote-relay-tests.yml
  - Purpose: relay transport reliability checks only; this is not handshake proof.

Use `TRACEABILITY.md` for pinned run links associated with completed NAs.

## Artifact trust model

- Trust source + commit + CI run linkage together.
- Prefer artifacts from linked workflow runs in `TRACEABILITY.md`.
- Treat detached binaries without source/commit/run linkage as untrusted.

## Log hygiene

- Logs and markers are expected to avoid secrets.
- Do not include tokens, private keys, bearer values, or credentials in shared artifacts.
