// ---------- batch %%LEVA%%: the player and staff search screen (search_screens.cpp, part 2): its answers ----------

/// The answer's return codes: -4 a new screen or a box opened, -9 the screen closes, -11 redraw.
const FINDER_OPENED: i32 = -4;
const FINDER_CLOSED: i32 = -9;
const FINDER_REDRAW: i32 = -11;
/// [0xb77c84]: a box or a file dialog has answered (the pending command and its argument are set).
const FINDER_PENDING: usize = 0xb77c84;
/// The default nation (id) of the club pick when the human has none.
const FINDER_DEFAULT_NATION: usize = 0x9cf2e4;
/// `confirm_push(title, heading, text, buttons, text colour, colour, code)` (0x591840) and `open_file_dialog(title,
/// heading, mode, directory, extension, default name, on ok, colour a, colour b, code, 0, 0, 0, 0)` (0x58f500), cdecl.
const FINDER_CONFIRM: usize = 0x591840;
const FINDER_FILE_DIALOG: usize = 0x58f500;
/// The file dialog's checks of each list kind (short list, quick search, staff search, scout search).
const FINDER_FILE_CHECKS: [usize; 4] = [0x7bd010, 0x7bd0d0, 0x7bd190, 0x7bd250];
/// `HUMAN_MANAGER::get_folder_path(manager, kind)` (0x5fc5b0, thiscall ret 8).
const FINDER_FOLDER_PATH: usize = 0x5fc5b0;
/// SearchEditSession methods (search_edit_session.cpp), thiscall: revert the current list (0x8042e0), save all
/// (0x804450), set the current list (0x8044a0, ret 4), unlock / lock (0x804750 / 0x804810), set the scout's scope
/// (0x8048e0, ret 0x14), reset the scout fields (0x8049a0), remove a player (0x804ab0, ret 4), clear the results
/// (0x804b60, ret 4), load a file (0x804cd0, ret 4), save the current list (0x804e10, ret 8), the destructor (0x813140).
const FINDER_REVERT: usize = 0x8042e0;
const FINDER_SAVE_ALL: usize = 0x804450;
const FINDER_SET_CURRENT: usize = 0x8044a0;
const FINDER_UNLOCK: usize = 0x804750;
const FINDER_LOCK: usize = 0x804810;
const FINDER_SET_SCOPE: usize = 0x8048e0;
const FINDER_RESET_SCOUT: usize = 0x8049a0;
const FINDER_REMOVE_PLAYER: usize = 0x804ab0;
const FINDER_CLEAR_RESULTS: usize = 0x804b60;
const FINDER_LOAD_FILE: usize = 0x804cd0;
const FINDER_SAVE_FILE: usize = 0x804e10;
const FINDER_SESSION_DROP: usize = 0x813140;
/// `PLAYER_SEARCH::set_assigned_club(club)` (0x7b3d50, ret 4); `search_screens_open_filters_screen(session)`
/// (0x813210, cdecl); `open_choose_friendly_teams_screen(tab, continent, nation, kind)` (0x5a0650, cdecl);
/// `notes_open_staff_note_screen(person)` (0x7961b0, cdecl); `open_club_squad_screen(club, reserves, restore, alt,
/// locked)` (0x453d60, cdecl); `staff_screens_open_staff_profile(person, tab, slot2, keep, mode)` (0x8702a0, cdecl).
const FINDER_SET_ASSIGNED_CLUB: usize = 0x7b3d50;
const FINDER_OPEN_FILTERS: usize = 0x813210;
const FINDER_OPEN_CLUB_PICK: usize = 0x5a0650;
const FINDER_OPEN_NOTE: usize = 0x7961b0;
const FINDER_OPEN_SQUAD: usize = 0x453d60;
const FINDER_OPEN_PROFILE: usize = 0x8702a0;
/// The header of a saved search file (`player_search_read_file_header`): the creator at +0x6b, the password at +0x134.
const FINDER_HEADER_CREATOR: usize = 0x6b;
const FINDER_HEADER_PASSWORD: usize = 0x134;

