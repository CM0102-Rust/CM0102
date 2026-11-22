// ---------- batch %%LEVA%%: the menu bar's commands and its periodic work (Menubar.cpp) ----------

/// The slot's u16 that is nonzero for a remote network player (screen manager +0x7d4 + 0x300 x slot) [inference: it
/// turns "Exit the game ?" into "Exit the network game ?", skips the auto save and the disk check, gives -5 and -12].
const MENUBAR_SLOT_REMOTE: usize = 0x7d4;
/// The Config fields read here: +4 the option flags, +0xd the auto save interval in days (i16), +0xf the save folder.
const MENUBAR_CONFIG_FLAGS: usize = 4;
const MENUBAR_CONFIG_AUTOSAVE_DAYS: usize = 0xd;
const MENUBAR_CONFIG_SAVE_FOLDER: usize = 0xf;
/// Config flag: save automatically every `MENUBAR_CONFIG_AUTOSAVE_DAYS` days.
const MENUBAR_CONFIG_AUTOSAVE: u32 = 0x2000;
/// Config flag: the bar's header shows the paused match instead of the date on both phases of the flashing.
const MENUBAR_CONFIG_MATCH_HEADER: u32 = 0x10000;
/// The save name buffer the file dialog edits and the routine that checks the typed name (`check_saved_game_file`).
const MENUBAR_SAVE_NAME: usize = 0xdd6ed4;
const MENUBAR_SAVE_NAME_CHECK: usize = 0x838e10;
/// The ".sav" extension.
const MENUBAR_SAVE_EXTENSION: usize = 0xa5aab8;
/// Below this many free MB of disk (or memory) the game stops with the disk full (no memory) box.
const MENUBAR_LOW_SPACE_MB: i32 = 0x19;

/// The answer codes of the boxes the menu bar opens (`G_PENDING_COMMAND_CODE`; the answer in `G_PENDING_COMMAND_ARG`:
/// 1 No, 2 Yes; for the save dialogs the typed path, allocated).
#[allow(dead_code)]
mod menubar_answer {
    pub(crate) const SAVE_GAME: u32 = 7;
    pub(crate) const GO_ON_HOLIDAY: u32 = 8; // [inference: the holiday screen's Yes]
    pub(crate) const EXIT_NETWORK_ON_HOLIDAY: u32 = 9;
    pub(crate) const RESERVE_TEAM: u32 = 0xa;
    pub(crate) const EXIT_GAME: u32 = 0xb;
    pub(crate) const RETURN_FROM_HOLIDAY: u32 = 0x2a;
    pub(crate) const SAVE_BEFORE_EXIT: u32 = 0x37;
    pub(crate) const AUTO_SAVE: u32 = 0x3a;
    pub(crate) const RESTART_GAME: u32 = 0x3d;
    pub(crate) const SAVE_BEFORE_RESTART: u32 = 0x3e;
    pub(crate) const NATIONAL_TEAMS: u32 = 0x44;
}

/// One run of the menu bar's periodic work. The port runs it on the slot's real record and the text buffer
/// (`MENUBAR_TEXT`). The comparer runs it `dry` before the original: on a copy of the record and a text buffer of its
/// own, with no call that changes the game or the screen (the widgets, the redraw, the saves, the boxes); an outcome that
/// needs one of them sets `unsure`. It also notes the clock thresholds the decisions read (`ms_marks`, `seconds`): the
/// prediction holds only if the original's run did not cross them.
struct MenubarCommandTick {
    dry: bool,
    rec: usize,
    text: usize,
    unsure: bool,
    ms_marks: [i32; 2],
    marks: usize,
    seconds: bool,
}

impl MenubarCommandTick {
    /// A threshold of `utils_get_time_ms` the run compared with.
    fn mark_ms(&mut self, at: i32) {
        if self.marks < self.ms_marks.len() {
            self.ms_marks[self.marks] = at;
            self.marks += 1;
        }
    }

    /// Dry: the outcome needs running; returns the value that stops the dry run.
    fn give_up(&mut self) -> i32 {
        self.unsure = true;
        0
    }
}

/// The menu bar record of the slot's manager: 0 without a manager or when he is not among the last 16 staff records
/// (`[manager] − G_STAFF_COUNT + 16`, signed, outside 0..16).
unsafe fn menubar_command_record() -> usize {
    let manager = read_usize(menubar_slot(MENUBAR_SLOT_MANAGER));
    if manager == 0 {
        return 0;
    }
    let k = read_u32(manager).wrapping_sub(read_u32(G_STAFF_COUNT)).wrapping_add(0x10) as i32;
    if !(0..0x10).contains(&k) {
        return 0;
    }
    MENUBAR_RECORDS + k as usize * MENUBAR_RECORD_SIZE
}

/// The slot's manager.
unsafe fn menubar_command_manager() -> usize {
    read_usize(menubar_slot(MENUBAR_SLOT_MANAGER))
}

/// The slot belongs to a remote network player (`MENUBAR_SLOT_REMOTE`).
unsafe fn menubar_command_remote() -> bool {
    read_u16(menubar_slot(MENUBAR_SLOT_REMOTE)) != 0
}

/// `get_heading_colour(flag)` (0x839590): the pale grey (1) or the red (0) colour, zero-extended as the original's AX.
unsafe fn menubar_command_colour(flag: u32) -> u32 {
    exe::get_heading_colour(flag) as u32
}

/// `ScreenManager::redraw_screen(all, 0)`, not in a dry run.
unsafe fn menubar_command_redraw(t: &MenubarCommandTick, all: u32) {
    if !t.dry {
        exe::screenmanager_redraw_screen(SCREEN_MANAGER, all, 0);
    }
}

/// `langlib_sprintf` with no arguments into the run's text buffer.
unsafe fn menubar_command_text(t: &MenubarCommandTick, format: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    text(t.text, format);
}

/// The two strings are equal (the original's byte loop).
unsafe fn menubar_command_same_text(a: usize, b: usize) -> bool {
    let mut k = 0;
    loop {
        let c = read_u8(a + k);
        if c != read_u8(b + k) {
            return false;
        }
        if c == 0 {
            return true;
        }
        k += 1;
    }
}

/// `strcpy(dst, src)` (the original's rep movs of the length + 1).
unsafe fn menubar_command_copy_text(dst: usize, src: usize) {
    let mut k = 0;
    loop {
        let c = read_u8(src + k);
        write_u8(dst + k, c);
        if c == 0 {
            return;
        }
        k += 1;
    }
}

/// The Continue item (`menubar_record::CONTINUE_WIDGET`): the text `format`, the style (only for the active button),
/// the flags, the text flags and the id, a redraw, and the new state.
unsafe fn menubar_command_set_continue(t: &MenubarCommandTick, format: usize, style: Option<u32>, flags: u32, text_flags: u32, id: u32, state: u8) {
    menubar_command_text(t, format);
    if !t.dry {
        let w = || read_u16(t.rec + menubar_record::CONTINUE_WIDGET) as u32;
        if let Some(style) = style {
            exe::screenmanager_set_widget_style(SCREEN_MANAGER, w(), style);
        }
        exe::screenmanager_set_widget_flags(SCREEN_MANAGER, w() as usize, flags);
        exe::screenmanager_set_widget_text_flags(SCREEN_MANAGER, w(), text_flags);
        exe::screenmanager_set_widget_text(SCREEN_MANAGER, w(), t.text);
        exe::screenmanager_set_widget_id(SCREEN_MANAGER, w(), id);
    }
    menubar_command_redraw(t, 0);
    write_u8(t.rec + menubar_record::CONTINUE_STATE, state);
}

/// The status item at the bottom (`menubar_record::STATUS_WIDGET`): the colour, the text, the id, a redraw, and the kind
/// with the counts of players waiting and in the game.
#[allow(clippy::too_many_arguments)]
unsafe fn menubar_command_set_status(t: &MenubarCommandTick, colour: usize, text: usize, id: u32, kind: u8, waiting: i32, total: i32) {
    if !t.dry {
        let w = || read_u16(t.rec + menubar_record::STATUS_WIDGET) as u32;
        exe::screenmanager_set_widget_text_colour(SCREEN_MANAGER, w(), colour_word(colour) as u32);
        exe::screenmanager_set_widget_text(SCREEN_MANAGER, w(), text);
        exe::screenmanager_set_widget_id(SCREEN_MANAGER, w(), id);
    }
    menubar_command_redraw(t, 0);
    write_u8(t.rec + menubar_record::STATUS_KIND, kind);
    write_i32(t.rec + menubar_record::WAITING, waiting);
    write_i32(t.rec + menubar_record::TOTAL, total);
}

