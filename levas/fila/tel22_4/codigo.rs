// ---------- batch %%LEVA%%: the competition screen's input (comp_screens.cpp, part 1): the screen's clicks and the clicks of its Stages, Results, Fixtures, Schedule, Team Stats, Player Stats and Average Points tabs ----------

/// Set on a network game [inference: match_screens]; with it the competition screen takes a pending "print to file"
/// command (code `COMP_CLICK_PRINTOUT_DIALOG`) left by the Save Printout file box.
const COMP_CLICK_NETWORK_FLAG: usize = 0xb77c84;
/// The game option bit (in the dword at [G_CONFIG] + 4) that prints a printout straight away instead of asking for a
/// file name [inference].
const COMP_CLICK_PRINTOUT_STRAIGHT_AWAY: u32 = 0x1000000;
/// The game option bit (same dword) passed on to the printout writer as its last argument when the file name box
/// answers [inference].
const COMP_CLICK_PRINTOUT_SAVE_OPTION: u32 = 0x4000000;
/// The dialog code of the competition screen's Save Printout file name box.
const COMP_CLICK_PRINTOUT_DIALOG: u32 = 0x39;
/// The default file name buffer the Save Printout box edits (0xad9c64).
const COMP_CLICK_PRINTOUT_FILE_NAME: usize = 0xad9c64;
/// The time (ms) the live draw of the Stages tab last made a tie's fixtures [inference].
const COMP_CLICK_DRAW_TIMER: usize = 0xadae08;

/// The flag dword of the game options ([G_CONFIG] + 4).
unsafe fn comp_click_options() -> u32 {
    read_u32(read_usize(G_CONFIG) + 4)
}

/// `ScreenManager::get_widget_action_id` (0x468b40, thiscall, ret 4) of a widget of the current screen.
unsafe fn comp_click_action_id(widget: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x468b40usize);
    f(G_SCREEN_MANAGER, widget)
}

/// `ScreenManager::get_widget_action_data` (0x468b90) of a widget of the current screen, as a signed number.
unsafe fn comp_click_action_data(widget: u32) -> i32 {
    exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as i32
}

/// Keeps a list's scroll position: param `row` = the scroll row (`ScreenManager::get_area_scroll_row`) of the area in
/// param `area`.
unsafe fn comp_click_keep_scroll(row: u32, area: u32) {
    let r = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(area));
    set_screen_param(row, r);
}

/// A competition screen routine called through its exe address (the hook routes it): cdecl(widget) -> eax.
unsafe fn comp_click_call(address: usize, widget: u32) -> i32 {
    let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(address);
    f(widget)
}

/// Frees the list of param `k` through `LLIST::scalar_deleting_destructor(1)` (0x497910) when there is one and clears
/// the param (the main handler's way; the tab handlers destroy and delete inline, `comp_free_list_param`).
unsafe fn comp_click_delete_list(k: u32) {
    if get_screen_param(k) == 0 {
        return;
    }
    let list = get_screen_param(k);
    if list != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x497910usize);
        f(list, 1);
    }
    set_screen_param(k, 0);
}

/// The menu bar's answer to a widget the screen does not handle: `menubar_handle_command`, and when it does not take
/// it, `menubar_handle_object_link`.
unsafe fn comp_click_menubar(widget: u32) -> i32 {
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// A stage and round picked (from a 1000 × (stage + 1) + round code, see `comp_click_split_code`): with `results` param 1 = 1 (the Results tab) first; when the stage (param 2) or the round (param 3) change,
/// they are written, param 4 = 0, the list of param 5 freed (`destructor`: through 0x497910, otherwise inline) and
/// param 6 = 0. Returns whether they changed.
unsafe fn comp_click_pick_round(stage: i32, round: i32, results: bool, destructor: bool) -> bool {
    if results {
        set_screen_param(1, 1);
    }
    if stage as usize == get_screen_param(2) && round as usize == get_screen_param(3) {
        return false;
    }
    set_screen_param(2, stage as usize);
    set_screen_param(3, round as usize);
    set_screen_param(4, 0);
    if destructor {
        comp_click_delete_list(5);
    } else {
        comp_free_list_param(5);
    }
    set_screen_param(6, 0);
    true
}

/// The stage and round of a 1000 × (stage + 1) + round code (division and remainder truncated, as the original's
/// `idiv`).
fn comp_click_split_code(code: i32) -> (i32, i32) {
    (code / 1000 - 1, code % 1000)
}

/// `printouts_print_comp_report` (0x7d7c80, cdecl): the competition's printout `kind` into `file` (0 = to the
/// printer) with the option `option`; nonzero when it was written.
unsafe fn comp_click_print(comp_record: usize, kind: usize, file: usize, option: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7d7c80usize);
    f(comp_record, kind, file, option)
}

