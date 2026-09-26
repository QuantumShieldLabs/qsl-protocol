//! NA-0785 PLAN F03 (FIXTURE_DESIGN sec 1): the ONE place a test reads profile values.
//!
//! No test file spells a profile string, store leaf, location variable or init selector;
//! every fixture and pin reads `ACTIVE`. `TARGET` carries the operator-allocated successor
//! values (RBANK_F03_C01_O2_O5, DOC-CAN-003 12.9) and `IMPLEMENTED` what the integration
//! head implements today. `ACTIVE` stays `&IMPLEMENTED` until the identity-landing
//! assignment flips it (ruling R2 / DD-1); that flip is one line plus the SR-18 census of
//! every remaining literal (FIXTURE_DESIGN sec 4), and is not F03's job.

pub struct SuccessorProfile {
    /// C01 A02: the profile identifier a successor vault's payload carries.
    pub id: &'static str,
    /// C01 A19: the store leaf under XDG_CONFIG_HOME ("qsc-" + TAG).
    pub store_leaf: &'static str,
    /// C01 A20: the location override; QSC_CONFIG_DIR at the implemented head.
    pub location_env: &'static str,
    /// `vault init` selector for a messaging (successor) vault.
    pub init_args: &'static [&'static str],
    /// `vault init` selector for an ordinary, storage-only vault that can never establish
    /// a directional peer (FIXTURE_DESIGN sec 2(a)).
    pub ordinary_init_args: &'static [&'static str],
    /// The refusal code `handshake init` emits from an ordinary vault.
    pub ordinary_handshake_refusal: &'static str,
}

/// The allocated successor values (RBANK_F03_C01_O2_O5, ed682274...). Not implemented by
/// the integration head; carried so the identity landing changes one line.
pub const TARGET: SuccessorProfile = SuccessorProfile {
    id: "QSL-SUCCESSOR-01",
    store_leaf: "qsc-succ01",
    location_env: "QSC_SUCC01_CONFIG_DIR",
    init_args: &["--mode", "messaging", "--protection", "local-checkpoint"],
    ordinary_init_args: &["--mode", "storage-only", "--protection", "local-checkpoint"],
    ordinary_handshake_refusal: "directional_mode_storage_only",
};

/// The development profile the integration head implements (DOC-CAN-003 sec 12; RETIRED
/// for the successor by C01 A21).
pub const IMPLEMENTED: SuccessorProfile = SuccessorProfile {
    id: "NA0780-DIR-INTEGRATION-03",
    store_leaf: "qsc",
    location_env: "QSC_CONFIG_DIR",
    init_args: &["--protocol", "directional-v1"],
    ordinary_init_args: &["--protocol", "owner-free-v1"],
    ordinary_handshake_refusal: "directional_reserve_missing",
};

/// Flipped to `&TARGET` by the identity-landing assignment ONLY.
pub const ACTIVE: &SuccessorProfile = &IMPLEMENTED;
