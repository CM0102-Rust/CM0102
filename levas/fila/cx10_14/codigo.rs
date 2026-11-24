// ---------- cx10_14: the Dutch First Division's own data written to the saved game ----------

/// Writes the Dutch First Division's four period winner flags (+0xee to +0xf1) to the saved game, one byte each
/// (`exe::crt_fwrite`, unchecked), in the order the load reads them; returns what the last fwrite returns.
/// Original: `HolFirst::save_extra_data` (0x5f75b0, hol_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_first_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx10_14 dutch period winners saved");
    let mut r = 0;
    for k in 0..4 {
        r = exe::crt_fwrite(this + 0xee + k, 1, 1, file) as u32; // the period winner flag of period k
    }
    r
}

/// Comparer for `hol_first_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file)
}