/// Writes the printout of the pending "print to file" command (`G_PENDING_COMMAND_CODE` = 0x39, only on a network
/// game, `COMP_CLICK_NETWORK_FLAG`): the command is cleared; with a file name (`G_PENDING_COMMAND_ARG`, then cleared)
/// the printout kind of param 0x2b is written into it (the last argument 1 with `COMP_CLICK_PRINTOUT_SAVE_OPTION`).
/// A failed write shows "The printout "<file>" could not be saved." ("Unable To Save", "Save Printout", OK only, the
/// competition's colours) and answers −4; the name is freed either way. `None` when the click goes on.
unsafe fn comp_click_pending_printout(comp: usize) -> Option<i32> {
    if read_u32(COMP_CLICK_NETWORK_FLAG) == 0 || read_u32(G_PENDING_COMMAND_CODE) != COMP_CLICK_PRINTOUT_DIALOG {
        return None;
    }
    let file = read_usize(G_PENDING_COMMAND_ARG);
    write_u32(G_PENDING_COMMAND_CODE, 0);
    if file == 0 {
        return None;
    }
    covered!("tel22_4 pending printout");
    write_u32(G_PENDING_COMMAND_ARG, 0);
    let option = (comp_click_options() & COMP_CLICK_PRINTOUT_SAVE_OPTION != 0) as u32;
    if comp_click_print(read_usize(comp + 4), get_screen_param(0x2b), file, option) != 0 {
        exe::crt_operator_delete(file);
        return None;
    }
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(read_usize(comp + 4), &mut c1, &mut c2);
    let (mut heading, mut title) = ([0u8; 0xd0], [0u8; 0xd0]);
    let message = comp_lang_format()(JOB_TEXT, 0x99082c, file) as u32 as usize; // "The printout "<%s - file>" could not be saved."
    let h = comp_lang_format()(heading.as_mut_ptr() as usize, 0x99081c) as u32 as usize; // "Unable To Save"
    let t = comp_lang_format()(title.as_mut_ptr() as usize, 0x99080c) as u32 as usize; // "Save Printout"
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    confirm(t, h, message, 0, c1, c2, 0);
    exe::crt_operator_delete(file);
    Some(-4)
}

/// The Print button (id 0x19, the printout kind the widget's value): with `COMP_CLICK_PRINTOUT_STRAIGHT_AWAY` the
/// printout goes to the printer at once (answer 0); otherwise the kind goes to param 0x2b and the "Save Printout" /
/// "Enter File Name" file box opens on the human's printouts folder (`HUMAN_MANAGER::get_folder_path` of the screen
/// slot's human, kind 4), ".txt", the name buffer `COMP_CLICK_PRINTOUT_FILE_NAME`, the competition's colours and the
/// dialog code 0x39 (−4).
unsafe fn comp_click_print_button(comp: usize, w: i16) -> i32 {
    if comp_click_options() & COMP_CLICK_PRINTOUT_STRAIGHT_AWAY != 0 {
        covered!("tel22_4 printout straight away");
        comp_click_print(read_usize(comp + 4), tel26_item_value(w) as usize, 0, 1);
        return 0;
    }
    set_screen_param(0x2b, tel26_item_value(w) as usize);
    let (mut c1, mut c2) = (0u32, 0u32);
    exe::database_get_comp_colours(read_usize(comp + 4), &mut c1, &mut c2);
    let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x5fc5b0usize);
    let file_box: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32) = core::mem::transmute(0x58f500usize);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let dir = folder(read_usize(G_HUMAN_MANAGER), human, 4);
    let (mut heading, mut title) = ([0u8; 0xd0], [0u8; 0xd0]);
    let h = comp_lang_format()(heading.as_mut_ptr() as usize, 0x9907f4) as u32 as usize; // "Enter File Name"
    let t = comp_lang_format()(title.as_mut_ptr() as usize, 0x99080c) as u32 as usize; // "Save Printout"
    file_box(t, h, 1, dir, 0x990804, COMP_CLICK_PRINTOUT_FILE_NAME, 0, c1, c2, COMP_CLICK_PRINTOUT_DIALOG, 0, 0, 0, 0); // ".txt"
    -4
}

