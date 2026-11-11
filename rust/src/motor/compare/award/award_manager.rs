use super::*;

// The comparers of the routines in ../award/award_manager.rs.

/// Comparer for `award_raise_winner_reputation`.
pub(crate) unsafe extern "C" fn compare_award_raise_winner_reputation(person: usize, club: usize, nation: u32, world: u32, player: u32) {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, u32, u32, u32)>(IDX_AWARD_RAISE_WINNER_REPUTATION);
    let mut regions = Vec::new();
    winner_reputation_regions(&mut regions, person, club);
    compare_month_award("award_raise_winner_reputation", &regions, || o(person, club, nation, world, player), || award_raise_winner_reputation(person, club, nation, world, player));
}
