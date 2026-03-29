Goals: G4, G5

Status: Supporting
Owner: QSC maintainers
Last-Updated: 2026-03-29

# QSC Front Door

Use this directory as the qbuild-first, AWS-free operator front door for `qsc`.

## Validated baseline

- Build from a fresh local checkout (qbuild-scoped when applicable):
  `cargo build -p qsc --release --locked`
- Use [LOCAL_TWO_CLIENT_RUNBOOK.md](./LOCAL_TWO_CLIENT_RUNBOOK.md) for the current truthful
  end-to-end baseline.
- That baseline is the one aligned to current TUI/operator behavior, route-token header carriage,
  and the validated post-`w0` migration posture.

## Compatibility surfaces

- [REMOTE_TWO_CLIENT_AWS_RUNBOOK.md](./REMOTE_TWO_CLIENT_AWS_RUNBOOK.md) is compatibility-only
  evidence for a non-baseline remote lane.
- [REMOTE_SOAK_PLAYBOOK.md](./REMOTE_SOAK_PLAYBOOK.md) is compatibility-only operational evidence.
- `docs/qsc/DOC-QSC-003_*` and `docs/qsc/DOC-QSC-004_*` remain supporting packaging/demo context,
  but they are not the operator front door.

## Product-surface guardrails

- Do not paste route tokens, bearer tokens, passphrases, or other secrets into logs.
- Canonical relay examples use `X-QSL-Route-Token` headers and token-free `/v1/pull?max=N` paths.
- When `QSC_ATTACHMENT_SERVICE` is set, the validated post-`w0` lane uses `w2` for new
  `<= 4 MiB` sends and defaults legacy receive handling to retired.