/// The Results tab's clicks (tab 2): with a widget the list's scroll row is kept (param 0xf, area of param 0x10).
/// Action 0xc ("<< Date", the earlier match day): the shown day (params 8 and 9) moves to the "later" pair (0xc and
/// 0xd), the "earlier" pair (0xa and 0xb) becomes the shown day and is cleared; action 0xd ("Date >>") the mirror. Both
/// free the day's fixture list (param 0xe, through 0x497910) and clear its scroll row (param 0xf): −11. Action 0xe
/// picks a stage and round (`comp_click_pick_round`, through the destructor): −11, changed or not. Action 0xf opens the
/// league progress of the fixture in the widget's data (−4). Otherwise `None` (the menu bar).
unsafe fn comp_click_results_tab(widget: u32) -> Option<i32> {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0xf, 0x10);
    }
    if comp_click_action_id(widget) == 0xc {
        covered!("tel22_4 results earlier day");
        set_screen_param(0xc, get_screen_param(8));
        set_screen_param(0xd, get_screen_param(9));
        set_screen_param(8, get_screen_param(0xa));
        set_screen_param(9, get_screen_param(0xb));
        set_screen_param(0xa, 0);
        set_screen_param(0xb, 0);
        comp_click_delete_list(0xe);
        set_screen_param(0xf, 0);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xd {
        covered!("tel22_4 results later day");
        set_screen_param(0xa, get_screen_param(8));
        set_screen_param(0xb, get_screen_param(9));
        set_screen_param(8, get_screen_param(0xc));
        set_screen_param(9, get_screen_param(0xd));
        set_screen_param(0xc, 0);
        set_screen_param(0xd, 0);
        comp_click_delete_list(0xe);
        set_screen_param(0xf, 0);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xe {
        let stage = comp_click_action_data(widget) / 1000 - 1;
        let round = comp_click_action_data(widget) % 1000;
        comp_click_pick_round(stage, round, true, true);
        return Some(-11);
    }
    if comp_click_action_id(widget) == 0xf {
        exe::comp_open_league_progress_for_fixture(comp_click_action_data(widget) as usize);
        return Some(-4);
    }
    None
}

/// The Team Stats tab's clicks (tab 7): with a widget the list's scroll row is kept (param 0x26, area of param 0x27);
/// action 0x16 picks the statistic (the widget's data) into param 0x23 when it changes, frees the sorted table of param
/// 0x24 (crt free, params 0x24 and 0x25 cleared) and answers −11. Otherwise `None` (the menu bar).
unsafe fn comp_click_team_stats_tab(widget: u32) -> Option<i32> {
    if widget as i16 != -1 {
        comp_click_keep_scroll(0x26, 0x27);
    }
    if comp_click_action_id(widget) != 0x16 {
        return None;
    }
    if comp_click_action_data(widget) as usize == get_screen_param(0x23) {
        return None;
    }
    covered!("tel22_4 team stat picked");
    set_screen_param(0x23, comp_click_action_data(widget) as usize);
    comp_free_block_param(0x24);
    Some(-11)
}

/// Rule: the competition screen's input: a pending printout to file, the screen's own buttons, then the open tab's
/// clicks, and what is left to the menu bar.
///
/// After `comp_click_pending_printout` (−4 when the write failed), by the widget's id (`tel26_item_id`, the default
/// command id without a widget): 1 opens the competition screen of the widget's value (`comp_screens_open_comp_screen`
/// with −2, −2, 1; −4); 0x18 the history screen of the competition's English name (`history_screen_open`, −4); 0x19 the
/// Print button (`comp_click_print_button`); 2 the tab bar: a different tab (the widget's value) goes to param 1 (−11),
/// the same tab goes on; 3 the award screen of the widget's value (−4); 4 the competition history screen
/// (`comp_screens_open_history_screen` of the competition record, 0; −4). Then by the tab (param 1, its low byte): 1 the
/// Stages tab (`comp_screens_handle_stages_tab_click`), 2 `comp_click_results_tab`, 3 the Fixtures tab, 4 the Schedule
/// tab, 7 `comp_click_team_stats_tab`, 8 the Player Stats tab, 9 the Average Points tab, 10 the referee stats
/// (`official_stats_screen_handle_click`); their nonzero answer comes back, otherwise (and on tabs 5, 6 and the others)
/// the menu bar's answer.
/// Original: `comp_screens_handle_comp_screen_click` (0x4965d0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_comp_screen_click(widget: u32) -> i32 {
    let comp = get_screen_param(0);
    if let Some(r) = comp_click_pending_printout(comp) {
        return r;
    }
    let w = widget as i16;
    match tel26_item_id(w) {
        1 => {
            let open: unsafe extern "C" fn(usize, i32, i32, u32) = core::mem::transmute(0x48f4a0usize);
            open(tel26_item_value(w) as usize, -2, -2, 1);
            return -4;
        }
        0x18 => {
            let open: unsafe extern "C" fn(usize) = core::mem::transmute(0x5f11c0usize);
            open(comp_get_english_name(read_usize(comp + 4)));
            return -4;
        }
        0x19 => return comp_click_print_button(comp, w),
        _ => {}
    }
    if tel26_item_id(w) == 2 && tel26_item_value(w) as usize != get_screen_param(1) {
        covered!("tel22_4 tab picked");
        set_screen_param(1, tel26_item_value(w) as usize);
        return -11;
    }
    if tel26_item_id(w) == 3 {
        let open: unsafe extern "C" fn(usize) = core::mem::transmute(0x414c20usize);
        open(tel26_item_value(w) as usize);
        return -4;
    }
    if tel26_item_id(w) == 4 {
        let open: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x498780usize);
        open(read_usize(get_screen_param(0) + 4), 0);
        return -4;
    }
    let r = match get_screen_param(1) as u8 as i8 {
        1 => comp_click_call(0x497050, widget),
        2 => match comp_click_results_tab(widget) {
            Some(r) => return r,
            None => 0,
        },
        3 => comp_click_call(0x497930, widget),
        4 => comp_click_call(0x497dd0, widget),
        7 => match comp_click_team_stats_tab(widget) {
            Some(r) => return r,
            None => 0,
        },
        8 => comp_click_call(0x498300, widget),
        9 => comp_click_call(0x498520, widget),
        10 => comp_click_call(0x79aba0, widget),
        _ => 0,
    };
    if r != 0 {
        return r;
    }
    comp_click_menubar(widget)
}

