// ---------- sbH_1: the Argentine Second Division promotion cup and promotion play-offs (arg_second.cpp) ----------

/// The season column (a signed byte, 0..3) the by-season order of the averages rows compares
/// (`comp_compare_average_rows_by_season`, 0x48f450); the Argentine Second Division sets it to 2, the season being
/// played.
const ARG_AVERAGE_SORT_SEASON: usize = 0xadae04;
/// The qsort orders of the averages rows: by the points average (`comp_compare_average_rows`) and by the points of
/// one season (`comp_compare_average_rows_by_season`).
const ORDER_AVERAGE_ROWS: usize = 0x48f410;
const ORDER_AVERAGE_ROWS_BY_SEASON: usize = 0x48f450;
/// The last club the Argentine Second Division relegated by averages (a club pointer at League +0xee) [inference: only
/// written here].
const ARG_SECOND_RELEGATED_CLUB: usize = 0xee;

/// A knockout stage of a league: the round table of stage `stage` (virtual `comp_vtable::CREATE_SCHEDULE`(stage,
/// &rounds, &extra, 0), thiscall, ret 0x10) and the Cup_Stage (`exe::crt_operator_new`(0xb2); Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: the league, `clubs`, the list, the rounds, the competition, the round
/// table, the year, `stage`, 1, the extra, 0x14, `flag`, 0, 0, no seeds; the original pushes the rounds, the extra and
/// the year with leftovers above the word, which the constructor does not read). Returns the stage (0 when the new
/// failed) and the round table; the caller stores the stage and frees the table.
pub(crate) unsafe fn league_knockout_stage(l: League, stage: u32, clubs: u32, list: usize, flag: u32) -> (usize, usize) {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(l.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(l.addr(), stage, &mut rounds, &mut extra, 0);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let p = exe::crt_operator_new(0xb2);
    let built = if p != 0 {
        construct(p, l.addr(), clubs, list, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, stage, 1, extra as u16 as u32, 0x14, flag, 0, 0, 0)
    } else {
        0
    };
    (built, table)
}

/// Sorts the averages rows of an Argentine league (`exe::crt_qsort`(rows, `ARG_AVERAGE_COUNT`, `ARG_AVERAGE_ROW_SIZE`,
/// `order`), cdecl).
unsafe fn sort_average_rows(l: League, order: usize) {
    exe::crt_qsort(read_usize(l.addr() + ARG_AVERAGES), this_average_count(l) as u32, ARG_AVERAGE_ROW_SIZE as u32, order);
}

/// A copy of the averages rows of an Argentine league in the by-season order of season 2 (for the comparers: the
/// league's own rows are left as they are).
unsafe fn average_rows_by_season(l: League) -> Vec<u8> {
    let n = this_average_count(l).max(0) as usize;
    let mut rows = vec![0u8; n * ARG_AVERAGE_ROW_SIZE];
    core::ptr::copy_nonoverlapping(read_usize(l.addr() + ARG_AVERAGES) as *const u8, rows.as_mut_ptr(), rows.len());
    write_u8(ARG_AVERAGE_SORT_SEASON, 2);
    exe::crt_qsort(rows.as_mut_ptr() as usize, n as u32, ARG_AVERAGE_ROW_SIZE as u32, ORDER_AVERAGE_ROWS_BY_SEASON);
    rows
}

/// Relegates the two Argentine Second Division clubs with the lowest averages: from the last averages row up (row
/// `ARG_AVERAGE_COUNT` − k − 1, for k below `league::CLUB_COUNT`), a club whose row in this league's table
/// (`exe::league_find_club_row`(this, club, 0)) is not yet status 3 gets status 3, its progress (virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, −2, 0x1e, 0x1e, 0x1e)), `ARG_SECOND_RELEGATED_CLUB` and the announcement
/// (virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`(club, 3, −1, 0, 0)); it stops after two. The club of a row is taken by
/// its id unchecked (`club_at`, as the original).
unsafe fn relegate_by_averages(l: League) {
    let progress: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(l.addr()) + comp_vtable::SET_CLUB_PROGRESS));
    let announce: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(l.addr()) + comp_vtable::ANNOUNCE_TEAM_STATUS));
    let mut relegated = 0;
    let mut k: i16 = 0;
    while k < l.club_count() && relegated < 2 {
        let club = club_at(arg_average_row_club(l, this_average_count(l) - k as i32 - 1) as isize).addr();
        let row = exe::league_find_club_row(l.addr(), club, 0);
        if row != 0 && LeagueTableRow(row).status() != 3 {
            covered!("sbH_1 argentine second division relegated by averages");
            LeagueTableRow(row).set_status(3);
            relegated += 1;
            progress(l.addr(), club, 0xffff_fffe, 0x1e, 0x1e, 0x1e);
            write_usize(l.addr() + ARG_SECOND_RELEGATED_CLUB, club);
            announce(l.addr(), club, 3, 0xffff_ffff, 0, 0);
        }
        k += 1;
    }
}

