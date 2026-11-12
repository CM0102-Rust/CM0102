// ---------- 286a leva: league stages: the League_Stage constructor and destructor (league_stage.cpp) ----------

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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CONSTRUCT_LEAGUE_STAGE)(
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_LEAGUE_STAGE)(this);
    SCOREBOARD.record(read_usize(this + 0x18) == 0, || format!("destroy_league_stage({:#x}): +0x18 = {:#x}", this, read_usize(this + 0x18)));
}

// ---------- 286a leva: the constructors of the Italian super cups and of the US Open Cup (ita_c1_super.cpp, ita_super.cpp, usa_open_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn ita_super_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn ita_super_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn ita_super_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

const ITA_SUPER_VTABLE: usize = 0x96c8b8;
const ITA_C1_SUPER_VTABLE: usize = 0x96c1c4;
const USA_OPEN_CUP_VTABLE: usize = 0x970998;

/// The body of the Italian Super Cup constructor (see `construct_ita_super`).
unsafe fn construct_ita_super_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, ITA_SUPER_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_index(0x11);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 italian super cup new");
    let init_team_list: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x65fa60usize);
    if init_team_list(this) < 0 {
        show_error_at(0x9ea934, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9ea934, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea934, 0x58);
        return this;
    }
    this
}

/// The fixed fields the Italian Super Cup constructor writes on a new cup (offset, width, value).
const ITA_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Italian Super Cup constructor `ItaSuper::ItaSuper(year, record)`, run once at career creation. On the Cup::Cup
/// base (`exe::new_cup`): the competition record at +4, the vtable (0x96c8b8), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), 3 substitutions allowed, 7 substitutes named, discipline rule 17 and `run_by_stages()` = 0;
/// no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk
/// 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the two clubs
/// (ItaSuper::init_team_list 0x65fa60, thiscall, ret 0; the error box of line 0x43 if it returns a negative value); the
/// rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of
/// line 0x58 without them). Every failure returns this. [the bench, substitutions, discipline and type fields read
/// from the competitions reference, §2.1]
/// Original: `ItaSuper::ItaSuper` (0x65f670, ita_super.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_super_body(this, year, record) // cqv5: the Italian Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_ita_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_SUPER)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_SUPER_VTABLE, &ITA_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the Serie C1 Super Cup constructor (see `construct_ita_c1_super`).
unsafe fn construct_ita_c1_super_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, ITA_C1_SUPER_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_substitutions_allowed(3);
    cup.set_substitutes_named(7);
    cup.set_discipline_index(0x11);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 serie c1 super cup new");
    let init_team_list: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63c9d0usize);
    if init_team_list(this) < 0 {
        show_error_at(0x9ea1c8, 0x43);
        return this;
    }
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0x9ea1c8, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0x9ea1c8, 0x58);
        return this;
    }
    this
}

/// The fixed fields the Serie C1 Super Cup constructor writes on a new cup (offset, width, value).
const ITA_C1_SUPER_CTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x11), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Serie C1 Super Cup constructor `ItaC1Super::ItaC1Super(year, record)`, run once at career creation. On the
/// Cup::Cup base (`exe::new_cup`): the competition record at +4, the vtable (0x96c1c4), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), 3 substitutions allowed, 7 substitutes named, discipline rule 17 and `run_by_stages()` = 0;
/// no prize fund (`cup::PRIZE_FUND` stays as the base left it). If the saved state loads (Cup::load_from_disk
/// 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the two-club list
/// (ItaC1Super::init_team_list 0x63c9d0, thiscall, ret 0; the error box of line 0x43 if it returns a negative value);
/// the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790,
/// thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of
/// line 0x58 without them). Every failure returns this.
/// Original: `ItaC1Super::ItaC1Super` (0x63c5e0, ita_c1_super.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    construct_ita_c1_super_body(this, year, record) // cqv5: the Serie C1 Super Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_ita_c1_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_C1_SUPER)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, ITA_C1_SUPER_VTABLE, &ITA_C1_SUPER_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c1_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of the US Open Cup constructor (see `construct_usa_open_cup`).
unsafe fn construct_usa_open_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, USA_OPEN_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_stage_count(0);
    cup.set_current_stage(0xffff_ffff);
    cup.set_club_sides(1);
    cup.set_discipline_index(0x1e);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_5 us open cup new");
    let collect_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90b530usize);
    if collect_entrants(this) < 0 {
        show_error_at(0xaa20e0, 0x43);
        return this;
    }
    cup.set_prize_fund(0xf4240);
    ita_super_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) < 0 {
        show_error_at(0xaa20e0, 0x51);
        return this;
    }
    if !ita_super_ctor_create_stats(this) {
        show_error_at(0xaa20e0, 0x58);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the US Open Cup constructor writes on a new cup (offset, width, value).
const USA_OPEN_CUP_CTOR_FIELDS: [(usize, u8, u32); 8] = [(cup::PRIZE_FUND, 4, 0xf4240), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::DISCIPLINE_INDEX, 1, 0x1e), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The US Open Cup constructor `UsaOpenCup::UsaOpenCup(year, record)`, run once at career creation. On the Cup::Cup
/// base (`exe::new_cup`): the competition record at +4, the vtable (0x970998), the year (`cup::YEAR`),
/// `current_round()` = 0, `stage_index()` = 0xff, `stage_count()` = 0, `current_stage()` = −1, `club_sides()` = 1
/// (clubs of one nation), discipline rule 30 and `run_by_stages()` = 0; the bench and substitutions stay as the base
/// left them. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this, 1), ret 4; returns ≠ 0), it
/// returns this. On a new cup, in order: the entrants (UsaOpenCup::collect_entrants 0x90b530, thiscall, ret 0; the
/// error box of line 0x43 if it returns a negative value); the prize fund (`cup::PRIZE_FUND`) = 1,000,000; the rounds
/// (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret
/// 0; the error box of line 0x51 if it returns a negative value); the statistics at +8 (the error box of line 0x58
/// without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked). Every failure returns this.
/// Original: `UsaOpenCup::UsaOpenCup` (0x90aeb0, usa_open_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    construct_usa_open_cup_body(this, year, record) // cqv5: the US Open Cup
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`ita_super_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_usa_open_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_USA_OPEN_CUP)(this, year, record);
    let error = ita_super_ctor_plan_error(this, r, year, record, USA_OPEN_CUP_VTABLE, &USA_OPEN_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_usa_open_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

// ---------- 286a leva: the Copa do Brasil: the constructor and the 71 entrants (bra_cup.cpp) ----------

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
unsafe fn bra_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
unsafe fn bra_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
unsafe fn bra_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

const BRA_CUP_VTABLE: usize = 0x967cf8;

/// The body of the Copa do Brasil constructor (see `construct_bra_cup`).
unsafe fn construct_bra_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, BRA_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(1);
    cup.set_discipline_index(6);
    cup.set_run_by_stages(0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_6 copa do brasil new");
    let select_entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427f00usize);
    select_entrants(this);
    let build_team_table: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427c20usize);
    if build_team_table(this) == 0 {
        show_error_at(0x98a3bc, 0x4b);
        return this;
    }
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_prize_fund(0x7a120);
    bra_cup_ctor_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if map_fixture_tree(this) == 0 {
        show_error_at(0x98a3bc, 0x59);
        return this;
    }
    if !bra_cup_ctor_create_stats(this) {
        show_error_at(0x98a3bc, 0x60);
        return this;
    }
    let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    rank_teams(this);
    this
}

/// The fixed fields the Copa do Brasil constructor writes on a new cup (offset, width, value).
const BRA_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x6), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Copa do Brasil constructor `BraCup::BraCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the competition record at +4, the vtable (0x967cf8), the year (`cup::YEAR`), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 1 (clubs of one nation),
/// discipline rule 6 and `run_by_stages()` = 0. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this,
/// 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the 71 entrants (BraCup::select_entrants
/// 0x427f00, thiscall, ret 0, unchecked); the 71-team table (BraCup::build_team_table 0x427c20, thiscall, ret 0; the
/// error box of line 0x4b if it returns 0); 7 substitutes named, 3 substitutions allowed and the prize fund
/// (`cup::PRIZE_FUND`) = 500,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree
/// (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x59 if it returns 0); the statistics at +8
/// (the error box of line 0x60 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0,
/// unchecked). Every failure returns this. [the bench, substitutions, discipline and type fields read from the
/// competitions reference, §2.1]
/// Original: `BraCup::BraCup` (0x427430, bra_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    construct_bra_cup_body(this, year, record) // cqv6: the Copa do Brasil
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`bra_cup_ctor_plan_error`).
unsafe extern "thiscall" fn compare_construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_CUP)(this, year, record);
    let error = bra_cup_ctor_plan_error(this, r, year, record, BRA_CUP_VTABLE, &BRA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_bra_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The 71 fixed Copa do Brasil names, in slot order (texts in the exe, read from there like the original): Vasco (0),
/// Remo, Paysandu, River, Sampaio Corrêa, Nacional (AM), ABC, América (RN), Ceará, Confiança, CRB, Sergipe, Comercial
/// Bandeirantes, Comercial, Juventus, Serra, Tocantinópolis, União, Camaçari, Fluminense de Feira, Juazeiro, Vitória,
/// Caxias, Internacional, Juventude, Pelotas, Anapolina, Anápolis, Atlético Goianiense, Goiânia, Atlético Mineiro,
/// Caldense, Uberlândia, Villa Nova, Paraná, Londrina, Malutrom, Coritiba, Central, Ferroviário, Porto, Santa Cruz,
/// América (RJ), Americano, Bangu, Botafogo, Avaí, Criciúma, Figueirense, Tubarão, Botafogo (SP), Palmeiras, Rio
/// Branco, Santos, Sport, Ponte Preta, São Paulo, Gama, São Raimundo, Fluminense, Goiás, Vila Nova, Náutico, Bahia,
/// Fortaleza, Joinville, América (MG), Grêmio, Flamengo, Atlético Paranaense and Corinthians (70).
const BRA_CUP_ENTRANT_NAMES: [usize; 71] = [0x98a398, 0x98aa20, 0x98aa0c, 0x98a9f4, 0x98a9d4, 0x98a9b8, 0x98a9a4, 0x98a988, 0x98a974, 0x98a954, 0x98a93c, 0x98a924, 0x98a8f8, 0x98a8e0, 0x98a8b4, 0x98a8a0, 0x98a880, 0x98a86c, 0x98a854, 0x98a830, 0x98a818, 0x98a800, 0x98a7dc, 0x98a7c0, 0x98a7a8, 0x98a790, 0x98a770, 0x98a758, 0x98a73c, 0x98a724, 0x98a70c, 0x98a6ec, 0x98a6d0, 0x98a6b4, 0x98a6a4, 0x98a68c, 0x98a67c, 0x98a664, 0x98a650, 0x98a634, 0x98a61c, 0x98a600, 0x98a5e4, 0x98a5c4, 0x98a5ac, 0x98a58c, 0x98a578, 0x98a560, 0x98a544, 0x98a52c, 0x98a510, 0x98a4f0, 0x98a4d4, 0x98a4bc, 0x98a4a4, 0x98a484, 0x98a46c, 0x98a450, 0x98a344, 0x98a434, 0x98a420, 0x98a380, 0x98a364, 0x98a294, 0x98a408, 0x98a3f0, 0x98a328, 0x98a304, 0x98a2e4, 0x98a2c8, 0x98a2a8];

/// Is the club's name (`club::NAME`) equal to the text `t` (the original's inline comparison, byte by byte up to the
/// terminating zero)?
unsafe fn bra_cup_club_name_is(c: usize, t: usize) -> bool {
    let mut i = 0usize;
    loop {
        let a = read_u8(c + club::NAME + i);
        if a != read_u8(t + i) {
            return false;
        }
        if a == 0 {
            return true;
        }
        i += 1;
    }
}

/// The twelve Copa do Brasil state leagues: the key, the control byte (the best-club mark in step 2, later the count
/// of the others), the slot group (0: slots 0x36..; 1: 0x3d..; 2: 0x42..) and the ceiling for the others (5 or 7). The
/// order is the original's test order; the indices are the original's stack bytes minus 0x13. [inference: the three
/// with ceiling 7 are the biggest state leagues]
const BRA_CUP_STATE_LEAGUES: [(usize, usize, usize, i8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 0xc, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0xb, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0x7, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0xd, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0x9, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0xf, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0xe, 1, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0x4, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0xa, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0x5, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0x6, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0x8, 2, 5),
];

/// The club's state league (`club::RESERVE_DIVISION`; the record's +0 is its id), in the original's test order.
unsafe fn bra_cup_state_league_of(c: usize) -> Option<usize> {
    let id = read_usize(read_usize(c + club::RESERVE_DIVISION));
    BRA_CUP_STATE_LEAGUES.iter().position(|e| id == read_usize(e.0))
}

/// The body of `bra_cup_select_entrants` (also run by its comparer).
unsafe fn bra_cup_select_entrants_body(this: usize) {
    write_u8(this + 0x4b, 0xff);
    write_u16(this + cup::ENTRANT_CAPACITY, 0x47);
    write_u16(this + cup::ENTRANT_COUNT, 0x47);
    let book = exe::crt_calloc(6, 0x47);
    write_u32(this + cup::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0x98a3bc, 0x1e0);
        return;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let nation = read_usize(c + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            for (k, &t) in BRA_CUP_ENTRANT_NAMES.iter().enumerate() { // cqv6: the 71 names
                if bra_cup_club_name_is(c, t) {
                    write_u32(read_usize(this + cup::ENTRANT_BOOK) + k * 6, c as u32); // cqv6: the slot of the name
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut k: i16 = 0;
    while k < 0x47 {
        if read_usize(read_usize(this + cup::ENTRANT_BOOK) + k as usize * 6) == 0 {
            break;
        }
        k += 1;
    }
    if k == 0x47 {
        covered!("cqv_6 the 71 names");
        return;
    }
    covered!("cqv_6 fill from the state leagues");
    // the original's counters (stack bytes, zeroed at the start): [0] the next slot for the others, [1..3] the count of
    // each group (0x3d, 0x42, 0x36), [4..0xf] the mark and count of each state league, [0x10] those of the
    // KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION division
    let mut b = [0i8; 0x11];
    let group_count = |g: usize| [3usize, 1, 2][g];
    let group_first_slot = |g: usize| [0x36i32, 0x3d, 0x42][g];
    let mut list = [0usize; 200];
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if n >= 0xc8 {
            break;
        }
        let c = base + i as u16 as usize * club::SIZE;
        if read_usize(c + club::RESERVE_DIVISION) != 0 && bra_cup_state_league_of(c).is_some() {
            list[n as usize] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x4b07d0usize);
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, g, _) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] == 0 {
                let cg = group_count(g);
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + (group_first_slot(g) + b[cg] as i32) as usize * 6, c as u32);
                b[mark] = 1;
                b[cg] = b[cg].wrapping_add(1);
                list[j] = 0;
            }
        }
        let c = list[j];
        let d = if c != 0 { read_usize(c + club::DIVISION) } else { 0 };
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION) && b[0x10] < 5 {
            write_u32(read_usize(this + cup::ENTRANT_BOOK) + (0x36 + b[3] as i32) as usize * 6, c as u32);
            b[3] = b[3].wrapping_add(1);
            b[0x10] = b[0x10].wrapping_add(1);
            list[j] = 0;
        }
    }
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if c == 0 {
            continue;
        }
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, _, ceiling) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] < ceiling {
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + b[0] as i32 as usize * 6, c as u32);
                b[0] = b[0].wrapping_add(1);
                b[mark] = b[mark].wrapping_add(1);
            }
        }
    }
    if !(b[0] == 0x36 && b[3] == 7 && b[1] == 5 && b[2] == 5) {
        show_error_at(0x98a3bc, 0x3ce);
    }
}