/// `confirm_push(title, heading, message, buttons, fore, back, code)` (0x591840, cdecl): the confirm box (`buttons` 1 =
/// No and Yes, 0 = Ok); its answer comes back with `code` in `G_PENDING_COMMAND_CODE`.
#[allow(clippy::too_many_arguments)]
unsafe fn menubar_command_box(title: usize, heading: usize, message: usize, buttons: u32, fore: u32, back: u32, code: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    f(title, heading, message, buttons, fore, back, code);
}

/// "Please Confirm" (No / Yes) over the question in the text buffer, titled `title`, in the colours `fore` and `back`,
/// answer `code`. The heading is formatted with the box's arguments, as the original pushes them once for both calls.
unsafe fn menubar_command_please_confirm(title: usize, message: usize, fore: u32, back: u32, code: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut heading = [0u8; 200];
    let h = text(heading.as_mut_ptr() as usize, 0x9909b8, message, 1u32, fore, back, code) as u32 as usize;
    menubar_command_box(title, h, message, 1, fore, back, code);
}

/// The game's title (`get_game_title` 0x839570, "Championship Manager 2001/02"), the title of the boxes with no person.
unsafe fn menubar_command_game_title(heading: usize) -> usize {
    core::mem::transmute::<usize, MenubarCdecl1>(0x839570)(heading)
}

/// An Ok box titled with the game's title: the heading `heading` over the message in the text buffer (formatted with
/// `message_format`, the path first when there is one), in the pale grey and the 0xaebdcc colours, code 0.
unsafe fn menubar_command_ok_box(message_format: usize, path: Option<usize>, heading: usize) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (grey, back) = (colour_word(G_GUI_COLOUR_PALE_GREY) as u32, colour_word(0xaebdcc) as u32);
    let m = match path {
        Some(path) => text(MENUBAR_TEXT, message_format, path, 0u32, grey, back, 0u32),
        None => text(MENUBAR_TEXT, message_format, 0u32, grey, back, 0u32),
    } as u32 as usize;
    let mut h = [0u8; 200];
    let h = text(h.as_mut_ptr() as usize, heading, m, 0u32, grey, back, 0u32) as u32 as usize;
    let title = menubar_command_game_title(h);
    menubar_command_box(title, h, m, 0, grey, back, 0);
}

/// The kit colours of a club (or nation) record, `database_get_club_kit_colours(club, 1, ..)`: the routine writes the
/// two low words, the high ones stay.
unsafe fn menubar_command_kit_colours(club: usize, fore: &mut u32, back: &mut u32) {
    exe::database_get_club_kit_colours(club, 1, fore, back);
}

/// A person's "Please Confirm" box: `person`'s name (`database_format_staff_name`, 200 bytes) as the title, the colours
/// starting as the heading colours (pale grey, red) and then his club's kit colours (or his nation's, when he has no
/// club), the question `question` (formatted with 1, the colours and the code), answer `code`. `before` runs between the
/// colours and the name (the original sets the holiday there).
unsafe fn menubar_command_person_confirm(person: usize, question: usize, code: u32, before: impl FnOnce()) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut fore = menubar_command_colour(1);
    let mut back = menubar_command_colour(0);
    before();
    let mut name = [0u8; 200];
    exe::database_format_staff_name(person, name.as_mut_ptr() as usize, 200, 1);
    let club = if read_usize(person + staff::CLUB_JOB) != 0 { read_usize(person + staff::CLUB_JOB) } else { read_usize(person + staff::NATIONAL_JOB) };
    if club != 0 {
        menubar_command_kit_colours(club, &mut fore, &mut back);
    }
    let m = text(MENUBAR_TEXT, question, 1u32, fore, back, code) as u32 as usize;
    menubar_command_please_confirm(name.as_ptr() as usize, m, fore, back, code);
}

/// The control question of the club or the nation (0x425, 0x433): the manager's name as the title, the question
/// (`senior` when he already controls the second team, else `all`) with the team's name, the team's kit colours
/// (no defaults: the routine writes the low words), answer `code`.
unsafe fn menubar_command_control_confirm(team_field: usize, controls: impl FnOnce() -> bool, senior: usize, all: usize, code: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let manager = menubar_command_manager();
    let mut name = [0u8; 200];
    exe::database_format_staff_name(manager, name.as_mut_ptr() as usize, 200, 1);
    let controls = controls();
    let team = read_usize(manager + team_field);
    // a nation record has its short name and gender char at the club's offsets
    let g = read_i8(team + club::SHORT_GENDER_NAME) as i32;
    text(MENUBAR_TEXT, if controls { senior } else { all }, g, g, team + club::SHORT_NAME);
    let (mut fore, mut back) = (0u32, 0u32);
    menubar_command_kit_colours(team, &mut fore, &mut back);
    menubar_command_please_confirm(name.as_ptr() as usize, MENUBAR_TEXT, fore, back, code);
}

/// The save file dialog (`open_file_dialog` 0x58f500): the prompt "Enter File Name" for the name `name` in the save
/// folder with the ".sav" extension and the name check, the heading `heading`, in the pale grey and red colours, answer
/// `code` and the last flag `last` (1 for the plain "Save Game"). The prompt and the heading are formatted with the
/// dialog's arguments, as the original pushes them once for the three calls.
unsafe fn menubar_command_file_dialog(heading: usize, name: usize, code: u32, last: u32) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let folder = read_usize(G_CONFIG) + MENUBAR_CONFIG_SAVE_FOLDER;
    let back = menubar_command_colour(0);
    let fore = menubar_command_colour(1);
    let (mut p, mut h) = ([0u8; 200], [0u8; 200]);
    let p = text(p.as_mut_ptr() as usize, 0x9907f4, 1u32, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code,
        0u32, 0u32, 0u32, last) as u32 as usize;
    let h = text(h.as_mut_ptr() as usize, heading, p, 1u32, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code,
        0u32, 0u32, 0u32, last) as u32 as usize;
    #[allow(clippy::type_complexity)]
    let f: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32) =
        core::mem::transmute(0x58f500usize);
    f(h, p, 1, folder, MENUBAR_SAVE_EXTENSION, name, MENUBAR_SAVE_NAME_CHECK, fore, back, code, 0, 0, 0, last);
}

/// The dialog with the configured save name copied into `MENUBAR_SAVE_NAME`.
unsafe fn menubar_command_file_dialog_copy(heading: usize, code: u32, last: u32) {
    menubar_command_copy_text(MENUBAR_SAVE_NAME, read_usize(G_CONFIG) + config::SAVE_NAME);
    menubar_command_file_dialog(heading, MENUBAR_SAVE_NAME, code, last);
}

/// A save failed (`save_game_data` gave 0): the last save date becomes today's (`Date::operator=` of `G_CURRENT_DATE`
/// into `G_LAST_AUTOSAVE_DATE`); then the disk full box (`G_GAME_IN_PROGRESS_FLAG`, set by the save) or the no memory
/// box (`G_GAME_SESSION_STARTED_FLAG`), both cleared, with code 2; otherwise the "could not be saved" box with the path
/// ("Unable To Save", titled `title_heading` formatted, or the game's title). Gives whether that last box was shown.
unsafe fn menubar_command_save_failed(path: usize, title_heading: Option<usize>) -> bool {
    let mut date = [0u32; 2];
    exe::date_operator_3d(G_LAST_AUTOSAVE_DATE, date.as_mut_ptr() as usize, G_CURRENT_DATE);
    if read_u32(G_GAME_IN_PROGRESS_FLAG) != 0 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 0);
        write_u32(G_GAME_SESSION_STARTED_FLAG, 0);
        tel35_cdecl1(0x826a60, 2);
        return false;
    }
    if read_u32(G_GAME_SESSION_STARTED_FLAG) != 0 {
        write_u32(G_GAME_IN_PROGRESS_FLAG, 0);
        write_u32(G_GAME_SESSION_STARTED_FLAG, 0);
        tel35_cdecl1(0x826ce0, 2);
        return false;
    }
    match title_heading {
        // the auto save: titled "Auto Save Game" over "Unable To Save"
        Some(title_format) => {
            let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let (grey, back) = (colour_word(G_GUI_COLOUR_PALE_GREY) as u32, colour_word(0xaebdcc) as u32);
            let m = text(MENUBAR_TEXT, 0xa5aa74, path, 0u32, grey, back, 0u32) as u32 as usize;
            let (mut h, mut tt) = ([0u8; 200], [0u8; 200]);
            let h = text(h.as_mut_ptr() as usize, 0x99081c, m, 0u32, grey, back, 0u32) as u32 as usize;
            let title = text(tt.as_mut_ptr() as usize, title_format, h, m, 0u32, grey, back, 0u32) as u32 as usize;
            menubar_command_box(title, h, m, 0, grey, back, 0);
        }
        None => menubar_command_ok_box(0xa5aa74, Some(path), 0x99081c),
    }
    true
}

