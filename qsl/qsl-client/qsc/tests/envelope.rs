use assert_cmd::prelude::*;
use predicates::prelude::*;

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

#[test]
fn envelope_plan_deterministic_bucket_and_bundle() {
    let mut cmd = qsc_cmd();
    cmd.args([
        "util",
        "envelope",
        "--tick-count",
        "3",
        "--interval-ms",
        "100",
        "--max-ticks",
        "3",
        "--max-bundle",
        "128",
        "--max-count",
        "3",
        "--payload-lens",
        "10,20,50,80",
    ]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("QSC_MARK/1 event=envelope_plan"))
        .stdout(predicate::str::contains("ticks=3"))
        .stdout(predicate::str::contains("interval_ms=100"))
        .stdout(predicate::str::contains("bucket_size=128"))
        .stdout(predicate::str::contains("bundle_len=80"))
        .stdout(predicate::str::contains("payload_count=3"));
}

#[test]
fn envelope_tick_bound_enforced() {
    let mut cmd = qsc_cmd();
    cmd.args([
        "util",
        "envelope",
        "--tick-count",
        "5",
        "--interval-ms",
        "100",
        "--max-ticks",
        "4",
        "--payload-lens",
        "10",
    ]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains(
            "code=envelope_tick_limit_exceeded",
        ));
}

#[test]
fn envelope_payload_too_large_rejected() {
    let mut cmd = qsc_cmd();
    cmd.args([
        "util",
        "envelope",
        "--max-bundle",
        "128",
        "--payload-lens",
        "200",
    ]);

    cmd.assert()
        .failure()
        .stdout(predicate::str::contains("QSC_MARK/1 event=error"))
        .stdout(predicate::str::contains("code=envelope_payload_too_large"));
}
