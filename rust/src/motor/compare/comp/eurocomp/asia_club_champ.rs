use super::*;

// The comparers of the routines in ../comp/eurocomp/asia_club_champ.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/asia_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `asia_club_champ_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let last = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let after = read_u32_as_usize(this + 0x30) as i32;
    let expected = if last { f } else if f + 1 == 1 { 2 } else { f + 1 };
    SCOREBOARD.record(after == expected, || format!("asia_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, after));
}

/// Comparer for `create_group_stages` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_create_group_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_GROUP_STAGES);
    if read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let v = group_stage_qualifiers(this);
    o(this);
    let checks = |g: usize, de: &[usize]| {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        if t == 0 || read_i16(f + 0x3e) != 4 || read_i16(f + 0x40) != read_i16(this + 0x40) {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
        let mut b = de.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = checks(1, &v[..4]) && checks(2, &v[4..]) && read_u32_as_usize(this + 0x30) == 2;
    SCOREBOARD.record(ok, || format!("create_group_stages({:#x}): os grupos não batem com os vivos {:x?}", this, v));
}

/// Comparer for `asia_club_champ_create_semi_final_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SEMI_FINAL_STAGE);
    let stages = read_u32_as_usize(this + 0xc);
    if stages == 0 || read_u32_as_usize(stages + 4) == 0 || read_u32_as_usize(stages + 8) == 0 {
        return o(this);
    }
    let mut plan = asia_semi_finalists(this).to_vec();
    o(this);
    let stage = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 0xc);
    if stage == 0 {
        SCOREBOARD.record(false, || format!("asia_club_champ_create_semi_final_stage({:#x}) sem fase", this));
        return;
    }
    let list = read_u32_as_usize(stage + 0xa7);
    let mut found: Vec<usize> = (0..4).map(|k| read_u32_as_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + 0x3e) == 4 && read_u32_as_usize(stage + 4) == read_u32_as_usize(this + 4);
    SCOREBOARD.record(same, || format!("asia_club_champ_create_semi_final_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// Comparer for `asia_club_champ_create_round_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_CLUB_CHAMP_CREATE_ROUND_TABLE);
    asia_club_champ_compare_round_table("asia_club_champ_create_round_table", stage, asian_club_champ_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_club_champ_create_round_table(this, stage, r, e, l))
}

/// Comparer for `asia_club_champ_build_team_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ASIA_CLUB_CHAMP_BUILD_TEAM_TABLE);
    compare_team_table("asia_club_champ_build_team_table", this, || o(this), || asia_club_champ_build_team_table(this))
}
