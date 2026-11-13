// ---------- batch %%LEVA%%: the award manager: a nation's awards and the award history files (award/award_manager.cpp) ----------

/// Fields of the AWARD_MANAGER object and of its nation award objects.
pub(crate) mod award_manager_consts {
    /// The two FILE_LLISTs of the award history: year awards (+0) and team awards (+4).
    pub(crate) const YEAR_AWARD_FILE: usize = 0x0;
    pub(crate) const TEAM_AWARD_FILE: usize = 0x4;
    /// The 30-slot table of nation award objects.
    pub(crate) const NATION_AWARDS: usize = 0x8;
    pub(crate) const NATION_AWARD_SLOTS: usize = 30;
    /// A nation award object's nation (+0xe) and its kind byte (+0x12, 1 = a nation's awards).
    pub(crate) const AWARDS_NATION: usize = 0xe;
    pub(crate) const AWARDS_KIND: usize = 0x12;
    /// "E:\dev\CM3\cm3 00-01\cm3\code\award\award_manager.cpp" (the error box).
    pub(crate) const AWARD_MANAGER_CPP: usize = 0x987e24;
}

/// Appends a year award history record to the year award file list.
///
/// `AWARD_MANAGER::add_year_award_record(record, first, last)`: `exe::file_llist_add_node` on the first file list
/// (`YEAR_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_year_award_record` (0x4142c0, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::YEAR_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_year_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_year_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(%%I0%%)(this, record, first, last)
}

/// Appends a team award history record to the team award file list.
///
/// `AWARD_MANAGER::add_team_award_record(record, first, last)`: `exe::file_llist_add_node` on the second file list
/// (`TEAM_AWARD_FILE`) with (first, last, record), and returns what it returns.
/// Original: `AWARD_MANAGER::add_team_award_record` (0x414300, award/award_manager.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    exe::file_llist_add_node(read_usize(this + award_manager_consts::TEAM_AWARD_FILE), first, last, record as *const u8)
}

/// Comparer for `award_manager_add_team_award_record`.
///
/// The record is written to the history file: it cannot run twice. Only the original runs.
pub(crate) unsafe extern "thiscall" fn compare_award_manager_add_team_award_record(this: usize, record: usize, first: usize, last: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize>(%%I1%%)(this, record, first, last)
}

/// The nation award object of a nation: the first of the 30 slots (`NATION_AWARDS`) holding one of kind 1 for that
/// nation, or 0.
///
/// `AWARD_MANAGER::find_nation_awards(nation)`. Without a nation, the game's error box (line 0x2b3) and 0.
/// Original: `AWARD_MANAGER::find_nation_awards` (0x414340, award/award_manager.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn find_nation_awards(this: usize, nation: usize) -> usize {
    use award_manager_consts as am;
    if nation == 0 {
        show_error_at(am::AWARD_MANAGER_CPP, 0x2b3);
        return 0;
    }
    covered!("mwg_6 nation awards");
    let table = read_usize(this + am::NATION_AWARDS);
    (0..am::NATION_AWARD_SLOTS)
        .map(|i| read_usize(table + i * 4))
        .find(|&e| e != 0 && read_u8(e + am::AWARDS_KIND) == 1 && read_usize(e + am::AWARDS_NATION) == nation) // mwg6: the nation's awards
        .unwrap_or(0)
}

compare_value!(compare_find_nation_awards, find_nation_awards, %%I2%%, (nation: usize) -> usize);
