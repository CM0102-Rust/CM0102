// ---------- cx09_6: the own data of the Belgian Third Division and the Croatian First Division in the saved game (bel_third.cpp, cro_a1.cpp) ----------

/// The Belgian Third Division's three period flag pairs (2 bytes each, read and written one byte at a time) and its
/// four stored club ids (the Fourth Division clubs kept for promotion, 4 bytes each).
const BEL_THIRD_PERIOD_FLAGS: [usize; 3] = [0xfe, 0x100, 0x102];
const BEL_THIRD_STORED_CLUBS: usize = 0xee;
/// The Croatian First Division's play-off pair count (1 byte).
const CRO_A1_PLAYOFF_PAIRS: usize = 0xee;

/// Reads the Belgian Third Division's own data from the saved game (`exe::crt_fread`, unchecked): the three period
/// flag pairs (two 1-byte items each) and the four stored club ids (four 4-byte items). Returns 1.
/// Original: `BelThird::load_extra_data` (0x424220, bel_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_third_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx09_6 belgian third division data loaded");
    for flags in BEL_THIRD_PERIOD_FLAGS {
        exe::crt_fread(this + flags, 1, 2, file);
    }
    exe::crt_fread(this + BEL_THIRD_STORED_CLUBS, 4, 4, file);
    1
}

/// Writes the Belgian Third Division's own data to the saved game (`exe::crt_fwrite`, unchecked), in the order the
/// load reads it: the three period flag pairs and the four stored club ids. Returns the last write's count.
/// Original: `BelThird::save_extra_data` (0x424280, bel_third.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_third_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx09_6 belgian third division data saved");
    for flags in BEL_THIRD_PERIOD_FLAGS {
        exe::crt_fwrite(this + flags, 1, 2, file);
    }
    exe::crt_fwrite(this + BEL_THIRD_STORED_CLUBS, 4, 4, file)
}

/// Reads the Croatian First Division's own data from the saved game: the play-off pair count (one byte,
/// `exe::crt_fread`, unchecked). Returns 1.
/// Original: `CroA1::load_extra_data` (0x5152f0, cro_a1.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cro_a1_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx09_6 croatian first division data loaded");
    exe::crt_fread(this + CRO_A1_PLAYOFF_PAIRS, 1, 1, file);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_bel_third_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("bel_third_load_extra_data", this, file, &[(BEL_THIRD_STORED_CLUBS, 0x16)], || o(this, file), || bel_third_load_extra_data(this, file))
}

/// Comparer for `bel_third_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}

pub(crate) unsafe extern "thiscall" fn compare_cro_a1_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    compare_cup_extra_data_load("cro_a1_load_extra_data", this, file, &[(CRO_A1_PLAYOFF_PAIRS, 1)], || o(this, file), || cro_a1_load_extra_data(this, file))
}
