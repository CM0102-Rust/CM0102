// ---------- cx04_3: the UEFA Cup's own data and the FIFA ranking table in the saved game (uefa_cup.cpp, fifa_rankings.cpp) ----------

/// Writes the UEFA Cup's own data to the saved game: the count of clubs in the preliminary round
/// (`uefa_cup::PRELIMINARY_CLUB_COUNT`, 1 byte; `exe::crt_fwrite`, unchecked). Returns the write's count.
/// Original: `UefaCup::save_extra_data` (0x903340, uefa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn uefa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx04_3 uefa cup data saved");
    exe::crt_fwrite(this + uefa_cup::PRELIMINARY_CLUB_COUNT, 1, 1, file)
}

/// The file of the FIFA ranking table inside the saved game's archive.
const FIFA_RANKINGS_DAT: usize = 0x9c752c; // "fifa_rankings.dat"
/// fifa_rankings.cpp, for the error boxes.
const FIFA_RANKINGS_CPP: usize = 0x9c74f4;

/// zclose (`FN_ZCLOSE`, cdecl) with the value it leaves in eax, which the save passes on.
unsafe fn zclose_result(file: *mut usize) -> u32 {
    let f: unsafe extern "C" fn(*mut usize) -> u32 = core::mem::transmute(FN_ZCLOSE);
    f(file)
}

/// Writes the FIFA ranking table into fifa_rankings.dat of the saved game's archive, as `fifa_rankings_load_dat` reads
/// it: the file opened for writing (`exe::zopen`(archive, name, 1, 0x16, 4); none: the box at line 0x299); the table,
/// `G_NATION_COUNT` entries of 0x2c bytes in one write (short: line 0x2a0); for each nation the results count
/// (`fifa_ranking_entry::HISTORY_COUNT`, 2 bytes; short: line 0x2aa) and the history (`HISTORY`, count × 5 bytes, a
/// write even when the count is 0; short: line 0x2b1); the rollover month (`fifa_rankings_fields::ROLLOVER_MONTH`, 1
/// byte; line 0x2b9) and the date of the last update (`LAST_UPDATE_DATE`, 8 bytes; line 0x2c0); then the file closed
/// (`FN_ZCLOSE`), whose result it returns. Every write is `exe::zwrite`(source, size, count, file), cdecl.
///
/// Defect of the original: after an error box the file is left open (no zclose) and the routine returns what the box
/// left in eax (the copy returns 0 there).
/// Original: `FIFA_RANKINGS::save_dat` (0x58dae0, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fifa_rankings_save_dat(this: usize, archive: usize) -> u32 {
    let mut file = exe::zopen(archive, FIFA_RANKINGS_DAT, 1, 0x16, 4);
    if file == 0 {
        show_error_at(FIFA_RANKINGS_CPP, 0x299);
        return 0;
    }
    let nations = read_i32(G_NATION_COUNT);
    if exe::zwrite(read_usize(this), 0x2c, nations as u32, file) != nations as u32 {
        show_error_at(FIFA_RANKINGS_CPP, 0x2a0);
        return 0;
    }
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let e = read_usize(this) + i as usize * 0x2c;
        if exe::zwrite(e + fifa_ranking_entry::HISTORY_COUNT, 2, 1, file) != 1 {
            show_error_at(FIFA_RANKINGS_CPP, 0x2aa);
            return 0;
        }
        let count = read_i16(e + fifa_ranking_entry::HISTORY_COUNT) as i32;
        if exe::zwrite(read_usize(e + fifa_ranking_entry::HISTORY), 5, count as u32, file) as i32 != read_i16(read_usize(this) + i as usize * 0x2c + fifa_ranking_entry::HISTORY_COUNT) as i32 {
            show_error_at(FIFA_RANKINGS_CPP, 0x2b1);
            return 0;
        }
        i += 1;
    }
    if exe::zwrite(this + fifa_rankings_fields::ROLLOVER_MONTH, 1, 1, file) != 1 {
        show_error_at(FIFA_RANKINGS_CPP, 0x2b9);
        return 0;
    }
    if exe::zwrite(this + fifa_rankings_fields::LAST_UPDATE_DATE, 8, 1, file) != 1 {
        show_error_at(FIFA_RANKINGS_CPP, 0x2c0);
        return 0;
    }
    covered!("cx04_3 fifa rankings saved");
    zclose_result(&mut file)
}

