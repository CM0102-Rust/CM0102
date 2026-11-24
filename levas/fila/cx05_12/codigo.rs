// ---------- cx05_12: the friendly mini cup: the constructor, the round table and the four clubs (mini_cup.cpp) ----------

/// The mini cup's vtable.
const MINI_CUP_VTABLE_12: usize = 0x96d8ac;
/// The source file of the mini cup's error boxes.
const MINI_CUP_FILE: usize = 0xa5ab00;
/// The mini cup's start date (8 bytes: day of the year, year, leap-year dword) [inference: the tournament's start].
const MINI_CUP_START_DATE: usize = 0xb2;
/// The mini cup's tie rules word, copied into each round's `cup_round::TIE_RULES`.
const MINI_CUP_TIE_RULES: usize = 0xba;
/// `MiniCup::set_up_clubs(clubs)` (thiscall, ret 4), called by the constructor through its detour.
const FN_MINICUP_SET_UP_CLUBS: usize = 0x766290;

/// A club's stadium id for the competition's fixed venue: the id (+0) of the stadium record at +0x69, or −1 without
/// one [inference: +0x69 is the club's stadium pointer].
unsafe fn venue_of_club(club: usize) -> i32 {
    let stadium = read_usize(club + 0x69); // the club's stadium record
    if stadium != 0 { read_i32(stadium) } else { -1 }
}

