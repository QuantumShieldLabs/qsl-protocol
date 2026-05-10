Goals: G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-05-09

# Demo Acceptance Criteria

## Posture

The demo is a non-production acceptance surface. It exists to prove inspectable protocol-adjacent behavior, reject behavior, and metadata constraints. It must not claim production readiness.

## One-command target

The accepted demo shape is a single local command or CI target that sets up two demo peers, starts only local loopback services, performs the required positive and negative flows, and exits nonzero on the first failed invariant.

The command should be deterministic enough for CI and local Linux execution, and the relevant subset should remain inspectable on macOS through protected build/smoke checks.

## Valid flow acceptance

A valid demo run must prove:

- peer stores initialize with safe local permissions
- relay configuration is explicit
- peer registration succeeds with required authorization
- establish succeeds only with the explicit demo identity override or verified identity path
- send succeeds
- decrypt/receive releases the expected plaintext to the intended peer
- output keeps the demo-only warning posture visible

## Reject-flow acceptance

The demo acceptance target must include negative cases for:

- downgrade or unsupported-suite attempt
- malformed establish or malformed message input
- missing or invalid relay authorization
- replayed or stale establish record where applicable
- invalid KT or malformed KT evidence once the demo path carries KT evidence

Reject cases must be fail-closed and must not silently fall back to weaker behavior.

## KT negative evidence scenario

The demo acceptance lane includes a bounded, non-production KT-negative proof
through the demo smoke surface. The proof invokes the canonical KT verifier
vectors and accepted-state no-mutation regression, emits stable KT markers, and
keeps the limitation explicit: it does not mean the `qshield` establish command
accepts live user-supplied KT evidence.

The accepted behavior is deterministic rejection for invalid KT evidence, no
downgrade to implicit disabled KT mode, no accepted KT state mutation on the
proved reject path, and no production KT readiness claim.

## Attachment path

If attachment demo support is active for the selected lane, acceptance must prove descriptor validation, fetch/decrypt, integrity checks, and final delivery-state transition for the happy path. It must also reject malformed descriptors, integrity mismatch, stale fetch capability, and oversize or unsupported forms without releasing plaintext or falsely confirming peer delivery.

The NA-0260 qshield demo proof is an active non-production attachment proof. It
must emit:

```text
DEMO_ATTACHMENT_DESCRIPTOR_OK
DEMO_ATTACHMENT_FETCH_DECRYPT_OK
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
DEMO_ATTACHMENT_NO_SECRET_LEAK_OK
NA0260_ATTACHMENT_DEMO_READY_OK
```

The accepted NA-0260 behavior is bounded to encrypted descriptor and attachment
payload demo wires through the local qshield relay, receiver-side integrity
validation before output write, tampered-ciphertext rejection, and checked
transcript/relay-output no-leak proof. It does not imply production
qsl-server, qsl-attachments, production relay, or release readiness.

If attachment demo support is not active for the selected lane, the acceptance report must say so and avoid implying attachment readiness.

## Metadata expectations

The demo acceptance target should preserve:

- loopback-only default service binding
- explicit unsafe acknowledgement for non-loopback service binding
- required relay authorization
- store directory and file permission checks
- bounded queue and polling behavior
- stable, documented output markers for CI
- no credential-bearing operator material exposure in evidence

## Non-goals

The demo does not prove production deployment readiness, production authentication UX, production relay availability, attachment-service hardening, or full metadata privacy. Those require separate release-readiness evidence.

## CI linkage

`demo-cli-build`, `demo-cli-smoke`, and `metadata-conformance-smoke` remain the required protected demo/metadata contexts. Docs-only PRs may resolve them through the existing docs/governance-only path, but implementation lanes must run the executable smoke commands.
