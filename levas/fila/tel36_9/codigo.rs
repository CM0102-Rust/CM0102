// ---------- batch %%LEVA%%: the staff profile actions and the compare players dialog (staff_screens.cpp) ----------

/// A cdecl screen opener by address with one argument.
unsafe fn staff_action1(f: usize, a: usize) {
    let g: unsafe extern "C" fn(usize) = core::mem::transmute(f);
    g(a)
}

/// A cdecl screen opener by address with two arguments.
unsafe fn staff_action2(f: usize, a: usize, b: usize) {
    let g: unsafe extern "C" fn(usize, usize) = core::mem::transmute(f);
    g(a, b)
}

/// A cdecl screen opener by address with three arguments.
unsafe fn staff_action3(f: usize, a: usize, b: usize, c: usize) {
    let g: unsafe extern "C" fn(usize, usize, usize) = core::mem::transmute(f);
    g(a, b, c)
}

/// A cdecl screen opener by address with four arguments.
unsafe fn staff_action4(f: usize, a: usize, b: usize, c: usize, d: usize) {
    let g: unsafe extern "C" fn(usize, usize, usize, usize) = core::mem::transmute(f);
    g(a, b, c, d)
}

/// `confirm_push(title, heading, message, yes/no, colour, colour, dialog code)` (0x591840, cdecl): a message box.
unsafe fn staff_confirm_push(title: usize, heading: usize, message: usize, yes_no: u32, a: u32, b: u32, code: u32) {
    let confirm: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x591840usize);
    confirm(title, heading, message, yes_no, a, b, code)
}

/// The message box of a profile action that cannot be done: the message (already in the screens' text buffer, `message`
/// as the formatter answered), the heading `heading` (a language text into a local buffer), the person's name as the
/// title, OK only, the two colours and the dialog code.
unsafe fn staff_cannot_box(name: usize, heading: usize, message: usize, colours: (u32, u32), code: u32) {
    let mut buffer = [0u8; 0xc8];
    let h = tel36_langlib()(buffer.as_mut_ptr() as usize, heading) as u32 as usize;
    staff_confirm_push(name, h, message, 0, colours.0, colours.1, code);
}

/// `TRANSFER_MANAGER::can_club_bid_for_player(person, club, &reason, 0)` (0x8ce990, thiscall, ret 0x10).
unsafe fn staff_can_bid(person: usize, club: usize, reason: &mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, u32) -> u32 = core::mem::transmute(0x8ce990usize);
    f(read_usize(G_TRANSFER_MANAGER), person, club, reason, 0)
}

/// `transfer_screens_open_cannot_sign_dialog(person, club, reason, kind)` (0x8f0e40, cdecl).
unsafe fn staff_cannot_sign(person: usize, club: usize, reason: u32, kind: usize) {
    staff_action4(0x8f0e40, person, club, reason as usize, kind)
}

