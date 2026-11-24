// ---------- cx09_7: the Finnish First Division's own data in the saved game (fin_first.cpp) ----------

/// The Finnish First Division's four stored Second Division club ids (4 bytes each), picked for promotion by
/// FinFirst::create_play_off_stages.
const FIN_FIRST_STORED_CLUBS: usize = 0xee;

/// Reads the Finnish First Division's own data from the saved game: the four stored Second Division club ids (four
/// 4-byte items, `exe::crt_fread`, unchecked). Returns 1.
/// Original: `FinFirst::load_extra_data` (0x594fb0, fin_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fin_first_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx09_7 finnish first division data loaded");
    exe::crt_fread(this + FIN_FIRST_STORED_CLUBS, 4, 4, file);
    1
}

/// Writes the Finnish First Division's own data to the saved game: the four stored Second Division club ids
/// (`exe::crt_fwrite`, unchecked). Returns the write's count.
/// Original: `FinFirst::save_extra_data` (0x594fd0, fin_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fin_first_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx09_7 finnish first division data saved");
    exe::crt_fwrite(this + FIN_FIRST_STORED_CLUBS, 4, 4, file)
}

pub(crate) unsafe extern "thiscall" fn compare_fin_first_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("fin_first_load_extra_data", this, file, &[(FIN_FIRST_STORED_CLUBS, 0x10)], || o(this, file), || fin_first_load_extra_data(this, file))
}

/// Comparer for `fin_first_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_fin_first_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
