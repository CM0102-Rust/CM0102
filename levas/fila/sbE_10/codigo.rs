// ---------- sbE_10: the Polish First Division table, its second group and its final stage (pol_first.cpp) ----------

/// The seeding slots of the 16 clubs (by last season's place, the promoted clubs after the 12 that stayed): the first
/// eight slots are the first group's table, the other eight the second group's.
pub(crate) const POL_FIRST_SEEDS: [u8; 16] = [1, 4, 5, 8, 9, 12, 13, 16, 2, 3, 6, 7, 10, 11, 14, 15];

/// A Polish First Division club's seed: its last position (`club::LAST_POSITION`) when its last division
/// (`club::LAST_DIVISION`) was this league; a promoted club's position 1 or 2 becomes 15 or 16, 3 becomes 14 and 4
/// becomes 13; any other as it is.
pub(crate) unsafe fn polish_seed(club: Club, this: League) -> u8 {
    let p = read_u8(club.addr() + club::LAST_POSITION);
    if read_usize(club.addr() + club::LAST_DIVISION) == this.competition().addr() {
        return p;
    }
    match p {
        1 | 2 => p + 0xe,
        3 => 0xe,
        4 => 0xd,
        _ => p,
    }
}

/// The plan of the Polish First Division's club list: the clubs of `KEY_CLUB_COMP_POLISH_FIRST_DIVISION` in database
/// order (16-bit index), each in the first empty slot whose seed (`POL_FIRST_SEEDS`) is its own (`polish_seed`), or
/// else in the first empty slot; with no empty slot left, the club is left out.
pub(crate) unsafe fn polish_first_division_plan(this: League) -> [usize; 16] {
    let mut list = [0usize; 16];
    let mut i: i16 = 0;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let d = c.division().addr();
        if d != 0 && read_u32(d) == read_u32(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) {
            let seed = polish_seed(c, this);
            let slot = (0..16).find(|&k| POL_FIRST_SEEDS[k] == seed && list[k] == 0).or_else(|| (0..16).find(|&k| list[k] == 0));
            if let Some(k) = slot {
                list[k] = c.addr(); // sbE_10: the club's seeding slot
            }
        }
        i = i.wrapping_add(1);
    }
    list
}

/// Rule: the Polish First Division's 16 clubs are seeded by last season's place into two groups of eight (slots 1, 4, 5,
/// 8, 9, 12, 13 and 16 in the first; the promoted clubs take places 13 to 16); the first group is this league's table;
/// TV pays 500,000 to each club.
///
/// `PolFirst::create_table()`: `league::CLUB_COUNT` = 8; the table (`exe::crt_malloc` of 0x1d8; none: the pol_first.cpp
/// (0xa78598) box at line 0x13b and 0); the old club list (`POL_FIRST_CLUB_LIST`) freed and a new one of 16 dwords
/// (`exe::crt_calloc`(16, 4); none: the box of line 0x146 and 0) filled as `polish_first_division_plan`; rows 0 to the
/// club count − 1 with the list's first clubs (League::init_table_row, thiscall(row, club, 0, 0), ret 0x10); the TV
/// money (League::pay_tv_rights, 0x7a120 to every club); 1. No prize fund.
/// Original: `PolFirst::create_table` (0x7c9450, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_create_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(8);
    let t = exe::crt_malloc(0x1d8);
    l.set_table(t);
    if t == 0 {
        show_error_at(0xa78598, 0x13b);
        return 0;
    }
    let old = read_usize(this + POL_FIRST_CLUB_LIST);
    if old != 0 {
        exe::crt_free(old);
    }
    let list = exe::crt_calloc(0x10, 4);
    write_usize(this + POL_FIRST_CLUB_LIST, list);
    if list == 0 {
        show_error_at(0xa78598, 0x146);
        return 0;
    }
    covered!("sbE_10 polish first division table");
    for (k, &c) in polish_first_division_plan(l).iter().enumerate() {
        write_usize(list + 4 * k, c);
    }
    let mut k: i16 = 0;
    while k < l.club_count() {
        exe::league_init_table_row(this, k as u16 as u32, read_usize(list + 4 * k as usize), 0, 0);
        k += 1;
    }
    eng_prm_pay_tv_rights(this, 0x7a120, 0); // sbE_10: the TV money of each club
    1
}

/// Comparer for `pol_first_create_table`: only the original runs (it allocates and pays the clubs), against the plan:
/// the club list and the first group's rows.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let l = League(this);
    let plan = polish_first_division_plan(l);
    let r = o(this);
    let list = read_usize(this + POL_FIRST_CLUB_LIST);
    let ok = r == 1 && list != 0 && (0..16).all(|k| read_usize(list + 4 * k) == plan[k])
        && (0..8).all(|k| read_usize(l.table() + k * league_table_row::SIZE) == plan[k]);
    SCOREBOARD.record(ok, || format!("pol_first_create_table({:#x}) original={} plan={:x?}", this, r, plan));
    r
}

