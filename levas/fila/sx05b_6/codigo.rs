// ---------- sx05b_6: loading the team award, the year award and the year ratings from a saved game ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\award\team_award.cpp" and "...\year_award.cpp" (the error boxes).
pub(crate) const TEAM_AWARD_CPP: usize = 0xa8c87c;
pub(crate) const YEAR_AWARD_CPP: usize = 0xad59b0;

/// The "Load Game Error" box of a save game load: "v%s %s.%s %d (%s)" of the source file `cpp` at `line` with the
/// message, then the box ("The save game is corrupted...", `0x9af334`). The mouse flag is left alone.
pub(crate) unsafe fn save_load_error_box(cpp: usize, line: u32, message: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(cpp, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message);
    comp_lang_format()(0xae28bc, 0x9af3e0);
    comp_lang_format()(0xae26c4, 0x9af334, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Reads one field of an award from the save (`exe::zread`, one item of `size` bytes); a short read shows the load
/// error box of `cpp` at `line` with `message`.
unsafe fn award_read(dest: usize, size: u32, file: usize, cpp: usize, line: u32, message: usize) -> bool {
    if exe::zread(dest, size, 1, file) == 1 {
        return true;
    }
    save_load_error_box(cpp, line, message);
    false
}

/// Loads a team award from a saved game: the update date (8 bytes), the 0x205-byte selection record, the substitutes
/// count (1) and the first and last history offsets (4 each), in that order; 1, or 0 after the load error box of the
/// first short read (lines 0x69, 0x70, 0x77, 0x7e, 0x85, each with its message).
/// Original: `TEAM_AWARD::load_from_disk` (0x8af660, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_load_from_disk(this: usize, file: usize) -> u32 {
    use team_award_obj as ta;
    let fields: [(usize, u32, u32, usize); 5] = [
        (ta::UPDATE_DATE, 8, 0x69, 0xa8c844),
        (ta::RECORD, ta::RECORD_SIZE, 0x70, 0xa8c80c),
        (ta::SUBSTITUTES, 1, 0x77, 0xa8c7d8),
        (ta::HISTORY_FIRST, 4, 0x7e, 0xa8c798),
        (ta::HISTORY_LAST, 4, 0x85, 0xa8c758),
    ];
    for (off, size, line, message) in fields {
        if !award_read(this + off, size, file, TEAM_AWARD_CPP, line, message) {
            return 0;
        }
    }
    covered!("sx05b_6 team award loaded");
    1
}

/// Loads a year award from a saved game: the first and last history offsets (4 bytes each; a short read, the load
/// error box of line 0x70 or 0x77 and 0); then, with a scope, its shortlist is built from the save
/// (`construct_award_shortlist`(scope, file) in 0x40 bytes of `exe::crt_operator_new`): out of memory, the "Memory
/// Allocation Error" box (line 0x81, "Cannot create AWARD_SHORTLIST") and 0. Otherwise 1.
/// Original: `YEAR_AWARD::load_from_disk` (0x92fff0, year_award.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn year_award_load_from_disk(this: usize, scope: usize, file: usize) -> u32 {
    use year_award_obj as ya;
    if !award_read(this + ya::HISTORY_FIRST, 4, file, YEAR_AWARD_CPP, 0x70, 0xad5978)
        || !award_read(this + ya::HISTORY_LAST, 4, file, YEAR_AWARD_CPP, 0x77, 0xad5940) {
        return 0;
    }
    if scope == 0 {
        return 1;
    }
    covered!("sx05b_6 year award shortlist loaded");
    let p = exe::crt_operator_new(0x40);
    let shortlist = if p != 0 { construct_award_shortlist(p, scope, file) } else { 0 };
    write_usize(this + ya::SHORTLIST, shortlist);
    if shortlist == 0 {
        memory_error_box_of(YEAR_AWARD_CPP, 0x81, 0xad5900);
        return 0;
    }
    1
}

/// The "Memory Allocation Error" box of a source file at a line with a message ("v%s %s.%s %d (%s)"); the game goes on.
pub(crate) unsafe fn memory_error_box_of(cpp: usize, line: u32, message: usize) {
    let version: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let (mut name, mut ext) = ([0u8; 256], [0u8; 256]);
    exe::crt_strtok(cpp, 0usize, 0usize, name.as_mut_ptr(), ext.as_mut_ptr());
    let v = version(name.as_mut_ptr(), ext.as_mut_ptr(), line, message);
    comp_crt_format()(0xae24d0, 0x9a8120, v, name.as_ptr(), ext.as_ptr(), line, message);
    comp_lang_format()(0xae28bc, 0x9a8108);
    comp_lang_format()(0xae26c4, 0x9a7ff8, 0xae24d0);
    exe::gui_show_message_box(0xae28bc, 0xae26c4, 0);
}

/// Loads the year ratings table from a saved game: the entry count (4 bytes, into +4; a short read, the "Error" box
/// of line 0x60 and 0); with entries, the capacity becomes the count and the entries (21 bytes each, `exe::crt_malloc`;
/// out of memory, the box of line 0x6c and 0) are read in one go (short, the box of line 0x73 and 0). Otherwise 1.
/// Original: `YEAR_RATINGS::load_from_disk` (0x931390, year_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    use year_ratings_consts as yr;
    if exe::zread(this + 4, 4, 1, file) != 1 {
        show_error_at(yr::YEAR_RATINGS_CPP, 0x60);
        return 0;
    }
    let count = read_u32(this + 4);
    if count == 0 {
        return 1;
    }
    covered!("sx05b_6 year ratings loaded");
    write_u32(this + 8, count);
    let entries = exe::crt_malloc(count.wrapping_mul(yr::ENTRY_SIZE as u32));
    write_usize(this, entries);
    if entries == 0 {
        show_error_at(yr::YEAR_RATINGS_CPP, 0x6c);
        return 0;
    }
    if exe::zread(entries, yr::ENTRY_SIZE as u32, count, file) != read_u32(this + 4) {
        show_error_at(yr::YEAR_RATINGS_CPP, 0x73);
        return 0;
    }
    1
}

/// Comparer for `team_award_load_from_disk`: reading the save cannot be done twice; only the original runs and a
/// loaded award must keep its substitutes count in range (0 to 9).
pub(crate) unsafe extern "thiscall" fn compare_team_award_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    if r == 1 {
        let subs = read_i8(this + team_award_obj::SUBSTITUTES);
        SCOREBOARD.record((0..=9).contains(&subs), || format!("team_award_load_from_disk({:#x}): substitutes {}", this, subs));
    }
    r
}

/// Comparer for `year_award_load_from_disk`: only the original (it reads the save and builds the shortlist); a load
/// with a scope must leave a shortlist.
pub(crate) unsafe extern "thiscall" fn compare_year_award_load_from_disk(this: usize, scope: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%)(this, scope, file);
    if r == 1 && scope != 0 {
        SCOREBOARD.record(read_usize(this + year_award_obj::SHORTLIST) != 0, || format!("year_award_load_from_disk({:#x}): no shortlist", this));
    }
    r
}

/// Comparer for `year_ratings_load_from_disk`: only the original; the loaded count must fit the capacity.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, file);
    if r == 1 {
        let (count, capacity) = (read_u32(this + 4), read_u32(this + 8));
        SCOREBOARD.record(count == 0 || count == capacity, || format!("year_ratings_load_from_disk({:#x}): count {} capacity {}", this, count, capacity));
    }
    r
}
