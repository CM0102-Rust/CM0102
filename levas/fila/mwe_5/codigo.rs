// ---------- batch %%LEVA%%: the Allsvenskan constructor and the league rules of the two Finnish divisions (swe_prm.cpp, fin_prm.cpp, fin_first.cpp) ----------

/// The six regional sections of the Swedish Second Division (Norrland, East and West Svealand, East, West and South
/// Götaland), in the original's comparison order.
pub(crate) const SWEDISH_SECOND_DIVISION_SECTIONS: [usize; 6] = [KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_SVEALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_SVEALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_EAST_GOTALAND, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_WEST_GOTALAND,
    KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_SOUTH_GOTALAND];

/// Where the Allsvenskan constructor files club `c`: Some((new division, new reserve division)) or None (untouched).
///
/// A club in one of the six Second Division sections keeps the section in `club::RESERVE_DIVISION` and goes into the
/// Second Division (`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`); a Swedish club (`club::NATION` with the id of
/// `KEY_NATION_SWEDEN`) outside the Allsvenskan, the Superettan and the Second Division goes into the lower division
/// (`KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION`); a club without division, or without nation, is untouched.
pub(crate) unsafe fn swedish_club_filing(c: usize) -> Option<(usize, Option<usize>)> {
    let comp = read_usize(c + club::DIVISION);
    if comp == 0 {
        return None;
    }
    let id = read_usize(comp);
    if SWEDISH_SECOND_DIVISION_SECTIONS.iter().any(|&g| read_usize(g) == id) {
        return Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), Some(comp))); // mwe5: a section club goes into the Second Division
    }
    let nation = read_usize(c + club::NATION);
    if nation == 0 || read_usize(nation) != read_usize(KEY_NATION_SWEDEN) {
        return None;
    }
    if [KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION, KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION].iter().any(|&g| read_usize(g) == id) {
        return None;
    }
    Some((comp_record_of(KEY_CLUB_COMP_SWEDISH_LOWER_DIVISION), None))
}

/// The number of club records the filing loop walks: `G_CLUB_COUNT` − 2 × `G_NATION_COUNT` (re-read each turn).
pub(crate) unsafe fn swedish_filing_count() -> i32 {
    (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2))
}

/// Builds the Swedish Premier Division (Allsvenskan): discipline rule 28 (the Swedish league one), the league only; with
/// no saved state, it first files every Second Division section club under the Second Division (the section kept as
/// the reserve division) and every other Swedish club outside the top three divisions under the lower division; then
/// the rules, the table, the fixtures, the order, the stats and the predicted positions.
///
/// `SwePrm::SwePrm(year, record)`: League::League (`exe::new_league`), the record (`league::COMPETITION`), the vtable
/// 0x970198, the year (word) and the discipline byte 0x1c; the saved state (`exe::league_load_from_temp_file`, push 1)
/// ends it. Otherwise, for each of the first `G_CLUB_COUNT` − 2 × `G_NATION_COUNT` club records (`club::SIZE` bytes
/// from `G_CLUB_TABLE`, the count re-read each turn), the filing of `swedish_club_filing` (the new division as the
/// competition record, `G_CLUB_COMP_TABLE` + id × 0x6b); then stage index 0xff, current stage −1, no extra stages,
/// `SwePrm::set_up_rules` (0x890550) and `SwePrm::create_table` (0x890430) (direct, thiscall, ret 0),
/// `exe::league_create_fixtures`, `exe::league_sort_table`(0), the Comp_Stats into `+8` and
/// `exe::league_set_predicted_positions`; each failing step shows the box of swe_prm.cpp (0xa8b578) at lines 0x82,
/// 0x89, 0x90, 0x98, 0x9f and returns. Returns this.
/// Original: `SwePrm::SwePrm` (0x88fb10, swe_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_prm(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), 0x970198);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(discipline_rule::SWEDEN_LEAGUE);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!("mwe_5 new allsvenskan");
    let mut i = 0i32;
    let mut c = read_usize(G_CLUB_TABLE);
    while i < swedish_filing_count() {
        if let Some((division, section)) = swedish_club_filing(c) {
            if let Some(s) = section {
                write_usize(c + club::RESERVE_DIVISION, s);
            }
            write_usize(c + club::DIVISION, division);
        }
        i += 1;
        c += club::SIZE;
    }
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(0);
    this.set_stages(0);
    let file = 0xa8b578;
    if swe_prm_call(0x890550, this) == 0 {
        show_error_at(file, 0x82);
        return this.addr();
    }
    if swe_prm_call(0x890430, this) == 0 {
        show_error_at(file, 0x89);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x90);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x98);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(file, 0x9f);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// A direct thiscall with no arguments on the league (set_up_rules, create_table), returning eax.
