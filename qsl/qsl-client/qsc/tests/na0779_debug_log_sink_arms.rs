//! NA-0779 (D-1422) STOP 003 -- the typed event sink's arms, RED FIRST.
//!
//! The sink is process-global (one slot), so every test here serialises on ARM_LOCK and installs and
//! removes its own sink. No test touches marker ROUTING or the queue: the CLI path is untouched by
//! construction and its byte-identity is proven outside cargo (the loopback markers, STOP 003 sec 5).
use qsc::output::event::{
    event_from_marker, event_from_marker_with, event_sink_installed, set_event_sink,
    utc_rfc3339_ms, Allowlist, Event, Level, Outcome, Source, ALLOWLIST, BOOL_KEYS, ENUM_KEYS,
    INT_KEYS, LEVEL_DETAILED_ONLY, LEVEL_EVENTS, UNLISTED,
};
use qsc::output::{emit_marker, print_marker, qsc_mark, set_marker_routing, MarkerRouting};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

static ARM_LOCK: OnceLock<Mutex<()>> = OnceLock::new();
fn arm_lock() -> MutexGuard<'static, ()> {
    ARM_LOCK
        .get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|p| p.into_inner())
}

fn capture() -> Arc<Mutex<Vec<Event>>> {
    let got: Arc<Mutex<Vec<Event>>> = Arc::new(Mutex::new(Vec::new()));
    let sink_side = Arc::clone(&got);
    set_event_sink(Some(Box::new(move |ev: &Event| {
        sink_side.lock().unwrap().push(ev.clone());
    })));
    got
}

/// THE SEVEN PLANTS (kickoff L5; STOP 002 sec 9): an alias, an invite code, a bearer token, a route
/// token, a relay URL, a key-shaped value, and an identity-derived hash -- each under the REAL key the
/// tree uses for that thing, at a REAL marker name of the level lists.
const PLANTS: [(&str, &str, &str); 7] = [
    ("contacts_add", "peer", "PLANT-alias-dana-oncology"),
    (
        "invite_cleared",
        "invite_id",
        "PLANT-QSLI-1-invitecode0123456789",
    ),
    (
        "relay_token_set",
        "token",
        "PLANT-bearer-Zx9Qw8Er7Ty6Ui5Op4As3Df2Gh1Jk0",
    ),
    (
        "recv_start",
        "mailbox",
        "PLANT-route-token-0f1e2d3c4b5a69788796a5b4c3d2e1f0",
    ),
    (
        "relay_server_info",
        "attachments_service_url",
        "https://plant.example.invalid/v1/attachments",
    ),
    (
        "outbox_contact",
        "peer_key",
        "PLANT-key-MCowBQYDK2VwAyEAplantplantplantplantplant",
    ),
    ("recv_start", "mailbox_hash", "PLANT-hash8-deadbeef"),
];

/// A permissive allowlist for the red arm: every plant key admitted as an enum whose vocabulary IS the
/// plant value, so "copy every key" is what this list does. It proves the input carried the plants and
/// that the arm can go red; the green arm uses the real list on the same input.
fn permissive() -> Allowlist {
    Allowlist {
        ints: INT_KEYS,
        bools: BOOL_KEYS,
        enums: &[
            ("peer", &["PLANT-alias-dana-oncology"]),
            ("invite_id", &["PLANT-QSLI-1-invitecode0123456789"]),
            ("token", &["PLANT-bearer-Zx9Qw8Er7Ty6Ui5Op4As3Df2Gh1Jk0"]),
            (
                "mailbox",
                &["PLANT-route-token-0f1e2d3c4b5a69788796a5b4c3d2e1f0"],
            ),
            (
                "attachments_service_url",
                &["https://plant.example.invalid/v1/attachments"],
            ),
            (
                "peer_key",
                &["PLANT-key-MCowBQYDK2VwAyEAplantplantplantplantplant"],
            ),
            ("mailbox_hash", &["PLANT-hash8-deadbeef"]),
        ],
        events: LEVEL_EVENTS,
        detailed_only: LEVEL_DETAILED_ONLY,
    }
}

