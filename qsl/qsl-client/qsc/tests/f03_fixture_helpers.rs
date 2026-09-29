// NA-0785 PLAN F03 / S4 -- the fixture helpers' own tests (FIXTURE_DESIGN secs 1-2).
//
// Each helper's MEANING is asserted once, here, not in every caller:
//   A-i   init_ordinary_vault: `handshake init` from it REFUSES with the ACTIVE ordinary code,
//         even with a genuine, trusted contact and a live relay.
//   A-ii  init_successor_vault: carries profile::ACTIVE; a pre-populated cfg is refused
//         directional_fresh_vault_required by the fixture's own precondition, before the
//         product runs, so the directory is left untouched.
//   A-iii init_real_pair: both sides handshake_complete over the leasing relay (and the
//         mock), and the session carries a message.
// Mutation controls (SR-06) are run by the seat against temporary copies of the helpers;
// they are recorded in the lane evidence, never committed.
// The caller census fails if new callers of the two declared legacy-meaning helpers appear.

mod common;

use common::profile;
use common::{PairRelay, VaultKind};
use std::fs;
use std::path::Path;

const PASS: &str = "f03-fixture-helpers-pass";

fn text(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn listing(dir: &Path) -> Vec<(String, u64)> {
    let mut entries: Vec<(String, u64)> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            let len = entry.metadata().unwrap().len();
            (entry.file_name().to_string_lossy().into_owned(), len)
        })
        .collect();
    entries.sort();
    entries
}

fn public_identity(fixture: &common::VaultFixture, label: &str) -> [String; 3] {
    let shown = fixture.run_ok(&["identity", "show", "--as", label]);
    ["identity_fp=", "identity_kem_pk=", "identity_sig_pk="].map(|field| {
        let value = shown
            .lines()
            .find_map(|line| line.strip_prefix(field))
            .expect("public identity field");
        common::scraped_marker_value(field, value)
    })
}

/// A-i. An ordinary vault can never establish a directional peer: with its own identity, a
/// genuine peer identity pinned as a contact, a trusted device and a live leasing relay,
/// `handshake init` refuses with the ACTIVE ordinary code and pushes nothing.
#[test]
fn ordinary_vault_handshake_init_refuses() {
    let ordinary = common::init_ordinary_vault("f03_ai_ordinary", PASS);
    assert_eq!(ordinary.kind, VaultKind::Ordinary);
    let peer = common::init_successor_vault("f03_ai_peer", PASS);
    peer.run_ok(&["identity", "rotate", "--as", "bob", "--confirm"]);
    let [fp, kem, sig] = public_identity(&peer, "bob");
    ordinary.run_ok(&["identity", "rotate", "--as", "alice", "--confirm"]);
    ordinary.run_ok(&[
        "relay",
        "inbox-set",
        "--token",
        "f03_ai_route_alice_0123456789",
    ]);
    ordinary.run_ok(&[
        "contacts",
        "add",
        "--label",
        "bob",
        "--fp",
        &fp,
        "--kem-pk",
        &kem,
        "--sig-pk",
        &sig,
        "--route-token",
        "f03_ai_route_bob_0123456789ab",
    ]);
    let devices = ordinary.run_ok(&["contacts", "device", "list", "--label", "bob"]);
    let device = devices
        .lines()
        .find_map(|line| line.strip_prefix("device="))
        .and_then(|line| line.split_whitespace().next())
        .expect("contact device");
    let device = common::scraped_marker_value("device", device);
    ordinary.run_ok(&[
        "contacts",
        "device",
        "trust",
        "--label",
        "bob",
        "--device",
        &device,
        "--confirm",
    ]);
    let relay = common::start_qsl_server(1024 * 1024, 16, None);
    let out = ordinary.run(&[
        "handshake",
        "init",
        "--as",
        "alice",
        "--peer",
        "bob",
        "--relay",
        relay.base_url(),
        "--suite-mode",
        "suite-required",
    ]);
    let output = text(&out);
    let refusal = format!("code={}", profile::ACTIVE.ordinary_handshake_refusal);
    assert!(
        !out.status.success() && output.contains(&refusal),
        "A-i: handshake init from an ordinary vault must refuse with {refusal}: {output}"
    );
}

/// A-ii. The successor vault carries profile::ACTIVE, lives under the fixture root (never
/// under the working directory), and a pre-populated cfg is refused
/// directional_fresh_vault_required by the fixture BEFORE the product runs.
#[test]
fn successor_vault_is_active_profile_and_refuses_prepopulated_cfg() {
    let fresh = common::init_successor_vault("f03_aii_fresh", PASS);
    assert_eq!(fresh.kind, VaultKind::Successor);
    let (_, profile_id) = common::vault_payload_identity(&fresh.cfg, PASS);
    assert_eq!(
        profile_id,
        profile::ACTIVE.id,
        "A-ii: successor vault profile"
    );
    match std::env::var_os("QSC_TEST_ROOT") {
        Some(root) => assert!(
            fresh.cfg.starts_with(root),
            "E-7: fixture under QSC_TEST_ROOT"
        ),
        None => assert!(
            fresh.cfg.starts_with(std::env::temp_dir()),
            "E-7: fixture in temp"
        ),
    }
    assert!(
        !fresh.cfg.starts_with(std::env::current_dir().unwrap())
            || std::env::var_os("QSC_TEST_ROOT").is_some(),
        "E-7: fixture state never under the working directory"
    );

    let iso = common::fixture_isolation("f03_aii_prepopulated");
    let cfg = iso.root.join("cfg");
    fs::create_dir_all(&cfg).unwrap();
    fs::write(cfg.join("stray"), b"x").unwrap();
    let before = listing(&cfg);
    let refused = common::try_init_successor_vault_at(iso, cfg.clone(), PASS)
        .err()
        .expect("A-ii: a pre-populated cfg must be refused");
    assert!(
        refused.contains("directional_fresh_vault_required"),
        "A-ii: refusal code: {refused}"
    );
    assert_eq!(
        listing(&cfg),
        before,
        "A-ii: the fixture must refuse before the product touches the pre-populated cfg"
    );
}

