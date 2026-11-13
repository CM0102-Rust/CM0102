// ---------- batch %%LEVA%%: the rules of the Russian Premier League and the Turkish divisions (rus_prm.cpp, tur_first.cpp, tur_second.cpp) ----------

/// The id global of the division below the Turkish top flight (0x9cf8d0, unnamed in the community map:
/// `g_key_club_comp_turkish_unmatched_a`, docs/chaves_uso.csv).
pub(crate) const KEY_CLUB_COMP_TURKISH_UNMATCHED_A: usize = 0x9cf8d0;

/// The rules of the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); nobody goes up, 2 go down (+0xc1) to the First Division
/// (`KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`); +0x52 = 2; 7 substitutes, 3 substitutions; points ties settled by
/// tie-break matches (+0xe6 = 1); the rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn rus_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) as u32), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the two Turkish divisions: 2 legs, 3 points a win and 1 a draw; tie-breaks 4, 1, 2, 3 (head to head,
/// goal difference, goals for, wins); `up` go up directly (+0xbe), none to play-offs, 3 go down (+0xc1); the given
/// leagues above and below; +0x52 = 2; 7 substitutes, 3 substitutions; no rule bits set.
pub(crate) unsafe fn turkish_league_rules(up: u32, above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2),
        (0xc8, 1, 3), (0xbe, 1, up), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, above), (0x4a, 1, 3), (0x52, 1, 2), (0x20, 4, below), (0x49, 1, 7)]
}

/// The Turkish First Division: nobody goes up, nothing above, the division of id [0x9cf8d0] below.
pub(crate) unsafe fn tur_first_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(0, 0xffff_ffff, read_usize(KEY_CLUB_COMP_TURKISH_UNMATCHED_A) as u32)
}

/// The Turkish Second Division: 3 go up to the top flight (`KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION`), the second
/// division category B (`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`) below.
pub(crate) unsafe fn tur_second_rules() -> Vec<(usize, u8, u32)> {
    turkish_league_rules(3, read_usize(KEY_CLUB_COMP_TURKISH_PREMIER_DIVISION) as u32, read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) as u32) // mwd14: three go up
}

/// Rule: the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down; rule bit 0x40
/// (`rus_prm_rules`).
/// Original: `RusPrm::set_up_rules` (0x7eb740, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 russian premier league rules");
    apply_league_rules_with_flags(League(this), 0x40, &rus_prm_rules())
}

/// Rule: the Turkish First Division: 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal difference,
/// goals for, wins; nobody goes up, 3 go down (`turkish_league_rules`).
/// Original: `TurFirst::set_up_rules` (0x8fdf90, tur_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish first division rules");
    eng_prm_apply_league_rules(League(this), &tur_first_rules())
}

/// Rule: the Turkish Second Division: as the First Division, with 3 going up to the top flight and 3 down to the
/// second division category B.
/// Original: `TurSecond::set_up_rules` (0x8ff280, tur_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 turkish second division rules");
    eng_prm_apply_league_rules(League(this), &tur_second_rules())
}

/// Comparer for `rus_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("rus_prm_set_up_rules", League(this), rus_prm_rules(), || o(this), || rus_prm_set_up_rules(this))
}

/// Comparer for `tur_first_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("tur_first_set_up_rules", League(this), tur_first_rules(), || o(this), || tur_first_set_up_rules(this))
}

/// Comparer for `tur_second_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("tur_second_set_up_rules", League(this), tur_second_rules(), || o(this), || tur_second_set_up_rules(this))
}
