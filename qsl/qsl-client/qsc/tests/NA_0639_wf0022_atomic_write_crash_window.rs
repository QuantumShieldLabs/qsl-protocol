// NA-0639 (directive QSL-DIR-2026-07-12-575, D575): WF-0022 atomic-write
// crash-window harness.
//
// EXERCISES the real `write_atomic` (src/fs_store/mod.rs) through the compiled
// qsc binary — `config set policy-profile <v>` performs exactly one
// write_atomic on <QSC_CONFIG_DIR>/config.txt once the store layout exists.
// The harness does NOT modify write_atomic or any production code.
//
// Invariant under test (OLD-XOR-NEW): a reader of the target path always
// observes the complete old content or the complete new content — never a
// torn/partial write, and the temp file is never observed as the live target.
//
// Interruption model (SIMULATED crash, named point): write_atomic's window is
//   create tmp (`config.txt.tmp.<pid>`) -> write_all -> sync_all -> rename
// (src/fs_store/mod.rs:105-122). The parent test watches for the tmp file and,
// on its appearance, removes WRITE permission from the store directory. The
// child (which already holds an open fd) completes write_all + sync_all, then
// its `fs::rename(tmp, target)` is DENIED — the process aborts with the
// on-disk state a crash immediately before the rename would leave:
// target = OLD (intact), tmp = NEW (complete), rename never applied.
// Limits: this denies the rename at the directory-permission seam rather than
// killing the kernel; power-loss / page-cache / fsync-lying semantics are out
// of scope. A pass asserts the invariant under THIS simulated model only.
//
// Non-vacuity (WF-0017): the same classifier that proves the positive cases is
// run against a deliberately NON-atomic, test-only write path (truncate then
// write in place). The negative controls MUST detect a torn state; if they
// could not, the harness would be vacuous.

#![cfg(unix)]

use assert_cmd::Command;
use std::fs::{self, Permissions};
use std::io::Write as _;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const OLD_CONTENT: &str = "policy_profile=baseline\n";
const NEW_CONTENT: &str = "policy_profile=strict\n";
const CONFIG_FILE: &str = "config.txt";

fn test_root() -> PathBuf {
    let base = if let Ok(v) = std::env::var("QSC_TEST_ROOT") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else if let Ok(v) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v).join("qsc-test-tmp")
    } else {
        PathBuf::from("target").join("qsc-test-tmp")
    };
    if base.is_absolute() {
        base
    } else {
        std::env::current_dir().expect("cwd").join(base)
    }
}

fn unique_dir(tag: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    test_root().join(format!("{tag}_{}_{}", std::process::id(), nonce))
}

fn ensure_dir_700(path: &Path) {
    fs::create_dir_all(path).unwrap();
    fs::set_permissions(path, Permissions::from_mode(0o700)).unwrap();
}

fn qsc_cmd(cfg: &Path) -> Command {
    let mut cmd = assert_cmd::cargo::cargo_bin_cmd!("qsc");
    cmd.env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain");
    cmd
}

/// Seed the store: layout + config.txt = OLD_CONTENT via the real binary.
fn seed_store(cfg: &Path) {
    ensure_dir_700(cfg);
    qsc_cmd(cfg)
        .args(["config", "set", "policy-profile", "baseline"])
        .assert()
        .success();
    assert_eq!(
        fs::read_to_string(cfg.join(CONFIG_FILE)).unwrap(),
        OLD_CONTENT,
        "seed write must land exactly OLD_CONTENT"
    );
}

/// The shared OLD-XOR-NEW classifier. Both the positive cases and the
/// negative controls go through this single function: the invariant holds
/// iff every observation classifies as Old or New, never Torn.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Observed {
    Old,
    New,
    Torn,
}

fn classify(content: &str) -> Observed {
    if content == OLD_CONTENT {
        Observed::Old
    } else if content == NEW_CONTENT {
        Observed::New
    } else {
        Observed::Torn
    }
}

fn tmp_residue(cfg: &Path) -> Option<PathBuf> {
    fs::read_dir(cfg).ok()?.flatten().find_map(|e| {
        let name = e.file_name();
        let name = name.to_string_lossy();
        name.starts_with(&format!("{CONFIG_FILE}.tmp."))
            .then(|| e.path())
    })
}

