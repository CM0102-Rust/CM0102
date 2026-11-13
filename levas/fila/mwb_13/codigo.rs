// ---------- mwb_13: the Scottish Premier Division (sco_prm.cpp): constructor, table and rules ----------

/// The Scottish Premier Division: a stadium of 10,000 places, all seated, discipline rule 0x18, no stages beyond the
/// league, and the split's table compare (0x7f3220) given back to a saved season played in four rounds.
pub(crate) const SCOTTISH_PREMIER_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f608,
    stadium: Some((0x2710, 0x2710)),
    discipline: 0x18, // mwb13: the Premier discipline rule
    playoff_in_1999: false,
    split_compare: Some(0x7f3220),
    rules: 0x7f2970,
    table: 0x7f2820,
    file: 0xa7af70,
    rows: [0x53, 0x5a, 0x61, 0x68, 0x6f],
};

/// The Scottish Premier Division at career creation: a stadium of 10,000 places (all seated), discipline rule 0x18, the
/// rules, the table with its 12 clubs (10 in 1999), the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5); a saved season played in four rounds (the split) outside 1999
/// gets back the split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220).
/// Original: `ScoPrm::ScoPrm` (0x7f0b80, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_PREMIER_DIVISION, "mwb_13 new scottish premier")
}

/// Comparer for `construct_sco_prm` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_british_division_ctor("construct_sco_prm", League(this), year, record, &SCOTTISH_PREMIER_DIVISION, || o(this, year, record))
}

/// The number of Premier clubs: 10 in 1999, 12 from 2000.
pub(crate) unsafe fn scottish_premier_clubs(this: League) -> i16 {
    if this.year() == 0x7cf { 0xa } else { 0xc }
}

/// The Premier table: 12 clubs (10 in 1999), those whose division is the Scottish Premier, in database order; the prize
/// fund of 50,000 per place (League::calc_prize_fund) and 500,000 of TV money to every club (League::pay_tv_rights with
/// no division filter); out of memory, the box of line 0x2a7
/// Original: `ScoPrm::create_table` (0x7f2820, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_table(this: usize) -> u32 {
    let n = scottish_premier_clubs(League(this));
    fill_british_division_table(League(this), n, 0xa7af70, 0x2a7, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), Some(0x7a120), 0, "mwb_13 scottish premier table") // mwb13: the Premier rows
}

/// Comparer for `sco_prm_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let n = scottish_premier_clubs(League(this));
    compare_british_division_table("sco_prm_create_table", League(this), n, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}

/// The Premier rule writes before the round table: in 1999 4 rounds, nobody promoted, 1 to the relegation play-off and
/// nobody relegated directly, the games per club not computed (−1); from 2000 3 rounds, the bottom club relegated and 38
/// games per club; then no custom table compare (`league::CUSTOM_COMPARE` = 0), 3 points for a win and 1 for a draw,
/// category 2, match and round rules 0, club sides, tie-breaks 1, 2 and 3, no division above (−1) and the First below,
/// 3 substitutions, squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn scottish_premier_rules(this: League) -> Vec<(usize, u8, u32)> {
    let mut v = if this.year() == 0x7cf {
        vec![(league::LEGS, 2, 4), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1),
             (league::RELEGATED_COUNT, 1, 0), (league::GAMES_PER_CLUB, 2, 0xffff)]
    } else {
        vec![(league::LEGS, 2, 3), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
             (league::RELEGATED_COUNT, 1, 1), (league::GAMES_PER_CLUB, 2, 0x26)]
    };
    v.extend_from_slice(&[(league::CUSTOM_COMPARE, 4, 0), (league::WIN_POINTS, 1, 3), // mwb13: the Premier points for a win
                          (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0),
                          (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3),
                          (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION) as u32),
                          (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// rule: the Scottish Premier Division (12 clubs) plays 3 rounds (33 matches each) and then the split: 3 points for a
/// win and 1 for a draw, the bottom club goes down to the First (in 1999: 10 clubs, 4 rounds, the bottom club to a
/// relegation play-off); tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5
/// substitutes named and 3 allowed, squad flags 2; builds the round schedule (virtual +0x3c, 38 rounds) and, outside
/// 1999, sets the round count to 33 (the rounds before the split); returns 1
/// Original: `ScoPrm::set_up_rules` (0x7f2970, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_set_up_rules(this: usize) -> u32 {
    let league = League(this);
    covered!("mwb_13 scottish premier rules");
    eng_prm_apply_league_rules(league, &scottish_premier_rules(league));
    if league.year() != 0x7cf {
        write_u16(this + league::ROUND_COUNT, 0x21);
    }
    1
}

/// Comparer for `sco_prm_set_up_rules` (side by side on the rule bytes and, outside 1999, the round count; the copy
/// without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let mut written = scottish_premier_rules(League(this));
    if League(this).year() != 0x7cf {
        written.push((league::ROUND_COUNT, 2, 0x21));
    }
    eng_prm_compare_league_rules("sco_prm_set_up_rules", League(this), written, || o(this), || sco_prm_set_up_rules(this))
}
