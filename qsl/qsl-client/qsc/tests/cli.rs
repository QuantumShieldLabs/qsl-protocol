use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn version_is_printable() {
    let mut cmd = Command::cargo_bin("qsc").unwrap();
    cmd.arg("--version");
    cmd.assert().success();
}

#[test]
fn status_is_deterministic_marker() {
    let mut cmd = Command::cargo_bin("qsc").unwrap();
    cmd.arg("status");
    cmd.assert()
        .success()
        .stdout(contains("QSC_MARK/1 event=status ok=true locked=unknown"));
}
