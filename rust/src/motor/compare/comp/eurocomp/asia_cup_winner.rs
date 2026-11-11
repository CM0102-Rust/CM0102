use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_cup_winner.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_cup_winner.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_cup_winner_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_create_knockout_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_CREATE_KNOCKOUT_STAGE);
    let stages = this.stages();
    if stages == 0 || (0..3).any(|g| read_usize(stages + g * 4) == 0) || this.table() == 0 {
        return o(this.addr());
    }
    let table = |g: i32| if g < 0 { this.table() } else { read_usize(read_usize(stages + g as usize * 4) + league::TABLE) };
    let mut plan: Vec<usize> = AFRICAN_NATIONS_QUARTER_FINAL_SLOTS.iter().map(|&(g, p)| read_usize(table(g) + p as usize * 0x3b)).collect();
    o(this.addr());
    let stage = Cup(read_usize(this.stages() + 0xc));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("asia_cup_winner_create_knockout_stage({:#x}) without stage", this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: Vec<usize> = (0..8).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 8 && read_usize(stage.addr() + 4) == this.competition().addr() && this.current_stage() == 3;
    SCOREBOARD.record(same, || format!("asia_cup_winner_create_knockout_stage({:#x}) plan={:x?} list={:x?}", this.addr(), pl, ac));
}

/// Comparer for `asia_cup_winner_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_advance_stage(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CUP_WINNER_ADVANCE_STAGE);
    let f = this.current_stage() as i32;
    let last = f >= (this.stage_count() as i32).wrapping_sub(1);
    o(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("asia_cup_winner_advance_stage({:#x}) stage {} -> {}", this.addr(), f, after));
}

/// Comparer for `create_finals_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_finals_stage(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_FINALS_STAGE);
    if this.team_table() == 0 || this.stages() == 0 || read_usize(this.stages()) == 0 {
        return o(this.addr());
    }
    let mut plan = asian_cup_winners_finalists(this).to_vec();
    o(this.addr());
    let stage = Cup(read_usize(this.stages() + 4));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("create_finals_stage({:#x}) without stage", this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: Vec<usize> = (0..4).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 4 && read_usize(stage.addr() + 4) == read_usize(this.addr() + 4);
    SCOREBOARD.record(same, || format!("create_finals_stage({:#x}) plan={:x?} list={:x?}", this.addr(), pl, ac));
}

/// Comparer for `asia_cup_winner_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_asia_cup_winner_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CUP_WINNER_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("asia_cup_winner_create_round_table", stage, asian_cup_winners_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_cup_winner_create_round_table(this, stage, r, e, l))
}

/// Comparer for `create_team_entries`.
pub(crate) unsafe extern "thiscall" fn compare_create_team_entries(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CREATE_TEAM_ENTRIES);
    compare_team_table("create_team_entries", Cup(this), || o(this), || create_team_entries(this))
}
