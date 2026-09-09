Goals: G4
Status: Archive
Owner: QSL maintainers
Last-Updated: 2026-09-09

# September 2026 audit archive — publication draft

These are historical audit findings and recommendations, supplied for reproduction and review. They are not a present-day security assessment, acceptance result or claim that the reported defects are repaired. This archive is prepared for a records PR; publication is not complete until that PR lands.

The six files below were copied byte-for-byte from the Director's original audit handoff, retained in the operator audit bank. Original filenames and bytes are preserved. A class-based infrastructure/secret scan and review of provenance-bearing text found no private material requiring redaction; **no redactions were made**. The SHA-256 values below verify exact source copies. Report archive hashes identify the input archives, not Git commits. Source coordinates belong to those historical reports and can drift in later code. The private bank and runtime logs are not included.

| File | SHA-256 for exact verification |
| --- | --- |
| [AUDIT_qsl-protocol_security_2026-09-03.md](AUDIT_qsl-protocol_security_2026-09-03.md) | `e4d91c0863e639afef6e7c5416a8c1941121b4fe0a29cdaf498864d4e7e38da3` |
| [AUDIT_qsl-server_desktop_interaction_2026-09-03.md](AUDIT_qsl-server_desktop_interaction_2026-09-03.md) | `ea256c9990f29b18db10dd2111c86d41ec388f0cb0373a53426a47370352ae03` |
| [AUDIT_qsl-desktop_2026-09-03.md](AUDIT_qsl-desktop_2026-09-03.md) | `40b8bceca2f1673f441b6a66d11c97c1ab840dd1cffe677908065d177dd96e50` |
| [AUDIT_harness_exp.rs](AUDIT_harness_exp.rs) | `67313de62dbd826333fbb810007a4701b3d717791bd41880321ef9c02bc4cb3c` |
| [AUDIT_harness_results.txt](AUDIT_harness_results.txt) | `d17ac9198e2467faadcf6e75dab7b2e5f966f0d6c4b771f178aaba874cc34b60` |
| [RECOMMENDATIONS_qsl_program_2026-09-04.md](RECOMMENDATIONS_qsl_program_2026-09-04.md) | `fbee4e3cb49ba04e1cb13af3e7ce869059d039dc5a5137404b1aa77715f0b85c` |

## Reproduce the historical mock experiments

The protocol report's header identifies its ratchet source by SHA-256. That exact digest was recomputed equal against the NA-0779 protocol source during closeout preparation:

```text
d4c12526e754272e432a983e44e29708d1617e78c9199c7865dd3a2939dde524
```

Verify `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` with `sha256sum` in the protocol checkout before comparison. A mismatch means a different source revision; retain that fact and do not call the result an exact reproduction. This file digest does not independently establish identity of every supporting source module. Protocol commit `4e03092fb14a` supplies the closeout comparison tree; the report itself came from an archive without Git metadata.

The delivered experiment file imports a crate named `harness`; it is not a standalone Cargo project. With the pinned Rust 1.95 toolchain, make a scratch Cargo package outside the checkout, copy `AUDIT_harness_exp.rs` unchanged to `src/bin/exp.rs`, and alias the real reference crate as `harness` in the manifest. Use an absolute path to the verified checkout:

```toml
[package]
name = "audit-reproduction"
version = "0.0.0"
edition = "2021"

[workspace]

[dependencies]
harness = { package = "quantumshield_refimpl", path = "/absolute/path/to/qsl-protocol/tools/refimpl/quantumshield_refimpl", default-features = false, features = ["stdcrypto"] }
```

Select the checkout's existing build environment and shared Cargo target before invoking Cargo; do not install or silently change the toolchain. From the scratch package run `cargo run --release --bin exp`, capture stdout and stderr separately, and compare the experiment results with `AUDIT_harness_results.txt`. Keep the generated Cargo.lock, toolchain version and source identities beside the output. The stdcrypto dependency feature is needed to compile the reference crate; the delivered experiment still explicitly calls Mock and ToyDh. This recipe preserves the delivered mock primitive implementations. The historical scratch crate removed derives to accommodate Rust 1.75; that source modification is not part of this pinned-toolchain recipe.

## Limitations and the confirming work still owed

Experiments A–D use **mock primitives** and characterize the tested state-machine paths and sizes under those mocks. They do not test cryptographic strength, real-primitive interoperability, a live relay, desktop acceptance, or deployment safety. The original results are retained as historical output; a reproduction result must be labeled separately with its actual toolchain and supporting sources.

The **StdCrypto rerun remains owed** in the approved ratchet lane. Enabling the real crate's default features alone does not replace the explicit Mock/ToyDh implementations in this experiment file. A separately reviewed adapter must use the real Hash/Kmac/Aead/X25519Dh implementations, retain the A–D scenarios, and record discriminating results. The two-party interleaving simulator must be demonstrated red on the relevant baseline and green after any approved repair; existing conformance tests alone are not a reproduction of A–D. No repair or confirming StdCrypto result is supplied by this archive.

Recommendations describe proposals from the report date. The governance queue, subsequent decisions and roadmap govern their current disposition. The public repository permits review and critique; it does not establish that independent cryptographic review occurred. The thirty filing-to-source mappings and later qualifications are in the NA-0779 as-built record when the companion records PR lands.

## Recipe validation during closeout preparation

The unchanged delivered harness was run with the manifest above under Rust 1.95 in the existing shared build target. It exited 0; stdout was byte-identical to the 2500-byte historical results file. Two existing unused-assignment warnings were retained. An initial manifest without the stdcrypto dependency feature failed because the reference crate imports sha2 outside that feature gate; enabling the existing feature fixed the scratch manifest only. The experiment continued to call Mock/ToyDh throughout. This validates the reproduction recipe and its mock results, not the still-owed StdCrypto confirmation or any repair.
