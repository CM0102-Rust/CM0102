// ---------- sx05b_25: the award history lists and the team award drawing's formation (award screens) ----------

/// `AWARD_MANAGER::get_year_award_records(first, last)` (0x4142e0) and `get_team_award_records(first, last)`
/// (0x414320), thiscall on `G_AWARD_MANAGER`, ret 8: a new LLIST of the history records between two offsets (or 0).
const FN_AWARD_MANAGER_GET_YEAR_AWARD_RECORDS: usize = 0x4142e0;
const FN_AWARD_MANAGER_GET_TEAM_AWARD_RECORDS: usize = 0x414320;

unsafe fn award_history_records(f: usize, first: u32, last: u32) -> usize {
    let get: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(f);
    get(read_usize(G_AWARD_MANAGER), first, last)
}

/// The team award's past teams: the history records between its offsets (`AWARD_MANAGER::get_team_award_records`)
/// and, when the selection is complete (filled ≥ 11 + substitutes, signed bytes), the current team record (0x205 bytes
/// from +0x4e) appended (`exe::llist_add_tail`) to that list, or to a new one when there was none (0xe bytes of
/// `exe::crt_operator_new` and `exe::new_llist`; 0 when out of memory). Returns the list.
/// Original: `TEAM_AWARD::get_history_list` (0x8b00a0, team_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn team_award_get_history_list(this: usize) -> usize {
    use team_award_obj as ta;
    let mut list = award_history_records(FN_AWARD_MANAGER_GET_TEAM_AWARD_RECORDS, read_u32(this + ta::HISTORY_FIRST), read_u32(this + ta::HISTORY_LAST));
    let t = TeamAward(this);
    if (t.filled() as i32) < t.slot_count() as i32 {
        return list;
    }
    covered!("sx05b_25 current team listed");
    if list == 0 {
        let p = exe::crt_operator_new(0xe);
        list = if p != 0 { exe::new_llist(p) } else { 0 };
    }
    exe::llist_add_tail(list, this + ta::RECORD, ta::RECORD_SIZE);
    list
}

/// The year award's past winners: the history records between its offsets (`AWARD_MANAGER::get_year_award_records`).
/// Original: `YEAR_AWARD::get_history_list` (0x9304d0, year_award.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn year_award_get_history_list(this: usize) -> usize {
    award_history_records(FN_AWARD_MANAGER_GET_YEAR_AWARD_RECORDS, read_u32(this + year_award_obj::HISTORY_FIRST), read_u32(this + year_award_obj::HISTORY_LAST))
}

/// The eleven position codes of the team award drawing by the formation mode (+0x25c): modes 0, 1 and 2 their own,
/// any other the fourth.
const TEAM_AWARD_FORMATIONS: [[u32; 11]; 4] = [
    [1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x810, 0x410, 0x140, 0x440, 0x90],
    [1, 0x804, 0x84, 0x104, 0x404, 0x110, 0x210, 0x410, 0x140, 0x240, 0x440],
    [1, 0x808, 0x88, 0x104, 0x404, 0x202, 0x410, 0x210, 0x140, 0x440, 0x110],
    [1, 0x804, 0x84, 0x202, 0x204, 0x208, 0x90, 0x810, 0x220, 0x140, 0x440],
];

/// Sets the eleven slots of `formation` (`exe::formation_set_slot_position`(slot, code, 0)) to the codes of the award's
/// formation mode (`TEAM_AWARD_FORMATIONS`), for the award screen's team drawing.
/// Original: `TEAM_AWARD::apply_formation` (0x8b0730, team_award.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn team_award_apply_formation(this: usize, formation: usize) {
    let mode = read_u8(this + team_award_obj::FORMATION_MODE);
    let codes = &TEAM_AWARD_FORMATIONS[match mode { 0 => 0, 1 => 1, 2 => 2, _ => 3 }];
    for (slot, &code) in codes.iter().enumerate() {
        exe::formation_set_slot_position(formation, slot as u32, code, 0);
    }
}

/// The contents of a history list (each node's `size` bytes), for the comparers.
unsafe fn award_list_contents(list: usize, size: usize) -> Vec<Vec<u8>> {
    llist_items(list).iter().map(|&d| (0..size).map(|i| read_u8(d + i)).collect()).collect()
}

/// The comparer of a history list: both run (each builds its own list); the records must be the same, and the copy's
/// list is freed.
unsafe fn compare_award_history(original: usize, copy: usize, size: usize, what: &str) -> usize {
    let (a, b) = (award_list_contents(original, size), award_list_contents(copy, size));
    if copy != 0 {
        exe::llist_scalar_deleting_destructor(copy, 1);
    }
    SCOREBOARD.record(a == b, || format!("{} (award screen history): original {} records, rust {}", what, a.len(), b.len()));
    original
}

pub(crate) unsafe extern "thiscall" fn compare_team_award_get_history_list(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%)(this);
    compare_award_history(o, team_award_get_history_list(this), team_award_obj::RECORD_SIZE as usize, "team_award_get_history_list")
}

pub(crate) unsafe extern "thiscall" fn compare_year_award_get_history_list(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%)(this);
    compare_award_history(o, year_award_get_history_list(this), 0x4a, "year_award_get_history_list")
}

/// Comparer for `team_award_apply_formation`: side by side over the whole formation (`formation_fields::SIZE`).
pub(crate) unsafe extern "thiscall" fn compare_team_award_apply_formation(this: usize, formation: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let (_, _, equal, a, b) = crate::comparison::side_by_side(&[(formation, formation_fields::SIZE)], || o(this, formation), || team_award_apply_formation(this, formation));
    SCOREBOARD.record(equal, || format!("team_award_apply_formation(mode {}) (award screen team) {:?}", read_u8(this + team_award_obj::FORMATION_MODE), first_differences(&a, &b)));
}
