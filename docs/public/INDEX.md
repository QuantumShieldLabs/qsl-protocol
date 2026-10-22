# Evidence and security status

QSL publishes specifications, tests, analysis and known limits for review. The project remains research-stage, not independently audited and not production-ready.

## Read the evidence in context

| Question | Evidence route | Limit |
|---|---|---|
| What does the protocol specify? | [Canonical documents](../canonical/) | Draft normative text is not implementation evidence. |
| What behavior do vectors cover? | [Suite-2 inputs](../../inputs/suite2/) | A defined category is not evidence that every case executed. |
| Which claims have supporting artifacts? | [Release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md) | Check the named revision, assumptions and outstanding gaps. |
| What should an external reviewer inspect? | [Review package](EXTERNAL_REVIEW_PACKAGE.md) | A package is not a completed external audit. |
| What may public copy say? | [Claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) | Architecture descriptions do not establish production assurance. |
| How did published evidence develop? | [Progress archive](PROGRESS.md) | Dated entries describe historical states. |
| What do the demonstrations establish? | [Demo acceptance criteria](../demo/DEMO_ACCEPTANCE_CRITERIA.md) | Synthetic experiments are not deployment qualification. |
| What is the service boundary? | [Relay and attachment service plan](QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md) | A plan does not establish production readiness. |

## Security properties have different evidence

Content confidentiality, authentication, durability, recovery, forward secrecy, post-compromise behavior and metadata privacy need separate arguments. A relay that cannot decrypt content may still observe identifiers, sizes, timing and access patterns. QSL does not currently establish anonymity or protection against global traffic correlation.

Bounded formal models use stated abstractions. Tests execute specific source and configurations. Neither alone proves the whole application or deployment secure.

## Review and report

Inspect [traceability](../../TRACEABILITY.md) for requirement/evidence links and the repository history for changes since a report. When citing a result, retain its source revision, commands, scope and unresolved limits.

For specification ambiguity or a nonsensitive reproducible issue, follow [CONTRIBUTING.md](../../CONTRIBUTING.md). For a sensitive finding, use [SECURITY.md](../../SECURITY.md) and keep exploit details private.
