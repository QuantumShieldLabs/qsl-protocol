// NA-0616 (ENG-0001): self-label footgun remediation.
// A config dir is meant to hold one self-identity. First-run auto-create and explicit
// `identity rotate` are allowed; silently auto-minting a SECOND, divergent self-identity
// via an inconsistent/typo'd `--as` on the handshake path must fail closed.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Output;

mod common;

fn test_root() -> PathBuf {
    let root = env::var("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("target"));
    let root = root.join("qsc-test-tmp");
    fs::create_dir_all(&root).unwrap();
    root
}

fn fresh_cfg(name: &str) -> PathBuf {
    let cfg = test_root().join(format!("na0616_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&cfg);
    fs::create_dir_all(&cfg).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&cfg, fs::Permissions::from_mode(0o700)).unwrap();
    }
    common::init_mock_vault(&cfg);
    cfg
}

fn out_text(out: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

fn rotate(cfg: &Path, label: &str) -> Output {
    common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args(["identity", "rotate", "--as", label, "--confirm"])
        .output()
        .expect("identity rotate")
}

fn identity_show_fp(cfg: &Path, label: &str) -> String {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args(["identity", "show", "--as", label])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", out_text(&out));
    out_text(&out)
        .lines()
        .find_map(|l| l.strip_prefix("identity_fp=").map(str::to_string))
        .expect("identity_fp in output")
}

// NA-0633 (ENG-0038): the peer's full identity KEM key, needed to authenticate it as the responder.
fn identity_show_kem(cfg: &Path, label: &str) -> String {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args(["identity", "show", "--as", label])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", out_text(&out));
    out_text(&out)
        .lines()
        .find_map(|l| l.strip_prefix("identity_kem_pk=").map(str::to_string))
        .expect("identity_kem_pk in output")
}

// NA-0634 (D571): the peer's full identity SIGNING key, provisioned alongside the KEM key.
fn identity_show_sig(cfg: &Path, label: &str) -> String {
    let out = common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args(["identity", "show", "--as", label])
        .output()
        .expect("identity show");
    assert!(out.status.success(), "{}", out_text(&out));
    out_text(&out)
        .lines()
        .find_map(|l| l.strip_prefix("identity_sig_pk=").map(str::to_string))
        .expect("identity_sig_pk in output")
}

fn contacts_add(cfg: &Path, peer: &str, fp: &str, kem_pk: &str, sig_pk: &str, token: &str) -> Output {
    common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "contacts",
            "add",
            "--label",
            peer,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
            "--sig-pk",
            sig_pk,
            "--route-token",
            token,
        ])
        .output()
        .expect("contacts add")
}

fn handshake_init(cfg: &Path, label: &str) -> Output {
    // An unreachable relay is fine: the self-identity gate runs before any network use.
    common::qsc_std_command()
        .env("QSC_CONFIG_DIR", cfg)
        .args([
            "handshake",
            "init",
            "--as",
            label,
            "--peer",
            "peer0",
            "--relay",
            "http://127.0.0.1:9",
        ])
        .output()
        .expect("handshake init")
}

fn self_record(cfg: &Path, label: &str) -> PathBuf {
    cfg.join("identities").join(format!("self_{label}.json"))
}

#[test]
fn rotate_creates_canonical_self() {
    let cfg = fresh_cfg("rotate_self");
    let out = rotate(&cfg, "self");
    assert!(out.status.success(), "{}", out_text(&out));
    assert!(
        self_record(&cfg, "self").exists(),
        "self record should exist"
    );
}

#[test]
fn second_divergent_self_label_fails_closed() {
    let cfg = fresh_cfg("divergent");
    assert!(rotate(&cfg, "self").status.success());
    // Set peer0 up as an authenticated contact (real fp from a separate config dir +
    // route token) so handshake init reaches the self-identity gate, which runs before
    // the network push.
    let peer_cfg = fresh_cfg("divergent_peer");
    assert!(rotate(&peer_cfg, "self").status.success());
    let peer_fp = identity_show_fp(&peer_cfg, "self");
    let peer_kem = identity_show_kem(&peer_cfg, "self");
    let peer_sig = identity_show_sig(&peer_cfg, "self");
    assert!(contacts_add(
        &cfg,
        "peer0",
        &peer_fp,
        &peer_kem, &peer_sig,
        "route_token_peer0_abcdefghijklmnop"
    )
    .status
    .success());
    // A typo'd/inconsistent handshake --as must fail closed, not silently mint.
    let out = handshake_init(&cfg, "alice");
    let text = out_text(&out);
    assert!(
        text.contains("identity_self_ambiguous"),
        "expected identity_self_ambiguous; got: {text}"
    );
    // No divergent identity was minted (no mutation on reject).
    assert!(
        !self_record(&cfg, "alice").exists(),
        "divergent self identity must not be created"
    );
}

#[test]
fn consistent_self_label_is_not_blocked_by_the_gate() {
    let cfg = fresh_cfg("consistent");
    assert!(rotate(&cfg, "self").status.success());
    // Using the same (canonical) label loads the existing identity; it may fail later on
    // the unreachable relay, but never with the ambiguity gate.
    let text = out_text(&handshake_init(&cfg, "self"));
    assert!(
        !text.contains("identity_self_ambiguous"),
        "consistent label must not trip the gate: {text}"
    );
}

#[test]
fn explicit_rotate_of_second_label_is_allowed() {
    let cfg = fresh_cfg("explicit_second");
    assert!(rotate(&cfg, "self").status.success());
    // Explicit rotate is the intentional multi-identity path and bypasses the gate.
    let out = rotate(&cfg, "bob");
    assert!(out.status.success(), "{}", out_text(&out));
    assert!(self_record(&cfg, "bob").exists());
}
