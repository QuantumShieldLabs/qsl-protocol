//! NA-0751 (D-1393) — THE INVITE SURFACE, IN-PROCESS, AND THE SEVENTH CONNECT REASON.
//!
//! ⚠ ENVIRONMENT IS SET ONCE, BEFORE ANY SERVER STARTS. This file starts fixtures, whose
//! runtimes spawn threads, and `std::env::set_var` is unsound once other threads are running.
//! Every test takes the file guard.
//!
//! ⚠ THE SEED-FALLBACK PAIR IS PERMITTED HERE, and only here among this lane's files. It is
//! the suite's standard idiom (62 of 133 qsc test files set it) and it is the ONLY route the
//! tree demonstrates to a stored session: `qsp_session_store_key_load`
//! (`protocol_state:172-177`) needs the fallback because a mock vault has no real store key
//! (`tests/qsp_status_truthy.rs:150-151` does exactly this). It is FORBIDDEN in
//! `na0751_facade_locked_control.rs`, whose seals concern real lock behaviour.
//!
//! ⚠ THE ACTIVE ARM IS DRIVEN BY A SUBPROCESS, and that is a measurement, not a preference:
//! every session-establishing function in `protocol_state` is `pub(crate)` or private, so an
//! integration test — an external crate — has no `pub` path to a stored session. The
//! subprocess writes the session into the config dir; the facade then reads it IN-PROCESS,
//! which is what makes this a facade assertion rather than a CLI one.

mod common;

use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, OnceLock};
use std::{env, fs};

use qsc::facade::{connect_status, invite_list_at, ConnectReason, ConnectState, InviteStateKind};

const ROUTE_TOKEN_PEER0: &str = "route_token_peer0_abcdefghijklmnop";
const PASS: &str = "correct horse battery staple";

fn guard() -> MutexGuard<'static, ()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
        .lock()
        .unwrap_or_else(|e| e.into_inner())
}

