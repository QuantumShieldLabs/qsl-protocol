//! NA-0760 — the workspace-licence property, made self-enforcing.
//!
//! The organisation's published claim is that source is `AGPL-3.0-only`. NA-0758's public-docs
//! audit measured that claim FALSE in the package metadata (census row `LI-04`): `qsc` declared
//! `MIT OR Apache-2.0`, `quantumshield_refimpl` declared `Apache-2.0 OR MIT`, and three further
//! manifests declared nothing at all. NA-0760 corrected every one of them.
//!
//! A one-shot correction makes the claim true on the day it lands. This test makes it *stay*
//! true: it asks `cargo metadata` the same question a reader of the published claim would ask,
//! and fails if any workspace crate answers with anything other than `AGPL-3.0-only`.
//!
//! The instrument is deliberately fail-closed in four independent ways, because a licence test
//! that can pass vacuously is worse than no test at all:
//!
//!   1. `cargo metadata` must exit successfully — a failed invocation is a FAIL, never a skip.
//!   2. The package set must be non-empty.
//!   3. Every crate this workspace is known to contain must be PRESENT in the answer, so a
//!      narrowed or truncated `cargo metadata` cannot pass by reporting nothing.
//!   4. A crate whose manifest carries NO `license` field reports `null`, which is an offender
//!      here rather than a skipped row — that is exactly the state three of these manifests
//!      were in before this lane.

use std::process::Command;

/// The operator-ruled licence for every crate published from this workspace.
const EXPECTED_LICENSE: &str = "AGPL-3.0-only";

/// Every crate the root `Cargo.toml` lists as a workspace member.
///
/// Hard-coded on purpose. If a member is added or removed, this test must be updated in the
/// same commit — which forces the licence question to be answered for the new crate rather
/// than inherited silently.
const EXPECTED_MEMBERS: &[&str] = &[
    "qsc",
    "qshield-cli",
    "quantumshield_refimpl",
    "refimpl_actor",
];

#[test]
fn every_workspace_crate_declares_agpl_3_0_only() {
    // `CARGO` is set by cargo for the process it spawns; using it rather than a bare "cargo"
    // keeps the child on the same toolchain as the test run.
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--offline",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata must be runnable from the crate directory");

    // (1) a failed invocation is a FAIL, not a skip.
    assert!(
        output.status.success(),
        "cargo metadata exited {:?}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr),
    );

    let metadata: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("cargo metadata must emit valid JSON");

    let packages = metadata["packages"]
        .as_array()
        .expect("cargo metadata output must carry a `packages` array");

    // (2) an empty answer must not read as "nothing wrong".
    assert!(
        !packages.is_empty(),
        "cargo metadata reported ZERO packages — the instrument would pass vacuously",
    );

    // (3) the answer must cover the whole workspace, not a subset of it.
    let reported: Vec<&str> = packages
        .iter()
        .map(|p| p["name"].as_str().unwrap_or("<unnamed>"))
        .collect();
    let absent: Vec<&&str> = EXPECTED_MEMBERS
        .iter()
        .filter(|m| !reported.contains(*m))
        .collect();
    assert!(
        absent.is_empty(),
        "cargo metadata did not report every known workspace member.\n  \
         absent: {absent:?}\n  reported: {reported:?}\n\
         Either the workspace membership changed (update EXPECTED_MEMBERS in the same commit \
         that changes it) or the metadata answer is narrower than the workspace.",
    );

    // (4) the property itself. A missing `license` field arrives as `null` and is an offender.
    let offenders: Vec<String> = packages
        .iter()
        .filter_map(|p| {
            let name = p["name"].as_str().unwrap_or("<unnamed>");
            match p["license"].as_str() {
                Some(EXPECTED_LICENSE) => None,
                Some(other) => Some(format!("{name}: declares {other:?}")),
                None => Some(format!("{name}: declares NO license field")),
            }
        })
        .collect();

    assert!(
        offenders.is_empty(),
        "the published claim is that source is {EXPECTED_LICENSE}, but {} of {} workspace \
         crate(s) disagree:\n  {}\n\nEvery crate in this workspace must carry \
         `license = \"{EXPECTED_LICENSE}\"` in its Cargo.toml.",
        offenders.len(),
        packages.len(),
        offenders.join("\n  "),
    );
}
