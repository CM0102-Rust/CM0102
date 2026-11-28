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
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TEAM_AWARD_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let subs = read_i8(this + team_award_obj::SUBSTITUTES);
        SCOREBOARD.record((0..=9).contains(&subs), || format!("team_award_load_from_disk({:#x}): substitutes {}", this, subs));
    }
    r
}

/// Comparer for `year_award_load_from_disk`: only the original (it reads the save and builds the shortlist); a load
/// with a scope must leave a shortlist.
pub(crate) unsafe extern "thiscall" fn compare_year_award_load_from_disk(this: usize, scope: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_YEAR_AWARD_LOAD_FROM_DISK)(this, scope, file);
    if r == 1 && scope != 0 {
        SCOREBOARD.record(read_usize(this + year_award_obj::SHORTLIST) != 0, || format!("year_award_load_from_disk({:#x}): no shortlist", this));
    }
    r
}

/// Comparer for `year_ratings_load_from_disk`: only the original; the loaded count must fit the capacity.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_load_from_disk(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_YEAR_RATINGS_LOAD_FROM_DISK)(this, file);
    if r == 1 {
        let (count, capacity) = (read_u32(this + 4), read_u32(this + 8));
        SCOREBOARD.record(count == 0 || count == capacity, || format!("year_ratings_load_from_disk({:#x}): count {} capacity {}", this, count, capacity));
    }
    r
}

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
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_TEAM_AWARD_SAVE_TO_DISK)(this, file)
}

/// Comparer for `year_award_save_to_disk`: only the original.
pub(crate) unsafe extern "thiscall" fn compare_year_award_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_YEAR_AWARD_SAVE_TO_DISK)(this, file)
}

/// Comparer for `year_ratings_save_to_disk`: only the original.
pub(crate) unsafe extern "thiscall" fn compare_year_ratings_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_YEAR_RATINGS_SAVE_TO_DISK)(this, file)
}

// ---------- sx04b_7: the month ratings' save and a nation's awards in the saved game (month_ratings.cpp, nation_awards.cpp) ----------

/// `MONTH_RATINGS::save_to_disk(file)` (0x767d60, thiscall, ret 4), as `NationAwards::save` calls it.
const FN_MONTH_RATINGS_SAVE_TO_DISK: usize = 0x767d60;
/// The size of a `MONTH_RATINGS` object.
const MONTH_RATINGS_SIZE: u32 = 0xf;
/// A nation's virtual `init_awards(file)` (slot 0: builds its award table, each award loading itself from the file)
/// and each award's virtual `save_to_disk(file)` (slot +4).
const NATION_AWARDS_INIT_SLOT: usize = 0x0;
const AWARD_SAVE_SLOT: usize = 0x4;

/// Writes the month ratings to the saved game as `month_ratings_load_from_disk` reads them: the entry count, the
/// entries (when there are any), the month and the year (`exe::zwrite`); a short write shows the "Error" box
/// (month_ratings.cpp 138, 147, 155, 162) and stops there.
/// Original: `MONTH_RATINGS::save_to_disk` (0x767d60, month_ratings.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn month_ratings_save_to_disk(this: usize, file: usize) {
    if exe::zwrite(this + MONTH_RATINGS_COUNT, 4, 1, file) != 1 {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x8a);
    }
    let count = read_u32(this + MONTH_RATINGS_COUNT);
    if count != 0 && exe::zwrite(read_usize(this + MONTH_RATINGS_ENTRIES), MONTH_RATING_ENTRY_SIZE, count, file) != read_u32(this + MONTH_RATINGS_COUNT) {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x93);
    }
    if exe::zwrite(this + month_ratings_fields::MONTH, 1, 1, file) != 1 {
        return tel26_error_box(MONTH_RATINGS_CPP, 0x9b);
    }
    if exe::zwrite(this + month_ratings_fields::YEAR, 2, 1, file) != 1 { // sx04b_7: the ratings' year
        return tel26_error_box(MONTH_RATINGS_CPP, 0xa2);
    }
    covered!("sx04b_7 month ratings saved");
}

/// Comparer for `month_ratings_save_to_disk`: it writes the file once; only the original runs (the save proof compares
/// the written game).
pub(crate) unsafe extern "thiscall" fn compare_month_ratings_save_to_disk(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_MONTH_RATINGS_SAVE_TO_DISK)(this, file)
}

/// Loads a nation's awards from the saved game: the nation's virtual `init_awards(file)` (its awards, each read from
/// the file), then a 4-byte flag; with the flag set, new month ratings (`exe::crt_operator_new` of 0xf bytes,
/// `MONTH_RATINGS::MONTH_RATINGS(file)`, which reads them; 0 when out of memory) go to
/// `nation_awards_fields::MONTH_RATINGS`. Returns 1, or 0 after the "Error" box (nation_awards.cpp 2059) when the
/// flag cannot be read. The original keeps the new object under an SEH frame; nothing here throws.
/// Original: `NationAwards::load` (0x76a280, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_load(this: usize, file: usize) -> u32 {
    let init: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this) + NATION_AWARDS_INIT_SLOT));
    init(this, file);
    let mut has_ratings = 0u32;
    if exe::zread(&mut has_ratings as *mut u32 as usize, 4, 1, file) != 1 {
        tel26_error_box(NATION_AWARDS_CPP, 0x80b);
        return 0;
    }
    if has_ratings != 0 { // sx04b_7: the saved month ratings
        covered!("sx04b_7 nation month ratings loaded");
        let p = exe::crt_operator_new(MONTH_RATINGS_SIZE);
        let ratings = if p != 0 {
            let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_MONTH_RATINGS_CONSTRUCT);
            construct(p, file)
        } else {
            0
        };
        write_usize(this + nation_awards_fields::MONTH_RATINGS, ratings);
    }
    1
}

/// Comparer for `nation_awards_load`: the load reads the file once and allocates; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_load(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NATION_AWARDS_LOAD)(this, file)
}

/// Saves a nation's awards to the saved game as `nation_awards_load` reads them: each award of the table
/// (`nation_awards_fields::TABLE`, `COUNT` read again at each step) through its virtual `save_to_disk(file)`, then the
/// 4-byte flag of whether the nation has month ratings (taken before the awards), and the month ratings themselves
/// (`MONTH_RATINGS::save_to_disk`) when it has them. A short write of the flag: the "Error" box (nation_awards.cpp
/// 2091) and nothing more.
/// Original: `NationAwards::save` (0x76a380, nation_awards.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nation_awards_save(this: usize, file: usize) {
    let has_ratings = (read_usize(this + nation_awards_fields::MONTH_RATINGS) != 0) as u32;
    let mut k = 0i16;
    while k < read_i16(this + nation_awards_fields::COUNT) {
        let award = read_usize(read_usize(this + nation_awards_fields::TABLE) + 4 * k as usize);
        let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(award) + AWARD_SAVE_SLOT));
        save(award, file);
        k += 1;
    }
    if exe::zwrite(&has_ratings as *const u32 as usize, 4, 1, file) != 1 { // sx04b_7: the month ratings flag
        return tel26_error_box(NATION_AWARDS_CPP, 0x82b);
    }
    let ratings = read_usize(this + nation_awards_fields::MONTH_RATINGS);
    if ratings != 0 {
        covered!("sx04b_7 nation month ratings saved");
        let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_MONTH_RATINGS_SAVE_TO_DISK);
        save(ratings, file);
    }
}

/// Comparer for `nation_awards_save`: it writes the file once; only the original runs (the save proof compares the
/// written game).
pub(crate) unsafe extern "thiscall" fn compare_nation_awards_save(this: usize, file: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_NATION_AWARDS_SAVE)(this, file)
}

// --- sx09c_4: discipline.dat in the save: Discipline::load (0x555860) and Discipline::save (0x555dd0), discipline.cpp ---

/// The save item "discipline.dat".
const DISCIPLINE_DAT: usize = 0x9b8f58;
/// The source path of discipline.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\discipline.cpp").
const DISCIPLINE_CPP_PATH: usize = 0x9b8f18;
/// `RulingBody::save` (0x7e7b80, ruling_body.cpp; thiscall(body, &z), ret 4): writes one rule set's entries and pending
/// bans to the open item.
const FN_RULING_BODY_SAVE: usize = 0x7e7b80;

/// Reads one counted table of the discipline object from the open item: the i16 count into `count` (box `row_count`
/// when short); with a count, the capacity field takes it, the array (`malloc` of count × `size`) goes into `table`
/// and its records are read (boxes `row_alloc`, `row_read`). Returns the line of the failure.
unsafe fn discipline_read_table(z: usize, this: usize, count: usize, capacity: usize, table: usize, size: u32,
                                rows: (u32, u32, u32)) -> Result<(), u32> {
    if exe::zread(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n == 0 {
        return Ok(());
    }
    write_i16(this + capacity, n);
    let array = exe::crt_malloc((n as i32 as u32).wrapping_mul(size));
    write_usize(this + table, array);
    if array == 0 {
        return Err(rows.1);
    }
    if exe::zread(array, size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.2);
    }
    Ok(())
}

/// Reads the discipline object from the save's "discipline.dat". Without a save (line 0xda) or the item (0xe2): the
/// discipline.cpp "Error" box and 0. Then: the per-person records (`G_STAFF_COUNT` × 29 bytes, `malloc` into
/// `discipline_fields::PERSON_RECORDS`; lines 0xea, 0xf2); the rule sets (`discipline_create_rule_sets` with the address
/// of the open item: each rule set reads its entries and pending bans); the ban array (`BAN_COUNT`, `BAN_CAPACITY`,
/// `BANS`, 20 bytes each; lines 0xfd, 0x10a, 0x112) and the nation index (`NATION_INDEX_COUNT`, `NATION_INDEX_CAPACITY`,
/// `NATION_INDEX`, 8 bytes each; lines 0x11b, 0x128, 0x130), each allocated only with a count; those failures close the
/// item and answer 0. Then the held-ban count (i16; a short read shows the box of line 0x139 and counts 0) and that many 10-byte
/// held bans into the `HELD_BANS` list (`LLIST::add_tail`; a short read shows the box of line 0x143 and goes on with the
/// next one). The item closed, 1.
/// Original: `Discipline::load` (0x555860, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_load(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xda);
        return 0;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0xe2);
        return 0;
    }
    covered!("sx09c_4 load discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let records = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(0x1d));
        write_usize(this + discipline_fields::PERSON_RECORDS, records);
        if records == 0 {
            return Err(0xea);
        }
        if exe::zread(records, 0x1d, read_u32(G_STAFF_COUNT), *z) != read_u32(G_STAFF_COUNT) {
            return Err(0xf2);
        }
        discipline_create_rule_sets(this, z as *mut usize as u32); // sx09c_4: the rule sets read their entries
        discipline_read_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BAN_CAPACITY,
                              discipline_fields::BANS, 0x14, (0xfd, 0x10a, 0x112))?;
        discipline_read_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX_CAPACITY,
                              discipline_fields::NATION_INDEX, 8, (0x11b, 0x128, 0x130))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return 0;
    }
    let mut held = 0i16;
    if exe::zread(&mut held as *mut i16 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x139);
        held = 0;
    }
    let mut record = [0u8; 0xa];
    let mut k = 0i16;
    while k < held {
        if exe::zread(record.as_mut_ptr() as usize, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x143);
        } else {
            exe::llist_add_tail(this + discipline_fields::HELD_BANS, record.as_ptr() as usize, 0xa);
        }
        k += 1;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `discipline_load`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_discipline_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DISCIPLINE_LOAD)(this, save)
}

/// Writes one counted table of the discipline object: the i16 count (line `rows.0` when short) and, with a count,
/// its records of `size` bytes (line `rows.1`).
unsafe fn discipline_write_table(z: usize, this: usize, count: usize, table: usize, size: u32, rows: (u32, u32)) -> Result<(), u32> {
    if exe::zwrite(this + count, 2, 1, z) != 1 {
        return Err(rows.0);
    }
    let n = read_i16(this + count);
    if n != 0 && exe::zwrite(read_usize(this + table), size, n as i32 as u32, z) != read_i16(this + count) as i32 as u32 {
        return Err(rows.1);
    }
    Ok(())
}

/// Writes the discipline object to the save's "discipline.dat": the per-person records (`G_STAFF_COUNT` × 29 bytes), each
/// existing rule set of the 43 (`RulingBody::save` with the address of the open item), the ban array (i16 count and
/// 20-byte bans), the nation index (i16 count and 8-byte items), the held-ban count (the list's i16) and the 10-byte
/// held bans from the list's head until it runs out. Without a save (line 0x163) or the item (0x16b): the
/// discipline.cpp "Error" box. A short write of the records or tables (lines 0x172, 0x185, 0x18f, 0x198, 0x1a2): the box
/// and the item closed. A short held-ban count (0x1ac) only shows the box; a short held ban (0x1b7) shows it and stops
/// the list. The item closed. The original returns nothing (eax is left over; the caller ignores it).
/// Original: `Discipline::save` (0x555dd0, discipline.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn discipline_save(this: usize, save: usize) {
    if save == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x163);
        return;
    }
    let mut z = exe::zopen(save, DISCIPLINE_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x16b);
        return;
    }
    covered!("sx09c_4 save discipline.dat");
    let tables = |z: &mut usize| -> Result<(), u32> {
        let n = read_u32(G_STAFF_COUNT);
        if exe::zwrite(read_usize(this + discipline_fields::PERSON_RECORDS), 0x1d, n, *z) != read_u32(G_STAFF_COUNT) {
            return Err(0x172);
        }
        let save_body: unsafe extern "thiscall" fn(usize, *mut usize) = core::mem::transmute(FN_RULING_BODY_SAVE);
        for k in 0..0x2busize {
            let body = read_usize(read_usize(this + discipline_fields::RULING_BODIES) + 4 * k);
            if body != 0 {
                save_body(body, z as *mut usize); // sx09c_4: each rule set
            }
        }
        discipline_write_table(*z, this, discipline_fields::BAN_COUNT, discipline_fields::BANS, 0x14, (0x185, 0x18f))?;
        discipline_write_table(*z, this, discipline_fields::NATION_INDEX_COUNT, discipline_fields::NATION_INDEX, 8, (0x198, 0x1a2))
    };
    if let Err(row) = tables(&mut z) {
        show_error_box(DISCIPLINE_CPP_PATH, row);
        exe::zclose(&mut z);
        return;
    }
    let held_list = Llist(this + discipline_fields::HELD_BANS);
    let held = held_list.count() as i32; // the original stores the i16 in a dword and writes 2 bytes
    if exe::zwrite(&held as *const i32 as usize, 2, 1, z) != 1 {
        show_error_box(DISCIPLINE_CPP_PATH, 0x1ac);
    }
    held_list.set_cursor(held_list.head());
    let mut record = exe::llist_get_next(held_list);
    while record != 0 {
        if exe::zwrite(record, 0xa, 1, z) != 1 {
            show_error_box(DISCIPLINE_CPP_PATH, 0x1b7);
            break;
        }
        record = exe::llist_get_next(held_list);
    }
    exe::zclose(&mut z);
}

/// Comparer for `discipline_save`: it writes into the save container, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch, item by item).
pub(crate) unsafe extern "thiscall" fn compare_discipline_save(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_DISCIPLINE_SAVE)(this, save)
}

// ---------- batch 342: loading the injuries and the human managers from a saved game (injury.cpp, human_manager.cpp; sx12b_4) ----------

/// The absolute value of the saved game's data version (`G_GAME_DATA_VERSION`), as the loaders compare it.
unsafe fn saved_data_version() -> i32 {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs()
}

/// Reads the injury state from the `injury.dat` item of a saved game; returns 1, or 0 after an error.
///
/// `Injury::load(save)`: no container: the "Error" box (injury.cpp line 0x15f) and 0; `injury.dat` not opened
/// (`exe::zopen`): line 0x167 and 0. Then, in order, each failing with the box, the item closed and 0: the physical
/// records (`injury_fields::PHYSICAL_RECORDS`, `malloc` of `G_STAFF_COUNT` x 31 bytes, line 0x16f; read, 0x177), the
/// injury slot count (`injury_fields::INJURY_ENTRY_COUNT`, 0x17f) and the 0x30-byte injuries
/// (`injury_fields::INJURY_ENTRIES`, `malloc`, 0x188). Saves older than data version 0x3e024 stored 0x2c bytes per
/// injury (each read alone, 0x196): the days left (`injury_entry::DAYS_LEFT`) and the training days left
/// (`injury_entry::TRAINING_DAYS_LEFT`) become the days from today (`G_CURRENT_DATE`, `Date::days_since`) to the
/// injury date plus `DAYS_OUT` and plus `TRAINING_RETURN_DAYS` (`Date::add_days`); newer ones store all 0x30 (0x1a5).
/// The recovery news count (+0x14, 0x1ae) and, when not zero, `injury_fields::RECOVERY_NEWS_COUNT` = it and the
/// 22-byte entries (`injury_fields::RECOVERY_NEWS_QUEUE`, `malloc`, 0x1bd; read, 0x1c5); the outbreak count
/// (`injury_fields::OUTBREAK_CAPACITY`, 0x1ce) and, when not zero, `injury_fields::OUTBREAK_COUNT` = it and the 17-byte
/// outbreaks (`injury_fields::OUTBREAKS`, `malloc`, 0x1de; read, 0x1e7); the word count of the match knock list (0x1f0)
/// and, when not zero, a new list (`injury_fields::MATCH_HIT_LIST`: the old one destroyed, `operator new` of 14 bytes,
/// `LLIST::LLIST`) and its 6-byte items (each read, 0x207, and added at the tail). The item closed, 1.
/// Original: `Injury::load` (0x62ab00, injury.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn injury_load(this: usize, save: usize) -> u32 {
    use injury_fields as f;
    const FILE: usize = 0x9e7f28; // "...\injury.cpp"
    if save == 0 {
        show_error_at(FILE, 0x15f);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9e7f68usize, 0, 0x16, 4); // "injury.dat"
    if z == 0 {
        show_error_at(FILE, 0x167);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    let records = exe::crt_malloc(read_u32(G_STAFF_COUNT).wrapping_mul(31));
    write_usize(this + f::PHYSICAL_RECORDS, records);
    if records == 0 {
        return failed(&mut z, 0x16f);
    }
    if exe::zread(records, 0x1f, read_u32(G_STAFF_COUNT), z) != read_u32(G_STAFF_COUNT) {
        return failed(&mut z, 0x177);
    }
    if exe::zread(this + f::INJURY_ENTRY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x17f);
    }
    let entries = exe::crt_malloc(read_u32(this + f::INJURY_ENTRY_COUNT).wrapping_mul(0x30));
    write_usize(this + f::INJURY_ENTRIES, entries);
    if entries == 0 {
        return failed(&mut z, 0x188);
    }
    if saved_data_version() < 0x3e024 {
        covered!("sx12b_4 injuries of an old save");
        for k in 0..read_i32(this + f::INJURY_ENTRY_COUNT).max(0) as usize {
            let e = read_usize(this + f::INJURY_ENTRIES) + k * 0x30;
            if exe::zread(e, 0x2c, 1, z) != 1 {
                return failed(&mut z, 0x196);
            }
            for (days, left) in [(injury_entry::DAYS_OUT, injury_entry::DAYS_LEFT), (injury_entry::TRAINING_RETURN_DAYS, injury_entry::TRAINING_DAYS_LEFT)] {
                let mut end = [0u8; 8];
                let d = exe::date_add_days(e + injury_entry::DATE, end.as_mut_ptr() as usize, read_u16(e + days) as u32);
                let n = exe::date_days_since(d, read_u32(G_CURRENT_DATE), read_u32(G_CURRENT_DATE + 4));
                write_u16(read_usize(this + f::INJURY_ENTRIES) + k * 0x30 + left, n as u16);
            }
        }
    } else if exe::zread(entries, 0x30, read_u32(this + f::INJURY_ENTRY_COUNT), z) != read_u32(this + f::INJURY_ENTRY_COUNT) {
        return failed(&mut z, 0x1a5);
    }
    if exe::zread(this + 0x14, 4, 1, z) != 1 {
        return failed(&mut z, 0x1ae); // 0x14: the recovery news capacity [inference]
    }
    let news = read_u32(this + 0x14);
    if news != 0 {
        write_u32(this + f::RECOVERY_NEWS_COUNT, news); // sx12b_4: the recovery news count
        let queue = exe::crt_malloc(news.wrapping_mul(0x16));
        write_usize(this + f::RECOVERY_NEWS_QUEUE, queue);
        if queue == 0 {
            return failed(&mut z, 0x1bd);
        }
        if exe::zread(queue, 0x16, read_u32(this + 0x14), z) != read_u32(this + 0x14) {
            return failed(&mut z, 0x1c5);
        }
    }
    if exe::zread(this + f::OUTBREAK_CAPACITY, 4, 1, z) != 1 {
        return failed(&mut z, 0x1ce);
    }
    let outbreaks = read_u32(this + f::OUTBREAK_CAPACITY);
    if outbreaks != 0 {
        write_u32(this + f::OUTBREAK_COUNT, outbreaks);
        let array = exe::crt_malloc(outbreaks.wrapping_mul(0x11));
        write_usize(this + f::OUTBREAKS, array);
        if array == 0 {
            return failed(&mut z, 0x1de);
        }
        if exe::zread(array, 0x11, read_u32(this + f::OUTBREAK_CAPACITY), z) != read_u32(this + f::OUTBREAK_CAPACITY) {
            return failed(&mut z, 0x1e7);
        }
    }
    let mut knocks = 0i16;
    if exe::zread(&mut knocks as *mut i16 as usize, 2, 1, z) != 1 {
        return failed(&mut z, 0x1f0);
    }
    if knocks != 0 {
        let old = read_usize(this + f::MATCH_HIT_LIST);
        if old != 0 {
            let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            destroy(old);
            exe::crt_operator_delete(old);
        }
        let list = exe::crt_operator_new(0xe);
        write_usize(this + f::MATCH_HIT_LIST, if list != 0 { exe::new_llist(list) } else { 0 });
        let mut k = 0i16;
        while k < knocks {
            let mut item = [0u8; 6];
            if exe::zread(item.as_mut_ptr() as usize, 6, 1, z) != 1 {
                return failed(&mut z, 0x207);
            }
            exe::llist_add_tail(read_usize(this + f::MATCH_HIT_LIST), item.as_ptr() as usize, 6);
            k += 1;
        }
    }
    covered!("sx12b_4 injuries loaded");
    exe::zclose(&mut z);
    1
}

/// The static buffer the human manager loader reads an old-layout tactic slot into (three of 0x1e75 bytes).
const OLD_TACTIC_BUFFER: usize = 0xb5e138;
const OLD_TACTIC_SLOT_SIZE: usize = 0x1e75;

