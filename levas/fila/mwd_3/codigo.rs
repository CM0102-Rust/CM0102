// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2a.cpp, cro_a2b.cpp): the tables ----------

/// Rule: the Croatian First Division has 16 clubs, those whose national division is it, in database order; when full,
/// the prize fund (30,000 a share) and TV pays 100,000 to each club.
///
/// `club_count` = 16; the table (`exe::crt_malloc` of 16 × 0x3b; out of memory, the box at line 0x145 and 0); the
/// clubs of the competition of id `KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4` until full
/// (`eng_prm_fill_league_table`); full: the prize fund (League::calc_prize_fund, push 30000 -> thiscall, ret 4),
/// `prize_mode` = 1 and the TV money (League::pay_tv_rights `exe::league_pay_tv_rights`: push 0; push 100000 ->
/// thiscall, ret 8) and 1; otherwise 0.
/// Original: `CroA1::create_table` (0x5147b0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53c0, 0x145, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || {
        covered!("mwd_3 croatian first division tv money");
        eng_prm_pay_tv_rights(this, 0x186a0, 0) // mwd3: the First Division TV money
    })
}

/// Rule: the Croatian Second Division north group has 16 clubs (its own, in database order); when full, the prize fund
/// (10,000 a share) and TV pays 20,000 to each club. As `cro_a1_create_table`, with the competition
/// `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`.
/// Original: `CroA2a::create_table` (0x515cb0, cro_a2a.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2a_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b53f8, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || {
        covered!("mwd_3 croatian second division north tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Rule: the Croatian Second Division south group, as the north group (16 clubs, 10,000 a share, 20,000 TV money),
/// with the competition `KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH`.
/// Original: `CroA2b::create_table` (0x516dd0, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0x9b5430, 0x145, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || {
        covered!("mwd_3 croatian second division south tv money");
        eng_prm_pay_tv_rights(this, 0x4e20, 0)
    })
}

/// Comparer for `cro_a1_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_table("cro_a1_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4, 0x7530, || o(this))
}

/// Comparer for `cro_a2a_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2a_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_table("cro_a2a_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, 0x2710, || o(this))
}

/// Comparer for `cro_a2b_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_table("cro_a2b_create_table", League(this), 0x10, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH, 0x2710, || o(this))
}
