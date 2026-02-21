use assert_cmd::Command as AssertCommand;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Category {
    ReadOnly,
    Config,
    StateTransition,
    Wizard,
    Dangerous,
    Harness,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum ExpectedLockTransition {
    None,
    Lock,
    Unlock,
    InitWizard,
}

struct CmdSpec {
    name: &'static str,
    category: Category,
    allowed_when_locked: bool,
    expected_lock_transition: ExpectedLockTransition,
    samples: &'static [&'static str],
}

fn catalog() -> &'static [CmdSpec] {
    &[
        CmdSpec {
            name: "help",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/help"],
        },
        CmdSpec {
            name: "exithelp",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/exithelp"],
        },
        CmdSpec {
            name: "focus",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/focus events"],
        },
        CmdSpec {
            name: "inspector",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/inspector status"],
        },
        CmdSpec {
            name: "ins",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/ins status"],
        },
        CmdSpec {
            name: "back",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/back"],
        },
        CmdSpec {
            name: "unfocus",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/unfocus"],
        },
        CmdSpec {
            name: "down",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/down"],
        },
        CmdSpec {
            name: "up",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/up"],
        },
        CmdSpec {
            name: "pgdn",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/pgdn"],
        },
        CmdSpec {
            name: "pagedown",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/pagedown"],
        },
        CmdSpec {
            name: "pgup",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/pgup"],
        },
        CmdSpec {
            name: "pageup",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/pageup"],
        },
        CmdSpec {
            name: "status",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/status"],
        },
        CmdSpec {
            name: "account",
            category: Category::Dangerous,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/account show"],
        },
        CmdSpec {
            name: "envelope",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/envelope"],
        },
        CmdSpec {
            name: "contacts",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/contacts list"],
        },
        CmdSpec {
            name: "verify",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/verify peer-0 ABCD-EFGH-JKMN-PQRS-T"],
        },
        CmdSpec {
            name: "messages",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/messages list"],
        },
        CmdSpec {
            name: "files",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/files list"],
        },
        CmdSpec {
            name: "autolock",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/autolock show", "/autolock set 10"],
        },
        CmdSpec {
            name: "poll",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/poll show", "/poll set fixed 10", "/poll set adaptive"],
        },
        CmdSpec {
            name: "polling",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/polling show"],
        },
        CmdSpec {
            name: "relay",
            category: Category::Config,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &[
                "/relay show",
                "/relay set endpoint https://relay.example.test",
                "/relay clear",
                "/relay test",
            ],
        },
        CmdSpec {
            name: "vault",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/vault where"],
        },
        CmdSpec {
            name: "device",
            category: Category::ReadOnly,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/device show"],
        },
        CmdSpec {
            name: "send",
            category: Category::Dangerous,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/send"],
        },
        CmdSpec {
            name: "receive",
            category: Category::Dangerous,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/receive"],
        },
        CmdSpec {
            name: "handshake",
            category: Category::Dangerous,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/handshake status"],
        },
        CmdSpec {
            name: "export",
            category: Category::Dangerous,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/export"],
        },
        CmdSpec {
            name: "injectmsg",
            category: Category::Harness,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/injectmsg peer-0 RECEIVED"],
        },
        CmdSpec {
            name: "injectevent",
            category: Category::Harness,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/injectevent activity test"],
        },
        CmdSpec {
            name: "key",
            category: Category::Harness,
            allowed_when_locked: true,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/key down"],
        },
        CmdSpec {
            name: "lock",
            category: Category::StateTransition,
            allowed_when_locked: false,
            expected_lock_transition: ExpectedLockTransition::Lock,
            samples: &["/lock"],
        },
        CmdSpec {
            name: "unlock",
            category: Category::StateTransition,
            allowed_when_locked: true,
            expected_lock_transition: ExpectedLockTransition::Unlock,
            samples: &["/unlock"],
        },
        CmdSpec {
            name: "init",
            category: Category::Wizard,
            allowed_when_locked: true,
            expected_lock_transition: ExpectedLockTransition::InitWizard,
            samples: &["/init"],
        },
        CmdSpec {
            name: "exit",
            category: Category::StateTransition,
            allowed_when_locked: true,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/exit"],
        },
        CmdSpec {
            name: "quit",
            category: Category::StateTransition,
            allowed_when_locked: true,
            expected_lock_transition: ExpectedLockTransition::None,
            samples: &["/quit"],
        },
    ]
}

