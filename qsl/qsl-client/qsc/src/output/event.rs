//! NA-0779 (D-1422) -- THE DEBUG LOG'S TYPED EVENT SINK: the engine half of the operator's diagnostic log.
//!
//! WHAT THIS IS. A second consumer of the ONE marker choke point (`emit_marker`, this module's parent).
//! Every marker the engine emits is offered to an optional sink as a TYPED `Event` built by
//! `event_from_marker`, which copies a kv pair ONLY if its key is in one of three allowlists generated
//! from the marker census (`event_tables.rs`: INT_KEYS, BOOL_KEYS, ENUM_KEYS with each enum's CLOSED
//! vocabulary) and DROPS everything else WITHOUT READING THE VALUE. That is the redaction: an alias, an
//! invite code, a token, a route token, a URL, key material, free text, a raw size or an identity-derived
//! hash never enters because its KEY is not listed -- not because a filter recognised the value. An enum
//! value outside its vocabulary becomes the literal `?`. The record has no free-text field.
//!
//! WHAT THIS IS NOT. It is not a change to what the CLI prints: `emit_marker` formats and routes the
//! marker line exactly as before, and the CLI never installs a sink (opt-in per session, `set_event_sink`;
//! the slot is `None` until a host installs one). It is not the env-switched file logger beside it
//! (`log_marker`, `QSC_LOG`): that one is the ENV lane's and is neither used nor touched here.
//!
//! THE RECORD. STOP_NA0779_002 sec 3 (the model the operator blessed 2026-09-05,
//! `RBANK_debug_log_event_model_blessed_20260905.md`): `seq` and `utc` are the HOST's (the ring assigns
//! them); this crate supplies the typed body and the one line formatter every consumer shares
//! (`Event::to_line`), so the viewer, the export and the harness capture render identical bytes.
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

include!("event_tables.rs");

/// The literal an enum value outside its closed vocabulary becomes. Never a copy of the input.
pub const UNLISTED: &str = "?";

/// The two levels of the model. An `Events` event is also part of `Detailed`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Level {
    Events,
    Detailed,
}

/// Who emitted the event. The engine only ever produces `Engine`; the desktop's gateway and UI
/// produce the other two through the same record type so one formatter serves all three.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Engine,
    Gateway,
    Ui,
}

/// A derived outcome: from an `ok` field, a reject/refuse name, or a skip name.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Outcome {
    Ok,
    Fail,
    Refused,
    Skipped,
}

/// The typed event. Every string in it is a `&'static str` taken FROM A TABLE (a name from the level
/// lists, a code or reason from a closed vocabulary, an enum member), so by type nothing here can be a
/// copy of an input value: the constructor can only SELECT a listed literal, never carry one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Event {
    pub level: Level,
    pub source: Source,
    pub name: &'static str,
    pub code: Option<&'static str>,
    pub outcome: Option<Outcome>,
    pub reason: Option<&'static str>,
    pub duration_ms: Option<u32>,
    pub ints: Vec<(&'static str, i64)>,
    pub bools: Vec<(&'static str, bool)>,
    pub enums: Vec<(&'static str, &'static str)>,
}

/// The allowlist the constructor applies. `event_from_marker` uses `ALLOWLIST`; a test may pass its own
/// to prove the arm discriminates (STOP 003's red arm passes a permissive list and sees the plants).
#[derive(Clone, Copy, Debug)]
pub struct Allowlist {
    pub ints: &'static [&'static str],
    pub bools: &'static [&'static str],
    pub enums: &'static [(&'static str, &'static [&'static str])],
    pub events: &'static [&'static str],
    pub detailed_only: &'static [&'static str],
}

/// THE allowlist: the census's classification, the operator's bless on bytes.
pub const ALLOWLIST: Allowlist = Allowlist {
    ints: INT_KEYS,
    bools: BOOL_KEYS,
    enums: ENUM_KEYS,
    events: LEVEL_EVENTS,
    detailed_only: LEVEL_DETAILED_ONLY,
};

type Sink = Box<dyn Fn(&Event) + Send + Sync + 'static>;
static SINK_INSTALLED: AtomicBool = AtomicBool::new(false);
static SINK: OnceLock<Mutex<Option<Sink>>> = OnceLock::new();

fn sink_cell() -> &'static Mutex<Option<Sink>> {
    SINK.get_or_init(|| Mutex::new(None))
}

/// Install (Some) or remove (None) the session's sink. The host calls it at unlock with the log on and
/// again with `None` at lock. With no sink installed `feed` costs one relaxed atomic load and returns.
pub fn set_event_sink(sink: Option<Sink>) {
    let installed = sink.is_some();
    *sink_cell().lock().unwrap_or_else(|p| p.into_inner()) = sink;
    SINK_INSTALLED.store(installed, Ordering::SeqCst);
}

pub fn event_sink_installed() -> bool {
    SINK_INSTALLED.load(Ordering::Relaxed)
}

/// Called by `emit_marker` between formatting the line and routing it. Never changes the line.
pub(crate) fn feed(event: &str, code: Option<&str>, kv: &[(&str, &str)]) {
    if !SINK_INSTALLED.load(Ordering::Relaxed) {
        return;
    }
    if let Some(ev) = event_from_marker(event, code, kv) {
        let guard = sink_cell().lock().unwrap_or_else(|p| p.into_inner());
        if let Some(sink) = guard.as_ref() {
            sink(&ev);
        }
    }
}

fn listed(table: &'static [&'static str], s: &str) -> Option<&'static str> {
    table.iter().copied().find(|m| *m == s)
}

