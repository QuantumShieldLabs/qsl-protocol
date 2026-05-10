Goals: G1, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10
Replaces: n/a
Superseded-By: n/a

# Public Demo Touch-and-Feel Readiness

## Posture

The current public demo and desktop prototype are non-production evidence surfaces. They help a visitor, reviewer, or operator see the current Suite-2 demo behavior, bounded reject behavior, and desktop shell shape, but they do not prove production release readiness.

Safe public wording:

- "The repository includes a local, loopback-only public demo acceptance runner."
- "The desktop GUI is a bounded qsc sidecar prototype for guided demo review."
- "Current evidence is non-production and release-gated."
- "KT-negative and attachment demo proofs are bounded, non-production evidence; production KT, production attachments, keychain active-ops, production relay, and qsl-attachments hardening remain open."

Do not claim:

- production-ready protocol, relay, attachment service, desktop app, or public demo;
- proven true Triple Ratchet;
- quantum-proof, anonymous, or metadata-free messaging;
- production KT deployment readiness;
- production attachment service readiness; or
- external cryptographic review completion.

## Current Public Demo Command

Run from the repository root:

```bash
scripts/ci/demo_cli_smoke.sh
```

This command builds `qshield-cli` and `refimpl_actor`, allocates an ephemeral loopback relay, creates isolated temporary Alice/Bob stores, and exits nonzero on the first failed invariant.

The latest attachment-expanded NA-0260 local transcript is outside the
repository at:

```text
/srv/qbuild/tmp/NA-0260_attachment_demo_artifacts_20260510T041841Z/demo_cli_smoke.log
```

The run ended with:

```text
DEMO_POSITIVE_SEND_RECEIVE_DECRYPT_OK
DEMO_NEGATIVE_KT_REJECT_OK
DEMO_NEGATIVE_KT_NO_MUTATION_OK
DEMO_KT_NON_PRODUCTION_BOUNDARY_OK
NA0259_KT_NEGATIVE_DEMO_READY_OK
DEMO_ATTACHMENT_DESCRIPTOR_OK
DEMO_ATTACHMENT_FETCH_DECRYPT_OK
DEMO_ATTACHMENT_INTEGRITY_REJECT_OK
DEMO_ATTACHMENT_OPAQUE_BOUNDARY_OK
DEMO_ATTACHMENT_NO_SECRET_LEAK_OK
NA0260_ATTACHMENT_DEMO_READY_OK
DEMO_NO_SECRET_LEAK_OK
DEMO_ACCEPTANCE_OK
demo-cli-smoke: OK
```

## What The Demo Proves

The current one-command demo proves:

- the demo CLI and refimpl actor build on the local qbuild host;
- Alice and Bob initialize isolated local demo stores;
- the relay starts on a loopback-only address by default;
- relay state-changing paths require bearer authorization;
- malformed relay JSON is rejected without echoing the supplied secret sentinel or relay token;
- invalid relay identifiers reject through the CLI path;
- replayed establish records reject fail-closed;
- bounded KT-negative verifier checks reject selected invalid KT evidence without mutating accepted verifier state;
- a demo-only attachment descriptor and payload are encrypted, fetched, decrypted, integrity-checked, and compared to the sender payload;
- a tampered attachment ciphertext rejects before an output file is written;
- Alice and Bob register authorized bundles;
- Alice and Bob establish demo sessions with explicit demo-only override;
- Alice sends `hello-na0246`;
- Bob receives/decrypts the plaintext and the sender marker remains inspectable; and
- output keeps the non-production warning posture visible.

## What The Demo Does Not Prove

The current demo does not prove:

- production authentication UX;
- production relay or qsl-server readiness;
- qsl-attachments production hardening;
- live `qshield establish` ingestion of arbitrary KT evidence;
- production KT service or log operation;
- cross-host KT-negative behavior;
- qsl-server or qsl-attachments production readiness;
- cross-host/private-network attachment proof;
- production desktop release readiness;
- cross-host or Tailscale reproducibility;
- anonymity or metadata elimination; or
- production release approval.

## Metadata Conformance Command

Run from the repository root:

```bash
scripts/ci/metadata_conformance_smoke.sh
```

The 2026-05-09 NA-0256 local transcript is outside the repository at:

```text
/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_20260509T044612Z/metadata_conformance_smoke.log
```

The run ended with:

```text
metadata-conformance-smoke: OK
```

This smoke proves selected metadata and operator-safety constraints for the demo surface: loopback defaults, explicit unsafe public bind acknowledgement, required relay authorization, sanitized errors, store permissions, queue/rate/quota bounds, bundle consumption, replay rejection, padding metadata checks, and no token/sentinel echo in checked error paths.

## What Changed After NA-0259 And NA-0260

NA-0259 added a bounded KT-negative proof to the public demo runner. The proof invokes canonical KT verifier vectors, accepted-state no-mutation coverage, and an explicit non-production disabled-shape boundary before emitting KT readiness markers.