/// Rule: the auto save of the team menu.
///
/// With the Config flag `MENUBAR_CONFIG_AUTOSAVE`, a local slot (`MENUBAR_SLOT_REMOTE` clear), no manager being created
/// and no flagged screen: with no save file yet (`G_GAME_HAS_SAVE_FILE_FLAG` clear) the file dialog asks for the name
/// ("Auto Save Game", answer `menubar_answer::AUTO_SAVE`) and the routine gives -4; otherwise, once the interval
/// (`MENUBAR_CONFIG_AUTOSAVE_DAYS`, i16) has passed since the last save (`Date::days_since` of `G_LAST_AUTOSAVE_DATE`),
/// it saves into the folder and name of the Config (`_makepath`, `save_game_data` mode 0x16) and redraws the whole screen;
/// a failure gives its box (`menubar_command_save_failed`) and -4.
unsafe fn menubar_command_autosave(t: &mut MenubarCommandTick) -> Option<i32> {
    let config = read_usize(G_CONFIG);
    if read_u32(config + MENUBAR_CONFIG_FLAGS) & MENUBAR_CONFIG_AUTOSAVE == 0
        || menubar_command_remote()
        || menubar_being_created()
        || !menubar_no_flagged_screens()
    {
        return None;
    }
    if read_u32(G_GAME_HAS_SAVE_FILE_FLAG) == 0 {
        if t.dry {
            return Some(t.give_up());
        }
        covered!("tel35_7 auto save asks the name");
        menubar_command_file_dialog_copy(0xa5aaa8, menubar_answer::AUTO_SAVE, 0);
        return Some(-4);
    }
    let interval = read_i16(config + MENUBAR_CONFIG_AUTOSAVE_DAYS) as i32;
    let days = exe::date_days_since(G_CURRENT_DATE, read_u32(G_LAST_AUTOSAVE_DATE), read_u32(G_LAST_AUTOSAVE_DATE + 4));
    if days < interval {
        return None;
    }
    if t.dry {
        return Some(t.give_up());
    }
    covered!("tel35_7 auto save");
    let mut path = [0u8; 260];
    let config = read_usize(G_CONFIG);
    exe::crt_makepath(path.as_mut_ptr(), 0usize, config + MENUBAR_CONFIG_SAVE_FOLDER, config + config::SAVE_NAME, MENUBAR_SAVE_EXTENSION);
    if exe::save_game_data(path.as_ptr(), 0x16) != 0 {
        exe::screenmanager_redraw_screen(SCREEN_MANAGER, 1, 0);
        return None;
    }
    menubar_command_save_failed(path.as_ptr() as usize, Some(0xa5aaa8));
    Some(-4)
}

/// Rule: the Continue button of the team menu follows the game.
///
/// During a match day with the human's match on (`G_MATCH_DAY`, its `match_day_fields::HUMAN_MATCH` with a
/// `day_match::STATE` other than 2 and 8 (finished)) and no paused match in the record: "Latest Scores" (id 0x418,
/// state 3). Otherwise, with no manager being created, a current screen without the flag or with an exit routine
/// (`get_current_screen_flag`, `current_screen_has_exit_fn`) and a team menu built with context 0: "Waiting For Players"
/// (style 0x1061) when the manager is done (`ScreenManager::is_manager_done`), else "Continue Game" (0x1021), both id
/// 0x3e8 and state 1; leaving the "Latest Scores" state that way, while not done, gives -11 instead. Otherwise the
/// greyed "Continue Game" with no id (state 2). Each change is made once (the record's `CONTINUE_STATE`).
unsafe fn menubar_command_continue_button(t: &mut MenubarCommandTick) -> Option<i32> {
    let state = read_u8(t.rec + menubar_record::CONTINUE_STATE);
    let day = read_usize(G_MATCH_DAY);
    let human_match = if day != 0 { read_usize(day + match_day_fields::HUMAN_MATCH) } else { 0 };
    let match_on = human_match != 0 && {
        let s = read_u8(human_match + day_match::STATE);
        s != 2 && s != 8
    };
    if match_on && read_u32(t.rec + menubar_record::PAUSED_MATCH) == 0 {
        if state != 3 {
            covered!("tel35_7 latest scores button");
            menubar_command_set_continue(t, 0xa5a728, None, 2, 0xc, 0x418, 3);
        }
        return None;
    }
    let screen_free = || {
        core::mem::transmute::<usize, MenubarThis0>(0x7fed30)(SCREEN_MANAGER) == 0
            || core::mem::transmute::<usize, MenubarThis0>(0x7fedd0)(SCREEN_MANAGER) != 0
    };
    if !menubar_being_created() && screen_free() && read_u32(t.rec + menubar_record::CONTEXT) == 0 {
        if state != 1 {
            if exe::screenmanager_is_manager_done(SCREEN_MANAGER, menubar_command_manager() as u32) != 0 {
                covered!("tel35_7 waiting for players button");
                menubar_command_set_continue(t, 0xa16d34, Some(0x1061), 2, 0xc, 0x3e8, 1);
            } else {
                if state == 3 {
                    return Some(-11);
                }
                covered!("tel35_7 continue button");
                menubar_command_set_continue(t, 0xa16d24, Some(0x1021), 2, 0xc, 0x3e8, 1);
            }
        }
    } else if state != 2 {
        covered!("tel35_7 continue button greyed");
        menubar_command_set_continue(t, 0xa16d24, None, 1, 0x2c, 0, 2);
    }
    None
}

/// Rule: the bar's header flashes between the date and the match line, once a second.
///
/// When the time (`utils_get_time_ms`, signed) reaches the record's `REFRESH_TIME`, the `UPDATING` phase flips and the
/// next time is one second later. Phase 1, during a match day being played (`G_MATCH_DAY_PLAYING_FLAG`) with matches
/// left (finished × 100 / matches, as a signed byte, below 100; no matches: 0): "Updating <percentage>%...". Otherwise,
/// in phase 1 or with the Config flag `MENUBAR_CONFIG_MATCH_HEADER`, the paused match "home v away" (short names with
/// their gender chars); otherwise (or with no paused match) the date (`Date::get_weekday_numeric_date_string` of
/// `G_CURRENT_DATE`, 0x7d0, the day slice). A text different from the record's `HEADER_TEXT` goes to the header widget,
/// with a redraw, and is kept.
unsafe fn menubar_command_header(t: &mut MenubarCommandTick) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let rec = t.rec;
    let at = read_i32(rec + menubar_record::REFRESH_TIME);
    t.mark_ms(at);
    if (exe::utils_get_time_ms() as i32) < at {
        return;
    }
    covered!("tel35_7 header flips");
    let phase = 1u8.wrapping_sub(read_u8(rec + menubar_record::UPDATING));
    write_u8(rec + menubar_record::UPDATING, phase);
    write_u32(rec + menubar_record::REFRESH_TIME, exe::utils_get_time_ms().wrapping_add(1000));
    let mut written = false;
    if phase == 1 {
        let day = read_usize(G_MATCH_DAY);
        if day != 0 {
            let matches = read_i32(day + match_day_fields::MATCH_COUNT);
            let percentage = || if matches == 0 { 0 } else { read_i32(day + match_day_fields::FINISHED_COUNT).wrapping_mul(100).wrapping_div(matches) as i8 };
            if (matches == 0 || percentage() < 100) && read_u32(G_MATCH_DAY_PLAYING_FLAG) != 0 {
                covered!("tel35_7 header updating");
                text(t.text, 0xa5a764, percentage() as i32);
                written = true;
            }
        }
    }
    if !written {
        let paused = read_usize(rec + menubar_record::PAUSED_MATCH);
        if (phase == 1 || read_u32(read_usize(G_CONFIG) + MENUBAR_CONFIG_FLAGS) & MENUBAR_CONFIG_MATCH_HEADER != 0) && paused != 0 {
            covered!("tel35_7 header match");
            let away = read_usize(paused + day_match::AWAY_CLUB);
            let home = read_usize(paused + day_match::HOME_CLUB);
            let a = read_i8(away + club::SHORT_GENDER_NAME) as i32;
            let h = read_i8(home + club::SHORT_GENDER_NAME) as i32;
            text(t.text, 0xa5a738, h, h, a, a, home + club::SHORT_NAME, away + club::SHORT_NAME);
        } else {
            let date: MenubarThis3 = core::mem::transmute(0x54a950usize);
            date(G_CURRENT_DATE, t.text, 0x7d0, read_u8(G_CURRENT_DAY_SLICE) as usize);
        }
    }
    if !menubar_command_same_text(rec + menubar_record::HEADER_TEXT, t.text) {
        if !t.dry {
            exe::screenmanager_set_widget_text(SCREEN_MANAGER, read_u16(rec + menubar_record::HEADER_WIDGET) as u32, t.text);
        }
        menubar_command_redraw(t, 0);
        menubar_command_copy_text(rec + menubar_record::HEADER_TEXT, t.text);
    }
}

