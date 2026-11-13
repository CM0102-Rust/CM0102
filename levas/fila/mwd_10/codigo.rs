// ---------- batch %%LEVA%%: the season turnover of the Russian and Turkish cups and the Greek Super Cup ----------

/// Rule: the Russian Cup season turnover: last edition's tables freed, its statistics archived, the year + 1, the 48
/// entrants (`RusCup::select_entrants` 0x7e99a0), the rounds, the fixture tree and the predicted positions
/// (`start_new_cup_season`; lines 0x1ce, 0x1d5, 0x1dc of rus_cup.cpp).
/// Original: `RusCup::start_new_season` (0x7e9d10, rus_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7e99a0, 0xa7ac58, [0x1ce, 0x1d5, 0x1dc], 1, "mwd_10 russian cup turnover")
}

/// Rule: the Turkish Cup season turnover, as the Russian Cup's, with its 64 entrants (`TurCup::collect_entrants`
/// 0x8fcb90; lines 0x200, 0x207, 0x20e of tur_cup.cpp).
/// Original: `TurCup::start_new_season` (0x8fcdb0, tur_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x8fcb90, 0xaa180c, [0x200, 0x207, 0x20e], 1, "mwd_10 turkish cup turnover") // mwd10: the Turkish Cup turnover
}

/// Rule: the Greek Super Cup season turnover: the finished edition's tables freed, its statistics archived, the year
/// + 1, the two clubs (`GreSuper::select_teams` 0x5ec430: the league champion and the cup winner), the round and the
/// fixture tree; no predicted positions (`usa_mls_all_stars_cup_start_new_season`; lines 0x136, 0x13d, 0x144 of
/// gre_super.cpp).
/// Original: `GreSuper::start_new_season` (0x5ec6b0, gre_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5ec430, 0x9cdb58, [0x136, 0x13d, 0x144], 1, false, "mwd_10 greek super cup turnover")
}

/// Comparer for `rus_cup_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("rus_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `tur_cup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_tur_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("tur_cup_start_new_season", Cup(this), || o(this))
}

/// Comparer for `gre_super_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("gre_super_start_new_season", Cup(this), || o(this))
}
