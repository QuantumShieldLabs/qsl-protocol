Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# NA-0278 Public README and Branch Cleanup Audit

Directive: QSL-DIR-2026-05-13-081 / NA-0278

## Executive summary

NA-0278 refreshed the public README so an outside reviewer can understand why
QSL exists, what evidence currently supports the project, and where the
research-stage boundaries remain. The change is documentation/governance only:
no protocol, crypto, runtime, service, website, workflow, script, Cargo,
dependency, branch-protection, or public-safety configuration path changed.

NA-0278 also audited stale qsl-protocol branches read-only. Four non-main
branches remain on GitHub. Each maps to a closed/unmerged pull request and is a
candidate for future deletion only if a later directive includes explicit
operator approval. No branch deletion was performed.

## README audit

Baseline findings:

- The README was concise and safe, but the first screen did not explain why
  post-quantum messaging needs protocol evidence beyond algorithm selection.
- The README linked to the onboarding, docs, and public posture front doors, but
  did not point directly to the release evidence map, external review package,
  demo acceptance criteria, traceability, decisions, or service-boundary plan.
- The README already preserved the research-stage/no-production posture.
- The README did not overclaim production readiness, completed external review,
  metadata elimination, or mature production messenger parity.

## README changes made

Changed sections:

- Added `Why this matters` near the top.
- Expanded `Start here` with direct reviewer links.
- Added one evidence-trail bullet to `What this project is`.
- Added explicit negative boundaries to `What this project is not`.
- Added `What is proven now`.
- Added `How to inspect evidence`.

The refresh now states that post-quantum messaging requires evidence for
fail-closed negotiation, replay and identity boundaries, metadata visibility,
demo reproduction, and service-hardening limits. The wording remains bounded to
current evidence and does not claim release approval.

## Public claim safety

Protected claim boundaries:

- QSL remains research-stage and is not approved for production use.
- No production deployment or service readiness is claimed.
- No quantum-proof claim is made.
- No anonymity, untraceable, or metadata-free messaging claim is made.
- External review is presented as a package/readiness surface, not completed
  approval.
- No proven true Triple Ratchet claim is made.

## Evidence links added or checked

README links added or made direct:

- [START_HERE.md](../../../START_HERE.md)
- [docs/INDEX.md](../../INDEX.md)
- [docs/public/INDEX.md](../../public/INDEX.md)
- [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [docs/public/EXTERNAL_REVIEW_PACKAGE.md](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md](../../public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [DECISIONS.md](../../../DECISIONS.md)

These links support the reviewer path without creating new required meaning
outside the existing governance spine and public evidence docs.

## Stale branch audit

Read-only commands used:

- `gh repo view QuantumShieldLabs/qsl-protocol --json defaultBranchRef,nameWithOwner`
- `gh api /repos/QuantumShieldLabs/qsl-protocol/branches --paginate`
- `gh pr list --repo QuantumShieldLabs/qsl-protocol --state open --json number,title,headRefName,headRefOid,baseRefName,url --limit 200`
- `gh pr list --repo QuantumShieldLabs/qsl-protocol --state closed --json number,title,headRefName,headRefOid,baseRefName,mergedAt,mergeCommit,url --limit 200`
- targeted `gh pr list --state all --head <branch>` checks for each stale branch
- targeted `gh pr view` checks for PR #657, #660, #722, and #750

Branch inventory:

| Branch | SHA | Protected | Associated PR | PR state | Merged | Active in open PR | Active evidence reference | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `main` | `5de144526fdb` | yes | n/a | n/a | n/a | n/a | default branch | keep |
| `na-0221-handshake-fail-closed` | `d2967a24378d` | no | #657 | closed | no | no | no current active reference found | candidate for future deletion with explicit approval |
| `na-0221-auth-establishment-fail-closed-v3` | `9c0aad694d74` | no | #660 | closed | no | no | no current active reference found | candidate for future deletion with explicit approval |
| `na-0237a-public-safety-red-main-admission` | `4a066db485a5` | no | #722 | closed | no | no | no current active reference found | candidate for future deletion with explicit approval |
| `na-0250b-public-safety-qsc-adversarial-admission` | `62dafd0c2427` | no | #750 | closed | no | no | no current active reference found | candidate for future deletion with explicit approval |

Open PR inventory:

- `gh pr list --state open` returned `[]`.

## PR mapping

- PR #657 maps to `na-0221-handshake-fail-closed`; state `CLOSED`;
  `mergedAt` null; no merge commit.
- PR #660 maps to `na-0221-auth-establishment-fail-closed-v3`; state
  `CLOSED`; `mergedAt` null; no merge commit.
- PR #722 maps to `na-0237a-public-safety-red-main-admission`; state
  `CLOSED`; `mergedAt` null; no merge commit.
- PR #750 maps to `na-0250b-public-safety-qsc-adversarial-admission`; state
  `CLOSED`; `mergedAt` null; no merge commit.

## Future branch deletion approval requirements

No branch deletion is authorized by NA-0278.

If deletion is desired later, a future directive must include an explicit line
substantially equivalent to:

`Operator approves deleting the following qsl-protocol remote branches: na-0221-handshake-fail-closed, na-0221-auth-establishment-fail-closed-v3, na-0237a-public-safety-red-main-admission, na-0250b-public-safety-qsc-adversarial-admission.`

That future directive should re-verify open PR state, branch protection, branch
SHAs, and governance references immediately before deletion.

## Validation summary

Planned validation for this docs/governance lane:

- queue helper reports READY_COUNT `1` and READY `NA-0278`.
- decisions helper reports D-0526 once and no duplicate decision IDs.
- scope guard reports only allowed README/docs/governance/testplan/decision/
  traceability/journal paths.
- link-check passes.
- added-line leak scan passes.
- overclaim scan finds no affirmative unsupported claims.
- `cargo audit --deny warnings` passes.
- `cargo tree -i rustls-webpki --locked` reports `rustls-webpki v0.103.13`.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
  passes.
- formal/model checks pass if present.
- required CI passes before merge.

## Known limitations

- The branch audit is a point-in-time GitHub API audit; it does not delete
  branches.
- Branch cleanup recommendations are operational hygiene recommendations only.
- README improvements make the reviewer path clearer; they do not create new
  protocol evidence or production readiness.
- External review remains not complete.
- Rate limiting and global route-count caps in qsl-server remain future-gated.
