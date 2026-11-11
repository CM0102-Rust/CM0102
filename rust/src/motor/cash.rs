use super::*;

/// CRT fread of one record of tam bytes; returns records read.
pub(crate) unsafe fn crt_fread_one(buf: usize, size: u32, f: usize) -> u32 {
    exe::crt_fread(buf, size, 1, f)
}

/// CRT fclose.
pub(crate) unsafe fn cash_crt_fclose(f: usize) {
    exe::crt_fclose(f);
}

/// Error box of formation.cpp at the given line.
pub(crate) unsafe fn formation_error_box(row: u32) {
    show_error_at(0x9cc3dc, row);
}

/// Byte swap of a dword.
pub(crate) fn swap_bytes_u32(v: u32) -> u32 {
    v.swap_bytes()
}

/// Failed read: box unless quiet, closes the file and returns 0.
pub(crate) unsafe fn fail_read(quiet: u32, row: u32, f: usize) -> u32 {
    if quiet == 0 {
        formation_error_box(row);
    }
    cash_crt_fclose(f);
    0
}

/// swaps the two bytes of each word of an array
/// Original: `swap_bytes_16_array` (0x43f2d0, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_16_array(p: usize, count: u32) {
    for k in 0..count as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}

/// reverses the four bytes of each dword of an array
/// Original: `swap_bytes_32_array` (0x43f300, cash.cpp, cdecl).
pub(crate) unsafe extern "C" fn swap_bytes_32_array(p: usize, count: u32) {
    for k in 0..count as usize {
        write_u32(p + 4 * k, swap_bytes_u32(read_u32(p + 4 * k)));
    }
}

