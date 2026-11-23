use super::*;

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
    let (mut first, mut second) = ([0u8; 200], [0u8; 200]);
    exe::database_format_player_positions(Staff(read_usize(a)).player().addr(), first.as_mut_ptr() as usize, 200, 4);
    exe::database_format_player_positions(Staff(read_usize(b)).player().addr(), second.as_mut_ptr() as usize, 200, 4);
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
    let morale: unsafe extern "stdcall" fn(usize) -> i8 = core::mem::transmute(FN_CONTRACT_MANAGER_GET_PLAYER_MORALE);
    let first = morale(read_usize(a)) as i32;
    let second = morale(read_usize(b)) as i32;
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DROP_TRAINING_EDIT_SESSION)(this)
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
    exe::training_edit_session_apply_changes(session);
    exe::drop_training_edit_session(session);
    exe::crt_operator_delete(session);
    set_screen_param(0, 0);
}

/// Comparer for `training_screens_close`: only the original runs (it writes the squad's training and frees the
/// session), and parameter 0 is checked afterwards (cleared when there was a session, unchanged otherwise).
pub(crate) unsafe extern "C" fn compare_training_screens_close() {
    let before = get_screen_param(0);
    original_fn::<unsafe extern "C" fn()>(IDX_TRAINING_SCREENS_CLOSE)();
    let after = get_screen_param(0);
    SCOREBOARD.record(after == 0, || format!("training screen (close): param 0 {:#x} -> {:#x}, expected 0", before, after));
}

/// Rule: opening the training screen of a club: a new edit session of its squad's training.
///
/// The screen opens with `open_screen` (builder `training_screens_draw` 0x8b5a10, input
/// `training_screens_handle_event` 0x8b83f0, flag 1, no leave, exit `training_screens_close` 0x8b97a0); if it does not
/// open nothing else happens. A session (operator new 0x819; `TRAINING_EDIT_SESSION::TRAINING_EDIT_SESSION(club)`
/// 0x8b0a00, or 0 when the allocation fails) becomes parameter 0; parameters 1, 2 and 3 are 0, 4 is 1 and 5 is −1.
/// Original: `training_screens_open` (0x8b5930, training_screens.cpp, cdecl, SEH)
pub(crate) unsafe extern "C" fn training_screens_open(club: usize) {
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8b5a10usize, 0x8b83f0usize, 1, 0usize, 0x8b97a0) == 0 {
        return;
    }
    covered!("tel25_2 open the training screen");
    let block = exe::crt_operator_new(0x819);
    let session = if block != 0 {
        exe::new_training_edit_session(block, club)
    } else {
        0
    };
    set_screen_param(0, session);
    set_screen_param(1, 0);
    set_screen_param(2, 0);
    set_screen_param(3, 0);
    set_screen_param(4, 1);
    set_screen_param(5, usize::MAX);
}

/// Comparer for `training_screens_open`: only the original runs (it opens a screen and allocates the session), and the
/// parameters are checked against the plan afterwards (a session in 0; 0, 0, 0, 1, −1 in 1..5) when the screen opened.
pub(crate) unsafe extern "C" fn compare_training_screens_open(club: usize) {
    let before = get_screen_param(0);
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRAINING_SCREENS_OPEN)(club);
    let got = [get_screen_param(0), get_screen_param(1), get_screen_param(2), get_screen_param(3), get_screen_param(4), get_screen_param(5)];
    if got[0] == before {
        return; // the screen did not open
    }
    let ok = got[0] != 0 && got[1..] == [0, 0, 0, 1, usize::MAX];
    SCOREBOARD.record(ok, || format!("training screen (open {:#x}): params {:x?}, expected a session and 0, 0, 0, 1, -1", club, got));
}

/// Rule: opening the schedule editor on the session's current schedule: the editor works on a copy of it.
///
/// No session: the training_screens.cpp 1756 error box and nothing else. Otherwise a new `TRAINING_SCHEDULE`
/// (operator new 0x9b, `TRAINING_SCHEDULE::TRAINING_SCHEDULE` 0x8b4e10; a failed allocation leaves it 0) gets the 0x9b
/// bytes of the session's schedule number `session` +0x7d9 (a signed byte; the schedules from +0xfa, 0x9b each). The
/// screen opens with `open_screen` (builder `training_screens_draw_schedule_editor` 0x8b9940, input
/// `training_screens_schedule_editor_handle_event` 0x8ba530, flag 1, no leave, no exit); when it opens, parameter 0 is
/// the session, 1 the copy and 2 the copy's name (+9, written with the copy flag: the screen keeps its own text).
/// Original: `training_screens_open_schedule_editor` (0x8b97e0, training_screens.cpp, cdecl, SEH)
pub(crate) unsafe extern "C" fn training_screens_open_schedule_editor(session: usize) {
    if session == 0 {
        tel26_error_box(TRAINING_SCREENS_CPP, 0x6dc);
        return;
    }
    let block = exe::crt_operator_new(TRAINING_SCHEDULE_SIZE as u32);
    let schedule = if block != 0 {
        exe::new_training_schedule(block)
    } else {
        0
    };
    let source = session + 0xfa + read_i8(session + 0x7d9) as isize as usize * TRAINING_SCHEDULE_SIZE;
    core::ptr::copy(source as *const u8, schedule as *mut u8, TRAINING_SCHEDULE_SIZE);
    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8b9940usize, 0x8ba530usize, 1, 0usize, 0) == 0 {
        return;
    }
    covered!("tel25_2 open the schedule editor");
    set_screen_param(0, session);
    set_screen_param(1, schedule);
    exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 2, schedule + 9, 1);
}

/// Comparer for `training_screens_open_schedule_editor`: only the original runs (it opens a screen and allocates), and
/// afterwards parameter 0 must be the session and parameter 1 a copy of the session's current schedule.
pub(crate) unsafe extern "C" fn compare_training_screens_open_schedule_editor(session: usize) {
    let before = get_screen_param(1);
    original_fn::<unsafe extern "C" fn(usize)>(IDX_TRAINING_SCREENS_OPEN_SCHEDULE_EDITOR)(session);
    let schedule = get_screen_param(1);
    if session == 0 || schedule == before || schedule == 0 || get_screen_param(0) != session {
        return; // no session (the error box) or the screen did not open
    }
    let source = session + 0xfa + read_i8(session + 0x7d9) as isize as usize * TRAINING_SCHEDULE_SIZE;
    let same = (0..TRAINING_SCHEDULE_SIZE).all(|k| read_u8(source + k) == read_u8(schedule + k));
    SCOREBOARD.record(same, || format!("schedule editor (open, session {:#x}): the copy {:#x} differs from the schedule {:#x}", session, schedule, source));
}

/// Rule: the schedule editor's input: the level buttons set a category's level (3: 0, 4: 10, 5: 25, 6: 50 for the
/// category in the widget), 7 and 8 the position and side training, the name box marks the schedule as renamed, 2
/// cancels and the Ok action keeps the edited schedule in the session.
///
/// Parameter 0 is the session, 1 the schedule being edited. The action of the widget (its id, or the injected command):
/// 3, 4 and 5: `TRAINING_SCHEDULE::set_category_level` (0x8b4f30, thiscall(category, level)) with the widget's value (0
/// for widget −1) and level 0, 10 or 0x19; 9: when the name in parameter 2 differs from the schedule's name (+9), the
/// byte +0x15 becomes 8 and the dword +0x93 0 [inference: renamed, so no longer a preset]; all four return −11. Then,
/// asked through `exe::display_get_widget_id`: 6: level 0x32 for the category `exe::display_get_widget_data`; 7:
/// `set_position_training` (0x8b5090) and 8: `set_side_training` (0x8b50c0) with the widget's data, −11; 2: the
/// schedule freed (its destructor is empty), −9; otherwise, when `ScreenManager::get_widget_action_id` (0x468b40) is
/// 1: `TRAINING_EDIT_SESSION::set_current_schedule` (0x8b0c90) with the schedule, then the schedule's deleting
/// destructor (0x8ba920, flags 1), −9; anything else goes to `exe::menubar_handle_object_link`.
/// Original: `training_screens_schedule_editor_handle_event` (0x8ba530, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_schedule_editor_handle_event(widget: u32) -> i32 {
    let session = get_screen_param(0);
    let schedule = get_screen_param(1);
    let w = widget as i16;
    match tel26_item_id(w) {
        3 => {
            exe::training_schedule_set_category_level(schedule, tel26_item_value(w), 0);
            return -11;
        }
        4 => {
            exe::training_schedule_set_category_level(schedule, tel26_item_value(w), 10);
            return -11;
        }
        9 => {
            let (mut a, mut b) = (get_screen_param(2), schedule + 9);
            let differ = loop {
                let (x, y) = (read_u8(a), read_u8(b));
                if x != y {
                    break true;
                }
                if x == 0 {
                    break false;
                }
                a += 1;
                b += 1;
            };
            if differ {
                covered!("tel25_2 schedule renamed");
                write_u8(schedule + 0x15, 8);
                write_u32(schedule + 0x93, 0);
            }
            return -11;
        }
        5 => {
            exe::training_schedule_set_category_level(schedule, tel26_item_value(w), 0x19);
            return -11;
        }
        _ => {}
    }
    match training_widget_action(widget) {
        6 => {
            exe::training_schedule_set_category_level(schedule, exe::display_get_widget_data(current_human_display(), widget) as u32, 0x32);
            -11
        }
        7 => {
            exe::training_schedule_set_position_training(schedule, exe::display_get_widget_data(current_human_display(), widget));
            -11
        }
        8 => {
            exe::training_schedule_set_side_training(schedule, exe::display_get_widget_data(current_human_display(), widget));
            -11
        }
        2 => {
            if schedule != 0 {
                exe::crt_operator_delete(schedule);
            }
            -9
        }
        _ => {
            if exe::screenmanager_get_widget_action_id(G_SCREEN_MANAGER, widget) == 1 {
                covered!("tel25_2 keep the edited schedule");
                exe::training_edit_session_set_current_schedule(session, schedule);
                if schedule != 0 {
                    exe::training_schedule_scalar_deleting_destructor(schedule, 1);
                }
                return -9;
            }
            exe::menubar_handle_object_link(widget)
        }
    }
}

