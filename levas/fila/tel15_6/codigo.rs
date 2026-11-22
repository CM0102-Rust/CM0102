// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 5): the player instructions dialog ----------

/// The player instruction rows: (heading, [(choice text, takes the empty argument, the bit is in the high dword of
/// the 64-bit player instructions, flag bit)]): Passing, Tackling, Pressing, Pass To, Set Pieces (A), Set Pieces (D),
/// Free Role, Forward Runs, Run With Ball, Hold Up Ball, Long Shots, Try Through Balls and Cross Ball. A low-dword
/// choice is event 4, a high-dword one event 7, the bit the value.
const PLAYER_INSTRUCTION_ROWS: [(usize, &[(usize, bool, bool, u32)]); 13] = [
    (0xa8bf4c, &[(0xa8bda8, true, false, 1), (0xa8bd7c, true, false, 2), (0xa8bd50, true, false, 4), (0xa8bd24, true, false, 8), (0xa8bcf8, true, false, 0x10)]),
    (0xa8bf40, &[(0xa8bcd8, true, false, 0x200), (0xa8bcb8, true, false, 0x400), (0xa8bc98, true, false, 0x800), (0xa8bc78, true, false, 0x1000)]),
    (0xa8bf34, &[(0xa8bc58, true, false, 0x2000), (0xa8bc3c, true, false, 0x4000), (0xa8bc1c, true, false, 0x8000)]),
    (0xa8bf28, &[(0xa8bbfc, true, false, 0x20), (0xa8bbdc, true, false, 0x40), (0xa8bbbc, true, false, 0x80), (0xa8bb9c, true, false, 0x100)]),
    (0xa8bf18, &[(0xa8bb5c, true, false, 0x10000), (0xa8bb24, true, false, 0x4000_0000), (0xa8bae4, true, false, 0x20000)]),
    (0xa8bf08, &[(0xa8baa4, true, false, 0x40000), (0xa8ba6c, true, false, 0x80000), (0xa8ba2c, true, false, 0x8000_0000)]),
    (0xa8befc, &[(0x98e8cc, false, false, 0x10_0000), (0x98e8d0, false, false, 0x20_0000)]),
    (0xa8beec, &[(0x98e8cc, false, false, 0x40_0000), (0x98e8d0, false, false, 0x80_0000)]),
    (0xa8bedc, &[(0x98e8cc, false, false, 0x100_0000), (0x98e8d0, false, false, 0x200_0000)]),
    (0xa8becc, &[(0x98e8cc, false, false, 0x400_0000), (0x98e8d0, false, false, 0x800_0000)]),
    (0xa8bec0, &[(0x98e8cc, false, false, 0x1000_0000), (0x98e8d0, false, false, 0x2000_0000)]),
    (0xa8beac, &[(0x98e8cc, false, true, 2), (0x98e8d0, false, true, 1)]),
    (0xa8bea0, &[(0x98e8cc, false, true, 8), (0x98e8d0, false, true, 4)]),
];

/// `Tactics_Screen::get_slot_number_and_name_text(row, out, size, name format, opponent)` (0x8aa830).
unsafe fn tactics_dialog_number_and_name(ts: usize, row: u8, out: *mut u8, size: u32, format: u32, opponent: u32) {
    let g: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(tactics_dialog_fns::GET_SLOT_NUMBER_AND_NAME_TEXT);
    g(ts, row as u32, out, size, format, opponent);
}

/// The position slot of a row (`get_slot_pitch_position(row, out, opponent 0)`; 0 when the row is not on the pitch:
/// the original leaves its local unset there).
unsafe fn tactics_dialog_row_slot(ts: usize, row: u8) -> u8 {
    let mut slot = 0u8;
    tactics_calls::slot_pitch_position(ts, row as i8, &mut slot);
    slot
}

