// ---------- sx05b_7: saving the team award, the year award and the year ratings ----------

/// Writes one field to the save (`exe::zwrite`, one item of `size` bytes); a short write shows the "Error" box of
/// `cpp` at `line` (`show_error_at`).
unsafe fn award_write(src: usize, size: u32, file: usize, cpp: usize, line: u32) -> bool {
    if exe::zwrite(src, size, 1, file) == 1 {
        return true;
    }
    show_error_at(cpp, line);
    false
}

/// Saves a team award as `team_award_load_from_disk` reads it: the update date, the selection record, the
/// substitutes count and the two history offsets; a short write shows the "Error" box (lines 0x96, 0x9d, 0xa4, 0xab,
/// 0xb2) and stops there.
/// Original: `TEAM_AWARD::save_to_disk` (0x8af870, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_save_to_disk(this: usize, file: usize) {
    use team_award_obj as ta;
    let fields: [(usize, u32, u32); 5] = [
        (ta::UPDATE_DATE, 8, 0x96),
        (ta::RECORD, ta::RECORD_SIZE, 0x9d),
        (ta::SUBSTITUTES, 1, 0xa4),
        (ta::HISTORY_FIRST, 4, 0xab),
        (ta::HISTORY_LAST, 4, 0xb2),
    ];
    for (off, size, line) in fields {
        if !award_write(this + off, size, file, TEAM_AWARD_CPP, line) {
            return;
        }
    }
    covered!("sx05b_7 team award saved");
}

/// Saves a year award as `year_award_load_from_disk` reads it: the two history offsets (a short write, the "Error"
/// box of line 0x94 or 0x9b) and the shortlist when there is one (`award_shortlist_save`).
/// Original: `YEAR_AWARD::save_to_disk` (0x9301f0, year_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_award_save_to_disk(this: usize, file: usize) {
    use year_award_obj as ya;
    if !award_write(this + ya::HISTORY_FIRST, 4, file, YEAR_AWARD_CPP, 0x94) || !award_write(this + ya::HISTORY_LAST, 4, file, YEAR_AWARD_CPP, 0x9b) {
        return;
    }
    let shortlist = read_usize(this + ya::SHORTLIST);
    if shortlist != 0 {
        covered!("sx05b_7 year award shortlist saved");
        let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_AWARD_SHORTLIST_SAVE);
        save(shortlist, file);
    }
}

/// `AWARD_SHORTLIST::save(file)` (0x417720, thiscall, ret 4), through its hook.
const FN_AWARD_SHORTLIST_SAVE: usize = 0x417720;

/// Saves the year ratings table as `year_ratings_load_from_disk` reads it: the entry count (a short write, the "Error"
/// box of line 0x86) and, with entries, the 21-byte entries in one go (short, line 0x8f).
/// Original: `YEAR_RATINGS::save_to_disk` (0x9314e0, year_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn year_ratings_save_to_disk(this: usize, file: usize) {
    use year_ratings_consts as yr;
    if exe::zwrite(this + 4, 4, 1, file) != 1 {
        return show_error_at(yr::YEAR_RATINGS_CPP, 0x86);
    }
    let count = read_u32(this + 4);
    if count != 0 && exe::zwrite(read_usize(this), yr::ENTRY_SIZE as u32, count, file) != read_u32(this + 4) {
        show_error_at(yr::YEAR_RATINGS_CPP, 0x8f);
    }
}

/// Comparer for `team_award_save_to_disk`: the save file cannot be written twice; only the original.
pub(crate) unsafe extern "thiscall" fn compare_team_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, file)
}

/// Comparer for `year_award_save_to_disk`: only the original.
pub(crate) unsafe extern "thiscall" fn compare_year_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, file)
}

/// Comparer for `year_ratings_save_to_disk`: only the original.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, file)
}
