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
}
