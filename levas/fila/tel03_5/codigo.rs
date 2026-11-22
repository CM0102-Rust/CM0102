// ---------- batch %%LEVA%%: the club screen's input (club_screens.cpp, part 4) ----------

/// The game option bit (in the dword at [G_CONFIG] + 4) that prints a club screen printout straight away instead of
/// asking for a file name [inference].
const PRINTOUT_STRAIGHT_AWAY: u32 = 0x1000000;
/// The game option bit (same dword) passed on to the printout writer as its last argument when the file name box
/// answers [inference].
const PRINTOUT_SAVE_OPTION: u32 = 0x4000000;
/// The dialog code of the club screen's Save Printout file name box.
const CLUB_SCREEN_PRINTOUT_DIALOG: u32 = 0x38;
/// The default file name buffer the Save Printout box edits (0xad9c64).
const PRINTOUT_FILE_NAME: usize = 0xad9c64;

/// The texts of a "no last match" box: the message for a national team's 'B' side, for a reserve side and for a first
/// team, and the heading.
struct NoLastMatchTexts {
    b_side: usize,
    reserves: usize,
    first_team: usize,
    heading: usize,
}

/// "No Recent Matches": the club has no last match (param 2).
const NO_RECENT_MATCHES: NoLastMatchTexts = NoLastMatchTexts { b_side: 0x9906c0, reserves: 0x990680, first_team: 0x9905a8, heading: 0x9906ac };
/// "No Tactics Available": the last match is no longer in the match stats.
const NO_LAST_MATCH_TACTICS: NoLastMatchTexts = NoLastMatchTexts { b_side: 0x9907b8, reserves: 0x990748, first_team: 0x9906f8, heading: 0x9907a0 };
/// "No Stats Available": the last match is no longer in the match stats.
const NO_LAST_MATCH_STATS: NoLastMatchTexts = NoLastMatchTexts { b_side: 0x990640, reserves: 0x9905f8, first_team: 0x9905cc, heading: 0x99062c };

/// The flag dword of the game options ([G_CONFIG] + 4).
unsafe fn club_screen_options() -> u32 {
    read_u32(read_usize(G_CONFIG) + 4)
}

/// The printout writer of printouts.cpp (0x7d4460, cdecl): the club's printout `kind` (the reserves side with
/// `reserves`) into `file` (0 = to the printer) with the option `option`; nonzero when it was written.
unsafe fn club_screen_write_printout(club: usize, reserves: usize, kind: usize, file: usize, option: u32) -> u32 {
    let f: unsafe extern "C" fn(usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7d4460usize);
    f(club, reserves, kind, file, option)
}

/// `confirm_push` (0x591840) with the OK button only and no dialog code: a message box in the club's colours.
unsafe fn club_screen_message_box(title: usize, heading: usize, message: usize, colours: (usize, usize)) {
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, usize, usize, u32) = core::mem::transmute(0x591840usize);
    confirm(title, heading, message, 0, colours.0, colours.1, 0);
}

/// `open_club_squad_screen` (0x453d60, cdecl): the squad screen of `club` (its reserves with `reserves`), restoring
/// the views (1), no alternative view, not locked.
unsafe fn club_screen_open_squad(club: usize, reserves: usize) {
    let open: unsafe extern "C" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x453d60usize);
    open(club, reserves, 1, 0, 0);
}

/// `ScreenManager::get_widget_drop_target` (0x468bd0) of the club screen's widget: the widget a drag ended on, or −1.
unsafe fn club_screen_drop_target(widget: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x468bd0usize);
    f(G_SCREEN_MANAGER, widget)
}

/// `ScreenManager::get_widget_action_data` of the club screen's widget (the value attached to it, 0 for −1).
unsafe fn club_screen_data(widget: u32) -> u32 {
    exe::screenmanager_get_widget_action_data(G_SCREEN_MANAGER, widget) as u32
}

/// The new value of a flags param when bit `bit` is picked: the bits of the group (`groups`, one mask each) that holds
/// it are cleared and the bit set; None when `bit` is not a single bit of one of the groups (nothing is written).
fn club_screen_exclusive_pick(flags: u32, bit: u32, groups: &[u32]) -> Option<u32> {
    if !bit.is_power_of_two() {
        return None;
    }
    groups.iter().find(|&&g| g & bit != 0).map(|&g| (flags & !g) | bit)
}