/// The texts of the boxes, one per list kind (short list, quick search, staff search, scout search).
mod finder_text {
    pub(crate) const KIND: [usize; 4] = [0xa7dd94, 0xa7e960, 0xa7dd74, 0xa7e9c8]; // "Short List", "Quick Search", ..
    pub(crate) const LOAD: [usize; 4] = [0xa7fc74, 0xa7fc60, 0xa7fc4c, 0xa7fc38]; // "Load Short List" ..
    pub(crate) const SAVE: [usize; 4] = [0xa7f9c8, 0xa7f940, 0xa7f8b8, 0xa7f830]; // "Save Short List" ..
    pub(crate) const DELETE: [usize; 4] = [0xa7f6fc, 0xa7f6e8, 0xa7f6d4, 0xa7f6c0]; // "Delete Short List" ..
    pub(crate) const LOCK: [usize; 4] = [0xa7ec2c, 0xa7ebc0, 0xa7eb54, 0xa7eae8]; // "Lock Short List" ..
    pub(crate) const UNLOCK: [usize; 4] = [0xa7ede0, 0xa7ed70, 0xa7ed00, 0xa7ec90]; // "Unlock Short List" ..
    pub(crate) const NOT_SAVED: [usize; 4] = [0xa7ef1c, 0xa7eeb4, 0xa7ee4c, 0xa7f29c]; // "Short List Not Saved" ..
    pub(crate) const LOCKED_LOAD: [usize; 4] = [0xa7fbf8, 0xa7fba4, 0xa7fb60, 0xa7fb1c]; // "The short list "<%s>" has been locked by <%s>."
    pub(crate) const NOT_LOADED: [usize; 4] = [0xa7fae8, 0xa7fab4, 0xa7fa80, 0xa7fa4c]; // ".. could not be loaded."
    pub(crate) const LOCKED_SAVE: [usize; 4] = [0xa7f9d8, 0xa7f954, 0xa7f8cc, 0xa7f844]; // "The file .. Please save the .. with a different file name."
    pub(crate) const NOT_SAVED_FILE: [usize; 4] = [0xa7f7fc, 0xa7f7c8, 0xa7f794, 0xa7f760]; // ".. could not be saved."
    pub(crate) const LOCKED_DELETE: [usize; 4] = [0xa7f724; 4]; // "The file "<%s>" has been locked by <%s>."
    pub(crate) const NOT_DELETED: [usize; 4] = [0xa7f68c, 0xa7f654, 0xa7f61c, 0xa7f5e4]; // ".. could not be deleted."
    pub(crate) const LOAD_ANOTHER: [usize; 4] = [0xa7ef34, 0xa7eecc, 0xa7ee64, 0xa7e8b8]; // "The short list "<%s>" has not been saved. Load another short list ?"
    pub(crate) const SAVE_TO_LOCK: [usize; 4] = [0xa7ec3c, 0xa7ebd4, 0xa7eb68, 0xa7eafc]; // ".. must be saved before the locking takes effect .."
    pub(crate) const SAVE_TO_UNLOCK: [usize; 4] = [0xa7edf4, 0xa7ed84, 0xa7ed14, 0xa7eca4];
    pub(crate) const RESET: [usize; 4] = [0xa7eaa8, 0xa7ea58, 0xa7ea18, 0xa7e9d8]; // "Changes have been made to the .. Reset all changes ?"
    pub(crate) const CLEAR: [usize; 4] = [0xa7e99c, 0xa7e970, 0xa7e934, 0xa7e908]; // "Delete all players from the .. ?"
    pub(crate) const DELETE_PLAYER: [usize; 4] = [0xa7f4d8, 0xa7f4a8, 0xa7f478, 0xa7f448]; // "Delete <%s> from the .. ?"
    pub(crate) const CANNOT_DELETE: [usize; 4] = [0xa7f5ac, 0xa7f574, 0xa7f53c, 0xa7f504]; // "<%s> cannot be deleted from the .."
    pub(crate) const UNABLE_LOAD: usize = 0xa7fbe8;
    pub(crate) const UNABLE_SAVE: usize = 0x99081c;
    pub(crate) const UNABLE_DELETE: usize = 0xa7f710;
    pub(crate) const PLEASE_CONFIRM: usize = 0x9909b8;
    pub(crate) const CHANGES_MADE: usize = 0xa7ea98;
    pub(crate) const PLAYER_SEARCH: usize = 0xa7dd84;
    pub(crate) const SCOUT_NOT_SAVED: usize = 0xa7f29c;
    pub(crate) const ASSIGN_SCOUT: usize = 0x9918c4;
    pub(crate) const RECALL_SCOUT: usize = 0xa7e7fc;
    pub(crate) const SELECT_FILE: usize = 0xa7fc84;
    pub(crate) const ENTER_FILE_NAME: usize = 0x9907f4;
    pub(crate) const EXTENSION: usize = 0xa7fc90; // ".pls"
}

/// What the answer's pieces share: the session, the human, the box colours (params 1 and 2), params 3 (lock on save),
/// 5 (pick mode), 6 (the person to delete) and 7 (pages), the clicked item, and the buffers of the boxes (the list's
/// name, the file header, the file name, the box title and heading, the scout's name, the place and the path).
struct FinderInput {
    s: usize,
    mgr: usize,
    c1: u32,
    c2: u32,
    lock_owner: usize,
    pick: usize,
    pending_person: usize,
    pages: i16,
    item: i16,
    name: [u8; 0xc8],
    header: [u8; 0x170],
    file: [u8; 0x104],
    title: [u8; 0xc8],
    heading: [u8; 0xc8],
    scout: [u8; 0xc8],
    place: [u8; 0xc8],
    path: [u8; 0x104],
}