/// The 71 Copa do Brasil entrants, chosen in the cup constructor (0x427430) at career creation. The entrant book
/// (`cup::ENTRANT_BOOK`, `exe::crt_calloc`(6, 0x47), cdecl: 0x47 six-byte records; out of memory, the error box of line
/// 0x1e0), `cup::ENTRANT_COUNT` = `cup::ENTRANT_CAPACITY` = 0x47 and +0x4b = 0xff.
/// 1) In club table order, each Brazilian club (`club::NATION` = [`KEY_NATION_BRAZIL`]) whose name (`club::NAME`) is
///    one of the 71 fixed ones (`BRA_CUP_ENTRANT_NAMES`) goes to that name's slot. With all 71 slots filled, done.
/// 2) Otherwise (an edited database): up to 200 clubs of the twelve state leagues (`club::RESERVE_DIVISION`,
///    `BRA_CUP_STATE_LEAGUES`) in a list sorted by reputation (`exe::crt_qsort` with 0x4b07d0, cdecl); the best of each
///    state league goes to its group (slots 0x36.., 0x3d.. or 0x42..), and up to 5 from the
///    [`KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION`] division (`club::DIVISION`) to group 0x36; then the others,
///    up to 5 per state league (7 in the three biggest [inference]), from slot 0 on. Without 54 + 7 + 5 + 5, the error
///    box of line 0x3ce.
/// Original: `BraCup::select_entrants` (0x427f00, bra_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_cup_select_entrants(this: usize) {
    bra_cup_select_entrants_body(this)
}

/// Side by side from the same state (`cup::ENTRANT_COUNT`/`cup::ENTRANT_CAPACITY`, +0x4b, [0xb67a34]; it draws
/// nothing): the original leaves its book in `cup::ENTRANT_BOOK`, the copy builds its own, compared over the 0x47
/// six-byte records and freed.
unsafe extern "thiscall" fn compare_bra_cup_select_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CUP_SELECT_ENTRANTS);
    let before = read_usize(this + cup::ENTRANT_BOOK);
    let book_original = core::cell::Cell::new(0usize);
    let book_rust = core::cell::Cell::new(0usize);
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(this + cup::ENTRANT_COUNT, 4usize), (this + 0x4b, 1), (0xb67a34, 4)], || {
        o(this);
        book_original.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, before as u32);
    }, || {
        bra_cup_select_entrants_body(this);
        book_rust.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, book_original.get() as u32);
    });
    let (lo, lr) = (book_original.get(), book_rust.get());
    let book = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x47 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x47 * 6);
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && book, || format!("bra_cup_select_entrants({:#x}) memory {} book {}", this, same, book));
}

// ---------- 286a leva: a Copa da Inglaterra (FA Cup): os 152 inscritos (eng_fa_cup.cpp) ----------

/// True if the club's division id is one of the five English league divisions (the generic lower division
/// `KEY_CLUB_COMP_A_LOWER_DIVISION`, the Premier and the three Football League divisions) or the Conference
/// [inference from the 20 + 24 x 3 + 22].
unsafe fn fa_cup_is_league_division(id: usize) -> bool {
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, KEY_CLUB_COMP_ENGLISH_CONFERENCE].iter().any(|&g| id == read_usize(g))
}

/// The original keeps its two lists side by side on the stack (the Conference one, 22 places, and right after it the
/// non-league one, 300): the copy uses a single array, the second list from index 22, so it overflows as the original
/// would (it never does).
const FA_CUP_NON_LEAGUE_START: i32 = 22;

/// Removes club `c` from the list starting at `start` (`n` used): swaps it with the last one and decrements the count;
/// returns whether it was found.
fn fa_cup_remove_club(v: &mut [usize; 322], start: i32, n: &mut i32, c: usize) -> bool {
    let mut found = false;
    let mut j = 0i32;
    while (j as i16) < *n as i16 {
        if v[(start + j) as usize] == c {
            *n -= 1;
            v[(start + j) as usize] = v[(start + *n as i16 as i32) as usize];
            found = true;
        }
        j += 1;
    }
    found
}

/// Writes the club in slot `k` of the cup's team table (`cup::TEAM_TABLE`, 6-byte entries: the club, then +4 and +5
/// zeroed) and counts it.
unsafe fn fa_cup_add_entrant(cup: Cup, k: &mut i32, c: usize) {
    let t = cup.team_table() + *k as usize * 6;
    write_usize(t, c);
    write_u8(t + 4, 0);
    write_u8(t + 5, 0);
    *k += 1;
}

/// The clubs of division `key`, in club table order, except those in `skip`, until the team table is full
/// (k == `cup::TEAM_COUNT`).
unsafe fn fa_cup_add_division(cup: Cup, k: &mut i32, key: usize, skip: &[usize]) {
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        if eng_auto_cup_is_in_comp(c, key) && !skip.contains(&c) {
            fa_cup_add_entrant(cup, k, c);
            if *k == cup.team_count() as i32 {
                return;
            }
        }
        i = i.wrapping_add(1);
    }
}

/// Builds the 152 FA Cup entrants. Called by the cup's daily update (`eng_fa_cup_update_for_date`, 0x570550) every 3
/// October; n = 1 up to 2002-06-01.
///
/// 1) The English clubs (`club::NATION` = `KEY_NATION_ENGLAND`) of the competition `KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP`
/// of the same year (`cup::YEAR`), searched among the [0xb63dfc] competitions of the list [0xb63e00] (the community's
/// `g_league_world_club_comp_count` and `g_league_world_club_comp_list`) by the id of its record (`league::COMPETITION`):
/// its list through the competition's virtual +0x58 (push 0; push &list; mov ecx -> thiscall, ret 8; returns how many;
/// the list is freed with `exe::crt_free`), up to 2 (the Club World Championship: Manchester United's withdrawal in
/// 1999/2000 [inference]). In 1999, with 1 found and `eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED` != 2,
/// `eng_fa_cup::FIFTH_ROUND_ENTRY` = 1 and `eng_fa_cup::EXEMPTION_COUNT` = 0 (the club leaves the cup); otherwise
/// `FIFTH_ROUND_ENTRY` = 0 and `EXEMPTION_COUNT` = how many.
/// 2) `cup::TEAM_COUNT` = 152 and the table (`exe::crt_malloc` of 0x390, not zeroed) in `cup::TEAM_TABLE` (out of memory:
/// the box for line 0x1d2 and -1).
/// 3) With the nation rule (the competition's nation `league_selected` has `LOWER_DIVISIONS_LOADED`) or in 2001 (year
/// 0x7d1): the winner and runner-up of the last season of `KEY_CLUB_COMP_ENGLISH_FA_TROPHY` (`exe::comp_util_get_latest_placed_club`,
/// cdecl, on the record `G_CLUB_COMP_TABLE` + 0x6b x the key's id) get a place; otherwise only the first club of the
/// table, if it is in the Conference (the original tests the same club over and over without advancing its loop).
/// 4) In club table order: the Conference clubs (`KEY_CLUB_COMP_ENGLISH_CONFERENCE`) in one list; the English clubs
/// (`club::NATION` = the record `G_NATION_TABLE` + `nation::SIZE` x the England id) outside the league divisions, up to
/// 300, in another (the non-league clubs).
/// 5) The two prize places leave their own list: from the Conference, with 21 places instead of 22 for each one; from
/// the non-league list, with 37 instead of 38 (when not found, the first of the list is replaced by the last).
/// 6) Two Conference clubs drawn (`exe::utils_random`(n - 2), cdecl, twice) go to the last two positions; the
/// non-league clubs sorted by reputation with the best ones shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`: push
/// places; push n; push list -> cdecl).
/// 7) The table: the non-league places (38), the Conference places (22), the two prize clubs, the Football League's
/// third (`KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION`), second and first divisions (except the clubs of step 1 and, in the
/// third, the prize clubs), each until the table is full; the last 24 (the first division) shuffled by 60 swaps of two
/// drawn positions (`exe::utils_random`(0x18) twice); the Premier (`KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION`); last,
/// the clubs of step 1. With `FIFTH_ROUND_ENTRY` = 1, `cup::TEAM_COUNT` drops by one. Returns 1 if the table is full
/// (k == `cup::TEAM_COUNT`), otherwise 0.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
unsafe fn fa_cup_collect_entrants(this: usize) -> i32 {
    let cup = Cup(this);
    // 1) the English clubs of the Club World Championship of the same year
    let mut same_year = [0usize; 2];
    let mut nf: i16 = 0;
    let n_comp = read_usize(0xb63dfc) as i32;
    if n_comp > 0 {
        let v = read_usize(0xb63e00);
        let mut i: i16 = 0;
        let mut found = 0usize;
        loop {
            let c = read_usize(v + i as i32 as usize * 4);
            if read_usize(read_usize(c + league::COMPETITION)) == read_usize(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) {
                found = c;
                break;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n_comp {
                break;
            }
        }
        if found != 0 && read_i16(found + cup::YEAR) == cup.year() {
            // the competition's virtual +0x58 (no name yet): fills a list of its clubs and returns how many
            let club_list: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(found) + 0x58));
            let mut list = 0usize;
            let n = club_list(found, &mut list, 0) as i16;
            let mut j: i16 = 0;
            while j < n && nf < 2 {
                let c = read_usize(list + j as i32 as usize * 4);
                let club_nation = read_usize(c + club::NATION);
                if club_nation != 0 && read_usize(club_nation) == read_usize(KEY_NATION_ENGLAND) {
                    same_year[nf as usize] = c;
                    nf += 1;
                }
                j += 1;
            }
            if list != 0 {
                exe::crt_free(list);
            }
        }
    }
    if cup.year() == 0x7cf && nf == 1 && read_u8(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED) != 2 {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY, nf as u8);
        write_u8(this + eng_fa_cup::EXEMPTION_COUNT, 0);
    } else {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 0);
        write_u8(this + eng_fa_cup::EXEMPTION_COUNT, nf as u8);
    }
    // 2) the table
    cup.set_team_count(0x98);
    let t = exe::crt_malloc(0x390);
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(0x9c42cc, 0x1d2);
        return -1;
    }
    // 3) the prize places
    let base = read_usize(G_CLUB_TABLE);
    let mut a = 0usize;
    let mut b = 0usize;
    let cup_nation = read_usize(read_usize(this + league::COMPETITION) + competition::NATION);
    if read_u8(cup_nation + nation::LEAGUE_SELECTED) & league_selected::LOWER_DIVISIONS_LOADED != 0 || cup.year() == 0x7d1 {
        covered!("cqv_7 trophy winner and runner-up");
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        a = exe::comp_util_get_latest_placed_club(record, 0); // cqv7: the winner
        let record = read_usize(G_CLUB_COMP_TABLE) + read_usize(KEY_CLUB_COMP_ENGLISH_FA_TROPHY).wrapping_mul(0x6b);
        b = exe::comp_util_get_latest_placed_club(record, 1);
    } else if read_usize(G_CLUB_COUNT) as i32 > 0 && eng_auto_cup_is_in_comp(base, KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
        // simplification: the original repeats the test of the first club without advancing its loop; one test is enough
        a = base;
    }
    // 4) the Conference and the non-league clubs
    // simplification: the original does not zero the lists (stack leftovers); it only reads past what it filled when
    // clubs are missing (fewer than 38 non-league), which the career never has; the copy uses 0, which the original
    // also skips
    let mut v = [0usize; 322];
    let mut nconf = 0i32;
    let mut nout = 0i32;
    let england = read_usize(G_NATION_TABLE) + read_usize(KEY_NATION_ENGLAND).wrapping_mul(nation::SIZE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let l = read_usize(c + club::DIVISION);
        if l != 0 {
            let id = read_usize(l);
            if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
                // simplification: the original does not check the size of the Conference list (22 places; the Conference has 22)
                v[nconf as i16 as usize] = c;
                nconf += 1;
            } else if !fa_cup_is_league_division(id) && read_usize(c + club::NATION) == england && (nout as i16) < 0x12c {
                v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize] = c;
                nout += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    // 5) the prize clubs leave their own list
    let mut conf_places: i8 = 0x16;
    let mut non_league_places: i8 = 0x26;
    for (k, &p) in [a, b].iter().enumerate() {
        if p == 0 || read_usize(p + club::DIVISION) == 0 {
            continue;
        }
        let id = read_usize(read_usize(p + club::DIVISION));
        if id == read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) {
            fa_cup_remove_club(&mut v, 0, &mut nconf, p);
            if k == 0 {
                conf_places = 0x15;
            } else {
                conf_places -= 1;
            }
        } else if !fa_cup_is_league_division(id) {
            if !fa_cup_remove_club(&mut v, FA_CUP_NON_LEAGUE_START, &mut nout, p) {
                nout -= 1;
                v[FA_CUP_NON_LEAGUE_START as usize + k] = v[(FA_CUP_NON_LEAGUE_START + nout as i16 as i32) as usize];
            }
            if k == 0 {
                non_league_places = 0x25;
            } else {
                non_league_places -= 1;
            }
        }
    }
    // 6) two drawn Conference clubs go to the end; the non-league clubs by reputation
    let n = nconf as i16 as i32;
    let r1 = exe::utils_random(n - 2) as i8 as i32;
    let r2 = exe::utils_random(n - 2) as i8 as i32;
    v.swap(r1 as usize, (n - 1) as usize);
    v.swap(r2 as usize, (n - 2) as usize);
    // simplification: the original pushes the places in an edi with garbage in the high 16 bits; 0x4afbd0 reads the word
    exe::comp_util_sort_and_shuffle_top_clubs(v.as_mut_ptr() as usize + FA_CUP_NON_LEAGUE_START as usize * 4, nout as u32, non_league_places as i16 as u16 as u32); // cqv7: the non-league clubs by reputation
    // 7) the table
    let mut k = 0i32;
    for j in 0..(non_league_places as i32).max(0) {
        let c = v[(FA_CUP_NON_LEAGUE_START + j) as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    for j in 0..(conf_places as i32).max(0) {
        let c = v[j as usize];
        if c != 0 {
            fa_cup_add_entrant(cup, &mut k, c);
        }
    }
    if a != 0 {
        fa_cup_add_entrant(cup, &mut k, a);
    }
    if b != 0 {
        fa_cup_add_entrant(cup, &mut k, b);
    }
    let (x1, x2) = (same_year[0], same_year[1]);
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION, &[a, b, x1, x2]); fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION, &[x1, x2]); // cqv7: the third and the second
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION, &[x1, x2]);
    covered!("cqv_7 first division shuffled");
    for _ in 0..0x3c { // cqv7: 60 swaps in the first division
        let r1 = exe::utils_random(0x18) as i8 as i32;
        let r2 = exe::utils_random(0x18) as i8 as i32;
        let t = cup.team_table();
        let p1 = t + (k - r1 - 1) as usize * 6;
        let p2 = t + (k - r2 - 1) as usize * 6;
        let v1 = read_usize(p1);
        write_usize(p1, read_usize(p2));
        write_usize(p2, v1);
    }
    fa_cup_add_division(cup, &mut k, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, &[x1, x2]);
    let nb = read_i8(this + eng_fa_cup::EXEMPTION_COUNT);
    let mut j: i16 = 0;
    while j < nb as i16 {
        fa_cup_add_entrant(cup, &mut k, same_year[j as usize]);
        j += 1;
    }
    if read_u8(this + eng_fa_cup::FIFTH_ROUND_ENTRY) == 1 {
        cup.set_team_count(cup.team_count().wrapping_sub(1));
    }
    (k == cup.team_count() as i32) as i32
}