/// One interruption trial. Returns true if the interruption demonstrably
/// landed inside the crash window (rename denied), false if the child's
/// rename won the race (inconclusive; caller retries).
fn interruption_trial(cfg: &Path) -> bool {
    // Target must start as OLD, with no tmp residue.
    assert_eq!(classify(&fs::read_to_string(cfg.join(CONFIG_FILE)).unwrap()), Observed::Old);
    if let Some(p) = tmp_residue(cfg) {
        fs::remove_file(p).unwrap();
    }

    // Spawn the real binary writing NEW; std::process so we hold the Child.
    let mut child = std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .args(["config", "set", "policy-profile", "strict"])
        .env("QSC_CONFIG_DIR", cfg)
        .env("QSC_MARK_FORMAT", "plain")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("spawn qsc");

    // Watch for the deterministic tmp name write_atomic uses:
    // "<file>.tmp.<child pid>" (src/fs_store/mod.rs:105-110). On appearance,
    // deny the directory write permission: the child's open fd lets it finish
    // write_all + sync_all, but fs::rename(tmp, target) then fails.
    let tmp_path = cfg.join(format!("{CONFIG_FILE}.tmp.{}", child.id()));
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut interrupted = false;
    loop {
        if tmp_path.exists() {
            fs::set_permissions(cfg, Permissions::from_mode(0o500)).unwrap();
            interrupted = true;
            break;
        }
        if let Some(_status) = child.try_wait().expect("try_wait") {
            break; // child finished before we ever saw the tmp — inconclusive
        }
        if Instant::now() > deadline {
            break;
        }
        std::hint::spin_loop();
    }
    let status = child.wait().expect("wait qsc");
    // Restore permissions before any assertion can bail out.
    fs::set_permissions(cfg, Permissions::from_mode(0o700)).unwrap();

    if !interrupted || status.success() {
        // Rename won the race: target is NEW. Reset to OLD and report
        // inconclusive so the caller retries.
        assert_eq!(
            classify(&fs::read_to_string(cfg.join(CONFIG_FILE)).unwrap()),
            Observed::New,
            "uninterrupted run must land exactly NEW_CONTENT"
        );
        qsc_cmd(cfg)
            .args(["config", "set", "policy-profile", "baseline"])
            .assert()
            .success();
        return false;
    }

    // Interruption landed between temp-write and rename. Assert the
    // crash-window invariant:
    // (1) subsequent reader sees the target byte-identical to OLD;
    let target_now = fs::read_to_string(cfg.join(CONFIG_FILE)).unwrap();
    assert_eq!(
        classify(&target_now),
        Observed::Old,
        "crash window violated OLD-XOR-NEW: target = {target_now:?}"
    );
    // (2) the temp file exists as residue beside the target — it was never
    //     observed AS the live target — and holds the complete NEW content
    //     (write_all finished; only the rename was denied);
    let residue = tmp_residue(cfg).expect("tmp residue must exist in the crash window");
    assert_eq!(
        fs::read_to_string(&residue).unwrap(),
        NEW_CONTENT,
        "tmp residue must hold the complete NEW content"
    );
    // (3) recovery: cleaning the residue and re-running the write lands NEW
    //     exactly (the NEW arm of OLD-XOR-NEW after a completed rename).
    fs::remove_file(&residue).unwrap();
    qsc_cmd(cfg)
        .args(["config", "set", "policy-profile", "strict"])
        .assert()
        .success();
    assert_eq!(
        classify(&fs::read_to_string(cfg.join(CONFIG_FILE)).unwrap()),
        Observed::New
    );
    true
}

/// POSITIVE, crash window: interrupt the real write_atomic between its
/// temp-write and its rename; a subsequent reader must see OLD exactly.
/// The test FAILS unless at least one trial demonstrably lands in-window —
/// an exercise that never fires cannot certify anything.
#[test]
fn wf0022_crash_window_subsequent_reader_sees_old_xor_new() {
    let cfg = unique_dir("na0639_crash_window");
    seed_store(&cfg);

    let mut landed = 0u32;
    for _ in 0..25 {
        if interruption_trial(&cfg) {
            landed += 1;
            break;
        }
        // interruption_trial reset the target to OLD before returning false
    }
    assert!(
        landed >= 1,
        "no trial landed inside the crash window (25 attempts) — the \
         interruption mechanism did not fire; the property was NOT exercised"
    );
}

