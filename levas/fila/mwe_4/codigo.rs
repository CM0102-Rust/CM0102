// ---------- batch %%LEVA%%: the league rules of the Swedish First Division and of the Portuguese Premier and Second Divisions (swe_first.cpp, por_prm.cpp, por_second.cpp) ----------

/// The division below when the nation has its lower leagues playable: the nation of the league's competition
/// (`competition::NATION`) with bit 4 of `.league_selected()` gives the id of global `below`, otherwise −1.
pub(crate) unsafe fn lower_division_when_playable(this: usize, below: usize) -> u32 {
    let nation = Nation(read_usize(read_usize(this + league::COMPETITION) + competition::NATION));
    if nation.league_selected() & 4 != 0 { read_usize(below) as u32 } else { 0xffff_ffff }
}

/// The rule writes of the Swedish First Division (Superettan, 16 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks goal difference and goals for (the third code 0 ends the list); 2 promoted directly, 1 to the
/// promotion play-off (the 3rd, against the Allsvenskan's 12th), no relegation play-off and 3 relegated; the
/// Allsvenskan above and the Second Division below when the Swedish lower leagues are playable (bit 4), otherwise −1;
/// 3 substitutions, the squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn swedish_first_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 2), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    v.push((league::RELEGATED_COUNT, 1, 3)); // mwe4: the Superettan relegates 3
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, lower_division_when_playable(this, KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION))]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// The rule writes of the Portuguese Premier Division (18 clubs), in the original's order.
///
/// Two legs, 3 points for a win and 1 for a draw, league matches, no match rule flags and no round draw rules, club
/// sides; tie-breaks head-to-head (code 4), goal difference, goals for and code 3; nobody goes up, no play-offs, 3
/// relegated; no division above, 3 substitutions, the squad flags 2, the Second Division below and 7 substitutes named.
pub(crate) unsafe fn portuguese_premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1)];
    v.push((league::TIEBREAKS, 1, 4)); // mwe4: the Portuguese Premier breaks ties head to head first
    v.extend_from_slice(&[(league::TIEBREAKS + 1, 1, 1), (league::TIEBREAKS + 2, 1, 2), (league::TIEBREAKS + 3, 1, 3)]);
    v.extend_from_slice(&[(league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 3)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)]);
    v.extend_from_slice(&[(league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION) as u32), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// The rule writes of the Portuguese Second Division (the Liga de Honra, 18 clubs), in the original's order.
///
/// Two legs, 3 points for a win, 34 rounds written beforehand in `league::ROUND_COUNT` (the round table then writes
/// its own count there), 1 point for a draw, league matches, club sides; tie-breaks head-to-head, goal difference,
/// goals for and code 3; 3 promoted, no play-offs, 3 relegated; the Premier Division above and the Second Division B
/// below when the Portuguese lower leagues are playable (bit 4), otherwise −1; 3 substitutions, the squad flags 2 and 7
/// substitutes named. It does not touch the match rule flags nor the round draw rules.
pub(crate) unsafe fn portuguese_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 2), (league::WIN_POINTS, 1, 3), (league::ROUND_COUNT, 2, 0x22), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1)];
    v.extend_from_slice(&[(league::TIEBREAKS, 1, 4), (league::TIEBREAKS + 1, 1, 1), (league::TIEBREAKS + 2, 1, 2), (league::TIEBREAKS + 3, 1, 3)]);
    v.push((league::PROMOTED_COUNT, 1, 3)); // mwe4: the Liga de Honra promotes 3
    v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 3)]);
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_PORTUGUESE_PREMIER_DIVISION) as u32), (league::LOWER_DIVISION_ID, 4, lower_division_when_playable(this, KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B))]);
    v.extend_from_slice(&[(league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 7)]);
    v
}

/// Sets up the rules of the Swedish First Division (Superettan, 16 clubs): two legs, 3 points for a win, tie-breaks
/// goal difference and goals for; the top 2 go up, the 3rd plays the promotion play-off against the Allsvenskan's 12th
/// and the bottom 3 go down (to the Second Division when it is playable); 5 substitutes and 3 substitutions.
///
/// `SweFirst::set_up_rules()`: the writes of `swedish_first_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`: push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `SweFirst::set_up_rules` (0x88f090, swe_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_first_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 superettan rules");
    eng_prm_apply_league_rules(League(this), &swedish_first_rules(this))
}

/// Sets up the rules of the Portuguese Premier Division (18 clubs): two legs, 3 points for a win, ties broken head to
/// head first, then goal difference and goals for; nobody goes up and the bottom 3 go down to the Liga de Honra; 7
/// substitutes and 3 substitutions.
///
/// `PorPrm::set_up_rules()`: the writes of `portuguese_premier_rules` and the main-stage round table (virtual
/// `comp_vtable::CREATE_SCHEDULE`) into `league::ROUND_DATES`. Returns 1.
/// Original: `PorPrm::set_up_rules` (0x7cf930, por_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 portuguese premier rules");
    eng_prm_apply_league_rules(League(this), &portuguese_premier_rules())
}

/// Sets up the rules of the Portuguese Second Division (the Liga de Honra, 18 clubs): two legs, 3 points for a win,
/// ties broken head to head first; the top 3 go up and the bottom 3 go down (to the Second Division B when it is
/// playable); 7 substitutes and 3 substitutions.
///
/// `PorSecond::set_up_rules()`: the writes of `portuguese_second_rules` (with 34 written into `league::ROUND_COUNT`
/// before the round table) and the main-stage round table (virtual `comp_vtable::CREATE_SCHEDULE`) into
/// `league::ROUND_DATES`. Returns 1.
/// Original: `PorSecond::set_up_rules` (0x7d16a0, por_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn por_second_set_up_rules(this: usize) -> u32 {
    covered!("mwe_4 liga de honra rules");
    eng_prm_apply_league_rules(League(this), &portuguese_second_rules(this))
}

/// Comparer for `swe_first_set_up_rules`: the dry run of `eng_prm_compare_league_rules`.
pub(crate) unsafe extern "thiscall" fn compare_swe_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("swe_first_set_up_rules", League(this), swedish_first_rules(this), || o(this), || swe_first_set_up_rules(this))
}

/// Comparer for `por_prm_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("por_prm_set_up_rules", League(this), portuguese_premier_rules(), || o(this), || por_prm_set_up_rules(this))
}

/// Comparer for `por_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_por_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("por_second_set_up_rules", League(this), portuguese_second_rules(this), || o(this), || por_second_set_up_rules(this))
}
