// ---------- batch %%LEVA%%: the season turn of the Belgian and Dutch Super Cups and the German League Cup (bel_super.cpp, hol_super.cpp, ger_lge_cup.cpp) ----------

/// At the season turn the Belgian Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (BelSuper::select_teams), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`BelSuper::select_teams`, thiscall, ret 0), the
/// source file and the box lines (0x134, 0x13b, 0x142) change.
/// Original: `BelSuper::start_new_season` (0x421540, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x4212f0, 0x989ccc, [0x134, 0x13b, 0x142], 1, true, "mwc_15 bel super turnover") // mwc15: the Belgian Super Cup
}

/// Comparer for `bel_super_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_bel_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("bel_super_start_new_season", Cup(this), || o(this))
}

/// At the season turn the Dutch Super Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (HolSuper::select_teams), the rounds, the tie tree (no predicted positions).
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`HolSuper::select_teams`, thiscall, ret 0), the
/// source file and the box lines (0x121, 0x128, 0x12f) change; no predicted positions: it returns the eax of the tie
/// tree.
/// Original: `HolSuper::start_new_season` (0x5f8e40, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5f8c10, 0x9cefdc, [0x121, 0x128, 0x12f], 1, false, "mwc_15 hol super turnover") // mwc15: the Dutch Super Cup
}

/// Comparer for `hol_super_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_hol_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("hol_super_start_new_season", Cup(this), || o(this))
}

/// At the season turn the German League Cup is rebuilt: last edition's tables freed, the history archived, the year +1,
/// the entrants (GerLgeCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`GerLgeCup::collect_entrants`, thiscall, ret 0),
/// the source file and the box lines (0x1ac, 0x1b3, 0x1ba) change.
/// Original: `GerLgeCup::start_new_season` (0x5dc9e0, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5dc700, 0x9cd450, [0x1ac, 0x1b3, 0x1ba], 1, true, "mwc_15 ger lge cup turnover") // mwc15: the German League Cup
}

/// Comparer for `ger_lge_cup_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_ger_lge_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ger_lge_cup_start_new_season", Cup(this), || o(this))
}
