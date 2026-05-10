Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# Attachment Public Demo Readiness

## Proof Mode

NA-0260 uses a minimal demo-only attachment evidence surface in `qshield` and
`scripts/ci/demo_cli_smoke.sh`.

The proof mode is Path 2 from the directive: a bounded qshield public demo
surface creates an encrypted attachment descriptor message and an encrypted
attachment ciphertext message, stores both as opaque Suite-2 demo wires in the
local demo relay queue, fetches them through the existing authenticated relay
poll path, decrypts them on the receiver side, validates descriptor-bound
ciphertext integrity, and writes the plaintext only to the receiver output
directory after validation.

Run:

```bash
scripts/ci/demo_cli_smoke.sh
```

The attachment-specific qshield commands are:

```bash
qshield attachment send --store <alice-store> --peer bob --path <payload> --demo-unauthenticated-override
qshield attachment recv --store <bob-store> --out <output-dir> --demo-unauthenticated-override
qshield attachment send --store <alice-store> --peer bob --path <payload> --demo-unauthenticated-override --tamper-ciphertext
```

The tamper command is test-only and queues a ciphertext that no longer matches
the descriptor hash. The receive command must reject it before writing an
output file.

## Transcript Artifact

Artifact directory:

```text
/srv/qbuild/tmp/NA-0260_attachment_demo_artifacts_20260510T041841Z/
```

Transcript:

```text
/srv/qbuild/tmp/NA-0260_attachment_demo_artifacts_20260510T041841Z/demo_cli_smoke.log
```

## Stable Markers

The demo smoke emits these attachment markers only after the corresponding
checks pass:

```text
DEMO_ATTACHMENT_DESCRIPTOR_OK
DEMO_ATTACHMENT_FETCH_DECRYPT_OK
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
DEMO_ATTACHMENT_NO_SECRET_LEAK_OK
NA0260_ATTACHMENT_DEMO_READY_OK
```

Existing positive, negative, KT-negative, and no-secret markers remain present:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NEGATIVE_KT_NO_MUTATION_OK
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
DEMO_NO_SECRET_LEAK_OK
DEMO_ACCEPTANCE_OK
```

## What Is Proven

- The public qshield demo still initializes two peers, starts the local demo
  relay, registers peers, establishes sessions, sends, receives, and decrypts
  the expected normal message.
- A demo-only attachment descriptor is created and encrypted as a Suite-2 demo
  wire before it reaches the relay.
- A demo-only attachment payload is encrypted as a separate Suite-2 demo wire
  before it reaches the relay.
- The receiver fetches both opaque relay queue entries through the existing
  authenticated poll path.
- The receiver decrypts the descriptor, validates descriptor shape and
  non-production fields, validates the ciphertext length/hash, decrypts the
  attachment payload, and writes the output file only after validation.
- The smoke compares the receiver output file to the sender payload.
- A tampered ciphertext rejects deterministically with
  `attachment_integrity_reject`.
- The tampered reject path writes no receiver output file.
- The demo transcript and relay startup log are checked for relay-token and
  sentinel plaintext leakage.

## What Is Not Proven

- This does not prove qsl-server production relay readiness.
- This does not prove qsl-attachments production service readiness.
- This does not prove production attachment authentication, retention, resume,
  quota, or multi-node durability behavior.
- This does not change protocol, wire, key schedule, crypto state-machine,
  qsp protocol-core, qsc production attachment implementation, qsl-server,
  qsl-attachments, qsc-desktop, website, external website, branch protection,
  public-safety, workflows, or Cargo dependencies.
- This does not claim that the demo relay is a production attachment service.
- This does not claim full metadata privacy; descriptor and traffic metadata
  limitations remain release-readiness topics.

## Opaque-Ciphertext Boundary

The demo relay stores and fetches qshield Suite-2 demo wire hex for both the
descriptor message and attachment payload message. The relay does not receive
attachment plaintext, and the smoke checks that command output plus relay log
evidence do not print the payload sentinel or relay token.

The receiver intentionally writes decrypted plaintext to the chosen output
directory after descriptor and ciphertext integrity checks pass. That output
file is the delivery result, not relay storage evidence.

## Positive Descriptor / Fetch / Decrypt Summary

The positive path:

1. Alice runs `qshield attachment send`.
2. qshield builds a descriptor containing demo-only version/type, attachment id,
   filename hint, ciphertext length, ciphertext SHA-256, `suite2_e2e_demo_wire_v1`
   encryption context, and `qshield_demo_relay_poll_v1` locator kind.
3. qshield encrypts the descriptor and attachment payload through the existing
   refimpl actor Suite-2 demo e2e path.
4. The local demo relay queues only opaque wire hex values.
5. Bob runs `qshield attachment recv`, fetches the pair, decrypts the descriptor,
   validates descriptor-bound ciphertext metadata, decrypts the attachment, and
   writes the payload.
6. The smoke verifies the output bytes match the input bytes before emitting
   `DEMO_ATTACHMENT_FETCH_DECRYPT_OK`.

## Negative / Integrity / Missing-Auth Summary

- Missing relay authorization remains covered by the existing
  `DEMO_NEGATIVE_AUTH_REJECT_OK` proof against the same demo relay auth gate.
- Malformed relay input, invalid relay id, and establish replay remain covered
  by the existing negative demo markers.
- Attachment integrity is covered by the tampered-ciphertext path. The
  descriptor is generated over the original ciphertext, the queued ciphertext is
  then modified, and `qshield attachment recv` rejects with
  `attachment_integrity_reject`.
- The tampered reject path writes no receiver output file before
  `DEMO_ATTACHMENT_INTEGRITY_REJECT_OK` is emitted.

## No Plaintext / Secret Leakage Statement

The smoke uses a sentinel inside the attachment payload and checks command
output plus relay startup output before emitting
`DEMO_ATTACHMENT_NO_SECRET_LEAK_OK`. The relay token is not printed. The
attachment plaintext is expected only in the receiver output file after
successful fetch/decrypt/integrity validation.

## Remaining Gaps

- qsl-server and qsl-attachments production hardening remain separate.
- Cross-host/private-network attachment proof remains future work.
- The qshield attachment surface is a demo proof command, not the qsc
  production attachment implementation.

## NA-0261 Public Summary Refresh

NA-0261 consumes this NA-0260 proof into the public evidence map, external
review package, website handoff, and public demo summary. The refresh does not
strengthen the underlying claim: attachment readiness remains a local,
non-production qshield demo proof, not qsl-server production relay readiness,
qsl-attachments production service readiness, or cross-host/private-network
attachment proof.

## Safe Public Wording

Use:

```text
The public demo includes a non-production attachment proof: it creates an
encrypted descriptor and encrypted attachment payload, fetches opaque relay
entries, decrypts valid payloads on the receiver side, rejects tampered
ciphertext, and checks that relay/demo logs do not expose attachment plaintext
or relay tokens.
```

Do not use:

```text
Attachments are production-ready.
qsl-server or qsl-attachments are production-hardened.
The demo relay is a production attachment service.
Attachment metadata is eliminated.
Tampered attachments are accepted.
```
