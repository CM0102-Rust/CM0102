// ---------- batch %%LEVA%%: the record copies and the date comparison of comp_screens.cpp: the youngest/oldest player record, the attendance record and Date::operator== ----------

/// The size of a youngest or oldest player record (Comp_Stats and Club_Records).
const PLAYER_AGE_RECORD_SIZE: usize = 0x3f;
/// The size of an attendance record: the attendance and the match record (0x1f bytes) after it.
const ATTENDANCE_RECORD_SIZE: usize = 4 + 0x1f;

/// Copies a youngest or oldest player record (0x3f bytes): the age as a date, the holder, the holder's birth date and
/// the match it happened in.
///
/// `player_age_record_assign(this, source)`: the date at +0 (`copy_record_date`), the dwords +8, +0xc, +0x10 and
/// +0x14 [inference: the person, his club, the age in days and years], the date at +0x18, then the match record at
/// +0x20 field by field as `match_record_assign` copies it (the original has it inline). Returns `this`.
/// Original: `player_age_record_assign` (0x4a1fb0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_age_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 player age record copy");
    copy_record_date(this, source);
    for o in [8usize, 0xc, 0x10, 0x14] {
        write_u32(this + o, read_u32(source + o));
    }
    // tel39_5: the second date
    copy_record_date(this + 0x18, source + 0x18);
    match_record_assign(this + 0x20, source + 0x20);
    this
}

/// Copies an attendance record (the highest and lowest attendances): the attendance (dword) and its match record.
///
/// `attendance_record_assign(this, source)`: dword +0, then `match_record_assign(this + 4, source + 4)`. Returns
/// `this`.
/// Original: `attendance_record_assign` (0x4ab5b0, comp_screens.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn attendance_record_assign(this: usize, source: usize) -> usize {
    covered!("tel39_5 attendance record copy");
    // tel39_5: the attendance
    write_u32(this, read_u32(source));
    match_record_assign(this + 4, source + 4);
    this
}

/// Whether two game dates are the same day: the year (word +2) and the day of the year (word +0) equal; the third
/// field (the leap-year flag at +4) is not compared.
///
/// `Date::operator==(this, other)`: `other` comes by value (8 bytes on the stack: `day_year` holds the day in its low
/// word and the year in its high word, `_leap` the flag). 1 when both words match, else 0; the year is compared first.
/// Original: `Date::operator==` (0x4a7f20, comp_screens.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn date_equals(this: usize, day_year: u32, _leap: u32) -> u32 {
    covered!("tel39_5 date equals");
    // tel39_5: the year, then the day
    ((day_year >> 16) as u16 == read_u16(this + 2) && day_year as u16 == read_u16(this)) as u32
}

/// Comparer for `player_age_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_player_age_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PLAYER_AGE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, PLAYER_AGE_RECORD_SIZE)], || o(this, source), || player_age_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("player_age_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `attendance_record_assign`: side by side over the record and the return.
pub(crate) unsafe extern "thiscall" fn compare_attendance_record_assign(this: usize, source: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_ATTENDANCE_RECORD_ASSIGN);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, ATTENDANCE_RECORD_SIZE)], || o(this, source), || attendance_record_assign(this, source));
    SCOREBOARD.record(same && r == n, || format!("attendance_record_assign({:#x}, {:#x}) original {:x?} rust {:x?}", this, source, mo, mr));
    r
}

/// Comparer for `date_equals`: the answer side by side (nothing is written).
pub(crate) unsafe extern "thiscall" fn compare_date_equals(this: usize, day_year: u32, leap: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_DATE_EQUALS)(this, day_year, leap);
    let n = date_equals(this, day_year, leap);
    SCOREBOARD.record(r == n, || format!("Date::operator==({:#x}: {:#x}, {:#x}) original={} rust={}", this, read_u32(this), day_year, r, n));
    r
}
