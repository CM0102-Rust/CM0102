// ---------- rx08_5: the European World Cup qualifying: the group stages, the runners-up table and the play-off stage (wc_europe_league.cpp) ----------

/// The section's virtual `CREATE_SCHEDULE` (+0x3c) for `stage`: the round table, with the round count and the stage id
/// written into the two words.
unsafe fn wc_europe_league_schedule(this: usize, stage: u32, rounds: &mut i16, stage_id: &mut i16) -> usize {
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    create_schedule(this, stage, rounds, stage_id, 0)
}

/// rule: each European qualifying group after the section's own is a league stage of its drawn nations (or of a given
/// list), home and away with the section's points, category and tie-breaks, and its draw news goes out
///
/// `this.create_group_stage(group, teams, count)`: the group's schedule (`wc_europe_league_schedule`). Without `teams`:
/// the group's size (WcEuropeLeague::get_group_size(group), a signed word), a list of that many (`exe::crt_malloc`;
/// none: the error box of line 0x3a7 and 0) copied from the drawn slots of group + 1 (+0xf7, six per group), and the
/// count is the size; with `teams`, the schedule is freed (`exe::crt_free`) and none is passed. The stage
/// (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58:
/// the section, the count, the list, 2, the record, the schedule, the rounds, `league::WIN_POINTS`,
/// `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the tie-breaks `league::TIEBREAKS`, the places
/// `league::PROMOTED_COUNT`, the year, the group, the stage id, `league::IMPORTANCE`, 4, 0, `league::LEAGUE_FLAGS`, −1,
/// 0, 2) into `league::STAGES`[group]; none: the box of line 0x3c3 and 0. Then the drawn list and the schedule freed,
/// `league::CURRENT_STAGE` = group, the group draw news (`exe::league_send_group_draw_news` of the stage) and 1.
/// Original: `WcEuropeLeague::create_group_stage` (0x9196d0, wc_europe_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = League(this);
    let cpp = wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP;
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let mut schedule = wc_europe_league_schedule(this, group, &mut rounds, &mut stage_id);
    let (list, count) = if teams == 0 {
        let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
        let size = size_of(this, group);
        let list = exe::crt_malloc((size as i32 as u32).wrapping_mul(4));
        if list == 0 {
            show_error_at(cpp, 0x3a7);
            return 0;
        }
        let drawn = read_usize(this + 0xf7) + (group as usize + 1) * 6 * 4; // rx08_5: the group after the section's own
        for k in 0..(size as i32).max(0) as usize {
            write_usize(list + k * 4, read_usize(drawn + k * 4)); // rx08_5: the drawn nations in draw order
        }
        (list, size as u16 as u32)
    } else {
        if schedule != 0 {
            exe::crt_free(schedule);
        }
        schedule = 0;
        (teams, count)
    };
    covered!("rx08_5 european group stage");
    let p = exe::crt_operator_new(0xee);
    // simplification: the original pushes the rounds, the stage id, the bytes and the words with garbage in the high
    // halves of the registers, which the constructor does not read: the copy passes clean values
    let stage = if p != 0 {
        construct(p, this, count, list, 2, league.competition().addr(), schedule, rounds as u16 as u32, league.win_points() as u32, league.draw_points() as u32,
            league.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, league.year() as u16 as u32, group, stage_id as u16 as u32,
            read_u8(this + league::IMPORTANCE) as u32, 4, 0, read_u16(this + league::LEAGUE_FLAGS) as u32, -1, 0, 2)
    } else {
        0
    };
    let slot = league.stages() + group as usize * 4;
    write_usize(slot, stage);
    if read_usize(slot) == 0 {
        show_error_at(cpp, 0x3c3);
        return 0;
    }
    if teams == 0 {
        exe::crt_free(list);
    }
    if schedule != 0 {
        exe::crt_free(schedule);
    }
    league.set_current_stage(group as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_europe_league_create_group_stage`: it builds the stage and sends news, so only the original runs;
/// the plan: 1, the stage in its slot with the count's rows (the group's size when drawn) and the current stage the group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(%%I0%%);
    let r = o(this, group, teams, count);
    let league = League(this);
    let stage = League(read_usize(league.stages() + group as usize * 4));
    if r != 1 || stage.is_null() || (teams == 0 && read_u8(this + WC_EUROPE_EXEMPT) > 3) {
        return r;
    }
    let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let n = if teams == 0 { size_of(this, group) } else { count as i16 };
    let ok = stage.club_count() == n && league.current_stage() == group as usize;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_group_stage({:#x}, {}): {} rows of {}, current stage {}", this, group, stage.club_count(), n, league.current_stage() as i32));
    r
}

/// The European qualifying sections, from the section itself (−1) to the last group stage (`WC_EUROPE_LAST_GROUP`,
/// signed), in order; empty when the last group is below −1.
unsafe fn wc_europe_league_groups(this: usize) -> impl Iterator<Item = League> {
    let last = read_u8(this + WC_EUROPE_LAST_GROUP) as i8 as i32;
    (-1..=last).map(move |s| if s == -1 { League(this) } else { League(read_usize(League(this).stages() + s as usize * 4)) })
}

/// The club of row `k` of a league table (`league::TABLE`, `league_table_row::SIZE` bytes each).
unsafe fn table_club(league: League, k: usize) -> usize {
    read_usize(league.table() + k * league_table_row::SIZE)
}

/// A nation out of the European qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record`
/// with the competition, 0x41c and 0x64) and the end of its campaign (`clear_national_team_competition`).
unsafe fn wc_europe_league_eliminate(this: usize, club: usize) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), 0x41c, 0x64);
    clear_national_team_competition(Club(club)); // rx08_5: out of the qualifying
}

