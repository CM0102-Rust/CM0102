// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 3): its event handler ----------

/// The Tactics_Screen fields the event handler uses that tel14_1 does not name.
#[allow(dead_code)]
mod tactics_event_fields {
    /// u8: the view before the last tab or zone change [inference].
    pub(crate) const PREVIOUS_VIEW: usize = 0x2728;
    /// u32: the lock state is saved in the formation file (Lock and Unlock ask to save when it is not) [inference].
    pub(crate) const LOCK_SAVED: usize = 0x26c7;
    /// u32: changes were made since the screen opened (Cancel asks before throwing them away).
    pub(crate) const CHANGES_MADE: usize = 0x4a5f;
    /// The copied coordinates of one ball zone (Copy) and of every zone of a phase (Copy All), pasted by Paste.
    pub(crate) const COPIED_ZONE: usize = 0xc2a1;
    pub(crate) const COPIED_PHASE: usize = 0xc2cd;
    /// The size of a team data block (the working one, the undo copy and the clipboard: 0xf07 dwords and 3 bytes).
    pub(crate) const TEAM_DATA_SIZE: usize = 0x3c1f;
}

/// The display item fields of a drag and drop (+0x18c x item): the item of the pair (the one dropped on, or dragged
/// onto this one) [inference], the item rectangle (x1, y1, x2, y2) and the rectangle where it was let go.
#[allow(dead_code)]
mod tactics_event_item {
    pub(crate) const PARTNER: usize = 0xba9dc;
    pub(crate) const RECT: usize = 0xba96e;
    pub(crate) const DROP_RECT: usize = 0xba9be;
}

/// The results of the handler: done (-4), the screen closed (-9), redraw (-0xb).
const TACTICS_EVENT_DONE: i32 = -4;
const TACTICS_EVENT_CLOSED: i32 = -9;
const TACTICS_EVENT_REDRAW: i32 = -0xb;
/// The flag that dispatches the answer of a pending dialog (`G_PENDING_COMMAND_CODE` / `_ARG`).
const TACTICS_PENDING_DISPATCH: usize = 0xb77c84;
/// The pitch positions offered in the overview (words, -1 at the end).
const TACTICS_PITCH_POSITIONS: usize = 0x9cc368;
/// The static paths of the preset folder (Load Preset, Save As Preset).
const TACTICS_PRESET_LOAD_PATH: usize = 0xde1d14;
const TACTICS_PRESET_SAVE_PATH: usize = 0xde1e2c;
/// "CM3_TACTICS", the empty string and the file extensions.
const TACTICS_FOLDER_KEY: usize = 0x9ccae4;
const TACTICS_NO_TEXT: usize = 0xad9c64;
const TACTICS_EXT_TCT: usize = 0xa8c3b0;
const TACTICS_EXT_PCT: usize = 0xa8c0c8;
/// The file dialog callbacks: a formation file and a preset file chosen.
const TACTICS_ON_FILE: usize = 0x5b87e0;
const TACTICS_ON_PRESET: usize = 0x5b88a0;
/// The screens the handler opens: Team Instructions (editable and read-only builders, its handler) and the player
/// instructions of a row (builder, handler).
const TACTICS_TEAM_INSTRUCTIONS_FN: usize = 0x8a2db0;
const TACTICS_TEAM_INSTRUCTIONS_RO_FN: usize = 0x8a4830;
const TACTICS_TEAM_INSTRUCTIONS_EVENT_FN: usize = 0x8a4840;
const TACTICS_PLAYER_INSTRUCTIONS_FN: usize = 0x8a5330;
const TACTICS_PLAYER_INSTRUCTIONS_EVENT_FN: usize = 0x8a6f80;
/// `Tactics_Team_Data::~Tactics_Team_Data` (0x899e50, tel14_1), called by address.
const TACTICS_TEAM_DATA_DESTRUCT_FN: usize = 0x899e50;

/// The handler's translated texts.
mod tactics_event_texts {
    pub(crate) const SELECT_FILE: usize = 0xa7fc84;
    pub(crate) const ENTER_FILE_NAME: usize = 0x9907f4;
    pub(crate) const LOAD_FORMATION: usize = 0xa8c3a0;
    pub(crate) const LOAD_PRESET_FORMATION: usize = 0xa8c2dc;
    pub(crate) const SAVE_FORMATION: usize = 0xa8c254;
    pub(crate) const SAVE_AS_PRESET: usize = 0xa8c1cc;
    pub(crate) const DELETE_FORMATION: usize = 0xa8c1b8;
    pub(crate) const SET_TO_DEFAULT: usize = 0xa8bf84;
    pub(crate) const UNLOCK_FORMATION: usize = 0xa8c060;
    pub(crate) const LOCK_FORMATION: usize = 0xa8bffc;
    pub(crate) const UNABLE_TO_LOAD: usize = 0xa7fbe8;
    pub(crate) const UNABLE_TO_SAVE: usize = 0x99081c;
    pub(crate) const UNABLE_TO_DELETE: usize = 0xa7f710;
    pub(crate) const NOT_SAVED: usize = 0xa8c11c;
    pub(crate) const TACTICS: usize = 0x98e840;
    pub(crate) const CHANGES_MADE: usize = 0xa7ea98;
    pub(crate) const PLEASE_CONFIRM: usize = 0x9909b8;
    /// "The formation "<file>" has been locked by <creator>." (load and delete; the save has its own copy).
    pub(crate) const LOCKED_BY: usize = 0xa8c360;
    pub(crate) const LOCKED_BY_SAVE: usize = 0xa8c264;
    pub(crate) const NOT_LOADED: usize = 0xa8c32c;
    pub(crate) const PRESET_NOT_LOADED: usize = 0xa8c2f4;
    pub(crate) const NOT_SAVED_FILE: usize = 0xa8c224;
    pub(crate) const NOT_SAVED_AS_PRESET: usize = 0xa8c1e8;
    pub(crate) const NOT_DELETED: usize = 0xa8c184;
    pub(crate) const ASK_DEFAULT: usize = 0xa8c130;
    pub(crate) const ASK_LOAD: usize = 0xa8c0d0;
    pub(crate) const ASK_SAVE_UNLOCK: usize = 0xa8c074;
    pub(crate) const ASK_SAVE_LOCK: usize = 0xa8c00c;
    pub(crate) const ASK_CANCEL: usize = 0xa8bfb8;
    pub(crate) const NO_RECENT: usize = 0x9906ac;
    pub(crate) const NO_RECENT_MATCHES: usize = 0x9905a8;
    pub(crate) const NO_RECENT_RESERVE: usize = 0x990680;
    pub(crate) const NO_RECENT_B: usize = 0x9906c0;
    pub(crate) const NO_TACTICS: usize = 0x9907a0;
    pub(crate) const NO_TACTICS_MATCH: usize = 0x9906f8;
    pub(crate) const NO_TACTICS_RESERVE: usize = 0x990748;
    pub(crate) const NO_TACTICS_B: usize = 0x9907b8;
    pub(crate) const CLUB_RESERVES: usize = 0x990728;
    pub(crate) const CLUB_B_SQUAD: usize = 0x990780;
}

