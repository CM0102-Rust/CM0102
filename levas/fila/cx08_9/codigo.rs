// ---------- cx08_9: the Argentine Second Division stages and its three zones (arg_second.cpp) ----------

/// The League_Stage slot the Argentine Second Division asks before its promotion cup: the Premier Division's first
/// stage must be over (slot 0xc, Comp::all_stages_complete in the League vtable) [inference: the al it returns].
const STAGE_COMPLETE_SLOT: usize = 0xc;

/// Rule: the Argentine Second Division moves through its stages: the three zones after the first phase (stage 0), the
/// promotion cup after the zones (stage 3, only once the Premier Division's Clausura is over) and the promotion
/// play-offs (stage 4).
///
/// `ArgSecond::advance_stage()`: at the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the
/// arg_second.cpp box of line 0x2eb and nothing else. At stage 3, when the Premier Division's (`G_COMP_OBJECT_TABLE` by
/// `KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`) stage 0 answers 0 to `STAGE_COMPLETE_SLOT` (its al), nothing. Otherwise
/// the current stage + 1 and, by the new stage, 0: ArgSecond::create_zone_stages (0x4081b0), 3:
/// ArgSecond::create_promotion_cup_stage (0x408630), 4: ArgSecond::create_promotion_playoffs (0x408910) (thiscall, ret
/// 0, through their detours).
/// Original: `ArgSecond::advance_stage` (0x4080c0, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(ARG_SECOND_FILE, 0x2eb);
    }
    if stage == 3 {
        let first = read_usize(League(comp_by_global(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION)).stages());
        let done: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(first) + STAGE_COMPLETE_SLOT));
        if done(first) as u8 == 0 {
            return;
        }
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("cx08_9 argentine second next stage");
    match stage {
        0 => call_on_league(0x4081b0, l),
        3 => call_on_league(0x408630, l),
        4 => call_on_league(0x408910, l),
        _ => {}
    }
}

/// Comparer for `arg_second_advance_stage`: only the original (it may build stages); afterwards the current stage
/// must be the same (at the last stage, or waiting at 3) or one more.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_advance_stage(this: usize) {
    let l = League(this);
    let before = l.current_stage() as i32;
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let after = l.current_stage() as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("arg_second_advance_stage({:#x}) before={} after={}", this, before, after));
}

/// The first-phase table rows of each of the three zones (row 0, the first-phase winner, is in none).
const ARG_SECOND_ZONE_ROWS: [[usize; 8]; 3] = [[1, 10, 11, 12, 13, 14, 15, 24], [2, 7, 8, 9, 16, 17, 18, 23], [3, 4, 5, 6, 19, 20, 21, 22]];

/// Rule: after the first phase the 24 Argentine Second Division clubs below the leader are split into three zones of
/// eight by their place (zone A 2nd, 11th to 16th and 25th; zone B 3rd, 8th to 10th, 17th to 19th and 24th; zone C 4th
/// to 7th and 20th to 23rd), each a home-and-away league stage.
///
/// `ArgSecond::create_zone_stages()`: the list (`exe::crt_malloc` of 0x60; none: the arg_second.cpp box of line 0x31b
/// and nothing else) with the clubs of the table rows of `ARG_SECOND_ZONE_ROWS`, zone after zone; then for each zone g:
/// its round table (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)), a League_Stage
/// (`exe::crt_operator_new`(0xee); League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this league, 8 clubs, the
/// zone's part of the list, 2 legs, the competition, the table and its count, the win, draw and category bytes, the
/// tie-breaks, the places (1, 0, 0, 0), the year, g, the label, the importance, 1, 0, rule bits 8, −1, prize fund 0 and
/// prize mode 2; 0 without memory) into stage slot g and the round table freed. With all three stages,
/// `league::CURRENT_STAGE` = 2; otherwise the box of line 0x359.
/// Defect of the original: the zone list is never freed (96 bytes every season).
/// Original: `ArgSecond::create_zone_stages` (0x4081b0, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_create_zone_stages(this: usize) {
    let this = League(this);
    let list = exe::crt_malloc(0x60);
    if list == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x31b);
    }
    for (g, rows) in ARG_SECOND_ZONE_ROWS.iter().enumerate() {
        for (k, &r) in rows.iter().enumerate() {
            write_usize(list + 0x20 * g + 4 * k, read_usize(this.table() + r * league_table_row::SIZE));
        }
    }
    covered!("cx08_9 argentine zones");
    let places: [u8; 4] = [1, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    for g in 0..3u32 {
        let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
        let p = exe::crt_operator_new(0xee);
        let s = if p != 0 {
            ctor(p, this.addr(), 8, list + 0x20 * g as usize, 2, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32,
                this.draw_points() as u32, this.match_category() as u32, this.addr() + league::TIEBREAKS, places.as_ptr() as usize, this.year() as u16 as u32,
                g, label as u16 as u32, this.importance() as u32, 1, 0, 8, -1, 0, 2)
        } else {
            0
        };
        write_usize(this.stages() + 4 * g as usize, s);
        exe::crt_free(table);
    }
    if (0..3).all(|g| read_usize(this.stages() + 4 * g) != 0) {
        this.set_current_stage(2);
    } else {
        show_error_at(ARG_SECOND_FILE, 0x359);
    }
}

/// Comparer for `arg_second_create_zone_stages`: only the original (it builds the stages); afterwards the three zone
/// stages must hold the clubs of their rows (planned beforehand) and the current stage must be 2.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_create_zone_stages(this: usize) {
    let l = League(this);
    let plan: Vec<usize> = ARG_SECOND_ZONE_ROWS.iter().flatten().map(|&r| read_usize(l.table() + r * league_table_row::SIZE)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let mut ok = l.current_stage() == 2;
    for g in 0..3usize {
        let s = League(read_usize(l.stages() + 4 * g));
        ok = ok && !s.is_null() && s.club_count() == 8;
        if ok {
            let mut zone: Vec<usize> = (0..8).map(|k| read_usize(s.table() + k * league_table_row::SIZE)).collect();
            let mut want = plan[8 * g..8 * g + 8].to_vec();
            zone.sort();
            want.sort();
            ok = zone == want;
        }
    }
    SCOREBOARD.record(ok, || format!("arg_second_create_zone_stages({:#x}): current {}", this, l.current_stage() as i32));
}
