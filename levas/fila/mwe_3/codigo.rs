// ---------- batch %%LEVA%%: the league rules of the Norwegian Premier and First Divisions and of the Swedish Premier Division (nor_prm.cpp, nor_first.cpp, swe_prm.cpp) ----------

/// The rule writes of the Norwegian Premier Division (Tippeligaen, 14 clubs), in the original's order.
///
/// Two legs (`league::LEGS`), 3 points for a win and 1 for a draw, league matches (category 2), no match rule flags
/// and no round draw rules, club sides; tie-breaks goal difference, goals for and code 3; nobody goes up (no division
/// above: −1), no promotion play-off, 1 club to the relegation play-off against the First Division (the 12th) and 2
/// relegated directly; the First Division below; 3 substitutions, the squad flags 2 and 7 substitutes named.
pub(crate) unsafe fn norwegian_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATION_PLAYOFF_COUNT, 1, 1)); // mwe3: the Norwegian Premier relegation play-off place
    v.push((league::RELEGATED_COUNT, 1, 2)); // mwe3: the Norwegian Premier relegates 2
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The rule writes of the Norwegian First Division (Adeccoligaen, 16 clubs; 14 when the database has only 14), in the
/// original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, club sides; tie-breaks goal difference, goals for and
/// code 3; 2 promoted directly and 1 to the promotion play-off (the 3rd against the Premier Division's 12th), no
/// relegation play-off and 4 relegated (no division below: −1); the Premier Division above; 3 substitutions, the squad
/// flags 2 and 7 substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn norwegian_first_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 2)); // mwe3: the Norwegian First Division promotes 2
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 4)]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_NORWEGIAN_PREMIER_DIVISION) as u32), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The rule writes of the Swedish Premier Division (Allsvenskan, 14 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks goal difference and goals for (the third code 0 ends the list); nobody goes up, no promotion
/// play-off, 1 club to the relegation play-off (the 12th, against the Superettan's 3rd) and 2 relegated; the First
/// Division below; the squad flags 2, 3 substitutions and 5 substitutes named.
pub(crate) unsafe fn swedish_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2)];
    v.push((league::WIN_POINTS, 1, 3)); // mwe3: the Allsvenskan gives 3 points for a win
    v.extend_from_slice(&[(league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)]);
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 2)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// Sets up the rules of the Norwegian Premier Division (14 clubs): two legs, 3 points for a win; nobody goes up, the
/// 12th goes to the relegation play-off and the bottom 2 go down to the First Division; 7 substitutes and 3
/// substitutions.
///
/// `NorPrm::set_up_rules()`: the writes of `norwegian_premier_rules` (legs, points, category, flags, tie-breaks 1, 2,
/// 3, the bands 0 / 0 / 1 / 2, no division above, the First Division below, substitutions, squad flags 2 and
/// substitutes) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push
/// &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
/// Original: `NorPrm::set_up_rules` (0x792700, nor_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian premier rules");
    eng_prm_apply_league_rules(League(this), &norwegian_premier_rules())
}

/// Sets up the rules of the Norwegian First Division (16 clubs): two legs, 3 points for a win; the top 2 go up, the
/// 3rd plays the promotion play-off against the Premier Division's 12th and the bottom 4 go down; 7 substitutes and 3
/// substitutions.
///
/// `NorFirst::set_up_rules()`: the writes of `norwegian_first_rules` (the bands 2 / 1 / 0 / 4, the Premier Division
/// above, nothing below) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `NorFirst::set_up_rules` (0x790b00, nor_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 norwegian first division rules");
    eng_prm_apply_league_rules(League(this), &norwegian_first_rules())
}

/// Sets up the rules of the Swedish Premier Division (Allsvenskan, 14 clubs): two legs, 3 points for a win, tie-breaks
/// goal difference and goals for; nobody goes up, the 12th plays the relegation play-off and the bottom 2 go down to
/// the Superettan; 5 substitutes and 3 substitutions.
///
/// `SwePrm::set_up_rules()`: the writes of `swedish_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `SwePrm::set_up_rules` (0x890550, swe_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_3 swedish premier rules");
    eng_prm_apply_league_rules(League(this), &swedish_premier_rules())
}

/// Comparer for `nor_prm_set_up_rules`: the dry run of `eng_prm_compare_league_rules` (the original builds the round
/// table; the copy writes the rules only).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("nor_prm_set_up_rules", League(this), norwegian_premier_rules(), || o(this), || nor_prm_set_up_rules(this))
}

/// Comparer for `nor_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("nor_first_set_up_rules", League(this), norwegian_first_rules(), || o(this), || nor_first_set_up_rules(this))
}

/// Comparer for `swe_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("swe_prm_set_up_rules", League(this), swedish_premier_rules(), || o(this), || swe_prm_set_up_rules(this))
}