/// The box shown when the last match of the club screen is missing (`texts`): for a side other than the first team
/// (`reserves`) the message for a national team's 'B' side or a reserve side and the title "<club> 'B' Squad" or "<club>
/// Reserves" (the short name with its gender byte, twice); for the first team the first team message and the club's
/// name as the title. The heading is the texts' own; the box is in the club's colours.
unsafe fn club_screen_no_last_match_box(texts: &NoLastMatchTexts, club: usize, reserves: usize, colours: (usize, usize)) {
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut heading, mut title) = ([0u8; 0x100], [0u8; 0x100]);
    if reserves != 0 {
        let b_side = exe::database_is_national_team(club) != 0;
        let gender = read_i8(club + club::SHORT_GENDER_NAME) as i32;
        let message = lang(CLUB_TABS_TEXT, if b_side { texts.b_side } else { texts.reserves });
        let h = lang(heading.as_mut_ptr() as usize, texts.heading);
        // "{}<%s - club name>{} 'B' Squad" / "{}<%s - club name>{} Reserves"
        let t = lang(title.as_mut_ptr() as usize, if b_side { 0x990780 } else { 0x990728 }, gender, gender, club + club::SHORT_NAME);
        club_screen_message_box(t, h, message, colours);
    } else {
        let message = lang(CLUB_TABS_TEXT, texts.first_team);
        let h = lang(heading.as_mut_ptr() as usize, texts.heading);
        club_screen_message_box(club + club::NAME, h, message, colours);
    }
}

/// Whether the club's last match (param 2) is still in the match stats (`Match_Stats::fixture_entry_matches` with its
/// stats index +8, its two clubs +0xc and +0x10 and its two date words +0x2a and +0x28; the original passes the words
/// in dword slots whose high halves are register leftovers, 0 here: the callee compares words).
unsafe fn club_screen_last_match_found(last_match: usize) -> bool {
    exe::match_stats_fixture_entry_matches(read_usize(G_MATCH_STATS), read_i32(last_match + 8), read_i32(last_match + 0xc), read_i32(last_match + 0x10), read_u16(last_match + 0x2a) as i32, read_u16(last_match + 0x28) as i32) != 0
}

/// The answer of the club screen's Save Printout box (dialog code 0x38, the answer a new file name string): the code
/// goes back to 0; with a name, the answer too, and the printout of kind param 0x1f is written to it (the option bit
/// `PRINTOUT_SAVE_OPTION` passed as 1 or 0); when it cannot be written, the "Unable To Save" box ("The printout
/// "<file>" could not be saved.", title "Save Printout") and the answer −4. The name string is deleted either way.
unsafe fn club_screen_printout_answer(club: usize, reserves: usize, colours: (usize, usize)) -> Option<i32> {
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) == 0 || read_u32(G_PENDING_COMMAND_CODE) != CLUB_SCREEN_PRINTOUT_DIALOG {
        return None;
    }
    let file = read_usize(G_PENDING_COMMAND_ARG);
    write_u32(G_PENDING_COMMAND_CODE, 0);
    if file == 0 {
        return None;
    }
    write_u32(G_PENDING_COMMAND_ARG, 0);
    let option = (club_screen_options() & PRINTOUT_SAVE_OPTION != 0) as u32;
    if club_screen_write_printout(club, reserves, get_screen_param(0x1f), file, option) != 0 {
        exe::crt_operator_delete(file);
        return None;
    }
    covered!("tel03_5 printout not saved");
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let (mut heading, mut title) = ([0u8; 0x100], [0u8; 0x100]);
    let message = lang(CLUB_TABS_TEXT, 0x99082c, file); // "The printout "<%s - file>" could not be saved."
    let h = lang(heading.as_mut_ptr() as usize, 0x99081c); // "Unable To Save"
    let t = lang(title.as_mut_ptr() as usize, 0x99080c); // "Save Printout"
    club_screen_message_box(t, h, message, colours);
    exe::crt_operator_delete(file);
    Some(-4)
}