impl FinderInput {
    fn np(&self) -> usize {
        self.name.as_ptr() as usize
    }
    fn sp(&self) -> usize {
        self.scout.as_ptr() as usize
    }
    fn pp(&self) -> usize {
        self.place.as_ptr() as usize
    }
    fn fp(&self) -> usize {
        self.file.as_ptr() as usize
    }
    fn creator(&self) -> usize {
        self.header.as_ptr() as usize + FINDER_HEADER_CREATOR
    }
    /// The kind of the current list as an index of the texts (1, 2, 3; anything else is a scout search).
    unsafe fn kind(&self) -> usize {
        match finder_type(self.s) {
            1 => 0,
            2 => 1,
            3 => 2,
            _ => 3,
        }
    }
    /// The current list's PLAYER_SEARCH.
    unsafe fn search(&self) -> usize {
        finder_current(self.s) + finder_list::SEARCH
    }
    /// The current list was saved to a file.
    unsafe fn saved(&self) -> bool {
        read_u32(finder_current(self.s) + finder_list::SAVED) != 0
    }
    /// The clicked item's id: its id when it has one, otherwise the current command id.
    unsafe fn id(&self) -> u16 {
        let id = if self.item != -1 { read_u16(finder_item_at(self.item) + display_item::ID) } else { 0 };
        if id != 0 { id } else { read_u16(G_CURRENT_COMMAND_ID) }
    }
    /// The clicked item's value (0 for none).
    unsafe fn value(&self) -> usize {
        if self.item != -1 { read_usize((finder_display() as isize + display::ITEM_VALUE as isize + self.item as isize * display_list::ITEM_SIZE as isize) as usize) } else { 0 }
    }
    /// A box (`confirm_push`): the screens' buffer as the text, the heading and the title translated into their
    /// buffers (in that order), `buttons` (1 = yes/no) and the pending `code` its answer sets; the answer opens it.
    unsafe fn confirm(&mut self, title: usize, heading: usize, buttons: u32, code: u32) -> i32 {
        let lang = finder_lang();
        let h = lang(self.heading.as_mut_ptr() as usize, heading) as usize;
        let t = lang(self.title.as_mut_ptr() as usize, title) as usize;
        let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_CONFIRM);
        f(t, h, JOB_TEXT, buttons, self.c1, self.c2, code);
        FINDER_OPENED
    }
    /// The file dialog of the list's kind (`open_file_dialog`: the human's folder of kind 2, ".pls", the kind's check),
    /// in `mode` (0 load, 1 save, 2 delete) with the default name and the pending `code`.
    unsafe fn file_dialog(&mut self, title: usize, heading: usize, mode: u32, default: usize, code: u32) -> i32 {
        let lang = finder_lang();
        let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FINDER_FOLDER_PATH);
        let dir = folder(read_usize(G_HUMAN_MANAGER), 0, 2);
        let h = lang(self.heading.as_mut_ptr() as usize, heading) as usize;
        let t = lang(self.title.as_mut_ptr() as usize, title) as usize;
        let f: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32) =
            core::mem::transmute(FINDER_FILE_DIALOG);
        f(t, h, mode, dir, finder_text::EXTENSION, default, FINDER_FILE_CHECKS[self.kind()], self.c1, self.c2, code, 0, 0, 0, 0);
        FINDER_OPENED
    }
    /// The file at `path` is locked by another human: its header (kind 4) has a password that is not the current
    /// human's.
    unsafe fn locked(&mut self, path: usize) -> bool {
        let read: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FINDER_READ_FILE_HEADER);
        read(path, self.header.as_mut_ptr() as usize, 4);
        let pw = self.header.as_ptr() as usize + FINDER_HEADER_PASSWORD;
        read_u8(pw) != 0 && !finder_same_text(finder_password(0), pw)
    }
    /// The file box of the list's kind: the file name copied, the text of the kind with the file (and the creator),
    /// "Unable To .." and the kind's title, no buttons.
    unsafe fn file_box(&mut self, path: usize, texts: [usize; 4], creator: bool, heading: usize, titles: [usize; 4]) -> i32 {
        finder_copy(&mut self.file, path);
        let k = self.kind();
        if creator {
            finder_lang()(JOB_TEXT, texts[k], self.fp(), self.creator());
        } else {
            finder_lang()(JOB_TEXT, texts[k], self.fp());
        }
        self.confirm(titles[k], heading, 0, 0)
    }
    /// The pending file name is freed and the argument cleared (after a file box opened by a dialog's answer).
    unsafe fn drop_path(&self, path: usize) -> i32 {
        exe::crt_operator_delete(path);
        write_u32(G_PENDING_COMMAND_ARG, 0);
        FINDER_OPENED
    }
    /// The current list's lock toggled (unlocked when locked, locked otherwise).
    unsafe fn toggle_lock(&self) {
        let f = if read_u8(finder_current(self.s) + finder_list::LOCKED) != 0 { FINDER_UNLOCK } else { FINDER_LOCK };
        finder_call0(f, self.s);
    }
    /// `SearchEditSession::set_search_scope(nation, region, competition, youth, next opposition)`.
    unsafe fn scope(&self, nation: u32, region: u32, comp: u32, youth: u32, next: u32) {
        let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_SET_SCOPE);
        f(self.s, nation, region, comp, youth, next);
    }
    /// The scout's name (style 1) into the scout buffer.
    unsafe fn scout_name(&mut self, scout: usize, style: u32) {
        exe::database_format_staff_name(scout, self.sp(), 0xc8, style);
    }
}

/// Item `k` of the current slot's Display.
unsafe fn finder_item_at(k: i16) -> usize {
    display_item_at(finder_display(), k)
}
/// A thiscall without arguments returning eax.
unsafe fn finder_call0(f: usize, this: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    g(this)
}
/// The two C strings are equal (strcmp).
unsafe fn finder_same_text(a: usize, b: usize) -> bool {
    let mut i = 0;
    loop {
        let (x, y) = (read_u8(a + i), read_u8(b + i));
        if x != y {
            return false;
        }
        if x == 0 {
            return true;
        }
        i += 1;
    }
}

/// The club pick of the scout's "Club" assignment: the human's club's nation, else his nation when it is a playable
/// key nation (+0x11c), else the first playable key nation, else the default nation; the club pick screen (tab 4)
/// of that nation's continent opens.
unsafe fn finder_open_club_pick(st: &FinderInput) -> i32 {
    let mut n = finder_nation(read_i32(FINDER_DEFAULT_NATION));
    let club = read_usize(st.mgr + staff::CLUB_JOB);
    let own = read_usize(st.mgr + staff::NATION);
    if club != 0 {
        n = read_usize(club + club::NATION);
    } else if own != 0 && read_u8(own + 0x11c) != 0 {
        n = own;
    } else {
        for k in 0..0x23i16 {
            let e = read_usize((G_LEAGUE_TABLE as isize + k as isize * 0x48) as usize);
            if e != 0 && read_u8(e + 0x11c) != 0 {
                n = e;
                break;
            }
        }
    }
    let f: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(FINDER_OPEN_CLUB_PICK);
    f(4, read_usize(n + nation::CONTINENT), n, 0);
    FINDER_OPENED
}

