// ---------- %%LEVA%%a leva: league stages: the League_Stage constructor and destructor (league_stage.cpp) ----------

/// The clubs of the `league::CLUB_COUNT` rows of the table (`league::TABLE`, `league_table_row::SIZE` bytes each),
/// sorted (to compare them as a set).
unsafe fn league_stage_table_clubs(this: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..read_i16(this + league::CLUB_COUNT).max(0) as usize).map(|i| read_usize(read_usize(this + league::TABLE) + league_table_row::SIZE * i)).collect();
    v.sort_unstable();
    v
}

const LEAGUE_STAGE_VTABLE: usize = 0x96d0f0;

/// A round-robin stage inside a competition (the state league and J1 halves, the third division groups, the MLS
/// conferences, the cup and qualifier groups...): `League_Stage::League_Stage(parent, club_count, clubs, legs, record,
/// round_dates, round_count, win_points, draw_points, category, tiebreaks, places, year, stage_index, stage_id,
/// importance, club_sides, match_rules, league_flags, games_per_club, prize_fund, prize_mode)`. League::League
/// (`exe::new_league`); the vtable 0x96d0f0; the 22 arguments into the fields, in the original's order:
/// `league::CLUB_COUNT` (word), `league::LEGS` (word), `league::COMPETITION`, `league::ROUND_COUNT` (word),
/// `league::WIN_POINTS` / `league::DRAW_POINTS` / `league::MATCH_CATEGORY` (bytes), the 4 tie-breaks copied into
/// `league::TIEBREAKS`, the 4 places copied into `league::PROMOTED_COUNT`.. `league::RELEGATED_COUNT`, `league::YEAR`
/// (word), `league::STAGE_INDEX` (byte), `league::STAGE_ID` (word), `league::IMPORTANCE` (byte), `league::CLUB_SIDES`
/// (byte), `league::MATCH_RULES`, `league::LEAGUE_FLAGS`, `league::GAMES_PER_CLUB` (words), `league::PRIZE_FUND`
/// (dword), `league::PRIZE_MODE` (byte); `league::CURRENT_STAGE` = `league::STAGE_COUNT` =
/// `league::LOWER_DIVISION_ID` = `league::UPPER_DIVISION_ID` = −1, `league::COMP_TYPE` = 2,
/// `league::SUBSTITUTES_NAMED` and `league::SUBSTITUTIONS_ALLOWED` = the parent competition's. With the round dates
/// given: a copy of them (`exe::crt_malloc` of `league_round_date::SIZE` × rounds; out of memory, the error box of
/// line 0x60 and the end) in `league::ROUND_DATES`; otherwise `league::ROUND_DATES` = 0. The table:
/// `exe::crt_malloc` of `league_table_row::SIZE` × clubs in `league::TABLE` (out of memory, the error box 0x6f and the
/// end); each row i (`exe::league_init_table_row`(this, i, club or 0, 0, 0), thiscall, ret 0x10; the original pushes a
/// dword with i in the low byte and stack garbage above it; the routine reads only the byte). With rounds: the
/// fixtures (`exe::league_create_fixtures`; 0: the error box 0x7f and the end); without them: a single round
/// (`exe::crt_malloc` of 0x41 in `league::ROUND_DATES`; out of memory, the error box 0x8a and the end) with today's
/// date ([`G_CURRENT_DATE`] in the day word, [`G_CURRENT_DATE_YEAR`] − the stage year in the year-offset word, 0 in
/// the time-slice byte) and `league::ROUND_COUNT` = 1. Finally the sort (`exe::league_sort_table`(this, 0)). Returns
/// this. (The original's SEH frame has no destructor on the normal path.)
/// Original: `League_Stage::League_Stage` (0x68b100, league_stage.cpp, thiscall, ret 0x58).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn construct_league_stage(this: usize, parent: usize, club_count: u32, clubs: usize, legs: u32, record: u32, round_dates: usize,
                                                 round_count: u32, win_points: u32, draw_points: u32, category: u32, tiebreaks: usize, places: usize, year: u32, stage_index: u32,
                                                 stage_id: u32, importance: u32, club_sides: u32, match_rules: u32, league_flags: u32, games_per_club: u32, prize_fund: u32, prize_mode: u32) -> usize {
    exe::new_league(this);
    covered!("cqv_4 league stage");
    let w = |o: usize, v: u32| write_u16(this + o, v as u16);
    let b = |o: usize, v: u32| write_u8(this + o, v as u8);
    let x = |o: usize, v: u32| write_u32(this + o, v);
    w(league::CLUB_COUNT, club_count);
    w(league::LEGS, legs);
    w(league::ROUND_COUNT, round_count);
    b(league::WIN_POINTS, win_points); // cqv4: the stage's points for a win
    x(league::COMPETITION, record);
    b(league::MATCH_CATEGORY, category);
    b(league::IMPORTANCE, importance);
    x(0, LEAGUE_STAGE_VTABLE as u32);
    b(league::DRAW_POINTS, draw_points); // cqv4: the stage's points for a draw
    for i in 0..4 {
        b(league::TIEBREAKS + i, read_u8(tiebreaks + i) as u32);
    }
    for i in 0..4 {
        b(league::PROMOTED_COUNT + i, read_u8(places + i) as u32);
    }
    w(league::YEAR, year);
    x(league::CURRENT_STAGE, 0xffff_ffff);
    x(league::STAGE_COUNT, 0xffff_ffff);
    x(league::LOWER_DIVISION_ID, 0xffff_ffff);
    x(league::UPPER_DIVISION_ID, 0xffff_ffff);
    b(league::STAGE_INDEX, stage_index);
    w(league::STAGE_ID, stage_id);
    b(league::CLUB_SIDES, club_sides);
    w(league::GAMES_PER_CLUB, games_per_club);
    b(league::COMP_TYPE, 2);
    w(league::MATCH_RULES, match_rules);
    w(league::LEAGUE_FLAGS, league_flags);
    b(league::SUBSTITUTES_NAMED, read_u8(parent + league::SUBSTITUTES_NAMED) as u32);
    b(league::SUBSTITUTIONS_ALLOWED, read_u8(parent + league::SUBSTITUTIONS_ALLOWED) as u32);
    x(league::PRIZE_FUND, prize_fund);
    b(league::PRIZE_MODE, prize_mode);
    if round_dates != 0 {
        let size = (read_i16(this + league::ROUND_COUNT) as i32 * league_round_date::SIZE as i32) as u32;
        let copy = exe::crt_malloc(size);
        x(league::ROUND_DATES, copy as u32);
        if copy == 0 {
            show_error_at(0x9f1630, 0x60);
            return this;
        }
        core::ptr::copy_nonoverlapping(round_dates as *const u8, copy as *mut u8, size as usize);
    } else {
        x(league::ROUND_DATES, 0);
    }
    let table = exe::crt_malloc((read_i16(this + league::CLUB_COUNT) as i32 * league_table_row::SIZE as i32) as u32);
    x(league::TABLE, table as u32);
    if table == 0 {
        show_error_at(0x9f1630, 0x6f);
        return this;
    }
    let total = read_i16(this + league::CLUB_COUNT) as i32;
    let mut i: i8 = 0;
    while (i as i32) < total {
        let c = if clubs != 0 { read_usize(clubs + 4 * i as usize) } else { 0 };
        exe::league_init_table_row(this, i as u8 as u32, c, 0, 0);
        i = i.wrapping_add(1);
    }
    if read_usize(this + league::ROUND_DATES) != 0 {
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9f1630, 0x7f);
            return this;
        }
    } else {
        let r = exe::crt_malloc(0x41);
        x(league::ROUND_DATES, r as u32);
        if r == 0 {
            show_error_at(0x9f1630, 0x8a);
            return this;
        }
        covered!("cqv_4 single round stage");
        write_u16(r + league_round_date::DAY, read_u16(G_CURRENT_DATE));
        write_i16(r + league_round_date::YEAR_OFFSET, read_i16(G_CURRENT_DATE_YEAR).wrapping_sub(read_i16(this + league::YEAR)));
        write_u8(r + league_round_date::TIME_SLICE, 0);
        w(league::ROUND_COUNT, 1);
    }
    exe::league_sort_table(this, 0usize);
    this
}

