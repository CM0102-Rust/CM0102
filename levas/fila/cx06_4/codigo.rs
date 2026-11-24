// ---------- cx06_4: the European Championship qualifiers' round tables, group stages and day (euro_champ_qual.cpp) ----------

/// The qualifiers' group round table: ten matchdays (`D`: day, month from 0 = January, years after the edition year,
/// weekday, code 1 or 2, value 0), each with its first match slot empty (`L`): 2 and 6 September, 7 and 11 October,
/// 24 and 28 March, 6 June, 2 and 6 September and 7 October.
pub(crate) const EURO_QUAL_GROUP_OPS: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 2, 8, 0, 5, 1, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 6, 8, 0, 2, 2, 0), Cqp12L(1, 0, -1, -1, -1, 0),
    Cqp12D(2, 7, 9, 0, 5, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0), Cqp12D(3, 0xb, 9, 0, 2, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0),
    Cqp12D(4, 0x18, 2, 1, 5, 1, 0), Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x1c, 2, 1, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 6, 5, 1, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 2, 8, 1, 5, 1, 0), Cqp12L(7, 0, -1, -1, -1, 0),
    Cqp12D(8, 6, 8, 1, 2, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0), Cqp12D(9, 7, 9, 1, 5, 1, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// The qualifiers' play-off round: the draw on 11 October of the year after the edition year (`S`, weekday 2), the
/// two legs from 11 November (`J`, weekday 5, time 1), round code 0xa0, tie rules 3, category 10, two legs (+0x21 2),
/// +0x22 4 and the prize words cleared.
pub(crate) const EURO_QUAL_PLAYOFF_OPS: [GoldcupScheduleOp; 15] = [
    Cqp12S(0, 0xb, 9, 1, 2), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x7, 0xa0), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0),
    Cqp12B(0x17, 0xa), Cqp12W(0xf, 3), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 2), Cqp12B(0x22, 4), Cqp12X(0x5c, 0),
    Cqp12X(0x60, 0), Cqp12X(0x64, 0),
];

/// The play-off size with one European host: 6 nations, 3 ties.
pub(crate) const EURO_QUAL_PLAYOFF_SIX: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 6), Cqp12W(0x1a, 3), Cqp12W(0x1c, 6)];

/// The play-off size with two European hosts: 8 nations, 4 ties.
pub(crate) const EURO_QUAL_PLAYOFF_EIGHT: [GoldcupScheduleOp; 3] = [Cqp12W(0x18, 8), Cqp12W(0x1a, 4), Cqp12W(0x1c, 8)];

/// The round table of the qualifiers' stage `stage` (signed byte): up to the last group stage (+0xff) the group table
/// (a league, 10 rounds, the stage id 0x3e9 + stage, 0x28a bytes, box line 0xbb); the play-off stage (+0x101, unless
/// it is also the runners-up stage +0x100) the play-off round (not a league, 1 round, id 0, 0x68 bytes, box line 0xf5)
/// with the size of the format (`euro_champ_qual::GROUP_FORMAT` 1 or 2); any other stage none.
pub(crate) unsafe fn euro_qual_phase(this: usize, stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f <= read_u8(this + 0xff) as i8 { // the last group stage
        return Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0x28a, row: 0xbb,
            ops: &EURO_QUAL_GROUP_OPS, ops2: &[], name: "cx06_4 qualifiers group rounds" });
    }
    if f == read_u8(this + 0x100) as i8 || f != read_u8(this + 0x101) as i8 { // the runners-up / play-off stages
        return None;
    }
    let ops2: &'static [GoldcupScheduleOp] = match read_u8(this + euro_champ_qual::GROUP_FORMAT) {
        1 => &EURO_QUAL_PLAYOFF_SIX,
        2 => &EURO_QUAL_PLAYOFF_EIGHT,
        _ => &[],
    };
    Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0xf5, ops: &EURO_QUAL_PLAYOFF_OPS, ops2,
        name: "cx06_4 qualifiers play-off round" })
}