/// The answers to the boxes and file dialogs the screen opened (the pending command, cleared; with the argument 2 the
/// box was accepted): 0x19, 0x1a, 0x1b, 0x3f, 0x45 the scout's nation, region, competition, youth players and next
/// opposition; 0x40 the club (the club pick opens); 0x1c the recall; 0x1d load another (the load dialog opens); 0x1e,
/// 0x1f, 0x20 the file chosen to load, save (with the lock toggle of param 11 and the lock owner of param 3) and delete
/// (a box when the file is locked by another human or fails); 0x21 save before the lock changes (the save dialog
/// opens); 0x22 reset; 0x23 clear; 0x24 load the recent file of param 4; 0x25 cancel the changes (the screen
/// closes); 0x26 delete the person of param 6. `Some(code)` when the answer returns.
unsafe fn finder_pending(st: &mut FinderInput) -> Option<i32> {
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let arg = || read_u32(G_PENDING_COMMAND_ARG);
    let clear_code = || write_u32(G_PENDING_COMMAND_CODE, 0);
    let clear_arg = || write_u32(G_PENDING_COMMAND_ARG, 0);
    if code() == 0x19 {
        clear_code();
        if arg() == 2 {
            st.scope(get_screen_param(8) as u32, u32::MAX, u32::MAX, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x40 {
        clear_code();
        if arg() == 2 {
            covered!("tel19_2 search answer club scope");
            get_scout(st.search());
            finder_call1(FINDER_SET_ASSIGNED_CLUB, st.search(), (-2i32) as u32);
            clear_arg();
            return Some(finder_open_club_pick(st));
        }
        clear_arg();
    }
    if code() == 0x3f {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, u32::MAX, 1, 0);
        }
        clear_arg();
    }
    if code() == 0x1a {
        clear_code();
        if arg() == 2 {
            let r = get_screen_param(9) as u8;
            st.scope(u32::MAX, r as u32, u32::MAX, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x45 {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, u32::MAX, 0, 1);
        }
        clear_arg();
    }
    if code() == 0x1b {
        clear_code();
        if arg() == 2 {
            st.scope(u32::MAX, u32::MAX, get_screen_param(0xa) as u32, 0, 0);
        }
        clear_arg();
    }
    if code() == 0x1c {
        clear_code();
        if arg() == 2 {
            finder_call0(FINDER_RESET_SCOUT, st.s);
        }
        clear_arg();
    }
    if code() == 0x1d {
        clear_code();
        if arg() == 2 {
            clear_arg();
            let k = st.kind();
            return Some(st.file_dialog(finder_text::LOAD[k], finder_text::SELECT_FILE, 0, 0, 0x1e));
        }
        clear_arg();
    }
    if code() == 0x1e {
        let path = arg() as usize;
        clear_code();
        if path != 0 {
            covered!("tel19_2 search answer load file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_LOAD, true, finder_text::UNABLE_LOAD, finder_text::LOAD);
                return Some(st.drop_path(path));
            }
            if finder_call1(FINDER_LOAD_FILE, st.s, path as u32) == 0 {
                st.file_box(path, finder_text::NOT_LOADED, false, finder_text::UNABLE_LOAD, finder_text::LOAD);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x1f {
        clear_code();
        let toggle = get_screen_param(0xb);
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 0usize, 0);
        let path = arg() as usize;
        if path != 0 {
            covered!("tel19_2 search answer save file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_SAVE, true, finder_text::UNABLE_SAVE, finder_text::SAVE);
                return Some(st.drop_path(path));
            }
            if toggle != 0 {
                st.toggle_lock();
            }
            let save: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FINDER_SAVE_FILE);
            if save(st.s, path, st.lock_owner) == 0 {
                if toggle != 0 {
                    st.toggle_lock();
                }
                st.file_box(path, finder_text::NOT_SAVED_FILE, false, finder_text::UNABLE_SAVE, finder_text::SAVE);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x20 {
        let path = arg() as usize;
        clear_code();
        if path != 0 {
            covered!("tel19_2 search answer delete file");
            if st.locked(path) {
                st.file_box(path, finder_text::LOCKED_DELETE, true, finder_text::UNABLE_DELETE, finder_text::DELETE);
                return Some(st.drop_path(path));
            }
            if exe::crt_remove(path as *const u8) != 0 {
                st.file_box(path, finder_text::NOT_DELETED, false, finder_text::UNABLE_DELETE, finder_text::DELETE);
                return Some(st.drop_path(path));
            }
            exe::crt_operator_delete(path);
            clear_arg();
        }
    }
    if code() == 0x21 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            return Some(finder_save_dialog(st));
        }
        exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 0usize, 0);
        clear_arg();
    }
    if code() == 0x22 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call0(FINDER_REVERT, st.s);
        }
        clear_arg();
    }
    if code() == 0x23 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call1(FINDER_CLEAR_RESULTS, st.s, 1);
        }
        clear_arg();
    }
    if code() == 0x24 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            if let Some(r) = finder_load_recent(st) {
                return Some(r);
            }
        }
        clear_arg();
    }
    if code() == 0x25 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            return Some(finder_close(st));
        }
        clear_arg();
    }
    if code() == 0x26 {
        clear_code();
        if arg() == 2 {
            clear_arg();
            finder_call1(FINDER_REMOVE_PLAYER, st.s, st.pending_person as u32);
        }
        clear_arg();
    }
    None
}

/// The save dialog of the list's kind ("Enter File Name", mode 1, pending 0x1f): the default name is the list's name,
/// except for a short list not from a file (empty).
unsafe fn finder_save_dialog(st: &mut FinderInput) -> i32 {
    let k = st.kind();
    let default = if k == 0 && read_u32(finder_current(st.s) + finder_list::FROM_FILE) == 0 { 0xad9c64 } else { st.np() };
    st.file_dialog(finder_text::SAVE[k], finder_text::ENTER_FILE_NAME, 1, default, 0x1f)
}

