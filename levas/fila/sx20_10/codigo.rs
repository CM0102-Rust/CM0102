// --- sx20_10: the training edit session of the training screens (training_edit_session.cpp, 0x8b0a00..0x8b0e70) ---

/// The layout of a training edit session (`TRAINING_EDIT_SESSION_SIZE` bytes): +0 the 50 squad entries of 5 bytes
/// (+0 the player, +4 his schedule slot), the 11 schedules of 0x9b bytes (the tenth is a spare), the 54-byte coach
/// assignment table, the selected schedule, a changed flag per schedule, the session changed flag, the club, the
/// player list (LLIST) and how many it lists.
const TRAINING_SESSION_ENTRY_SIZE: usize = 5;
const TRAINING_SESSION_ENTRIES: usize = 50;
const TRAINING_SESSION_SCHEDULES: usize = 0xfa;
const TRAINING_SESSION_COACHES: usize = 0x7a3;
const TRAINING_COACHES_SIZE: usize = 0x36;
const TRAINING_SESSION_SELECTED: usize = 0x7d9;
const TRAINING_SESSION_SCHEDULE_CHANGED: usize = 0x7da;
const TRAINING_SESSION_CHANGED: usize = 0x802;
const TRAINING_SESSION_CLUB: usize = 0x806;
const TRAINING_SESSION_LIST: usize = 0x80a;
const TRAINING_SESSION_LISTED: usize = 0x818;
const TRAINING_EDIT_SESSION_SIZE: usize = 0x819;
/// training_edit_session.cpp, the source path of the error boxes.
const TRAINING_EDIT_SESSION_SOURCE: usize = 0xa8cc88;
const FN_TRAINING_SCHEDULE_COUNT_PLAYERS: usize = 0x8b5000;
const FN_TRAINING_MANAGER_COLLECT_SQUAD_SCHEDULES: usize = 0x8b3410;
const FN_TRAINING_MANAGER_GET_COACH_ASSIGNMENTS_COPY: usize = 0x8b35a0;
const FN_TRAINING_MANAGER_SET_PLAYER_SCHEDULE: usize = 0x8b2250;
const FN_TRAINING_MANAGER_SET_COACH_ASSIGNMENTS: usize = 0x8b23b0;
const FN_TRAINING_SCHEDULE_LOAD_FROM_FILE: usize = 0x8b5220;
const FN_TRAINING_SCHEDULE_SAVE_TO_FILE: usize = 0x8b53d0;
const FN_HUMAN_MANAGER_ADD_RECENT_FILE: usize = 0x5fcb60;

/// The address of schedule `k` of the session.
fn training_session_schedule(this: usize, k: i32) -> usize {
    (this + TRAINING_SESSION_SCHEDULES).wrapping_add((k as usize).wrapping_mul(TRAINING_SCHEDULE_SIZE))
}

/// The selected schedule of the session.
unsafe fn training_session_selected(this: usize) -> usize {
    training_session_schedule(this, read_i8(this + TRAINING_SESSION_SELECTED) as i32)
}

/// The club's manager, whose schedules and coach table the session edits.
unsafe fn training_session_manager(this: usize) -> usize {
    Club(read_usize(this + TRAINING_SESSION_CLUB)).manager().addr()
}

/// `TRAINING_SCHEDULE::count_players(list or 0, the session's squad entries, order or 0)`: how many players the
/// schedule applies to, listing them when given a list (after sorting the entries when given an order).
unsafe fn training_schedule_count_players(schedule: usize, list: usize, entries: usize, order: usize) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u8 = core::mem::transmute(FN_TRAINING_SCHEDULE_COUNT_PLAYERS);
    f(schedule, list, entries, order)
}

/// `HUMAN_MANAGER::add_recent_file(path, 0 (the current network player), kind 3)`: a training schedule file.
unsafe fn training_session_recent_file(path: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_ADD_RECENT_FILE);
    f(read_usize(G_HUMAN_MANAGER), path, 0, 3);
}