/// Rule: the Stages tab's clicks and its live draw: a tie's fixtures are made one at a time, 1.5 s apart, while the
/// draw runs.
///
/// With a widget the list's scroll row is kept (param 6, area of param 7). Without one (the screen's tick) and a cup
/// stage being drawn (param 0x34, round param 3, tie param 0x32): with param 0x33 set, the tie's fixture state
/// (`cup_tie::FIXTURE_STATE`) 3 answers −11; without it, the next tie of the round (when there is one,
/// `cup_round::TIES`) in state 2 answers −11. Then, with the draw running (param 0x35) and more than 1500 ms since
/// `COMP_CLICK_DRAW_TIMER`, the timer is set and `Cup::create_tie_fixtures(round, −1, 1)` makes the next tie (then the
/// tail below). By the id: 5 picks a stage and round (no param 1, inline free; −11 when they change); 0xb sets the draw
/// running flag (param 0x35) to the widget's value (−11); 9 (one tie) and 10 (all) make the ties' fixtures of the stage
/// (param 2) and round (param 3) until it says no more (9 stops after the second); 6 a different view (param 4) is
/// written (−11); 7 opens the league progress screen of the stage (param 2) with the widget's value (params 0..4; −11);
/// 8 a fixture: played goes to the Results tab on its day (params 1 = 2, 8, 9; 0xb, 0xd cleared, list 0xe freed, 0xf
/// cleared), not played to the Fixtures tab (params 1 = 3, 0x11, 0x12; 0x14, 0x16 cleared, list 0x17 freed, 0x18
/// cleared) (−11). The tail of the draw: param 6 = −1, params 0x11..0x17 cleared (the fixtures list of param 0x17 is not
/// freed: a defect of the original), −11. Otherwise 0.
/// Original: `comp_screens_handle_stages_tab_click` (0x497050, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_stages_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    let tail = || {
        set_screen_param(6, usize::MAX);
        for k in 0x11..=0x17 {
            set_screen_param(k, 0);
        }
        -11
    };
    if w != -1 {
        comp_click_keep_scroll(6, 7);
    } else {
        let cup = get_screen_param(0x34);
        if cup != 0 {
            let round_param = get_screen_param(3);
            let round = round_param as i16 as isize;
            let tie = get_screen_param(0x32) as i16 as isize;
            let ties = || read_usize((read_usize(cup + cup::TIE_TABLES) as isize + round * 4) as usize);
            if get_screen_param(0x33) != 0 {
                if read_u8((ties() as isize + tie * 0x35 + cup_tie::FIXTURE_STATE as isize) as usize) == 3 {
                    return -11;
                }
            } else {
                let count = read_i16((read_usize(cup + cup::ROUND_TABLE) as isize + round * cup_round::SIZE as isize + cup_round::TIES as isize) as usize) as isize;
                if tie + 1 < count && read_u8((ties() as isize + (tie + 1) * 0x35 + cup_tie::FIXTURE_STATE as isize) as usize) == 2 {
                    return -11;
                }
            }
            if get_screen_param(0x35) != 0 && exe::utils_get_time_ms() > read_u32(COMP_CLICK_DRAW_TIMER).wrapping_add(1500) {
                covered!("tel22_4 live draw tie");
                write_u32(COMP_CLICK_DRAW_TIMER, exe::utils_get_time_ms());
                exe::cup_create_tie_fixtures(cup, round_param as u32, u32::MAX, 1);
                return tail();
            }
        }
    }
    if tel26_item_id(w) == 5 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        if comp_click_pick_round(stage, round, false, false) {
            return -11;
        }
    }
    if tel26_item_id(w) == 0xb {
        set_screen_param(0x35, tel26_item_value(w) as usize);
        return -11;
    }
    if tel26_item_id(w) == 9 || tel26_item_id(w) == 0xa {
        covered!("tel22_4 stage fixtures made");
        let comp = get_screen_param(0);
        let stage_index = get_screen_param(2);
        let round = get_screen_param(3) as u32;
        let stage = exe::comp_get_stage(comp, stage_index as i32);
        if exe::cup_create_tie_fixtures(stage, round, u32::MAX, 1) != 0 {
            while tel26_item_id(w) != 9 && exe::cup_create_tie_fixtures(stage, round, u32::MAX, 1) != 0 {}
        }
        return tail();
    }
    if tel26_item_id(w) == 6 {
        if tel26_item_value(w) as usize != get_screen_param(4) {
            set_screen_param(4, tel26_item_value(w) as usize);
            return -11;
        }
    }
    if tel26_item_id(w) == 7 {
        let comp = get_screen_param(0);
        let stage = exe::comp_get_stage(comp, get_screen_param(2) as i32);
        let value = tel26_item_value(w) as usize;
        if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x49b990usize, 0x49bef0usize, 0, 0usize, 0) == 0 {
            return -11;
        }
        set_screen_param(0, stage);
        set_screen_param(1, value);
        set_screen_param(2, 0);
        set_screen_param(3, 0);
        set_screen_param(4, 0);
        return -11;
    }
    if tel26_item_id(w) == 8 {
        let fixture = tel26_item_value(w) as usize;
        if read_u8(fixture + fixture::HOME_GOALS) != 0xff {
            covered!("tel22_4 stage fixture to results");
            set_screen_param(1, 2);
            set_screen_param(8, read_u16(fixture + fixture::DAY) as usize);
            set_screen_param(9, read_u16(fixture + fixture::YEAR) as usize);
            set_screen_param(0xb, 0);
            set_screen_param(0xd, 0);
            comp_free_list_param(0xe);
            set_screen_param(0xf, 0);
        } else {
            covered!("tel22_4 stage fixture to fixtures");
            set_screen_param(1, 3);
            set_screen_param(0x11, read_u16(fixture + fixture::DAY) as usize);
            set_screen_param(0x12, read_u16(fixture + fixture::YEAR) as usize);
            set_screen_param(0x14, 0);
            set_screen_param(0x16, 0);
            comp_free_list_param(0x17);
            set_screen_param(0x18, 0);
        }
        return -11;
    }
    0
}

