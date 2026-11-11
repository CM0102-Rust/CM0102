use super::*;

// The comparers of the routines in ../comp/leagues/jap_j1.rs.

/// Comparer for `jap_j1_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_advance_stage(this: usize) {
    let this = League(this);
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_JAP_J1_ADVANCE_STAGE)(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { before } else { before + 1 }, || format!("jap_j1_advance_stage({:#x}) before={} after={}", this.addr(), before, after));
}

/// Comparer for `create_championship_final_stage`.
///
/// Only the original runs (it allocates the list, the table and the stage and draws the round), checked against the
/// plan made beforehand: the champions of the two halves. Afterwards, the stage of the third slot exists, with +0x3e =
/// 2, +4 = the league's +4 and the two champions in its list (+0xa7, 6 bytes; as a set), and +0x30 = 2.
pub(crate) unsafe extern "thiscall" fn compare_create_championship_final_stage(this: usize) {
    let this = League(this);
    let (a, b) = (half_champion(this, 0), half_champion(this, 1));
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_CHAMPIONSHIP_FINAL_STAGE)(this.addr());
    let stage = Cup(read_usize(this.stages() + 8));
    let same = !stage.is_null() && {
        let l = stage.team_table();
        let (x, y) = (read_usize(l), read_usize(l + 6));
        stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && ((x, y) == (a, b) || (x, y) == (b, a)) && this.current_stage() == 2
    };
    SCOREBOARD.record(same, || format!("create_championship_final_stage({:#x}) champions={:#x},{:#x} stage={:#x} +0x30={}", this.addr(), a, b, stage.addr(), this.current_stage()));
}

/// Comparer for `jap_j1_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let before = this.year();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_START_NEW_SEASON)(this.addr());
    let after = this.year();
    SCOREBOARD.record(after == before.wrapping_add(1) && this.stage_count() == 3, || format!("jap_j1_start_new_season({:#x}) year before={} after={} +0x2c={}", this.addr(), before, after, this.stage_count()));
    r
}

/// Comparer for `jap_j1_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_CREATE_TABLE);
    compare_japanese_table("jap_j1_create_table", League(this), 0x10, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, true, 0x186a0, || o(this))
}

/// Comparador: como o do Goiano.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_set_up_rules(this: usize) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_JAP_J1_SET_UP_RULES)(this), || {
        j1_rules(this);
        1
    });
    SCOREBOARD.record(igual && r == n, || format!("jap_j1_set_up_rules({:x}) original={} rust={} {:?} {:?}", this, r, n, mo, mr));
    r
}

/// Comparer for `jap_j1_create_half_season_stage`.
pub(crate) unsafe extern "thiscall" fn compare_jap_j1_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_JAP_J1_CREATE_HALF_SEASON_STAGE);
    compare_half_season_stage("jap_j1_create_half_season_stage", League(this), k, || o(this, k))
}