/// The club screen's Print button (action 0x22, the printout kind the widget's value): with the option bit
/// `PRINTOUT_STRAIGHT_AWAY` the printout goes to the printer at once (answer 0); otherwise the kind goes to param 0x1f
/// and the "Save Printout" / "Enter File Name" file box opens on the human's printouts folder
/// (`HUMAN_MANAGER::get_folder_path` of the screen slot's human, kind 4), ".txt", the name buffer `PRINTOUT_FILE_NAME`,
/// the club's colours and the dialog code 0x38 (−4).
unsafe fn club_screen_printout_action(w: i16, club: usize, reserves: usize, colours: (usize, usize)) -> i32 {
    let kind = club_tabs_value(w);
    if club_screen_options() & PRINTOUT_STRAIGHT_AWAY != 0 {
        covered!("tel03_5 printout straight away");
        club_screen_write_printout(club, reserves, kind, 0, 1);
        return 0;
    }
    set_screen_param(0x1f, kind);
    let lang: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let folder: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x5fc5b0usize);
    let file_box: unsafe extern "C" fn(usize, usize, u32, usize, usize, usize, u32, usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x58f500usize);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let dir = folder(read_usize(G_HUMAN_MANAGER), human, 4);
    let (mut heading, mut title) = ([0u8; 0x100], [0u8; 0x100]);
    let h = lang(heading.as_mut_ptr() as usize, 0x9907f4); // "Enter File Name"
    let t = lang(title.as_mut_ptr() as usize, 0x99080c); // "Save Printout"
    file_box(t, h, 1, dir, 0x990804, PRINTOUT_FILE_NAME, 0, colours.0, colours.1, CLUB_SCREEN_PRINTOUT_DIALOG, 0, 0, 0, 0); // ".txt"
    -4
}

/// The club screen's squad link (action 0x1f): a widget whose value is a club (2000 or more, or −2000 or less) opens
/// that club's first team squad; a value between them other than 0 does nothing (answer 0); value 0 or no widget opens
/// the other side of this club (the reserves from the first team and back) (−4).
unsafe fn club_screen_squad_link(w: i16, club: usize, reserves: usize) -> i32 {
    if w != -1 {
        let v = club_tabs_value(w) as i32;
        if v < 2000 && v > -2000 {
            if v != 0 {
                return 0;
            }
        } else {
            club_screen_open_squad(v as usize, 1);
            return -4;
        }
    }
    club_screen_open_squad(club, (reserves == 0) as usize);
    -4
}

/// The club screen's Tactics button (action 3): a club the current human controls opens its tactics screen (no
/// match, −1); otherwise the tactics of the club's last match (param 2) when it is still in the match stats, else the
/// "No Recent Matches" or "No Tactics Available" box (−4).
unsafe fn club_screen_last_match_tactics(club: usize, reserves: usize, last_match: usize, colours: (usize, usize)) -> i32 {
    if exe::human_manager_current_controls_club(club, 0, 1) != 0 {
        exe::tactics_screen_open(club, reserves as u32, 0usize, 1, -1, 0);
    } else if last_match == 0 {
        club_screen_no_last_match_box(&NO_RECENT_MATCHES, club, reserves, colours);
    } else if club_screen_last_match_found(last_match) {
        covered!("tel03_5 last match tactics");
        exe::tactics_screen_open(club, reserves as u32, 0usize, 0, read_i32(last_match + 8), 0);
    } else {
        club_screen_no_last_match_box(&NO_LAST_MATCH_TACTICS, club, reserves, colours);
    }
    -4
}

/// The club screen's Match Stats button (action 5): the details screen of the club's last match (param 2) when it is
/// still in the match stats, else the "No Recent Matches" or "No Stats Available" box (−4).
unsafe fn club_screen_last_match_stats(club: usize, reserves: usize, last_match: usize, colours: (usize, usize)) -> i32 {
    if last_match == 0 {
        club_screen_no_last_match_box(&NO_RECENT_MATCHES, club, reserves, colours);
    } else if club_screen_last_match_found(last_match) {
        let details: unsafe extern "C" fn(usize) = core::mem::transmute(0x71b4a0usize);
        details(read_usize(last_match + 8));
    } else {
        club_screen_no_last_match_box(&NO_LAST_MATCH_STATS, club, reserves, colours);
    }
    -4
}

