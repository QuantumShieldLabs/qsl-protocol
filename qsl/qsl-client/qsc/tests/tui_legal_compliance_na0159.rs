use assert_cmd::Command as AssertCommand;
use std::path::{Path, PathBuf};

fn safe_test_root() -> PathBuf {
    std::env::temp_dir().join("qsc-tests")
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain")
        .args(["tui"])
        .output()
        .expect("run tui headless");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

#[test]
fn init_flow_shows_legal_acceptance_and_accepts_i_agree() {
    let cfg = safe_test_root().join(format!("na0159_init_legal_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/init;\
/key A;/key l;/key i;/key a;/key s;/key enter;\
/key S;/key t;/key r;/key o;/key n;/key g;/key P;/key a;/key s;/key s;/key p;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/key 4;/key enter;\
/key S;/key t;/key r;/key o;/key n;/key g;/key P;/key a;/key s;/key s;/key p;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/key 4;/key enter;\
/key I;/key space;/key A;/key G;/key R;/key E;/key E;/key enter;\
/exit",
    );

    assert!(
        out.contains("cmdbar_text=Focus: CMD | Confirm (I AGREE/N): I AGREE█")
            && out.contains("main_step=init_decision")
            && out.contains("event=tui_init ok=true"),
        "init wizard should show legal acceptance prompt and accept I AGREE: {}",
        out
    );
}

#[test]
fn legal_page_renders_full_text_without_overclaiming() {
    let cfg = safe_test_root().join(format!("na0159_legal_page_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let script = "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I AGREE;\
                  /unlock StrongPassphrase1234;/inspector legal;/exit";
    let out = run_headless(&cfg, script);

    assert!(
        out.contains("event=tui_legal_fulltext sections=summary,warranty,operator,privacy,init overclaim=none")
            && out.contains("event=tui_render mode=home layout=h3 inspector=legal"),
        "legal page should render full text and explicit no-overclaim note: {}",
        out
    );
}

#[test]
fn about_page_includes_proof_links_and_no_secret_strings() {
    let cfg = safe_test_root().join(format!("na0159_about_page_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let script = "/init DemoUser StrongPassphrase1234 StrongPassphrase1234 I AGREE;\
                  /unlock StrongPassphrase1234;/inspector about;/exit";
    let out = run_headless(&cfg, script);

    assert!(
        out.contains(
            "event=tui_about_links governance=1 traceability=1 decisions=1 docs=1 tests=1"
        ) && out.contains("event=tui_render mode=home layout=h3 inspector=about"),
        "about page should include governance/docs/tests proof links: {}",
        out
    );
    assert!(
        !out.contains("Bearer ")
            && !out.contains("relay_inbox_token=set")
            && !out.contains("relay_auth_token="),
        "about/legal rendering should not leak secret-like strings: {}",
        out
    );
}