/// Rule: the Fixtures tab's clicks: moving to the earlier or later fixture day, picking a stage and round, opening a
/// fixture's league progress.
///
/// With a widget the list's scroll row is kept (param 0x18, area of param 0x19). Id 0x10 (earlier): the shown day
/// (params 0x11 and 0x12) moves to the "later" pair (0x15 and 0x16), the "earlier" pair (0x13 and 0x14) becomes the
/// shown day and is cleared; id 0x11 (later) the mirror; both free the list of param 0x17 and clear its scroll row
/// (param 0x18): −11. Id 0x12 picks a stage and round (`comp_click_pick_round` with param 1 = 1, inline free): −11.
/// Id 0x13 opens the league progress of the widget's fixture (−4). Otherwise 0.
/// Original: `comp_screens_handle_fixtures_tab_click` (0x497930, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_fixtures_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x18, 0x19);
    }
    let id = tel26_item_id(w);
    if id == 0x10 || id == 0x11 {
        let (from, to) = if id == 0x10 { (0x13, 0x15) } else { (0x15, 0x13) };
        covered!("tel22_4 fixtures day moved");
        set_screen_param(to, get_screen_param(0x11));
        set_screen_param(to + 1, get_screen_param(0x12));
        set_screen_param(0x11, get_screen_param(from));
        set_screen_param(0x12, get_screen_param(from + 1));
        set_screen_param(from, 0);
        set_screen_param(from + 1, 0);
        comp_free_list_param(0x17);
        set_screen_param(0x18, 0);
        return -11;
    }
    if id == 0x12 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        comp_click_pick_round(stage, round, true, false);
        return -11;
    }
    if id == 0x13 {
        exe::comp_open_league_progress_for_fixture(tel26_item_value(w) as usize);
        return -4;
    }
    0
}

