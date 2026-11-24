// ---------- cx05_13: the friendly mini league: the constructor, the schedule and the table (mini_league.cpp) ----------

/// The mini league's vtable.
const MINI_LEAGUE_VTABLE_13: usize = 0x96d94c;
/// The source file of the mini league's error boxes.
const MINI_LEAGUE_FILE: usize = 0xa5ab3c;
/// The mini league's start date (8 bytes: day of the year, year, leap-year dword) [inference: the tournament's start].
const MINI_LEAGUE_START_DATE: usize = 0xee;
/// `MiniLeague::alloc_table(clubs)` (thiscall, ret 4), called by the constructor through its detour.
const FN_MINILEAGUE_ALLOC_TABLE: usize = 0x766bf0;
/// `League::set_predicted_positions` (thiscall, ret 0).
const FN_LEAGUE_SET_PREDICTED_POSITIONS_13: usize = 0x68a850;

/// Builds a friendly mini league of `club_count` clubs.
///
/// League::League (`exe::new_league`), the start date constructed (`exe::date_date_54a110`); `league::CLUB_COUNT`, the
/// draw points, the vtable, the competition record, the win points, `league::MATCH_CATEGORY` 1, the four tie-break
/// codes from `tiebreaks`, the four promotion bytes from `promotion` (cleared again below), the year, `club_sides`,
/// `league::CURRENT_STAGE`, `league::LOWER_DIVISION_ID` and `league::UPPER_DIVISION_ID` −1, `league::STAGE_INDEX`
/// 0xff, `league::STAGE_COUNT` 0, `league::COMP_TYPE` 2 (league), `league::STAGE_ID` 0, `league::MATCH_RULES`; the
/// start date copied from `start` (Date::operator=); `league::LEAGUE_FLAGS` 0; the venue from the first club
/// (`venue_of_club` of cx05_12); 9 substitutes named and 9 allowed, `league::LEGS` 1, `league::ROUND_DRAW_RULES` 0 and
/// the promotion bytes 0; the round table of stage −1 (virtual `comp_vtable::CREATE_SCHEDULE`(−1,
/// &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0)) into `league::ROUND_DATES`. Then the table (`MiniLeague::
/// alloc_table`; 0: the box at line 0x56), the fixtures (`exe::league_create_fixtures`; 0: the box at line 0x5c), the
/// sorted table (`exe::league_sort_table`(this, 0); 0: the box at line 0x63), the statistics (Comp_Stats; null: the box
/// at line 0x6a) and the predicted positions (League::set_predicted_positions). Returns this.
/// Original: `MiniLeague::MiniLeague` (0x766500, mini_league.cpp, thiscall, ret 0x2c).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn construct_mini_league(this: usize, club_count: u32, clubs: usize, record: usize, start: usize, win: u32, draw: u32, tiebreaks: usize,
        promotion: usize, year: u32, club_sides: u32, match_rules: u32) -> usize {
    let league = League(this);
    exe::new_league(league);
    exe::date_date_54a110(this + MINI_LEAGUE_START_DATE);
    write_u16(this + league::CLUB_COUNT, club_count as u16);
    write_u8(this + league::DRAW_POINTS, draw as u8);
    write_usize(this, MINI_LEAGUE_VTABLE_13);
    write_usize(this + league::COMPETITION, record);
    write_u8(this + league::WIN_POINTS, win as u8);
    write_u8(this + league::MATCH_CATEGORY, 1);
    for k in 0..4usize {
        write_u8(this + league::TIEBREAKS + k, read_u8(tiebreaks + k));
    }
    for k in 0..4usize {
        write_u8(this + league::PROMOTED_COUNT + k, read_u8(promotion + k));
    }
    write_u16(this + league::YEAR, year as u16);
    write_u8(this + league::CLUB_SIDES, club_sides as u8);
    write_i32(this + league::CURRENT_STAGE, -1);
    write_i32(this + league::LOWER_DIVISION_ID, -1);
    write_i32(this + league::UPPER_DIVISION_ID, -1);
    write_u8(this + league::STAGE_INDEX, 0xff);
    write_u32(this + league::STAGE_COUNT, 0);
    write_u8(this + league::COMP_TYPE, 2);
    write_u16(this + league::STAGE_ID, 0);
    write_u16(this + league::MATCH_RULES, match_rules as u16);
    let mut copy = [0u32; 2];
    exe::date_operator_3d(this + MINI_LEAGUE_START_DATE, copy.as_mut_ptr() as usize, start);
    write_u16(this + league::LEAGUE_FLAGS, 0);
    write_i32(this + league::FIXED_STADIUM_ID, venue_of_club(read_usize(clubs)));
    mini_league_rules(this);
    let alloc_table: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MINILEAGUE_ALLOC_TABLE);
    if alloc_table(this, clubs) == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0x56);
        return this;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0x5c);
        return this;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0x63);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_usize(this + league::COMP_STATS, stats);
    if stats == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0x6a);
        return this;
    }
    covered!("cx05_13 mini league built");
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS_13);
    predicted(this);
    this
}

