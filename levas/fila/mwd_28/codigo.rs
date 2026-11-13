// ---------- batch %%LEVA%%: the rules of the Polish divisions and the Turkish second division B (pol_first.cpp, pol_second.cpp, tur_second_b.cpp) ----------

/// The rules of the Polish First Division (16 clubs in two groups, then a championship and a relegation group): 2 legs,
/// 3 points a win and 1 a draw, league matches, +0xab = +0xdb = 0, national clubs; tie-breaks 4, 1, 2 (head to head,
/// goal difference, goals for; +0xc8 not written); 4 places in the promotion band (+0xbe), none to the play-offs, none
/// down directly (+0xc1 = 0: the relegation goes through the final stage); nothing above, the Second Division below;
/// +0x52 = 0; 7 substitutes, 3 substitutions; the rule bits +0xd9 = 8 (a word).
pub(crate) unsafe fn pol_first_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x52, 1, 0),
        (0x20, 4, read_usize(KEY_CLUB_COMP_POLISH_SECOND_DIVISION) as u32), (0x49, 1, 7), (0x4a, 1, 3), (0xd9, 2, 8)]
}

/// The rules of the Polish Second Division: as the First Division's, with 2 up directly (+0xbe) and 2 to the
/// promotion play-off (+0xbf), 6 down in 2001 (20 clubs) and 4 afterwards (+0xc1); the First Division above, nothing
/// below; +0xd9 = 3.
pub(crate) unsafe fn pol_second_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let down = if League(this).year() == 0x7d1 { 6 } else { 4 }; // mwd28: six down in 2001
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xbe, 1, 2), (0xbf, 1, 2), (0xc0, 1, 0), (0xc1, 1, down), (0x52, 1, 0), (0x1c, 4, read_usize(KEY_CLUB_COMP_POLISH_FIRST_DIVISION) as u32),
        (0x4a, 1, 3), (0xd9, 2, 3), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)]
}

/// The rules of the Turkish second division category B (five regional groups, then promotion and classification
/// groups): 2 legs, 3 points a win and 1 a draw, league matches, national clubs; tie-breaks 1, 2 (goal difference,
/// goals for; +0xc7 = 0 ends); none up directly, 2 to the play-offs (+0xbf), none down; category A above, nothing
/// below; 7 substitutes, 3 substitutions; +0xd9 = 2 (a word). +0xab, +0xdb, +0xc8 and +0x52 are not written.
pub(crate) unsafe fn tur_second_b_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 0), (0xbe, 1, 0), (0xbf, 1, 2),
        (0xc0, 1, 0), (0xc1, 1, 0), (0x4a, 1, 3), (0xd9, 2, 2), (0x1c, 4, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_A) as u32),
        (0x20, 4, 0xffff_ffff), (0x49, 1, 7)]
}

/// Rule: the Polish First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal
/// difference, goals for; 4 in the promotion band, nobody down directly; the Second Division below (`pol_first_rules`).
/// Original: `PolFirst::set_up_rules` (0x7c9690, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 polish first division rules");
    eng_prm_apply_league_rules(League(this), &pol_first_rules())
}

/// Rule: the Polish Second Division: 2 up directly and 2 to the promotion play-off, 6 down in 2001 and 4 afterwards
/// (`pol_second_rules`).
/// Original: `PolSecond::set_up_rules` (0x7cbf40, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 polish second division rules");
    eng_prm_apply_league_rules(League(this), &pol_second_rules(this))
}

/// Rule: the Turkish second division category B: 3 points a win and 1 a draw; tie-breaks goal difference, goals for; 2
/// to the play-offs; category A above (`tur_second_b_rules`).
/// Original: `TurSecondB::set_up_rules` (0x900460, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_up_rules(this: usize) -> u32 {
    covered!("mwd_28 turkish category b rules");
    eng_prm_apply_league_rules(League(this), &tur_second_b_rules())
}

/// Comparer for `pol_first_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_pol_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("pol_first_set_up_rules", League(this), pol_first_rules(), || o(this), || pol_first_set_up_rules(this))
}

/// Comparer for `pol_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("pol_second_set_up_rules", League(this), pol_second_rules(this), || o(this), || pol_second_set_up_rules(this))
}

/// Comparer for `tur_second_b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("tur_second_b_set_up_rules", League(this), tur_second_b_rules(), || o(this), || tur_second_b_set_up_rules(this))
}
