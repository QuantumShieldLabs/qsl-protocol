Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# Demo Adversarial Stress Testing

## Purpose

This document describes the bounded NA-0262 public-demo adversarial stress
harness. The harness pressure-tests the non-production `qshield` demo surface
for fail-closed auth, malformed input, replay, relay identity, queue/cap,
attachment integrity, KT-negative, chaos/restart, leak-safety, and panic-safety
behavior.

The harness is not production hardening evidence. It does not prove production
relay readiness, qsl-server readiness, qsl-attachments readiness, public
internet exposure safety, or release approval.

## Command

Run the baseline profile from the repository root:

```bash
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
```

The default profile is `baseline`, so this is equivalent:

```bash
scripts/ci/demo_adversarial_stress.sh
```

The local/manual profile is:

```bash
DEMO_STRESS_PROFILE=extended scripts/ci/demo_adversarial_stress.sh
```

Both profiles are bounded by `DEMO_STRESS_MAX_RUNTIME_S`, which defaults to
`900` seconds. The harness exits nonzero instead of running indefinitely.

## Artifact Paths

Each run creates a transcript bundle under:

```text
/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_<timestamp>/
```

Important files:

- `demo_adversarial_stress_transcript.log`
- `demo_adversarial_stress_markers.log`
- `direct_relay_abuse.log`
- `demo_cli_smoke.log`
- response bodies and relay logs for the direct local abuse pass

The transcript must not contain relay tokens, injected secret sentinels, or
panic/backtrace output.

## Baseline Profile

The baseline profile is intended for local and CI use. It uses only loopback
targets and temporary process state.

Baseline coverage:

- missing, wrong, wrong-scheme, and empty relay auth rejects;
- malformed JSON, wrong content type, empty body, and bounded oversized body
  rejects;
- invalid relay id reject;
- establish replay reject;
- unauthorized send no-mutation proof where the local relay exposes queue
  observation;
- bounded recipient queue cap proof;
- controlled relay kill/restart recovery proof;
- post-stress positive qshield demo smoke;
- attachment tamper/integrity reject proof through the existing demo smoke;
- KT-negative verifier/no-mutation proof through the existing demo smoke;
- no token/secret/sentinel leak scan; and
- no panic/backtrace/unwrap-output scan.

Expected baseline markers include:

```text
DEMO_STRESS_POSITIVE_BASELINE_OK
DEMO_STRESS_AUTH_REJECT_OK
DEMO_STRESS_MALFORMED_REJECT_OK
DEMO_STRESS_REPLAY_REJECT_OK
DEMO_STRESS_RELAY_ID_REJECT_OK
DEMO_STRESS_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_STRESS_KT_REJECT_OK
DEMO_STRESS_QUEUE_OR_RATE_BOUND_OK
DEMO_STRESS_CHAOS_RECOVERY_OK
DEMO_STRESS_NO_SECRET_LEAK_OK
DEMO_STRESS_NO_PANIC_OK
NA0262_DEMO_ADVERSARIAL_STRESS_OK
```

The baseline profile records `UNSUPPORTED_PORT_IN_USE_BASELINE` because the
port-in-use check is reserved for the extended profile.

## Extended Profile

The extended profile runs the baseline checks and adds a bounded port-in-use
negative proof. It remains local/manual and still targets only loopback.

Expected additional marker:

```text
DEMO_STRESS_EXTENDED_PORT_IN_USE_REJECT_OK
```

## What Is Proven

The harness proves the current non-production demo rejects the selected
unauthorized, malformed, replay, invalid-id, queue-cap, attachment-integrity,
and KT-negative inputs in bounded local runs. It also proves the controlled
relay can be killed and restarted, and that a positive demo path still passes
after the direct stress phase.

Where no-mutation is claimed, it is limited to an observable local relay queue
check for unauthorized send and to existing KT/attachment proof surfaces.

## What Is Not Proven

The harness does not prove:

- production qsl-server readiness;
- production qsl-attachments readiness;
- public internet attack resistance;
- firewall, router, or Tailscale admin safety;
- release readiness;
- live production KT service operation;
- real cross-host/private-network stress; or
- full no-mutation for relay internals that are not externally observable.

## Public Wording

Safe public wording:

> The non-production local demo includes a bounded adversarial stress harness
> that checks selected fail-closed and leak-safe behaviors.

Unsafe wording:

> The demo is production hardened.

> The relay is safe for public internet exposure.

> The attachment or KT services are production ready.

## Known Gaps

- Cross-host/private-network stress remains a successor lane.
- Desktop/sidecar stress remains separate.
- qsl-server and qsl-attachments production hardening remain separate.
- The harness does not claim broad fuzzing or denial-of-service coverage.
