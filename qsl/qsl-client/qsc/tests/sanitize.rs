use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn sanitize_strips_ansi_and_controls_and_emits_marker() {
    let payload = "hi\x1b[31mRED\x1b[0m\x07\x08!";
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.args(["util", "sanitize", "--print", payload]);
    cmd.assert().success().stdout(predicate::eq(
        "hiRED!\nQSC_MARK/1 event=util_sanitize code=ok\n",
    ));
}

#[test]
fn sanitize_usage_is_deterministic_marker() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.args(["util", "sanitize"]);
    cmd.assert()
        .failure()
        .stdout(predicate::eq("QSC_MARK/1 event=util_sanitize code=usage\n"));
}