/// Reads one tactic slot stored in the layout of saves older than data version 0x3e021 into `slot` (0x1ecd bytes).
///
/// The old slot (0x1e75 bytes) is read into the static buffer `OLD_TACTIC_BUFFER` + `buffer` x 0x1e75; its first 0x188b
/// bytes (the formation) are copied to the slot; then the eleven new 8-byte player entries at +0x188b each get the
/// dword at +0x588 + 4 x `which` of the slot (read again for each player) and 0, and the player instructions 8 and 2
/// are set (`Formation::set_player_instruction`); last, 0x642 bytes from the buffer's +0x188b go to the slot's +0x18e3.
/// Defects of the original: the old slot's tail is only 0x5ea bytes, so the copy takes 0x58 bytes of the next buffer
/// (past the third one: the globals from 0xb63c97) and writes 0x58 bytes past the slot (into the next one, or past the
/// fifth slot into the record's +0x9a01..); the dword it spreads over the players is picked by the slot's position in
/// its pass (`which`), not by the player.
unsafe fn read_old_tactic_slot(z: usize, buffer: usize, slot: usize, which: usize) {
    let old = OLD_TACTIC_BUFFER + buffer * OLD_TACTIC_SLOT_SIZE;
    exe::zread(old, OLD_TACTIC_SLOT_SIZE as u32, 1, z);
    core::ptr::copy(old as *const u8, slot as *mut u8, 0x188b);
    for j in 0..11usize {
        let legacy = read_u32(slot + 0x588 + 4 * which);
        write_u32(slot + 0x188b + 8 * j, legacy);
        write_u32(slot + 0x188f + 8 * j, 0);
        exe::formation_set_player_instruction(slot, j as i8, 8u64 << 32);
        exe::formation_set_player_instruction(slot, j as i8, 2u64 << 32);
    }
    core::ptr::copy((old + 0x188b) as *const u8, (slot + 0x18e3) as *mut u8, 0x642);
}

/// The fields of a human record read from the save in order (offset, size, count), after its first two tactic slots
/// and before the saved lineup: the holiday flag, the option bytes, the draft contract, the two offers, +0x9a61, the
/// holiday stamp, the active slot, the password and its typed entry, the computer control flag, the commentary speed,
/// the watched fixture, the search, training and print folders with their recent files, the squad view and the five
/// holiday options.
const HUMAN_RECORD_SAVED_FIELDS: [(usize, u32, u32); 20] = [
    (0x9a01, 4, 1), (0x9a05, 4, 1), (0x9a09, 0x50, 1), (0x9a59, 4, 1), (0x9a5d, 4, 1), (0x9a61, 4, 1), (0x9a65, 4, 1),
    (0x9a69, 1, 1), (0x9a6a, 1, 0x32), (0x9a9c, 1, 0x32), (0x9ace, 4, 1), (0x9ad2, 2, 1), (0x9ad4, 0x4f, 1),
    (0x9ba8, 1, 0x104), (0x9cac, 1, 0x410), (0xa0bc, 1, 0x104), (0xa1c0, 1, 0x410), (0xa5d0, 1, 0x104), (0xa6d4, 0xf, 1),
    (0x9b23, 1, 5),
];

/// Reads the human manager table from the `human_manager.dat` item of a saved game; returns 1, or 0 after an error.
///
/// `HUMAN_MANAGER::load(save)`: no container: the "Error" box (human_manager.cpp line 0x828) and 0; the item not
/// opened: line 0x830 and 0. The record count (`human_manager_fields::RECORD_COUNT`; failing: 0x837, the item closed,
/// 0) and the records (`operator new` of count x 0xa6ef + 4, the count first, `HUMAN_RECORD` constructors; none:
/// 0x840, closed, 0). For each record: the first two tactic slots (older than data version 0x3e021: each in the old
/// layout, `read_old_tactic_slot`), each with a team instruction word (+0x584) below 0x8000 given 0x20000
/// (`Formation::set_team_instruction`); the fields of `HUMAN_RECORD_SAVED_FIELDS` (unchecked); the 16 saved lineup
/// pairs (older than 0x3e005: 16 players, each with position -1); the nickname count (`NICKNAME_COUNT`, failing:
/// 0x892 and 0 with the item left open), the capacity (+0xa6e7) = it and, when not zero, the 30-byte nicknames
/// (`malloc`, 0x8a2; read, 0x8aa; both 0 with the item left open), otherwise no array. Then for each record: the two
/// offers cleared and, with a watched fixture's competition (+0x9ae8), its pointers rebuilt from the stored indices:
/// competition (`G_CLUB_COMP_TABLE` + [+0x9ad4] x 0x6b), the two clubs (+0x9af0, +0x9af4 from +0x9ae0, +0x9ae4,
/// `G_CLUB_TABLE`) and the stadium (+0x9aec from +0x9ad8, `G_STADIUM_TABLE`). Older than 0x3e001: the byte +0x59 of
/// the 16 human manager staff rows = 20. The dwords +8 (0x8d1) and `human_manager_fields::HOLIDAY_COUNT` (0x8d9), each
/// failing with the item closed and 0; for each record the last three tactic slots (old layout: `read_old_tactic_slot`
/// from the first buffer). The item closed, 1. Defects of the original: the three errors of the nicknames leave the
/// item open; the old [this] table is not freed.
/// Original: `HUMAN_MANAGER::load` (0x5fee50, human_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn human_manager_load(this: usize, save: usize) -> u32 {
    const FILE: usize = 0x9cf11c; // "...\human_manager.cpp"
    if save == 0 {
        show_error_at(FILE, 0x828);
        return 0;
    }
    let mut z = exe::zopen(save, 0x9cf164usize, 0, 0x16, 4); // "human_manager.dat"
    if z == 0 {
        show_error_at(FILE, 0x830);
        return 0;
    }
    let failed = |z: &mut usize, row: u32| {
        show_error_at(FILE, row);
        exe::zclose(z);
        0
    };
    if exe::zread(this + human_manager_fields::RECORD_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x837);
    }
    let count = read_u32(this + human_manager_fields::RECORD_COUNT);
    let block = exe::crt_operator_new(count.wrapping_mul(0xa6ef).wrapping_add(4));
    let records = if block != 0 {
        write_u32(block, count);
        exe::crt_vector_constructor_iterator(block + 4, 0xa6ef, count, 0x5fb690usize, 0x5fb700usize); // HUMAN_RECORD: 0xa6ef bytes, constructor, destructor
        block + 4
    } else {
        0
    };
    write_usize(this, records);
    if records == 0 {
        return failed(&mut z, 0x840);
    }
    let record_count = || read_i32(this + human_manager_fields::RECORD_COUNT).max(0) as usize;
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        if saved_data_version() < 0x3e021 {
            covered!("sx12b_4 human tactics of an old save");
            for k in 0..2 {
                read_old_tactic_slot(z, k, record + k * 0x1ecd, k);
            }
        } else {
            exe::zread(record, 0x1ecd, 2, z);
        }
        for k in 0..2 {
            if read_i32(record + k * 0x1ecd + 0x584) < 0x8000 { // sx12b_4: +0x584, the team instruction word
                exe::formation_set_team_instruction(record + k * 0x1ecd, 0x20000);
            }
        }
        for (field, size, n) in HUMAN_RECORD_SAVED_FIELDS {
            exe::zread(record + field, size, n, z);
        }
        if saved_data_version() < 0x3e005 {
            let mut players = [0u32; 0x10];
            exe::zread(players.as_mut_ptr() as usize, 4, 0x10, z);
            for (k, player) in players.iter().enumerate() {
                write_u32(record + human_manager_record::SAVED_LINEUP + 8 * k, *player);
                write_u32(record + human_manager_record::SAVED_LINEUP + 8 * k + 4, u32::MAX);
            }
        } else {
            exe::zread(record + human_manager_record::SAVED_LINEUP, 8, 0x10, z);
        }
        if exe::zread(record + human_manager_record::NICKNAME_COUNT, 4, 1, z) != 1 {
            show_error_at(FILE, 0x892); // the original leaves the item open here
            return 0;
        }
        write_u32(record + 0xa6e7, read_u32(record + human_manager_record::NICKNAME_COUNT)); // the nickname capacity
        let n = read_u32(record + 0xa6e7);
        if n != 0 {
            let list = exe::crt_malloc(n.wrapping_mul(0x1e));
            write_usize(record + human_manager_record::NICKNAMES, list);
            if list == 0 {
                show_error_at(FILE, 0x8a2);
                return 0;
            }
            if exe::zread(list, 0x1e, read_u32(record + human_manager_record::NICKNAME_COUNT), z) != read_u32(record + human_manager_record::NICKNAME_COUNT) {
                show_error_at(FILE, 0x8aa);
                return 0;
            }
        } else {
            write_usize(record + human_manager_record::NICKNAMES, 0);
        }
    }
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        write_u32(record + 0x9a59, 0); // the bid offer
        write_u32(record + 0x9a5d, 0); // the confirm offer
        if read_usize(record + 0x9ae8) != 0 {
            // the watched fixture's pointers from the indices the save keeps
            write_usize(record + 0x9ae8, read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_u32(record + human_manager_record::WATCHED_FIXTURE) as usize).wrapping_mul(0x6b)));
            write_usize(record + 0x9af0, club_at(read_i32(record + 0x9ae0) as isize).0);
            write_usize(record + 0x9af4, club_at(read_i32(record + 0x9ae4) as isize).0);
            write_usize(record + 0x9aec, read_usize(G_STADIUM_TABLE).wrapping_add((read_u32(record + 0x9ad8) as usize).wrapping_mul(stadium::SIZE)));
        }
    }
    if saved_data_version() < 0x3e001 {
        let staff = read_i32(G_STAFF_COUNT);
        for row in staff - 0x10..staff {
            write_u8(read_usize(G_STAFF_TABLE).wrapping_add((row as usize).wrapping_mul(staff::SIZE)) + 0x59, 0x14);
        }
    }
    if exe::zread(this + 8, 4, 1, z) != 1 {
        return failed(&mut z, 0x8d1);
    }
    if exe::zread(this + human_manager_fields::HOLIDAY_COUNT, 4, 1, z) != 1 {
        return failed(&mut z, 0x8d9);
    }
    for i in 0..record_count() {
        let record = human_manager_record_at(this, i);
        if saved_data_version() < 0x3e021 {
            for m in 0..3 {
                read_old_tactic_slot(z, m, record + 0x3d9a + m * 0x1ecd, m);
            }
        } else {
            exe::zread(record + 0x3d9a, 0x1ecd, 3, z); // sx12b_4: the last three tactic slots
        }
    }
    covered!("sx12b_4 human managers loaded");
    exe::zclose(&mut z);
    1
}

/// Comparer for `injury_load`: it allocates and reads the save item: it cannot run twice. Only the original runs; the
/// copy is proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_injury_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_INJURY_LOAD)(this, save)
}

/// Comparer for `human_manager_load`: it allocates and reads the save item: it cannot run twice. Only the original
/// runs; the copy is proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_human_manager_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HUMAN_MANAGER_LOAD)(this, save)
}

// ---------- batch 342: writing the hall of fame file (hall_of_fame.cpp; sx12b_5) ----------

/// `hall_of_fame_unscramble_and_check` (cdecl(buffer, length, checksum) -> 1/0), called through the exe so that its
/// own detour runs.
const FN_HALL_OF_FAME_UNSCRAMBLE: usize = 0x5f0680;

/// Writes the hall of fame (28 tables, 0x52080 bytes) to `hall_of_fame.bin`; returns 1, or 0 after an error.
///
/// `HallOfFame::save()`: `hall_of_fame.bin` opened "wb"; failing: the box with the translated "Error saving to disk" and
/// its message (0x9ce464) and 0. The header (the magic dword 0x3abfb2 and the version byte 1, 5 bytes; failing: the
/// "Error" box, hall_of_fame.cpp line 0x294, the file closed, 0). A null `this`: the boxes of lines 0x375 and 0x29c,
/// closed, 0. The table is scrambled in place: each byte i is added (unscrambled) to the checksum and XORed with the
/// low byte of the key dword i mod 480 (0x9cdc88); then for i from 0 up, the bytes at key[i mod 480] mod 0x52080 and
/// key[(i + 1) mod 480] mod 0x52080 (signed remainders) are swapped. The scrambled table is written (failing: line
/// 0x2a4), then the checksum (0x2ac), each failing with the file closed and 0; the table is unscrambled and checked
/// again (`hall_of_fame_unscramble_and_check`; failing: 0x2b4, closed, 0). The file closed, 1. Called by the game save
/// (0x82e430), the hall of fame screen (0x825cc0) and the constructor when the load fails. Defect of the original: a
/// failed write of the table or the checksum leaves the table in memory scrambled.
/// Original: `HallOfFame::save` (0x5f00c0, hall_of_fame.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn hall_of_fame_save(this: usize) -> u32 {
    use hall_of_fame_consts as hf;
    const BYTES: usize = hf::TABLES * hf::TABLE_SIZE;
    let file = exe::crt_fopen(0x9ce450usize, 0x9a7f28usize); // "hall_of_fame.bin", "wb"
    if file == 0 {
        let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let (mut title, mut message) = ([0u8; 0x200], [0u8; 0x200]);
        text(title.as_mut_ptr() as usize, 0x9ce4e4); // "Error saving to disk"
        text(message.as_mut_ptr() as usize, 0x9ce464);
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 0);
        return 0;
    }
    let failed = |row: u32| {
        show_error_at(hf::HALL_OF_FAME_CPP, row);
        exe::crt_fclose(file);
        0
    };
    let mut header = [0xb2u8, 0xbf, 0x3a, 0x00, 1]; // the magic 0x3abfb2 and the version 1
    if exe::crt_fwrite(header.as_mut_ptr() as usize, 5, 1, file) != 1 {
        return failed(0x294);
    }
    if this == 0 {
        show_error_at(hf::HALL_OF_FAME_CPP, 0x375);
        return failed(0x29c);
    }
    let key = |i: usize| read_i32(0x9cdc88 + 4 * (i % 0x1e0)); // the 480 key dwords
    let mut checksum = 0u32;
    for i in 0..BYTES {
        let b = read_u8(this + i);
        checksum = checksum.wrapping_add(b as u32); // sx12b_5: the checksum of the plain bytes
        write_u8(this + i, b ^ key(i) as u8);
    }
    for i in 0..BYTES {
        let a = this.wrapping_add((key(i) % BYTES as i32) as usize);
        let b = this.wrapping_add((key(i + 1) % BYTES as i32) as usize);
        let t = read_u8(a);
        write_u8(a, read_u8(b));
        write_u8(b, t);
    }
    if exe::crt_fwrite(this, BYTES as u32, 1, file) != 1 {
        return failed(0x2a4);
    }
    if exe::crt_fwrite(&mut checksum as *mut u32 as usize, 4, 1, file) != 1 {
        return failed(0x2ac);
    }
    let check: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_HALL_OF_FAME_UNSCRAMBLE);
    if check(this, BYTES as i32, checksum) == 0 {
        return failed(0x2b4);
    }
    covered!("sx12b_5 hall of fame saved");
    exe::crt_fclose(file);
    1
}

/// Comparer for `hall_of_fame_save`: the original writes the file; the copy then writes it again (the same file name,
/// the same table): the file bytes, the table and the return are compared. When the original fails (a box), only the
/// original.
pub(crate) unsafe extern "thiscall" fn compare_hall_of_fame_save(this: usize) -> u32 {
    let bytes = hall_of_fame_consts::TABLES * hall_of_fame_consts::TABLE_SIZE;
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HALL_OF_FAME_SAVE)(this);
    if r == 0 {
        return r;
    }
    let written = std::fs::read("hall_of_fame.bin").unwrap_or_default();
    let table = core::slice::from_raw_parts(this as *const u8, bytes).to_vec();
    let n = hall_of_fame_save(this);
    let again = std::fs::read("hall_of_fame.bin").unwrap_or_default();
    let now = core::slice::from_raw_parts(this as *const u8, bytes);
    let first = written.iter().zip(again.iter()).position(|(x, y)| x != y);
    SCOREBOARD.record(r == n && written == again && table == now, || format!("HallOfFame::save original {} rust {}: file sizes {} {}, first different byte {:?}, table equal {}",
        r, n, written.len(), again.len(), first, table == now));
    r
}

// ---------- batch 342: the board object and the loading of man_conf.dat (manager_manager.cpp) ----------

/// The source path of manager_manager.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\manager_manager.cpp").
const MANAGER_MANAGER_CPP_PATH: usize = 0x9f18a4;
/// The save item "man_conf.dat".
const MAN_CONF_DAT: usize = 0x9f18d8;
/// The game data versions of the man_conf.dat layout: board records of 0x49 bytes above the first (before, 0x41 bytes
/// without the last ultimatum date); the ultimatum and ban appeal lists above the second; 0x19-byte ban appeals from
/// the third (before, 0x15 bytes).
const MAN_CONF_49_BYTE_RECORDS_VERSION: i32 = 0x3e040;
const MAN_CONF_LISTS_VERSION: i32 = 0x3e033;
const MAN_CONF_25_BYTE_APPEALS_VERSION: i32 = 0x3e035;
/// The bytes the original patches to drop the CD requirement in this routine: the drive type's `jne` (75 56, `nop`s in
/// cm0102-1080) and the found test (85 c0, `xor` in cm0102-1080).
const MANAGER_MANAGER_CD_TYPE_JUMP: usize = 0x68dc41;
const MANAGER_MANAGER_CD_FOUND_TEST: usize = 0x68dc85;

/// The CD search at the start of the constructor: each CD drive (`GetLogicalDriveStringsA`, `GetDriveTypeA` = 5) is
/// looked at for "<root>cm0102.exe" (`exe::file_find_first`) until one has it, with the "Insert CD" box (OK) in
/// between. A no-CD exe patched in this routine (as cm0102-1080) is honoured by reading those bytes.
unsafe fn manager_manager_find_cd() {
    let drives: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let kind: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(MANAGER_MANAGER_CD_TYPE_JUMP) == 0x90;
    let always_found = read_u8(MANAGER_MANAGER_CD_FOUND_TEST) != 0x85;
    loop {
        // simplification: the buffer starts zeroed (as check_game_cd)
        let mut list = [0u8; 0x100];
        if drives(200, list.as_mut_ptr()) != 0 && list[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = list.as_ptr() as usize + i;
                let t = kind(root);
                if any_drive || t == 5 {
                    let (mut name, mut found) = ([0u8; 0x104], [0u8; 0x114]);
                    format(name.as_mut_ptr() as usize, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    if exe::file_find_first(name.as_ptr() as usize, found.as_mut_ptr() as usize, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(found.as_mut_ptr() as usize);
                }
                // simplification: the original steps with a signed byte (only past 32 drives would it differ)
                i += 4;
                if list[i] == 0 {
                    break;
                }
            }
        }
        covered!("sx14b_2 board insert CD");
        let (mut title, mut message) = ([0u8; 0x100], [0u8; 0x100]);
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The five owned lists, in the original's order: vacancies (+8), departures (+4), +0xc (0xe-byte items), ultimatums
/// (+0x10) and ban appeals (+0x14); each `operator new(0xe)` + `LLIST::LLIST`, 0 when the allocation fails.
unsafe fn manager_manager_new_lists(this: usize) {
    use manager_manager_fields as f;
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = exe::crt_operator_new(0xe);
        write_usize(this + field, if list != 0 { exe::new_llist(list) } else { 0 });
    }
}

/// The board record table for every club (`G_CLUB_COUNT` × 0x49 bytes, `malloc`); out of memory, the fatal
/// manager_manager.cpp box at `row` (the box, `game_fatal_cleanup`, exit(−1)).
unsafe fn manager_manager_new_table(this: usize, row: u32) {
    let table = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(0x49));
    write_usize(this + manager_manager_fields::BOARD_RECORDS, table);
    if table == 0 {
        let exit: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
        show_error_box(MANAGER_MANAGER_CPP_PATH, row);
        exe::game_fatal_cleanup();
        exe::shared_noop_ret0_5a8f60();
        exit(-1);
    }
}

/// Builds the board object (MANAGER_MANAGER, `G_MANAGER_MANAGER`): waits for the game CD, creates the five lists and,
/// with a save, reads them and the board records from man_conf.dat (`load_man_conf`); without a save, or when the
/// reading fails (everything it built freed, as `drop_manager_manager`, and the lists made again after the table),
/// the table for every club (`manager_manager_new_table`, fatal box at line 0x69 or 0x9f) filled fresh by
/// `init_club_board_records`. Returns the object.
/// Original: `MANAGER_MANAGER::MANAGER_MANAGER` (0x68dbf0, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_manager_manager(this: usize, save: usize) -> usize {
    manager_manager_find_cd();
    manager_manager_new_lists(this);
    if save == 0 {
        manager_manager_new_table(this, 0x69);
    } else {
        if load_man_conf(this, save) != 0 {
            covered!("sx14b_2 board from the save");
            return this;
        }
        covered!("sx14b_2 board save unreadable");
        drop_manager_manager(this);
        manager_manager_new_table(this, 0x9f);
        manager_manager_new_lists(this);
    }
    init_club_board_records(this);
    this
}

/// Comparer for `new_manager_manager`: it allocates and reads the save item, so it cannot run twice; only the original
/// runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_manager_manager(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_MANAGER_MANAGER)(this, save)
}