/// Comparer for `training_screens_schedule_editor_handle_event`: the actions that only change the schedule (3 to 9)
/// side by side over its 0x9b bytes and the return; cancel, Ok and the menu bar (they free the schedule, write the
/// session or open screens) run the original only.
pub(crate) unsafe extern "C" fn compare_training_screens_schedule_editor_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRAINING_SCREENS_SCHEDULE_EDITOR_HANDLE_EVENT);
    let schedule = get_screen_param(1);
    let w = widget as i16;
    let id = tel26_item_id(w);
    let edits = matches!(id, 3 | 4 | 5 | 9) || matches!(training_widget_action(widget), 6 | 7 | 8);
    if schedule == 0 || !edits || (id == 9 && get_screen_param(2) == 0) {
        // simplification: freeing the schedule, keeping it in the session and the menu bar do not run twice
        return o(widget);
    }
    let regions = [(schedule, TRAINING_SCHEDULE_SIZE)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(widget), || training_screens_schedule_editor_handle_event(widget));
    SCOREBOARD.record(equal && ro == rr, || format!("schedule editor (input, widget {} action {}): return {} x {}, differences in {:?}", w, id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the coaches panel: who coaches each training category. Coaches no longer at the club leave the list first;
/// each row has the coach's name and the five categories, lit (yellow) for those he takes.
///
/// Parameter 1 is the coaches being edited (six entries, `TRAINING_COACH_ENTRY`), 2 the club. The green of the
/// categories is made once (`TRAINING_COACH_GREEN`). Each entry with a coach (id ≠ −1) is kept when the person has a
/// non-player record (`staff::NON_PLAYER`) and is the club's assistant manager (`club::ASSISTANT_MANAGER`) or one of
/// its five coaches (`club::COACHES`, the person's id at +0 of each); otherwise it becomes empty (id −1, the five bytes
/// and the last byte 0). Then the background picture (`exe::background_set_picture(0)`), a full-screen area (its right
/// and bottom edges read from the exe, patched in the 1080 exe), the dark green frame (0x78, 0x8c)–(0x301, 0x1cb)
/// (`graphics_rgb_to_pixel(0, 0x38, 0, 0)`), the title "Coaches", the Cancel (1) and Ok (2) buttons in a two-column
/// area (weights 3, 1), the pitch (`draw_pitch` (0x8c, 0xd7)–(0x2ed, 0x18f), its four outputs unused) and a 6 × 6
/// area (weights 10, 6, 6, 6, 6, 6): row k has in column 0 the coach's name (a space then the name, a button with id
/// 0x7d7 and the person), and in columns 1 to 5 the category names (`training_schedule_get_category_name` 0x8b5520):
/// without a coach, grey text; otherwise buttons with id 3 + k and the category as value, yellow on a 0x221 style when
/// the coach takes the category, green otherwise. A seventh row would show the training_screens.cpp 2378 box (it
/// cannot happen).
/// Original: `training_screens_draw_coaches_panel` (0x8ba940, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_draw_coaches_panel() {
    let made = read_u8(TRAINING_COACH_GREEN_MADE);
    if made & 1 == 0 {
        write_u8(TRAINING_COACH_GREEN_MADE, made | 1);
        write_u16(TRAINING_COACH_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let buttons_weights: [u8; 2] = [3, 1];
    let grid_weights: [u8; 6] = [0xa, 6, 6, 6, 6, 6];
    let coaches = get_screen_param(1);
    let club = Club(get_screen_param(2));
    for k in 0..6 {
        let entry = coaches + k * TRAINING_COACH_ENTRY;
        let id = read_i32(entry);
        if id == -1 {
            continue;
        }
        let at_club = staff_at(id as isize).non_player().addr() != 0 && (-1..5).any(|slot: i32| {
            let person = if slot == -1 { club.assistant_manager() } else { club.coaches(slot as usize) };
            person != 0 && read_i32(person) == id
        });
        if !at_club {
            covered!("tel25_2 coach left the club");
            write_i32(entry, -1);
            write_u32(entry + 4, 0);
            write_u8(entry + 8, 0);
        }
    }
    exe::background_set_picture(0usize);
    tel26_add_area(current_human_display(), [0, 0, TRAINING_COACHES_RIGHT_EDGE.read_unaligned(), TRAINING_COACHES_BOTTOM_EDGE.read_unaligned(), 1, 0, 1, 0, 2, 0, u32::MAX]);
    let frame = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0x78, 0x8c, 0x301, 0x1cb, 1, 0, 1, 0, 0x30, frame, u32::MAX]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d104);
    comp_text_item([0x82, 0x96, 0x2f7, 0xc3, 0x30, read_u16(G_GUI_COLOUR_BLACK) as u32, 0xc, 4, read_u16(G_GUI_COLOUR_PALE_GREY) as u32,
        TRAINING_TEXT_BUFFER as u32, 1]);
    let buttons = tel26_add_area(current_human_display(), [0x82, 0x1a3, 0x2f7, 0x1c1, 2, buttons_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98a1a0);
    tel26_area_button([buttons, 0, 0, 0x30, read_u16(G_GUI_COLOUR_GREY) as u32, 0xc, 3, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, TRAINING_TEXT_BUFFER as u32, 1, 0]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x990f7c);
    tel26_area_button([buttons, 1, 0, 0x30, read_u16(G_GUI_COLOUR_GREY) as u32, 0xc, 3, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, TRAINING_TEXT_BUFFER as u32, 2, 0]);
    let (mut x1, mut y1, mut x2, mut y2) = (0i32, 0i32, 0i32, 0i32);
    type DrawPitch = unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, u32);
    let draw_pitch: DrawPitch = core::mem::transmute(FN_DRAW_PITCH);
    draw_pitch(0x8c, 0xd7, 0x2ed, 0x18f, &mut x1, &mut y1, &mut x2, &mut y2, 1, buttons, 0x4000_0000);
    let grid = tel26_add_area(current_human_display(), [0x91, 0xdc, 0x2e8, 0x18a, 6, grid_weights.as_ptr() as u32, 6, 0, 1, 0, u32::MAX]);
    for k in 0..6u32 {
        let entry = coaches + k as usize * TRAINING_COACH_ENTRY;
        let link = 3 + k;
        if read_i32(entry) != -1 {
            write_u16(TRAINING_TEXT_BUFFER, 0x0020); // " "
            let person = staff_at(read_i32(entry) as isize).addr();
            exe::database_format_staff_name(person, TRAINING_TEXT_BUFFER + 1, 0x7d0, 1);
            tel26_area_button([grid, 0, k, 1, 0, 1, 2, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32, 0x7d7, person as u32]);
        }
        for category in 0..5u32 {
            exe::training_schedule_get_category_name(category, TRAINING_TEXT_BUFFER, 0x7d0);
            if read_i32(entry) == -1 {
                tel26_area_text([grid, category + 1, k, 1, 0, 0xc, 2, read_u16(G_GUI_COLOUR_GREY) as u32, TRAINING_TEXT_BUFFER as u32]);
            } else if read_u8(entry + 4 + category as usize) != 0 {
                let yellow = read_u16(G_GUI_COLOUR_YELLOW) as u32;
                tel26_area_button([grid, category + 1, k, 0x221, yellow, 0xc, 2, yellow, TRAINING_TEXT_BUFFER as u32, link, category]);
            } else {
                tel26_area_button([grid, category + 1, k, 1, 0, 0xc, 2, read_u16(TRAINING_COACH_GREEN) as u32, TRAINING_TEXT_BUFFER as u32, link, category]);
            }
        }
    }
}

/// Comparer for `training_screens_draw_coaches_panel`: side by side over the display list
/// (`compare_match_screen_build`) plus the coaches being edited (the entries it empties) and the green colour.
pub(crate) unsafe extern "C" fn compare_training_screens_draw_coaches_panel() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRAINING_SCREENS_DRAW_COACHES_PANEL);
    let coaches = get_screen_param(1);
    if coaches == 0 || get_screen_param(2) == 0 {
        return o();
    }
    compare_match_screen_build("training coaches panel", &[(coaches, 6 * TRAINING_COACH_ENTRY), (TRAINING_COACH_GREEN_MADE, 4)], || o(),
        || training_screens_draw_coaches_panel());
}