/// Rule: the Schedule tab's clicks: a schedule date goes to the Results tab when matches were played that day, else to
/// the Fixtures tab when matches are due; or a stage and round are picked.
///
/// With a widget the list's scroll row is kept (param 0x1c, area of param 0x1d). Id 0x14: the widget's schedule entry
/// (day +0xc, year +0xe) is looked up (`FIXTURE_MANAGER::scan_fixtures` of the competition, into a local list): played
/// matches (slice byte 2, the 4th argument 0, the 15th 1) found on exactly that date: Results tab (params 1 = 2, 8 and
/// 9 the day, 0xb and 0xd cleared, list 0xe freed, 0xf cleared; −11); else matches to play (slice 0, the 4th argument
/// 1, the 15th 0) on that date: Fixtures tab (params 1 = 3, 0x11 and 0x12, 0x14 and 0x16 cleared, list 0x17 freed,
/// 0x18 cleared; −11); else the next ids are tried. Id 0x15 picks a stage and round (`comp_click_pick_round` with param
/// 1 = 1, inline free): −11. Otherwise 0.
/// Original: `comp_screens_handle_schedule_tab_click` (0x497dd0, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_schedule_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x1c, 0x1d);
    }
    if tel26_item_id(w) == 0x14 {
        type Scan = unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32, u32, u32, u32) -> u16;
        let scan: Scan = core::mem::transmute(FN_FIXTURE_MANAGER_SCAN_FIXTURES);
        let mut list = [0u32; 4];
        let lp = list.as_mut_ptr() as usize;
        exe::new_llist(lp);
        let comp = get_screen_param(0);
        let entry = tel26_item_value(w) as usize;
        let comp_id = read_u32(read_usize(comp + 4));
        let m = u32::MAX;
        for played in [true, false] {
            let mut day = read_u16(entry + 0xc) as u32;
            let mut year = read_u16(entry + 0xe) as u32;
            let mut slice: u8 = if played { 2 } else { 0 };
            let found = scan(read_usize(G_FIXTURE_MANAGER), &mut year, &mut day, &mut slice, (!played) as u32, comp_id, m, m, m, m, m, m, m, 0xffff_fffe,
                lp, played as u32, 0xffff_fffe, m, 1);
            if found != 0 && day & 0xffff == read_i16(entry + 0xc) as i32 as u32 && year & 0xffff == read_i16(entry + 0xe) as i32 as u32 {
                if played {
                    covered!("tel22_4 schedule to results");
                    set_screen_param(1, 2);
                    set_screen_param(8, (day & 0xffff) as usize);
                    set_screen_param(9, (year & 0xffff) as usize);
                    set_screen_param(0xb, 0);
                    set_screen_param(0xd, 0);
                    comp_free_list_param(0xe);
                    set_screen_param(0xf, 0);
                } else {
                    covered!("tel22_4 schedule to fixtures");
                    set_screen_param(1, 3);
                    set_screen_param(0x11, (day & 0xffff) as usize);
                    set_screen_param(0x12, (year & 0xffff) as usize);
                    set_screen_param(0x14, 0);
                    set_screen_param(0x16, 0);
                    comp_free_list_param(0x17);
                    set_screen_param(0x18, 0);
                }
                exe::drop_llist(lp);
                return -11;
            }
        }
        exe::drop_llist(lp);
    }
    if tel26_item_id(w) == 0x15 {
        let (stage, round) = comp_click_split_code(tel26_item_value(w) as i32);
        comp_click_pick_round(stage, round, true, false);
        return -11;
    }
    0
}

