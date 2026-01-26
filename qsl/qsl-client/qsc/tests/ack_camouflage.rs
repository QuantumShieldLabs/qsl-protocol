use std::str;

fn qsc_cmd() -> assert_cmd::Command {
    assert_cmd::cargo::cargo_bin_cmd!("qsc")
}

fn extract_kv(output: &str, key: &str) -> Option<String> {
    for part in output.split_whitespace() {
        if let Some(rest) = part.strip_prefix(&format!("{key}=")) {
            return Some(rest.to_string());
        }
    }
    None
}

#[test]
fn ack_size_class_matches_small_msg() {
    let mut small = qsc_cmd();
    small.args([
        "util",
        "envelope",
        "--tick-count",
        "1",
        "--interval-ms",
        "100",
        "--max-ticks",
        "4",
        "--max-bundle",
        "128",
        "--max-count",
        "1",
        "--payload-lens",
        "1",
    ]);
    let out_small = small.output().expect("small msg plan");
    assert!(out_small.status.success());
    let small_stdout = String::from_utf8_lossy(&out_small.stdout);
    let small_bucket = extract_kv(&small_stdout, "bucket_size").expect("bucket_size");

    let mut ack = qsc_cmd();
    ack.args([
        "envelope",
        "plan-ack",
        "--deterministic",
        "--tick-count",
        "1",
        "--interval-ms",
        "100",
        "--max-ticks",
        "4",
        "--max-bundle",
        "128",
        "--max-count",
        "1",
        "--small-len",
        "1",
    ]);
    let out_ack = ack.output().expect("ack plan");
    assert!(out_ack.status.success());
    let ack_stdout = String::from_utf8_lossy(&out_ack.stdout);
    let ack_bucket = extract_kv(&ack_stdout, "size_class").expect("size_class");

    assert_eq!(ack_bucket, small_bucket);
}

#[test]
fn ack_behavior_deterministic() {
    let mut ack1 = qsc_cmd();
    ack1.args([
        "envelope",
        "plan-ack",
        "--deterministic",
        "--tick-count",
        "1",
        "--interval-ms",
        "100",
        "--max-ticks",
        "4",
        "--max-bundle",
        "128",
        "--max-count",
        "1",
        "--small-len",
        "1",
    ]);
    let out1 = ack1.output().expect("ack plan 1");
    assert!(out1.status.success());

    let mut ack2 = qsc_cmd();
    ack2.args([
        "envelope",
        "plan-ack",
        "--deterministic",
        "--tick-count",
        "1",
        "--interval-ms",
        "100",
        "--max-ticks",
        "4",
        "--max-bundle",
        "128",
        "--max-count",
        "1",
        "--small-len",
        "1",
    ]);
    let out2 = ack2.output().expect("ack plan 2");
    assert!(out2.status.success());

    assert_eq!(out1.stdout, out2.stdout);
}