/// Rule: the status item at the bottom of the bar follows the news, the chat and the players waiting.
///
/// In an unlocked slot (`MENUBAR_SLOT_LOCKED`) with unread news (`News::count_unread`) outside the match menu: "You Have
/// News" in red (id 0x3e9, kind 1). Otherwise, with a chat message waiting for the manager (`MENUBAR_CHAT_PENDING`)
/// outside the match menu: "Chat Message" in red (0x421, kind 2). Otherwise, in an unlocked slot with players waiting
/// (`count_human_managers(1)`) outside the match menu: "Player Waiting" (two in the game) or "<waiting> <total>
/// Players" in yellow (0x3fc, kind 3, redone when a count changes). Otherwise the empty item (black, no id, kind 0).
/// Each change is made once (the record's `STATUS_KIND`); news and chat zero the counts.
unsafe fn menubar_command_status(t: &mut MenubarCommandTick) {
    let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let rec = t.rec;
    let kind = read_u8(rec + menubar_record::STATUS_KIND);
    let unlocked = || read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0;
    if unlocked()
        && read_usize(G_NEWS) != 0
        && exe::news_count_unread(read_usize(G_NEWS), menubar_command_manager(), 0) > 0
        && read_u8(rec + menubar_record::MODE) != 3
    {
        if kind != 1 {
            covered!("tel35_7 status news");
            menubar_command_text(t, 0xa5a3e0);
            menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x3e9, 1, 0, 0);
        }
        return;
    }
    let manager = menubar_command_manager();
    let chat = manager != 0 && {
        let k = read_u16(manager).wrapping_sub(read_u16(G_STAFF_COUNT)).wrapping_add(0x10) as i16;
        read_u32((MENUBAR_CHAT_PENDING as isize + k as isize * 4) as usize) != 0
    };
    if chat && read_u8(rec + menubar_record::MODE) != 3 {
        if kind != 2 {
            covered!("tel35_7 status chat");
            menubar_command_text(t, 0xa5a3d0);
            menubar_command_set_status(t, G_GUI_COLOUR_RED, t.text, 0x421, 2, 0, 0);
        }
        return;
    }
    if unlocked() {
        let waiting = menubar_count_humans(1);
        if waiting > 0 && read_u8(rec + menubar_record::MODE) != 3 {
            let total = menubar_count_humans(0);
            if kind == 3 && waiting == read_i32(rec + menubar_record::WAITING) && total == read_i32(rec + menubar_record::TOTAL) {
                return;
            }
            covered!("tel35_7 status players waiting");
            if total == 2 {
                menubar_command_text(t, 0xa5a3c0);
            } else {
                text(t.text, 0xa5a394, waiting, total);
            }
            menubar_command_set_status(t, G_GUI_COLOUR_YELLOW, t.text, 0x3fc, 3, waiting, total);
            return;
        }
    }
    if kind != 0 {
        covered!("tel35_7 status cleared");
        menubar_command_set_status(t, G_GUI_COLOUR_BLACK, MENUBAR_EMPTY, 0, 0, 0, 0);
    }
}

/// Rule: the menu bar's periodic work, on every call.
///
/// With the slot's record and no box answer to dispatch (`TEL35_NETWORK_FLAG`, the screen manager's "a box answered"
/// flag): in the team menu (mode 2) the auto save (`menubar_command_autosave`), the Continue button
/// (`menubar_command_continue_button`) and the flashing header (`menubar_command_header`); in every menu the status
/// item (`menubar_command_status`). The auto save and the Continue button can end the routine.
unsafe fn menubar_command_periodic(t: &mut MenubarCommandTick) -> Option<i32> {
    if t.rec == 0 || read_u32(TEL35_NETWORK_FLAG) != 0 {
        return None;
    }
    if read_u8(t.rec + menubar_record::MODE) == 2 {
        if let Some(r) = menubar_command_autosave(t) {
            return Some(r);
        }
        if let Some(r) = menubar_command_continue_button(t) {
            return Some(r);
        }
        menubar_command_header(t);
    }
    menubar_command_status(t);
    None
}

/// Rule: the game goes on (-5) unless space is short.
///
/// In a local slot with less than `MENUBAR_LOW_SPACE_MB` MB of free disk: the holiday ends (`set_on_holiday(0)`) and
/// the disk full box opens (code 3), -4. A remote slot goes on; a local one with enough free memory goes on; otherwise
/// the holiday ends and the no memory box opens (code 3), -4.
unsafe fn menubar_command_go_on(t: &mut MenubarCommandTick) -> i32 {
    if !menubar_command_remote() && exe::os_get_free_disk_space_mb(0) < MENUBAR_LOW_SPACE_MB {
        if t.dry {
            return t.give_up();
        }
        exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
        tel35_cdecl1(0x826a60, 3);
        return -4;
    }
    if menubar_command_remote() || exe::os_get_free_memory_mb() >= MENUBAR_LOW_SPACE_MB {
        return -5;
    }
    if t.dry {
        return t.give_up();
    }
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
    tel35_cdecl1(0x826ce0, 3);
    -4
}

/// Continue (the countdown's end and the 0x3e8 command): in an unlocked slot with urgent news
/// (`News::has_urgent_items(manager, 1)`) the news screen opens instead (-4); otherwise `menubar_command_go_on`.
unsafe fn menubar_command_continue(t: &mut MenubarCommandTick) -> i32 {
    if read_u16(menubar_slot(MENUBAR_SLOT_LOCKED)) == 0
        && core::mem::transmute::<usize, MenubarThis2>(0x787350)(read_usize(G_NEWS), menubar_command_manager(), 1) != 0
    {
        if t.dry {
            return t.give_up();
        }
        covered!("tel35_7 urgent news");
        exe::news_screens_open_news_screen(menubar_command_manager(), 0);
        return -4;
    }
    menubar_command_go_on(t)
}

