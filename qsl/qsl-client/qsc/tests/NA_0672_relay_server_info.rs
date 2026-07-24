// NA-0672 (D608, D-1300): server-info consumer + relay token trio.
//
// SOCKET-FREE by construction. This file exercises the PURE classifier
// `relay_server_info_from_parts` over captured JSON bodies -- no listener, no
// loopback, no external host. The two live-transport outcomes (`CertNotTrusted`,
// `Unreachable`) cannot arise from a status+body pair; they are produced by the
// send-error path inside `relay_server_info` and are proven at ACCEPTANCE against
// the real inspiron rig (directive §4), the NA-0663 substitute-pin precedent.
//
// What the classifier CAN produce, and what is pinned here:
//   * Reachable{Open}, Reachable{Bearer}         (200 + valid auth.mode)
//   * AuthRequired{token_was_sent:true/false}    (401 + parseable QSL challenge)
//   * NotAQslRelay                               (200 w/o auth.mode; 401 w/o QSL
//                                                 challenge; any other status)
// plus the full-document deserialize and its additive-only unknown-field
// tolerance.

use qsc::transport::{relay_server_info_from_parts, RelayAuthMode, RelayServerInfoOutcome};
use serde_json::json;

/// A full 200 server-info document with the given `auth.mode`, matching the
/// DOC-SRV-006 / NA-0652 nested contract.
fn full_doc_body(mode: &str) -> serde_json::Value {
    json!({
        "server": "qsl-server",
        "version": "0.1.0",
        "name": "Ops Relay",
        "api": ["push_v1", "pull_v1", "pull_ack_lease_v1"],
        "auth": { "mode": mode },
        "limits": { "max_body_bytes": 1_048_576, "max_queue_depth": 1000 },
        "retention": { "ttl_secs": 604_800 },
        "directory": { "mode": "none" },
        "attachments": { "service_url": "https://att.example.test" },
        "kt": { "mode": "none" },
        "min_client_version": "0.1.0"
    })
}

/// The EXACT unauthenticated 401 challenge the deployed relay returns, pinned by
/// the Phase-0 pre-flight (`{"auth":{"mode":"bearer"},"server":"qsl-server"}`).
fn live_probe_401_body() -> serde_json::Value {
    json!({ "auth": { "mode": "bearer" }, "server": "qsl-server" })
}

#[test]
fn reachable_open_200_full_document() {
    let body = full_doc_body("open");
    match relay_server_info_from_parts(200, Some(&body), false) {
        RelayServerInfoOutcome::Reachable { auth_mode, doc } => {
            assert_eq!(auth_mode, RelayAuthMode::Open);
            assert_eq!(doc.auth_mode, RelayAuthMode::Open);
            assert_eq!(doc.name, "Ops Relay");
            assert_eq!(doc.version, "0.1.0");
            assert_eq!(doc.api, vec!["push_v1", "pull_v1", "pull_ack_lease_v1"]);
            assert_eq!(doc.max_body_bytes, 1_048_576);
            assert_eq!(doc.max_queue_depth, 1000);
            assert_eq!(doc.retention_ttl_secs, 604_800);
            assert_eq!(doc.directory_mode, "none");
            assert_eq!(doc.kt_mode, "none");
            assert_eq!(
                doc.attachments_service_url.as_deref(),
                Some("https://att.example.test")
            );
            assert_eq!(doc.min_client_version.as_deref(), Some("0.1.0"));
        }
        other => panic!("expected Reachable{{Open}}, got {other:?}"),
    }
}

#[test]
fn reachable_bearer_200_full_document() {
    let body = full_doc_body("bearer");
    match relay_server_info_from_parts(200, Some(&body), true) {
        RelayServerInfoOutcome::Reachable { auth_mode, doc } => {
            assert_eq!(auth_mode, RelayAuthMode::Bearer);
            assert_eq!(doc.auth_mode, RelayAuthMode::Bearer);
        }
        other => panic!("expected Reachable{{Bearer}}, got {other:?}"),
    }
}

#[test]
fn optional_fields_null_map_to_none() {
    let body = json!({
        "auth": { "mode": "open" },
        "name": "",
        "api": [],
        "attachments": { "service_url": null },
        "min_client_version": null
    });
    match relay_server_info_from_parts(200, Some(&body), false) {
        RelayServerInfoOutcome::Reachable { doc, .. } => {
            assert_eq!(doc.name, "");
            assert!(doc.attachments_service_url.is_none());
            assert!(doc.min_client_version.is_none());
            // Missing nested groups fall back to their defaults.
            assert_eq!(doc.max_body_bytes, 0);
            assert_eq!(doc.retention_ttl_secs, 0);
            assert_eq!(doc.directory_mode, "");
        }
        other => panic!("expected Reachable, got {other:?}"),
    }
}

// --- the two 401 disambiguations (token_was_sent) ---------------------------

#[test]
fn auth_required_token_rejected_when_token_was_sent() {
    let body = live_probe_401_body();
    assert_eq!(
        relay_server_info_from_parts(401, Some(&body), true),
        RelayServerInfoOutcome::AuthRequired {
            token_was_sent: true
        }
    );
}

