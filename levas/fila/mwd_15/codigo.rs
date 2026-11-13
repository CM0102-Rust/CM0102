// ---------- batch %%LEVA%%: the tables of the Greek divisions and the Russian Premier League (gre_prm.cpp, gre_second.cpp, rus_prm.cpp) ----------

/// Rule: the Greek First Division has 14 clubs, those whose national division is it
/// (`KEY_CLUB_COMP_GREEK_PREMIER_DIVISION`), in database order; when full, the prize fund (100,000 a share) and TV
/// pays 500,000 to each club (`eng_prm_fill_league_table`: the table of 14 × 0x3b, the box at line 0x163; League::
/// calc_prize_fund push 100000; League::pay_tv_rights push 0; push 500000).
/// Original: `GrePrm::create_table` (0x5ead60, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdae4, 0x163, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek first division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0) // mwd15: the Greek TV money
    })
}

/// Rule: the Greek Second Division has 14 clubs (`KEY_CLUB_COMP_GREEK_SECOND_DIVISION`); when full, the same prize
/// fund (100,000 a share) and TV money (500,000) as the First Division (the box at line 0x169).
/// Original: `GreSecond::create_table` (0x5ebcb0, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0xe, 0x9cdb1c, 0x169, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || {
        covered!("mwd_15 greek second division tv money");
        eng_prm_pay_tv_rights(this, 0x7a120, 0)
    })
}

/// Rule: the Russian Premier League has 16 clubs (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`); when full, the prize fund
/// (50,000 a share) and TV pays 300,000 to each club (the box at line 0x13c).
/// Original: `RusPrm::create_table` (0x7eb610, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0xa7acc8, 0x13c, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || {
        covered!("mwd_15 russian premier league tv money");
        eng_prm_pay_tv_rights(this, 0x493e0, 0)
    })
}

/// Comparer for `gre_prm_create_table` (only the original runs; the planned clubs and the prize fund checked).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    eng_prm_compare_league_table("gre_prm_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_PREMIER_DIVISION, 0x186a0, || o(this))
}

/// Comparer for `gre_second_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    eng_prm_compare_league_table("gre_second_create_table", League(this), 0xe, KEY_CLUB_COMP_GREEK_SECOND_DIVISION, 0x186a0, || o(this))
}

/// Comparer for `rus_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    eng_prm_compare_league_table("rus_prm_create_table", League(this), 0x10, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || o(this))
}