/// Rule: the Player Stats tab's clicks: picking the statistic or the position filter rebuilds the table.
///
/// With a widget the list's scroll row is kept (param 0x21, area of param 0x22). Id 0x17: a different statistic (the
/// widget's value) goes to param 0x1e; id 0x1b: a different filter goes to param 0x36; either frees the built table of
/// param 0x1f (crt free, params 0x1f and 0x20 cleared) and answers −11. An unchanged statistic tries the filter id; the
/// rest answer 0.
/// Original: `comp_screens_handle_player_stats_tab_click` (0x498300, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_player_stats_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if w != -1 {
        comp_click_keep_scroll(0x21, 0x22);
    }
    for (id, k) in [(0x17u16, 0x1eu32), (0x1b, 0x36)] {
        if tel26_item_id(w) == id && tel26_item_value(w) as usize != get_screen_param(k) {
            covered!("tel22_4 player stat picked");
            set_screen_param(k, tel26_item_value(w) as usize);
            comp_free_block_param(0x1f);
            return -11;
        }
    }
    0
}

/// Rule: the Average Points tab's clicks: id 5 picks the stage (the widget's value / 1000 − 1) into param 0x2a when it
/// changes (−11); otherwise 0.
/// Original: `comp_screens_handle_average_points_tab_click` (0x498520, comp_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn comp_screens_handle_average_points_tab_click(widget: u32) -> i32 {
    let w = widget as i16;
    if tel26_item_id(w) != 5 {
        return 0;
    }
    let stage = (tel26_item_value(w) as i32 / 1000 - 1) as usize;
    if stage == get_screen_param(0x2a) {
        return 0;
    }
    set_screen_param(0x2a, stage);
    -11
}

/// The competition screen's click comparer: side by side over the screen manager and the current screen's parameter
/// block (`screen_param_regions`) and the answer, when `pure` (the click only reads and writes params); otherwise only
/// the original runs.
unsafe fn comp_click_compare(name: &str, widget: u32, pure: bool, original: impl FnOnce() -> i32, rust: impl FnOnce() -> i32) -> i32 {
    if !pure {
        // simplification: opening screens, printing, the file box, making fixtures, freeing lists and the menu bar do
        // not run twice; those clicks are proven in Rust mode
        return original();
    }
    let mut regions = screen_param_regions();
    regions.push((G_SCREEN_MANAGER, 0x3840));
    let (ro, rr, equal, mo, mr) = crate::comparison::side_by_side(&regions, original, rust);
    SCOREBOARD.record(equal && ro == rr, || format!("competition screen {}(widget {}): return {} x {}, differences in {:?}", name, widget as i16, ro, rr, first_differences(&mo, &mr)));
    ro
}