/// Builds an edit session for a club: its 11 schedules (`TRAINING_SCHEDULE::TRAINING_SCHEDULE`), an empty player
/// list, the squad entries with each player's schedule slot (`collect_squad_schedules`), copies of the manager's ten
/// schedules (`get_manager_schedule`) with their changed flags cleared, his coach assignment table
/// (`get_coach_assignments_copy`), unchanged and with schedule 0 selected. Returns the session.
/// Original: `TRAINING_EDIT_SESSION::TRAINING_EDIT_SESSION` (0x8b0a00, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_training_edit_session(this: usize, club: Club) -> usize {
    exe::crt_vector_constructor_iterator(this + TRAINING_SESSION_SCHEDULES, TRAINING_SCHEDULE_SIZE as u32, 11, FN_TRAINING_SCHEDULE_TRAINING_SCHEDULE, 0x5a53c0usize);
    exe::new_llist(this + TRAINING_SESSION_LIST);
    write_usize(this + TRAINING_SESSION_CLUB, club.addr());
    let collect: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_COLLECT_SQUAD_SCHEDULES);
    collect(read_usize(G_TRAINING_MANAGER), club.addr(), this);
    for k in 0..10i8 {
        // simplification: the original pushes the slot in a dword whose high bytes are stack leftovers (read as a char)
        let schedule = exe::training_manager_get_manager_schedule(read_usize(G_TRAINING_MANAGER), training_session_manager(this), k);
        core::ptr::copy_nonoverlapping(schedule as *const u8, training_session_schedule(this, k as i32) as *mut u8, TRAINING_SCHEDULE_SIZE);
        write_u32(this + TRAINING_SESSION_SCHEDULE_CHANGED + 4 * k as usize, 0);
    }
    let coaches: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_GET_COACH_ASSIGNMENTS_COPY);
    coaches(read_usize(G_TRAINING_MANAGER), training_session_manager(this), this + TRAINING_SESSION_COACHES);
    write_u32(this + TRAINING_SESSION_CHANGED, 0);
    write_i8(this + TRAINING_SESSION_SELECTED, 0); // sx20_10: schedule 0 selected
    this
}

/// Copies the session back to the training manager: the ten schedules over the manager's, each player of the squad
/// entries to his schedule slot (`set_player_schedule`) and the coach table (`set_coach_assignments`). Returns 1.
/// Original: `TRAINING_EDIT_SESSION::apply_changes` (0x8b0b40, training_edit_session.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn training_edit_session_apply_changes(this: usize) -> u32 {
    for k in 0..10i8 {
        let schedule = exe::training_manager_get_manager_schedule(read_usize(G_TRAINING_MANAGER), training_session_manager(this), k);
        core::ptr::copy_nonoverlapping(training_session_schedule(this, k as i32) as *const u8, schedule as *mut u8, TRAINING_SCHEDULE_SIZE);
    }
    let set_player: unsafe extern "thiscall" fn(usize, usize, u8) = core::mem::transmute(FN_TRAINING_MANAGER_SET_PLAYER_SCHEDULE);
    for k in 0..TRAINING_SESSION_ENTRIES {
        let entry = this + k * TRAINING_SESSION_ENTRY_SIZE;
        if read_usize(entry) != 0 {
            // simplification: the original pushes the slot in ecx with the manager's leftover high bytes (read as a char)
            set_player(read_usize(G_TRAINING_MANAGER), read_usize(entry), read_u8(entry + 4)); // sx20_10: each player's slot
        }
    }
    let set_coaches: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_TRAINING_MANAGER_SET_COACH_ASSIGNMENTS);
    set_coaches(read_usize(G_TRAINING_MANAGER), training_session_manager(this), this + TRAINING_SESSION_COACHES);
    1
}

/// Selects schedule `index` (0 to 9) for editing and returns 1; any other index shows the error box (line 0x67) and
/// returns 0.
/// Original: `TRAINING_EDIT_SESSION::select_schedule` (0x8b0bf0, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_select_schedule(this: usize, index: i8) -> u32 {
    if !(0..=9).contains(&index) { // sx20_10: the schedules 0 to 9
        show_error_at(TRAINING_EDIT_SESSION_SOURCE, 0x67);
        return 0;
    }
    write_i8(this + TRAINING_SESSION_SELECTED, index);
    1
}