fn dir700(p: &Path) {
    fs::create_dir_all(p).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(p, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn fresh(tag: &str) -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let dir = root
        .join("qsc-test-tmp")
        .join(format!("na0751_invite_{tag}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir700(&dir);
    dir
}

/// ⚠ ONE call, BEFORE any server. Everything downstream inherits it.
fn set_env_once(cfg: &Path) {
    env::set_var("QSC_CONFIG_DIR", cfg);
    env::set_var("QSC_QSP_SEED", "1");
    env::set_var("QSC_ALLOW_SEED_FALLBACK", "1");
    env::set_var("QSC_UNSAFE_TEST_SEED_FALLBACK", "1");
    env::set_var("QSC_MARK_FORMAT", "plain");
}

#[test]
fn na0751_w1_the_seventh_reason_active_handshake() {
    let _g = guard();
    let cfg = fresh("active");
    set_env_once(&cfg);
    common::init_mock_vault(&cfg);

    // The tree's own route to a stored session (`qsp_status_truthy.rs:132-173`): a relay
    // fixture plus a `send` subprocess. Env is already set; the server starts after it.
    let server = common::start_inbox_server(1024 * 1024, 32);

    let add = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "contacts", "add", "--label", "peer-0", "--fp", "fp-pinned-test",
            "--route-token", ROUTE_TOKEN_PEER0,
        ])
        .output()
        .expect("contacts add");
    assert!(add.status.success(), "contacts add: {add:?}");

    let msg = cfg.join("msg.bin");
    fs::write(&msg, b"hello").unwrap();
    let send = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .env("QSC_QSP_SEED", "1")
        .env("QSC_ALLOW_SEED_FALLBACK", "1")
        .env("QSC_UNSAFE_TEST_SEED_FALLBACK", "1")
        .env("QSC_MARK_FORMAT", "plain")
        .args([
            "send", "--transport", "relay", "--relay", server.base_url(),
            "--to", "peer-0", "--file", msg.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(send.status.success(), "send must establish a session");

    // ⛳ THE FACADE READS IT IN-PROCESS. This is the seventh reason, and the union with
    // `na0751_facade_connect_reason_totality.rs`'s six is SEVEN.
    //
    // ⚠ THE IN-PROCESS SIDE MUST UNLOCK THE **REAL** MOCK VAULT, and the reason is measured:
    // `qsp_session_store_key_load` (`protocol_state:168-181`) returns the REAL stored key when
    // `vault::secret_get` succeeds and the TEST FALLBACK key only when it fails with
    // "vault_missing"/"vault_locked". The subprocess carried `--unlock-passphrase-env`, so it
    // wrote the blob under the REAL key. Setting the process flag alone — without loading the
    // passphrase — makes `secret_get` fail, takes the fallback branch, and the blob then fails
    // integrity (`session_integrity_failed`). Same config dir, two different keys.
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE)
        .expect("in-process unlock of the same mock vault the subprocess used");
    qsc::set_vault_unlocked(true);
    let s = connect_status("peer-0");
    assert_eq!(s.state, ConnectState::Active, "the only ACTIVE arm");
    assert_eq!(s.reason, ConnectReason::Handshake, "the seventh reason");
    assert_eq!(s.reason.as_wire(), "handshake");
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_invite_surface_is_driven_in_process_against_the_relay_fixture() {
    let _g = guard();
    let cfg = fresh("surface");
    set_env_once(&cfg);

    // A REAL vault, created and unlocked in-process — the capability `set_vault_unlocked`
    // (`lib.rs:192`) and `unlock_with_passphrase` (`vault/mod.rs:203`) provide. No shaping
    // precedent is claimed for this: `NA_0671` is cited for the capability only.
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);

    // ⛳ THE FIRST IN-PROCESS DRIVER OF THE INVITE SURFACE. No test in the tree called these
    // as Rust functions before this lane.
    let code = qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, None)
        .expect("invite_create through the facade");
    assert!(
        code.starts_with("QSLI-1-"),
        "the mint returns the full shareable code once: {code}"
    );

    // The list surface carries NO capability and NO endpoint.
    let listed = qsc::facade::invite_list().expect("invite_list");
    assert_eq!(listed.len(), 1, "one invite was minted");
    let row = &listed[0];
    assert!(!row.invite_id.is_empty());
    assert_eq!(row.state, InviteStateKind::Active);
    assert!(row.expiry > 0);
    // W7-adjacent — and NA-0755 v2 turns it from a DENYLIST into an ALLOWLIST (SR-15 A-3).
    //
    // ⚠ The old form banned two known-bad values by name (`cap:`, the relay URL). That admits
    // EVERY new sensitive field by construction, and the comment claiming the field set was
    // closed went false the moment the set grew. The allowlist below fails on ANY field the
    // boundary has not been ruled to carry, which is the property the doc at `InviteSummary`
    // actually states.
    let rendered = format!("{row:?}");
    assert!(!rendered.contains("cap:"), "no capability on the list DTO");
    assert!(
        !rendered.contains(relay.base_url()),
        "no relay endpoint on the list DTO"
    );
    let expected_fields = [
        "invite_id", "state", "expiry", "revocable", "label", "created",
    ];
    let seen: Vec<&str> = expected_fields
        .iter()
        .copied()
        .filter(|f| rendered.contains(&format!("{f}:")))
        .collect();
    assert_eq!(
        seen.len(),
        expected_fields.len(),
        "every ruled field must be present in the Debug rendering: {rendered}"
    );
    let field_count = rendered.matches(": ").count();
    assert_eq!(
        field_count,
        expected_fields.len(),
        "the Debug rendering carries EXACTLY the ruled field set and nothing else — a new \
         field must be ruled onto this boundary before it can ship: {rendered}"
    );
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_w11_the_expiry_overlay_and_its_boundary_second() {
    let _g = guard();
    let cfg = fresh("expiry");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, None).expect("mint");
    let expiry = qsc::facade::invite_list().expect("list")[0].expiry;

    // BEFORE expiry: Active.
    let before = invite_list_at(expiry - 1).expect("list_at before");
    assert_eq!(before[0].state, InviteStateKind::Active, "alive before expiry");

    // ⛳ THE BOUNDARY SECOND, drivable ONLY because of the `_at` seam. The overlay is the exact
    // complement of the soft cap's own live-door test at `invite/mod.rs:829`
    // (`state == Active && expiry > now`): here `Active && expiry <= now` becomes Expired.
    let at = invite_list_at(expiry).expect("list_at boundary");
    assert_eq!(
        at[0].state,
        InviteStateKind::Expired,
        "expiry == now is EXPIRED, the exact complement of `expiry > now`"
    );

    let after = invite_list_at(expiry + 1).expect("list_at after");
    assert_eq!(after[0].state, InviteStateKind::Expired, "dead after expiry");
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0751_invite_revoke_commits_locally_and_the_list_is_how_a_screen_reads_it() {
    let _g = guard();
    let cfg = fresh("revoke");
    set_env_once(&cfg);
    qsc::vault::vault_init_with_passphrase(PASS).expect("vault init");
    qsc::vault::unlock_with_passphrase(PASS).expect("unlock");
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").expect("identity");

    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    qsc::facade::invite_create(Some("self"), relay.base_url(), 3600, None).expect("mint");

    let before = qsc::facade::invite_list().expect("list");
    assert_eq!(before.len(), 1);
    let id = before[0].invite_id.clone();
    // `revocable` comes off the residual list here: the relay returns the one-shot revoke token
    // at create, and the surface exposes only its PRESENCE.
    assert!(before[0].revocable, "a freshly minted invite carries a revoke token");
    assert_eq!(before[0].state, InviteStateKind::Active);

    // ⛳ THE VERB. `Result<(), FacadeError>` — no structured outcome, because `invite_revoke`
    // returns ONE FLAT VALUE and two of its codes are minted byte-identically on both sides of
    // its internal commit boundary (`invite/mod.rs:919-921`). See `ENG-0215`.
    qsc::facade::invite_revoke(&id).expect("revoke against the fixture");

    // ⛳ AND THIS IS THE COMPOSITION PATTERN `E10` IS SATISFIED BY, EXECUTED ONCE: a screen that
    // needs to tell "revoked here, relay pending" from "nothing happened" calls the list and
    // reads the state. Here the relay call succeeded too, so the state is simply Revoked.
    let after = qsc::facade::invite_list().expect("list after revoke");
    assert_eq!(after.len(), 1, "revoke does not delete the record");
    assert_eq!(
        after[0].state,
        InviteStateKind::Revoked,
        "the local commit is visible through the list — the entry point of the composition pattern"
    );
    assert_eq!(after[0].invite_id, id, "same invite, new state");

    // MUTATION-ADJACENT CONTROL: the arms differ. Before the call the state was Active; a test
    // that asserted Revoked without having read Active first would not discriminate.
    assert_ne!(before[0].state, after[0].state, "the revoke changed the observed state");
    qsc::set_vault_unlocked(false);
}

#[test]
fn na0780_self_invitation_is_rejected_at_both_redemption_boundaries() {
    let _g = guard();
    let cfg = fresh("self_reject");
    set_env_once(&cfg);
    common::init_mock_vault(&cfg);
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").unwrap();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let direct = qsc::facade::invite_create(None, relay.base_url(), 3600, None).unwrap();
    let facade = qsc::facade::invite_create(None, relay.base_url(), 3600, None).unwrap();
    let direct_result = qsc::invite::invite_redeem(&direct, "myself-direct", None);
    let facade_result = qsc::facade::invite_redeem(&facade, "myself-facade", None);
    assert_eq!(
        direct_result.err(),
        Some("invite_self"),
        "direct handler must reject ownership"
    );
    assert_eq!(
        facade_result.err().map(|e| e.as_wire()),
        Some("self_invitation"),
        "facade must reject ownership"
    );
    qsc::set_vault_unlocked(false);
}

// Compare without Debug-printing vault or identity bytes on failure.
fn snapshot(dir: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, dir: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                visit(root, &path, out);
            } else {
                out.push((
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(path).unwrap(),
                ));
            }
        }
    }
    let mut out = Vec::new();
    visit(dir, dir, &mut out);
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out
}