/// The FA Cup's 152 entrants: see `fa_cup_collect_entrants`.
/// Original: `EngFaCup::collect_entrants` (0x56fab0, eng_fa_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn eng_fa_cup_collect_entrants(this: usize) -> i32 {
    fa_cup_collect_entrants(this)
}

/// Side by side from the same state (the random generator, `cup::TEAM_COUNT`, `eng_fa_cup::EXEMPTION_COUNT` and
/// `FIFTH_ROUND_ENTRY`, `G_MOUSE_LEFT_BUTTON_DOWN`): the original leaves its table in `cup::TEAM_TABLE`, the copy builds its own
/// (without counting the draws), compared on the filled slots (the club of each one; the table is not zeroed) and
/// freed; the return value too.
unsafe extern "thiscall" fn compare_eng_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_ENG_FA_CUP_COLLECT_ENTRANTS);
    let before = read_usize(this + cup::TEAM_TABLE);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(this + cup::TEAM_COUNT, 2usize), (this + eng_fa_cup::EXEMPTION_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, before);
        r
    }, || {
        let r = crate::rng::without_counting(|| fa_cup_collect_entrants(this));
        lr.set(read_usize(this + cup::TEAM_TABLE));
        write_usize(this + cup::TEAM_TABLE, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = (read_i16(this + cup::TEAM_COUNT).max(0) as usize).min(0x98);
    let table = lo == 0 || lr == 0 || a != 1 || (0..n).all(|j| read_usize(lo + j * 6) == read_usize(lr + j * 6) && read_u16(lo + j * 6 + 4) == read_u16(lr + j * 6 + 4));
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && table, || format!("eng_fa_cup_collect_entrants({:#x}) returned {} x {} memory {} table {}", this, a, b, same, table));
    a
}

// ---------- 286a leva: os construtores da Copa da Itália e da Copa da Série C (ita_c_cup.cpp, ita_cup.cpp) ----------

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS`; the original's SEH frame covers only that `new`. Returns whether they were created.
unsafe fn ita_cups_create_comp_stats(this: League) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record
/// (`league::COMPETITION`) and the year (`league::YEAR`); on a new cup (`league::STAGE_INDEX` = `stage_index::MAIN`),
/// each fixed field the constructor writes (offset, width, value) and the rounds (`cup::ROUND_TABLE`, when `rounds`)
/// and the statistics created.
unsafe fn ita_cups_check_constructor_plan(this: usize, r: usize, year: u32, owner: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + league::COMPETITION) != owner || read_u16(this + league::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + league::COMPETITION), read_u16(this + league::YEAR)));
    }
    if read_u8(this + league::STAGE_INDEX) != stage_index::MAIN {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + league::COMP_STATS) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + league::COMP_STATS)));
    }
    None
}

/// The constructor of the Coppa Italia, on the League base (`exe::new_league`, thiscall, ret 0): the fixed fields (the
/// vtable 0x96c318, the competition record in `league::COMPETITION`, the year in `league::YEAR`, 3 substitutions
/// allowed, 7 substitutes named, discipline rule `ITALY_CUP` (18), importance 0xc); if the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; returns != 0), returns this. On a new cup, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = -1,
/// `league::STAGE_COUNT` = 8 and the stage array in `league::STAGES` (`exe::crt_malloc` of 0x20, cdecl, unchecked);
/// the 48 entrants (`ita_cup_select_entrants`, 0x63faf0, thiscall, ret 0, unchecked); the statistics in
/// `league::COMP_STATS` (the box for line 0x54 without them); the predicted positions (ItaCup::set_predicted_positions,
/// 0x640210, thiscall, ret 0, unchecked); the rules (`ita_cup_set_up_rules`, 0x63f350, thiscall, ret 0; the box for
/// line 0x5e if it returns 0); the first group table (`ita_cup_create_first_group_table`, 0x63f250, thiscall, ret 0;
/// the box for line 0x65 if it returns 0); the fixtures (`exe::league_create_fixtures`, thiscall, ret 0; the box for
/// line 0x6b if it returns 0); the table order (`exe::league_sort_table`(this, 0), thiscall, ret 4; the box for line
/// 0x72 if it returns 0); the stage of each group (`ita_cup_create_group_stage`, 0x63f640, push i; mov ecx, this ->
/// thiscall, ret 4) for groups 0 to 6. Every failing step shows its box (`show_error_at`) and returns this. Returns
/// this. Rule: no prize fund (`league::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3
/// substitutions; discipline rule 18 [`league::SUBSTITUTES_NAMED`/`SUBSTITUTIONS_ALLOWED`/`DISCIPLINE_RULE`/
/// `CLUB_SIDES` from the competitions reference reading, section 2.1].
/// Original: `ItaCup::ItaCup` (0x63e900, ita_cup.cpp, thiscall, ret 8).
unsafe fn ita_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x96c318);
    write_u16(this + league::YEAR, year as u16);
    league.set_substitutions_allowed(0x3);
    league.set_substitutes_named(0x7);
    league.set_discipline_rule(discipline_rule::ITALY_CUP);
    league.set_importance(0xc);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqv_8 coppa italia new");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffffffff);
    write_u32(this + league::STAGE_COUNT, 0x8);
    league.set_stages(exe::crt_malloc(0x20));
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63faf0usize);
    f(this);
    if !ita_cups_create_comp_stats(league) {
        show_error_at(0x9ea23c, 0x54);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x640210usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f350usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x5e);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f250usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x65);
        return this;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0x9ea23c, 0x6b);
        return this;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0x9ea23c, 0x72);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63f640usize);
    for i in 0..7 {
        f(this, i);
    }
    this
}

/// The fixed fields the Coppa Italia constructor writes on a new cup (offset, width, value).
const COPPA_ITALIA_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 0x8), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 0x7), (league::SUBSTITUTIONS_ALLOWED, 1, 0x3), (league::DISCIPLINE_RULE, 1, 0x12), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];

/// The Coppa Italia constructor: see `ita_cup_constructor_body`.
/// Original: `ItaCup::ItaCup` (0x63e900, ita_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_cup(this: usize, year: u32, owner: usize) -> usize {
    ita_cup_constructor_body(this, year, owner) // cqv8: the Coppa Italia
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_ita_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_CUP)(this, year, owner);
    let error = ita_cups_check_constructor_plan(this, r, year, owner, 0x96c318, &COPPA_ITALIA_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Serie C cup, on the League base (`exe::new_league`, thiscall, ret 0): the fixed fields (the
/// vtable 0x96c264, the competition record in `league::COMPETITION`, the year in `league::YEAR`, 3 substitutions
/// allowed, 7 substitutes named, discipline rule `ITALY_CUP` (18), importance 0xc); if the saved state loads
/// (`exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; returns != 0), returns this. On a new cup, in order:
/// `league::LEAGUE_FLAGS` = 0x28, `league::STAGE_INDEX` = `stage_index::MAIN`, `league::CURRENT_STAGE` = -1,
/// `league::STAGE_COUNT` = 0x10 and the stage array in `league::STAGES` (`exe::crt_malloc` of 0x40, cdecl,
/// unchecked); the 90 entrants (`ita_c_cup_select_entrants`, 0x63dfe0, thiscall, ret 0, unchecked); the statistics in
/// `league::COMP_STATS` (the box for line 0x4f without them); the predicted positions (ItaCCup::set_predicted_positions,
/// 0x63e580, thiscall, ret 0, unchecked); the rules (`ita_c_cup_set_up_rules`, 0x63d6e0, thiscall, ret 0; the box for
/// line 0x59 if it returns 0); the first group table (`init_first_group`, 0x63d5e0, thiscall, ret 0; the box for line
/// 0x60 if it returns 0); the fixtures (`exe::league_create_fixtures`, thiscall, ret 0; the box for line 0x66 if it
/// returns 0); the table order (`exe::league_sort_table`(this, 0), thiscall, ret 4; the box for line 0x6d if it returns
/// 0); the stage of each group (ItaCCup::create_group_stage, 0x63d9e0, push i; mov ecx, this -> thiscall, ret 4) for
/// groups 0 to 14. Every failing step shows its box (`show_error_at`) and returns this. Returns this. Rule: no prize
/// fund (`league::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3 substitutions; discipline
/// rule 18 [from the competitions reference reading, section 2.1].
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
unsafe fn ita_c_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x96c264);
    write_u16(this + league::YEAR, year as u16);
    league.set_substitutions_allowed(0x3);
    league.set_substitutes_named(0x7);
    league.set_discipline_rule(discipline_rule::ITALY_CUP);
    league.set_importance(0xc);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqv_8 serie c cup new");
    league.set_league_flags(0x28);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffffffff);
    write_u32(this + league::STAGE_COUNT, 0x10);
    league.set_stages(exe::crt_malloc(0x40));
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63dfe0usize);
    f(this);
    if !ita_cups_create_comp_stats(league) {
        show_error_at(0x9ea204, 0x4f);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63e580usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d6e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x59);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d5e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x60);
        return this;
    }
    if exe::league_create_fixtures(league) == 0 {
        show_error_at(0x9ea204, 0x66);
        return this;
    }
    if exe::league_sort_table(league, 0) == 0 {
        show_error_at(0x9ea204, 0x6d);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63d9e0usize);
    for i in 0..15 {
        f(this, i);
    }
    this
}

/// The fixed fields the Serie C cup constructor writes on a new cup (offset, width, value).
const SERIE_C_CUP_FIELDS: [(usize, u8, u32); 7] = [(league::STAGE_COUNT, 4, 0x10), (league::STAGE_INDEX, 1, 0xff), (league::SUBSTITUTES_NAMED, 1, 0x7), (league::SUBSTITUTIONS_ALLOWED, 1, 0x3), (league::DISCIPLINE_RULE, 1, 0x12), (league::IMPORTANCE, 1, 0xc), (league::LEAGUE_FLAGS, 2, 0x28)];

/// The Serie C cup constructor: see `ita_c_cup_constructor_body`.
/// Original: `ItaCCup::ItaCCup` (0x63cc40, ita_c_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    ita_c_cup_constructor_body(this, year, owner) // cqv8: the Serie C cup
}

/// Only the original (it allocates, draws and reads the save file), checking the plan (`ita_cups_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_ita_c_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ITA_C_CUP)(this, year, owner);
    let error = ita_cups_check_constructor_plan(this, r, year, owner, 0x96c264, &SERIE_C_CUP_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_ita_c_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

// ---------- 286a leva: os construtores do Troféu da Liga inglesa e da Liga dos Campeões (eng_auto_cup.cpp, european_cup.cpp) ----------

/// The cup's rounds through its virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push -1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `cup::ROUND_TABLE`.
unsafe fn eng_auto_european_create_rounds(cup: Cup) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = f(cup.addr(), 0xffff_ffff, cup.addr() + cup::ROUND_COUNT, cup.addr() + cup::STAGE_LABEL, 0);
    cup.set_round_table(t);
}