/// Rule: the coaches panel's input: a category button toggles that coach's category, Cancel throws the edit away and
/// Ok keeps it in the session.
///
/// Parameter 0 is the session, 1 the coaches being edited. Actions 3 to 8 (coach rows 0 to 5): the byte of the
/// category in the widget's value (its low byte, signed) of that row (+4) flips between 0 and 1, −11. Action 1: the
/// edit is freed (crt free), −9. Then, asked through `exe::display_get_widget_id`: 2:
/// `TRAINING_EDIT_SESSION::set_coach_record` (0x8b0ce0) with the edit, then freed, −9; anything else goes to
/// `exe::menubar_handle_object_link`.
/// Original: `training_screens_coaches_panel_handle_event` (0x8baea0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_coaches_panel_handle_event(widget: u32) -> i32 {
    let session = get_screen_param(0);
    let coaches = get_screen_param(1);
    let w = widget as i16;
    let id = tel26_item_id(w);
    if (3..=8).contains(&id) {
        covered!("tel25_2 toggle a coach's category");
        let row = (id as u8).wrapping_sub(3) as i8 as isize;
        let category = tel26_item_value(w) as u8 as i8 as isize;
        let flag = (coaches as isize + row * TRAINING_COACH_ENTRY as isize + category + 4) as usize;
        write_u8(flag, (read_u8(flag) == 0) as u8);
        return -11;
    }
    if id == 1 {
        exe::crt_free(coaches);
        return -9;
    }
    if training_widget_action(widget) == 2 {
        covered!("tel25_2 keep the coaches");
        exe::training_edit_session_set_coach_record(session, coaches);
        exe::crt_free(coaches);
        return -9;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `training_screens_coaches_panel_handle_event`: the category toggles side by side over the six entries
/// and the return; Cancel, Ok and the menu bar (they free the edit, write the session or open screens) run the original
/// only.
pub(crate) unsafe extern "C" fn compare_training_screens_coaches_panel_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRAINING_SCREENS_COACHES_PANEL_HANDLE_EVENT);
    let coaches = get_screen_param(1);
    let id = tel26_item_id(widget as i16);
    if coaches == 0 || !(3..=8).contains(&id) {
        // simplification: freeing the edit, keeping it in the session and the menu bar do not run twice
        return o(widget);
    }
    let regions = [(coaches, 6 * TRAINING_COACH_ENTRY)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(widget), || training_screens_coaches_panel_handle_event(widget));
    SCOREBOARD.record(equal && ro == rr, || format!("training coaches panel (input, widget {} action {}): return {} x {}, differences in {:?}", widget as i16, id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// Rule: the schedule editor: the schedule's name box, the level of each of the five training categories (0, 10, 25 or
/// 50: the current one yellow, the others green buttons) and the position and side it targets; Cancel, and Ok only
/// when the schedule has a name.
///
/// Parameter 0 is the session, 1 the schedule being edited. The green is made once (`SCHEDULE_EDITOR_GREEN`). A
/// schedule whose byte +0x15 is 7 has its name (+9) emptied first. Then the background picture
/// (`exe::background_set_picture(0)`), a full-screen area (its right and bottom edges read from the exe, patched in the
/// 1080 exe), the dark green frame (0xa0, 0x7d)–(0x2d9, 0x1da), the title (the session's own copy of the schedule's
/// name, +0xfa + 0x9b × (i8)[session+0x7d9] + 9), a two-column area (weights 3, 1) with Cancel (id 2) and Ok (id 1;
/// grey text with style 0x2c when the name is empty), the pitch (`draw_pitch` (0xb4, 0xc8)–(0x2c5, 0x19e), its outputs
/// unused) and three areas over the same rectangle (0xb9, 0xcd)–(0x2c0, 0x199), 8 rows each: 4 columns (weights 10, 2,
/// 10, 12) for the name, 5 (10, 6, 6, 6, 6) for the categories and 5 (10, 6, 6, 11, 1) for the targets. Row 0: " Name"
/// and an edit box (an item 0x80018 on the name, 11 characters, id 9). Rows 1 to 5: a space and the category name
/// (`training_schedule_get_category_name` 0x8b5520) and the four levels (`training_schedule_get_intensity_name`
/// 0x8b5690 of 0, 10, 0x19, 0x32; the level from `exe::training_schedule_get_category_level`; links 3 to 6 with the
/// category as value). Row 6: " New Position" (`schedule_editor_target_row`, mask 0x207f, link 7, Specific = 1, the 7
/// position bits); row 7: " New Side" (mask 0x4a80, link 8, Specific = 0x80, the 3 side bits).
/// Original: `training_screens_draw_schedule_editor` (0x8b9940, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_draw_schedule_editor() {
    let made = read_u8(SCHEDULE_EDITOR_GREEN_MADE);
    if made & 1 == 0 {
        write_u8(SCHEDULE_EDITOR_GREEN_MADE, made | 1);
        write_u16(SCHEDULE_EDITOR_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let buttons_weights: [u8; 2] = [3, 1];
    let name_weights: [u8; 4] = [0xa, 2, 0xa, 0xc];
    let level_weights: [u8; 5] = [0xa, 6, 6, 6, 6];
    let target_weights: [u8; 5] = [0xa, 6, 6, 0xb, 1];
    let session = get_screen_param(0);
    let schedule = get_screen_param(1);
    if read_u8(schedule + 0x15) == 7 {
        write_u8(schedule + 9, 0);
    }
    exe::background_set_picture(0usize);
    tel26_add_area(current_human_display(), [0, 0, SCHEDULE_EDITOR_RIGHT_EDGE.read_unaligned(), SCHEDULE_EDITOR_BOTTOM_EDGE.read_unaligned(), 1, 0, 1, 0, 2, 0, u32::MAX]);
    let frame = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0xa0, 0x7d, 0x2d9, 0x1da, 1, 0, 1, 0, 0x30, frame, u32::MAX]);
    let title = session + 0xfa + read_i8(session + 0x7d9) as isize as usize * TRAINING_SCHEDULE_SIZE + 9;
    comp_text_item([0xaa, 0x87, 0x2cf, 0xb4, 0x30, read_u16(G_GUI_COLOUR_BLACK) as u32, 0xc, 4, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, title as u32, 1]);
    let buttons = tel26_add_area(current_human_display(), [0xaa, 0x1b2, 0x2cf, 0x1d0, 2, buttons_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale_grey) = (read_u16(G_GUI_COLOUR_GREY) as u32, read_u16(G_GUI_COLOUR_PALE_GREY) as u32);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98a1a0);
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale_grey, TRAINING_TEXT_BUFFER as u32, 2, 0]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x990f7c);
    let name = schedule + 9;
    if read_u8(name) == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale_grey, TRAINING_TEXT_BUFFER as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale_grey, TRAINING_TEXT_BUFFER as u32, 1, 0]);
    }
    let (mut x1, mut y1, mut x2, mut y2) = (0i32, 0i32, 0i32, 0i32);
    type DrawPitch = unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, u32);
    let draw_pitch: DrawPitch = core::mem::transmute(FN_DRAW_PITCH);
    draw_pitch(0xb4, 0xc8, 0x2c5, 0x19e, &mut x1, &mut y1, &mut x2, &mut y2, 1, buttons, 0x4040_0000);
    let name_area = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 4, name_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let levels = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 5, level_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let targets = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 5, target_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let heading = read_u16(0xae3154) as u32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ee04);
    tel26_area_text([name_area, 0, 0, 1, 0, 1, 3, heading, TRAINING_TEXT_BUFFER as u32]);
    tel26_add_item(current_human_display(), [0x80018, 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 2, read_u16(SCHEDULE_EDITOR_GREEN) as u32, name as u32, 0xb, 9, 0, name_area]);
    let mut row = 1u32;
    for category in 0..5u32 {
        write_u16(TRAINING_TEXT_BUFFER, 0x0020); // " "
        exe::training_schedule_get_category_name(category, TRAINING_TEXT_BUFFER + 1, 0x7d0);
        tel26_area_text([levels, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
        let level = exe::training_schedule_get_category_level(schedule, category);
        for (col, value) in [(1u32, 0i8), (2, 0xa), (3, 0x19), (4, 0x32)] {
            exe::training_schedule_get_intensity_name(value as u32, TRAINING_TEXT_BUFFER, 0x7d0);
            schedule_editor_choice(levels, col, row, level == value, col + 2, category);
        }
        row += 1;
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d430);
    tel26_area_text([targets, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
    schedule_editor_target_row(targets, row, schedule, 0xa8d404, 0xa8d3d4, 0x207f, 0x2000, 7, 1, &SCHEDULE_POSITION_BITS);
    row += 1;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d3c8);
    tel26_area_text([targets, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
    schedule_editor_target_row(targets, row, schedule, 0xa8d3a0, 0xa8d374, 0x4a80, 0x4000, 8, 0x80, &SCHEDULE_SIDE_BITS);
}

/// Comparer for `training_screens_draw_schedule_editor`: side by side over the display list
/// (`compare_match_screen_build`) plus the schedule being edited (its name may be emptied) and the static colours.
pub(crate) unsafe extern "C" fn compare_training_screens_draw_schedule_editor() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRAINING_SCREENS_DRAW_SCHEDULE_EDITOR);
    let schedule = get_screen_param(1);
    if schedule == 0 || get_screen_param(0) == 0 {
        return o();
    }
    compare_match_screen_build("schedule editor", &[(schedule, TRAINING_SCHEDULE_SIZE), (0xde1f48, 8)], || o(),
        || training_screens_draw_schedule_editor());
}

/// Rule: the training screen of a club: the ten schedule tabs, the Schedule, View (and Attributes), Edit or Coaches
/// and Filter menus, the current schedule's description and the squad list in the chosen view, filtered and sorted,
/// with Cancel and Ok.
///
/// Parameter 0 is the edit session; 1 and 2 the title colours (the club's kit colours,
/// `database_get_club_kit_colours(club, 1)`, when both are 0). `TRAINING_SQUAD_VIEW` points at the current schedule's
/// view. The session's squad list is rebuilt in the view's order (`0x8b0d60` with the comparer of the sort flag:
/// 0x800 status, 0x1000 name, 0x8000 position, 0x10000 value, 0x40000 attribute, 0x100000 age, 0x800000 form,
/// 0x1000000 morale, 0x2000000 injury, else unsorted). Then `0x41c8e0(6, 0)`, the title "<club> Training", the menu
/// bar (4), the tabs (`training_schedule_tabs`), a three-column area on the tab bar's line (0x6e..0x1e5) with the
/// Schedule and View menus, the Edit/Filter line (`training_edit_and_filter`) and the list
/// (`training_squad_list`, `training_squad_row`). The list area gets the view's scroll row back
/// (`Area::set_scroll_row`) and becomes parameter 5; last, Cancel (link 9) and Ok (link 0xa) in a two-column area
/// (weights 3, 1). The attribute colours: risen 0xae31aa; fallen red 0xae31a8 (yellow when colourblind); unchanged
/// yellow (0xae3152 when colourblind).
/// Original: `training_screens_draw` (0x8b5a10, training_screens.cpp, cdecl, SEH)
pub(crate) unsafe extern "C" fn training_screens_draw() {
    let mut empty_list = [0u8; 0xe];
    exe::new_llist(empty_list.as_mut_ptr() as usize);
    let mut tabs = [0u8; 25 * TRAINING_TAB_SIZE];
    exe::crt_vector_constructor_iterator(tabs.as_mut_ptr() as usize, TRAINING_TAB_SIZE as u32, 25, 0x4568f0usize, 0x5a53c0usize);
    let colourblind = read_u8(G_PLAYER_SETUP_COLOURBLIND_FLAG) != 0;
    let colours = TrainingAttributeColours {
        up: job_colour(JOB_COLOUR_31AA),
        down: if colourblind { job_colour(G_GUI_COLOUR_YELLOW) } else { job_colour(G_GUI_COLOUR_RED) },
        same: if colourblind { job_colour(TRAINING_COLOUR_3152) } else { job_colour(G_GUI_COLOUR_YELLOW) },
    };
    let session = get_screen_param(0);
    let mut first = get_screen_param(1) as u32;
    let mut second = get_screen_param(2) as u32;
    let club = Club(read_usize(session + TRAINING_SESSION_CLUB));
    let view_index = read_i8(session + TRAINING_SESSION_CURRENT) as i32;
    write_usize(TRAINING_SQUAD_VIEW, training_current_schedule(session) + 0x18);
    if first as u16 == 0 && second as u16 == 0 {
        covered!("tel25_5 the title colours from the kit");
        exe::database_get_club_kit_colours(club, 1, &mut first, &mut second);
        set_screen_param(1, (first & 0xffff) as usize);
        set_screen_param(2, (second & 0xffff) as usize);
    }
    let flags = read_u32(read_usize(TRAINING_SQUAD_VIEW));
    let order = [(0x800u32, 0x8bb190usize), (0x1000, 0x8bb2a0), (0x8000, 0x8bb340), (0x10000, 0x8bb460), (0x40000, 0x8bb510),
        (0x100000, 0x8bb620), (0x800000, 0x8bb6d0), (0x1000000, 0x8bb7d0), (0x2000000, 0x8bb8d0)]
        .iter().find(|&&(bit, _)| flags & bit != 0).map_or(0, |&(_, f)| f);
    let mut squad = 0usize;
    exe::training_edit_session_list_schedule_players(session, &mut squad, order);
    exe::background_set_section_picture(6, 0);
    let gender = (club.short_gender_name() as i8) as i32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d16c, gender, gender, club.addr() + club::SHORT_NAME); // "<club> Training"
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, second, first, TRAINING_TEXT_BUFFER);
    exe::menubar_build(4, 0);
    let (top, bottom) = training_schedule_tabs(session, view_index, tabs.as_mut_ptr() as usize);
    let top_area = tel26_add_area(current_human_display(), [0x6e, top as u32, 0x1e5, (top + 0x14) as u32, 3, 0, 1, 0, 1, 0, u32::MAX]);
    training_schedule_menu(session, top_area);
    let view = read_usize(TRAINING_SQUAD_VIEW);
    training_view_menu(top_area, view, read_u16(squad) == 0);
    training_edit_and_filter(session, view, top);
    let area = training_squad_list(session, view, squad, club.addr(), top, bottom, top_area, &colours);
    let display = current_human_display();
    if area as i16 != -1 {
        exe::area_set_scroll_row(display_area_at(display, area as i16), read_usize(read_usize(TRAINING_SQUAD_VIEW) + TRAINING_VIEW_SCROLL_ROW));
    }
    set_screen_param(5, area as i16 as isize as usize);
    let buttons_weights: [u8; 2] = [3, 1];
    let buttons = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 2, buttons_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98a1a0); // "Cancel"
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale, TRAINING_TEXT_BUFFER as u32, 9, 0]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x990f7c); // "Ok"
    tel26_area_button([buttons, 1, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 3, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0xa, 0]);
    exe::crt_vector_destructor_iterator(tabs.as_mut_ptr() as usize, TRAINING_TAB_SIZE as u32, 25, 0x5a53c0usize);
    exe::drop_llist(empty_list.as_mut_ptr() as usize);
}

/// Comparer for `training_screens_draw`: side by side over the display list (`compare_match_screen_build`) plus what
/// the build writes or rolls outside it: the session's 50 squad entries (sorted in place), the squad view pointer,
/// the attribute groups (sorted in place), and what `update_player_training` touches for each listed player (his
/// person and player sheet, the training entries and the position training count, the contract lookup scratch).
/// The session's squad list itself is not restored: it is cleared and rebuilt (freed and new nodes) on each run.
/// When any listed player's training is due (`training_update_pending`, tel07_2) only the original runs.
pub(crate) unsafe extern "C" fn compare_training_screens_draw() {
    let o = original_fn::<unsafe extern "C" fn()>(IDX_TRAINING_SCREENS_DRAW);
    let session = get_screen_param(0);
    if session == 0 {
        return o();
    }
    let mut extra = vec![(session, 0xfa), (session + 0x818, 1), (TRAINING_SQUAD_VIEW, 4), (TRAINING_ATTRIBUTE_GROUPS, 0x48),
        (G_CONTRACT_LOOKUP_RESULT, 4)];
    let manager = read_usize(G_TRAINING_MANAGER);
    if manager != 0 {
        let entries = read_usize(manager + training_manager_fields::PLAYER_ENTRIES);
        if entries != 0 {
            extra.push((entries, read_u32(G_PLAYER_DATA_COUNT) as usize * 6));
        }
        extra.push((manager + training_manager_fields::POSITION_TRAINING_COUNT, 4));
        let items = read_usize(manager + training_manager_fields::POSITION_TRAINING_ITEMS);
        if items != 0 {
            extra.push((items, read_u32(manager + training_manager_fields::POSITION_TRAINING_COUNT) as usize * 8));
        }
    }
    for k in 0..50 {
        let person = Staff(read_usize(session + k * 5));
        if training_update_pending(person.addr()) {
            // simplification: a player's week of training rolls once, in the original (restoring the entries would
            // roll it again in the copy, and its injuries and position items lie outside these regions)
            return o();
        }
        if !person.is_null() {
            extra.push((person.addr(), staff::SIZE));
            let sheet = person.player().addr();
            if sheet != 0 {
                extra.push((sheet, 0x46));
            }
        }
    }
    compare_match_screen_build("training screen", &extra, || o(), || training_screens_draw());
}

