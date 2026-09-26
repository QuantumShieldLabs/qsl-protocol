// Fresh acceptance uses only extracted, unchanged handshake/relay helpers.
#![allow(dead_code, unused_imports)]
use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::aead::{Aead, Payload};
use chacha20poly1305::KeyInit;
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};
use quantumshield_refimpl::crypto::stdcrypto::{
    runtime_pq_kem_ciphertext_bytes, runtime_pq_kem_keypair, runtime_pq_kem_public_key_bytes,
    runtime_pq_kem_secret_key_bytes, runtime_pq_sig_public_key_bytes,
    runtime_pq_sig_signature_bytes, StdCrypto,
};
use quantumshield_refimpl::crypto::traits::{Kmac, PqKem768};
use quantumshield_refimpl::qse::Envelope;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::ratchet::send_pq_advertise;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::{Duration, Instant};

mod common;

const ROUTE_TOKEN_ALICE: &str = "route_token_alice_abcdefghijklmnop";
const ROUTE_TOKEN_BOB: &str = "route_token_bob_abcdefghijklmnopqr";

fn kem_pk_len() -> usize {
    runtime_pq_kem_public_key_bytes()
}

fn kem_ct_len() -> usize {
    runtime_pq_kem_ciphertext_bytes()
}

fn safe_test_root() -> PathBuf {
    let root = if let Ok(v) = env::var("QSC_TEST_ROOT") {
        PathBuf::from(v)
    } else if let Ok(v) = env::var("CARGO_TARGET_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::from("target")
    };
    let root = root.join("qsc-test-tmp");
    ensure_dir_700(&root);
    root
}

fn ensure_dir_700(path: &Path) {
    if let Ok(meta) = fs::symlink_metadata(path) {
        if meta.is_file() {
            let _ = fs::remove_file(path);
        }
    }
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn session_path(cfg: &Path, peer: &str) -> PathBuf {
    cfg.join("qsp_sessions").join(format!("{}.qsv", peer))
}

fn post_raw(relay: &str, channel: &str, body: Vec<u8>) {
    let url = format!("{}/v1/push", relay.trim_end_matches('/'));
    let client = reqwest::blocking::Client::new();
    let _ = client
        .post(url)
        .header("X-QSL-Route-Token", channel)
        .body(body)
        .send();
}

fn run_qsc(cfg: &Path, args: &[&str]) -> std::process::Output {
    qsc_cfg_cmd(cfg).args(args).output().expect("qsc command")
}

fn qsc_cfg_cmd(cfg: &Path) -> std::process::Command {
    let mut cmd = common::qsc_std_command();
    cmd.env("QSC_CONFIG_DIR", cfg);
    cmd
}

fn output_text(out: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&out.stdout).to_string();
    text.push_str(&String::from_utf8_lossy(&out.stderr));
    text
}

fn contacts_add_with_route(cfg: &Path, label: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "route-set",
            "--label",
            label,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn relay_inbox_set(cfg: &Path, token: &str) {
    let out = run_qsc(cfg, &["relay", "inbox-set", "--token", token]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stdout)
    );
}

fn identity_fp(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("identity_fp=") {
            return value.to_string();
        }
    }
    panic!("missing identity_fp in output: {}", text);
}

fn identity_kem_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("identity_kem_pk=") {
            return value.to_string();
        }
    }
    panic!("missing identity_kem_pk in output: {}", text);
}

fn identity_sig_pk(cfg: &Path, label: &str) -> String {
    let out = run_qsc(cfg, &["identity", "show", "--as", label]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let text = String::from_utf8_lossy(&out.stdout);
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("identity_sig_pk=") {
            return value.to_string();
        }
    }
    panic!("missing identity_sig_pk in output: {}", text);
}

fn init_identity(cfg: &Path, label: &str) {
    let out = run_qsc(cfg, &["identity", "rotate", "--as", label, "--confirm"]);
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn contacts_add_pinned_with_route(cfg: &Path, label: &str, fp: &str, kem_pk: &str, sig_pk: &str, token: &str) {
    let out = run_qsc(
        cfg,
        &[
            "contacts",
            "add",
            "--label",
            label,
            "--fp",
            fp,
            "--kem-pk",
            kem_pk,
            "--sig-pk",
            sig_pk,
            "--route-token",
            token,
        ],
    );
    assert!(
        out.status.success(),
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let list = run_qsc(cfg, &["contacts", "device", "list", "--label", label]);
    assert!(
        list.status.success(),
        "{}{}",
        String::from_utf8_lossy(&list.stdout),
        String::from_utf8_lossy(&list.stderr)
    );
    let list_text = String::from_utf8_lossy(&list.stdout);
    let device_id = list_text
        .lines()
        .find(|line| line.starts_with("device="))
        .and_then(|line| {
            line.split_whitespace()
                .find_map(|tok| tok.strip_prefix("device="))
        })
        .unwrap_or_else(|| panic!("missing device id in output: {list_text}"));
    let trust = run_qsc(
        cfg,
        &[
            "contacts",
            "device",
            "trust",
            "--label",
            label,
            "--device",
            device_id,
            "--confirm",
        ],
    );
    assert!(
        trust.status.success(),
        "{}{}",
        String::from_utf8_lossy(&trust.stdout),
        String::from_utf8_lossy(&trust.stderr)
    );
}

fn seed_authenticated_pair(alice_cfg: &Path, bob_cfg: &Path) {
    init_identity(alice_cfg, "alice");
    init_identity(bob_cfg, "bob");
    let alice_fp = identity_fp(alice_cfg, "alice");
    let alice_kem = identity_kem_pk(alice_cfg, "alice");
    let alice_sig = identity_sig_pk(alice_cfg, "alice");
    let bob_fp = identity_fp(bob_cfg, "bob");
    let bob_kem = identity_kem_pk(bob_cfg, "bob");
    let bob_sig = identity_sig_pk(bob_cfg, "bob");
    contacts_add_pinned_with_route(alice_cfg, "bob", bob_fp.as_str(), bob_kem.as_str(), bob_sig.as_str(), ROUTE_TOKEN_BOB);
    contacts_add_pinned_with_route(bob_cfg, "alice", alice_fp.as_str(), alice_kem.as_str(), alice_sig.as_str(), ROUTE_TOKEN_ALICE);
}

fn hs_dance(alice_cfg: &Path, bob_cfg: &Path, relay: &str, server:&common::InboxTestServer) {
    seed_authenticated_pair(alice_cfg, bob_cfg);
    relay_inbox_set(alice_cfg, ROUTE_TOKEN_ALICE);
    relay_inbox_set(bob_cfg, ROUTE_TOKEN_BOB);
    let init = qsc_cfg_cmd(alice_cfg)
        .args([
            "handshake",
            "init",
            "--as",
            "alice",
            "--peer",
            "bob",
            "--relay",
            relay,
        ])
        .output()
        .expect("hs init");
    assert!(init.status.success(), "{}", output_text(&init));
    let originals=server.drain_channel(ROUTE_TOKEN_BOB);
    assert_eq!(originals.len(),1);
    let mut changed=originals[0].clone();
    let block_len=u16::from_be_bytes([changed[7],changed[8]]) as usize;
    assert!(changed[9..9+block_len].windows(25).any(|w|w==b"NA0780-DIR-INTEGRATION-03"));
    changed[9+block_len-1]^=1;
    server.replace_channel(ROUTE_TOKEN_BOB,vec![changed]);
    let _rejected=run_qsc(bob_cfg,&["handshake","poll","--as","bob","--peer","alice","--relay",relay,"--max","4"]);
    assert!(!session_path(bob_cfg,"alice").exists(),"wrong profile established a session");
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),"wrong profile got a response");
    server.replace_channel(ROUTE_TOKEN_BOB,originals);
    for (cfg, me, peer) in [
        (bob_cfg, "bob", "alice"),
        (alice_cfg, "alice", "bob"),
        (bob_cfg, "bob", "alice"),
    ] {
        let out = qsc_cfg_cmd(cfg)
            .args([
                "handshake",
                "poll",
                "--as",
                me,
                "--peer",
                peer,
                "--relay",
                relay,
                "--max",
                "4",
            ])
            .output()
            .expect("hs poll");
        assert!(out.status.success(), "{}", output_text(&out));
    }
    assert!(
        session_path(alice_cfg, "bob").exists(),
        "alice session missing"
    );
    assert!(
        session_path(bob_cfg, "alice").exists(),
        "bob session missing"
    );
}

fn send_msg(cfg: &Path, relay: &str, to: &str, path: &Path) -> String {
    let out = qsc_cfg_cmd(cfg)
        .args([
            "send",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--to",
            to,
            "--file",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("send");
    assert!(out.status.success(), "{}", output_text(&out));
    output_text(&out)
}

fn recv_msg_drain(
    cfg: &Path,
    relay: &str,
    mailbox: &str,
    from: &str,
    out_dir: &Path,
) -> std::process::Output {
    qsc_cfg_cmd(cfg)
        .args([
            "receive",
            "--transport",
            "relay",
            "--relay",
            relay,
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--receipt-mode", "immediate",
            "--max",
            "8",
            "--out",
            out_dir.to_str().unwrap(),
        ])
        .output()
        .expect("recv")
}



fn integration_state(cfg:&Path,peer:&str)->serde_json::Value {
    env::set_var("QSC_CONFIG_DIR",cfg);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let raw=qsc::vault::secret_get(&format!("na0780_directional_transaction_v2/{peer}")).unwrap().expect("candidate transaction");
    serde_json::from_str(&raw).unwrap()
}
fn delivered_count(out:&Path)->usize {
    fs::read_dir(out).unwrap().filter_map(Result::ok).filter(|e|e.path().extension().is_some_and(|s|s=="bin")).count()
}
fn poll_candidate(cfg:&Path,relay:&str,mailbox:&str,peer:&str,out:&Path) {
    let result=recv_msg_drain(cfg,relay,mailbox,peer,out);
    assert!(result.status.success(),"candidate poll failed: {}",output_text(&result));
}
// Diagnostic-only progress; original acceptance assertions remain in their order.
fn acceptance_phase<T>(name: &str, work: impl FnOnce() -> T) -> T {
    use std::io::Write;
    let Some(root) = std::env::var_os("NA0780_PHASE_DIR") else { return work(); };
    let root = std::path::PathBuf::from(root);
    assert!(root.is_dir());
    assert!(name.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-'));
    let start = std::time::Instant::now();
    let emit = |event: &str| {
        static SEQUENCE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let sequence = SEQUENCE.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let unix_ms = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis();
        let record = serde_json::json!({"phase":name,"event":event,"elapsed_ms":start.elapsed().as_millis(),"sequence":sequence,"unix_ms":unix_ms});
        let path = root.join(format!("{sequence:06}_{name}.{event}.json"));
        let temporary = path.with_extension("pending");
        let mut file = std::fs::OpenOptions::new().write(true).create_new(true).open(&temporary).unwrap();
        writeln!(file, "{record}").unwrap(); file.sync_all().unwrap();
        // Link publishes complete bytes without overwriting an earlier result.
        std::fs::hard_link(&temporary, &path).unwrap();
        std::fs::remove_file(&temporary).unwrap();
        std::fs::File::open(&root).unwrap().sync_all().unwrap();
        eprintln!("NA0780_PHASE {record}");
    };
    emit("begin");
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(work)) {
        Ok(value) => { emit("pass"); value }
        Err(error) => { emit("failed"); std::panic::resume_unwind(error) }
    }
}

#[test]
fn directional_integration_acceptance() {
    for k in ["QSC_QSP_SEED","QSC_ALLOW_SEED_FALLBACK","QSC_UNSAFE_TEST_SEED_FALLBACK"] {env::remove_var(k);}
    for order in 0..2 {
        acceptance_phase(&format!("o{order}_all"),||{
        let base=safe_test_root().join(format!("directional_integration_{}_{}",std::process::id(),order));
        assert!(!base.exists());ensure_dir_700(&base);
        let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
        for dir in [&a,&b,&ao,&bo] {ensure_dir_700(dir);}
        acceptance_phase(&format!("o{order}_vault_init"),||{common::init_mock_vault(&a);common::init_mock_vault(&b);});
        let server=acceptance_phase(&format!("o{order}_relay_start"),||common::start_inbox_server(1024*1024,128));let relay=server.base_url();
        acceptance_phase(&format!("o{order}_handshake"),||hs_dance(&a,&b,&relay,&server));
        acceptance_phase(&format!("o{order}_profile_assertions"),||{
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        assert_eq!(sa["version"],"NA0780-DIR-INTEGRATION-03");assert_eq!(sa["core"]["sid"],sb["core"]["sid"]);
        assert_eq!(sa["core"]["root"],sb["core"]["root"]);
        println!("NA0780_ACCEPT group=authenticated_profile order={order} result=pass");
        });
        let af=base.join("a.body");let bf=base.join("b.body");
        for round in 0..28 {
            fs::write(&af,format!("NDI1 application A {order}/{round}")).unwrap();
            fs::write(&bf,format!("{{\"ns\":\"qsc\",\"type\":\"ack\",\"application\":\"B {order}/{round}\"}}")).unwrap();
            let accepted_a=acceptance_phase(&format!("o{order}_r{round}_send_a"),||acceptance_enqueue_once(&a,&relay,"bob",&af));
            let accepted_b=acceptance_phase(&format!("o{order}_r{round}_send_b"),||acceptance_enqueue_once(&b,&relay,"alice",&bf));
            // Reorder exact admitted ciphertexts and inject one tampered duplicate
            // before the legitimate wire; no cryptographic state is injected.
            if round==0 {
                for mailbox in [ROUTE_TOKEN_ALICE,ROUTE_TOKEN_BOB] {
                    let original=server.drain_channel(mailbox);
                    let mut schedule=Vec::new();
                    for raw in original.into_iter().rev() {
                        let mut bad=raw.clone(); if let Some(last)=bad.last_mut(){*last^=1;}
                        schedule.extend([bad,raw.clone(),raw]);
                    }
                    server.replace_channel(mailbox,schedule);
                }
            }
            // Both user sends finish before either poll. One order is reversed.
            for poll in 0..4 {
                acceptance_phase(&format!("o{order}_r{round}_poll{poll}"),||{
                if order==0 {acceptance_phase(&format!("o{order}_r{round}_poll{poll}_b"),||poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo));acceptance_phase(&format!("o{order}_r{round}_poll{poll}_a"),||poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao));}
                else {acceptance_phase(&format!("o{order}_r{round}_poll{poll}_a"),||poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao));acceptance_phase(&format!("o{order}_r{round}_poll{poll}_b"),||poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo));}
                });
            }
            // Preserve the original four crossed-poll rounds above. Any remaining
            // operation is serviced through receive/retry; it is never enqueued again.
            for recovery in 0..=4 {
                let done_a=acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_status_a"),||acceptance_same_operation_done(&a,"bob",&accepted_a));
                let done_b=acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_status_b"),||acceptance_same_operation_done(&b,"alice",&accepted_b));
                if done_a && done_b {break;}
                assert!(recovery<4,"same accepted operations did not complete within bounded service");
                acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_retry_a"),||retry_queue(&a,&relay));
                acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_retry_b"),||retry_queue(&b,&relay));
                if order==0 {
                    acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_poll_b"),||poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo));
                    acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_poll_a"),||poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao));
                } else {
                    acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_poll_a"),||poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao));
                    acceptance_phase(&format!("o{order}_r{round}_recover{recovery}_poll_b"),||poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo));
                }
            }
            acceptance_phase(&format!("o{order}_r{round}_assertions"),||{
            assert_eq!(delivered_count(&ao),round+1,"A missing or duplicate delivery");
            assert_eq!(delivered_count(&bo),round+1,"B missing or duplicate delivery");
            let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
            assert!(sa["send"].as_object().unwrap().len()+sa["recv"].as_object().unwrap().len()<=3);
            assert!(sb["send"].as_object().unwrap().len()+sb["recv"].as_object().unwrap().len()<=3);
            });
        }
        acceptance_phase(&format!("o{order}_target_assertions"),||{
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        assert!(sa["core"]["local_consumed_prefix"].as_u64().unwrap()>4,"A targets not retired");
        assert!(sb["core"]["local_consumed_prefix"].as_u64().unwrap()>4,"B targets not retired");
        println!("NA0780_ACCEPT group=crossed_traffic_and_targets order={order} result=pass");
        });
        acceptance_phase(&format!("o{order}_cut_recovery"),||{
        // Actual process cut after outgoing state/ciphertext commit, before push.
        fs::write(&af,b"restart admitted application").unwrap();
        let cut=acceptance_phase(&format!("o{order}_cut_send"),||qsc_cfg_cmd(&a).env("QSC_NA0780_CUT","after_prepare_commit")
            .args(["send","--transport","relay","--relay",&relay,"--to","bob","--file",af.to_str().unwrap()]).output().unwrap());
        assert_eq!(cut.status.code(),Some(86),"send cut not reached");
        let saved=integration_state(&a,"bob");let generation=saved["generation"].as_u64().unwrap();
        assert_eq!(qsc::na0780_test_stale_generation("bob",generation-1),Err("directional_stale_generation"));
        for poll in 0..4 {
            acceptance_phase(&format!("o{order}_cut_poll{poll}_a"),||poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao));
            acceptance_phase(&format!("o{order}_cut_poll{poll}_b"),||poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo));
        }
        assert_eq!(delivered_count(&bo),29,"admitted send not recovered once");
        println!("NA0780_ACCEPT group=send_cut_stale_generation order={order} result=pass");
        });
        });
    }
}

#[test]
fn directional_integration_resume() {
    let base=PathBuf::from(env::var("NA0780_RESUME_BASE").expect("saved fixture path"));
    let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    for(cfg,peer,label) in [(&a,"bob","A"),(&b,"alice","B")] {
        let s=integration_state(cfg,peer);
        let summary=|name:&str|s[name].as_object().unwrap().iter().map(|(g,e)|serde_json::json!({"epoch":g,"next":e["next"],"prefix":e["prefix"],"confirmed":e["confirmed"],"terminal":e["terminal"]})).collect::<Vec<_>>();
        println!("NA0780_STATE {}",serde_json::json!({"peer":label,"seq":s["core"]["seq"],"owner":s["core"]["owner"],"send":summary("send"),"recv":summary("recv"),"flights":s["flights"].as_object().unwrap().len(),"responses":s["dispositions"].as_object().unwrap().len()}));
    }
    // Fresh relay/processes, original encrypted client state. Recover exact
    // outstanding traffic and unsent responses; no snapshot rollback or key edit.
    for _ in 0..8 {poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);}
    println!("NA0780_ACCEPT group=retained_capacity_recovery result=pass");
    let before_a=delivered_count(&ao);let before_b=delivered_count(&bo);
    let af=base.join("resume-a.body");let bf=base.join("resume-b.body");
    fs::write(&af,b"resume A real application").unwrap();fs::write(&bf,b"resume B real application").unwrap();
    send_msg(&a,&relay,"bob",&af);send_msg(&b,&relay,"alice",&bf);
    for _ in 0..4 {poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);}
    assert!(delivered_count(&ao)>before_a);assert!(delivered_count(&bo)>before_b);
    // Drive the existing T seam, never a root/key/turn injection. Stop as soon
    // as both directions have more than four authenticated consumptions.
    for cycle in 0..12 {
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        if sa["core"]["local_consumed_prefix"].as_u64().unwrap()>4 && sb["core"]["local_consumed_prefix"].as_u64().unwrap()>4 {break;}
        let now=sa["last_boundary"].as_u64().unwrap().max(sb["last_boundary"].as_u64().unwrap())+901;
        env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
        fs::write(&af,format!("cadence A {cycle}")).unwrap();fs::write(&bf,format!("cadence B {cycle}")).unwrap();
        send_msg(&a,&relay,"bob",&af);send_msg(&b,&relay,"alice",&bf);
        for _ in 0..4 {poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);}
    }
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    assert!(sa["core"]["local_consumed_prefix"].as_u64().unwrap()>4,"A target cycles incomplete");
    assert!(sb["core"]["local_consumed_prefix"].as_u64().unwrap()>4,"B target cycles incomplete");
    println!("NA0780_ACCEPT group=target_cycles_retained_pair result=pass");
}

#[test]
fn directional_late_commit() {
    use std::process::Stdio;
    let base=PathBuf::from(env::var("NA0780_RESUME_BASE").unwrap());
    let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    // Recover retained work without claiming that polls alone deliver anything.
    for _ in 0..2 {poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);}
    let body=base.join("late.body");fs::write(&body,b"late commit operation").unwrap();
    let gate=base.join("late-gate");
    let logfile=fs::File::create(base.join("late-child.log")).unwrap();
    let mut child=qsc_cfg_cmd(&a).env("QSC_NA0780_COMMIT_GATE",&gate)
        .args(["send","--transport","relay","--relay",&relay,"--to","bob","--file",body.to_str().unwrap()])
        .stdout(Stdio::from(logfile.try_clone().unwrap())).stderr(Stdio::from(logfile)).spawn().unwrap();
    let deadline=Instant::now()+Duration::from_secs(60);
    while !gate.with_extension("ready").exists(){
        assert!(child.try_wait().unwrap().is_none(),"send exited before commit gate");
        assert!(Instant::now()<deadline,"commit gate not reached");thread::sleep(Duration::from_millis(20));
    }
    // The original sender is now suspended outside the store lock.
    let waiting=integration_state(&a,"bob");
    let original:Vec<u8>=serde_json::from_value(waiting["flights"].as_object().unwrap().values().find(|f|f["id"].as_str().is_some_and(|id|!id.is_empty()) && !f["accepted"].as_bool().unwrap()).unwrap()["wire"].clone()).unwrap();
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let other=base.join("other.body");fs::write(&other,b"newer outstanding operation").unwrap();
    send_msg(&a,&relay,"bob",&other);
    let before=integration_state(&a,"bob");
    assert!(!before["flights"].as_object().unwrap().is_empty(),"newer operation must remain outstanding");
    fs::write(gate.with_extension("release"),b"release").unwrap();
    let status=child.wait().unwrap();
    let after=integration_state(&a,"bob");
    assert!(before==after,"late commit changed authoritative newer state");
    assert!(fs::read_dir(&bo).unwrap().filter_map(Result::ok).any(|f|fs::read(f.path()).ok().as_deref()==Some(b"late commit operation")),"actual payload missing");
    if env::var_os("NA0780_EXPECT_LATE_FAILURE").is_some(){
        assert!(!status.success());assert!(fs::read_to_string(base.join("late-child.log")).unwrap().contains("FLIGHT_MISSING"));
        println!("NA0780_ACCEPT group=late_commit_reproduced_no_state_overwrite result=pass");
    }else{
        assert!(status.success(),"delayed completed operation must succeed idempotently");
        assert!(qsc::na0780_test_commit_probe("bob",b"late commit operation",original.clone()).is_ok());
        let mut bad=original;*bad.last_mut().unwrap()^=1;
        assert_eq!(qsc::na0780_test_commit_probe("bob",b"late commit operation",bad),Err("directional_queue_conflict"));
        let other:Vec<u8>=serde_json::from_value(before["flights"].as_object().unwrap().values().find(|f|!f["id"].as_str().unwrap().is_empty()).unwrap()["wire"].clone()).unwrap();
        assert_eq!(qsc::na0780_test_commit_probe("bob",b"late commit operation",other),Err("directional_queue_conflict"));
        assert!(integration_state(&a,"bob")==before,"commit probes changed newer transaction");
        println!("NA0780_ACCEPT group=late_commit_idempotent_exact_conflict_guards result=pass");
    }
}