/// Copies `schedule` (0x9b bytes) into the selected slot and marks that slot and the session as changed.
/// Original: `TRAINING_EDIT_SESSION::set_current_schedule` (0x8b0c90, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_set_current_schedule(this: usize, schedule: usize) {
    core::ptr::copy_nonoverlapping(schedule as *const u8, training_session_selected(this) as *mut u8, TRAINING_SCHEDULE_SIZE);
    let selected = read_i8(this + TRAINING_SESSION_SELECTED) as isize;
    write_u32((this + TRAINING_SESSION_SCHEDULE_CHANGED).wrapping_add((4 * selected) as usize), 1);
    write_u32(this + TRAINING_SESSION_CHANGED, 1);
}

/// Copies an edited 54-byte coach assignment table into the session and marks the session as changed.
/// Original: `TRAINING_EDIT_SESSION::set_coach_record` (0x8b0ce0, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_set_coach_record(this: usize, coaches: usize) {
    core::ptr::copy_nonoverlapping(coaches as *const u8, (this + TRAINING_SESSION_COACHES) as *mut u8, TRAINING_COACHES_SIZE);
    write_u32(this + TRAINING_SESSION_CHANGED, 1);
}

/// How many session players schedule `index` applies to (`TRAINING_SCHEDULE::count_players` without a list); -1 is
/// the selected schedule.
/// Original: `TRAINING_EDIT_SESSION::count_schedule_players` (0x8b0d10, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_count_schedule_players(this: usize, index: i8) -> u8 {
    let schedule = if index == -1 { training_session_selected(this) } else { training_session_schedule(this, index as i32) };
    training_schedule_count_players(schedule, 0, this, 0)
}

/// Lists the players of the selected schedule: with `out`, `*out` is the session's player list, filled (after
/// sorting the squad entries by `order`, when given); without, only counted. The count goes to the session (+0x818)
/// and is returned.
/// Original: `TRAINING_EDIT_SESSION::list_schedule_players` (0x8b0d60, training_edit_session.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn training_edit_session_list_schedule_players(this: usize, out: usize, order: usize) -> u8 {
    let n = if out != 0 {
        write_usize(out, this + TRAINING_SESSION_LIST);
        training_schedule_count_players(training_session_selected(this), this + TRAINING_SESSION_LIST, this, order)
    } else {
        training_schedule_count_players(training_session_selected(this), 0, this, 0)
    };
    write_u8(this + TRAINING_SESSION_LISTED, n);
    n
}

/// Moves `player` to schedule slot `slot` in the session (the first of the 50 entries holding him) and marks the
/// session as changed; a player not in the session changes nothing.
/// Original: `TRAINING_EDIT_SESSION::set_player_group` (0x8b0dc0, training_edit_session.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn training_edit_session_set_player_group(this: usize, player: usize, slot: u8) {
    for k in 0..TRAINING_SESSION_ENTRIES {
        let entry = this + k * TRAINING_SESSION_ENTRY_SIZE;
        if read_usize(entry) == player {
            write_u8(entry + 4, slot); // sx20_10: the new group
            write_u32(this + TRAINING_SESSION_CHANGED, 1);
            return;
        }
    }
}

/// Loads a schedule file (`TRAINING_SCHEDULE::load_from_file`) into the selected slot; on success marks the slot and
/// the session as changed, puts the file at the top of the manager's recent training files and returns 1, else 0.
/// Original: `TRAINING_EDIT_SESSION::load_schedule_file` (0x8b0e00, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_load_schedule_file(this: usize, path: usize) -> u32 {
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRAINING_SCHEDULE_LOAD_FROM_FILE);
    let r = load(training_session_selected(this), path);
    if r == 0 {
        return r;
    }
    let selected = read_i8(this + TRAINING_SESSION_SELECTED) as isize;
    write_u32((this + TRAINING_SESSION_SCHEDULE_CHANGED).wrapping_add((4 * selected) as usize), 1);
    write_u32(this + TRAINING_SESSION_CHANGED, 1);
    training_session_recent_file(path);
    1
}

