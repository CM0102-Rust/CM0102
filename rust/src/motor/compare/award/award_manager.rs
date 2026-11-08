use super::*;

// The comparers of the routines in ../award/award_manager.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../award/award_manager.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `award_raise_winner_reputation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn compare_award_raise_winner_reputation(person: usize, club: usize, nation: u32, mundo: u32, player: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(IDX_AWARD_RAISE_WINNER_REPUTATION);
    let mut regions = Vec::new();
    winner_reputation_regions(&mut regions, person, club);
    compare_month_award("award_raise_winner_reputation", &regions, || o(person, club, nation, mundo, player), || award_raise_winner_reputation(person, club, nation, mundo, player));
}