/// Builds the round table of a qualifiers' stage (`euro_qual_phase` with the round-table machine `build_phase_table`):
/// ten group matchdays for each group, one play-off round of two legs for 6 or 8 nations. With the play-off and a
/// format other than 1 or 2, the box at line 0x11b after the table (which is still returned).
/// Original: `EuroChampQual::create_schedule` (0x57d190, euro_champ_qual.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let plan = euro_qual_phase(this, stage);
    let unknown_format = plan.is_some_and(|p| p.league == 0) && !matches!(read_u8(this + euro_champ_qual::GROUP_FORMAT), 1 | 2);
    let table = build_phase_table(League(this), plan, rounds, extra, league, 0x9c4674);
    if table != 0 && unknown_format {
        show_error_at(0x9c4674, 0x11b); // euro_champ_qual.cpp
    }
    table
}

/// Creates the League_Stage of qualifying group `g` + 1 (stage `g`).
///
/// The round table of stage `g` (`comp_vtable::CREATE_SCHEDULE`: g, &rounds, &id, 0). Without a team list, the group's
/// size (`exe::eurochampqual_get_group_team_count`(g)) and a list of its drawn teams (`euro_champ_qual::DRAWN_TEAMS`,
/// slots 6 × (g + 1) on; `exe::crt_malloc`; out of memory: the box at line 0x3b4 and 0); with `list`, the round table
/// is freed (the stage gets none) and `count` teams of `list` are used. The stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the qualifiers, the teams, the
/// list, 2, `.competition()`, the round table, the rounds, the win, draw and category bytes, &tie-breaks, &promoted,
/// the year, g, the id, the importance, 4, 0, `.league_flags()`, −1, 0, 2) into `.stages()`[g]; none: the box at line
/// 0x3d0 and 0 (the list and the round table are not freed). Then the own list and the round table freed,
/// `.current_stage()` = g, the group draw news (`exe::league_send_group_draw_news` on the stage) and 1.
/// Original: `EuroChampQual::create_group_stage` (0x57ed10, euro_champ_qual.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let l = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let mut table = schedule(this, g, &mut rounds, &mut id, 0);
    let (teams, n) = if list == 0 {
        let n = exe::eurochampqual_get_group_team_count(this, g);
        let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
        if teams == 0 {
            show_error_at(0x9c4674, 0x3b4); // euro_champ_qual.cpp
            return 0;
        }
        covered!("cx06_4 group drawn teams");
        let drawn = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
        for k in 0..n.max(0) as usize {
            write_usize(teams + 4 * k, read_usize(drawn + 4 * (6 * (g as usize + 1) + k)));
        }
        (teams, n as u16 as u32)
    } else {
        if table != 0 {
            exe::crt_free(table);
        }
        table = 0;
        (list, count as u16 as u32)
    };
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this, n, teams, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, g, id as u16 as u32,
            l.importance() as u32, 4, 0, l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages() + 4 * g as usize, stage);
    if read_usize(l.stages() + 4 * g as usize) == 0 {
        show_error_at(0x9c4674, 0x3d0);
        return 0;
    }
    if list == 0 {
        exe::crt_free(teams);
    }
    if table != 0 {
        exe::crt_free(table);
    }
    l.set_current_stage(g as usize);
    exe::league_send_group_draw_news(read_usize(l.stages() + 4 * g as usize));
    1
}

