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

fn assert_ordered(haystack: &str, a: &str, b: &str, c: &str) {
    let ai = haystack.find(a).expect("missing a");
    let bi = haystack.find(b).expect("missing b");
    let ci = haystack.find(c).expect("missing c");
    assert!(
        ai < bi && bi < ci,
        "expected {a} < {b} < {c} in: {haystack}"
    );
}

#[test]
fn nav_marker_hidden_when_cmd_focused() {
    let cfg = safe_test_root().join(format!("na0131_nav_marker_focus_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/key slash;/key esc;/exit");
    assert!(
        out.contains("event=tui_nav_render selected_markers=0"),
        "nav marker should be hidden while cmd focused: {}",
        out
    );
    assert!(
        out.contains("event=tui_nav_render selected_markers=1"),
        "nav marker should appear once when nav focused: {}",
        out
    );
}

#[test]
fn init_wizard_step1_renders_optionb_and_alias_echo_uppercase() {
    let cfg = safe_test_root().join(format!("na0131_wizard_step1_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/init;/key M;/key a;/key t;/key t;/key h;/key e;/key w;/key _;/key 0;/key 1;/exit",
    );
    assert!(
        out.contains("This will create an encrypted vault to store your identity, contacts, messages, and files.")
            && out.contains("Choose a strong passphrase — there is no recovery if it’s lost."),
        "missing Option B explanatory text: {}",
        out
    );
    assert!(
        out.contains("main_input=Alias: Matthew_01"),
        "alias input not reflected in main wizard line: {}",
        out
    );
    assert!(
        out.contains("cmdbar_text=Alias: Matthew_01█"),
        "alias input not echoed in command bar with cursor: {}",
        out
    );
}

#[test]
fn init_wizard_passphrase_steps_mask_input() {
    let cfg = safe_test_root().join(format!("na0131_wizard_masking_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/init;/key A;/key l;/key i;/key a;/key s;/key enter;/key S;/key e;/key c;/key r;/key 3;/key t;/key P;/key a;/key s;/key s;/key P;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/exit",
    );
    assert!(
        out.contains("main_step=init_passphrase")
            && out.contains("main_input=Passphrase: •")
            && out.contains("cmdbar_text=Passphrase: •"),
        "passphrase step should mask input in main and command bar: {}",
        out
    );
    assert!(
        !out.contains("Secr3tPassPhrase123"),
        "plaintext passphrase leaked in output: {}",
        out
    );
}

#[test]
fn init_wizard_ack_last_requires_exact_text() {
    let cfg = safe_test_root().join(format!("na0131_wizard_ack_exact_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(
        &cfg,
        "/init;\
/key A;/key l;/key i;/key a;/key s;/key enter;\
/key S;/key t;/key r;/key o;/key n;/key g;/key P;/key a;/key s;/key s;/key p;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/key 4;/key enter;\
/key S;/key t;/key r;/key o;/key n;/key g;/key P;/key a;/key s;/key s;/key p;/key h;/key r;/key a;/key s;/key e;/key 1;/key 2;/key 3;/key 4;/key enter;\
/key I;/key space;/key u;/key n;/key d;/key e;/key r;/key s;/key t;/key a;/key n;/key d;/key enter;\
/key I;/key space;/key U;/key N;/key D;/key E;/key R;/key S;/key T;/key A;/key N;/key D;/key enter;\
/exit",
    );
    assert!(
        out.contains("main_step=init_ack")
            && out.contains("event=tui_init_reject code=ack_required"),
        "ack step should reject non-exact acknowledgement: {}",
        out
    );
    assert!(
        out.contains("event=tui_init ok=true")
            && out.contains("event=tui_locked_shell")
            && out.contains("main=locked"),
        "exact acknowledgement should complete init and return to locked shell: {}",
        out
    );
}

#[test]
fn error_line_placement_under_input() {
    let cfg = safe_test_root().join(format!("na0131_error_placement_{}", std::process::id()));
    std::fs::create_dir_all(&cfg).expect("create cfg");

    let out = run_headless(&cfg, "/init;/key a;/key enter;/exit");
    let line = out
        .lines()
        .find(|candidate| {
            candidate.contains("event=tui_locked_shell")
                && candidate.contains("main_error=<redacted>")
        })
        .unwrap_or_else(|| panic!("missing locked-shell marker with error in {out}"));
    assert_ordered(line, "main_input=", "main_error=", "main_hints=");
    assert!(
        line.contains("main_error=<redacted>"),
        "expected redacted validation error line under input: {}",
        line
    );
}
