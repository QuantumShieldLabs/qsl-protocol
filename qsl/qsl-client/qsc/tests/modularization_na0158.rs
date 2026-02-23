use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn modularization_keeps_cli_surface_stable() {
    let mut cmd = Command::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(contains("status"))
        .stdout(contains("config"))
        .stdout(contains("vault"))
        .stdout(contains("tui"))
        .stdout(contains("relay"));
}