/// Frees the board object: the board record table (`free`) and the five lists (`LLIST::~LLIST` and `operator
/// delete`), in the order +8, +4, +0xc, +0x10, +0x14; each present one freed and its pointer zeroed.
/// Original: `MANAGER_MANAGER::~MANAGER_MANAGER` (0x68e0e0, manager_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_manager_manager(this: usize) {
    use manager_manager_fields as f;
    let table = read_usize(this + f::BOARD_RECORDS);
    if table != 0 {
        exe::crt_free(table);
        write_usize(this + f::BOARD_RECORDS, 0);
    }
    for field in [f::VACANCY_LIST, f::DEPARTURE_LIST, 0xc, f::ULTIMATUM_LIST, f::BAN_APPEAL_LIST] {
        let list = read_usize(this + field);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_manager_manager`: it frees, so it cannot run twice; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_manager_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_MANAGER_MANAGER)(this)
}

/// Converts the old 0x41-byte board records into the 0x49-byte `board_record`: the bytes as they are (the date pairs
/// through `Date::operator=`), except that +3 takes only its 5 low bits (the 3 high bits keep what `malloc` left, as
/// in the original), and `board_record::LAST_ULTIMATUM_DATE` becomes "never" (23/06/1971).
unsafe fn man_conf_convert_old_records(table: usize, old: usize, count: i32) {
    for k in 0..count.max(0) as usize {
        let (o, n) = (old + k * 0x41, table + k * 0x49);
        write_u8(n, read_u8(o));
        write_u8(n + 1, read_u8(o + 1));
        write_u8(n + 2, read_u8(o + 2));
        write_u8(n + 3, (read_u8(n + 3) & 0xe0) | (read_u8(o + 3) & 0x1f));
        core::ptr::copy_nonoverlapping((o + 4) as *const u8, (n + 4) as *mut u8, 0x14);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(n + board_record::LAST_REVIEW_DATE, tmp.as_mut_ptr() as usize, o + board_record::LAST_REVIEW_DATE);
        exe::date_operator_3d(n + board_record::JOB_APPLICATION_DATE, tmp.as_mut_ptr() as usize, o + board_record::JOB_APPLICATION_DATE);
        core::ptr::copy_nonoverlapping((o + 0x28) as *const u8, (n + 0x28) as *mut u8, 0x18);
        write_u8(n + board_record::DISCIPLINE_COUNTERS, read_u8(o + board_record::DISCIPLINE_COUNTERS));
        let mut never = [0u16; 4];
        let d = exe::date_date_549f70(&mut never, 0x17, 6, 0x7b3, u32::MAX);
        exe::date_operator_3d(n + board_record::LAST_ULTIMATUM_DATE, tmp.as_mut_ptr() as usize, d);
    }
}

/// Reads one list of man_conf.dat: the count (dword), then each item of `size` bytes appended to `list`
/// (`LLIST::add_tail`); false on a short read.
unsafe fn man_conf_read_list(z: usize, list: usize, size: u32) -> bool {
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    let mut item = [0u8; 0x26];
    for _ in 0..count {
        if exe::zread(item.as_mut_ptr() as usize, size, 1, z) != 1 {
            return false;
        }
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, size);
    }
    true
}

/// The ban appeal list with the old 0x15-byte items (before `MAN_CONF_25_BYTE_APPEALS_VERSION`): each becomes the
/// 0x19-byte item with its 0x15 bytes (the date through `Date::operator=`) and 0xff in the four new ones.
unsafe fn man_conf_read_old_appeals(z: usize, list: usize, count: i32) -> bool {
    for _ in 0..count {
        let (mut old, mut item) = ([0u8; 0x15], [0u8; 0x19]);
        if exe::zread(old.as_mut_ptr() as usize, 0x15, 1, z) != 1 {
            return false;
        }
        item[..5].copy_from_slice(&old[..5]);
        let mut tmp = [0u8; 8];
        exe::date_operator_3d(item.as_mut_ptr() as usize + 5, tmp.as_mut_ptr() as usize, old.as_mut_ptr() as usize + 5);
        item[0xd..0x15].copy_from_slice(&old[0xd..0x15]);
        item[0x15..].fill(0xff);
        exe::llist_add_tail(list, item.as_mut_ptr() as usize, 0x19);
    }
    true
}

/// The body of `load_man_conf` with the item open: false on a short read, an empty table or a failed allocation.
unsafe fn man_conf_read(this: usize, z: usize) -> bool {
    use manager_manager_fields as f;
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 || count == 0 {
        return false;
    }
    let table = exe::crt_malloc((count as u32).wrapping_mul(0x49));
    write_usize(this + f::BOARD_RECORDS, table);
    if table == 0 {
        return false;
    }
    let version = read_i32(G_GAME_DATA_VERSION).wrapping_abs();
    if version > MAN_CONF_49_BYTE_RECORDS_VERSION {
        if exe::zread(table, 0x49, count as u32, z) != count as u32 {
            return false;
        }
    } else {
        covered!("sx14b_2 old board records");
        let old = exe::crt_malloc((count as u32).wrapping_mul(0x41));
        if old == 0 {
            return false;
        }
        if exe::zread(old, 0x41, count as u32, z) != count as u32 {
            exe::crt_free(old);
            return false;
        }
        man_conf_convert_old_records(table, old, count);
        exe::crt_free(old);
    }
    if !man_conf_read_list(z, read_usize(this + f::VACANCY_LIST), 0x26)
        || !man_conf_read_list(z, read_usize(this + f::DEPARTURE_LIST), 6)
        || !man_conf_read_list(z, read_usize(this + 0xc), 0xe)
    {
        return false;
    }
    if version <= MAN_CONF_LISTS_VERSION {
        return true;
    }
    if !man_conf_read_list(z, read_usize(this + f::ULTIMATUM_LIST), 4) {
        return false;
    }
    let appeals = read_usize(this + f::BAN_APPEAL_LIST);
    if version >= MAN_CONF_25_BYTE_APPEALS_VERSION {
        return man_conf_read_list(z, appeals, 0x19);
    }
    let mut n = 0i32;
    if exe::zread(&mut n as *mut i32 as usize, 4, 1, z) != 1 {
        return false;
    }
    covered!("sx14b_2 old ban appeals");
    man_conf_read_old_appeals(z, appeals, n)
}

/// Reads the board object from the save's "man_conf.dat": nothing (0) without a save or the item. The club count and
/// the board records (`board_record`, `malloc`; 0x41-byte records before `MAN_CONF_49_BYTE_RECORDS_VERSION`,
/// converted), then the lists: vacancies (0x26-byte items), departures (6), +0xc (0xe) and, from
/// `MAN_CONF_LISTS_VERSION`, the ultimatums (4) and the ban appeals (0x19, or 0x15 converted before
/// `MAN_CONF_25_BYTE_APPEALS_VERSION`). The item is closed in every case after it opened; 1 when everything was read,
/// 0 on a short read, an empty table (club count 0) or a failed allocation (with no box: the constructor then builds
/// the board fresh).
/// Original: `MANAGER_MANAGER::load_man_conf` (0x68e180, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_man_conf(this: usize, save: usize) -> u32 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, MAN_CONF_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx14b_2 load man_conf.dat");
    // the local dates the original builds with Date::Date (0x54a110) and never reads are left out
    let ok = man_conf_read(this, z);
    exe::zclose(&mut z);
    ok as u32
}

/// Comparer for `load_man_conf`: it reads the save item and allocates, so it cannot run twice; only the original runs.
/// The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_load_man_conf(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_MAN_CONF)(this, save)
}

// ---------- batch 342: the saving of man_conf.dat (manager_manager.cpp) ----------

/// Writes one list to man_conf.dat: the count (`LLIST` +0, a signed word, as a dword) and each item of `size` bytes
/// from the head. Answers the manager_manager.cpp box row of the failure: `rows` = (count not written, list shorter
/// than its count, item not written).
unsafe fn man_conf_write_list(z: usize, list: usize, size: u32, rows: (u32, u32, u32)) -> Result<(), u32> {
    let mut count = read_i16(list) as i32;
    if exe::zwrite(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return Err(rows.0);
    }
    write_u32(list + 0xa, read_u32(list + 2)); // the cursor at the head
    let mut k = 0i32;
    while k < read_i16(list) as i32 {
        let item = exe::llist_get_next(list);
        if item == 0 {
            return Err(rows.1);
        }
        if exe::zwrite(item, size, 1, z) != 1 {
            return Err(rows.2);
        }
        k += 1;
    }
    Ok(())
}

/// The body of `save_man_conf` with the item open: the error row on a failure.
unsafe fn man_conf_write(this: usize, z: usize) -> Result<(), u32> {
    use manager_manager_fields as f;
    if exe::zwrite(G_CLUB_COUNT, 4, 1, z) != 1 {
        return Err(0x228);
    }
    let clubs = read_u32(G_CLUB_COUNT);
    if clubs == 0 {
        return Err(0x22e);
    }
    if exe::zwrite(read_usize(this + f::BOARD_RECORDS), 0x49, clubs, z) != read_u32(G_CLUB_COUNT) {
        return Err(0x234);
    }
    man_conf_write_list(z, read_usize(this + f::VACANCY_LIST), 0x26, (0x23d, 0x247, 0x24d))?;
    man_conf_write_list(z, read_usize(this + f::DEPARTURE_LIST), 6, (0x256, 0x260, 0x266))?;
    man_conf_write_list(z, read_usize(this + 0xc), 0xe, (0x26f, 0x279, 0x27f))?;
    man_conf_write_list(z, read_usize(this + f::ULTIMATUM_LIST), 4, (0x288, 0x292, 0x298))?;
    man_conf_write_list(z, read_usize(this + f::BAN_APPEAL_LIST), 0x19, (0x2a2, 0x2ac, 0x2b2))
}

/// Writes the board object to the save's "man_conf.dat" (`exe::zopen` for writing): the club count, the board
/// records (`G_CLUB_COUNT` × 0x49 bytes) and the five lists (vacancies 0x26-byte items, departures 6, +0xc 0xe,
/// ultimatums 4, ban appeals 0x19), each with its count. Without a save (row 0x21a) or the item (0x222): the
/// manager_manager.cpp "Error" box; any later failure (club count not written 0x228, no clubs 0x22e, records 0x234,
/// then the rows of each list) shows the box and closes the item; done, the item is closed.
/// Original: `MANAGER_MANAGER::save_man_conf` (0x68e920, manager_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_man_conf(this: usize, save: usize) {
    const MANAGER_MANAGER_CPP: usize = 0x9f18a4;
    if save == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x21a);
    }
    let mut z = exe::zopen(save, 0x9f18d8, 1, 0x16, 4); // "man_conf.dat"
    if z == 0 {
        return show_error_box(MANAGER_MANAGER_CPP, 0x222);
    }
    covered!("sx14b_3 save man_conf.dat");
    if let Err(row) = man_conf_write(this, z) {
        show_error_box(MANAGER_MANAGER_CPP, row);
    }
    exe::zclose(&mut z);
}

/// Comparer for `save_man_conf`: it writes the save item, so it cannot run twice; only the original runs. The copy is
/// proven by the save proof (the .qst written with and without the routine, compared item by item).
pub(crate) unsafe extern "thiscall" fn compare_save_man_conf(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_MAN_CONF)(this, save)
}

// --- sx16_7: the news store: News::News (0x785b20), News::~News (0x785d00), News::load_from_zip (0x787780) ---

/// The game CD check of the constructors (News, OfficialsManager, PLAYER_REGEN): among the drives
/// (GetLogicalDriveStringsA, 200 bytes) the first CD drive (GetDriveTypeA 5) holding "cm0102.exe" (`file_find_first`
/// answering 0; otherwise the search is closed with `file_find_close`); none: the "Insert CD" box (`langlib_sprintf` of
/// 0x987df4 and 0x987dc4, the box with 1) and again, forever. `drive_jne` and `found_test` are the addresses of the
/// drive type jump (`jne`) and of the found test (`test eax, eax`) in the constructor: the no-CD exes (cm0102-1080)
/// make the first a nop and the second a xor, and the copy reads those bytes from the running exe.
pub(crate) unsafe fn game_cd_check(drive_jne: usize, found_test: usize) {
    let drive_strings: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(read_usize(0x9670f8));
    let drive_type: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(read_usize(0x967100));
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let text: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let any_drive = read_u8(drive_jne) == 0x90;
    let always_found = read_u8(found_test) != 0x85; // the original has `test eax, eax` (85 c0)
    loop {
        let mut drives = [0u8; 0xc8];
        if drive_strings(0xc8, drives.as_mut_ptr()) != 0 && drives[0] != 0 {
            let mut i = 0usize;
            loop {
                let root = drives.as_ptr() as usize + i;
                if any_drive || drive_type(root) == 5 {
                    let mut name = [0u8; 0x104];
                    let mut found = [0u8; 0x114];
                    let n = name.as_mut_ptr() as usize;
                    // "%s" + "cm0102.exe" passed as ten characters, as the original does
                    sprintf(n, 0x987e00, root, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let f = found.as_mut_ptr() as usize;
                    if exe::file_find_first(n, f, 0) == 0 || always_found {
                        return;
                    }
                    exe::file_find_close(f);
                }
                // simplification: the original walks with a signed byte (steps of 4); it would only pass 127 with more
                // than 32 drives
                i += 4;
                if i >= drives.len() || drives[i] == 0 {
                    break;
                }
            }
        }
        let mut title = [0u8; 0x100];
        let mut message = [0u8; 0x100];
        text(title.as_mut_ptr() as usize, 0x987df4); // "Insert CD"
        text(message.as_mut_ptr() as usize, 0x987dc4); // "Please insert the CM01/02 CD and click on OK"
        exe::gui_show_message_box(title.as_ptr() as usize, message.as_ptr() as usize, 1);
    }
}

/// The source path of news.cpp in the "Error" box ("E:\dev\CM3\cm3 00-01\cm3\code\news.cpp").
const NEWS_CPP_PATH: usize = 0xa693ec;
/// The save item "news.dat".
const NEWS_DAT: usize = 0xa69490;
/// The news item size (223 bytes) and the inbox record size (start, position, count).
const NEWS_ITEM_SIZE: u32 = 0xdf;
const NEWS_INBOX_SIZE: u32 = 0xc;
/// The inbox count of a new store: one per human manager slot.
const NEWS_INBOXES: u32 = 0x10;

/// The news.cpp "Error" box at `row` that stops the game (the fatal cleanup and exit(-1)).
unsafe fn news_fatal(row: u32) -> ! {
    show_error_box(NEWS_CPP_PATH, row);
    fatal_exit()
}

/// Builds the news store: first the game CD (`game_cd_check`), then, with a save, the store read from it
/// (`News::load_from_zip` through the exe); without a save or when the reading fails, an empty store: no items
/// (`news_fields::SLOT_COUNT` 0 and the item pointer 0), 16 inboxes of 12 bytes (`malloc(0xc0)`, each start and
/// position −1 and count 0; no memory: the fatal box at line 0x1ef9) and the last news id 0. Returns the store.
/// Defect of the original: a store whose reading failed halfway keeps the buffers it had allocated (they leak).
/// Original: `News::News` (0x785b20, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_news(this: usize, save: usize) -> usize {
    game_cd_check(0x785b5a, 0x785b9e);
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x787780usize); // News::load_from_zip
    if save != 0 && load(this, save) != 0 {
        covered!("sx16_7 news from the save");
        return this;
    }
    covered!("sx16_7 empty news store");
    write_u32(this + news_fields::SLOT_COUNT, 0);
    write_usize(this, 0); // the item store
    write_u32(this + 0xc, NEWS_INBOXES); // the inbox count
    let inboxes = exe::crt_malloc(NEWS_INBOXES * NEWS_INBOX_SIZE);
    write_usize(this + news_fields::HUMAN_INBOXES, inboxes);
    if inboxes == 0 {
        news_fatal(0x1ef9);
    }
    for k in 0..read_i32(this + 0xc).max(0) as usize {
        let inbox = inboxes + k * NEWS_INBOX_SIZE as usize;
        write_i32(inbox, -1);
        write_i32(inbox + 4, -1);
        write_u32(inbox + 8, 0);
    }
    write_u32(this + news_fields::LAST_ID, 0);
    this
}

/// Comparer for `new_news`: it asks for the CD, allocates and may read the save, so it cannot run twice; only the
/// original runs. The copy is proven by the new career (Rust mode) and by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_new_news(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_NEWS)(this, save)
}

/// Frees the news store: the item store (its pointer and `news_fields::SLOT_COUNT` zeroed) and the inboxes (pointer
/// and count zeroed), each when present.
/// Original: `News::~News` (0x785d00, news.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_news(this: usize) {
    let items = read_usize(this);
    if items != 0 {
        exe::crt_free(items);
        write_usize(this, 0);
        write_u32(this + news_fields::SLOT_COUNT, 0);
    }
    let inboxes = read_usize(this + news_fields::HUMAN_INBOXES);
    if inboxes != 0 {
        exe::crt_free(inboxes);
        write_usize(this + news_fields::HUMAN_INBOXES, 0);
        write_u32(this + 0xc, 0); // the inbox count
    }
}

/// Comparer for `drop_news`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_NEWS)(this)
}

/// Reads the news store from the save's "news.dat": the item count (`news_fields::SLOT_COUNT`), the 223-byte items
/// (`malloc`; none, the store pointer 0), the inbox count, the 12-byte inboxes and the last news id; closes the item
/// and answers 1. Without a save (box line 0x2500) or without the item (line 0x2508): the "Error" box and 0. A short
/// read or a failed allocation stops the game (fatal box, lines 0x250f, 0x251b, 0x2523, 0x2531, 0x253a, 0x2542,
/// 0x254a).
/// Original: `News::load_from_zip` (0x787780, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn news_load_from_zip(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(NEWS_CPP_PATH, 0x2500);
        return 0;
    }
    let mut z = exe::zopen(save, NEWS_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_box(NEWS_CPP_PATH, 0x2508);
        return 0;
    }
    covered!("sx16_7 load news.dat");
    let count = this + news_fields::SLOT_COUNT;
    if exe::zread(count, 4, 1, z) != 1 {
        news_fatal(0x250f);
    }
    if read_u32(count) != 0 {
        let items = exe::crt_malloc(read_u32(count).wrapping_mul(NEWS_ITEM_SIZE));
        write_usize(this, items);
        if items == 0 {
            news_fatal(0x251b);
        }
        if exe::zread(items, NEWS_ITEM_SIZE, read_u32(count), z) != read_u32(count) {
            news_fatal(0x2523);
        }
    } else {
        write_usize(this, 0);
    }
    let inbox_count = this + 0xc;
    if exe::zread(inbox_count, 4, 1, z) != 1 {
        news_fatal(0x2531);
    }
    let inboxes = exe::crt_malloc(read_u32(inbox_count).wrapping_mul(NEWS_INBOX_SIZE));
    write_usize(this + news_fields::HUMAN_INBOXES, inboxes);
    if inboxes == 0 {
        news_fatal(0x253a);
    }
    if exe::zread(inboxes, NEWS_INBOX_SIZE, read_u32(inbox_count), z) != read_u32(inbox_count) {
        news_fatal(0x2542);
    }
    if exe::zread(this + news_fields::LAST_ID, 4, 1, z) != 1 {
        news_fatal(0x254a);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `news_load_from_zip`: it reads the save item and allocates, so only the original runs. The copy is
/// proven by loading the holiday save (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_news_load_from_zip(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NEWS_LOAD_FROM_ZIP)(this, save)
}

// --- sx16_8: the notes manager: Notes::Notes (0x794ad0), Notes::~Notes (0x794ba0), Notes::load_notes (0x794be0) ---

/// The record sizes of a notes slot's two lists: the staff notes (0x114 bytes) and the second list's records.
const NOTES_FIRST_RECORD_SIZE: u32 = 0x114;
const NOTE_SECOND_SIZE: u32 = 0x10c;
/// The source path of notes.cpp in the "Error" box (the save routines; the screens' item has its own name for it).
const NOTES_SAVE_SOURCE: usize = 0xa69a18;
/// The save item "notes.dat".
const NOTES_DAT: usize = 0xa69a40;
/// The game data version from which saves carry notes.dat (above it, as an absolute value).
const NOTES_FIRST_VERSION: i32 = 0x3e032;

/// Builds the notes manager: the 16 manager slots get their two empty note lists (`operator new` of 14 bytes and
/// `LLIST::LLIST`; a failed allocation leaves the pointer 0); with a save of a game data version above 0x3e032 (the
/// absolute `G_GAME_DATA_VERSION`), the notes are read from it (`Notes::load_notes` through the exe; its answer is not
/// used). Returns the manager.
/// Original: `Notes::Notes` (0x794ad0, notes.cpp, thiscall, ret 4; SEH frame for the list allocations, nothing thrown).
pub(crate) unsafe extern "thiscall" fn new_notes(this: usize, save: usize) -> usize {
    for k in 0..0x20usize {
        let list = exe::crt_operator_new(0xe);
        write_usize(this + k * 4, if list != 0 { exe::new_llist(list) } else { 0 });
    }
    if save != 0 && read_i32(G_GAME_DATA_VERSION).wrapping_abs() > NOTES_FIRST_VERSION {
        covered!("sx16_8 notes from the save");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x794be0usize); // Notes::load_notes
        load(this, save);
    }
    this
}

/// Comparer for `new_notes`: it allocates (and may read the save), so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_notes(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_NOTES)(this, save)
}

/// Destroys the notes manager: each of the 32 note lists present is emptied (`LLIST::~LLIST`) and freed (`operator
/// delete`); the pointers stay.
/// Original: `Notes::~Notes` (0x794ba0, notes.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_notes(this: usize) {
    for k in 0..0x20usize {
        let list = read_usize(this + k * 4);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
    }
}

/// Comparer for `drop_notes`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_notes(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_NOTES)(this)
}

/// The notes.cpp "Error" box at `row` of a failed save item, the item closed: the answer 0.
unsafe fn notes_item_error(row: u32, z: &mut usize) -> u8 {
    show_error_box(NOTES_SAVE_SOURCE, row);
    exe::zclose(z);
    0
}

/// Reads notes.dat from a saved game: nothing (0) without a save or without the item. For each of the 16 manager
/// slots: the count of the first list and its 0x114-byte notes, each appended (`LLIST::add_tail`); the count of the
/// second list and its 0x10c-byte records, appended to it. Closes the item and answers 1. A short read shows the
/// notes.cpp "Error" box (counts lines 0x73 and 0x88, records 0x7d and 0x91), closes the item and answers 0 (the notes
/// read so far stay).
/// Original: `Notes::load_notes` (0x794be0, notes.cpp, thiscall, ret 4; SEH frame for local dates, nothing thrown).
pub(crate) unsafe extern "thiscall" fn notes_load_notes(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, NOTES_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx16_8 load notes.dat");
    let mut record = [0u8; NOTES_FIRST_RECORD_SIZE as usize];
    for k in 0..0x10usize {
        for (list, size, row_count, row_record) in [(this + k * 8, NOTES_FIRST_RECORD_SIZE, 0x73, 0x7d), (this + k * 8 + 4, NOTE_SECOND_SIZE, 0x88, 0x91)] {
            let mut count = 0i32;
            if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
                return notes_item_error(row_count, &mut z);
            }
            for _ in 0..count.max(0) {
                if exe::zread(record.as_mut_ptr() as usize, size, 1, z) != 1 {
                    return notes_item_error(row_record, &mut z);
                }
                exe::llist_add_tail(read_usize(list), record.as_ptr() as usize, size);
            }
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `notes_load_notes`: it reads the save item and allocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_notes_load_notes(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_NOTES_LOAD_NOTES)(this, save)
}

// --- sx16_9: saving news.dat (0x787c30), notes.dat (0x794f00) and retire.dat (0x7ad7c0) ---

/// The news.cpp "Error" box at `row` that stops the game while saving (the fatal cleanup and exit(-1)).
unsafe fn news_save_fatal(row: u32) -> ! {
    show_error_box(0xa693ec, row); // news.cpp
    fatal_exit()
}

/// Writes the news store to "news.dat" in the save: the item count (`news_fields::SLOT_COUNT`), the 223-byte items (when
/// any), the inbox count, the 12-byte inboxes and the last news id; closes the item and answers 1. Without a save:
/// the "Error" box (line 0x2565) and 0. Not opening the item or a short write stops the game (fatal box, lines 0x256d,
/// 0x2574, 0x257f, 0x2588, 0x2590, 0x2598).
/// Original: `News::save_to_zip` (0x787c30, news.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn news_save_to_zip(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(0xa693ec, 0x2565);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa69490usize, 1, 0x16, 4); // "news.dat"
    if z == 0 {
        news_save_fatal(0x256d);
    }
    covered!("sx16_9 save news.dat");
    let count = this + news_fields::SLOT_COUNT;
    if exe::zwrite(count, 4, 1, z) != 1 {
        news_save_fatal(0x2574);
    }
    let n = read_u32(count);
    if n != 0 && exe::zwrite(read_usize(this), 0xdf, n, z) != n {
        news_save_fatal(0x257f);
    }
    let inbox_count = this + 0xc;
    if exe::zwrite(inbox_count, 4, 1, z) != 1 {
        news_save_fatal(0x2588);
    }
    let n = read_u32(inbox_count);
    if exe::zwrite(read_usize(this + news_fields::HUMAN_INBOXES), 0xc, n, z) != n {
        news_save_fatal(0x2590);
    }
    if exe::zwrite(this + news_fields::LAST_ID, 4, 1, z) != 1 {
        news_save_fatal(0x2598);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `news_save_to_zip`: it writes to the save container, so only the original runs; the copy is proven by
/// the save proof.
pub(crate) unsafe extern "thiscall" fn compare_news_save_to_zip(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NEWS_SAVE_TO_ZIP)(this, save)
}

/// The notes.cpp "Error" box at `row` while saving, the item closed: the answer 0.
unsafe fn notes_save_error(row: u32, z: &mut usize) -> u8 {
    show_error_box(0xa69a18, row); // notes.cpp
    exe::zclose(z);
    0
}

/// Writes notes.dat to the saved game: for each of the 16 manager slots, the count of the first list (its signed
/// word) and its 0x114-byte notes, then the count of the second list and its 0x10c-byte records, each list walked from
/// its head (`LLIST::get_next`); closes the item and answers 1. Without a save (box line 0xb2) or not opening the item
/// (0xba): the "Error" box and 0. A failed write (counts 0xc4 and 0xdd, records 0xd4 and 0xed) or a list shorter than
/// its count (0xce, 0xe7): the box, the item closed and 0.
/// Original: `Notes::save_notes` (0x794f00, notes.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn notes_save_notes(this: usize, save: usize) -> u8 {
    if save == 0 {
        show_error_box(0xa69a18, 0xb2);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa69a40usize, 1, 0x16, 4); // "notes.dat"
    if z == 0 {
        show_error_box(0xa69a18, 0xba);
        return 0;
    }
    covered!("sx16_9 save notes.dat");
    for k in 0..0x10usize {
        for (slot, size, row_count, row_missing, row_record) in [(k * 8, 0x114u32, 0xc4, 0xce, 0xd4), (k * 8 + 4, 0x10c, 0xdd, 0xe7, 0xed)] {
            let list = read_usize(this + slot);
            let mut count = read_i16(list + llist::COUNT) as i32;
            if exe::zwrite(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
                return notes_save_error(row_count, &mut z);
            }
            write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
            let mut i = 0i32;
            while i < read_i16(read_usize(this + slot) + llist::COUNT) as i32 {
                let record = exe::llist_get_next(read_usize(this + slot));
                if record == 0 {
                    return notes_save_error(row_missing, &mut z);
                }
                if exe::zwrite(record, size, 1, z) != 1 {
                    return notes_save_error(row_record, &mut z);
                }
                i += 1;
            }
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `notes_save_notes`: only the original (it writes to the save container).
pub(crate) unsafe extern "thiscall" fn compare_notes_save_notes(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_NOTES_SAVE_NOTES)(this, save)
}

/// The four regen tables of retire.dat in their order: (table, count, record size, the player_regen.cpp lines of the
/// count and of the records when saving).
const RETIRE_DAT_SAVE: [(usize, usize, u32, u32, u32); 4] = [
    (player_regen_fields::RETIREMENTS, player_regen_fields::RETIREMENT_COUNT, 0x1a, 0x1a55, 0x1a5c),
    (player_regen_fields::REGEN_QUEUE, player_regen_fields::REGEN_QUEUE_COUNT, 0xc, 0x1a63, 0x1a6a),
    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, 0x10, 0x1a71, 0x1a78),
    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, 0x10, 0x1a7f, 0x1a86),
];

