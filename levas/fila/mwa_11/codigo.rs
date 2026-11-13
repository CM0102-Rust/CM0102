// ---------- batch %%LEVA%%: the Irish Premier Division: the constructor, the rules and the table (ire_prm.cpp) ----------

/// The Irish Premier Division's constructor: the vtable 0x96c014, discipline rule `discipline_rule::IRELAND`, no extra
/// stage, the ten slots from +0xee reset, IrePrm::set_up_rules (0x63a900) and IrePrm::create_table (0x63a740), the
/// lines 0x4b, 0x52, 0x59, 0x62 and 0x69 of ire_prm.cpp.
pub(crate) const IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96c014, discipline: discipline_rule::IRELAND,
    play_off_stage: false, reset_ten_slots: true, rules: 0x63a900, table: 0x63a740, file: 0x9ea080, rows: [0x4b, 0x52, 0x59, 0x62, 0x69] };

/// Rule: the Irish Premier Division at career creation: the year and the database competition, the Irish discipline
/// rules, its ten own slots cleared and, with nothing saved, a plain league: the rules, the table, the fixtures, the
/// order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against JapJ2::JapJ2:
/// the same steps with the ten 5-byte slots from +0xee set to −1 before loading).
/// Original: `IrePrm::IrePrm` (0x639bb0, ire_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, "mwa_11 irish premier new") // mwa11: the Irish Premier
}

unsafe extern "thiscall" fn compare_construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_league_constructor("construct_ire_prm", League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// The Irish Premier Division's rules: three meetings (`league::LEGS` = 3; the table sets 4 with ten clubs), 3 points
/// for a win and 1 for a draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides,
/// the tie-breaks 1 and 2 (goal difference, goals for), no promotion, one club to the relegation play-off and one
/// relegated (the table sets 1 and 3 with twelve clubs), the squad flags 0, 5 substitutes on the bench and 3
/// substitutions, `league::LEAGUE_FLAGS` 0x103; no division above, the First Division below.
pub(crate) const IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa11: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5), (league::LEAGUE_FLAGS, 2, 0x103)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_IRISH_FIRST_DIVISION),
};

/// Rule: the Irish Premier Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for;
/// the last goes down to the First Division and the second last plays the play-off (with twelve clubs, three go down
/// and one plays the play-off); 5 substitutes on the bench and 3 substitutions; no division above.
///
/// `set_up_irish_league_rules` with `IRISH_PREMIER_RULES`. Returns 1.
/// Original: `IrePrm::set_up_rules` (0x63a900, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_11 irish premier rules");
    set_up_irish_league_rules(League(this), &IRISH_PREMIER_RULES)
}

unsafe extern "thiscall" fn compare_ire_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_irish_league_rules("ire_prm_set_up_rules", this, &IRISH_PREMIER_RULES, || o(this))
}

/// How many clubs of the division whose id is in the key global `key` (all `G_CLUB_COUNT` clubs, a 16-bit counter as
/// in the original).
pub(crate) unsafe fn irish_division_club_count(key: usize) -> i16 {
    let mut n = 0i16;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let d = club_at(i as isize).division();
        if !d.is_null() && read_usize(d.addr()) == read_usize(key) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    n
}

/// The body of `ire_prm_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only ten or twelve are accepted (otherwise 0): ten meet four times (`league::LEGS` = 4), twelve
/// three times. The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x186 of
/// ire_prm.cpp and 0) with the clubs in database order; with twelve rows filled, one club to the relegation play-off and
/// three relegated; full, the prize fund for a 30,000 share per place (`set_prize_fund`) and 1; otherwise 0.
pub(crate) unsafe fn create_irish_premier_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    this.set_club_count(n);
    match n {
        0xa => this.set_legs(4),
        0xc => this.set_legs(3),
        _ => return 0,
    }
    let full = match ita_ser_a_fill_league_table(this, n, 0x9ea080, 0x186, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) {
        Some(f) => f,
        None => return 0,
    };
    if full && n == 0xc {
        covered!("mwa_11 irish premier of twelve");
        this.set_relegation_playoff_count(1);
        this.set_relegated_count(3); // mwa11: three go down from a twelve-club Premier
    }
    if !full {
        return 0;
    }
    covered!("mwa_11 irish premier table");
    set_prize_fund(this, 0x7530); // mwa11: the Irish Premier quota
    1
}

/// Rule: the Irish Premier Division table takes the division's clubs in database order; only ten (meeting four times)
/// or twelve (three times, with three relegated and one in the play-off) are accepted; when full, the prize fund for a
/// 30,000 share per place, paid by table position at the season's end.
/// Original: `IrePrm::create_table` (0x63a740, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_table(this: usize) -> u32 {
    create_irish_premier_table(League(this))
}

/// The comparer of the Irish tables (they allocate the table: only the original runs): the plan is the count of the
/// division's clubs; out of the accepted counts the routine must return 0; otherwise the rows, the return and the
/// prize (`ita_ser_a_compare_league_table`) and the meetings `legs`.
pub(crate) unsafe fn compare_irish_counted_table(name: &str, this: League, key: usize, legs: impl Fn(i16) -> Option<i16>, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let n = irish_division_club_count(key);
    match legs(n) {
        None => {
            let r = original();
            SCOREBOARD.record(r == 0, || format!("{}({:#x}) with {} clubs returned {}", name, this.addr(), n, r));
            r
        }
        Some(l) => {
            let r = ita_ser_a_compare_league_table(name, this, n, key, quota, original);
            SCOREBOARD.record(this.legs() == l, || format!("{}({:#x}) with {} clubs: legs {} (plan {})", name, this.addr(), n, this.legs(), l));
            r
        }
    }
}

unsafe extern "thiscall" fn compare_ire_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_irish_counted_table("ire_prm_create_table", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, |n| match n { 0xa => Some(4), 0xc => Some(3), _ => None }, Some(0x7530), || o(this))
}
