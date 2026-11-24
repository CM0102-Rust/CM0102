// ---------- sx21b_16: appending a four-byte value at the end of a data file (utils.cpp) ----------

/// Appends one four-byte value at the end of a file (`fseek` to the end, `fwrite`); returns what `fwrite` returns.
/// Original: `utils_append_u32_to_file` (0x90c1a0, utils.cpp, cdecl).
pub(crate) unsafe extern "C" fn utils_append_u32_to_file(file: usize, value: u32) -> u32 {
    exe::crt_fseek(file, 0, 2);
    let mut v = value;
    covered!("sx21b_16 value appended");
    exe::crt_fwrite(&mut v as *mut u32 as usize, 4, 1, file) // sx21b_16: four bytes
}

/// Comparer for `utils_append_u32_to_file`: it writes the file, so only the original runs.
pub(crate) unsafe extern "C" fn compare_utils_append_u32_to_file(file: usize, value: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, u32) -> u32>(%%I0%%)(file, value)
}