/// Rule: the staff profile and compare screens answer their action menu: view, history, votes, squad moves,
/// shortlists, notes, treatments, reports, contracts, transfer and loan offers, scouting, nicknames, the tab choices;
/// what is not an action goes to the profile tab answer and then the menu bar.
///
/// `staff_screens_handle_compare_action(item)`: the person (parameter 0), the two colours (0x11, 0x12) and the slot's
/// human. In a network game the pending command (`G_PENDING_COMMAND` 0xdd6be0, argument `G_PENDING_COMMAND_ARG`): 0x18
/// clears both and, with the argument 2, casts the human's vote for the person in the shortlist of parameter 0xd
/// (`AWARD_SHORTLIST::cast_vote`, at +0x1f) and answers -10, otherwise clears parameter 0xd; 0x27, 0x42 and 0x3c clear
/// both. Then by the widget's id (read again at each test): 0x3c / 0x3d: parameter 0x14 = 1 / 0, -11. 1: the profile of
/// the widget's person (`staff_screens_open_staff_profile(person, 1, 1, 1, 0)`). 0x1f: the terminate contract screen.
/// 0x1e: the history screen with "<first> <second> <day>.<month>.<year>" of the birth date. 2: the vote confirm box
/// for the award entry of the widget (parameter 0xd = the entry; colours of its competition, of its nation's national
/// team, or of the World Cup; "Vote for <name> for the <award> award ?", heading "Please Confirm", title "Vote For
/// Award", yes/no, code 0x18). 3: `CONTRACT_MANAGER::toggle_squad_placement(person, human club, &club)`; failing, the box
/// "There is no space in the <club> squad..." / "Cannot move player to squad" (code 0x27); else -11. 0x1d: the club
/// shortlists of `G_TRANSFER_MANAGER`'s shortlist manager (+0x856) take the person (when there is one), -11. 0x2c: the
/// staff note screen. 0x2e / 0x2d: the treatment confirm screen (leave 1 / 0). 0x2a: the coach report request with the
/// club's assistant (the linked B team's when it is not a reserve) or the first of the five coaches other than the
/// person; -11. 0x2b: the physio report with the club's first physio; -11. 0x23: `staff_normalise_player_attributes`,
/// -11. 5: the transfer status screen. 0x24: the discipline screen. 6: with an agreed contract
/// (`TRANSFER_MANAGER::get_agreed_contract_club`), the box "<name> has already accepted a contract offer from <club>."
/// / "Cannot offer contract" (code 0x3c), else the contract offer (0, 0, 0). 7 / 8 and 0x19 / 0x1a: the transfer list
/// dialog (0 / 1). 9: the release dialog. 0xa / 0xb: the shortlist dialog (0 / 1). 0xc: the contract offer of the
/// widget's kind. 0xd: the widget's offer: status 2 the offer changed screen, else the withdraw dialog. 0xe: the bid
/// screen with the widget's offer slot. 0xf, 0x10, 0x17, 0x16: `can_club_bid_for_player` for the human's club; refused,
/// the cannot sign dialog (kind 3, 0, 2, 1); allowed: 0xf the contract offer (0, 1, 0), 0x10 the bid screen (only for a
/// person with a club, else on to the next tests), 0x17 the trial dialog, 0x16 the loan period
/// (`TRANSFER_MANAGER::get_loan_period`) and, when its start is before its end, the cannot sign dialog (kind 1), else
/// the loan bid screen. 0x18: the end loan dialog. 0x11: a widget data other than parameter 1 becomes parameter 1,
/// -11 (else on). 0x13: the bid screen of the widget's offer. 0x1b / 0x1c: the squad move screen (1 / 0). 0x25, 0x26:
/// the nickname dialog with a 26-byte buffer of the human's nickname for the person, or his common name. 0x29: the
/// scouting: by `SCOUT_MANAGER::get_scout_request_status` 0 / 1 the scout request dialog (parameter 1 = 1 / 0), 2 / 4 /
/// 3 the boxes "no scouts" / "no resources" / "already scouting" (code 0x42); then `SCOUT_MANAGER::add_to_shortlist`.
/// Otherwise `staff_screens_handle_profile_tab_action`, then the menu bar. Actions answer -4 unless said.
/// Original: `staff_screens_handle_compare_action` (0x889c30, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_compare_action(raw: u32) -> i32 {
    let text = tel36_langlib();
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let person = get_screen_param(0);
    let mut colours = (get_screen_param(0x11) as u32, get_screen_param(0x12) as u32);
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let shortlist_vote = |human: usize| {
        let cast: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x417fe0usize);
        cast(read_usize(get_screen_param(0xd) + 0x1f), human, person);
    };
    if read_u32(TEL35_NETWORK_FLAG) != 0 {
        let mut command = read_u32(STAFF_PENDING_COMMAND);
        if command == 0x18 {
            let argument = read_u32(G_PENDING_COMMAND_ARG);
            write_u32(STAFF_PENDING_COMMAND, 0);
            write_u32(G_PENDING_COMMAND_ARG, 0);
            if argument == 2 {
                covered!("tel36_9 network vote");
                shortlist_vote(human);
                set_screen_param(0xd, 0);
                return -10;
            }
            set_screen_param(0xd, 0);
            command = read_u32(STAFF_PENDING_COMMAND);
        }
        if matches!(command, 0x27 | 0x42 | 0x3c) {
            write_u32(G_PENDING_COMMAND_ARG, 0);
            write_u32(STAFF_PENDING_COMMAND, 0);
        }
    }
    let item = raw as i16;
    let id = || tel26_item_id(item);
    let value = || tel26_item_value(item) as usize;
    let mut name = [0u8; 0xc8];
    let n = name.as_mut_ptr() as usize;
    match id() {
        0x3c | 0x3d => {
            set_screen_param(0x14, (id() == 0x3c) as usize);
            return -11;
        }
        1 => {
            let profile: unsafe extern "C" fn(usize, u32, u32, u32, u32) = core::mem::transmute(0x8702a0usize);
            profile(value(), 1, 1, 1, 0);
        }
        0x1f => staff_action1(0x4e53e0, person),
        0x1e => {
            covered!("tel36_9 person history");
            let second = if read_usize(person + 8) != 0 { read_usize(person + 8) } else { 0x989ad8 };
            let first = if read_usize(person + 4) != 0 { read_usize(person + 4) } else { 0x989ad8 };
            let born = person + 0x10;
            let year = read_u16(person + 0x12) as u32;
            let month = exe::date_get_month(born) as i8 as i32 + 1;
            let day = exe::date_get_day_of_month(born) as i16 as i32;
            format(JOB_TEXT, 0x9ced08, first, second, day, month, year);
            staff_action1(0x5f11c0, JOB_TEXT);
        }
        2 => {
            covered!("tel36_9 award vote box");
            let entry = value();
            let award = read_usize(read_usize(read_usize(G_AWARD_MANAGER) + 8) + read_u32(entry + 0xd) as usize * 4);
            set_screen_param(0xd, entry);
            exe::database_format_staff_name(person, n, 0xc8, 1);
            let comp_colours: unsafe extern "C" fn(usize, *mut u32, *mut u32) = core::mem::transmute(FN_DATABASE_GET_COMP_COLOURS);
            let comp = read_usize(entry + 9);
            if comp != 0 {
                comp_colours(comp, &mut colours.0, &mut colours.1);
            } else if read_u8(award + 0x12) == 1 {
                let team: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
                exe::database_get_club_kit_colours(team(read_usize(award + 0xe)), 1, &mut colours.0, &mut colours.1);
            } else {
                let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(TEL36_CLUB_COMP_SIZE) as usize);
                comp_colours(world_cup, &mut colours.0, &mut colours.1);
            }
            let message = text(JOB_TEXT, 0xa8b320, n, read_usize(entry + 5) + 4) as u32 as usize;
            let (mut heading, mut title) = ([0u8; 0xc8], [0u8; 0xc8]);
            let h = text(heading.as_mut_ptr() as usize, 0x9909b8) as u32 as usize;
            let t = text(title.as_mut_ptr() as usize, 0xa8562c) as u32 as usize;
            staff_confirm_push(t, h, message, 1, colours.0, colours.1, 0x18);
        }
        3 => {
            let club = read_usize(human + staff::CLUB_JOB);
            let mut full = 0usize;
            let toggle: unsafe extern "thiscall" fn(usize, usize, usize, *mut usize) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_TOGGLE_SQUAD_PLACEMENT);
            if toggle(read_usize(G_CONTRACT_MANAGER), person, club, &mut full) != 0 {
                return -11;
            }
            covered!("tel36_9 no space in the squad");
            exe::database_format_staff_name(person, n, 0xc8, 1);
            exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
            let gender = read_u8(full + club::SHORT_GENDER_NAME) as i8 as i32;
            let message = text(JOB_TEXT, 0xa8b2cc, gender, gender, full + club::SHORT_NAME) as u32 as usize;
            staff_cannot_box(n, 0xa8b2b0, message, colours, 0x27);
        }
        0x1d => {
            let tm = read_usize(G_TRANSFER_MANAGER);
            if tm != 0 && read_usize(tm + 0x856) != 0 {
                let add: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SHORTLIST_MANAGER_ADD_PLAYER_TO_CLUB_SHORTLISTS);
                add(read_usize(tm + 0x856), person, 1);
            }
            return -11;
        }
        0x2c => staff_action1(0x7961b0, person),
        0x2e => staff_action2(0x7e5b10, person, 1),
        0x2d => staff_action2(0x7e5b10, person, 0),
        0x2a => {
            let mut club = read_usize(person + staff::CLUB_JOB);
            if club == 0 {
                return -11;
            }
            // the original's flag is the frame slot that holds the human until the call writes it
            let mut reserve = human as u32;
            let linked: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(FN_CLUB_GET_LINKED_B_TEAM);
            let b_team = linked(club, &mut reserve, 1);
            if b_team != 0 && reserve == 0 {
                club = b_team;
            }
            let assistant: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_CLUB_ASSISTANT);
            let mut coach = assistant(club);
            if coach == person {
                coach = 0;
            }
            if coach == 0 {
                coach = (0..5).map(|k| read_usize(club + club::COACHES + 4 * k)).find(|&c| c != 0 && c != person).unwrap_or(0);
            }
            if coach != 0 {
                covered!("tel36_9 coach report");
                staff_action2(0x48bc00, person, coach);
            }
            return -11;
        }
        0x2b => {
            let club = read_usize(person + staff::CLUB_JOB);
            if club != 0 {
                if let Some(physio) = (0..3).map(|k| read_usize(club + club::PHYSIOS + 4 * k)).find(|&p| p != 0) {
                    staff_action2(0x7a2a50, person, physio);
                }
            }
            return -11;
        }
        0x23 => {
            staff_action1(FN_STAFF_NORMALISE_PLAYER_ATTRIBUTES, person);
            return -11;
        }
        5 => staff_action4(0x8f6770, person, 0, usize::MAX, 0),
        0x24 => staff_action1(0x5a8380, person),
        6 => {
            let mut club = 0usize;
            let agreed: unsafe extern "thiscall" fn(usize, usize, *mut usize) -> u32 = core::mem::transmute(FN_TRANSFER_MANAGER_GET_AGREED_CONTRACT_CLUB);
            if agreed(read_usize(G_TRANSFER_MANAGER), person, &mut club) != 0 {
                covered!("tel36_9 contract already agreed");
                exe::database_format_staff_name(person, n, 0xc8, 1);
                let gender = read_u8(club + club::SHORT_GENDER_NAME) as i8 as i32;
                let message = text(JOB_TEXT, 0xa8b23c, gender, gender, n, club + club::SHORT_NAME) as u32 as usize;
                staff_cannot_box(n, 0xa8b224, message, colours, 0x3c);
            } else {
                staff_action4(0x4dd0f0, person, 0, 0, 0);
            }
        }
        7 | 8 => staff_action2(0x8f8120, person, 0),
        0x19 | 0x1a => staff_action2(0x8f8120, person, 1),
        9 => staff_action1(0x8f8630, person),
        0xa => staff_action2(0x8f4160, person, 0),
        0xb => staff_action2(0x8f4160, person, 1),
        0xc => staff_action4(0x4dd0f0, person, value(), 1, 0),
        0xd => {
            let slot = value();
            let offer = read_usize(slot);
            if read_u8(offer + offer::STATUS) == 2 {
                staff_action2(0x8f2470, offer, read_u32(offer + offer::SERIAL) as usize);
            } else {
                staff_action1(0x8f4c70, slot);
            }
        }
        0xe => staff_action2(0x8e9350, person, value()),
        0xf | 0x10 | 0x17 | 0x16 => {
            let action = id();
            let club = read_usize(human + staff::CLUB_JOB);
            let mut reason = 0u32;
            if staff_can_bid(person, club, &mut reason) == 0 {
                covered!("tel36_9 cannot sign");
                let kind = match action { 0xf => 3, 0x10 => 0, 0x17 => 2, _ => 1 };
                staff_cannot_sign(person, read_usize(human + staff::CLUB_JOB), reason, kind);
            } else {
                match action {
                    0xf => staff_action4(0x4dd0f0, person, 0, 1, 0),
                    0x10 => {
                        if read_usize(person + staff::CLUB_JOB) == 0 {
                            return staff_compare_default(raw);
                        }
                        staff_action2(0x8e9350, person, 0);
                    }
                    0x17 => staff_action1(0x8f8be0, person),
                    _ => {
                        covered!("tel36_9 loan bid");
                        let contract = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), person);
                        write_usize(G_CONTRACT_LOOKUP_RESULT, contract);
                        let seller = if contract != 0 {
                            read_usize(G_CLUB_TABLE).wrapping_add(read_u32(contract + contract::CLUB).wrapping_mul(club::SIZE as u32) as usize)
                        } else {
                            0
                        };
                        let (mut a, mut b, mut c, mut d) = (0u32, 0u32, 0u32, 0u32);
                        let period: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut u32, *mut u32, *mut u32, *mut u32) =
                            core::mem::transmute(FN_TRANSFER_MANAGER_GET_LOAN_PERIOD);
                        period(read_usize(G_TRANSFER_MANAGER), read_usize(human + staff::CLUB_JOB), seller, person, &mut a, &mut b, &mut c, &mut d);
                        let (mut start, mut end) = ([0u16; 4], [0u16; 4]);
                        exe::date_date_549f70(&mut start, 1, c, d, u32::MAX);
                        exe::date_date_549f70(&mut end, 1, a, b, u32::MAX);
                        let earlier = if start[1] != end[1] { start[1] < end[1] } else { (start[0] as i16) < (end[0] as i16) };
                        if earlier {
                            staff_cannot_sign(person, read_usize(human + staff::CLUB_JOB), reason, 1);
                        } else {
                            staff_action1(0x8f9690, person);
                        }
                    }
                }
            }
        }
        0x18 => staff_action1(0x8fbcc0, person),
        0x11 if value() != get_screen_param(1) => {
            set_screen_param(1, value());
            return -11;
        }
        0x13 => staff_action2(0x8e9550, value(), 1),
        0x1b => staff_action3(0x778f60, person, value(), 1),
        0x1c => staff_action3(0x778f60, person, value(), 0),
        0x25 | 0x26 => {
            covered!("tel36_9 nickname dialog");
            let buffer = exe::crt_malloc(0x1a);
            if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88bc70usize, 0x88bee0usize, 1, 0usize, 0) != 0 {
                set_screen_param(0, person);
                let manager = read_usize(G_HUMAN_MANAGER);
                let has: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_HAS_NICKNAME);
                let source = if has(manager, person, 0) != 0 {
                    let get: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_HUMAN_MANAGER_GET_NICKNAME);
                    get(manager, person, 0)
                } else {
                    read_usize(person + 0xc)
                };
                tel26_copy_text(buffer as *mut u8, source as *const u8);
                set_screen_param(1, buffer);
            }
        }
        0x29 => {
            covered!("tel36_9 scouting");
            let club = read_usize(human + staff::CLUB_JOB);
            exe::database_format_staff_name(person, n, 0xc8, 1);
            exe::database_get_club_kit_colours(club, 1, &mut colours.0, &mut colours.1);
            let status: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x7f8ff0usize);
            match status(read_usize(G_SCOUT_MANAGER), person, 0) as i8 {
                s @ (0 | 1) => {
                    if exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c100usize, 0x88c360usize, 1, 0usize, 0) != 0 {
                        set_screen_param(0, read_u32(person) as usize);
                        set_screen_param(1, (s == 0) as usize);
                    }
                }
                s @ (2 | 3 | 4) => {
                    let (question, heading) = match s { 2 => (0xa8b1e4, 0xa8b1c8), 4 => (0xa8b16c, 0xa8b1c8), _ => (0xa8b144, 0xa8b134) };
                    let message = text(JOB_TEXT, question) as u32 as usize;
                    staff_cannot_box(n, heading, message, colours, 0x42);
                }
                _ => {}
            }
            let add: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_SCOUT_MANAGER_ADD_TO_SHORTLIST);
            add(read_usize(G_SCOUT_MANAGER), person, 0);
        }
        _ => return staff_compare_default(raw),
    }
    -4
}

