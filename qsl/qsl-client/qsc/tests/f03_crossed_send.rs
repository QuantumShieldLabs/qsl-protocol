// NA-0785 PLAN F03 / S5 -- INSTRUMENTS (c): THE A1 CROSSED-SEND REGRESSION (F00 R2; K-09).
//
// Two REAL peers (init_real_pair over the real in-process leasing qsl-server: 2 s pull lease,
// every pull after a 3 s wait so an unACKed item is redeliverable) send CROSSED -- both send
// before either pulls -- for two rounds, then a sequential permanence probe in each direction.
// DEFAULT receipts (the head refuses --receipt-mode off). Every one of the 12 steps must hold:
// each send is accepted, and each receive decrypts the exact bytes exactly once.
//
// Delta symbol: the receive dispatch. main routes an ordinary frame to the legacy shared-root
// path; a crossed round there fails `qsp_scka_adv code=qsp_auth_failed` -> qsp_scka_adv_reject and
// the rejected item wedges the direction, probe included (RED at main 6c601568 with default
// receipts, measured by the seat before this file existed, 5 of 6 receives failing, twice). The
// head routes it to protocol_state::directional_receive_update (transport/mod.rs:492 ->
// protocol_state/mod.rs:1308) and the directional core's exclusive root transition: GREEN.
// A synthetic local run; it says nothing about production or the real relay deployment.

mod common;

use common::{PairRelay, VaultFixture};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

fn private_dir(path: &Path) {
    fs::create_dir_all(path).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o700)).unwrap();
    }
}

fn combined(out: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    )
}

#[test]
fn crossed_sends_over_the_leasing_relay_do_not_wedge() {
    let server = common::start_qsl_server_with_store(1024 * 1024, 32, None, 2);
    let relay = server.base_url().to_string();
    let pair = common::init_real_pair(
        "f03_c_crossed",
        PairRelay::Leasing(&server),
        ("alice", "f03_c_route_alice_0123456789"),
        ("bob", "f03_c_route_bob_0123456789ab"),
    );
    let (alice, bob) = (&pair.a, &pair.b);
    let work = alice.iso.root.join("steps");
    private_dir(&work);

    let send = |from: &VaultFixture, to: &str, step: &str| -> bool {
        let body = work.join(format!("{step}.bin"));
        fs::write(&body, step.as_bytes()).unwrap();
        let out = from.run(&[
            "send",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--to",
            to,
            "--file",
            body.to_str().unwrap(),
        ]);
        println!(
            "F03C step={step} op=send exit={:?}\n{}--- end {step}",
            out.status.code(),
            combined(&out)
        );
        out.status.success()
    };
    let receive = |at: &VaultFixture, mailbox: &str, from: &str, step: &str, want: &str| -> bool {
        // > the 2 s lease: anything leased but not ACKed by an earlier pull is redeliverable.
        thread::sleep(Duration::from_secs(3));
        let out_dir = work.join(format!("out_{step}"));
        private_dir(&out_dir);
        let out = at.run(&[
            "receive",
            "--transport",
            "relay",
            "--relay",
            &relay,
            "--mailbox",
            mailbox,
            "--from",
            from,
            "--max",
            "8",
            "--out",
            out_dir.to_str().unwrap(),
        ]);
        // The head names outputs recv_<hash>.bin and may re-project retained events into a fresh
        // --out, so the wanted bytes must be present EXACTLY ONCE.
        let hits = fs::read_dir(&out_dir)
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter(|entry| {
                let name = entry.file_name().to_string_lossy().into_owned();
                name.starts_with("recv_") && name.ends_with(".bin")
            })
            .filter(|entry| fs::read(entry.path()).unwrap() == want.as_bytes())
            .count();
        let ok = out.status.success() && hits == 1;
        println!(
            "F03C step={step} op=recv exit={:?} want={want} hits={hits} ok={ok}\n{}--- end {step}",
            out.status.code(),
            combined(&out)
        );
        ok
    };

    let mut results: Vec<(String, bool)> = Vec::new();
    for round in ["r1", "r2"] {
        // CROSSED: both peers send before either pulls.
        let a_body = format!("{round}-alice");
        let b_body = format!("{round}-bob");
        results.push((format!("{round}-send-alice"), send(alice, "bob", &a_body)));
        results.push((format!("{round}-send-bob"), send(bob, "alice", &b_body)));
        results.push((
            format!("{round}-bob-recv(alice->bob)"),
            receive(
                bob,
                &pair.b_route,
                "alice",
                &format!("{round}-bob-recv"),
                &a_body,
            ),
        ));
        results.push((
            format!("{round}-alice-recv(bob->alice)"),
            receive(
                alice,
                &pair.a_route,
                "bob",
                &format!("{round}-alice-recv"),
                &b_body,
            ),
        ));
    }
    // Permanence probe: sequential, not crossed.
    results.push(("p-send-alice".into(), send(alice, "bob", "p-alice")));
    results.push((
        "p-bob-recv(alice->bob)".into(),
        receive(bob, &pair.b_route, "alice", "p-bob-recv", "p-alice"),
    ));
    results.push(("p-send-bob".into(), send(bob, "alice", "p-bob")));
    results.push((
        "p-alice-recv(bob->alice)".into(),
        receive(alice, &pair.a_route, "bob", "p-alice-recv", "p-bob"),
    ));

    for (step, ok) in &results {
        println!("F03C RESULT {step} ok={ok}");
    }
    let failed: Vec<&String> = results
        .iter()
        .filter(|(_, ok)| !ok)
        .map(|(s, _)| s)
        .collect();
    assert_eq!(results.len(), 12, "every step ran");
    assert!(
        failed.is_empty(),
        "F03C crossed sends wedged or lost: {failed:?}"
    );
}