/// Rule: the training screen's input: the schedule tabs choose the current schedule; Load, Save, Save As, Delete and
/// the recent files go through file dialogs (asking first when the schedule is not saved); a squad row dropped on a
/// tab moves the player to that schedule, on itself opens the player; the filter, sort and view links change the
/// squad view; Coaches opens the coaches panel, Edit the schedule editor; Cancel (asking when something changed) and
/// Ok (applying the changes) leave.
///
/// Parameter 0 is the edit session; 1 and 2 the title colours of the dialogs. `TRAINING_SQUAD_VIEW` points at the
/// current schedule's view; a real widget (not −1) stores the list's scroll row (`get_area_scroll_row` of parameter
/// 5) in the view (+9). With a widget or `TRAINING_PENDING_COMMANDS` the schedule's file name (+0x2e), or else its
/// name, is copied as the dialogs' name (200 bytes; otherwise the original leaves the buffer uninitialised). Then the
/// dialog answers (`training_dialog_answers`) and the link of the widget (`tel26_item_id`): 1 another tab (value ≠
/// the current): `TRAINING_EDIT_SESSION::set_current_schedule` (0x8b0bf0), −11; 2 Load: not saved (+0x93 = 0) asks
/// (0xc8), else the load dialog (0xc9); 0xd Save: the path "<folder>\<name>.tsh" (`_makepath`) saved (−11) or the
/// error box; 0xe Save As: the save dialog (0xca) with the name; 0xf Delete: the delete dialog (0xcb); 0x10 a recent
/// file: parameter 4 = the value, not saved asks (0xce), else loaded (−11) or the error box; 0xc a squad row dropped
/// on an item (`TRAINING_ITEM_DROPPED_ON`) of link 0xc (the same person: `0x8702a0(person, 1, 1, 0, 0)` opens him,
/// −4; another: 0) or 1 (a tab other than 0, and not a preset (kind 7): `0x8b0dc0` moves the player there; −11), any
/// other 0xc: 0; 0x11 a filter bit not already on: a position bit (1, 2, 4, 8, 0x10) clears the position bits, a side
/// bit (0x20, 0x40, 0x80, 0x100) the side bits, then it is set; −11; 8 an attribute header: the attribute sort (flag
/// 0x40000), reversed when it was already that attribute and not reversed; −11; 3 the view and sort links: 1, 2, 4, 8
/// and 0x8000000 clear 0x800000f, 0x10..0x200 clear 0x3f0 (the attribute group), the sort flags 0x400..0x4000000
/// toggle the reversal and clear the other sort flags; then the flag is set; −11; 4 Coaches: the coaches panel opens
/// on a copy (malloc 0x36) of the session's coaches (`open_screen` 0x8ba940 / 0x8baea0; parameters: the session, the
/// copy, the club), −4 (no session: box 2194; no memory: box 2202); 5 Edit: `training_screens_open_schedule_editor`,
/// −4; 9 Cancel: with changes (+0x802) asks (0xcf), else leaves; 0xa Ok: `TRAINING_EDIT_SESSION::apply_changes`
/// (0x8b0b40) and leaves; anything else to `exe::menubar_handle_command`, then `exe::menubar_handle_object_link`.
/// Original: `training_screens_handle_event` (0x8b83f0, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_handle_event(widget: u32) -> i32 {
    let session = get_screen_param(0);
    let first = get_screen_param(1) as u32;
    let second = get_screen_param(2) as u32;
    let w = widget as i16;
    write_usize(TRAINING_SQUAD_VIEW, training_current_schedule(session) + 0x18);
    if w != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(5));
        write_usize(read_usize(TRAINING_SQUAD_VIEW) + TRAINING_VIEW_SCROLL_ROW, row);
    }
    let mut name = [0u8; 200];
    if read_u32(TRAINING_PENDING_COMMANDS) != 0 || w != -1 {
        let schedule = training_current_schedule(session);
        let source = if read_u8(schedule + TRAINING_SCHEDULE_FILE) != 0 { schedule + TRAINING_SCHEDULE_FILE } else { schedule + TRAINING_SCHEDULE_NAME };
        let copy: unsafe extern "C" fn(*mut u8, usize, u32) -> usize = core::mem::transmute(FN_9466D0);
        copy(name.as_mut_ptr(), source, 200);
        name[199] = 0;
    }
    let mut text = TrainingDialogText { header: [0; 200], title: [0; 200], path: [0; 0x104] };
    if read_u32(TRAINING_PENDING_COMMANDS) != 0 {
        if let Some(r) = training_dialog_answers(session, &mut text, &name, first, second) {
            return r;
        }
    }
    let id = tel26_item_id(w);
    let value = tel26_item_value(w);
    let schedule = training_current_schedule(session);
    if id == 1 && value as i8 != read_i8(session + TRAINING_SESSION_CURRENT) {
        covered!("tel25_6 another schedule tab");
        exe::training_edit_session_select_schedule(session, value);
        return -11;
    }
    if id == 2 {
        if read_u32(schedule + TRAINING_SCHEDULE_SAVED) == 0 {
            return training_not_saved(&mut text, &name, first, second, 0xc8);
        }
        return training_file_dialog(&mut text, 0xa7fc84, 0, 0, 0xa8d354, first, second, 0xc9); // "Select File", "Load Training Schedule"
    }
    if id == 0xd {
        let folder = training_schedule_folder();
        exe::crt_makepath(text.path.as_mut_ptr(), TRAINING_EMPTY_TEXT, folder, name.as_ptr() as usize, TRAINING_FILE_EXTENSION);
        if training_save_file(session, text.path.as_ptr() as usize) != 0 {
            return -11;
        }
        let path = text.path.as_ptr() as usize;
        // "could not be saved", "Unable To Save", "Save Training Schedule"
        return training_file_error(&mut text, path, 0xa8d2e0, 0x99081c, 0xa8d2c8, first, second);
    }
    if id == 0xe {
        return training_file_dialog(&mut text, 0x9907f4, 1, name.as_ptr() as usize, 0xa8d2c8, first, second, 0xca); // "Enter File Name"
    }
    if id == 0xf {
        return training_file_dialog(&mut text, 0xa7fc84, 2, 0, 0xa8d270, first, second, 0xcb); // "Delete Training Schedule"
    }
    if id == 0x10 {
        set_screen_param(4, value as usize);
        if read_u32(training_current_schedule(session) + TRAINING_SCHEDULE_SAVED) == 0 {
            return training_not_saved(&mut text, &name, first, second, 0xce);
        }
        training_recent_path(&mut text);
        if training_load_file(session, text.path.as_ptr() as usize) != 0 {
            return -11;
        }
        let path = text.path.as_ptr() as usize;
        return training_file_error(&mut text, path, 0xa8d318, 0xa7fbe8, 0xa8d354, first, second);
    }
    if id == 0xc {
        let target = if w != -1 { training_item_dword(w, TRAINING_ITEM_DROPPED_ON) as i16 } else { -1 };
        if target != -1 {
            let target_id = match training_item_dword(target, display::ITEM_ID) as u16 { 0 => read_u16(G_CURRENT_COMMAND_ID), x => x };
            if target_id == 0xc || target_id == 1 {
                let person = if w != -1 { value } else { 0 };
                let target_value = training_item_dword(target, display::ITEM_VALUE);
                if target_id == 0xc {
                    if person != target_value {
                        return 0;
                    }
                    covered!("tel25_6 open the player from his row");
                    let open: unsafe extern "C" fn(u32, u32, u32, u32, u32) = core::mem::transmute(FN_STAFF_SCREENS_OPEN_STAFF_PROFILE);
                    open(person, 1, 1, 0, 0);
                    return -4;
                }
                let number = target_value as u8;
                if number != 0 {
                    covered!("tel25_6 a player dropped on a schedule");
                    let k = if number == 0xff { read_i8(session + TRAINING_SESSION_CURRENT) } else { number as i8 };
                    if read_u8(training_schedule(session, k) + TRAINING_SCHEDULE_KIND) != 7 {
                        exe::training_edit_session_set_player_group(session, person, number as u32);
                    }
                    return -11;
                }
            }
        }
        return 0;
    }
    let view = read_usize(TRAINING_SQUAD_VIEW);
    if id == 0x11 {
        let filter = read_u32(view + TRAINING_VIEW_FILTER);
        if value & filter == 0 {
            let kept = match value {
                1 | 2 | 4 | 8 | 0x10 => filter & 0xffff_ffe0,
                0x20 | 0x40 | 0x80 | 0x100 => filter & 0xffff_fe1f,
                _ => return -11,
            };
            write_u32(view + TRAINING_VIEW_FILTER, kept);
            write_u32(view + TRAINING_VIEW_FILTER, read_u32(view + TRAINING_VIEW_FILTER) | value);
            return -11;
        }
    }
    if id == 8 {
        let attribute = value as u8;
        let again = read_u32(view) & 0x40000 != 0 && attribute == read_u8(view + 4) && read_u32(view + TRAINING_VIEW_REVERSED) == 0;
        write_u32(view + TRAINING_VIEW_REVERSED, again as u32);
        write_u32(view, read_u32(view) & 0xfc6a_67ff);
        write_u32(view, read_u32(view) | 0x40000);
        write_u8(view + 4, attribute);
        return -11;
    }
    if id == 3 {
        match value {
            1 | 2 | 4 | 8 | 0x800_0000 => write_u32(view, read_u32(view) & 0xf7ff_fff0),
            0x10 | 0x20 | 0x40 | 0x80 | 0x100 | 0x200 => write_u32(view, read_u32(view) & 0xffff_fc0f),
            0x400 | 0x800 | 0x1000 | 0x2000 | 0x4000 | 0x8000 | 0x10000 | 0x20000 | 0x40000 | 0x80000 | 0x100000 | 0x200000 | 0x400000
            | 0x800000 | 0x100_0000 | 0x200_0000 | 0x400_0000 => {
                let again = read_u32(view) & value != 0 && read_u32(view + TRAINING_VIEW_REVERSED) == 0;
                write_u32(view + TRAINING_VIEW_REVERSED, again as u32);
                write_u32(view, read_u32(view) & 0xfc00_03ff);
            }
            _ => return -11,
        }
        write_u32(view, read_u32(view) | value);
        return -11;
    }
    if id == 4 {
        if session == 0 {
            tel26_error_box(TRAINING_SCREENS_CPP, 0x892);
            return -4;
        }
        let copy = exe::crt_malloc(0x36);
        if copy == 0 {
            tel26_error_box(TRAINING_SCREENS_CPP, 0x89a);
            return -4;
        }
        core::ptr::copy((session + TRAINING_SESSION_COACHES) as *const u8, copy as *mut u8, 0x36);
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x8ba940usize, 0x8baea0usize, 1, 0usize, 0) == 0 {
            return -4;
        }
        covered!("tel25_6 open the coaches panel");
        set_screen_param(0, session);
        set_screen_param(1, copy);
        set_screen_param(2, read_usize(session + TRAINING_SESSION_CLUB));
        return -4;
    }
    if id == 5 {
        let open: unsafe extern "C" fn(usize) = core::mem::transmute(FN_TRAINING_SCREENS_OPEN_SCHEDULE_EDITOR);
        open(session);
        return -4;
    }
    if id == 9 {
        if read_u32(session + TRAINING_SESSION_CHANGED) != 0 {
            let body = comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d198, 1, first, second, 0xcf) as usize; // "Changes have been made.."
            return training_message(&mut text, body, 0xa7ea98, 0x98e834, 1, first, second, 0xcf); // "Changes Made", "Training"
        }
        return training_leave(session);
    }
    if id == 0xa {
        exe::training_edit_session_apply_changes(session);
        return training_leave(session);
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `training_screens_handle_event`: the links that only change the squad view (0x11 filter, 8 attribute
/// sort, 3 view and sort) or the session (1 another tab in range, 0xc a row dropped on a tab) run side by side over
/// the view, the session (its list header aside) and the return; everything else (dialog answers, file dialogs and
/// files, opening screens or the player, leaving, the menu bar) runs the original only.
pub(crate) unsafe extern "C" fn compare_training_screens_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(IDX_TRAINING_SCREENS_HANDLE_EVENT);
    let session = get_screen_param(0);
    let w = widget as i16;
    if session == 0 || read_u32(TRAINING_PENDING_COMMANDS) != 0 {
        return o(widget);
    }
    let id = tel26_item_id(w);
    let value = tel26_item_value(w);
    let filter_on = read_u32(training_current_schedule(session) + 0x18 + TRAINING_VIEW_FILTER) & value != 0;
    let edits = match id {
        // a filter bit already on and the current tab fall through to the menu bar
        0x11 => !filter_on,
        8 | 3 => true,
        1 => (0..=9).contains(&(value as i8)) && value as i8 != read_i8(session + TRAINING_SESSION_CURRENT),
        0xc => w != -1 && {
            let target = training_item_dword(w, TRAINING_ITEM_DROPPED_ON) as i16;
            target != -1 && match training_item_dword(target, display::ITEM_ID) as u16 { 0 => read_u16(G_CURRENT_COMMAND_ID), x => x } == 1
        },
        _ => false,
    };
    if !edits {
        // simplification: dialogs, files, new screens and leaving do not run twice
        return o(widget);
    }
    let view = training_current_schedule(session) + 0x18;
    let regions = [(session, 0x80a), (session + 0x818, 1), (view, 0x7d), (TRAINING_SQUAD_VIEW, 4)];
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(widget), || training_screens_handle_event(widget));
    SCOREBOARD.record(equal && ro == rr, || format!("training screen (input, widget {} link {}): return {} x {}, differences in {:?}", w, id, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// `get_player_status_code` (0x4849f0, club_screens.cpp, cdecl): the player's status code (no club, no fixture).
pub(crate) unsafe fn training_status_code(person: usize) -> u8 {
    exe::get_player_status_code(person, 0, 0)
}

/// The name tie-break shared by the training squad sorts: `a` and `b` point at the two `Staff` pointers. No first
/// person: 1; no second: −1; otherwise both names formatted (`exe::database_format_staff_name`, style 3, 200 bytes)
/// and compared with `exe::utils_string_compare(first, second, 1, 0)`.
pub(crate) unsafe fn training_compare_names(a: usize, b: usize) -> i32 {
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
pub(crate) unsafe fn training_sort_nulls(a: usize, b: usize) -> Option<i32> {
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

/// The action id of widget `w` asked of the current human's Display (`exe::display_get_widget_id`; 0, and widget −1,
/// become the injected command id `G_CURRENT_COMMAND_ID`).
pub(crate) unsafe fn training_widget_action(widget: u32) -> u16 {
    let id = exe::display_get_widget_id(current_human_display(), widget);
    if id == 0 {
        read_u16(G_CURRENT_COMMAND_ID)
    } else {
        id
    }
}

/// A choice cell of the schedule editor, its text already in the buffer: the chosen one is yellow text on the 0x221
/// style; the others are green buttons with the link and the value.
pub(crate) unsafe fn schedule_editor_choice(area: u32, col: u32, row: u32, chosen: bool, link: u32, value: u32) {
    if chosen {
        let yellow = read_u16(G_GUI_COLOUR_YELLOW) as u32;
        tel26_area_text([area, col, row, 0x221, yellow, 0xc, 2, yellow, TRAINING_TEXT_BUFFER as u32]);
    } else {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, read_u16(SCHEDULE_EDITOR_GREEN) as u32, TRAINING_TEXT_BUFFER as u32, link, value]);
    }
}

/// The "New Position" / "New Side" row of the schedule editor (row `row` of `area`): "None" (column 1) chosen when the
/// schedule has none of the `mask` bits, "Specific" (column 2, value `specific`) when it has some; with some, a popup
/// button in column 4 lists `bits` (items of kind 2 with the link and the bit as value) and, unless the `exact` bit is
/// set, column 3 shows the bits named (two spaces, then `formation_get_position_flag_name` 0x5b7ab0).
pub(crate) unsafe fn schedule_editor_target_row(area: u32, row: u32, schedule: usize, none_text: usize, specific_text: usize, mask: u16, exact: u16,
    link: u32, specific: u32, bits: &[u16]) {
    let targets = || read_u16(schedule + SCHEDULE_TARGETS);
    comp_lang_format()(TRAINING_TEXT_BUFFER, none_text, SCHEDULE_EMPTY_TEXT);
    schedule_editor_choice(area, 1, row, targets() & mask == 0, link, 0);
    comp_lang_format()(TRAINING_TEXT_BUFFER, specific_text, SCHEDULE_EMPTY_TEXT);
    schedule_editor_choice(area, 2, row, targets() & mask != 0, link, specific);
    if targets() & mask == 0 {
        return;
    }
    covered!("tel25_4 a specific training target");
    let black = read_u16(G_GUI_COLOUR_BLACK) as u32;
    let owner = tel26_area_button([area, 4, row, 0x30, black, 0xc, 1, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, 0, 0, 0]);
    let popup_colour = read_u16(0xae31aa) as u32;
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x500030, popup_colour, owner]);
    for &bit in bits {
        exe::formation_get_position_flag_name(bit as u32, TRAINING_TEXT_BUFFER, 0x7d0);
        tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, popup_colour, read_u16(G_GUI_COLOUR_YELLOW) as u32, 1, 1,
            read_u16(G_GUI_COLOUR_BLACK) as u32, TRAINING_TEXT_BUFFER as u32, 0, link, bit as i16 as i32 as u32, popup]);
    }
    if targets() & exact == 0 {
        write_u16(TRAINING_TEXT_BUFFER, read_u16(SCHEDULE_INDENT));
        write_u8(TRAINING_TEXT_BUFFER + 2, 0);
        exe::formation_get_position_flag_name((targets() & mask) as u32, TRAINING_TEXT_BUFFER + 2, 0x7d0);
        tel26_area_text([area, 3, row, 1, 0, 1, 2, read_u16(G_GUI_COLOUR_YELLOW) as u32, TRAINING_TEXT_BUFFER as u32]);
    }
}