fn retained_paths()->(PathBuf,PathBuf,PathBuf,PathBuf,PathBuf){
    let base=PathBuf::from(env::var("NA0780_RESUME_BASE").unwrap());
    (base.clone(),base.join("alice"),base.join("bob"),base.join("a-out"),base.join("b-out"))
}
fn payload_once(out:&Path,body:&[u8]) {
    assert_eq!(fs::read_dir(out).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(body)).count(),1,"payload must be byte-exact and unique");
}
fn paired_polls(a:&Path,b:&Path,ao:&Path,bo:&Path,relay:&str,n:usize){
    for _ in 0..n {poll_candidate(a,relay,ROUTE_TOKEN_ALICE,"bob",ao);poll_candidate(b,relay,ROUTE_TOKEN_BOB,"alice",bo);}
}
#[test]
fn directional_second_order(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    paired_polls(&a,&b,&ao,&bo,&relay,2);
    let af=base.join("second-a.body");let bf=base.join("second-b.body");
    fs::write(&af,b"second order A typed application").unwrap();fs::write(&bf,b"second order B typed application").unwrap();
    send_msg(&a,&relay,"bob",&af);send_msg(&b,&relay,"alice",&bf);
    // Reverse the earlier crossing's first receiver, with both sends already done.
    paired_polls(&a,&b,&ao,&bo,&relay,3);
    payload_once(&bo,&fs::read(&af).unwrap());payload_once(&ao,&fs::read(&bf).unwrap());
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    assert!(sa["core"]["root"]==sb["core"]["root"]);
    assert!(sa["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    assert!(sb["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    println!("NA0780_ACCEPT group=second_crossing_order_actual_payloads_and_receipts result=pass");
}
#[test]
fn directional_target_retirement(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let initial_a=integration_state(&a,"bob")["core"]["local_consumed_prefix"].as_u64().unwrap();
    let initial_b=integration_state(&b,"alice")["core"]["local_consumed_prefix"].as_u64().unwrap();
    let af=base.join("target-a.body");let bf=base.join("target-b.body");let mut cycles=0;
    for cycle in 0..12 {
        paired_polls(&a,&b,&ao,&bo,&relay,2);
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        if sa["core"]["local_consumed_prefix"].as_u64().unwrap()>=initial_a+5 && sb["core"]["local_consumed_prefix"].as_u64().unwrap()>=initial_b+5 {break;}
        let now=sa["last_boundary"].as_u64().unwrap().max(sb["last_boundary"].as_u64().unwrap())+901;
        env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
        fs::write(&af,format!("retirement A {cycle}")).unwrap();fs::write(&bf,format!("retirement B {cycle}")).unwrap();
        send_msg(&a,&relay,"bob",&af);send_msg(&b,&relay,"alice",&bf);
        paired_polls(&a,&b,&ao,&bo,&relay,2);
        payload_once(&bo,&fs::read(&af).unwrap());payload_once(&ao,&fs::read(&bf).unwrap());
        for (cfg,peer) in [(&a,"bob"),(&b,"alice")] {
            let s=integration_state(cfg,peer);
            assert!(s["send"].as_object().unwrap().len()+s["recv"].as_object().unwrap().len()<=3);
            assert!(s["core"]["local"].as_object().unwrap().len()<=1);assert!(s["core"]["peer"].as_object().unwrap().len()<=1);
            assert!(s["flights"].as_object().unwrap().values().all(|f|f["id"]==""),"application receipts incomplete");
        }
        cycles+=1;
    }
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    assert!(sa["core"]["local_consumed_prefix"].as_u64().unwrap()>=initial_a+5 && sb["core"]["local_consumed_prefix"].as_u64().unwrap()>=initial_b+5,"target retirement incomplete");
    assert!(sa["core"]["root"]==sb["core"]["root"]);
    assert!(sa["recv_floor"].is_number() && sb["recv_floor"].is_number(),"receiver closures not proved");
    println!("NA0780_ACCEPT group=sustained_target_retirement cycles={cycles} each_direction_new_consumptions=5 result=pass");
}
fn queue_operation_present(cfg:&Path,peer:&str,file:&Path)->bool {
    let result=std::process::Command::new(env::current_exe().unwrap()).args(["--exact","directional_queue_worker","--nocapture"])
        .env("NA0780_WORKER_CFG",cfg).env("NA0780_WORKER_PEER",peer).env("NA0780_WORKER_BODY",file)
        .env_remove("NA0780_WORKER_RECOVER").output().unwrap();
    assert!(result.status.success(),"fixture queue lookup failed or duplicate intended operation");
    String::from_utf8_lossy(&result.stdout).contains("NA0780_QUEUE present=true")
}

// Accept only demonstrated admission backpressure, after proving one durable enqueue.
fn acceptance_enqueue_once(cfg:&Path,relay:&str,peer:&str,file:&Path)->qsc::msgqueue::QueuedMessage {
    let body=fs::read(file).unwrap();
    assert!(acceptance_queue_records(cfg,peer).iter().all(|r|r.body!=body),"operation already enqueued");
    let out=qsc_cfg_cmd(cfg).args(["send","--transport","relay","--relay",relay,"--to",peer,"--file",file.to_str().unwrap()]).output().unwrap();
    let mut matches:Vec<_>=acceptance_queue_records(cfg,peer).into_iter().filter(|r|r.body==body).collect();
    assert_eq!(matches.len(),1,"one send must create exactly one durable operation");
    let rec=matches.pop().unwrap();
    if !out.status.success() {
        assert!(output_text(&out).lines().any(|line|line=="QSC_MARK/1 event=error code=RECEIPT_CONTEXT_CAPACITY"),"unexpected admission failure: {}",output_text(&out));
        assert_eq!(rec.state.as_str(),"QUEUED","capacity refusal must retain queued work");
        assert!(!rec.is_packed(),"refused boundary must not fabricate a packed operation");
    } else {
        assert!(matches!(rec.state,qsc::msgqueue::MsgState::Sent|qsc::msgqueue::MsgState::Delivered));
    }
    rec
}

fn acceptance_same_operation_done(cfg:&Path,peer:&str,expected:&qsc::msgqueue::QueuedMessage)->bool {
    let records=acceptance_queue_records(cfg,peer);
    let matches:Vec<_>=records.iter().filter(|r|r.body==expected.body).collect();
    assert_eq!(matches.len(),1,"no duplicate or missing accepted operation");
    let current=matches[0];
    assert!(current.msg_id==expected.msg_id && current.seq==expected.seq && current.peer==expected.peer && current.body==expected.body,"accepted operation identity changed");
    if let Some(hash)=expected.directional_wire_hash {assert_eq!(current.directional_wire_hash,Some(hash));}
    assert!(matches!(current.state,qsc::msgqueue::MsgState::Queued|qsc::msgqueue::MsgState::Sent|qsc::msgqueue::MsgState::Delivered),"accepted operation entered terminal failure");
    current.state==qsc::msgqueue::MsgState::Delivered
}

// Read-only synthetic-fixture queue inspection, using the existing record format.
// Never log the store key, message ID, body, or ciphertext.
fn acceptance_queue_records(cfg: &Path, peer: &str) -> Vec<qsc::msgqueue::QueuedMessage> {
    use sha2::{Digest, Sha512};
    let digest = Sha512::digest(peer.as_bytes());
    let contact: String = digest[..8].iter().map(|b| format!("{b:02x}")).collect();
    let entries = match fs::read_dir(cfg.join("msgqueue_v1").join(&contact)) {
        Ok(entries) => entries,
        Err(error) if error.kind()==std::io::ErrorKind::NotFound => return Vec::new(),
        Err(error) => panic!("queue inspection failed: {error}"),
    };
    env::set_var("QSC_CONFIG_DIR", cfg);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let encoded = qsc::vault::secret_get("msgqueue_store_key_v1").unwrap().unwrap();
    assert_eq!(encoded.len(), 64);
    let key: Vec<u8> = encoded.as_bytes().chunks_exact(2).map(|pair| {
        u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap()
    }).collect();
    assert_eq!(key.len(), 32);
    let mut records = Vec::new();
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("rec") { continue; }
        let filename = path.file_name().unwrap().to_str().unwrap();
        let (sequence, id) = filename.strip_suffix(".rec").unwrap().split_once('_').unwrap();
        let sequence: u64 = sequence.parse().unwrap();
        let aad = format!("qsc.msgqueue.v1|{contact}|{id}|{sequence}");
        let raw = fs::read(&path).unwrap();
        let clear = ChaCha20Poly1305::new(Key::from_slice(&key)).decrypt(
            Nonce::from_slice(&raw[..12]), Payload { msg: &raw[12..], aad: aad.as_bytes() }
        ).expect("synthetic queue authentication failed");
        let rec: qsc::msgqueue::QueuedMessage = serde_json::from_slice(&clear).unwrap();
        assert!(rec.msg_id == id && rec.seq == sequence && rec.peer == peer);
        records.push(rec);
    }
    records.sort_by_key(|rec| rec.seq);
    records
}

fn r5_snapshot(base: &Path, output: &Path, label: &str) -> (serde_json::Value, serde_json::Value) {
    let a = integration_state(&base.join("alice"), "bob");
    let b = integration_state(&base.join("bob"), "alice");
    let summary = |s: &serde_json::Value| {
        let epochs = |direction: &str| s[direction].as_object().unwrap().iter().map(|(id, e)|
            serde_json::json!({"epoch":id,"next":e["next"],"prefix":e["prefix"],"confirmed":e["confirmed"],"terminal":e["terminal"]})
        ).collect::<Vec<_>>();
        serde_json::json!({"send":epochs("send"),"recv":epochs("recv"),"send_floor":s["send_floor"],"recv_floor":s["recv_floor"],
            "role":s["core"]["role"],"owner":s["core"]["owner"],"sequence":s["core"]["seq"],"demand":s["demand"],"cadence":s["since_boundary"],
            "local_targets":s["core"]["local"].as_object().unwrap().len(),"peer_targets":s["core"]["peer"].as_object().unwrap().len(),
            "flights":s["flights"].as_object().unwrap().values().map(|f|serde_json::json!({"epoch":f["epoch"],"slot":f["slot"],"application":f["id"]!="","accepted":f["accepted"]})).collect::<Vec<_>>(),
            "dispositions":s["dispositions"].as_object().unwrap().iter().map(|(slot,d)|serde_json::json!({"slot":slot,"pending":d["response_pending"]})).collect::<Vec<_>>(),
            "events":s["events"].as_object().unwrap().len(),"completed":s["completed"].as_object().unwrap().len()})
    };
    let queue = acceptance_queue_records(&base.join("alice"), "bob");
    let body = fs::read(base.join("a.body")).unwrap();
    let matches: Vec<_> = queue.iter().filter(|r| r.body == body).collect();
    assert_eq!(matches.len(), 1, "saved exact operation must remain unique");
    let rec = matches[0];
    let value = serde_json::json!({"alice":summary(&a),"bob":summary(&b),"roots_equal":a["core"]["root"]==b["core"]["root"],
        "selected_queue":{"count":matches.len(),"sequence":rec.seq,"state":rec.state.as_str(),"packed":rec.is_packed(),"attempts":rec.attempts,"last_error":rec.last_error},
        "deliveries_a":delivered_count(&base.join("a-out")),"deliveries_b":delivered_count(&base.join("b-out"))});
    fs::write(output.join(format!("{label}.json")), serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    println!("NA0780_R5 snapshot={label}");
    (a,b)
}

#[test]
fn directional_r5_copied_diagnosis() {
    let base = PathBuf::from(env::var("NA0780_R5_COPY").expect("copied fixture required"));
    let output = PathBuf::from(env::var("NA0780_R5_OUTPUT").expect("new evidence directory required"));
    fs::create_dir(&output).unwrap();
    let (a,b,ao,bo) = (base.join("alice"),base.join("bob"),base.join("a-out"),base.join("b-out"));
    let (before_a,before_b) = r5_snapshot(&base,&output,"00_before");
    if env::var_os("NA0780_R5_EXPERIMENT").is_none() { return; }
    // Fresh mock relay: the terminated relay's queues do not survive. Normal
    // receive flushes resend retained exact flights; no snapshot state is edited.
    let body = fs::read(base.join("a.body")).unwrap();
    let selected = acceptance_queue_records(&a,"bob").into_iter().find(|r|r.body==body).unwrap();
    assert_eq!(selected.state.as_str(),"QUEUED");
    let server = common::start_inbox_server(1024*1024,128); let relay=server.base_url();
    assert!(before_a["flights"].as_object().unwrap().is_empty());
    assert!(before_b["flights"].as_object().unwrap().is_empty());
    assert_eq!(before_b["send"]["1"]["terminal"],before_b["send"]["1"]["prefix"]);
    assert_eq!(before_b["send"]["1"]["prefix"],7);
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let traffic = server.drain_channel(ROUTE_TOKEN_ALICE);
    let (waiting_a,waiting_b)=r5_snapshot(&base,&output,"01_bob_normal_receive");
    assert_eq!(waiting_a,before_a,"Bob receive must not edit Alice's store");
    assert!(waiting_b["send"].get("1").is_none(),"normal closure preparation must retire Bob send1");
    assert_eq!(waiting_b["flights"].as_object().unwrap().len(),1);
    for flight in waiting_b["flights"].as_object().unwrap().values() {
        let wire:Vec<u8>=serde_json::from_value(flight["wire"].clone()).unwrap();
        assert!(traffic.contains(&wire),"relay must hold exact durably retained Bob closure flight");
        assert_eq!(wire[4],0,"closure must be ordinary, not a forced refresh");
        assert_eq!(flight["epoch"],3);assert_eq!(flight["slot"],1);assert_eq!(flight["id"],"");
    }
    server.replace_channel(ROUTE_TOKEN_ALICE,traffic);
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let (after_a,_) = r5_snapshot(&base,&output,"02_alice_authenticated_intake");
    assert!(after_a["recv"].get("1").is_none(),"authenticated closure must retire Alice recv1");
    fs::write(output.join("capacity-change.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "alice_retired_recv":before_a["recv"].as_object().unwrap().keys().filter(|k|after_a["recv"].get(k.as_str()).is_none()).collect::<Vec<_>>()
    })).unwrap()).unwrap();
    for round in 0..4 {
        retry_queue(&a,&relay); // Same queued operation; never send/enqueue it again.
        poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
        poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
        r5_snapshot(&base,&output,&format!("03_recovery_{round}"));
        let records=acceptance_queue_records(&a,"bob");
        let matches:Vec<_>=records.iter().filter(|r|r.body==body).collect();
        assert_eq!(matches.len(),1);
        let current=matches[0];
        assert!(current.msg_id==selected.msg_id && current.seq==selected.seq && current.peer==selected.peer && current.body==selected.body,"queued operation identity changed");
        if current.state.as_str()=="DELIVERED" {
            payload_once(&bo,&body);
            assert_eq!(delivered_count(&bo),6,"original five plus same operation exactly once");
            assert_eq!(delivered_count(&ao),5,"no new Bob enqueue in copied-state experiment");
            println!("NA0780_R5 same_operation_delivered=true duplicate_enqueue=false");
            return;
        }
    }
    panic!("same queued operation did not complete in bounded normal recovery");
}
fn freeze_fixture_clock(a:&Path,b:&Path) {
    let sa=integration_state(a,"bob");let sb=integration_state(b,"alice");
    let now=sa["last_boundary"].as_u64().unwrap().max(sb["last_boundary"].as_u64().unwrap());
    env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
}
#[test]
fn directional_receipt_loss(){
    use sha2::{Digest,Sha512};
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    // Resume the failed fixture's existing admission. This case never calls send.
    let af=base.join("loss.body");let body=fs::read(&af).expect("saved receipt-loss operation missing");
    assert!(queue_operation_present(&a,"bob",&af),"saved intended queue operation missing");
    freeze_fixture_clock(&a,&b);
    let hash=Sha512::digest(&body);let expected_hash=serde_json::json!(hash[..32].to_vec());
    let mut intended=None;
    for round in 0..8 {
        // Retry packs the SAME encrypted queue row when normal closure frees room.
        retry_queue(&a,&relay);
        let sa=integration_state(&a,"bob");
        let matches:Vec<_>=sa["flights"].as_object().unwrap().values().filter(|f|f["id"]!="" && f["body_hash"]==expected_hash).cloned().collect();
        assert!(matches.len()<=1,"duplicate intended ciphertext obligations");
        if let Some(app)=matches.into_iter().next() {
            let wire:Vec<u8>=app["wire"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
            let channel=server.drain_channel(ROUTE_TOKEN_BOB);
            let sent=channel.iter().any(|raw|raw==&wire);server.replace_channel(ROUTE_TOKEN_BOB,channel);
            assert!(sent,"fixture intended ciphertext was not actually pushed");
            intended=Some((app,wire));break;
        }
        let sb=integration_state(&b,"alice");
        let qa=server.drain_channel(ROUTE_TOKEN_ALICE);let qb=server.drain_channel(ROUTE_TOKEN_BOB);
        server.replace_channel(ROUTE_TOKEN_ALICE,qa.clone());server.replace_channel(ROUTE_TOKEN_BOB,qb.clone());
        paired_polls(&a,&b,&ao,&bo,&relay,1);
        let after_a=integration_state(&a,"bob");let after_b=integration_state(&b,"alice");
        println!("NA0780_TRANSITION {}",serde_json::json!({"phase":"closure_before_loss_injection","round":round,"a":retirement_history(&after_a),"b":retirement_history(&after_b)}));
        let next_a=server.drain_channel(ROUTE_TOKEN_ALICE);let next_b=server.drain_channel(ROUTE_TOKEN_BOB);
        let changed=sa!=after_a || sb!=after_b || qa!=next_a || qb!=next_b;
        server.replace_channel(ROUTE_TOKEN_ALICE,next_a);server.replace_channel(ROUTE_TOKEN_BOB,next_b);
        assert!(changed,"closure_precondition_no_progress: preserve queued operation; loss not injected");
    }
    let(app,wire)=intended.expect("closure_precondition_capacity_unresolved: normal closure did not admit saved operation; loss not injected");
    assert!(queue_operation_present(&a,"bob",&af),"intended queue row disappeared");
    let epoch=app["epoch"].as_u64().unwrap();let slot=app["slot"].as_u64().unwrap() as u32;let key=format!("{epoch}:{slot}");
    println!("NA0780_FAULT phase=intended_ciphertext_sent same_queued_operation=true");
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let delivered=server.drain_channel(ROUTE_TOKEN_ALICE);
    let receipt=delivered.iter().find(|r|r.len()==113 && r.starts_with(b"NDR1") && u64::from_be_bytes(r[21..29].try_into().unwrap())==epoch && u32::from_be_bytes(r[61..65].try_into().unwrap())==slot).expect("loss injection did not reach intended authenticated receipt").clone();
    let dropped=delivered.iter().filter(|r|**r==receipt).count();assert!(dropped>0);
    let retained:Vec<_>=delivered.into_iter().filter(|r|*r!=receipt).collect();assert!(!retained.contains(&receipt));
    server.replace_channel(ROUTE_TOKEN_ALICE,retained);
    save_exchange_record(&base.join("receipt-loss-injection.json"),&serde_json::json!({"intended_ciphertext_sent":true,"same_queued_operation":true,"intended_receipts_dropped":dropped,"epoch":epoch,"slot":slot,"source_manifest":env::var("NA0780_SOURCE_MANIFEST").unwrap()}));
    println!("NA0780_FAULT phase=intended_receipt_dropped count={dropped}");
    payload_once(&bo,&body);
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let after=integration_state(&a,"bob");assert!(after["flights"][&key]["wire"]==app["wire"],"loss changed or retired intended ciphertext");
    let retry=server.drain_channel(ROUTE_TOKEN_BOB);assert!(retry.contains(&wire),"recovery did not resend exact intended ciphertext");server.replace_channel(ROUTE_TOKEN_BOB,retry);
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let replay=server.drain_channel(ROUTE_TOKEN_ALICE);assert!(replay.contains(&receipt),"exact application receipt bytes were not replayed");server.replace_channel(ROUTE_TOKEN_ALICE,replay);
    paired_polls(&a,&b,&ao,&bo,&relay,2);
    payload_once(&bo,&body);assert!(queue_operation_present(&a,"bob",&af));
    assert!(integration_state(&a,"bob")["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    println!("NA0780_ACCEPT group=receipt_loss_exact_wire_response_and_delivery result=pass");
}
fn process_cut_case(point:&str){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);
    assert!(retirement_settled(&integration_state(&a,"bob"),&integration_state(&b,"alice")),"fault fixture must start at saved settled checkpoint");
    let af=base.join(format!("{point}.body"));let bytes=format!("process cut {point} payload");fs::write(&af,bytes.as_bytes()).unwrap();
    let output=if point=="after_prepare_commit" {
        qsc_cfg_cmd(&a).env("QSC_NA0780_CUT",point).args(["send","--transport","relay","--relay",&relay,"--to","bob","--file",af.to_str().unwrap()]).output().unwrap()
    }else{
        send_msg(&a,&relay,"bob",&af);
        qsc_cfg_cmd(&b).env("QSC_NA0780_CUT",point).args(["receive","--transport","relay","--relay",&relay,"--mailbox",ROUTE_TOKEN_BOB,"--from","alice","--max","64","--out",bo.to_str().unwrap()]).output().unwrap()
    };
    assert_eq!(output.status.code(),Some(86),"process cut not reached");
    let committed=integration_state(if point=="after_prepare_commit"{&a}else{&b},if point=="after_prepare_commit"{"bob"}else{"alice"});
    assert!(!committed[if point=="after_prepare_commit"{"flights"}else{"events"}].as_object().unwrap().is_empty(),"recoverable obligation absent after cut");
    paired_polls(&a,&b,&ao,&bo,&relay,3);
    payload_once(&bo,bytes.as_bytes());
    assert!(integration_state(&b,"alice")["events"].as_object().unwrap().is_empty());
    assert!(integration_state(&a,"bob")["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    println!("NA0780_ACCEPT group={point}_process_recovery result=pass");
}
#[test]fn directional_send_cut(){process_cut_case("after_prepare_commit");}
#[test]fn directional_receive_cut(){process_cut_case("after_receive_commit");}
#[test]fn directional_projection_cut(){process_cut_case("after_timeline_projection");}
#[test]
fn directional_offline_capacity(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);
    assert!(retirement_settled(&integration_state(&a,"bob"),&integration_state(&b,"alice")),"fault fixture must start at saved settled checkpoint");
    let mut accepted=Vec::new();let mut blocked=None;
    for n in 0..20 {
        let bytes=format!("offline capacity {n}");let af=base.join("offline.body");fs::write(&af,bytes.as_bytes()).unwrap();
        let before=integration_state(&a,"bob");
        let out=qsc_cfg_cmd(&a).args(["send","--transport","relay","--relay",&relay,"--to","bob","--file",af.to_str().unwrap()]).output().unwrap();
        if out.status.success(){accepted.push(bytes);}else{
            let after=integration_state(&a,"bob");assert!(before==after,"capacity refusal changed authoritative transaction");
            blocked=Some(bytes);break;
        }
    }
    assert!(blocked.is_some(),"offline admission did not reach its bound");assert!(!accepted.is_empty(),"no admitted offline work");
    let held=integration_state(&a,"bob");
    assert!(!held["flights"].as_object().unwrap().is_empty());
    // No receiver ran during all preparations; transport acceptance is not completion.
    for bytes in &accepted {assert!(!fs::read_dir(&bo).unwrap().filter_map(Result::ok).any(|e|fs::read(e.path()).ok().as_deref()==Some(bytes.as_bytes())));}
    paired_polls(&a,&b,&ao,&bo,&relay,4);
    let retry=qsc_cfg_cmd(&a).args(["outbox","retry","--relay",&relay]).output().unwrap();assert!(retry.status.success(),"capacity queue retry failed");
    paired_polls(&a,&b,&ao,&bo,&relay,3);
    for bytes in &accepted {payload_once(&bo,bytes.as_bytes());}payload_once(&bo,blocked.as_ref().unwrap().as_bytes());
    assert!(integration_state(&a,"bob")["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    println!("NA0780_ACCEPT group=offline_capacity_retention_retry_delivery admitted={} result=pass",accepted.len());
}
#[test]
fn directional_negative_admission(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);
    assert!(retirement_settled(&integration_state(&a,"bob"),&integration_state(&b,"alice")),"fault fixture must start at saved settled checkpoint");
    let af=base.join("negative.body");fs::write(&af,b"legitimate after hostile wire").unwrap();send_msg(&a,&relay,"bob",&af);
    let original=server.drain_channel(ROUTE_TOKEN_BOB);
    let raw=original.iter().find(|x|!x.starts_with(b"NDR1")).unwrap();
    let before=integration_state(&b,"alice");
    let mut tampered=raw.clone();*tampered.last_mut().unwrap()^=1;
    for bad in [&tampered[..],&raw[..raw.len()-1],b"unsupported candidate frame"] {
        assert!(qsc::na0780_test_receive_probe("alice",bad).is_err());assert!(integration_state(&b,"alice")==before,"rejected frame changed current transaction");
    }
    let generation=before["generation"].as_u64().unwrap();
    assert_eq!(qsc::na0780_test_stale_generation("alice",generation-1),Err("directional_stale_generation"));
    assert!(integration_state(&b,"alice")==before);
    // Exercise rejection classification through the actual receive loop as well
    // as the direct no-mutation probes above, then admit the untouched valid frame.
    let mut mixed=vec![tampered,raw[..raw.len()-1].to_vec(),b"unsupported candidate frame".to_vec()];
    mixed.extend(original);server.replace_channel(ROUTE_TOKEN_BOB,mixed);paired_polls(&a,&b,&ao,&bo,&relay,3);
    payload_once(&bo,&fs::read(&af).unwrap());
    println!("NA0780_ACCEPT group=hostile_wire_stale_generation_no_mutation_legitimate_delivery result=pass");
}
#[test]
fn directional_checkpoint_counters(){
    let(_,a,b,_,_)=retained_paths();
    for(cfg,peer,label)in [(&a,"bob","A"),(&b,"alice","B")] {
        let s=integration_state(cfg,peer);
        let epochs=|name:&str|s[name].as_object().unwrap().iter().map(|(id,e)|serde_json::json!({"id":id,"next":e["next"],"prefix":e["prefix"],"confirmed":e["confirmed"],"terminal":e["terminal"]})).collect::<Vec<_>>();
        println!("NA0780_STATE {}",serde_json::json!({"peer":label,"seq":s["core"]["seq"],"owner":s["core"]["owner"],"role":s["core"]["role"],"local_consumed":s["core"]["local_consumed_prefix"],"peer_selected":s["core"]["peer_selected_prefix"],"local_targets":s["core"]["local"].as_object().unwrap().len(),"peer_targets":s["core"]["peer"].as_object().unwrap().len(),"send":epochs("send"),"recv":epochs("recv"),"flights":s["flights"].as_object().unwrap().values().map(|f|serde_json::json!({"epoch":f["epoch"],"slot":f["slot"],"app":f["id"]!=""})).collect::<Vec<_>>(),"demand":s["demand"],"last_boundary":s["last_boundary"]}));
    }
}
#[test]
fn directional_second_order_finish(){
    let(_,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    // Continue the timed-out case's exact saved applications; never enqueue them again.
    paired_polls(&a,&b,&ao,&bo,&relay,1);
    payload_once(&bo,b"second order A typed application");payload_once(&ao,b"second order B typed application");
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    assert!(sa["flights"].as_object().unwrap().values().all(|f|f["id"]==""));assert!(sb["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
    assert!(sa["core"]["root"]==sb["core"]["root"]);
    println!("NA0780_ACCEPT group=second_order_saved_payloads_receipts_roots result=pass");
}
fn retry_queue(cfg:&Path,relay:&str){
    let out=qsc_cfg_cmd(cfg).args(["outbox","retry","--relay",relay]).output().unwrap();
    assert!(out.status.success(),"queue drain failed");
}
fn send_or_capacity(cfg:&Path,relay:&str,peer:&str,file:&Path){
    let out=qsc_cfg_cmd(cfg).args(["send","--transport","relay","--relay",relay,"--to",peer,"--file",file.to_str().unwrap()]).output().unwrap();
    if !out.status.success(){let text=output_text(&out);assert!(text.contains("RECEIPT_CONTEXT_CAPACITY") || text.contains("SEND_WINDOW") || text.contains("WAIT_TURN"),"unexpected send refusal");}
}
#[test]
fn directional_target_retirement_resume(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let start_a=integration_state(&a,"bob")["core"]["local_consumed_prefix"].as_u64().unwrap();let start_b=integration_state(&b,"alice")["core"]["local_consumed_prefix"].as_u64().unwrap();
    fs::write(base.join("retirement-goal.json"),serde_json::to_vec(&serde_json::json!({"a":start_a+5,"b":start_b+5})).unwrap()).unwrap();
    // Previous failure is visible capacity backpressure: A's retained ordinary wire
    // carries the final closure that B needs before admitting its queued boundary.
    paired_polls(&a,&b,&ao,&bo,&relay,1);retry_queue(&b,&relay);paired_polls(&a,&b,&ao,&bo,&relay,1);
    for name in ["target-a.body","target-b.body"] {
        let body=fs::read(base.join(name)).unwrap();payload_once(if name=="target-a.body"{&bo}else{&ao},&body);
    }
    for cycle in 0..10 {
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        if sa["core"]["local_consumed_prefix"].as_u64().unwrap()>=start_a+5 && sb["core"]["local_consumed_prefix"].as_u64().unwrap()>=start_b+5 {
            assert!(sa["core"]["root"]==sb["core"]["root"]);
            println!("NA0780_ACCEPT group=sustained_retirement_resume new_each=5 result=pass");return;
        }
        let now=sa["last_boundary"].as_u64().unwrap().max(sb["last_boundary"].as_u64().unwrap())+901;env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
        let af=base.join("target-next-a.body");let bf=base.join("target-next-b.body");fs::write(&af,format!("target continuation A {cycle}")).unwrap();fs::write(&bf,format!("target continuation B {cycle}")).unwrap();
        send_or_capacity(&a,&relay,"bob",&af);send_or_capacity(&b,&relay,"alice",&bf);
        paired_polls(&a,&b,&ao,&bo,&relay,1);retry_queue(&a,&relay);retry_queue(&b,&relay);paired_polls(&a,&b,&ao,&bo,&relay,1);
        payload_once(&bo,&fs::read(&af).unwrap());payload_once(&ao,&fs::read(&bf).unwrap());
        for(cfg,peer) in [(&a,"bob"),(&b,"alice")] {
            let s=integration_state(cfg,peer);assert!(s["send"].as_object().unwrap().len()+s["recv"].as_object().unwrap().len()<=3);assert!(s["core"]["local"].as_object().unwrap().len()<=1 && s["core"]["peer"].as_object().unwrap().len()<=1);assert!(s["flights"].as_object().unwrap().values().all(|f|f["id"]==""));
        }
    }
    panic!("target retirement bound not reached");
}
fn settle_target(a:&Path,b:&Path,ao:&Path,bo:&Path,relay:&str,want:Option<(&[u8],&[u8])>){
    for _ in 0..4 {
        paired_polls(a,b,ao,bo,relay,1);retry_queue(a,relay);retry_queue(b,relay);
        let sa=integration_state(a,"bob");let sb=integration_state(b,"alice");
        let has=|out:&Path,body:&[u8]|fs::read_dir(out).unwrap().filter_map(Result::ok).any(|e|fs::read(e.path()).ok().as_deref()==Some(body));
        if sa["flights"].as_object().unwrap().values().all(|f|f["id"]=="") && sb["flights"].as_object().unwrap().values().all(|f|f["id"]=="") && sa["core"]["root"]==sb["core"]["root"] && want.is_none_or(|(ab,bb)|has(bo,ab)&&has(ao,bb)){return;}
    }
    panic!("target payload/receipt completion still pending at bounded drain");
}
#[test]
fn directional_target_step(){
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let goal:serde_json::Value=serde_json::from_slice(&fs::read(base.join("retirement-goal.json")).unwrap()).unwrap();
    let cursor_path=base.join("retirement-step.json");
    let mut step:serde_json::Value=if cursor_path.exists(){serde_json::from_slice(&fs::read(&cursor_path).unwrap()).unwrap()}else{serde_json::json!({"next":0,"pending":false})};
    let mut n=step["next"].as_u64().unwrap();
    let af=base.join("step-a.body");let bf=base.join("step-b.body");
    if step["pending"]==true {
        // A case timeout can interrupt before either real CLI call enqueues. The
        // fixture cursor is intent, not admission evidence. Query each encrypted
        // queue in a fresh authenticated process; enqueue only a missing operation.
        ensure_test_operation(&a,"bob",&af,&relay);ensure_test_operation(&b,"alice",&bf,&relay);
        let ab=fs::read(&af).unwrap();let bb=fs::read(&bf).unwrap();settle_target(&a,&b,&ao,&bo,&relay,Some((&ab,&bb)));payload_once(&bo,&ab);payload_once(&ao,&bb);n+=1;
        step=serde_json::json!({"next":n,"pending":false});fs::write(&cursor_path,serde_json::to_vec(&step).unwrap()).unwrap();
    }else{settle_target(&a,&b,&ao,&bo,&relay,None);}
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    if sa["core"]["local_consumed_prefix"].as_u64().unwrap()<goal["a"].as_u64().unwrap() || sb["core"]["local_consumed_prefix"].as_u64().unwrap()<goal["b"].as_u64().unwrap(){
        let now=sa["last_boundary"].as_u64().unwrap().max(sb["last_boundary"].as_u64().unwrap())+901;env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
        let ab=format!("bounded target step A {n}");let bb=format!("bounded target step B {n}");fs::write(&af,ab.as_bytes()).unwrap();fs::write(&bf,bb.as_bytes()).unwrap();
        // Persist the fixture cursor before calls so a timed-out step can drain its
        // admitted operations. It never changes protocol state or target goals.
        fs::write(&cursor_path,serde_json::to_vec(&serde_json::json!({"next":n,"pending":true})).unwrap()).unwrap();
        send_or_capacity(&a,&relay,"bob",&af);send_or_capacity(&b,&relay,"alice",&bf);
        settle_target(&a,&b,&ao,&bo,&relay,Some((ab.as_bytes(),bb.as_bytes())));payload_once(&bo,ab.as_bytes());payload_once(&ao,bb.as_bytes());
        fs::write(&cursor_path,serde_json::to_vec(&serde_json::json!({"next":n+1,"pending":false})).unwrap()).unwrap();
    }
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    for s in [&sa,&sb] {assert!(s["send"].as_object().unwrap().len()+s["recv"].as_object().unwrap().len()<=3);assert!(s["core"]["local"].as_object().unwrap().len()<=1&&s["core"]["peer"].as_object().unwrap().len()<=1);}
    let ca=sa["core"]["local_consumed_prefix"].as_u64().unwrap();let cb=sb["core"]["local_consumed_prefix"].as_u64().unwrap();
    println!("NA0780_ACCEPT group=target_step_payloads_receipts_bounds a_consumed={ca} b_consumed={cb} result=pass");
    if ca>=goal["a"].as_u64().unwrap()&&cb>=goal["b"].as_u64().unwrap(){
        assert!(sa["recv_floor"].is_number()&&sb["recv_floor"].is_number());
        println!("NA0780_ACCEPT group=sustained_target_retirement new_each=5 result=pass");
    }
}
#[test]
fn directional_queue_worker(){
    let Some(cfg)=env::var_os("NA0780_WORKER_CFG")else{return;};
    let cfg=PathBuf::from(cfg);let peer=env::var("NA0780_WORKER_PEER").unwrap();let file=PathBuf::from(env::var_os("NA0780_WORKER_BODY").unwrap());
    env::set_var("QSC_CONFIG_DIR",&cfg);qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let body=fs::read(&file).unwrap();let present=qsc::na0780_test_queue_contains(&peer,&body).unwrap();
    println!("NA0780_QUEUE present={present}");
    if !present && env::var_os("NA0780_WORKER_RECOVER").is_some(){send_or_capacity(&cfg,&env::var("NA0780_WORKER_RELAY").unwrap(),&peer,&file);}
}
fn ensure_test_operation(cfg:&Path,peer:&str,file:&Path,relay:&str){
    let out=std::process::Command::new(env::current_exe().unwrap()).args(["--exact","directional_queue_worker","--nocapture"])
        .env("NA0780_WORKER_CFG",cfg).env("NA0780_WORKER_PEER",peer).env("NA0780_WORKER_BODY",file)
        .env("NA0780_WORKER_RELAY",relay).env("NA0780_WORKER_RECOVER","1").output().unwrap();
    assert!(out.status.success(),"test-operation queue recovery failed");
}

// Read-only retirement diagnosis: use a copied checkpoint and normal vault unlock.
// Only counters and equality predicates leave the authenticated fixture.
#[test]
fn directional_retirement_snapshot() {
    let (base,a,b,ao,bo)=retained_paths();
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    let goal:serde_json::Value=serde_json::from_slice(&fs::read(base.join("retirement-goal.json")).unwrap()).unwrap();
    let cursor:serde_json::Value=serde_json::from_slice(&fs::read(base.join("retirement-step.json")).unwrap()).unwrap();
    for (label,s) in [("A",&sa),("B",&sb)] {
        let epochs=|name:&str|s[name].as_object().unwrap().iter().map(|(id,e)|serde_json::json!({"epoch":id,"next":e["next"],"prefix":e["prefix"],"confirmed":e["confirmed"],"terminal":e["terminal"],"holes":e["holes"]})).collect::<Vec<_>>();
        let ids=|name:&str|s["core"][name].as_object().unwrap().keys().cloned().collect::<Vec<_>>();
        println!("NA0780_RETIREMENT {}",serde_json::json!({"endpoint":label,"seq":s["core"]["seq"],"role":s["core"]["role"],"owner":s["core"]["owner"],"local_consumed":s["core"]["local_consumed_prefix"],"peer_selected":s["core"]["peer_selected_prefix"],"local_target_ids":ids("local"),"peer_target_ids":ids("peer"),"send":epochs("send"),"recv":epochs("recv"),"send_floor":s["send_floor"],"recv_floor":s["recv_floor"],"flights":s["flights"].as_object().unwrap().values().map(|f|serde_json::json!({"epoch":f["epoch"],"slot":f["slot"],"application":f["id"]!="","transport_accepted":f["accepted"]})).collect::<Vec<_>>(),"receipts":s["dispositions"].as_object().unwrap().iter().map(|(slot,d)|serde_json::json!({"slot":slot,"response_pending":d["response_pending"]})).collect::<Vec<_>>(),"event_count":s["events"].as_object().unwrap().len(),"completion_count":s["completed"].as_object().unwrap().len(),"demand":s["demand"],"request_sent":s["request_sent"],"since_boundary":s["since_boundary"],"last_boundary":s["last_boundary"]}));
    }
    let mut delivered=Vec::new();
    for (label,cfg,peer,file,out) in [("A",&a,"bob",base.join("step-a.body"),&bo),("B",&b,"alice",base.join("step-b.body"),&ao)] {
        let result=std::process::Command::new(env::current_exe().unwrap()).args(["--exact","directional_queue_worker","--nocapture"])
            .env("NA0780_WORKER_CFG",cfg).env("NA0780_WORKER_PEER",peer).env("NA0780_WORKER_BODY",&file).env_remove("NA0780_WORKER_RECOVER").output().unwrap();
        assert!(result.status.success(),"authenticated queue snapshot failed");
        let present=String::from_utf8_lossy(&result.stdout).contains("NA0780_QUEUE present=true");
        let body=fs::read(file).unwrap();let count=fs::read_dir(out).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(body.as_slice())).count();
        delivered.push(serde_json::json!({"sender":label,"queued":present,"exact_receiver_files":count}));
    }
    let target_pass=sa["core"]["local_consumed_prefix"].as_u64().unwrap()>=goal["a"].as_u64().unwrap() && sb["core"]["local_consumed_prefix"].as_u64().unwrap()>=goal["b"].as_u64().unwrap();
    println!("NA0780_RETIREMENT_SUMMARY {}",serde_json::json!({"goal":goal,"cursor":cursor,"roots_equal":sa["core"]["root"]==sb["core"]["root"],"target_assertion":target_pass,"receiver_floor_assertion":sa["recv_floor"].is_number()&&sb["recv_floor"].is_number(),"current_operations":delivered}));
}

// The acceptance runner invokes exactly one exchange per case. Its terminal record
// is committed before any next exchange is admitted; prior fixture cases stay intact.
fn retirement_history(s:&serde_json::Value)->serde_json::Value {
    let core=&s["core"]; let local=core["local"].as_object().unwrap();let peer=core["peer"].as_object().unwrap();
    let consumed=core["local_consumed_prefix"].as_u64().unwrap();let selected=core["peer_selected_prefix"].as_u64().unwrap();
    assert!(local.len()<=1 && peer.len()<=1,"target storage bound");
    assert!(local.keys().all(|id|id.parse::<u64>()==Ok(consumed+1)),"retired local target retained or target gap");
    assert!(peer.keys().all(|id|id.parse::<u64>()==Ok(selected+1)),"spent peer target retained or target gap");
    assert_eq!(core["local_next"].as_u64().unwrap(),consumed+local.len() as u64,"local allocation history");
    assert_eq!(core["peer_max"].as_u64().unwrap(),selected+peer.len() as u64,"peer advertisement history");
    let sends=s["send"].as_object().unwrap();let recvs=s["recv"].as_object().unwrap();
    assert!(sends.len()+recvs.len()<=3,"receipt context bound");
    for (name,floor) in [("send","send_floor"),("recv","recv_floor")] {
        for (g,e) in s[name].as_object().unwrap() {
            let g=g.parse::<u64>().unwrap();assert!(s[floor].as_u64().is_none_or(|f|g>f),"closed receipt context retained");
            let next=e["next"].as_u64().unwrap();let prefix=e["prefix"].as_u64().unwrap();let confirmed=e["confirmed"].as_u64().unwrap();
            assert!(confirmed<=prefix && prefix<=next,"receipt prefix ordering");
            assert!(e["terminal"].as_u64().is_none_or(|n|n>=next),"receipt terminal inconsistent");
        }
    }
    let flights=s["flights"].as_object().unwrap();let dispositions=s["dispositions"].as_object().unwrap();
    assert!(flights.values().filter(|f|f["id"]!="").count()<=64 && flights.values().filter(|f|f["id"]=="").count()<=1,"flight count bound");
    for (g,e) in sends {
        let prefix=e["prefix"].as_u64().unwrap();let next=e["next"].as_u64().unwrap();
        assert!(next-prefix<=16,"send window bound");
        for slot in prefix..next {
            let key=format!("{g}:{slot}");let acked=e["holes"].as_array().unwrap().iter().any(|n|n.as_u64()==Some(slot));
            assert_ne!(flights.contains_key(&key),acked,"unaccounted outgoing obligation or double credit");
        }
    }
    for f in flights.values() {
        let g=f["epoch"].as_u64().unwrap().to_string();let e=sends.get(&g).expect("flight verifier missing");
        let slot=f["slot"].as_u64().unwrap();assert!(slot>=e["prefix"].as_u64().unwrap()&&slot<e["next"].as_u64().unwrap(),"flight outside outstanding interval");
    }
    for (key,d) in dispositions {
        let (g,n)=key.split_once(':').unwrap();let n=n.parse::<u64>().unwrap();let e=recvs.get(g).expect("receipt sealer missing");
        assert!(n>=e["confirmed"].as_u64().unwrap(),"confirmed receipt witness retained");
        assert!(n<e["prefix"].as_u64().unwrap()||e["holes"].as_array().unwrap().iter().any(|v|v.as_u64()==Some(n)),"receipt lacks admitted slot");
        assert_eq!(d["receipt"].as_array().unwrap().len(),113,"receipt length");
    }
    for (g,e) in recvs {
        for n in e["confirmed"].as_u64().unwrap()..e["prefix"].as_u64().unwrap() {
            assert!(dispositions.contains_key(&format!("{g}:{n}")),"unconfirmed admitted work lacks receipt witness");
        }
    }
    let epochs=core["recv"].as_object().unwrap();assert!(epochs.len()<=2,"receive core epoch bound");
    let skips:usize=epochs.values().map(|e|e["skipped"].as_object().unwrap().len()).sum();assert!(skips<=16,"skip bound");
    for (g,e) in epochs {
        assert!(s["recv_floor"].as_u64().is_none_or(|f|g.parse::<u64>().unwrap()>f),"closed core epoch retained");
        if e["terminal"].is_number(){for name in ["ec","pq"]{assert!(e[name].as_array().unwrap().iter().all(|v|v.as_u64()==Some(0)),"ended epoch chain retained");}}
    }
    let outgoing:usize=flights.values().map(|f|f["wire"].as_array().unwrap().len()).sum();
    let incoming:usize=dispositions.values().map(|d|d["receipt"].as_array().unwrap().len()).sum::<usize>()+s["events"].as_object().unwrap().values().map(|e|e["body"].as_array().unwrap().len()).sum::<usize>();
    let bytes=serde_json::to_vec(s).unwrap().len();
    assert!(outgoing<=4*1024*1024 && incoming<=4*1024*1024 && bytes<=16*1024*1024,"retained byte bounds");
    assert!(s["events"].as_object().unwrap().len()<=64 && s["completed"].as_object().unwrap().len()<=64,"projection count bounds");
    serde_json::json!({"consumed":consumed,"selected":selected,"local_targets":local.len(),"peer_targets":peer.len(),"send_floor":s["send_floor"],"recv_floor":s["recv_floor"],"contexts":sends.len()+recvs.len(),"core_epochs":epochs.len(),"skips":skips,"flights":flights.len(),"receipts":dispositions.len(),"outgoing_bytes":outgoing,"incoming_bytes":incoming,"record_bytes":bytes,"seq":core["seq"]})
}
fn retirement_settled(a:&serde_json::Value,b:&serde_json::Value)->bool {
    a["core"]["root"]==b["core"]["root"] && a["send_floor"]==b["recv_floor"] && b["send_floor"]==a["recv_floor"] &&
    [a,b].iter().all(|s|["flights","events","completed"].iter().all(|k|s[*k].as_object().unwrap().is_empty()) && s["dispositions"].as_object().unwrap().values().all(|d|d["response_pending"]==false))
}
fn save_exchange_record(path:&Path,value:&serde_json::Value) {
    use std::io::Write;
    let tmp=path.with_extension("new");let mut file=fs::File::create(&tmp).unwrap();
    file.write_all(&serde_json::to_vec_pretty(value).unwrap()).unwrap();file.sync_all().unwrap();fs::rename(tmp,path).unwrap();
}
#[test]
fn directional_target_exchange() {
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let cursor_path=base.join("retirement-step.json");let cursor:serde_json::Value=serde_json::from_slice(&fs::read(&cursor_path).unwrap()).unwrap();
    let n=cursor["next"].as_u64().unwrap();let af=base.join("step-a.body");let bf=base.join("step-b.body");
    let initial_a=integration_state(&a,"bob");let initial_b=integration_state(&b,"alice");
    let pending=cursor["pending"]==true;
    let now=initial_a["last_boundary"].as_u64().unwrap().max(initial_b["last_boundary"].as_u64().unwrap())+if pending {0}else{901};
    env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",now.to_string());
    if !pending {
        assert!(retirement_settled(&initial_a,&initial_b),"prior exchange not settled before admission");
        fs::write(&af,format!("bounded target step A {n}")).unwrap();fs::write(&bf,format!("bounded target step B {n}")).unwrap();
        save_exchange_record(&cursor_path,&serde_json::json!({"next":n,"pending":true}));
    }
    ensure_test_operation(&a,"bob",&af,&relay);ensure_test_operation(&b,"alice",&bf,&relay);
    let ab=fs::read(&af).unwrap();let bb=fs::read(&bf).unwrap();
    // Every round delivers retained traffic/receipts and retries capacity-blocked
    // queued work. Stop if a full round changes neither authoritative endpoint.
    for round in 0..12 {
        let before_a=integration_state(&a,"bob");let before_b=integration_state(&b,"alice");
        let relay_snapshot=|| {
            let qa=server.drain_channel(ROUTE_TOKEN_ALICE);let qb=server.drain_channel(ROUTE_TOKEN_BOB);
            server.replace_channel(ROUTE_TOKEN_ALICE,qa.clone());server.replace_channel(ROUTE_TOKEN_BOB,qb.clone());(qa,qb)
        };
        let before_relay=relay_snapshot();
        paired_polls(&a,&b,&ao,&bo,&relay,1);retry_queue(&a,&relay);retry_queue(&b,&relay);
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        let ha=retirement_history(&sa);let hb=retirement_history(&sb);
        println!("NA0780_TRANSITION {}",serde_json::json!({"step":n,"round":round,"a":ha,"b":hb,"roots_equal":sa["core"]["root"]==sb["core"]["root"]}));
        let has=|out:&Path,body:&[u8]|fs::read_dir(out).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(body)).count();
        if retirement_settled(&sa,&sb) && has(&bo,&ab)==1 && has(&ao,&bb)==1 {
            for (old,new) in [(&initial_a,&sa),(&initial_b,&sb)] {for floor in ["send_floor","recv_floor"] {assert!(new[floor].as_u64()>=old[floor].as_u64(),"retirement floor regressed");}}
            let record=serde_json::json!({"step":n,"status":"pass","payloads_exact_once":true,"protocol_flights_cleared":true,"closure_floors_matched":true,"a":ha,"b":hb,"source_manifest":env::var("NA0780_SOURCE_MANIFEST").unwrap()});
            save_exchange_record(&base.join(format!("exchange-{n:03}.json")),&record);
            save_exchange_record(&cursor_path,&serde_json::json!({"next":n+1,"pending":false}));
            println!("NA0780_ACCEPT group=retained_exchange step={n} result=pass");return;
        }
        assert!(sa!=before_a || sb!=before_b || relay_snapshot()!=before_relay,"no progress: unchanged endpoints and relay obligations after exact traffic/receipt/queue actions");
    }
    panic!("exchange incomplete after twelve transition rounds; preserve outstanding obligations");
}
#[test]
fn directional_retirement_verify() {
    let(base,a,b,_,_)=retained_paths();let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    let goal:serde_json::Value=serde_json::from_slice(&fs::read(base.join("retirement-goal.json")).unwrap()).unwrap();
    let ha=retirement_history(&sa);let hb=retirement_history(&sb);
    assert!(ha["consumed"].as_u64().unwrap()>=goal["a"].as_u64().unwrap() && hb["consumed"].as_u64().unwrap()>=goal["b"].as_u64().unwrap(),"seven target consumptions required per endpoint");
    assert!(retirement_settled(&sa,&sb),"outstanding receipt/projection/closure or root mismatch");
    assert_eq!(ha["consumed"],hb["selected"],"A consumption/peer selection history");assert_eq!(hb["consumed"],ha["selected"],"B consumption/peer selection history");
    assert!(sa["recv_floor"].as_u64().unwrap()>15 && sb["recv_floor"].as_u64().unwrap()>16,"receiver history did not retire beyond step6 checkpoint");
    let cursor:serde_json::Value=serde_json::from_slice(&fs::read(base.join("retirement-step.json")).unwrap()).unwrap();assert_eq!(cursor["pending"],false);
    let record=serde_json::json!({"status":"pass","goal":goal,"a":ha,"b":hb,"roots_equal":true,"history_and_storage_checked":true,"source_manifest":env::var("NA0780_SOURCE_MANIFEST").unwrap()});
    save_exchange_record(&base.join("retirement-verified.json"),&record);
    println!("NA0780_ACCEPT group=sustained_target_retirement_history_storage result=pass");
}

#[test]
fn directional_full_window_receipt_progress() {
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);
    let queued=base.join("offline.body");let blocked=fs::read(&queued).unwrap();assert!(queue_operation_present(&a,"bob",&queued));
    let before=integration_state(&a,"bob");let original=before["flights"].clone();let apps=original.as_object().unwrap().values().filter(|f|f["id"]!="").count();
    assert_eq!(apps,16,"saved full-window regression precondition");
    assert!(original.as_object().unwrap().values().all(|f|f["id"]!=""));
    let started=Instant::now();let no_receipt=recv_msg_drain(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    // Baseline runner5 fails at this same pre-pull call with SEND_WINDOW.
    assert!(no_receipt.status.success(),"full-window receive must reach bounded empty inbox");
    assert!(started.elapsed()<Duration::from_secs(60),"no-receipt receive did not return bounded waiting");
    assert!(output_text(&no_receipt).contains("directional_maintenance_waiting"),"maintenance deferral was not exercised");
    assert!(integration_state(&a,"bob")==before,"no-receipt deferral changed authoritative obligations");
    let sent=server.drain_channel(ROUTE_TOKEN_BOB);
    for f in original.as_object().unwrap().values(){let raw:Vec<u8>=f["wire"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();assert!(sent.contains(&raw),"deferral lost a saved ciphertext");}
    server.replace_channel(ROUTE_TOKEN_BOB,sent);
    println!("NA0780_ACCEPT group=full_window_no_receipt_bounded_exact_retention result=pass");
    let receive_all=qsc_cfg_cmd(&b).args(["receive","--transport","relay","--relay",&relay,"--mailbox",ROUTE_TOKEN_BOB,"--from","alice","--max","64","--out",bo.to_str().unwrap()]).output().unwrap();
    assert!(receive_all.status.success(),"peer could not authenticate retained full-window traffic");
    let inbound=server.drain_channel(ROUTE_TOKEN_ALICE);
    assert!(inbound.iter().filter(|r|r.starts_with(b"NDR1")).count()>=apps,"capacity-releasing receipts not produced");
    let held=integration_state(&a,"bob");
    let mut forged=inbound.iter().find(|r|r.starts_with(b"NDR1")).unwrap().clone();*forged.last_mut().unwrap()^=1;
    assert_eq!(qsc::na0780_test_receive_probe("bob",&forged),Err("RECEIPT_AUTH"));
    assert!(integration_state(&a,"bob")==held,"forged receipt released capacity or changed obligations");
    server.replace_channel(ROUTE_TOKEN_ALICE,inbound);
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let released=integration_state(&a,"bob");
    for key in original.as_object().unwrap().keys(){assert!(released["flights"].get(key).is_none(),"authenticated receipt did not clear original flight");}
    let maintenance:Vec<_>=released["flights"].as_object().unwrap().iter().filter(|(_,f)|f["id"]=="").map(|(key,f)|(key.clone(),f.clone())).collect();
    assert_eq!(maintenance.len(),1,"deferred maintenance was not retried after receipts");
    let (maintenance_key,maintenance_flight)=&maintenance[0];
    let exact:Vec<u8>=maintenance_flight["wire"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
    let outgoing=server.drain_channel(ROUTE_TOKEN_BOB);assert!(outgoing.contains(&exact),"deferred maintenance ciphertext not sent");server.replace_channel(ROUTE_TOKEN_BOB,outgoing);
    println!("NA0780_ACCEPT group=full_window_receipts_release_deferred_maintenance_sent result=pass");
    for round in 0..8 {
        let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
        let relay_snapshot=|| {
            let qa=server.drain_channel(ROUTE_TOKEN_ALICE);let qb=server.drain_channel(ROUTE_TOKEN_BOB);
            server.replace_channel(ROUTE_TOKEN_ALICE,qa.clone());server.replace_channel(ROUTE_TOKEN_BOB,qb.clone());(qa,qb)
        };
        let prior_relay=relay_snapshot();
        // Let the peer authenticate the deferred work before trying blocked app admission.
        paired_polls(&a,&b,&ao,&bo,&relay,1);retry_queue(&a,&relay);
        let aa=integration_state(&a,"bob");let bb=integration_state(&b,"alice");
        println!("NA0780_TRANSITION {}",serde_json::json!({"phase":"full_window_recovery","round":round,"a":retirement_history(&aa),"b":retirement_history(&bb)}));
        let delivered=fs::read_dir(&bo).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(blocked.as_slice())).count();
        if aa["flights"].get(maintenance_key).is_none() && aa["flights"].as_object().unwrap().values().all(|f|f["id"]=="") && delivered==1 {
            for n in 0..apps {payload_once(&bo,format!("offline capacity {n}").as_bytes());}
            payload_once(&bo,&blocked);assert!(queue_operation_present(&a,"bob",&queued));
            println!("NA0780_ACCEPT group=offline_capacity_saved_maintenance_and_queued_payload_complete result=pass");return;
        }
        assert!(aa!=sa || bb!=sb || relay_snapshot()!=prior_relay,"no recovery progress after deferred-work delivery");
    }
    panic!("offline saved recovery incomplete; preserve deferred maintenance and queued payload");
}
#[test]
fn directional_scheduler_fail_closed() {
    let(_,a,b,ao,_)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);let original=integration_state(&a,"bob");
    // Existing authentication and generation checks remain outside the narrow
    // scheduler SEND_WINDOW match. Neither malformed input may mutate state.
    assert!(qsc::na0780_test_receive_probe("bob",b"NDR1 malformed").is_err());
    assert_eq!(qsc::na0780_test_stale_generation("bob",original["generation"].as_u64().unwrap()-1),Err("directional_stale_generation"));
    assert!(integration_state(&a,"bob")==original);
    for (name,error) in [("profile","TRANSACTION_PROFILE"),("counter","COUNTER_OVERFLOW")] {
        let mut invalid=original.clone();
        if name=="profile" {invalid["version"]=serde_json::json!("invalid-test-profile");}
        else {
            invalid["core"]["send"]["next"]=serde_json::json!(u32::MAX);
            let epoch=invalid["core"]["send"]["id"].as_u64().unwrap().to_string();
            invalid["send"][&epoch]["next"]=serde_json::json!(u32::MAX);
        }
        let raw=serde_json::to_string(&invalid).unwrap();qsc::vault::secret_set("na0780_directional_transaction/bob",&raw).unwrap();
        let out=recv_msg_drain(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
        assert!(!out.status.success() && output_text(&out).contains(error),"unrelated scheduler/store failure was not propagated");
        assert!(qsc::vault::secret_get("na0780_directional_transaction/bob").unwrap().unwrap()==raw,"failed scheduling persisted staged state");
        assert!(server.drain_channel(ROUTE_TOKEN_BOB).is_empty(),"failure released ciphertext");
        qsc::vault::secret_set("na0780_directional_transaction/bob",&serde_json::to_string(&original).unwrap()).unwrap();
    }
    println!("NA0780_ACCEPT group=maintenance_unrelated_corrupt_state_counter_auth_generation_fail_closed result=pass");
}

#[test]
fn directional_receive_save_failure_recovery() {
    let(base,a,b,ao,bo)=retained_paths();let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    freeze_fixture_clock(&a,&b);
    let file=base.join("save-failure.body");let body=b"focused transaction save recovery";fs::write(&file,body).unwrap();
    send_msg(&a,&relay,"bob",&file);
    let sender=integration_state(&a,"bob");let flights:Vec<_>=sender["flights"].as_object().unwrap().values().filter(|f|f["id"]!="").cloned().collect();assert_eq!(flights.len(),1);
    let app=&flights[0];let slot=format!("{}:{}",app["epoch"].as_u64().unwrap(),app["slot"].as_u64().unwrap());
    let wire:Vec<u8>=app["wire"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
    let queued=server.drain_channel(ROUTE_TOKEN_BOB);assert!(queued.contains(&wire));server.replace_channel(ROUTE_TOKEN_BOB,queued);
    let failed=qsc_cfg_cmd(&b).env("QSC_NA0780_RECEIVE_SAVE_FAULT","1").args(["receive","--transport","relay","--relay",&relay,"--mailbox",ROUTE_TOKEN_BOB,"--from","alice","--max","64","--out",bo.to_str().unwrap()]).output().unwrap();
    let text=output_text(&failed);
    assert!(text.contains("event=directional_receive_save_fault") && text.contains("write_error=vault_write_failed") && text.contains("stored_state_unchanged=true"),"valid receive did not reach real atomic writer fault after flush");
    // Every nonempty relay ACK flush emits one of these terminal markers.
    assert!(["event=relay_ack", "event=ack_legacy_complete", "event=ack_failed", "event=ack_eligibility_violation"].iter().all(|marker|!text.contains(marker)),"failed receive attempted an ACK before durable disposition");
    println!("NA0780_ACCEPT group=failed_receive_no_premature_ack result=pass");
    let receiver=integration_state(&b,"alice");assert!(receiver["dispositions"].get(&slot).is_none(),"failed save admitted a disposition");assert!(receiver["events"].as_object().unwrap().is_empty());
    let outputs=fs::read_dir(&bo).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(body.as_slice())).count();assert_eq!(outputs,0);
    let response=server.drain_channel(ROUTE_TOKEN_ALICE);assert!(response.iter().all(|r|!r.starts_with(b"NDR1")),"receipt escaped before durable disposition");server.replace_channel(ROUTE_TOKEN_ALICE,response);
    assert!(integration_state(&a,"bob")["flights"][&slot]["wire"]==app["wire"],"failed receive lost sender recovery wire");
    if env::var_os("NA0780_EXPECT_MASKED_SAVE_ERROR").is_some() {
        assert!(failed.status.success() && text.contains("recv_skip_summary"),"baseline did not reproduce masked local write failure");
        println!("NA0780_ACCEPT group=reproduced_masked_valid_receive_save_failure result=pass");return;
    }
    assert!(!failed.status.success() && text.contains("event=error code=vault_write_failed"),"local write failure was hidden as ordinary non-admission");
    println!("NA0780_ACCEPT group=valid_receive_save_failure_reported_no_disposition_or_receipt result=pass");
    // The test relay pops on pull. Recovery must come from the real sender's
    // retained exact flight; it is not a production lease or power-loss claim.
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let retry=server.drain_channel(ROUTE_TOKEN_BOB);assert!(retry.contains(&wire),"sender did not recover exact failed frame");server.replace_channel(ROUTE_TOKEN_BOB,retry);
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    payload_once(&bo,body);
    let replies=server.drain_channel(ROUTE_TOKEN_ALICE);
    let receipt=replies.iter().find(|r|r.len()==113 && r.starts_with(b"NDR1") && u64::from_be_bytes(r[21..29].try_into().unwrap())==app["epoch"].as_u64().unwrap() && u32::from_be_bytes(r[61..65].try_into().unwrap()) as u64==app["slot"].as_u64().unwrap()).expect("recovered receive did not produce intended receipt").clone();
    // Lose this receipt once to verify the successful retry's durable witness.
    server.replace_channel(ROUTE_TOKEN_ALICE,replies.into_iter().filter(|r|r!=&receipt).collect());
    poll_candidate(&a,&relay,ROUTE_TOKEN_ALICE,"bob",&ao);poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let exact=server.drain_channel(ROUTE_TOKEN_ALICE);assert!(exact.contains(&receipt),"recovered disposition did not replay exact receipt");server.replace_channel(ROUTE_TOKEN_ALICE,exact);
    paired_polls(&a,&b,&ao,&bo,&relay,2);payload_once(&bo,body);
    assert!(integration_state(&a,"bob")["flights"].get(&slot).is_none());assert!(queue_operation_present(&a,"bob",&file));
    println!("NA0780_ACCEPT group=failed_save_retry_exactly_once_valid_receipt_recovery result=pass");
}

#[test]
fn directional_receipt_capacity_saved_recovery() {
    let(base,a,b,ao,bo)=retained_paths();freeze_fixture_clock(&a,&b);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let before=integration_state(&a,"bob");
    assert_eq!(before["send"].as_object().unwrap().len()+before["recv"].as_object().unwrap().len(),3);
    let app=before["flights"].as_object().unwrap().values().find(|f|f["id"]!="").unwrap().clone();
    let slot=format!("{}:{}",app["epoch"].as_u64().unwrap(),app["slot"].as_u64().unwrap());
    let wire:Vec<u8>=serde_json::from_value(app["wire"].clone()).unwrap();
    let call_a=||qsc_cfg_cmd(&a).args(["receive","--transport","relay","--relay",&relay,"--mailbox",ROUTE_TOKEN_ALICE,"--from","bob","--max","8","--out",ao.to_str().unwrap()]).output().unwrap();
    let empty=call_a();
    if env::var_os("NA0780_EXPECT_CAPACITY_BLOCK").is_some() {
        assert!(!empty.status.success() && output_text(&empty).contains("RECEIPT_CONTEXT_CAPACITY"));
        assert_eq!(integration_state(&a,"bob"),before);
        poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
        let controls=server.drain_channel(ROUTE_TOKEN_ALICE);assert!(!controls.is_empty());server.replace_channel(ROUTE_TOKEN_ALICE,controls);
        let blocked=call_a();assert!(!blocked.status.success() && output_text(&blocked).contains("RECEIPT_CONTEXT_CAPACITY"));
        assert_eq!(integration_state(&a,"bob"),before);
        println!("NA0780_ACCEPT group=receipt_context_preflush_blocks_peer_maintenance result=pass");return;
    }
    assert!(empty.status.success(),"empty receive must defer capacity: {}",output_text(&empty));
    let waiting=integration_state(&a,"bob");assert_eq!(waiting["core"],before["core"]);assert_eq!(waiting["send"],before["send"]);assert_eq!(waiting["recv"],before["recv"]);assert_eq!(waiting["demand"],before["demand"]);assert_eq!(waiting["flights"][&slot]["wire"],app["wire"]);
    let retry=server.drain_channel(ROUTE_TOKEN_BOB);assert!(retry.contains(&wire));server.replace_channel(ROUTE_TOKEN_BOB,retry);
    // Bob processes the exact outstanding application again and retries his
    // saved maintenance. Alice must admit its closure before preparing refresh.
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let replies=server.drain_channel(ROUTE_TOKEN_ALICE);
    assert!(replies.iter().any(|r|r.len()==113 && r.starts_with(b"NDR1") && u64::from_be_bytes(r[21..29].try_into().unwrap())==26 && u32::from_be_bytes(r[61..65].try_into().unwrap())==2));
    server.replace_channel(ROUTE_TOKEN_ALICE,replies);
    let recovered=call_a();assert!(recovered.status.success(),"closure receive failed: {}",output_text(&recovered));
    let after=integration_state(&a,"bob");assert!(after["recv"].get("25").is_none(),"old receive context not retired by closure");assert!(after["flights"].get(&slot).is_none(),"original receipt not committed");
    payload_once(&bo,&fs::read(base.join("save-failure.body")).unwrap());
    assert!(queue_operation_present(&a,"bob",&base.join("save-failure.body")));
    println!("NA0780_ACCEPT group=saved_context_closure_exact_retry_single_delivery_receipt_recovery result=pass");
}

// Each case starts from a fresh COPY of the settled real-client checkpoint.
// State access below reopens the encrypted store. No test rewinds live state.
fn admission_pair()->(PathBuf,PathBuf,PathBuf,PathBuf,PathBuf,&'static str,&'static str,&'static str){
    let(base,a,b,ao,bo)=retained_paths();freeze_fixture_clock(&a,&b);
    let sa=integration_state(&a,"bob");
    if sa["core"]["owner"]==sa["core"]["role"] {(base,a,b,ao,bo,"bob","alice",ROUTE_TOKEN_BOB)}
    else {(base,b,a,bo,ao,"alice","bob",ROUTE_TOKEN_ALICE)}
}
fn saved_prepare(cfg:&Path,peer:&str,id:&str,body:&[u8],now:u64,adv:bool,control:bool)->Vec<u8>{
    integration_state(cfg,peer);
    qsc::na0780_test_prepare(peer,id,body,now,adv,control).expect("actual locked prepare")
}
fn wire_slot(raw:&[u8])->String {
    format!("{}:{}",u64::from_be_bytes(raw[22..30].try_into().unwrap()),u32::from_be_bytes(raw[30..34].try_into().unwrap()))
}
fn saved_accept(cfg:&Path,peer:&str,raw:&[u8])->Vec<u8>{
    integration_state(cfg,peer);qsc::na0780_test_receive_probe(peer,raw).expect("actual persisted receive");
    serde_json::from_value(integration_state(cfg,peer)["dispositions"][wire_slot(raw)]["receipt"].clone()).unwrap()
}
fn saved_ack(cfg:&Path,peer:&str,ack:&[u8]){
    integration_state(cfg,peer);qsc::na0780_test_receive_probe(peer,ack).expect("actual persisted protocol receipt");
}
fn saved_reject(cfg:&Path,peer:&str,raw:&[u8],error:&'static str){
    let before=integration_state(cfg,peer);
    let key=format!("na0780_directional_transaction_v2/{peer}");
    let encoded=qsc::vault::secret_get(&key).unwrap().unwrap();
    assert_eq!(qsc::na0780_test_receive_probe(peer,raw),Err(error),"exact admission refusal class");
    assert_eq!(qsc::vault::secret_get(&key).unwrap().unwrap(),encoded,"rejection changed serialized authoritative state");
    assert!(integration_state(cfg,peer)==before,"rejection changed reloaded state");
}
fn assert_flight(cfg:&Path,peer:&str,raw:&[u8]){
    let state=integration_state(cfg,peer);
    let saved:Vec<u8>=serde_json::from_value(state["flights"][wire_slot(raw)]["wire"].clone()).unwrap();
    assert_eq!(saved,raw,"exact retry flight changed");
}
// Complete the saved, owed refreshes through the same locked preparation,
// authenticated receive and exact protocol-receipt paths as client work. No
// fixture flags/ownership/counters are assigned, and no old snapshot is restored.
fn complete_ordinary_fixture(s:&Path,r:&Path,sp:&str,rp:&str){
    let a=integration_state(s,sp);let b=integration_state(r,rp);
    let now=a["last_boundary"].as_u64().unwrap().max(b["last_boundary"].as_u64().unwrap());
    let summary=|x:&serde_json::Value|serde_json::json!({"seq":x["core"]["seq"],"role":x["core"]["role"],"owner":x["core"]["owner"],"demand":x["demand"],"cadence":x["since_boundary"],"contexts":x["send"].as_object().unwrap().len()+x["recv"].as_object().unwrap().len()});
    println!("NA0780_SETUP before {}",serde_json::json!({"sender":summary(&a),"receiver":summary(&b)}));
    // Both endpoints have measured saved demand. Each authenticated refresh
    // grants the other's turn; its final receipt promise releases the old context
    // before the next owed boundary is prepared.
    for (from,to,fp,tp) in [(s,r,sp,rp),(r,s,rp,sp)] {
        let before=integration_state(from,fp);
        assert_eq!(before["demand"],true,"fixture must actually owe refresh");
        assert_eq!(before["core"]["owner"],before["core"]["role"],"refresh requires actual ownership");
        assert!(before["send"].as_object().unwrap().len()+before["recv"].as_object().unwrap().len()<3,"close old contexts before next activation");
        let refresh=saved_prepare(from,fp,"",&[1],now,false,true);
        assert_eq!(refresh[4],1,"owed refresh must activate normally");
        let ack=saved_accept(to,tp,&refresh);assert_flight(from,fp,&refresh);saved_ack(from,fp,&ack);
        let closure=saved_prepare(from,fp,"",&[2],now,false,true);
        assert_eq!(closure[4],0,"closure itself must not grant a turn");
        let ack=saved_accept(to,tp,&closure);assert_flight(from,fp,&closure);saved_ack(from,fp,&ack);
    }
    let ready=integration_state(s,sp);let peer=integration_state(r,rp);
    assert_eq!(ready["demand"],false,"client refresh must clear demand");
    assert!(ready["since_boundary"].as_u64().unwrap()<4,"ordinary cadence precondition");
    assert!(!ready["core"]["send"].is_null(),"active send epoch required");
    assert_eq!(ready["core"]["owner"],ready["core"]["role"],"real lifecycle must return turn");
    assert!(ready["core"]["root"]==peer["core"]["root"],"authenticated endpoints must agree");
    assert!(ready["flights"].as_object().unwrap().is_empty(),"refresh/closure protocol receipts incomplete");
    assert!(ready["send"].as_object().unwrap().len()+ready["recv"].as_object().unwrap().len()<3);
    assert!(now.saturating_sub(ready["last_boundary"].as_u64().unwrap())<900);
    println!("NA0780_SETUP after {}",serde_json::json!({"sender":summary(&ready),"receiver":summary(&peer)}));
}
fn delayed_epoch_case(kind:&str){
    let(_,s,r,so,ro,sp,rp,route)=admission_pair();
    if kind=="application" || kind=="control" {complete_ordinary_fixture(&s,&r,sp,rp);}
    let initial=integration_state(&s,sp);let now=initial["last_boundary"].as_u64().unwrap();
    let body=format!("delayed old epoch {kind}");
    let old=saved_prepare(&s,sp,if kind=="application"{"delayed-app"}else{""},
        if kind=="application"{body.as_bytes()}else if kind=="control"{&[0]}else{&[]},now,kind=="adv",kind!="application");
    assert_eq!(old[4],0,"held frame must use existing epoch");
    let old_epoch=u64::from_be_bytes(old[22..30].try_into().unwrap());
    let after_old=integration_state(&s,sp);
    let terminal=qsc::na0780_test_hostile_wire(&after_old["core"].to_string(),"terminal",0).unwrap();
    let boundary=saved_prepare(&s,sp,"after-held-boundary",b"boundary overtakes held old traffic",now+900,false,false);
    assert_eq!(boundary[4],1,"actual authorized boundary required");
    let boundary_ack=saved_accept(&r,rp,&boundary);
    let installed=integration_state(&r,rp);
    assert!(installed["core"]["recv"].get(old_epoch.to_string()).is_some(),"missing old slot must retain its epoch");
    saved_reject(&r,rp,&terminal,"TERMINAL_BOUND");assert_flight(&s,sp,&old);assert_flight(&s,sp,&boundary);
    let ack=saved_accept(&r,rp,&old);let admitted=integration_state(&r,rp);
    for field in ["root","seq","owner","local_consumed_prefix","peer_selected_prefix"] {
        assert!(admitted["core"][field]==installed["core"][field],"delayed traffic changed unauthorized root/turn/target state");
    }
    if kind=="control" {assert_eq!(admitted["demand"],true,"authenticated old refresh must request work");}
    if kind=="adv" {assert_eq!(admitted["core"]["peer"].as_object().unwrap().len(),1,"retained authenticated ADV must install eligible target");}
    assert_eq!(saved_accept(&r,rp,&old),ack,"delayed duplicate must replay exact saved receipt");
    saved_ack(&s,sp,&boundary_ack);saved_ack(&s,sp,&ack);
    // Actual sender promise is available only after both exact protocol receipts.
    let closure=saved_prepare(&s,sp,"",&[2],now+900,false,true);
    let closure_ack=saved_accept(&r,rp,&closure);saved_ack(&s,sp,&closure_ack);
    assert!(integration_state(&r,rp)["recv_floor"].as_u64().unwrap()>=old_epoch,"authenticated final promise must close old epoch");
    saved_reject(&r,rp,&old,"CLOSED_REPLAY");
    // Exercise the real relay receive entry point on closed replay followed by
    // new legitimate ciphertext; it must project prior app events once and progress.
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let later=saved_prepare(&s,sp,"after-delayed-reject",b"legitimate after delayed rejection",now+900,false,false);
    assert_flight(&s,sp,&later);post_raw(&relay,route,old);post_raw(&relay,route,later);
    poll_candidate(&r,&relay,route,rp,&ro);
    payload_once(&ro,b"legitimate after delayed rejection");
    if kind=="application" {payload_once(&ro,body.as_bytes());}
    let _=so;
    println!("NA0780_ACCEPT group=delayed_{kind}_retained_terminal_exact_receipt_closed_replay_progress result=pass");
}
#[test] fn directional_delayed_application(){delayed_epoch_case("application");}
#[test] fn directional_delayed_control(){delayed_epoch_case("control");}
#[test] fn directional_delayed_adv(){delayed_epoch_case("adv");}

#[test]
fn directional_skip_bound_admission(){
    let(_,s,r,_,ro,sp,rp,route)=admission_pair();let initial=integration_state(&s,sp);
    let bad=qsc::na0780_test_hostile_wire(&initial["core"].to_string(),"gap",0).unwrap();
    saved_reject(&r,rp,&bad,"SKIP_BOUND");
    let raw=saved_prepare(&s,sp,"after-skip-bound",b"legitimate after skip refusal",initial["last_boundary"].as_u64().unwrap(),false,false);
    assert_flight(&s,sp,&raw);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    post_raw(&relay,route,bad);post_raw(&relay,route,raw.clone());poll_candidate(&r,&relay,route,rp,&ro);
    payload_once(&ro,b"legitimate after skip refusal");
    let ack=saved_accept(&r,rp,&raw);assert_flight(&s,sp,&raw);saved_ack(&s,sp,&ack);
    println!("NA0780_ACCEPT group=authenticated_skip_bound_no_mutation_exact_retry_progress result=pass");
}

#[test]
fn directional_spent_target_admission(){
    let(_,s,r,_,ro,sp,rp,route)=admission_pair();let initial=integration_state(&s,sp);
    let receiver=integration_state(&r,rp);let spent=receiver["core"]["local_consumed_prefix"].as_u64().unwrap();assert!(spent>0);
    let bad=qsc::na0780_test_hostile_wire(&initial["core"].to_string(),"spent",spent.try_into().unwrap()).unwrap();
    saved_reject(&r,rp,&bad,"TARGET_SPENT");
    let raw=saved_prepare(&s,sp,"after-spent-target",b"legitimate after spent target",initial["last_boundary"].as_u64().unwrap()+900,false,false);
    assert_eq!(raw[4],1);assert_flight(&s,sp,&raw);
    let ack=saved_accept(&r,rp,&raw);assert_eq!(saved_accept(&r,rp,&raw),ack,"committed boundary duplicate must use exact witness");
    assert_flight(&s,sp,&raw);saved_ack(&s,sp,&ack);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    post_raw(&relay,route,bad);post_raw(&relay,route,raw);poll_candidate(&r,&relay,route,rp,&ro);
    payload_once(&ro,b"legitimate after spent target");
    assert_eq!(integration_state(&r,rp)["core"]["local_consumed_prefix"],receiver["core"]["local_consumed_prefix"]);
    println!("NA0780_ACCEPT group=authenticated_spent_target_no_mutation_boundary_duplicate_progress result=pass");
}

#[test]
fn directional_target_equivocation_admission(){
    let(_,s,r,_,ro,sp,rp,route)=admission_pair();let initial=integration_state(&s,sp);let now=initial["last_boundary"].as_u64().unwrap();
    let adv=saved_prepare(&s,sp,"",&[],now,true,true);let ack=saved_accept(&r,rp,&adv);saved_ack(&s,sp,&ack);
    let advertised=integration_state(&r,rp);let eligible=advertised["core"]["peer"].as_object().unwrap();assert_eq!(eligible.len(),1);
    let id:u32=eligible.keys().next().unwrap().parse().unwrap();let sender=integration_state(&s,sp);
    let bad=qsc::na0780_test_hostile_wire(&sender["core"].to_string(),"adv",id).unwrap();
    saved_reject(&r,rp,&bad,"TARGET_EQUIVOCATION");
    assert_eq!(saved_accept(&r,rp,&adv),ack,"original ADV retry retains exact response");
    let raw=saved_prepare(&s,sp,"after-equivocation",b"legitimate after conflicting advertisement",now,false,false);assert_flight(&s,sp,&raw);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    post_raw(&relay,route,bad);post_raw(&relay,route,raw);poll_candidate(&r,&relay,route,rp,&ro);
    payload_once(&ro,b"legitimate after conflicting advertisement");
    let after=integration_state(&r,rp);
    assert!(after["core"]["peer"]==advertised["core"]["peer"],"equivocation replaced eligible key");
    println!("NA0780_ACCEPT group=authenticated_target_equivocation_original_adv_retry_progress result=pass");
}

#[test]
fn directional_fixture_scheduler_inspect(){
    let(base,a,b,_,_)=retained_paths();
    let rows=[(&a,"bob","A"),(&b,"alice","B")].into_iter().map(|(cfg,peer,label)|{
        let s=integration_state(cfg,peer);
        serde_json::json!({"endpoint":label,"role":s["core"]["role"],"owner":s["core"]["owner"],"seq":s["core"]["seq"],"demand":s["demand"],"since_boundary":s["since_boundary"],"last_boundary":s["last_boundary"],"send_epoch":s["core"]["send"]["id"],"contexts":s["send"].as_object().unwrap().len()+s["recv"].as_object().unwrap().len(),"flights":s["flights"].as_object().unwrap().len(),"local_targets":s["core"]["local"].as_object().unwrap().len(),"peer_targets":s["core"]["peer"].as_object().unwrap().len()})
    }).collect::<Vec<_>>();
    save_exchange_record(&base.join("scheduler-inspection.json"),&serde_json::json!(rows));
    println!("NA0780_SCHEDULER {}",serde_json::json!(rows));
    println!("NA0780_ACCEPT group=fixture_scheduler_inspection_only result=pass");
}

// F3: each failure and retry stays in this test process, through the same
// locked transaction callers used by receive and send preparation. A directory
// at the writer's exact create_new path causes a real, pre-rename save failure.
#[test]
fn directional_same_process_save_retry() {
    let(_,s,r,so,ro,sp,rp,_)=admission_pair();
    complete_ordinary_fixture(&s,&r,sp,rp);
    let now=integration_state(&s,sp)["last_boundary"].as_u64().unwrap();
    let raw=saved_prepare(&s,sp,"f3-receive",b"same process receipt",now,false,false);
    assert_eq!(raw[4],0);
    qsc::na0780_test_seal_observer_controls();
    qsc::na0780_test_seal_observer(true);
    let before=integration_state(&r,rp);
    let outputs=delivered_count(&ro);
    env::set_var("QSC_NA0780_RECEIVE_SAVE_FAULT","1");
    let failed=qsc::na0780_test_receive_response(rp,&raw);
    env::remove_var("QSC_NA0780_RECEIVE_SAVE_FAULT");
    assert_eq!(failed,Err("vault_write_failed"),"no receipt may return on failed commit");
    assert!(integration_state(&r,rp)==before);
    assert_eq!(delivered_count(&ro),outputs);
    assert_eq!(qsc::na0780_test_seal_observer(false),[1,0,0,0,0]);
    let retry=qsc::na0780_test_receive_response(rp,&raw);
    let reproduce=env::var_os("NA0780_F3_REPRODUCE").is_some();
    if reproduce {
        assert_eq!(retry,Err("KEY_NONCE_REUSE"));
        assert!(integration_state(&r,rp)==before);
        println!("NA0780_ACCEPT group=f3_receipt_same_process_wedge_no_release_reproduced result=pass");
    } else {
        let receipt=retry.expect("same-process receipt retry must recover").unwrap();
        let stored=integration_state(&r,rp);
        let exact:Vec<u8>=serde_json::from_value(stored["dispositions"][wire_slot(&raw)]["receipt"].clone()).unwrap();
        assert_eq!(receipt,exact);
        assert_eq!(qsc::na0780_test_receive_response(rp,&raw).unwrap(),Some(receipt.clone()));
        assert_eq!(qsc::na0780_test_seal_observer(false),[2,1,0,0,0],"exact committed receipt replay must not seal");
        saved_ack(&s,sp,&receipt);
        println!("NA0780_ACCEPT group=f3_receipt_same_process_recovery_committed_exact_retry result=pass");
    }
    qsc::na0780_test_seal_observer(true);
    let before=integration_state(&s,sp);
    let outputs=delivered_count(&so);
    let blocker=s.join(format!("vault.qsv.tmp.{}",std::process::id()));
    fs::create_dir(&blocker).unwrap();
    let failed=qsc::na0780_test_prepare(sp,"f3-send",b"uncommitted send",now,false,false);
    fs::remove_dir(&blocker).unwrap();
    assert_eq!(failed,Err("vault_write_failed"),"failed preparation must not return ciphertext");
    assert!(integration_state(&s,sp)==before);
    assert_eq!(delivered_count(&so),outputs);
    assert_eq!(qsc::na0780_test_seal_observer(false),[2,0,0,0,0]);
    let retry=qsc::na0780_test_prepare(sp,"f3-send",b"uncommitted send",now,false,false);
    if reproduce {
        assert_eq!(retry,Err("KEY_NONCE_REUSE"));
        assert!(integration_state(&s,sp)==before);
        println!("NA0780_ACCEPT group=f3_send_same_process_wedge_no_release_reproduced result=pass");
    } else {
        let wire=retry.expect("same-process send retry must recover");
        assert_eq!(wire[4],0);
        assert_flight(&s,sp,&wire);
        integration_state(&s,sp);
        assert_eq!(qsc::na0780_test_prepare(sp,"f3-send",b"uncommitted send",now,false,false).unwrap(),wire);
        assert_eq!(qsc::na0780_test_prepare(sp,"f3-send",b"different length",now,false,false),Err("APPLICATION_ID_CONFLICT"));
        assert_eq!(qsc::na0780_test_seal_observer(false),[4,2,0,0,0],"exact committed wire replay must not seal");
        let receipt=saved_accept(&r,rp,&wire);saved_ack(&s,sp,&receipt);
        println!("NA0780_ACCEPT group=f3_send_same_process_recovery_committed_exact_retry result=pass");
    }
}


#[test]
fn directional_same_process_changed_uncommitted_body() {
    let(_,s,r,_,_,sp,rp,_)=admission_pair();complete_ordinary_fixture(&s,&r,sp,rp);
    let now=integration_state(&s,sp)["last_boundary"].as_u64().unwrap();
    qsc::na0780_test_seal_observer(true);
    let before=integration_state(&s,sp);
    let blocker=s.join(format!("vault.qsv.tmp.{}",std::process::id()));fs::create_dir(&blocker).unwrap();
    let failed=qsc::na0780_test_prepare(sp,"f3-changed",b"body-one",now,false,false);
    fs::remove_dir(&blocker).unwrap();assert_eq!(failed,Err("vault_write_failed"));
    assert!(integration_state(&s,sp)==before);
    let wire=qsc::na0780_test_prepare(sp,"f3-changed",b"body-two",now,false,false).unwrap();
    assert_eq!(wire[4],0);assert_flight(&s,sp,&wire);
    // Header repeats exactly; equal-length body changes at the speculative pair.
    assert_eq!(qsc::na0780_test_seal_observer(false),[4,2,1,0,0]);
    integration_state(&s,sp);
    assert_eq!(qsc::na0780_test_prepare(sp,"f3-changed",b"body-two",now,false,false).unwrap(),wire);
    assert_eq!(qsc::na0780_test_prepare(sp,"f3-changed",b"body-one",now,false,false),Err("APPLICATION_ID_CONFLICT"));
    assert_eq!(qsc::na0780_test_seal_observer(false),[4,2,1,0,0]);
    let receipt=saved_accept(&r,rp,&wire);saved_ack(&s,sp,&receipt);
    let stats=qsc::na0780_test_seal_observer(false);assert_eq!(stats[3],0);assert_eq!(stats[4],0);
    println!("NA0780_ACCEPT group=f3_changed_uncommitted_equal_length_body_internal_conflict_no_exposure_exact_retry result=pass");
}

// F1/F2: selective temporary denial, with exact retries filtered at every
// recipient poll. Separate fresh fixtures/results; no state or clock rollback.
// Timing is observation only. Begin markers are flushed before blocking work;
// an interrupted phase remains identifiable even without its end marker.
fn suppression_timed<T>(started:&Instant,round:u32,phase:&str,work:impl FnOnce()->T)->T {
    let phase_start=Instant::now();
    println!("NA0780_TIMING {}",serde_json::json!({"round":round,"phase":phase,"event":"begin","case_ms":started.elapsed().as_millis()}));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let result=work();
    println!("NA0780_TIMING {}",serde_json::json!({"round":round,"phase":phase,"event":"end","elapsed_ms":phase_start.elapsed().as_millis(),"case_ms":started.elapsed().as_millis()}));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    result
}
fn selective_suppression(boundary:bool) {
    use sha2::{Digest,Sha512};
    let started=Instant::now();
    let(base,s,r,so,ro,sp,rp,route_r)=suppression_timed(&started,0,"fixture_open",admission_pair);
    let route_s=if route_r==ROUTE_TOKEN_ALICE {ROUTE_TOKEN_BOB}else{ROUTE_TOKEN_ALICE};
    suppression_timed(&started,0,"fixture_refresh",||complete_ordinary_fixture(&s,&r,sp,rp));
    let setup_start=Instant::now();
    println!("NA0780_TIMING {}",serde_json::json!({"round":0,"phase":"fixture_prepare","event":"begin","case_ms":started.elapsed().as_millis()}));
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let initial=integration_state(&s,sp);let initial_r=integration_state(&r,rp);
    let now=initial["last_boundary"].as_u64().unwrap().max(initial_r["last_boundary"].as_u64().unwrap());
    let selected_body=if boundary {b"held exact boundary".as_slice()}else{b"held exact ordinary".as_slice()};
    let expected_completion=Sha512::digest(selected_body)[..32].to_vec();
    let selected=saved_prepare(&s,sp,"selected-suppression",selected_body,now+if boundary {900}else{0},false,false);
    assert_eq!(selected[4],u8::from(boundary),"selected frame class must be exact");
    let key=wire_slot(&selected);let epoch=u64::from_be_bytes(selected[22..30].try_into().unwrap());
    if !boundary {
        let follower=saved_prepare(&s,sp,"overtaking-boundary",b"boundary over ordinary gap",now+900,false,false);
        assert_eq!(follower[4],1,"ordinary gap must cross a real boundary");
    }
    let emitted_seq=integration_state(&s,sp)["core"]["seq"].as_u64().unwrap();
    // Independent reverse traffic must still be offered and delivered normally.
    saved_prepare(&r,rp,"independent-reverse",b"reverse during suppression",now,false,false);
    env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",(now+900).to_string());
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    let count_body=|out:&Path,body:&[u8]|fs::read_dir(out).unwrap().filter_map(Result::ok).filter(|e|fs::read(e.path()).ok().as_deref()==Some(body)).count();
    let receipt_matches=|raw:&Vec<u8>|raw.len()==113 && raw.starts_with(b"NDR1") && raw[21..29]==selected[22..30] && raw[61..65]==selected[30..34];
    println!("NA0780_TIMING {}",serde_json::json!({"round":0,"phase":"fixture_prepare","event":"end","elapsed_ms":setup_start.elapsed().as_millis(),"case_ms":started.elapsed().as_millis()}));
    let mut suppressed=0;
    for round in 1..=3 {
        let round_started=Instant::now();
        suppression_timed(&started,round,"suppression_sender_poll",||poll_candidate(&s,&relay,route_s,sp,&so));
        let queue=server.drain_channel(route_r);let count=queue.iter().filter(|raw|**raw==selected).count();
        assert!(count>0,"sender did not retry retained exact flight");suppressed+=count;
        server.replace_channel(route_r,queue.into_iter().filter(|raw|*raw!=selected).collect());
        suppression_timed(&started,round,"suppression_receiver_poll",||poll_candidate(&r,&relay,route_r,rp,&ro));
        let responses=server.drain_channel(route_s);
        assert!(!responses.iter().any(receipt_matches),"receipt for suppressed ciphertext");
        server.replace_channel(route_s,responses);
        let (a,b)=suppression_timed(&started,round,"suppression_state_reload",||(integration_state(&s,sp),integration_state(&r,rp)));
        suppression_timed(&started,round,"suppression_flight_reload",||assert_flight(&s,sp,&selected));
        assert!(a["completed"].get("selected-suppression").is_none());
        assert!(b["dispositions"].get(&key).is_none());
        assert!(b["events"].get("selected-suppression").is_none());
        assert_eq!(count_body(&ro,selected_body),0,"suppressed application delivered");
        assert!(a["send"].get(epoch.to_string()).is_some(),"outstanding verifier retired");
        assert!(a["send_floor"].as_u64().is_none_or(|f|f<epoch));
        if boundary {
            assert!(b["core"]["root"]==initial_r["core"]["root"]);
            assert_eq!(b["core"]["seq"],initial_r["core"]["seq"]);
            assert!(b["recv"].get(epoch.to_string()).is_none(),"unreceived boundary installed");
        } else {
            assert!(b["recv"].get(epoch.to_string()).is_some(),"missing old epoch prematurely retired");
            assert!(b["recv_floor"].as_u64().is_none_or(|f|f<epoch));
            // A required next refresh cannot consume context past the old gap.
            suppression_timed(&started,round,"suppression_capacity_probe",|| {
            integration_state(&r,rp);
            assert_eq!(qsc::na0780_test_prepare(rp,"blocked-refresh",b"refresh waits safely",now+900,false,false),Err("RECEIPT_CONTEXT_CAPACITY"));
            assert!(integration_state(&r,rp)==b,"capacity refusal mutated persisted state");
            });
        }
        assert!(started.elapsed()<Duration::from_secs(600),"case deadline exceeded");
        println!("NA0780_SUPPRESSION {}",serde_json::json!({"boundary":boundary,"round":round,"suppressed_exact":count,"sender":retirement_history(&a),"receiver":retirement_history(&b),"no_false_delivery":true}));
        println!("NA0780_TIMING {}",serde_json::json!({"round":round,"phase":"suppression_round","event":"end","elapsed_ms":round_started.elapsed().as_millis(),"case_ms":started.elapsed().as_millis()}));
    }
    payload_once(&so,b"reverse during suppression");
    // Restore only the originally committed bytes; future retries are unfiltered.
    post_raw(&relay,route_r,selected.clone());
    let progress_s=base.join("suppression-progress-s.body");let progress_r=base.join("suppression-progress-r.body");
    fs::write(&progress_s,b"subsequent legitimate sender progress").unwrap();fs::write(&progress_r,b"subsequent legitimate receiver progress").unwrap();
    let mut progress_requested=false;let mut exact_receipt_seen=false;
    let mut committed_receipt_witness:Option<Vec<u8>>=None;
    for round in 1..=8 {
        let round_started=Instant::now();
        suppression_timed(&started,round,"restoration_receiver_poll",||poll_candidate(&r,&relay,route_r,rp,&ro));
        let responses=server.drain_channel(route_s);
        if let Some(receipt)=responses.iter().find(|raw|receipt_matches(raw)) {
            let state=suppression_timed(&started,round,"restoration_receipt_reload",||integration_state(&r,rp));
            let retired_witness=state["dispositions"].get(&key).is_none();
            let saved:Vec<u8>=if let Some(disposition)=state["dispositions"].get(&key) {
                serde_json::from_value(disposition["receipt"].clone()).unwrap()
            } else {
                // An exact response already queued by this receive batch may
                // outlive its saved witness after authenticated no-retry closure.
                // Never accept absence before observing the committed witness.
                assert!(exact_receipt_seen && committed_receipt_witness.is_some());
                let slot=u32::from_be_bytes(selected[30..34].try_into().unwrap()) as u64;
                assert!(state["recv_floor"].as_u64().is_some_and(|floor|floor>=epoch)
                    || state["recv"][epoch.to_string()]["confirmed"].as_u64().is_some_and(|prefix|prefix>slot),
                    "missing witness requires authenticated no-retry coverage");
                committed_receipt_witness.as_ref().unwrap().clone()
            };
            if let Some(prior)=&committed_receipt_witness {assert_eq!(&saved,prior,"committed witness changed");}
            assert_eq!(*receipt,saved,"released receipt must be the committed witness");
            committed_receipt_witness=Some(saved);exact_receipt_seen=true;
            println!("NA0780_RECEIPT_EVIDENCE {}",serde_json::json!({"round":round,"stage":"committed_witness_exact","retired_witness":retired_witness,"retirement_covered":retired_witness,"epoch":epoch}));
        }
        server.replace_channel(route_s,responses);
        suppression_timed(&started,round,"restoration_sender_poll",||poll_candidate(&s,&relay,route_s,sp,&so));
        let (a,b)=suppression_timed(&started,round,"restoration_state_reload",||(integration_state(&s,sp),integration_state(&r,rp)));
        let recovered=a["flights"].get(&key).is_none();
        if recovered {
            // This selected test ID has no queue projection that could consume
            // its completion. The normal receive path inserts this exact body
            // commitment only after authenticating the receipt for flight.wire.
            let completion:Vec<u8>=serde_json::from_value(a["completed"]["selected-suppression"].clone()).expect("authenticated durable completion required, not flight absence alone");
            assert_eq!(completion,expected_completion,"selected completion must bind the exact prepared body");
            assert!(exact_receipt_seen && committed_receipt_witness.is_some(),"completion requires observed exact committed receipt");
            println!("NA0780_RECEIPT_EVIDENCE {}",serde_json::json!({"round":round,"stage":"authenticated_sender_completion","body_commitment_matches":true,"selected_flight_absent":true,"epoch":epoch}));
        }
        if recovered && !progress_requested {
            assert!(exact_receipt_seen,"flight cleared without observed authenticated witness");payload_once(&ro,selected_body);
            env::set_var("QSC_UNSAFE_TEST_CLOCK_UNIX_S",(now+1801).to_string());
            suppression_timed(&started,round,"restoration_progress_send",||{send_or_capacity(&s,&relay,sp,&progress_s);send_or_capacity(&r,&relay,rp,&progress_r);});progress_requested=true;
        }
        if progress_requested {suppression_timed(&started,round,"restoration_queue_retry",||{retry_queue(&s,&relay);retry_queue(&r,&relay);});}
        let (a,b)=suppression_timed(&started,round,"restoration_state_reload",||(integration_state(&s,sp),integration_state(&r,rp)));
        println!("NA0780_RESTORATION {}",serde_json::json!({"boundary":boundary,"round":round,"selected_receipted":recovered,"sender":retirement_history(&a),"receiver":retirement_history(&b)}));
        println!("NA0780_TIMING {}",serde_json::json!({"round":round,"phase":"restoration_round","event":"end","elapsed_ms":round_started.elapsed().as_millis(),"case_ms":started.elapsed().as_millis()}));
        let closed=boundary || (a["send_floor"].as_u64().is_some_and(|f|f>=epoch) && b["recv_floor"].as_u64().is_some_and(|f|f>=epoch));
        if recovered && closed && progress_requested && a["core"]["root"]==b["core"]["root"] && a["core"]["seq"].as_u64().unwrap()>emitted_seq && count_body(&ro,b"subsequent legitimate sender progress")==1 && count_body(&so,b"subsequent legitimate receiver progress")==1 {
            payload_once(&ro,selected_body);assert!(started.elapsed()<Duration::from_secs(600));
            println!("NA0780_ACCEPT group={}_suppression_exact_retries_three_rounds_authenticated_recovery_progress suppressed={} recovery_rounds={} result=pass",if boundary{"boundary"}else{"ordinary"},suppressed,round);return;
        }
    }
    panic!("restored delivery did not recover and progress within eight rounds");
}
#[test] fn directional_ordinary_suppression_recovery(){selective_suppression(false);}
#[test] fn directional_boundary_suppression_recovery(){selective_suppression(true);}

// Read-only inspection on a fresh copy of the stopped timed ordinary fixture.
// The original result's restoration round1 records sender send_floor=27.
#[test]
fn directional_suppression_saved_receipt_inspect() {
    use sha2::{Digest,Sha512};
    let(_,a,b,ao,bo)=retained_paths();
    let sa=integration_state(&a,"bob");let sb=integration_state(&b,"alice");
    let id="selected-suppression";
    assert_ne!(sa["completed"].get(id).is_some(),sb["completed"].get(id).is_some());
    let(sender,receiver,out)=if sa["completed"].get(id).is_some(){(&sa,&sb,&bo)}else{(&sb,&sa,&ao)};
    let completion:Vec<u8>=serde_json::from_value(sender["completed"][id].clone()).expect("durable selected completion required");
    assert_eq!(completion,Sha512::digest(b"held exact ordinary")[..32]);
    assert!(sender["flights"].as_object().unwrap().values().all(|f|f["id"]!=id));
    assert_eq!(sender["send_floor"],27,"must match saved restoration round1 trace");
    assert!(receiver["recv_floor"].as_u64().is_some_and(|floor|floor>=27));
    assert!(receiver["recv"].get("27").is_none());
    assert!(receiver["dispositions"].as_object().unwrap().keys().all(|key|key.split_once(':').unwrap().0.parse::<u64>().unwrap()>27));
    payload_once(out,b"held exact ordinary");
    println!("NA0780_ACCEPT group=saved_failure_authenticated_completion_body_match_receiver_floor_covers_selected_epoch_single_payload result=pass");
    println!("NA0780_SAVED_RECEIPT {}",serde_json::json!({"sender_completion_matches":true,"selected_flight_absent":true,"sender_floor":sender["send_floor"],"receiver_floor":receiver["recv_floor"],"selected_epoch_context_absent":true,"single_payload":true}));
}

// Bounded hosted-CI coverage. This is not the 56-round acceptance or a resume.
#[test]
fn directional_ci_fresh_crossed_delivery() {
    for key in ["QSC_QSP_SEED", "QSC_ALLOW_SEED_FALLBACK", "QSC_UNSAFE_TEST_SEED_FALLBACK"] {
        env::remove_var(key);
    }
    for order in 0..2 {
        let base = safe_test_root().join(format!("directional_ci_{}_{}", std::process::id(), order));
        assert!(!base.exists());
        ensure_dir_700(&base);
        let a = base.join("alice"); let b = base.join("bob");
        let ao = base.join("a-out"); let bo = base.join("b-out");
        for dir in [&a, &b, &ao, &bo] { ensure_dir_700(dir); }
        common::init_mock_vault(&a); common::init_mock_vault(&b);
        let server = common::start_inbox_server(1024 * 1024, 128);
        let relay = server.base_url();
        hs_dance(&a, &b, &relay, &server);
        let sa = integration_state(&a, "bob"); let sb = integration_state(&b, "alice");
        assert_eq!(sa["version"], "NA0780-DIR-INTEGRATION-03");
        assert_eq!(sa["core"]["sid"], sb["core"]["sid"]);
        assert_eq!(sa["core"]["root"], sb["core"]["root"]);
        let af = base.join("a.body"); let bf = base.join("b.body");
        fs::write(&af, format!("fresh CI A {order}")).unwrap();
        fs::write(&bf, format!("fresh CI B {order}")).unwrap();
        let accepted_a = acceptance_enqueue_once(&a, &relay, "bob", &af);
        let accepted_b = acceptance_enqueue_once(&b, &relay, "alice", &bf);
        for _ in 0..4 {
            if order == 0 {
                poll_candidate(&b, &relay, ROUTE_TOKEN_BOB, "alice", &bo);
                poll_candidate(&a, &relay, ROUTE_TOKEN_ALICE, "bob", &ao);
            } else {
                poll_candidate(&a, &relay, ROUTE_TOKEN_ALICE, "bob", &ao);
                poll_candidate(&b, &relay, ROUTE_TOKEN_BOB, "alice", &bo);
            }
        }
        assert!(acceptance_same_operation_done(&a, "bob", &accepted_a));
        assert!(acceptance_same_operation_done(&b, "alice", &accepted_b));
        payload_once(&bo, &fs::read(&af).unwrap());
        payload_once(&ao, &fs::read(&bf).unwrap());
        println!("NA0780_CI order={order} same_operation_delivered=2 result=pass");
    }
}

/// Fresh authenticated peers; hostile AEAD-valid bodies are constructed only from
/// a disposable sender snapshot. Every reject must leave all durable files intact.
#[test]
fn directional_successor_authenticated_malformed_no_mutation() {
    if !common::directional_case_child("directional_successor_authenticated_malformed_no_mutation") { return; }
    // Optional rendezvous belongs only to the exact parent-control invocation.
    let mut overlap = env::var("QSC_NA0780_OBSERVER_PROBE").ok().map(|value| {
        use std::io::{Read, Write};
        let port: u16 = value.strip_prefix("malformed-overlap:").expect("exact overlap probe").parse().unwrap();
        let mut stream = std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from(([127, 0, 0, 1], port)), Duration::from_secs(5)).unwrap();
        stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        stream.write_all(b"B").unwrap();
        let mut ack = [0]; stream.read_exact(&mut ack).unwrap(); assert_eq!(ack, *b"B");
        stream
    });
    let base=safe_test_root().join(format!("na0780_successor_malformed_{}",std::process::id()));
    assert!(!base.exists());ensure_dir_700(&base);
    let a=base.join("alice");let b=base.join("bob");let bo=base.join("bob-out");
    for dir in [&a,&b,&bo] {ensure_dir_700(dir);}
    common::init_mock_vault(&a);common::init_mock_vault(&b);
    let server=common::start_inbox_server(1024*1024,64);let relay=server.base_url();
    hs_dance(&a,&b,&relay,&server);
    let file=base.join("first");fs::write(&file,b"honest before malformed").unwrap();
    send_msg(&a,&relay,"bob",&file);
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let sender=common::directional_state(&a,"bob");
    let snapshot=serde_json::to_string(&sender["core"]).unwrap();
    env::set_var("QSC_CONFIG_DIR",&b);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    fn tree(path:&Path)->std::collections::BTreeMap<PathBuf,Vec<u8>> {
        fn walk(root:&Path,path:&Path,out:&mut std::collections::BTreeMap<PathBuf,Vec<u8>>) {
            for entry in fs::read_dir(path).unwrap() {
                let entry=entry.unwrap();let kind=entry.file_type().unwrap();assert!(!kind.is_symlink());
                if kind.is_dir(){walk(root,&entry.path(),out)}else{assert!(kind.is_file());out.insert(entry.path().strip_prefix(root).unwrap().to_owned(),fs::read(entry.path()).unwrap());}
            }
        }
        let mut out=std::collections::BTreeMap::new();walk(path,path,&mut out);out
    }
    let durable=tree(&b);let outputs=tree(&bo);
    let pending=server.drain_channel(ROUTE_TOKEN_ALICE);
    for mode in ["body_profile","body_kind","body_padding_profile","body_length","body_payload_length","body_padding","body_closure","body_request"] {
        let raw=qsc::na0780_test_hostile_wire(&snapshot,mode,0).unwrap();
        assert!(qsc::na0780_test_receive_response("alice",&raw).is_err(),"authenticated malformed body must reject");
        assert!(tree(&b)==durable,"reject mutated durable receiver state");
        assert!(tree(&bo)==outputs,"reject produced output");
        assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),"reject released a receipt");
    }
    server.replace_channel(ROUTE_TOKEN_ALICE,pending);
    let file=base.join("second");fs::write(&file,b"honest after malformed").unwrap();
    send_msg(&a,&relay,"bob",&file);
    poll_candidate(&b,&relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let contents=tree(&bo);
    assert_eq!(contents.values().filter(|v|v.as_slice()==b"honest before malformed").count(),1);
    assert_eq!(contents.values().filter(|v|v.as_slice()==b"honest after malformed").count(),1);
    assert!(outputs.iter().all(|(p,bytes)|contents.get(p)==Some(bytes)),"prior outputs unchanged");
    if let Some(stream) = overlap.as_mut() {
        use std::io::{Read, Write};
        stream.write_all(b"E").unwrap();
        let mut ack = [0]; stream.read_exact(&mut ack).unwrap(); assert_eq!(ack, *b"E");
    }
}

#[test]
fn directional_successor_authenticated_malformed_no_mutation_parent_control() {
    use std::io::{Read, Write};
    let environment: std::collections::BTreeMap<_, _> = env::vars_os().collect();
    let passphrase_present = qsc::vault::has_process_passphrase();
    let unchanged = || {
        assert!(environment == env::vars_os().collect(), "parent config/environment changed");
        assert_eq!(qsc::vault::has_process_passphrase(), passphrase_present, "parent unlock state changed");
    };
    let listener = std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
    listener.set_nonblocking(true).unwrap();
    let probe = format!("malformed-overlap:{}", listener.local_addr().unwrap().port());
    std::thread::scope(|scope| {
        // The existing wrapper owns, bounds and reaps this exact child, including
        // on failure. No change to process-global environment or cleanup semantics.
        let child = scope.spawn(|| assert!(!common::directional_case_child_bounded(
            "directional_successor_authenticated_malformed_no_mutation",
            Duration::from_secs(590), Some(&probe))));
        let ready_deadline = Instant::now() + Duration::from_secs(5);
        let mut stream = loop {
            unchanged();
            match listener.accept() {
                Ok((stream, address)) => { assert!(address.ip().is_loopback()); break stream; }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    assert!(Instant::now() < ready_deadline, "child overlap readiness deadline");
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("child overlap accept: {error}"),
            }
        };
        stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        stream.set_write_timeout(Some(Duration::from_secs(5))).unwrap();
        for (phase, budget) in [(b'B', Duration::from_secs(5)), (b'E', Duration::from_secs(580))] {
            let deadline = Instant::now() + budget;
            loop {
                unchanged();
                assert!(Instant::now() < deadline, "child overlap phase deadline");
                let mut message = [0];
                match stream.read(&mut message) {
                    Ok(1) => { assert_eq!(message[0], phase); break; }
                    Ok(_) => panic!("child exited before overlap rendezvous"),
                    Err(error) if matches!(error.kind(), std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut) => {
                        assert!(Instant::now() < deadline, "child overlap phase deadline");
                    }
                    Err(error) => panic!("child overlap read: {error}"),
                }
            }
            // Child cannot leave this phase until the parent has checked and ACKed.
            // Two ACKed phases prove observation while the child is actually live.
            unchanged(); stream.write_all(&[phase]).unwrap();
            println!("R06 overlap_phase={} parent_observed_live_child=true", if phase == b'B' { "begin" } else { "end" });
        }
        child.join().expect("bounded isolated fixture passed");
    });
    unchanged();
}

fn review_tree(root: &Path) -> std::collections::BTreeMap<PathBuf, Vec<u8>> {
    fn walk(root: &Path, path: &Path, out: &mut std::collections::BTreeMap<PathBuf, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap(); let kind = entry.file_type().unwrap();
            assert!(!kind.is_symlink());
            if kind.is_dir() { walk(root, &entry.path(), out); }
            else { assert!(kind.is_file()); out.insert(entry.path().strip_prefix(root).unwrap().to_owned(), fs::read(entry.path()).unwrap()); }
        }
    }
    let mut out = std::collections::BTreeMap::new(); walk(root, root, &mut out); out
}

fn review_receive(cfg: &Path, relay: &str, out: &Path, fail_write: bool) -> std::process::Output {
    let mut command = qsc_cfg_cmd(cfg);
    command.args(["receive", "--transport", "relay", "--relay", relay, "--mailbox", ROUTE_TOKEN_BOB,
        "--from", "alice", "--receipt-mode", "immediate", "--max", "16", "--out", out.to_str().unwrap()]);
    if fail_write { command.env("QSC_NA0780_RECEIVE_SAVE_FAULT", "1"); }
    command.output().expect("actual transport receive")
}

fn review_r01_batch(fixed: bool) {
    let base = safe_test_root().join(format!("r01_{}_{}", if fixed { "fixed" } else { "baseline" }, std::process::id()));
    assert!(!base.exists()); ensure_dir_700(&base);
    let a = base.join("alice"); let b = base.join("bob"); let bo = base.join("bob-out");
    for path in [&a, &b, &bo] { ensure_dir_700(path); }
    common::init_directional_pair(&a, "alice", ROUTE_TOKEN_ALICE, &b, "bob", ROUTE_TOKEN_BOB);
    let server = common::start_inbox_server(1024 * 1024, 64);
    server.enable_review_leases(); let relay = server.base_url();
    let file = base.join("payload"); fs::write(&file, b"R01 honest A").unwrap();
    send_msg(&a, relay, "bob", &file);
    let honest_a = server.review_lease_snapshot(ROUTE_TOKEN_BOB).retained;
    assert!(!honest_a.is_empty());
    let sender = common::directional_state(&a, "bob");
    let snapshot = serde_json::to_string(&sender["core"]).unwrap();
    // Unlock only this exact isolated child for the existing direct receive seam.
    // All authenticated snapshots use read-only sessions; no parent unlock occurs.
    env::set_var("QSC_CONFIG_DIR", &b);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let durable = review_tree(&b); let outputs = review_tree(&bo);
    let modes = [
        ("body_padding_profile", "INTEGRATION_PADDING_PROFILE"),
        ("body_padding_size", "INTEGRATION_PADDING_SIZE"),
        ("body_padding", "INTEGRATION_PADDING_NONZERO"),
        ("body_maintenance", "INTEGRATION_BODY"),
        ("body_file_shape", "INTEGRATION_FILE_SHAPE"),
        ("body_request", "INTEGRATION_FILE_REQUEST"),
        ("body_file_gated", "INTEGRATION_FILE_GATED"),
    ];
    let mut rejected = Vec::new();
    for (mode, code) in modes.into_iter().take(if fixed { 7 } else { 1 }) {
        let raw = qsc::na0780_test_hostile_wire(&snapshot, mode, 0).unwrap();
        assert_eq!(qsc::na0780_test_receive_response("alice", &raw).err(), Some(code), "exact authenticated producer code");
        assert!(review_tree(&b) == durable, "rejected producer changed durable state");
        assert!(review_tree(&bo) == outputs, "rejected producer wrote output");
        let response = reqwest::blocking::Client::new().post(format!("{relay}/v1/push"))
            .header("X-QSL-Route-Token", ROUTE_TOKEN_BOB).body(raw.clone()).send().unwrap();
        assert!(response.status().is_success());
        let response: serde_json::Value = response.json().unwrap();
        rejected.push((response["id"].as_str().unwrap().to_owned(), raw));
        println!("R01 authenticated producer={code} durable_unchanged=true");
    }
    fs::write(&file, b"R01 honest B").unwrap(); send_msg(&a, relay, "bob", &file);
    let queued = server.review_lease_snapshot(ROUTE_TOKEN_BOB).retained;
    let honest_b: Vec<_> = queued.iter().filter(|(id, _)| !honest_a.iter().chain(&rejected).any(|(old, _)| id == old)).cloned().collect();
    assert!(!honest_b.is_empty());
    assert_eq!(queued.iter().map(|(id, _)| id).collect::<Vec<_>>(), honest_a.iter().chain(&rejected).chain(&honest_b).map(|(id, _)| id).collect::<Vec<_>>(), "ordered A/rejected/B");
    let result = review_receive(&b, relay, &bo, false); let text = output_text(&result);
    fs::write(base.join("batch-receive.log"), &text).unwrap();
    if fixed { assert!(result.status.success(), "fixed actual receive failed: {text}"); }
    else {
        assert!(!result.status.success(), "baseline must interrupt actual receive");
        assert!(text.contains("code=INTEGRATION_PADDING_PROFILE"), "different baseline failure: {text}");
        println!("R01 expected_baseline_interruption=INTEGRATION_PADDING_PROFILE exit={:?}", result.status.code());
    }
    let after = server.review_lease_snapshot(ROUTE_TOKEN_BOB);
    assert_eq!(after.pulls[0], queued.iter().map(|(id, _)| id.clone()).collect::<Vec<_>>(), "actual ordered pull batch");
    let acked: Vec<_> = after.acks.iter().flatten().collect();
    for (id, raw) in &rejected {
        assert!(after.retained.iter().any(|(kept, bytes)| kept == id && bytes == raw), "rejected bytes retained");
        assert!(!acked.contains(&id), "rejected frame ACKed");
    }
    for (id, _) in &honest_a { assert!(acked.contains(&id)); }
    let delivered = review_tree(&bo);
    assert_eq!(delivered.values().filter(|v| v.as_slice() == b"R01 honest A").count(), 1);
    assert_eq!(delivered.values().filter(|v| v.as_slice() == b"R01 honest B").count(), usize::from(fixed));
    assert_eq!(delivered_count(&bo), if fixed { 2 } else { 1 }, "no rejected output");
    for (id, raw) in &honest_b {
        assert_eq!(acked.contains(&id), fixed);
        assert_eq!(after.retained.iter().any(|(kept, bytes)| kept == id && bytes == raw), !fixed);
    }
    let receiver = common::directional_state(&b, "alice");
    let dispositions = receiver["dispositions"].as_object().unwrap();
    let expected = honest_a.len() + if fixed { honest_b.len() } else { 0 };
    assert_eq!(dispositions.len(), expected, "no rejected durable disposition");
    let receipts: Vec<_> = server.directional_pushes().into_iter().filter(|p| p.body.starts_with(b"NDR1")).collect();
    assert_eq!(receipts.len(), expected, "no rejected NDR1");
    for receipt in receipts {
        assert!(receipt.status == 200 && receipt.response_written);
        assert!(dispositions.values().any(|d| serde_json::from_value::<Vec<u8>>(d["receipt"].clone()).unwrap() == receipt.body));
    }
    if fixed {
        // A fresh honest operation reaches the existing actual writer fault seam.
        // A storage failure must still abort, retain its leased input and emit no receipt/output.
        fs::write(&file, b"R01 local write failure").unwrap(); send_msg(&a, relay, "bob", &file);
        let before = common::directional_state(&b, "alice");
        let before_outputs = review_tree(&bo);
        let before_acks = server.review_lease_snapshot(ROUTE_TOKEN_BOB).acks;
        let before_receipts = server.directional_pushes().iter().filter(|p| p.body.starts_with(b"NDR1")).count();
        let before_retained = server.review_lease_snapshot(ROUTE_TOKEN_BOB).retained;
        let failed = review_receive(&b, relay, &bo, true); let text = output_text(&failed);
        fs::write(base.join("local-write-failure.log"), &text).unwrap();
        assert!(!failed.status.success(), "local write error swallowed");
        assert!(text.contains("event=directional_receive_save_fault") && text.contains("stored_state_unchanged=true"), "actual writer fault not observed: {text}");
        assert!(!text.contains("write_error=none"));
        let after = common::directional_state(&b, "alice");
        assert!(before["core"] == after["core"] && before["dispositions"] == after["dispositions"] && before["events"] == after["events"], "failed admission mutated receiver");
        assert!(review_tree(&bo) == before_outputs);
        let lease = server.review_lease_snapshot(ROUTE_TOKEN_BOB);
        assert_eq!(lease.retained, before_retained); assert_eq!(lease.acks, before_acks);
        assert_eq!(server.directional_pushes().iter().filter(|p| p.body.starts_with(b"NDR1")).count(), before_receipts);
        println!("R01 fixed seven_codes=pass local_write_failure=propagated retained=true no_ack_or_receipt=true");
    }
}

// Historical pre-correction instrument only; deliberately absent from libtest
// discovery. Its exact original registered source, compile1 pins and passing
// defect-demonstration result are preserved in the existing followup evidence.
fn directional_review_r01_receive_batch_baseline() {
    if !common::directional_case_child("directional_review_r01_receive_batch_baseline") { return; }
    review_r01_batch(false);
}

#[test]
fn directional_review_r01_receive_batch_fixed() {
    if !common::directional_case_child("directional_review_r01_receive_batch_fixed") { return; }
    review_r01_batch(true);
}


// R02 integration-delta witnesses only. These are code proposals, not executed
// acceptance of the retained-control theorem or the seven-case allocation.
fn r02_session(cfg: &Path) -> qsc::vault::VaultSession {
    assert!(matches!(env::var("QSC_NA0780_ISOLATED_CASE").as_deref(),
        Ok("directional_r02_fresh_and_ordinary_writers" | "directional_r02_genuine_receipt_restart" | "directional_r02_funded_release_at_saturation" | "directional_r02_serializer_maintenance" | "directional_r02_repeated_controls" | "directional_r02_queuefull_matrix" | "directional_r02_completion_cuts")));
    env::set_var("QSC_CONFIG_DIR", cfg);
    qsc::vault::open_session_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap()
}
fn r02_owner(cfg: &Path) -> serde_json::Value {
    let session = r02_session(cfg);
    let raw = qsc::vault::session_get(&session,"na0780_directional_owner_v1").unwrap().unwrap();
    serde_json::from_str(&raw).unwrap()
}
fn r02_assert_pair(cfg: &Path, peer: &str) {
    let owner = r02_owner(cfg);
    let state = common::directional_state(cfg,peer);
    let reserve = &owner["peers"][peer];
    assert_eq!(reserve["sid"],state["core"]["sid"]);
    assert_eq!(reserve["generation"],state["generation"]);
    assert_eq!(reserve["peer"],peer);
    assert!(reserve["vault_future"].as_u64().unwrap()>0);
    for entry in owner["entries"].as_object().unwrap().values() {
        assert_eq!(entry["sid"],owner["peers"][entry["peer"].as_str().unwrap()]["sid"]);
        assert!(entry["projection"].as_u64().unwrap() <= entry["charge"]["vault_bytes"].as_u64().unwrap());
    }
}
#[test]
fn directional_r02_fresh_and_ordinary_writers() {
    if !common::directional_case_child("directional_r02_fresh_and_ordinary_writers") {return;}
    let base=safe_test_root().join(format!("r02_init_{}",std::process::id()));
    assert!(!base.exists());ensure_dir_700(&base);
    let ordinary=base.join("ordinary");ensure_dir_700(&ordinary);
    let pass=common::write_passphrase_file(&base,"ordinary-init",common::TEST_MOCK_VAULT_PASSPHRASE);
    let init=std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR",&ordinary).env("QSC_DISABLE_KEYCHAIN","1")
        .args(["vault","init","--protocol","owner-free-v1","--non-interactive",
            "--key-source","passphrase","--passphrase-file",pass.to_str().unwrap()])
        .output().unwrap();
    assert!(init.status.success(),"{}",output_text(&init));
    let mut session=r02_session(&ordinary);
    assert!(qsc::vault::session_get(&session,"na0780_directional_owner_v1").unwrap().is_none());
    qsc::vault::session_set(&mut session,"r02.ordinary.one","one").unwrap();
    qsc::vault::secret_set_with_passphrase("r02.ordinary.two","two",common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    qsc::vault::secret_set("r02.ordinary.three","three").unwrap();
    // Stale ordinary session persistence must retain unrelated intervening writes.
    qsc::vault::persist_session(&mut session).unwrap();
    for (key,value) in [("r02.ordinary.one","one"),("r02.ordinary.two","two"),("r02.ordinary.three","three")] {
        assert_eq!(qsc::vault::session_get(&r02_session(&ordinary),key).unwrap().as_deref(),Some(value));
    }
    let unchanged=fs::read(ordinary.join("vault.qsv")).unwrap();
    for key in ["na0780_directional_owner_v1","na0780_directional_transaction_v2/bob","na0780_directional_transaction/bob"] {
        assert!(qsc::vault::session_set(&mut session,key,"{}").is_err());
        assert_eq!(fs::read(ordinary.join("vault.qsv")).unwrap(),unchanged);
    }
    // Fixture precondition (measured in the CC direct evaluation): handshake init resolves the
    // peer route token before perform_handshake_init_with_route runs directional_owner_load, so an
    // ordinary vault without a pinned contact refuses with QSC_ERR_CONTACT_ROUTE_TOKEN_REQUIRED and
    // never reaches the directional admission refusal this case asserts. Give the ordinary vault a
    // pinned contact from a throwaway owner-free peer identity; no directional state is created.
    let peer_src=base.join("peer-source");ensure_dir_700(&peer_src);
    let peer_init=std::process::Command::new(assert_cmd::cargo::cargo_bin!("qsc"))
        .env("QSC_CONFIG_DIR",&peer_src).env("QSC_DISABLE_KEYCHAIN","1")
        .args(["vault","init","--protocol","owner-free-v1","--non-interactive",
            "--key-source","passphrase","--passphrase-file",pass.to_str().unwrap()])
        .output().unwrap();
    assert!(peer_init.status.success(),"{}",output_text(&peer_init));
    let rotated=run_qsc(&peer_src,&["identity","rotate","--as","bob","--confirm"]);assert!(rotated.status.success(),"{}",output_text(&rotated));
    let public=output_text(&run_qsc(&peer_src,&["identity","show","--as","bob"]));
    let field=|prefix:&str| public.lines().find_map(|l|l.strip_prefix(prefix)).expect("identity public field").to_owned();
    let added=run_qsc(&ordinary,&["contacts","add","--label","bob","--fp",&field("identity_fp="),
        "--kem-pk",&field("identity_kem_pk="),"--sig-pk",&field("identity_sig_pk="),"--route-token",ROUTE_TOKEN_BOB]);
    assert!(added.status.success(),"{}",output_text(&added));
    let server=common::start_inbox_server(1024*1024,16);
    let refused=run_qsc(&ordinary,&["handshake","init","--as","alice","--peer","bob","--relay",server.base_url()]);
    assert!(!refused.status.success());
    assert!(output_text(&refused).contains("directional_reserve_missing") || output_text(&refused).contains("directional_profile_required"));
    assert!(server.drain_channel(ROUTE_TOKEN_BOB).is_empty());
    let owned=base.join("owned");ensure_dir_700(&owned);common::init_mock_vault(&owned);
    let owner=r02_owner(&owned);assert_eq!(owner["generation"],0);
    assert!(owner["peers"].as_object().unwrap().is_empty());
    assert!(owner["entries"].as_object().unwrap().is_empty());
    let before=fs::read(owned.join("vault.qsv")).unwrap();
    let mut session=r02_session(&owned);
    assert!(qsc::vault::persist_session(&mut session).is_err());
    assert_eq!(fs::read(owned.join("vault.qsv")).unwrap(),before);
    assert_eq!(qsc::vault::vault_init_directional_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE),Err("vault_exists"));
    assert_eq!(fs::read(owned.join("vault.qsv")).unwrap(),before);
    qsc::vault::session_set(&mut session,"r02.unrelated","preserved").unwrap();
    assert_eq!(r02_owner(&owned),owner);
    // Real filesystem failure before initial commit; no test-only writer or state injection.
    let blocked=base.join("blocked");ensure_dir_700(&blocked);
    fs::write(blocked.join("prior-data"),b"preserve existing development data").unwrap();
    env::set_var("QSC_CONFIG_DIR",&blocked);
    assert!(qsc::vault::vault_init_directional_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).is_err());
    assert!(!blocked.join("vault.qsv").exists());
    assert_eq!(fs::read(blocked.join("prior-data")).unwrap(),b"preserve existing development data");
    env::set_var("QSC_CONFIG_DIR",&owned);
    r02_real_layout_refusals(&base,&owned);
}

#[test]
fn directional_r02_genuine_receipt_restart() {
    if !common::directional_case_child("directional_r02_genuine_receipt_restart") {return;}
    let base=safe_test_root().join(format!("r02_receipt_{}",std::process::id()));
    assert!(!base.exists());ensure_dir_700(&base);
    let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
    for dir in [&a,&b,&ao,&bo] {ensure_dir_700(dir);}
    common::init_mock_vault(&a);common::init_mock_vault(&b);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    // Existing normal caller fixture also sends an old-profile handshake and
    // requires refusal before completing the genuine successor handshake.
    hs_dance(&a,&b,relay,&server);r02_assert_pair(&a,"bob");r02_assert_pair(&b,"alice");
    // Activation-time state: core.root here is the establishment root that derived the epoch-0
    // receipt context (directional_delivery.rs:435). The product replaces core.root on every
    // epoch schedule (directional_core.rs:546/:746), so the later post-poll state cannot
    // reproduce that key; the persisted context key itself never changes (measured, round 2).
    let activation=common::directional_state(&a,"bob");
    let primer=base.join("primer");fs::write(&primer,b"R02 closure prerequisite").unwrap();
    send_msg(&a,relay,"bob",&primer);
    poll_candidate(&b,relay,ROUTE_TOKEN_BOB,"alice",&bo);
    poll_candidate(&a,relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let file=base.join("first");fs::write(&file,b"R02 original payload").unwrap();
    send_msg(&a,relay,"bob",&file);
    let first=common::directional_state(&a,"bob");
    let flight=first["flights"].as_object().unwrap().values().find(|f|f["id"]!="").unwrap().clone();
    let key=format!("{}:{}",flight["epoch"].as_u64().unwrap(),flight["slot"].as_u64().unwrap());
    poll_candidate(&b,relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let mut held=server.drain_channel(ROUTE_TOKEN_ALICE);
    let index=held.iter().position(|raw| raw.starts_with(b"NDR1") && raw.len()>=65
        && u64::from_be_bytes(raw[21..29].try_into().unwrap())==flight["epoch"].as_u64().unwrap()
        && u32::from_be_bytes(raw[61..65].try_into().unwrap()) as u64==flight["slot"].as_u64().unwrap()).expect("actual authenticated peer receipt");
    let receipt=held.remove(index);
    use base64::Engine;
    let proof=base64::engine::general_purpose::STANDARD.decode(flight["closure_proof"].as_str().unwrap()).unwrap();
    assert_eq!(proof.len(),136);assert!(proof[0]>0,"carrier must carry real prior coverage");
    r02_verify_actual_receipt_domains(&activation,&flight,&receipt);
    // Persist a newer obligation while the real older receipt remains delayed.
    let next=base.join("next");fs::write(&next,b"R02 newer payload").unwrap();send_msg(&a,relay,"bob",&next);
    let before=common::directional_state(&a,"bob");
    assert_eq!(before["send"]["0"]["context"]["key"],activation["send"]["0"]["context"]["key"],"epoch-0 context key unchanged across polls");
    let newer:Vec<_>=before["flights"].as_object().unwrap().iter().filter(|(k,f)|*k!=&key && f["id"]!="")
        .map(|(k,f)|(k.clone(),f.clone())).collect();assert!(!newer.is_empty());
    assert_eq!(before["flights"][&key],flight,"immutable saved carrier until genuine NDR1");
    // An advertisement sealed before this carrier may also have its genuine
    // receipt held. Release those exact lower-slot receipts first so e.admit can
    // close the gap; sealing the newer Flight advances next, never prefix.
    let mut release=Vec::new();
    held.retain(|raw| {
        let preceding=raw.starts_with(b"NDR1") && raw.len()==113
            && u64::from_be_bytes(raw[21..29].try_into().unwrap())==flight["epoch"].as_u64().unwrap()
            && (u32::from_be_bytes(raw[61..65].try_into().unwrap()) as u64)<flight["slot"].as_u64().unwrap();
        if preceding {release.push(raw.clone());}
        !preceding
    });
    release.push(receipt.clone());
    server.replace_channel(ROUTE_TOKEN_ALICE,release);
    poll_candidate(&a,relay,ROUTE_TOKEN_ALICE,"bob",&ao); // separate CLI process: actual restart/load
    let after=common::directional_state(&a,"bob");assert!(after["flights"].get(&key).is_none());
    let mut advanced=false;
    for c in proof[1..1+45*proof[0] as usize].chunks_exact(45) {
        let epoch=u64::from_be_bytes(c[..8].try_into().unwrap()).to_string();
        let count=u32::from_be_bytes(c[40..44].try_into().unwrap()) as u64;
        assert_eq!(c[44],0,"this fixture requires nonfinal coverage, not retired-context inference");
        let prior=before["send"][&epoch]["confirmed"].as_u64().unwrap();
        assert_eq!(after["send"][&epoch]["confirmed"].as_u64().unwrap(),prior.max(count));
        assert!(count<after["send"][&epoch]["prefix"].as_u64().unwrap(),
            "actual post-receipt prefix strictly exceeds exact saved carrier coverage");
        advanced|=count>prior;
    }
    assert!(advanced,"receipt must advance actual saved coverage");
    for (k,f) in &newer {assert_eq!(after["flights"][k]["wire"],f["wire"]);assert_eq!(after["flights"][k]["closure_proof"],f["closure_proof"]);}
    server.replace_channel(ROUTE_TOKEN_ALICE,vec![receipt]);
    poll_candidate(&a,relay,ROUTE_TOKEN_ALICE,"bob",&ao);
    let replay=common::directional_state(&a,"bob");
    for (epoch,e) in after["send"].as_object().unwrap() {
        assert_eq!(replay["send"][epoch]["confirmed"],e["confirmed"],"old receipt replay cannot confirm newer obligations");
    }
    for (k,f) in &newer {assert_eq!(replay["flights"][k]["wire"],f["wire"]);assert_eq!(replay["flights"][k]["closure_proof"],f["closure_proof"]);}
    server.replace_channel(ROUTE_TOKEN_ALICE,held);
    for _ in 0..4 {poll_candidate(&b,relay,ROUTE_TOKEN_BOB,"alice",&bo);poll_candidate(&a,relay,ROUTE_TOKEN_ALICE,"bob",&ao);}
    payload_once(&bo,b"R02 original payload");payload_once(&bo,b"R02 newer payload");
    r02_assert_pair(&a,"bob");r02_assert_pair(&b,"alice");
}


// Independent arithmetic over ACTUAL persisted serializer output and promised
// scalar widths. It does not manufacture maximum-width protocol state or claim
// that the still-conditional retained-control envelope has been proven.
fn r02_observed_charge(cfg: &Path) -> u64 {
    let raw=fs::read(cfg.join("vault.qsv")).unwrap();
    assert_eq!(&raw[..6],b"QSCV03");
    let ciphertext=u32::from_le_bytes(raw[21..25].try_into().unwrap()) as usize;
    assert_eq!(raw.len(),53+ciphertext);
    let owner=r02_owner(cfg);
    let width=|v:&serde_json::Value| 20-v.as_u64().unwrap().to_string().len() as u64;
    let mut promised=width(&owner["generation"]);
    for peer in owner["peers"].as_object().unwrap().values() {
        promised+=width(&peer["generation"])+width(&peer["peer_future"])+width(&peer["vault_future"])
            +peer["vault_future"].as_u64().unwrap();
    }
    for entry in owner["entries"].as_object().unwrap().values() {
        promised+=width(&entry["generation"])+width(&entry["projection"])+width(&entry["charge"]["vault_bytes"])
            +entry["charge"]["vault_bytes"].as_u64().unwrap();
    }
    (ciphertext-16) as u64 + promised + 524288
}

#[test]
fn directional_r02_funded_release_at_saturation() {
    if !common::directional_case_child("directional_r02_funded_release_at_saturation") {return;}
    // Exactly the approved evaluation fixture; not a production budget choice.
    let base=safe_test_root().join(format!("R02-ordinary-E-X-saturation-{}",std::process::id()));
    assert!(!base.exists());ensure_dir_700(&base);
    let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
    for dir in [&a,&b,&ao,&bo] {ensure_dir_700(dir);}
    common::init_mock_vault(&a);common::init_mock_vault(&b);
    let server=common::start_inbox_server(1024*1024,128);let relay=server.base_url();
    hs_dance(&a,&b,relay,&server);
    r02_add_other_peer(&b,&base.join("carol"));
    let other_peer_credit=r02_owner(&b)["peers"]["carol"].clone();
    let e=vec![b'E';16000];let ef=base.join("E");fs::write(&ef,&e).unwrap();
    let sent=run_qsc(&a,&["send","--transport","relay","--relay",relay,"--to","bob","--file",ef.to_str().unwrap(),
        "--pad-bucket","standard","--pad-to","16384","--bucket-max","16384"]);
    assert!(sent.status.success(),"{}",output_text(&sent));
    let sa=common::directional_state(&a,"bob");
    let eflight=sa["flights"].as_object().unwrap().values().find(|f|f["id"]!="").unwrap();
    assert_eq!(eflight["id"].as_str().unwrap().len(),32);
    let cut=qsc_cfg_cmd(&b).env("QSC_NA0780_CUT","after_receive_commit")
        .args(["receive","--transport","relay","--relay",relay,"--mailbox",ROUTE_TOKEN_BOB,"--from","alice",
            "--max","8","--out",bo.to_str().unwrap()]).output().unwrap();
    assert_eq!(cut.status.code(),Some(86),"actual post-commit cut required");
    let pending=common::directional_state(&b,"alice");assert_eq!(pending["events"].as_object().unwrap().len(),1);
    assert_eq!(delivered_count(&bo),0);
    let held_e=server.drain_channel(ROUTE_TOKEN_BOB); // retained for exact replay below, not a retention proof
    let xf=base.join("X");fs::write(&xf,b"X").unwrap();
    let sent=run_qsc(&a,&["send","--transport","relay","--relay",relay,"--to","bob","--file",xf.to_str().unwrap(),
        "--pad-bucket","standard","--pad-to","1024","--bucket-max","16384"]);
    assert!(sent.status.success(),"{}",output_text(&sent));
    let sx=common::directional_state(&a,"bob");
    let xflight=sx["flights"].as_object().unwrap().values().find(|f|f["id"]!="" && f["id"]!=eflight["id"]).unwrap();
    assert_eq!(xflight["id"].as_str().unwrap().len(),32);
    let xwire:Vec<u8>=serde_json::from_value(xflight["wire"].clone()).unwrap();
    let held_x=server.drain_channel(ROUTE_TOKEN_BOB);assert!(held_x.contains(&xwire));
    // Reach saturation solely through ordinary authorized writes. Every refused
    // write leaves the ciphertext and independently funded event/credit intact.
    let mut session=r02_session(&b);let mut lo=0usize;let mut hi=16777217usize;
    qsc::vault::session_set(&mut session,"r02.fixture.filler","").unwrap();
    while hi-lo>1 {
        let mid=lo+(hi-lo)/2;let before=fs::read(b.join("vault.qsv")).unwrap();
        match qsc::vault::session_set(&mut session,"r02.fixture.filler",&"f".repeat(mid)) {
            Ok(())=>lo=mid,
            Err(error)=>{assert_eq!(error,"directional_aggregate_waiting");assert_eq!(fs::read(b.join("vault.qsv")).unwrap(),before);hi=mid;}
        }
    }
    assert!(lo>0);let saturated=r02_observed_charge(&b);assert_eq!(saturated,16777216);
    env::set_var("QSC_CONFIG_DIR",&b);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let before=fs::read(b.join("vault.qsv")).unwrap();
    assert_eq!(qsc::na0780_test_receive_response("alice",&xwire),Err("TRANSACTION_CAPACITY"));
    assert_eq!(fs::read(b.join("vault.qsv")).unwrap(),before);
    assert_eq!(delivered_count(&bo),0);
    // Empty-inbox normal receive MUST release the already-funded E independently
    // of accepting X. No filler deletion or control-credit borrowing is allowed.
    poll_candidate(&b,relay,ROUTE_TOKEN_BOB,"alice",&bo);
    let after_e=r02_observed_charge(&b);let released_e=saturated.checked_sub(after_e).unwrap();
    assert!(released_e>0);payload_once(&bo,&e);
    let before_x=fs::read(b.join("vault.qsv")).unwrap();
    // First successful X admission is through the real transport intake. Stop at
    // the existing durable-commit cut to measure X before projection frees it.
    server.replace_channel(ROUTE_TOKEN_BOB,vec![xwire.clone()]);
    let admitted=qsc_cfg_cmd(&b).env("QSC_NA0780_CUT","after_receive_commit")
        .args(["receive","--transport","relay","--relay",relay,"--mailbox",ROUTE_TOKEN_BOB,
            "--from","alice","--out",bo.to_str().unwrap(),"--max","16"]).output().unwrap();
    assert_eq!(admitted.status.code(),Some(86),"{}",output_text(&admitted));
    assert_ne!(fs::read(b.join("vault.qsv")).unwrap(),before_x);
    assert_eq!(delivered_count(&bo),1,"X is durable but not yet projected");
    assert!(!r02_pending_receipts(&common::directional_state(&b,"alice")).is_empty());
    let after_x=r02_observed_charge(&b);let required_x=after_x.checked_sub(after_e).unwrap();
    assert!(required_x>0 && released_e>=required_x);
    let mut replay=held_e;replay.extend(held_x);server.replace_channel(ROUTE_TOKEN_BOB,replay);
    for _ in 0..4 {poll_candidate(&b,relay,ROUTE_TOKEN_BOB,"alice",&bo);poll_candidate(&a,relay,ROUTE_TOKEN_ALICE,"bob",&ao);}
    payload_once(&bo,&e);payload_once(&bo,b"X");r02_assert_pair(&a,"bob");r02_assert_pair(&b,"alice");
    assert_eq!(qsc::vault::session_get(&r02_session(&b),"r02.fixture.filler").unwrap().unwrap().len(),lo);
    assert_eq!(r02_owner(&b)["peers"]["carol"],other_peer_credit,"E/X never borrows the other peer reserve");
}


// Observe a REAL peer-produced receipt. Re-derive its two possible domain inputs
// from the authenticated epoch-zero root; never construct a replacement receipt,
// inject a key, or force protocol state. The positive arm prevents vacuous negatives.
fn r02_verify_actual_receipt_domains(state:&serde_json::Value,flight:&serde_json::Value,receipt:&[u8]) {
    use quantumshield_refimpl::crypto::traits::{Aead,Hash};
    fn lp(bytes:&[u8])->Vec<u8> {let mut out=(bytes.len() as u32).to_be_bytes().to_vec();out.extend(bytes);out}
    assert_eq!(flight["epoch"].as_u64().unwrap(),0,"fresh epoch-zero consumer required");
    assert_eq!(receipt.len(),113);assert_eq!(&receipt[..4],b"NDR1");
    let sid:Vec<u8>=serde_json::from_value(state["core"]["sid"].clone()).unwrap();
    let mut root:Vec<u8>=serde_json::from_value(state["core"]["root"].clone()).unwrap();
    let wire:Vec<u8>=serde_json::from_value(flight["wire"].clone()).unwrap();
    let slot=flight["slot"].as_u64().unwrap() as u32;
    assert!(receipt[4..20]==sid);assert_eq!(receipt[20],state["core"]["role"].as_u64().unwrap() as u8);
    let derive=|profile:&[u8]| {
        let mut input=lp(b"NA0780-DIR-EPOCH-CORE-01");input.extend(&sid);input.extend(lp(profile));
        input.extend(&receipt[21..29]);input.push(receipt[20]);input.extend(&receipt[29..61]);
        StdCrypto.kmac256(&root,"NA0780.DE1/RECEIPT_KEY",&input,32)
    };
    let mut successor:[u8;32]=derive(b"NA0780-DIR-INTEGRATION-03").try_into().unwrap();
    let mut predecessor:[u8;32]=derive(b"NA0780-DIR-INTEGRATION-02").try_into().unwrap();
    let mut actual:Vec<u8>=serde_json::from_value(state["send"]["0"]["context"]["key"].clone()).unwrap();
    assert!(actual.as_slice()==successor.as_slice() && successor!=predecessor,"actual epoch uses exact successor key domain");
    let ad=|profile:&[u8]| {let mut out=lp(profile);out.extend(lp(b"NA0780-DIR-EPOCH-CORE-01"));out.extend(&receipt[..65]);out};
    let mut nonce=[0u8;12];nonce[8..].copy_from_slice(&slot.to_be_bytes());
    let expected=StdCrypto.sha512(&wire);
    let clear=StdCrypto.open(&successor,&nonce,&ad(b"NA0780-DIR-INTEGRATION-03"),&receipt[65..]).unwrap();
    assert!(clear==expected[..32],"real receipt authenticates exact saved wire");
    assert!(StdCrypto.open(&predecessor,&nonce,&ad(b"NA0780-DIR-INTEGRATION-03"),&receipt[65..]).is_err());
    assert!(StdCrypto.open(&successor,&nonce,&ad(b"NA0780-DIR-INTEGRATION-02"),&receipt[65..]).is_err());
    assert!(StdCrypto.open(&predecessor,&nonce,&ad(b"NA0780-DIR-INTEGRATION-02"),&receipt[65..]).is_err());
    root.fill(0);actual.fill(0);successor.fill(0);predecessor.fill(0);
}

// Public production transport entry, including projection, intake, replay and ACK
// flushing. Unlike na0780_test_receive_response, this is normal transport admission.
fn r02_receive(cfg:&Path, server:&common::InboxTestServer, mailbox:&str, peer:&str, out:&Path)->Result<(),String> {
    env::set_var("QSC_CONFIG_DIR",cfg);
    qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    r02_receive_unlocked(server,mailbox,peer,out)
}
fn r02_receive_unlocked(server:&common::InboxTestServer, mailbox:&str, peer:&str, out:&Path)->Result<(),String> {
    qsc::transport::receive_execute(qsc::ReceiveArgs {
        transport:Some(qsc::cmd::SendTransport::Relay),relay:Some(server.base_url().to_owned()),
        from:Some(peer.to_owned()),mailbox:Some(mailbox.to_owned()),out:Some(out.to_owned()),max:Some(16),
        legacy_receive_mode:None,attachment_service:None,max_file_size:None,max_file_chunks:None,
        deterministic_meta:false,interval_ms:None,poll_interval_ms:None,poll_ticks:None,
        batch_max_count:None,poll_max_per_tick:None,bucket_max:None,meta_seed:None,
        emit_receipts:None,receipt_mode:None,receipt_batch_window_ms:None,receipt_jitter_ms:None,
        file_confirm_mode:None,
    }).map_err(|e|format!("{e:?}"))
}
fn r02_pair_fixture(name:&str)->(PathBuf,PathBuf,PathBuf,PathBuf,PathBuf) {
    let base=safe_test_root().join(format!("{name}-{}",std::process::id()));
    assert!(!base.exists());ensure_dir_700(&base);
    let a=base.join("alice");let b=base.join("bob");let ao=base.join("a-out");let bo=base.join("b-out");
    for dir in [&a,&b,&ao,&bo] {ensure_dir_700(dir);}
    // Separate handshake relay: the observation relay below starts genuinely empty.
    common::init_directional_pair(&a,"alice",ROUTE_TOKEN_ALICE,&b,"bob",ROUTE_TOKEN_BOB);
    (base,a,b,ao,bo)
}
fn r02_pending_receipts(state:&serde_json::Value)->Vec<Vec<u8>> {
    state["dispositions"].as_object().unwrap().values()
        .filter(|d|d["response_pending"]==true)
        .map(|d|serde_json::from_value(d["receipt"].clone()).unwrap()).collect()
}
fn r02_one_attempt(journal:&[common::DirectionalPushAttempt]) {
    for (i,attempt) in journal.iter().enumerate() {
        assert!(attempt.response_written,"fault observation needs an actual response");
        assert!(journal[..i].iter().all(|prior|prior.body!=attempt.body),"one attempt per exact response per invocation");
    }
}
#[test]
fn directional_r02_queuefull_matrix() {
    if !common::directional_case_child("directional_r02_queuefull_matrix") {return;}
    // Four finite sub-arms in ONE proposed allocation; no retry of a failing arm.
    for (label,plan) in [("first-429",vec![429,200,200]),("middle-429",vec![200,429,200]),
                         ("first-500",vec![500]),("middle-500",vec![200,500])] {
        let (base,a,b,_ao,bo)=r02_pair_fixture(&format!("r02-batch-{label}"));
        let server=common::start_inbox_server(1024*1024,128);
        server.enable_review_leases();
        for n in 0..3 {
            let path=base.join(format!("body-{n}"));fs::write(&path,format!("{label}-{n}")).unwrap();
            send_msg(&a,server.base_url(),"bob",&path);
        }
        let inputs=server.review_lease_snapshot(ROUTE_TOKEN_BOB).retained;
        assert_eq!(inputs.len(),3);
        server.record_directional_pushes();server.r02_push_plan(&plan);
        let error=r02_receive(&b,&server,ROUTE_TOKEN_BOB,"alice",&bo).unwrap_err();
        if plan.contains(&429) {assert!(error.contains("relay_inbox_queue_full"));}
        else {assert!(error.contains("relay_inbox_push_failed"),"non-QueueFull must propagate separately: {error}");}
        let journal=server.directional_pushes();r02_one_attempt(&journal);
        assert_eq!(journal.iter().map(|p|p.status).collect::<Vec<_>>(),plan,
            "QueueFull continues eligible intake; other error interrupts immediately");
        assert!(journal.iter().all(|p|p.body.starts_with(b"NDR1")));
        let state=common::directional_state(&b,"alice");
        let pending=r02_pending_receipts(&state);
        assert!(state["flights"].as_object().unwrap().is_empty(),
            "backpressure/error cannot prepare a fresh control Flight in this initially send-free fixture");
        let failed:Vec<_>=journal.iter().filter(|p|p.status!=200).map(|p|p.body.clone()).collect();
        assert_eq!(pending,failed,"only the exact rejected genuine response remains pending");
        let snapshot=server.review_lease_snapshot(ROUTE_TOKEN_BOB);
        let acked:Vec<_>=snapshot.acks.iter().flatten().cloned().collect();
        let expected:Vec<_>=plan.iter().enumerate().filter(|(_,status)|**status==200)
            .map(|(i,_)|inputs[i].0.clone()).collect();
        assert_eq!(acked,expected,"eligible ACKs flush even when the call returns an error");
        for (id,raw) in &inputs {
            assert_eq!(snapshot.retained.iter().any(|(r,bytes)|r==id && bytes==raw),!acked.contains(id));
        }
        server.record_directional_pushes();
        // Original rejected input is still leased, not destructively re-delivered.
        // Recovery retries the durable response, without claiming lease-expiry coverage.
        r02_receive(&b,&server,ROUTE_TOKEN_BOB,"alice",&bo).unwrap();
        let replay=server.directional_pushes();r02_one_attempt(&replay);
        for bytes in failed {assert_eq!(replay.iter().filter(|p|p.body==bytes && p.status==200).count(),1);}
        assert!(r02_pending_receipts(&common::directional_state(&b,"alice")).is_empty());
    }
}

// Block only write_atomic's own create_new sibling, in this synthetic fixture.
// No protocol state, queue contents, permissions or process clocks are altered.
struct R02WriteBlock(PathBuf);
impl R02WriteBlock {
    fn new(target:&Path)->Self {
        let p=target.with_file_name(format!("{}.tmp.{}",target.file_name().unwrap().to_str().unwrap(),std::process::id()));
        assert!(!p.exists());fs::create_dir(&p).unwrap();Self(p)
    }
}
impl Drop for R02WriteBlock {fn drop(&mut self){fs::remove_dir(&self.0).expect("remove owned empty write blocker");}}
fn r02_record_path(cfg:&Path,id:&str)->PathBuf {
    let mut matches=Vec::new();
    for contact in fs::read_dir(cfg.join("msgqueue_v1")).unwrap() {
        let contact=contact.unwrap();if !contact.file_type().unwrap().is_dir(){continue;}
        for item in fs::read_dir(contact.path()).unwrap() {
            let p=item.unwrap().path();if p.file_name().unwrap().to_str().unwrap().ends_with(&format!("_{id}.rec")){matches.push(p);}
        }
    }
    assert_eq!(matches.len(),1);matches.remove(0)
}
#[test]
fn directional_r02_completion_cuts() {
    if !common::directional_case_child("directional_r02_completion_cuts") {return;}
    let (base,a,b,ao,bo)=r02_pair_fixture("r02-completion");
    let server=common::start_inbox_server(1024*1024,128);
    let body=base.join("first");fs::write(&body,b"durable completion").unwrap();send_msg(&a,server.base_url(),"bob",&body);
    let sent=common::directional_state(&a,"bob");
    let (first_key,first)=sent["flights"].as_object().unwrap().iter().find(|(_,f)|f["id"]!="").unwrap();
    let id=first["id"].as_str().unwrap().to_owned();
    poll_candidate(&b,server.base_url(),ROUTE_TOKEN_BOB,"alice",&bo);
    let replies=server.drain_channel(ROUTE_TOKEN_ALICE);
    let receipt=replies.into_iter().find(|r|r.starts_with(b"NDR1") &&
        u64::from_be_bytes(r[21..29].try_into().unwrap())==first["epoch"].as_u64().unwrap() &&
        u32::from_be_bytes(r[61..65].try_into().unwrap()) as u64==first["slot"].as_u64().unwrap()).unwrap();
    let other=base.join("other");fs::write(&other,b"unrelated pending flight").unwrap();send_msg(&a,server.base_url(),"bob",&other);
    let pending=common::directional_state(&a,"bob");
    let (other_key,other_flight)=pending["flights"].as_object().unwrap().iter()
        .find(|(k,f)|*k!=first_key && f["id"]!="").unwrap();
    server.replace_channel(ROUTE_TOKEN_ALICE,vec![receipt]);
    let cut=qsc_cfg_cmd(&a).env("QSC_NA0780_CUT","after_receive_commit")
        .args(["receive","--transport","relay","--relay",server.base_url(),"--mailbox",ROUTE_TOKEN_ALICE,
            "--from","bob","--out",ao.to_str().unwrap(),"--max","16"]).output().unwrap();
    assert_eq!(cut.status.code(),Some(86),"{}",output_text(&cut));
    let durable=common::directional_state(&a,"bob");assert!(durable["completed"].get(&id).is_some());
    assert!(server.drain_channel(ROUTE_TOKEN_ALICE).is_empty(),"restart has empty inbox");
    env::set_var("QSC_CONFIG_DIR",&a);qsc::vault::protection::unlock_guarded(common::TEST_MOCK_VAULT_PASSPHRASE).unwrap();
    let queue_path=r02_record_path(&a,&id);let queue_before=fs::read(&queue_path).unwrap();
    server.record_directional_pushes();
    {
        let _block=R02WriteBlock::new(&queue_path);
        let error=r02_receive_unlocked(&server,ROUTE_TOKEN_ALICE,"bob",&ao).unwrap_err();
        assert!(error.contains("msgqueue_write_failed"),"{error}");
        assert_eq!(fs::read(&queue_path).unwrap(),queue_before);
        assert!(common::directional_state(&a,"bob")["completed"].get(&id).is_some());
        assert!(server.directional_pushes().is_empty(),"local failure before remote replay");
    }
    // Timeline delivery was persisted before the queue failure. Its idempotent
    // retry now performs no vault write; the following blocker hits the pair save.
    let vault_before=fs::read(a.join("vault.qsv")).unwrap();
    {
        let _block=R02WriteBlock::new(&a.join("vault.qsv"));
        let error=r02_receive_unlocked(&server,ROUTE_TOKEN_ALICE,"bob",&ao).unwrap_err();
        assert!(error.contains("vault_write_failed"),"pair-save Local failure: {error}");
        assert_eq!(fs::read(a.join("vault.qsv")).unwrap(),vault_before);
        let records=common::directional_queue_records(&a,"bob");
        assert_eq!(records.iter().find(|r|r.msg_id==id).unwrap().state,qsc::msgqueue::MsgState::Delivered);
        assert!(common::directional_state(&a,"bob")["completed"].get(&id).is_some());
        assert!(server.directional_pushes().is_empty());
    }
    let charged=r02_observed_charge(&a);
    server.r02_push_plan(&[429]);
    let error=r02_receive(&a,&server,ROUTE_TOKEN_ALICE,"bob",&ao).unwrap_err();
    assert!(error.contains("relay_inbox_queue_full"));
    let retired=common::directional_state(&a,"bob");assert!(retired["completed"].get(&id).is_none());
    assert_eq!(retired["flights"][other_key],*other_flight,"unrelated immutable retry survives recovery");
    let after=r02_observed_charge(&a);assert!(after<charged,"completion frees its own funded liability");
    let entries=r02_owner(&a)["entries"].clone();
    assert!(entries.as_object().unwrap().values().filter(|e|e["operation"]==id).all(|e|e["projection"]==0),
        "completed projection is no longer owed; retained closure liabilities stay funded");
    server.record_directional_pushes();server.r02_push_plan(&[429]);
    let restarted=run_qsc(&a,&["receive","--transport","relay","--relay",server.base_url(),
        "--mailbox",ROUTE_TOKEN_ALICE,"--from","bob","--out",ao.to_str().unwrap(),"--max","16"]);
    assert!(!restarted.status.success());assert!(output_text(&restarted).contains("relay_inbox_queue_full"));
    assert_eq!(r02_observed_charge(&a),after,"same recovery has no double debit, including reserved generation widths");
    assert_eq!(r02_owner(&a)["entries"],entries);
    let attempts=server.directional_pushes();assert_eq!(attempts.len(),1);
    assert_eq!(attempts[0].body,serde_json::from_value::<Vec<u8>>(other_flight["wire"].clone()).unwrap());
    r02_assert_pair(&a,"bob");payload_once(&bo,b"durable completion");
}

fn r02_real_layout_refusals(base:&Path,valid:&Path) {
    use rand_core::RngCore;
    let original=fs::read(valid.join("vault.qsv")).unwrap();
    assert_eq!(&original[..6],b"QSCV03");
    let params=Params::new(u32::from_le_bytes(original[9..13].try_into().unwrap()),
        u32::from_le_bytes(original[13..17].try_into().unwrap()),
        u32::from_le_bytes(original[17..21].try_into().unwrap()),Some(32)).unwrap();
    let mut key=[0u8;32];
    Argon2::new(Algorithm::Argon2id,Version::V0x13,params)
        .hash_password_into(common::TEST_MOCK_VAULT_PASSPHRASE.as_bytes(),&original[25..41],&mut key).unwrap();
    let cipher=ChaCha20Poly1305::new(Key::from_slice(&key));
    let plain=cipher.decrypt(Nonce::from_slice(&original[41..53]),Payload{msg:&original[53..],aad:&original[..53]}).unwrap();
    let valid_payload:serde_json::Value=serde_json::from_slice(&plain).unwrap();
    assert!(qsc::vault::open_session_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE).is_ok());
    let server=common::start_inbox_server(1024*1024,16);server.record_directional_pushes();
    // Authenticated negative FILE fixtures, not modified protocol counters, keys
    // or receipts. Each gets a fresh AEAD nonce; source fixture remains untouched.
    for case in ["old-version","old-profile","unknown-profile","missing-owner","corrupt-owner","ordinary-with-owner","orphan-peer"] {
        let cfg=base.join(format!("layout-{case}"));assert!(!cfg.exists());ensure_dir_700(&cfg);
        let mut payload=valid_payload.clone();
        match case {
            "old-version"=>payload["version"]=serde_json::json!(3),
            "old-profile"=>payload["protocol"]=serde_json::json!("NA0780-DIR-INTEGRATION-02"),
            "unknown-profile"=>payload["protocol"]=serde_json::json!("invalid-test-profile"),
            "missing-owner"=>{payload["secrets"].as_object_mut().unwrap().remove("na0780_directional_owner_v1");},
            "corrupt-owner"=>payload["secrets"]["na0780_directional_owner_v1"]=serde_json::json!("{"),
            "ordinary-with-owner"=>payload["protocol"]=serde_json::json!("NA0780-OWNER-FREE-01"),
            "orphan-peer"=>payload["secrets"]["na0780_directional_transaction_v2/bob"]=serde_json::json!("{}"),
            _=>unreachable!(),
        }
        let bytes=serde_json::to_vec(&payload).unwrap();let mut header=original[..53].to_vec();
        header[21..25].copy_from_slice(&u32::try_from(bytes.len()+16).unwrap().to_le_bytes());
        rand_core::OsRng.fill_bytes(&mut header[41..53]);
        let encrypted=cipher.encrypt(Nonce::from_slice(&header[41..53]),Payload{msg:&bytes,aad:&header}).unwrap();
        let mut raw=header;raw.extend(encrypted);fs::write(cfg.join("vault.qsv"),&raw).unwrap();
        #[cfg(unix)] {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(cfg.join("vault.qsv"),fs::Permissions::from_mode(0o600)).unwrap();
        }
        env::set_var("QSC_CONFIG_DIR",&cfg);
        let expected=match case {
            "old-version"|"old-profile"|"unknown-profile"=>"vault_version_unsupported",
            "missing-owner"|"orphan-peer"=>"directional_reserve_missing",
            "corrupt-owner"=>"directional_owner_tampered",
            "ordinary-with-owner"=>"directional_owner_binding",_=>unreachable!(),
        };
        let opened=qsc::vault::open_session_with_passphrase(common::TEST_MOCK_VAULT_PASSPHRASE);
        assert_eq!(opened.err(),Some(expected),"real authenticated open: {case}");
        let denied=run_qsc(&cfg,&["receive","--transport","relay","--relay",server.base_url(),
            "--mailbox",ROUTE_TOKEN_ALICE,"--from","bob","--out",base.to_str().unwrap(),"--max","1"]);
        assert!(!denied.status.success(),"normal admission accepted {case}");
        assert!(output_text(&denied).contains(expected),"wrong admission rejection for {case}: {}",output_text(&denied));
        assert_eq!(fs::read(cfg.join("vault.qsv")).unwrap(),raw,"refusal must not repair {case}");
        assert!(server.directional_pushes().is_empty());
    }
    key.fill(0);assert_eq!(fs::read(valid.join("vault.qsv")).unwrap(),original);
}

fn r02_add_other_peer(shared:&Path,other:&Path) {
    const ROUTE:&str="route_token_carol_abcdefghijklmnop";
    fn ok(cfg:&Path,args:&[&str])->String {let out=run_qsc(cfg,args);assert!(out.status.success(),"{}",output_text(&out));output_text(&out)}
    fn field(s:&str,prefix:&str)->String {
        let value=s.lines().find_map(|l|l.strip_prefix(prefix)).expect("identity public field");
        assert_ne!(value,common::REDACTION_SENTINEL,"public identity field redacted");
        value.to_owned()
    }
    ensure_dir_700(other);common::init_mock_vault(other);
    ok(other,&["identity","rotate","--as","carol","--confirm"]);
    ok(other,&["relay","inbox-set","--token",ROUTE]);
    let bp=ok(shared,&["identity","show","--as","bob"]);let cp=ok(other,&["identity","show","--as","carol"]);
    for (cfg,name,route,public) in [(shared,"carol",ROUTE,cp.as_str()),(other,"bob",ROUTE_TOKEN_BOB,bp.as_str())] {
        ok(cfg,&["contacts","add","--label",name,"--fp",&field(public,"identity_fp="),
            "--kem-pk",&field(public,"identity_kem_pk="),"--sig-pk",&field(public,"identity_sig_pk="),"--route-token",route]);
        let devices=ok(cfg,&["contacts","device","list","--label",name]);
        let dev=devices.lines().find_map(|l|l.strip_prefix("device=")).unwrap().split_whitespace().next().unwrap();
        assert_ne!(dev,common::REDACTION_SENTINEL,"public device field redacted");
        ok(cfg,&["contacts","device","trust","--label",name,"--device",dev,"--confirm"]);
    }
    let server=common::start_inbox_server(1024*1024,16);
    ok(shared,&["handshake","init","--as","bob","--peer","carol","--relay",server.base_url(),"--suite-mode","suite-required"]);
    for (cfg,me,peer) in [(other,"carol","bob"),(shared,"bob","carol"),(other,"carol","bob")] {
        ok(cfg,&["handshake","poll","--as",me,"--peer",peer,"--relay",server.base_url(),"--max","4","--suite-mode","suite-required"]);
    }
    r02_assert_pair(shared,"carol");
}
fn r02_fill_evaluation_vault(cfg:&Path) {
    // Named fresh evaluation fixture only. Positive writes use the ordinary writer;
    // no direct payload mutation or limit override. Binary search is finite (25 writes).
    let mut lo=0usize;let mut hi=16_777_216usize;
    while lo<hi {
        let mid=(lo+hi+1)/2;let before=fs::read(cfg.join("vault.qsv")).unwrap();
        let mut session=r02_session(cfg);
        match qsc::vault::session_set(&mut session,"r02.fixture.filler",&"F".repeat(mid)) {
            Ok(())=>lo=mid,
            Err(code)=>{assert_eq!(code,"directional_aggregate_waiting");assert_eq!(fs::read(cfg.join("vault.qsv")).unwrap(),before);hi=mid-1;}
        }
    }
    assert!(lo>0);assert_eq!(r02_observed_charge(cfg),16_777_216);
}
fn r02_control_refs(owner:&serde_json::Value,peer:&str)->Vec<(u8,u64,u32)> {
    use base64::Engine;
    let raw=base64::engine::general_purpose::STANDARD.decode(owner["peers"][peer]["control"]["control_refs"].as_str().unwrap()).unwrap();
    assert_eq!(raw.len(),1620);
    raw.chunks_exact(45).filter(|r|r[0]!=0).map(|r|(r[0],u64::from_be_bytes(r[1..9].try_into().unwrap()),u32::from_be_bytes(r[9..13].try_into().unwrap()))).collect()
}
fn r02_nested_measurement(cfg:&Path,peer:&str)->(usize,usize) {
    let session=r02_session(cfg);let owner=r02_owner(cfg);let state=common::directional_state(cfg,peer);
    let raw=qsc::vault::session_get(&session,&format!("na0780_directional_transaction_v2/{peer}")).unwrap().unwrap();
    // Actual producer's serialized text, then its actual JSON-string layer.
    let nested=serde_json::to_vec(&raw).unwrap();assert!(nested.len()>raw.len());
    let outer=|v:&serde_json::Value|serde_json::to_vec(&serde_json::to_string(v).unwrap()).unwrap().len() as u64;
    let refs=r02_control_refs(&owner,peer);
    assert!(refs.len()<=36);
    for epoch in state["recv"].as_object().unwrap().keys() {
        assert!(refs.iter().filter(|(_,g,_)|g.to_string()==*epoch).count()<=18);
    }
    let mut residual=state.clone();
    residual.as_object_mut().unwrap().retain(|k,_|matches!(k.as_str(),"flights"|"dispositions"|"events"|"completed"|"recv"));
    residual["flights"].as_object_mut().unwrap().retain(|_,f|f["id"]!="");
    residual["dispositions"].as_object_mut().unwrap().retain(|k,_|!refs.iter().any(|(_,g,n)|*k==format!("{g}:{n}")));
    let mut holes=serde_json::Map::new();
    for (epoch,e) in state["recv"].as_object().unwrap() {
        let ordinary:Vec<_>=e["holes"].as_array().unwrap().iter().filter(|n|!refs.iter().any(|(_,g,s)|g.to_string()==*epoch && *s as u64==n.as_u64().unwrap())).cloned().collect();
        if !ordinary.is_empty(){holes.insert(epoch.clone(),serde_json::json!({"holes":ordinary}));}
    }
    residual["recv"]=serde_json::Value::Object(holes);
    let p=&owner["peers"][peer];let mut without=p.clone();without.as_object_mut().unwrap().remove("control");
    let retained=outer(&state)-outer(&residual)+outer(p)-outer(&without);
    assert_eq!(retained+p["vault_future"].as_u64().unwrap(),2*(288920+36775)+2*524288,
        "actual nested retained bytes plus future are conserved");
    // Inner serialized retained controls include the actual fixed witness codec,
    // map punctuation, control Flights/dispositions and current send-hole widths.
    let singleton=|key:&str,value:&serde_json::Value| {
        let map=std::collections::BTreeMap::from([(key,value)]);
        serde_json::to_vec(&map).unwrap().len() as u64-1
    };
    let mut inner=serde_json::to_vec(&p["control"]).unwrap().len() as u64+11;
    for (_,g,n) in &refs {inner+=singleton(&format!("{g}:{n}"),&state["dispositions"][format!("{g}:{n}")])+11;}
    for (key,f) in state["flights"].as_object().unwrap() {if f["id"]=="" {inner+=singleton(key,f);}}
    for e in state["send"].as_object().unwrap().values(){inner+=11*e["holes"].as_array().unwrap().len() as u64;}
    assert!(inner<=36775,"actual retained control serializer exceeds candidate bound");
    let mut context=state.clone();
    for name in ["flights","dispositions","events","completed"] {context[name]=serde_json::json!({});}
    for name in ["send","recv"] {for e in context[name].as_object_mut().unwrap().values_mut(){e["holes"]=serde_json::json!([]);}}
    assert!(serde_json::to_vec(&context).unwrap().len()<=288920,"actual reachable context exceeds conditional envelope");
    assert_eq!(p["peer_future"].as_u64().unwrap()+inner+serde_json::to_vec(&context).unwrap().len() as u64,
        36775+288920,"actual inner retained plus remaining conservation");
    // Independent scalar width oracle only; never write a maximum protocol counter.
    assert_eq!(serde_json::to_vec(&u64::MAX).unwrap().len(),20);
    let partition=(state["send"].as_object().unwrap().len(),state["recv"].as_object().unwrap().len());
    assert!(partition.0+partition.1<=3);
    partition
}
#[test]
fn directional_r02_serializer_maintenance() {
    if !common::directional_case_child("directional_r02_serializer_maintenance") {return;}
    let (base,a,b,ao,bo)=r02_pair_fixture("R02-two-peer-utility");
    let c=base.join("carol");r02_add_other_peer(&b,&c);
    let server=common::start_inbox_server(1024*1024,128);
    poll_candidate(&a,server.base_url(),ROUTE_TOKEN_ALICE,"bob",&ao); // genuine advertisement
    poll_candidate(&b,server.base_url(),ROUTE_TOKEN_BOB,"alice",&bo); // owner's genuine grant
    let before_send=common::directional_state(&b,"alice");
    let (key,maintenance)=before_send["flights"].as_object().unwrap().iter().find(|(_,f)|f["id"]=="").unwrap();
    let wire:Vec<u8>=serde_json::from_value(maintenance["wire"].clone()).unwrap();
    assert_eq!(&wire[..4],b"NDE1");assert_eq!(wire[4],1,"typed owner grant, not an advertisement receipt");
    let unrelated=base.join("retry");fs::write(&unrelated,b"unrelated immutable retry").unwrap();send_msg(&b,server.base_url(),"alice",&unrelated);
    // Only the real grant goes to Alice. Holding synthetic relay traffic is not
    // used as retention evidence; case5 supplies the lease-backed retention proof.
    let _held=server.drain_channel(ROUTE_TOKEN_ALICE);server.replace_channel(ROUTE_TOKEN_ALICE,vec![wire]);
    poll_candidate(&a,server.base_url(),ROUTE_TOKEN_ALICE,"bob",&ao);
    let receipts=server.drain_channel(ROUTE_TOKEN_BOB);
    let receipt=receipts.into_iter().find(|r|r.starts_with(b"NDR1") &&
        u64::from_be_bytes(r[21..29].try_into().unwrap())==maintenance["epoch"].as_u64().unwrap() &&
        u32::from_be_bytes(r[61..65].try_into().unwrap()) as u64==maintenance["slot"].as_u64().unwrap()).unwrap();
    r02_fill_evaluation_vault(&b);
    let before=common::directional_state(&b,"alice");let owner_before=r02_owner(&b);
    let charge_before=r02_observed_charge(&b);r02_nested_measurement(&b,"alice");
    let historical=|charge:u64,owner:&serde_json::Value| -> i128 {
        let p=&owner["peers"]["alice"];
        charge as i128-p["vault_future"].as_u64().unwrap() as i128+2*(p["peer_future"].as_u64().unwrap() as i128+524288)
    };
    server.replace_channel(ROUTE_TOKEN_BOB,vec![receipt]);
    let result=qsc_cfg_cmd(&b).env("QSC_NA0780_CUT","after_receive_commit")
        .args(["receive","--transport","relay","--relay",server.base_url(),"--mailbox",ROUTE_TOKEN_BOB,
            "--from","alice","--out",bo.to_str().unwrap(),"--max","16"]).output().unwrap();
    assert_eq!(result.status.code(),Some(86),"genuine funded retirement must fit: {}",output_text(&result));
    let after=common::directional_state(&b,"alice");let owner_after=r02_owner(&b);let charge_after=r02_observed_charge(&b);
    assert!(after["flights"].get(key).is_none());assert!(charge_after<=charge_before);
    assert!(historical(charge_after,&owner_after)>historical(charge_before,&owner_before),
        "historical 2*(inner future+margin) falsely increases liability on this real retirement");
    assert_eq!(owner_after["peers"]["carol"],owner_before["peers"]["carol"],"no other-peer credit borrowing");
    let mut ordinary=0;
    for (k,f) in before["flights"].as_object().unwrap() {if f["id"]!="" {ordinary+=1;assert_eq!(after["flights"][k],*f);}}
    assert!(ordinary>0);r02_nested_measurement(&b,"alice");r02_assert_pair(&b,"carol");
}

fn r02_witnesses(owner:&serde_json::Value,peer:&str)->std::collections::BTreeMap<u64,Vec<u8>> {
    use base64::Engine;
    let raw=base64::engine::general_purpose::STANDARD.decode(owner["peers"][peer]["control"]["recv_epoch_state"].as_str().unwrap()).unwrap();
    assert_eq!(raw.len(),154);
    raw.chunks_exact(77).filter(|e|e[44]&1!=0).map(|e|(u64::from_be_bytes(e[..8].try_into().unwrap()),e.to_vec())).collect()
}
#[test]
fn directional_r02_repeated_controls() {
    if !common::directional_case_child("directional_r02_repeated_controls") {return;}
    let (base,a,b,ao,bo)=r02_pair_fixture("r02-control-causes");
    let server=common::start_inbox_server(1024*1024,128);
    let mut partitions=std::collections::BTreeSet::new();
    let mut classes=std::collections::BTreeMap::<u8,std::collections::BTreeSet<(u64,u32,u64)>>::new();
    let mut previous=std::collections::BTreeMap::<String,(serde_json::Value,serde_json::Value)>::new();
    let mut pruned=false;let mut surviving=false;let mut retired=false;let mut replenished=false;
    // Normal sends supply the existing four-message boundary cause. No clock,
    // owner, epoch, prefix or request bit is forced. This is a finite scenario,
    // not a retry loop or a proof that every reachable maximum was enumerated.
    for round in 0..12 {
        let (sender,peer,recipient,mailbox,out)=if round%2==0 {(&a,"bob",&b,ROUTE_TOKEN_BOB,&bo)}else{(&b,"alice",&a,ROUTE_TOKEN_ALICE,&ao)};
        // Drive normal empty-inbox control preparation before each burst: four alternating polls,
        // this round's recipient first (after round 0 that is the previous sender; in round 0 it is
        // Bob), so an advertisement sealed by the next requester is receipted and consumed before
        // its control turn comes again and an in-flight control does not pre-empt the request path.
        for (cfg,mailbox,peer,out) in [(recipient,mailbox,if peer=="bob" {"alice"} else {"bob"},out),(sender,if peer=="bob" {ROUTE_TOKEN_ALICE} else {ROUTE_TOKEN_BOB},peer,if peer=="bob" {&ao} else {&bo})].iter().cycle().take(4) {
            poll_candidate(cfg,server.base_url(),mailbox,peer,out);
        }
        // Burst sizes 4 (even rounds) / 3 (odd rounds). since_boundary is cumulative until a side's own
        // boundary, so three messages do not keep the odd-round sender permanently below the four-message
        // cause; in the measured run they leave it not-due at the even-round sender's request point, so
        // the owner does not grant first and the four-message sender reaches the request path.
        for item in 0..(if round%2==0 {4} else {3}) {
            let body=format!("authenticated-control-cause-{round}-{item}");let file=base.join(format!("cause-{round}-{item}"));fs::write(&file,body.as_bytes()).unwrap();
            send_msg(sender,server.base_url(),peer,&file);
            for (cfg,p) in [(&a,"bob"),(&b,"alice")] {partitions.insert(r02_nested_measurement(cfg,p));}
            poll_candidate(recipient,server.base_url(),mailbox,if peer=="bob" {"alice"}else{"bob"},out);
            payload_once(out,body.as_bytes());
            for (cfg,p) in [(&a,"bob"),(&b,"alice")] {
                partitions.insert(r02_nested_measurement(cfg,p));
                let state=common::directional_state(cfg,p);let owner=r02_owner(cfg);
                let refs=r02_control_refs(&owner,p);let witnesses=r02_witnesses(&owner,p);
                for (class,g,n) in &refs {classes.entry(*class).or_default().insert((*g,*n,state["core"]["role"].as_u64().unwrap()));}
                if let Some((old_state,old_owner))=previous.get(p) {
                    let old_refs=r02_control_refs(old_owner,p);let old_witnesses=r02_witnesses(old_owner,p);
                    let removed:Vec<_>=old_refs.iter().filter(|r|!refs.contains(r)).collect();
                    pruned|=!removed.is_empty();
                    if !removed.is_empty() {replenished|=owner["peers"][p]["peer_future"].as_u64().unwrap()>old_owner["peers"][p]["peer_future"].as_u64().unwrap();}
                    for (epoch,w) in old_witnesses {
                        if state["recv"].get(epoch.to_string()).is_some() {
                            let now=witnesses.get(&epoch).expect("live epoch witness survives receipt pruning");
                            assert_eq!(&now[8..40],&w[8..40]);assert!(now[44]>>2>=w[44]>>2);
                            if w[44]&2!=0 {assert_ne!(now[44]&2,0);assert_eq!(&now[40..44],&w[40..44]);assert_eq!(&now[45..],&w[45..]);}
                            surviving|=removed.iter().any(|(_,g,_)|*g==epoch);
                        } else {
                            assert!(!witnesses.contains_key(&epoch));retired=true;
                            assert!(old_state["recv"].get(epoch.to_string()).is_some());
                        }
                    }
                }
                previous.insert(p.to_owned(),(state,owner));
            }
        }
    }
    assert!(partitions.contains(&(2,1)) && partitions.contains(&(1,2)),"both actual context partitions required");
    for class in 1..=4 {assert!(classes.get(&class).is_some_and(|v|v.len()>=2),"repeated authenticated cause class {class} was not reached");}
    assert!(pruned && surviving && retired && replenished,"prune/replenish and surviving/retired witnesses must be observed, never inferred from eventual success");
    r02_assert_pair(&a,"bob");r02_assert_pair(&b,"alice");
}
