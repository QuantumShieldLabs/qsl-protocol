use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf()
}

#[test]
fn remote_testing_contract_is_secondary_and_placeholder_free() {
    let doc = fs::read_to_string(
        repo_root().join("docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md"),
    )
    .expect("doc");

    assert!(
        doc.contains("compatibility-only remote evidence")
            && doc.contains("not the validated qbuild/local front door")
            && doc.contains("LOCAL_TWO_CLIENT_RUNBOOK.md"),
        "remote testing contract should point operators at the validated local front door: {doc}"
    );
    assert!(
        doc.contains("scripts/demo/qsc_remote_relay_smoke.sh")
            && doc.contains("scripts/demo/qsc_remote_handshake_smoke.sh"),
        "remote testing contract should keep the remote script anchors explicit: {doc}"
    );
    assert!(
        !doc.contains("TBD if needed")
            && !doc.contains(".github/workflows/")
            && !doc.contains("workflow_dispatch"),
        "remote testing contract should not keep placeholder or workflow-era wording: {doc}"
    );
}