/// Writes the four regen tables (`RETIRE_DAT_SAVE`) to "retire.dat" in the save, each as its count and then its
/// records (when any); closes the item. Without a save (line 0x1a46) or not opening the item (0x1a4e): the
/// player_regen.cpp "Error" box and nothing more.
/// Defect of the original: a failed write shows the box and closes the item but goes on writing the rest into the
/// closed item (more boxes), and the item is closed again at the end.
/// Original: `PLAYER_REGEN::save_retire_dat` (0x7ad7c0, player_regen.cpp, thiscall, ret 4; no answer).
pub(crate) unsafe extern "thiscall" fn save_retire_dat(this: usize, save: usize) {
    const SOURCE: usize = 0xa6bf9c; // player_regen.cpp
    if save == 0 {
        show_error_box(SOURCE, 0x1a46);
        return;
    }
    let mut z = exe::zopen(save, 0xa701a4usize, 1, 0x16, 4); // "retire.dat"
    if z == 0 {
        show_error_box(SOURCE, 0x1a4e);
        return;
    }
    covered!("sx16_9 save retire.dat");
    for (table, count, size, row_count, row_records) in RETIRE_DAT_SAVE {
        if exe::zwrite(this + count, 4, 1, z) != 1 {
            show_error_box(SOURCE, row_count);
            exe::zclose(&mut z);
        }
        let n = read_u32(this + count);
        if n != 0 && exe::zwrite(read_usize(this + table), size, n, z) != n {
            show_error_box(SOURCE, row_records);
            exe::zclose(&mut z);
        }
    }
    exe::zclose(&mut z);
}

/// Comparer for `save_retire_dat`: only the original (it writes to the save container).
pub(crate) unsafe extern "thiscall" fn compare_save_retire_dat(this: usize, save: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SAVE_RETIRE_DAT)(this, save)
}

// --- sx16_10: the officials manager: OfficialsManager::OfficialsManager (0x79b580), ~OfficialsManager (0x79b800),
// load_official_stats (0x79b850) ---

/// The save item "off.dat".
const OFF_DAT: usize = 0xa6a260;
/// The game data version from which off.dat holds the 17-byte statistics rows (before it, 16-byte rows).
const OFF_DAT_17_BYTES_VERSION: i32 = 0x3e043;

/// The harshness code of an official from his discipline (`official::DISCIPLINE`, signed): above 18 harsh (1), above 12
/// strict (2), above 5 fair (3), otherwise lenient (4).
unsafe fn official_harshness(official: usize) -> u8 {
    match read_i8(official + official::DISCIPLINE) {
        d if d > 0x12 => 1,
        d if d > 0xc => 2,
        d if d > 5 => 3,
        _ => 4,
    }
}

/// Builds the officials manager: first the game CD (`game_cd_check`, item sx16_7); with a save whose off.dat reads
/// (`OfficialsManager::load_official_stats` through the exe), that is all. Otherwise the officials are set up
/// (`OfficialsManager::set_up_officials` through the exe) and a fresh 17-byte statistics row is made for each official
/// (`malloc`, `G_OFFICIAL_COUNT` rows): his id, the average grade, fouls, grade sum, matches and cards 0, the harshness
/// by his discipline (`official_harshness`) and, without a save, the last grade 0xff (none).
/// Defect of the original: when a save is given but its off.dat does not read, the rows are made without the last
/// grade (`official_stats::LAST_GRADE` keeps what `malloc` left).
/// Original: `OfficialsManager::OfficialsManager` (0x79b580, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_officials_manager(this: usize, save: usize) -> usize {
    game_cd_check(0x79b5c2, 0x79b604);
    if save != 0 {
        let load: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x79b850usize); // load_official_stats
        if load(this, save) != 0 {
            covered!("sx16_10 officials from the save");
            return this;
        }
    }
    covered!("sx16_10 fresh officials");
    let set_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x79bf60usize); // set_up_officials
    set_up(this);
    let count = read_i32(G_OFFICIAL_COUNT);
    let rows = exe::crt_malloc((count as u32).wrapping_mul(OFFICIAL_STATS_SIZE as u32));
    write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
    for k in 0..count.max(0) {
        let row = read_usize(this + officials_manager_fields::GRADE_ROWS) + k as usize * OFFICIAL_STATS_SIZE;
        write_i32(row, k); // the official's id
        write_u32(row + official_stats::AVERAGE_GRADE, 0);
        write_u16(row + official_stats::MATCHES, 0);
        write_u8(row + official_stats::CARDS, 0);
        write_u16(row + official_stats::GRADE_SUM, 0);
        write_u16(row + official_stats::FOULS, 0);
        if save == 0 {
            write_u8(row + official_stats::LAST_GRADE, 0xff);
        }
        write_u8(row + 0xe, official_harshness(official_record(k))); // the harshness code
    }
    this
}

/// Comparer for `new_officials_manager`: it asks for the CD, allocates and may read the save; only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_officials_manager(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_OFFICIALS_MANAGER)(this, save)
}

/// Frees the officials manager's buffers: the busy flags, the nation blocks and the statistics rows, each present one
/// freed and its pointer zeroed.
/// Original: `OfficialsManager::~OfficialsManager` (0x79b800, officials_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_officials_manager(this: usize) {
    for field in [officials_manager_fields::BUSY_FLAGS, officials_manager_fields::NATION_BLOCKS, officials_manager_fields::GRADE_ROWS] {
        let p = read_usize(this + field);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + field, 0);
        }
    }
}

/// Comparer for `drop_officials_manager`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_officials_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_OFFICIALS_MANAGER)(this)
}

/// Converts the 16-byte statistics rows of an old off.dat into the 17-byte rows: the id, the average grade, the fouls,
/// the grade sum, the matches, the harshness and the cards copied, the last grade 0xff.
unsafe fn officials_convert_old_rows(this: usize, old: usize, count: i32) {
    let rows = read_usize(this + officials_manager_fields::GRADE_ROWS);
    for k in 0..count.max(0) as usize {
        let (o, n) = (old + k * 0x10, rows + k * OFFICIAL_STATS_SIZE);
        write_u32(n, read_u32(o));
        write_u32(n + official_stats::AVERAGE_GRADE, read_u32(o + 4));
        write_u16(n + official_stats::GRADE_SUM, read_u16(o + 0xa));
        write_u16(n + official_stats::MATCHES, read_u16(o + 0xc));
        write_u8(n + 0xe, read_u8(o + 0xe)); // the harshness code
        write_u8(n + official_stats::CARDS, read_u8(o + 0xf));
        write_u16(n + official_stats::FOULS, read_u16(o + 8));
        write_u8(n + official_stats::LAST_GRADE, 0xff);
    }
}

/// Reads off.dat from a saved game: nothing (0) without a save or without the item. The official count
/// (`officials_manager_fields::BUSY_COUNT`; 0 answers 0); the statistics rows: 17 bytes from game data version 0x3e043
/// (absolute `G_GAME_DATA_VERSION`), before it 16-byte rows converted (`officials_convert_old_rows`); the nation count
/// and the 0x102-byte nation blocks; the busy flags (`calloc`, one byte per official; no memory: the fatal box at
/// line 0x125). Closes the item and answers 1; a short read or a failed allocation closes it and answers 0 (no box).
/// Defects of the original: the 17-byte rows' allocation and the nation blocks' are not checked; the old rows' buffer
/// leaks when their read fails.
/// Original: `OfficialsManager::load_official_stats` (0x79b850, officials_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_official_stats(this: usize, save: usize) -> u8 {
    if save == 0 {
        return 0;
    }
    let mut z = exe::zopen(save, OFF_DAT, 0, 0x16, 4);
    if z == 0 {
        return 0;
    }
    covered!("sx16_10 load off.dat");
    let fail = |z: &mut usize| {
        exe::zclose(z);
        0u8
    };
    let mut count = 0i32;
    if exe::zread(&mut count as *mut i32 as usize, 4, 1, z) != 1 {
        return fail(&mut z);
    }
    write_i32(this + officials_manager_fields::BUSY_COUNT, count);
    if count == 0 {
        return fail(&mut z);
    }
    let n = count as u32;
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() < OFF_DAT_17_BYTES_VERSION {
        covered!("sx16_10 old off.dat rows");
        let old = exe::crt_malloc(n.wrapping_mul(0x10));
        if old == 0 || exe::zread(old, 0x10, n, z) != n {
            return fail(&mut z);
        }
        let rows = exe::crt_malloc(n.wrapping_mul(OFFICIAL_STATS_SIZE as u32));
        write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
        if rows == 0 {
            return fail(&mut z);
        }
        officials_convert_old_rows(this, old, count);
        exe::crt_free(old);
    } else {
        let rows = exe::crt_malloc(n.wrapping_mul(OFFICIAL_STATS_SIZE as u32));
        write_usize(this + officials_manager_fields::GRADE_ROWS, rows);
        if exe::zread(rows, OFFICIAL_STATS_SIZE as u32, n, z) != n {
            return fail(&mut z);
        }
    }
    let mut nations = 0u32;
    if exe::zread(&mut nations as *mut u32 as usize, 4, 1, z) != 1 {
        return fail(&mut z);
    }
    let blocks = exe::crt_malloc(nations.wrapping_mul(0x102));
    write_usize(this + officials_manager_fields::NATION_BLOCKS, blocks);
    if exe::zread(blocks, 0x102, nations, z) != nations {
        return fail(&mut z);
    }
    let busy = crt_calloc(read_u32(this + officials_manager_fields::BUSY_COUNT), 1);
    write_usize(this + officials_manager_fields::BUSY_FLAGS, busy);
    if busy == 0 {
        officials_out_of_memory(0x125);
    }
    if exe::zread(busy, 1, n, z) != n {
        return fail(&mut z);
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_official_stats`: it reads the save item and allocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_load_official_stats(this: usize, save: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(IDX_LOAD_OFFICIAL_STATS)(this, save)
}

// --- sx16_11: the regen manager: PLAYER_REGEN::PLAYER_REGEN (0x7a5380), ~PLAYER_REGEN (0x7a56f0),
// load_retire_dat (0x7adcd0) ---

/// The four regen tables in their order (retire.dat and the constructor): (table, count, capacity, record size, the
/// constructor's out-of-memory line, the loading lines of the count, the allocation and the records).
const REGEN_TABLES: [(usize, usize, usize, u32, u32, [u32; 3]); 4] = [
    (player_regen_fields::RETIREMENTS, player_regen_fields::RETIREMENT_COUNT, player_regen_fields::RETIREMENT_CAPACITY, 0x1a, 0x7f, [0x1aac, 0x1ab6, 0x1abe]),
    (player_regen_fields::REGEN_QUEUE, player_regen_fields::REGEN_QUEUE_COUNT, player_regen_fields::REGEN_QUEUE_CAPACITY, 0xc, 0x83, [0x1ac7, 0x1ad1, 0x1ad9]),
    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, player_regen_fields::SPARE_NON_PLAYER_CAPACITY, 0x10, 0x87, [0x1ae2, 0x1aec, 0x1af4]),
    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, player_regen_fields::PLAYER_DATA_POOL_CAPACITY, 0x10, 0x8b, [0x1afd, 0x1b07, 0x1b0f]),
];
/// The room the regen tables get beyond their count (250 records).
const REGEN_TABLE_SLACK: u32 = 0xfa;
/// The source path of player_regen.cpp in the "Error" box.
const PLAYER_REGEN_CPP_PATH: usize = 0xa6bf9c;

/// Builds the regen manager: first the game CD (`game_cd_check`, item sx16_7); the four tables empty with room for 250
/// records each (`malloc`, `REGEN_TABLES`); the per-nation club lists (`build_nation_club_lists` through the exe). A new
/// game: the retirement list built fresh (`build_initial_retirement_list`) and the last ten non-player rows
/// (`G_NON_PLAYER_COUNT` − k − 1) added as spare records with no nations (`add_spare_non_player_record(row, −1, −1)`); a
/// save: the tables read from retire.dat (`load_retire_dat`, its answer unused), all through the exe. Then a table left
/// without memory stops the game (player_regen.cpp fatal box, lines 0x7f, 0x83, 0x87, 0x8b). Returns the manager.
/// Defect of the original: the allocations are tested only after they were used.
/// Original: `PLAYER_REGEN::PLAYER_REGEN` (0x7a5380, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn new_player_regen(this: usize, save: usize) -> usize {
    game_cd_check(0x7a53c9, 0x7a540d);
    for (table, count, capacity, size, _, _) in REGEN_TABLES {
        write_usize(this + table, exe::crt_malloc(size * REGEN_TABLE_SLACK));
        write_u32(this + capacity, REGEN_TABLE_SLACK);
        write_u32(this + count, 0);
    }
    let call1 = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize) = core::mem::transmute(f);
        g(this)
    };
    call1(0x7acfc0); // build_nation_club_lists
    if save == 0 {
        covered!("sx16_11 new game regens");
        call1(0x7a58f0); // build_initial_retirement_list
        let add: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x7a81a0usize); // add_spare_non_player_record
        for k in 0..0xa {
            add(this, (read_i32(G_NON_PLAYER_COUNT) - k - 1) as u32, u32::MAX, u32::MAX);
        }
    } else {
        covered!("sx16_11 regens from the save");
        let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7adcd0usize); // load_retire_dat
        load(this, save);
    }
    for (table, _, _, _, row, _) in REGEN_TABLES {
        if read_usize(this + table) == 0 {
            show_error_box(PLAYER_REGEN_CPP_PATH, row);
            fatal_exit();
        }
    }
    this
}

/// Comparer for `new_player_regen`: it asks for the CD, allocates and builds the lists (or reads the save); only the
/// original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_player_regen(this: usize, save: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_NEW_PLAYER_REGEN)(this, save)
}

/// Frees the regen manager: the four tables (each present one freed and its pointer zeroed) and, when there are any,
/// the per-nation club lists (`G_NATION_COUNT` of them: each present list emptied, `LLIST::~LLIST`, and deleted), then
/// their array (freed and zeroed).
/// Original: `PLAYER_REGEN::~PLAYER_REGEN` (0x7a56f0, player_regen.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn drop_player_regen(this: usize) {
    for (table, _, _, _, _, _) in REGEN_TABLES {
        let p = read_usize(this + table);
        if p != 0 {
            exe::crt_free(p);
            write_usize(this + table, 0);
        }
    }
    let lists = read_usize(this); // the per-nation club lists
    if lists == 0 {
        return;
    }
    let mut k = 0i32;
    while k < read_i32(G_NATION_COUNT) {
        let list = read_usize(read_usize(this) + k as usize * 4);
        if list != 0 {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        k += 1;
    }
    exe::crt_free(read_usize(this));
    write_usize(this, 0);
}

/// Comparer for `drop_player_regen`: it frees, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_player_regen(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_PLAYER_REGEN)(this)
}

/// Reads the four regen tables from "retire.dat" in the save: for each (`REGEN_TABLES`), the count, the capacity count
/// + 250 and the table reallocated to it (`realloc`), then the records (when any). Closes the item and answers 1.
/// Without a save (line 0x1a9d) or the item (0x1aa5): the player_regen.cpp "Error" box and 0. A short read or a failed
/// reallocation: the box, the item closed and 0.
/// Original: `PLAYER_REGEN::load_retire_dat` (0x7adcd0, player_regen.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_retire_dat(this: usize, save: usize) -> u32 {
    if save == 0 {
        show_error_box(PLAYER_REGEN_CPP_PATH, 0x1a9d);
        return 0;
    }
    let mut z = exe::zopen(save, 0xa701a4usize, 0, 0x16, 4); // "retire.dat"
    if z == 0 {
        show_error_box(PLAYER_REGEN_CPP_PATH, 0x1aa5);
        return 0;
    }
    covered!("sx16_11 load retire.dat");
    for (table, count, capacity, size, _, [row_count, row_alloc, row_records]) in REGEN_TABLES {
        let fail = |row: u32, z: &mut usize| {
            show_error_box(PLAYER_REGEN_CPP_PATH, row);
            exe::zclose(z);
            0u32
        };
        if exe::zread(this + count, 4, 1, z) != 1 {
            return fail(row_count, &mut z);
        }
        let room = read_u32(this + count).wrapping_add(REGEN_TABLE_SLACK);
        write_u32(this + capacity, room);
        let records = exe::crt_realloc(read_usize(this + table), room.wrapping_mul(size));
        write_usize(this + table, records);
        if records == 0 {
            return fail(row_alloc, &mut z);
        }
        let n = read_u32(this + count);
        if n != 0 && exe::zread(records, size, n, z) != n {
            return fail(row_records, &mut z);
        }
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_retire_dat`: it reads the save item and reallocates, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_load_retire_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_RETIRE_DAT)(this, save)
}

// --- sx17_1: PLAYER_SEARCH::PLAYER_SEARCH (0x7ae390), PLAYER_SEARCH::save (0x7b2360) and PLAYER_SEARCH::load (0x7b17e0), player_search.cpp ---

/// A field of the PLAYER_SEARCH record as the saved game stores it: offset, element size, element count and the
/// player_search.cpp "Error" box line of a short write (`player_search_save`) or read (`player_search_load`).
type PlayerSearchSaveField = (usize, u32, u32, u32);

/// The size of the scouted club record (+0x170): the club id and 50 scouted player slots of 0x1a2 bytes.
const PLAYER_SEARCH_CLUB_RECORD_SIZE: u32 = 0x51a8;

/// Builds an empty player search: the dates (+0x184, the scouting start and end, +0x1b6, the competition date) and the
/// two filter records constructed, then the owner −1, the slot −1, the saved file magic +5 = 0x24a9e and format version
/// +9 = 1 (`player_search_read_file_header` checks both), the list type 4, +0xb 0, the manager name (+0x70) and password (+0x139) empty, +0x16c the game's random seed, no scout, no
/// assignment (nation, club, region, competition, target club −1; base and opponent flags 0), the scouting start
/// 27/04/1969 [inference: the date constructor's arguments are day 27, month 4, year 1969], no reports, no results
/// (array, capacity and count 0), the report mark 1, +0x2a8 and +0x2ac 0 and no scouted club record. Returns `this`.
/// Original: `PLAYER_SEARCH::PLAYER_SEARCH` (0x7ae390, player_search.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn new_player_search(this: usize) -> usize {
    for date in [0x184, player_search_fields::SCOUTING_START_DATE, player_search_fields::SCOUTING_END_DATE, 0x1b6, SCOUT_DAY_COMPETITION_DATE] {
        exe::date_date_54a110(this + date);
    }
    finder_call0(FINDER_FILTERS_SET_DEFAULTS, this + 0x1fa); // the SearchFilters record (0x99 bytes)
    search_filter_record_init(this + 0x293);
    write_i32(this + SCOUT_DAY_OWNER, -1);
    write_i8(this + SCOUT_DAY_SLOT, -1);
    write_u32(this + 5, 0x24a9e); // the saved file magic
    write_u8(this + 9, 1); // the saved file format version
    write_u8(this + player_search_fields::LIST_TYPE, 4); // sx17_1: the default list type
    write_u8(this + 0xb, 0);
    write_u8(this + 0x70, 0); // the manager's name (201 bytes)
    write_u8(this + 0x139, 0); // the manager's password
    write_u32(this + 0x16c, read_u32(G_GAME_RANDOM_SEED));
    write_i32(this + player_search_fields::SCOUT_ID, -1);
    write_i32(this + player_search_fields::ASSIGNED_NATION, -1);
    write_i8(this + player_search_fields::ASSIGNED_REGION, -1);
    write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
    write_i32(this + player_search_fields::ASSIGNED_COMP, -1);
    write_u32(this + player_search_fields::ASSIGNED_BASE, 0);
    write_u32(this + player_search_fields::ASSIGNED_OPPONENT, 0);
    write_i32(this + SCOUT_DAY_TARGET_CLUB, -1);
    let mut date = [0u16; 4];
    let start = exe::date_date_549f70(&mut date, 0x1b, 4, 0x7b1, u32::MAX);
    let mut copy = [0u8; 8];
    exe::date_operator_3d(this + player_search_fields::SCOUTING_START_DATE, copy.as_mut_ptr() as usize, start);
    write_u8(this + SCOUT_DAY_REPORTS_DONE, 0);
    write_usize(this + player_search_fields::RESULTS, 0);
    write_u32(this + SCOUT_DAY_RESULT_CAPACITY, 0);
    write_u32(this + player_search_fields::RESULT_COUNT, 0);
    write_u32(this + SCOUT_DAY_REPORT_MARK, 1);
    write_u32(this + 0x2a8, 0);
    write_u32(this + 0x2ac, 0);
    write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, 0);
    this
}

