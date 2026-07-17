Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-09
Replaces: n/a
Superseded-By: n/a

# NA-0256 Public Demo and Desktop Readiness Audit

Directive: QSL-DIR-2026-05-09-052 / NA-0256
Goals: G1, G4, G5

## Objective

Validate and package the current public demo and desktop prototype touch-and-feel surface with executable local proof, operator-visible artifact references, truthful host limitations, and explicit non-production boundaries.

## Scope Conclusion

NA-0256 is a demo/desktop readiness evidence lane. It does not change protocol wire behavior, crypto state machines, qsl-server, qsl-attachments, website or external website files, Cargo manifests, Cargo lockfiles, `.github`, branch protection, public-safety configuration, or production service implementation.

No runtime source change was required. The patch records validation evidence, D-0479, traceability, a public demo readiness document, this audit, the testplan, and the rolling journal entry.

## Baseline Proof

- Starting `origin/main`: `20c100c1cda4`
- Starting READY state: `READY_COUNT 1`, sole READY `NA-0256 — Public Demo and Desktop Touch-and-Feel Readiness Hardening`
- Decision state before work: D-0478 existed once; D-0479 and D-0480 were absent; duplicate decision count was zero.
- Branch protection: `public-safety` was present in the required contexts with the expected CI contexts.
- Latest main `public-safety`: success on the expected starting main SHA.
- PR preservation: PR #750 and PR #722 were closed and unmerged; PR #708 was merged.

## Artifact Package

Artifact directory:

```text
/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_20260509T044612Z/
```

Manifest:

```text
/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_20260509T044612Z/ARTIFACT_MANIFEST.txt
```

Generated artifacts:

- `demo_cli_smoke.log`
- `metadata_conformance_smoke.log`
- `qshield_help.txt`
- `qshield_relay_help.txt`
- `qshield_establish_help.txt`
- `qshield_send_help.txt`
- `qshield_recv_help.txt`
- `qsc_desktop_npm_ci.log`
- `qsc_desktop_npm_build.log`
- `qsc_desktop_prepare_sidecar.log`
- `qsc_desktop_tauri_build.log`
- `qsc_desktop_contract_test.log`
- `qsc_qsp_protocol_gate_test.log`

No screenshot artifact was generated because the host has no browser or `xvfb`, and native Tauri packaging is host-limited by missing `pkg-config`.

## Local Validation Summary

| Command | Result | Evidence |
| --- | ---: | --- |
| `scripts/ci/demo_cli_smoke.sh` | PASS | `DEMO_ACCEPTANCE_OK`; positive send/receive/decrypt marker; reject markers; no-secret marker. |
| `scripts/ci/metadata_conformance_smoke.sh` | PASS | Ended with `metadata-conformance-smoke: OK`. |
| qshield help output capture | PASS | Help text captured for top-level, relay, establish, send, and recv commands. |
| qshield panic/unwrap scan | PASS | Only direct `unwrap`/`panic` findings in `apps/qshield-cli/src` are test-only relay lock-poison coverage. |
| `npm ci` in `qsl/qsl-client/qsc-desktop` | PASS with warnings | npm reported existing audit warnings; no dependency update or lockfile change was made. |
| `npm run build` in `qsl/qsl-client/qsc-desktop` | PASS | Vite production frontend build completed. |
| `npm run prepare:sidecar` in `qsl/qsl-client/qsc-desktop` | PASS | Built release `qsc` and copied the sidecar into Tauri resources. |
| `npm run tauri:build` in `qsl/qsl-client/qsc-desktop` | HOST-LIMITED | Reached native backend compilation, then failed because `pkg-config` is unavailable for `glib-sys`. |
| `cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1` | PASS | 3 tests passed. |
| `cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1` | PASS | 6 tests passed. |

## Demo Positive Proof

The one-command demo transcript proves:

- two temporary peer stores initialized;
- loopback relay started;
- authorized peer registration succeeded;
- Alice/Bob demo establishment succeeded with explicit demo-only override;
- Alice sent `hello-na0246`;
- Bob received/decrypted the plaintext; and
- the output included the intended sender.