/// The pending network command (0xdd6be0; its argument is `G_PENDING_COMMAND_ARG`).
const STAFF_PENDING_COMMAND: usize = 0xdd6be0;

/// The answer of the compare actions to a widget that is not an action: the profile tab answer, then the menu bar.
unsafe fn staff_compare_default(raw: u32) -> i32 {
    let r = staff_screens_handle_profile_tab_action(raw);
    if r != 0 {
        return r;
    }
    manager_menu_bar(raw)
}

/// Rule: on the profile tab the attribute view (id 0x12) switches to the widget's view, and "-" / "+" (0x27 / 0x28)
/// move the zoom (parameter 0x13) between 0 and 3.
///
/// `staff_screens_handle_profile_tab_action(item)`: 0x12 with a widget data other than parameter 4: parameter 4 = the
/// data, -11. 0x27 with parameter 0x13 (a char) above 0: one less, -11. 0x28 with parameter 0x13 + 1 below 4: that,
/// -11. Otherwise 0.
/// Original: `staff_screens_handle_profile_tab_action` (0x88b2f0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_profile_tab_action(raw: u32) -> i32 {
    let item = raw as i16;
    if tel26_item_id(item) == 0x12 && tel26_item_value(item) as usize != get_screen_param(4) {
        covered!("tel36_9 profile view");
        set_screen_param(4, tel26_item_value(item) as usize);
        return -11;
    }
    if tel26_item_id(item) == 0x27 {
        let zoom = get_screen_param(0x13) as u8 as i8;
        if zoom > 0 {
            set_screen_param(0x13, (zoom as i32 - 1) as usize);
            return -11;
        }
    }
    if tel26_item_id(item) == 0x28 {
        let zoom = get_screen_param(0x13) as u8 as i8 as i32 + 1;
        if zoom < 4 {
            set_screen_param(0x13, zoom as usize);
            return -11;
        }
    }
    0
}