fn assert_session_carries_a_message(pair: &common::RealPair, relay: &str, tag: &str) {
    let body = pair.a.iso.root.join("msg.bin");
    let want = format!("{tag} a->b over a real handshake").into_bytes();
    fs::write(&body, &want).unwrap();
    pair.a.run_ok(&[
        "send",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--to",
        &pair.b_label,
        "--file",
        body.to_str().unwrap(),
    ]);
    let out_dir = pair.b.iso.root.join("received");
    fs::create_dir_all(&out_dir).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&out_dir, fs::Permissions::from_mode(0o700)).unwrap();
    }
    pair.b.run_ok(&[
        "receive",
        "--transport",
        "relay",
        "--relay",
        relay,
        "--mailbox",
        &pair.b_route,
        "--from",
        &pair.a_label,
        "--max",
        "8",
        "--out",
        out_dir.to_str().unwrap(),
    ]);
    let received: Vec<Vec<u8>> = fs::read_dir(&out_dir)
        .unwrap()
        .map(|entry| fs::read(entry.unwrap().path()).unwrap())
        .collect();
    assert_eq!(
        received.iter().filter(|bytes| **bytes == want).count(),
        1,
        "{tag}: the peer decrypts the exact bytes exactly once"
    );
}

/// A-iii. A real pair over the LEASING relay: both sides handshake_complete (asserted by
/// the helper) and the established session carries a message a -> b.
#[test]
fn real_pair_over_leasing_relay_completes_both_sides() {
    let relay = common::start_qsl_server(1024 * 1024, 32, None);
    let pair = common::init_real_pair(
        "f03_aiii_leasing",
        PairRelay::Leasing(&relay),
        ("alice", "f03_aiii_route_alice_0123456"),
        ("bob", "f03_aiii_route_bob_0123456789"),
    );
    for side in [&pair.a, &pair.b] {
        assert_eq!(side.kind, VaultKind::Successor);
    }
    assert_session_carries_a_message(&pair, relay.base_url(), "A-iii leasing");
}

/// A-iii (relay-independent arm). The same helper over the delete-on-pull mock.
#[test]
fn real_pair_over_mock_relay_completes_both_sides() {
    let relay = common::start_inbox_server(1024 * 1024, 32);
    let pair = common::init_real_pair(
        "f03_aiii_mock",
        PairRelay::Mock(&relay),
        ("alice", "f03_aiii_mock_alice_01234567"),
        ("bob", "f03_aiii_mock_bob_0123456789"),
    );
    assert_session_carries_a_message(&pair, relay.base_url(), "A-iii mock");
}

/// Call sites of the two declared legacy-meaning helpers, SEALED at the integration head
/// b968b023 before the S4 edit (lane evidence a/census/CENSUS_SEALED_before_edit.json):
/// (name, files, occurrences).
const SEALED_CALLERS: [(&str, usize, usize); 2] = [
    ("init_mock_vault", 104, 320),
    ("init_passphrase_vault", 7, 14),
];

fn ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn call_sites(src: &str, name: &str) -> usize {
    let bytes = src.as_bytes();
    let mut count = 0;
    let mut from = 0;
    while let Some(found) = src[from..].find(name) {
        let at = from + found;
        let end = at + name.len();
        let mut next = end;
        while next < bytes.len() && bytes[next].is_ascii_whitespace() {
            next += 1;
        }
        let bounded = (at == 0 || !ident_byte(bytes[at - 1]))
            && (end == bytes.len() || !ident_byte(bytes[end]));
        if bounded && bytes.get(next) == Some(&b'(') && !src[..at].ends_with("fn ") {
            count += 1;
        }
        from = end;
    }
    count
}

fn census_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            census_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

/// No new caller of the declared legacy-meaning helpers: the census over tests/ (this file
/// excluded) must not exceed the count sealed before the S4 edit. Moving callers onto the
/// F03 helpers lowers it; the seal is then lowered in the same edit.
#[test]
fn old_helper_caller_census_does_not_grow() {
    let tests = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let mut files = Vec::new();
    census_files(&tests, &mut files);
    files.retain(|p| p.file_name().and_then(|n| n.to_str()) != Some("f03_fixture_helpers.rs"));
    assert!(files.len() > 100, "census input: {} files", files.len());
    for (name, sealed_files, sealed_sites) in SEALED_CALLERS {
        let per_file: Vec<usize> = files
            .iter()
            .map(|p| call_sites(&fs::read_to_string(p).unwrap(), name))
            .filter(|n| *n > 0)
            .collect();
        let sites: usize = per_file.iter().sum();
        println!(
            "F03 CENSUS {name}: files={} call_sites={sites} (sealed files={sealed_files} \
             call_sites={sealed_sites}; scanned {} files)",
            per_file.len(),
            files.len()
        );
        assert!(
            per_file.len() <= sealed_files && sites <= sealed_sites,
            "new callers of {name}: use init_ordinary_vault / init_successor_vault / \
             init_real_pair (FIXTURE_DESIGN sec 2)"
        );
    }
}