/// The statistics (`exe::crt_operator_new` of 0x5ce, cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) into
/// `league::COMP_STATS` (the Comp base field); the original's SEH frame covers only that `new`. Returns whether they
/// were created.
unsafe fn eng_auto_european_create_comp_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + league::COMP_STATS, s);
    s != 0
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record
/// (`league::COMPETITION`) and the year (`cup::YEAR`); on a new cup (`cup::STAGE_INDEX` = `stage_index::MAIN`), each
/// fixed field the constructor writes (offset, width, value) and the rounds (`cup::ROUND_TABLE`, when `rounds`) and the
/// statistics created.
unsafe fn eng_auto_european_check_constructor_plan(this: usize, r: usize, year: u32, owner: usize, vt: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vt || read_usize(this + league::COMPETITION) != owner || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + league::COMPETITION), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != stage_index::MAIN {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + league::COMP_STATS) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + league::COMP_STATS)));
    }
    None
}

/// The constructor of the English League Trophy, on the Cup base (`exe::new_cup`, thiscall, ret 0): the fixed fields
/// (the competition record in `league::COMPETITION`, the vtable 0x969894, the year in `cup::YEAR`,
/// `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN`, `cup::CURRENT_STAGE` = -1, `cup::STAGE_COUNT` =
/// 2, `cup::COMP_TYPE` = `comp_type::CUP`, +0x1c and +0x20 = -1, `eng_auto_cup::SELECTED_CLUBS` = 0, discipline rule
/// `ENGLAND` (9), 5 substitutes named, 3 substitutions allowed); if the saved state loads (Cup::load_from_disk 0x51fc00:
/// push 1; mov ecx, this -> thiscall, ret 4; returns != 0), returns this. On a new cup, in order: the stage array in
/// `cup::STAGES` (`exe::crt_malloc` of 4 x `cup::STAGE_COUNT`, or 0 with no stages); `cup::IMPORTANCE_BIAS` = 0xf and
/// `cup::RUN_BY_STAGES` = 0; the 56 section clubs (`select_section_clubs`, 0x56b730, thiscall, ret 0; the box for line
/// 0x52 if it returns 0); the entrants (`eng_auto_cup_collect_entrants`, 0x56b3c0, thiscall, ret 0; the box for line
/// 0x59 if it returns 0); the prize fund (`cup::PRIZE_FUND` = 400,000); the rounds (`eng_auto_european_create_rounds`);
/// the statistics in `league::COMP_STATS` (the box for line 0x68 without them); the section stage
/// (`create_section_stage`, 0x56bab0, thiscall, ret 0, unchecked); the list `eng_auto_cup::SELECTED_CLUBS` freed
/// (`exe::crt_free`) and zeroed; the tie tree (`map_fixture_tree`, 0x518790, thiscall, ret 0; the box for line 0x78 if
/// it returns 0); the final ranking (`eng_auto_cup_rank_teams`, 0x56c010, thiscall, ret 0, unchecked). Every failing
/// step shows its box (`show_error_at`) and returns this. Returns this. Rule: prize fund of 400,000; 5 substitutes on the
/// bench and 3 substitutions; discipline rule 9 [from the competitions reference reading, section 2.1].
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
unsafe fn eng_auto_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_usize(this + league::COMPETITION, owner);
    write_usize(this, 0x969894);
    write_u16(this + cup::YEAR, year as u16);
    cup.set_current_round(0x0);
    cup.set_stage_index(stage_index::MAIN);
    write_u32(this + cup::CURRENT_STAGE, 0xffffffff);
    write_u32(this + cup::STAGE_COUNT, 0x2);
    cup.set_comp_type(comp_type::CUP);
    write_u32(this + 0x1c, 0xffffffff);
    write_u32(this + 0x20, 0xffffffff);
    write_u32(this + eng_auto_cup::SELECTED_CLUBS, 0x0);
    cup.set_discipline_index(discipline_rule::ENGLAND);
    cup.set_substitutes_named(0x5);
    cup.set_substitutions_allowed(0x3);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this, 1) != 0 {
        return this;
    }
    covered!("cqv_9 league trophy new");
    let stages = cup.stage_count() as i32;
    cup.set_stages(if stages > 0 { exe::crt_malloc((stages << 2) as u32) } else { 0 });
    cup.set_importance_bias(0xf);
    cup.set_run_by_stages(0x0);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b730usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x52);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b3c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x59);
        return this;
    }
    cup.set_prize_fund(0x61a80);
    eng_auto_european_create_rounds(cup);
    if !eng_auto_european_create_comp_stats(this) {
        show_error_at(0x9c4180, 0x68);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56bab0usize);
    f(this);
    let l = read_usize(this + eng_auto_cup::SELECTED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this + eng_auto_cup::SELECTED_CLUBS, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x78);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56c010usize);
    f(this);
    this
}

/// The fixed fields the League Trophy constructor writes on a new cup (offset, width, value).
const LEAGUE_TROPHY_FIELDS: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (cup::PRIZE_FUND, 4, 0x61a80), (cup::STAGE_COUNT, 4, 0x2), (cup::COMP_TYPE, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x5), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x9), (cup::IMPORTANCE_BIAS, 1, 0xf), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0), (eng_auto_cup::SELECTED_CLUBS, 4, 0x0)];

/// The English League Trophy constructor: see `eng_auto_cup_constructor_body`.
/// Original: `EngAutoCup::EngAutoCup` (0x56aa30, eng_auto_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    eng_auto_cup_constructor_body(this, year, owner) // cqv9: the League Trophy
}

/// Only the original (it allocates, draws and reads the save file), checking the plan
/// (`eng_auto_european_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_eng_auto_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ENG_AUTO_CUP)(this, year, owner);
    let error = eng_auto_european_check_constructor_plan(this, r, year, owner, 0x969894, &LEAGUE_TROPHY_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_eng_auto_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the Champions League, on the Cup base (`exe::new_cup`, thiscall, ret 0): the fixed fields (the
/// year in `cup::YEAR`, the vtable 0x96a1d0, `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN`,
/// `cup::CURRENT_STAGE` = -1, `cup::STAGE_COUNT` = 0xd, the competition record in `league::COMPETITION`,
/// `cup::CLUB_SIDES` = 2 (clubs of several nations), +0x1c and +0x20 = -1, +0x52 = 3, 7 substitutes named, 3
/// substitutions allowed, `cup::IMPORTANCE_BIAS` = 0xa, discipline rule `EUROPE` (1), the stage array in `cup::STAGES`
/// (`exe::crt_malloc` of 0x34, cdecl, unchecked), `cup::STAGE_LABEL` = 0, `cup::RUN_BY_STAGES` = 0); if the saved state
/// loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this -> thiscall, ret 4; returns != 0), returns this. On a new
/// cup, in order: the year's European seeding (`update_season`, 0x9035a0, on `G_UEFA_SEEDING` if there is one: push 0;
/// mov ecx -> thiscall, ret 4); the 71 entrants (`european_cup_collect_entrants`, 0x584fa0, thiscall, ret 0,
/// unchecked); the qualifying table (`build_qualifying_entry_table`, 0x584990, thiscall, ret 0; the box for line 0x74 if
/// it returns 0); the rounds (`eng_auto_european_create_rounds`); the tie tree (`map_fixture_tree`, 0x518790, thiscall,
/// ret 0; the box for line 0x82 if it returns 0); the statistics in `league::COMP_STATS` (the box for line 0x89 without
/// them); the final ranking (`european_cup_rank_teams`, 0x586900, thiscall, ret 0, unchecked). Every failing step shows
/// its box (`show_error_at`) and returns this. Returns this. Rule: type 2 (clubs of several nations); no prize fund
/// (`cup::PRIZE_FUND` stays as the base left it); 7 substitutes on the bench and 3 substitutions; discipline rule 1 [from
/// the competitions reference reading, section 2.1].
/// Original: `EuropeanCup::EuropeanCup` (0x5837c0, european_cup.cpp, thiscall, ret 8).
unsafe fn european_cup_constructor_body(this: usize, year: u32, owner: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(cup);
    write_u16(this + cup::YEAR, year as u16);
    write_usize(this, 0x96a1d0);
    cup.set_current_round(0x0);
    cup.set_stage_index(stage_index::MAIN);
    write_u32(this + cup::CURRENT_STAGE, 0xffffffff);
    write_u32(this + cup::STAGE_COUNT, 0xd);
    write_usize(this + league::COMPETITION, owner);
    cup.set_club_sides(0x2);
    write_u32(this + 0x1c, 0xffffffff);
    write_u32(this + 0x20, 0xffffffff);
    write_u8(this + 0x52, 0x3);
    cup.set_substitutes_named(0x7);
    cup.set_substitutions_allowed(0x3);
    cup.set_importance_bias(0xa);
    cup.set_discipline_index(discipline_rule::EUROPE);
    cup.set_stages(exe::crt_malloc(0x34));
    cup.set_stage_label(0x0);
    cup.set_run_by_stages(0x0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this, 1) != 0 {
        return this;
    }
    covered!("cqv_9 champions league new");
    // the year's European seeding (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [G_UEFA_SEEDING] -> thiscall, ret 4)
    let s = read_usize(G_UEFA_SEEDING);
    if s != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        f(s, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584fa0usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584990usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x74);
        return this;
    }
    eng_auto_european_create_rounds(cup);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x82);
        return this;
    }
    if !eng_auto_european_create_comp_stats(this) {
        show_error_at(0x9c4900, 0x89);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x586900usize);
    f(this);
    this
}

/// The fixed fields the Champions League constructor writes on a new cup (offset, width, value).
const CHAMPIONS_LEAGUE_FIELDS: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (cup::STAGE_COUNT, 4, 0xd), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x2), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1), (cup::IMPORTANCE_BIAS, 1, 0xa), (0x52, 1, 0x3), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The Champions League constructor: see `european_cup_constructor_body`.
/// Original: `EuropeanCup::EuropeanCup` (0x5837c0, european_cup.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn construct_european_cup(this: usize, year: u32, owner: usize) -> usize {
    european_cup_constructor_body(this, year, owner) // cqv9: the Champions League
}

/// Only the original (it allocates, draws and reads the save file), checking the plan
/// (`eng_auto_european_check_constructor_plan`).
unsafe extern "thiscall" fn compare_construct_european_cup(this: usize, year: u32, owner: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_EUROPEAN_CUP)(this, year, owner);
    let error = eng_auto_european_check_constructor_plan(this, r, year, owner, 0x96a1d0, &CHAMPIONS_LEAGUE_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_european_cup({:#x}, {}, {:#x}): {}", this, year as u16, owner, error.clone().unwrap_or_default()));
    r
}

// ---------- 286a leva: a Copa das Nações da Oceania: o dia do sorteio, o sorteio das seis seleções e a tabela do primeiro grupo (oceania_nations.cpp) ----------

/// The draw date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the day
/// `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
unsafe fn oceania_nations_draw_date(this: League) -> (u16, u16) {
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which 0x549ef0 also writes
    let mut date = [0u32; 2];
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    exe::new_date(date.as_mut_ptr() as usize, this.next_draw_day() as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// The day of the Oceania Nations Cup (the virtual `comp_vtable::UPDATE_FOR_DATE`; SEH frame only for the local dates,
/// no destructor). Without `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov ecx ->
/// thiscall, ret 8). Before the draw (today's year lower, unsigned; or the same year and the day lower, signed) it
/// returns 0 without the league day. On the draw day (`oceania_nations_draw_date`), with `league::ACTIVE` = 0: the six
/// nations (`oceania_nations_select_and_draw_teams`, 0x799e30, thiscall, ret 0), the predicted positions (the virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group table (`oceania_nations_create_group_table`,
/// 0x7995a0, thiscall, ret 0; 0: the box for line 0x405 and 0), the fixtures (`exe::league_create_fixtures`; 0: the box
/// for line 0x40b and 0), the sorted table (`exe::league_sort_table`(this, 0): push 0; mov ecx -> thiscall, ret 4; 0:
/// the box for line 0x412 and 0), the second group stage (0x799a60(this, 0): push 0; mov ecx -> thiscall, ret 4), the
/// entrant list +0xee freed (`exe::crt_free`) and zeroed, its count +0xf2 (word) = 0 and `league::ACTIVE` = 1. Then the
/// league day, whose return value it passes on.
/// Original: `OceaniaNations::update_for_date` (0x79a100, oceania_nations.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    let select_and_draw: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x799e30usize);
    let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7995a0usize);
    let second_group: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x799a60usize);
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if full == 0 {
        return league_day(this, today, 0);
    }
    let (day, year) = oceania_nations_draw_date(league);
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 {
        covered!("cqv_10 draw day");
        select_and_draw(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        if first_group(this) == 0 {
            show_error_at(0xa6a0ec, 0x405);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xa6a0ec, 0x40b);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xa6a0ec, 0x412);
            return 0;
        }
        second_group(this, 0);
        exe::crt_free(read_usize(this + 0xee));
        write_u32(this + 0xee, 0);
        write_i16(this + 0xf2, 0);
        write_u32(this + league::ACTIVE, 1);
    }
    league_day(this, today, full)
}

// simplification: the draw, the tables, the fixtures and the league day cannot run twice. Only the original; the plan on
// the draw day (the copy's condition): `league::ACTIVE` = 1, the list +0xee and +0xf2 zeroed, the first group table and
// the second group stage created.
unsafe extern "thiscall" fn compare_oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OCEANIA_NATIONS_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        let stages = league.stages();
        let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_i16(this + 0xf2) == 0 && league.table() != 0 && stages != 0 && read_usize(stages) != 0;
        SCOREBOARD.record(ok, || format!("oceania_nations_update_for_date({:#x}): the draw did not build the groups (+0x45 {})", this, league.active()));
    }
    r
}