/// The eight clubs of the Argentine Second Division promotion cup: the leaders of the three zones (the first table
/// row of stages 0, 1 and 2) in slots 0, 2 and 4, then the free slots filled in the order of the averages rows `rows`
/// (`count` rows; the caller sorts them by this season's points) with the clubs not yet in the list, skipping the club
/// at the top of the Second Division's own table; it stops at the first club with no free slot left.
unsafe fn promotion_cup_clubs(l: League, rows: usize, count: i32) -> [usize; 8] {
    let mut clubs = [0usize; 8];
    for zone in 0..3 {
        clubs[2 * zone] = LeagueTableRow(League(read_usize(l.stages() + 4 * zone)).table()).club() as usize;
    }
    let leader = read_i32(LeagueTableRow(l.table()).club() as usize);
    for k in 0..count.max(0) as usize {
        let id = read_i32(rows + k * ARG_AVERAGE_ROW_SIZE);
        if id == leader || clubs.iter().any(|&c| c != 0 && read_i32(c) == id) {
            continue;
        }
        match clubs.iter().position(|&c| c == 0) {
            Some(free) => clubs[free] = club_at(id as isize).addr(),
            None => break,
        }
    }
    clubs
}

/// Rule: at the end of the Argentine Second Division the two clubs with the lowest points average go down, and the
/// promotion cup is drawn: the three zone winners and the five best clubs of the season's points (the table leader,
/// promoted directly, left out).
///
/// `ArgSecond::create_promotion_cup_stage()`: the averages up to date (`update_relegation_averages_virtual`(0));
/// `relegate_by_averages`; `ARG_AVERAGE_SORT_SEASON` = 2 and the rows sorted by season, the eight clubs
/// (`promotion_cup_clubs`), the rows sorted back by the average; the stage (`league_knockout_stage`(3, 8 clubs, flag 1))
/// into stage slot 3. Without a stage: the arg_second.cpp box of line 0x3e3 (the round table is not freed: a leak of
/// the original). With one: Cup::set_up_round (`exe::cup_set_up_round`(stage, 0)), the round table freed and
/// `league::CURRENT_STAGE` = 3.
/// Original: `ArgSecond::create_promotion_cup_stage` (0x408630, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_create_promotion_cup_stage(this: usize) {
    let l = League(this);
    update_relegation_averages_virtual(l, 0);
    relegate_by_averages(l);
    write_u8(ARG_AVERAGE_SORT_SEASON, 2);
    sort_average_rows(l, ORDER_AVERAGE_ROWS_BY_SEASON);
    let clubs = promotion_cup_clubs(l, read_usize(l.addr() + ARG_AVERAGES), this_average_count(l));
    sort_average_rows(l, ORDER_AVERAGE_ROWS);
    let (stage, table) = league_knockout_stage(l, 3, 8, clubs.as_ptr() as usize, 1);
    write_usize(l.stages() + 4 * 3, stage);
    if stage == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x3e3);
    }
    covered!("sbH_1 argentine second division promotion cup");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    l.set_current_stage(3);
}

/// The club of the first team of the cup stage `cup` (its `cup::TEAM_COUNT` entries of 6 bytes) whose
/// `cup_team_entry::STATUS` passes `wanted`.
unsafe fn cup_team_with_status(cup: Cup, wanted: impl Fn(u8) -> bool) -> Option<usize> {
    let teams = cup.team_table();
    (0..cup.team_count().max(0) as usize).map(|k| teams + 6 * k).find(|&e| wanted(read_u8(e + cup_team_entry::STATUS))).map(|e| read_usize(e + cup_team_entry::CLUB))
}

/// The Second Division club of the second promotion play-off: the first club, in the order of the averages rows `rows`
/// (`count` rows, sorted by this season's points), that played the promotion cup `cup` and whose outcome there
/// (`cup_team_entry::STATUS`) is neither 1 nor 2.
unsafe fn second_playoff_club(cup: Cup, rows: usize, count: i32) -> Option<usize> {
    let teams = cup.team_table();
    for k in 0..count.max(0) as usize {
        let id = read_i32(rows + k * ARG_AVERAGE_ROW_SIZE);
        for e in (0..cup.team_count().max(0) as usize).map(|j| teams + 6 * j) {
            let club = read_usize(e + cup_team_entry::CLUB);
            if read_i32(club) == id && !matches!(read_u8(e + cup_team_entry::STATUS), 1 | 2) {
                return Some(club);
            }
        }
    }
    None
}

/// The club of averages row `k` of the Argentine Premier Division (taken by its id unchecked, as the original).
unsafe fn premier_average_club(k: i32) -> usize {
    club_at(arg_average_row_club(League(comp_object_of_key(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION)), k) as isize).addr()
}

