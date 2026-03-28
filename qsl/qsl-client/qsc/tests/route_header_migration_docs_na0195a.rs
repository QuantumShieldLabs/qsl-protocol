use std::fs;
use std::path::{Path, PathBuf};

fn qsc_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn collect_supported_docs_and_scripts(root: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(root).expect("read_dir");
    for entry in entries {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        let file_type = entry.file_type().expect("file type");
        if file_type.is_dir() {
            let name = entry.file_name();
            if matches!(name.to_str(), Some("src" | "tests" | "target")) {
                continue;
            }
            collect_supported_docs_and_scripts(path.as_path(), out);
            continue;
        }
        let ext = match path.extension().and_then(|v| v.to_str()) {
            Some(ext) => ext,
            None => continue,
        };
        if matches!(ext, "md" | "py" | "sh") {
            out.push(path);
        }
    }
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("relative path")
        .display()
        .to_string()
}

#[test]
fn supported_docs_and_scripts_do_not_embed_route_tokens_in_urls() {
    let root = qsc_root();
    let mut files = Vec::new();
    collect_supported_docs_and_scripts(root.as_path(), &mut files);
    assert!(!files.is_empty(), "expected docs/script files");
    for path in files {
        let text = fs::read_to_string(&path).expect("read file");
        assert!(
            !text.contains("/v1/push/"),
            "{} still embeds route tokens in push URLs",
            relative(root.as_path(), path.as_path())
        );
        assert!(
            !text.contains("/v1/pull/"),
            "{} still embeds route tokens in pull URLs",
            relative(root.as_path(), path.as_path())
        );
    }
}

#[test]
fn canonical_operator_examples_use_route_token_header_and_not_authorization_overload() {
    let root = qsc_root();
    let runbook = fs::read_to_string(root.join("LOCAL_TWO_CLIENT_RUNBOOK.md")).expect("runbook");
    assert!(
        runbook.contains("X-QSL-Route-Token: test_mailbox"),
        "local runbook should show canonical route-token header example"
    );
    assert!(
        runbook.contains("/v1/pull?max=1"),
        "local runbook should show canonical token-free pull path"
    );
    assert!(
        runbook.contains("QSC_LEGACY_IN_MESSAGE_STAGE=w2"),
        "local runbook should document the validated post-w0 W2 control truthfully"
    );
    assert!(
        runbook.contains("by itself it does not activate the validated post-`w0` send defaults"),
        "local runbook should make clear that file-send attachment-service override is not the activation trigger"
    );
    assert!(
        runbook.contains("post-`w0` retired defaults for validated deployments"),
        "local runbook should document validated post-w0 activation truthfully"
    );
    assert!(
        runbook.contains("> 4 MiB` sends are unchanged by `w0|w2`"),
        "local runbook should state that W0/W2 does not alter above-threshold attachment-first behavior"
    );
    assert!(
        !runbook.contains("Rollback/coexistence restore for new legacy-sized sends:"),
        "local runbook must not keep the retired W0 rollback section"
    );
    assert!(
        runbook.contains("legacy_in_message_stage_retired_post_w0"),
        "local runbook should document the retired send-stage reject marker"
    );
    assert!(
        runbook.contains("legacy_receive_mode_retired_post_w0"),
        "local runbook should document the retired receive-mode control reject marker"
    );
    assert!(
        runbook.contains("legacy_receive_retired_post_w0"),
        "local runbook should document the explicit post-w0 legacy receive reject marker"
    );
    assert!(
        runbook.contains("event=file_xfer_reject code=legacy_receive_retired_post_w0"),
        "local runbook should document the explicit post-w0 file reject marker"
    );

    let soak = fs::read_to_string(root.join("scripts/remote_soak.py")).expect("remote soak script");
    assert!(
        soak.contains("\"X-QSL-Route-Token\": route_token"),
        "remote soak should send canonical route-token header"
    );
    assert!(
        soak.contains("/v1/pull?max="),
        "remote soak should use canonical token-free pull path"
    );
    assert!(
        !soak.contains("Authorization\": route_token")
            && !soak.contains("Authorization': route_token"),
        "route token must not overload Authorization header"
    );

    let aws_runbook =
        fs::read_to_string(root.join("REMOTE_TWO_CLIENT_AWS_RUNBOOK.md")).expect("aws runbook");
    assert!(
        aws_runbook.contains("1.2MB` example below is still legacy-sized under the current `4 MiB` boundary"),
        "AWS runbook should classify the 1.2MB example against the current 4 MiB boundary truthfully"
    );
    assert!(
        aws_runbook.contains("does not exercise the validated post-`w0` activation lane"),
        "AWS runbook should make clear it is not the validated post-w0 lane"
    );
    assert!(
        aws_runbook.contains("legacy-only compatibility coverage for a non-validated lane"),
        "AWS runbook should classify any remaining coexistence usage as non-validated compatibility coverage"
    );
}