#[test]
fn t1_redaction_arm_red_then_green_seven_plants() {
    let _g = arm_lock();
    // RED: under the permissive list every plant reaches the line -- 7 of 7 -- so the arm can fail.
    let red: Vec<String> = PLANTS
        .iter()
        .map(|(name, key, value)| {
            event_from_marker_with(&permissive(), name, None, &[(key, value)])
                .expect("a listed name")
                .to_line(1, 0)
        })
        .collect();
    let red_hits = PLANTS
        .iter()
        .zip(red.iter())
        .filter(|((_, _, v), line)| line.contains(v))
        .count();
    assert_eq!(
        red_hits, 7,
        "RED ARM: copy-every-key must show all seven plants: {red:?}"
    );
    // GREEN: the REAL allowlist, the SAME input, through the REAL choke point with a sink installed.
    set_marker_routing(MarkerRouting::InApp); // keep the seven marker lines off this test's stdout
    let got = capture();
    for (name, key, value) in PLANTS.iter() {
        emit_marker(name, None, &[(key, value)]);
    }
    set_event_sink(None);
    set_marker_routing(MarkerRouting::Stdout);
    let events = got.lock().unwrap().clone();
    assert_eq!(
        events.len(),
        7,
        "every plant marker's NAME is listed, so seven events arrive"
    );
    let lines: Vec<String> = events
        .iter()
        .enumerate()
        .map(|(i, e)| e.to_line(i as u64, 0))
        .collect();
    let green_hits = PLANTS
        .iter()
        .filter(|(_, _, v)| lines.iter().any(|l| l.contains(v)))
        .count();
    assert_eq!(
        green_hits, 0,
        "GREEN ARM: 0 of 7 plants may reach a line: {lines:?}"
    );
    // and not even a fragment: no plant value's first twelve characters appear anywhere in the debug form
    for (_, _, v) in PLANTS.iter() {
        let frag = &v[..12];
        assert!(
            !events.iter().any(|e| format!("{e:?}").contains(frag)),
            "a plant fragment {frag:?} reached the typed event"
        );
    }
}

#[test]
fn t2_level_membership_and_the_never_names() {
    let _g = arm_lock();
    let e = event_from_marker("vault_unlock", None, &[]).expect("events-level name");
    assert_eq!(e.level, Level::Events);
    let d = event_from_marker("relay_pull_diagnostic", None, &[]).expect("detailed-only name");
    assert_eq!(d.level, Level::Detailed);
    assert!(
        event_from_marker("na0645_inapp_probe", None, &[]).is_none(),
        "a test probe never enters"
    );
    assert!(
        event_from_marker("some_marker_added_upstream", None, &[]).is_none(),
        "an unlisted name never enters"
    );
    assert_eq!(
        LEVEL_EVENTS.len(),
        105,
        "104 census names + the one constant name (C2)"
    );
    assert_eq!(LEVEL_DETAILED_ONLY.len(), 60);
    let both: Vec<&&str> = LEVEL_EVENTS
        .iter()
        .filter(|n| LEVEL_DETAILED_ONLY.contains(n))
        .collect();
    assert!(both.is_empty(), "a name in both lists: {both:?}");
}

#[test]
fn t3_closed_vocabularies_and_typed_fields() {
    let _g = arm_lock();
    let e = event_from_marker(
        "handshake_reject",
        Some("REJECT_QSC_HS_REPLAY"),
        &[
            ("reason", "peer_mismatch"),
            ("msg", "A2"),
            ("role", "initiator"),
            ("count", "3"),
            ("ok", "false"),
        ],
    )
    .unwrap();
    assert_eq!(e.code, Some("REJECT_QSC_HS_REPLAY"));
    assert_eq!(e.reason, Some("peer_mismatch"));
    assert_eq!(e.outcome, Some(Outcome::Fail));
    assert!(e.enums.contains(&("msg", "A2")) && e.enums.contains(&("role", "initiator")));
    assert!(e.ints.contains(&("count", 3)));
    // a value outside its vocabulary becomes `?`, never a copy of the input
    let q = event_from_marker(
        "handshake_reject",
        Some("not-a-listed-code-PLANT"),
        &[("reason", "PLANT-free-text-reason"), ("msg", "PLANT")],
    )
    .unwrap();
    assert_eq!(q.code, Some(UNLISTED));
    assert_eq!(q.reason, Some(UNLISTED));
    assert!(q.enums.contains(&("msg", UNLISTED)));
    assert!(!format!("{q:?}").contains("PLANT"));
    // an int that does not parse and a bool that is neither are dropped, not carried
    let t = event_from_marker(
        "recv_commit",
        None,
        &[("count", "PLANT-not-a-number"), ("ok", "maybe-PLANT")],
    )
    .unwrap();
    assert!(t.ints.is_empty() && t.bools.is_empty() && t.outcome.is_none());
    // the three ratchet counters are INT keys
    for k in ["nr", "ns", "pn"] {
        assert!(
            INT_KEYS.contains(&k),
            "{k} joins the ratchet markers as an int"
        );
    }
    // every enum vocabulary member is code-shaped ASCII (no spaces, no quotes), so a line is ASCII by construction
    for (k, vocab) in ENUM_KEYS.iter() {
        for m in vocab.iter() {
            assert!(
                m.is_ascii() && !m.contains(' ') && !m.contains('"'),
                "vocabulary member {m:?} of {k}"
            );
        }
    }
    assert_eq!(ALLOWLIST.ints.len(), INT_KEYS.len());
}