/// The rule fields a mini league writes before its round table (the constructor and `MiniLeague::set_up_rules`): 9
/// substitutes named and 9 allowed, one leg, no round draw rules, the four promotion bytes cleared, and the round table
/// of stage −1 (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0)) into
/// `league::ROUND_DATES` (the old one is not freed).
pub(crate) unsafe fn mini_league_rules(this: usize) {
    write_u8(this + league::SUBSTITUTES_NAMED, 9); // cx05_13: the friendly bench
    write_u8(this + league::SUBSTITUTIONS_ALLOWED, 9);
    write_u16(this + league::LEGS, 1); // cx05_13: each pair meets once
    write_u16(this + league::ROUND_DRAW_RULES, 0);
    for k in 0..4usize {
        write_u8(this + league::PROMOTED_COUNT + k, 0);
    }
    let create: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    write_usize(this + league::ROUND_DATES, create(this, -1, this + league::ROUND_COUNT, this + league::STAGE_ID, 0));
}

/// Comparer for `construct_mini_league`: only the original (it allocates the table, the fixtures and the statistics);
/// then the plan: this, the vtable, the record, the year, the club count, one leg and three rounds.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_construct_mini_league(this: usize, club_count: u32, clubs: usize, record: usize, start: usize, win: u32, draw: u32, tiebreaks: usize,
        promotion: usize, year: u32, club_sides: u32, match_rules: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, usize, usize, u32, u32, usize, usize, u32, u32, u32) -> usize>(%%I0%%);
    let r = o(this, club_count, clubs, record, start, win, draw, tiebreaks, promotion, year, club_sides, match_rules);
    let ok = r == this && read_usize(this) == MINI_LEAGUE_VTABLE_13 && read_usize(this + league::COMPETITION) == record && read_u16(this + league::YEAR) == year as u16
        && read_u16(this + league::CLUB_COUNT) == club_count as u16 && read_u16(this + league::LEGS) == 1 && read_i16(this + league::ROUND_COUNT) == 3;
    SCOREBOARD.record(ok, || format!("construct_mini_league({:#x}): clubs {}, rounds {}", this, read_u16(this + league::CLUB_COUNT), read_i16(this + league::ROUND_COUNT)));
    r
}

/// One round of the mini league's round dates (records of 0x41 bytes): the day, the year offset from `league::YEAR`,
/// the byte +4 = 1, the dword +0x3d = 1 and its three match slots: on the date's weekday at time 2 and at time 1
/// (`exe::league_round_set_match_slot`(t, round, slot, weekday, time, 3, 0x10), cdecl), the third empty.
unsafe fn mini_league_round(this: usize, t: usize, round: i32, date: usize) {
    let r = t + round as usize * 0x41;
    write_u16(r, read_u16(date));
    write_u16(r + 2, read_u16(date + 2).wrapping_sub(read_u16(this + league::YEAR)));
    write_u8(r + 4, 1);
    write_u32(r + 0x3d, 1);
    exe::league_round_set_match_slot(t, round, 0, exe::date_get_weekday(date), 2, 3, 0x10);
    exe::league_round_set_match_slot(t, round, 1, exe::date_get_weekday(date), 1, 3, 0x10);
    exe::league_round_set_match_slot(t, round, 2, -1, -1, -1, 0);
}