pub(crate) unsafe fn swe_prm_call(f: usize, this: League) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(this.addr())
}

/// The rule writes of the Finnish Premier Division (Veikkausliiga, 12 clubs), in the original's order.
///
/// Three legs (33 rounds), 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw
/// rules, club sides; tie-breaks goal difference, goals for and code 3; nobody goes up, no promotion play-off, 1 club
/// to the relegation play-off (the 11th, against the First Division) and 1 relegated; `league::LEAGUE_FLAGS` = 3 (the
/// champion status and promotion status 0); 7 substitutes named, 3 substitutions, no division above, the First
/// Division below and the squad flags 2.
pub(crate) unsafe fn finnish_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 3)];
    v.extend_from_slice(&[(league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)]);
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1)]);
    v.push((league::RELEGATED_COUNT, 1, 1)); // mwe5: the Veikkausliiga relegates 1
    v.extend_from_slice(&[(league::LEAGUE_FLAGS, 2, 3), (league::SUBSTITUTES_NAMED, 1, 7), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_FINNISH_FIRST_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v
}

/// The rule writes of the Finnish First Division (Ykkönen, the North group of 10 in the league stage), in the
/// original's order.
///
/// Three legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for
/// and code 3; 2 promoted, no promotion play-off, 1 to the relegation play-off and 2 relegated (no division below:
/// −1); 3 substitutions, the Premier Division above, the squad flags 2, `league::LEAGUE_FLAGS` = 8 (promotion status
/// 6) and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn finnish_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe5: the Ykkonen promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_FINNISH_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::LEAGUE_FLAGS, 2, 8), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// Sets up the rules of the Finnish Premier Division (Veikkausliiga, 12 clubs): three legs (33 rounds), 3 points for a
/// win; nobody goes up, the 11th plays the relegation play-off and the 12th goes down to the Ykkönen; 7 substitutes and
/// 3 substitutions.
///
/// `FinPrm::set_up_rules()`: the writes of `finnish_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `FinPrm::set_up_rules` (0x595c90, fin_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_5 veikkausliiga rules");
    eng_prm_apply_league_rules(League(this), &finnish_premier_rules())
}

/// Sets up the rules of the Finnish First Division (Ykkönen): three legs, 3 points for a win; 2 go up, 1 plays the
/// relegation play-off and 2 go down; 7 substitutes and 3 substitutions.
///
/// `FinFirst::set_up_rules()`: the writes of `finnish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `FinFirst::set_up_rules` (0x593b80, fin_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn fin_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_5 ykkonen rules");
    eng_prm_apply_league_rules(League(this), &finnish_first_rules())
}

/// Comparer for `construct_swe_prm`: only the original runs (it allocates); the filing is planned beforehand and
/// checked club by club afterwards, with the fields of `compare_division_league_ctor`.
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let loaded = league_will_load_saved_state(record);
    let mut plan: Vec<(usize, usize, usize)> = Vec::new();
    if !loaded {
        let mut c = read_usize(G_CLUB_TABLE);
        for _ in 0..swedish_filing_count().max(0) {
            if let Some((d, s)) = swedish_club_filing(c) {
                plan.push((c, d, s.unwrap_or(read_usize(c + club::RESERVE_DIVISION))));
            }
            c += club::SIZE;
        }
    }
    let p = LeagueDivision { vtabela: 0x970198, discipline: discipline_rule::SWEDEN_LEAGUE, rules: 0x890550, table: 0x890430, file: 0xa8b578, rows: [0x82, 0x89, 0x90, 0x98, 0x9f], low: false };
    let r = compare_division_league_ctor("construct_swe_prm", League(this), year, record, &p, || o(this, year, record));
    let wrong: Vec<&(usize, usize, usize)> = plan.iter().filter(|&&(c, d, s)| read_usize(c + club::DIVISION) != d || read_usize(c + club::RESERVE_DIVISION) != s).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("construct_swe_prm({:#x}) filing: {} clubs planned, wrong (club, division, reserve) = {:x?}", this, plan.len(), &wrong[..wrong.len().min(8)]));
    r
}

/// Comparer for `fin_prm_set_up_rules`: the dry run of `eng_prm_compare_league_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fin_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("fin_prm_set_up_rules", League(this), finnish_premier_rules(), || o(this), || fin_prm_set_up_rules(this))
}

/// Comparer for `fin_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("fin_first_set_up_rules", League(this), finnish_first_rules(), || o(this), || fin_first_set_up_rules(this))
}