fn assert_self_without_effects(cfg: &Path, code: &str) {
    let before = snapshot(cfg);
    for _ in 0..3 {
        assert_eq!(
            qsc::facade::invite_preflight(code, None)
                .err()
                .map(|e| e.as_wire()),
            Some("self_invitation")
        );
        assert_eq!(
            qsc::invite::invite_redeem(code, "self-direct", None).err(),
            Some("invite_self")
        );
        assert_eq!(
            qsc::facade::invite_redeem(code, "self-facade", None)
                .err()
                .map(|e| e.as_wire()),
            Some("self_invitation")
        );
    }
    assert!(
        snapshot(cfg) == before,
        "rejection must preserve every file, including contacts and redemption state"
    );
}

#[test]
fn na0780_history_restart_clear_rotation_and_unavailable_storage() {
    let _g = guard();
    let cfg = fresh("ownership_history");
    set_env_once(&cfg);
    common::init_mock_vault(&cfg);
    qsc::vault::unlock_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    qsc::set_vault_unlocked(true);
    qsc::identity::identity_ensure("self").unwrap();
    let relay = common::start_qsl_server(2 * 1024 * 1024, 512, None);
    let codes: Vec<_> = (0..3)
        .map(|_| qsc::facade::invite_create(None, relay.base_url(), 3600, None).unwrap())
        .collect();
    for code in &codes {
        assert_self_without_effects(&cfg, code);
    }

    // A fresh process must consult persisted ownership, not an in-memory latest code.
    let before = snapshot(&cfg);
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .args([
            "invite",
            "redeem",
            "--code",
            &codes[0],
            "--alias",
            "restart-self",
        ])
        .output()
        .unwrap();
    assert!(
        !out.status.success(),
        "fresh process must reject self invitation"
    );
    let output = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        output.contains("invite_self"),
        "restart must fail for ownership"
    );
    assert!(snapshot(&cfg) == before, "restart rejection must not write");

    // A live listener records any TCP attempt, independently of relay response/errors.
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let mut payload = qsc::invite::decode_invite_code(&codes[0]).unwrap();
    payload.relay_ep = format!("http://{}", listener.local_addr().unwrap());
    let probe = qsc::invite::encode_invite_code(&payload).unwrap();
    assert_self_without_effects(&cfg, &probe);
    assert!(
        matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock),
        "self rejection opened a connection"
    );

    let original = qsc::vault::secret_get("invite.created").unwrap().unwrap();
    let mut old: serde_json::Value = serde_json::from_str(&original).unwrap();
    for row in old["invites"].as_object_mut().unwrap().values_mut() {
        let row = row.as_object_mut().unwrap();
        for key in ["revoke_token", "created_unix", "label"] {
            row.remove(key);
        }
        row.insert("state".into(), "Creating".into());
    }
    qsc::vault::secret_set("invite.created", &old.to_string()).unwrap();
    for code in &codes {
        assert_self_without_effects(&cfg, code);
    }

    // Exercise the actual clear verb, not just a fabricated empty UI list.
    for row in qsc::invite::invite_list().unwrap() {
        qsc::invite::invite_clear(&row.invite_id).unwrap();
    }
    for code in &codes {
        assert_self_without_effects(&cfg, code);
    }
    assert_self_without_effects(&cfg, &probe);
    assert!(matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));

    // Lock between preflight and redeem: handler must recheck lock and preserve storage.
    qsc::set_vault_unlocked(false);
    let before = snapshot(&cfg);
    assert_eq!(
        qsc::facade::invite_preflight(&probe, None)
            .err()
            .map(|e| e.as_wire()),
        Some("locked")
    );
    assert_eq!(
        qsc::invite::invite_redeem(&probe, "locked", None).err(),
        Some("vault_locked")
    );
    assert!(snapshot(&cfg) == before);
    qsc::set_vault_unlocked(true);

    // Corrupt history refuses even when the current public commitment would match.
    qsc::vault::secret_set("invite.created", "not-json").unwrap();
    let before = snapshot(&cfg);
    assert!(qsc::facade::invite_preflight(&probe, None).is_err());
    assert_eq!(
        qsc::invite::invite_redeem(&probe, "corrupt", None).err(),
        Some("invite_malformed")
    );
    assert!(snapshot(&cfg) == before);
    qsc::vault::secret_set("invite.created", "{\"invites\":{}}").unwrap();

    let public_path = cfg.join("identities/self_self.json");
    let public = fs::read(&public_path).unwrap();
    let mut legacy: serde_json::Value = serde_json::from_slice(&public).unwrap();
    legacy.as_object_mut().unwrap().remove("sig_pk");
    for bytes in [
        legacy.to_string().into_bytes(),
        b"not-json".to_vec(),
        b"{\"kem_pk\":[],\"sig_pk\":[]}".to_vec(),
    ] {
        fs::write(&public_path, bytes).unwrap();
        let before = snapshot(&cfg);
        assert_eq!(
            qsc::facade::invite_preflight(&probe, None)
                .err()
                .map(|e| e.as_wire()),
            Some("store_unavailable")
        );
        assert_eq!(
            qsc::invite::invite_redeem(&probe, "legacy", None).err(),
            Some("invite_ownership_unavailable")
        );
        assert!(
            snapshot(&cfg) == before,
            "no key upgrade or migration during ownership reads"
        );
    }
    // Retained mint IDs remain sufficient even if old public records lack signing keys.
    qsc::vault::secret_set("invite.created", &original).unwrap();
    assert_self_without_effects(&cfg, &probe);
    fs::write(&public_path, &public).unwrap();

    let rotate = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", &cfg)
        .args(["identity", "rotate", "--confirm"])
        .output()
        .unwrap();
    assert!(rotate.status.success(), "fixture identity rotation");
    assert_self_without_effects(&cfg, &probe);
    qsc::vault::secret_set("invite.created", "{\"invites\":{}}").unwrap();
    let before = snapshot(&cfg);
    assert!(
        qsc::facade::invite_preflight(&probe, None).is_ok(),
        "documented limit: rotated identity and erased history cannot recognize the old code"
    );
    assert!(snapshot(&cfg) == before);
    fs::remove_file(&public_path).unwrap();
    let before = snapshot(&cfg);
    assert!(qsc::facade::invite_preflight(&probe, None).is_ok());
    assert!(
        snapshot(&cfg) == before,
        "missing identity must not be generated"
    );
    assert!(qsc::facade::invite_preflight("bad-code", None).is_err());
    assert!(matches!(listener.accept(), Err(e) if e.kind() == std::io::ErrorKind::WouldBlock));
    qsc::set_vault_unlocked(false);
}