/// The record of the nation of key `key` (global): `G_NATION_TABLE` + `nation::SIZE` x [key].
unsafe fn oceania_nations_nation_record(key: usize) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE))
}

/// The six nations of the Oceania Nations Cup and their draw. +0xf2 = 6 and the list +0xee of 6 national teams
/// (`exe::crt_malloc`(0x18), cdecl; out of memory, the box for line 0x39e and 0). Of the Oceania nations (continent
/// `KEY_CONTINENT_OCEANIA`, `nation::CONTINENT`) other than Australia (`KEY_NATION_AUSTRALIA`) and New Zealand
/// (`KEY_NATION_NEW_ZEALAND`): up to 10 Melanesian (`exe::database_is_melanesian_nation`) and up to 10 Polynesian
/// (`exe::database_is_polynesian_nation`; a Melanesian one beyond the 10 falls through to the Polynesian test), each by
/// its main national team (`exe::database_get_national_team`). A draw of 2 (`exe::utils_random`(2), cdecl) puts
/// Australia in slot 0 and New Zealand in slot 3 (with 1) or the other way round; the Melanesian and the Polynesian
/// teams are sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 2): 3 pushes -> cdecl); another
/// draw puts the first two Melanesian teams in slots 1 and 4 (or 4 and 1) and another the first two Polynesian ones in
/// slots 2 and 5 (or 5 and 2). Returns 1.
/// Original: `OceaniaNations::select_and_draw_teams` (0x799e30, oceania_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    write_i16(this + 0xf2, 6);
    let list = exe::crt_malloc(0x18);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0xa6a0ec, 0x39e);
        return 0;
    }
    let (mut mel, mut pol) = ([0usize; 10], [0usize; 10]);
    let (mut nm, mut np) = (0i8, 0i8);
    let oceania = read_usize(KEY_CONTINENT_OCEANIA);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let rec = read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE));
        let cont = Nation(rec).continent();
        let k = i as i32;
        if cont != 0 && read_usize(cont) == oceania && k != read_usize(KEY_NATION_AUSTRALIA) as i32 && k != read_usize(KEY_NATION_NEW_ZEALAND) as i32 {
            if exe::database_is_melanesian_nation(rec) != 0 && nm < 10 {
                mel[nm as usize] = exe::database_get_national_team(rec);
                nm += 1;
            } else if exe::database_is_polynesian_nation(rec) != 0 && np < 10 {
                // cqv10: up to 10 Polynesian teams
                pol[np as usize] = exe::database_get_national_team(rec);
                np += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("cqv_10 oceania cup draw");
    let (australia, new_zealand) = (oceania_nations_nation_record(KEY_NATION_AUSTRALIA), oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND));
    if exe::utils_random(2) != 0 { // cqv10: the draw of Australia and New Zealand
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(new_zealand));
    } else {
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(new_zealand));
    }
    exe::comp_util_sort_and_shuffle_top_clubs(mel.as_mut_ptr() as usize, nm as i16 as u16 as u32, 2); // cqv10: the Melanesian teams with 2 picks
    exe::comp_util_sort_and_shuffle_top_clubs(pol.as_mut_ptr() as usize, np as i16 as u16 as u32, 2);
    let slot = |k: usize| read_usize(this + 0xee) + k * 4;
    if exe::utils_random(2) != 0 {
        write_usize(slot(1), mel[0]);
        write_usize(slot(4), mel[1]);
    } else {
        write_usize(slot(4), mel[0]);
        write_usize(slot(1), mel[1]);
    }
    if exe::utils_random(2) != 0 {
        write_usize(slot(2), pol[0]);
        write_usize(slot(5), pol[1]);
    } else {
        write_usize(slot(5), pol[0]);
        write_usize(slot(2), pol[1]);
    }
    1
}

// simplification: the list is allocated and the draws count: it cannot run twice. Only the original; the plan: returns
// 1, +0xf2 = 6 and, in the list, Australia and New Zealand in slots 0 and 3 (in either order).
unsafe extern "thiscall" fn compare_oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_SELECT_AND_DRAW_TEAMS);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xee);
        let mut a = [read_usize(list), read_usize(list + 0xc)];
        let mut b = [exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_AUSTRALIA)), exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND))];
        a.sort_unstable();
        b.sort_unstable();
        SCOREBOARD.record(read_i16(this + 0xf2) == 6 && a == b, || format!("oceania_nations_select_and_draw_teams({:#x}): slots 0 and 3 {:x?} expected {:x?}", this, a, b));
    }
    r
}

/// The first group of the Oceania Nations Cup has 3 national teams (`league::CLUB_COUNT`); the table
/// `league::TABLE` (`exe::crt_malloc`(0xb1): 3 rows of `league_table_row::SIZE` bytes; out of memory, the box for line
/// 0x246 and 0), each row i with national team i of the list +0xee (`exe::league_init_table_row`: push 0; push 0; push
/// team; push i; mov ecx -> thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`) and returns 1.
/// The body of the Oceania qualifier's 0x91ec00 with 3 rows and the list in 4-byte steps.
/// Original: `OceaniaNations::create_group_table` (0x7995a0, oceania_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_nations_create_group_table(this: usize) -> u32 {
    let league = League(this);
    league.set_club_count(3);
    let t = exe::crt_malloc(0xb1);
    league.set_table(t);
    if t == 0 {
        show_error_at(0xa6a0ec, 0x246);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(league, i as u8 as u32, read_usize(read_usize(this + 0xee) + i as isize as usize * 4), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

// simplification: the table is allocated and the news goes to the managers: it cannot run twice. Only the original; the
// plan: returns 1, `league::CLUB_COUNT` = 3 and in row i national team i of the list with position i (`league_table_row::POSITION`).
unsafe extern "thiscall" fn compare_oceania_nations_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_CREATE_GROUP_TABLE);
    let list = read_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 3] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this);
    let league = League(this);
    let t = league.table();
    let mut read = [(0usize, 0u8); 3];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && league.club_count() == 3 && t != 0 && (0..3).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("oceania_nations_create_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}

// ---------- 286a leva: as quartas da Mercosul (os cinco primeiros e os três melhores segundos) e as posições previstas da Libertadores e da Mercosul (conmebol_merc.cpp, conmebol_liber.cpp) ----------

/// Group `g` of the Mercosur Cup: -1 is the cup itself (group A), the others the stages `league::STAGES`[g].
unsafe fn conmebol_merc_group(this: usize, g: i8) -> usize {
    if g == -1 {
        this
    } else {
        read_usize(read_usize(this + league::STAGES) + g as isize as usize * 4)
    }
}

/// The Mercosur Cup quarter-finals, at the end of the five groups (SEH frame only for the stage `new`s, no destructor
/// on the normal path). A league stage of the runners-up (`exe::crt_operator_new`(0xee) and the League_Stage
/// constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, 5, no teams, 0, the
/// competition record (`league::COMPETITION`), no table, 0, `league::WIN_POINTS`, `league::DRAW_POINTS`, 0,
/// &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year, -2, 0, 0, 1, 0, 2, -1, 0, 2; words and bytes clean), with
/// row 1 (the runner-up) of each group copied (`league_table_row::SIZE` bytes) into its row and the dword
/// `league_table_row::POSITION_HISTORY` zeroed (the points stay [inference]), and the table sorted
/// (`exe::league_sort_table`(stage, 0)). An array of 8 clubs (`exe::crt_malloc`(0x20); out of memory, the box for line
/// 0x248 and it returns without destroying the runners-up stage): the first `league::PROMOTED_COUNT` of group A
/// (`league::TABLE`) and of each of the groups B..E (stages 0..3, `exe::league_get_table_row`), and the first three of
/// the runners-up table. Each of those three gets one more place in its group (`league::PROMOTED_COUNT` of the group +
/// 1), its row marked (`league_table_row::STATUS` = 6) and the status (the virtual `comp_vtable::ANNOUNCE_TEAM_STATUS`(club,
/// 6, group, 0, 0): 5 pushes; mov ecx -> thiscall). Runners-up 4 and 5 stay out (the virtual
/// `comp_vtable::SET_CLUB_PROGRESS`(club, -1, 3, 3, 3)). Whoever played the groups and is not among the 8 gets the
/// failed-to-qualify record (`exe::staff_records_add_failed_to_qualify_record`(`G_STAFF_RECORDS`, club, competition
/// record, 0x41c, 0x28): 4 pushes; mov ecx -> thiscall, ret 0x10). The runners-up stage is destroyed (the virtual
/// `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1). The quarter-finals: the round table of stage 4 (the virtual
/// `comp_vtable::CREATE_SCHEDULE`), the stage (`exe::crt_operator_new`(0xb2) and the Cup_Stage constructor
/// `FN_CUP_STAGE_CUP_STAGE`, ret 0x3c: the cup, 8, the array, the rounds, the competition record, the table, the year,
/// 4, 2, the extra, 0x14, 0, 0, 0, 0) in `league::STAGES`[4]; the array and the table freed; without a stage, the box
/// for line 0x2ab; with it, the draw of the first round (`exe::cup_draw_round`(stage, 0): push 0; mov ecx -> thiscall,
/// ret 4).
/// Original: `ConmebolMerc::create_quarter_final_stage` (0x4c53d0, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_create_quarter_final_stage(this: usize) {
    #[allow(clippy::type_complexity)]
    let league_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, i32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    #[allow(clippy::type_complexity)]
    let cup_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let b = |d: usize| read_u8(this + d) as u32;
    let p = exe::crt_operator_new(0xee);
    let runners_up = if p != 0 {
        league_stage(p, this, 5, 0, 0, read_usize(this + league::COMPETITION), 0, 0, b(league::WIN_POINTS), b(league::DRAW_POINTS), 0, this + league::TIEBREAKS, this + league::PROMOTED_COUNT, read_i16(this + league::YEAR) as u16 as u32, -2, 0, 0, 1, 0, 2, -1, 0, 2)
    } else {
        0
    };
    for g in 0..5usize {
        let source = read_usize(conmebol_merc_group(this, g as i8 - 1) + league::TABLE) + league_table_row::SIZE; // cqv11: the row of the group runner-up
        let target = read_usize(runners_up + league::TABLE) + g * league_table_row::SIZE;
        core::ptr::copy(source as *const u8, target as *mut u8, league_table_row::SIZE);
        write_u32(read_usize(runners_up + league::TABLE) + g * league_table_row::SIZE + league_table_row::POSITION_HISTORY, 0);
    }
    exe::league_sort_table(runners_up, 0);
    let v = exe::crt_malloc(0x20);
    if v == 0 {
        return show_error_at(0x9a7e5c, 0x248);
    }
    let mut k = 0usize;
    for i in 0..read_i8(this + league::PROMOTED_COUNT).max(0) as usize {
        write_usize(v + k * 4, read_usize(read_usize(this + league::TABLE) + i * league_table_row::SIZE));
        k += 1;
    }
    for g in 0..4usize {
        let stage = read_usize(read_usize(this + league::STAGES) + g * 4);
        for i in 0..read_i8(this + league::PROMOTED_COUNT).max(0) as i32 {
            write_usize(v + k * 4, read_usize(exe::league_get_table_row(stage, i)));
            k += 1;
        }
    }
    for s in 0..3i32 {
        covered!("cqv_11 best runners-up");
        write_usize(v + (k + s as usize) * 4, read_usize(exe::league_get_table_row(runners_up, s))); // cqv11: runner-up s into the quarter-finals
        for g in -1i8..4 {
            let group = conmebol_merc_group(this, g);
            let club = read_usize(exe::league_get_table_row(runners_up, s));
            if read_usize(exe::league_get_table_row(group, 1)) == club {
                let places = group + league::PROMOTED_COUNT;
                write_u8(places, read_u8(places).wrapping_add(1));
                write_u8(exe::league_get_table_row(group, 1) + league_table_row::STATUS, 6);
                let status: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::ANNOUNCE_TEAM_STATUS));
                // simplification: the original pushes the dword whose low byte is the group (the high ones are those of the count of 8, < 256: 0)
                status(this, read_usize(exe::league_get_table_row(runners_up, s)), 6, g as u8 as u32, 0, 0);
                break;
            }
        }
    }
    for s in 3..5i32 {
        let out: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
        out(this, read_usize(exe::league_get_table_row(runners_up, s)), -1, 3, 3, 3);
    }
    for g in -1i8..4 {
        let group = conmebol_merc_group(this, g);
        for r in 0..read_i16(group + league::CLUB_COUNT).max(0) as usize {
            let club = read_usize(read_usize(group + league::TABLE) + r * league_table_row::SIZE);
            if !(0..8).any(|j| read_usize(v + j * 4) == club) {
                exe::staff_records_add_failed_to_qualify_record(read_usize(G_STAFF_RECORDS), club, read_usize(this + league::COMPETITION), 0x41c, 0x28);
            }
        }
    }
    if runners_up != 0 {
        let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(runners_up) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
        destroy(runners_up, 1);
    }
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this, 4, &mut rounds, &mut extra, 0);
    let eight = v; // cqv11: the 8 quarter-finalists in order
    let p = exe::crt_operator_new(0xb2);
    let quarter_finals = if p != 0 {
        cup_stage(p, this, 8, eight, rounds as u16 as u32, read_usize(this + league::COMPETITION), table, read_i16(this + league::YEAR) as u16 as u32, 4, 2, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(read_usize(this + league::STAGES) + 0x10, quarter_finals);
    exe::crt_free(v);
    exe::crt_free(table);
    let f = read_usize(read_usize(this + league::STAGES) + 0x10);
    if f == 0 {
        return show_error_at(0x9a7e5c, 0x2ab);
    }
    exe::cup_draw_round(f, 0);
}

// simplification: the stages are allocated, the records and the round draw cannot run twice. Only the original; the
// plan: the 8 quarter-finalists are the first `league::PROMOTED_COUNT` of each group and three runners-up, and the three
// runners-up that got in are the best placed among the five runners-up from before (the highest scoring: [inference]
// the table order is that of 0x6827d0, so it only checks that each one is a group runner-up and that no club repeats);
// the stage in `league::STAGES`[4] has the 8 in its team table (`cup::TEAM_TABLE`, 6-byte rows).
unsafe extern "thiscall" fn compare_conmebol_merc_create_quarter_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_CREATE_QUARTER_FINAL_STAGE);
    let q = read_i8(this + league::PROMOTED_COUNT);
    let stages = read_usize(this + league::STAGES);
    if !(1..=2).contains(&q) || stages == 0 || (0..4).any(|g| read_usize(stages + g * 4) == 0) {
        return o(this);
    }
    let mut first: Vec<usize> = Vec::new();
    let mut second: Vec<usize> = Vec::new();
    for g in -1i8..4 {
        let group = conmebol_merc_group(this, g);
        for i in 0..q as i32 {
            first.push(read_usize(exe::league_get_table_row(group, i)));
        }
        second.push(read_usize(exe::league_get_table_row(group, 1)));
    }
    o(this);
    let f = read_usize(read_usize(this + league::STAGES) + 0x10);
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..8).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    let got_in: Vec<usize> = read.iter().copied().filter(|c| !first.contains(c)).collect();
    let ok = read.len() == 8
        && first.iter().all(|c| read.contains(c))
        && got_in.len() + first.len() == 8
        && got_in.iter().all(|c| second.contains(c))
        && {
            read.sort_unstable();
            read.windows(2).all(|w| w[0] != w[1])
        };
    SCOREBOARD.record(ok, || format!("conmebol_merc_create_quarter_final_stage({:#x}): quarter-finals {:x?} group winners {:x?} runners-up {:x?}", this, read, first, second));
}

