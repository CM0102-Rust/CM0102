// ---------- cx13_10: the Spanish First Division's and the Russian Premier League's own data in the saved game ----------

/// After a saved game is loaded, the Spanish First Division rebuilds its Spanish lower division (`lower_division`):
/// when the nation of its competition does not have the lower divisions loaded (`league_selected` without
/// `league_selected::LOWER_DIVISIONS_LOADED`), a new one (operator new `exe::crt_operator_new` of 0xee and
/// SpaLower::SpaLower `exe::new_spalower` with the year; null when out of memory), otherwise none (0). Nothing is
/// read from the file; returns 1. The original pushes the year as a dword whose high half is not set; SpaLower::SpaLower
/// only reads the word.
/// Original: `SpaFirst::load_extra_data` (0x8508c0, spa_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spa_first_load_extra_data(this: usize, _file: usize) -> u32 {
    let l = League(this);
    let f = SpaFirstFields(this);
    if l.competition().nation().league_selected() & league_selected::LOWER_DIVISIONS_LOADED != 0 {
        f.set_lower_division(0);
        return 1;
    }
    covered!("cx13_10 spanish lower division rebuilt");
    let p = exe::crt_operator_new(0xee);
    let lower = if p != 0 { exe::new_spalower(p, l.year() as u16 as u32) } else { 0 }; // cx13_10: the lower division
    f.set_lower_division(lower);
    1
}

/// Comparer for `spa_first_load_extra_data`: only the original (it builds the lower division); afterwards the lower
/// division exists exactly when the nation lacks the lower divisions, and the result is 1.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_load_extra_data(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    let want = League(this).competition().nation().league_selected() & league_selected::LOWER_DIVISIONS_LOADED == 0;
    let lower = SpaFirstFields(this).lower_division();
    SCOREBOARD.record(r == 1 && (lower != 0) == want, || format!("spa_first_load_extra_data({:#x}) returned {} lower {:#x} expected {}", this, r, lower, want));
    r
}

/// Saves the Spanish First Division's own data: when it has a Spanish lower division (`lower_division`), that one's
/// virtual `comp_vtable::SAVE_TO_TEMP_FILE` with 1 (the file argument is not passed on). Returns what that returns;
/// without a lower division the original returns whatever eax held on entry, and the copy 0.
/// Original: `SpaFirst::save_extra_data` (0x850950, spa_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn spa_first_save_extra_data(this: usize, _file: usize) -> u32 {
    let lower = SpaFirstFields(this).lower_division();
    if lower == 0 {
        return 0;
    }
    covered!("cx13_10 spanish lower division saved");
    let save: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(lower) + comp_vtable::SAVE_TO_TEMP_FILE));
    save(lower, 1) // cx13_10: the lower division's temporary file
}

/// Comparer for `spa_first_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_spa_first_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

/// Writes the Russian Premier League's play-off pair count (the byte +0xee, set by the title play-off) to the saved
/// game (`exe::crt_fwrite` of 1 record of 1 byte) and returns what fwrite returns.
/// Original: `RusPrm::save_extra_data` (0x7ec1f0, rus_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn rus_prm_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx13_10 russian play-off count saved");
    exe::crt_fwrite(this + 0xee, 1, 1, file) // cx13_10: the play-off pair count
}

/// Comparer for `rus_prm_save_extra_data`: only the original (it writes the saved game).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file)
}