/// The stage comparer: runs only the original (it allocates the table and the rounds and draws the fixtures),
/// checking what it writes and no routine inside rewrites: the return value, the vtable, the record, the clubs, the
/// legs, the year, the stage index, the importance, the points, the category, the copied tie-breaks and places,
/// `league::COMP_TYPE` = 2, the parent's substitutes, the prize fund and mode, the allocated table and rounds and the
/// table's clubs (as a set: the sort reorders them) equal to the given list.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn compare_construct_league_stage(this: usize, parent: usize, club_count: u32, clubs: usize, legs: u32, record: u32, round_dates: usize,
                                                          round_count: u32, win_points: u32, draw_points: u32, category: u32, tiebreaks: usize, places: usize, year: u32, stage_index: u32,
                                                          stage_id: u32, importance: u32, club_sides: u32, match_rules: u32, league_flags: u32, games_per_club: u32, prize_fund: u32, prize_mode: u32) -> usize {
    let mut list: Vec<usize> = if clubs != 0 { (0..(club_count as u16 as i16).max(0) as usize).map(|i| read_usize(clubs + 4 * i)).collect() } else { Vec::new() };
    list.sort_unstable();
    let tiebreak_codes: [u8; 4] = core::array::from_fn(|i| read_u8(tiebreaks + i));
    let place_counts: [u8; 4] = core::array::from_fn(|i| read_u8(places + i));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(%%I0%%)(
        this, parent, club_count, clubs, legs, record, round_dates, round_count, win_points, draw_points, category, tiebreaks, places, year, stage_index, stage_id, importance, club_sides, match_rules, league_flags, games_per_club, prize_fund, prize_mode);
    let mut ok = r == this && read_usize(this) == LEAGUE_STAGE_VTABLE && read_usize(this + league::COMPETITION) == record as usize && read_i16(this + league::CLUB_COUNT) == club_count as i16
        && read_i16(this + league::LEGS) == legs as i16 && read_i16(this + league::YEAR) == year as i16 && read_u8(this + league::STAGE_INDEX) == stage_index as u8
        && read_u8(this + league::IMPORTANCE) == importance as u8 && read_u8(this + league::WIN_POINTS) == win_points as u8 && read_u8(this + league::DRAW_POINTS) == draw_points as u8
        && read_u8(this + league::MATCH_CATEGORY) == category as u8
        && (0..4).all(|i| read_u8(this + league::TIEBREAKS + i) == tiebreak_codes[i] && read_u8(this + league::PROMOTED_COUNT + i) == place_counts[i]) && read_u8(this + league::COMP_TYPE) == 2
        && read_u8(this + league::SUBSTITUTES_NAMED) == read_u8(parent + league::SUBSTITUTES_NAMED) && read_u8(this + league::SUBSTITUTIONS_ALLOWED) == read_u8(parent + league::SUBSTITUTIONS_ALLOWED)
        && read_usize(this + league::PRIZE_FUND) == prize_fund as usize
        && read_u8(this + league::PRIZE_MODE) == prize_mode as u8 && read_usize(this + league::TABLE) != 0 && read_usize(this + league::ROUND_DATES) != 0;
    if ok && clubs != 0 {
        ok = league_stage_table_clubs(this) == list;
    }
    SCOREBOARD.record(ok, || format!("construct_league_stage({:#x}): returned {:#x}, vtable {:#x}, clubs {}, win points {}, table {:#x}, rounds {:#x}", this, r, read_usize(this), read_i16(this + league::CLUB_COUNT), read_u8(this + league::WIN_POINTS), read_usize(this + league::TABLE), read_usize(this + league::ROUND_DATES)));
    r
}