/// Loads the recent file of param 4 (kind 2) into the current list: a box when it is locked by another human or does
/// not load; `None` when it loaded.
unsafe fn finder_load_recent(st: &mut FinderInput) -> Option<i32> {
    let recent: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FINDER_GET_RECENT_FILE);
    let path = st.path.as_mut_ptr() as usize;
    recent(read_usize(G_HUMAN_MANAGER), get_screen_param(4) as u32, path, 0x104, 0, 0, 2);
    if st.locked(path) {
        return Some(st.file_box(path, finder_text::LOCKED_LOAD, true, finder_text::UNABLE_LOAD, finder_text::LOAD));
    }
    if finder_call1(FINDER_LOAD_FILE, st.s, path as u32) == 0 {
        return Some(st.file_box(path, finder_text::NOT_LOADED, false, finder_text::UNABLE_LOAD, finder_text::LOAD));
    }
    None
}

/// The screen closes: the session destroyed and freed, the scout manager without a session; -9.
unsafe fn finder_close(st: &FinderInput) -> i32 {
    if st.s != 0 {
        finder_call0(FINDER_SESSION_DROP, st.s);
        exe::crt_operator_delete(st.s);
    }
    set_edit_session(read_usize(G_SCOUT_MANAGER), 0, 0);
    FINDER_CLOSED
}

/// The scout's assignment confirmations (ids 4, 0x1f, 0x20, 0x22, 5, 6): the scout's name and the box asking to assign
/// him ("not saved" first when the scout search was never saved); the club assignment of a saved search opens the
/// club pick directly. `None` when the id is not one of them.
unsafe fn finder_assign(st: &mut FinderInput, id: u16) -> Option<i32> {
    let lang = finder_lang();
    let saved = |st: &FinderInput| unsafe { st.saved() };
    match id {
        4 => {
            covered!("tel19_2 search assign nation");
            let nation = st.value();
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 8, nation, 0);
            let n = finder_nation(nation as i32);
            finder_copy(&mut st.place, n + 4);
            st.scout_name(scout, 1);
            let g = read_i8(finder_nation(nation as i32) + nation::GENDER_NAME) as i32;
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f3e4usize, g, g, st.np(), st.sp(), st.pp()); // "The scout search .. not saved. Assign scout .. to {}<nation>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x19))
            } else {
                lang(JOB_TEXT, 0xa7f3b0usize, g, g, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - nation>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x19))
            }
        }
        0x1f => {
            let scout = get_scout(st.search());
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f348usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to find youth players ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x3f))
            } else {
                lang(JOB_TEXT, 0xa7f314usize, st.sp()); // "Assign scout <%s> to find youth players ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x3f))
            }
        }
        0x20 => {
            let scout = get_scout(st.search());
            if !saved(st) {
                st.scout_name(scout, 1);
                lang(JOB_TEXT, 0xa7f2b4usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to watch a club ?"
                return Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x40));
            }
            covered!("tel19_2 search assign club");
            finder_call1(FINDER_SET_ASSIGNED_CLUB, st.search(), (-2i32) as u32);
            Some(finder_open_club_pick(st))
        }
        0x22 => {
            let scout = get_scout(st.search());
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f238usize, st.np(), st.sp()); // ".. not saved. Assign scout .. to next opposition ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x45))
            } else {
                lang(JOB_TEXT, 0xa7f208usize, st.sp()); // "Assign scout <%s> to next opposition ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x45))
            }
        }
        5 => {
            let region = st.value() as u8;
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 9, region as usize, 0);
            get_region_name(region as u32, st.pp(), 0xc8, u32::MAX);
            let form: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FINDER_REGION_ARTICLE_FORM);
            let f = form(region as u32) as i8 as i32;
            st.scout_name(scout, 1);
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f1a4usize, f, f, st.np(), st.sp(), st.pp()); // ".. not saved. Assign scout .. to {}<region>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x1a))
            } else {
                lang(JOB_TEXT, 0xa7f170usize, f, f, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - region>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1a))
            }
        }
        6 => {
            let comp = st.value();
            let scout = get_scout(st.search());
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xa, comp, 0);
            let c = finder_comp(comp as i32);
            finder_copy(&mut st.place, c + 4);
            st.scout_name(scout, 1);
            let g = read_i8(finder_comp(comp as i32) + 0x37) as i32;
            if !saved(st) {
                lang(JOB_TEXT, 0xa7f104usize, g, g, st.np(), st.sp(), st.pp()); // ".. not saved. Assign scout .. to {}<competition>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::SCOUT_NOT_SAVED, 1, 0x1b))
            } else {
                lang(JOB_TEXT, 0xa7f0ccusize, g, g, st.sp(), st.pp()); // "Assign scout <%s> to {}<%s - competition>{} ?"
                Some(st.confirm(finder_text::ASSIGN_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1b))
            }
        }
        _ => None,
    }
}

