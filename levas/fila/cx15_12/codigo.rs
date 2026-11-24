// ---------- cx15_12: the African World Cup qualifiers' own data in the saved game (wc_african_cup.cpp) ----------

/// Where a World Cup qualifying section keeps its national team list: the exempt count (a byte), the entry count (a
/// dword), the list (national team pointers in memory, club ids in the file, −1 for none) and the source file of its
/// error boxes. Shared by the African (`AFRICAN_QUALIFIER_LIST`) and Asian (cx15_13) sections.
pub(crate) struct QualifierList {
    pub(crate) exempt: usize,
    pub(crate) count: usize,
    pub(crate) list: usize,
    pub(crate) file: usize,
}

/// The African section: the exempt count `wc_cup_turnover_consts::AFRICAN_FORMAT`, the places +0xb6, the entrant list
/// `wc_cup_turnover_consts::CUP_ENTRANT_LIST`.
pub(crate) const AFRICAN_QUALIFIER_LIST: QualifierList = QualifierList {
    exempt: wc_cup_turnover_consts::AFRICAN_FORMAT,
    count: 0xb6,
    list: wc_cup_turnover_consts::CUP_ENTRANT_LIST,
    file: wc_cup_turnover_consts::WC_AFRICAN_CUP_CPP,
};

/// The national team list read from the file turned back into club records: an id other than −1 is the club of that
/// index in the club table (`club_at`), −1 gives null; `count` entries (re-read on each pass, signed).
pub(crate) unsafe fn qualifier_list_from_ids(this: usize, q: &QualifierList) {
    let mut k = 0i32;
    while k < read_i32(this + q.count) {
        let at = read_usize(this + q.list) + 4 * k as usize;
        let id = read_i32(at);
        write_usize(at, if id != -1 { club_at(id as isize).addr() } else { 0 });
        k += 1;
    }
}

/// Writes a qualifying section's own data to the saved game stream `file` (`exe::crt_fwrite`, cdecl, results not
/// checked): the exempt count (1 byte), the entry count (4 bytes) and the list as club ids (each record's id, −1 for
/// none, written in place over the pointers and turned back with `qualifier_list_from_ids` afterwards). Returns the
/// entry count, as the original's eax.
pub(crate) unsafe fn save_qualifier_list(this: usize, q: &QualifierList, file: usize) -> u32 {
    exe::crt_fwrite(this + q.exempt, 1, 1, file);
    exe::crt_fwrite(this + q.count, 4, 1, file);
    let mut k = 0i32;
    while k < read_i32(this + q.count) {
        let at = read_usize(this + q.list) + 4 * k as usize;
        let team = read_usize(at);
        write_u32(at, if team != 0 { read_u32(team) } else { u32::MAX });
        k += 1;
    }
    exe::crt_fwrite(read_usize(this + q.list), 4, read_u32(this + q.count), file);
    qualifier_list_from_ids(this, q);
    read_u32(this + q.count)
}

/// Reads the African section's own data from the saved game stream `file` (`exe::crt_fread`, cdecl): the exempt count
/// (1 byte; short: the error box of line 0x4e3 and 0), the places (4 bytes; the box of line 0x4eb), a new 52-entry list
/// (`exe::crt_malloc(0xd0)`, the old one not freed; out of memory, the box of line 0x4f3), the places' worth of ids
/// (short: the box of line 0x4fb) turned into club records (`qualifier_list_from_ids`). Returns 1.
/// Original: `WcAfricanCup::load_extra_data` (0x910f00, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let q = &AFRICAN_QUALIFIER_LIST;
    if exe::crt_fread(this + q.exempt, 1, 1, file) != 1 {
        show_error_at(q.file, 0x4e3);
        return 0;
    }
    if exe::crt_fread(this + q.count, 4, 1, file) != 1 {
        show_error_at(q.file, 0x4eb);
        return 0;
    }
    let list = exe::crt_malloc(0xd0);
    write_usize(this + q.list, list);
    if list == 0 {
        show_error_at(q.file, 0x4f3);
        return 0;
    }
    if exe::crt_fread(list, 4, read_u32(this + q.count), file) != read_u32(this + q.count) {
        show_error_at(q.file, 0x4fb);
        return 0;
    }
    covered!("cx15_12 african qualifiers loaded");
    qualifier_list_from_ids(this, q);
    1
}

/// Writes the African section's own data (`save_qualifier_list` with `AFRICAN_QUALIFIER_LIST`).
/// Original: `WcAfricanCup::save_extra_data` (0x911110, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx15_12 african qualifiers saved");
    save_qualifier_list(this, &AFRICAN_QUALIFIER_LIST, file)
}

/// The plan of a loaded qualifier list: every entry null or a club record of the club table.
pub(crate) unsafe fn qualifier_list_is_clubs(this: usize, q: &QualifierList) -> bool {
    let (table, n) = (read_usize(G_CLUB_TABLE), read_i32(G_CLUB_COUNT).max(0) as usize);
    let list = read_usize(this + q.list);
    (0..read_i32(this + q.count).max(0) as usize).all(|k| {
        let c = read_usize(list + 4 * k);
        c == 0 || (c >= table && c < table + n * club::SIZE && (c - table) % club::SIZE == 0)
    })
}

/// Comparer for `wc_african_cup_load_extra_data`: it reads the saved game (it cannot run twice), so only the original
/// runs; then the plan: 1 and a list of club records.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_load_extra_data(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    let ok = r == 1 && qualifier_list_is_clubs(this, &AFRICAN_QUALIFIER_LIST);
    SCOREBOARD.record(ok, || format!("wc_african_cup_load_extra_data({:#x}) returned {} with {} entries", this, r, read_i32(this + AFRICAN_QUALIFIER_LIST.count)));
    r
}

/// Comparer for `wc_african_cup_save_extra_data`: it writes the saved game (it cannot run twice), so only the original
/// runs; then the plan: the list is back as it was (the ids turned into records again).
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_save_extra_data(this: usize, file: usize) -> u32 {
    let q = &AFRICAN_QUALIFIER_LIST;
    let list = read_usize(this + q.list);
    let n = read_i32(this + q.count).clamp(0, 0x34) as usize;
    let before: Vec<usize> = (0..if list == 0 { 0 } else { n }).map(|k| read_usize(list + 4 * k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file);
    let after: Vec<usize> = (0..before.len()).map(|k| read_usize(list + 4 * k)).collect();
    SCOREBOARD.record(before == after, || format!("wc_african_cup_save_extra_data({:#x}): the list changed", this));
    r
}
