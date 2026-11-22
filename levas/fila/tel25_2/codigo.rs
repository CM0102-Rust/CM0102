// ---------- batch %%LEVA%%: the training screens (2): opening the training screen and the schedule editor, the schedule editor's input and the coaches panel (training_screens.cpp) ----------

/// The coaches panel's "assigned" text colour [0xde1f4a] (a static word, green: `graphics_rgb_to_pixel(0, 0xc0, 0,
/// 0)`) and the flag byte [0xde1f48] whose bit 0 says it is made.
const TRAINING_COACH_GREEN: usize = 0xde1f4a;
const TRAINING_COACH_GREEN_MADE: usize = 0xde1f48;
/// The coaches being edited: six entries of 9 bytes (malloc 0x36, copied from the session +0x7a3): +0 the staff id
/// (−1 none), +4 one byte per training category (5) set when the coach takes it.
const TRAINING_COACH_ENTRY: usize = 9;
/// The immediates the 1920x1080 exe patches in the coaches panel: the right (push 0x31f at 0x8baa5f) and bottom (push
/// 0x257 at 0x8baa57) edges of its full-screen area, read from the running exe.
const TRAINING_COACHES_RIGHT_EDGE: *const u32 = 0x8baa60 as *const u32;
const TRAINING_COACHES_BOTTOM_EDGE: *const u32 = 0x8baa58 as *const u32;

/// The action id of widget `w` asked of the current human's Display (`exe::display_get_widget_id`; 0, and widget −1,
/// become the injected command id `G_CURRENT_COMMAND_ID`).
unsafe fn training_widget_action(widget: u32) -> u16 {
    let id = exe::display_get_widget_id(current_human_display(), widget);
    if id == 0 {
        read_u16(G_CURRENT_COMMAND_ID)
    } else {
        id
    }
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
        let construct: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b0a00usize);
        construct(block, club)
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
    original_fn::<unsafe extern "C" fn(usize)>(%%I0%%)(club);
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
        let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8b4e10usize);
        construct(block)
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
    original_fn::<unsafe extern "C" fn(usize)>(%%I1%%)(session);
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
    let set_level: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x8b4f30usize);
    match tel26_item_id(w) {
        3 => {
            set_level(schedule, tel26_item_value(w), 0);
            return -11;
        }
        4 => {
            set_level(schedule, tel26_item_value(w), 10);
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
            set_level(schedule, tel26_item_value(w), 0x19);
            return -11;
        }
        _ => {}
    }
    match training_widget_action(widget) {
        6 => {
            set_level(schedule, exe::display_get_widget_data(current_human_display(), widget) as u32, 0x32);
            -11
        }
        7 => {
            let set_position: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8b5090usize);
            set_position(schedule, exe::display_get_widget_data(current_human_display(), widget));
            -11
        }
        8 => {
            let set_side: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8b50c0usize);
            set_side(schedule, exe::display_get_widget_data(current_human_display(), widget));
            -11
        }
        2 => {
            if schedule != 0 {
                exe::crt_operator_delete(schedule);
            }
            -9
        }
        _ => {
            let action: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x468b40usize);
            if action(G_SCREEN_MANAGER, widget) == 1 {
                covered!("tel25_2 keep the edited schedule");
                let keep: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8b0c90usize);
                keep(session, schedule);
                if schedule != 0 {
                    let delete: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x8ba920usize);
                    delete(schedule, 1);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
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
    let category_name: unsafe extern "C" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x8b5520usize);
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
            category_name(category, TRAINING_TEXT_BUFFER, 0x7d0);
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

/// The text buffer the screens format into (0xde1f64).
pub(crate) const TRAINING_TEXT_BUFFER: usize = 0xde1f64;

/// Comparer for `training_screens_draw_coaches_panel`: side by side over the display list
/// (`compare_match_screen_build`) plus the coaches being edited (the entries it empties) and the green colour.
pub(crate) unsafe extern "C" fn compare_training_screens_draw_coaches_panel() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I3%%);
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
        let keep: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8b0ce0usize);
        keep(session, coaches);
        exe::crt_free(coaches);
        return -9;
    }
    exe::menubar_handle_object_link(widget)
}

/// Comparer for `training_screens_coaches_panel_handle_event`: the category toggles side by side over the six entries
/// and the return; Cancel, Ok and the menu bar (they free the edit, write the session or open screens) run the original
/// only.
pub(crate) unsafe extern "C" fn compare_training_screens_coaches_panel_handle_event(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I4%%);
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
