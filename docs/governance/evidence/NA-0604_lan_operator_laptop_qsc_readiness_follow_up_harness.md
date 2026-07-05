# NA-0604 LAN Operator Laptop qsc Readiness Follow-Up Harness

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-05

Goals: G1, G2, G3, G4, G5

## Summary

NA-0604 consumed the D-1197/D-1198 NA-0603 handoff and fresh qwork proof from
`2026-07-05T15:54:03Z`. It generated a local-only operator laptop readiness
packet, consumed the returned class-safe laptop proof, and classified the qsc
readiness gap as `NA0604_QSC_TLS_REQUIRED_POLICY_CONFIRMED`.

The laptop qsc binary was ready, existing private values were present only on
the laptop, local qsc state setup passed, seed-fallback environment class was
absent, cleanup was complete, and private-value publication class was `no`.

## Result Classification

`NA0604_QSC_TLS_REQUIRED_POLICY_CONFIRMED`

The endpoint class was `http_non_loopback`; qsc endpoint policy expected class
was `tls_required_expected`; and qsc endpoint policy probe class was
`tls_required_gap_confirmed`.

This aligns with the read-only qsc source review: qsc accepts HTTPS relay
endpoints generally, accepts HTTP only for loopback endpoints, and rejects HTTP
non-loopback endpoints with the TLS-required policy before network send.

## Source Review

- qsc endpoint policy source: `qsl/qsl-client/qsc/src/adversarial/route.rs`
- qsc policy test surface: `qsl/qsl-client/qsc/src/main.rs`
- HTTPS endpoint class: accepted by policy
- HTTP loopback endpoint class: accepted by policy
- HTTP non-loopback endpoint class: TLS-required policy reject

No source, test, workflow, dependency, or lockfile mutation occurred.

## Operator Boundary

The operator ran the laptop packet manually. Codex did not run commands on the
laptop, did not SSH to the laptop, did not install or enable an SSH server, and
did not set up a second Codex on the laptop.

## Private-Material Boundary

The class-safe laptop result had:

- private IPv4 literal count: 0
- URL literal count: 0
- auth value count: 0
- exact prior private-value hit count: 0
- `private_values_published_class`: `no`

No raw endpoint, private port, hostname, topology, token, Authorization value,
route token, capability value, payload/body/plaintext, ciphertext body, seed,
key material, raw command line, raw log, or private material is published.

## Seed Fallback / Cleanup

Seed fallback environment class was `absent`. Laptop cleanup class was
`complete`.

## Successor / Closeout Boundary

NA-0604 identifies the next technical need as a TLS endpoint strategy or
authorization decision before retrying the LAN tiny-message path. This evidence
does not edit `NEXT_ACTIONS.md`, does not mark NA-0604 DONE, and does not
restore a successor because no approved successor block was provided for
closeout.

Recommended successor class: LAN qsc HTTPS/TLS relay endpoint strategy
authorization. The future successor should not weaken qsc endpoint policy, and
should decide between a private-LAN HTTPS relay strategy, a TLS-capable relay
fronting strategy, or another explicit security-reviewed path.

## Claim Boundary

NA-0604 does not prove LAN tiny-message E2EE delivery. It does not prove
qsl-server TLS readiness. It makes no public readiness claim. It makes no
production readiness claim. It makes no remote readiness claim. It makes no
Tailnet readiness claim. It makes no LAN readiness claim. It makes no crypto
completion claim. It makes no attachment completion claim. It makes no
vulnerability freedom claim. It makes no bug freedom claim. It makes no side-channel freedom claim.