NA-0260 added a bounded attachment proof to the qshield demo. The proof creates encrypted descriptor and payload messages, fetches opaque relay entries, validates descriptor-bound ciphertext metadata, decrypts valid payloads on the receiver side, rejects tampered ciphertext, and checks that relay/demo logs do not expose attachment plaintext or relay tokens.

Both additions are demo-only evidence. They do not change protocol, wire, crypto, qsl-server, qsl-attachments, qsc-desktop, website, branch-protection, public-safety, workflow, or Cargo behavior.

## qshield CLI Touch Points

NA-0256 captured current help output for these commands under the artifact directory:

```text
qshield_help.txt
qshield_relay_help.txt
qshield_establish_help.txt
qshield_send_help.txt
qshield_recv_help.txt
```

The inspected qshield surface keeps the CLI labeled as a non-production demo, keeps unsafe non-loopback bind behind explicit acknowledgement, avoids printing the relay token, and uses normal `Result`-based error returns for the user-facing command surface. The only direct `unwrap`/`panic` findings in `apps/qshield-cli/src` are inside test-only relay lock-poison coverage.

## Desktop Prototype Status

The desktop prototype remains a bounded Linux/macOS Tauri shell over the qsc sidecar. It is not a second client-core implementation and does not implement protocol/session/attachment logic in frontend JavaScript.

Run from `qsl/qsl-client/qsc-desktop`:

```bash
npm ci
npm run build
npm run prepare:sidecar
npm run tauri:build
```

Run from the repository root:

```bash
cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1
cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
```

NA-0256 local results before native package prerequisites were available:

- `npm ci`: passed, with existing npm audit warnings reported by npm.
- `npm run build`: passed.
- `npm run prepare:sidecar`: passed and copied the built `qsc` sidecar into Tauri resources.
- `cargo test -p qsc --locked --test desktop_gui_contract_na0215b -- --test-threads=1`: passed, 3 tests.
- `cargo test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1`: passed, 6 tests.
- `npm run tauri:build`: host-limited after successful sidecar prep and frontend build because the earlier Ubuntu qbuild host did not have `pkg-config` for the GLib native dependency chain.

NA-0258 later produced bounded native package and screenshot proof on a
provisioned Ubuntu host:

- `npm run tauri:build`: passed and emitted one Linux AppImage bundle.
- packaged-app Xvfb screenshot: captured from the native AppImage launch.
- `desktop_gui_contract_na0215b`: passed, 3 tests.
- `qsp_protocol_gate`: passed, 6 tests.
- `send_commit`: passed, 3 tests.
- `cargo audit --deny warnings`: passed.
- `cargo tree -i rustls-webpki --locked`: resolved `rustls-webpki v0.103.13`.

The NA-0258 native package and screenshot artifacts are outside the repository
at:

```text
/srv/qbuild/tmp/NA-0258_native_desktop_artifacts_20260509T194934Z/
```

The screenshot artifact is `qsc-desktop-appimage-xvfb-scrot.png`. This is
bounded prototype evidence, not production desktop release approval.

## Desktop Prototype Boundaries

The desktop prototype status remains:

- passphrase-backed sidecar operations only for active flows;
- memory-only, child-scoped passphrase handling in the backend bridge;
- keychain-backed active operations deferred;
- handshake/session-establish UI out of scope;
- attachment UI out of scope;
- full transcript-history UI out of scope;
- multiprofile UI out of scope;
- `protocol_inactive` surfaced fail-closed rather than bypassed; and
- no production-ready desktop claim.

## Artifact Package

Local artifact directory:

```text
/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_20260509T044612Z/
```

Manifest:

```text
/srv/qbuild/tmp/NA-0256_demo_desktop_artifacts_20260509T044612Z/ARTIFACT_MANIFEST.txt
```

The artifact package contains command transcripts, qshield help output, and the host-limited Tauri build log. Binary screenshots are not committed, and none were generated on this host.

## Related Evidence

- [Demo acceptance criteria](DEMO_ACCEPTANCE_CRITERIA.md)
- [KT-negative public demo readiness](KT_NEGATIVE_PUBLIC_DEMO_READINESS.md)
- [Attachment public demo readiness](ATTACHMENT_PUBLIC_DEMO_READINESS.md)
- [Native desktop package and screenshot readiness](NATIVE_DESKTOP_PACKAGE_SCREENSHOT_READINESS.md)
- [NA-0256 audit](../governance/evidence/NA-0256_public_demo_desktop_readiness_audit.md)
- [NA-0259 audit](../governance/evidence/NA-0259_kt_negative_demo_readiness_audit.md)
- [NA-0260 audit](../governance/evidence/NA-0260_attachment_demo_readiness_audit.md)
- [NA-0258 audit](../governance/evidence/NA-0258_native_desktop_package_screenshot_audit.md)
- [NA-0256 testplan](../../tests/NA-0256_public_demo_desktop_readiness_testplan.md)
- [qsc desktop prototype README](../../qsl/qsl-client/qsc-desktop/README.md)