fn unique_cfg_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    std::env::current_dir()
        .expect("cwd")
        .join("target")
        .join("qsc-tests")
        .join(format!("{tag}-{}-{nonce}", std::process::id()))
}

fn ensure_dir_700(path: &Path) {
    std::fs::create_dir_all(path).expect("create dir");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).expect("chmod");
    }
}

fn run_headless(cfg: &Path, script: &str, unlocked: bool) -> String {
    let mut cmd = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"));
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_TUI_HEADLESS", "1")
        .env("QSC_TUI_SCRIPT", script)
        .env("QSC_TUI_COLS", "140")
        .env("QSC_TUI_ROWS", "40")
        .env("QSC_MARK_FORMAT", "plain");
    if unlocked {
        cmd.env("QSC_TUI_TEST_UNLOCK", "1");
    }
    let out = cmd.args(["tui"]).output().expect("run headless tui");
    let mut combined = String::from_utf8_lossy(&out.stdout).to_string();
    combined.push_str(&String::from_utf8_lossy(&out.stderr));
    combined
}

fn execute_command_and_assert_responsive(cfg: &Path, command: &str, unlocked: bool) -> String {
    let script = format!("{};/status;/key down;/status;/exit", command);
    let out = run_headless(cfg, &script, unlocked);
    assert!(
        out.contains("event=tui_cmd cmd=status"),
        "command {} wedged UI before follow-up status: {}",
        command,
        out
    );
    assert!(
        out.contains("event=tui_cmd cmd=key") || out.contains("event=tui_cmd cmd=down"),
        "command {} wedged UI before follow-up key navigation: {}",
        command,
        out
    );
    out
}

fn assert_has_deterministic_cmd_result(out: &str, command: &str) {
    assert!(
        out.contains("event=tui_cmd_result kind=ok")
            || out.contains("event=tui_cmd_result kind=err"),
        "command {} must emit deterministic cmd result marker: {}",
        command,
        out
    );
}

fn init_vault(cfg: &Path, passphrase: &str) {
    ensure_dir_700(cfg);
    let out = AssertCommand::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_DISABLE_KEYCHAIN", "1")
        .env("QSC_PASSPHRASE", passphrase)
        .args([
            "vault",
            "init",
            "--non-interactive",
            "--passphrase-env",
            "QSC_PASSPHRASE",
        ])
        .output()
        .expect("vault init");
    assert!(out.status.success(), "vault init failed");
}

#[test]
fn catalog_guard_matches_dispatch_and_help_sources() {
    let src = std::fs::read_to_string(format!("{}/src/main.rs", env!("CARGO_MANIFEST_DIR")))
        .expect("read main.rs");

    let mut names = BTreeSet::new();
    for spec in catalog() {
        names.insert(spec.name);
        assert!(
            src.contains(&format!("\"{}\"", spec.name)),
            "catalog command missing from source dispatch/help literals: {}",
            spec.name
        );
    }

    let required = [
        "autolock",
        "account",
        "back",
        "contacts",
        "device",
        "down",
        "envelope",
        "exit",
        "exithelp",
        "export",
        "files",
        "focus",
        "handshake",
        "help",
        "injectevent",
        "injectmsg",
        "init",
        "ins",
        "inspector",
        "key",
        "lock",
        "messages",
        "pagedown",
        "pageup",
        "pgdn",
        "pgup",
        "poll",
        "polling",
        "quit",
        "receive",
        "relay",
        "send",
        "status",
        "unfocus",
        "unlock",
        "up",
        "vault",
        "verify",
    ];
    let expected: BTreeSet<_> = required.into_iter().collect();
    assert_eq!(
        names, expected,
        "catalog must list every parser-supported command"
    );

    for line in src
        .lines()
        .filter(|l| l.trim_start().starts_with("cmd: \""))
    {
        let trimmed = line.trim();
        let value = trimmed
            .trim_start_matches("cmd: \"")
            .trim_end_matches(',')
            .trim_end_matches('\"');
        let root = value
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .split('|')
            .next()
            .unwrap_or_default();
        if !root.is_empty() {
            assert!(
                expected.contains(root),
                "help root '{}' not represented in command catalog",
                root
            );
        }
    }

    let mut locked_allowed = BTreeSet::new();
    let mut lock_transitions = BTreeSet::new();
    for spec in catalog() {
        if spec.allowed_when_locked {
            locked_allowed.insert(spec.name);
        }
        if spec.expected_lock_transition != ExpectedLockTransition::None {
            lock_transitions.insert(spec.name);
        }
    }
    assert_eq!(
        locked_allowed,
        BTreeSet::from(["exit", "init", "key", "quit", "unlock"]),
        "locked-mode allowlist changed; update catalog tests"
    );
    assert_eq!(
        lock_transitions,
        BTreeSet::from(["init", "lock", "unlock"]),
        "state-transition expectations changed; update catalog tests"
    );
}