#[test]
fn t4_opt_in_no_sink_no_event_and_the_slot_clears() {
    let _g = arm_lock();
    set_event_sink(None);
    assert!(!event_sink_installed());
    set_marker_routing(MarkerRouting::InApp);
    emit_marker("vault_unlock", None, &[("ok", "true")]); // nobody listens; nothing to assert but no panic
    let got = capture();
    assert!(event_sink_installed());
    emit_marker("vault_unlock", None, &[("ok", "true")]);
    set_event_sink(None);
    emit_marker("vault_unlock", None, &[("ok", "true")]); // after removal: not captured
    set_marker_routing(MarkerRouting::Stdout);
    assert_eq!(
        got.lock().unwrap().len(),
        1,
        "exactly the one marker emitted while the sink was installed"
    );
    assert!(!event_sink_installed());
}

#[test]
fn t5_the_line_every_consumer_shares() {
    let _g = arm_lock();
    let e = Event {
        level: Level::Detailed,
        source: Source::Engine,
        name: "relay_pull_diagnostic",
        code: None,
        outcome: Some(Outcome::Ok),
        reason: None,
        duration_ms: Some(133),
        ints: vec![("max", 16), ("count", 1)],
        bools: vec![("auth_present", true)],
        enums: vec![
            ("op", "pull"),
            ("api", "relay_pull_v1"),
            ("status_class", "2xx"),
        ],
    };
    let line = e.to_line(1201, 1_788_623_041_140);
    assert_eq!(
        line,
        "seq=1201 utc=2026-09-05T15:44:01.140Z lvl=d src=eng ev=relay_pull_diagnostic out=ok dur=133 n.count=1 n.max=16 c.api=relay_pull_v1 c.op=pull c.status_class=2xx b.auth_present=t"
    );
    assert!(line.is_ascii());
    assert_eq!(utc_rfc3339_ms(0), "1970-01-01T00:00:00.000Z");
    assert_eq!(utc_rfc3339_ms(951_868_800_000), "2000-03-01T00:00:00.000Z");
}

/// THE CENSUS'S BLIND SPOT, PINNED (STOP 003 E-1). STOP 002's census parsed `emit_marker(...)` call sites and
/// excluded the two thin wrappers `print_marker` / `qsc_mark` (output/mod.rs :170 / :127) as "helper re-entries"
/// without following them to their callers. Those callers emit 19 literal names at 31 sites; 18 of them are in
/// NO level list (banked: CENSUS_NA0779_wrapper_sites.txt). Under the blessed tables an unlisted NAME never
/// enters, so the gap is FAIL-CLOSED: this arm proves it through the real wrappers with a sink installed, and
/// the one wrapper-emitted name that IS listed (`vault_unlock`, vault/mod.rs :821) is the positive control. When
/// the operator rules the 18 in, the regenerated tables change this arm's count with them.
const WRAPPER_ONLY_NAMES: [&str; 18] = [
    "ack_plan",
    "config_get",
    "config_set",
    "config_set_refused",
    "doctor",
    "envelope_plan",
    "history_limit",
    "queue_limit",
    "retry_bound",
    "send_attempt",
    "send_commit",
    "send_prepare",
    "send_retry",
    "status",
    "timeout_ok",
    "util_sanitize",
    "vault_init",
    "vault_status",
];

#[test]
fn t6_wrapper_emitted_names_are_fail_closed_until_ruled() {
    let _g = arm_lock();
    for n in WRAPPER_ONLY_NAMES.iter() {
        assert!(
            !LEVEL_EVENTS.contains(n) && !LEVEL_DETAILED_ONLY.contains(n),
            "{n} is in a level list: regenerate this arm with the ruled tables"
        );
    }
    set_marker_routing(MarkerRouting::InApp);
    let got = capture();
    for n in WRAPPER_ONLY_NAMES.iter() {
        if *n == "util_sanitize" {
            qsc_mark(n, "ok");
        } else {
            print_marker(
                n,
                &[
                    ("ok", "true"),
                    ("path", "PLANT-path-vault-dir"),
                    ("remedy", "PLANT free text"),
                ],
            );
        }
    }
    let unlisted_seen = got.lock().unwrap().len();
    // the positive control: the wrapper path DOES reach the sink for a listed name
    print_marker("vault_unlock", &[("ok", "true"), ("state", "unlocked")]);
    set_event_sink(None);
    set_marker_routing(MarkerRouting::Stdout);
    let events = got.lock().unwrap().clone();
    assert_eq!(
        unlisted_seen, 0,
        "an unlisted name reached the sink: {events:?}"
    );
    assert_eq!(
        events.len(),
        1,
        "the positive control arrives through print_marker"
    );
    assert_eq!(events[0].name, "vault_unlock");
    assert!(events[0].bools.contains(&("ok", true)));
    // MEASURED, NOT DESIGNED: `unlocked` is only ever written at the wrapper site, so the census never saw
    // it and the blessed `state` vocabulary lacks it -- the value renders as `?` until ruled in (E-1).
    assert!(events[0].enums.contains(&("state", UNLISTED)));
    assert!(!format!("{events:?}").contains("PLANT"));
}
