// ---------- batch %%LEVA%%: the training screens (1): the squad list sort orders, the edit session's destructor and leaving the screen (training_screens.cpp) ----------

/// "E:\dev\CM3\cm3 00-01\cm3\code\training_screens.cpp", the path the training_screens.cpp error boxes print.
pub(crate) const TRAINING_SCREENS_CPP: usize = 0xa8d1d8;
/// The squad view the training screen's builder points at [0xde1f50] (inside the edit session: +0x112 + 0x7d × the
/// view); its byte +4 is the attribute shown in the sortable attribute column [inference: the builder writes it, the
/// attribute sort reads it].
pub(crate) const TRAINING_SQUAD_VIEW: usize = 0xde1f50;
/// The size of a `TRAINING_SCHEDULE` (operator new 0x9b; the edit session keeps 11 of them from +0xfa).
pub(crate) const TRAINING_SCHEDULE_SIZE: usize = 0x9b;

/// `get_player_status_code` (0x4849f0, club_screens.cpp, cdecl): the player's status code (no club, no fixture).
unsafe fn training_status_code(person: usize) -> u8 {
    let f: unsafe extern "C" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4849f0usize);
    f(person, 0, 0)
}

/// The name tie-break shared by the training squad sorts: `a` and `b` point at the two `Staff` pointers. No first
/// person: 1; no second: −1; otherwise both names formatted (`exe::database_format_staff_name`, style 3, 200 bytes)
/// and compared with `exe::utils_string_compare(first, second, 1, 0)`.
unsafe fn training_compare_names(a: usize, b: usize) -> i32 {
    if read_usize(a) == 0 {
        return 1;
    }
    if read_usize(b) == 0 {
        return -1;
    }
    let (mut first, mut second) = ([0u8; 200], [0u8; 200]);
    exe::database_format_staff_name(read_usize(a), first.as_mut_ptr() as usize, 200, 3);
    exe::database_format_staff_name(read_usize(b), second.as_mut_ptr() as usize, 200, 3);
    exe::utils_string_compare(first.as_ptr(), second.as_ptr(), 1, 0)
}

/// The null checks every training sort starts with: no first entry pointer: Some(1); no second: Some(−1); then the
/// same for the two `Staff` pointers they hold; both present: None.
unsafe fn training_sort_nulls(a: usize, b: usize) -> Option<i32> {
    if a == 0 {
        return Some(1);
    }
    if b == 0 {
        return Some(-1);
    }
    if read_usize(a) == 0 {
        return Some(1);
    }
    if read_usize(b) == 0 {
        return Some(-1);
    }
    None
}