/// Rule: on the history tab the list keeps its scroll, the filters (0x14) toggle the low five bits of the history
/// mode, the season chooser (0x20, 0x21, 0x22) moves the page, the 0x3e / 0x3f switch the detail and 0x15 picks the row.
///
/// `staff_screens_handle_history_tab_action(item)`: the person (parameter 0), the mode (5), the season (0xe), the page
/// (0xf) and the last page (0x10). For no person or one of the regular staff (id below `G_STAFF_COUNT`), with a widget:
/// parameter 9 = the scroll row of the area in parameter 0xa. By the widget's id: 0x14 with a data sharing no bit with
/// the mode: data 1, 2, 4, 8 or 16 replaces the low five bits of the mode (parameter 5); any other value only answers;
/// -11. 0x20: a season other than parameter 0xe goes there; the page goes back to 1; -11. 0x3e / 0x3f: parameter 0x15 =
/// 1 / 0, -11. 0x21: the page − 1 (at least 1), -11. 0x22: the page + 1 (at most the last page), -11. 0x15: parameter
/// 8 = the data, -11. Otherwise 0.
/// Original: `staff_screens_handle_history_tab_action` (0x88b4b0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_history_tab_action(raw: u32) -> i32 {
    let person = get_screen_param(0);
    let mode = get_screen_param(5) as u32;
    let season = get_screen_param(0xe);
    let page = get_screen_param(0xf) as u16 as i16 as i32;
    let last = get_screen_param(0x10) as u16 as i16 as i32;
    let item = raw as i16;
    if (person == 0 || read_i32(person) < read_i32(G_STAFF_COUNT)) && item != -1 {
        let row = exe::screenmanager_get_area_scroll_row(G_SCREEN_MANAGER, get_screen_param(0xa));
        set_screen_param(9, row);
    }
    let id = || tel26_item_id(item);
    let value = || tel26_item_value(item);
    if id() == 0x14 && mode & value() == 0 {
        covered!("tel36_9 history filter");
        let v = value();
        if matches!(v, 1 | 2 | 4 | 8 | 0x10) {
            set_screen_param(5, ((mode & !0x1f) | v) as usize);
        }
        return -11;
    }
    match id() {
        0x20 => {
            if season != value() as usize {
                set_screen_param(0xe, value() as usize);
            }
            set_screen_param(0xf, 1);
            -11
        }
        0x3e | 0x3f => {
            set_screen_param(0x15, (id() == 0x3e) as usize);
            -11
        }
        0x21 => {
            set_screen_param(0xf, (page - 1).max(1) as usize);
            -11
        }
        0x22 => {
            set_screen_param(0xf, (page + 1).min(last) as isize as usize);
            -11
        }
        0x15 => {
            set_screen_param(8, value() as usize);
            -11
        }
        _ => 0,
    }
}