/// The session's current schedule.
pub(crate) unsafe fn training_current_schedule(session: usize) -> usize {
    training_schedule(session, read_i8(session + TRAINING_SESSION_CURRENT))
}

/// The schedule tabs: the default name of each of the ten schedules is set, and each tab shows the name, or for a
/// schedule of kind other than −1 and 7 "name (n)" with n `0x8b0d10` (how many of the squad follow it); a preset (kind
/// 7) is grey. Returns the tab bar's top and bottom (`gui_add_tab_bar`, from 0x50 and 0x221).
pub(crate) unsafe fn training_schedule_tabs(session: usize, view_index: i32, tabs: usize) -> (i32, i32) {
    let format: CompFormatFn = comp_crt_format();
    for k in 0..10i8 {
        let schedule = training_schedule(session, k);
        exe::training_schedule_set_default_name(schedule);
        let table = tabs + k as usize * TRAINING_TAB_SIZE;
        let kind = read_u8(schedule + TRAINING_SCHEDULE_KIND);
        if kind == 0xff || kind == 7 {
            tel26_copy_text((table + 4) as *mut u8, (schedule + TRAINING_SCHEDULE_NAME) as *const u8);
        } else {
            let n = exe::training_edit_session_count_schedule_players(session, k as u8 as u32) as i8;
            format(table + 4, 0xa8d164, schedule + TRAINING_SCHEDULE_NAME, n as i32); // "%s (%d)"
        }
        if read_u8(schedule + TRAINING_SCHEDULE_KIND) == 7 {
            write_u16(table + 2, read_u16(G_GUI_COLOUR_GREY));
        }
        write_u32(table + 0xcc, 1);
        write_i32(table + 0xd0, k as i32);
    }
    let (mut top, mut bottom) = (0x50i32, 0x221i32);
    exe::gui_add_tab_bar(tabs, 10, view_index, &mut top, &mut bottom, 1);
    (top, bottom)
}

/// The Schedule menu (column 0 of the top area): greyed when the current schedule's kind is 0 or −1; otherwise a
/// popup with Load (link 2), Save (0xd; only when not saved and with a file), Save As (0xe; not for a preset), Delete
/// (0xf) and the last four schedule files (`0x5fc760` of the human manager): each, when it names a file, "n. name"
/// (link 0x10, value n − 1; greyed when `0x8b5820` cannot read it), the first one after a separator.
pub(crate) unsafe fn training_schedule_menu(session: usize, area: u32) {
    let format: CompFormatFn = comp_crt_format();
    type RecentFile = unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32, u32) -> u32;
    let recent: RecentFile = core::mem::transmute(FN_HUMAN_MANAGER_GET_RECENT_FILE);
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d138, TRAINING_EMPTY_TEXT); // "Schedule<..>"
    let schedule = training_current_schedule(session);
    let kind = read_u8(schedule + TRAINING_SCHEDULE_KIND);
    if kind == 0 || kind == 0xff {
        tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        return;
    }
    let owner = tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    let text = TRAINING_TEXT_BUFFER as u32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7f4); // "Load"
    training_menu_item(popup, 2, 0x10, 1, text, 2, 0);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7dc); // "Save"
    let schedule = training_current_schedule(session);
    if read_u32(schedule + TRAINING_SCHEDULE_SAVED) == 0 && read_u32(schedule + TRAINING_SCHEDULE_HAS_FILE) != 0 {
        training_menu_item(popup, 2, 0x10, 1, text, 0xd, 0);
    } else {
        training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e7c4); // "Save As"
    if read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND) != 7 {
        training_menu_item(popup, 2, 0x10, 1, text, 0xe, 0);
    } else {
        training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa69fb4); // "Delete"
    training_menu_item(popup, 2, 0x10, 1, text, 0xf, 0);
    let mut separator = u32::MAX;
    let (mut file, mut read_back) = ([0u8; 0x104], [0u8; 0x6c]);
    for k in 0..4u32 {
        let manager = read_usize(G_HUMAN_MANAGER);
        recent(manager, k, file.as_mut_ptr(), 0x104, 0, 0, 3);
        if file[0] == 0 {
            continue;
        }
        covered!("tel25_5 a recent schedule file");
        let ok = exe::training_schedule_read_file_header(file.as_ptr(), read_back.as_mut_ptr());
        recent(read_usize(G_HUMAN_MANAGER), k, file.as_mut_ptr(), 0x104, 1, 0, 3);
        if separator as i16 == -1 {
            separator = training_menu_item(popup, 1, 0x100_0010, 1, 0, 0, 0);
        }
        format(TRAINING_TEXT_BUFFER, 0xa7e728, k + 1, file.as_ptr()); // "%d. %s"
        if ok != 0 {
            training_menu_item(popup, 2, 0x10, 1, text, 0x10, k);
        } else {
            training_menu_item(popup, 2, 0x10, 0x21, text, 0, 0);
        }
    }
}

/// The View menu (column 1 of the top area; greyed when the squad list is empty): General (link 3, value 1) and
/// Attributes (value 4), ticked by the view flags; in the Attributes view, the Attributes menu (column 2): Physical,
/// Mental, Goalkeeping, Defensive and Attacking (link 3, values 0x10 to 0x100).
pub(crate) unsafe fn training_view_menu(area: u32, view: usize, squad_empty: bool) {
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x988004); // "View"
    if squad_empty {
        tel26_area_text([area, 1, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        return;
    }
    let owner = tel26_area_button([area, 1, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d10c, TRAINING_EMPTY_TEXT); // "  General<..>"
    training_menu_choice(popup, read_u8(view) & 1 != 0, 3, 1);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa7e6e8); // "  Attributes"
    training_menu_choice(popup, read_u8(view) & 4 != 0, 3, 4);
    if read_u8(view) & 4 == 0 {
        return;
    }
    covered!("tel25_5 the attributes view menu");
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ece8); // "Attributes"
    let owner = tel26_area_button([area, 2, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    // "  Physical", "  Mental", "  Goalkeeping", "  Defensive", "  Attacking"
    for (text, bit) in [(0xa7e6a4usize, 0x10u32), (0xa7e698, 0x20), (0xa7e688, 0x40), (0xa7e67c, 0x80), (0xa7e670, 0x100)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, read_u32(view) & bit != 0, 3, bit);
    }
}

/// The Edit (or Coaches) button and the Filter menu, on the tab bar's line: a schedule of kind −1 shows Coaches (link
/// 4), kind 0 a grey Edit, any other Edit (link 5); the Filter popup lists All Positions, Goalkeepers, Defenders,
/// Midfielders, Attackers, a separator, All Sides, Left Sided, Central and Right Sided (link 0x11, the filter bit as
/// value), ticked by the view's filter.
pub(crate) unsafe fn training_edit_and_filter(session: usize, view: usize, top: i32) {
    let (grey, pale) = (job_colour(G_GUI_COLOUR_GREY), job_colour(G_GUI_COLOUR_PALE_GREY));
    let area = tel26_add_area(current_human_display(), [0x212, top as u32, 0x30c, (top + 0x14) as u32, 2, 0, 1, 0, 1, 0, u32::MAX]);
    let kind = read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND);
    if kind == 0xff {
        comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d104); // "Coaches"
        tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 4, 0]);
    } else {
        comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8be68); // "Edit"
        if read_u8(training_current_schedule(session) + TRAINING_SCHEDULE_KIND) == 0 {
            tel26_area_text([area, 0, 0, 0x30, grey, 0x2c, 1, pale, TRAINING_TEXT_BUFFER as u32]);
        } else {
            tel26_area_button([area, 0, 0, 0x30, grey, 0xc, 1, pale, TRAINING_TEXT_BUFFER as u32, 5, 0]);
        }
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ebf8); // "Filter"
    let owner = tel26_add_button([0x28f, top as u32, 0x30c, (top + 0x14) as u32, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1,
        job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 0, 0]);
    let popup = training_popup(owner);
    let filter = |bit: u32| read_u32(view + TRAINING_VIEW_FILTER) & bit != 0;
    // " All Positions", " Goalkeepers", " Defenders", " Midfielders", " Attackers"
    for (text, bit) in [(0x98ebe8usize, 1u32), (0x98ebd8, 2), (0x98ebcc, 4), (0x98ebbc, 8), (0x98ebb0, 0x10)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, filter(bit), 0x11, bit);
    }
    training_menu_item(popup, 1, 0x100_0010, 1, 0, 0, 0);
    // " All Sides", " Left Sided", " Central", " Right Sided"
    for (text, bit) in [(0x98eba4usize, 0x20u32), (0x98eb98, 0x40), (0x98eb8c, 0x80), (0x98eb7c, 0x100)] {
        comp_lang_format()(TRAINING_TEXT_BUFFER, text);
        training_menu_choice(popup, filter(bit), 0x11, bit);
    }
}