/// The end of a round-robin stage (at the season turnover): the vtable 0x96d0f0, the table freed
/// (`exe::league_free_table`(this, 0)), the round dates (`league::ROUND_DATES`) freed (`exe::crt_free`, not zeroed),
/// the array at +0x18 freed and zeroed, and the League destructor (`exe::drop_league`). (The original's SEH frame has
/// no destructor on the normal path.)
/// Original: `League_Stage::~League_Stage` (0x68b6a0, league_stage.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn destroy_league_stage(this: usize) {
    write_usize(this, LEAGUE_STAGE_VTABLE);
    exe::league_free_table(this, 0usize);
    let r = read_usize(this + league::ROUND_DATES);
    if r != 0 {
        exe::crt_free(r);
    }
    let v = read_usize(this + 0x18);
    if v != 0 {
        exe::crt_free(v);
        write_usize(this + 0x18, 0);
    }
    exe::drop_league(this);
}

/// The destructor comparer: runs only the original (it frees; it cannot run twice), checking the +0x18 array zeroed.
unsafe extern "thiscall" fn compare_destroy_league_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    SCOREBOARD.record(read_usize(this + 0x18) == 0, || format!("destroy_league_stage({:#x}): +0x18 = {:#x}", this, read_usize(this + 0x18)));
}