/// The Libertadores predicted positions: by the rounds of stage 7 (the knockout) and in blocks of 8 for those who stay
/// in the groups (box at line 0x30e); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolLiber::set_predicted_positions` (0x4c4210, conmebol_liber.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_liber_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 7, 8, 0x9a7de4, 0x30e)
}

unsafe extern "thiscall" fn compare_conmebol_liber_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_SET_PREDICTED_POSITIONS);
    conmebol_compare_predicted_positions("conmebol_liber_set_predicted_positions", this, 7, 8, || o(this))
}

/// The Mercosur Cup predicted positions: by the rounds of stage 4 (the quarter-finals on) and in blocks of 5 for those
/// who stay in the groups (box at line 0x338); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolMerc::set_predicted_positions` (0x4c5a90, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 4, 5, 0x9a7e5c, 0x338)
}

unsafe extern "thiscall" fn compare_conmebol_merc_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_SET_PREDICTED_POSITIONS);
    conmebol_compare_predicted_positions("conmebol_merc_set_predicted_positions", this, 4, 5, || o(this))
}

/// The predicted positions the CONMEBOL cups give (0x4c2a90, 0x4c4210 and 0x4c5a90, the same body), in order: the list
/// of invited clubs (+0xee, +0xf2 clubs) already sorted; the rounds of the table `table` (`cup_round::SIZE`-byte
/// records; the word `cup_round::TIES` of each, those who go out in it), from the last to the first: the last gives
/// position 1 to those who go out in it and, like all of them, the current position (which goes up with each club) to
/// as many more; those left over go in blocks of `block` with the position of the start of the block. Returns (club,
/// position).
unsafe fn conmebol_predicted_positions_plan(this: usize, table: usize, rounds: i16, block: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let list = read_usize(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut current: u8 = 1;
    let mut r = rounds as i32 - 1;
    while r >= 0 {
        let p = table + r as usize * cup_round::SIZE + cup_round::TIES;
        if r == rounds as i32 - 1 {
            let mut j: i16 = 0;
            while j < read_i16(p) {
                v.push((read_usize(list + i as isize as usize * 4), current));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        current = bl;
        let mut j: i16 = 0;
        while j < read_i16(p) {
            v.push((read_usize(list + i as isize as usize * 4), current));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < read_usize(this + 0xf2) as i32 {
        v.push((read_usize(list + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + block {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The body of the predicted positions: without statistics (`league::COMP_STATS`), the box `row` of `file`; the list
/// +0xee of +0xf2 invited clubs sorted by reputation (`exe::crt_qsort`(list, n, 4, 0x4b07d0): 4 pushes; add esp ->
/// cdecl); the round table of stage `stage` by the virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push
/// &rounds; push stage; mov ecx, this -> thiscall, ret 0x10); each club of the plan (`conmebol_predicted_positions_plan`)
/// with its position (`exe::comp_stats_set_club_final_position`: push position; push club; mov ecx, stats -> thiscall,
/// ret 8; the original pushes the byte with leftovers in the high bytes, 0x4a2540 reads the byte); the table freed
/// (`exe::crt_free`).
unsafe fn conmebol_set_predicted_positions(this: usize, stage: u32, block: u32, file: usize, row: u32) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(file, row);
    }
    exe::crt_qsort(read_usize(this + 0xee), read_usize(this + 0xf2) as u32, 4, 0x4b07d0);
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this, stage, &mut rounds, &mut extra, 0);
    for (c, p) in conmebol_predicted_positions_plan(this, table, rounds, block) {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), c, p as u32);
    }
    exe::crt_free(table);
}

// simplification: Comp_Stats::set_club_final_position adds the club's entry (reallocates): it cannot run twice. Only the
// original; the plan, with the list already sorted by it: each invited club with an entry and the positions
// (`comp_stats_club::FINAL_POSITION`, `exe::comp_stats_find_club_entry`) increasing in the list order, the first 1.
// (Asking again for the stage's round table changes the cup's calendar: the 07/11 comparison gave the Cup.cpp line 1278
// box; the plan calls nothing but the search.)
unsafe fn conmebol_compare_predicted_positions(name: &str, this: usize, _stage: u32, _block: u32, o: impl FnOnce()) {
    o();
    let stats = read_usize(this + league::COMP_STATS);
    let list = read_usize(this + 0xee);
    let n = read_usize(this + 0xf2) as i32;
    if stats == 0 || list == 0 || !(1..=256).contains(&n) {
        return;
    }
    let mut read: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = read_usize(list + k * 4);
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        read.push(if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) as i32 } else { -1 });
    }
    let ok = read[0] == 1 && read.iter().all(|&x| x >= 1) && read.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("{}({:#x}): positions {:?}", name, this, read));
}

// ---------- 286a leva: a virada da temporada das copas da CONMEBOL: a Libertadores, a Mercosul e a Merconorte (conmebol_liber.cpp, conmebol_merc.cpp, con_merc_cup.cpp) ----------

/// The first half of the three cups' season turnover (the same body): `league::SEASON_CLOSED` = 0 and the table freed
/// (`exe::league_free_table`(this, 0): push 0; mov ecx -> thiscall, ret 4); the calendar `league::ROUND_DATES` freed
/// (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1) (push 1; mov ecx -> thiscall, ret 4); the invited clubs list
/// +0xee freed (not zeroed); with the current stage (`league::CURRENT_STAGE`) >= 0, the existing stages 0..=current
/// destroyed (the virtual `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1: push 1; mov ecx -> thiscall).
unsafe fn conmebol_free_season(this: League) {
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + league::ROUND_DATES, 0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let c = read_usize(this.addr() + 0xee);
    if c != 0 {
        exe::crt_free(c);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
}

/// The year (`league::YEAR`, word) goes up by one and the current stage (`league::CURRENT_STAGE`) goes back to -1.
unsafe fn conmebol_next_year(this: League) {
    this.set_year(this.year().wrapping_add(1));
    write_u32(this.addr() + league::CURRENT_STAGE, u32::MAX);
}

/// The rules again (the virtual `comp_vtable::SET_UP_RULES`, thiscall, ret 0); 0: the box for line `row` of `file`
/// and 0 (the original returns the box's eax; the caller does not read it). Returns what the rules returned.
unsafe fn conmebol_set_up_rules_again(this: League, file: usize, row: u32) -> u32 {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    let r = rules(this.addr());
    if r == 0 {
        show_error_at(file, row);
    }
    r
}

/// The Libertadores season turnover: the first half (`conmebol_free_season`), the next year (`conmebol_next_year`),
/// the CONMEBOL seeding for the new season (ConmebolSeeding::update_for_new_season 0x4c61b0(`G_CONMEBOL_SEEDING`, 0):
/// push 0; mov ecx -> thiscall, ret 4: the nations' places), `league::ACTIVE` = 0 (the draw still to be made) and the
/// rules (`conmebol_set_up_rules_again`, the box for line 0x1e3).
/// Original: `ConmebolLiber::start_new_season` (0x4c3980, conmebol_liber.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_liber_start_new_season(this: usize) -> u32 {
    let league = League(this);
    let seed: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4c61b0usize);
    conmebol_free_season(league);
    covered!("cqv_12 libertadores turnover");
    conmebol_next_year(league); // cqv12: the Libertadores of the next year
    seed(read_usize(G_CONMEBOL_SEEDING), 0); // cqv12: the places of the new Libertadores
    write_u32(this + league::ACTIVE, 0); // cqv12: the new Libertadores not drawn yet
    conmebol_set_up_rules_again(league, 0x9a7de4, 0x1e3)
}

// simplification: the turnover frees tables and stages and archives the statistics: it cannot run twice. Only the
// original; the plan: `league::SEASON_CLOSED` = 0, the year + 1 and (Libertadores and Mercosur) `league::ACTIVE` = 0
// (the new rules rebuild the calendar `league::ROUND_DATES`).
unsafe fn conmebol_compare_new_season(name: &str, this: usize, clears_active: bool, o: impl FnOnce() -> u32) -> u32 {
    let league = League(this);
    let year = league.year();
    let r = o();
    let new = league.year();
    let ok = read_usize(this + league::SEASON_CLOSED) == 0 && new == year.wrapping_add(1) && (!clears_active || league.active() == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x4c {}, +0xba {:#x}, +0x45 {}", name, this, year, new, read_usize(this + league::SEASON_CLOSED), league.round_dates(), league.active()));
    r
}

unsafe extern "thiscall" fn compare_conmebol_liber_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_START_NEW_SEASON);
    conmebol_compare_new_season("conmebol_liber_start_new_season", this, true, || o(this))
}

/// The Mercosur Cup season turnover: the first half (`conmebol_free_season`), the next year, `league::ACTIVE` = 0 and
/// the rules (the box for line 0x1c6).
/// Original: `ConmebolMerc::start_new_season` (0x4c5040, conmebol_merc.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn conmebol_merc_start_new_season(this: usize) -> u32 {
    let league = League(this);
    conmebol_free_season(league);
    conmebol_next_year(league);
    write_u32(this + league::ACTIVE, 0);
    conmebol_set_up_rules_again(league, 0x9a7e5c, 0x1c6)
}

unsafe extern "thiscall" fn compare_conmebol_merc_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_START_NEW_SEASON);
    conmebol_compare_new_season("conmebol_merc_start_new_season", this, true, || o(this))
}

/// The Merconorte season turnover: the first half (`conmebol_free_season`), the next year and the rules; only with them
/// `league::ACTIVE` = 0 (the box for line 0x1b1 leaves it as it was).
/// Original: `ConMercCup::start_new_season` (0x4c1fb0, con_merc_cup.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn con_merc_cup_start_new_season(this: usize) -> u32 {
    let league = League(this);
    conmebol_free_season(league);
    conmebol_next_year(league);
    let r = conmebol_set_up_rules_again(league, 0x9a7d6c, 0x1b1);
    if r != 0 {
        write_u32(this + league::ACTIVE, 0);
    }
    r
}

unsafe extern "thiscall" fn compare_con_merc_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_MERC_CUP_START_NEW_SEASON);
    conmebol_compare_new_season("con_merc_cup_start_new_season", this, false, || o(this))
}

// ---------- 286a leva: as posições previstas e o sorteio dos grupos do Campeonato de Clubes da Oceania; as posições previstas da Copa Africana de Nações e da eliminatória da Oceania (oceania_club_champ.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

/// The positions in the order of the array sorted by reputation (`exe::comp_stats_set_club_final_position`: push
/// position; push club; mov ecx, stats (`league::COMP_STATS`) -> thiscall, ret 8).
unsafe fn predicted_by_reputation_write(this: usize, v: usize, positions: &[u32]) {
    for (k, &p) in positions.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), read_usize(v + k * 4), p);
    }
}

/// `exe::crt_qsort`(v, n, 4, 0x4b07d0) (4 pushes; add esp -> cdecl): the reputation (`club::REPUTATION`) descending,
/// nulls at the end.
unsafe fn predicted_by_reputation_sort(v: usize, n: u32) {
    exe::crt_qsort(v, n, 4, 0x4b07d0);
}

/// The predicted positions of the 8 Oceania Club Championship entrants, by reputation.
const OCEANIA_CLUB_CHAMP_POSITIONS: [u32; 8] = [1, 2, 3, 3, 5, 5, 7, 7];

/// The predicted positions of the Oceania Club Championship and its group draw: without statistics
/// (`league::COMP_STATS`), the box for line 0x3a0; otherwise the 8 chosen clubs (+0xee, +0xf2 clubs) sorted by
/// reputation (`predicted_by_reputation_sort`), the positions 1, 2, 3, 3, 5, 5, 7, 7 and the list shuffled
/// (`exe::comp_util_shuffle_pointer_array`(list, n): push n (word); push list; add esp, 8 -> cdecl: 3n drawn swaps).
/// That shuffle is what makes the groups: the first 4 go to group A, the others to B.
/// Original: `OceaniaClubChamp::set_predicted_positions` (0x798640, oceania_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_club_champ_set_predicted_positions(this: usize) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(0xa6a068, 0x3a0);
    }
    let list = read_usize(this + 0xee);
    predicted_by_reputation_sort(list, read_usize(this + 0xf2) as u32); // cqv13: the 8 by reputation
    predicted_by_reputation_write(this, read_usize(this + 0xee), &OCEANIA_CLUB_CHAMP_POSITIONS);
    covered!("cqv_13 oceania group draw");
    let n = read_i16(this + 0xf2) as u16 as u32;
    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n); // cqv13: the 8 shuffled
}