/// rule: when the European groups end, the best runners-up go to the play-off: one of them, drawn, plays the Asian
/// section, the others play each other home and away; every other nation not through (status 7) is out
///
/// Without a runners-up stage (+0x100 = −1) or a play-off stage (`WC_EUROPE_PLAY_OFF_STAGE` = −1): every row of every
/// group (`wc_europe_league_groups`) not at status 7 (`league_table_row::STATUS`) is out (`wc_europe_league_eliminate`).
/// Otherwise the play-off size N (8 with up to one exempt nation, 6 with more) and a list of N + 1 (`exe::crt_malloc`;
/// none: the error box of line 0x444 and nothing else) with the clubs of the runners-up stage's rows 0..N
/// (`exe::league_get_table_row`); one drawn (`exe::utils_random`(N + 1)) goes to Asia and its place takes the last one
/// (which stays at the end too). Each group row: a club among the first N of the list (the first match), or the Asian
/// one, goes on (status 1) with the qualified record for round 0xa0 (`european_cup_add_qualified_record`(club, the
/// competition, 0, 0xa0, 0x1e)); the Asian section's record (`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`) only
/// when the list entry found is the Asian one; any other row not at status 7 is out. Then, on the last group processed
/// (the runners-up stage when there is no group), its first rows down to its row count less N get the section's
/// virtual `SET_CLUB_PROGRESS`(club, −1, 1, 1, 1). The play-off schedule (`wc_europe_league_schedule` of the play-off
/// stage), the stage (`exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the section, N, the list, the rounds, the record, the schedule, the year, the play-off stage, 4, the stage id,
/// 0x14, 0, 0, 0, 0) into `league::STAGES`[play-off]; the list and the schedule freed; no stage: the box of line 0x49f;
/// otherwise its first round (`exe::cup_set_up_round`(stage, 0)) and `league::CURRENT_STAGE` the play-off stage.
/// Original: `WcEuropeLeague::create_play_off_stage` (0x919910, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_play_off_stage(this: usize) {
    let league = League(this);
    let runners_up = read_u8(this + 0x100) as i8;
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let not_through = |l: League, k: usize| read_u8(l.table() + k * league_table_row::SIZE + league_table_row::STATUS) != 7;
    if runners_up == -1 || play_off == -1 {
        covered!("rx08_5 european groups end without a play-off");
        for group in wc_europe_league_groups(this) {
            for k in 0..group.club_count().max(0) as usize {
                if not_through(group, k) {
                    wc_europe_league_eliminate(this, table_club(group, k));
                }
            }
        }
        return;
    }
    let n: i16 = if read_u8(this + WC_EUROPE_EXEMPT) <= 1 { 8 } else { 6 }; // rx08_5: eight play-off places with up to one exempt nation
    let list = exe::crt_malloc((n as u32 + 1) * 4);
    if list == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x444);
        return;
    }
    covered!("rx08_5 european play-off");
    let runners_up_stage = League(read_usize(league.stages() + runners_up as isize as usize * 4));
    for k in 0..=n as usize {
        write_usize(list + k * 4, read_usize(exe::league_get_table_row(runners_up_stage, k as i32)));
    }
    let drawn = exe::utils_random(n as i32 + 1) as usize;
    let asia = read_usize(list + drawn * 4);
    write_usize(list + drawn * 4, read_usize(list + n as usize * 4));
    let mut last = runners_up_stage;
    for group in wc_europe_league_groups(this) {
        last = group;
        for k in 0..group.club_count().max(0) as usize {
            let club = table_club(group, k);
            let found = (0..n as usize).find(|&j| read_usize(list + j * 4) == club).unwrap_or(n as usize);
            if found < n as usize || club == asia {
                write_u8(group.table() + k * league_table_row::SIZE + league_table_row::STATUS, 1);
                let comp = if read_usize(list + found * 4) == asia { club_comp_record(read_usize(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)) } else { league.competition().addr() };
                european_cup_add_qualified_record(club, comp, 0, 0xa0, 0x1e);
            } else if not_through(group, k) {
                wc_europe_league_eliminate(this, club);
            }
        }
    }
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    for k in 0..(last.club_count() as i32 - n as i32).max(0) as usize {
        progress(this, table_club(last, k), -1, 1, 1, 1);
    }
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let schedule = wc_europe_league_schedule(this, play_off as u8 as u32, &mut rounds, &mut stage_id);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, list, rounds as u16 as u32, league.competition().addr(), schedule, league.year() as u16 as u32, play_off as u8 as u32, 4, stage_id as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = league.stages() + play_off as isize as usize * 4;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(schedule);
    if read_usize(slot) == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x49f);
        return;
    }
    exe::cup_set_up_round(read_usize(slot), 0);
    league.set_current_stage(play_off as isize as usize);
}

