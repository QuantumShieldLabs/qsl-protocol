use quantumshield_refimpl::crypto::stdcrypto::StdCrypto;
use quantumshield_refimpl::suite2::establish::init_from_base_handshake;
use quantumshield_refimpl::suite2::types::{SUITE2_PROTOCOL_VERSION, SUITE2_SUITE_ID};

fn derive_send_hk(dh_init: [u8; 32], pq_init_ss: [u8; 32], sid: [u8; 16]) -> [u8; 32] {
    let c = StdCrypto;
    let st = init_from_base_handshake(
        &c,
        true,
        SUITE2_PROTOCOL_VERSION,
        SUITE2_SUITE_ID,
        &sid,
        &dh_init,
        &pq_init_ss,
        &[0xA1; 32],
        &[0xB2; 32],
        true,
    )
    .expect("base handshake derive");
    st.send.hk_s
}

#[test]
fn suite2_init_depends_on_both_pq_and_dh_inputs() {
    let sid = [0x11; 16];
    let dh_x = [0x22; 32];
    let dh_x2 = [0x23; 32];
    let pq_p = [0x44; 32];
    let pq_p2 = [0x45; 32];

    let hk_a = derive_send_hk(dh_x, pq_p, sid);
    let hk_b = derive_send_hk(dh_x2, pq_p, sid);
    let hk_c = derive_send_hk(dh_x, pq_p2, sid);

    assert_ne!(hk_a, hk_b, "dh_init must affect key schedule");
    assert_ne!(hk_a, hk_c, "pq_init_ss must affect key schedule");
}
