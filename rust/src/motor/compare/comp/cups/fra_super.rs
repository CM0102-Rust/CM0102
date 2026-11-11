use super::*;

// The comparers of the routines in ../comp/cups/fra_super.rs.

/// Comparer for `fra_super_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_fra_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_FRA_SUPER_CREATE_ROUND_TABLE);
    if this.year() != 0x7d1 {
        return o(this.addr(), stage, rounds, extra, out);
    }
    compare_cup_round_table("fra_super_create_round_table", stage, rounds, extra, out, || o(this.addr(), stage, rounds, extra, out),
        |r, e, s| fra_super_create_round_table(this.addr(), stage, r, e, s))
}

/// Comparer for `fra_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_fra_super_select_teams(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_FRA_SUPER_SELECT_TEAMS);
    eng_cc_cup_compare_entrant_list("fra_super_select_teams", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || french_super_cup_select_teams(this),
        |r, _| if r == 1 { 12 } else { 0 })
}
