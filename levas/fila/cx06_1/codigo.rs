// ---------- cx06_1: the destructors of the European Championship qualifiers and the Confederations Cup, and the qualifiers' progress text ----------

/// Destroys the European Championship qualifiers: the vtable 0x96a0d4 back, the table freed (`exe::league_free_table`
/// with 0), the round dates freed and cleared, every stage up to the current one destroyed and the stage vector freed
/// and cleared (`delete_stages_and_vector`), the stored group results (the `LLIST` at +0x10: `exe::drop_llist` and the
/// operator delete), the statistics (`delete_comp_stats`), the entrant list (+0xee, freed but not cleared), then
/// League::~League (`exe::drop_league`). The pointers of the list, the statistics and the entrants are left as they are.
/// Original: `EuroChampQual::~EuroChampQual` (0x57d0a0, euro_champ_qual.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_euro_champ_qual(this: usize) {
    let l = League(this);
    covered!("cx06_1 euro qualifiers destroyed");
    write_usize(this, 0x96a0d4); // the qualifiers' vtable
    exe::league_free_table(this, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    delete_stages_and_vector(l);
    let results = read_usize(this + 0x10); // the stored group results (LLIST)
    if results != 0 {
        exe::drop_llist(results);
        exe::crt_operator_delete(results);
    }
    delete_comp_stats(l.comp_stats());
    let entrants = read_usize(this + 0xee); // the entrant list
    if entrants != 0 {
        exe::crt_free(entrants);
    }
    exe::drop_league(this);
}

/// The scalar deleting destructor of the European Championship qualifiers: `EuroChampQual::~EuroChampQual` and, with
/// bit 0 of `flags`, the object (`delete_through`).
/// Original: `EuroChampQual::scalar_deleting_destructor` (0x57d080, euro_champ_qual.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_euro_champ_qual(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x57d0a0)
}

/// Destroys the Confederations Cup: the national tournaments' body (`drop_stats_league` with the vtable 0x96a270: the
/// table, the round dates, every stage up to the current one and the stage vector, the statistics, then
/// League::~League).
/// Original: `FifaConfed::~FifaConfed` (0x58b4a0, fifa_confed.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn drop_fifa_confed(this: usize) {
    covered!("cx06_1 confederations cup destroyed");
    drop_stats_league(this, 0x96a270); // the Confederations Cup vtable
}

/// The scalar deleting destructor of the Confederations Cup: `FifaConfed::~FifaConfed` and, with bit 0 of `flags`, the
/// object (`delete_through`).
/// Original: `FifaConfed::scalar_deleting_destructor` (0x58b480, fifa_confed.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn delete_fifa_confed(this: usize, flags: u32) -> usize {
    delete_through(this, flags, 0x58b4a0)
}

/// Writes a nation's progress in the European Championship qualifiers into `buf`: in a group stage (`round` 0x3e8 to
/// 0x3f5) "Qualified For Finals" for status 7 (with the competition name's gender, `competition::GENDER_NAME` of
/// `comp`), otherwise "Group Stage"; elsewhere (the play-off) "Qualified For Finals" for status 1, otherwise "Playoff".
/// The texts are translated (`FN_LANGLIB_SPRINTF`). A buffer under 30 bytes shows the euro_champ_qual.cpp box (line
/// 0x31d) and writes nothing. Returns what the writer returned (the original leaves it in eax; 0 after the box). `this`
/// and `sub_round` are not read.
/// Original: `EuroChampQual::write_progress_text` (0x57e9d0, euro_champ_qual.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn euro_champ_qual_write_progress_text(_this: usize, buf: usize, size: i32, round: u32, _sub_round: u32, status: u32, comp: usize) -> i32 {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    if size < 0x1e {
        show_error_at(0x9c4674, 0x31d); // euro_champ_qual.cpp
        return 0;
    }
    let qualified_for_finals = || format(buf, 0x9c48a4, read_u8(comp + competition::GENDER_NAME) as i8 as i32, 0xad9c64usize); // "Qualified For Finals"
    let status = status as u8;
    if (0x3e8..=0x3f5).contains(&(round as u16 as i16)) {
        covered!("cx06_1 qualifiers group progress");
        return if status == 7 { qualified_for_finals() } else { format(buf, 0x99b800) }; // "Group Stage"
    }
    covered!("cx06_1 qualifiers playoff progress");
    if status == 1 { qualified_for_finals() } else { format(buf, 0x9a751c) } // "Playoff"
}

/// Comparer for `euro_champ_qual_write_progress_text`: the original into the caller's buffer, the copy into a local one
/// of the same size (up to 4 KB) side by side; the texts must match. Below 30 bytes (the box) only the original.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_qual_write_progress_text(this: usize, buf: usize, size: i32, round: u32, sub_round: u32, status: u32, comp: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32, usize) -> i32>(%%I4%%);
    let r = o(this, buf, size, round, sub_round, status, comp);
    if size < 0x1e || buf == 0 {
        return r;
    }
    let mut local = vec![0u8; (size as usize).min(0x1000)];
    euro_champ_qual_write_progress_text(this, local.as_mut_ptr() as usize, local.len() as i32, round, sub_round, status, comp);
    let c = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes().to_vec();
    let (mo, mr) = (c(buf as *const u8), c(local.as_ptr()));
    SCOREBOARD.record(mo == mr, || format!("euro_champ_qual_write_progress_text(round {:#x}, status {}): original {:?} rust {:?}", round as u16, status as u8, String::from_utf8_lossy(&mo), String::from_utf8_lossy(&mr)));
    r
}

/// Comparers of the destructors: they free memory (they cannot run twice); only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_drop_euro_champ_qual(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_euro_champ_qual(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%)(this, flags)
}

pub(crate) unsafe extern "thiscall" fn compare_drop_fifa_confed(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}

pub(crate) unsafe extern "thiscall" fn compare_delete_fifa_confed(this: usize, flags: u32) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I3%%)(this, flags)
}