/// The plan of the predicted positions: the clubs `clubs` (taken before the original), each with the position written
/// (`comp_stats_club::FINAL_POSITION` of its entry, `exe::comp_stats_find_club_entry`) after it; by reputation
/// (`club::REPUTATION`) descending the positions make `positions` (reputation ties may swap with each other: the qsort
/// is not stable). Only with as many clubs as positions and none null.
unsafe fn predicted_by_reputation_check(name: &str, this: usize, clubs: &[usize], positions: &[u32]) {
    let stats = read_usize(this + league::COMP_STATS);
    if stats == 0 || clubs.len() != positions.len() || clubs.iter().any(|&c| c == 0) {
        return;
    }
    let mut v: Vec<(i16, u8)> = clubs
        .iter()
        .map(|&c| {
            let e = exe::comp_stats_find_club_entry(stats, c);
            (Club(c).reputation(), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
        })
        .collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let read: Vec<u32> = v.iter().map(|x| x.1 as u32).collect();
    SCOREBOARD.record(read == positions, || format!("{}({:#x}): positions by reputation {:?}", name, this, read));
}

// simplification: Comp_Stats::set_club_final_position adds entries and the shuffle draws: it cannot run twice. Only the
// original; the plan: the positions by reputation and the list after it a permutation of the one before.
unsafe extern "thiscall" fn compare_oceania_club_champ_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_SET_PREDICTED_POSITIONS);
    let list = read_usize(this + 0xee);
    let n = read_i16(this + 0xf2);
    if list == 0 || !(0..=64).contains(&n) {
        return o(this);
    }
    let mut before: Vec<usize> = (0..n as usize).map(|k| read_usize(list + k * 4)).collect();
    o(this);
    let mut after: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this + 0xee) + k * 4)).collect();
    predicted_by_reputation_check("oceania_club_champ_set_predicted_positions", this, &before, &OCEANIA_CLUB_CHAMP_POSITIONS);
    before.sort_unstable();
    after.sort_unstable();
    SCOREBOARD.record(before == after, || format!("oceania_club_champ_set_predicted_positions({:#x}): the shuffled list is not the same", this));
}

/// The body of the predicted positions with a copy (0x4027d0 and 0x91ff30): without statistics (`league::COMP_STATS`),
/// the box `rows.0`; a copy of the `n` pointers of the list (`exe::crt_malloc`(4n): push; add esp, 4 -> cdecl; out of
/// memory, the box `rows.1`), sorted by reputation (`predicted_by_reputation_sort`); the positions in that order; the
/// copy freed (`exe::crt_free`).
unsafe fn predicted_by_reputation_of_copy(this: usize, list: usize, n: i32, positions: &[u32], file: usize, rows: (u32, u32)) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(file, rows.0);
    }
    let v = exe::crt_malloc((n as u32).wrapping_mul(4));
    if v == 0 {
        return show_error_at(file, rows.1);
    }
    core::ptr::copy_nonoverlapping(list as *const u8, v as *mut u8, (n as u32).wrapping_mul(4) as usize);
    predicted_by_reputation_sort(v, n as u32);
    predicted_by_reputation_write(this, v, positions);
    exe::crt_free(v);
}

/// The predicted positions of the 16 African Nations Cup finalists, by reputation.
const AFRICAN_NATIONS_POSITIONS: [u32; 16] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13];

/// The predicted positions of the African Nations Cup: the 16 finalists (+0xee, +0xf2 word) by reputation
/// (`predicted_by_reputation_of_copy`): 1, 2, 3, 4, four 5s, four 9s and four 13s (boxes at lines 0x3f4 and 0x3fc).
/// Original: `AfricanNations::set_predicted_positions` (0x4027d0, african_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn african_nations_set_predicted_positions(this: usize) {
    predicted_by_reputation_of_copy(this, read_usize(this + 0xee), read_i16(this + 0xf2) as i32, &AFRICAN_NATIONS_POSITIONS, 0x9870f8, (0x3f4, 0x3fc))
}

// simplification: Comp_Stats::set_club_final_position adds entries: it cannot run twice. Only the original; the plan:
// the positions by reputation.
unsafe extern "thiscall" fn compare_african_nations_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_SET_PREDICTED_POSITIONS);
    let (list, n) = (read_usize(this + 0xee), read_i16(this + 0xf2));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    predicted_by_reputation_check("african_nations_set_predicted_positions", this, &clubs, &AFRICAN_NATIONS_POSITIONS);
}

/// The predicted positions of the Oceania World Cup qualifier, by strength (9 or 10 nations).
const WC_OCEANIA_LEAGUE_POSITIONS: [u32; 10] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 10];

/// The predicted positions of the Oceania World Cup qualifier: its national teams (+0xef, +0xf3 dword) by strength (the
/// reputation `club::REPUTATION` of the national teams' clubs, `predicted_by_reputation_of_copy`): 1, 2, 3, 3, 5, 5, 7,
/// 7, 9 and, if there are 10, the tenth 10 (boxes at lines 0x52a and 0x532).
/// Original: `WcOceaniaLeague::set_predicted_positions` (0x91ff30, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_set_predicted_positions(this: usize) {
    let n = read_usize(this + 0xf3) as i32;
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_of_copy(this, read_usize(this + 0xef), n, &WC_OCEANIA_LEAGUE_POSITIONS[..k], 0xad4afc, (0x52a, 0x532))
}

unsafe extern "thiscall" fn compare_wc_oceania_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_SET_PREDICTED_POSITIONS);
    let (list, n) = (read_usize(this + 0xef), read_usize(this + 0xf3) as i32);
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_check("wc_oceania_league_set_predicted_positions", this, &clubs, &WC_OCEANIA_LEAGUE_POSITIONS[..k]);
}

// ---------- 286a leva: os calendários Baiano, Centro-Oeste, Gaúcho, Goiano (as datas e os jogos de cada rodada da fase de pontos corridos e do mata-mata) (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_goias.cpp) ----------
/// The Bahia state league, the round-robin stage: 18 rounds (from 11 January to 20 April), 1170 bytes, 64 operations.
const BAHIA_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The Bahia state league, stage 0: 2 rounds (from 21 April to 10 May), 208 bytes, 36 operations.
const BAHIA_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x249f0), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the Bahia round table: the round-robin stage (-1) (a league): 18 rounds, from 11 January to 20 April; stage 0
/// (knockout or final stage): 2 rounds, from 21 April to 10 May.
fn bra_reg_bahia_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &BAHIA_LEAGUE_STAGE_OPS, name: "cqw_1 bahia league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &BAHIA_STAGE_0_OPS, name: "cqw_1 bahia stage 0" }),
        _ => None,
    }
}

/// The Bahia state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_bahia_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegBahia::create_schedule` (0x42f5d0, bra_reg_bahia.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_bahia_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_bahia_schedule_stage(stage), rounds, extra, league, 0x98acac)
}

unsafe extern "thiscall" fn compare_bra_reg_bahia_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_BAHIA_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_bahia_create_schedule", stage, bra_reg_bahia_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_bahia_create_schedule(this, stage, r, e, l))
}

/// The Centre-West state league, the round-robin stage: 15 rounds (from 11 January to 6 April), 975 bytes, 56 operations.
const CENTRAL_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x19, 1, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 4, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0xb, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x12, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x19, 2, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 1, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 4, 3, 0, 2, 2, 0), Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 8, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0),
    Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0),
    Cqp11D(0xb, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0),
    Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 0x19, 3, 0, 2, 2, 0), Cqp11L(0xc, 0, -1, -1, -1, 0),
    Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0),
    Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0), Cqp11L(0xe, 0, 5, 1, 1, 0),
    Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
];

/// The Centre-West state league, stage 0: 3 rounds (from 7 April to 10 May), 312 bytes, 54 operations.
const CENTRAL_STAGE_0_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 7, 4, 0, 0), Cqp11J(0, 0xd, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x78), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x15, 4, 0, 0), Cqp11J(1, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x6f, 0x82), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 8), Cqp11W(0x80, 4), Cqp11W(0x82, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 4, 5, 0, 0), Cqp11J(2, 0xa, 5, 0, 6, 1, 0), Cqp11W(0xd9, 2),
    Cqp11W(0xe8, 2), Cqp11W(0xd7, 0x96), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11B(0xe7, 8), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xdf, 0x804), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0x3d090), Cqp11X(0x134, 0x186a0),
];

/// The stages of the Centre-West round table: the round-robin stage (-1) (a league): 15 rounds, from 11 January to 6 April; stage 0
/// (knockout or final stage): 3 rounds, from 7 April to 10 May.
fn bra_reg_central_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa4,
            ops: &CENTRAL_LEAGUE_STAGE_OPS, name: "cqw_1 centre-west league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x114,
            ops: &CENTRAL_STAGE_0_OPS, name: "cqw_1 centre-west stage 0" }),
        _ => None,
    }
}

/// The Centre-West state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_central_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegCentral::create_schedule` (0x4307b0, bra_reg_central.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_central_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_central_schedule_stage(stage), rounds, extra, league, 0x98ae10)
}

unsafe extern "thiscall" fn compare_bra_reg_central_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_CENTRAL_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_central_create_schedule", stage, bra_reg_central_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_central_create_schedule(this, stage, r, e, l))
}

/// The Rio Grande do Sul (Gaúcho) state league, the round-robin stage: 17 rounds (from 11 January to 20 April), 1105 bytes, 62 operations.
const GAUCHO_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 62] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0x12, 2, 0, 6, 1, 0),
    Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0), Cqp11D(7, 0x19, 2, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 1, 3, 0, 6, 1, 0),
    Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 8, 3, 0, 6, 1, 0),
    Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0),
    Cqp11D(0xb, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0),
    Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0),
    Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0),
    Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0), Cqp11L(0xe, 0, 5, 1, 1, 0),
    Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0), Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
];

/// The Rio Grande do Sul (Gaúcho) state league, stage 0: 2 rounds (from 21 April to 10 May), 208 bytes, 36 operations.
const GAUCHO_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x3d090), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the Rio Grande do Sul (Gaúcho) round table: the round-robin stage (-1) (a league): 17 rounds, from 11 January to 20 April; stage 0
/// (knockout or final stage): 2 rounds, from 21 April to 10 May.
fn bra_reg_gaucho_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 17, extra: 0, extra_check: false, size: 0x451, row: 0xa8,
            ops: &GAUCHO_LEAGUE_STAGE_OPS, name: "cqw_1 gaucho league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x122,
            ops: &GAUCHO_STAGE_0_OPS, name: "cqw_1 gaucho stage 0" }),
        _ => None,
    }
}

/// The Rio Grande do Sul (Gaúcho) state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_gaucho_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegGaucho::create_schedule` (0x431ac0, bra_reg_gaucho.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, 0x98ae50)
}

unsafe extern "thiscall" fn compare_bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_GAUCHO_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_gaucho_create_schedule", stage, bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_gaucho_create_schedule(this, stage, r, e, l))
}

/// The Goiás state league, the round-robin stage: 13 rounds (from 11 January to 6 April), 845 bytes, 52 operations.
const GOIAS_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 52] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x19, 1, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 4, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0xb, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x12, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x19, 2, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 1, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 8, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0xf, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0),
    Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0),
    Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0),
];

/// The Goiás state league, stage 0: 3 rounds (from 7 April to 10 May), 312 bytes, 54 operations.
const GOIAS_STAGE_0_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 7, 4, 0, 0), Cqp11J(0, 0xd, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x78), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x15, 4, 0, 0), Cqp11J(1, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x6f, 0x82), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 8), Cqp11W(0x80, 4), Cqp11W(0x82, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 4, 5, 0, 0), Cqp11J(2, 0xa, 5, 0, 6, 1, 0), Cqp11W(0xd9, 2),
    Cqp11W(0xe8, 2), Cqp11W(0xd7, 0x96), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11B(0xe7, 8), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xdf, 0x804), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0x3d090), Cqp11X(0x134, 0x186a0),
];

/// The stages of the Goiás round table: the round-robin stage (-1) (a league): 13 rounds, from 11 January to 6 April; stage 0
/// (knockout or final stage): 3 rounds, from 7 April to 10 May.
fn bra_reg_goias_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 13, extra: 0, extra_check: false, size: 0x34d, row: 0xa4,
            ops: &GOIAS_LEAGUE_STAGE_OPS, name: "cqw_1 goias league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x10c,
            ops: &GOIAS_STAGE_0_OPS, name: "cqw_1 goias stage 0" }),
        _ => None,
    }
}

/// The Goiás state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_goias_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegGoias::create_schedule` (0x432e50, bra_reg_goias.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_goias_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_goias_schedule_stage(stage), rounds, extra, league, 0x98ae90)
}

unsafe extern "thiscall" fn compare_bra_reg_goias_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_GOIAS_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_goias_create_schedule", stage, bra_reg_goias_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_goias_create_schedule(this, stage, r, e, l))
}

// ---------- batch 286: the Minas Gerais, North, Northeast and Paraná schedules (the dates and matches of each round of the round-robin and knockout stages) (bra_reg_minas_gerais.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp, bra_reg_parana.cpp) ----------

/// The Minas Gerais state league, the round-robin stage (−1): 22 rounds (11 Jan to 3 May), 1430 bytes, 78 operations.
const MINAS_GERAIS_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 4, 3, 0, 2, 2, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 8, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0xf, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 3, 0, 2, 2, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0),
    Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 6, 4, 0, 6, 1, 0),
    Cqp11L(0x10, 0, 5, 1, 1, 0), Cqp11L(0x10, 1, 5, 1, 1, 0), Cqp11L(0x10, 2, -1, -1, -1, 0),
    Cqp11D(0x11, 0xd, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0),
    Cqp11L(0x11, 2, -1, -1, -1, 0), Cqp11D(0x12, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0),
    Cqp11D(0x13, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0), Cqp11L(0x13, 1, 5, 1, 1, 0),
    Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 0x1b, 4, 0, 6, 1, 0), Cqp11L(0x14, 0, 5, 1, 1, 0),
    Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0), Cqp11D(0x15, 3, 5, 0, 6, 1, 0),
    Cqp11L(0x15, 0, -1, -1, -1, 0),
];

