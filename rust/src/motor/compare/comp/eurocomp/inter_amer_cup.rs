use super::*;

// The comparers of the routines in ../comp/eurocomp/inter_amer_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/inter_amer_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `inter_amer_cup_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_inter_amer_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_INTER_AMER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("inter_amer_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| inter_amer_cup_create_round_table(this, stage, r, e, s))
}
