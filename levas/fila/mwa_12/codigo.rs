// ---------- batch %%LEVA%%: the Irish First Division: the constructor, the rules and the table (ire_first.cpp) ----------

/// The Irish First Division's constructor: the vtable 0x96bccc, discipline rule `discipline_rule::IRELAND`, one extra
/// stage (the promotion play-off), IreFirst::set_up_rules (0x635aa0) and IreFirst::create_table (0x635910), the lines
/// 0x49, 0x50, 0x57, 0x60 and 0x67 of ire_first.cpp.
pub(crate) const IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96bccc, discipline: discipline_rule::IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x635aa0, table: 0x635910, file: 0x9e9ed0, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// Rule: the Irish First Division at career creation: the year and the database competition, the Irish discipline rules
/// and, with nothing saved, the league and one extra stage (the promotion play-off): the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `IreFirst::IreFirst` (0x634c40, ire_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, "mwa_12 irish first new") // mwa12: the Irish First
}

unsafe extern "thiscall" fn compare_construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_league_constructor("construct_ire_first", League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// The Irish First Division's rules: four meetings (the table sets 3 with eleven or twelve clubs), 3 points for a win
/// and 1 for a draw, match category 2, club sides, the tie-breaks 1 and 2, one promoted directly and one to the
/// promotion play-off, no relegation, the squad flags 2, 5 substitutes on the bench and 3 substitutions; the Premier
/// Division above, none below.
pub(crate) const IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa12: one up, one to the play-off
        (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)],
    upper: Some(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION),
    lower: None,
};

/// Rule: the Irish First Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the
/// champion goes up to the Premier Division and the runner-up plays the promotion play-off; no relegation (no division
/// below); 5 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `IRISH_FIRST_RULES`. Returns 1.
/// Original: `IreFirst::set_up_rules` (0x635aa0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_12 irish first rules");
    set_up_irish_league_rules(League(this), &IRISH_FIRST_RULES)
}

unsafe extern "thiscall" fn compare_ire_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_irish_league_rules("ire_first_set_up_rules", this, &IRISH_FIRST_RULES, || o(this))
}

/// The meetings of an Irish First Division of `n` clubs: nine or ten meet four times, eleven or twelve three times;
/// other counts are refused.
pub(crate) fn irish_first_meetings(n: i16) -> Option<i16> {
    match n {
        9 | 10 => Some(4), // mwa12: nine or ten clubs meet four times
        11 | 12 => Some(3),
        _ => None,
    }
}

/// The body of `ire_first_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only nine to twelve are accepted (otherwise 0), with the meetings of `irish_first_meetings`.
/// The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x1b6 of
/// ire_first.cpp and 0) with the clubs in database order; returns whether it filled up (no prize fund).
pub(crate) unsafe fn create_irish_first_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
    this.set_club_count(n);
    let legs = match irish_first_meetings(n) {
        Some(l) => l,
        None => return 0,
    };
    this.set_legs(legs);
    covered!("mwa_12 irish first table");
    (ita_ser_a_fill_league_table(this, n, 0x9e9ed0, 0x1b6, KEY_CLUB_COMP_IRISH_FIRST_DIVISION) == Some(true)) as u32
}

/// Rule: the Irish First Division table takes the division's clubs in database order; only nine to twelve are
/// accepted: nine or ten meet four times, eleven or twelve three times; no prize fund.
/// Original: `IreFirst::create_table` (0x635910, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_create_table(this: usize) -> u32 {
    create_irish_first_table(League(this))
}

unsafe extern "thiscall" fn compare_ire_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_irish_counted_table("ire_first_create_table", League(this), KEY_CLUB_COMP_IRISH_FIRST_DIVISION, irish_first_meetings, None, || o(this))
}
