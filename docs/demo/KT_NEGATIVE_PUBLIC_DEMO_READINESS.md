Goals: G1, G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# KT-Negative Public Demo Readiness

## Proof Mode

NA-0259 uses a minimal demo-only KT evidence surface in
`scripts/ci/demo_cli_smoke.sh`.

The proof does not add live KT evidence input to `qshield establish`. Instead,
the one-command demo smoke now runs the already-merged canonical KT verifier
evidence after the normal qshield positive and negative demo flows:

```bash
scripts/ci/demo_cli_smoke.sh
```

The KT portion invokes:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
```

This is a truthful public demo readiness proof because the public demo runner
now carries KT-related evidence through a bounded verifier path and exposes
stable pass/fail markers. It is not a production KT deployment proof.

## Transcript Artifact

Artifact directory:

```text
/srv/qbuild/tmp/NA-0259_kt_negative_demo_artifacts_20260510T002546Z/
```

Transcript:

```text
/srv/qbuild/tmp/NA-0259_kt_negative_demo_artifacts_20260510T002546Z/demo_cli_smoke_kt_negative_transcript.log
```

## Stable Markers

The demo smoke now emits these KT readiness markers only after the corresponding
commands pass:

```text
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NEGATIVE_KT_NO_MUTATION_OK
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
NA0259_KT_NEGATIVE_DEMO_READY_OK
```

Existing positive and negative markers remain:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
DEMO_NO_SECRET_LEAK_OK
DEMO_ACCEPTANCE_OK
```

## What Is Proven

- The qshield public demo still initializes two peers, starts a loopback relay,
  registers peers, establishes sessions, sends, receives, and decrypts the
  expected plaintext.
- Missing relay authorization, malformed relay JSON, invalid relay id, and
  establish replay still reject fail-closed.
- The canonical KT verifier accepts valid first-seen and advanced KT evidence.
- The canonical KT verifier rejects stale STH, missing STH, inclusion mismatch,
  and missing consistency proof cases from the KT vector set.
- A rejected KT consistency advancement leaves the accepted KT state unchanged.
- Disabled KT shape is accepted only through the explicit non-production
  boundary tested by the verifier.
- The demo transcript is checked for relay-token and sentinel leakage.

## What Is Not Proven

- This does not prove production KT deployment readiness.
- This does not prove that `qshield establish` accepts arbitrary live
  user-supplied KT evidence.
- This does not prove cross-host KT-negative behavior.
- This does not change the protocol, wire format, key schedule, crypto state
  machine, qsl-server, qsl-attachments, qsc-desktop, website, or external
  website.

## No Fake KT Evidence Statement

The KT proof uses existing canonical verifier tests and vector-defined
mutations. It does not mark an arbitrary command failure as KT success, does not
fabricate a success transcript, and does not claim that disabled or malformed KT
evidence is valid. KT-negative readiness is limited to deterministic rejection
and accepted-state no-mutation on the proved verifier path.

## No-Mutation Boundary

The no-mutation claim applies to the canonical KT verifier accepted-state
snapshot in
`kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state`.
The test primes an accepted STH, presents a tree-size advancement with invalid
consistency evidence, receives a deterministic KT reject, and compares the
accepted-state snapshot before and after the reject.

The qshield demo stores are not used as KT trust stores in this proof, so this
document does not claim qshield live KT-store mutation behavior.

## Remaining Gaps

- A future lane may add a reviewer-facing qshield subcommand or live demo input
  for KT evidence if it can be done without changing protocol or crypto
  semantics.
- Cross-host KT-negative proof remains separate.
- Production KT service, log operation, and external verification remain out of
  scope.
- `docs/public/**` release-readiness summaries should be refreshed in a later
  public-docs lane if NA-0259 is closed and the Director authorizes that scope.

## Safe Public Wording

Use:

```text
The public demo includes a non-production KT-negative verifier proof: the demo
runner exercises canonical KT vectors, proves deterministic reject behavior for
selected invalid evidence, and proves accepted KT state is unchanged on the
covered reject path.
```

Do not use:

```text
KT is production-ready.
The public demo verifies real-world KT logs.
qshield establish now accepts live KT evidence.
Malformed KT evidence is accepted.
```