/// Comparer for `wc_europe_league_create_play_off_stage`: it draws, writes records and builds the stage, so only the
/// original runs; the plan: with both stages set, the current stage is the play-off stage, which exists.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let both = read_u8(this + 0x100) != 0xff && read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) != 0xff;
    o(this);
    if both {
        let league = League(this);
        let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8 as isize as usize;
        let stage = read_usize(league.stages() + play_off * 4);
        SCOREBOARD.record(stage != 0 && league.current_stage() == play_off, || format!("wc_europe_league_create_play_off_stage({:#x}): stage {:#x}, current {}", this, stage, league.current_stage() as i32));
    }
}

/// rule: the European runners-up are ranked by their record against the top four of their own group: each group's
/// runner-up row in the runners-up table is the one of a four-team sub-league of the group's games among its top four
///
/// The period: from the first round date to the last one (`league::ROUND_DATES`, day and year offset + the year;
/// Date::Date `exe::new_date`). For each group (`wc_europe_league_groups`): its table sorted (`exe::league_sort_table`),
/// its runner-up (row 1, `exe::league_get_table_row`) and the ids of its top four in a new list (`exe::crt_malloc`(0x10);
/// none: the error box of line 0x57c and return, without the final sort); the sub-league of their games
/// (`exe::crt_operator_new`(0xee) and SubLeague::SubLeague `FN_SUBLEAGUE_SUBLEAGUE`, thiscall, ret 0x30: 4, the ids, −1,
/// −1, the two dates, the competition id, `league::WIN_POINTS`, `league::DRAW_POINTS`, the tie-breaks, 2 × n × (n − 1)
/// with n the group's row count, and the section's stored results +0x10); when the runner-up is one of its first 4
/// rows, that row is copied into the next row of the runners-up table (+0x100; the row advances only on a copy) with its
/// head-to-head and position-history pointers cleared (the sub-league owns them); the sub-league destroyed (its virtual
/// 0 with 1) and the list freed. At the end the runners-up table sorted.
/// Original: `WcEuropeLeague::fill_runners_up_table` (0x91a240, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_fill_runners_up_table(this: usize) {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, i32, i32, usize, usize, i32, u32, u32, usize, u32, usize) -> usize = core::mem::transmute(FN_SUBLEAGUE_SUBLEAGUE);
    let league = League(this);
    let (mut from, mut to) = ([0u8; 8], [0u8; 8]);
    let dates = league.round_dates();
    exe::new_date(from.as_mut_ptr() as usize, read_u16(dates) as u32, read_u16(dates + 2).wrapping_add(league.year() as u16) as u32);
    let last = dates.wrapping_add((league.round_count() as i32 as usize).wrapping_mul(league_round_date::SIZE)).wrapping_sub(league_round_date::SIZE);
    exe::new_date(to.as_mut_ptr() as usize, read_u16(last) as u32, read_u16(last + 2).wrapping_add(league.year() as u16) as u32);
    let runners_up = || League(read_usize(league.stages() + read_u8(this + 0x100) as i8 as isize as usize * 4));
    let mut row = 0usize;
    for group in wc_europe_league_groups(this) {
        exe::league_sort_table(group, 0);
        let runner = read_usize(exe::league_get_table_row(group, 1));
        let ids = exe::crt_malloc(0x10);
        if ids == 0 {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x57c);
            return;
        }
        for i in 0..4 {
            write_u32(ids + 4 * i as usize, read_u32(read_usize(exe::league_get_table_row(group, i))));
        }
        let p = exe::crt_operator_new(0xee);
        let n = group.club_count() as i32;
        let sub = if p != 0 {
            construct(p, 4, ids, -1, -1, from.as_ptr() as usize, to.as_ptr() as usize, read_i32(league.competition().addr()), league.win_points() as u32,
                league.draw_points() as u32, this + league::TIEBREAKS, n.wrapping_sub(1).wrapping_mul(n).wrapping_mul(2) as u32, read_usize(this + 0x10)) // rx08_5: both legs between the four
        } else {
            0
        };
        if let Some(j) = (0..4).find(|&j| read_usize(exe::league_get_table_row(sub, j)) == runner) {
            covered!("rx08_5 european runner-up row copied");
            let target = runners_up().table() + row;
            core::ptr::copy_nonoverlapping((League(sub).table() + j as usize * league_table_row::SIZE) as *const u8, target as *mut u8, league_table_row::SIZE);
            write_u32(target + league_table_row::POSITION_HISTORY, 0);
            write_u32(target + league_table_row::H2H_ENTRIES, 0);
            row += league_table_row::SIZE;
        }
        if sub != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(sub) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(sub, 1);
        }
        exe::crt_free(ids);
    }
    exe::league_sort_table(runners_up(), 0);
}

/// Comparer for `wc_europe_league_fill_runners_up_table`: side by side on the runners-up table's rows; the groups'
/// sorts and the sub-leagues are the same on both sides.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_fill_runners_up_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let stage = League(read_usize(League(this).stages() + read_u8(this + 0x100) as i8 as isize as usize * 4));
    if stage.is_null() || stage.table() == 0 {
        return o(this);
    }
    let regions = [(stage.table(), stage.club_count().clamp(0, 10) as usize * league_table_row::SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || wc_europe_league_fill_runners_up_table(this));
    SCOREBOARD.record(same, || format!("wc_europe_league_fill_runners_up_table({:#x}): {}", this, first_differences(&mo, &mr).join(", ")));
}
