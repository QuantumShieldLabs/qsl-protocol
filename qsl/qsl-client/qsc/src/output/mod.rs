use serde_json::Map;
use std::collections::VecDeque;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Mutex, OnceLock};

const MARKER_SCHEMA_V1: u8 = 1;
const PANIC_REDACTED_MARKER: &str = "QSC_MARK/1 event=panic code=panic_redacted";
pub(crate) const PANIC_DEMO_SENTINEL: &str = "QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK";

#[derive(Debug, Clone, Copy)]
enum MarkerFormat {
    Plain,
    Jsonl,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum MarkerRouting {
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

pub(crate) fn install_panic_redaction_hook() {
    std::panic::set_hook(Box::new(|_| {
        let _ = std::io::stderr().write_all(PANIC_REDACTED_MARKER.as_bytes());
        let _ = std::io::stderr().write_all(b"\n");
    }));
}

pub(crate) fn emit_tui_named_marker(label: &str, fields: &[(&str, &str)]) {
    if !(env_bool("QSC_TUI_HEADLESS") || env_bool("QSC_TUI_TEST_MODE")) {
        return;
    }
    let mut line = String::from(label);
    for (k, v) in fields {
        line.push(' ');
        line.push_str(k);
        line.push('=');
        line.push_str(v);
    }
    println!("{}", line);
}

pub(crate) fn emit_cli_named_marker(label: &str, fields: &[(&str, &str)]) {
    let mut line = String::from(label);
    for (k, v) in fields {
        line.push(' ');
        line.push_str(k);
        line.push('=');
        line.push_str(v);
    }
    println!("{}", line);
}

pub(crate) fn qsc_mark(event: &str, code: &str) {
    emit_marker(event, Some(code), &[]);
}

pub(crate) fn qsc_sanitize_terminal_text(input: &str) -> String {
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

pub(crate) fn print_marker(event: &str, kv: &[(&str, &str)]) {
    emit_marker(event, None, kv);
}

pub(crate) fn print_error_marker(code: &str) -> ! {
    emit_marker("error", Some(code), &[]);
    process::exit(1);
}

pub(crate) fn set_marker_routing(routing: MarkerRouting) {
    let value = match routing {
        MarkerRouting::Stdout => 0,
        MarkerRouting::InApp => 1,
    };
    MARKER_ROUTING.store(value, Ordering::SeqCst);
}

pub(crate) fn marker_queue() -> &'static Mutex<VecDeque<String>> {
    MARKER_QUEUE.get_or_init(|| Mutex::new(VecDeque::new()))
}

pub(crate) fn init_output_policy(reveal: bool) {
    let _ = OUTPUT_POLICY.set(OutputPolicy { reveal });
}

pub(crate) fn emit_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
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

pub(crate) fn redact_text_for_output(value: &str) -> String {
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