/// The competition screen's pick of a stage and round frees nothing: unchanged, or no list in param 5.
unsafe fn comp_click_pick_is_pure(code: i32) -> bool {
    let (stage, round) = comp_click_split_code(code);
    (stage as usize == get_screen_param(2) && round as usize == get_screen_param(3)) || get_screen_param(5) == 0
}

/// Comparer for `comp_screens_handle_comp_screen_click`: side by side for the tab bar and the Results and Team Stats
/// clicks that free nothing; the tabs with handlers of their own go through those (only the original here).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_comp_screen_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    let w = widget as i16;
    let pending = read_u32(COMP_CLICK_NETWORK_FLAG) != 0 && read_u32(G_PENDING_COMMAND_CODE) == COMP_CLICK_PRINTOUT_DIALOG;
    let id = tel26_item_id(w);
    let pure = !pending && match id {
        1 | 0x18 | 0x19 | 3 | 4 => false,
        2 if tel26_item_value(w) as usize != get_screen_param(1) => true,
        _ => match get_screen_param(1) as u8 {
            2 => match comp_click_action_id(widget) {
                0xc | 0xd => get_screen_param(0xe) == 0,
                0xe => comp_click_pick_is_pure(comp_click_action_data(widget)),
                _ => false,
            },
            7 => comp_click_action_id(widget) == 0x16 && comp_click_action_data(widget) as usize != get_screen_param(0x23) && get_screen_param(0x24) == 0,
            _ => false,
        },
    };
    comp_click_compare("handle_comp_screen_click", widget, pure, || o(widget), || comp_screens_handle_comp_screen_click(widget))
}

/// Comparer for `comp_screens_handle_stages_tab_click`: side by side unless the live draw may make a tie (the tick with
/// a stage being drawn), ids 9 and 10 (fixtures), 7 (a screen) or 8 with a list to free.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_stages_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let w = widget as i16;
    let pure = !(w == -1 && get_screen_param(0x34) != 0) && match tel26_item_id(w) {
        5 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        7 | 9 | 0xa => false,
        8 => get_screen_param(0xe) == 0 && get_screen_param(0x17) == 0,
        _ => true,
    };
    comp_click_compare("handle_stages_tab_click", widget, pure, || o(widget), || comp_screens_handle_stages_tab_click(widget))
}

/// Comparer for `comp_screens_handle_fixtures_tab_click`: side by side unless a list is freed or the league progress
/// opens.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_fixtures_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    let w = widget as i16;
    let pure = match tel26_item_id(w) {
        0x10 | 0x11 => get_screen_param(0x17) == 0,
        0x12 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        0x13 => false,
        _ => true,
    };
    comp_click_compare("handle_fixtures_tab_click", widget, pure, || o(widget), || comp_screens_handle_fixtures_tab_click(widget))
}

/// Comparer for `comp_screens_handle_schedule_tab_click`: side by side unless a list may be freed (the scan's local
/// list is made and freed in each run).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_schedule_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I3%%);
    let w = widget as i16;
    let pure = match tel26_item_id(w) {
        0x14 => get_screen_param(0xe) == 0 && get_screen_param(0x17) == 0,
        0x15 => comp_click_pick_is_pure(tel26_item_value(w) as i32),
        _ => true,
    };
    comp_click_compare("handle_schedule_tab_click", widget, pure, || o(widget), || comp_screens_handle_schedule_tab_click(widget))
}

/// Comparer for `comp_screens_handle_player_stats_tab_click`: side by side unless the built table (param 0x1f) is
/// there to free.
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_player_stats_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I4%%);
    let pure = get_screen_param(0x1f) == 0;
    comp_click_compare("handle_player_stats_tab_click", widget, pure, || o(widget), || comp_screens_handle_player_stats_tab_click(widget))
}

/// Comparer for `comp_screens_handle_average_points_tab_click`: side by side (it only writes param 0x2a).
pub(crate) unsafe extern "C" fn compare_comp_screens_handle_average_points_tab_click(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I5%%);
    comp_click_compare("handle_average_points_tab_click", widget, true, || o(widget), || comp_screens_handle_average_points_tab_click(widget))
}