#[test]
fn all_readonly_commands_do_not_lock_or_wedge() {
    let cfg = unique_cfg_dir("na0138_catalog_readonly");
    ensure_dir_700(&cfg);
    for spec in catalog()
        .iter()
        .filter(|s| s.category == Category::ReadOnly)
    {
        for sample in spec.samples {
            let out = execute_command_and_assert_responsive(&cfg, sample, true);
            assert!(
                !out.contains("event=tui_lock_state locked=LOCKED"),
                "readonly command {} unexpectedly locked UI: {}",
                sample,
                out
            );
            assert!(
                !out.contains("locked_unlock_required"),
                "readonly command {} should not require unlock while unlocked: {}",
                sample,
                out
            );
        }
    }
}

#[test]
fn all_config_commands_do_not_lock_or_wedge_and_persist() {
    let cfg = unique_cfg_dir("na0138_catalog_config");
    init_vault(&cfg, "StrongPassphrase1234");

    for spec in catalog().iter().filter(|s| s.category == Category::Config) {
        for sample in spec.samples {
            let wrapped = format!(
                "/unlock StrongPassphrase1234;{};/status;/key down;/status;/exit",
                sample
            );
            let out = run_headless(&cfg, wrapped.as_str(), false);
            assert!(
                !out.contains("event=tui_lock_state locked=LOCKED"),
                "config command {} unexpectedly locked UI: {}",
                sample,
                out
            );
            assert!(
                out.contains("event=tui_cmd cmd=status"),
                "config command {} wedged before follow-up status: {}",
                sample,
                out
            );
        }
    }

    let persisted = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 12;/poll show;/exit",
        false,
    );
    assert!(
        persisted.contains("event=tui_poll_show ok=true mode=fixed interval_seconds=12"),
        "fixed poll mode did not persist: {}",
        persisted
    );

    let reject = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 1;/poll show;/exit",
        false,
    );
    assert!(
        reject.contains("event=tui_poll_set code=poll_invalid_seconds ok=false"),
        "invalid poll interval reject missing: {}",
        reject
    );
    assert!(
        reject.contains("event=tui_poll_show ok=true mode=fixed interval_seconds=12"),
        "invalid poll interval mutated persisted settings: {}",
        reject
    );
    assert!(
        !reject.contains("event=tui_lock_state locked=LOCKED"),
        "invalid poll interval should not lock UI: {}",
        reject
    );
}

#[test]
fn state_transition_commands_behave_deterministically() {
    let cfg = unique_cfg_dir("na0138_catalog_state_transitions");
    ensure_dir_700(&cfg);

    let lock_out = run_headless(&cfg, "/lock;/exit", true);
    assert!(
        lock_out.contains("event=tui_lock_state locked=LOCKED reason=explicit_command")
            && lock_out.contains("event=tui_locked_shell"),
        "/lock must transition to deterministic locked shell: {}",
        lock_out
    );

    let cfg_vault = unique_cfg_dir("na0138_catalog_unlock");
    init_vault(&cfg_vault, "StrongPassphrase1234");
    let unlock_out = run_headless(
        &cfg_vault,
        "/unlock StrongPassphrase1234;/status;/exit",
        false,
    );
    assert!(
        unlock_out.contains("event=tui_lock_state locked=UNLOCKED reason=explicit_command")
            && unlock_out.contains("event=tui_cmd cmd=status"),
        "/unlock must return to usable unlocked state: {}",
        unlock_out
    );
}