/// `langlib_sprintf` (variadic: the original pushes the arguments of the next call with each text, and they go in
/// here too).
unsafe fn tactics_sprintf() -> unsafe extern "C" fn(usize, usize, ...) -> usize {
    core::mem::transmute(FN_LANGLIB_SPRINTF)
}

/// `confirm_push(title, heading, text, buttons, colour a, colour b, code)` (0x591840) with the translated heading and
/// title (each sprintf gets the arguments still on the stack, as in the original). `text` is already formatted.
unsafe fn tactics_confirm(c: (u32, u32), title: usize, heading: usize, text: usize, buttons: u32, code: u32) {
    let s = tactics_sprintf();
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = s(hb.as_mut_ptr() as usize, heading, text, buttons, c.0, c.1, code);
    let t = s(tb.as_mut_ptr() as usize, title, h, text, buttons, c.0, c.1, code);
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    f(t, h, text, buttons, c.0, c.1, code);
}

/// `open_file_dialog(title, heading, mode, folder, extension, default name, on ok, colour a, colour b, code, 0, 0,
/// 0, 0)` (0x58f500) with the translated heading and title.
unsafe fn tactics_file_dialog(c: (u32, u32), title: usize, heading: usize, mode: u32, folder: usize, ext: usize, name: usize, on_ok: usize, code: u32) {
    let s = tactics_sprintf();
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = s(hb.as_mut_ptr() as usize, heading, mode, folder, ext, name, on_ok, c.0, c.1, code, 0, 0, 0, 0);
    let t = s(tb.as_mut_ptr() as usize, title, h, mode, folder, ext, name, on_ok, c.0, c.1, code, 0, 0, 0, 0);
    let f: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x58f500usize);
    f(t, h, mode, folder, ext, name, on_ok, c.0, c.1, code, 0, 0, 0, 0);
}

/// `HUMAN_MANAGER::get_folder_path(current manager, 1)` (0x5fc5b0): the formations folder.
unsafe fn tactics_folder() -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x5fc5b0usize);
    f(read_usize(G_HUMAN_MANAGER), 0, 1)
}

/// `utils_build_file_path("CM3_TACTICS", "")` copied into a static path buffer (the inline strcpy).
unsafe fn tactics_preset_folder(dst: usize) -> usize {
    let f: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    tel26_copy_text(dst as *mut u8, f(TACTICS_FOLDER_KEY, TACTICS_NO_TEXT) as *const u8);
    dst
}

/// `formation_read_file_password(path, password, creator, 0)` (0x5b8600); true when the file has a password that is
/// not the current manager's (the inline strcmp against `HUMAN_MANAGER::get_password`).
unsafe fn tactics_locked_by_other(path: usize, password: &mut [u8; 0x34], creator: &mut [u8; 200]) -> bool {
    let f: unsafe extern "C" fn(usize, *mut u8, *mut u8, u32) -> i32 = core::mem::transmute(0x5b8600usize);
    f(path, password.as_mut_ptr(), creator.as_mut_ptr(), 0);
    if password[0] == 0 {
        return false;
    }
    let mine = tactics_calls::password(0) as *const u8;
    let mut k = 0;
    loop {
        let b = *mine.add(k);
        if b != password[k] {
            return true;
        }
        if b == 0 {
            return false;
        }
        k += 1;
    }
}

/// `Tactics_Screen::load_formation(path, preset)` (0x8a95d0) and `save_formation(path, locking manager)` (0x8a9700).
unsafe fn tactics_load(ts: usize, path: usize, preset: u32) -> u32 {
    tactics_calls::thiscall2(0x8a95d0, ts, path as u32, preset)
}
unsafe fn tactics_save(ts: usize, path: usize, manager: usize) -> u32 {
    tactics_calls::thiscall2(0x8a9700, ts, path as u32, manager as u32)
}

/// `Tactics_Screen::set_preset_formation(preset)` (0x8a9950).
unsafe fn tactics_set_preset(ts: usize, preset: usize) {
    display_method1(0x8a9950, ts, preset as u32);
}

/// The undo copy of the working team data (`UNDO_DATA`), with `CAN_UNDO` set.
unsafe fn tactics_event_save_undo(ts: usize) {
    use tactics_screen as t;
    core::ptr::copy_nonoverlapping((ts + t::FORMATION) as *const u8, (ts + t::UNDO_DATA) as *mut u8, tactics_event_fields::TEAM_DATA_SIZE);
    write_u32(ts + t::CAN_UNDO, 1);
}

/// The working team data back from the undo copy.
unsafe fn tactics_event_restore_undo(ts: usize) {
    use tactics_screen as t;
    core::ptr::copy_nonoverlapping((ts + t::UNDO_DATA) as *const u8, (ts + t::FORMATION) as *mut u8, tactics_event_fields::TEAM_DATA_SIZE);
}

/// The row of the captain (the row whose player has the id `CAPTAIN`), or -1.
unsafe fn tactics_captain_row(ts: usize) -> i8 {
    let mut r: i8 = 0;
    loop {
        let p = tactics_calls::slot_player(ts, r);
        if p != 0 && read_u32(p) == read_u32(ts + tactics_screen::CAPTAIN) {
            return r;
        }
        r += 1;
        if r >= 0x14 {
            return -1;
        }
    }
}

/// Destroys and frees the Tactics_Screen the way the Cancel and Ok answers do: the three team data blocks by their
/// destructor (clipboard, undo, working), the two match team blocks and the memory.
unsafe fn tactics_event_free(ts: usize) {
    use tactics_screen as t;
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::CLIPBOARD_DATA);
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::UNDO_DATA);
    display_method(TACTICS_TEAM_DATA_DESTRUCT_FN, ts + t::FORMATION);
    exe::crt_vector_destructor_iterator(ts + t::MATCH_TEAMS, 0x625, 2, MATCH_TEAM_DESTRUCT_FN);
    exe::crt_operator_delete(ts);
}

/// A copy of the Tactics_Screen (`Tactics_Screen::Tactics_Screen(const Tactics_Screen*)`, 0x8a7e00), or 0 when the
/// allocation fails.
unsafe fn tactics_screen_copy(ts: usize) -> usize {
    let p = exe::crt_operator_new(tactics_screen::SIZE);
    if p == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8a7e00usize);
    f(p, ts)
}

