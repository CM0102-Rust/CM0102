// ---------- batch %%LEVA%%: the Northern Irish First Division: the constructor, the rules and the table (nir_first.cpp) ----------

/// The Northern Irish First Division's constructor: the vtable 0x96db64, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, one extra stage (the promotion play-off), NirFirst::set_up_rules (0x78bd70) and
/// NirFirst::create_table (0x78bc50), the lines 0x49, 0x50, 0x57, 0x60 and 0x67 of nir_first.cpp.
pub(crate) const NORTHERN_IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96db64, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x78bd70, table: 0x78bc50, file: 0xa6965c, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// Rule: the Northern Irish First Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, the league and one extra stage (the promotion play-off): the rules,
/// the table, the fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `NirFirst::NirFirst` (0x78b010, nir_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, "mwa_10 northern irish first new") // mwa10: the Northern Irish First
}

unsafe extern "thiscall" fn compare_construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_irish_league_constructor("construct_nir_first", League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish First Division's rules: four meetings, the tie-breaks 1, 2 and 4, 3 points for a win and 1 for a
/// draw, match category 2, club sides, one promoted directly and one to the promotion play-off, no relegation, 3
/// substitutes on the bench and 3 substitutions, the squad flags 2; the Premier Division above, none below.
pub(crate) const NORTHERN_IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::TIEBREAKS + 2, 1, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2),
        (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa10: one up, one to the play-off
        (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)],
    upper: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION),
    lower: None,
};

/// Rule: the Northern Irish First Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties split
/// by goal difference and goals for; the champion goes up to the Premier Division and the runner-up plays the promotion
/// play-off; no relegation (no division below); 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_FIRST_RULES`. Returns 1.
/// Original: `NirFirst::set_up_rules` (0x78bd70, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_10 northern irish first rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_FIRST_RULES)
}

unsafe extern "thiscall" fn compare_nir_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_irish_league_rules("nir_first_set_up_rules", this, &NORTHERN_IRISH_FIRST_RULES, || o(this))
}

/// Rule: the Northern Irish First Division table has ten clubs, the division's in database order; when full, the prize
/// fund for a 300 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION`; out of memory, the error box
/// at line 0x190 of nir_first.cpp); not full, 0; full, the prize fund (`exe::league_calc_prize_fund`(this, 300)) in
/// `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirFirst::create_table` (0x78bc50, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa6965c, 0x190, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_10 northern irish first table");
    set_prize_fund(League(this), 0x12c); // mwa10: the Northern Irish First quota
    1
}

unsafe extern "thiscall" fn compare_nir_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    ita_ser_a_compare_league_table("nir_first_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, Some(0x12c), || o(this))
}
