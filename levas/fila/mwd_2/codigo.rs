// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the rules ----------

/// Sets the bits `flags` in the rule bits (`league::LEAGUE_FLAGS`, byte +0xd9: `or byte ptr [esi + 0xd9], flags`), then
/// writes the (offset, size, value) rules and builds the main stage round table (`eng_prm_apply_league_rules`: virtual
/// +0x3c(−1, &+0xa9, &+0x3a, 0) into +0xba, skipped in the comparer's rehearsal). Returns 1.
pub(crate) unsafe fn apply_league_rules_with_flags(this: League, flags: u8, written: &[(usize, u8, u32)]) -> u32 {
    write_u8(this.addr() + league::LEAGUE_FLAGS, read_u8(this.addr() + league::LEAGUE_FLAGS) | flags);
    eng_prm_apply_league_rules(this, written)
}

/// The rules of the three Croatian leagues (16 clubs each), given the ids of the league above (+0x1c) and below
/// (+0x20).
///
/// 2 legs (+0x3c), 3 points a win (+0xc2) and 1 a draw (+0xc3), league matches (+0xc4 = 2), +0xab = 0 and +0xdb = 0
/// (words), national clubs (+0x42 = 1), tie-breaks 4, 1, 2 (+0xc5..+0xc7: head to head, goal difference, goals for;
/// +0xc8 = 0 ends), nobody goes up directly nor to the play-offs (+0xbe, +0xbf, +0xc0 = 0), 2 go down (+0xc1), +0x52 =
/// 2, 3 substitutions (+0x4a) of 7 substitutes (+0x49) and points ties settled by tie-break matches (+0xe6 = 1); the
/// rule bit 0x40 is set separately (`apply_league_rules_with_flags`). The list also carries +0xd9 for the comparer.
pub(crate) unsafe fn croatian_league_rules(above: u32, below: u32) -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), // mwd2: the Croatian points
        (0xc7, 1, 2), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, above), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, below), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// The rules of the Croatian First Division: nothing above, the Second Division north group
/// (`KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`) below.
pub(crate) unsafe fn cro_a1_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(0xffff_ffff, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32)
}

/// The rules of the Croatian Second Division north group: the First Division above, the south group below.
pub(crate) unsafe fn cro_a2a_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4) as u32, read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH) as u32)
}

/// The rules of the Croatian Second Division south group: the north group above (as the original: the south group's
/// upper league is the north group, not the First Division), nothing below.
pub(crate) unsafe fn cro_a2b_rules() -> Vec<(usize, u8, u32)> {
    croatian_league_rules(read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) as u32, 0xffff_ffff)
}

/// Rule: the Croatian First Division: 16 clubs, 2 legs, 3 points a win and 1 a draw; tie-breaks head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down to the Second
/// Division (north group); 7 substitutes, 3 substitutions; rule bit 0x40 (`croatian_league_rules`).
/// Original: `CroA1::set_up_rules` (0x5148e0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian first division rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a1_rules())
}

/// Rule: the Croatian Second Division north group: as the First Division, with the First Division above and the south
/// group below.
/// Original: `CroA2a::set_up_rules` (0x515de0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division north rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2a_rules())
}

/// Rule: the Croatian Second Division south group: as the First Division, with the north group above and nothing
/// below.
/// Original: `CroA2b::set_up_rules` (0x516f00, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_set_up_rules(this: usize) -> u32 {
    covered!("mwd_2 croatian second division south rules");
    apply_league_rules_with_flags(League(this), 0x40, &cro_a2b_rules())
}

/// Comparer for `cro_a1_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_rules("cro_a1_set_up_rules", League(this), cro_a1_rules(), || o(this), || cro_a1_set_up_rules(this))
}

/// Comparer for `cro_a2a_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_rules("cro_a2a_set_up_rules", League(this), cro_a2a_rules(), || o(this), || cro_a2a_set_up_rules(this))
}

/// Comparer for `cro_a2b_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_rules("cro_a2b_set_up_rules", League(this), cro_a2b_rules(), || o(this), || cro_a2b_set_up_rules(this))
}
