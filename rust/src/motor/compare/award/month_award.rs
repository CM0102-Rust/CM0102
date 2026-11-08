use super::*;

// The comparers of the routines in ../award/month_award.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../award/month_award.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `award_best_rated_player` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_award_best_rated_player(this: usize, notes: usize, age: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_AWARD_BEST_RATED_PLAYER)(this, notes, age)
}

/// Comparer for `set_winner` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_set_winner(this: usize, id: i32, club_id: i32, month: u32, year: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, u32)>(IDX_SET_WINNER);
    let mut regions = vec![(this + 0x1d, 12 * 11)];
    if id > -1 {
        winner_reputation_regions(&mut regions, month_ratings_staff_by_id(id), if club_id > -1 { month_award_club_by_id(club_id) } else { 0 });
    }
    compare_month_award("set_winner", &regions, || o(this, id, club_id, month, year), || set_winner(this, id, club_id, month, year));
}