/// Comparer for `new_player_search`: side by side over the whole record (0x2b0 bytes) and the return; the bytes the
/// constructor does not write keep what was there on both sides.
pub(crate) unsafe extern "thiscall" fn compare_new_player_search(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_NEW_PLAYER_SEARCH);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&[(this, 0x2b0)], || o(this), || new_player_search(this));
    SCOREBOARD.record(same && r == n, || format!("new_player_search({:#x}) {:?}", this, first_differences(&mo, &mr)));
    r
}

/// The fields `player_search_save` writes before the result list, in order.
const PLAYER_SEARCH_SAVED_HEAD: [PlayerSearchSaveField; 17] = [
    (SCOUT_DAY_OWNER, 4, 1, 0x90c), (SCOUT_DAY_SLOT, 1, 1, 0x913), (5, 0x16b, 1, 0x91a),
    (player_search_fields::SCOUT_NAME_REF, 0x18, 1, 0x921), (player_search_fields::ASSIGNED_CLUB, 4, 1, 0x928),
    (player_search_fields::ASSIGNED_NATION, 4, 1, 0x92f), (player_search_fields::ASSIGNED_REGION, 1, 1, 0x936),
    (player_search_fields::ASSIGNED_COMP, 4, 1, 0x93d), (player_search_fields::ASSIGNED_BASE, 4, 1, 0x944),
    (player_search_fields::ASSIGNED_OPPONENT, 4, 1, 0x94b), (SCOUT_DAY_TARGET_CLUB, 4, 1, 0x953),
    (player_search_fields::SCOUTING_END_DATE, 8, 1, 0x95a), (player_search_fields::SCOUTING_START_DATE, 8, 1, 0x961),
    (SCOUT_DAY_REPORTS_DONE, 1, 1, 0x968), (0x1b6, 8, 1, 0x96f), (SCOUT_DAY_COMPETITION_DATE, 8, 1, 0x976),
    (0x1c6, 4, 10, 0x97d), // ten dwords
];

/// The fields `player_search_save` writes after the result list: the SearchFilters record, the filter record, the
/// report mark, +0x2a8 and +0x2ac.
const PLAYER_SEARCH_SAVED_TAIL: [PlayerSearchSaveField; 5] = [
    (0x1fa, 0x99, 1, 0x9a0), (0x293, 0x11, 1, 0x9a7), (SCOUT_DAY_REPORT_MARK, 4, 1, 0x9ae), (0x2a8, 4, 1, 0x9b5),
    (0x2ac, 4, 1, 0x9bc),
];

/// Writes one search into the saved game (`zwrite` to the open item `save`): no item, the player_search.cpp "Error"
/// box (line 0x905) and 0. The fields of `PLAYER_SEARCH_SAVED_HEAD` in order (+5..+0x16f is the header a saved search
/// file also starts with); then the invalid results are dropped
/// (`purge_invalid_results` without relinking), the result capacity and count, the results themselves (0x2d bytes
/// each) when there are any, and the fields of `PLAYER_SEARCH_SAVED_TAIL`. A short write shows the box with the field's
/// line and answers 0. Last, with an assigned club (> −1) and its scouted club record, the record (0x51a8 bytes); a short
/// write there shows the box (line 0x1ffb) but still answers 1.
/// Original: `PLAYER_SEARCH::save` (0x7b2360, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_save(this: usize, save: usize) -> u32 {
    let fail = |line: u32| {
        show_error_box(0xa701b0, line); // player_search.cpp
        0
    };
    if save == 0 {
        return fail(0x905);
    }
    for (offset, size, count, line) in PLAYER_SEARCH_SAVED_HEAD {
        if exe::zwrite(this + offset, size, count, save) != count {
            return fail(line);
        }
    }
    exe::player_search_purge_invalid_results(this, 0);
    if exe::zwrite(this + SCOUT_DAY_RESULT_CAPACITY, 4, 1, save) != 1 {
        return fail(0x987);
    }
    let count = this + player_search_fields::RESULT_COUNT;
    if exe::zwrite(count, 4, 1, save) != 1 {
        return fail(0x98e);
    }
    if read_i32(count) > 0 {
        covered!("sx17_1 save the results");
        if exe::zwrite(read_usize(this + player_search_fields::RESULTS), SCOUT_DAY_RESULT_SIZE as u32, read_u32(count), save) != read_u32(count) {
            return fail(0x998);
        }
    }
    for (offset, size, count, line) in PLAYER_SEARCH_SAVED_TAIL {
        if exe::zwrite(this + offset, size, count, save) != count {
            return fail(line);
        }
    }
    let record = read_usize(this + player_search_fields::SCOUTED_CLUB_RECORD);
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 && record != 0 {
        covered!("sx17_1 save the scouted club record");
        if exe::zwrite(record, PLAYER_SEARCH_CLUB_RECORD_SIZE, 1, save) != 1 {
            show_error_box(0xa701b0, 0x1ffb); // player_search.cpp
        }
    }
    1
}

/// Comparer for `player_search_save`: it writes into the save item, so it cannot run twice; only the original runs.
/// The copy is proven by the save proof (the .qst written with and without the batch).
pub(crate) unsafe extern "thiscall" fn compare_player_search_save(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PLAYER_SEARCH_SAVE)(this, save)
}

/// True when the absolute game data version (`G_GAME_DATA_VERSION`) is above `version`: the fields added later are
/// only in saves from then on.
unsafe fn player_search_data_newer_than(version: i32) -> bool {
    read_i32(G_GAME_DATA_VERSION).wrapping_abs() > version
}

/// Reads the results of an old save (game data up to 0x3e032), 0x2c bytes each, into the 0x2d-byte entries: the
/// name ref (+0..+0xc), the two dates (`Date::operator=`), the flags +0x20, the ratings +0x21 and +0x25 and the bytes
/// +0x29..+0x2b; the new byte +0x2c is 0xff. The old buffer (`malloc`, not checked) is freed after the conversion;
/// answers false on a short read (the buffer is then left allocated, like the original).
unsafe fn player_search_load_old_results(this: usize, save: usize) -> bool {
    let old = exe::crt_malloc(read_u32(this + SCOUT_DAY_RESULT_CAPACITY).wrapping_mul(0x2c));
    let n = read_u32(this + player_search_fields::RESULT_COUNT);
    if exe::zread(old, 0x2c, n, save) != n {
        return false;
    }
    let mut k = 0usize;
    while (k as i32) < read_i32(this + player_search_fields::RESULT_COUNT) {
        let (from, to) = (old + k * 0x2c, read_usize(this + player_search_fields::RESULTS) + k * SCOUT_DAY_RESULT_SIZE);
        let mut copy = [0u8; 8];
        write_u8(to + 0x29, read_u8(from + 0x29));
        write_u32(to + 0x21, read_u32(from + 0x21));
        exe::date_operator_3d(to + search_result::LAST_UPDATE_DATE, copy.as_mut_ptr() as usize, from + search_result::LAST_UPDATE_DATE);
        write_u8(to + 0x2a, read_u8(from + 0x2a));
        write_u32(to + 0x25, read_u32(from + 0x25));
        for d in [0, 4, 8, 0xc] {
            write_u32(to + d, read_u32(from + d));
        }
        exe::date_operator_3d(to + 0x10, copy.as_mut_ptr() as usize, from + 0x10);
        write_u8(to + 0x2b, read_u8(from + 0x2b));
        write_u8(to + search_result::FLAGS, read_u8(from + search_result::FLAGS));
        write_u8(to + 0x29, read_u8(from + 0x29));
        write_u8(to + 0x2c, 0xff); // the byte old saves did not have
        k += 1;
    }
    exe::crt_free(old);
    true
}

/// Reads one search from the saved game (`zread` from the open item `save`): no item, the player_search.cpp "Error"
/// box (line 0x7fd) and 0. In order: the owner, the slot, +5 (0x16b bytes), the scout's name ref, the assigned club
/// (below −1 becomes −1), nation, region and competition; the base flag only from game data 0x3e011 on, the opponent
/// flag and the target club only from 0x3e03a on; the scouting end, the start (from 0x3e011 on), the reports, +0x1b6,
/// the competition date and the ten dwords at +0x1c6; the result capacity and count. The old result array is freed;
/// with a capacity, a new one (`malloc`, 0x2d bytes each; when it fails: the box, line 0x89f, `game_fatal_cleanup` and
/// `exit(-1)`), and with a count the results: 0x2d bytes each from game data 0x3e033 on, else the old 0x2c-byte
/// entries converted (`player_search_load_old_results`). Then the SearchFilters record, the filter record, the report
/// mark, +0x2a8 and +0x2ac. A short read shows the box with the field's line and answers 0. Last, with an assigned club
/// (> −1): a new scouted club record (`operator new`, 0x51a8 bytes) read from the save and added to the global list of
/// club records (`G_SCOUT_CLUB_TABLE_LIST`); a short read shows the box (line 0x1fc3) and the record is kept but not
/// listed; a failed allocation leaves 0. Answers 1.
/// Original: `PLAYER_SEARCH::load` (0x7b17e0, player_search.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn player_search_load(this: usize, save: usize) -> u32 {
    let fail = |line: u32| {
        show_error_box(0xa701b0, line); // player_search.cpp
        0
    };
    if save == 0 {
        return fail(0x7fd);
    }
    let read = |offset: usize, size: u32, count: u32| exe::zread(this + offset, size, count, save) == count;
    let head = [(SCOUT_DAY_OWNER, 4, 0x804), (SCOUT_DAY_SLOT, 1, 0x80b), (5, 0x16b, 0x812),
                (player_search_fields::SCOUT_NAME_REF, 0x18, 0x819), (player_search_fields::ASSIGNED_CLUB, 4, 0x820)];
    for (offset, size, line) in head {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) < -1 {
        write_i32(this + player_search_fields::ASSIGNED_CLUB, -1);
    }
    let assignment = [(player_search_fields::ASSIGNED_NATION, 4, 0x82a), (player_search_fields::ASSIGNED_REGION, 1, 0x831),
                      (player_search_fields::ASSIGNED_COMP, 4, 0x838)];
    for (offset, size, line) in assignment {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if player_search_data_newer_than(0x3e010) && !read(player_search_fields::ASSIGNED_BASE, 4, 1) {
        return fail(0x841);
    }
    if player_search_data_newer_than(0x3e039) {
        if !read(player_search_fields::ASSIGNED_OPPONENT, 4, 1) {
            return fail(0x84b);
        }
        if !read(SCOUT_DAY_TARGET_CLUB, 4, 1) {
            return fail(0x851);
        }
    }
    if !read(player_search_fields::SCOUTING_END_DATE, 8, 1) {
        return fail(0x859);
    }
    if player_search_data_newer_than(0x3e010) && !read(player_search_fields::SCOUTING_START_DATE, 8, 1) {
        return fail(0x862);
    }
    let middle = [(SCOUT_DAY_REPORTS_DONE, 1, 1, 0x86a), (0x1b6, 8, 1, 0x871), (SCOUT_DAY_COMPETITION_DATE, 8, 1, 0x878),
                  (0x1c6, 4, 10, 0x87f), (SCOUT_DAY_RESULT_CAPACITY, 4, 1, 0x886), (player_search_fields::RESULT_COUNT, 4, 1, 0x88d)];
    for (offset, size, count, line) in middle {
        if !read(offset, size, count) {
            return fail(line);
        }
    }
    let results = this + player_search_fields::RESULTS;
    if read_usize(results) != 0 {
        exe::crt_free(read_usize(results));
        write_usize(results, 0);
    }
    if read_i32(this + SCOUT_DAY_RESULT_CAPACITY) > 0 {
        let array = exe::crt_malloc(read_u32(this + SCOUT_DAY_RESULT_CAPACITY).wrapping_mul(SCOUT_DAY_RESULT_SIZE as u32));
        write_usize(results, array);
        if array == 0 {
            show_error_box(0xa701b0, 0x89f); // player_search.cpp
            exe::game_fatal_cleanup();
            exe::shared_noop_ret0_5a8f60();
            exe::crt_exit(-1);
        }
        let n = read_u32(this + player_search_fields::RESULT_COUNT);
        if (n as i32) > 0 {
            covered!("sx17_1 load the results");
            if player_search_data_newer_than(0x3e032) {
                if exe::zread(array, SCOUT_DAY_RESULT_SIZE as u32, n, save) != n {
                    return fail(0x8a9);
                }
            } else if !player_search_load_old_results(this, save) {
                return fail(0x8b3);
            }
        }
    }
    let tail = [(0x1fa, 0x99, 0x8ce), (0x293, 0x11, 0x8d5), (SCOUT_DAY_REPORT_MARK, 4, 0x8dc), (0x2a8, 4, 0x8e3), (0x2ac, 4, 0x8ea)];
    for (offset, size, line) in tail {
        if !read(offset, size, 1) {
            return fail(line);
        }
    }
    if read_i32(this + player_search_fields::ASSIGNED_CLUB) > -1 {
        covered!("sx17_1 load the scouted club record");
        let record = exe::crt_operator_new(PLAYER_SEARCH_CLUB_RECORD_SIZE);
        if record != 0 {
            if exe::zread(record, PLAYER_SEARCH_CLUB_RECORD_SIZE, 1, save) != 1 {
                show_error_box(0xa701b0, 0x1fc3); // player_search.cpp
            } else {
                exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, record, 4); // sx17_1: the record joins the global list
            }
        }
        write_usize(this + player_search_fields::SCOUTED_CLUB_RECORD, record);
    }
    1
}

/// Comparer for `player_search_load`: it reads the save item and allocates, so it cannot run twice; only the original
/// runs. The copy is proven by loading the holiday save in the save proof (levas/N/CARGA).
pub(crate) unsafe extern "thiscall" fn compare_player_search_load(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_PLAYER_SEARCH_LOAD)(this, save)
}

// --- sx20_9: writing and freeing the tactics store (tactics.cpp: Tactics::write_to_save 0x8995d0, Tactics::free_data
// 0x8961e0) and the old shortlist slot record (shortlist_manager.cpp: ShortlistOldSlot::ShortlistOldSlot 0x83b360) ---

/// tactics.cpp, the source path of the error boxes.
const TACTICS_SOURCE: usize = 0xa8b908;
/// The size of a tactic block (`tactics_fields::CUSTOM_TACTICS`, `PRESET_TACTICS`), of a club tactics row and of a
/// per-tactic statistics record.
const TACTIC_BLOCK_SIZE: u32 = 0x18e3;
const TACTICS_ROW_SIZE: u32 = 0x91;
const TACTIC_STATS_SIZE: u32 = 0x28;

/// Writes the tactics store into the saved game: without a save the error box (line 0x96c) and 0; the "tactics.dat"
/// item (missing: line 0x974 and 0); the custom tactic count and blocks (0x18e3 bytes), the preset count and blocks,
/// the club row count and rows (0x91 bytes), the statistics count (+4) and records (0x28 bytes from `[+0]`) and the
/// ten default values from +8; closes the item and returns 1. A short write shows the box (lines 0x97b to 0x9bb),
/// closes the item and returns 0.
/// Original: `Tactics::write_to_save` (0x8995d0, tactics.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn tactics_write_to_save(this: usize, save: usize) -> u32 {
    use tactics_fields as f;
    if save == 0 {
        show_error_at(TACTICS_SOURCE, 0x96c);
        return 0;
    }
    let mut z = zip_open_item(save, 0xa8b940); // "tactics.dat"
    if z == 0 {
        show_error_at(TACTICS_SOURCE, 0x974);
        return 0;
    }
    covered!("sx20_9 save tactics");
    let parts: [(usize, u32, u32, u32); 9] = [
        (this + f::CUSTOM_TACTIC_COUNT, 4, 1, 0x97b),
        (read_usize(this + f::CUSTOM_TACTICS), TACTIC_BLOCK_SIZE, read_u32(this + f::CUSTOM_TACTIC_COUNT), 0x983),
        (this + f::PRESET_TACTIC_COUNT, 4, 1, 0x98b),
        (read_usize(this + f::PRESET_TACTICS), TACTIC_BLOCK_SIZE, read_u32(this + f::PRESET_TACTIC_COUNT), 0x993),
        (this + f::ROW_COUNT, 4, 1, 0x99b),
        (read_usize(this + f::ROWS), TACTICS_ROW_SIZE, read_u32(this + f::ROW_COUNT), 0x9a3), // sx20_9: the club rows
        (this + 4, 4, 1, 0x9ab), // the statistics count
        (read_usize(this + f::TACTIC_STATS), TACTIC_STATS_SIZE, read_u32(this + 4), 0x9b3),
        (this + 8, 4, 10, 0x9bb), // the ten default values
    ];
    for (data, size, count, line) in parts {
        if zip_write(data, size, count, z) != count {
            show_error_at(TACTICS_SOURCE, line);
            zip_close(&mut z);
            return 0;
        }
    }
    zip_close(&mut z);
    1
}

/// Frees an array of tactic blocks built with `new[]` (the `eh vector destructor iterator` with the empty 0x5a53c0
/// over the count stored just before it, then `delete` of the allocation) and clears the pointer and its count.
unsafe fn tactics_free_blocks(this: usize, blocks: usize, count: usize) {
    let p = read_usize(this + blocks);
    if p == 0 {
        return;
    }
    exe::crt_vector_destructor_iterator(p, TACTIC_BLOCK_SIZE, read_u32(p - 4), 0x5a53c0usize);
    exe::crt_operator_delete(p - 4);
    write_usize(this + blocks, 0);
    write_usize(this + count, 0);
}

/// Frees the tactics store: the custom and preset tactic blocks (`tactics_free_blocks`), the club rows (`free`) and
/// the per-tactic statistics (`delete`), each with its pointer and count cleared.
/// Original: `Tactics::free_data` (0x8961e0, tactics.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn tactics_free_data(this: usize) {
    use tactics_fields as f;
    tactics_free_blocks(this, f::CUSTOM_TACTICS, f::CUSTOM_TACTIC_COUNT);
    tactics_free_blocks(this, f::PRESET_TACTICS, f::PRESET_TACTIC_COUNT);
    if read_usize(this + f::ROWS) != 0 {
        exe::crt_free(read_usize(this + f::ROWS));
        write_usize(this + f::ROWS, 0);
        write_usize(this + f::ROW_COUNT, 0);
    }
    covered!("sx20_9 tactics freed");
    if read_usize(this + f::TACTIC_STATS) != 0 {
        exe::crt_operator_delete(read_usize(this + f::TACTIC_STATS));
        write_usize(this + f::TACTIC_STATS, 0);
        write_usize(this + 4, 0); // the statistics count
    }
}

/// Builds one 0x24-byte old-format shortlist slot record (from saves of an older version): the default date
/// (`Date::Date`) in its field at +0x1a. Returns the record.
/// Original: `ShortlistOldSlot::ShortlistOldSlot` (0x83b360, shortlist_manager.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn shortlist_old_slot_shortlist_old_slot(this: usize) -> usize {
    exe::date_date_54a110(this + 0x1a);
    this
}

/// Comparer for `tactics_write_to_save`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_write_to_save(this: usize, save: usize) -> u32 {
    // simplification: writes into the save container (cannot run twice); the copy is proven by the save proof
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_TACTICS_WRITE_TO_SAVE)(this, save)
}

/// Comparer for `tactics_free_data`.
pub(crate) unsafe extern "thiscall" fn compare_tactics_free_data(this: usize) {
    // simplification: frees memory (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_TACTICS_FREE_DATA)(this)
}

/// Comparer for `shortlist_old_slot_shortlist_old_slot`: side by side over the record's date (8 bytes at +0x1a).
pub(crate) unsafe extern "thiscall" fn compare_shortlist_old_slot_shortlist_old_slot(this: usize) -> usize {
    let original = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_SHORTLIST_OLD_SLOT_SHORTLIST_OLD_SLOT);
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(this + 0x1a, 8)], || original(this), || shortlist_old_slot_shortlist_old_slot(this));
    SCOREBOARD.record(same && r == c, || format!("shortlist_old_slot_shortlist_old_slot({:#x}) {:?}", this, first_differences(&mo, &mr)));
    r
}

// ---------- sx19b_3: a ruling body in the saved game: loading and saving (ruling_body/ruling_body.cpp) ----------

/// The pending ban node of a ruling body's list (0x70 bytes): its fixture copy at +0x20.
const PENDING_BAN_NODE_SIZE: u32 = 0x70;
const PENDING_BAN_FIXTURE: usize = 0x20;

/// The ruling body's virtual slot `slot` (thiscall(body, save item), ret 4): +0x18 `load_extra_data`, +0x1c
/// `save_extra_data`.
unsafe fn ruling_body_extra_data(this: usize, slot: usize, file: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + slot));
    f(this, file)
}

/// Reads the discipline entry array and the pending ban list from a saved game, then the body's own extra data.
///
/// `RulingBody::load(file)` (file = the save item, its stream at +0), from the base constructor with a save
/// (`construct_ruling_body`): the entry count (`ruling_body::ENTRY_COUNT`, 4 bytes); with entries, the capacity
/// (`ENTRY_CAPACITY`) = the count and a new array (`exe::crt_malloc`, 14 bytes each; out of memory: the box at line
/// 0x6c) in `ENTRIES`, read whole; then the number of pending bans (16 bits) and each 0x70-byte node, whose fixture copy
/// (+0x20) gets its competition, stadium and both clubs back as pointers from their ids (`G_CLUB_COMP_TABLE`,
/// `G_STADIUM_TABLE`, `G_CLUB_TABLE`) before it goes to the end of the body's list (`exe::llist_add_tail`, copied).
/// Last, the body's `load_extra_data` (vtable +0x18): returns 1 when it answers. A short read: the ruling_body.cpp
/// "Error" box with the field's line (0x5f, 0x74, 0x7e, 0x8b) and 0.
/// Original: `RulingBody::load` (0x7e78a0, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_load(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zread(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x5f);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 {
        write_u32(this + ruling_body::ENTRY_CAPACITY, count);
        let entries = exe::crt_malloc(count.wrapping_mul(14));
        write_usize(this + ruling_body::ENTRIES, entries);
        if entries == 0 {
            show_error_at(RULING_BODY_CPP, 0x6c);
            return 0;
        }
        if exe::zread(entries, 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
            show_error_at(RULING_BODY_CPP, 0x74);
            return 0;
        }
    }
    // simplification: the original constructs the node's two dates (+4, +0x14) first; the read overwrites them
    let mut node = [0u8; PENDING_BAN_NODE_SIZE as usize];
    let mut pending = 0i16;
    if exe::zread(&mut pending as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0x7e);
        return 0;
    }
    let mut i = 0i16;
    while i < pending {
        if exe::zread(node.as_mut_ptr() as usize, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
            show_error_at(RULING_BODY_CPP, 0x8b);
            return 0;
        }
        covered!("sx19b_3 pending ban loaded");
        let f = Fixture(node.as_mut_ptr() as usize + PENDING_BAN_FIXTURE);
        // the club competition records are 0x6b bytes
        f.set_competition(Competition(read_usize(G_CLUB_COMP_TABLE).wrapping_add((f.comp_id() as usize).wrapping_mul(0x6b))));
        f.set_stadium(read_usize(G_STADIUM_TABLE).wrapping_add(f.stadium_id().wrapping_mul(stadium::SIZE)));
        f.set_home_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.home_club_id().wrapping_mul(club::SIZE)))); // sx19b_3: the home club relinked
        f.set_away_club(Club(read_usize(G_CLUB_TABLE).wrapping_add(f.away_club_id().wrapping_mul(club::SIZE))));
        exe::llist_add_tail(read_usize(this + RULING_BODY_LIST), node.as_ptr() as usize, PENDING_BAN_NODE_SIZE);
        i += 1;
    }
    (ruling_body_extra_data(this, 0x18, file) != 0) as u32
}