#[test]
fn auth_required_token_absent_when_no_token_sent() {
    let body = live_probe_401_body();
    assert_eq!(
        relay_server_info_from_parts(401, Some(&body), false),
        RelayServerInfoOutcome::AuthRequired {
            token_was_sent: false
        }
    );
}

#[test]
fn live_deployed_401_probe_body_is_a_valid_challenge() {
    // Pins the Phase-0 pre-flight contract: the deployed relay's unauthenticated
    // 401 body carries auth.mode, so it classifies AuthRequired, never
    // NotAQslRelay. If the deployed challenge shape ever drops auth.mode this
    // fails, which is exactly the STOP the directive's pre-flight guards.
    let body = live_probe_401_body();
    assert!(matches!(
        relay_server_info_from_parts(401, Some(&body), true),
        RelayServerInfoOutcome::AuthRequired { .. }
    ));
}

// --- the two NotAQslRelay cases (FLAG-2 RULE) -------------------------------

#[test]
fn not_a_qsl_relay_200_without_auth_mode() {
    // A host answered 200 but the body is not a QSL relay contract.
    let body = json!({ "hello": "world", "server": "some-proxy" });
    assert_eq!(
        relay_server_info_from_parts(200, Some(&body), false),
        RelayServerInfoOutcome::NotAQslRelay
    );
}

#[test]
fn not_a_qsl_relay_200_with_unrecognised_auth_mode() {
    // auth.mode present but not in {open,bearer}: outside the contract boundary.
    let body = json!({ "auth": { "mode": "mutual-tls" } });
    assert_eq!(
        relay_server_info_from_parts(200, Some(&body), false),
        RelayServerInfoOutcome::NotAQslRelay
    );
}

#[test]
fn not_a_qsl_relay_401_without_qsl_challenge_body() {
    // A generic reverse-proxy 401 with NO parseable QSL challenge. The FLAG-2
    // RULE: the app must NOT tell a user "this relay requires a token" about
    // something that is not a relay.
    assert_eq!(
        relay_server_info_from_parts(401, None, false),
        RelayServerInfoOutcome::NotAQslRelay
    );
    let generic = json!({ "error": "unauthorized" });
    assert_eq!(
        relay_server_info_from_parts(401, Some(&generic), false),
        RelayServerInfoOutcome::NotAQslRelay
    );
    // A proxy 401 with token sent is STILL not-a-relay -- token_was_sent never
    // upgrades a non-challenge into AuthRequired.
    assert_eq!(
        relay_server_info_from_parts(401, Some(&generic), true),
        RelayServerInfoOutcome::NotAQslRelay
    );
}

#[test]
fn other_statuses_answered_but_not_the_contract_are_not_a_qsl_relay() {
    let body = full_doc_body("bearer");
    for status in [301u16, 404, 418, 500, 503] {
        assert_eq!(
            relay_server_info_from_parts(status, Some(&body), false),
            RelayServerInfoOutcome::NotAQslRelay,
            "status {status} should be NotAQslRelay"
        );
    }
}

// --- additive-only tolerance (OBS-H) ----------------------------------------

#[test]
fn unknown_fields_are_tolerated_top_level_and_nested() {
    // A newer relay adds fields an older client has never seen; the client must
    // still parse the document it understands and ignore the rest.
    let body = json!({
        "server": "qsl-server",
        "version": "0.2.0",
        "name": "Future Relay",
        "api": ["push_v1", "pull_v1", "pull_ack_lease_v1", "sealed_sender_v1"],
        "auth": { "mode": "bearer", "future_auth_field": { "nested": true } },
        "limits": { "max_body_bytes": 2048, "max_queue_depth": 9, "future_limit": 42 },
        "retention": { "ttl_secs": 3600 },
        "directory": { "mode": "none" },
        "attachments": { "service_url": null },
        "kt": { "mode": "none" },
        "min_client_version": null,
        "brand_new_top_level_object": { "anything": [1, 2, 3] }
    });
    match relay_server_info_from_parts(200, Some(&body), true) {
        RelayServerInfoOutcome::Reachable { auth_mode, doc } => {
            assert_eq!(auth_mode, RelayAuthMode::Bearer);
            assert_eq!(doc.version, "0.2.0");
            assert_eq!(doc.max_body_bytes, 2048);
            assert_eq!(doc.max_queue_depth, 9);
            assert!(doc.api.contains(&"sealed_sender_v1".to_string()));
        }
        other => panic!("expected Reachable{{Bearer}}, got {other:?}"),
    }
}

#[test]
fn reachable_classification_keys_only_on_auth_mode_despite_bad_field_types() {
    // auth.mode is a valid challenge, but a sibling field has the wrong JSON
    // type. The classification must NOT collapse to NotAQslRelay over a cosmetic
    // quirk: it stays Reachable, with the unparseable fields at their defaults.
    let body = json!({
        "auth": { "mode": "open" },
        "limits": { "max_body_bytes": "not-a-number", "max_queue_depth": 5 },
        "version": "0.1.0"
    });
    match relay_server_info_from_parts(200, Some(&body), false) {
        RelayServerInfoOutcome::Reachable { auth_mode, doc } => {
            assert_eq!(auth_mode, RelayAuthMode::Open);
            assert_eq!(doc.max_body_bytes, 0);
            assert_eq!(doc.max_queue_depth, 0);
        }
        other => panic!("expected Reachable{{Open}}, got {other:?}"),
    }
}