/// Rule: the training squad list sorted by player status, ascending (the status code byte difference), then by name.
///
/// `a` and `b` point at two `Staff` pointers (qsort). The null checks (`training_sort_nulls`), then the byte difference
/// `status(a) − status(b)` of `get_player_status_code(person, 0, 0)` taken as a signed byte; when it is 0, the name
/// (`training_compare_names`).
/// Original: `training_screens_compare_by_status_then_name` (0x8bb190, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_status_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let d = training_status_code(read_usize(a)).wrapping_sub(training_status_code(read_usize(b))) as i8;
    if d != 0 {
        covered!("tel25_1 sort by status");
        return d as i32;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by name only.
///
/// `a` and `b` point at two `Staff` pointers: no first pointer: 1, no second: −1, then the name
/// (`training_compare_names`).
/// Original: `training_screens_compare_by_name` (0x8bb2a0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    covered!("tel25_1 sort by name");
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by the positions text (alphabetically), then by name.
///
/// After the null checks, each player's positions text (`database_format_player_positions` 0x53ee50 of the person's
/// `staff::PLAYER`, 200 bytes, form 4) compared with `exe::utils_string_compare(first, second, 1, 0)`; equal texts go
/// to the name (`training_compare_names`).
/// Original: `training_screens_compare_by_position_then_name` (0x8bb340, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_position_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
    let (mut first, mut second) = ([0u8; 200], [0u8; 200]);
    positions(Staff(read_usize(a)).player().addr(), first.as_mut_ptr() as usize, 200, 4);
    positions(Staff(read_usize(b)).player().addr(), second.as_mut_ptr() as usize, 200, 4);
    let d = exe::utils_string_compare(first.as_ptr(), second.as_ptr(), 1, 0);
    if d != 0 {
        covered!("tel25_1 sort by position");
        return d;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by value, ascending, then by name.
///
/// After the null checks, `value(a) − value(b)` (`staff::VALUE`, 32-bit wrapping); 0 goes to the name.
/// Original: `training_screens_compare_by_value_then_name` (0x8bb460, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_value_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let d = (Staff(read_usize(a)).value() as i32).wrapping_sub(Staff(read_usize(b)).value() as i32);
    if d != 0 {
        covered!("tel25_1 sort by value");
        return d;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by the attribute of the sortable column, highest first, then by name.
///
/// After the null checks, the attribute id is the byte +4 of the squad view [`TRAINING_SQUAD_VIEW`]; the result is the
/// byte difference `attribute(b) − attribute(a)` of `exe::staff_get_player_attribute` taken as a signed byte; 0 goes
/// to the name.
/// Original: `training_screens_compare_by_attribute_then_name` (0x8bb510, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_attribute_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let first = exe::staff_get_player_attribute(read_usize(a), read_u8(read_usize(TRAINING_SQUAD_VIEW) + 4) as u32);
    let second = exe::staff_get_player_attribute(read_usize(b), read_u8(read_usize(TRAINING_SQUAD_VIEW) + 4) as u32);
    let d = (second as u8).wrapping_sub(first as u8) as i8;
    if d != 0 {
        covered!("tel25_1 sort by attribute");
        return d as i32;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by age, oldest first, then by name.
///
/// After the null checks, `age(b) − age(a)` (`staff::AGE` read as a signed byte); 0 goes to the name.
/// Original: `training_screens_compare_by_age_then_name` (0x8bb620, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_age_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let d = read_i8(read_usize(b) + staff::AGE) as i32 - read_i8(read_usize(a) + staff::AGE) as i32;
    if d != 0 {
        covered!("tel25_1 sort by age");
        return d;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by recent form, best first, then by name.
///
/// After the null checks, `form(b) − form(a)` of `exe::playerstats_get_recent_form` (on [`G_PLAYER_STATS`], the
/// person's `staff::PLAYER`, signed bytes); 0 goes to the name.
/// Original: `training_screens_compare_by_form_then_name` (0x8bb6d0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_form_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let first = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), Staff(read_usize(a)).player().addr()) as i32;
    let second = exe::playerstats_get_recent_form(read_usize(G_PLAYER_STATS), Staff(read_usize(b)).player().addr()) as i32;
    let d = second - first;
    if d != 0 {
        covered!("tel25_1 sort by form");
        return d;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by morale, highest first, then by name.
///
/// After the null checks, `morale(b) − morale(a)` of `contract_manager_get_player_morale` (0x4cc2d0, stdcall; its low
/// byte, signed); 0 goes to the name.
/// Original: `training_screens_compare_by_morale_then_name` (0x8bb7d0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_morale_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let morale: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x4cc2d0usize);
    let first = morale(read_usize(a)) as i8 as i32;
    let second = morale(read_usize(b)) as i8 as i32;
    let d = second - first;
    if d != 0 {
        covered!("tel25_1 sort by morale");
        return d;
    }
    training_compare_names(a, b)
}

/// Rule: the training squad list sorted by injury status (ascending), then by condition (highest first), then by name.
///
/// After the null checks, `status(a) − status(b)` of `exe::injury_get_player_injury_status` (on [`G_INJURY`], signed
/// bytes); then `condition(b) − condition(a)` of `exe::injury_get_condition(.., 1)` (signed words, the second player
/// asked first); 0 goes to the name.
/// Original: `training_screens_compare_by_injury_then_name` (0x8bb8d0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_compare_by_injury_then_name(a: *const usize, b: *const usize) -> i32 {
    let (a, b) = (a as usize, b as usize);
    if let Some(r) = training_sort_nulls(a, b) {
        return r;
    }
    let first = exe::injury_get_player_injury_status(read_usize(G_INJURY), read_usize(a)) as i8 as i32;
    let second = exe::injury_get_player_injury_status(read_usize(G_INJURY), read_usize(b)) as i8 as i32;
    let d = first - second;
    if d != 0 {
        covered!("tel25_1 sort by injury status");
        return d;
    }
    let second = exe::injury_get_condition(read_usize(G_INJURY), read_usize(b), 1) as i32;
    let first = exe::injury_get_condition(read_usize(G_INJURY), read_usize(a), 1) as i32;
    let d = second - first;
    if d != 0 {
        covered!("tel25_1 sort by condition");
        return d;
    }
    training_compare_names(a, b)
}

compare_order!(compare_training_screens_compare_by_status_then_name, training_screens_compare_by_status_then_name, %%I0%%);
compare_order!(compare_training_screens_compare_by_name, training_screens_compare_by_name, %%I1%%);
compare_order!(compare_training_screens_compare_by_position_then_name, training_screens_compare_by_position_then_name, %%I2%%);
compare_order!(compare_training_screens_compare_by_value_then_name, training_screens_compare_by_value_then_name, %%I3%%);
compare_order!(compare_training_screens_compare_by_attribute_then_name, training_screens_compare_by_attribute_then_name, %%I4%%);
compare_order!(compare_training_screens_compare_by_age_then_name, training_screens_compare_by_age_then_name, %%I5%%);
compare_order!(compare_training_screens_compare_by_form_then_name, training_screens_compare_by_form_then_name, %%I6%%);
compare_order!(compare_training_screens_compare_by_morale_then_name, training_screens_compare_by_morale_then_name, %%I7%%);
compare_order!(compare_training_screens_compare_by_injury_then_name, training_screens_compare_by_injury_then_name, %%I8%%);

/// Rule: destroying a training edit session empties its list and runs the (empty) destructor of its 11 schedules.
///
/// `this` is the session (0x819 bytes): the LLIST at +0x80a is destroyed (`exe::drop_llist`), then the CRT vector
/// destructor iterator runs over the 11 `TRAINING_SCHEDULE`s at +0xfa (0x9b bytes each) with the empty destructor
/// 0x5a53c0. The memory is not freed here.
/// Original: `TRAINING_EDIT_SESSION::~TRAINING_EDIT_SESSION` (0x8b9740, training_screens.cpp, thiscall)
pub(crate) unsafe extern "thiscall" fn drop_training_edit_session(this: usize) {
    exe::drop_llist(this + 0x80a);
    exe::crt_vector_destructor_iterator(this + 0xfa, TRAINING_SCHEDULE_SIZE as u32, 0xb, 0x5a53c0usize);
}

/// Comparer for `drop_training_edit_session`: only the original runs (it destroys the list).
pub(crate) unsafe extern "thiscall" fn compare_drop_training_edit_session(this: usize) {
    // simplification: destroying the list frees its nodes and cannot run twice; proven in Rust mode
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I9%%)(this)
}

/// Rule: leaving the training screen applies the session's changes to the squad and frees the session.
///
/// The session is screen parameter 0; when there is one: `TRAINING_EDIT_SESSION::apply_changes` (0x8b0b40, thiscall),
/// its destructor (`drop_training_edit_session` through 0x8b9740), `exe::crt_operator_delete`, and parameter 0 cleared.
/// Original: `training_screens_close` (0x8b97a0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_close() {
    let session = get_screen_param(0);
    if session == 0 {
        return;
    }
    covered!("tel25_1 close the training screen");
    let apply: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b0b40usize);
    apply(session);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b9740usize);
    destroy(session);
    exe::crt_operator_delete(session);
    set_screen_param(0, 0);
}

/// Comparer for `training_screens_close`: only the original runs (it writes the squad's training and frees the
/// session), and parameter 0 is checked afterwards (cleared when there was a session, unchanged otherwise).
pub(crate) unsafe extern "C" fn compare_training_screens_close() {
    let before = get_screen_param(0);
    original_fn::<unsafe extern "C" fn()>(%%I10%%)();
    let after = get_screen_param(0);
    SCOREBOARD.record(after == 0, || format!("training screen (close): param 0 {:#x} -> {:#x}, expected 0", before, after));
}