/// Saves the selected schedule to a file (`TRAINING_SCHEDULE::save_to_file`); on success puts the file at the top of
/// the manager's recent training files and returns 1, else 0.
/// Original: `TRAINING_EDIT_SESSION::save_schedule_file` (0x8b0e70, training_edit_session.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn training_edit_session_save_schedule_file(this: usize, path: usize) -> u32 {
    let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_TRAINING_SCHEDULE_SAVE_TO_FILE);
    let r = save(training_session_selected(this), path);
    if r == 0 {
        return r;
    }
    training_session_recent_file(path);
    1
}

/// Runs a session routine side by side over the whole session: the session and the answer must match.
unsafe fn training_session_compare<R: PartialEq + core::fmt::Debug>(name: &str, this: usize, original: impl FnOnce() -> R, copy: impl FnOnce() -> R) -> R {
    let (r, c, same, mo, mr) = crate::comparison::side_by_side(&[(this, TRAINING_EDIT_SESSION_SIZE)], original, copy);
    SCOREBOARD.record(same && r == c, || format!("{} (training edit session {:#x}): original={:?} rust={:?} {:?}", name, this, r, c, first_differences(&mo, &mr)));
    r
}

/// Comparer for `training_edit_session_training_edit_session`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_training_edit_session(this: usize, club: Club) -> usize {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, Club) -> usize>(%%I0%%);
    training_session_compare("training_edit_session_training_edit_session", this, || original(this, club),
        || training_edit_session_training_edit_session(this, club))
}

/// Comparer for `training_edit_session_apply_changes`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_apply_changes(this: usize) -> u32 {
    // simplification: writes into the training manager (moves players, refreshes their training): only the original runs
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%)(this)
}

/// Comparer for `training_edit_session_select_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_select_schedule(this: usize, index: i8) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I2%%);
    if !(0..=9).contains(&index) {
        return original(this, index); // the error box: once
    }
    training_session_compare("training_edit_session_select_schedule", this, || original(this, index),
        || training_edit_session_select_schedule(this, index))
}

/// Comparer for `training_edit_session_set_current_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_set_current_schedule(this: usize, schedule: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I3%%);
    training_session_compare("training_edit_session_set_current_schedule", this, || original(this, schedule),
        || training_edit_session_set_current_schedule(this, schedule))
}

/// Comparer for `training_edit_session_set_coach_record`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_set_coach_record(this: usize, coaches: usize) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I4%%);
    training_session_compare("training_edit_session_set_coach_record", this, || original(this, coaches),
        || training_edit_session_set_coach_record(this, coaches))
}

/// Comparer for `training_edit_session_count_schedule_players`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_count_schedule_players(this: usize, index: i8) -> u8 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u8>(%%I5%%);
    training_session_compare("training_edit_session_count_schedule_players", this, || original(this, index),
        || training_edit_session_count_schedule_players(this, index))
}

/// Comparer for `training_edit_session_list_schedule_players`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_list_schedule_players(this: usize, out: usize, order: usize) -> u8 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I6%%);
    if out != 0 {
        // simplification: fills the session's list (allocated nodes) and may sort the entries: only the original runs
        return original(this, out, order);
    }
    training_session_compare("training_edit_session_list_schedule_players", this, || original(this, out, order),
        || training_edit_session_list_schedule_players(this, out, order))
}

/// Comparer for `training_edit_session_set_player_group`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_set_player_group(this: usize, player: usize, slot: u8) {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8)>(%%I7%%);
    training_session_compare("training_edit_session_set_player_group", this, || original(this, player, slot),
        || training_edit_session_set_player_group(this, player, slot))
}

/// Comparer for `training_edit_session_load_schedule_file`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_load_schedule_file(this: usize, path: usize) -> u32 {
    // simplification: reads a file and changes the recent file list (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I8%%)(this, path)
}

/// Comparer for `training_edit_session_save_schedule_file`.
pub(crate) unsafe extern "thiscall" fn compare_training_edit_session_save_schedule_file(this: usize, path: usize) -> u32 {
    // simplification: writes a file and changes the recent file list (cannot run twice)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I9%%)(this, path)
}