/// The recall box (id 7) for the scout's current assignment (club, youth players, next opposition, nation, region or
/// competition), pending 0x1c; `None` without an assignment.
unsafe fn finder_recall(st: &mut FinderInput) -> Option<i32> {
    let lang = finder_lang();
    let (mut club, mut nation, mut comp, mut youth, mut next) = (0u32, 0u32, 0u32, 0u32, 0u32);
    let mut region = [0u8; 4];
    if get_scout_assignment(st.search(), &mut club, &mut nation, region.as_mut_ptr(), &mut comp, &mut youth, &mut next) == 0 {
        return None;
    }
    covered!("tel19_2 search recall scout");
    let scout = get_scout(st.search());
    if club as i32 > -1 {
        let c = finder_club(club as i32);
        finder_copy(&mut st.place, c + club::SHORT_NAME);
        st.scout_name(scout, 1);
        let g = read_i8(finder_club(club as i32) + club::SHORT_GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7f098usize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - club>{} ?"
    } else if youth != 0 {
        // the nation's name is copied too (unused; the original reads it with the nation the assignment has, -1)
        let n = finder_nation(nation as i32);
        finder_copy(&mut st.place, n + 4);
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7f060usize, st.sp()); // "Recall scout <%s> from finding youth players ?"
    } else if next != 0 {
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7f020usize, st.sp()); // "Recall scout <%s> from researching next opposition ?"
    } else if nation as i32 > -1 {
        let n = finder_nation(nation as i32);
        finder_copy(&mut st.place, n + 4);
        st.scout_name(scout, 1);
        let g = read_i8(finder_nation(nation as i32) + nation::GENDER_NAME) as i32;
        lang(JOB_TEXT, 0xa7efecusize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - nation>{} ?"
    } else if region[0] as i8 > -1 {
        get_region_name(region[0] as u32, st.pp(), 0xc8, u32::MAX);
        let form: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FINDER_REGION_ARTICLE_FORM);
        let f = form(region[0] as u32) as i8 as i32;
        st.scout_name(scout, 1);
        lang(JOB_TEXT, 0xa7efb8usize, f, f, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - region>{} ?"
    } else {
        let c = finder_comp(comp as i32);
        finder_copy(&mut st.place, c + 4);
        st.scout_name(scout, 1);
        let g = read_i8(finder_comp(comp as i32) + 0x37) as i32;
        lang(JOB_TEXT, 0xa7ef80usize, g, g, st.sp(), st.pp()); // "Recall scout <%s> from {}<%s - competition>{} ?"
    }
    Some(st.confirm(finder_text::RECALL_SCOUT, finder_text::PLEASE_CONFIRM, 1, 0x1c))
}

/// The list commands: 8 load (the "not saved" box first, pending 0x1d, else the load dialog, 0x1e); 9 save (to the
/// human's folder under the list's name; a box when the file is locked by another human or fails); 0xa save as and 0xb
/// save for (param 3 = the other human for 0xb; the save dialog); 0xc delete (the delete dialog, 0x20); 0xd unlock and
/// 0xe lock (param 11 = 1 and the box to save first, 0x21); 0xf reset (the box when there are changes, 0x22; else the
/// list reverted at once); 0x10 clear (0x23); 0x11 a recent file (param 4; the "not saved" box first, 0x24).
unsafe fn finder_list_command(st: &mut FinderInput, id: u16) -> Option<i32> {
    let lang = finder_lang();
    let k = st.kind();
    match id {
        8 => {
            if !st.saved() {
                lang(JOB_TEXT, finder_text::LOAD_ANOTHER[k], st.np());
                return Some(st.confirm(finder_text::LOAD[k], finder_text::NOT_SAVED[k], 1, 0x1d));
            }
            Some(st.file_dialog(finder_text::LOAD[k], finder_text::SELECT_FILE, 0, 0, 0x1e))
        }
        9 => {
            covered!("tel19_2 search save");
            let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FINDER_FOLDER_PATH);
            let dir = folder(read_usize(G_HUMAN_MANAGER), 0, 2);
            let path = st.path.as_mut_ptr() as usize;
            exe::crt_makepath(st.path.as_mut_ptr(), 0xad9c64usize, dir, st.np(), finder_text::EXTENSION);
            if st.locked(path) {
                return Some(st.file_box(path, finder_text::LOCKED_SAVE, true, finder_text::UNABLE_SAVE, finder_text::SAVE));
            }
            let save: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FINDER_SAVE_FILE);
            if save(st.s, path, 0) == 0 {
                return Some(st.file_box(path, finder_text::NOT_SAVED_FILE, false, finder_text::UNABLE_SAVE, finder_text::SAVE));
            }
            Some(FINDER_REDRAW)
        }
        0xa | 0xb => {
            let other = if id == 0xb { st.value() } else { 0 };
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 3, other, 0);
            Some(finder_save_dialog(st))
        }
        0xc => Some(st.file_dialog(finder_text::DELETE[k], finder_text::SELECT_FILE, 2, 0, 0x20)),
        0xd | 0xe => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 0xb, 1usize, 0);
            let (text, title) = if id == 0xd { (finder_text::SAVE_TO_UNLOCK, finder_text::UNLOCK) } else { (finder_text::SAVE_TO_LOCK, finder_text::LOCK) };
            lang(JOB_TEXT, text[k]);
            Some(st.confirm(title[k], finder_text::NOT_SAVED[k], 1, 0x21))
        }
        0xf => {
            if read_u32((st.s as isize + read_i8(st.s) as isize * 4 + finder_session::RESET_AVAILABLE as isize) as usize) != 0 {
                lang(JOB_TEXT, finder_text::RESET[k]);
                return Some(st.confirm(finder_text::KIND[k], finder_text::CHANGES_MADE, 1, 0x22));
            }
            finder_call0(FINDER_REVERT, st.s);
            Some(FINDER_REDRAW)
        }
        0x10 => {
            lang(JOB_TEXT, finder_text::CLEAR[k]);
            Some(st.confirm(finder_text::KIND[k], finder_text::PLEASE_CONFIRM, 1, 0x23))
        }
        0x11 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 4, st.value(), 0);
            if !st.saved() {
                lang(JOB_TEXT, finder_text::LOAD_ANOTHER[k], st.np());
                return Some(st.confirm(finder_text::LOAD[k], finder_text::NOT_SAVED[k], 1, 0x24));
            }
            covered!("tel19_2 search recent file");
            Some(finder_load_recent(st).unwrap_or(FINDER_REDRAW))
        }
        _ => None,
    }
}