/// The club screen's buttons that work on every tab, by the widget's action id (`club_tabs_action`): 1 opens the
/// squad of the widget's club (−4); 0x1f the squad link (`club_screen_squad_link`); 0x21 the hall of fame of the club's
/// English name (−4); 0x22 Print (`club_screen_printout_action`); 2 changes the tab (param 4 gets the widget's value,
/// −11) when it differs; 4 opens the training screen (−4); 3 Tactics and 5 Match Stats of the last match; 6 the
/// club's history (−4). None: on to the open tab.
unsafe fn club_screen_common_actions(w: i16, club: usize, reserves: usize, last_match: usize, colours: (usize, usize)) -> Option<i32> {
    match club_tabs_action(w) {
        1 => {
            club_screen_open_squad(club_tabs_value(w), reserves);
            Some(-4)
        }
        0x1f => Some(club_screen_squad_link(w, club, reserves)),
        0x21 => {
            let hall_of_fame: unsafe extern "C" fn(usize) = core::mem::transmute(0x5f11c0usize);
            hall_of_fame(exe::club_get_english_name(club));
            Some(-4)
        }
        0x22 => Some(club_screen_printout_action(w, club, reserves, colours)),
        2 if club_tabs_value(w) != get_screen_param(4) => {
            covered!("tel03_5 tab change");
            set_screen_param(4, club_tabs_value(w)); // tel03_5 the tab change
            Some(-11)
        }
        4 => {
            let training: unsafe extern "C" fn(usize) = core::mem::transmute(0x8b5930usize);
            training(club);
            Some(-4)
        }
        3 => Some(club_screen_last_match_tactics(club, reserves, last_match, colours)),
        5 => Some(club_screen_last_match_stats(club, reserves, last_match, colours)),
        6 => {
            let history: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x469e70usize);
            history(club, 0, 0);
            Some(-4)
        }
        _ => None,
    }
}

/// The squad tab's view flags (param 7) and column flags (param 0x18) [inference: the View and column choices of the
/// squad list]. A widget with id 7 (`Display::get_widget_id`, or the injected id) picks a view bit (the widget's
/// value): already set, 0x800 is cleared and any other does nothing; not set, the bits of its group are replaced
/// (group 0x103f: 1, 2, 4, 8, 0x10, 0x20, 0x1000; group 0x7c0: 0x40..0x400; 0x800 alone); the answer is −11 whatever
/// the value. Action 8 picks a column bit not yet set: the groups 0x1f, 0x1e0, 0x600, 0x3800, 0xc000, 0x30000, 0xc0000
/// and 0x300000 (−11, nothing written for a value that is not one bit of them); a bit already set goes on. None: on to
/// the other squad answers.
unsafe fn club_screen_squad_view_flags(widget: u32, view: u32, columns: u32) -> Option<i32> {
    let mut id = exe::display_get_widget_id(current_human_display(), widget);
    if id == 0 {
        id = read_u16(G_CURRENT_COMMAND_ID);
    }
    if id == 7 {
        covered!("tel03_5 squad view");
        let bit = club_screen_data(widget);
        let picked = if view & bit != 0 {
            if bit == 0x800 { Some(view - 0x800) } else { None }
        } else {
            club_screen_exclusive_pick(view, bit, &[0x103f, 0x7c0, 0x800]) // tel03_5 the squad view groups
        };
        if let Some(v) = picked {
            set_screen_param(7, v as usize);
        }
        return Some(-11);
    }
    if club_history_action(widget) == 8 {
        let bit = club_screen_data(widget);
        if columns & bit == 0 {
            if let Some(v) = club_screen_exclusive_pick(columns, bit, &[0x1f, 0x1e0, 0x600, 0x3800, 0xc000, 0x30000, 0xc0000, 0x300000]) {
                set_screen_param(0x18, v as usize);
            }
            return Some(-11);
        }
    }
    None
}

/// The squad tab's sort and filter answers (params 8, 9, 0x19 and 0x1a) [inference: the list's filter and its sort
/// columns]. Action 9 with a value whose low byte differs from param 9's stores the value (−11). Action 0xc: param 8
/// keeps its bits 0xc0000000, gets the widget's value and the flag 0x1000000 set when the value's bits were already in
/// param 8 without the flag (else cleared) (−11). Actions 0xd and 0xe (bits 0x400 and 0x800, params 0x19 and 0x1a):
/// param 8 keeps its bits 0xc0000000 and gets the action's bit and the flag 0x1000000 when the action's bit was set,
/// the value's low byte equals the param's and the flag was clear (else not); the param gets the value (−11). None:
/// on to the other squad answers.
unsafe fn club_screen_squad_sort(widget: u32, sort: u32, filter: u8, sort_a: u8, sort_b: u8) -> Option<i32> {
    const FLAG: u32 = 0x1000000;
    let action = club_history_action(widget);
    if action == 9 {
        if club_screen_data(widget) as u8 != filter {
            set_screen_param(9, club_screen_data(widget) as usize);
            return Some(-11);
        }
        return None;
    }
    if action == 0xc {
        let flags = if sort & club_screen_data(widget) != 0 && sort & FLAG == 0 { sort | FLAG } else { sort & !FLAG };
        set_screen_param(8, (club_screen_data(widget) | (flags & 0xc1000000)) as usize);
        return Some(-11);
    }
    for (id, bit, param, current) in [(0xd, 0x400u32, 0x19u32, sort_a), (0xe, 0x800, 0x1a, sort_b)] {
        if action == id {
            let again = sort & bit != 0 && club_screen_data(widget) as u8 == current && sort & FLAG == 0;
            let flags = if again { sort | FLAG } else { sort & !FLAG };
            set_screen_param(8, ((flags & 0xc1000000) | bit) as usize);
            set_screen_param(param, club_screen_data(widget) as usize);
            return Some(-11);
        }
    }
    None
}