/// Builds the player instructions dialog of a row (parameter 0 the tactics screen's object, 1 the row).
///
/// The first build sets the choice green (`tactics_dialog_statics`). No background picture; the whole-screen area and
/// the dialog box (dark green rgb 0, 0x38, 0); the title "<number> <name>" of the row (format 1) and a popup button
/// listing the other rows on the pitch (event 1, the row as value) to switch player. Cancel (event 2) and Ok (3); a
/// pitch (`draw_pitch`, scale 4) in the buttons area. The instruction area (6 columns, weights 5, 2, 2, 2, 2, 2; 14
/// rows) with the 13 rows of `PLAYER_INSTRUCTION_ROWS` for the row's position slot (`Formation::get_player_
/// instructions`, 64 bits): each choice ticked when its bit is set, else a button (there is no read-only case). The
/// marking row (row 13, weights 0x28, 0x10, 0x10, 0x10, 0x1b, 5; `Formation::get_marking_type`): Zonal (event 5,
/// value 2), None (1) and Man (4), ticked by the marking type's bits. Man marking adds a popup of the opponents (the
/// match squad of a record, on the pitch in a live match; event 6, the player), the marked player (marking bit 8,
/// `Formation::get_marking_target`) shown as a linked button, the text "  Opposition" or "  Opp (<number and
/// initials>)" of the marked row (`get_slot_marking_target_row`), written in column 4 with marking bit 0x10, and the
/// popup's last row with that text without its two leading spaces (value 0). The opponent index assert (line 0x1b8e)
/// cannot fail and is left out.
/// Original: `tactics_build_player_instructions_dialog` (0x8a5330, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_build_player_instructions_dialog() {
    use tactics_dialog_statics as s;
    use tactics_screen as t;
    covered!("tel15_6 build the player instructions");
    let c = tactics_colour;
    tactics_dialog_choice_colour(s::PLAYER_GUARD, s::PLAYER_CHOICE);
    let button_widths: [u8; 2] = [3, 1];
    let instruction_widths: [u8; 6] = [5, 2, 2, 2, 2, 2];
    let marking_widths: [u8; 6] = [0x28, 0x10, 0x10, 0x10, 0x1b, 5];
    let ts = get_screen_param(0);
    let row = get_screen_param(1) as u8;
    exe::background_set_picture(0usize);
    tactics_area([0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let back = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tactics_area([0x91, 0x37, 0x2e8, 0x220, 1, 0, 1, 0, 0x30, back, u32::MAX]);
    tactics_dialog_number_and_name(ts, row, JOB_TEXT as *mut u8, 0x7d0, 1, 0);
    tactics_dialog_text_item([0x9b, 0x41, 0x2de, 0x6e, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    let mut popup = u32::MAX;
    for r in 0..0x14u8 {
        if r == row || tactics_calls::slot_pitch_position(ts, r as i8, core::ptr::null_mut()) == 0 {
            continue;
        }
        tactics_dialog_number_and_name(ts, r, JOB_TEXT as *mut u8, 0x7d0, 1, 0);
        if popup as u16 == 0xffff {
            let button = tel26_add_button([0xa0, 0x46, 0xac, 0x56, 0x30, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), 0, 0, 0]);
            popup = tactics_area([0, 0, 0, 0, 0, 0, 0, 0, 0x30, c(JOB_COLOUR_31AA), button]);
        }
        tactics_dialog_popup_row(JOB_TEXT as u32, 1, r as u32, popup);
    }
    let buttons = tactics_area([0x9b, 0x1f8, 0x2de, 0x216, 2, button_widths.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98a1a0);
    tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    tactics_text(0x990f7c);
    tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 3, 0]);
    tactics_dialog_pitch(0xa5, 0x82, 0x2d4, 0x1e4, buttons, 4.0);
    let area = tactics_area([0xaa, 0x87, 0x2cf, 0x1df, 6, instruction_widths.as_ptr() as u32, 0xe, 0, 1, 0, u32::MAX]);
    let slot = tactics_dialog_row_slot(ts, row);
    let fm = ts + t::FORMATION;
    let instructions = exe::formation_get_player_instructions(fm, slot as i8);
    let (low, high) = (instructions as u32, (instructions >> 32) as u32);
    for (r, &(heading, choices)) in PLAYER_INSTRUCTION_ROWS.iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(area, r as u32);
        for (k, &(format, takes_empty, in_high, bit)) in choices.iter().enumerate() {
            tactics_dialog_choice_text(format, takes_empty);
            let checked = if in_high { high & bit != 0 } else { low & bit != 0 };
            tactics_dialog_choice(area, k as u32 + 1, r as u32, checked, true, s::PLAYER_CHOICE, if in_high { 7 } else { 4 }, bit);
        }
    }
    let marking_area = tactics_area([0xaa, 0x87, 0x2cf, 0x1df, 6, marking_widths.as_ptr() as u32, 0xe, 0, 1, 0, u32::MAX]);
    let marking = exe::formation_get_marking_type(fm, slot as i8);
    tactics_text1(0xa8be78, TACTICS_EMPTY);
    tactics_dialog_heading(marking_area, 0xd);
    tactics_text1(0xa8b9e8, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 1, 0xd, marking & 2 != 0, true, s::PLAYER_CHOICE, 5, 2);
    tactics_text1(0xa8ba0c, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 2, 0xd, marking & 1 != 0, true, s::PLAYER_CHOICE, 5, 1);
    tactics_text1(0xa8b984, TACTICS_EMPTY);
    tactics_dialog_choice(marking_area, 3, 0xd, marking & 4 != 0, true, s::PLAYER_CHOICE, 5, 4);
    if marking & 4 == 0 {
        return;
    }
    covered!("tel15_6 man marking: the opponents");
    let popup = tactics_dialog_popup(marking_area, 5, 0xd);
    let mut name = [0u8; 0xc8];
    for r in 0..0x14i8 {
        if read_u32(ts + t::HAS_MATCH_RECORD) != 0 && r >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) + 0xb) as i8 {
            continue;
        }
        let slot_of = |r: i8| tactics_screen_calls::call3(tactics_screen_fns::GET_SLOT_PITCH_POSITION, ts, r as u8 as u32, 0, 1);
        if slot_of(r) == 0 && read_usize(ts + t::MATCH) != 0 {
            continue;
        }
        let p = tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, r as u8 as u32, 1) as usize;
        if p == 0 {
            continue;
        }
        if marking & 8 != 0 && exe::formation_get_marking_target(fm, slot as i8) == p {
            exe::database_format_staff_name(p, name.as_mut_ptr() as usize, 0xc8, 2);
            let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
            f(JOB_TEXT, 0x987ff0, name.as_ptr());
            let link = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
            tel26_area_button([marking_area, 4, 0xd, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), JOB_TEXT as u32, link, p as u32]);
        }
        tactics_dialog_number_and_name(ts, r as u8, JOB_TEXT as *mut u8, 0x7d0, 1, 1);
        tactics_dialog_popup_row(JOB_TEXT as u32, 6, p as u32, popup);
    }
    tactics_text(0xa8c500);
    let get_row: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(tactics_screen_fns::GET_SLOT_MARKING_TARGET_ROW);
    let target = get_row(ts, row as u32, 0) as i8;
    if target > -1 && tactics_screen_calls::call2(tactics_screen_fns::GET_SLOT_PLAYER, ts, target as u8 as u32, 1) != 0 {
        tactics_dialog_number_and_name(ts, target as u8, name.as_mut_ptr(), 0xc8, 6, 1);
        tactics_text1(0xa8c4d4, name.as_ptr() as usize);
    }
    if marking & 0x10 != 0 {
        tel26_area_text([marking_area, 4, 0xd, 1, 0, 1, 2, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
    }
    tactics_dialog_popup_row(JOB_TEXT as u32 + 2, 6, 0, popup);
}

/// The events of the player instructions dialog (parameter 0 the tactics screen's object it edits, 1 the row, 2 a
/// Tactics_Screen copy taken when it opened).
///
/// Another player (1) puts the item's value in parameter 1 and redraws. Cancel (2) copies the copy back over the
/// screen's object (`tactics_screen_assign`); Ok (3) clears the object's undo flag; both throw the copy away and close
/// (-9). On the row's position slot: a low-dword instruction (4) on an editable object saves the undo copy and sets
/// it (`Formation::set_player_instruction(slot, value, 0)`, the slot argument keeping the upper bytes of the item
/// as the original's reused stack slot does): refused, the undo copy goes back, else the team data is marked changed;
/// a high-dword one (7) through `Tactics_Screen::set_player_instruction(slot, 0, value)`; a marking type (5)
/// through `set_slot_marking_type`; a marked opponent (6): the player with type 8 (`set_slot_marking_type`, then
/// `set_slot_marking_target`), none with type 0x10 and target 0. These redraw (-0xb). Anything else goes to the menu
/// bar (`menubar_handle_object_link`).
/// Original: `tactics_player_instructions_dialog_handle_event` (0x8a6f80, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_player_instructions_dialog_handle_event(item: u32) -> i32 {
    use tactics_dialog_fns as f;
    use tactics_screen as t;
    let ts = get_screen_param(0);
    let row = get_screen_param(1) as u8;
    let copy = get_screen_param(2);
    let it = item as i16;
    match tel26_item_id(it) {
        1 => {
            covered!("tel15_6 player instructions: another player");
            set_screen_param(1, tel26_item_value(it) as usize);
            TACTICS_DIALOG_REDRAW
        }
        2 => {
            covered!("tel15_6 player instructions cancel");
            tactics_screen_assign(ts, copy);
            tactics_dialog_close(copy)
        }
        3 => {
            covered!("tel15_6 player instructions ok");
            write_u32(ts + t::CAN_UNDO, 0);
            tactics_dialog_close(copy)
        }
        4 => {
            let slot = tactics_dialog_row_slot(ts, row);
            let value = tel26_item_value(it);
            if read_u32(ts + t::READ_ONLY) != 0 {
                return TACTICS_DIALOG_REDRAW;
            }
            covered!("tel15_6 a player instruction");
            tactics_save_undo(ts);
            let slot_arg = (item & 0xffff_ff00) | slot as u32;
            if tactics_screen_calls::call3(tactics_screen_fns::FORMATION_SET_PLAYER_INSTRUCTION, ts + t::FORMATION, slot_arg, value, 0) == 0 {
                tactics_restore_undo(ts);
            } else {
                write_u32(ts + tactics_screen_state::TEAM_DATA_CHANGED, 1);
            }
            TACTICS_DIALOG_REDRAW
        }
        7 => {
            covered!("tel15_6 a high player instruction");
            let slot = tactics_dialog_row_slot(ts, row);
            tactics_screen_calls::call3(f::SET_PLAYER_INSTRUCTION, ts, slot as u32, 0, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        5 => {
            covered!("tel15_6 a marking type");
            let slot = tactics_dialog_row_slot(ts, row);
            tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot as u32, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        6 => {
            covered!("tel15_6 a marked opponent");
            let slot = tactics_dialog_row_slot(ts, row) as u32;
            let target = tel26_item_value(it);
            if target != 0 {
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot, 8);
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TARGET, ts, slot, target);
            } else {
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TYPE, ts, slot, 0x10);
                tactics_screen_calls::call2(f::SET_SLOT_MARKING_TARGET, ts, slot, 0);
            }
            TACTICS_DIALOG_REDRAW
        }
        _ => exe::menubar_handle_object_link(item),
    }
}