/// Builds a four-club friendly mini cup.
///
/// Cup::Cup (`exe::new_cup`), the start date constructed (`exe::date_date_54a110`); the year (word), the competition
/// record, the vtable, `cup::CURRENT_STAGE` −1, `cup::STAGE_COUNT` 0, `cup::STAGE_INDEX` 0xff, `cup::COMP_TYPE` 1 (cup),
/// `cup::RUN_BY_STAGES` 0, `cup::CURRENT_ROUND` 0, `cup::CLUB_SIDES` = `club_sides`, `cup::STAGE_LABEL` 0xffff; the
/// start date copied from `start` (Date::operator= `exe::date_operator_3d`); the tie rules word, 9 substitutes named and
/// 9 allowed, `cup::NEXT_DRAW_ROUND` 0; the venue `cup::VENUE_STADIUM_ID` from the first club (`venue_of_club`). Then
/// the clubs (`MiniCup::set_up_clubs`; 0: the box at line 0x3c and this), the statistics (`exe::crt_operator_new`
/// (0x5ce), Comp_Stats::Comp_Stats; null: the box at line 0x44 and this), the round table of stage −1 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0)) into `cup::ROUND_TABLE` and the
/// fixture tree (`exe::cup_map_fixture_tree`; negative: the box at line 0x4e). Returns this.
/// Original: `MiniCup::MiniCup` (0x765bf0, mini_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn construct_mini_cup(this: usize, clubs: usize, record: usize, start: usize, year: u32, club_sides: u32, tie_rules: u32) -> usize {
    exe::new_cup(this);
    exe::date_date_54a110(this + MINI_CUP_START_DATE);
    write_u16(this + cup::YEAR, year as u16);
    write_usize(this + league::COMPETITION, record);
    write_usize(this, MINI_CUP_VTABLE_12);
    write_i32(this + cup::CURRENT_STAGE, -1);
    write_u32(this + cup::STAGE_COUNT, 0);
    write_u8(this + cup::STAGE_INDEX, 0xff);
    write_u8(this + cup::COMP_TYPE, 1);
    write_u8(this + cup::RUN_BY_STAGES, 0);
    write_u8(this + cup::CURRENT_ROUND, 0);
    write_u8(this + cup::CLUB_SIDES, club_sides as u8);
    write_u16(this + cup::STAGE_LABEL, 0xffff);
    let mut copy = [0u32; 2];
    exe::date_operator_3d(this + MINI_CUP_START_DATE, copy.as_mut_ptr() as usize, start);
    write_u16(this + MINI_CUP_TIE_RULES, tie_rules as u16);
    write_u8(this + cup::SUBSTITUTES_NAMED, 9); // cx05_12: the friendly bench
    write_u8(this + cup::SUBSTITUTIONS_ALLOWED, 9);
    write_u8(this + cup::NEXT_DRAW_ROUND, 0);
    write_i32(this + cup::VENUE_STADIUM_ID, venue_of_club(read_usize(clubs))); // cx05_12: the host's ground
    let set_up_clubs: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MINICUP_SET_UP_CLUBS);
    if set_up_clubs(this, clubs) == 0 {
        show_error_at(MINI_CUP_FILE, 0x3c);
        return this;
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_usize(this + league::COMP_STATS, stats);
    if stats == 0 {
        show_error_at(MINI_CUP_FILE, 0x44);
        return this;
    }
    let create: unsafe extern "thiscall" fn(usize, i32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    write_usize(this + cup::ROUND_TABLE, create(this, -1, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0));
    covered!("cx05_12 mini cup built");
    if exe::cup_map_fixture_tree(this) < 0 {
        show_error_at(MINI_CUP_FILE, 0x4e);
    }
    this
}

/// Comparer for `construct_mini_cup`: only the original (it allocates the tables, the statistics and the fixture
/// tree); then the plan: this, the vtable, the record, the year, four teams and the three rounds.
pub(crate) unsafe extern "thiscall" fn compare_construct_mini_cup(this: usize, clubs: usize, record: usize, start: usize, year: u32, club_sides: u32, tie_rules: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> usize>(%%I0%%);
    let r = o(this, clubs, record, start, year, club_sides, tie_rules);
    let ok = r == this && read_usize(this) == MINI_CUP_VTABLE_12 && read_usize(this + league::COMPETITION) == record && read_u16(this + cup::YEAR) == year as u16
        && read_i16(this + cup::TEAM_COUNT) == 4 && read_i16(this + cup::ROUND_COUNT) == 3 && read_u16(this + MINI_CUP_TIE_RULES) == tie_rules as u16;
    SCOREBOARD.record(ok, || format!("construct_mini_cup({:#x}): teams {}, rounds {}", this, read_i16(this + cup::TEAM_COUNT), read_i16(this + cup::ROUND_COUNT)));
    r
}

/// One knockout round of the mini cup in the round table (records of `cup_round::SIZE`): the round's date (day and
/// year offset from `cup::YEAR`) at +3/+5 after the draw date at +0/+2, the time +6, the round code and number, the
/// tie rules (the cup's word, `extra_tie_rules` added), +0x13 0x10 or 1, the teams, the ties, the new teams, one leg.
#[allow(clippy::too_many_arguments)]
unsafe fn mini_cup_round(t: usize, draw: (u16, u8), date: (u16, u8), time: u8, code: u16, number: u16, tie_rules: u16, b13: u32, teams: u16, ties: u16, new_teams: u16) {
    write_u16(t, draw.0);
    write_u8(t + 2, draw.1);
    write_u16(t + cup_round::DAY, date.0);
    write_u8(t + cup_round::YEAR_OFFSET, date.1);
    write_u8(t + cup_round::TIME_SLICE, time);
    write_u16(t + cup_round::ROUND_CODE, code);
    write_u16(t + cup_round::ROUND_NUMBER, number);
    write_u16(t + cup_round::DRAW_RULES, 0);
    write_u16(t + cup_round::TIE_RULES, tie_rules);
    write_u16(t + cup_round::TWO_LEG_TIE_RULES, 0);
    write_u32(t + cup_round::VENUE_VALUE, b13);
    write_u8(t + 0x17, 1);
    write_u16(t + 0x18, teams);
    write_u16(t + 0x1a, ties);
    write_u16(t + 0x1c, new_teams);
    write_u16(t + 0x1e, 0);
    write_u8(t + 0x20, 0);
    write_u8(t + 0x21, 1); // one leg
    write_u8(t + 0x22, 0);
    write_u32(t + 0x5c, 0);
    write_u32(t + 0x60, 0);
    write_u32(t + 0x64, 0);
}

/// The round table of the mini cup, for the whole cup only (stage 0xff; any other: 0 without writing).
///
/// `*league` 0 (when given), `*rounds` 3 and `*extra` 0; the table of three rounds (`exe::crt_malloc`(0x138); out of
/// memory: the box at line 0xa1 and 0). From the start date (`MINI_CUP_START_DATE`): the semi-finals (code 0x82, round
/// 0, the cup's tie rules with 0x100, 4 teams, 2 ties, 4 new) on the start day, with the two semi-final days on the
/// start date's weekday (`exe::cup_set_round_match_day`(t, 0, slot, weekday, time, 3, 0x10): slot 0 at time 2, slot 1
/// at time 1, slot 2 none); the final (code 0x96, round 1, 2 teams, 1 tie, none new) and the third place (code 0x8c,
/// round 2, the same) the next day (Date::add_days), both drawn on the start day.
/// Original: `MiniCup::create_round_table` (0x765f70, mini_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn mini_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    let mut date = [read_u32(this + MINI_CUP_START_DATE), read_u32(this + MINI_CUP_START_DATE + 4)];
    let d = date.as_mut_ptr() as usize;
    if !league.is_null() {
        *league = 0;
    }
    *rounds = 3;
    *extra = 0;
    let t = exe::crt_malloc(0x138);
    if t == 0 {
        show_error_at(MINI_CUP_FILE, 0xa1);
        return 0;
    }
    covered!("cx05_12 mini cup rounds");
    let start = (read_u16(d), 0u8);
    let rules = read_u16(this + MINI_CUP_TIE_RULES);
    // the semi-finals: the draw and the date at the start, the fixtures by weekday
    write_u16(t, start.0);
    write_u8(t + 2, 0);
    write_u8(t + cup_round::TIME_SLICE, 1);
    write_u16(t + cup_round::DAY, start.0);
    write_u8(t + cup_round::YEAR_OFFSET, 0);
    write_u32(t + cup_round::VENUE_VALUE, 1);
    exe::cup_set_round_match_day(t, 0, 0, exe::date_get_weekday(d), 2, 3, 0x10);
    exe::cup_set_round_match_day(t, 0, 1, exe::date_get_weekday(d), 1, 3, 0x10);
    exe::cup_set_round_match_day(t, 0, 2, -1, -1, -1, 0);
    mini_cup_round(t, start, start, 1, 0x82, 0, rules | 0x100, 1, 4, 2, 4); // cx05_12: the semi-finals
    // the final and the third place: the next day
    let mut next = [0u32; 2];
    let n = exe::date_add_days(d, next.as_mut_ptr() as usize, 1); // cx05_12: the final the next day
    let mut copy = [0u32; 2];
    exe::date_operator_3d(d, copy.as_mut_ptr() as usize, n);
    let day = read_u16(d);
    let offset = read_u8(d + 2).wrapping_sub(read_u8(this + cup::YEAR));
    let semi = (read_u16(t + cup_round::DAY), read_u8(t + cup_round::YEAR_OFFSET));
    mini_cup_round(t + cup_round::SIZE, semi, (day, offset), 2, 0x96, 1, rules, 0x10, 2, 1, 0);
    mini_cup_round(t + 2 * cup_round::SIZE, semi, (day, offset), 1, 0x8c, 2, rules, 0x10, 2, 1, 0);
    t
}