/// Rule: with no widget (item −1) the bar continues the game by itself.
///
/// The holiday countdown (the record's `HOLIDAY_PENDING`): after 5 s (time() − the slot's `MENUBAR_SLOT_CONTINUE_TIME`)
/// it stops (the time −1, the flag 0) and continues (`menubar_command_continue`); before, the seconds left go to the
/// Continue item when they change (`COUNTDOWN`). Then, while the manager is not done: a manager on holiday
/// (`HUMAN_MANAGER::is_on_holiday`) goes on (`menubar_command_go_on`) 2 s after his holiday stamp; with no manager, in
/// interactive mode (`G_INTERACTIVE_MODE_FLAG`) outside a network game (or one with more than one screen record and a
/// local slot), 5 s after the menu bar was built (`MENUBAR_BUILD_TIME`, renewed while the mouse button is down). With a
/// widget, a manager on holiday gets his holiday stamp renewed.
unsafe fn menubar_command_idle(t: &mut MenubarCommandTick, item: i16) -> Option<i32> {
    let human_manager = read_usize(G_HUMAN_MANAGER);
    if item != -1 {
        if menubar_command_manager() != 0 && exe::human_manager_is_on_holiday(human_manager, 0usize) != 0 && !t.dry {
            exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), exe::utils_get_time_ms(), 0usize);
        }
        return None;
    }
    if t.rec != 0 && read_u32(t.rec + menubar_record::HOLIDAY_PENDING) != 0 {
        t.seconds = true;
        let mut now = 0i32;
        exe::at_946ebb(&mut now);
        let elapsed = now.wrapping_sub(read_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME)));
        if elapsed > 5 {
            if t.dry {
                return Some(t.give_up());
            }
            covered!("tel35_7 countdown over");
            write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
            write_u32(t.rec + menubar_record::HOLIDAY_PENDING, 0);
            return Some(menubar_command_continue(t));
        }
        if read_i32(t.rec + menubar_record::COUNTDOWN) != elapsed {
            covered!("tel35_7 countdown");
            let format: MenubarSprintf = core::mem::transmute(FN_CRT_SPRINTF);
            format(t.text, 0x98806c, 5 - elapsed);
            if !t.dry {
                exe::screenmanager_set_widget_text(SCREEN_MANAGER, read_u16(t.rec + menubar_record::CONTINUE_WIDGET) as u32, t.text);
            }
            menubar_command_redraw(t, 0);
            write_i32(t.rec + menubar_record::COUNTDOWN, elapsed);
        }
    }
    let manager = menubar_command_manager();
    if exe::screenmanager_is_manager_done(SCREEN_MANAGER, manager as u32) != 0 {
        return None;
    }
    if manager != 0 {
        if exe::human_manager_is_on_holiday(human_manager, 0usize) != 0 {
            let stamp = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> i32>(0x5fdc30)(read_usize(G_HUMAN_MANAGER), 0)
                .wrapping_add(2000);
            t.mark_ms(stamp);
            if exe::utils_get_time_ms() as i32 >= stamp {
                covered!("tel35_7 holiday goes on");
                return Some(menubar_command_go_on(t));
            }
        }
        return None;
    }
    if read_u32(G_INTERACTIVE_MODE_FLAG) != 0
        && (read_u32(MENUBAR_NETWORK_GAME) == 0
            || (core::mem::transmute::<usize, MenubarThis0>(0x8033c0)(SCREEN_MANAGER) as i16 > 1 && !menubar_command_remote()))
    {
        t.seconds = true;
        let mut now = 0i32;
        exe::at_946ebb(&mut now);
        let mut built = read_i32(MENUBAR_BUILD_TIME);
        if read_u32(G_MOUSE_LEFT_BUTTON_DOWN) != 0 {
            built = now;
            if !t.dry {
                write_i32(MENUBAR_BUILD_TIME, now);
            }
        }
        if now >= built.wrapping_add(5) {
            covered!("tel35_7 interactive goes on");
            return Some(menubar_command_go_on(t));
        }
    }
    None
}

/// The holiday starts: `set_on_holiday(1)`, the holiday stamp now, `ScreenManager::send_clear_click`.
unsafe fn menubar_command_start_holiday(on: u32) {
    exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), on, 0usize);
    exe::human_manager_set_holiday_stamp(read_usize(G_HUMAN_MANAGER), exe::utils_get_time_ms(), 0usize);
    core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
}

/// The saved game is up to date: a save exists (`G_GAME_HAS_SAVE_FILE_FLAG`), its date (`G_LAST_AUTOSAVE_DATE`: the
/// year unsigned, then the day signed) is not before today and nothing changed since (`G_GAME_UNSAVED_CHANGES_FLAG`).
unsafe fn menubar_command_saved_up_to_date() -> bool {
    if read_u32(G_GAME_HAS_SAVE_FILE_FLAG) == 0 {
        return false;
    }
    let (year, today) = (read_u16(G_LAST_AUTOSAVE_DATE + 2), read_u16(G_CURRENT_DATE_YEAR));
    if year < today || (year == today && read_i16(G_LAST_AUTOSAVE_DATE) < read_i16(G_CURRENT_DATE)) {
        return false;
    }
    read_u32(G_GAME_UNSAVED_CHANGES_FLAG) == 0
}

/// Before a restart or an exit the game is offered to be saved: a team menu record in a local slot, no manager being
/// created, no flagged screen, and the saved game not up to date.
unsafe fn menubar_command_save_first(rec: usize) -> bool {
    rec != 0
        && read_u8(rec + menubar_record::MODE) == 2
        && !menubar_command_remote()
        && !menubar_being_created()
        && menubar_no_flagged_screens()
        && !menubar_command_saved_up_to_date()
}

/// Ends the program (`game_fatal_cleanup`, the no-op, `exit(0)`).
unsafe fn menubar_command_exit_program() {
    exe::game_fatal_cleanup();
    exe::shared_noop_ret0_5a8f60();
    exe::crt_exit(0);
}