/// The squad tab's line-up drags (the human's tactics, `G_TACTICS`, of the club and side). Action 0xa dropped on
/// another 0xa widget (two line-up places, the widgets' values): the same place empties it when it holds a player
/// (`Tactics::clear_selection_slot`); two places swap (`Tactics::swap_selection_slots`) when either holds one (−11).
/// Action 0xa dropped on a player (0x7d7, 0x7dc or 0xb, the value the player): a player other than the place's leaves
/// the selection (`Tactics::remove_player_from_selection`, no roles) and takes the place (−11). Action 0xb (a player):
/// the first empty place in the order of the active formation's eleven places
/// (`Formation::get_slots_in_position_order`), then places 11..19, takes him (−11); none empty goes on. The original
/// passes the place bytes in dword slots whose high bytes are stack or register leftovers (0 here: the callees read the
/// byte). None: on to the other squad answers.
unsafe fn club_screen_squad_lineup_drag(widget: u32, club: usize, reserves: usize) -> Option<i32> {
    let tactics = read_usize(G_TACTICS);
    let selected = |slot: u8| exe::tactics_get_selected_player(tactics, club, slot as u32, reserves as i32);
    let put = |slot: u8, person: usize| {
        exe::tactics_remove_player_from_selection(tactics, club, person, reserves as u32, 0);
        exe::tactics_set_selected_player(tactics, club, slot as u32, person, reserves as u32);
    };
    if club_history_action(widget) == 0xa {
        let target = club_screen_drop_target(widget);
        if target as i16 != -1 && club_history_action(target) == 0xa {
            let a = club_screen_data(widget) as u8;
            let b = club_screen_data(target) as u8;
            if a == b {
                if selected(a) != 0 {
                    exe::tactics_clear_selection_slot(tactics, club, a as u32, reserves as u32);
                    return Some(-11);
                }
            } else if selected(a) != 0 || selected(b) != 0 {
                covered!("tel03_5 line-up swap");
                let swap: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x8973e0usize);
                swap(tactics, club, a as u32, b as u32, reserves as u32);
                return Some(-11);
            }
        }
    }
    if let Some(r) = club_screen_squad_number_drag(widget) {
        return Some(r);
    }
    if club_history_action(widget) == 0xa {
        let target = club_screen_drop_target(widget);
        if target as i16 != -1 && matches!(club_history_action(target), 0x7d7 | 0x7dc | 0xb) {
            let slot = club_screen_data(widget) as u8;
            let person = club_screen_data(target) as usize;
            if person != selected(slot) {
                put(slot, person);
                return Some(-11);
            }
        }
    }
    if club_history_action(widget) == 0x23 {
        let target = club_screen_drop_target(widget);
        if target as i16 != -1 && matches!(club_history_action(target), 0x7d7 | 0x24) {
            let number = club_screen_data(widget) as u8;
            let person = club_screen_data(target) as usize;
            let hm = read_usize(G_HUMAN_MANAGER);
            if person != exe::human_manager_get_squad_number_player(hm, number as i32, 0usize) {
                let clear_player: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6013c0usize);
                let set_number: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x601200usize);
                clear_player(hm, person, 0);
                set_number(hm, number as u32, person, 0);
                return Some(-11);
            }
        }
    }
    if club_history_action(widget) == 0xb {
        let person = club_screen_data(widget) as usize;
        let formation = exe::human_manager_get_active_tactic(read_usize(G_HUMAN_MANAGER), 0usize);
        let order_of: unsafe extern "thiscall" fn(usize, *mut u8, u32) = core::mem::transmute(0x5b31b0usize);
        let mut order = [0u8; 0xb];
        order_of(formation, order.as_mut_ptr(), 0xb);
        for k in 0..0x14u8 {
            let slot = if k < 0xb { order[k as usize] } else { k };
            if selected(slot) == 0 {
                put(slot, person);
                return Some(-11);
            }
        }
    }
    None
}