/// Comparer for `ruling_body_load`: it reads the save stream, so only the original runs; checked afterwards: the
/// capacity equals the count when there are entries.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_load(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RULING_BODY_LOAD)(this, file);
    let (n, c) = (read_u32(this + ruling_body::ENTRY_COUNT), read_u32(this + ruling_body::ENTRY_CAPACITY));
    SCOREBOARD.record(r == 0 || n == 0 || n == c, || format!("ruling_body_load({:#x}): {} entries, capacity {}", this, n, c));
    r
}

/// Writes the discipline entry array and the pending ban list to a saved game, then the body's own extra data.
///
/// `RulingBody::save(file)`, from `Discipline::save` (each of the 43 bodies): the entry count
/// (`ruling_body::ENTRY_COUNT`, 4 bytes) and, with entries, the array (`ENTRIES`, 14 bytes each); the number of
/// pending bans (the list's 16-bit count, 0 without a list) and, with any, each node of the list (0x70 bytes, from the
/// head). Last, the body's `save_extra_data` (vtable +0x1c), whose answer is returned. A short write: the
/// ruling_body.cpp "Error" box with the field's line (0xae, 0xb8, 0xc7, 0xd5) and nothing more (the caller ignores
/// the answer; the copy returns 0).
/// Original: `RulingBody::save` (0x7e7b80, ruling_body.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ruling_body_save(this: usize, file: usize) -> u32 {
    let stream = read_usize(file);
    if exe::zwrite(this + ruling_body::ENTRY_COUNT, 4, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xae);
        return 0;
    }
    let count = read_u32(this + ruling_body::ENTRY_COUNT);
    if count != 0 && exe::zwrite(read_usize(this + ruling_body::ENTRIES), 14, count, stream) != read_u32(this + ruling_body::ENTRY_COUNT) {
        show_error_at(RULING_BODY_CPP, 0xb8);
        return 0;
    }
    let list = Llist(read_usize(this + RULING_BODY_LIST));
    let pending: u32 = if list.is_null() { 0 } else { list.count() as u16 as u32 }; // sx19b_3: the pending ban count
    if exe::zwrite(&pending as *const u32 as usize, 2, 1, stream) != 1 {
        show_error_at(RULING_BODY_CPP, 0xc7);
        return 0;
    }
    if pending as u16 != 0 {
        list.set_cursor(list.head());
        let mut node = exe::llist_get_next(list);
        while node != 0 {
            covered!("sx19b_3 pending ban saved");
            if exe::zwrite(node, PENDING_BAN_NODE_SIZE, 1, stream) != 1 {
                show_error_at(RULING_BODY_CPP, 0xd5);
                return 0;
            }
            node = exe::llist_get_next(list);
        }
    }
    ruling_body_extra_data(this, 0x1c, file)
}

/// Comparer for `ruling_body_save`: it writes the save stream, so only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_ruling_body_save(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_RULING_BODY_SAVE)(this, file)
}

// ---------- cx01_6: the file search of the game (comp_util.cpp) ----------

/// C runtime `_findfirst(pattern, data)` (cdecl): the search handle, −1 when nothing matches.
const FN_CRT_FINDFIRST: usize = 0x9467ce;
/// C runtime `_findnext(handle, data)` (cdecl): 0, −1 when there is no further entry.
const FN_CRT_FINDNEXT: usize = 0x946898;
/// C runtime `_findclose(handle)` (cdecl).
const FN_CRT_FINDCLOSE: usize = 0x94695d;

/// The C runtime's `_finddata_t` (0x118 bytes): the attributes +0, the size +0x10 and the name +0x14 (260 bytes).
const FINDDATA_SIZE: usize = 0x118;
const FINDDATA_ATTRIB: usize = 0x0;
const FINDDATA_FILE_SIZE: usize = 0x10;
const FINDDATA_NAME: usize = 0x14;

/// The game's search state (0x114 bytes): the handle +0, the size +4, the attributes +8, the name
/// (`file_find_state::FILE_NAME`, 255 characters and the terminator at +0x10f) and the kind wanted +0x110.
const FIND_HANDLE: usize = 0x0;
const FIND_SIZE: usize = 0x4;
const FIND_ATTRIB: usize = 0x8;
const FIND_KIND: usize = 0x110;

/// Whether a found entry is of the kind wanted: kind 0x10 takes directories (attribute bit 0x10), kind 0 takes plain
/// files; any other kind takes nothing.
fn find_kind_matches(kind: u32, attrib: u32) -> bool {
    (kind == 0x10 && attrib & 0x10 != 0) || (kind == 0 && attrib & 0x10 == 0)
}

/// Steps the open search `state` with `_findnext` until an entry of the kind wanted (the one already in `data`
/// first) and stores its size, attributes and name (`exe::at_9466d0`, the C runtime strncpy, 255 characters and a
/// terminator). Returns 0, or −1 when the search runs out (the handle stays open).
unsafe fn find_take_matching(state: usize, data: usize) -> i32 {
    let next: unsafe extern "C" fn(i32, usize) -> i32 = core::mem::transmute(FN_CRT_FINDNEXT);
    while !find_kind_matches(read_u32(state + FIND_KIND), read_u32(data + FINDDATA_ATTRIB)) {
        if next(read_i32(state + FIND_HANDLE), data) == -1 {
            return -1;
        }
    }
    write_u32(state + FIND_ATTRIB, read_u32(data + FINDDATA_ATTRIB));
    write_u32(state + FIND_SIZE, read_u32(data + FINDDATA_FILE_SIZE));
    exe::at_9466d0(state + file_find_state::FILE_NAME, (data + FINDDATA_NAME) as *const u8, 0xff);
    write_u8(state + file_find_state::FILE_NAME + 0xff, 0);
    0
}

/// Starts a file search for `pattern` that wants `kind` (0x10 directories, 0 plain files) in the search state `state`:
/// the kind is stored, the handle of `_findfirst` too; −1 when nothing matches, otherwise the first entry of the kind
/// wanted (`find_take_matching`), 0, or −1 when there is none.
/// Original: `file_find_first` (0x4c0aa0, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_first(pattern: usize, state: usize, kind: u32) -> i32 {
    let first: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_CRT_FINDFIRST);
    let mut data = [0u8; FINDDATA_SIZE];
    write_u32(state + FIND_KIND, kind);
    let handle = first(pattern, data.as_mut_ptr() as usize);
    write_i32(state + FIND_HANDLE, handle);
    if handle == -1 {
        covered!("cx01_6 find first: nothing");
        return -1;
    }
    covered!("cx01_6 find first");
    find_take_matching(state, data.as_mut_ptr() as usize)
}

/// Steps the search in `state` to the next entry of the kind wanted: −1 for a closed search (handle −1); otherwise
/// `_findnext` (into a record first filled with the stored size, attributes and 256 bytes from the name, which the
/// call overwrites), −1 when there is no further entry, else the first entry of the kind wanted (`find_take_matching`).
/// Original: `file_find_next` (0x4c0b50, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_next(state: usize) -> i32 {
    if read_i32(state + FIND_HANDLE) == -1 {
        return -1;
    }
    let next: unsafe extern "C" fn(i32, usize) -> i32 = core::mem::transmute(FN_CRT_FINDNEXT);
    let mut data = [0u8; FINDDATA_SIZE];
    let d = data.as_mut_ptr() as usize;
    write_u32(d + FINDDATA_FILE_SIZE, read_u32(state + FIND_SIZE));
    write_u32(d + FINDDATA_ATTRIB, read_u32(state + FIND_ATTRIB));
    exe::at_9466d0(d + FINDDATA_NAME, (state + file_find_state::FILE_NAME) as *const u8, 0x100);
    if next(read_i32(state + FIND_HANDLE), d) == -1 {
        covered!("cx01_6 find next: no more");
        return -1;
    }
    covered!("cx01_6 find next");
    find_take_matching(state, d)
}

/// Closes the search in `state`: −1 for one already closed (handle −1); otherwise `_findclose`, the handle −1 and 0.
/// Original: `file_find_close` (0x4c0c10, comp_util.cpp, cdecl).
pub(crate) unsafe extern "C" fn file_find_close(state: usize) -> i32 {
    let handle = read_i32(state + FIND_HANDLE);
    if handle == -1 {
        return -1;
    }
    covered!("cx01_6 find close");
    let close: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_CRT_FINDCLOSE);
    close(handle);
    write_i32(state + FIND_HANDLE, -1);
    0
}

/// Comparer for `file_find_first`: the original into the caller's state and the copy into a local one (closed
/// afterwards); the return, the size, the attributes, the name and the kind must match (the handles differ).
pub(crate) unsafe extern "C" fn compare_file_find_first(pattern: usize, state: usize, kind: u32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> i32>(IDX_FILE_FIND_FIRST)(pattern, state, kind);
    let mut mine = [0u8; 0x114];
    let m = mine.as_mut_ptr() as usize;
    let n = file_find_first(pattern, m, kind);
    let name = |p: usize| { let s = core::slice::from_raw_parts(p as *const u8, 0x100); s[..s.iter().position(|&c| c == 0).unwrap_or(0x100)].to_vec() };
    let found = |s: usize| (read_u32(s + FIND_SIZE), read_u32(s + FIND_ATTRIB), name(s + file_find_state::FILE_NAME), read_u32(s + FIND_KIND));
    let (a, b) = (found(state), found(m));
    // the fields after a failed search are whatever was there before: only the return counts
    SCOREBOARD.record(r == n && (r != 0 || a == b), || format!("file_find_first({:?}, kind {:#x}): original {} {:x?} rust {} {:x?}", String::from_utf8_lossy(&c_string_bytes(pattern)), kind, r, a, n, b));
    if read_i32(m + FIND_HANDLE) != -1 {
        let close: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_CRT_FINDCLOSE);
        close(read_i32(m + FIND_HANDLE));
    }
    r
}

/// Comparer for `file_find_next`: it advances the open search (cannot run twice); only the original runs.
pub(crate) unsafe extern "C" fn compare_file_find_next(state: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_FILE_FIND_NEXT)(state)
}

/// Comparer for `file_find_close`: it closes the search (cannot run twice); only the original runs.
pub(crate) unsafe extern "C" fn compare_file_find_close(state: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(usize) -> i32>(IDX_FILE_FIND_CLOSE)(state)
}

// ---------- cx01_7: the CONMEBOL seeding constructor and a cup's temporary file (conmebol_seeding.cpp, Cup.cpp) ----------

/// `ConmebolSeeding::load_from_temp_file` (0x4c6970, conmebol_seeding.cpp, thiscall, ret 0): 1 when the seeding came
/// back from its temporary file.
const FN_CONMEBOL_SEEDING_LOAD_FROM_TEMP_FILE: usize = 0x4c6970;
/// `Comp_Stats::save` (0x4ac4e0, comp_stats.cpp, thiscall(file), ret 4).
const FN_COMP_STATS_SAVE: usize = 0x4ac4e0;
/// `Cup_Stage::save_to_file` (0x523eb0, cup_stage.cpp, thiscall(file), ret 4).
const FN_CUP_STAGE_SAVE_TO_FILE: usize = 0x523eb0;
/// `League_Stage::save_to_file` (0x68ca40, league_stage.cpp, thiscall(file), ret 4).
const FN_LEAGUE_STAGE_SAVE_TO_FILE: usize = 0x68ca40;

/// Builds the CONMEBOL seeding: the game CD check (`game_cd_check`, the drive type jump 0x4c601a and the found test
/// 0x4c605d); then, when the seeding comes back from its temporary file (`ConmebolSeeding::load_from_temp_file`), that
/// is all. Otherwise the year of the last update and the row count are cleared and the nations are walked in table
/// order (`G_NATION_TABLE`, 0x122 bytes each) while there are fewer than 11 rows: Mexico and every nation of the South
/// American continent get a 10-byte row (+0 the nation id, +4 the nation, +8 its Libertadores places: 4 for Brazil and
/// Argentina, 1 for Venezuela and Mexico, 3 for the others). The Copa dos Campeões winner (`BRA_CHAMP_CUP_WINNER_ID`)
/// becomes −1. Returns the object.
/// Original: `ConmebolSeeding::ConmebolSeeding` (0x4c5fe0, conmebol_seeding.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn new_conmebol_seeding(this: usize) -> usize {
    game_cd_check(0x4c601a, 0x4c605d);
    let load: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_CONMEBOL_SEEDING_LOAD_FROM_TEMP_FILE);
    if load(this) != 0 {
        covered!("cx01_7 conmebol seeding from the file");
        return this;
    }
    covered!("cx01_7 conmebol seeding built");
    write_u16(this + conmebol_seeding::LAST_SEASON_YEAR, 0);
    write_u8(this + conmebol_seeding::NATION_COUNT, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(G_NATION_COUNT) {
        let rows = read_u8(this + conmebol_seeding::NATION_COUNT) as i8;
        if rows >= 0xb {
            break;
        }
        let nation = Nation(read_usize(G_NATION_TABLE) + k as usize * 0x122);
        let id = read_u32(nation.addr());
        let continent = nation.continent();
        if id == read_u32(KEY_NATION_MEXICO) || (continent != 0 && read_u32(continent) == read_u32(KEY_CONTINENT_SOUTH_AMERICA)) {
            let row = this + rows as isize as usize * 10;
            write_u32(row, id); // the row's nation id
            write_usize(row + 4, nation.addr()); // the row's nation
            let places = if id == read_u32(KEY_NATION_BRAZIL) || id == read_u32(KEY_NATION_ARGENTINA) {
                4
            } else if id == read_u32(KEY_NATION_VENEZUELA) || id == read_u32(KEY_NATION_MEXICO) {
                1
            } else {
                3
            };
            write_u8(row + 8, places); // the row's Libertadores places
            write_u8(this + conmebol_seeding::NATION_COUNT, rows as u8 + 1);
        }
        k += 1;
    }
    write_u32(this + conmebol_seeding::BRA_CHAMP_CUP_WINNER_ID, u32::MAX);
    this
}

/// Writes `count` items of `size` bytes from `data` into the open file `f` (`exe::crt_fwrite`).
unsafe fn temp_write(f: usize, data: usize, size: u32, count: u32) {
    exe::crt_fwrite(data, size, count, f);
}

/// Turns the clubs of the first `count` 6-byte entries of `book` into their ids (−1 for none), in place.
unsafe fn club_entries_to_ids(book: usize, count: i16) {
    for i in 0..count.max(0) as usize {
        let e = book + 6 * i;
        let club = read_usize(e);
        write_u32(e, if club != 0 { read_u32(club) } else { u32::MAX });
    }
}

/// Turns the ids of the first `count` 6-byte entries of `book` back into club pointers (null for −1), in place.
unsafe fn club_entries_from_ids(book: usize, count: i16) {
    for i in 0..count.max(0) as usize {
        let e = book + 6 * i;
        let id = read_i32(e);
        write_usize(e, if id != -1 { club_at(id as isize).addr() } else { 0 });
    }
}

/// The name of a cup's temporary file, written at +0x53 (80 bytes): the competition's long name with '/' and '.' made
/// '_', then "%s_%d.tmp" with the number.
unsafe fn cup_temp_file_name(this: League, number: i32) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let mut name = [0u8; 0x100];
    format(name.as_mut_ptr() as usize, 0x991944, this.competition().name_addr()); // "%s"
    for c in name.iter_mut().take_while(|c| **c != 0) {
        if *c == b'/' || *c == b'.' {
            *c = b'_';
        }
    }
    format(this.addr() + 0x53, 0x99b838, name.as_ptr() as usize, number); // "%s_%d.tmp"
}

/// The header of a cup's temporary file, after the name and the statistics: the class's own data (virtual +0x24) and
/// the fields in the order `Cup::load_from_disk` reads them back.
unsafe fn cup_save_header(this: League, f: usize) {
    let class_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x24));
    class_data(this.addr(), f);
    let a = this.addr();
    for (field, size) in [(cup::PRIZE_FUND, 4), (league::SQUAD_FLAGS, 1), (cup::IMPORTANCE_BIAS, 1), (league::ENTRANTS_ARE_QUALIFIERS, 1),
        (cup::SEASON_CLOSED, 4), (cup::VENUE_STADIUM_ID, 4), (cup::SUBSTITUTES_NAMED, 1), (cup::SUBSTITUTIONS_ALLOWED, 1), (cup::SEASON_SET_UP, 4),
        (cup::RUN_BY_STAGES, 1), (cup::ROUND_COUNT, 2), (cup::TEAM_COUNT, 2), (cup::YEAR, 2), (cup::CURRENT_ROUND, 1), (cup::NEXT_DRAW_ROUND, 1),
        (cup::CLUB_SIDES, 1), (cup::STAGE_LABEL, 2), (league::UPPER_DIVISION_ID, 4), (league::LOWER_DIVISION_ID, 4)] {
        temp_write(f, a + field, size, 1);
    }
    // the points-average table: its pointer +0x18 as it is, its row count +0x34 and its 0x13-byte rows
    temp_write(f, a + 0x18, 4, 1);
    temp_write(f, a + 0x34, 2, 1);
    let averages = read_usize(a + 0x18);
    if averages != 0 {
        temp_write(f, averages, 0x13, read_i16(a + 0x34) as i32 as u32);
    }
}

/// The stages of a cup in its temporary file: for each stage up to the current one, its type and, for a cup stage, its
/// team and round counts, each round's tie count and `Cup_Stage::save_to_file`; for a league stage, its club and round
/// counts and `League_Stage::save_to_file`.
unsafe fn cup_save_stages(this: League, f: usize) {
    let mut k = 0i32;
    while k <= this.current_stage() as i32 {
        let stage = League(read_usize(this.stages() + k as usize * 4));
        temp_write(f, stage.addr() + league::COMP_TYPE, 1, 1);
        match stage.comp_type() {
            1 => {
                temp_write(f, stage.addr() + cup::TEAM_COUNT, 2, 1);
                temp_write(f, stage.addr() + cup::ROUND_COUNT, 2, 1);
                let s = Cup(stage.addr());
                for r in 0..s.round_count().max(0) as usize {
                    temp_write(f, s.round_table() + r * cup_round::SIZE + cup_round::TIES, 2, 1);
                }
                let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_CUP_STAGE_SAVE_TO_FILE);
                save(stage.addr(), f);
            }
            2 => {
                temp_write(f, stage.addr() + league::CLUB_COUNT, 2, 1);
                temp_write(f, stage.addr() + league::ROUND_COUNT, 2, 1);
                let save: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LEAGUE_STAGE_SAVE_TO_FILE);
                save(stage.addr(), f);
            }
            _ => {}
        }
        k += 1;
    }
}

/// Writes a cup's state into its CM3_TEMP temporary file, in the order `Cup::load_from_disk` reads it.
///
/// `Cup::save_cup_to_disk(by_id)`. The file name at +0x53 (`cup_temp_file_name`) with the competition record's id;
/// without `by_id` (its low byte), a cup whose season is not set up (`SEASON_SET_UP`) writes nothing, otherwise the
/// name takes the cup's `YEAR`. Without a round table, team table or tie tables, the Cup.cpp error box (line 0x147e).
/// The file opened in CM3_TEMP ("wb", `open_temp_file_for_write`); unopened, the error box (line 0x1488; the message
/// "unable to open save file" is printed into a local the box never shows). Then: the 0x50-byte name, the statistics
/// flag (4 bytes) and the statistics (`Comp_Stats::save`), the header (`cup_save_header`), the competition id (the
/// record's first dword; without a record the error box of line 0x14c3 and −1), the round table (0x68 bytes per
/// round), the team table (6 bytes per team, its clubs written as ids and turned back into pointers), each round's
/// ties (0x35 bytes each, `cup_round::TIES`), `STAGE_COUNT`, `CURRENT_STAGE`, `STAGE_INDEX`, the stages
/// (`cup_save_stages`), `ENTRANT_CAPACITY`, `ENTRANT_COUNT` and the entrant book (its first count clubs as ids, the
/// whole capacity written, then the clubs back). The file closed.
/// Defect of the original: with entrants but no book (+0x14 null), the id pass reads through null.
/// Original: `Cup::save_cup_to_disk` (0x5210f0, Cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_save_cup_to_disk(this: usize, by_id: u32) {
    let this = League(this);
    let cup = Cup(this.addr());
    let has_stats = (this.comp_stats() != 0) as u32;
    let no_competition = u32::MAX;
    cup_temp_file_name(this, read_i32(this.competition().addr()));
    if by_id as u8 == 0 {
        if cup.season_set_up() == 0 {
            return;
        }
        cup_temp_file_name(this, this.year() as i32);
    }
    if cup.round_table() == 0 || cup.team_table() == 0 || cup.tie_tables() == 0 {
        show_error_at(CUP_SOURCE, 0x147e);
        return;
    }
    let name = this.addr() + 0x53;
    let f = open_temp_file_for_write(name);
    if f == 0 {
        let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        let mut message = [0u8; 0x100];
        format(message.as_mut_ptr() as usize, 0x9b58dc, name); // "CUP::save_cup_to_disk() - unable to open save file'%s'"
        show_error_at(CUP_SOURCE, 0x1488);
        return;
    }
    covered!("cx01_7 cup saved");
    temp_write(f, name, 0x50, 1);
    temp_write(f, &has_stats as *const u32 as usize, 4, 1);
    if has_stats != 0 {
        let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_STATS_SAVE);
        save(this.comp_stats(), f);
    }
    cup_save_header(this, f);
    if !this.competition().is_null() {
        temp_write(f, this.competition().addr(), 4, 1);
    } else {
        show_error_at(CUP_SOURCE, 0x14c3);
        temp_write(f, &no_competition as *const u32 as usize, 4, 1);
    }
    temp_write(f, cup.round_table(), cup_round::SIZE as u32, cup.round_count() as i32 as u32);
    club_entries_to_ids(cup.team_table(), cup.team_count());
    temp_write(f, cup.team_table(), 6, cup.team_count() as i32 as u32);
    club_entries_from_ids(cup.team_table(), cup.team_count());
    for r in 0..cup.round_count().max(0) as usize {
        let ties = read_i16(cup.round_table() + r * cup_round::SIZE + cup_round::TIES) as i32;
        temp_write(f, read_usize(cup.tie_tables() + r * 4), (ties * 0x35) as u32, 1);
    }
    temp_write(f, this.addr() + cup::STAGE_COUNT, 4, 1);
    temp_write(f, this.addr() + cup::CURRENT_STAGE, 4, 1);
    temp_write(f, this.addr() + cup::STAGE_INDEX, 1, 1);
    cup_save_stages(this, f);
    temp_write(f, this.addr() + cup::ENTRANT_CAPACITY, 2, 1);
    temp_write(f, this.addr() + cup::ENTRANT_COUNT, 2, 1);
    let book = this.entrant_book();
    club_entries_to_ids(book, this.entrant_count());
    if book != 0 {
        temp_write(f, book, 6, this.entrant_capacity() as i32 as u32);
    }
    club_entries_from_ids(book, this.entrant_count());
    exe::crt_fclose(f);
}

