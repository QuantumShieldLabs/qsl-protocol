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

### Local self-invitation guard (NA-0780)

Desktop Connect can call `qsc::facade::invite_preflight(code, self_label)` before
redemption. On `FacadeError::SelfInvitation` (`self_invitation`), show:
“This invitation was created by this app. Ask the other person for their invitation.”
The redemption handler repeats the same ownership check before saving redemption
state, contacting the relay, or creating a pending contact. Direct engine callers
receive `invite_self`. Do not cache preflight success across edits or lock changes.

Ownership uses all retained mint IDs regardless of display state, then compares
the invitation's commitment with the selected existing public identity. Clearing
creating rows does not hide invitations minted with the current keys. These reads
do not create, migrate, or rotate keys. A locked vault, unreadable mint store, or
invalid/incomplete public identity refuses the check; a legacy public record with
no signing key cannot establish commitment ownership without retained mint history.
The facade reports unavailable public ownership data as `store_unavailable`.

If both an old mint record and its original public identity have been removed
(for example, cleared history followed by identity rotation), available storage
cannot establish ownership of that old invitation. Missing identity plus absent
mint history has the same limitation. Preflight success means only that available
local data did not identify a self invitation. It does not authenticate the code,
check relay availability, or replace redemption's expiry, commitment, signature,
and single-use checks. The existing single-identity label resolution still applies.
Local fixture tests do not establish two-device acceptance.

Complete historical recognition would require additional retained ownership
information, for example historical public identity commitments or minted IDs
that survive clear, rotation and deletion. Choosing retention, privacy and
explicit deletion behavior requires separate design approval; this milestone
adds no such storage and does not fully satisfy rejection of every historically
self-minted invitation.
