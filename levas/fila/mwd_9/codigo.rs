// ---------- batch %%LEVA%%: the season turnover of the Croatian, Polish and Polish League cups ----------

/// Rule: the Croatian Cup season turnover: last edition's tables freed, its statistics archived, the year + 1, the 48
/// entrants (`cro_cup_collect_entrants`), the rounds, the fixture tree and the predicted positions
/// (`start_new_cup_season`, the template of the knockout cups; lines 0x1a5, 0x1ac, 0x1b3 of cro_cup.cpp).
/// Original: `CroCup::start_new_season` (0x517d60, cro_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x517bb0, 0x9b5468, [0x1a5, 0x1ac, 0x1b3], 1, "mwd_9 croatian cup turnover") // mwd9: the Croatian Cup turnover
}

/// Rule: the Polish Cup season turnover, as the Croatian Cup's, with the Polish Cup entrants (`PolCup::collect_entrants`
/// 0x7c8550; lines 0x1de, 0x1e5, 0x1ec of pol_cup.cpp).
/// Original: `PolCup::start_new_season` (0x7c88a0, pol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7c8550, 0xa78564, [0x1de, 0x1e5, 0x1ec], 1, "mwd_9 polish cup turnover")
}

/// Rule: the Polish League Cup season turnover, as the Croatian Cup's, with its entrants (`PolLgeCup::collect_entrants`
/// 0x7cad30; lines 0x1f3, 0x1fa, 0x201 of pol_lge_cup.cpp).
/// Original: `PolLgeCup::start_new_season` (0x7cafe0, pol_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_lge_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7cad30, 0xa785d4, [0x1f3, 0x1fa, 0x201], 1, "mwd_9 polish league cup turnover")
}

/// Comparer for `cro_cup_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("cro_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `pol_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("pol_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `pol_lge_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("pol_lge_cup_start_new_season", Cup(this), || o(this))
}