/// The view and sort commands: 0x12 a view (the main views 1..0x40 and Scout Report replace the main view, keeping
/// the attribute groups; the groups 0x80..0x1000 replace the group); 0x1e a column (the same column again flips the
/// direction; the results recomputed), 0x14 a statistic column and 0x15 an attribute column (the same one again flips
/// the direction); 0x17..0x1a the pages (-10, -1, +1, +10, within 1 and the page count). Returns -11.
unsafe fn finder_view_command(st: &mut FinderInput, id: u16) -> Option<i32> {
    let view = read_usize(FINDER_VIEW_FLAGS);
    match id {
        0x12 => {
            let v = st.value() as u32;
            match v {
                1 | 2 | 4 | 8 | 0x10 | 0x20 | 0x40 | 0x2000 => {
                    write_u32(view, read_u32(view) & 0xffffdf80);
                    write_u32(read_usize(FINDER_VIEW_FLAGS), read_u32(read_usize(FINDER_VIEW_FLAGS)) | v);
                }
                0x80 | 0x100 | 0x200 | 0x400 | 0x800 | 0x1000 => {
                    write_u32(view, read_u32(view) & 0xffffe07f);
                    write_u32(read_usize(FINDER_VIEW_FLAGS), read_u32(read_usize(FINDER_VIEW_FLAGS)) | v);
                }
                _ => {}
            }
            Some(FINDER_REDRAW)
        }
        0x1e => {
            let v = st.value() as u32;
            if v != 0 && v.is_power_of_two() && v <= 0x20000000 {
                write_u32(st.s + finder_session::CACHED, 0);
                let f = read_usize(FINDER_VIEW_FLAGS);
                if read_u32(f + 4) & v != 0 && read_u32(f + 0xa) == 0 {
                    write_u32(f + 0xa, 1);
                } else {
                    write_u32(f + 0xa, 0);
                }
                write_u32(read_usize(FINDER_VIEW_FLAGS) + 4, read_u32(read_usize(FINDER_VIEW_FLAGS) + 4) & 0xc0000000);
                let f = read_usize(FINDER_VIEW_FLAGS);
                write_u32(f + 4, read_u32(f + 4) | v);
            }
            Some(FINDER_REDRAW)
        }
        0x14 | 0x15 => {
            write_u32(st.s + finder_session::CACHED, 0);
            let (bit, at) = if id == 0x14 { (0x400u32, 8usize) } else { (0x800, 9) };
            let f = read_usize(FINDER_VIEW_FLAGS);
            if read_u32(f + 4) & bit != 0 && st.value() as u8 == read_u8(f + at) && read_u32(f + 0xa) == 0 {
                write_u32(f + 0xa, 1);
            } else {
                write_u32(f + 0xa, 0);
            }
            let f = read_usize(FINDER_VIEW_FLAGS);
            write_u32(f + 4, read_u32(f + 4) & 0xe0000000);
            let f = read_usize(FINDER_VIEW_FLAGS);
            write_u32(f + 4, read_u32(f + 4) | bit);
            write_u8(read_usize(FINDER_VIEW_FLAGS) + at, st.value() as u8);
            Some(FINDER_REDRAW)
        }
        0x17 | 0x18 => {
            let page = read_i16(view + 0xe) as i32 + if id == 0x17 { -0xa } else { -1 };
            write_i16(view + 0xe, if page > 1 { page as i16 } else { 1 });
            Some(FINDER_REDRAW)
        }
        0x19 | 0x1a => {
            let page = read_i16(view + 0xe) as i32 + if id == 0x19 { 1 } else { 0xa };
            let last = st.pages as i32;
            write_i16(view + 0xe, if page < last { page as i16 } else { last as i16 });
            Some(FINDER_REDRAW)
        }
        _ => None,
    }
}

