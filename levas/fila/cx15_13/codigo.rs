// ---------- cx15_13: the Asian World Cup qualifiers' own data in the saved game (wc_asia_league.cpp) ----------

/// The Asian section: the exempt count +0xee, the entrant count +0xf3, the national team list
/// `wc_turnover_consts::ENTRANT_LIST`.
pub(crate) const ASIAN_QUALIFIER_LIST: QualifierList = QualifierList {
    exempt: 0xee,
    count: 0xf3,
    list: wc_turnover_consts::ENTRANT_LIST,
    file: wc_turnover_consts::WC_ASIA_LEAGUE_CPP,
};

/// Reads the Asian section's own data from the saved game stream `file` (`exe::crt_fread`, cdecl): the exempt count (1
/// byte; short: the error box of line 0x646 and 0) and the entrant count (4 bytes; the box of line 0x64e); with
/// entrants, a new list of 4 × count bytes (`exe::crt_malloc`, the old one not freed; out of memory, the box of line
/// 0x659), the ids (short: the box of line 0x661) turned into club records (`qualifier_list_from_ids`); without
/// entrants the list stays as it was. Returns 1.
/// Original: `WcAsiaLeague::load_extra_data` (0x914200, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_load_extra_data(this: usize, file: usize) -> u32 {
    let q = &ASIAN_QUALIFIER_LIST;
    if exe::crt_fread(this + q.exempt, 1, 1, file) != 1 {
        show_error_at(q.file, 0x646);
        return 0;
    }
    if exe::crt_fread(this + q.count, 4, 1, file) != 1 {
        show_error_at(q.file, 0x64e);
        return 0;
    }
    let n = read_u32(this + q.count);
    if n == 0 {
        return 1;
    }
    let list = exe::crt_malloc(n.wrapping_mul(4));
    write_usize(this + q.list, list);
    if list == 0 {
        show_error_at(q.file, 0x659);
        return 0;
    }
    if exe::crt_fread(list, 4, read_u32(this + q.count), file) != read_u32(this + q.count) {
        show_error_at(q.file, 0x661);
        return 0;
    }
    covered!("cx15_13 asian qualifiers loaded");
    qualifier_list_from_ids(this, q);
    1
}

/// Writes the Asian section's own data (`save_qualifier_list` with `ASIAN_QUALIFIER_LIST`).
/// Original: `WcAsiaLeague::save_extra_data` (0x914410, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_save_extra_data(this: usize, file: usize) -> u32 {
    covered!("cx15_13 asian qualifiers saved");
    save_qualifier_list(this, &ASIAN_QUALIFIER_LIST, file)
}

/// Comparer for `wc_asia_league_load_extra_data`: only the original (it reads the saved game); then the plan: 1 and a
/// list of club records.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_load_extra_data(this: usize, file: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%)(this, file);
    let q = &ASIAN_QUALIFIER_LIST;
    let ok = r == 1 && (read_i32(this + q.count) == 0 || qualifier_list_is_clubs(this, q));
    SCOREBOARD.record(ok, || format!("wc_asia_league_load_extra_data({:#x}) returned {} with {} entries", this, r, read_i32(this + q.count)));
    r
}

/// Comparer for `wc_asia_league_save_extra_data`: only the original (it writes the saved game); then the plan: the list
/// is back as it was.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_save_extra_data(this: usize, file: usize) -> u32 {
    let q = &ASIAN_QUALIFIER_LIST;
    let list = read_usize(this + q.list);
    let n = read_i32(this + q.count).clamp(0, 0x2c) as usize;
    let before: Vec<usize> = (0..if list == 0 { 0 } else { n }).map(|k| read_usize(list + 4 * k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, file);
    let after: Vec<usize> = (0..before.len()).map(|k| read_usize(list + 4 * k)).collect();
    SCOREBOARD.record(before == after, || format!("wc_asia_league_save_extra_data({:#x}): the list changed", this));
    r
}
