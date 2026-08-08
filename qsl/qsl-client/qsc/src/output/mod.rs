use serde_json::Map;
use std::collections::VecDeque;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

const MARKER_SCHEMA_V1: u8 = 1;
const PANIC_REDACTED_MARKER: &str = "QSC_MARK/1 event=panic code=panic_redacted";
pub const PANIC_DEMO_SENTINEL: &str = "QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK";

#[derive(Debug, Clone, Copy)]
enum MarkerFormat {
    Plain,
    Jsonl,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MarkerRouting {
    Stdout,
    InApp,
}

static MARKER_ROUTING: AtomicU8 = AtomicU8::new(0);
static MARKER_QUEUE: OnceLock<Mutex<VecDeque<String>>> = OnceLock::new();

#[derive(Clone, Copy)]
struct OutputPolicy {
    reveal: bool,
}

static OUTPUT_POLICY: OnceLock<OutputPolicy> = OnceLock::new();

pub fn install_panic_redaction_hook() {
    std::panic::set_hook(Box::new(|_| {
        let _ = std::io::stderr().write_all(PANIC_REDACTED_MARKER.as_bytes());
        let _ = std::io::stderr().write_all(b"\n");
    }));
}

// ---------------------------------------------------------------------------
// NA-0700 (D634 A2-FINAL item 1a; D-1340) — the routed raw-line sink.
//
// The 23 raw print statements (the 21 payload sites in contacts/lib plus the
// two named-marker emitters below) route through here instead of bypassing
// the output layer. Four constraints, ruled verbatim (R132):
//   - bypass `format_marker_line` — no `QSC_MARK/1` prefix, no kv redaction
//     on the raw Stdout path;
//   - bypass `marker_format()` — raw lines ignore QSC_MARK_FORMAT (a
//     jsonl-honouring sink would reshape them unseen);
//   - bypass `log_marker` — no new QSC_LOG side effects;
//   - EMIT SYNCHRONOUSLY AT THE CALL POINT via `println!` — same LineWriter,
//     same lock, same line-atomic write; a queue-then-flush sink would
//     reorder against unbuffered stderr for every `2>&1` consumer.
// Under default Stdout routing the emitted bytes are IDENTICAL to the direct
// prints these calls replace (the golden-output control is the instrument).
//
// InApp arms, ruled semantics (D-1340):
//   - payload lines are DROPPED (R143) — the desktop consumes return values,
//     not payload; fingerprints and public keys are a correlation surface in
//     shareable debug artifacts (ring buffer → debug log → bug report) even
//     though they are not secrets;
//   - CLI and TUI named-marker lines are queued REDACT-ON-QUEUE (R148/R152):
//     each field value passes through `redact_value_for_output` — the SAME
//     shared gate the formatted `QSC_MARK/1` vocabulary already uses
//     (`format_marker_line`), never a bespoke copy (R156) — before line
//     assembly. One mechanism, both vocabularies, no carve-out. The queue's
//     resulting two-vocabulary shape (`QSC_MARK/1 event=…` + `LABEL k=v`) is
//     a stated decision; the first token discriminates.
// ---------------------------------------------------------------------------

/// Raw payload line (the `key=value` data lines the CLI prints for scripts
/// and peers): Stdout = byte-identical synchronous print; InApp = DROPPED
/// (R143 — the routing decision the bypass never made).
pub(crate) fn emit_raw_payload_line(line: &str) {
    match marker_routing() {
        MarkerRouting::Stdout => println!("{}", line),
        MarkerRouting::InApp => {}
    }
}

fn emit_named_marker_line(label: &str, fields: &[(&str, &str)]) {
    match marker_routing() {
        MarkerRouting::Stdout => {
            let mut line = String::from(label);
            for (k, v) in fields {
                line.push(' ');
                line.push_str(k);
                line.push('=');
                line.push_str(v);
            }
            println!("{}", line);
        }
        MarkerRouting::InApp => {
            let mut line = String::from(label);
            for (k, v) in fields {
                line.push(' ');
                line.push_str(k);
                line.push('=');
                line.push_str(&redact_value_for_output(k, v));
            }
            let mut queue = marker_queue().lock().expect("marker queue lock");
            queue.push_back(line);
        }
    }
}

pub(crate) fn emit_tui_named_marker(label: &str, fields: &[(&str, &str)]) {
    // I-5 (R119(c)): the env gate stays the FIRST statement of the routed
    // path — with the TUI envs unset this emits NOTHING to either
    // destination, so the desktop's silence is unchanged.
    if !(env_bool("QSC_TUI_HEADLESS") || env_bool("QSC_TUI_TEST_MODE")) {
        return;
    }
    emit_named_marker_line(label, fields);
}

pub(crate) fn emit_cli_named_marker(label: &str, fields: &[(&str, &str)]) {
    emit_named_marker_line(label, fields);
}

pub fn qsc_mark(event: &str, code: &str) {
    emit_marker(event, Some(code), &[]);
}

pub fn qsc_sanitize_terminal_text(input: &str) -> String {
    // Terminal-safe deterministic sanitizer:
    // - drop ESC (0x1b) and ASCII control chars (except \n and \t)
    // - drop DEL (0x7f)
    let mut out = String::with_capacity(input.len());
    let mut it = input.chars().peekable();
    let mut in_csi = false;
    while let Some(ch) = it.next() {
        let c = ch as u32;
        if in_csi {
            // ANSI CSI sequences end at a final byte in the range 0x40-0x7E.
            if (0x40..=0x7e).contains(&c) {
                in_csi = false;
            }
            continue;
        }
        if c == 0x1b || c == 0x7f {
            // If this is a CSI introducer, skip until its final byte.
            if let Some('[') = it.peek().copied() {
                let _ = it.next();
                in_csi = true;
            }
            continue;
        }
        if ch == '\n' || ch == '\t' {
            out.push(ch);
            continue;
        }
        if c < 0x20 {
            continue;
        }
        if ch.is_control() {
            continue;
        }
        out.push(ch);
    }
    out
}

pub fn print_marker(event: &str, kv: &[(&str, &str)]) {
    emit_marker(event, None, kv);
}

// NA-0646 (D582) PR-B: the messaging core returns errors instead of exiting the
// process (a library cannot exit its host). Exit semantics live ONLY in the bin's
// single Err->emit+exit adapter. `Emitted` = the marker(s) were already emitted at
// the error site (the byte-safest pattern for kv/dynamic markers); `Code` = the bin
// adapter emits the plain error marker exactly as print_error_marker used to.
#[derive(Debug)]
pub enum CliError {
    Code(String),
    Emitted,
}

impl CliError {
    pub fn code(code: impl Into<String>) -> Self {
        CliError::Code(code.into())
    }
}

pub type CliResult<T = ()> = Result<T, CliError>;

// D581 KEEP -> NA-0646 (D582): part of the library's pub GUI surface, seeded for the GUI
// phase; dormant until the GUI consumes it (dead_code allowance retained meanwhile).
#[allow(dead_code)]
pub fn set_marker_routing(routing: MarkerRouting) {
    let value = match routing {
        MarkerRouting::Stdout => 0,
        MarkerRouting::InApp => 1,
    };
    MARKER_ROUTING.store(value, Ordering::SeqCst);
}

pub fn marker_queue() -> &'static Mutex<VecDeque<String>> {
    MARKER_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

pub fn init_output_policy(reveal: bool) {
    let _ = OUTPUT_POLICY.set(OutputPolicy { reveal });
}

pub fn emit_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
    let line = format_marker_line(event, code, kv);
    match marker_routing() {
        MarkerRouting::Stdout => println!("{}", line),
        MarkerRouting::InApp => {
            let mut queue = marker_queue().lock().expect("marker queue lock");
            queue.push_back(line);
        }
    }
    log_marker(event, code, kv);
}

