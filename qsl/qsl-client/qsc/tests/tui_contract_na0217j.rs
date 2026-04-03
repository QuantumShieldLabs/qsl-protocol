use assert_cmd::Command as AssertCommand;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

fn unique_cfg_dir(tag: &str) -> PathBuf {
    static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
    let dir = env::temp_dir().join(format!(
        "qsc-na0217j-{}-{}-{}",
        tag,
        std::process::id(),
        NEXT_ID.fetch_add(1, Ordering::SeqCst)
    ));
    ensure_dir_700(&dir);
    dir
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn run_headless(cfg: &Path, script: &str) -> String {
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_TEST_UNLOCK", "1")
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

fn controller_render_lines(out: &str) -> Vec<&str> {
    out.lines()
        .filter(|line| {
            line.contains("event=tui_cmd ")
                || line.contains("event=tui_help_item")
                || line.contains("event=tui_help_rendered")
                || line.contains("event=tui_inspector")
                || line.contains("event=tui_render mode=home")
                || line.contains("event=tui_status_view")
                || line.contains("event=tui_commands_spacing")
        })
        .collect()
}

#[test]
fn headless_controller_render_markers_stay_deterministic() {
    let cfg = unique_cfg_dir("deterministic");
    let script = "/help;/inspector settings;/poll show;/exit";

    let first = run_headless(&cfg, script);
    let second = run_headless(&cfg, script);

    assert!(
        first.contains("event=tui_help_rendered"),
        "missing help render marker: {first}"
    );
    assert!(
        first.contains("event=tui_inspector pane=settings"),
        "missing settings inspector marker: {first}"
    );
    assert!(
        first.contains("event=tui_render mode=home layout=h3"),
        "missing home render marker: {first}"
    );
    assert!(
        first.contains("event=tui_status_view"),
        "missing status view marker: {first}"
    );
    assert!(
        !controller_render_lines(&first).is_empty(),
        "missing filtered controller/render markers: {first}"
    );

    assert_eq!(
        controller_render_lines(&first),
        controller_render_lines(&second),
        "controller/render markers changed across equivalent runs"
    );
}