#[test]
fn wizard_commands_enter_visible_prompt_state_and_cancel() {
    let cfg = unique_cfg_dir("na0138_catalog_wizard");
    ensure_dir_700(&cfg);

    let out = run_headless(&cfg, "/init;/key esc;/status;/exit", false);
    assert!(
        out.contains("event=tui_init_wizard step=alias")
            && out.contains("main_step=init_alias")
            && out.contains("event=tui_locked_cmd_reject code=locked_unlock_required cmd=status")
            && out.contains("event=tui_cmd cmd=exit"),
        "wizard did not enter visible prompt or failed to cancel cleanly: {}",
        out
    );
}

#[test]
fn locked_allowed_commands_only() {
    let cfg = unique_cfg_dir("na0138_catalog_locked_commands");
    init_vault(&cfg, "StrongPassphrase1234");

    let allowed = run_headless(
        &cfg,
        "/unlock;/key esc;/unlock StrongPassphrase1234;/lock;/exit",
        false,
    );
    assert!(
        allowed.contains("event=tui_unlock_prompt step=passphrase")
            && allowed.contains("event=tui_lock_state locked=UNLOCKED reason=explicit_command")
            && allowed.contains("event=tui_lock_state locked=LOCKED reason=explicit_command"),
        "locked allowed commands should work deterministically: {}",
        allowed
    );

    let rejected = run_headless(
        &cfg,
        "/status;/help;/poll show;/key down;/unlock StrongPassphrase1234;/exit",
        false,
    );
    assert!(
        rejected.contains("event=tui_locked_cmd_reject")
            && rejected.contains("locked_unlock_required")
            && rejected.contains("event=tui_lock_state locked=UNLOCKED reason=explicit_command"),
        "locked disallowed commands must reject without wedging and still allow unlock: {}",
        rejected
    );
}

#[test]
fn command_activity_does_not_relock_after_timeout_boundary() {
    let cfg = unique_cfg_dir("na0138_command_activity");
    ensure_dir_700(&cfg);
    let out = run_headless(&cfg, "wait 599000;/status;wait 2000;/status;/exit", true);
    assert!(
        !out.contains("event=tui_autolock ok=true"),
        "command activity should refresh inactivity timer and avoid immediate relock: {}",
        out
    );
}

#[test]
fn every_catalog_command_emits_deterministic_result_marker() {
    let locked_cfg = unique_cfg_dir("na0144_catalog_locked_results");
    init_vault(&locked_cfg, "StrongPassphrase1234");

    let unlocked_cfg = unique_cfg_dir("na0144_catalog_unlocked_results");
    ensure_dir_700(&unlocked_cfg);

    for spec in catalog() {
        for sample in spec.samples {
            let out = if spec.allowed_when_locked {
                run_headless(&locked_cfg, format!("{sample};/exit").as_str(), false)
            } else {
                run_headless(&unlocked_cfg, format!("{sample};/exit").as_str(), true)
            };
            assert_has_deterministic_cmd_result(&out, sample);
            assert!(
                !out.contains("tui_error:"),
                "command {} produced interactive wedge/error path: {}",
                sample,
                out
            );
        }
    }
}

#[test]
fn reject_paths_emit_err_without_lock_mutation() {
    let cfg = unique_cfg_dir("na0144_reject_paths");
    init_vault(&cfg, "StrongPassphrase1234");

    let invalid_poll = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/poll set fixed 1;/status;/exit",
        false,
    );
    assert!(
        invalid_poll.contains("event=tui_cmd_result kind=err")
            && invalid_poll.contains("event=tui_poll_set code=poll_invalid_seconds ok=false")
            && !invalid_poll.contains("event=tui_lock_state locked=LOCKED"),
        "poll reject must emit deterministic err and avoid lock mutation: {}",
        invalid_poll
    );

    let invalid_contacts = run_headless(
        &cfg,
        "/unlock StrongPassphrase1234;/contacts add;/status;/exit",
        false,
    );
    assert!(
        invalid_contacts.contains("event=tui_cmd_result kind=err")
            && invalid_contacts.contains("event=tui_contacts_invalid reason=missing_label")
            && !invalid_contacts.contains("event=tui_lock_state locked=LOCKED"),
        "contacts reject must emit deterministic err and avoid lock mutation: {}",
        invalid_contacts
    );
}
