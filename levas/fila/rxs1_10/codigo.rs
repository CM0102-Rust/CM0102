/// Copies a 110-byte staff record (`staff::SIZE`) into another and returns the destination.
///
/// `Staff::operator=(src)` on `dest`: the fields one by one in the record's order, the five embedded dates through
/// `Date::operator=` (an 8-byte copy each); every byte of the record is copied. Called by the fake players' record
/// swap (`database_shuffle_player_records`) and by `VIRTUAL_STAFF::relocate_virtual_players` (0x90e9a0).
/// Original: `Staff::operator=` (0x5335f0, Database.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn staff_assign(dest: usize, src: usize) -> usize {
    covered!("rxs1_10 staff record copied");
    core::ptr::copy_nonoverlapping(src as *const u8, dest as *mut u8, staff::SIZE); // rxs1_10: the whole record
    dest
}

/// Comparer for `staff_assign`: side by side, the original into `dest` and the copy into a local record from the same
/// source; the bytes and the returned pointer must agree.
pub(crate) unsafe extern "thiscall" fn compare_staff_assign(dest: usize, src: usize) -> usize {
    let mut mine = [0u8; 0x6e];
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(dest, src);
    staff_assign(mine.as_mut_ptr() as usize, src);
    let theirs = core::slice::from_raw_parts(dest as *const u8, 0x6e);
    SCOREBOARD.record(r == dest && theirs == &mine[..], || format!("staff_assign(dest {:#x}, src {:#x}): original {:x?} rust {:x?}", dest, src, theirs, &mine[..]));
    r
}