/// One row of the squad list: the player's schedule name (when its number is 2 or more), the status cell, the name (a
/// kind 0x46 item with link 0xc and the person; grey when injured), then by view: General: positions, age, form,
/// morale and condition ("Inj" when injured, else n%); Attributes: each attribute of the group in a colour by the
/// training change; last, the value (`Cash` 5) or "Free".
pub(crate) unsafe fn training_squad_row(session: usize, area: u32, row: u32, entry: usize, club: usize, table: usize, colours: &TrainingAttributeColours) {
    let format: CompFormatFn = comp_crt_format();
    let lang = comp_lang_format();
    let text = TRAINING_TEXT_BUFFER as u32;
    let person = Staff(read_usize(entry));
    exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), person, 0);
    let number = read_i8(entry + 4);
    let name = if number > 1 { training_schedule(session, number) + TRAINING_SCHEDULE_NAME } else { 0 };
    tel26_area_text([area, 0, row, 0x30, job_colour(G_GUI_COLOUR_DARK_BLUE), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), name as u32]);
    let status = exe::get_player_status_code(person, club, (0) as u32 as usize);
    training_status_cell(area, row, person.addr(), club, status);
    exe::database_format_staff_name(person, TRAINING_TEXT_BUFFER, 0x7d0, 3);
    let ink = if status == 6 { job_colour(G_GUI_COLOUR_GREY) } else { job_colour(G_GUI_COLOUR_PALE_GREY) };
    tel26_add_item(current_human_display(), [0x46, 0, 0, 0, 0, 2, row, 1, 0, 0, 1, 2, ink, text, 0, 0xc, person.addr() as u32, area]);
    let view = read_usize(TRAINING_SQUAD_VIEW);
    let mut words = [0u8; 200];
    if read_u32(view) & 1 != 0 {
        exe::database_format_player_positions(person.player().addr(), (words.as_mut_ptr()) as *mut u8 as usize, 0xc8, 1);
        format(TRAINING_TEXT_BUFFER, 0x987ff0, words.as_ptr()); // "  %s"
        tel26_area_text([area, 3, row, 1, 0, 1, 2, job_colour(JOB_COLOUR_BDE8), text]);
        format(TRAINING_TEXT_BUFFER, 0x98806c, (person.age() as i8) as i32); // "%d"
        tel26_area_text([area, 4, row, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDE8), text]);
        exe::playerstats_format_recent_ratings(read_usize(G_PLAYER_STATS), person.player().addr(), TRAINING_TEXT_BUFFER, 0x7d0);
        tel26_area_text([area, 5, row, 1, 0, 0xc, 1, job_colour(G_GUI_COLOUR_YELLOW), text]);
        // the original loads ecx with the contract manager; the routine is stdcall and does not read it
        let morale: unsafe extern "stdcall" fn(usize, *mut u8, u32) = core::mem::transmute(FN_CONTRACT_MANAGER_WRITE_MORALE_TEXT);
        morale(person.addr(), words.as_mut_ptr(), 0xc8);
        format(TRAINING_TEXT_BUFFER, 0x991944, words.as_ptr()); // "%s"
        tel26_area_text([area, 6, row, 1, 0, 0xc, 2, job_colour(G_GUI_COLOUR_YELLOW), text]);
        if exe::injury_get_player_injury_status(read_usize(G_INJURY), person) == 2 {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
        } else {
            let condition = exe::injury_get_condition(read_usize(G_INJURY), person, 1);
            format(TRAINING_TEXT_BUFFER, 0x98e8d4, condition as i32); // "%d%%"
        }
        tel26_area_text([area, 7, row, 1, 0, 0xc, 2, job_colour(JOB_COLOUR_BDD4), text]);
    } else if read_u32(view) & 4 != 0 && read_u8(table) != 0xff {
        let mut k = 0u8;
        while read_u8(table + k as usize) != 0xff {
            let attribute = read_u8(table + k as usize) as u32;
            format(TRAINING_TEXT_BUFFER, 0x98806c, exe::staff_get_player_attribute(person, attribute) as i32); // "%d"
            let delta = exe::training_manager_get_training_attribute_change(read_usize(G_TRAINING_MANAGER), person, read_u8(table + k as usize) as u32);
            let ink = if delta > 0 { colours.up } else if delta < 0 { colours.down } else { colours.same };
            tel26_area_text([area, k as u32 + 3, row, 1, 0, 0xc, 2, ink, text]);
            k = k.wrapping_add(1);
        }
    }
    let value = person.value() as u32;
    if club != 0 && value != 0 {
        let cash = value;
        let amount = exe::cash_get_display_amount(&cash, 5);
        exe::cash_format_money(TRAINING_TEXT_BUFFER, 0x7d0, amount, 5);
    } else {
        lang(TRAINING_TEXT_BUFFER, 0x98e614); // "Free"
    }
    tel26_area_text([area, 0xc, row, 0x30, job_colour(JOB_COLOUR_3192), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), text]);
}

/// The squad list under the tab bar: the schedule's description (when it has a file name) and two areas, then, when
/// the squad is not empty, the players that pass the filter in a new list, the headers of the view and one row each
/// (from the end when the view is reversed). Returns the area whose scroll row is restored.
pub(crate) unsafe fn training_squad_list(session: usize, view: usize, squad: usize, club: usize, mut y: i32, bottom: i32, top_area: u32,
    colours: &TrainingAttributeColours) -> u32 {
    y += 0x19;
    let file = training_current_schedule(session) + TRAINING_SCHEDULE_FILE;
    if read_u8(file) != 0 {
        exe::at_9466d0(TRAINING_TEXT_BUFFER, (file) as usize as *const u8, 0x7d0);
        write_u8(TRAINING_TEXT_BUFFER + 0x7cf, 0);
        comp_text_item([0x6e, y as u32, 0x30c, (y + 0x23) as u32, 2, 0, 0xc, 3, job_colour(G_GUI_COLOUR_YELLOW), TRAINING_TEXT_BUFFER as u32, 1]);
        y += 0x28;
    }
    let mut area = tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, (y + 0x14) as u32, 9, 0, 1, 0, 1, 0, u32::MAX]);
    tel26_add_area(current_human_display(), [0x6e, y as u32, 0x30c, bottom as u32, 1, 0, 1, 0, 2, 0, u32::MAX]);
    if read_u16(squad) == 0 {
        return area;
    }
    covered!("tel25_5 the squad list");
    let block = exe::crt_operator_new(0xe);
    let shown = if block != 0 {
        exe::new_llist(block)
    } else {
        0
    };
    write_u32(squad + 0xa, read_u32(squad + 2));
    loop {
        let node = exe::llist_get_next(squad);
        if node == 0 {
            break;
        }
        if training_passes_filter(view, read_usize(node)) {
            exe::llist_add_tail(shown, node, 5);
        }
    }
    let height = bottom - y - 0x10;
    let rows = (height / exe::font_get_height(2)) as u8;
    let row_height = ftol(height as f64 / rows as f64 + 0.5);
    let right = if (read_i16(shown) as i32) < rows as i32 { 0x30c } else { 0x2f6 };
    let general_weights: [u8; 13] = [0xa, 5, 0x1a, 0xc, 5, 0xa, 0xa, 6, 0, 0, 0, 0, 0xa];
    let attribute_weights: [u8; 13] = [0xa, 5, 0x15, 5, 5, 5, 5, 5, 5, 5, 5, 5, 0xa];
    // the attribute group; without a group bit the original keeps the stack dword whose low byte holds the rows
    let mut table = (top_area & 0xffff_ff00) as usize | rows as usize;
    let flags = read_u32(view);
    if flags & 1 != 0 {
        let header = tel26_add_area(current_human_display(), [0x6e, (y + 8) as u32, right, (row_height + y + 7) as u32, 0xd,
            general_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        // "Inf", "   Name", "   Position<..>", "Age", "Form", "Morale", "Cond.", "Value"
        training_list_header(header, 1, 0xc, 0x98ea64, 0x800);
        training_list_header(header, 2, 1, 0x98ea5c, 0x1000);
        training_list_header(header, 3, 1, 0xa8d0d4, 0x8000);
        training_list_header(header, 4, 0xc, 0x98e9dc, 0x100000);
        training_list_header(header, 5, 0xc, 0x98e9f8, 0x800000);
        training_list_header(header, 6, 0xc, 0x98e9f0, 0x1000000);
        training_list_header(header, 7, 0xc, 0x98e9e8, 0x2000000);
        training_list_header(header, 0xc, 0xc, 0x98ea0c, 0x10000);
        area = tel26_add_area(current_human_display(), [0x6e, (y + row_height + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd,
            general_weights.as_ptr() as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX]);
    } else if flags & 4 != 0 {
        covered!("tel25_5 the attributes view");
        let header = tel26_add_area(current_human_display(), [0x6e, (y + 8) as u32, right, (row_height + y + 7) as u32, 0xd,
            attribute_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        training_list_header(header, 1, 0xc, 0x98ea64, 0x800);
        training_list_header(header, 2, 1, 0x98ea5c, 0x1000);
        let flags = read_u32(view);
        for (bit, group) in [(0x10u32, 0usize), (0x20, 0x30), (0x40, 0xc), (0x80, 0x18), (0x100, 0x24), (0x200, 0x3c)] {
            if flags & bit != 0 {
                table = TRAINING_ATTRIBUTE_GROUPS + group;
            }
        }
        exe::crt_qsort(table, 9, 1, 0x81ae60usize);
        let mut k = 0u8;
        while read_u8(table + k as usize) != 0xff {
            exe::database_get_player_attribute_name(read_u8(table + k as usize) as u32, TRAINING_TEXT_BUFFER, 0x7d0, 1);
            tel26_area_button([header, k as u32 + 3, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY),
                TRAINING_TEXT_BUFFER as u32, 8, read_i8(table + k as usize) as i32 as u32]);
            k = k.wrapping_add(1);
        }
        while (k as i8) < 9 {
            tel26_area_text([header, k as u32 + 3, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), 0xc, 1, job_colour(G_GUI_COLOUR_PALE_GREY), 0x98ba0c]); // "-"
            k += 1;
        }
        training_list_header(header, 0xc, 0xc, 0x98ea0c, 0x10000);
        area = tel26_add_area(current_human_display(), [0x6e, (y + row_height + 8) as u32, 0x30c, (bottom - 8) as u32, 0xd,
            attribute_weights.as_ptr() as u32, rows.wrapping_sub(1) as u32, 0, 1, 0, u32::MAX]);
    }
    let reversed = read_u32(view + TRAINING_VIEW_REVERSED) != 0;
    write_u32(shown + 0xa, read_u32(shown + if reversed { 6 } else { 2 }));
    let mut row = 0u32;
    loop {
        let entry = exe::llist_get_current(shown);
        if entry == 0 {
            break;
        }
        if read_u32(read_usize(TRAINING_SQUAD_VIEW) + TRAINING_VIEW_REVERSED) == 0 {
            exe::llist_get_next(shown);
        } else {
            exe::llist_get_prev(shown);
        }
        training_squad_row(session, area, row, entry, club, table, colours);
        row += 1;
    }
    if shown != 0 {
        exe::drop_llist(shown);
        exe::crt_operator_delete(shown);
    }
    area
}

/// Field `offset` (dword) of display item `w` of the current human's Display.
pub(crate) unsafe fn training_item_dword(w: i16, offset: usize) -> u32 {
    read_u32((current_human_display() as isize + offset as isize + w as isize * display_list::ITEM_SIZE as isize) as usize)
}

/// The schedule folder of the human manager (`0x5fc5b0` thiscall(0, 3)).
pub(crate) unsafe fn training_schedule_folder() -> usize {
    exe::human_manager_get_folder_path(read_usize(G_HUMAN_MANAGER), (0) as u32 as usize, 3)
}

/// A file dialog (`0x58f500`, 14 arguments) for the schedule files: the prompt (mode 0 "Select File" to load, 1
/// "Enter File Name" to save with `default` proposed, 2 "Select File" to delete), the title, the title colours and the
/// pending command `code` its answer comes back with. Returns −4.
pub(crate) unsafe fn training_file_dialog(text: &mut TrainingDialogText, prompt: usize, mode: u32, default: usize, title: usize, first: u32, second: u32, code: u32) -> i32 {
    let lang = comp_lang_format();
    let folder = training_schedule_folder();
    let header = lang(text.header.as_mut_ptr() as usize, prompt, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first, second,
        code, 0, 0, 0, 0) as usize;
    let caption = lang(text.title.as_mut_ptr() as usize, title, header, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first,
        second, code, 0, 0, 0, 0) as usize;
    type FileDialog = unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32);
    let dialog: FileDialog = core::mem::transmute(FN_OPEN_FILE_DIALOG);
    dialog(caption, header, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first, second, code, 0, 0, 0, 0);
    -4
}

/// A message box (`0x591840`, 7 arguments) over the text already formatted (`body`, the return of the format into the
/// screens' buffer): the header, the title, `question` (1 asks yes/no), the title colours and the pending command
/// `code` the answer comes back with. Returns −4.
pub(crate) unsafe fn training_message(text: &mut TrainingDialogText, body: usize, header: usize, title: usize, question: u32, first: u32, second: u32, code: u32) -> i32 {
    let lang = comp_lang_format();
    let h = lang(text.header.as_mut_ptr() as usize, header, body, question, first, second, code) as usize;
    let t = lang(text.title.as_mut_ptr() as usize, title, h, body, question, first, second, code) as usize;
    exe::confirm_push(t, h, body, question, first, second, code);
    -4
}

/// "The training schedule "<file>" could not be <loaded / saved / deleted>." under "Unable To <...>".
pub(crate) unsafe fn training_file_error(text: &mut TrainingDialogText, file: usize, format: usize, header: usize, title: usize, first: u32, second: u32) -> i32 {
    let body = comp_lang_format()(TRAINING_TEXT_BUFFER, format, file, 0, first, second, 0) as usize;
    training_message(text, body, header, title, 0, first, second, 0)
}

/// "The training schedule "<name>" has not been saved. Load another schedule ?" (answer `code`).
pub(crate) unsafe fn training_not_saved(text: &mut TrainingDialogText, name: &[u8; 200], first: u32, second: u32, code: u32) -> i32 {
    let body = comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d220, name.as_ptr(), 1, first, second, code) as usize;
    training_message(text, body, 0xa8d20c, 0xa8d354, 1, first, second, code) // "Schedule Not Saved", "Load Training Schedule"
}

