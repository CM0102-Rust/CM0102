use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_club_champ.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_club_champ_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_advance_stage(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = this.current_stage() as i32;
    let last = f >= (this.stage_count() as i32).wrapping_sub(1);
    o(this.addr());
    let after = this.current_stage() as i32;
    let expected = if last { f } else if f + 1 == 1 { 2 } else { f + 1 };
    SCOREBOARD.record(after == expected, || format!("asia_club_champ_advance_stage({:#x}) fase {} -> {}", this.addr(), f, after));
}

/// Comparer for `create_group_stages`.
pub(crate) unsafe extern "thiscall" fn compare_create_group_stages(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_GROUP_STAGES);
    if this.stages() == 0 || read_usize(this.stages()) == 0 {
        return o(this.addr());
    }
    let v = group_stage_qualifiers(this);
    o(this.addr());
    let checks = |g: usize, of: &[usize]| {
        let f = League(read_usize(this.stages() + g * 4));
        let t = if !f.is_null() { f.table() } else { 0 };
        if t == 0 || f.club_count() != 4 || f.year() != this.year() {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| read_usize(t + k * 0x3b)).collect();
        let mut b = of.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = checks(1, &v[..4]) && checks(2, &v[4..]) && this.current_stage() == 2;
    SCOREBOARD.record(ok, || format!("create_group_stages({:#x}): os grupos não batem com os vivos {:x?}", this.addr(), v));
}

/// Comparer for `asia_club_champ_create_semi_final_stage`.
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_create_semi_final_stage(this: usize) {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SEMI_FINAL_STAGE);
    let stages = this.stages();
    if stages == 0 || read_usize(stages + 4) == 0 || read_usize(stages + 8) == 0 {
        return o(this.addr());
    }
    let mut plan = asia_semi_finalists(this).to_vec();
    o(this.addr());
    let stage = Cup(read_usize(this.stages() + 0xc));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("asia_club_champ_create_semi_final_stage({:#x}) sem fase", this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: Vec<usize> = (0..4).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 4 && read_usize(stage.addr() + 4) == read_usize(this.addr() + 4);
    SCOREBOARD.record(same, || format!("asia_club_champ_create_semi_final_stage({:#x}) plano={:x?} lista={:x?}", this.addr(), pl, ac));
}

/// Comparer for `asia_club_champ_create_round_table`.
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CLUB_CHAMP_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("asia_club_champ_create_round_table", stage, asian_club_champ_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_club_champ_create_round_table(this, stage, r, e, l))
}

/// Comparer for `asia_club_champ_build_team_table`.
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ASIA_CLUB_CHAMP_BUILD_TEAM_TABLE);
    compare_team_table("asia_club_champ_build_team_table", Cup(this), || o(this), || asia_club_champ_build_team_table(this))
}
