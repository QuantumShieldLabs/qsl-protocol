// NA-0698 CONTROL P — the GATE-LIVENESS control (D-1338; directive D632 §6b as restated
// by Amendment 1 A1.8 / Director ruling R20).
//
// THIS FILE EXISTS ONLY ON THE THROWAWAY BRANCH `na0698-ci-probe` AND NEVER MERGES.
// It proves the newly added `qsc-sharded-suite` gate can REPORT FAILURE: a deliberately
// failing test must turn `qsc-shard-4` AND the aggregate `qsc-sharded-suite` RED on a pull
// request, while every one of its shard-mates still EXECUTES and reconciles by name under
// `--no-fail-fast`.
//
// A green run alone is not evidence that a gate can fail.

#[test]
fn ci_phase1_deliberate_red_probe() {
    panic!("CI_PHASE1_PROBE — deliberate red; this branch never merges");
}