/// The qualifiers' day. Before the draw date (`oceania_nations_draw_date`: `.next_draw_day()` of `.year()` +
/// `.next_draw_year_offset()`; today's year unsigned, its day signed) nothing happens and 0 is returned (al; the
/// League day does not run).
///
/// On the draw date itself, while not `active` and with `full`: the entrants (EuroChampQual::collect_entrants 0x57f600),
/// the groups draw (`euro_champ_qual_draw_groups`), group 1's table (`init_group_one_table`; 0: the box at line 0x5a8 and
/// 0), its fixtures (`exe::league_create_fixtures`; 0: the box at line 0x5af and 0), its table sorted
/// (`exe::league_sort_table`(this, 0); 0: the box at line 0x5b6 and 0), the stages of groups 2 to 9
/// (`euro_champ_qual_create_group_stage`(g, 0, 6), g = 0..7), the drawn teams freed and cleared, the runners-up stage
/// (EuroChampQual::create_runners_up_stage 0x57f920), the predicted positions (`comp_vtable::SET_PREDICTED_POSITIONS`),
/// the entrants freed and cleared with their count, and `active` 1. On other days, when the competition has a fixture
/// on the date (`fix_man_comp_has_fixture_on_date` on a copy of today), the runners-up table
/// (EuroChampQual::update_runners_up_table 0x57fa10). Then the League day (`exe::league_update_for_date`(this, today,
/// `full`)), whose result it returns.
/// Original: `EuroChampQual::update_for_date` (0x57fd60, euro_champ_qual.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let (draw_day, draw_year) = oceania_nations_draw_date(l);
    let (day, year) = (read_i16(today), read_u16(today + 2));
    if year < draw_year || (year == draw_year && day < draw_day as i16) {
        return 0;
    }
    if year == draw_year && day as u16 == draw_day && l.active() == 0 && full != 0 {
        covered!("cx06_4 qualifiers draw day");
        let collect_entrants: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x57f600usize); // EuroChampQual::collect_entrants
        collect_entrants(this);
        euro_champ_qual_draw_groups(this);
        if init_group_one_table(this) == 0 {
            show_error_at(0x9c4674, 0x5a8); // euro_champ_qual.cpp
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9c4674, 0x5af);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0x9c4674, 0x5b6);
            return 0;
        }
        for g in 0..8 {
            euro_champ_qual_create_group_stage(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this + euro_champ_qual::DRAWN_TEAMS));
        write_usize(this + euro_champ_qual::DRAWN_TEAMS, 0);
        let create_runners_up_stage: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57f920usize); // EuroChampQual::create_runners_up_stage
        create_runners_up_stage(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xee)); // the entrants
        write_usize(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
        l.set_active(1);
    } else {
        let date = [read_u32(today), read_u32(today + 4)];
        if fix_man_comp_has_fixture_on_date(date.as_ptr() as usize, read_u32(l.competition().addr())) != 0 {
            covered!("cx06_4 qualifiers match day");
            let update_runners_up_table: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x57fa10usize); // EuroChampQual::update_runners_up_table
            update_runners_up_table(this);
        }
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Comparer for `euro_champ_qual_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes
/// written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_phase_table("euro_champ_qual_create_schedule", stage, euro_qual_phase(this, stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_qual_create_schedule(this, stage, r, e, l))
}

/// Comparer for `euro_champ_qual_create_group_stage`: it allocates the stage and sends the draw news (it cannot run
/// twice), so only the original runs; the plan: on success the stage at `.stages()`[g] with the group's teams (the
/// drawn ones or the list, as a set: the stage sorts its table), `.current_stage()` = g.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_create_group_stage(this: usize, g: u32, list: usize, count: u32) -> u32 {
    let l = League(this);
    let expected: Vec<usize> = if list == 0 {
        let drawn = read_usize(this + euro_champ_qual::DRAWN_TEAMS);
        let n = exe::eurochampqual_get_group_team_count(this, g).clamp(0, 6) as usize;
        if drawn != 0 { (0..n).map(|k| read_usize(drawn + 4 * (6 * (g as usize + 1) + k))).collect() } else { Vec::new() }
    } else {
        (0..(count as u16 as i16).clamp(0, 8) as usize).map(|k| read_usize(list + 4 * k)).collect()
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(%%I1%%)(this, g, list, count);
    if r != 1 {
        return r;
    }
    let stage = League(read_usize(l.stages() + 4 * g as usize));
    let mut got: Vec<usize> = (0..stage.club_count().clamp(0, 8) as usize).map(|k| read_usize(stage.table() + k * league_table_row::SIZE)).collect();
    let mut want = expected.clone();
    got.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(got == want && l.current_stage() == g as usize, || format!("euro_champ_qual_create_group_stage({:#x}, {}): teams {:x?} expected {:x?}", this, g, got, want));
    r
}

/// Comparer for `euro_champ_qual_update_for_date`: the day draws, allocates and updates the league (it cannot run
/// twice), so only the original runs; the plan: after the draw day, `active` 1, the drawn teams and the entrants
/// cleared and group 1's table built.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let draws = full != 0 && l.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(l);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%)(this, today, full);
    if draws {
        let ok = l.active() == 1 && read_usize(this + euro_champ_qual::DRAWN_TEAMS) == 0 && read_usize(this + 0xee) == 0 && l.table() != 0;
        SCOREBOARD.record(ok, || format!("euro_champ_qual_update_for_date({:#x}): the draw day left active {}, table {:#x}", this, l.active(), l.table()));
    }
    r
}
