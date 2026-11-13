// ---------- batch %%LEVA%%: the season turn of the Belgian, Danish and German cups (bel_fa_cup.cpp, den_cup.cpp, ger_cup.cpp) ----------

/// At the season turn the Belgian Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (BelFaCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`BelFaCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x216, 0x21d, 0x224) change.
/// Original: `BelFaCup::start_new_season` (0x41db90, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x41d8d0, 0x989ae0, [0x216, 0x21d, 0x224], 1, true, "mwc_14 bel fa cup turnover") // mwc14: the Belgian Cup
}

/// Comparer for `bel_fa_cup_start_new_season`: only the original runs; the year goes up by one
/// (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_bel_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("bel_fa_cup_start_new_season", Cup(this), || o(this))
}

/// At the season turn the Danish Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (DenCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`DenCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x1fe, 0x205, 0x20c) change.
/// Original: `DenCup::start_new_season` (0x551500, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x551240, 0x9b8dfc, [0x1fe, 0x205, 0x20c], 1, true, "mwc_14 den cup turnover") // mwc14: the Danish Cup
}

/// Comparer for `den_cup_start_new_season`: only the original runs; the year goes up by one (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_den_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("den_cup_start_new_season", Cup(this), || o(this))
}

/// At the season turn the German Cup is rebuilt: last edition's tables freed, the history archived, the year +1, the
/// entrants (GerCup::collect_entrants), the rounds, the tie tree and the predicted positions.
///
/// The knockout-cup turnover template (`usa_mls_all_stars_cup_start_new_season`), checked instruction by instruction
/// against `JapSuper::start_new_season`: only the entrants routine (`GerCup::collect_entrants`, thiscall, ret 0), the
/// source file and the box lines (0x220, 0x227, 0x22e) change.
/// Original: `GerCup::start_new_season` (0x5d9780, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x5d9370, 0x9cd3e0, [0x220, 0x227, 0x22e], 1, true, "mwc_14 ger cup turnover") // mwc14: the German Cup
}

/// Comparer for `ger_cup_start_new_season`: only the original runs; the year goes up by one (`compare_cup_new_season`).
unsafe extern "thiscall" fn compare_ger_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ger_cup_start_new_season", Cup(this), || o(this))
}
