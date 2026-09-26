// NA-0785 PLAN F03 / S5 -- INSTRUMENTS (b): NEGATIVE PROFILE REFUSAL, one arm per consumer of the
// profile identifier (C01 A02). Every arm starts from GENUINE successor state built by the F03
// helpers, changes only the profile, and carries a positive control proving that the refusal is
// caused by the foreign profile alone. Codes are the ones this head emits, as measured.
//
//   b1 old-profile PEER   handshake_reject reason=REJECT_QSC_HS_INTEGRATION_PROFILE
//                         delta handshake/mod.rs:336-337 (the 0x7f80 value arm), before any pending
//                         write. RED at main 6c601568: REJECT_QSC_HS_UNKNOWN_CRITICAL (no 0x7f80 arm).
//   b2 old-profile VAULT  code=vault_version_unsupported, after authentication, nothing written
//                         delta vault/mod.rs:1980-1981 (check_directional_aggregate's profile arm).
//                         RED at main: vault_parse_failed (QSCV03 is an unknown magic there).
//                         NOT a distinct code: the same code refuses a wrong payload version.
//   b3 foreign Transaction code=TRANSACTION_PROFILE
//                         delta directional_delivery.rs:470-471, reached from the vault aggregate
//                         check (vault/mod.rs:1992) for every owned peer.
//   b4 foreign QueuedIntent  no test here: APPLICATION_ID_CONFLICT today (directional_delivery.rs:
//                         329, pinned by that module's own unit test); the distinct code is C01 O9,
//                         arm e2 -- OPEN.
//   b5 foreign receipt    a genuine receipt re-sealed with the retired profile in its AD
//                         (directional_delivery.rs:60-63) is NOT admitted: skipped, the message stays
//                         SENT, the flight stays outstanding. Its code (RECEIPT_AUTH, :103-105) is an
//                         expected non-admission (protocol_state/mod.rs:1286), so the CLI never
//                         names it; the disposition is what this arm observes.
// A synthetic local run; it says nothing about production or the real relay deployment.

mod common;

use chacha20poly1305::aead::{Aead, KeyInit, Payload};
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use common::{profile, PairRelay, VaultFixture, VaultKind};
use std::fs;
use std::path::Path;

const PASS: &str = "f03-profile-refusal-pass";
/// The RETIRED development profile the negative arms carry as their FOREIGN input. It is the
/// input under test, never a pin of the current profile (every such pin reads ACTIVE).
const RETIRED_PROFILE: &str = "NA0780-DIR-INTEGRATION-02";
/// directional_core::PROFILE, the epoch-core profile the receipt AD carries after the
/// integration profile. Not a successor profile id.
const CORE_PROFILE: &[u8] = b"NA0780-DIR-EPOCH-CORE-01";
/// b2's pinned old-profile vault: a QSCV03 envelope whose authenticated payload is version 4
/// with the retired profile and no secrets (lane evidence: its generator and sha).
const B2_FIXTURE_SHA256: &str = "e65c966a99f61fd7e34e1bfa3831e103fb5b890e2197794457d4ca99db479204";
const B2_FIXTURE: &[u8] =
    include_bytes!("fixtures/e65c966a99f61fd7e34e1bfa3831e103fb5b890e2197794457d4ca99db479204.qsv");
const B2_PASS: &str = "f03-b2-old-profile-vault-pass";
const HEADER_LEN: usize = 53;

fn text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn private_dir(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

/// `vault unlock` with the fixture's own passphrase (from a file outside the config dir).
fn unlock(fixture: &VaultFixture) -> std::process::Output {
    let input = fixture.iso.root.join("unlock-input");
    let file = common::write_passphrase_file(&input, "unlock", &fixture.passphrase);
    fixture.run(&[
        "vault",
        "unlock",
        "--non-interactive",
        "--passphrase-file",
        file.to_str().unwrap(),
    ])
}

/// Every regular file in `dir` with its bytes.
fn listing(dir: &Path) -> Vec<(String, Vec<u8>)> {
    let mut entries: Vec<(String, Vec<u8>)> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap())
        .filter(|e| e.file_type().unwrap().is_file())
        .map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            (name, fs::read(e.path()).unwrap())
        })
        .collect();
    entries.sort();
    entries
}

/// The authenticated vault, opened first-party with the fixture passphrase (header = AAD).
struct OpenVault {
    header: Vec<u8>,
    key: [u8; 32],
    payload: serde_json::Value,
}

