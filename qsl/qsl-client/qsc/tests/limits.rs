use predicates::prelude::*;

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

#[test]
fn queue_limit_enforced() {
    let mut cmd = qsc_cmd();
    cmd.args(["util", "queue", "--len", "65"]);
    cmd.assert().failure().stdout(predicate::str::contains(
        "QSC_MARK/1 event=error code=queue_limit_exceeded",
    ));
}

#[test]
fn retry_bound_enforced() {
    let mut cmd = qsc_cmd();
    cmd.args(["util", "retry", "--fail", "5"]);
    cmd.assert().failure().stdout(predicate::str::contains(
        "QSC_MARK/1 event=error code=retry_limit_exceeded",
    ));
}

#[test]
fn timeout_marker_stable() {
    let mut cmd = qsc_cmd();
    cmd.args(["util", "timeout", "--wait-ms", "20", "--timeout-ms", "10"]);
    cmd.assert().failure().stdout(predicate::str::contains(
        "QSC_MARK/1 event=error code=timeout_exceeded",
    ));
}
