// ---------- mwb_11: the League of Wales (wel_first.cpp): constructor, rules and turnover ----------

/// The League of Wales: discipline rule 0x28, no stadium requirement, no stages beyond the league; the table is
/// WelFirst::create_table (0x925b30, still in the original).
pub(crate) const LEAGUE_OF_WALES: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x970f80,
    stadium: None,
    discipline: 0x28, // mwb11: the League of Wales discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x925c50,
    table: 0x925b30,
    file: 0xad56b8,
    rows: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// The League of Wales at career creation: discipline rule 0x28, no stages beyond the league, the rules, the table with
/// its 18 clubs, the fixtures, the order, the statistics and the predicted positions (`construct_british_division`, the
/// mould of mwb_5).
/// Original: `WelFirst::WelFirst` (0x924e30, wel_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &LEAGUE_OF_WALES, "mwb_11 new league of wales")
}

/// Comparer for `construct_wel_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_british_division_ctor("construct_wel_first", League(this), year, record, &LEAGUE_OF_WALES, || o(this, year, record))
}

/// The League of Wales rule writes: match rules and round rules 0 (`league::MATCH_RULES`, `league::ROUND_DRAW_RULES`),
/// nobody promoted or relegated (no division above or below: −1), squad flags 0, 3 points for a win and 1 for a draw,
/// tie-breaks 1, 2 and 3, 2 rounds, category 2, club sides, 3 substitutions and 5 substitutes named.
pub(crate) fn league_of_wales_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0),
         (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), (league::SQUAD_FLAGS, 1, 0),
         (league::WIN_POINTS, 1, 3), // mwb11: the League of Wales points for a win
         (league::TIEBREAKS + 2, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEGS, 2, 2), (league::MATCH_CATEGORY, 1, 2), (league::TIEBREAKS + 1, 1, 2),
         (league::DRAW_POINTS, 1, 1), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff),
         (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// rule: the League of Wales (18 clubs) plays 2 rounds, 3 points for a win and 1 for a draw, nobody goes up or down
/// (there is no division above or below in the game); tie-breaks goal difference, goals for and position [inference
/// from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `WelFirst::set_up_rules` (0x925c50, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_11 league of wales rules");
    eng_prm_apply_league_rules(League(this), &league_of_wales_rules())
}

/// Comparer for `wel_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("wel_first_set_up_rules", League(this), league_of_wales_rules(), || o(this), || wel_first_set_up_rules(this))
}

/// rule: at the turnover the League of Wales records the final positions (virtual +0xa4), archives the season, frees the
/// table and the rounds, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the
/// predicted positions, the board expectations and the referees renewed (`british_division_new_season` with the Welsh
/// steps, mwb_6; the error boxes of lines 0x1f9, 0x200, 0x206 and 0x20d)
/// Original: `WelFirst::start_new_season` (0x925cf0, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &LEAGUE_OF_WALES, [0x1f9, 0x200, 0x206, 0x20d], true, 1, "mwb_11 league of wales turnover") // mwb11: the next League of Wales season
}

/// Comparer for `wel_first_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("wel_first_start_new_season", League(this), || o(this))
}
