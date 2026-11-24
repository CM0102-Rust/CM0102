// ---------- cx08_19: the Belgian Second Division's own data in the saved game (bel_second.cpp) ----------

/// The Belgian Second Division's own data in the temporary file, in the file's order: the three period winner flags
/// (`BEL_SECOND_PERIOD_WON` + 0, 1, 2; a byte each) and the two dwords at +0xee [inference: club ids kept for the
/// play-off].
pub(crate) const BEL_SECOND_EXTRA_DATA: [(usize, u32, u32); 4] = [(BEL_SECOND_PERIOD_WON, 1, 1), (BEL_SECOND_PERIOD_WON + 1, 1, 1), (BEL_SECOND_PERIOD_WON + 2, 1, 1), (0xee, 4, 2)];

/// Rule: the Belgian Second Division reads its three period winner flags and two dwords back from the saved game.
///
/// `BelSecond::load_extra_data(file)`: `exe::crt_fread` of each part of `BEL_SECOND_EXTRA_DATA` (no read is checked).
/// Returns 1.
/// Original: `BelSecond::load_extra_data` (0x420e30, bel_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_second_load_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx08_19 belgian second division data loaded");
    for (offset, size, count) in BEL_SECOND_EXTRA_DATA {
        exe::crt_fread(this + offset, size, count, file);
    }
    1
}

/// Rule: the Belgian Second Division writes its three period winner flags and two dwords to the saved game.
///
/// `BelSecond::save_extra_data(file)`: `exe::crt_fwrite` of each part of `BEL_SECOND_EXTRA_DATA` (none checked).
/// Returns the last write's count (what the original leaves in eax).
/// Original: `BelSecond::save_extra_data` (0x420e90, bel_second.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn bel_second_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx08_19 belgian second division data saved");
    let mut r = 0;
    for (offset, size, count) in BEL_SECOND_EXTRA_DATA {
        r = exe::crt_fwrite(this + offset, size, count, file);
    }
    r
}

/// Comparer for `bel_second_load_extra_data`: the original reads, the file goes back and the copy reads again; the
/// fields, the returns and the file positions must match (`compare_cup_extra_data_load`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_cup_extra_data_load("bel_second_load_extra_data", this, file, &[(0xee, 8), (BEL_SECOND_PERIOD_WON, 3)], || o(this, file), || bel_second_load_extra_data(this, file))
}

/// Comparer for `bel_second_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file)
}
