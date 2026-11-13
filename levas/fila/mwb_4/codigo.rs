// ---------- mwb_4: the season turnover of the Scottish Challenge Cup, the Scottish Cup and the Welsh Cup (sco_chal_cup.cpp, sco_fa_cup.cpp, wel_cup.cpp) ----------

/// The Challenge Cup turnover: frees last season's teams, rounds and ties, archives the statistics, moves to the next
/// year (+1) and rebuilds the entrants (`ScoChalCup::collect_entrants`), the rounds (virtual +0x8c) and the fixture tree
/// (virtual +0x94), each with its error box (lines 0x180, 0x187, 0x18e of sco_chal_cup.cpp) when it returns negative;
/// then the predicted positions (virtual +0x5c), whose eax it returns (`start_new_cup_season`: the same template as the
/// Brazilian Cup's, checked instruction by instruction).
/// Original: `ScoChalCup::start_new_season` (0x7ed130, sco_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_chal_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7ed010, 0xa7ad70, [0x180, 0x187, 0x18e], 1, "mwb_4 challenge cup turnover") // mwb4: the next Challenge Cup
}

/// Comparer for `sco_chal_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_chal_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_cup_new_season("sco_chal_cup_start_new_season", Cup(this), || o(this))
}

/// The Scottish Cup turnover: the same template, with the Scottish Cup entrants (`ScoFaCup::collect_entrants`) and the
/// error-box lines 0x20f, 0x216 and 0x21d of sco_fa_cup.cpp.
/// Original: `ScoFaCup::start_new_season` (0x7ede10, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7edaf0, 0xa7adac, [0x20f, 0x216, 0x21d], 1, "mwb_4 scottish cup turnover") // mwb4: the next Scottish Cup
}

/// Comparer for `sco_fa_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("sco_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The Welsh Cup turnover: the same template, with the Welsh Cup entrants (`WelCup::collect_entrants`) and the
/// error-box lines 0x17b, 0x182 and 0x189 of wel_cup.cpp.
/// Original: `WelCup::start_new_season` (0x924c50, wel_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x924ab0, 0xad5684, [0x17b, 0x182, 0x189], 1, "mwb_4 welsh cup turnover") // mwb4: the next Welsh Cup
}

/// Comparer for `wel_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("wel_cup_start_new_season", Cup(this), || o(this))
}