/// The squad tab's squad number drags (the current human's numbers, `G_HUMAN_MANAGER`): action 0x23 dropped on another
/// 0x23 widget (two numbers 0..49, the widgets' values): the same number is freed when it holds a player
/// (`HUMAN_MANAGER::clear_squad_number`); two numbers swap (`swap_squad_numbers`) when either holds one (−11). None:
/// on (the original checks this between the two line-up drags of action 0xa; the ids differ).
unsafe fn club_screen_squad_number_drag(widget: u32) -> Option<i32> {
    if club_history_action(widget) != 0x23 {
        return None;
    }
    let target = club_screen_drop_target(widget);
    if target as i16 == -1 || club_history_action(target) != 0x23 {
        return None;
    }
    let hm = read_usize(G_HUMAN_MANAGER);
    let holder = |n: u8| exe::human_manager_get_squad_number_player(hm, n as i32, 0usize);
    let a = club_screen_data(widget) as u8;
    let b = club_screen_data(target) as u8;
    if a == b {
        if holder(a) != 0 {
            let clear: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x601560usize);
            clear(hm, a as u32, 0);
            return Some(-11);
        }
    } else if holder(a) != 0 || holder(b) != 0 {
        let swap: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x6016e0usize);
        swap(hm, a as u32, b as u32, 0);
        return Some(-11);
    }
    None
}

/// Answers a click on the club screen's squad tab (param 0 the club, 1 the side; the original reads params 7, 8, 9,
/// 0x18, 0x19 and 0x1a first). A real widget saves the list's scroll row (area of param 0xb) into param 0xa. Then the
/// view and column flags (`club_screen_squad_view_flags`), the sort and filter (`club_screen_squad_sort`); action
/// 0x25 frees all 50 squad numbers when view bit 0x800 is set [inference: the numbers view], else clears or restores
/// the line-up selection (`Tactics::clear_or_restore_selection` with the widget's value) (−11); the line-up and squad
/// number drags (`club_screen_squad_lineup_drag`); action 0x24 (a player) takes the first free squad number 0..49
/// (−11); action 0xf opens the player's Set Transfer Status screen (−4). None: on to the club actions and the menu bar.
unsafe fn club_screen_squad_tab_input(widget: u32) -> Option<i32> {
    let w = widget as i16;
    let club = get_screen_param(0);
    let reserves = get_screen_param(1);
    let view = get_screen_param(7) as u32;
    let sort = get_screen_param(8) as u32;
    let filter = get_screen_param(9) as u8;
    let columns = get_screen_param(0x18) as u32;
    let sort_a = get_screen_param(0x19) as u8;
    let sort_b = get_screen_param(0x1a) as u8;
    if w != -1 {
        set_screen_param(0xa, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0xb)));
    }
    if let Some(r) = club_screen_squad_view_flags(widget, view, columns) {
        return Some(r);
    }
    if let Some(r) = club_screen_squad_sort(widget, sort, filter, sort_a, sort_b) {
        return Some(r);
    }
    if club_history_action(widget) == 0x25 {
        if view & 0x800 != 0 {
            let clear: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x601560usize);
            for k in 0..0x32u32 {
                clear(read_usize(G_HUMAN_MANAGER), k, 0);
            }
        } else {
            exe::tactics_clear_or_restore_selection(read_usize(G_TACTICS), club, reserves as u32, 1, club_screen_data(widget));
        }
        return Some(-11);
    }
    if let Some(r) = club_screen_squad_lineup_drag(widget, club, reserves) {
        return Some(r);
    }
    if club_history_action(widget) == 0x24 {
        let person = club_screen_data(widget) as usize;
        let hm = read_usize(G_HUMAN_MANAGER);
        for k in 0..0x32u8 {
            if exe::human_manager_get_squad_number_player(hm, k as i32, 0usize) == 0 {
                let set_number: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32 = core::mem::transmute(0x601200usize);
                set_number(read_usize(G_HUMAN_MANAGER), k as u32, person, 0);
                return Some(-11);
            }
        }
    }
    if club_history_action(widget) == 0xf {
        let status: unsafe extern "C" fn(usize, u32, i32, u32) = core::mem::transmute(0x8f6770usize);
        status(club_screen_data(widget) as usize, 0, -1, 0);
        return Some(-4);
    }
    None
}