/// The Minas Gerais state league, stage 2: 1 round (4 May to 10 May), 104 bytes, 18 operations.
const MINAS_GERAIS_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 4, 5, 0, 0), Cqp11J(0, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11B(0x21, 2),
    Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x1a, 1),
    Cqp11W(0xf, 0x804), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0x55730),
    Cqp11X(0x64, 0x186a0),
];

/// The stages of the round table of the Minas Gerais state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage has no table (None).
fn minas_gerais_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xa6,
            ops: &MINAS_GERAIS_SCHEDULE_LEAGUE_OPS, name: "cqw_2 minas gerais league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13a,
            ops: &MINAS_GERAIS_SCHEDULE_STAGE_2_OPS, name: "cqw_2 minas gerais stage 2" }),
        _ => None,
    }
}

/// The schedule of the Minas Gerais state league: stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`minas_gerais_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegMinasGerais::create_schedule` (0x434050, bra_reg_minas_gerais.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_minas_gerais_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), minas_gerais_schedule_stage(stage), rounds, extra, league, 0x98aed0)
}

/// Comparer for `bra_reg_minas_gerais_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_minas_gerais_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_MINAS_GERAIS_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_minas_gerais_create_schedule", stage, minas_gerais_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_minas_gerais_create_schedule(this, stage, r, e, l))
}

/// The North regional league, the round-robin stage (−1): 18 rounds (11 Jan to 20 Apr), 1170 bytes, 64 operations.
const NORTH_REGION_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The North regional league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
const NORTH_REGION_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x55730), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the round table of the North regional league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
fn north_region_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &NORTH_REGION_SCHEDULE_LEAGUE_OPS, name: "cqw_2 north league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &NORTH_REGION_SCHEDULE_STAGE_0_OPS, name: "cqw_2 north stage 0" }),
        _ => None,
    }
}

/// The schedule of the North regional league: stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`north_region_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegNorth::create_schedule` (0x435640, bra_reg_north.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_north_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), north_region_schedule_stage(stage), rounds, extra, league, 0x98af14)
}

/// Comparer for `bra_reg_north_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_north_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_NORTH_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_north_create_schedule", stage, north_region_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_north_create_schedule(this, stage, r, e, l))
}

/// The Northeast regional league, the round-robin stage (−1): 15 rounds (18 Jan to 20 Apr), 975 bytes, 56 operations.
const NORTHEAST_REGION_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x19, 1, 0, 6, 1, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 4, 2, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 0xb, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0x12, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x19, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 1, 3, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 8, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 0xf, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x14, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0),
];

/// The Northeast regional league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
const NORTHEAST_REGION_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x249f0), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the round table of the Northeast regional league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
fn northeast_region_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa8,
            ops: &NORTHEAST_REGION_SCHEDULE_LEAGUE_OPS, name: "cqw_2 northeast league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x118,
            ops: &NORTHEAST_REGION_SCHEDULE_STAGE_0_OPS, name: "cqw_2 northeast stage 0" }),
        _ => None,
    }
}

/// The schedule of the Northeast regional league: stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`northeast_region_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegNortheast::create_schedule` (0x4366f0, bra_reg_northeast.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_northeast_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), northeast_region_schedule_stage(stage), rounds, extra, league, 0x98af54)
}

/// Comparer for `bra_reg_northeast_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_northeast_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_NORTHEAST_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_northeast_create_schedule", stage, northeast_region_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_northeast_create_schedule(this, stage, r, e, l))
}

/// The Paraná state league, the round-robin stage (−1): 22 rounds (11 Jan to 3 May), 1430 bytes, 78 operations.
const PARANA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 4, 3, 0, 2, 2, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 8, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0xf, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 3, 0, 2, 2, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0),
    Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 6, 4, 0, 6, 1, 0),
    Cqp11L(0x10, 0, 5, 1, 1, 0), Cqp11L(0x10, 1, 5, 1, 1, 0), Cqp11L(0x10, 2, -1, -1, -1, 0),
    Cqp11D(0x11, 0xd, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0),
    Cqp11L(0x11, 2, -1, -1, -1, 0), Cqp11D(0x12, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0),
    Cqp11D(0x13, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0), Cqp11L(0x13, 1, 5, 1, 1, 0),
    Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 0x1b, 4, 0, 6, 1, 0), Cqp11L(0x14, 0, 5, 1, 1, 0),
    Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0), Cqp11D(0x15, 3, 5, 0, 6, 1, 0),
    Cqp11L(0x15, 0, -1, -1, -1, 0),
];

/// The Paraná state league, stage 2: 1 round (4 May to 10 May), 104 bytes, 18 operations.
const PARANA_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 4, 5, 0, 0), Cqp11J(0, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11B(0x21, 2),
    Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x1a, 1),
    Cqp11W(0xf, 0x804), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0x2ab98),
    Cqp11X(0x64, 0x186a0),
];

/// The stages of the round table of the Paraná state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage has no table (None).
fn parana_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xa6,
            ops: &PARANA_SCHEDULE_LEAGUE_OPS, name: "cqw_2 parana league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13a,
            ops: &PARANA_SCHEDULE_STAGE_2_OPS, name: "cqw_2 parana stage 2" }),
        _ => None,
    }
}

/// The schedule of the Paraná state league: stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`parana_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegParana::create_schedule` (0x437700, bra_reg_parana.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_parana_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), parana_schedule_stage(stage), rounds, extra, league, 0x98af98)
}

/// Comparer for `bra_reg_parana_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_parana_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_PARANA_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_parana_create_schedule", stage, parana_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_parana_create_schedule(this, stage, r, e, l))
}

// ---------- batch 286: the Pernambuco, Rio (Carioca), Santa Catarina and São Paulo (Paulista) schedules (the dates and matches of each round of the round-robin and knockout stages) (bra_reg_pern.cpp, bra_reg_rio.cpp, bra_reg_santa.cpp, bra_reg_sp.cpp) ----------

/// The Pernambuco state league, the round-robin stage (−1): 18 rounds (11 Jan to 20 Apr), 1170 bytes, 64 operations.
const PERNAMBUCO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The Pernambuco state league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
const PERNAMBUCO_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x2ab98), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the round table of the Pernambuco state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
fn pernambuco_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &PERNAMBUCO_SCHEDULE_LEAGUE_OPS, name: "cqw_3 pernambuco league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &PERNAMBUCO_SCHEDULE_STAGE_0_OPS, name: "cqw_3 pernambuco stage 0" }),
        _ => None,
    }
}

/// The schedule of the Pernambuco state league: stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`pernambuco_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegPern::create_schedule` (0x438bf0, bra_reg_pern.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_pern_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), pernambuco_schedule_stage(stage), rounds, extra, league, 0x98afd8)
}

/// Comparer for `bra_reg_pern_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_pern_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_PERN_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_pern_create_schedule", stage, pernambuco_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_pern_create_schedule(this, stage, r, e, l))
}

/// The Rio de Janeiro state league (Carioca), the round-robin stage (−1): 22 rounds (11 Jan to 3 May), 1430 bytes, 78 operations.
const RIO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 78] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 4, 3, 0, 2, 2, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 8, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0xf, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x19, 3, 0, 2, 2, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0), Cqp11D(0xf, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0),
    Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 6, 4, 0, 6, 1, 0),
    Cqp11L(0x10, 0, 5, 1, 1, 0), Cqp11L(0x10, 1, 5, 1, 1, 0), Cqp11L(0x10, 2, -1, -1, -1, 0),
    Cqp11D(0x11, 0xd, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, 5, 1, 1, 0), Cqp11L(0x11, 1, 5, 1, 1, 0),
    Cqp11L(0x11, 2, -1, -1, -1, 0), Cqp11D(0x12, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x12, 0, -1, -1, -1, 0),
    Cqp11D(0x13, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x13, 0, 5, 1, 1, 0), Cqp11L(0x13, 1, 5, 1, 1, 0),
    Cqp11L(0x13, 2, -1, -1, -1, 0), Cqp11D(0x14, 0x1b, 4, 0, 6, 1, 0), Cqp11L(0x14, 0, 5, 1, 1, 0),
    Cqp11L(0x14, 1, 5, 1, 1, 0), Cqp11L(0x14, 2, -1, -1, -1, 0), Cqp11D(0x15, 3, 5, 0, 6, 1, 0),
    Cqp11L(0x15, 0, -1, -1, -1, 0),
];

/// The Rio de Janeiro state league (Carioca), stage 2: 1 round (4 May to 10 May), 104 bytes, 18 operations.
const RIO_SCHEDULE_STAGE_2_OPS: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 4, 5, 0, 0), Cqp11J(0, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x18, 2), Cqp11W(0x1c, 2), Cqp11B(0x21, 2),
    Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x1a, 1),
    Cqp11W(0xf, 0x804), Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0xa4cb8),
    Cqp11X(0x64, 0x3d090),
];

/// The stages of the round table of the Rio de Janeiro state league (Carioca) (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage has no table (None).
fn rio_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 22, extra: 0, extra_check: false, size: 0x596, row: 0xa6,
            ops: &RIO_SCHEDULE_LEAGUE_OPS, name: "cqw_3 rio league" }),
        2 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13a,
            ops: &RIO_SCHEDULE_STAGE_2_OPS, name: "cqw_3 rio stage 2" }),
        _ => None,
    }
}

/// The schedule of the Rio de Janeiro state league (Carioca): stage −1, the round-robin league stage: 22 rounds from 11 Jan to 3 May; stage 2, the knockout or final stage: 1 round from 4 May to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`rio_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegRio::create_schedule` (0x439d40, bra_reg_rio.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_rio_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), rio_schedule_stage(stage), rounds, extra, league, 0x98b014)
}

/// Comparer for `bra_reg_rio_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_rio_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_RIO_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_rio_create_schedule", stage, rio_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_rio_create_schedule(this, stage, r, e, l))
}

/// The Santa Catarina state league, the round-robin stage (−1): 18 rounds (11 Jan to 20 Apr), 1170 bytes, 64 operations.
const SANTA_CATARINA_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The Santa Catarina state league, stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
const SANTA_CATARINA_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x1e848), Cqp11X(0xcc, 0xc350),
];

/// The stages of the round table of the Santa Catarina state league (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
fn santa_catarina_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &SANTA_CATARINA_SCHEDULE_LEAGUE_OPS, name: "cqw_3 santa catarina league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &SANTA_CATARINA_SCHEDULE_STAGE_0_OPS, name: "cqw_3 santa catarina stage 0" }),
        _ => None,
    }
}

/// The schedule of the Santa Catarina state league: stage −1, the round-robin league stage: 18 rounds from 11 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`santa_catarina_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegSanta::create_schedule` (0x43b430, bra_reg_santa.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_santa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), santa_catarina_schedule_stage(stage), rounds, extra, league, 0x98b050)
}

/// Comparer for `bra_reg_santa_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_santa_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_SANTA_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_santa_create_schedule", stage, santa_catarina_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_santa_create_schedule(this, stage, r, e, l))
}

/// The São Paulo state league (Paulista), the round-robin stage (−1): 15 rounds (18 Jan to 20 Apr), 975 bytes, 56 operations.
const SAO_PAULO_SCHEDULE_LEAGUE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0x12, 1, 0, 6, 2, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x19, 1, 0, 6, 1, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 4, 2, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 0xb, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0x12, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x19, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 1, 3, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 8, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 0xf, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0x16, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 6, 4, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 0x14, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, -1, -1, -1, 0),
];

/// The São Paulo state league (Paulista), stage 0: 2 rounds (21 Apr to 10 May), 208 bytes, 36 operations.
const SAO_PAULO_SCHEDULE_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0xa4cb8), Cqp11X(0xcc, 0x2ab98),
];

/// The stages of the round table of the São Paulo state league (Paulista) (virtual `comp_vtable::CREATE_SCHEDULE`): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage has no table (None).
fn sao_paulo_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa8,
            ops: &SAO_PAULO_SCHEDULE_LEAGUE_OPS, name: "cqw_3 sao paulo league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x118,
            ops: &SAO_PAULO_SCHEDULE_STAGE_0_OPS, name: "cqw_3 sao paulo stage 0" }),
        _ => None,
    }
}

/// The schedule of the São Paulo state league (Paulista): stage −1, the round-robin league stage: 15 rounds from 18 Jan to 20 Apr; stage 0, the knockout or final stage: 2 rounds from 21 Apr to 10 May; any other stage returns 0.
///
/// Virtual `comp_vtable::CREATE_SCHEDULE`, `create_schedule(stage, &rounds, &extra, &is_league)`, called by
/// League::create_fixtures and the season turns. For a stage with a table (`sao_paulo_schedule_stage`) it writes the outputs (is_league
/// when that pointer is not null, the rounds and the extra), allocates the round table (`exe::crt_malloc`, 0x41 bytes
/// per round; out of memory, the error box and 0) and writes the date (day, month, years after the season year
/// `year()`, weekday and code) and the matches of each round, in the original's order (`create_round_table`, the
/// cqp_11 schedule machine); the rest of the table is left as allocated. Returns the table, or 0 without writing
/// anything for a stage without one.
/// Original: `BraRegSp::create_schedule` (0x43c5a0, bra_reg_sp.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_reg_sp_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), sao_paulo_schedule_stage(stage), rounds, extra, league, 0x98b090)
}

/// Comparer for `bra_reg_sp_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_bra_reg_sp_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_BRA_REG_SP_CREATE_SCHEDULE);
    asia_club_champ_compare_round_table("bra_reg_sp_create_schedule", stage, sao_paulo_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_sp_create_schedule(this, stage, r, e, l))
}