/// Comparer for `tactics_build_player_instructions_dialog`: side by side over the built display list
/// (`compare_match_screen_build`), the dialog's and the pitch's static colours.
pub(crate) unsafe extern "C" fn compare_tactics_build_player_instructions_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let extra = [(tactics_dialog_statics::PLAYER_GUARD, 1), (tactics_dialog_statics::PLAYER_CHOICE, 2), (0xde1e20, 1), (0xde1f34, 2), (0xde1e24, 2)];
    compare_match_screen_build("tactics_build_player_instructions_dialog", &extra, || o(), || tactics_build_player_instructions_dialog());
}

// simplification: another player writes the screen parameters, Cancel and Ok destroy and free the copy and the menu
// bar can open another screen: only the original runs; the plan: parameter 1 takes the value, Cancel leaves the working
// team data equal to the copy's, Ok clears the undo flag, both close (-9). The instruction and marking events edit
// the screen's object: side by side over it.
/// Comparer for `tactics_player_instructions_dialog_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_player_instructions_dialog_handle_event(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let it = item as i16;
    let id = tel26_item_id(it);
    let (ts, copy) = (get_screen_param(0), get_screen_param(2));
    match id {
        4..=7 if ts != 0 => compare_tactics_method("player instructions event", &[tactics_screen_region(ts)], || o(item),
                                                   || tactics_player_instructions_dialog_handle_event(item)),
        1 => {
            let value = tel26_item_value(it) as usize;
            let r = o(item);
            SCOREBOARD.record(r == TACTICS_DIALOG_REDRAW && get_screen_param(1) == value, || format!("player instructions event 1: {} parameter 1 {:x}", r, get_screen_param(1)));
            r
        }
        2 | 3 if ts != 0 => {
            let n = tactics_screen_state::TEAM_DATA_SIZE;
            let kept = if id == 2 && copy != 0 { core::slice::from_raw_parts((copy + tactics_screen::FORMATION) as *const u8, n).to_vec() } else { Vec::new() };
            let r = o(item);
            let same = if id == 2 {
                copy == 0 || core::slice::from_raw_parts((ts + tactics_screen::FORMATION) as *const u8, n) == &kept[..]
            } else {
                read_u32(ts + tactics_screen::CAN_UNDO) == 0
            };
            SCOREBOARD.record(r == TACTICS_DIALOG_CLOSED && same, || format!("player instructions event {}: {} plan kept {}", id, r, same));
            r
        }
        _ => o(item),
    }
}