/// Answers a click on the club screen's transfers tab: a real widget saves the list's scroll row (area of param 0x14)
/// into param 0x13; action 0x1a picks a view bit not yet set among 1..0x80 (param 0x12 keeps its high bytes, −11, nothing
/// written for a value that is not one of them); actions 0x1b and 0x1c move the page (param 0x17, its low word) one
/// back or on (−11). None: on to the club actions and the menu bar.
unsafe fn club_screen_transfers_tab_input(widget: u32) -> Option<i32> {
    let view = get_screen_param(0x12) as u32;
    let page = get_screen_param(0x17) as u32;
    if widget as i16 != -1 {
        set_screen_param(0x13, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0x14)));
    }
    if club_history_action(widget) == 0x1a {
        let bit = club_screen_data(widget);
        if view & bit == 0 {
            if let Some(v) = club_screen_exclusive_pick(view, bit, &[0xff]) {
                set_screen_param(0x12, v as usize);
            }
            return Some(-11);
        }
    }
    if club_history_action(widget) == 0x1b {
        set_screen_param(0x17, (page & 0xffff).wrapping_sub(1) as usize); // tel03_5 the previous page
        return Some(-11);
    }
    if club_history_action(widget) == 0x1c {
        set_screen_param(0x17, (page & 0xffff).wrapping_add(1) as usize);
        return Some(-11);
    }
    None
}

/// Answers a click on the club screen's info tab: a real widget saves the scroll row (area of param 0x1d) into param
/// 0x1c when view bit 1 of param 0x1b is set or the current human does not control the club; action 0x20 picks a view
/// bit not yet set among 1..0x10 (param 0x1b keeps its other bits, −11, nothing written for a value that is not one of
/// them). None: on to the club actions and the menu bar.
unsafe fn club_screen_info_tab_input(widget: u32) -> Option<i32> {
    let club = get_screen_param(0);
    let view = get_screen_param(0x1b) as u32;
    if (view & 1 != 0 || exe::human_manager_current_controls_club(club, 0, 1) == 0) && widget as i16 != -1 {
        set_screen_param(0x1c, exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0x1d)));
    }
    if club_history_action(widget) == 0x20 {
        let bit = club_screen_data(widget);
        if view & bit == 0 {
            if let Some(v) = club_screen_exclusive_pick(view, bit, &[0x1f]) {
                set_screen_param(0x1b, v as usize);
            }
            return Some(-11);
        }
    }
    None
}

/// The club screen's answer without the original's two unused TEAM blocks (`club_screen_input`).
unsafe fn club_screen_answer(widget: u32) -> i32 {
    let w = widget as i16;
    let club = get_screen_param(0);
    let reserves = get_screen_param(1);
    let last_match = get_screen_param(2);
    let tab = get_screen_param(4) as u8 as i8;
    let colours = (get_screen_param(5), get_screen_param(6));
    if let Some(r) = club_screen_printout_answer(club, reserves, colours) {
        return r;
    }
    if let Some(r) = club_screen_common_actions(w, club, reserves, last_match, colours) {
        return r;
    }
    let by_tab = match tab {
        1 => club_screen_squad_tab_input(widget),
        2 => club_screen_transfers_tab_input(widget),
        3 | 4 => {
            let f: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(if tab == 3 { 0x468c10usize } else { 0x4691d0usize });
            let r = f(widget);
            if r != 0 { Some(r) } else { None }
        }
        5 => club_screen_info_tab_input(widget),
        _ => {
            covered!("tel03_5 club screen unknown tab");
            club_tabs_error_box(0x1f75);
            None
        }
    };
    if let Some(r) = by_tab {
        return r;
    }
    let club_action: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x482d10usize);
    let r = club_action(widget);
    if r != 0 {
        return r;
    }
    let r = exe::menubar_handle_command(widget);
    if r != 0 {
        return r;
    }
    exe::menubar_handle_object_link(widget)
}

/// Answers a click on the club screen (param 0 the club, 1 the side: reserves when set, 2 the club's last match, 4 the
/// tab, 5 and 6 the club's colours). First the answer of the Save Printout box (`club_screen_printout_answer`); then
/// the buttons of every tab (`club_screen_common_actions`); then the open tab: 1 squad
/// (`club_screen_squad_tab_input`), 2 transfers (`club_screen_transfers_tab_input`), 3 next match
/// (`club_screen_next_match_tab_input`) and 4 fixtures (`club_screen_fixtures_tab_input`) when they answer nonzero, 5
/// info (`club_screen_info_tab_input`), any other tab the club_screens.cpp "Error" box (line 8053); last, the club
/// screen actions (`handle_club_screen_action`), the menu bar commands and the object links, in that order. The
/// original builds two TEAM blocks (0x625 bytes each) on its stack on entry and destroys them on every exit without
/// using them; kept here.
/// Original: `club_screen_input` (0x4671f0, club_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn club_screen_input(widget: u32) -> i32 {
    let mut sides = [0u8; 2 * 0x625];
    let s = sides.as_mut_ptr() as usize;
    exe::crt_vector_constructor_iterator(s, 0x625, 2, 0x453c30usize, 0x453c10usize);
    let r = club_screen_answer(widget);
    exe::crt_vector_destructor_iterator(s, 0x625, 2, 0x453c10usize);
    r
}

