// ---------- cx02_15: the group cups' entrant list in the saved game (ita_cup.cpp, kor_league_cup.cpp, nir_lge_cup.cpp) ----------

/// The Coppa Italia entrant list (`load_group_cup_entrants`, ita_cup.cpp lines 0x3cb, 0x3d3 and 0x3db).
/// Original: `ItaCup::load_extra_data` (0x640050, ita_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9ea23c, (0x3cb, 0x3d3, 0x3db), false)
}

/// The Korean League Cup entrant list (`load_group_cup_entrants`, kor_league_cup.cpp lines 0x320, 0x328 and 0x330).
/// Original: `KorLeagueCup::load_extra_data` (0x66d1e0, kor_league_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9eb15c, (0x320, 0x328, 0x330), false)
}

/// The Northern Irish League Cup entrant list (`load_group_cup_entrants`, nir_lge_cup.cpp lines 0x34f, 0x357 and
/// 0x35f).
/// Original: `NirLgeCup::load_extra_data` (0x78db10, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0xa69698, (0x34f, 0x357, 0x35f), false)
}

pub(crate) unsafe extern "thiscall" fn compare_ita_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_group_cup_entrants_load("ita_cup_load_extra_data", this, file, || o(this, file), || ita_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_group_cup_entrants_load("kor_league_cup_load_extra_data", this, file, || o(this, file), || kor_league_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_group_cup_entrants_load("nir_lge_cup_load_extra_data", this, file, || o(this, file), || nir_lge_cup_load_extra_data(this, file))
}
