// ---------- cx10_13: the Irish Premier Division's and the Dutch First Division's own data in the saved game ----------

/// Reads the Irish Premier Division's fixed draw slot table from the saved game: 10 records of 5 bytes at +0xee
/// (`exe::crt_fread`); true (1) when all ten arrive.
/// Original: `IrePrm::load_extra_data` (0x63ac80, ire_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_prm_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx10_13 irish draw slots loaded");
    (exe::crt_fread(this + 0xee, 5, 10, file) == 10) as u32 // the fixed draw slot table
}

/// Writes the Irish Premier Division's fixed draw slot table to the saved game (10 records of 5 bytes at +0xee,
/// `exe::crt_fwrite`) and returns what fwrite returns.
/// Original: `IrePrm::save_extra_data` (0x63acb0, ire_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_prm_save_extra_data(this: usize, file: usize) -> u32 {
    exe::crt_fwrite(this + 0xee, 5, 10, file) as u32 // the fixed draw slot table
}

/// Reads the Dutch First Division's four period winner flags (+0xee to +0xf1) from the saved game, one byte each
/// (`exe::crt_fread`, unchecked); returns 1.
/// Original: `HolFirst::load_extra_data` (0x5f7550, hol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_first_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx10_13 dutch period winners loaded");
    for k in 0..4 {
        exe::crt_fread(this + 0xee + k, 1, 1, file); // the period winner flag of period k
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_ire_prm_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("ire_prm_load_extra_data", this, file, &[(0xee, 50)], || o(this, file), || ire_prm_load_extra_data(this, file))
}

/// Comparer for `ire_prm_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_ire_prm_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_hol_first_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_cup_extra_data_load("hol_first_load_extra_data", this, file, &[(0xee, 4)], || o(this, file), || hol_first_load_extra_data(this, file))
}
