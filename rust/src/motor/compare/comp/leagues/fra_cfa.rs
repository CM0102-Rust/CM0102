use super::*;

// The comparers of the routines in ../comp/leagues/fra_cfa.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/leagues/fra_cfa.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `fra_cfa_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_cfa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_CFA_CREATE_SCHEDULE);
    compare_cfa_schedule("fra_cfa_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| fra_cfa_create_schedule(this, stage, r, e, l))
}


/// Comparer for `fra_cfa_set_up_rules` (comparison mode: see the header of motor.rs).
///
/// [pt] O calendário (a virtual +0x3c) aloca: só a original, conferindo os campos e a devolução 1.
pub(crate) unsafe extern "thiscall" fn comparar_fra_cfa_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FRA_CFA_SET_UP_RULES);
    let r = o(this);
    let b = |k: usize| *((this + k) as *const u8);
    let ok = r == 1 && core::ptr::read_unaligned((this + 0x3c) as *const u16) == 2 && b(0xc2) == 3 && b(0xc3) == 1 && b(0xc4) == 2
        && b(0xc5) == 4 && b(0xc6) == 1 && b(0xc7) == 2 && b(0x42) == 1 && b(0xbe) == 0 && b(0xbf) == 0 && b(0xc0) == 0 && b(0xc1) == 0
        && read_u32_as_usize(this + 0x1c) == read_u32_as_usize(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION) && read_u32_as_usize(this + 0x20) == 0xffff_ffff
        && core::ptr::read_unaligned((this + 0xd9) as *const u16) == 0x43;
    SCOREBOARD.record(ok, || format!("fra_cfa_set_up_rules({:#x}): devolveu {}, campos fora do esperado", this, r));
    r
}

/// Comparer for `construct_fra_cfa` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_fra_cfa(this: usize, year: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_CONSTRUCT_FRA_CFA);
    compare_construct_group_league("construct_fra_cfa", this, year, &FRA_CFA_CFA_LEAGUE, || o(this, year))
}


/// Comparer for `destroy_fra_cfa` (comparison mode: see the header of motor.rs).
///
/// [pt] Soltam memória: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_destroy_fra_cfa(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_FRA_CFA)(this)
}


/// Comparer for `fra_cfa_create_group_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_fra_cfa_create_group_table(this: usize, clubs: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_FRA_CFA_CREATE_GROUP_TABLE);
    compare_french_table("fra_cfa_create_group_table", this, clubs, 0x12, &[KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, KEY_CLUB_COMP_FRENCH_SECOND_DIVISION], || o(this, clubs))
}