fn level_of(allow: &Allowlist, name: &str) -> Option<(Level, &'static str)> {
    if let Some(n) = listed(allow.events, name) {
        return Some((Level::Events, n));
    }
    if let Some(n) = listed(allow.detailed_only, name) {
        return Some((Level::Detailed, n));
    }
    None
}

/// The closed-vocabulary lookup for `code` and `reason` fields: the `reason` key's vocabulary (which
/// holds the REJECT_* codes and every literal marker code) is the shared one.
fn code_member(allow: &Allowlist, s: &str) -> Option<&'static str> {
    let vocab = allow
        .enums
        .iter()
        .find(|(k, _)| *k == "reason")
        .map(|(_, v)| *v)?;
    Some(listed(vocab, s).unwrap_or(UNLISTED))
}

/// Build the typed event from a marker's parts, under `allow`. Returns `None` when the name is in no
/// level list (a marker the model excludes, or one added upstream that no edit here has admitted).
pub fn event_from_marker_with(
    allow: &Allowlist,
    event: &str,
    code: Option<&str>,
    kv: &[(&str, &str)],
) -> Option<Event> {
    let (level, name) = level_of(allow, event)?;
    let mut ev = Event {
        level,
        source: Source::Engine,
        name,
        code: code.and_then(|c| code_member(allow, c)),
        outcome: None,
        reason: None,
        duration_ms: None,
        ints: Vec::new(),
        bools: Vec::new(),
        enums: Vec::new(),
    };
    for (k, v) in kv {
        if let Some(key) = listed(allow.ints, k) {
            if let Ok(n) = v.trim().parse::<i64>() {
                ev.ints.push((key, n));
            }
            continue;
        }
        if let Some(key) = listed(allow.bools, k) {
            match *v {
                "true" | "yes" | "1" => ev.bools.push((key, true)),
                "false" | "no" | "0" => ev.bools.push((key, false)),
                _ => {}
            }
            continue;
        }
        if let Some((key, vocab)) = allow.enums.iter().find(|(key, _)| key == k) {
            let member = listed(vocab, v).unwrap_or(UNLISTED);
            if *key == "reason" {
                ev.reason = Some(member);
            } else {
                ev.enums.push((key, member));
            }
            continue;
        }
        // Every other key -- DERIVED-FROM-IDENTITY, VALUE, or unknown -- is dropped here without its
        // value being read. This `continue` is the redaction.
    }
    ev.outcome = derive_outcome(name, &ev.bools);
    Some(ev)
}

/// The constructor under THE allowlist.
pub fn event_from_marker(event: &str, code: Option<&str>, kv: &[(&str, &str)]) -> Option<Event> {
    event_from_marker_with(&ALLOWLIST, event, code, kv)
}

fn derive_outcome(name: &str, bools: &[(&'static str, bool)]) -> Option<Outcome> {
    if let Some((_, ok)) = bools.iter().find(|(k, _)| *k == "ok") {
        return Some(if *ok { Outcome::Ok } else { Outcome::Fail });
    }
    if name.contains("reject") || name.contains("refuse") || name.contains("violation") {
        return Some(Outcome::Refused);
    }
    if name.contains("skip") || name.contains("ignored") || name.contains("not_consumed") {
        return Some(Outcome::Skipped);
    }
    if name.ends_with("_failed") || name == "error" {
        return Some(Outcome::Fail);
    }
    None
}

impl Level {
    pub fn tag(self) -> &'static str {
        match self {
            Level::Events => "e",
            Level::Detailed => "d",
        }
    }
}

impl Source {
    pub fn tag(self) -> &'static str {
        match self {
            Source::Engine => "eng",
            Source::Gateway => "gw",
            Source::Ui => "ui",
        }
    }
}

impl Outcome {
    pub fn tag(self) -> &'static str {
        match self {
            Outcome::Ok => "ok",
            Outcome::Fail => "fail",
            Outcome::Refused => "refused",
            Outcome::Skipped => "skipped",
        }
    }
}

/// RFC 3339 with milliseconds from Unix milliseconds, no dependency (Howard Hinnant's civil-from-days).
pub fn utc_rfc3339_ms(utc_ms: u64) -> String {
    let secs = (utc_ms / 1000) as i64;
    let ms = utc_ms % 1000;
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        y,
        m,
        d,
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60,
        ms
    )
}

impl Event {
    /// THE one line every consumer renders: the viewer, the export and the harness capture. ASCII by
    /// construction: every token is a table literal, an integer, `t`/`f`, or the timestamp.
    /// `seq` and `utc_ms` belong to the host's ring.
    pub fn to_line(&self, seq: u64, utc_ms: u64) -> String {
        let mut s = format!(
            "seq={} utc={} lvl={} src={} ev={}",
            seq,
            utc_rfc3339_ms(utc_ms),
            self.level.tag(),
            self.source.tag(),
            self.name
        );
        if let Some(c) = self.code {
            s.push_str(" code=");
            s.push_str(c);
        }
        if let Some(o) = self.outcome {
            s.push_str(" out=");
            s.push_str(o.tag());
        }
        if let Some(r) = self.reason {
            s.push_str(" reason=");
            s.push_str(r);
        }
        if let Some(d) = self.duration_ms {
            s.push_str(&format!(" dur={}", d));
        }
        let mut ints = self.ints.clone();
        ints.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in ints {
            s.push_str(&format!(" n.{}={}", k, v));
        }
        let mut enums = self.enums.clone();
        enums.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in enums {
            s.push_str(&format!(" c.{}={}", k, v));
        }
        let mut bools = self.bools.clone();
        bools.sort_by(|a, b| a.0.cmp(b.0));
        for (k, v) in bools {
            s.push_str(&format!(" b.{}={}", k, if v { "t" } else { "f" }));
        }
        debug_assert!(s.is_ascii());
        s
    }
}