/// Rule: the answers of the menu bar's boxes, when the screen manager has one to dispatch.
///
/// By the code (`G_PENDING_COMMAND_CODE`, cleared) and the answer (`G_PENDING_COMMAND_ARG`, cleared), in this order:
/// - `GO_ON_HOLIDAY` Yes: a remote slot with at most one active screen record asks "Exit the network game as you go on
///   holiday ?" (the manager's confirm, code `EXIT_NETWORK_ON_HOLIDAY`, -4); otherwise the holiday starts and the club's
///   shortlist is rebuilt (`Shortlist_Manager::rebuild_club_shortlist`).
/// - `EXIT_NETWORK_ON_HOLIDAY`: the holiday starts; Yes gives -12.
/// - `RETURN_FROM_HOLIDAY`: No keeps the holiday (it starts again); otherwise a club manager with the holiday option 3
///   gets his saved line-up back (`Tactics::clear_or_restore_selection`, then each of the 16 saved players still at
///   the club `Tactics::set_selected_player`).
/// - The saves (`SAVE_GAME`, `AUTO_SAVE`, `SAVE_BEFORE_RESTART`, `SAVE_BEFORE_EXIT`): an auto save answered with no name
///   sets the Config option 0x1000; the Config is applied and saved; with a typed path: a flagged screen gives the
///   "cannot be saved while a player is on a response screen" box (-4); otherwise the save name becomes the path's
///   (`_splitpath`), the game is saved (`save_game_data` 0x16; a failure gives its box, -4); the path is freed; then a
///   restart (`restart_game`) or an exit, and a redraw of the whole screen.
/// - `RESERVE_TEAM` and `NATIONAL_TEAMS` Yes: the control of the reserve team (of all the national teams) flips.
/// - `RESTART_GAME` Yes: a flagged screen gives the "cannot be restarted" box (-4); otherwise the save dialog "Save
///   Game Before Restart" (`menubar_command_save_first`, -4) or the restart.
/// - `EXIT_GAME` (also clearing the current command 0x402) Yes: a remote slot gives -12; a flagged screen the "cannot be
///   exited" box (-4); otherwise the save dialog "Save Game Before Exit" (-4) or the end of the program.
unsafe fn menubar_command_answer(rec: usize) -> Option<i32> {
    let code = || read_u32(G_PENDING_COMMAND_CODE);
    let answer = || read_u32(G_PENDING_COMMAND_ARG);
    let clear_answer = || write_u32(G_PENDING_COMMAND_ARG, 0);
    if code() == menubar_answer::GO_ON_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            clear_answer();
            if menubar_command_remote() && core::mem::transmute::<usize, MenubarThis0>(0x7feb70)(SCREEN_MANAGER) as i16 <= 1 {
                covered!("tel35_7 exit network on holiday");
                menubar_command_person_confirm(menubar_command_manager(), 0xa5aa44, menubar_answer::EXIT_NETWORK_ON_HOLIDAY, || {});
                return Some(-4);
            }
            covered!("tel35_7 goes on holiday");
            menubar_command_start_holiday(1);
            let club = read_usize(menubar_command_manager() + staff::CLUB_JOB);
            if club != 0 {
                exe::shortlist_manager_rebuild_club_shortlist(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), club);
            }
        }
        clear_answer();
    }
    if code() == menubar_answer::EXIT_NETWORK_ON_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        menubar_command_start_holiday(1);
        if answer() == 2 {
            clear_answer();
            return Some(-12);
        }
        clear_answer();
    }
    if code() == menubar_answer::RETURN_FROM_HOLIDAY {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        let manager = menubar_command_manager();
        if answer() == 1 {
            clear_answer();
            menubar_command_start_holiday(1);
        } else if read_usize(manager + staff::CLUB_JOB) != 0 && exe::human_manager_get_holiday_option(read_usize(G_HUMAN_MANAGER), 3, manager) != 0 {
            covered!("tel35_7 saved line-up back");
            let lineup = exe::human_manager_get_saved_lineup(read_usize(G_HUMAN_MANAGER), manager) as usize;
            let club = read_usize(manager + staff::CLUB_JOB);
            exe::tactics_clear_or_restore_selection(read_usize(G_TACTICS), club, 0, 0, 0);
            for k in 0..0x10u32 {
                let player = read_i32(lineup + k as usize * 8);
                if player > -1 {
                    let person = read_usize(G_STAFF_TABLE) + player as usize * 0x6e;
                    if read_usize(person + staff::CLUB_JOB) == read_usize(manager + staff::CLUB_JOB) {
                        // the original pushes the whole local whose low byte is the slot; the routine reads the byte
                        exe::tactics_set_selected_player(read_usize(G_TACTICS), read_usize(manager + staff::CLUB_JOB), k, person, 0);
                    }
                }
            }
        }
        clear_answer();
    }
    let c = code();
    if matches!(c, menubar_answer::SAVE_GAME | menubar_answer::AUTO_SAVE | menubar_answer::SAVE_BEFORE_RESTART | menubar_answer::SAVE_BEFORE_EXIT) {
        if c == menubar_answer::AUTO_SAVE && answer() == 0 {
            exe::config_set_option(read_usize(G_CONFIG), 0x1000);
        }
        let (restart, exit) = (code() == menubar_answer::SAVE_BEFORE_RESTART, code() == menubar_answer::SAVE_BEFORE_EXIT);
        write_u32(G_PENDING_COMMAND_CODE, 0);
        exe::config_apply_and_save(read_usize(G_CONFIG));
        let path = answer() as usize;
        if path != 0 {
            clear_answer();
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot save on a response screen");
                menubar_command_ok_box(0xa5aa00, None, 0x99081c);
                exe::crt_operator_delete(path);
                return Some(-4);
            }
            // _splitpath(path, 0, 0, the save name, 0): the name part becomes the save name
            exe::crt_strtok(path, 0usize, 0usize, (read_usize(G_CONFIG) + config::SAVE_NAME) as *mut u8, core::ptr::null_mut());
            if exe::save_game_data(path as *const u8, 0x16) == 0 {
                // the path is freed after the "could not be saved" box only; with the disk full or the no memory box
                // it stays allocated, as in the original
                if menubar_command_save_failed(path, None) {
                    exe::crt_operator_delete(path);
                }
                return Some(-4);
            }
            covered!("tel35_7 saved");
            exe::crt_operator_delete(path);
            if restart {
                core::mem::transmute::<usize, MenubarCdecl0>(0x68dbc0)();
            }
            if exit {
                menubar_command_exit_program();
            }
            exe::screenmanager_redraw_screen(SCREEN_MANAGER, 1, 0);
        }
    }
    if code() == menubar_answer::RESERVE_TEAM {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            covered!("tel35_7 reserve team control");
            let on = exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER), 0usize) == 0;
            core::mem::transmute::<usize, MenubarThis2>(0x5fd590)(read_usize(G_HUMAN_MANAGER), on as usize, 0);
        }
        clear_answer();
    }
    if code() == menubar_answer::NATIONAL_TEAMS {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            covered!("tel35_7 national teams control");
            let on = core::mem::transmute::<usize, MenubarThis2>(0x5fd6b0)(read_usize(G_HUMAN_MANAGER), 0, 0) == 0;
            core::mem::transmute::<usize, MenubarThis2>(0x5fd810)(read_usize(G_HUMAN_MANAGER), on as usize, 0);
        }
        clear_answer();
    }
    if code() == menubar_answer::RESTART_GAME {
        write_u32(G_PENDING_COMMAND_CODE, 0);
        if answer() == 2 {
            clear_answer();
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot restart");
                menubar_command_ok_box(0xa5a9b8, None, 0xa5a9a4);
                return Some(-4);
            }
            if menubar_command_save_first(rec) {
                covered!("tel35_7 save before restart");
                menubar_command_file_dialog_copy(0xa5a988, menubar_answer::SAVE_BEFORE_RESTART, 0);
                return Some(-4);
            }
            core::mem::transmute::<usize, MenubarCdecl0>(0x68dbc0)();
        }
        clear_answer();
    }
    if code() == menubar_answer::EXIT_GAME {
        if read_u16(G_CURRENT_COMMAND_ID) == 0x402 {
            write_u16(G_CURRENT_COMMAND_ID, 0);
        }
        let yes = answer() == 2;
        write_u32(G_PENDING_COMMAND_CODE, 0);
        clear_answer();
        if yes {
            if menubar_command_remote() {
                return Some(-12);
            }
            if !menubar_no_flagged_screens() {
                covered!("tel35_7 cannot exit");
                menubar_command_ok_box(0xa5a944, None, 0xa5a934);
                return Some(-4);
            }
            if menubar_command_save_first(rec) {
                covered!("tel35_7 save before exit");
                menubar_command_file_dialog(0xa5a91c, read_usize(G_CONFIG) + config::SAVE_NAME, menubar_answer::SAVE_BEFORE_EXIT, 0);
                return Some(-4);
            }
            menubar_command_exit_program();
        }
    }
    None
}

/// `find_screens` openers of the Find menu (`open_choose_friendly_teams_screen` 0x5a0650, cdecl): the kind, the data
/// and the third argument, with the fourth slot 0 (left on the stack by the key nation call).
unsafe fn menubar_command_find(kind: u32, a: usize, b: usize) {
    let f: unsafe extern "C" fn(u32, usize, usize, u32) = core::mem::transmute(0x5a0650usize);
    f(kind, a, b, 0);
}

/// Rule: the commands of the team bar and of the manager's menu.
///
/// 0x3e8 Continue (`menubar_command_continue`); 0x418 the latest scores of the human's match competition
/// (`open_latest_scores_screen(manager, day_match::DAY_COMPETITION)`); 0x3e9 the news; 0x42d stops the countdown (the
/// slot's `MENUBAR_SLOT_CONTINUE_TIME` −1, -11); 0x42e the manager stats; 0x41e the job information; 0x425 and 0x433
/// the control questions of the reserve team (code `RESERVE_TEAM`) and of the national teams (`NATIONAL_TEAMS`);
/// 0x41d and 0x41c resign from the nation and the club (`open_resign_confirm_dialog` of the manager manager); 0x3f2
/// retire; 0x420 and 0x41f the board confidence of the nation and the club; 0x434 compare players
/// (`staff_screens_open_compare_players_screen(manager, 1, 1, 1, 0)` when the scout manager holds both players, else the
/// compare dialog); 0x3eb the player search (−1); 0x415 the transfers; 0x3ec the manager's profile (tab 5); 0x3ef go on
/// holiday; 0x3f0 return from holiday (the holiday ends; the manager's confirm, code `RETURN_FROM_HOLIDAY`); 0x414
/// send a message (the item's data). Each gives -4 unless said.
unsafe fn menubar_command_manager_menu(t: &mut MenubarCommandTick, id: u16, item: i16) -> Option<i32> {
    let manager = menubar_command_manager();
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    match id {
        0x3e8 => {
            covered!("tel35_7 continue");
            return Some(menubar_command_continue(t));
        }
        0x418 => {
            let day_match = read_usize(read_usize(G_MATCH_DAY) + match_day_fields::HUMAN_MATCH);
            exe::open_latest_scores_screen(manager, read_usize(day_match + day_match::DAY_COMPETITION));
        }
        0x42e => {
            cdecl0(0x6ae1a0);
        }
        0x41e => {
            cdecl0(0x6b0a10);
        }
        0x425 => {
            covered!("tel35_7 reserve team question");
            let controls = || exe::human_manager_controls_reserve_team(read_usize(G_HUMAN_MANAGER), 0usize) != 0;
            menubar_command_control_confirm(staff::CLUB_JOB, controls, 0xa5a8ec, 0xa5a8c0, menubar_answer::RESERVE_TEAM);
        }
        0x433 => {
            covered!("tel35_7 national teams question");
            let controls = || core::mem::transmute::<usize, MenubarThis2>(0x5fd6b0)(read_usize(G_HUMAN_MANAGER), 0, 0) != 0;
            menubar_command_control_confirm(staff::NATIONAL_JOB, controls, 0xa5a88c, 0xa5a864, menubar_answer::NATIONAL_TEAMS);
        }
        0x41d | 0x41c => {
            let job = read_usize(manager + if id == 0x41d { staff::NATIONAL_JOB } else { staff::CLUB_JOB });
            core::mem::transmute::<usize, MenubarThis1>(0x69b780)(read_usize(G_MANAGER_MANAGER), job);
        }
        0x3f2 => {
            cdecl0(0x6b28b0);
        }
        0x420 | 0x41f => {
            let job = read_usize(manager + if id == 0x420 { staff::NATIONAL_JOB } else { staff::CLUB_JOB });
            tel35_cdecl1(0x6b2080, job);
        }
        0x434 => {
            let scouts = read_usize(G_SCOUT_MANAGER);
            let first = core::mem::transmute::<usize, MenubarThis1>(0x7f7920)(scouts, 1);
            let second = core::mem::transmute::<usize, MenubarThis1>(0x7f7920)(read_usize(G_SCOUT_MANAGER), 2);
            if first != 0 && second != 0 {
                let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x870650usize);
                f(manager, 1, 1, 1, 0);
            } else {
                tel35_cdecl1(0x88c440, manager);
            }
        }
        0x3e9 => {
            exe::news_screens_open_news_screen(manager, 0);
        }
        0x3eb => {
            tel35_cdecl1(0x808540, usize::MAX);
        }
        0x415 => {
            cdecl0(0x8f4fa0);
        }
        0x3ec => {
            let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
            f(manager, 5, 1, 0, 0);
        }
        0x3ef => {
            cdecl0(0x6b2d60);
        }
        0x3f0 => {
            covered!("tel35_7 return from holiday question");
            menubar_command_person_confirm(manager, 0xa5a84c, menubar_answer::RETURN_FROM_HOLIDAY, || {
                exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
            });
        }
        0x42d => {
            write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
            return Some(-11);
        }
        0x414 => {
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x7897f0usize);
            f(tel26_item_value(item), 0);
        }
        _ => return None,
    }
    Some(-4)
}