/// Comparer for `new_conmebol_seeding`: it waits for the CD and may read the temporary file (cannot run twice); only
/// the original runs.
pub(crate) unsafe extern "thiscall" fn compare_new_conmebol_seeding(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_NEW_CONMEBOL_SEEDING)(this)
}

/// The bytes of a cup's temporary file (the name at +0x53, in CM3_TEMP); empty when it cannot be read.
unsafe fn cup_temp_file_bytes(this: usize) -> Vec<u8> {
    let path = c_string_bytes(exe::utils_build_file_path(0x987da0, this + 0x53)); // "CM3_TEMP"
    std::fs::read(String::from_utf8_lossy(&path).into_owned()).unwrap_or_default()
}

/// Comparer for `cup_save_cup_to_disk`: the original writes the file, which is read back; the copy writes it again and
/// the two contents (and the name at +0x53) must match. The file is left as the copy wrote it, the same bytes.
pub(crate) unsafe extern "thiscall" fn compare_cup_save_cup_to_disk(this: usize, by_id: u32) {
    let name = |p: usize| { let s = core::slice::from_raw_parts(p as *const u8, 0x50); s[..s.iter().position(|&c| c == 0).unwrap_or(0x50)].to_vec() };
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CUP_SAVE_CUP_TO_DISK)(this, by_id);
    let name_o = name(this + 0x53);
    let o = cup_temp_file_bytes(this);
    cup_save_cup_to_disk(this, by_id);
    let name_r = name(this + 0x53);
    let r = cup_temp_file_bytes(this);
    SCOREBOARD.record(name_o == name_r && o == r, || format!("cup_save_cup_to_disk({:#x}, {}): file {:?} {} bytes, copy {:?} {} bytes, first difference at {:?}", this, by_id as u8,
        String::from_utf8_lossy(&name_o), o.len(), String::from_utf8_lossy(&name_r), r.len(), o.iter().zip(r.iter()).position(|(a, b)| a != b)));
}

// ---------- sx21b_5: a transfer offer in the saved game: loading and saving it (transfer_offer.cpp) ----------

/// Reads a transfer offer from the saved game: the 0x41-byte record, its rival offer list and its exchange players.
///
/// `TRANSFER_OFFER::load(stream, offers)`. The record (`exe::zread`, 0x41 bytes) and a word count; then that many offer
/// slot indices: an index below this offer's own `offer::SLOT_INDEX` names an offer already loaded (`offers[index]`,
/// whose rival list this offer shares: +0 takes that list; a missing one shows the box, line 0x99); every index also
/// goes into a new list of its own (`exe::llist_add_tail`, 4 bytes), which becomes the rival list when none was shared
/// and is otherwise emptied and freed. A count with no `offers` shows the box (line 0x7a). Then another word count and
/// that many 7-byte exchange player nodes into a new `offer::EXCHANGE_PLAYERS` list (none: 0). A short read shows the
/// box (lines 0x69, 0x70, 0x8d, 0xbe, 0xd0) and stops there.
/// Original: `TRANSFER_OFFER::load` (0x8e5790, transfer_offer.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn transfer_offer_load(this: usize, stream: usize, offers: usize) {
    const FILE: usize = 0xa9e318; // transfer_offer.cpp
    if exe::zread(this, 0x41, 1, stream) != 1 {
        show_error_at(FILE, 0x69);
        return;
    }
    let mut count = 0i16;
    if exe::zread(&mut count as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(FILE, 0x70);
        return;
    }
    if count != 0 {
        if offers == 0 {
            show_error_at(FILE, 0x7a);
            return;
        }
        write_u32(this, 0); // the rival offer list
        let list = transfer_offer_new_list();
        let mut k = 0i16;
        while k < count {
            let mut index = 0i32;
            if exe::zread(&mut index as *mut i32 as usize, 4, 1, stream) != 1 {
                show_error_at(FILE, 0x8d);
                return;
            }
            if index < Offer(this).slot_index() as i32 { // sx21b_5: an offer loaded before shares its list
                let earlier = read_usize(offers + index as usize * 4);
                if earlier == 0 {
                    show_error_at(FILE, 0x99);
                } else {
                    covered!("sx21b_5 shared rival list");
                    write_u32(this, read_u32(earlier));
                }
            }
            exe::llist_add_tail(list, &mut index as *mut i32 as usize, 4);
            k += 1;
        }
        if read_u32(this) == 0 {
            write_usize(this, list);
        } else {
            exe::llist_clear(list);
            if list != 0 {
                exe::drop_llist(list);
                exe::crt_operator_delete(list);
            }
        }
    }
    if exe::zread(&mut count as *mut i16 as usize, 2, 1, stream) != 1 {
        show_error_at(FILE, 0xbe);
        return;
    }
    if count == 0 {
        write_u32(this + offer::EXCHANGE_PLAYERS, 0);
        return;
    }
    let list = transfer_offer_new_list();
    write_usize(this + offer::EXCHANGE_PLAYERS, list);
    let mut k = 0i16;
    while k < count {
        let mut node = [0u8; 8];
        if exe::zread(node.as_mut_ptr() as usize, 7, 1, stream) != 1 {
            show_error_at(FILE, 0xd0);
            return;
        }
        covered!("sx21b_5 exchange player loaded");
        exe::llist_add_tail(read_usize(this + offer::EXCHANGE_PLAYERS), node.as_mut_ptr() as usize, 7); // sx21b_5: 7-byte nodes
        k += 1;
    }
}

/// A new empty list (`operator new(0xe)` and `LLIST::LLIST`), or 0 when the allocation fails.
unsafe fn transfer_offer_new_list() -> usize {
    let p = exe::crt_operator_new(0xe);
    if p != 0 { exe::new_llist(p) } else { 0 }
}

/// Comparer for `transfer_offer_load`: only the original (it reads the saved game's stream and allocates); the copy
/// is proven by loading the holiday save with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_load(this: usize, stream: usize, offers: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_TRANSFER_OFFER_LOAD)(this, stream, offers)
}

/// Writes a transfer offer to the saved game: the 0x41-byte record, its rival offer list and its exchange players.
///
/// `TRANSFER_OFFER::save(stream)`: the record (`exe::zwrite`, 0x41 bytes); the rival list's count (a word, 0 without
/// a list) and each of its 4-byte entries; the exchange list's count and each of its 7-byte nodes. A short write shows
/// the box (lines 0xee, 0x102, 0x110, 0x125, 0x133) and stops there.
/// Original: `TRANSFER_OFFER::save` (0x8e5b30, transfer_offer.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn transfer_offer_save(this: usize, stream: usize) {
    const FILE: usize = 0xa9e318; // transfer_offer.cpp
    if exe::zwrite(this, 0x41, 1, stream) != 1 {
        show_error_at(FILE, 0xee);
        return;
    }
    for (list, size, count_line, node_line) in [(read_usize(this), 4u32, 0x102, 0x110), (read_usize(this + offer::EXCHANGE_PLAYERS), 7, 0x125, 0x133)] {
        let mut count = if list != 0 { read_u16(list) } else { 0 };
        if exe::zwrite(&mut count as *mut u16 as usize, 2, 1, stream) != 1 {
            show_error_at(FILE, count_line);
            return;
        }
        if list == 0 {
            continue;
        }
        let l = Llist(list);
        l.set_cursor(l.head());
        loop {
            let d = exe::llist_get_next(l);
            if d == 0 {
                break;
            }
            covered!("sx21b_5 list entry saved");
            if exe::zwrite(d, size, 1, stream) != 1 { // sx21b_5: one entry
                show_error_at(FILE, node_line);
                return;
            }
        }
    }
}

/// Comparer for `transfer_offer_save`: only the original (it writes into the saved game's stream); the copy is proven
/// by the saved game written with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_transfer_offer_save(this: usize, stream: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_TRANSFER_OFFER_SAVE)(this, stream)
}

// ---------- sx21b_6: the training manager in the saved game: training.dat loaded and saved, and the destructor (training_manager.cpp) ----------

/// `TRAINING_SCHEDULE::~TRAINING_SCHEDULE` (thiscall), the vector destructor's element destructor.
const FN_TRAINING_SCHEDULE_DESTROY: usize = 0x5a53c0;
/// "training.dat", the saved game's item.
const TRAINING_DAT: usize = 0xa8ccc0;
/// "...\training_manager.cpp", for the error box.
const TRAINING_MANAGER_CPP: usize = 0xa8ccd0;

/// One block of training.dat: its address, record size, record count and the error box line of a short read/write.
unsafe fn training_dat_blocks(this: usize) -> [(usize, u32, u32, u32, u32); 4] {
    use training_manager_fields as tm;
    let players = read_u32(G_PLAYER_DATA_COUNT);
    [
        (read_usize(this + tm::PLAYER_ENTRIES), 6, players, 0x96, 0xf1),
        (this + tm::WEEK_COUNTER, 2, 1, 0x9d, 0xf8),
        (this, 0x9b, 0xa7, 0xa4, 0xff), // sx21b_6: the 167 schedules
        (this + 0x6529, 4, 1, 0xab, 0x106), // the position training count as saved
    ]
}

/// Reads training.dat from the saved game into the training manager; 1 when everything was read.
///
/// `TRAINING_MANAGER::load_training_dat(save)`: no save: the box (line 0x7f) and 0. The item (`exe::zopen` "training.dat",
/// reading) or the box (0x87). Then, each short read showing its box and returning 0: the player entries (6 bytes for
/// each of `G_PLAYER_DATA_COUNT` player sheets, into a new `malloc` block at `PLAYER_ENTRIES`; none: box 0x8f), the
/// `WEEK_COUNTER`, the 167 0x9b-byte schedules, the position training count (into +0x6529, copied to
/// `POSITION_TRAINING_COUNT`) and, when not 0, that many 8-byte items into a new block at `POSITION_TRAINING_ITEMS`
/// (none: box 0xb9; short: 0xc0), and the 0x60 nine-byte coach slots of `COACH_ASSIGNMENTS` (0xc8). The item is
/// closed (`exe::zclose`) only on success.
/// Original: `TRAINING_MANAGER::load_training_dat` (0x8b10d0, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn load_training_dat(this: usize, save: usize) -> u32 {
    use training_manager_fields as tm;
    if save == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x7f);
        return 0;
    }
    let mut z = exe::zopen(save, TRAINING_DAT, 0, 0x16, 4);
    if z == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x87);
        return 0;
    }
    let entries = exe::crt_malloc(read_u32(G_PLAYER_DATA_COUNT).wrapping_mul(6));
    write_usize(this + tm::PLAYER_ENTRIES, entries);
    if entries == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0x8f);
        return 0;
    }
    for (at, size, count, line, _) in training_dat_blocks(this) {
        if exe::zread(at, size, count, z) != count {
            show_error_at(TRAINING_MANAGER_CPP, line);
            return 0;
        }
    }
    let count = read_u32(this + 0x6529);
    write_u32(this + tm::POSITION_TRAINING_COUNT, count);
    if count != 0 {
        covered!("sx21b_6 position training loaded");
        let items = exe::crt_malloc(count.wrapping_mul(8));
        write_usize(this + tm::POSITION_TRAINING_ITEMS, items);
        if items == 0 {
            show_error_at(TRAINING_MANAGER_CPP, 0xb9);
            return 0;
        }
        if exe::zread(items, 8, read_u32(this + 0x6529), z) != read_u32(this + 0x6529) {
            show_error_at(TRAINING_MANAGER_CPP, 0xc0);
            return 0;
        }
    }
    if exe::zread(this + tm::COACH_ASSIGNMENTS, 9, 0x60, z) != 0x60 { // sx21b_6: the coach slots
        show_error_at(TRAINING_MANAGER_CPP, 0xc8);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `load_training_dat`: only the original (it reads the saved game's item and allocates); the copy is
/// proven by loading the holiday save with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_load_training_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_LOAD_TRAINING_DAT)(this, save)
}

/// Writes the training manager into the saved game's training.dat; 1 when everything was written.
///
/// `TRAINING_MANAGER::save_training_dat(save)`: no save: the box (line 0xe2) and 0. The item (`exe::zopen`
/// "training.dat", writing) or the box (0xea). Then, each short write showing its box and returning 0: the player
/// entries (0xf1), the `WEEK_COUNTER` (0xf8), the 167 schedules (0xff), the dword at +0x6529 (0x106), the position
/// training items when there is a block (as many as that dword; 0x110), and the coach slots (0x118). The item is
/// closed only on success.
/// Original: `TRAINING_MANAGER::save_training_dat` (0x8b1450, training_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn save_training_dat(this: usize, save: usize) -> u32 {
    use training_manager_fields as tm;
    if save == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0xe2);
        return 0;
    }
    let mut z = exe::zopen(save, TRAINING_DAT, 1, 0x16, 4);
    if z == 0 {
        show_error_at(TRAINING_MANAGER_CPP, 0xea);
        return 0;
    }
    for (at, size, count, _, line) in training_dat_blocks(this) {
        if exe::zwrite(at, size, count, z) != count {
            show_error_at(TRAINING_MANAGER_CPP, line);
            return 0;
        }
    }
    let items = read_usize(this + tm::POSITION_TRAINING_ITEMS);
    if items != 0 {
        covered!("sx21b_6 position training saved");
        if exe::zwrite(items, 8, read_u32(this + 0x6529), z) != read_u32(this + 0x6529) {
            show_error_at(TRAINING_MANAGER_CPP, 0x110);
            return 0;
        }
    }
    if exe::zwrite(this + tm::COACH_ASSIGNMENTS, 9, 0x60, z) != 0x60 {
        show_error_at(TRAINING_MANAGER_CPP, 0x118);
        return 0;
    }
    exe::zclose(&mut z);
    1
}

/// Comparer for `save_training_dat`: only the original (it writes the saved game's item); the copy is proven by the
/// saved game written with and without the batch.
pub(crate) unsafe extern "thiscall" fn compare_save_training_dat(this: usize, save: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SAVE_TRAINING_DAT)(this, save)
}

/// Destroys the training manager: frees the player entries and the position training items, and destroys the 167
/// schedules.
///
/// `TRAINING_MANAGER::~TRAINING_MANAGER()`: `PLAYER_ENTRIES` and `POSITION_TRAINING_ITEMS`, when present, freed
/// (`exe::crt_free`) and cleared; then `exe::crt_vector_destructor_iterator(this, 0x9b, 0xa7, ~TRAINING_SCHEDULE)`.
/// Original: `TRAINING_MANAGER::~TRAINING_MANAGER` (0x8b1070, training_manager.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn destroy_training_manager(this: usize) {
    use training_manager_fields as tm;
    for at in [tm::PLAYER_ENTRIES, tm::POSITION_TRAINING_ITEMS] {
        let p = read_usize(this + at);
        if p != 0 {
            exe::crt_free(p);
            write_u32(this + at, 0);
        }
    }
    exe::crt_vector_destructor_iterator(this, 0x9b, 0xa7, FN_TRAINING_SCHEDULE_DESTROY);
}

/// Comparer for `destroy_training_manager`: only the original (it frees memory).
pub(crate) unsafe extern "thiscall" fn compare_destroy_training_manager(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DESTROY_TRAINING_MANAGER)(this)
}

// ---------- cx02_13: the English cups' own data in the saved game (eng_cc_cup.cpp, eng_fa_cup.cpp) ----------

/// Reads the League Cup's own data from the saved game: the count of entrants in the European cups
/// (`eng_cc_cup::EUROPEAN_ENTRANT_COUNT`, 1 byte; `exe::crt_fread`). A short read shows the eng_cc_cup.cpp box (line
/// 0x2e4) and returns 0; otherwise 1.
/// Original: `EngCcCup::load_extra_data` (0x56d2e0, eng_cc_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_cc_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1, 1, file) != 1 {
        show_error_at(0x9c4224, 0x2e4); // eng_cc_cup.cpp
        return 0;
    }
    covered!("cx02_13 league cup data loaded");
    1
}

/// Reads the FA Cup's own data from the saved game, one byte each (`exe::crt_fread`, unchecked): the exemption count
/// (`eng_fa_cup::EXEMPTION_COUNT`) and the fifth-round entry choice (`FIFTH_ROUND_ENTRY`); then, when the saved game's
/// data version (`G_GAME_DATA_VERSION`, absolute value) is above 0x3e001, whether the fifth-round question was asked
/// (`FIFTH_ROUND_QUESTION_ASKED`), otherwise that byte becomes 3. Returns 1.
/// Original: `EngFaCup::load_extra_data` (0x570830, eng_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    exe::crt_fread(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file);
    exe::crt_fread(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);
    if read_i32(G_GAME_DATA_VERSION).wrapping_abs() > 0x3e001 {
        covered!("cx02_13 fa cup question flag loaded");
        exe::crt_fread(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 1, file);
    } else {
        write_u8(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 3);
    }
    1
}

/// Writes the FA Cup's own data to the saved game, one byte each (`exe::crt_fwrite`, unchecked), in the order the load
/// reads it: the exemption count, the fifth-round entry choice and whether the fifth-round question was asked. Returns
/// the last write's count.
/// Original: `EngFaCup::save_extra_data` (0x5708a0, eng_fa_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn eng_fa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx02_13 fa cup data saved");
    exe::crt_fwrite(this + eng_fa_cup::EXEMPTION_COUNT, 1, 1, file);
    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_ENTRY, 1, 1, file);
    exe::crt_fwrite(this + eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 1, 1, file)
}

/// Comparer of the cups' own data readers: the original reads first; then the file goes back to where it was and the
/// copy reads the same bytes again over the `fields` (the object as the original left it is put back); the returns,
/// the fields and the file positions must match.
pub(crate) unsafe fn compare_cup_extra_data_load(name: &str, this: usize, file: usize, fields: &[(usize, usize)], original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let at = exe::crt_ftell(file);
    let (mut end_o, mut end_c) = (0, 0);
    let regions: Vec<(usize, usize)> = fields.iter().map(|&(o, n)| (this + o, n)).collect();
    let (r, c, same, mo, mc) = crate::comparison::side_by_side(&regions, || {
        let r = original();
        end_o = exe::crt_ftell(file);
        r
    }, || {
        exe::crt_fseek(file, at, 0);
        let c = copy();
        end_c = exe::crt_ftell(file);
        c
    });
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(same && r == c && end_o == end_c, || format!("{}({:#x}) returned {} x {}, file at {} x {} (from {}), original {:x?} copy {:x?}", name, this, r, c, end_o, end_c, at, mo, mc));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_eng_cc_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ENG_CC_CUP_LOAD_EXTRA_DATA);
    compare_cup_extra_data_load("eng_cc_cup_load_extra_data", this, file, &[(eng_cc_cup::EUROPEAN_ENTRANT_COUNT, 1)], || o(this, file), || eng_cc_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ENG_FA_CUP_LOAD_EXTRA_DATA);
    compare_cup_extra_data_load("eng_fa_cup_load_extra_data", this, file, &[(eng_fa_cup::FIFTH_ROUND_QUESTION_ASKED, 3)], || o(this, file), || eng_fa_cup_load_extra_data(this, file))
}

/// Comparer for `eng_fa_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; the copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_eng_fa_cup_save_extra_data(this: usize, file: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ENG_FA_CUP_SAVE_EXTRA_DATA)(this, file)
}

// ---------- cx02_14: the group cups' entrant list in the saved game (gre_cup.cpp, ire_lge_cup.cpp, ita_c_cup.cpp) ----------

/// The entrant list of a cup built on a league with group stages, read from its temporary file: the count
/// (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes; `exe::crt_fread`); a short read shows the box of `lines.0` and
/// returns 0. With `empty_is_none` (the Dutch Cup) a count of 0 leaves the list null and returns 1. Otherwise a new
/// list of count × 4 bytes (`exe::crt_malloc`, into `ita_cup_fields::ENTRANT_CLUBS`; out of memory the box of
/// `lines.1` and 0) and the count's club ids read into it (short: the box of `lines.2` and 0); each id becomes its club
/// (`club_at`, the club table read each time), −1 a null slot (the count read again each time). Returns 1. The boxes
/// are the source file `file_name`'s.
pub(crate) unsafe fn load_group_cup_entrants(this: usize, file: usize, file_name: usize, lines: (u32, u32, u32), empty_is_none: bool) -> u32 {
    let cup = ItaCupFields(this);
    if exe::crt_fread(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file) != 1 {
        show_error_at(file_name, lines.0);
        return 0;
    }
    let count = || read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    if empty_is_none && count() == 0 {
        cup.set_entrant_clubs(0);
        return 1;
    }
    let list = exe::crt_malloc((count() as u32).wrapping_mul(4));
    cup.set_entrant_clubs(list);
    if list == 0 {
        show_error_at(file_name, lines.1);
        return 0;
    }
    if exe::crt_fread(list, 4, count() as u32, file) != count() as u32 {
        show_error_at(file_name, lines.2);
        return 0;
    }
    covered!("cx02_14 group cup entrants loaded");
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let id = read_i32(slot);
        write_usize(slot, if id == -1 { 0 } else { club_at(id as isize).addr() });
        k += 1;
    }
    1
}

/// The Greek Cup entrant list (`load_group_cup_entrants`, gre_cup.cpp lines 0x383, 0x38b and 0x393).
/// Original: `GreCup::load_extra_data` (0x5e9df0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9cdab0, (0x383, 0x38b, 0x393), false)
}

/// The Irish League Cup entrant list (`load_group_cup_entrants`, ire_lge_cup.cpp lines 0x381, 0x389 and 0x391).
/// Original: `IreLgeCup::load_extra_data` (0x638260, ire_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9e9f4c, (0x381, 0x389, 0x391), false)
}

/// The Serie C cup entrant list (`load_group_cup_entrants`, ita_c_cup.cpp lines 0x3cc, 0x3d4 and 0x3dc).
/// Original: `ItaCCup::load_extra_data` (0x63e3c0, ita_c_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_c_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9ea204, (0x3cc, 0x3d4, 0x3dc), false)
}