Positive proof marker:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
```

## Demo Negative / Reject Proof

The one-command demo transcript proves:

- missing relay authorization rejects;
- malformed JSON rejects and does not echo the token/sentinel input;
- invalid relay ID rejects through the CLI path; and
- replayed establish records reject with the expected replay marker.

Stable negative proof markers:

```text
DEMO_NEGATIVE_AUTH_REJECT_OK
DEMO_NEGATIVE_MALFORMED_REJECT_OK
DEMO_NEGATIVE_INVALID_RELAY_ID_REJECT_OK
DEMO_NEGATIVE_REPLAY_REJECT_OK
```

The metadata conformance transcript also proves selected sanitized error, permission, rate/queue/quota, padding, bundle-consumption, identity-binding, and replay checks for the current demo surface.

## No Secret / Token Leakage Proof

The demo smoke injects sentinel and relay-token material into rejected inputs and checks command output, response bodies, and relay output before printing:

```text
DEMO_NO_SECRET_LEAK_OK
```

The relay startup text says the token is configured but not printed. qshield help output was captured after correcting the qbuild target path and does not include a live token.

## Desktop Status Summary

The desktop prototype remains a qsc sidecar shell. The passing contract tests prove deterministic sidecar-facing surfaces for profile/doctor/vault/identity markers, contact/device trust surfaces, message delivery, and timeline truth. The passing protocol-gate tests prove inactive send/receive fail closed with `protocol_inactive`, active paths require actual protocol readiness, status output remains secret-safe, and inactive rejects do not create misleading state.

Frontend build and sidecar preparation passed. Native package proof is not claimed because this host lacks the system `pkg-config` prerequisite for the GLib dependency chain.

## Host Limitations

- `pkg-config`: not available.
- global `tauri`: not available, but npm script uses the project Tauri CLI package.
- browser: not available.
- `xvfb`: not available.

Because the directive forbids global system package installation and Cargo dependency changes, the correct outcome is to document native package and screenshot proof as host-limited.

## Recovered Failure Evidence

- Failing command: `target/debug/qshield --help` and related help captures.
- Classification: recoverable command-shape issue. The qbuild environment uses `CARGO_TARGET_DIR=/srv/qbuild/cache/targets/qsl-protocol`, so the expected binary was not under repo-local `target/debug`.
- Corrective action: resolved the real binary path and reran help capture from `$CARGO_TARGET_DIR/debug/qshield`.
- Final result: help output captured successfully in the artifact directory.

- Failing command: `npm run tauri:build`.
- Classification: recoverable host-limited validation outcome. The command completed sidecar prep and frontend build, then failed in native backend compilation because the host lacks `pkg-config` for `glib-sys`.
- Corrective action: did not install global system packages or alter dependencies; recorded package/screenshot proof as host-limited.
- Final result: frontend build, sidecar prep, and desktop contract proof passed; native package/AppImage proof remains open for a provisioned host.

## No-Production-Overclaim Statement

This evidence supports only non-production public demo readiness and bounded desktop prototype readiness. It does not support production-ready desktop, production relay, qsl-server readiness, qsl-attachments readiness, attachment demo readiness, KT-negative public demo readiness, anonymity, metadata elimination, or release approval claims.

## Residual Gaps

- KT-negative public demo proof remains open until the demo surface truthfully carries KT evidence.
- Attachment demo proof remains open until descriptor validation, fetch/decrypt, integrity, and negative rejects are executable in the demo path.
- Native package proof remains open for a fully provisioned Linux/macOS host.
- Screenshot proof remains open for a host with browser/display prerequisites.
- Keychain-backed active operations remain deferred.
- Production relay hardening remains out of scope.
- qsl-server and qsl-attachments production readiness remain out of scope.
- Cross-host/Tailscale demo reproducibility remains a successor lane.
- Metadata phase-2 work remains open.

## Recommendations

1. Promote cross-host/Tailscale demo reproducibility next so the public demo moves beyond loopback-only proof without claiming production service readiness.
2. Validate native desktop package output and screenshot capture on an already provisioned Linux/macOS host.
3. Add KT-negative public demo proof only after the demo carries truthful KT evidence.
4. Add attachment demo proof only after the descriptor/fetch/decrypt path is executable without changing qsl-server or qsl-attachments production semantics in this lane.

## Related Evidence

- [Public demo readiness document](../../demo/PUBLIC_DEMO_TOUCH_AND_FEEL_READINESS.md)
- [NA-0256 testplan](../../../tests/NA-0256_public_demo_desktop_readiness_testplan.md)
- [Demo acceptance criteria](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- qsc desktop prototype README — retired at NA-0651 (D-1274, 2026-07-16); see git history and DOC-QSC-009/010 (superseded, retained as history)