pub fn redact_text_for_output(value: &str) -> String {
    if output_policy().reveal {
        return value.to_string();
    }
    if should_redact_value("", value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn env_bool(name: &str) -> bool {
    match env::var(name) {
        Ok(v) => {
            let trimmed = v.trim();
            trimmed == "1"
                || trimmed.eq_ignore_ascii_case("true")
                || trimmed.eq_ignore_ascii_case("yes")
                || trimmed.eq_ignore_ascii_case("on")
        }
        Err(_) => false,
    }
}

fn marker_routing() -> MarkerRouting {
    match MARKER_ROUTING.load(Ordering::SeqCst) {
        1 => MarkerRouting::InApp,
        _ => MarkerRouting::Stdout,
    }
}

fn output_policy() -> OutputPolicy {
    *OUTPUT_POLICY
        .get()
        .unwrap_or(&OutputPolicy { reveal: false })
}

fn marker_format() -> MarkerFormat {
    match env::var("QSC_MARK_FORMAT").ok().as_deref() {
        Some("jsonl") | Some("JSONL") => MarkerFormat::Jsonl,
        _ => MarkerFormat::Plain,
    }
}

fn format_marker_line(event: &str, code: Option<&str>, kv: &[(&str, &str)]) -> String {
    match marker_format() {
        MarkerFormat::Plain => {
            let mut line = format!("QSC_MARK/1 event={}", event);
            if let Some(c) = code {
                line.push_str(&format!(" code={}", c));
            }
            for (k, v) in kv {
                let rv = redact_value_for_output(k, v);
                line.push_str(&format!(" {}={}", k, rv));
            }
            line
        }
        MarkerFormat::Jsonl => {
            let mut obj = Map::new();
            obj.insert("v".to_string(), serde_json::Value::from(MARKER_SCHEMA_V1));
            obj.insert("event".to_string(), serde_json::Value::from(event));
            if let Some(c) = code {
                obj.insert("code".to_string(), serde_json::Value::from(c));
            }
            if !kv.is_empty() {
                let mut kv_map = Map::new();
                for (k, v) in kv {
                    kv_map.insert(
                        (*k).to_string(),
                        serde_json::Value::from(redact_value_for_output(k, v)),
                    );
                }
                obj.insert("kv".to_string(), serde_json::Value::Object(kv_map));
            }
            serde_json::Value::Object(obj).to_string()
        }
    }
}

fn redact_value_for_output(key: &str, value: &str) -> String {
    if output_policy().reveal {
        return value.to_string();
    }
    if should_redact_value(key, value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn redact_value_for_log(key: &str, value: &str) -> String {
    if should_redact_value(key, value) {
        return "<redacted>".to_string();
    }
    value.to_string()
}

fn should_redact_value(key: &str, value: &str) -> bool {
    let k = key.to_ascii_lowercase();
    if k == "checked_dir" || k == "peer_fp" || k == "fp" || k == "pinned_fp" || k == "seen_fp" {
        return false;
    }
    if k == "value"
        || k == "config_dir"
        || k.contains("passphrase")
        || k.contains("secret")
        || k.contains("token")
        || k == "path"
        || k == "url"
        || k == "endpoint"
        || k == "timestamp"
    {
        return true;
    }
    looks_like_url(value) || looks_like_timestamp(value) || looks_high_cardinality(value)
}

fn looks_like_url(value: &str) -> bool {
    let v = value.to_ascii_lowercase();
    v.contains("http://") || v.contains("https://")
}

fn looks_like_timestamp(value: &str) -> bool {
    let v = value.as_bytes();
    if v.len() < 19 {
        return false;
    }
    value.contains('T') && value.contains(':') && value.contains('-')
}

fn looks_high_cardinality(value: &str) -> bool {
    value.len() >= 24 && value.chars().any(|c| c.is_ascii_digit())
}

fn log_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
    if env::var("QSC_LOG").ok().as_deref() != Some("1") {
        return;
    }
    let path = match env::var("QSC_LOG_PATH").ok() {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => return,
    };

    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    let mut obj = Map::new();
    obj.insert("v".to_string(), serde_json::Value::from(MARKER_SCHEMA_V1));
    obj.insert("event".to_string(), serde_json::Value::from(event));
    if let Some(c) = code {
        obj.insert("code".to_string(), serde_json::Value::from(c));
    }
    if !kv.is_empty() {
        let mut kv_map = Map::new();
        for (k, v) in kv {
            kv_map.insert(
                (*k).to_string(),
                serde_json::Value::from(redact_value_for_log(k, v)),
            );
        }
        obj.insert("kv".to_string(), serde_json::Value::Object(kv_map));
    }
    obj.insert("redacted".to_string(), serde_json::Value::from(true));

    let line = serde_json::Value::Object(obj).to_string() + "\n";
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .and_then(|mut f| f.write_all(line.as_bytes()));
}

// D581 KEEP -> NA-0646 (D582): the InApp routing is the GUI's event sink, pub since the
// crate split — its only
// producers were the TUI — but it is the event sink the GUI phase builds on. This test
// keeps it off zero coverage until then.
#[cfg(test)]
mod inapp_routing_tests {
    use super::{
        emit_marker, emit_raw_payload_line, emit_tui_named_marker, marker_queue,
        set_marker_routing, MarkerRouting,
    };
    use std::sync::{Mutex, MutexGuard, OnceLock};

    /// Marker ROUTING is process-global (one AtomicU8), and every test in
    /// this mod toggles it — so they ALL serialize here, or the parallel
    /// runner can flip a sibling's routing mid-test (a latent race the C4
    /// perturbation control exposed by shifting timings). The TUI-env
    /// mutations ride the same lock.
    static ROUTING_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn routing_lock() -> MutexGuard<'static, ()> {
        ROUTING_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|p| p.into_inner())
    }

    /// Extract exactly this test's lines from the process-global queue (the
    /// retain pattern the existing test set), leaving other tests' lines.
    fn drain_matching(needle: &str) -> Vec<String> {
        let mut queue = marker_queue().lock().expect("marker queue lock");
        let mut out = Vec::new();
        queue.retain(|line| {
            if line.contains(needle) {
                out.push(line.clone());
                false
            } else {
                true
            }
        });
        out
    }

    /// NA-0700 AM-3/R143 (control C2's target): payload lines are DROPPED
    /// under InApp — raw payload never enters the redaction-disciplined
    /// queue. Red-capable; delta symbol: `emit_raw_payload_line`'s InApp arm.
    #[test]
    fn payload_lines_dropped_under_inapp() {
        let _g = routing_lock();
        set_marker_routing(MarkerRouting::InApp);
        emit_raw_payload_line("identity_fp=QSCFP-na0700-payload-probe-0123456789");
        set_marker_routing(MarkerRouting::Stdout);
        assert!(
            drain_matching("na0700-payload-probe").is_empty(),
            "InApp must DROP payload lines (R143), not queue them"
        );
    }

    /// NA-0700 AM-6 (the fourth matrix cell; controls C1/C4's target): envs
    /// SET + InApp routes the TUI line into the queue REDACTED per AM-3 — the
    /// long digit-bearing thread value arrives as `<redacted>` through the
    /// SAME `redact_value_for_output` gate the formatted vocabulary uses; the
    /// short mode value passes verbatim. Delta symbol: the routed TUI emit
    /// call (`emit_named_marker_line`'s InApp arm).
    #[test]
    fn tui_named_line_queues_redacted_when_envs_set() {
        let _g = routing_lock();
        std::env::set_var("QSC_TUI_TEST_MODE", "1");
        set_marker_routing(MarkerRouting::InApp);
        emit_tui_named_marker(
            "QSC_TUI_NA0700_CELL4",
            &[
                ("mode", "immediate"),
                ("thread", "dr-smith-oncology-2024-line"),
            ],
        );
        set_marker_routing(MarkerRouting::Stdout);
        std::env::remove_var("QSC_TUI_TEST_MODE");
        let got = drain_matching("QSC_TUI_NA0700_CELL4");
        assert_eq!(
            got,
            vec!["QSC_TUI_NA0700_CELL4 mode=immediate thread=<redacted>".to_string()],
            "envs SET + InApp must queue the TUI line redact-on-queue"
        );
    }

    /// NA-0700 I-5 queue-empty half (control C3's target): with the TUI envs
    /// UNSET the routed TUI path emits NOTHING — the env gate stays the FIRST
    /// statement of the routed path (the delta symbol), so the desktop's
    /// silence is unchanged. The stdout-empty half of I-5 is carried by the
    /// golden control's envs-unset scenario (an in-process test cannot read
    /// its own stdout).
    #[test]
    fn tui_path_queues_nothing_when_envs_unset() {
        let _g = routing_lock();
        std::env::remove_var("QSC_TUI_TEST_MODE");
        std::env::remove_var("QSC_TUI_HEADLESS");
        set_marker_routing(MarkerRouting::InApp);
        emit_tui_named_marker("QSC_TUI_NA0700_SILENCE", &[("mode", "off")]);
        set_marker_routing(MarkerRouting::Stdout);
        assert!(
            drain_matching("QSC_TUI_NA0700_SILENCE").is_empty(),
            "envs UNSET: the routed TUI path must queue nothing"
        );
    }

    #[test]
    fn inapp_routing_queues_markers_and_stdout_routing_bypasses_queue() {
        let _g = routing_lock();
        set_marker_routing(MarkerRouting::InApp);
        emit_marker("na0645_inapp_probe", Some("keep"), &[("field", "value")]);
        set_marker_routing(MarkerRouting::Stdout);

        let queued = {
            let mut queue = marker_queue().lock().expect("marker queue lock");
            let mut found = None;
            queue.retain(|line| {
                if line.contains("event=na0645_inapp_probe") {
                    found = Some(line.clone());
                    false
                } else {
                    true
                }
            });
            found
        };
        let line = queued.expect("InApp-routed marker must land in the marker queue");
        assert!(
            line.starts_with("QSC_MARK/1 event=na0645_inapp_probe")
                && line.contains("code=keep")
                && line.contains("field=value"),
            "queued marker must carry the full formatted line: {line}"
        );

        emit_marker("na0645_stdout_probe", None, &[]);
        let queue = marker_queue().lock().expect("marker queue lock");
        assert!(
            !queue.iter().any(|l| l.contains("na0645_stdout_probe")),
            "Stdout-routed markers must not land in the marker queue"
        );
    }
}