/// Loads the schedule file `path` into the session's current schedule (`0x8b0e00`): nonzero when it worked.
pub(crate) unsafe fn training_load_file(session: usize, path: usize) -> u32 {
    exe::training_edit_session_load_schedule_file(session, path)
}

/// Saves the session's current schedule to `path` (`0x8b0e70`): nonzero when it worked.
pub(crate) unsafe fn training_save_file(session: usize, path: usize) -> u32 {
    exe::training_edit_session_save_schedule_file(session, path)
}

/// The recent schedule file in screen parameter 4 into the path (`0x5fc760` of the human manager, mode 0).
pub(crate) unsafe fn training_recent_path(text: &mut TrainingDialogText) {
    type RecentFile = unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32, u32) -> u32;
    let recent: RecentFile = core::mem::transmute(FN_HUMAN_MANAGER_GET_RECENT_FILE);
    let k = get_screen_param(4) as u32;
    recent(read_usize(G_HUMAN_MANAGER), k, text.path.as_mut_ptr(), 0x104, 0, 0, 3);
}

/// Leaving the screen: the edit session destroyed and freed (when there is one); −9.
pub(crate) unsafe fn training_leave(session: usize) -> i32 {
    if session != 0 {
        exe::drop_training_edit_session(session);
        exe::crt_operator_delete(session);
    }
    -9
}

/// The answers of the dialogs (only with `TRAINING_PENDING_COMMANDS`): 0xc8 yes to "load another" opens the load
/// dialog (0xc9); 0xc9 a file to load, 0xca a file to save (its name, `_splitpath`, becomes the schedule's file), 0xcb
/// a file to delete (`remove`): on failure the "could not be" box and −4, the name always freed; 0xcc yes opens the
/// save dialog (0xca) with the schedule's name when it has a file; 0xcd nothing; 0xce yes loads the recent file of
/// parameter 4; 0xcf yes to "cancel all changes" leaves (−9). Each answer clears the pending command.
pub(crate) unsafe fn training_dialog_answers(session: usize, text: &mut TrainingDialogText, name: &[u8; 200], first: u32, second: u32) -> Option<i32> {
    let mut code = read_u32(G_PENDING_COMMAND_CODE);
    if code == 0xc8 {
        code = 0;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if read_u32(G_PENDING_COMMAND_ARG) == 2 {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            // "Select File", "Load Training Schedule"
            return Some(training_file_dialog(text, 0xa7fc84, 0, 0, 0xa8d354, first, second, 0xc9));
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    for (answer, failure, header, title) in [(0xc9u32, 0xa8d318usize, 0xa7fbe8usize, 0xa8d354usize), (0xca, 0xa8d2e0, 0x99081c, 0xa8d2c8),
        (0xcb, 0xa8d28c, 0xa7f710, 0xa8d270)] {
        if code != answer {
            continue;
        }
        let file = read_usize(G_PENDING_COMMAND_ARG);
        code = 0;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if file == 0 {
            continue;
        }
        covered!("tel25_6 a schedule file chosen");
        let done = match answer {
            0xc9 => training_load_file(session, file) != 0,
            0xca => training_save_file(session, file) != 0,
            _ => exe::crt_remove(file as *const u8) == 0,
        };
        if !done {
            training_file_error(text, file, failure, header, title, first, second);
            exe::crt_operator_delete(file);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            return Some(-4);
        }
        if answer == 0xca {
            exe::crt_strtok(file, 0usize, 0usize, (training_current_schedule(session) + TRAINING_SCHEDULE_FILE) as *mut u8, core::ptr::null_mut());
        }
        exe::crt_operator_delete(file);
        code = read_u32(G_PENDING_COMMAND_CODE);
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code == 0xcc {
        code = 0;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if read_u32(G_PENDING_COMMAND_ARG) == 2 {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let default = if read_u32(training_current_schedule(session) + TRAINING_SCHEDULE_HAS_FILE) != 0 { name.as_ptr() as usize } else { TRAINING_EMPTY_TEXT };
            // "Enter File Name", "Save Training Schedule"
            return Some(training_file_dialog(text, 0x9907f4, 1, default, 0xa8d2c8, first, second, 0xca));
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code == 0xcd {
        code = 0;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code == 0xce {
        code = 0;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if read_u32(G_PENDING_COMMAND_ARG) == 2 {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            training_recent_path(text);
            if training_load_file(session, text.path.as_ptr() as usize) == 0 {
                let path = text.path.as_ptr() as usize;
                return Some(training_file_error(text, path, 0xa8d318, 0xa7fbe8, 0xa8d354, first, second));
            }
            code = read_u32(G_PENDING_COMMAND_CODE);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code == 0xcf {
        let yes = read_u32(G_PENDING_COMMAND_ARG) == 2;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        if yes {
            return Some(training_leave(session));
        }
    }
    None
}

/// An entry of the training screen's popup menus (an item of the current human's Display in area `popup`): kind 2
/// (0x2002 ticked, 1 a separator), flags, state (1, 0x21 greyed), text, link and value, in the popup colours.
pub(crate) unsafe fn training_menu_item(popup: u32, kind: u32, flags: u32, state: u32, text: u32, link: u32, value: u32) -> u32 {
    tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 0, 0, flags, job_colour(JOB_COLOUR_31AA), job_colour(JOB_COLOUR_BDEC), state, 1,
        job_colour(JOB_COLOUR_BDEA), text, 0, link, value, popup])
}

/// A ticked/plain popup entry with the text already in the buffer: link and value, ticked when `on`.
pub(crate) unsafe fn training_menu_choice(popup: u32, on: bool, link: u32, value: u32) {
    training_menu_item(popup, if on { 0x2002 } else { 2 }, 0x10, 1, TRAINING_TEXT_BUFFER as u32, link, value);
}

/// A popup owned by item `owner` (an empty area of style 0x130 in the popup colour).
pub(crate) unsafe fn training_popup(owner: u32) -> u32 {
    tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x130, job_colour(JOB_COLOUR_31AA), owner])
}

/// A header button of the squad list: column `col` of `area`, the sort flag `sort` as value of link 3, centred
/// (style 0xc) or left (1).
pub(crate) unsafe fn training_list_header(area: u32, col: u32, style: u32, text: usize, sort: u32) {
    comp_lang_format()(TRAINING_TEXT_BUFFER, text, TRAINING_EMPTY_TEXT);
    tel26_area_button([area, col, 0, 0x30, job_colour(G_GUI_COLOUR_GREY), style, 1, job_colour(G_GUI_COLOUR_PALE_GREY), TRAINING_TEXT_BUFFER as u32, 3, sort]);
}

/// Does player `person` pass the view's filter? All when both "all" bits (1 and 0x20) are on; otherwise each position
/// bit asks the position mask (`exe::formation_get_position_mask` of the player sheet) for goalkeeper (1), defender
/// (6), midfielder (0x38) or attacker (0x40), and each side bit for left (0x80), central (0x200) or right (0x800).
pub(crate) unsafe fn training_passes_filter(view: usize, person: usize) -> bool {
    let person = Staff(person);
    let f = read_u32(view + TRAINING_VIEW_FILTER);
    if f & 1 != 0 && f & 0x20 != 0 {
        return true;
    }
    let m = exe::formation_get_position_mask(person.player().addr());
    let f = read_u32(view + TRAINING_VIEW_FILTER);
    [(2u32, 1u16), (4, 6), (8, 0x38), (0x10, 0x40), (0x40, 0x80), (0x80, 0x200), (0x100, 0x800)]
        .iter().all(|&(bit, mask)| f & bit == 0 || m & mask != 0)
}

/// The status cell of a squad row (column 1): a short code and a link by `get_player_status_code` (0x4849f0): 1 Wpm, 2
/// Cup, 3 Ine, 9 Fgn (link 0x7e0 at a national team, else 0x7db), 4 on international duty (link 0x7d5 with the duty
/// club, else a plain text item with whatever the buffer holds), 5 Sus (0x7de / 0x7d9), 6 and 8 Inj (0x7dd / 0x7d8),
/// 0x15 Abs (kind 0x202), 7 Fgn, 0xa Yel (0x7de / 0x7d9, yellow from `graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0)`), 0x10 Ctr
/// (0x7da), 0x11 Ret (0x7e0 / 0x7db), 0xb..0xf, 0x12..0x14 and 0x18 the transfer codes (0x7db); others nothing.
pub(crate) unsafe fn training_status_cell(area: u32, row: u32, person: usize, club: usize, status: u8) {
    let text = TRAINING_TEXT_BUFFER as u32;
    let lang = comp_lang_format();
    // kind 2 item: colours f8/f9 `c`, f12 `c12`, link and value
    let item = |kind: u32, c8: u32, c9: u32, c12: u32, link: u32, value: u32| {
        tel26_add_item(current_human_display(), [kind, 0, 0, 0, 0, 1, row, 0x30, c8, c9, 0xc, 1, c12, text, 0, link, value, area]);
    };
    let (blue, pale, red_ink) = (job_colour(G_GUI_COLOUR_DARK_BLUE), job_colour(G_GUI_COLOUR_PALE_GREY), job_colour(TRAINING_COLOUR_BDCC));
    let national = |code: u32, other: u32| if exe::database_is_national_team(club) != 0 { code } else { other };
    match status {
        1 | 2 | 3 => {
            match status {
                1 => lang(TRAINING_TEXT_BUFFER, 0x98e9c8), // "Wpm"
                2 => lang(TRAINING_TEXT_BUFFER, 0x98e998, TRAINING_EMPTY_TEXT), // "Cup<..>"
                _ => lang(TRAINING_TEXT_BUFFER, 0x98e994), // "Ine"
            };
            let link = national(0x7e0, 0x7db);
            item(2, blue, blue, pale, link, person as u32);
        }
        4 => {
            let duty = exe::nationalteams_get_international_duty_club(read_usize(G_NATIONAL_TEAMS), person, G_CURRENT_DATE, club, 0, 0);
            if duty != 0 {
                item(2, blue, blue, pale, 0x7d5, duty);
            } else {
                tel26_add_item(current_human_display(), [1, 0, 0, 0, 0, 1, row, 0x30, blue, blue, 0xc, 1, pale, text, 0, 0, 0, area]);
            }
        }
        5 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e990); // "Sus"
            let link = national(0x7de, 0x7d9);
            item(2, red_ink, red_ink, pale, link, person as u32);
        }
        6 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
            let link = national(0x7dd, 0x7d8);
            item(2, red_ink, red_ink, pale, link, person as u32);
        }
        0x15 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e988); // "Abs"
            item(0x202, red_ink, red_ink, pale, 0x7db, person as u32);
        }
        7 | 9 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e984); // "Fgn"
            item(2, blue, blue, pale, 0x7db, person as u32);
        }
        8 => {
            lang(TRAINING_TEXT_BUFFER, 0x98e98c); // "Inj"
            let link = national(0x7dd, 0x7d8);
            let c = job_colour(TRAINING_COLOUR_3190);
            item(2, c, c, pale, link, person as u32);
        }
        0xa => {
            covered!("tel25_5 a yellow card status");
            lang(TRAINING_TEXT_BUFFER, 0x98e978); // "Yel"
            let link = national(0x7de, 0x7d9);
            let black = job_colour(G_GUI_COLOUR_BLACK);
            let first = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0) as u16 as u32;
            let second = exe::graphics_rgb_to_pixel(0xb4, 0xb4, 0, 0) as u16 as u32;
            item(2, second, first, black, link, person as u32);
        }
        0xb..=0x14 | 0x18 => {
            match status {
                0x18 => lang(TRAINING_TEXT_BUFFER, 0x98e970), // "Trn"
                0xb => lang(TRAINING_TEXT_BUFFER, 0x98e974),  // "Bid"
                0xc => lang(TRAINING_TEXT_BUFFER, 0x98e96c),  // "Wnt"
                0xd => lang(TRAINING_TEXT_BUFFER, 0x98e934),  // "Req"
                0xe => lang(TRAINING_TEXT_BUFFER, 0x98e908, TRAINING_EMPTY_TEXT), // "Lst<..>"
                0xf => lang(TRAINING_TEXT_BUFFER, 0x98e904),  // "Loa"
                0x10 => lang(TRAINING_TEXT_BUFFER, 0x98e900), // "Ctr"
                0x11 => lang(TRAINING_TEXT_BUFFER, 0x98e8f8), // "Ret "
                0x12 => lang(TRAINING_TEXT_BUFFER, 0x98e8f0), // "Bos "
                0x13 => lang(TRAINING_TEXT_BUFFER, 0x98e8e8), // "Unh "
                _ => lang(TRAINING_TEXT_BUFFER, 0x98e8e0),    // "Fut "
            };
            let link = match status {
                0x10 => 0x7da,
                0x11 => national(0x7e0, 0x7db),
                _ => 0x7db,
            };
            let c = job_colour(TRAINING_COLOUR_317C);
            item(2, c, c, job_colour(G_GUI_COLOUR_YELLOW), link, person as u32);
        }
        _ => {}
    }
}