/// Rule: the Polish First Division's second group of eight (the last eight of the seeded list) is stage 0, played home
/// and away with its own round table, and becomes the current stage.
///
/// `PolFirst::create_second_group_stage()`: the round table (virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra,
/// 0)); the list (`exe::crt_malloc` of 4 × `league::CLUB_COUNT`; none: the pol_first.cpp box at line 0x226 and nothing
/// else) with the clubs of `POL_FIRST_CLUB_LIST` from the ninth on; a League_Stage (`exe::crt_operator_new`(0xee);
/// League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this league, the club count, the list, 2 legs, the
/// competition, the round table and its rounds, the win, draw and category bytes, the tie-breaks, the places (the four
/// counts from `league::PROMOTED_COUNT`), the year, stage 0, the extra as its label, the importance, 1, 0, the rule bits
/// `league::LEAGUE_FLAGS`, −1, 0 and 2) into stage slot 0; none: the box of line 0x238; otherwise the list and the round
/// table freed and `league::CURRENT_STAGE` = 0. On the error paths the round table (and the list) are not freed (leaks
/// of the original).
/// Original: `PolFirst::create_second_group_stage` (0x7c9900, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_create_second_group_stage(this: usize) {
    let l = League(this);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 0, &mut rounds, &mut extra, 0);
    let list = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(4));
    if list == 0 {
        return show_error_at(0xa78598, 0x226);
    }
    let mut k = 0i32;
    while k < l.club_count() as i32 {
        write_usize(list + 4 * k as usize, read_usize(read_usize(this + POL_FIRST_CLUB_LIST) + 0x20 + 4 * k as usize)); // sbE_10: the second eight
        k += 1;
    }
    covered!("sbE_10 polish second group stage");
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this, l.club_count() as u16 as u32, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, 0, extra as u16 as u32, l.importance() as u32, 1, 0,
            l.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_usize(l.stages(), s);
    if s == 0 {
        return show_error_at(0xa78598, 0x238);
    }
    exe::crt_free(list);
    exe::crt_free(table);
    l.set_current_stage(0);
}

/// Comparer for `pol_first_create_second_group_stage`: only the original (it builds the stage); afterwards stage slot 0
/// must hold the last eight clubs of the list and be the current stage.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_create_second_group_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let l = League(this);
    let s = read_usize(l.stages());
    let list = read_usize(this + POL_FIRST_CLUB_LIST);
    let ok = s != 0 && League(s).club_count() == l.club_count() && l.current_stage() == 0
        && (0..l.club_count().max(0) as usize).all(|k| read_usize(League(s).table() + k * league_table_row::SIZE) == read_usize(list + 0x20 + 4 * k));
    SCOREBOARD.record(ok, || format!("pol_first_create_second_group_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}

/// The row block of a Polish group's results carried into the final stage: the overall, home and away won, drawn and
/// lost counts and goals (the played counts start again; the points are halved, `carry_halved_row`).
const POLISH_CARRIED_BYTES: [usize; 9] = [league_table_row::WON, league_table_row::HOME_WON, league_table_row::AWAY_WON,
    league_table_row::DRAWN, league_table_row::HOME_DRAWN, league_table_row::AWAY_DRAWN, league_table_row::LOST, league_table_row::HOME_LOST, league_table_row::AWAY_LOST];
const POLISH_CARRIED_WORDS: [usize; 6] = [league_table_row::GOALS_FOR, league_table_row::HOME_GOALS_FOR, league_table_row::AWAY_GOALS_FOR,
    league_table_row::GOALS_AGAINST, league_table_row::HOME_GOALS_AGAINST, league_table_row::AWAY_GOALS_AGAINST];

/// Carries a group row (`from`) into a final stage row (`to`): the points and the home points halved, rounding up
/// ((x + 1) / 2, toward zero), the away points their difference (16 bits); the won, drawn and lost counts and the goals
/// copied.
unsafe fn carry_halved_row(to: usize, from: usize) {
    let half = |x: i16| ((x as i32 + 1) / 2) as i16;
    let points = half(read_i16(from + league_table_row::POINTS));
    write_i16(to + league_table_row::POINTS, points); // sbE_10: half the points
    let home = half(read_i16(from + league_table_row::HOME_POINTS));
    write_i16(to + league_table_row::HOME_POINTS, home);
    write_i16(to + league_table_row::AWAY_POINTS, read_i16(to + league_table_row::POINTS).wrapping_sub(home));
    for &o in &POLISH_CARRIED_BYTES {
        write_u8(to + o, read_u8(from + o));
    }
    for &o in &POLISH_CARRIED_WORDS {
        write_u16(to + o, read_u16(from + o));
    }
}

/// The League_Stage of an eight-club group of the Polish final stage, built only for what its constructor does and
/// destroyed at once (virtual 0 with 1): the round table of stage 1 (virtual `comp_vtable::CREATE_SCHEDULE`(1, &rounds,
/// &extra, 0)), 8 clubs from `clubs`, 2 legs, the tie-breaks, the places (from `league::PROMOTED_COUNT`), stage 1, the
/// extra as its label, importance 0x14, 1, 0, rule bits 3, −1, 0 and 2. Returns the round table.
unsafe fn polish_final_group_pass(l: League, clubs: usize) -> usize {
    let this = l.addr();
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = schedule(this, 1, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xee);
    if p != 0 {
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        let s = ctor(p, this, 8, clubs, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32,
            l.match_category() as u32, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, l.year() as u16 as u32, 1, extra as u16 as u32, 0x14, 1, 0, 3, -1, 0, 2);
        if s != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(s)));
            destroy(s, 1);
        }
    }
    table
}