/// Rule: the commands of the Competitions and Find menus.
///
/// 0x3f3 the FIFA rankings; 0x40c the UEFA coefficients; 0x3f4, 0x3f5, 0x3f6 and 0x3f9 the find screens of kinds 1, 2,
/// 3 and 6 on the item's data and the main nation (`key_nation_get_main_nation(0)`); 0x3f7 and 0x3f8 kinds 4 and 5 on
/// the main continent of the data (`key_nation_get_main_continent(data, 0)`) with the data; 0x3fa the find screen
/// (`open_find_screen`) of the data; 0x429 the written history. Each gives -4.
unsafe fn menubar_command_find_menus(id: u16, item: i16) -> Option<i32> {
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    let data = || tel26_item_value(item) as usize;
    match id {
        0x3f3 => {
            cdecl0(0x49c0b0);
        }
        0x40c => {
            cdecl0(0x49c700);
        }
        0x3f4 | 0x3f5 | 0x3f6 | 0x3f9 => {
            let kind = match id { 0x3f4 => 1, 0x3f5 => 2, 0x3f6 => 3, _ => 6 };
            let d = data();
            let nation = tel35_cdecl1(0x669600, 0);
            menubar_command_find(kind, d, nation);
        }
        0x3f7 | 0x3f8 => {
            let d = data();
            let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x6695c0usize);
            let continent = f(d, 0);
            menubar_command_find(if id == 0x3f7 { 4 } else { 5 }, continent, d);
        }
        0x3fa => {
            tel35_cdecl1(0x5a2c30, data());
        }
        0x429 => {
            tel35_cdecl1(0x5f2b80, 1);
        }
        _ => return None,
    }
    Some(-4)
}

/// Rule: 0x3fd switches to another human manager (the item's data).
///
/// A manager away or done (`ScreenManager::is_manager_away` / `is_manager_done`) is marked not done and gets his
/// screen: the news with no job, otherwise his club's screen (`open_manager_club_screen`). Then the switch
/// (`ScreenManager::switch_to_manager`, `send_clear_click`), the new slot's countdown time −1 and the old record's
/// `HOLIDAY_PENDING` 0. A human on holiday (`HUMAN_MANAGER::is_on_holiday`) is asked to return from it (the holiday
/// ends; the manager's confirm, code `RETURN_FROM_HOLIDAY`, -4); otherwise the remaining commands are tried.
unsafe fn menubar_command_switch_manager(item: i16, rec: usize) -> Option<i32> {
    let next = tel26_item_value(item) as usize;
    covered!("tel35_7 switch manager");
    if exe::screenmanager_is_manager_away(SCREEN_MANAGER, next) != 0 || exe::screenmanager_is_manager_done(SCREEN_MANAGER, next as u32) != 0 {
        core::mem::transmute::<usize, MenubarThis2>(0x7ffeb0)(SCREEN_MANAGER, next, 0);
        if read_usize(next + staff::CLUB_JOB) == 0 && read_usize(next + staff::NATIONAL_JOB) == 0 {
            exe::news_screens_open_news_screen(next, 0);
        } else {
            exe::open_manager_club_screen(next, 0, 0, 0, 0);
        }
    }
    core::mem::transmute::<usize, MenubarThis1>(0x7ffcd0)(SCREEN_MANAGER, next);
    core::mem::transmute::<usize, MenubarThis0>(0x800000)(SCREEN_MANAGER);
    write_i32(menubar_slot(MENUBAR_SLOT_CONTINUE_TIME), -1);
    // a slot with no record writes at address 0xee, as the original
    write_u32(rec + menubar_record::HOLIDAY_PENDING, 0);
    if exe::human_manager_is_on_holiday(read_usize(G_HUMAN_MANAGER), 0usize) == 0 {
        return None;
    }
    // ponytail: the original reads the two heading colours before the switch; they are constants, read here again
    menubar_command_person_confirm(next, 0xa5a84c, menubar_answer::RETURN_FROM_HOLIDAY, || {
        exe::human_manager_set_on_holiday(read_usize(G_HUMAN_MANAGER), 0, 0usize);
    });
    Some(-4)
}

/// Rule: the commands of the game menu.
///
/// 0x3fb adds a manager: in a slot with a manager a new record of the screen manager
/// (`ScreenManager::add_manager_record` with the network join and leave handlers, -11), else `setup_add_human_manager`;
/// 0x3fc the manager status; 0x421 the manager chat screen (its builder and handler, parameter 0 = 0); 0x431 a new game;
/// 0x42a the hall of fame; 0x42b the credits (clearing the current command 0x42b); 0x42c the web sites; 0x42f "Restart
/// the game ?" (the game's title, code `RESTART_GAME`); 0x402 with no flagged screen "Exit the game ?" ("Exit the
/// network game ?" in a remote slot; code `EXIT_GAME`). Each gives -4 unless said; any other id, 0.
unsafe fn menubar_command_game_menu(id: u16) -> i32 {
    let cdecl0 = |f: usize| core::mem::transmute::<usize, MenubarCdecl0>(f)();
    match id {
        0x3fb => {
            if menubar_command_manager() == 0 {
                cdecl0(0x820d40);
                return -4;
            }
            covered!("tel35_7 add manager record");
            core::mem::transmute::<usize, MenubarThis3>(0x7ff8f0)(SCREEN_MANAGER, 0x838a80, 0x838c00, 0);
            -11
        }
        0x3fc => {
            cdecl0(0x81fea0);
            -4
        }
        0x421 => {
            if exe::screenmanager_open_screen(SCREEN_MANAGER, 0x7654d0usize, 0x7657c0usize, 0, 0usize, 0) != 0 {
                exe::screenmanager_set_screen_param(SCREEN_MANAGER, 0, 0usize, 0);
            }
            -4
        }
        0x431 => {
            cdecl0(0x81ce00);
            -4
        }
        0x42a => {
            cdecl0(0x825cc0);
            -4
        }
        0x42b => {
            if read_u16(G_CURRENT_COMMAND_ID) == 0x42b {
                write_u16(G_CURRENT_COMMAND_ID, 0);
            }
            cdecl0(0x4e5920);
            -4
        }
        0x42c => {
            cdecl0(0x5134c0);
            -4
        }
        0x42f | 0x402 => {
            let code = if id == 0x42f {
                covered!("tel35_7 restart question");
                menubar_text(0xa5a838);
                menubar_answer::RESTART_GAME
            } else {
                if !menubar_no_flagged_screens() {
                    return 0;
                }
                covered!("tel35_7 exit question");
                menubar_text(if menubar_command_remote() { 0xa5a820 } else { 0xa5a810 });
                menubar_answer::EXIT_GAME
            };
            let text: MenubarSprintf = core::mem::transmute(FN_LANGLIB_SPRINTF);
            let back = menubar_command_colour(0);
            let fore = menubar_command_colour(1);
            let mut h = [0u8; 200];
            let h = text(h.as_mut_ptr() as usize, 0x9909b8, MENUBAR_TEXT, 1u32, fore, back, code) as u32 as usize;
            let title = menubar_command_game_title(h);
            menubar_command_box(title, h, MENUBAR_TEXT, 1, fore, back, code);
            -4
        }
        _ => 0,
    }
}