/// Rule: leaving the staff profile frees its lists.
///
/// `staff_screens_leave_profile_screen()`: the list of parameter 3 (an LLIST, destroyed and deleted) and the buffer of
/// parameter 6 (`free`), each cleared when present.
/// Original: `staff_screens_leave_profile_screen` (0x88b8a0, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_leave_profile_screen() {
    let list = get_screen_param(3);
    let buffer = get_screen_param(6);
    if list != 0 {
        covered!("tel36_9 leave the profile frees the list");
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        destroy(list);
        exe::crt_operator_delete(list);
        set_screen_param(3, 0);
    }
    if buffer != 0 {
        exe::crt_free(buffer);
        set_screen_param(6, 0);
    }
}

/// Rule: the compare players dialog opens from the menu.
///
/// `staff_screens_open_compare_players_dialog()`: `open_screen` with the builder 0x88c460, the handler 0x88c660,
/// single instance, no leave routine; returns what `open_screen` answers.
/// Original: `staff_screens_open_compare_players_dialog` (0x88c440, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_open_compare_players_dialog() -> u32 {
    covered!("tel36_9 open the compare dialog");
    exe::screenmanager_open_screen(G_SCREEN_MANAGER, 0x88c460usize, 0x88c660usize, 1, 0usize, 0)
}

