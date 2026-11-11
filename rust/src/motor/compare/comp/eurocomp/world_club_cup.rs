use super::*;

// The comparers of the routines in ../comp/eurocomp/world_club_cup.rs.

/// Comparer for `world_club_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_world_club_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WORLD_CLUB_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("world_club_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| world_club_cup_create_round_table(this, stage, r, e, s))
}