/// Comparer for `mini_cup_create_round_table`: only the original (it allocates the table; the unwritten bytes are the
/// allocator's), then the plan: 3 rounds, extra 0, the round codes 0x82, 0x96 and 0x8c and the semi-finals on the start
/// day.
pub(crate) unsafe extern "thiscall" fn compare_mini_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    let t = o(this, stage, rounds, extra, league);
    if t != 0 {
        let codes = [0usize, 1, 2].map(|k| read_u16(t + k * cup_round::SIZE + cup_round::ROUND_CODE));
        let ok = *rounds == 3 && *extra == 0 && codes == [0x82, 0x96, 0x8c] && read_u16(t + cup_round::DAY) == read_u16(this + MINI_CUP_START_DATE);
        SCOREBOARD.record(ok, || format!("mini_cup_create_round_table({:#x}): codes {:x?}, rounds {}", this, codes, *rounds));
    }
    t
}

/// The mini cup's four clubs: `cup::TEAM_COUNT` 4 and the team table (`exe::crt_malloc`(0x18): six bytes per team; out
/// of memory: the box at line 0x105 and 0) with each club of `clubs` and its +4 and +5 bytes cleared. Returns 1.
/// Original: `MiniCup::set_up_clubs` (0x766290, mini_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn mini_cup_set_up_clubs(this: usize, clubs: usize) -> u32 {
    write_i16(this + cup::TEAM_COUNT, 4); // cx05_12: the four clubs of a mini cup
    let table = exe::crt_malloc(0x18);
    write_usize(this + cup::TEAM_TABLE, table);
    if table == 0 {
        show_error_at(MINI_CUP_FILE, 0x105);
        return 0;
    }
    let mut k = 0i32;
    while k < read_i16(this + cup::TEAM_COUNT) as i32 {
        let entry = table + k as usize * 6;
        write_usize(entry, read_usize(clubs + k as usize * 4));
        write_u8(entry + 5, 0);
        write_u8(entry + 4, 0);
        k += 1;
    }
    1
}

/// Comparer for `mini_cup_set_up_clubs`: only the original (it allocates the table), then the plan: four teams, the
/// clubs in order with their two bytes cleared.
pub(crate) unsafe extern "thiscall" fn compare_mini_cup_set_up_clubs(this: usize, clubs: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let r = o(this, clubs);
    if r == 1 {
        let table = read_usize(this + cup::TEAM_TABLE);
        let ok = read_i16(this + cup::TEAM_COUNT) == 4 && (0..4usize).all(|k| read_usize(table + k * 6) == read_usize(clubs + k * 4) && read_u16(table + k * 6 + 4) == 0);
        SCOREBOARD.record(ok, || format!("mini_cup_set_up_clubs({:#x}): the team table does not match the clubs", this));
    }
    r
}