/// POSITIVE, concurrent reader: repeated REAL write_atomic cycles
/// (baseline <-> strict through the binary) while a reader thread samples the
/// target continuously. Every sample must classify Old or New — never Torn.
#[test]
fn wf0022_concurrent_reader_sees_old_xor_new_across_real_writes() {
    let cfg = unique_dir("na0639_concurrent_pos");
    seed_store(&cfg);

    let stop = Arc::new(AtomicBool::new(false));
    let target = cfg.join(CONFIG_FILE);
    let reader = {
        let stop = Arc::clone(&stop);
        let target = target.clone();
        thread::spawn(move || {
            let mut samples = 0u64;
            let mut torn: Vec<String> = Vec::new();
            while !stop.load(Ordering::Relaxed) {
                if let Ok(content) = fs::read_to_string(&target) {
                    samples += 1;
                    if classify(&content) == Observed::Torn {
                        torn.push(content);
                    }
                }
            }
            (samples, torn)
        })
    };

    for i in 0..24 {
        let v = if i % 2 == 0 { "strict" } else { "baseline" };
        qsc_cmd(&cfg)
            .args(["config", "set", "policy-profile", v])
            .assert()
            .success();
    }
    stop.store(true, Ordering::Relaxed);
    let (samples, torn) = reader.join().unwrap();
    assert!(samples > 0, "reader thread never sampled the target");
    assert!(
        torn.is_empty(),
        "concurrent reader observed torn content across real write_atomic \
         cycles ({} of {} samples): {:?}",
        torn.len(),
        samples,
        &torn[..torn.len().min(3)]
    );
}

/// Test-only NON-atomic writer: truncate the live target in place, then
/// write the new content directly into it. This is the deliberately broken
/// path the negative controls use. It is local to this harness and is NOT a
/// change to production write_atomic.
fn torn_write_inplace(path: &Path, content: &str, pause: Duration) {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();
    f.sync_all().unwrap(); // truncated state durably visible: the torn window
    thread::sleep(pause);
    f.write_all(content.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

/// NEGATIVE CONTROL, crash window (WF-0017): a crash mid in-place write
/// leaves a prefix of NEW on the live target. The SAME classifier must
/// detect it as Torn — proving the positive assertions could have failed.
#[test]
fn wf0022_negative_control_inplace_crash_is_detected_as_torn() {
    let cfg = unique_dir("na0639_negctl_crash");
    seed_store(&cfg);
    let target = cfg.join(CONFIG_FILE);

    // Simulated crash mid in-place write: only half of NEW gets written.
    let half = &NEW_CONTENT[..NEW_CONTENT.len() / 2];
    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&target)
        .unwrap();
    f.write_all(half.as_bytes()).unwrap();
    f.sync_all().unwrap();
    drop(f); // crash: the rest of NEW never arrives

    let observed = fs::read_to_string(&target).unwrap();
    assert_eq!(
        classify(&observed),
        Observed::Torn,
        "the classifier MUST flag a half-written target; if this fails the \
         harness is vacuous (WF-0017)"
    );
}

/// NEGATIVE CONTROL, concurrent reader (WF-0017): the same reader loop run
/// against the non-atomic in-place writer MUST observe at least one torn
/// sample. If it cannot, the concurrent positive test is vacuous.
#[test]
fn wf0022_negative_control_concurrent_reader_catches_inplace_writer() {
    let cfg = unique_dir("na0639_negctl_concurrent");
    seed_store(&cfg);
    let target = cfg.join(CONFIG_FILE);

    let stop = Arc::new(AtomicBool::new(false));
    let reader = {
        let stop = Arc::clone(&stop);
        let target = target.clone();
        thread::spawn(move || {
            let mut torn = 0u64;
            while !stop.load(Ordering::Relaxed) {
                if let Ok(content) = fs::read_to_string(&target) {
                    if classify(&content) == Observed::Torn {
                        torn += 1;
                    }
                }
            }
            torn
        })
    };

    // The torn window is held open (truncate ... pause ... write) so the
    // reader deterministically observes it. Alternate old/new like the
    // positive test.
    for i in 0..10 {
        let v = if i % 2 == 0 { NEW_CONTENT } else { OLD_CONTENT };
        torn_write_inplace(&target, v, Duration::from_millis(5));
    }
    stop.store(true, Ordering::Relaxed);
    let torn = reader.join().unwrap();
    assert!(
        torn >= 1,
        "the reader never caught the deliberately torn writer — the \
         concurrent harness would be vacuous (WF-0017)"
    );
}