fn open_vault(cfg: &Path, passphrase: &str) -> OpenVault {
    use argon2::{Algorithm, Argon2, Params, Version};
    let bytes = fs::read(cfg.join("vault.qsv")).unwrap();
    let (header, ciphertext) = bytes.split_at(HEADER_LEN);
    let word = |at: usize| u32::from_le_bytes(header[at..at + 4].try_into().unwrap());
    let mut key = [0u8; 32];
    Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(word(9), word(13), word(17), Some(32)).unwrap(),
    )
    .hash_password_into(passphrase.as_bytes(), &header[25..41], &mut key)
    .unwrap();
    let clear = ChaCha20Poly1305::new(Key::from_slice(&key))
        .decrypt(
            Nonce::from_slice(&header[41..53]),
            Payload {
                msg: ciphertext,
                aad: header,
            },
        )
        .expect("vault authenticates under the fixture passphrase");
    OpenVault {
        header: header.to_vec(),
        key,
        payload: serde_json::from_slice(&clear).unwrap(),
    }
}

/// Re-seal the payload as the product does: the header (fresh nonce, exact ct_len) is the AAD.
fn reseal_vault(cfg: &Path, vault: &OpenVault) {
    use rand_core::{OsRng, RngCore};
    let plaintext = serde_json::to_vec(&vault.payload).unwrap();
    let mut header = vault.header.clone();
    header[21..25].copy_from_slice(&((plaintext.len() + 16) as u32).to_le_bytes());
    OsRng.fill_bytes(&mut header[41..53]);
    let sealed = ChaCha20Poly1305::new(Key::from_slice(&vault.key))
        .encrypt(
            Nonce::from_slice(&header[41..53]),
            Payload {
                msg: &plaintext,
                aad: &header,
            },
        )
        .unwrap();
    header.extend(sealed);
    fs::write(cfg.join("vault.qsv"), header).unwrap();
}

/// The one directional transaction record in the vault: (secret name, parsed record).
fn transaction(vault: &OpenVault) -> (String, serde_json::Value) {
    let found: Vec<(String, serde_json::Value)> = vault.payload["secrets"]
        .as_object()
        .unwrap()
        .iter()
        .filter_map(|(name, raw)| {
            let value: serde_json::Value = serde_json::from_str(raw.as_str()?).ok()?;
            let current = value.get("version")?.as_str()? == profile::ACTIVE.id;
            current.then(|| (name.clone(), value))
        })
        .collect();
    assert_eq!(found.len(), 1, "exactly one directional transaction record");
    found.into_iter().next().unwrap()
}

/// Two successor vaults with identities, inbox routes, pinned contacts and trusted devices:
/// everything init_real_pair does BEFORE its handshake.
fn pinned_peers(tag: &str, a_route: &str, b_route: &str) -> (VaultFixture, VaultFixture) {
    let a = common::init_successor_vault(&format!("{tag}_a"), PASS);
    let b = common::init_successor_vault(&format!("{tag}_b"), PASS);
    for (fixture, label, route) in [(&a, "alice", a_route), (&b, "bob", b_route)] {
        fixture.run_ok(&["identity", "rotate", "--as", label, "--confirm"]);
        fixture.run_ok(&["relay", "inbox-set", "--token", route]);
    }
    let field = |shown: &str, name: &str| {
        let value = shown.lines().find_map(|l| l.strip_prefix(name)).unwrap();
        common::scraped_marker_value(name, value)
    };
    let a_pub = a.run_ok(&["identity", "show", "--as", "alice"]);
    let b_pub = b.run_ok(&["identity", "show", "--as", "bob"]);
    for (fixture, label, route, shown) in
        [(&a, "bob", b_route, &b_pub), (&b, "alice", a_route, &a_pub)]
    {
        fixture.run_ok(&[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            &field(shown, "identity_fp="),
            "--kem-pk",
            &field(shown, "identity_kem_pk="),
            "--sig-pk",
            &field(shown, "identity_sig_pk="),
            "--route-token",
            route,
        ]);
        let devices = fixture.run_ok(&["contacts", "device", "list", "--label", label]);
        let device = devices
            .lines()
            .find_map(|l| l.strip_prefix("device="))
            .and_then(|l| l.split_whitespace().next())
            .unwrap();
        let device = common::scraped_marker_value("device", device);
        fixture.run_ok(&[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            &device,
            "--confirm",
        ]);
    }
    (a, b)
}

