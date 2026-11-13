// ---------- batch %%LEVA%%: the rules of the Greek divisions and the Russian First Division (gre_prm.cpp, gre_second.cpp, rus_first.cpp) ----------

/// The rules of the Greek First Division (16 clubs): 2 legs, 3 points a win and 1 a draw, league matches (+0xc4 = 2),
/// +0xab = +0xdb = 0, national clubs; tie-breaks 1, 2, 3 (goal difference, goals for, wins; +0xc8 = 0 ends); nobody
/// goes up nor to the play-offs, 2 go down (+0xc1) to the Second Division (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`,
/// +0x20); nothing above; +0x52 = 2; 7 substitutes, 3 substitutions; games per club not computed yet (+0xa7 = −1); the
/// rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn gre_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_GREEK_SECOND_DIVISION) as u32), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Greek Second Division: as the First Division, with 2 going up (+0xbe) and 2 down, the First
/// Division above (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`, +0x1c) and nothing below.
pub(crate) unsafe fn gre_second_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3),
        (0xc8, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xbe, 1, 2), (0xc1, 1, 2), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_GREEK_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); 2 go up (+0xbe), 3 go down (+0xc1); the Premier League above
/// (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`), nothing below; +0x52 = 2; 7 substitutes, 3 substitutions; +0xa7 = −1;
/// no rule bits set.
pub(crate) unsafe fn rus_first_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x4a, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION) as u32),
        (0x52, 1, 2), (0x20, 4, 0xffff_ffff), (0x49, 1, 7), (0xa7, 2, 0xffff)]
}

/// Rule: the Greek First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks goal difference, goals
/// for, wins; 2 go down to the Second Division; 7 substitutes, 3 substitutions; rule bit 0x40 (`gre_prm_rules`).
/// Original: `GrePrm::set_up_rules` (0x5eae90, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_prm_rules())
}

/// Rule: the Greek Second Division: as the First Division, 2 go up and 2 go down (`gre_second_rules`).
/// Original: `GreSecond::set_up_rules` (0x5ebde0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 greek second division rules");
    apply_league_rules_with_flags(League(this), 0x40, &gre_second_rules())
}

/// Rule: the Russian First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; 2 go up and 3 go down (`rus_first_rules`).
/// Original: `RusFirst::set_up_rules` (0x7ea810, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_13 russian first division rules");
    eng_prm_apply_league_rules(League(this), &rus_first_rules())
}

/// Comparer for `gre_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("gre_prm_set_up_rules", League(this), gre_prm_rules(), || o(this), || gre_prm_set_up_rules(this))
}

/// Comparer for `gre_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("gre_second_set_up_rules", League(this), gre_second_rules(), || o(this), || gre_second_set_up_rules(this))
}

/// Comparer for `rus_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_rus_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("rus_first_set_up_rules", League(this), rus_first_rules(), || o(this), || rus_first_set_up_rules(this))
}