/// Comparer of the group cups' entrant list readers: the original reads first (its list kept); then the file goes
/// back to where it was and the copy reads the same bytes into a list of its own; the returns, the counts, the file
/// positions and the lists' clubs must match. The copy's list is freed and the original's put back.
pub(crate) unsafe fn compare_group_cup_entrants_load(name: &str, this: usize, file: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let cup = ItaCupFields(this);
    let at = exe::crt_ftell(file);
    let r = original();
    if r == 0 {
        return r; // the error box was shown: the copy is not run again
    }
    let (end_o, list_o, count_o) = (exe::crt_ftell(file), cup.entrant_clubs(), read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT));
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let (end_c, list_c, count_c) = (exe::crt_ftell(file), cup.entrant_clubs(), read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT));
    let n = if r == 1 && c == 1 && count_o == count_c && list_o != 0 && list_c != 0 { count_o.max(0) as usize } else { 0 };
    let same = (0..n).all(|k| read_usize(list_o + 4 * k) == read_usize(list_c + 4 * k));
    if list_c != 0 && list_c != list_o {
        exe::crt_free(list_c);
    }
    cup.set_entrant_clubs(list_o);
    write_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT, count_o);
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(r == c && count_o == count_c && end_o == end_c && same && (list_o == 0) == (list_c == 0),
        || format!("{}({:#x}) returned {} x {}, count {} x {}, file at {} x {} (from {}), lists equal {}", name, this, r, c, count_o, count_c, end_o, end_c, at, same));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_gre_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GRE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("gre_cup_load_extra_data", this, file, || o(this, file), || gre_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_ire_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IRE_LGE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("ire_lge_cup_load_extra_data", this, file, || o(this, file), || ire_lge_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_ita_c_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ITA_C_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("ita_c_cup_load_extra_data", this, file, || o(this, file), || ita_c_cup_load_extra_data(this, file))
}

// ---------- cx02_15: the group cups' entrant list in the saved game (ita_cup.cpp, kor_league_cup.cpp, nir_lge_cup.cpp) ----------

/// The Coppa Italia entrant list (`load_group_cup_entrants`, ita_cup.cpp lines 0x3cb, 0x3d3 and 0x3db).
/// Original: `ItaCup::load_extra_data` (0x640050, ita_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ita_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9ea23c, (0x3cb, 0x3d3, 0x3db), false)
}

/// The Korean League Cup entrant list (`load_group_cup_entrants`, kor_league_cup.cpp lines 0x320, 0x328 and 0x330).
/// Original: `KorLeagueCup::load_extra_data` (0x66d1e0, kor_league_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9eb15c, (0x320, 0x328, 0x330), false)
}

/// The Northern Irish League Cup entrant list (`load_group_cup_entrants`, nir_lge_cup.cpp lines 0x34f, 0x357 and
/// 0x35f).
/// Original: `NirLgeCup::load_extra_data` (0x78db10, nir_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0xa69698, (0x34f, 0x357, 0x35f), false)
}

pub(crate) unsafe extern "thiscall" fn compare_ita_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_ITA_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("ita_cup_load_extra_data", this, file, || o(this, file), || ita_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_KOR_LEAGUE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("kor_league_cup_load_extra_data", this, file, || o(this, file), || kor_league_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_nir_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_NIR_LGE_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("nir_lge_cup_load_extra_data", this, file, || o(this, file), || nir_lge_cup_load_extra_data(this, file))
}

// ---------- cx02_16: the group cups' entrant list in the saved game, writing, and the Dutch Cup's (gre_cup.cpp, hol_cup.cpp) ----------

/// The entrant list of a cup built on a league with group stages, written to its temporary file in the order
/// `load_group_cup_entrants` reads it: the count (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes; `exe::crt_fwrite`,
/// unchecked); the list's clubs turned into their ids (a null slot −1; the list and the count read again each time);
/// the count's ids written (with `skip_null_list`, the Dutch Cup, not when the list is null); the ids turned back into
/// clubs (`club_at`; −1 a null slot). Returns the count as last read (the eax the original leaves).
pub(crate) unsafe fn save_group_cup_entrants(this: usize, file: usize, skip_null_list: bool) -> u32 {
    let cup = ItaCupFields(this);
    let count = || read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT);
    exe::crt_fwrite(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file);
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let club = read_usize(slot);
        write_u32(slot, if club != 0 { read_u32(club) } else { u32::MAX });
        k += 1;
    }
    if !skip_null_list || cup.entrant_clubs() != 0 {
        covered!("cx02_16 group cup entrants saved");
        exe::crt_fwrite(cup.entrant_clubs(), 4, count() as u32, file);
    }
    let mut k = 0i32;
    while k < count() {
        let slot = cup.entrant_clubs() + 4 * k as usize;
        let id = read_i32(slot);
        write_usize(slot, if id == -1 { 0 } else { club_at(id as isize).addr() });
        k += 1;
    }
    count() as u32
}

/// The entrant list of the Greek Cup and of the other group cups that share this virtual (the Irish, Northern Irish
/// and Korean League Cups, the Coppa Italia, the Serie C cup and the Welsh League and Premier Cups):
/// `save_group_cup_entrants`, the list always written.
/// Original: `GreCup::save_extra_data` (0x5e9fb0, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_save_extra_data(this: usize, file: usize) -> u32 {
    save_group_cup_entrants(this, file, false)
}

/// The Dutch Cup entrant list (`load_group_cup_entrants`, hol_cup.cpp lines 0x44b, 0x455 and 0x45d; a count of 0
/// leaves the list null).
/// Original: `HolCup::load_extra_data` (0x5f4e90, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_group_cup_entrants(this, file, 0x9cedd4, (0x44b, 0x455, 0x45d), true)
}

/// The Dutch Cup entrant list written (`save_group_cup_entrants`, not written when the list is null).
/// Original: `HolCup::save_extra_data` (0x5f5070, hol_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn hol_cup_save_extra_data(this: usize, file: usize) -> u32 {
    save_group_cup_entrants(this, file, true)
}

/// Comparer for `gre_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original runs;
/// the list must come back as it was (the clubs, not the ids). The copy is proven by the save proof.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_save_extra_data(this: usize, file: usize) -> u32 {
    compare_group_cup_entrants_save("gre_cup_save_extra_data", this, || original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_GRE_CUP_SAVE_EXTRA_DATA)(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HOL_CUP_LOAD_EXTRA_DATA);
    compare_group_cup_entrants_load("hol_cup_load_extra_data", this, file, || o(this, file), || hol_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_hol_cup_save_extra_data(this: usize, file: usize) -> u32 {
    compare_group_cup_entrants_save("hol_cup_save_extra_data", this, || original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_HOL_CUP_SAVE_EXTRA_DATA)(this, file))
}

/// Runs only the original writer and checks that the entrant list is back as it was before (clubs, not ids).
unsafe fn compare_group_cup_entrants_save(name: &str, this: usize, original: impl FnOnce() -> u32) -> u32 {
    let list = ItaCupFields(this).entrant_clubs();
    let n = if list != 0 { read_i32(this + ita_cup_fields::ENTRANT_CLUB_COUNT).max(0) as usize } else { 0 };
    let before: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).collect();
    let r = original();
    let after: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).collect();
    SCOREBOARD.record(before == after, || format!("{}({:#x}): the list changed ({} entries)", name, this, n));
    r
}

// ---------- cx02_17: a cup stage in the cup's temporary file (cup_stage.cpp) ----------

/// The Cup_Stage vtable.
const CUP_STAGE_VTABLE: usize = 0x969468;
/// The size of a cup tie (the records of `cup::TIE_TABLES`).
const TIE_SIZE: usize = 0x35;

/// Builds an empty cup stage for loading: the Cup base (`exe::new_cup`), the Cup_Stage vtable, the team table of
/// `teams` 6-byte entries (`cup::TEAM_TABLE`), the round table of `rounds` 0x68-byte rounds (`cup::ROUND_TABLE`), the
/// tie vector of `rounds` pointers (`cup::TIE_TABLES`) and, for each round, its `tie_counts[r]` 0x35-byte ties
/// (`exe::crt_malloc`, the counts taken as 16 bits). The team and round counts are not written here (the load reads
/// them). An allocation that fails shows the cup_stage.cpp box (lines 0x7e, 0x85, 0x8d, 0x97 for a round's ties) and
/// stops there. Returns the object.
/// Original: `Cup_Stage::Cup_Stage` (0x5230e0, cup_stage.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this, CUP_STAGE_VTABLE);
    let (teams, rounds) = (teams as i16 as i32, rounds as i16 as i32);
    cup.set_team_table(exe::crt_malloc((teams * 6) as u32));
    if cup.team_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x7e);
        return this;
    }
    cup.set_round_table(exe::crt_malloc((rounds * cup_round::SIZE as i32) as u32));
    if cup.round_table() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x85);
        return this;
    }
    cup.set_tie_tables(exe::crt_malloc((rounds * 4) as u32));
    if cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x8d);
        return this;
    }
    covered!("cx02_17 cup stage built for loading");
    for r in 0..rounds.max(0) as usize {
        let ties = exe::crt_malloc((*tie_counts.add(r) as i32 * TIE_SIZE as i32) as u32);
        write_usize(cup.tie_tables() + 4 * r, ties);
        if ties == 0 {
            show_error_at(CUP_STAGE_SOURCE, 0x97);
            break;
        }
    }
    this
}

/// One read of a cup stage's load: `count` items of `size` bytes at `at` (`exe::crt_fread`); fewer than `expected`
/// shows the cup_stage.cpp box at `line` and the game stops (`fatal_error_at`).
unsafe fn cup_stage_read(at: usize, size: u32, count: u32, file: usize, expected: u32, line: u32) {
    if exe::crt_fread(at, size, count, file) != expected {
        fatal_error_at(CUP_STAGE_SOURCE, line);
    }
}

/// The fields of a cup stage in its file, in order: (field, size, the box line of a short read).
const CUP_STAGE_FILE_FIELDS: [(usize, u32, u32); 15] = [(cup::PRIZE_FUND, 4, 0xe9), (cup::IMPORTANCE_BIAS, 1, 0xef), (cup::SUBSTITUTES_NAMED, 1, 0xf5),
    (cup::SUBSTITUTIONS_ALLOWED, 1, 0xfb), (cup::RUN_BY_STAGES, 1, 0x101), (cup::ROUND_COUNT, 2, 0x107), (cup::TEAM_COUNT, 2, 0x10d), (cup::YEAR, 2, 0x113),
    (cup::CURRENT_ROUND, 1, 0x11a), (cup::NEXT_DRAW_ROUND, 1, 0x121), (cup::STAGE_INDEX, 1, 0x128), (cup::CLUB_SIDES, 1, 0x12f), (cup::STAGE_LABEL, 2, 0x136),
    (league::UPPER_DIVISION_ID, 4, 0x13d), (league::LOWER_DIVISION_ID, 4, 0x144)];

/// Reads a cup stage back from the cup's temporary file, in the order `cup_stage_save_to_file` wrote it: the fields
/// (`CUP_STAGE_FILE_FIELDS`, one by one), the competition id (the record of that id in [`G_CLUB_COMP_TABLE`] into
/// `league::COMPETITION`, −1 none), the round table (`ROUND_COUNT` rounds of 0x68 bytes), the team table (`TEAM_COUNT`
/// 6-byte entries, their club ids turned into clubs: `club_entries_from_ids`) and each round's ties (the round's
/// `cup_round::TIES` × 0x35 bytes, one read). Any short read shows the box of its line and the game stops. Returns 1.
/// Defect of the original: the team table's read is checked against the round table's count read (the check of line
/// 0x163 repeats the round table's), so a short team table goes unnoticed.
/// Original: `Cup_Stage::load_from_file` (0x523380, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let cup = Cup(this);
    for (field, size, line) in CUP_STAGE_FILE_FIELDS {
        cup_stage_read(this + field, size, 1, file, 1, line);
    }
    let mut competition = 0i32;
    cup_stage_read(&mut competition as *mut i32 as usize, 4, 1, file, 1, 0x14b);
    write_usize(this + league::COMPETITION, if competition == -1 { 0 } else { read_usize(G_CLUB_COMP_TABLE).wrapping_add((competition as usize).wrapping_mul(0x6b)) });
    let rounds = cup.round_count() as i32 as u32;
    cup_stage_read(cup.round_table(), cup_round::SIZE as u32, rounds, file, rounds, 0x15b);
    exe::crt_fread(cup.team_table(), 6, cup.team_count() as i32 as u32, file); // checked against the round count: passes
    club_entries_from_ids(cup.team_table(), cup.team_count());
    covered!("cx02_17 cup stage loaded");
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        cup_stage_read(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file, 1, 0x17a);
        r += 1;
    }
    1
}

/// Writes a cup stage into the cup's temporary file (`exe::crt_fwrite`, unchecked), in the order
/// `cup_stage_load_from_file` reads it: without a round table, team table or tie vector, the cup_stage.cpp box (line
/// 0x19a) and nothing. The fields (`CUP_STAGE_FILE_FIELDS`), the competition record's id (its first dword; without a
/// record the box of line 0x1b9 and −1), the round table, the team table with its clubs written as ids and turned back
/// into clubs, and each round's ties.
/// Original: `Cup_Stage::save_to_file` (0x523eb0, cup_stage.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn cup_stage_save_to_file(this: usize, file: usize) {
    let cup = Cup(this);
    let no_competition = u32::MAX;
    if cup.round_table() == 0 || cup.team_table() == 0 || cup.tie_tables() == 0 {
        show_error_at(CUP_STAGE_SOURCE, 0x19a);
        return;
    }
    covered!("cx02_17 cup stage saved");
    for (field, size, _) in CUP_STAGE_FILE_FIELDS {
        exe::crt_fwrite(this + field, size, 1, file);
    }
    let competition = read_usize(this + league::COMPETITION);
    if competition != 0 {
        exe::crt_fwrite(competition, 4, 1, file);
    } else {
        show_error_at(CUP_STAGE_SOURCE, 0x1b9);
        exe::crt_fwrite(&no_competition as *const u32 as usize, 4, 1, file);
    }
    exe::crt_fwrite(cup.round_table(), cup_round::SIZE as u32, cup.round_count() as i32 as u32, file);
    club_entries_to_ids(cup.team_table(), cup.team_count());
    exe::crt_fwrite(cup.team_table(), 6, cup.team_count() as i32 as u32, file);
    club_entries_from_ids(cup.team_table(), cup.team_count());
    let mut r = 0i32;
    while r < cup.round_count() as i32 {
        let ties = read_i16(cup.round_table() + r as usize * cup_round::SIZE + cup_round::TIES) as i32;
        exe::crt_fwrite(read_usize(cup.tie_tables() + 4 * r as usize), (ties * TIE_SIZE as i32) as u32, 1, file);
        r += 1;
    }
}

/// Comparer for `new_cup_stage_for_load`: it allocates (it cannot run twice), so only the original runs; the vtable,
/// the return and the three tables must be there.
pub(crate) unsafe extern "thiscall" fn compare_new_cup_stage_for_load(this: usize, teams: u32, rounds: u32, tie_counts: *const i16) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *const i16) -> usize>(IDX_NEW_CUP_STAGE_FOR_LOAD)(this, teams, rounds, tie_counts);
    let cup = Cup(this);
    SCOREBOARD.record(r == this && read_usize(this) == CUP_STAGE_VTABLE && cup.team_table() != 0 && cup.round_table() != 0 && cup.tie_tables() != 0,
        || format!("new_cup_stage_for_load({:#x}, {}, {}) returned {:#x}, vtable {:#x}", this, teams as i16, rounds as i16, r, read_usize(this)));
    r
}

/// The bytes a cup stage's load leaves: the object's first 0xb2 bytes, the round table, the team table and each
/// round's ties.
unsafe fn cup_stage_loaded_regions(this: usize) -> Vec<(usize, usize)> {
    let cup = Cup(this);
    let rounds = cup.round_count().max(0) as usize;
    let mut regions = vec![(this, 0xb2usize), (cup.round_table(), rounds * cup_round::SIZE), (cup.team_table(), cup.team_count().max(0) as usize * 6)];
    for r in 0..rounds {
        let ties = read_i16(cup.round_table() + r * cup_round::SIZE + cup_round::TIES).max(0) as usize;
        regions.push((read_usize(cup.tie_tables() + 4 * r), ties * TIE_SIZE));
    }
    regions
}

/// Comparer for `cup_stage_load_from_file`: the original reads; the file goes back to where it was and the copy reads
/// the same bytes again over the same tables; the object, the tables, the return and the file position must match.
/// The original's state is put back.
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_load_from_file(this: usize, file: usize) -> u32 {
    let at = exe::crt_ftell(file);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CUP_STAGE_LOAD_FROM_FILE)(this, file);
    let end = exe::crt_ftell(file);
    let regions = cup_stage_loaded_regions(this);
    let original = crate::comparison::MemorySnapshot::read(&regions);
    exe::crt_fseek(file, at, 0);
    let c = cup_stage_load_from_file(this, file);
    let copy = crate::comparison::MemorySnapshot::read(&regions);
    let end_c = exe::crt_ftell(file);
    original.write_back();
    exe::crt_fseek(file, end, 0);
    SCOREBOARD.record(r == c && end == end_c && original == copy, || format!("cup_stage_load_from_file({:#x}) returned {} x {}, file at {} x {} (from {}), state equal {}", this, r, c, end, end_c, at, original == copy));
    r
}

/// Comparer for `cup_stage_save_to_file`: it writes the cup's temporary file (it cannot run twice), so only the
/// original runs; the team table must come back with its clubs. The copy is proven by the save proof (and by the
/// cup's file comparer of `cup_save_cup_to_disk`, which calls it).
pub(crate) unsafe extern "thiscall" fn compare_cup_stage_save_to_file(this: usize, file: usize) {
    let cup = Cup(this);
    let n = if cup.team_table() != 0 { cup.team_count().max(0) as usize } else { 0 };
    let before: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_CUP_STAGE_SAVE_TO_FILE)(this, file);
    let after: Vec<usize> = (0..n).map(|k| read_usize(cup.team_table() + 6 * k)).collect();
    SCOREBOARD.record(before == after, || format!("cup_stage_save_to_file({:#x}): the team table changed", this));
}

// ---------- cx03_11: the Scottish and Welsh league cups' own data in the saved game (sco_lge_cup.cpp, wel_lge_cup.cpp, wel_prm_cup.cpp) ----------

/// Reads the Scottish League Cup's own data from its temporary file: the byes byte (+0xb2, the clubs in the European
/// cups) (`exe::crt_fread` of 1 byte, cdecl); a short read: the box of line 0x2b8 of sco_lge_cup.cpp and 0. Returns 1.
/// Original: `ScoLgeCup::load_extra_data` (0x7f0ae0, sco_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    if exe::crt_fread(this + 0xb2, 1, 1, file) != 1 {
        show_error_at(0xa7af38, 0x2b8);
        return 0;
    }
    covered!("cx03_11 scottish league cup data loaded");
    1
}

/// The body of the Welsh league cups' `load_extra_data`: the entrant count (`ita_cup_fields::ENTRANT_CLUB_COUNT`, 4 bytes;
/// a short read: the box of line `rows[0]` and 0), the entrant list (`exe::crt_malloc` of 4 per entrant into
/// `ita_cup_fields::ENTRANT_CLUBS`; out of memory, the box of `rows[1]` and 0) read whole (a short read: the box of
/// `rows[2]` and 0), then each club index back into a club pointer (`club_at`; −1, a free slot, becomes null). Returns
/// 1.
unsafe fn load_welsh_cup_entrants(this: usize, file: usize, source: usize, rows: [u32; 3], branch: &'static str) -> u32 {
    let cup = ItaCupFields(this);
    if exe::crt_fread(this + ita_cup_fields::ENTRANT_CLUB_COUNT, 4, 1, file) != 1 {
        show_error_at(source, rows[0]);
        return 0;
    }
    let list = exe::crt_malloc(cup.entrant_club_count() as u32 * 4);
    cup.set_entrant_clubs(list);
    if list == 0 {
        show_error_at(source, rows[1]);
        return 0;
    }
    if exe::crt_fread(list, 4, cup.entrant_club_count() as u32, file) != cup.entrant_club_count() as u32 {
        show_error_at(source, rows[2]);
        return 0;
    }
    covered!(branch);
    let mut k = 0i32;
    while k < cup.entrant_club_count() as i32 {
        let slot = cup.entrant_clubs() + k as usize * 4;
        let index = read_i32(slot);
        write_usize(slot, if index == -1 { 0 } else { club_at(index as isize).addr() });
        k += 1;
    }
    1
}

/// Reads the Welsh League Cup's entrants from its temporary file (`load_welsh_cup_entrants`, the boxes of lines 0x348,
/// 0x350 and 0x358 of wel_lge_cup.cpp).
/// Original: `WelLgeCup::load_extra_data` (0x927330, wel_lge_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_welsh_cup_entrants(this, file, 0xad56f4, [0x348, 0x350, 0x358], "cx03_11 welsh league cup entrants loaded")
}

/// Reads the Welsh Premier Cup's entrants from its temporary file (`load_welsh_cup_entrants`, the boxes of lines 0x36e,
/// 0x376 and 0x37e of wel_prm_cup.cpp).
/// Original: `WelPrmCup::load_extra_data` (0x928c40, wel_prm_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wel_prm_cup_load_extra_data(this: usize, file: usize) -> u32 {
    load_welsh_cup_entrants(this, file, 0xad572c, [0x36e, 0x376, 0x37e], "cx03_11 welsh premier cup entrants loaded")
}

pub(crate) unsafe extern "thiscall" fn compare_sco_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCO_LGE_CUP_LOAD_EXTRA_DATA);
    compare_cup_extra_data_load("sco_lge_cup_load_extra_data", this, file, &[(0xb2, 1)], || o(this, file), || sco_lge_cup_load_extra_data(this, file))
}

/// Comparer of the Welsh entrant readers: the original reads first; then the file goes back to where it was and the
/// copy reads the same bytes into a list of its own; the returns, the counts, the lists and the file positions must
/// match. The copy's list is freed and the original's put back.
unsafe fn compare_welsh_cup_entrants(name: &str, this: usize, file: usize, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let cup = ItaCupFields(this);
    let at = exe::crt_ftell(file);
    let r = original();
    let (end_o, list_o, n_o) = (exe::crt_ftell(file), cup.entrant_clubs(), cup.entrant_club_count());
    exe::crt_fseek(file, at, 0);
    let c = copy();
    let (end_c, list_c, n_c) = (exe::crt_ftell(file), cup.entrant_clubs(), cup.entrant_club_count());
    let k = 4 * (n_o as i32).max(0) as usize;
    let same = n_o == n_c && (r == 0 || (list_o != 0 && list_c != 0
        && core::slice::from_raw_parts(list_o as *const u8, k) == core::slice::from_raw_parts(list_c as *const u8, k)));
    if list_c != 0 && list_c != list_o {
        exe::crt_free(list_c);
    }
    cup.set_entrant_clubs(list_o);
    cup.set_entrant_club_count(n_o);
    exe::crt_fseek(file, end_o, 0);
    SCOREBOARD.record(same && r == c && end_o == end_c, || format!("{}({:#x}) returned {} x {}, file at {} x {} (from {}), entrants {} x {}", name, this, r, c, end_o, end_c, at, n_o, n_c));
    r
}

pub(crate) unsafe extern "thiscall" fn compare_wel_lge_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_WEL_LGE_CUP_LOAD_EXTRA_DATA);
    compare_welsh_cup_entrants("wel_lge_cup_load_extra_data", this, file, || o(this, file), || wel_lge_cup_load_extra_data(this, file))
}

pub(crate) unsafe extern "thiscall" fn compare_wel_prm_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_WEL_PRM_CUP_LOAD_EXTRA_DATA);
    compare_welsh_cup_entrants("wel_prm_cup_load_extra_data", this, file, || o(this, file), || wel_prm_cup_load_extra_data(this, file))
}