fn raw_push(relay: &str, route: &str, body: &[u8]) {
    let status = reqwest::blocking::Client::new()
        .post(format!("{relay}/v1/push"))
        .header("X-QSL-Route-Token", route)
        .body(body.to_vec())
        .send()
        .unwrap()
        .status();
    assert!(status.is_success(), "raw push: {status}");
}

/// A lease-mode pull WITHOUT an ACK: what the relay still offers on `route`.
fn raw_lease_pull(relay: &str, route: &str) -> Vec<Vec<u8>> {
    let resp = reqwest::blocking::Client::new()
        .get(format!("{relay}/v1/pull?max=16&ack=lease"))
        .header("X-QSL-Route-Token", route)
        .send()
        .unwrap();
    if resp.status().as_u16() == 204 {
        return Vec::new();
    }
    let body: serde_json::Value = resp.json().unwrap();
    body["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|item| serde_json::from_value(item["data"].clone()).unwrap())
        .collect()
}

fn replace_once(haystack: &[u8], from: &[u8], to: &[u8]) -> Vec<u8> {
    assert_eq!(from.len(), to.len(), "same-length rewrite");
    let at: Vec<usize> = haystack
        .windows(from.len())
        .enumerate()
        .filter(|(_, w)| *w == from)
        .map(|(i, _)| i)
        .collect();
    assert_eq!(at.len(), 1, "the profile occurs exactly once");
    let mut out = haystack.to_vec();
    out[at[0]..at[0] + to.len()].copy_from_slice(to);
    out
}

/// b1. A genuine A1 whose 0x7f80 value names the retired profile is refused with its own code,
/// before any pending handshake state is written, and is not ACKed; the same responder admits
/// the genuine A1.
#[test]
fn b1_old_profile_peer_a1_is_refused() {
    let (a_route, b_route) = (
        "f03_b1_route_alice_012345678",
        "f03_b1_route_bob_0123456789a",
    );
    let (alice, bob) = pinned_peers("f03_b1", a_route, b_route);
    let capture = common::start_inbox_server(1024 * 1024, 16);
    alice.run_ok(&[
        "handshake",
        "init",
        "--as",
        "alice",
        "--peer",
        "bob",
        "--relay",
        capture.base_url(),
        "--suite-mode",
        "suite-required",
    ]);
    let frames = capture.drain_channel(b_route);
    assert_eq!(frames.len(), 1, "one genuine A1");
    let genuine = frames.into_iter().next().unwrap();
    assert!(genuine.starts_with(b"QHSM"), "a handshake frame");
    let foreign = replace_once(
        &genuine,
        profile::ACTIVE.id.as_bytes(),
        RETIRED_PROFILE.as_bytes(),
    );
    let poll = |relay: &common::QslRelayTestServer| {
        bob.run(&[
            "handshake",
            "poll",
            "--as",
            "bob",
            "--peer",
            "alice",
            "--relay",
            relay.base_url(),
            "--max",
            "4",
            "--suite-mode",
            "suite-required",
        ])
    };

    let leasing = common::start_qsl_server_with_store(1024 * 1024, 16, None, 2);
    raw_push(leasing.base_url(), b_route, &foreign);
    let before = listing(&bob.cfg);
    let refused = text(&poll(&leasing));
    assert!(
        refused.contains("event=handshake_reject reason=REJECT_QSC_HS_INTEGRATION_PROFILE"),
        "b1: an old-profile A1 must be refused REJECT_QSC_HS_INTEGRATION_PROFILE: {refused}"
    );
    assert!(
        !refused.contains("event=handshake_send"),
        "b1: no B1 answered"
    );
    assert_eq!(
        listing(&bob.cfg),
        before,
        "b1: no pending handshake state written"
    );
    std::thread::sleep(std::time::Duration::from_secs(3)); // > the 2 s lease
    assert!(
        raw_lease_pull(leasing.base_url(), b_route).contains(&foreign),
        "b1: the refused item is not ACKed"
    );

    let fresh = common::start_qsl_server_with_store(1024 * 1024, 16, None, 2);
    raw_push(fresh.base_url(), b_route, &genuine);
    let admitted = poll(&fresh);
    let admitted_text = text(&admitted);
    assert!(
        admitted.status.success() && admitted_text.contains("event=handshake_send msg=B1"),
        "b1 control: the genuine A1 is admitted: {admitted_text}"
    );
    assert_ne!(
        listing(&bob.cfg),
        before,
        "b1 control: admission writes state"
    );
}

/// b2. The pinned old-profile vault authenticates under its passphrase and is then refused;
/// nothing is written and no failure counter appears. The wrong passphrase is refused as a
/// locked vault instead, which proves the refusal above is about the payload's profile.
#[test]
fn b2_old_profile_vault_is_refused() {
    {
        use sha2::{Digest, Sha256};
        let digest: String = Sha256::digest(B2_FIXTURE)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        assert_eq!(digest, B2_FIXTURE_SHA256, "the pinned fixture bytes");
    }
    let iso = common::fixture_isolation("f03_b2");
    let cfg = iso.root.join("cfg");
    private_dir(&cfg);
    fs::write(cfg.join("vault.qsv"), B2_FIXTURE).unwrap();
    let old = VaultFixture {
        cfg,
        iso,
        passphrase: B2_PASS.to_owned(),
        kind: VaultKind::Successor,
    };
    assert_eq!(
        common::vault_payload_identity(&old.cfg, B2_PASS),
        (4, RETIRED_PROFILE.to_owned()),
        "b2: the fixture is an authenticated payload-4 vault of the retired profile"
    );
    let before = listing(&old.cfg);
    let refused = unlock(&old);
    let refused_text = text(&refused);
    assert!(
        !refused.status.success() && refused_text.contains("code=vault_version_unsupported"),
        "b2: the old-profile vault must be refused vault_version_unsupported: {refused_text}"
    );
    assert!(
        !refused_text.contains("code=vault_locked") && !refused_text.contains("vault_parse_failed")
    );
    assert_eq!(
        listing(&old.cfg),
        before,
        "b2: nothing written, no failure counter"
    );

    let wrong = VaultFixture {
        cfg: old.cfg.clone(),
        iso: old.iso.clone(),
        passphrase: "f03-b2-not-the-passphrase".to_owned(),
        kind: VaultKind::Successor,
    };
    let locked = text(&unlock(&wrong));
    assert!(
        locked.contains("code=vault_locked"),
        "b2 control: a wrong passphrase is refused before the payload is read: {locked}"
    );
    assert_eq!(listing(&old.cfg), before, "b2 control: nothing written");
}

/// b3. A genuine transaction record whose version names the retired profile is refused at
/// unlock with its own code; the same vault re-sealed unchanged is admitted.
#[test]
fn b3_foreign_profile_transaction_is_refused() {
    let relay = common::start_inbox_server(1024 * 1024, 16);
    let pair = common::init_real_pair(
        "f03_b3",
        PairRelay::Mock(&relay),
        ("alice", "f03_b3_route_alice_012345678"),
        ("bob", "f03_b3_route_bob_0123456789a"),
    );
    let alice = &pair.a;
    let mut vault = open_vault(&alice.cfg, &alice.passphrase);
    reseal_vault(&alice.cfg, &vault);
    let control = unlock(alice);
    assert!(
        control.status.success(),
        "b3 control: a re-seal that changes only the nonce is admitted: {}",
        text(&control)
    );

    let (name, _) = transaction(&vault);
    let raw = vault.payload["secrets"][&name].as_str().unwrap().to_owned();
    let current = format!("\"version\":\"{}\"", profile::ACTIVE.id);
    let foreign = format!("\"version\":\"{RETIRED_PROFILE}\"");
    assert_eq!(raw.matches(&current).count(), 1, "one version field");
    vault.payload["secrets"][&name] =
        serde_json::Value::String(raw.replacen(&current, &foreign, 1));
    reseal_vault(&alice.cfg, &vault);
    let before = listing(&alice.cfg);
    let refused = unlock(alice);
    let refused_text = text(&refused);
    assert!(
        !refused.status.success() && refused_text.contains("code=TRANSACTION_PROFILE"),
        "b3: a foreign-profile transaction must be refused TRANSACTION_PROFILE: {refused_text}"
    );
    assert_eq!(listing(&alice.cfg), before, "b3: nothing written");
}

fn lp(x: &[u8]) -> Vec<u8> {
    let mut out = (x.len() as u32).to_be_bytes().to_vec();
    out.extend(x);
    out
}

fn bytes_of(value: &serde_json::Value) -> Vec<u8> {
    serde_json::from_value(value.clone()).unwrap()
}

/// A receipt for `wire` at `slot` under the SENDER's stored receipt context, with the AD's
/// integration-profile field set to `integration_profile` (directional_delivery.rs:49-63, 80-87).
fn receipt_under(
    context: &serde_json::Value,
    slot: u32,
    wire: &[u8],
    integration_profile: &[u8],
) -> Vec<u8> {
    use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
    use quantumshield_refimpl::crypto::traits::{Aead as RefAead, Hash as RefHash};
    let mut prefix = b"NDR1".to_vec();
    prefix.extend(bytes_of(&context["sid"]));
    prefix.push(context["direction"].as_u64().unwrap() as u8);
    prefix.extend(context["epoch"].as_u64().unwrap().to_be_bytes());
    prefix.extend(bytes_of(&context["dh"]));
    prefix.extend(slot.to_be_bytes());
    let mut ad = lp(integration_profile);
    ad.extend(lp(CORE_PROFILE));
    ad.extend(&prefix);
    let mut nonce = [0u8; 12];
    nonce[8..].copy_from_slice(&slot.to_be_bytes());
    let key: [u8; 32] = bytes_of(&context["key"]).try_into().unwrap();
    let digest = RefHash::sha512(&StdCrypto, wire);
    prefix.extend(RefAead::seal(&StdCrypto, &key, &nonce, &ad, &digest[..32]));
    prefix
}

/// b5. The genuine receipt for a message is rebuilt byte-for-byte from the sender's stored
/// context (the construction control); the same receipt sealed with the retired profile in its
/// AD is not admitted -- skipped, the message stays SENT and its flight outstanding -- while the
/// genuine receipt then delivers it.
#[test]
fn b5_foreign_profile_receipt_is_not_admitted() {
    let relay = common::start_inbox_server(1024 * 1024, 32);
    let (a_route, b_route) = (
        "f03_b5_route_alice_012345678",
        "f03_b5_route_bob_0123456789a",
    );
    let pair = common::init_real_pair(
        "f03_b5",
        PairRelay::Mock(&relay),
        ("alice", a_route),
        ("bob", b_route),
    );
    let (alice, bob) = (&pair.a, &pair.b);
    let body = alice.iso.root.join("b5.bin");
    fs::write(&body, b"f03 b5 message").unwrap();
    alice.run_ok(&[
        "send",
        "--transport",
        "relay",
        "--relay",
        relay.base_url(),
        "--to",
        "bob",
        "--file",
        body.to_str().unwrap(),
    ]);
    let receive = |fixture: &VaultFixture, route: &str, from: &str, tag: &str| -> String {
        let out_dir = fixture.iso.root.join(tag);
        private_dir(&out_dir);
        fixture.run_ok(&[
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay.base_url(),
            "--mailbox",
            route,
            "--from",
            from,
            "--max",
            "8",
            "--out",
            out_dir.to_str().unwrap(),
        ])
    };
    let (_, tx) = transaction(&open_vault(&alice.cfg, &alice.passphrase));
    let (flight_key, flight) = tx["flights"]
        .as_object()
        .unwrap()
        .iter()
        .find(|(_, f)| !f["id"].as_str().unwrap().is_empty())
        .map(|(k, f)| (k.clone(), f.clone()))
        .expect("the message's outstanding flight");
    let (epoch, slot) = flight_key.split_once(':').unwrap();
    let slot: u32 = slot.parse().unwrap();
    let context = tx["send"][epoch]["context"].clone();
    let wire = bytes_of(&flight["wire"]);

    receive(bob, b_route, "alice", "bob_in");
    let to_alice = relay.drain_channel(a_route);
    let genuine = receipt_under(&context, slot, &wire, profile::ACTIVE.id.as_bytes());
    assert!(
        to_alice.contains(&genuine),
        "b5 control: the rebuilt receipt equals the peer's genuine receipt byte for byte"
    );

    let foreign = receipt_under(&context, slot, &wire, RETIRED_PROFILE.as_bytes());
    assert_eq!(foreign.len(), genuine.len());
    assert_eq!(
        &foreign[..65],
        &genuine[..65],
        "same prefix: only the AD's profile differs"
    );
    relay.enqueue_raw(a_route, foreign);
    let refused = receive(alice, a_route, "bob", "alice_in_foreign");
    assert!(
        refused.contains("event=recv_skip_summary count=1") && !refused.contains("to=DELIVERED"),
        "b5: a receipt sealed under the retired profile is not admitted: {refused}"
    );
    let (_, after) = transaction(&open_vault(&alice.cfg, &alice.passphrase));
    assert!(
        after["flights"].get(&flight_key).is_some(),
        "b5: the flight stays outstanding"
    );

    for frame in to_alice {
        relay.enqueue_raw(a_route, frame);
    }
    let delivered = receive(alice, a_route, "bob", "alice_in_genuine");
    assert!(
        delivered.contains("from=SENT to=DELIVERED"),
        "b5 control: the genuine receipt delivers the message: {delivered}"
    );
}
