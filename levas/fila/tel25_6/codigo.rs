// ---------- batch %%LEVA%%: the training screens (5): the training screen's input (training_screens.cpp) ----------

/// The pending command flag [0xb77c84] (set in a network game): the answers of the screen's dialogs come back
/// through `G_PENDING_COMMAND_CODE` (0xc8 to 0xcf) and `G_PENDING_COMMAND_ARG` (2 = yes, or the chosen file name).
const TRAINING_PENDING_COMMANDS: usize = 0xb77c84;
/// The item a display item was dropped on (+0x76 in the item, i16; −1 none) [inference: the drag of a squad row].
const TRAINING_ITEM_DROPPED_ON: usize = 0xba9dc;
/// The edit session's "changed" flag (+0x802) and its coaches (six 9-byte entries from +0x7a3).
const TRAINING_SESSION_CHANGED: usize = 0x802;
const TRAINING_SESSION_COACHES: usize = 0x7a3;
/// ".tsh" (0xa8d36c), the schedule files' extension.
const TRAINING_FILE_EXTENSION: usize = 0xa8d36c;
/// The file dialog's callback (0x8b57e0).
const TRAINING_FILE_CALLBACK: usize = 0x8b57e0;

/// The two text buffers of the dialogs (the header and the title) and the file path.
struct TrainingDialogText {
    header: [u8; 200],
    title: [u8; 200],
    path: [u8; 0x104],
}

/// Field `offset` (dword) of display item `w` of the current human's Display.
unsafe fn training_item_dword(w: i16, offset: usize) -> u32 {
    read_u32((current_human_display() as isize + offset as isize + w as isize * display_list::ITEM_SIZE as isize) as usize)
}

/// The schedule folder of the human manager (`0x5fc5b0` thiscall(0, 3)).
unsafe fn training_schedule_folder() -> usize {
    let folder: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5fc5b0usize);
    folder(read_usize(G_HUMAN_MANAGER), 0, 3)
}

/// A file dialog (`0x58f500`, 14 arguments) for the schedule files: the prompt (mode 0 "Select File" to load, 1
/// "Enter File Name" to save with `default` proposed, 2 "Select File" to delete), the title, the title colours and the
/// pending command `code` its answer comes back with. Returns −4.
unsafe fn training_file_dialog(text: &mut TrainingDialogText, prompt: usize, mode: u32, default: usize, title: usize, first: u32, second: u32, code: u32) -> i32 {
    let lang = comp_lang_format();
    let folder = training_schedule_folder();
    let header = lang(text.header.as_mut_ptr() as usize, prompt, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first, second,
        code, 0, 0, 0, 0) as usize;
    let caption = lang(text.title.as_mut_ptr() as usize, title, header, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first,
        second, code, 0, 0, 0, 0) as usize;
    type FileDialog = unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32);
    let dialog: FileDialog = core::mem::transmute(0x58f500usize);
    dialog(caption, header, mode, folder, TRAINING_FILE_EXTENSION, default, TRAINING_FILE_CALLBACK, first, second, code, 0, 0, 0, 0);
    -4
}

/// A message box (`0x591840`, 7 arguments) over the text already formatted (`body`, the return of the format into the
/// screens' buffer): the header, the title, `question` (1 asks yes/no), the title colours and the pending command
/// `code` the answer comes back with. Returns −4.
unsafe fn training_message(text: &mut TrainingDialogText, body: usize, header: usize, title: usize, question: u32, first: u32, second: u32, code: u32) -> i32 {
    let lang = comp_lang_format();
    let h = lang(text.header.as_mut_ptr() as usize, header, body, question, first, second, code) as usize;
    let t = lang(text.title.as_mut_ptr() as usize, title, h, body, question, first, second, code) as usize;
    let message: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    message(t, h, body, question, first, second, code);
    -4
}

/// "The training schedule "<file>" could not be <loaded / saved / deleted>." under "Unable To <...>".
unsafe fn training_file_error(text: &mut TrainingDialogText, file: usize, format: usize, header: usize, title: usize, first: u32, second: u32) -> i32 {
    let body = comp_lang_format()(TRAINING_TEXT_BUFFER, format, file, 0, first, second, 0) as usize;
    training_message(text, body, header, title, 0, first, second, 0)
}

/// "The training schedule "<name>" has not been saved. Load another schedule ?" (answer `code`).
unsafe fn training_not_saved(text: &mut TrainingDialogText, name: &[u8; 200], first: u32, second: u32, code: u32) -> i32 {
    let body = comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d220, name.as_ptr(), 1, first, second, code) as usize;
    training_message(text, body, 0xa8d20c, 0xa8d354, 1, first, second, code) // "Schedule Not Saved", "Load Training Schedule"
}

/// Loads the schedule file `path` into the session's current schedule (`0x8b0e00`): nonzero when it worked.
unsafe fn training_load_file(session: usize, path: usize) -> u32 {
    let load: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8b0e00usize);
    load(session, path)
}

/// Saves the session's current schedule to `path` (`0x8b0e70`): nonzero when it worked.
unsafe fn training_save_file(session: usize, path: usize) -> u32 {
    let save: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8b0e70usize);
    save(session, path)
}

/// The recent schedule file in screen parameter 4 into the path (`0x5fc760` of the human manager, mode 0).
unsafe fn training_recent_path(text: &mut TrainingDialogText) {
    type RecentFile = unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32, u32) -> u32;
    let recent: RecentFile = core::mem::transmute(0x5fc760usize);
    let k = get_screen_param(4) as u32;
    recent(read_usize(G_HUMAN_MANAGER), k, text.path.as_mut_ptr(), 0x104, 0, 0, 3);
}

/// Leaving the screen: the edit session destroyed and freed (when there is one); −9.
unsafe fn training_leave(session: usize) -> i32 {
    if session != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b9740usize);
        destroy(session);
        exe::crt_operator_delete(session);
    }
    -9
}

/// The answers of the dialogs (only with `TRAINING_PENDING_COMMANDS`): 0xc8 yes to "load another" opens the load
/// dialog (0xc9); 0xc9 a file to load, 0xca a file to save (its name, `_splitpath`, becomes the schedule's file), 0xcb
/// a file to delete (`remove`): on failure the "could not be" box and −4, the name always freed; 0xcc yes opens the
/// save dialog (0xca) with the schedule's name when it has a file; 0xcd nothing; 0xce yes loads the recent file of
/// parameter 4; 0xcf yes to "cancel all changes" leaves (−9). Each answer clears the pending command.
unsafe fn training_dialog_answers(session: usize, text: &mut TrainingDialogText, name: &[u8; 200], first: u32, second: u32) -> Option<i32> {
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
        let copy: unsafe extern "C" fn(*mut u8, usize, u32) -> usize = core::mem::transmute(0x9466d0usize);
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
        let choose: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8b0bf0usize);
        choose(session, value);
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
                    let open: unsafe extern "C" fn(u32, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
                    open(person, 1, 1, 0, 0);
                    return -4;
                }
                let number = target_value as u8;
                if number != 0 {
                    covered!("tel25_6 a player dropped on a schedule");
                    let k = if number == 0xff { read_i8(session + TRAINING_SESSION_CURRENT) } else { number as i8 };
                    if read_u8(training_schedule(session, k) + TRAINING_SCHEDULE_KIND) != 7 {
                        let assign: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x8b0dc0usize);
                        assign(session, person, number as u32);
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
        let open: unsafe extern "C" fn(usize) = core::mem::transmute(0x8b97e0usize);
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
        let apply: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b0b40usize);
        apply(session);
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
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
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