/// Rule: the two promotion play-offs of the Argentine Second Division against the Premier Division: the club 18th in
/// the Premier Division averages meets the promotion cup's club with outcome 2; the 17th meets the best club of the
/// season's points that played the cup with any other outcome but 1.
///
/// `ArgSecond::create_promotion_playoffs()`: the pair (`exe::crt_malloc`(8); none: the arg_second.cpp box of line
/// 0x407); the Premier Division (the object of [`KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`]) averages up to date
/// (`update_relegation_averages_virtual`(0)); pair[0] = the club of its averages row 17, pair[1] = the first team of
/// the promotion cup (stage 3) with status 2 (none: left as the malloc gave it, a defect of the original); stage 4
/// (`league_knockout_stage`(4, 2 clubs, flag 1)) into slot 4; without it the box of line 0x42b (the pair and the round
/// table not freed); with it Cup::set_up_round (`exe::cup_set_up_round`(stage, 0)) and the table freed. Then pair[0] =
/// the club of row 16; `ARG_AVERAGE_SORT_SEASON` = 2 and this league's rows sorted by season; pair[1] =
/// `second_playoff_club` (none: as it was); the rows sorted back by the average; stage 5 into slot 5 (without it the box
/// of line 0x46a); Cup::set_up_round, the table and the pair freed and `league::CURRENT_STAGE` = 5.
/// Original: `ArgSecond::create_promotion_playoffs` (0x408910, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_create_promotion_playoffs(this: usize) {
    let l = League(this);
    let cup = Cup(read_usize(l.stages() + 4 * 3));
    let pair = exe::crt_malloc(8);
    if pair == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x407);
    }
    update_relegation_averages_virtual(League(comp_object_of_key(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION)), 0);
    write_usize(pair, premier_average_club(17));
    if let Some(club) = cup_team_with_status(cup, |s| s == 2) {
        write_usize(pair + 4, club);
    }
    let (stage, table) = league_knockout_stage(l, 4, 2, pair, 1);
    write_usize(l.stages() + 4 * 4, stage);
    if stage == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x42b);
    }
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    write_usize(pair, premier_average_club(16));
    write_u8(ARG_AVERAGE_SORT_SEASON, 2);
    sort_average_rows(l, ORDER_AVERAGE_ROWS_BY_SEASON);
    if let Some(club) = second_playoff_club(cup, read_usize(l.addr() + ARG_AVERAGES), this_average_count(l)) {
        write_usize(pair + 4, club);
    }
    sort_average_rows(l, ORDER_AVERAGE_ROWS);
    let (stage, table) = league_knockout_stage(l, 5, 2, pair, 1);
    write_usize(l.stages() + 4 * 5, stage);
    if stage == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x46a);
    }
    covered!("sbH_1 argentine second division promotion play-offs");
    exe::cup_set_up_round(stage, 0);
    exe::crt_free(table);
    exe::crt_free(pair);
    l.set_current_stage(5);
}

/// The clubs of the first `n` entries of a cup stage's team table, sorted (the order the constructor keeps is not
/// relied on).
unsafe fn cup_stage_clubs(stage: usize, n: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..n).map(|k| read_usize(Cup(stage).team_table() + 6 * k + cup_team_entry::CLUB)).collect();
    v.sort();
    v
}

/// Comparer for `arg_second_create_promotion_cup_stage`: only the original (it relegates, announces and builds the
/// stage); afterwards stage 3 must exist and be the current one, with the copy's eight clubs (`promotion_cup_clubs` on
/// the rows sorted by season, worked out after).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_create_promotion_cup_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let l = League(this);
    let s = read_usize(l.stages() + 4 * 3);
    let rows = average_rows_by_season(l);
    let mut plan = promotion_cup_clubs(l, rows.as_ptr() as usize, this_average_count(l)).to_vec();
    plan.sort();
    let got = if s != 0 { cup_stage_clubs(s, 8) } else { Vec::new() };
    SCOREBOARD.record(s != 0 && l.current_stage() as i32 == 3 && got == plan,
        || format!("arg_second_create_promotion_cup_stage({:#x}): stage {:#x}, current {}, clubs {:x?} copy {:x?}", this, s, l.current_stage() as i32, got, plan));
}

/// Comparer for `arg_second_create_promotion_playoffs`: only the original (it builds the stages); afterwards stages 4
/// and 5 must exist, stage 5 be the current one, and each hold the copy's pair (the Premier Division's averages rows 17
/// and 16, the cup's club with outcome 2 and `second_playoff_club`, worked out after).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_create_promotion_playoffs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let l = League(this);
    let cup = Cup(read_usize(l.stages() + 4 * 3));
    let rows = average_rows_by_season(l);
    let winner = cup_team_with_status(cup, |s| s == 2);
    let mut first = vec![premier_average_club(17), winner.unwrap_or(0)];
    let mut second = vec![premier_average_club(16), second_playoff_club(cup, rows.as_ptr() as usize, this_average_count(l)).or(winner).unwrap_or(0)];
    first.sort();
    second.sort();
    let (s4, s5) = (read_usize(l.stages() + 4 * 4), read_usize(l.stages() + 4 * 5));
    let got = |s: usize| if s != 0 { cup_stage_clubs(s, 2) } else { Vec::new() };
    SCOREBOARD.record(s4 != 0 && s5 != 0 && l.current_stage() as i32 == 5 && got(s4) == first && got(s5) == second,
        || format!("arg_second_create_promotion_playoffs({:#x}): stages {:#x} {:#x}, current {}, clubs {:x?} {:x?} copy {:x?} {:x?}", this, s4, s5, l.current_stage() as i32, got(s4), got(s5), first, second));
}