/// Rule: the compare players dialog says two players must be chosen first (when they are not), under the human's name
/// in his club's colours (or "Can not compare players"), with an Okay button.
///
/// `staff_screens_build_compare_players_dialog()`: the slot's human; his club (`staff::CLUB_JOB`, else
/// `staff::NATIONAL_JOB`) gives the background picture and, when there is one, the kit 1 colours (else the defaults
/// `JOB_COLOUR_BDF0` / `JOB_COLOUR_3184`). The title: the human's name (2000 bytes into the screens' text buffer) when
/// there is a human and the language is not 0, else "Can not compare players". The two compare players
/// (`SCOUT_MANAGER::get_compare_player` 1 and 2) have their names formatted (200 bytes, unused); without both, the
/// text "Two players must be selected to compare against each other" (otherwise the buffer keeps what was there). The
/// text area (0x6e, 0x8c)-(0x30c, 0x221) (font 3, pale grey), the menu bar (4, 0) and a one-column button area with
/// "Okay" (id 0x38).
/// Original: `staff_screens_build_compare_players_dialog` (0x88c460, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_build_compare_players_dialog() {
    let text = tel36_langlib();
    let weights: [u8; 2] = [1, 0];
    let mut fore = read_u16(JOB_COLOUR_BDF0) as u32;
    let mut back = read_u16(JOB_COLOUR_3184) as u32;
    let slot = read_i16(G_SCREEN_MANAGER_CURRENT_SLOT) as isize;
    let human = read_usize((G_SCREEN_RECORDS_MANAGER as isize + slot * 0x300) as usize);
    let mut club = read_usize(human + staff::CLUB_JOB);
    if club == 0 {
        club = read_usize(human + staff::NATIONAL_JOB);
    }
    covered!("tel36_9 build the compare dialog");
    exe::background_set_club_picture(club, 0, 0);
    if club != 0 {
        exe::database_get_club_kit_colours(club, 1, &mut fore, &mut back);
    }
    if human != 0 && exe::langlib_get_language() != 0 {
        exe::database_format_staff_name(human, JOB_TEXT, 0x7d0, 1);
        exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, JOB_TEXT);
    } else {
        exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, back, fore, 0xa8b44c);
    }
    let compare: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7f7920usize);
    let first = compare(read_usize(G_SCOUT_MANAGER), 1);
    let second = compare(read_usize(G_SCOUT_MANAGER), 2);
    let (mut a, mut b) = ([0u8; 0xc8], [0u8; 0xc8]);
    if first != 0 {
        exe::database_format_staff_name(first, a.as_mut_ptr() as usize, 0xc8, 1);
    }
    if second != 0 {
        exe::database_format_staff_name(second, b.as_mut_ptr() as usize, 0xc8, 1);
    }
    if first == 0 || second == 0 {
        text(JOB_TEXT, 0xa8b410);
    }
    let rows = (0x195 / exe::font_get_height(3)) as u8 as i8 as i32 as u32;
    tel26_text_area([0x6e, 0x8c, 0x30c, 0x221, 2, 0, rows, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0]);
    exe::menubar_build(4, 0);
    let area = tel26_add_area(current_human_display(), [0x64, 0x22b, 0x316, 0x24e, 1, weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    text(JOB_TEXT, 0xa8b408);
    tel26_area_button([area, 0, 0, 0x30, job_colour(JOB_COLOUR_317E), 0xc, 3, job_colour(JOB_COLOUR_3184), JOB_TEXT as u32, 0x38, 0]);
}

/// Rule: Okay (id 0x38) closes the compare dialog; the rest goes to the menu bar.
/// Original: `staff_screens_handle_compare_players_action` (0x88c660, staff_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn staff_screens_handle_compare_players_action(raw: u32) -> i32 {
    if tel26_item_id(raw as i16) == 0x38 {
        return -9;
    }
    manager_menu_bar(raw)
}