/// Rule: the Polish final stage: the top four of each group go to the championship group and the other four of each to
/// the relegation group, all in one table of 16 that carries half their points and their results; the two groups of
/// eight also get their own rounds.
///
/// `PolFirst::create_final_stage()`: the list (`exe::crt_malloc` of 0x40; none: the pol_first.cpp box at line 0x277 and
/// nothing else) with rows 0 to 3 of this league (the first group) and of stage 0 (the second), alternating, then rows 4
/// to 7 the same way; the clubs of rows 0 to 3 get the record of `KEY_CLUB_COMP_IRISH_GROUP_A` (0x9cf814, so named in
/// the keys) and those of rows 4 to 7 that of `KEY_CLUB_COMP_IRISH_GROUP_B` (0x9cf818) in `club::RESERVE_DIVISION`. A
/// League_Stage of 16 (`exe::crt_operator_new`(0xee); 0x68b100: this league, 16 clubs, the list, 2 legs, the competition,
/// no round table and 0 rounds, the win, draw and category bytes, the tie-breaks, the places (0, 0, 2, 2), the year,
/// stage 1, label 0, importance 0x14, 1, 0, rule bits 3, 0xe, the prize fund of the share 0x11170 (League::calc_prize_fund,
/// computed only when the new succeeded) and prize mode 1) into stage slot 1 (unchecked); each of its rows gets the
/// club's row from this league (League::find_club_row(club, 0)) or else from stage 0 carried in (`carry_halved_row`);
/// its `league::CUSTOM_COMPARE` = `pol_first_compare_championship_group_first` (0x7ca450) and its order sorted
/// (League::sort_table(0)); `league::CURRENT_STAGE` = 1; then `polish_final_group_pass` for the first eight (its round
/// table freed) and for the last eight (its round table not freed: a leak of the original).
/// Defects of the original: the group mark overwrites the clubs' reserve division; the list is not freed.
/// Original: `PolFirst::create_final_stage` (0x7c9b60, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_create_final_stage(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(0x40);
    if list == 0 {
        return show_error_at(0xa78598, 0x277);
    }
    let second = League(read_usize(l.stages()));
    let mut n = 0usize;
    for (rows, key) in [(0..4usize, KEY_CLUB_COMP_IRISH_GROUP_A), (4..8, KEY_CLUB_COMP_IRISH_GROUP_B)] {
        for k in rows {
            for group in [l, second] {
                let club = read_usize(group.table() + k * league_table_row::SIZE);
                write_usize(list + 4 * n, club);
                n += 1;
            }
            for group in [l, second] {
                write_usize(read_usize(group.table() + k * league_table_row::SIZE) + club::RESERVE_DIVISION, comp_record_of(key)); // sbE_10: the final stage group
            }
        }
    }
    covered!("sbE_10 polish final stage");
    let places: [u8; 4] = [0, 0, 2, 2];
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        let fund = exe::league_calc_prize_fund(this, 0x11170);
        let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        ctor(p, this, 0x10, list, 2, l.competition().addr(), 0, 0, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
            this + league::TIEBREAKS, places.as_ptr() as usize, l.year() as u16 as u32, 1, 0, 0x14, 1, 0, 3, 0xe, fund as u32, 1)
    } else {
        0
    };
    write_usize(l.stages() + 4, s);
    let fin = League(s);
    let mut k = 0i32;
    while k < fin.club_count() as i32 {
        let to = fin.table() + k as usize * league_table_row::SIZE;
        let club = read_usize(to);
        let mut from = exe::league_find_club_row(this, club, 0);
        if from == 0 {
            from = exe::league_find_club_row(second, club, 0);
        }
        if from != 0 {
            carry_halved_row(to, from);
        }
        k += 1;
    }
    write_usize(s + league::CUSTOM_COMPARE, 0x7ca450);
    exe::league_sort_table(s, 0);
    l.set_current_stage(1);
    exe::crt_free(polish_final_group_pass(l, list));
    polish_final_group_pass(l, list + 0x20);
}

/// Comparer for `pol_first_create_final_stage`: only the original (it builds and destroys stages); afterwards stage slot
/// 1 must hold the 16 clubs with the championship-group order, and 1 be the current stage.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_create_final_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let l = League(this);
    let s = read_usize(l.stages() + 4);
    let ok = s != 0 && League(s).club_count() == 0x10 && read_usize(s + league::CUSTOM_COMPARE) == 0x7ca450 && l.current_stage() == 1;
    SCOREBOARD.record(ok, || format!("pol_first_create_final_stage({:#x}): stage {:#x}, current {}", this, s, l.current_stage() as i32));
}