/// Rule: answers the player and staff search screen.
///
/// First, when a box or a file dialog has answered ([`FINDER_PENDING`]), its pending command (`finder_pending`); the
/// list's name is copied for the boxes (also for any click). Then the clicked item's id (its id, or the current
/// command id): in pick mode (param 5) the pick button (0x1b) asks to delete the person from the list ("cannot be
/// deleted" when he is not in it; param 6 = the person, pending 0x26) and any other click leaves pick mode (-11);
/// otherwise 0x21 the person's note, 2 the human's squad, 3 another tab (-11), the scout's assignments
/// (`finder_assign`), 7 the recall (`finder_recall`), the list commands (`finder_list_command`), the view and sort
/// commands (`finder_view_command`), 0x13 the filters screen, 0x16 pick mode on/off (-11), 0x1b the person's profile,
/// 0x1c cancel (the box when the session has changes, pending 0x25; else the screen closes, -9), 0x1d Ok (every list
/// saved, the screen closes, -9); anything else goes to the menu bar (`menubar_handle_command`, then
/// `menubar_handle_object_link`). Returns -4 when a screen or a box opened, -9 when the screen closes, -11 to redraw.
/// Original: `search_screens_handle_player_search_input` (0x80eb00, search_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn search_screens_handle_player_search_input(item: u32) -> i32 {
    let mut st = FinderInput {
        s: 0,
        mgr: read_usize((G_SCREEN_RECORDS_MANAGER as isize + read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize * 0x300) as usize),
        c1: 0,
        c2: 0,
        lock_owner: 0,
        pick: 0,
        pending_person: 0,
        pages: 0,
        item: item as i16,
        name: [0; 0xc8],
        header: [0; 0x170],
        file: [0; 0x104],
        title: [0; 0xc8],
        heading: [0; 0xc8],
        scout: [0; 0xc8],
        place: [0; 0xc8],
        path: [0; 0x104],
    };
    st.s = get_screen_param(0);
    st.c1 = get_screen_param(1) as u32;
    st.c2 = get_screen_param(2) as u32;
    st.lock_owner = get_screen_param(3);
    st.pick = get_screen_param(5);
    st.pending_person = get_screen_param(6);
    st.pages = get_screen_param(7) as i16;
    let s = st.s;
    write_usize(FINDER_VIEW_FLAGS, finder_current(s) + finder_list::VIEW);
    if read_u32(FINDER_PENDING) != 0 || st.item != -1 {
        exe::at_9466d0(st.name.as_mut_ptr() as usize, (finder_current(s) + finder_list::NAME) as *const u8, 0xc8);
        st.name[0xc7] = 0;
    }
    if read_u32(FINDER_PENDING) != 0 {
        if let Some(r) = finder_pending(&mut st) {
            return r;
        }
    }
    let lang = finder_lang();
    if st.pick != 0 {
        if st.id() == 0x1b {
            covered!("tel19_2 search pick delete");
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, 0usize, 0);
            let p = st.value();
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 6, p, 0);
            let k = st.kind();
            if exe::player_search_contains_player(st.search(), p) != 0 {
                st.scout_name(p, 1);
                lang(JOB_TEXT, finder_text::DELETE_PLAYER[k], st.sp());
                return st.confirm(finder_text::PLAYER_SEARCH, finder_text::PLEASE_CONFIRM, 1, 0x26);
            }
            st.scout_name(p, 1);
            lang(JOB_TEXT, finder_text::CANNOT_DELETE[k], st.sp());
            return st.confirm(finder_text::PLAYER_SEARCH, finder_text::UNABLE_DELETE, 0, 0);
        }
        if st.item != -1 {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, 0usize, 0);
            return FINDER_REDRAW;
        }
    }
    let id = st.id();
    match id {
        0x21 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FINDER_OPEN_NOTE);
            f(st.value());
            return FINDER_OPENED;
        }
        2 => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_OPEN_SQUAD);
            f(read_usize(st.mgr + staff::CLUB_JOB), 0, 0, 1, 0);
            return FINDER_OPENED;
        }
        3 => {
            let v = st.value();
            if v as u8 != read_u8(s) {
                finder_call1(FINDER_SET_CURRENT, s, v as u32);
                return FINDER_REDRAW;
            }
        }
        _ => {}
    }
    if let Some(r) = finder_assign(&mut st, id) {
        return r;
    }
    if id == 7 {
        if let Some(r) = finder_recall(&mut st) {
            return r;
        }
    }
    let id = st.id();
    if let Some(r) = finder_list_command(&mut st, id) {
        return r;
    }
    if let Some(r) = finder_view_command(&mut st, id) {
        return r;
    }
    match id {
        0x13 => {
            let f: unsafe extern "C" fn(usize) = core::mem::transmute(FINDER_OPEN_FILTERS);
            f(s);
            FINDER_OPENED
        }
        0x16 => {
            exe::screenmanager_set_screen_param(G_SCREEN_MANAGER, 5, (st.pick == 0) as usize, 0);
            FINDER_REDRAW
        }
        0x1b => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(FINDER_OPEN_PROFILE);
            f(st.value(), 1, 1, 0, 0);
            FINDER_OPENED
        }
        0x1c => {
            if read_u32(s + 0x1b09) != 0 {
                lang(JOB_TEXT, 0xa7e874usize); // "Changes have been made to the player search. Cancel all changes ?"
                return st.confirm(finder_text::PLAYER_SEARCH, finder_text::CHANGES_MADE, 1, 0x25);
            }
            finder_close(&st)
        }
        0x1d => {
            covered!("tel19_2 search ok");
            finder_call0(FINDER_SAVE_ALL, s);
            finder_close(&st)
        }
        _ => {
            let r = exe::menubar_handle_command(item);
            if r != 0 {
                return r;
            }
            exe::menubar_handle_object_link(item)
        }
    }
}

/// Comparer for `search_screens_handle_player_search_input`.
///
/// The answers open screens, boxes and file dialogs, load and save files and free the session: only the original runs,
/// except for the view, sort, page and pick-mode commands without a pending answer, which only change the current
/// list's view flags, the "results cached" flag and parameter 5; those run side by side (the list's view fields, the
/// cache flag, the view global and the screen's parameters) and the return codes must match.
pub(crate) unsafe extern "C" fn compare_search_screens_handle_player_search_input(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let s = get_screen_param(0);
    let node = current_screen_node(G_SCREEN_MANAGER);
    let k = item as i16;
    let id = {
        let d = if k != -1 { read_u16(display_item_at(finder_display(), k) + display_item::ID) } else { 0 };
        if d != 0 { d } else { read_u16(G_CURRENT_COMMAND_ID) }
    };
    let simple = matches!(id, 0x12 | 0x14 | 0x15 | 0x16 | 0x17 | 0x18 | 0x19 | 0x1a | 0x1e);
    if s == 0 || node == 0 || read_u32(FINDER_PENDING) != 0 || get_screen_param(5) != 0 || !simple {
        // simplification: the other answers cannot run twice (screens, boxes, files); proven in Rust mode
        return o(item);
    }
    let view = finder_current(s) + finder_list::VIEW;
    let regions = [(view, 0x10), (s + finder_session::CACHED, 4), (FINDER_VIEW_FLAGS, 4),
        (node + screen_node_fields::PARAMS, screen_node_fields::PARAM_COUNT * 8)];
    let (ro, rr, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(item), || search_screens_handle_player_search_input(item));
    SCOREBOARD.record(same && ro == rr, || format!("player search screen answer (id {:#x}): returns {} / {}, differences in {:?}", id, ro, rr,
        first_differences(&mo, &mr)));
    ro
}