/// "E:\dev\CM3\cm3 00-01\cm3\code\training_screens.cpp", the path the training_screens.cpp error boxes print.
pub(crate) const TRAINING_SCREENS_CPP: usize = 0xa8d1d8;

/// The coaches panel's "assigned" text colour [0xde1f4a] (a static word, green: `graphics_rgb_to_pixel(0, 0xc0, 0,
/// 0)`) and the flag byte [0xde1f48] whose bit 0 says it is made.
pub(crate) const TRAINING_COACH_GREEN: usize = 0xde1f4a;

pub(crate) const TRAINING_COACH_GREEN_MADE: usize = 0xde1f48;

/// The coaches being edited: six entries of 9 bytes (malloc 0x36, copied from the session +0x7a3): +0 the staff id
/// (−1 none), +4 one byte per training category (5) set when the coach takes it.
pub(crate) const TRAINING_COACH_ENTRY: usize = 9;

/// The immediates the 1920x1080 exe patches in the coaches panel: the right (push 0x31f at 0x8baa5f) and bottom (push
/// 0x257 at 0x8baa57) edges of its full-screen area, read from the running exe.
pub(crate) const TRAINING_COACHES_RIGHT_EDGE: *const u32 = 0x8baa60 as *const u32;

pub(crate) const TRAINING_COACHES_BOTTOM_EDGE: *const u32 = 0x8baa58 as *const u32;

/// The text buffer the screens format into (0xde1f64).
pub(crate) const TRAINING_TEXT_BUFFER: usize = 0xde1f64;

/// The schedule editor's "choosable" colour [0xde1f4c] (a static word, green: `graphics_rgb_to_pixel(0, 0xc0, 0, 0)`)
/// and the flag byte [0xde1f49] whose bit 0 says it is made.
pub(crate) const SCHEDULE_EDITOR_GREEN: usize = 0xde1f4c;

pub(crate) const SCHEDULE_EDITOR_GREEN_MADE: usize = 0xde1f49;

/// The immediates the 1920x1080 exe patches in the schedule editor: the right (push 0x31f at 0x8b9a7b) and bottom (push
/// 0x257 at 0x8b9a73) edges of its full-screen area, read from the running exe.
pub(crate) const SCHEDULE_EDITOR_RIGHT_EDGE: *const u32 = 0x8b9a7c as *const u32;

pub(crate) const SCHEDULE_EDITOR_BOTTOM_EDGE: *const u32 = 0x8b9a74 as *const u32;

/// A schedule's target flags (word +0x16): the position bits (0x207f; 0x2000 alone means no specific position named)
/// and the side bits (0x4a80; 0x4000 the same for the side) [inference: from the editor's two blocks].
pub(crate) const SCHEDULE_TARGETS: usize = 0x16;

/// The position bits the "New Position" popup offers, and the side bits of "New Side".
pub(crate) const SCHEDULE_POSITION_BITS: [u16; 7] = [1, 2, 4, 8, 0x10, 0x20, 0x40];

pub(crate) const SCHEDULE_SIDE_BITS: [u16; 3] = [0x80, 0x200, 0x800];

/// "" (0xad9c64) and "  " (0xa78880).
pub(crate) const SCHEDULE_EMPTY_TEXT: usize = 0xad9c64;

pub(crate) const SCHEDULE_INDENT: usize = 0xa78880;

/// The edit session's fields the training screen reads: the current schedule (i8, 0..9), the schedules from +0xfa
/// (0x9b bytes each), the club, the squad list (an LLIST of 5-byte entries: +0 the person, +4 the schedule).
pub(crate) const TRAINING_SESSION_CURRENT: usize = 0x7d9;

pub(crate) const TRAINING_SESSION_CLUB: usize = 0x806;

/// A schedule's fields: +9 its name, +0x15 its kind (0 and −1 nothing to edit, 7 a preset that cannot be renamed),
/// +0x18 the squad view (0x7d bytes, see `TRAINING_SQUAD_VIEW`), +0x2e the file it was saved to, +0x93 saved since the
/// last change, +0x97 it has a file [inference: from the menu's Save and Load].
pub(crate) const TRAINING_SCHEDULE_NAME: usize = 9;

pub(crate) const TRAINING_SCHEDULE_KIND: usize = 0x15;

pub(crate) const TRAINING_SCHEDULE_FILE: usize = 0x2e;

pub(crate) const TRAINING_SCHEDULE_SAVED: usize = 0x93;

pub(crate) const TRAINING_SCHEDULE_HAS_FILE: usize = 0x97;

/// The squad view's fields: +0 the flags (bit 1 General, 4 Attributes, 0x10..0x200 the attribute group, 0x800..
/// 0x2000000 the sort column), +4 the sort attribute, +5 reversed, +9 the list's scroll row, +0xd the filter (1 all
/// positions, 2 goalkeepers, 4 defenders, 8 midfielders, 0x10 attackers, 0x20 all sides, 0x40 left, 0x80 central,
/// 0x100 right).
pub(crate) const TRAINING_VIEW_REVERSED: usize = 5;

pub(crate) const TRAINING_VIEW_SCROLL_ROW: usize = 9;

pub(crate) const TRAINING_VIEW_FILTER: usize = 0xd;

/// The tab of each schedule handed to `gui_add_tab_bar` (0xd4 bytes, `0x4568f0` builds them): +2 the colour, +4 the
/// text, +0xcc 1, +0xd0 the schedule.
pub(crate) const TRAINING_TAB_SIZE: usize = 0xd4;

/// "" (0xad9c64).
pub(crate) const TRAINING_EMPTY_TEXT: usize = 0xad9c64;

/// Colours the screen reads besides the gui colours: 0xae31aa (the popups), 0xae3152, 0xae317c, 0xae3190, 0xaebdcc.
pub(crate) const TRAINING_COLOUR_3152: usize = 0xae3152;

pub(crate) const TRAINING_COLOUR_317C: usize = 0xae317c;

pub(crate) const TRAINING_COLOUR_3190: usize = 0xae3190;

pub(crate) const TRAINING_COLOUR_BDCC: usize = 0xaebdcc;

/// The attribute groups of the Attributes view (nine attribute bytes and −1, sorted in place by the builder):
/// physical, mental, goalkeeping, defensive, attacking, and 0xa8d0c8 (all −1) for the view flag 0x200.
pub(crate) const TRAINING_ATTRIBUTE_GROUPS: usize = 0xa8d08c;

/// The colours of the attribute cells, read once at the start: risen, fallen and unchanged (the colourblind setting
/// swaps the last two).
pub(crate) struct TrainingAttributeColours {
    pub(crate) up: u32,
    pub(crate) down: u32,
    pub(crate) same: u32,
}

/// The pending command flag [0xb77c84] (set in a network game): the answers of the screen's dialogs come back
/// through `G_PENDING_COMMAND_CODE` (0xc8 to 0xcf) and `G_PENDING_COMMAND_ARG` (2 = yes, or the chosen file name).
pub(crate) const TRAINING_PENDING_COMMANDS: usize = 0xb77c84;

/// The item a display item was dropped on (+0x76 in the item, i16; −1 none) [inference: the drag of a squad row].
pub(crate) const TRAINING_ITEM_DROPPED_ON: usize = 0xba9dc;

/// The edit session's "changed" flag (+0x802) and its coaches (six 9-byte entries from +0x7a3).
pub(crate) const TRAINING_SESSION_CHANGED: usize = 0x802;

pub(crate) const TRAINING_SESSION_COACHES: usize = 0x7a3;

/// ".tsh" (0xa8d36c), the schedule files' extension.
pub(crate) const TRAINING_FILE_EXTENSION: usize = 0xa8d36c;

/// The file dialog's callback (0x8b57e0).
pub(crate) const TRAINING_FILE_CALLBACK: usize = 0x8b57e0;

/// The two text buffers of the dialogs (the header and the title) and the file path.
pub(crate) struct TrainingDialogText {
    pub(crate) header: [u8; 200],
    pub(crate) title: [u8; 200],
    pub(crate) path: [u8; 0x104],
}

/// The squad view the training screen's builder points at [0xde1f50] (inside the edit session: +0x112 + 0x7d × the
/// view); its byte +4 is the attribute shown in the sortable attribute column [inference: the builder writes it, the
/// attribute sort reads it].
pub(crate) const TRAINING_SQUAD_VIEW: usize = 0xde1f50;
