// ---------- batch %%LEVA%%: the missing fixed key report (index.cpp; sx12b_11) ----------

/// Reports a fixed key the database lacks: "Unable to find the %s index" with the key's name into the shared text
/// buffer 0xde1f64 (`sprintf`), then the "Error" box (index.cpp line 0x16b1). Called by `set_key_ids` (1235 sites)
/// only when a key's record is not found.
/// Original: `report_missing_key_id` (0x60d780, index.cpp, cdecl).
pub(crate) unsafe extern "C" fn report_missing_key_id(name: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    sprintf(0xde1f64, 0x9d93f4, name); // "Unable to find the %s index"
    show_error_at(0x9d93cc, 0x16b1); // "...\index.cpp"
}

/// Comparer for `report_missing_key_id`: only the original (the box).
pub(crate) unsafe extern "C" fn compare_report_missing_key_id(name: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(name)
}