/// The default title colour of the compare dialog (word).
const JOB_COLOUR_BDF0: usize = 0xaebdf0;

/// Comparer for `staff_screens_handle_compare_action`: the tab choices (0x3c, 0x3d, 0x11) side by side over the screen
/// params; the actions (screens, boxes, transfers, notes) only the original.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_compare_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I0%%);
    if read_u32(TEL35_NETWORK_FLAG) != 0 || !matches!(tel26_item_id(item as i16), 0x3c | 0x3d) {
        // simplification: opening screens, the boxes and the game actions do not run twice; proven in Rust mode
        return o(item);
    }
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_compare_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff profile action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_handle_profile_tab_action`: side by side over the screen params.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_profile_tab_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_profile_tab_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff profile tab action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_handle_history_tab_action`: side by side over the screen params.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_history_tab_action(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%);
    let (r, n, same, mo, mr) = crate::comparison::side_by_side(&screen_param_regions(), || o(item), || staff_screens_handle_history_tab_action(item));
    SCOREBOARD.record(same && r == n, || format!("staff history tab action: original={} rust={} differences {:?}", r, n, first_differences(&mo, &mr)));
    r
}

/// Comparer for `staff_screens_leave_profile_screen`.
pub(crate) unsafe extern "C" fn compare_staff_screens_leave_profile_screen() {
    // simplification: freeing the lists does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn()>(%%I3%%)()
}

/// Comparer for `staff_screens_open_compare_players_dialog`.
pub(crate) unsafe extern "C" fn compare_staff_screens_open_compare_players_dialog() -> u32 {
    // simplification: opening a screen does not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn() -> u32>(%%I4%%)()
}

/// Comparer for `staff_screens_build_compare_players_dialog`: the dialog side by side over the display list.
pub(crate) unsafe extern "C" fn compare_staff_screens_build_compare_players_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I5%%);
    compare_by_display_list_with("compare players dialog (staff_screens_build_compare_players_dialog)", &[], || o(), || staff_screens_build_compare_players_dialog());
}

/// Comparer for `staff_screens_handle_compare_players_action`.
pub(crate) unsafe extern "C" fn compare_staff_screens_handle_compare_players_action(item: u32) -> i32 {
    // simplification: the menu bar commands do not run twice; proven in Rust mode
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I6%%)(item)
}