/// The answer of a pending dialog of the screen (`G_PENDING_COMMAND_CODE` and `_ARG`, 2 = yes), each code cleared
/// when seen: 0x34 set to default, 0x2c load (the file dialog), 0x2d the file to load (refused when another
/// manager's password locks it), 0x36 the preset to load, 0x2e the file to save, 0x35 the preset to save, 0x2f the
/// file to delete (the file name is freed after each), 0x30 a recent file, 0x31 Cancel (frees the object: closed),
/// 0x32 Ok (the captain picked when the current one is not on the pitch, the changes applied, the object freed:
/// closed), 0x33 save before leaving (the file dialog). A failure is a confirm box. None when the event goes on to
/// the item dispatch.
unsafe fn tactics_event_dialog_answer(ts: usize, c: (u32, u32)) -> Option<i32> {
    use tactics_event_texts as x;
    use tactics_screen as t;
    let s = tactics_sprintf();
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let arg = || read_u32(G_PENDING_COMMAND_ARG);
    let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
    if code() == 0x34 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            tactics_set_preset(ts, get_screen_param(9));
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x2c {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let folder = tactics_folder();
            tactics_file_dialog(c, x::LOAD_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2d);
            return Some(TACTICS_EVENT_DONE);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x2d {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if tactics_load(ts, file, 0) == 0 {
                let text = s(JOB_TEXT, x::NOT_LOADED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x36 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the preset to load");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_load(ts, file, 1) == 0 {
                let text = s(JOB_TEXT, x::PRESET_NOT_LOADED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::LOAD_PRESET_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x2e {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to save");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY_SAVE, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if tactics_save(ts, file, get_screen_param(5)) == 0 {
                let text = s(JOB_TEXT, x::NOT_SAVED_FILE, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x35 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the preset to save");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            // Formation::save_to_disk(path, silent, obfuscated)
            if tactics_calls::thiscall3(0x5b6900, ts + t::FORMATION, file as u32, 1, 1) == 0 {
                let text = s(JOB_TEXT, x::NOT_SAVED_AS_PRESET, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::SAVE_AS_PRESET, x::UNABLE_TO_SAVE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x2f {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let file = arg() as usize;
        if file != 0 {
            covered!("tel14_3 answer: the file to delete");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if tactics_locked_by_other(file, &mut password, &mut creator) {
                let text = s(JOB_TEXT, x::LOCKED_BY, file, creator.as_ptr(), 0, c.0, c.1, 0);
                tactics_confirm(c, x::DELETE_FORMATION, x::UNABLE_TO_DELETE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            if exe::crt_remove(file as *const u8) != 0 {
                let text = s(JOB_TEXT, x::NOT_DELETED, file, 0, c.0, c.1, 0);
                tactics_confirm(c, x::DELETE_FORMATION, x::UNABLE_TO_DELETE, text, 0, 0);
                exe::crt_operator_delete(file);
                return Some(TACTICS_EVENT_DONE);
            }
            exe::crt_operator_delete(file);
        }
    }
    if code() == 0x30 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: a recent file");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let mut path = [0u8; 0x104];
            if let Some(r) = tactics_load_recent(ts, c, &mut path, &mut password, &mut creator) {
                return Some(r);
            }
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x31 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: cancel");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if ts != 0 {
                tactics_event_free(ts);
            }
            return Some(TACTICS_EVENT_CLOSED);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x32 {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if arg() == 2 {
            covered!("tel14_3 answer: ok");
            write_u32(G_PENDING_COMMAND_ARG, 0);
            let row = tactics_captain_row(ts);
            if tactics_calls::slot_pitch_position(ts, row, core::ptr::null_mut()) == 0 {
                display_method(0x8a84a0, ts); // Tactics_Screen::auto_select_captain
            }
            display_method(0x8a80d0, ts); // Tactics_Screen::apply_changes
            if ts != 0 {
                tactics_event_free(ts);
            }
            return Some(TACTICS_EVENT_CLOSED);
        }
        write_u32(G_PENDING_COMMAND_ARG, 0);
    }
    if code() == 0x33 {
        let yes = arg() == 2;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        if yes {
            covered!("tel14_3 answer: save before leaving");
            let name = if read_u32(ts + t::HAS_FILE) != 0 { ts + t::FORMATION } else { TACTICS_NO_TEXT };
            let folder = tactics_folder();
            tactics_file_dialog(c, x::SAVE_FORMATION, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_TCT, name, TACTICS_ON_FILE, 0x2e);
            return Some(TACTICS_EVENT_DONE);
        }
    }
    None
}

/// Loads the recent file of parameter 6 (`HUMAN_MANAGER::get_recent_file(index, path, 0x104, 0, 0, 1)`): a file
/// locked by another manager's password or that does not load is a confirm box (done). None when it loaded.
unsafe fn tactics_load_recent(ts: usize, c: (u32, u32), path: &mut [u8; 0x104], password: &mut [u8; 0x34], creator: &mut [u8; 200]) -> Option<i32> {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    let recent: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x5fc760usize);
    recent(read_usize(G_HUMAN_MANAGER), get_screen_param(6) as u32, path.as_mut_ptr(), 0x104, 0, 0, 1);
    let p = path.as_ptr() as usize;
    if tactics_locked_by_other(p, password, creator) {
        let text = s(JOB_TEXT, x::LOCKED_BY, p, creator.as_ptr(), 0, c.0, c.1, 0);
        tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
        return Some(TACTICS_EVENT_DONE);
    }
    if tactics_load(ts, p, 0) == 0 {
        let text = s(JOB_TEXT, x::NOT_LOADED, p, 0, c.0, c.1, 0);
        tactics_confirm(c, x::LOAD_FORMATION, x::UNABLE_TO_LOAD, text, 0, 0);
        return Some(TACTICS_EVENT_DONE);
    }
    None
}

/// The Tactics menu and its neighbours (ids 1 to 0xd and 0x22): 1 Team Instructions (a copy of the object on the
/// Team Instructions screen, the read-only builder without the original), 0x22 Penalty Takers (the club squad
/// screen), 2 Set To Default, 3 Load, 4 Load Preset, 5 Save, 6 Save As, 7 Save As Preset, 8 Save For, 9 Delete,
/// 0xa Unlock, 0xb Lock, 0xc a recent file and 0xd a View column. An unsaved formation asks first (2, 3, 0xc);
/// Lock and Unlock ask to save. None for the other ids.
unsafe fn tactics_event_menu(ts: usize, id: u16, item: i16, c: (u32, u32)) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_event_texts as x;
    use tactics_screen as t;
    let s = tactics_sprintf();
    let value = || tel26_item_value(item);
    match id {
        1 => {
            covered!("tel14_3 team instructions");
            let copy = tactics_screen_copy(ts);
            let mut owner = ts;
            if read_u32(copy + t::READ_ONLY) == 0 {
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_TEAM_INSTRUCTIONS_FN, TACTICS_TEAM_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) == 0 {
                    return Some(TACTICS_EVENT_DONE);
                }
            } else {
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_TEAM_INSTRUCTIONS_RO_FN, TACTICS_TEAM_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) == 0 {
                    return Some(TACTICS_EVENT_DONE);
                }
                owner = 0;
            }
            set_screen_param(0, copy);
            set_screen_param(1, owner);
            Some(TACTICS_EVENT_DONE)
        }
        0x22 => {
            covered!("tel14_3 penalty takers");
            let club = get_screen_param(1);
            let reserve = get_screen_param(3);
            let f: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x453d60usize);
            f(club, reserve, 0, 0, 1);
            Some(TACTICS_EVENT_DONE)
        }
        2 => {
            covered!("tel14_3 set to default");
            set_screen_param(9, value() as usize);
            if read_u32(ts + t::SAVED) != 0 {
                tactics_set_preset(ts, get_screen_param(9));
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_DEFAULT, ts + t::FORMATION, 1, c.0, c.1, 0x34);
            tactics_confirm(c, x::SET_TO_DEFAULT, x::NOT_SAVED, text, 1, 0x34);
            Some(TACTICS_EVENT_DONE)
        }
        3 => {
            covered!("tel14_3 load");
            if read_u32(ts + t::SAVED) != 0 {
                let folder = tactics_folder();
                tactics_file_dialog(c, x::LOAD_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2d);
                return Some(TACTICS_EVENT_DONE);
            }
            let text = s(JOB_TEXT, x::ASK_LOAD, ts + t::FORMATION, 1, c.0, c.1, 0x2c);
            tactics_confirm(c, x::LOAD_FORMATION, x::NOT_SAVED, text, 1, 0x2c);
            Some(TACTICS_EVENT_DONE)
        }
        4 => {
            covered!("tel14_3 load preset");
            let folder = tactics_preset_folder(TACTICS_PRESET_LOAD_PATH);
            tactics_file_dialog(c, x::LOAD_PRESET_FORMATION, x::SELECT_FILE, 0, folder, TACTICS_EXT_PCT, TACTICS_NO_TEXT, TACTICS_ON_PRESET, 0x36);
            Some(TACTICS_EVENT_DONE)
        }
        5 => {
            covered!("tel14_3 save");
            let mut path = [0u8; 0x104];
            let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
            let folder = tactics_folder();
            exe::crt_makepath(path.as_mut_ptr(), TACTICS_NO_TEXT, folder, ts + t::FORMATION, TACTICS_EXT_TCT);
            let p = path.as_ptr() as usize;
            let text = if tactics_locked_by_other(p, &mut password, &mut creator) {
                s(JOB_TEXT, x::LOCKED_BY_SAVE, p, creator.as_ptr(), 0, c.0, c.1, 0)
            } else {
                if tactics_save(ts, p, get_screen_param(5)) != 0 {
                    return Some(TACTICS_EVENT_REDRAW);
                }
                s(JOB_TEXT, x::NOT_SAVED_FILE, p, 0, c.0, c.1, 0)
            };
            tactics_confirm(c, x::SAVE_FORMATION, x::UNABLE_TO_SAVE, text, 0, 0);
            Some(TACTICS_EVENT_DONE)
        }
        6 | 8 => {
            covered!("tel14_3 save as / save for");
            set_screen_param(5, if id == 6 { 0 } else { value() as usize });
            let name = if read_u32(ts + t::HAS_FILE) != 0 { ts + t::FORMATION } else { TACTICS_NO_TEXT };
            let folder = tactics_folder();
            tactics_file_dialog(c, x::SAVE_FORMATION, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_TCT, name, TACTICS_ON_FILE, 0x2e);
            Some(TACTICS_EVENT_DONE)
        }
        7 => {
            covered!("tel14_3 save as preset");
            set_screen_param(5, 0);
            let folder = tactics_preset_folder(TACTICS_PRESET_SAVE_PATH);
            tactics_file_dialog(c, x::SAVE_AS_PRESET, x::ENTER_FILE_NAME, 1, folder, TACTICS_EXT_PCT, TACTICS_NO_TEXT, TACTICS_ON_PRESET, 0x35);
            Some(TACTICS_EVENT_DONE)
        }
        9 => {
            covered!("tel14_3 delete");
            let folder = tactics_folder();
            tactics_file_dialog(c, x::DELETE_FORMATION, x::SELECT_FILE, 2, folder, TACTICS_EXT_TCT, 0, TACTICS_ON_FILE, 0x2f);
            Some(TACTICS_EVENT_DONE)
        }
        0xa => {
            covered!("tel14_3 unlock");
            if read_u32(ts + t::READ_ONLY) == 0 && read_u8(ts + t::LOCKED) != 0 {
                tactics_event_save_undo(ts);
                write_u8(ts + t::LOCKED, 0);
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            if read_u32(ts + f::LOCK_SAVED) == 0 {
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_SAVE_UNLOCK, 1, c.0, c.1, 0x33);
            tactics_confirm(c, x::UNLOCK_FORMATION, x::NOT_SAVED, text, 1, 0x33);
            Some(TACTICS_EVENT_DONE)
        }
        0xb => {
            covered!("tel14_3 lock");
            if read_u32(ts + t::READ_ONLY) == 0 && read_u8(ts + t::LOCKED) == 0 && read_u8(tactics_calls::password(0)) != 0 {
                tactics_event_save_undo(ts);
                tel26_copy_text((ts + t::LOCKED) as *mut u8, tactics_calls::password(0) as *const u8);
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            if read_u32(ts + f::LOCK_SAVED) != 0 {
                return Some(TACTICS_EVENT_REDRAW);
            }
            let text = s(JOB_TEXT, x::ASK_SAVE_LOCK, 1, c.0, c.1, 0x33);
            tactics_confirm(c, x::LOCK_FORMATION, x::NOT_SAVED, text, 1, 0x33);
            Some(TACTICS_EVENT_DONE)
        }
        0xc => {
            covered!("tel14_3 recent file");
            set_screen_param(6, value() as usize);
            if read_u32(ts + t::SAVED) != 0 {
                let mut path = [0u8; 0x104];
                let (mut password, mut creator) = ([0u8; 0x34], [0u8; 200]);
                return Some(tactics_load_recent(ts, c, &mut path, &mut password, &mut creator).unwrap_or(TACTICS_EVENT_REDRAW));
            }
            let text = s(JOB_TEXT, x::ASK_LOAD, ts + t::FORMATION, 1, c.0, c.1, 0x30);
            tactics_confirm(c, x::LOAD_FORMATION, x::NOT_SAVED, text, 1, 0x30);
            Some(TACTICS_EVENT_DONE)
        }
        0xd => {
            covered!("tel14_3 view column");
            set_screen_param(10, value() as usize);
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// Last Match (id 0xe): the club's fixtures of the last 8 weeks (24 for a national team) from the fixture manager;
/// the latest played one (an entry whose byte +0x43 is not negative) opens its tactics read-only
/// (`tactics_screen_open` with the match record) when the match statistics still have it; otherwise a box: no
/// tactics for the last match, or no recent matches (the reserve and 'B' squad texts with the club name as title).
unsafe fn tactics_event_last_match(c: (u32, u32)) -> i32 {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    covered!("tel14_3 last match");
    let mut list = [0u8; 0x20];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let club = get_screen_param(1);
    let reserve = get_screen_param(3);
    let weeks = if exe::database_is_national_team(club) != 0 { 0x18 } else { 8 };
    let today_year = read_u16(G_CURRENT_DATE_YEAR) as u32;
    let (mut d1, mut d2) = ([0u8; 8], [0u8; 8]);
    let year = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut d1, weeks) + 2) as u32;
    let day = read_u16(exe::date_sub_weeks(G_CURRENT_DATE, &mut d2, weeks)) as u32;
    // ponytail: the callee reads only the low words of the four date arguments (the original leaves stack and
    // register leftovers in the high halves)
    exe::fixture_manager_find_fixtures(read_usize(G_FIXTURE_MANAGER), day, year, read_u16(G_CURRENT_DATE) as u32, today_year, read_u32(club), l,
        u32::MAX, 0xffff_fffe, (reserve == 0) as u32);
    let mut entry = 0usize;
    if read_u16(l) != 0 {
        write_u32(l + 0xa, read_u32(l + 6));
        loop {
            let e = exe::llist_get_prev(l);
            if e == 0 {
                break;
            }
            if read_i8(e + 0x43) >= 0 {
                entry = e;
                break;
            }
        }
    }
    let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
    let h = hb.as_mut_ptr() as usize;
    let tbuf = tb.as_mut_ptr() as usize;
    let (heading, text_fmt, title_fmt) = if entry != 0 {
        let found = exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(entry + 8), read_i32(entry + 0xc), read_i32(entry + 0x10),
            read_u16(entry + 0x2a) as i32, read_u16(entry + 0x28) as i32);
        if found != 0 {
            tactics_screen_open(club, reserve as u32, 0, 0, read_u32(entry + 8), 0);
            exe::drop_llist(l);
            return TACTICS_EVENT_DONE;
        }
        if reserve == 0 {
            (x::NO_TACTICS, x::NO_TACTICS_MATCH, 0)
        } else if exe::database_is_national_team(club) != 0 {
            (x::NO_TACTICS, x::NO_TACTICS_B, x::CLUB_B_SQUAD)
        } else {
            (x::NO_TACTICS, x::NO_TACTICS_RESERVE, x::CLUB_RESERVES)
        }
    } else if reserve == 0 {
        (x::NO_RECENT, x::NO_RECENT_MATCHES, 0)
    } else if exe::database_is_national_team(club) != 0 {
        (x::NO_RECENT, x::NO_RECENT_B, x::CLUB_B_SQUAD)
    } else {
        (x::NO_RECENT, x::NO_RECENT_RESERVE, x::CLUB_RESERVES)
    };
    let text = s(JOB_TEXT, text_fmt, 0, c.0, c.1, 0);
    let hd = s(h, heading, text, 0, c.0, c.1, 0);
    let title = if title_fmt == 0 {
        club + 4
    } else {
        let g = read_i8(club + 0x52) as i32;
        s(tbuf, title_fmt, g, g, club + 0x38, hd, text, 0, c.0, c.1, 0)
    };
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    f(title, hd, text, 0, c.0, c.1, 0);
    exe::drop_llist(l);
    TACTICS_EVENT_DONE
}

/// The Edit menu and the tabs (ids 0xf to 0x15): 0xf Undo, 0x10 Copy (the zone shown), 0x11 Copy All (the phase),
/// 0x12 Paste, 0x13 Reflect (the zone to the other flank, then shown), 0x14 Show or Hide Runs and 0x15 a tab
/// (Overview / With Ball / Without Ball; another value is the error box of line 0x147a). Copy and Reflect do nothing
/// in the overview. Redraw. None for the other ids.
unsafe fn tactics_event_edit(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let view = read_u8(ts + t::VIEW);
    match id {
        0xf => {
            covered!("tel14_3 undo");
            if read_u32(ts + t::CAN_UNDO) != 0 {
                tactics_event_restore_undo(ts);
                write_u32(ts + t::CAN_UNDO, 0);
            }
        }
        0x10 => {
            covered!("tel14_3 copy");
            if view != 1 {
                // Formation::get_zone_coordinates(phase, zone, out)
                tactics_calls::thiscall3(0x5b3f90, fm, (view == 2) as u32, read_u32(ts + t::ZONE), (ts + f::COPIED_ZONE) as u32);
                write_u8(ts + t::CAN_PASTE, 1);
            }
        }
        0x11 => {
            covered!("tel14_3 copy all");
            if view != 1 {
                // Formation::get_phase_coordinates(phase, out)
                tactics_calls::thiscall2(0x5b42a0, fm, (view == 2) as u32, (ts + f::COPIED_PHASE) as u32);
                write_u8(ts + t::CAN_PASTE, 2);
            }
        }
        0x12 => {
            covered!("tel14_3 paste");
            display_method(0x8a9cb0, ts); // Tactics_Screen::paste_positions
        }
        0x13 => {
            covered!("tel14_3 reflect");
            if read_u32(ts + t::READ_ONLY) == 0 && view != 1 {
                tactics_event_save_undo(ts);
                // Formation::mirror_zone_coordinates(phase, zone)
                tactics_calls::thiscall2(0x5b47a0, fm, (view == 2) as u32, read_u32(ts + t::ZONE));
                write_u32(ts + f::CHANGES_MADE, 1);
                write_i16(ts + t::ZONE, 2i16.wrapping_sub(read_i16(ts + t::ZONE)));
            }
        }
        0x14 => {
            covered!("tel14_3 show runs");
            write_u32(ts + t::SHOW_RUNS, (read_u32(ts + t::SHOW_RUNS) == 0) as u32);
        }
        0x15 => {
            covered!("tel14_3 tab");
            let tab = tel26_item_value(item) as u8;
            if item != -1 && (tab == 1 || tab == 2 || tab == 3) {
                write_u8(ts + f::PREVIOUS_VIEW, view);
                write_u8(ts + t::VIEW, tab);
                write_u32(ts + t::HOVER_ZONE, read_u32(ts + t::ZONE));
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x147a);
            }
        }
        _ => return None,
    }
    Some(TACTICS_EVENT_REDRAW)
}

/// The item of the drag and drop pair of `item` (-1 for none).
unsafe fn tactics_partner(item: i16) -> i16 {
    if item == -1 {
        return -1;
    }
    read_i16((current_human_display() as isize + item as isize * 0x18c + tactics_event_item::PARTNER as isize) as usize)
}

/// The player list (ids 0x16 to 0x1b): 0x16 a row number clicked (a row on the pitch opens the player instructions
/// screen with a copy of the object), 0x17 a row dropped on the captain mark (the new captain), 0x18 a row dropped
/// on a penalty place (set the taker; another place dropped there swaps the two), 0x19 two rows swapped, 0x1a a
/// player instruction (bit index and slot in the value) and 0x1b a marking type. Read-only screens change nothing;
/// a change saves the undo copy and marks the changes. None for the other ids and the drops that do not apply.
unsafe fn tactics_event_list(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let value = tel26_item_value(item);
    let partner = tactics_partner(item);
    let partner_id = if partner != -1 { tel26_item_id(partner) } else { 0 };
    match id {
        0x16 => {
            let row = value as u8 as i8;
            if tactics_calls::slot_pitch_position(ts, row, core::ptr::null_mut()) != 0 {
                covered!("tel14_3 player instructions");
                let copy = tactics_screen_copy(ts);
                if exe::screenmanager_open_screen(G_SCREEN_MANAGER, TACTICS_PLAYER_INSTRUCTIONS_FN, TACTICS_PLAYER_INSTRUCTIONS_EVENT_FN, 1, 0usize, 0) != 0 {
                    set_screen_param(0, ts);
                    set_screen_param(1, row as i32 as usize);
                    set_screen_param(2, copy);
                }
                return Some(TACTICS_EVENT_DONE);
            }
            None
        }
        0x17 if partner != -1 && partner_id == 0x16 => {
            let dragged = tel26_item_value(partner) as u8 as i8;
            if dragged == tactics_captain_row(ts) {
                return None;
            }
            covered!("tel14_3 new captain");
            if tactics_calls::slot_player(ts, dragged) == 0 {
                return Some(0);
            }
            let p = tactics_calls::slot_player(ts, dragged);
            if p == 0 {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x14b5);
            } else if read_u32(ts + t::READ_ONLY) == 0 && read_u32(p) != read_u32(ts + t::CAPTAIN) {
                tactics_event_save_undo(ts);
                write_u32(ts + t::CAPTAIN, read_u32(p));
                write_u32(ts + f::CHANGES_MADE, 1);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        0x18 if partner != -1 && (partner_id == 0x16 || partner_id == 0x18) => {
            let a = value as u8 as i8;
            let b = tel26_item_value(partner) as u8 as i8;
            if partner_id == 0x16 {
                let taker = if (0..=10).contains(&a) {
                    read_i8(ts + t::PENALTY_ORDER + a as usize)
                } else {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x16b9);
                    -1
                };
                if b != taker && tactics_calls::slot_pitch_position(ts, b, core::ptr::null_mut()) != 0 {
                    covered!("tel14_3 penalty taker");
                    // Tactics_Screen::set_penalty_taker(order, row)
                    tactics_calls::thiscall2(0x8a9120, ts, a as u8 as u32, b as u8 as u32);
                    return Some(TACTICS_EVENT_REDRAW);
                }
                None
            } else if a != b {
                covered!("tel14_3 swap penalty takers");
                // Tactics_Screen::swap_penalty_takers(a, b)
                tactics_calls::thiscall2(0x8a9270, ts, a as u8 as u32, b as u8 as u32);
                Some(TACTICS_EVENT_REDRAW)
            } else {
                None
            }
        }
        0x19 if partner != -1 && partner_id == 0x19 => {
            let other = tel26_item_value(partner);
            if value as u8 == other as u8 {
                return None;
            }
            covered!("tel14_3 swap rows");
            // Tactics_Screen::swap_slots(row a, row b, opponent)
            tactics_calls::thiscall3(0x8ab7e0, ts, value, other, 0);
            Some(TACTICS_EVENT_REDRAW)
        }
        0x1a | 0x1b => {
            let slot = ((value as i32 >> 6) as u8) & 0x1f;
            let mask: unsafe extern "C" fn(u32) -> u64 = core::mem::transmute(0x8ade80usize);
            let m = mask(value & 0xffff_ff3f);
            let (lo, hi) = (m as u32, (m >> 32) as u32);
            if id == 0x1a {
                covered!("tel14_3 player instruction");
                if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_player_instruction(slot, flag)
                    if tactics_calls::thiscall3(0x5b5290, fm, slot as u32, lo, hi) == 0 {
                        tactics_event_restore_undo(ts);
                        write_u32(ts + t::CAN_UNDO, 0);
                    } else {
                        write_u32(ts + f::CHANGES_MADE, 1);
                    }
                }
            } else if slot <= 10 {
                covered!("tel14_3 marking");
                if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_marking_type(slot, flag)
                    tactics_calls::thiscall2(0x5b57d0, fm, slot as u32, lo);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x1a25);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// `(int) sqrt(n)` on the x87 like the original (fild, fsqrt, the game's `_ftol` 0x945640).
unsafe fn tactics_sqrt(n: i32) -> i32 {
    let mut m = [n as u32, 0x945640, 0];
    core::arch::asm!(
        "fild dword ptr [edi]",
        "fsqrt",
        "call dword ptr [edi + 4]",
        "mov dword ptr [edi + 8], eax",
        in("edi") m.as_mut_ptr(),
        clobber_abi("C"),
    );
    m[2] as i32
}

/// A display item's rectangle at `field` (x1, y1, x2, y2).
unsafe fn tactics_item_rect(item: i16, field: usize) -> [i32; 4] {
    let a = (current_human_display() as isize + item as isize * 0x18c + field as isize) as usize;
    [read_i32(a), read_i32(a + 4), read_i32(a + 8), read_i32(a + 12)]
}

/// The pitch position of the overview list nearest to the dropped circle `d` in the pitch rectangle `p`: for each
/// position its pitch point (`get_pitch_point_for_position_flags`, wide when the line has a central slot), the
/// distance from the circle's centre (truncated square root). Returns (distance, position), 800 when none is nearer.
unsafe fn tactics_nearest_position(ts: usize, p: &[i32; 4], d: &[i32; 4]) -> (i32, u16) {
    use tactics_screen as t;
    let (mut best, mut pos) = (800i32, 0u16);
    let half_h = (d[3] - d[1]) / 2;
    let half_w = (d[2] - d[0]) / 2;
    let mut k = TACTICS_PITCH_POSITIONS;
    while read_i16(k) != -1 {
        let position = read_u16(k);
        let players = if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = tactics_calls::central_in_line(ts + t::FORMATION, position, players);
        let (mut x, mut y) = (0i32, 0i32);
        tactics_calls::pitch_point(position, p, &mut x, &mut y, wide);
        let dy = d[1].wrapping_sub(y).wrapping_add(half_h);
        let dx = d[0].wrapping_sub(x).wrapping_add(half_w);
        let dist = tactics_sqrt(dx.wrapping_mul(dx).wrapping_add(dy.wrapping_mul(dy)));
        if dist < best {
            best = dist;
            pos = position;
        }
        k += 2;
    }
    (best, pos)
}

/// The pitch (ids 0x1d to 0x1f): 0x1d a circle dropped on the pitch: in the overview at the nearest position (the
/// two slots swapped when another slot has it, else the slot moved there), in the zone views on the cell of the
/// 27 x 36 grid it overlaps most (the coordinates swapped with the slot there, else the slot's coordinate set);
/// 0x1e a circle's drop target: the slot's extra position set to the nearest; 0x1f a zone clicked (x = value / 100,
/// y = value % 100; out of range is the error box of line 0x15fa). None for the other ids and the drops that do
/// not apply.
unsafe fn tactics_event_pitch(ts: usize, id: u16, item: i16) -> Option<i32> {
    use tactics_event_fields as f;
    use tactics_screen as t;
    let fm = ts + t::FORMATION;
    let partner = tactics_partner(item);
    match id {
        0x1d | 0x1e if partner != -1 && tel26_item_id(partner) == 0x1c => {
            let slot = tel26_item_value(item) as u8 as i8;
            let d = tactics_item_rect(item, tactics_event_item::DROP_RECT);
            let p = tactics_item_rect(partner, tactics_event_item::RECT);
            if id == 0x1e {
                let (best, pos) = tactics_nearest_position(ts, &p, &d);
                if best >= 800 {
                    return None;
                }
                covered!("tel14_3 extra position");
                if !(0..=10).contains(&slot) {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x199c);
                } else if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    // Formation::set_slot_position(slot, position, extra)
                    tactics_calls::thiscall3(0x5b2ba0, fm, slot as u8 as u32, pos as u32, 1);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
                return Some(TACTICS_EVENT_REDRAW);
            }
            if read_u8(ts + t::VIEW) == 1 {
                let (best, pos) = tactics_nearest_position(ts, &p, &d);
                if best >= 800 {
                    return None;
                }
                let there = tactics_calls::slot_by_position(fm, pos);
                if there > -1 && there != slot {
                    covered!("tel14_3 swap slots on the pitch");
                    if !((0..=10).contains(&slot) && (0..=10).contains(&there)) {
                        tel26_error_box(TACTICS_SCREENS_CPP, 0x1977);
                    } else if read_u32(ts + t::READ_ONLY) == 0 {
                        tactics_event_save_undo(ts);
                        // Formation::swap_slots(a, b)
                        tactics_calls::thiscall2(0x5b2f30, fm, slot as u8 as u32, there as u8 as u32);
                        write_u32(ts + f::CHANGES_MADE, 1);
                    }
                    return Some(TACTICS_EVENT_REDRAW);
                }
                if there != -1 {
                    return None;
                }
                covered!("tel14_3 move a slot on the pitch");
                if !(0..=10).contains(&slot) {
                    tel26_error_box(TACTICS_SCREENS_CPP, 0x199c);
                } else if read_u32(ts + t::READ_ONLY) == 0 {
                    tactics_event_save_undo(ts);
                    tactics_calls::thiscall3(0x5b2ba0, fm, slot as u8 as u32, pos as u32, 0);
                    write_u32(ts + f::CHANGES_MADE, 1);
                }
                return Some(TACTICS_EVENT_REDRAW);
            }
            let (w, h) = (p[2] - p[0], p[3] - p[1]);
            let rect_overlap: unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(0x5e7f00usize);
            let (mut best, mut cell) = (0i32, 0u32);
            let mut acc_i = 0i32;
            for i in 0..0x1bu32 {
                let x2 = p[0] + (w + acc_i) / 0x1b;
                let x1 = p[0] + acc_i / 0x1b;
                let mut acc_j = 0i32;
                for j in 0..0x24u32 {
                    let area = rect_overlap(d[0], d[1], d[2], d[3], x1, p[3] - (h + acc_j) / 0x24, x2, p[3] - acc_j / 0x24);
                    if area > best {
                        cell = i | (j << 16);
                        best = area;
                    }
                    acc_j += h;
                }
                acc_i += w;
            }
            if best == 0 {
                return None;
            }
            let view = read_u8(ts + t::VIEW);
            // Formation::find_slot_at_coordinate(phase, zone, coordinate)
            let there = tactics_calls::thiscall3(0x5b4410, fm, (view == 2) as u32, read_u32(ts + t::ZONE), cell) as u8 as i8;
            if there > -1 {
                if there == slot {
                    return None;
                }
                covered!("tel14_3 swap coordinates");
                // Tactics_Screen::swap_slot_coordinates(a, b)
                tactics_calls::thiscall2(0x8a9d70, ts, slot as u8 as u32, there as u8 as u32);
                return Some(TACTICS_EVENT_REDRAW);
            }
            if there != -1 {
                return None;
            }
            covered!("tel14_3 set a coordinate");
            // Tactics_Screen::set_formation_slot_coordinate(slot, coordinate)
            tactics_calls::thiscall2(0x8a9e90, ts, slot as u8 as u32, cell);
            Some(TACTICS_EVENT_REDRAW)
        }
        0x1f => {
            covered!("tel14_3 zone");
            let v = tel26_item_value(item) as i32;
            let (zx, zy) = ((v / 100) as i16, (v % 100) as i16);
            if (0..=2).contains(&zx) && (0..=3).contains(&zy) {
                let old = read_u32(ts + t::ZONE);
                write_u32(ts + t::ZONE, zx as u16 as u32 | (zy as u16 as u32) << 16);
                write_u32(ts + t::HOVER_ZONE, old);
                write_u8(ts + f::PREVIOUS_VIEW, read_u8(ts + t::VIEW));
            } else {
                tel26_error_box(TACTICS_SCREENS_CPP, 0x15fa);
            }
            Some(TACTICS_EVENT_REDRAW)
        }
        _ => None,
    }
}

/// Cancel and Ok (ids 0x20, 0x21): Cancel with changes made asks first, else frees the object (closed); Ok asks
/// when leaving the match needs a confirmation (`build_return_to_match_text`, the box with the text object), else
/// applies the changes and frees the object (closed). None for the other ids.
unsafe fn tactics_event_leave(ts: usize, id: u16, text: usize, c: (u32, u32)) -> Option<i32> {
    use tactics_event_texts as x;
    let s = tactics_sprintf();
    match id {
        0x20 => {
            covered!("tel14_3 cancel");
            if read_u32(ts + tactics_event_fields::CHANGES_MADE) != 0 {
                let t = s(JOB_TEXT, x::ASK_CANCEL, 1, c.0, c.1, 0x31);
                tactics_confirm(c, x::TACTICS, x::CHANGES_MADE, t, 1, 0x31);
                return Some(TACTICS_EVENT_DONE);
            }
        }
        0x21 => {
            covered!("tel14_3 ok");
            if display_method_ret(0x8a8630, ts, text as u32) != 0 {
                let (mut hb, mut tb) = ([0u8; 200], [0u8; 200]);
                let h = s(hb.as_mut_ptr() as usize, x::PLEASE_CONFIRM, text, 1, c.0, c.1, 0x32);
                let t = s(tb.as_mut_ptr() as usize, x::TACTICS, h, text, 1, c.0, c.1, 0x32);
                let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x5919b0usize);
                f(t, h, text, 1, c.0, c.1, 0x32);
                return Some(TACTICS_EVENT_DONE);
            }
            display_method(0x8a80d0, ts); // Tactics_Screen::apply_changes
        }
        _ => return None,
    }
    if ts != 0 {
        tactics_screen_free_inline(ts);
    }
    Some(TACTICS_EVENT_CLOSED)
}

/// A thiscall with one argument that returns eax.
unsafe fn display_method_ret(f: usize, this: usize, a: u32) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(f);
    g(this, a)
}

/// The events of the tactics screen.
///
/// The Tactics_Screen of parameter 0 is built when missing (from parameters 1, 2, 3, 4, 7 and 8) and the club's
/// kit colours taken for the boxes. With a dialog answer to dispatch (`[0xb77c84]`), the answers first
/// (`tactics_event_dialog_answer`). Then the item's id (0 or item -1: the default command id): the Tactics menu
/// (`tactics_event_menu`), Last Match (`tactics_event_last_match`), Edit and the tabs (`tactics_event_edit`), the
/// player list (`tactics_event_list`), the pitch (`tactics_event_pitch`) and Cancel / Ok (`tactics_event_leave`).
/// Anything else goes to the menu bar (`menubar_handle_command`, then `menubar_handle_object_link`), else 0.
/// Results: -4 done, -9 the screen closed, -0xb redraw. A local Text holds the leave-the-match confirmation.
/// Original: `tactics_screen_handle_event` (0x89f710, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_handle_event(item: u32) -> i32 {
    let mut text = [0u32; 4];
    let tx = text.as_mut_ptr() as usize;
    display_method1(FN_TEXT_TEXT, tx, 0);
    let r = tactics_screen_handle_event_body(item, tx);
    display_method(FN_TEXT_DESTRUCTOR_TEXT, tx);
    r
}

unsafe fn tactics_screen_handle_event_body(cell: u32, text: usize) -> i32 {
    let item = cell as i16;
    let mut ts = get_screen_param(0);
    if ts == 0 {
        let p = exe::crt_operator_new(tactics_screen::SIZE);
        if p != 0 {
            let penalty = get_screen_param(8) as u32;
            let record = get_screen_param(7) as u32;
            let arg4 = get_screen_param(4) as u32;
            let reserve = get_screen_param(3) as u32;
            let m = get_screen_param(2);
            let club = get_screen_param(1);
            ts = tactics_screen_construct(p, club, m, reserve, arg4, record, penalty);
        }
        set_screen_param(0, ts);
    }
    let (mut a, mut b) = (0u32, 0u32);
    exe::database_get_club_kit_colours(read_usize(ts), read_u8(ts + tactics_screen::KIT) as u32, &mut a, &mut b);
    let c = (a, b);
    if read_u32(TACTICS_PENDING_DISPATCH) != 0 {
        if let Some(r) = tactics_event_dialog_answer(ts, c) {
            return r;
        }
    }
    let id = tel26_item_id(item);
    if id == 0xe {
        return tactics_event_last_match(c);
    }
    if let Some(r) = tactics_event_menu(ts, id, item, c) {
        return r;
    }
    if let Some(r) = tactics_event_edit(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_list(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_pitch(ts, id, item) {
        return r;
    }
    if let Some(r) = tactics_event_leave(ts, id, text, c) {
        return r;
    }
    let menubar: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_COMMAND);
    let r = menubar(cell);
    if r != 0 {
        return r;
    }
    let link: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_MENUBAR_HANDLE_OBJECT_LINK);
    link(cell)
}

/// The events that only write the view state of the screen object (side by side): a View column, Copy, Copy All,
/// Show Runs, a valid tab and a valid zone.
unsafe fn tactics_event_view_only(id: u16, item: i16) -> bool {
    let v = tel26_item_value(item);
    match id {
        0xd | 0x10 | 0x11 | 0x14 => true,
        0x15 => item != -1 && matches!(v as u8, 1..=3),
        0x1f => (0..=2).contains(&(v as i32 / 100)) && (0..=3).contains(&(v as i32 % 100)),
        _ => false,
    }
}

// simplification: the events that write the tactics or files, open dialogs or screens, or free the object do not
// run twice: original only, checked afterwards against the plan (Undo leaves the working data equal to the undo
// copy and CAN_UNDO clear; Cancel without changes closes; the handler's own results are -4, -9, -0xb or 0).
/// Comparer for `tactics_screen_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_screen_handle_event(item: u32) -> i32 {
    use tactics_screen as t;
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let ts = get_screen_param(0);
    let pending = read_u32(TACTICS_PENDING_DISPATCH) != 0 && matches!(read_u32(G_PENDING_COMMAND_CODE), 0x2c..=0x36);
    let id = tel26_item_id(item as i16);
    if ts != 0 && !pending && tactics_event_view_only(id, item as i16) {
        let mut reg = screen_param_regions();
        reg.push((ts, t::SIZE as usize));
        reg.push((G_PENDING_COMMAND_CODE, 8));
        let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&reg, || o(item), || tactics_screen_handle_event(item));
        SCOREBOARD.record(equal && ro == rr, || {
            let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
                let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
                Some(format!("{:#x}+{:#x}", a, k))
            }).collect();
            format!("tactics screen event (id {:#x}): original={} rust={} differences at {:?}", id, ro, rr, dif)
        });
        return ro;
    }
    let undo = ts != 0 && !pending && id == 0xf && read_u32(ts + t::CAN_UNDO) != 0;
    let cancel = ts != 0 && !pending && id == 0x20 && read_u32(ts + tactics_event_fields::CHANGES_MADE) == 0;
    let own = ts != 0 && !pending && (1..=0x22).contains(&id) && id != 0x1c;
    let r = o(item);
    if undo {
        let same = core::slice::from_raw_parts((ts + t::FORMATION) as *const u8, tactics_event_fields::TEAM_DATA_SIZE)
            == core::slice::from_raw_parts((ts + t::UNDO_DATA) as *const u8, tactics_event_fields::TEAM_DATA_SIZE);
        SCOREBOARD.record(same && read_u32(ts + t::CAN_UNDO) == 0 && r == TACTICS_EVENT_REDRAW, || format!("tactics screen undo: result {}, working data restored {}, can undo {}", r, same, read_u32(ts + t::CAN_UNDO)));
    } else if cancel {
        SCOREBOARD.record(r == TACTICS_EVENT_CLOSED, || format!("tactics screen cancel without changes: result {}, plan -9", r));
    } else if own && matches!(id, 0xf..=0x15 | 0x1a | 0x1b | 0x1f) {
        SCOREBOARD.record(r == TACTICS_EVENT_REDRAW, || format!("tactics screen event (id {:#x}): result {}, plan -0xb", id, r));
    }
    r
}
