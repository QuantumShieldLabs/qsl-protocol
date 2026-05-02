use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::ratchet::{recv_nonboundary_ooo, Suite2RecvState};
use quantumshield_refimpl::suite2::types;

fn recv_state() -> Suite2RecvState {
    Suite2RecvState {
        session_id: [0x24; 16],
        protocol_version: types::SUITE2_PROTOCOL_VERSION,
        suite_id: types::SUITE2_SUITE_ID,
        dh_pub: [0x41; 32],
        hk_r: [0x52; 32],
        ck_ec: [0x63; 32],
        ck_pq: [0x74; 32],
        nr: 7,
        mkskipped: Vec::new(),
    }
}

fn snapshot_recv(st: &Suite2RecvState) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&st.session_id);
    out.extend_from_slice(&st.protocol_version.to_be_bytes());
    out.extend_from_slice(&st.suite_id.to_be_bytes());
    out.extend_from_slice(&st.dh_pub);
    out.extend_from_slice(&st.hk_r);
    out.extend_from_slice(&st.ck_ec);
    out.extend_from_slice(&st.ck_pq);
    out.extend_from_slice(&st.nr.to_be_bytes());
    out.extend_from_slice(&(st.mkskipped.len() as u32).to_be_bytes());
    for entry in &st.mkskipped {
        out.extend_from_slice(&entry.dh_pub);
        out.extend_from_slice(&entry.n.to_be_bytes());
        out.extend_from_slice(&entry.mk);
    }
    out
}

#[test]
fn capability_commitment_flags_mismatch_rejects_without_mutation() {
    let crypto = StdCrypto;
    let state = recv_state();
    let before = snapshot_recv(&state);
    let hdr_ct = [0u8; 24];
    let body_ct = [0u8; 16];

    let out1 = recv_nonboundary_ooo(
        &crypto,
        &crypto,
        &crypto,
        state.clone(),
        types::FLAG_BOUNDARY,
        &hdr_ct,
        &body_ct,
    );
    let out2 = recv_nonboundary_ooo(
        &crypto,
        &crypto,
        &crypto,
        state.clone(),
        types::FLAG_BOUNDARY,
        &hdr_ct,
        &body_ct,
    );

    assert!(!out1.ok);
    assert_eq!(out1.reason, Some("REJECT_S2_LOCAL_UNSUPPORTED"));
    assert_eq!(out1.reason, out2.reason);
    assert_eq!(before, snapshot_recv(&out1.state));
    assert_eq!(before, snapshot_recv(&out2.state));
}