/// Reads the FIFA ranking table from fifa_rankings.dat of the saved game's archive (the constructor's load): the file
/// opened for reading (`exe::zopen`(archive, name, 0, 0x16, 4); none: the box at line 0x23a); the table allocated
/// (`exe::crt_malloc` of `G_NATION_COUNT` × 0x2c, into +0; none: line 0x242) and read in one go (short: line 0x249);
/// for each nation the saved results count into `fifa_ranking_entry::HISTORY_CAPACITY` (2 bytes; short: line 0x253),
/// copied to `HISTORY_COUNT`; with no results the history pointer is cleared, otherwise the history is allocated
/// (count × 5 bytes, into `HISTORY`; none: line 0x261) and read (short: line 0x268); then the rollover month (1 byte;
/// line 0x276) and the date of the last update (8 bytes; line 0x27d). Every read is `exe::zread`(dest, size, count,
/// file), cdecl. Closes the file (`exe::zclose`) and returns 1; after an error box, 0.
///
/// Defect of the original: after an error box the file is left open (no zclose).
/// Original: `FIFA_RANKINGS::load_dat` (0x58d7a0, fifa_rankings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn fifa_rankings_load_dat(this: usize, archive: usize) -> u32 {
    let mut file = exe::zopen(archive, FIFA_RANKINGS_DAT, 0, 0x16, 4);
    if file == 0 {
        show_error_at(FIFA_RANKINGS_CPP, 0x23a);
        return 0;
    }
    let table = exe::crt_malloc((read_i32(G_NATION_COUNT) * 0x2c) as u32);
    write_usize(this, table);
    if table == 0 {
        show_error_at(FIFA_RANKINGS_CPP, 0x242);
        return 0;
    }
    let nations = read_i32(G_NATION_COUNT);
    if exe::zread(table, 0x2c, nations as u32, file) != read_i32(G_NATION_COUNT) as u32 {
        show_error_at(FIFA_RANKINGS_CPP, 0x249);
        return 0;
    }
    let mut i = 0i32;
    while i < read_i32(G_NATION_COUNT) {
        let e = read_usize(this) + i as usize * 0x2c;
        if exe::zread(e + fifa_ranking_entry::HISTORY_CAPACITY, 2, 1, file) != 1 {
            show_error_at(FIFA_RANKINGS_CPP, 0x253);
            return 0;
        }
        write_i16(e + fifa_ranking_entry::HISTORY_COUNT, read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY));
        let count = read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY);
        if count == 0 {
            write_usize(e + fifa_ranking_entry::HISTORY, 0);
        } else {
            let history = exe::crt_malloc((count as i32 * 5) as u32);
            write_usize(e + fifa_ranking_entry::HISTORY, history);
            if history == 0 {
                show_error_at(FIFA_RANKINGS_CPP, 0x261);
                return 0;
            }
            if exe::zread(history, 5, count as i32 as u32, file) as i32 != read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY) as i32 {
                show_error_at(FIFA_RANKINGS_CPP, 0x268);
                return 0;
            }
        }
        i += 1;
    }
    if exe::zread(this + fifa_rankings_fields::ROLLOVER_MONTH, 1, 1, file) != 1 {
        show_error_at(FIFA_RANKINGS_CPP, 0x276);
        return 0;
    }
    if exe::zread(this + fifa_rankings_fields::LAST_UPDATE_DATE, 8, 1, file) != 1 {
        show_error_at(FIFA_RANKINGS_CPP, 0x27d);
        return 0;
    }
    covered!("cx04_3 fifa rankings loaded");
    exe::zclose(&mut file);
    1
}

/// Comparer for `uefa_cup_save_extra_data`: it writes the saved game; only the original runs (the save proof).
pub(crate) unsafe extern "thiscall" fn compare_uefa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file)
}

/// Comparer for `fifa_rankings_save_dat`: it writes the saved game; only the original runs (the save proof).
pub(crate) unsafe extern "thiscall" fn compare_fifa_rankings_save_dat(this: usize, archive: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, archive)
}

/// Comparer for `fifa_rankings_load_dat`: it opens and reads the archive (it cannot run twice); only the original, with
/// the plan when it returns 1: a table, and in each nation the results count equal to the capacity, with a history
/// exactly when the count is not 0.
pub(crate) unsafe extern "thiscall" fn compare_fifa_rankings_load_dat(this: usize, archive: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, archive);
    if r != 1 {
        return r;
    }
    let table = read_usize(this);
    let bad = (0..read_i32(G_NATION_COUNT)).find(|&i| {
        let e = table + i as usize * 0x2c;
        let count = read_i16(e + fifa_ranking_entry::HISTORY_COUNT);
        count != read_i16(e + fifa_ranking_entry::HISTORY_CAPACITY) || (count == 0) != (read_usize(e + fifa_ranking_entry::HISTORY) == 0)
    });
    SCOREBOARD.record(table != 0 && bad.is_none(), || format!("fifa_rankings_load_dat({:#x}): table {:#x}, nation {:?} with count and capacity apart", this, table, bad));
    r
}