/// The round dates of the mini league, for the whole league only (stage 0xff; any other: 0 without writing).
///
/// `*league` 1 (when given), `*rounds` 3 and `*extra` 0; the table of three rounds of 0x41 bytes
/// (`exe::crt_malloc`(0xc3); out of memory: the box at line 0xb6 and 0): round 0 on the start date
/// (`MINI_LEAGUE_START_DATE`), rounds 1 and 2 on the following days (Date::add_days by 1 each time), each with two
/// matches on its weekday (`mini_league_round`).
/// Original: `MiniLeague::create_schedule` (0x766970, mini_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn mini_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 3;
    *extra = 0;
    let t = exe::crt_malloc(0xc3);
    if t == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0xb6);
        return 0;
    }
    covered!("cx05_13 mini league schedule");
    let mut date = [read_u32(this + MINI_LEAGUE_START_DATE), read_u32(this + MINI_LEAGUE_START_DATE + 4)];
    let d = date.as_mut_ptr() as usize;
    for round in 0..3i32 {
        if round > 0 {
            let mut next = [0u32; 2];
            let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 1); // cx05_13: one round a day
            let mut copy = [0u32; 2];
            exe::date_operator_3d(d, copy.as_mut_ptr() as usize, n);
        }
        mini_league_round(this, t, round, d);
    }
    t
}

/// Comparer for `mini_league_create_schedule`: only the original (it allocates the table; the unwritten bytes are the
/// allocator's), then the plan: 3 rounds, extra 0 and the rounds on the start date and the two following days.
pub(crate) unsafe extern "thiscall" fn compare_mini_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    let t = o(this, stage, rounds, extra, league);
    if t != 0 {
        let days = [0usize, 1, 2].map(|k| read_u16(t + k * 0x41));
        let first = read_u16(this + MINI_LEAGUE_START_DATE);
        let ok = *rounds == 3 && *extra == 0 && days[0] == first && (days[1] == first.wrapping_add(1) || days[1] == 0) && read_u8(t + 4) == 1;
        SCOREBOARD.record(ok, || format!("mini_league_create_schedule({:#x}): days {:?} from {}", this, days, first));
    }
    t
}

/// The mini league table: `league::CLUB_COUNT` rows of `league_table_row::SIZE` (`exe::crt_malloc`; out of memory: the
/// box at line 0xee and 0), row i with club i of `clubs` (or no club when `clubs` is null) (`exe::league_init_table_row`
/// (this, i, club, 0, 0)). Returns 1.
/// Original: `MiniLeague::alloc_table` (0x766bf0, mini_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_league_alloc_table(this: usize, clubs: usize) -> u32 {
    let league = League(this);
    let n = read_i16(this + league::CLUB_COUNT) as i32;
    let t = exe::crt_malloc((n * league_table_row::SIZE as i32) as u32);
    league.set_table(t);
    if t == 0 {
        show_error_at(MINI_LEAGUE_FILE, 0xee);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + league::CLUB_COUNT) {
        let club = if clubs != 0 { read_usize(clubs + i as isize as usize * 4) } else { 0 }; // cx05_13: the club of the row
        // simplification: the row goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte
        exe::league_init_table_row(league, i as u8 as u32, club, 0, 0);
        i = i.wrapping_add(1);
    }
    1
}

/// Comparer for `mini_league_alloc_table`: only the original (each row allocates its history), then the plan: row i
/// with club i of the list.
pub(crate) unsafe extern "thiscall" fn compare_mini_league_alloc_table(this: usize, clubs: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let r = o(this, clubs);
    let t = League(this).table();
    if r == 1 && clubs != 0 && t != 0 {
        let n = read_i16(this + league::CLUB_COUNT).clamp(0, 64) as usize;
        let ok = (0..n).all(|i| read_usize(t + i * league_table_row::SIZE) == read_usize(clubs + i * 4) && read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION) == i as u8);
        SCOREBOARD.record(ok, || format!("mini_league_alloc_table({:#x}): the rows do not match the clubs", this));
    }
    r
}
