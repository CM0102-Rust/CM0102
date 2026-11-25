// ---------- rx09_12: the Swedish Second Division's own data in the saved game (swe_second.cpp) ----------

/// Reads the Swedish Second Division's own data from the saved game: the twelve lower division club ids kept for
/// promotion (+0xee, 12 dwords; `exe::crt_fread`, unchecked). Returns 1.
/// Original: `SweSecond::load_extra_data` (0x8936e0, swe_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn swe_second_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("rx09_12 swedish second data loaded");
    exe::crt_fread(this + 0xee, 4, 0xc, file); // rx09_12: the twelve promoted club ids
    1
}

/// Comparer for `swe_second_load_extra_data` (`compare_cup_extra_data_load`: the read side by side with the file put
/// back).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("swe_second_load_extra_data", this, file, &[(0xee, 0x30)], || o(this, file), || swe_second_load_extra_data(this, file))
}

/// Writes the Swedish Second Division's own data to the saved game: the twelve promoted club ids (+0xee, 12 dwords;
/// `exe::crt_fwrite`, unchecked). Returns the write's count.
/// Original: `SweSecond::save_extra_data` (0x893700, swe_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn swe_second_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("rx09_12 swedish second data saved");
    exe::crt_fwrite(this + 0xee, 4, 0xc, file) // rx09_12: the twelve promoted club ids written
}

/// Comparer for `swe_second_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