/// Whether a click on the club screen only writes the screen's params: the tab change to another tab, and on the squad
/// tab the view, column, sort and filter picks, on the transfers tab the view and the page, on the info tab the view.
/// Not with the Save Printout answer pending or with no widget.
unsafe fn club_screen_click_stays(widget: u32) -> bool {
    let w = widget as i16;
    if w == -1 || (read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && read_u32(G_PENDING_COMMAND_CODE) == CLUB_SCREEN_PRINTOUT_DIALOG) {
        return false;
    }
    match club_tabs_action(w) {
        1 | 0x1f | 0x21 | 0x22 | 3 | 4 | 5 | 6 => return false,
        2 if club_tabs_value(w) != get_screen_param(4) => return true,
        _ => {}
    }
    let action = club_history_action(widget);
    let data = club_screen_data(widget);
    match get_screen_param(4) as u8 {
        1 => {
            let mut id = exe::display_get_widget_id(current_human_display(), widget);
            if id == 0 {
                id = read_u16(G_CURRENT_COMMAND_ID);
            }
            id == 7
                || (action == 8 && get_screen_param(0x18) as u32 & data == 0)
                || (action == 9 && data as u8 != get_screen_param(9) as u8)
                || matches!(action, 0xc | 0xd | 0xe)
        }
        2 => (action == 0x1a && get_screen_param(0x12) as u32 & data == 0) || action == 0x1b || action == 0x1c,
        5 => action == 0x20 && get_screen_param(0x1b) as u32 & data == 0,
        _ => false,
    }
}

/// The answer the club screen must give on the paths that open a screen or a box, print or work on the game when it
/// does not depend on the game's state: the links (−4; the squad link 0 for a small value), Print (0 straight away,
/// else −4), Tactics and Match Stats (−4), freeing or restoring the squad numbers or the selection (−11) and the Set
/// Transfer Status link (−4) of the squad tab. None for the rest (the drags, the box answer, the tabs that have their
/// own comparer, the menu bar).
unsafe fn club_screen_planned_answer(widget: u32) -> Option<i32> {
    let w = widget as i16;
    if read_u32(CLUB_TABS_DIALOG_ANSWERED) != 0 && read_u32(G_PENDING_COMMAND_CODE) == CLUB_SCREEN_PRINTOUT_DIALOG && read_usize(G_PENDING_COMMAND_ARG) != 0 {
        return None;
    }
    match club_tabs_action(w) {
        1 | 0x21 | 3 | 4 | 5 | 6 => return Some(-4),
        0x1f => {
            let v = club_tabs_value(w) as i32;
            return Some(if w != -1 && v != 0 && v < 2000 && v > -2000 { 0 } else { -4 });
        }
        0x22 => return Some(if club_screen_options() & PRINTOUT_STRAIGHT_AWAY != 0 { 0 } else { -4 }),
        2 if club_tabs_value(w) != get_screen_param(4) => return Some(-11),
        _ => {}
    }
    if get_screen_param(4) as u8 != 1 {
        return None;
    }
    match club_history_action(widget) {
        0x25 => Some(-11),
        0xf => Some(-4),
        _ => None,
    }
}

/// Comparer for `club_screen_input`: the clicks that only write params (`club_screen_click_stays`) run side by side
/// (`compare_resposta`, the params and the answer); everything else (screens and boxes opened, printing, the line-up
/// and squad number drags, the next match and fixtures tabs, the menu bar) runs the original only, and its answer is
/// checked against the planned one where it does not depend on the game (`club_screen_planned_answer`).
pub(crate) unsafe extern "C" fn compare_club_screen_input(widget: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    if club_screen_click_stays(widget) {
        return compare_resposta("club screen input", widget, o, club_screen_input, false);
    }
    let action = club_tabs_action(widget as i16);
    let plan = club_screen_planned_answer(widget);
    let r = o(widget);
    if let Some(p) = plan {
        SCOREBOARD.record(r == p, || format!("club screen input (widget {:#x}, action {:#x}): answer {} planned {}", widget, action, r, p));
    }
    r
}