/// Rule: the command of a widget (or of the current command with no widget), after the periodic work.
///
/// The Back and Forward arrows (−2, −3) give their id when they are available (`is_back_unavailable(0)`,
/// `is_forward_unavailable`); then the team bar and manager's menu (`menubar_command_manager_menu`), the Competitions and
/// Find menus (`menubar_command_find_menus`), the manager switch (`menubar_command_switch_manager`), 0x3fe "Save Game"
/// (a local slot, no manager being created, no flagged screen: clearing the current command 0x3fe, the save dialog
/// with the Config's name, code `SAVE_GAME`, last flag 1, -4) and the game menu (`menubar_command_game_menu`). The id
/// is read again after the switch (the slot changed) and after "Save Game".
unsafe fn menubar_command_dispatch(t: &mut MenubarCommandTick, item: i16, rec: usize) -> i32 {
    if tel26_item_id(item) == 0xfffe && exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0 {
        return -2;
    }
    if tel26_item_id(item) == 0xfffd && exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0 {
        return -3;
    }
    let id = tel26_item_id(item);
    if let Some(r) = menubar_command_manager_menu(t, id, item) {
        return r;
    }
    if let Some(r) = menubar_command_find_menus(id, item) {
        return r;
    }
    if id == 0x3fd {
        if let Some(r) = menubar_command_switch_manager(item, rec) {
            return r;
        }
    }
    if tel26_item_id(item) == 0x3fe && !menubar_command_remote() && !menubar_being_created() && menubar_no_flagged_screens() {
        covered!("tel35_7 save game");
        if read_u16(G_CURRENT_COMMAND_ID) == 0x3fe {
            write_u16(G_CURRENT_COMMAND_ID, 0);
        }
        menubar_command_file_dialog_copy(0xa5a4b4, menubar_answer::SAVE_GAME, 1);
        return -4;
    }
    menubar_command_game_menu(tel26_item_id(item))
}

/// Rule: the menu bar's answer to a widget (item −1: none), on every frame of every screen.
///
/// 0x424 gives -11 at once. Then the periodic work (`menubar_command_periodic`: the auto save, the Continue button, the
/// flashing header, the status item), the idle continuation (`menubar_command_idle`), the match Continue
/// (`match_screens_handle_continue_command`, its nonzero answer returned), the boxes' answers when the screen manager
/// has one to dispatch (`menubar_command_answer`) and the command (`menubar_command_dispatch`). Each part can end the
/// routine; the answers -4 (a screen or a box opened), -5 (go on), -11, -12, -2 and -3 go to the screen manager; 0, not
/// handled (the object links come next).
/// Original: `menubar_handle_command` (0x7615d0, Menubar.cpp, cdecl).
pub(crate) unsafe extern "C" fn menubar_handle_command(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x424 {
        return -11;
    }
    let rec = menubar_command_record();
    let mut t = MenubarCommandTick { dry: false, rec, text: MENUBAR_TEXT, unsure: false, ms_marks: [0; 2], marks: 0, seconds: false };
    if let Some(r) = menubar_command_periodic(&mut t) {
        return r;
    }
    if let Some(r) = menubar_command_idle(&mut t, item) {
        return r;
    }
    let r = core::mem::transmute::<usize, unsafe extern "C" fn(u32) -> i32>(0x725ca0)(raw);
    if r != 0 {
        return r;
    }
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        if let Some(r) = menubar_command_answer(rec) {
            return r;
        }
    }
    menubar_command_dispatch(&mut t, item, rec)
}

/// The fixed answer of a command after the periodic work, for the comparer (none: it depends on what runs).
unsafe fn menubar_command_expected(id: u16) -> Option<i32> {
    match id {
        0 => Some(0),
        0xfffe => Some(if exe::screenmanager_is_back_unavailable(SCREEN_MANAGER, 0) == 0 { -2 } else { 0 }),
        0xfffd => Some(if exe::screenmanager_is_forward_unavailable(SCREEN_MANAGER) == 0 { -3 } else { 0 }),
        0x42d => Some(-11),
        0x3fb => Some(if menubar_command_manager() == 0 { -4 } else { -11 }),
        0x402 => Some(if menubar_no_flagged_screens() { -4 } else { 0 }),
        0x418 | 0x42e | 0x41e | 0x425 | 0x433 | 0x41d | 0x3f2 | 0x41c | 0x420 | 0x41f | 0x434 | 0x3e9 | 0x3eb | 0x415 | 0x3ec | 0x3ef
        | 0x3f0 | 0x414 | 0x3f3 | 0x40c | 0x3f4..=0x3fa | 0x429 | 0x3fc | 0x421 | 0x431 | 0x42a | 0x42b | 0x42c | 0x42f => Some(-4),
        _ => None,
    }
}

/// Comparer for `menubar_handle_command`.
///
/// The routine runs on every frame of every screen (the automatic career included) and its commands open screens,
/// save and leave the game: only the original runs. Before it the copy's periodic work and idle continuation run dry
/// (`MenubarCommandTick`: on a copy of the record and a text buffer of their own, calling nothing that changes the game
/// or the screen); after it, with no command (id 0), the original's record (without the refresh time, read from the
/// clock) and its answer must be the predicted ones; with a command, the answer must be the command's fixed one
/// (`menubar_command_expected`) or the periodic work's. Not compared: a network game (the widget changes are sent), a box
/// answer to dispatch, the match Continue (0x3e8, `match_screens_handle_continue_command`), an outcome that needs
/// running (a save, a box, a screen), and a run in which the clock crossed a threshold the decisions read.
pub(crate) unsafe extern "C" fn compare_menubar_handle_command(raw: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    if read_u32(MENUBAR_NETWORK_GAME) != 0 || read_u32(TEL35_NETWORK_FLAG) != 0 {
        // simplification: the widget changes are sent to the other players and the answers save, restart or leave the
        // game; only the original runs
        return o(raw);
    }
    let item = raw as i16;
    let id = tel26_item_id(item);
    let real = menubar_command_record();
    let mut copy = [0u8; MENUBAR_RECORD_SIZE];
    if real != 0 {
        core::ptr::copy_nonoverlapping(real as *const u8, copy.as_mut_ptr(), MENUBAR_RECORD_SIZE);
    }
    let mut text = [0u8; 0x200];
    let mut t = MenubarCommandTick {
        dry: true,
        rec: if real != 0 { copy.as_mut_ptr() as usize } else { 0 },
        text: text.as_mut_ptr() as usize,
        unsure: false,
        ms_marks: [0; 2],
        marks: 0,
        seconds: false,
    };
    let (ms0, s0) = (exe::utils_get_time_ms() as i32, exe::at_946ebb(core::ptr::null_mut()));
    let mut early = if id == 0x424 { Some(-11) } else { None };
    if early.is_none() {
        early = menubar_command_periodic(&mut t);
    }
    if early.is_none() {
        early = menubar_command_idle(&mut t, item);
    }
    let expected = match early {
        Some(v) => Some(v),
        None if id == 0x3e8 => None,
        None => menubar_command_expected(id),
    };
    let r = o(raw);
    let (ms1, s1) = (exe::utils_get_time_ms() as i32, exe::at_946ebb(core::ptr::null_mut()));
    let crossed = t.ms_marks[..t.marks].iter().any(|&at| ms0.wrapping_sub(at) < 0 && ms1.wrapping_sub(at) >= 0);
    if t.unsure || crossed || (t.seconds && s0 != s1) {
        return r;
    }
    if let Some(e) = expected {
        SCOREBOARD.record(r == e, || format!("menubar_handle_command (menu bar, command {:#x}): answer original={} rust={}", id, r, e));
    }
    if id == 0 && real != 0 {
        let skip = menubar_record::REFRESH_TIME..menubar_record::REFRESH_TIME + 4;
        let differ: Vec<usize> = (0..MENUBAR_RECORD_SIZE).filter(|k| !skip.contains(k) && read_u8(real + k) != copy[*k]).collect();
        SCOREBOARD.record(differ.is_empty(), || {
            format!("menubar_handle_command (menu bar record): bytes {:x?} differ, original {:x?} rust {:x?}", differ,
                differ.iter().map(|&k| read_u8(real + k)).collect::<Vec<_>>(), differ.iter().map(|&k| copy[k]).collect::<Vec<_>>())
        });
    }
    r
}
