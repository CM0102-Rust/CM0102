// ---------- cx08_8: the Argentine Second Division table, rules and turnover (arg_second.cpp) ----------

/// The fixed 25th club of the Argentine Second Division (Huracán de Tres Arroyos), added when only 24 clubs match.
const ARG_SECOND_FIXED_CLUB: usize = 0x9d020c;

/// The clubs of the Argentine Second Division table, in database order: those with a division (`club::DIVISION`) whose
/// id is that of `KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION` and with a `club::RESERVE_DIVISION`, up to `n`.
unsafe fn arg_second_division_clubs(n: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 && v.len() < n {
        let club = club_at(i as isize);
        let division = club.division().addr();
        if division != 0 && read_usize(club.addr() + club::RESERVE_DIVISION) != 0 && read_usize(division) == read_usize(KEY_CLUB_COMP_ARGENTINE_SECOND_DIVISION) {
            v.push(club.addr());
        }
        i += 1;
    }
    v
}

/// Rule: the Argentine Second Division (Nacional B) has 25 clubs; when only 24 are found, Huracán de Tres Arroyos is put
/// into the division as the 25th; no prize fund (prize mode 2).
///
/// `ArgSecond::fill_league_table()`: `league::CLUB_COUNT` = 25, the table (`exe::crt_malloc` of 25 × 0x3b; none: the
/// arg_second.cpp box of line 0x247 and 0) with `arg_second_division_clubs` in order (League::init_table_row
/// `exe::league_init_table_row`(row, club, 0, 0)). With exactly 24: the fixed club (`club_at` of the id at
/// `ARG_SECOND_FIXED_CLUB`) whose division is not this competition's record gets it (`club::DIVISION`) and the row 24,
/// and the count becomes 25. Not full: 0. Full: `league::PRIZE_FUND` = 0, `league::PRIZE_MODE` = 2 and 1.
/// Original: `ArgSecond::fill_league_table` (0x407cd0, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_fill_league_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0x19);
    let t = exe::crt_malloc(0x19 * 0x3b);
    this.set_table(t);
    if t == 0 {
        show_error_at(ARG_SECOND_FILE, 0x247);
        return 0;
    }
    let clubs = arg_second_division_clubs(0x19);
    for (k, &club) in clubs.iter().enumerate() {
        exe::league_init_table_row(this, k as u32, club, 0, 0);
    }
    let mut n = clubs.len() as i16;
    if n == 0x18 {
        let fixed = club_at(read_i32(ARG_SECOND_FIXED_CLUB) as isize);
        if fixed.division().addr() != this.competition().addr() {
            covered!("cx08_8 argentine second fixed club");
            write_usize(fixed.addr() + club::DIVISION, this.competition().addr());
            exe::league_init_table_row(this, 0x18, fixed, 0, 0);
            n = 0x19;
        }
    }
    if n != this.club_count() {
        return 0;
    }
    covered!("cx08_8 argentine second full table");
    this.set_prize_fund(0);
    this.set_prize_mode(2);
    1
}

/// Comparer for `arg_second_fill_league_table`: only the original runs (it allocates the table and may move the fixed
/// club), against the plan made beforehand (`arg_second_division_clubs`, the fixed club as the 25th when 24 match and it
/// is not in the division); the return, the count and the rows must match it.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_fill_league_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let l = League(this);
    let mut plan = arg_second_division_clubs(0x19);
    let fixed = club_at(read_i32(ARG_SECOND_FIXED_CLUB) as isize);
    if plan.len() == 0x18 && fixed.division().addr() != l.competition().addr() {
        plan.push(fixed.addr());
    }
    let r = o(this);
    let rows: Vec<usize> = if l.table() != 0 { (0..plan.len()).map(|k| read_usize(l.table() + k * 0x3b)).collect() } else { Vec::new() };
    let ok = r == (plan.len() == 0x19) as u32 && l.club_count() == 0x19 && rows == plan && (r == 0 || l.prize_mode() == 2);
    SCOREBOARD.record(ok, || format!("arg_second_fill_league_table({:#x}) returned {} plan {:x?} rows {:x?}", this, r, plan, rows));
    r
}

/// Sets up the rules of the Argentine Second Division: a single round, 3 points for a win and 1 for a draw, category
/// 2; one club promoted directly, no play-off or relegation places (relegation goes by the averages); above:
/// `KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION`; below: none; tie-breaker codes 1, 2, 3, 0; rule bits 3; 5 substitutes
/// named and 3 substitutions; then builds the main-stage round table through virtual +0x3c into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `ArgSecond::set_up_rules` (0x407e50, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_set_up_rules(this: usize) -> u32 {
    covered!("cx08_8 argentine second rules");
    fra_first_apply_league_rules(League(this), &arg_second_rules())
}

/// Comparer for `arg_second_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("arg_second_set_up_rules", League(this), arg_second_rules(), || o(this), || arg_second_set_up_rules(this))
}

/// The rule writes of the Argentine Second Division (offset, size, value), in the original's order.
pub(crate) unsafe fn arg_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::LEGS, 2, 1), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
         (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3),
         (league::TIEBREAKS + 3, 1, 0), (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
         (league::RELEGATED_COUNT, 1, 0), (league::LEAGUE_FLAGS, 2, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3),
         (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_ARGENTINE_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, 0xffff_ffff),
         (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// Rule: the Argentine Second Division turnover (called by the Premier Division's): the season reopened and rebuilt like
/// the Belgian Second Division's (`rebuild_season` with ArgSecond::fill_league_table at 0x407cd0 and the boxes of
/// arg_second.cpp at lines 0x2ba, 0x2c1, 0x2c7, 0x2ce; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `ArgSecond::start_new_season` (0x407f00, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x407cd0, ARG_SECOND_FILE, [0x2ba, 0x2c1, 0x2c7, 0x2ce], 1) {
        return 0;
    }
    covered!("cx08_8 argentine second turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `arg_second_start_new_season`: only the original runs; the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("arg_second_start_new_season", League(this), || o(this))
}
