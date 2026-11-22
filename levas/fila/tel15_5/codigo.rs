// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 4): the team instructions dialog ----------

/// The static colours of the two instruction dialogs, set on the first build (bit 0 of each guard): the green of the
/// choices (rgb 0, 0xc0, 0) of the team instructions and of the player instructions.
mod tactics_dialog_statics {
    pub(crate) const TEAM_GUARD: usize = 0xde1d10;
    pub(crate) const TEAM_CHOICE: usize = 0xde1e26;
    pub(crate) const PLAYER_GUARD: usize = 0xde1e28;
    pub(crate) const PLAYER_CHOICE: usize = 0xde1e2a;
}

/// The results of the dialogs' event handlers: the dialog closed (-9) and redraw (-0xb).
const TACTICS_DIALOG_CLOSED: i32 = -9;
const TACTICS_DIALOG_REDRAW: i32 = -0xb;

/// The Tactics_Screen routines the dialogs call by address.
#[allow(dead_code)]
mod tactics_dialog_fns {
    pub(crate) const SET_SET_PIECE_TAKER: usize = 0x8a8ed0;
    pub(crate) const SET_PLAYMAKER: usize = 0x8a9020;
    pub(crate) const SET_PLAYER_INSTRUCTION: usize = 0x8a9c20;
    pub(crate) const SET_SLOT_MARKING_TYPE: usize = 0x8aa070;
    pub(crate) const SET_SLOT_MARKING_TARGET: usize = 0x8aa150;
    pub(crate) const GET_SLOT_NUMBER_AND_NAME_TEXT: usize = 0x8aa830;
    pub(crate) const MATCH_STATS_RECORD_ASSIGN: usize = 0x8a5010;
}

/// Sets a dialog's choice colour the first time (bit 0 of `guard`, set before the colour is made).
unsafe fn tactics_dialog_choice_colour(guard: usize, colour: usize) {
    let g = read_u8(guard);
    if g & 1 == 0 {
        write_u8(guard, g | 1);
        write_u16(colour, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
}

/// `ScreenManager::add_text_item` (0x803fd0): x1, y1, x2, y2, five style fields, colour, text, alignment.
unsafe fn tactics_dialog_text_item(f: [u32; 11]) -> u32 {
    type AddText = unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32;
    let g: AddText = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    g(G_SCREEN_MANAGER, f[0], f[1], f[2], f[3], f[4], f[5], f[6], f[7], f[8], f[9], f[10])
}

/// The heading of an instruction row (column 0, in the heading red).
unsafe fn tactics_dialog_heading(area: u32, row: u32) {
    tel26_area_text([area, 0, row, 1, 0, 1, 3, tactics_colour(TACTICS_COLOUR_3154), JOB_TEXT as u32]);
}

/// One choice of an instruction row, its text already in `JOB_TEXT`: the ticked one in the tick colours (style
/// 0x221), else a button with the event `id` and its `value` (editable) or the plain text (read-only), both in the
/// dialog's choice colour.
unsafe fn tactics_dialog_choice(area: u32, col: u32, row: u32, checked: bool, editable: bool, colour: usize, id: u32, value: u32) {
    let tick = tactics_colour(JOB_COLOUR_BDEC);
    if checked {
        tel26_area_text([area, col, row, 0x221, tick, 0xc, 2, tick, JOB_TEXT as u32]);
    } else if editable {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, tactics_colour(colour), JOB_TEXT as u32, id, value]);
    } else {
        tel26_area_text([area, col, row, 1, 0, 0xc, 2, tactics_colour(colour), JOB_TEXT as u32]);
    }
}

/// The choice's text: the translated format, with the empty string as its argument when it takes one.
unsafe fn tactics_dialog_choice_text(format: usize, takes_empty: bool) {
    if takes_empty {
        tactics_text1(format, TACTICS_EMPTY);
    } else {
        tactics_text(format);
    }
}

/// `draw_pitch` (0x8ac600) by address: the pitch under a dialog, with the goals left and right; the four outputs
/// are not used.
unsafe fn tactics_dialog_pitch(left: i32, top: i32, right: i32, bottom: i32, area: u32, scale: f32) {
    let draw: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, f32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let mut r = [0i32; 4];
    let p = r.as_mut_ptr();
    draw(left, top, right, bottom, p, p.add(1), p.add(2), p.add(3), 1, area, scale);
}

/// `Tactics_Screen::get_slot_number_text(row, out, 0xc8, no fallback 0, opponent 0)` (0x8aa200) into `out`.
unsafe fn tactics_dialog_number_text(ts: usize, row: i8, out: *mut u8) {
    let g: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, u32) -> u32 = core::mem::transmute(tactics_screen_fns::GET_SLOT_NUMBER_TEXT);
    g(ts, row as u8 as u32, out, 0xc8, 0, 0);
}

/// "<number> <name>" of a player (`sprintf(JOB_TEXT, "%s %s")`, the CRT's) for a popup row; the name in format 2.
unsafe fn tactics_dialog_popup_text(player: usize, number: *const u8, name: *mut u8) {
    exe::database_format_staff_name(player, name as usize, 0xc8, 2);
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(JOB_TEXT, 0x98f0b8, number, name as *const u8);
}

/// A popup row of the current human's display: kind 2 with its text (0: the empty row), the event `id` and `value`.
unsafe fn tactics_dialog_popup_row(text: u32, id: u32, value: u32, popup: u32) {
    tactics_item([2, 0, 0, 0, 0, 0, 0, 0x10, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), 1, 1, tactics_colour(JOB_COLOUR_BDEA), text, 0, id,
        value, popup]);
}

/// The popup of a choice list: a button in `area` at (`col`, `row`) and the popup area it opens.
unsafe fn tactics_dialog_popup(area: u32, col: u32, row: u32) -> u32 {
    let button = tel26_area_button([area, col, row, 0x30, tactics_colour(JOB_COLOUR_BDEA), 0xc, 1, tactics_colour(JOB_COLOUR_3184), 0, 0, 0]);
    tactics_area([0, 0, 0, 0, 0, 0, 0, 0, 0x500030, tactics_colour(JOB_COLOUR_31AA), button])
}

/// The chosen player of a taker or playmaker choice under its popup: "  <name>" (first choice, the CRT's sprintf) or
/// "  or <name>" (second), linked to the player (national team or club kind), greyed (style 0x21) when he is not on
/// the pitch.
unsafe fn tactics_dialog_chosen(ts: usize, area: u32, col: u32, row: u32, second: bool, chosen: u32, on_pitch: bool, name: *mut u8) {
    exe::database_format_staff_name(chosen as usize, name as usize, 0xc8, 2);
    if second {
        tactics_text1(0xa8c3b8, name as usize);
    } else {
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(JOB_TEXT, 0x987ff0, name as *const u8);
    }
    let link = if exe::database_is_national_team(read_usize(ts + tactics_screen::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
    tel26_area_button([area, col, row, 1, 0, if on_pitch { 1 } else { 0x21 }, 2, tactics_colour(JOB_COLOUR_BDEC), JOB_TEXT as u32, link, chosen]);
}

/// The team instruction rows: (heading, [(choice text, takes the empty argument, flag bit of the formation's team
/// instructions)]): Mentality, Passing, Tackling, Pressing, Offside Trap, Counter Attack and Men Behind Ball.
const TEAM_INSTRUCTION_ROWS: [(usize, &[(usize, bool, u32)]); 7] = [
    (0xa8c4c8, &[(0xa8c4a0, true, 0x20000), (0xa8c478, true, 0x8000), (0xa8c450, true, 0x10000)]),
    (0xa8bf4c, &[(0xa8bd7c, true, 1), (0xa8bd50, true, 2), (0xa8bd24, true, 4), (0xa8bcf8, true, 8)]),
    (0xa8bf40, &[(0xa8bcb8, true, 0x10), (0xa8bc98, true, 0x20), (0xa8bc78, true, 0x40)]),
    (0xa8bf34, &[(0x98e8cc, false, 0x80), (0x98e8d0, false, 0x100)]),
    (0xa8c440, &[(0x98e8cc, false, 0x200), (0x98e8d0, false, 0x400)]),
    (0xa8c430, &[(0x98e8cc, false, 0x800), (0x98e8d0, false, 0x1000)]),
    (0xa8c41c, &[(0x98e8cc, false, 0x2000), (0x98e8d0, false, 0x4000)]),
];

/// The set piece taker choices (`g_team_instr_row_*` at 0x97039c): (kind, choice, event id): free kicks left and
/// right, corners left and right, each first choice and "or" second choice.
const TEAM_TAKER_CHOICES: [(u32, u32, u32); 8] = [(0, 0, 4), (0, 1, 5), (1, 0, 6), (1, 1, 7), (2, 0, 8), (2, 1, 9), (3, 0, 0xa), (3, 1, 0xb)];

/// Builds the team instructions dialog over the tactics screen (the Tactics_Screen copy of parameter 0).
///
/// The first build sets the choice green (`tactics_dialog_statics`). No background picture; the whole-screen area,
/// then the dialog box (0x145 high read-only, else 0x1ea, centred in 0x258; dark green rgb 0, 0x38, 0), the title
/// "Team Instructions", the buttons row at its bottom: Cancel (event 1) and Ok (2), or only Ok across it read-only.
/// A pitch (`draw_pitch`, scale 2) in the buttons area. The instruction area (5 columns, weights 2, 1, 1, 1, 1; 12
/// rows, 7 read-only) with the seven rows of `TEAM_INSTRUCTION_ROWS`: the heading and each choice, ticked when its bit
/// is set in the formation's team instruction flags, else a button with event 3 and the bit (read-only: the text).
/// Editable, the set piece area (weights 8, 7, 1, 7, 1) with the headings Free Kicks (L), (R), Corners (L), (R) and
/// Playmaker on rows 7..11, and for each taker choice of `TEAM_TAKER_CHOICES` (a second choice only once the first
/// is set) a popup next to it listing the players on the pitch ("<number> <name>", event the choice's id, value the
/// player) but the one chosen for the other choice, plus an empty row; under it the chosen taker
/// (`tactics_dialog_chosen`, greyed when not on the pitch). The two playmaker choices the same with events 0xe and
/// 0xf. Asserts on the taker and playmaker indices that cannot fail (lines 0x1618 and 0x165f) are left out.
/// Original: `tactics_build_team_instructions_dialog` (0x8a2db0, tactics_screens.cpp, cdecl; 0x8a4830 jumps here).
pub(crate) unsafe extern "C" fn tactics_build_team_instructions_dialog() {
    use tactics_dialog_statics as s;
    use tactics_screen as t;
    covered!("tel15_5 build the team instructions");
    let c = tactics_colour;
    tactics_dialog_choice_colour(s::TEAM_GUARD, s::TEAM_CHOICE);
    let widths: [u8; 7] = [3, 1, 2, 1, 1, 1, 1];
    let taker_widths: [u8; 5] = [8, 7, 1, 7, 1];
    let ts = get_screen_param(0);
    exe::background_set_picture(0usize);
    tactics_area([0, 0, 0x31f, 0x257, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let read_only = || read_u32(ts + t::READ_ONLY) != 0;
    let height: i32 = if read_only() { 0x145 } else { 0x1ea };
    let mut top = (0x258 - height) / 2;
    let mut bottom = top + height - 1;
    let back = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tactics_area([0xaa, top as u32, 0x2cf, bottom as u32, 1, 0, 1, 0, 0x30, back, u32::MAX]);
    tactics_text(0xa8bfa4);
    top += 10;
    bottom -= 10;
    tactics_dialog_text_item([0xb4, top as u32, 0x2c5, (top + 0x2d) as u32, 0x30, c(JOB_COLOUR_BDEA), 0xc, 4, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1]);
    let buttons_top = bottom - 0x1e;
    let buttons = tactics_area([0xb4, buttons_top as u32, 0x2c5, bottom as u32, 2, widths.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    top += 0x37;
    if !read_only() {
        tactics_text(0x98a1a0);
        tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
        tactics_text(0x990f7c);
        tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    } else {
        tactics_text(0x990f7c);
        tel26_add_button([0xb4, buttons_top as u32, 0x2c5, bottom as u32, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 2, 0]);
    }
    bottom -= 0x32;
    top += 10;
    tactics_dialog_pitch(0xbe, top, 0x2bb, bottom, buttons, 2.0);
    top += 5;
    bottom -= 5;
    let area = tactics_area([0xc3, top as u32, 0x2b6, bottom as u32, 5, widths[2..].as_ptr() as u32, if read_only() { 7 } else { 0xc }, 0, 1, 0, u32::MAX]);
    let flags = read_u32(ts + t::FORMATION + formation_fields::FLAGS);
    for (row, &(heading, choices)) in TEAM_INSTRUCTION_ROWS.iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(area, row as u32);
        for (k, &(format, takes_empty, bit)) in choices.iter().enumerate() {
            tactics_dialog_choice_text(format, takes_empty);
            tactics_dialog_choice(area, k as u32 + 1, row as u32, flags & bit != 0, !read_only(), s::TEAM_CHOICE, 3, bit);
        }
    }
    if read_only() {
        return;
    }
    covered!("tel15_5 the set piece takers and playmakers");
    let takers = tactics_area([0xc3, top as u32, 0x2b6, bottom as u32, 5, taker_widths.as_ptr() as u32, 0xc, 0, 1, 0, u32::MAX]);
    for (k, heading) in [0xa8c40c, 0xa8c3fc, 0xa8c3ec, 0xa8c3dc, 0xa8c3d0].into_iter().enumerate() {
        tactics_text(heading);
        tactics_dialog_heading(takers, 7 + k as u32);
    }
    let (mut number, mut name) = ([0u8; 0xc8], [0u8; 0xc8]);
    let (mut col, mut row) = (1u32, 7u32);
    let taker = |kind: u32, choice: u32| read_u32(ts + tactics_screen_state::SET_PIECE_TAKERS + ((kind * 2 + choice) * 4) as usize);
    for &(kind, choice, id) in TEAM_TAKER_CHOICES.iter() {
        let mut on_pitch = false;
        let mut popup = u32::MAX;
        if choice == 0 || taker(kind, 0) != 0 {
            for r in 0..0x14i8 {
                if tactics_calls::slot_pitch_position(ts, r, core::ptr::null_mut()) == 0 {
                    continue;
                }
                let p = tactics_calls::slot_player(ts, r);
                if p == 0 {
                    continue;
                }
                tactics_dialog_number_text(ts, r, number.as_mut_ptr());
                if p as u32 == taker(kind, choice) {
                    on_pitch = true;
                }
                if p as u32 == taker(kind, 1 - choice) {
                    continue;
                }
                if popup as u16 == 0xffff {
                    popup = tactics_dialog_popup(takers, col + 1, row);
                }
                tactics_dialog_popup_text(p, number.as_ptr(), name.as_mut_ptr());
                tactics_dialog_popup_row(JOB_TEXT as u32, id, p as u32, popup);
            }
            if popup as u16 != 0xffff {
                tactics_dialog_popup_row(0, id, 0, popup);
            }
        }
        let chosen = taker(kind, choice);
        if chosen != 0 {
            tactics_dialog_chosen(ts, takers, col, row, choice > 0, chosen, on_pitch, name.as_mut_ptr());
        }
        col += 2;
        if col > 3 {
            col = 1;
            row += 1;
        }
    }
    let playmaker = |k: u32| read_u32(ts + tactics_screen_state::PLAYMAKERS + (k * 4) as usize);
    let mut popup_col = col + 1;
    for choice in 0..2u32 {
        let mut on_pitch = false;
        let mut popup = u32::MAX;
        let id = 0xe + (choice != 0) as u32;
        if choice == 0 || playmaker(0) != 0 {
            for r in 0..0x14i8 {
                if tactics_calls::slot_pitch_position(ts, r, core::ptr::null_mut()) == 0 {
                    continue;
                }
                let p = tactics_calls::slot_player(ts, r);
                if p == 0 {
                    continue;
                }
                tactics_dialog_number_text(ts, r, number.as_mut_ptr());
                if p as u32 == playmaker(choice) {
                    on_pitch = true;
                }
                if p as u32 == playmaker((choice == 0) as u32) {
                    continue;
                }
                if popup as u16 == 0xffff {
                    popup = tactics_dialog_popup(takers, popup_col, row);
                }
                tactics_dialog_popup_text(p, number.as_ptr(), name.as_mut_ptr());
                tactics_dialog_popup_row(JOB_TEXT as u32, id, p as u32, popup);
            }
            if popup as u16 != 0xffff {
                tactics_dialog_popup_row(0, id, 0, popup);
            }
        }
        let chosen = playmaker(choice);
        if chosen != 0 {
            tactics_dialog_chosen(ts, takers, col, row, choice != 0, chosen, on_pitch, name.as_mut_ptr());
        }
        col += 2;
        popup_col += 2;
    }
}

/// Copies one Tactics_Screen over another field by field (the compiler's assignment, inline in the dialogs'
/// handlers): the arguments, the match record (`match_stats_record_assign`), the fixture copy, the record flag, the
/// opponent, the sides, the kit, the active tactic, the opponent's line-up, the read-only flag, the three team data
/// blocks, the copied zone and phase and the paste and undo flags.
unsafe fn tactics_screen_assign(dst: usize, src: usize) {
    use tactics_screen as t;
    use tactics_screen_state as s;
    let copy = |at: usize, n: usize| core::ptr::copy((src + at) as *const u8, (dst + at) as *mut u8, n);
    copy(0, 0x18);
    tactics_screen_calls::call1(tactics_dialog_fns::MATCH_STATS_RECORD_ASSIGN, dst + s::RECORD, (src + s::RECORD) as u32);
    copy(s::FIXTURE_COPY, 0x4f);
    copy(t::HAS_MATCH_RECORD, 4);
    copy(s::OPPONENT, 4);
    copy(t::SIDE, 1);
    copy(s::OPPONENT_SIDE, 1);
    copy(t::KIT, 1);
    copy(s::ACTIVE_TACTIC, 4);
    copy(s::OPPONENT_LINEUP, 0x50);
    copy(t::READ_ONLY, 4);
    copy(t::FORMATION, s::TEAM_DATA_SIZE);
    copy(t::UNDO_DATA, s::TEAM_DATA_SIZE);
    copy(t::CLIPBOARD_DATA, s::TEAM_DATA_SIZE);
    copy(s::COPIED_ZONE, 0x2c);
    copy(s::COPIED_PHASE, 0x210);
    copy(t::CAN_PASTE, 1);
    copy(t::CAN_UNDO, 4);
}

/// Closes a dialog: the Tactics_Screen copy, when there is one, destroyed (`~Tactics_Screen` by address) and freed;
/// the screen closed (-9). The parameter is left as it is.
unsafe fn tactics_dialog_close(copy: usize) -> i32 {
    if copy != 0 {
        display_method(TACTICS_DESTRUCT_FN, copy);
        exe::crt_operator_delete(copy);
    }
    TACTICS_DIALOG_CLOSED
}

/// The events of the team instructions dialog (parameter 0 the Tactics_Screen copy it edits, 1 the tactics screen's
/// own object, 0 read-only).
///
/// Cancel (1) throws the copy away. Ok (2) on an editable copy clears its undo flag and copies it over the screen's
/// object (`tactics_screen_assign`), then throws it away; both close (-9). A choice (3) on an editable copy saves the
/// undo copy and sets the team instruction (`Formation::set_team_instruction` with the item's value): refused, the
/// undo copy goes back (Undo withdrawn), else the team data is marked changed. A taker choice sets the set piece
/// taker of the copy (`set_set_piece_taker`): ids 4..0xd are kinds 0..4 (free kicks left and right, corners left and
/// right, penalties), even ids the first choice and odd the second; 0xe and 0xf the first and second playmaker
/// (`set_playmaker`), the player the item's value. These redraw (-0xb). Anything else goes to the menu bar
/// (`menubar_handle_object_link`).
/// Original: `tactics_team_instructions_dialog_handle_event` (0x8a4840, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_team_instructions_dialog_handle_event(item: u32) -> i32 {
    use tactics_screen as t;
    let copy = get_screen_param(0);
    let owner = get_screen_param(1);
    let it = item as i16;
    let id = tel26_item_id(it);
    match id {
        1 => {
            covered!("tel15_5 team instructions cancel");
            tactics_dialog_close(copy)
        }
        2 => {
            covered!("tel15_5 team instructions ok");
            if read_u32(copy + t::READ_ONLY) == 0 {
                write_u32(copy + t::CAN_UNDO, 0);
                tactics_screen_assign(owner, copy);
            }
            tactics_dialog_close(copy)
        }
        3 => {
            let value = tel26_item_value(it);
            if read_u32(copy + t::READ_ONLY) != 0 {
                return TACTICS_DIALOG_REDRAW;
            }
            covered!("tel15_5 a team instruction");
            tactics_save_undo(copy);
            if tactics_screen_calls::call1(FN_FORMATION_SET_TEAM_INSTRUCTION, copy + t::FORMATION, value) == 0 {
                tactics_restore_undo(copy);
            } else {
                write_u32(copy + tactics_screen_state::TEAM_DATA_CHANGED, 1);
            }
            TACTICS_DIALOG_REDRAW
        }
        4..=0xd => {
            covered!("tel15_5 a set piece taker");
            let target = get_screen_param(0);
            let k = id as u32 - 4;
            tactics_screen_calls::call3(tactics_dialog_fns::SET_SET_PIECE_TAKER, target, k / 2, k % 2, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        0xe | 0xf => {
            covered!("tel15_5 a playmaker");
            let target = get_screen_param(0);
            tactics_screen_calls::call2(tactics_dialog_fns::SET_PLAYMAKER, target, id as u32 - 0xe, tel26_item_value(it));
            TACTICS_DIALOG_REDRAW
        }
        _ => exe::menubar_handle_object_link(item),
    }
}

/// Comparer for `tactics_build_team_instructions_dialog`: side by side over the built display list
/// (`compare_match_screen_build`), the dialog's and the pitch's static colours.
pub(crate) unsafe extern "C" fn compare_tactics_build_team_instructions_dialog() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let extra = [(tactics_dialog_statics::TEAM_GUARD, 1), (tactics_dialog_statics::TEAM_CHOICE, 2), (0xde1e20, 1), (0xde1f34, 2), (0xde1e24, 2)];
    compare_match_screen_build("tactics_build_team_instructions_dialog", &extra, || o(), || tactics_build_team_instructions_dialog());
}

// simplification: Cancel and Ok destroy and free the copy, and the menu bar can open another screen: only the
// original runs; the plan: Ok leaves the screen's working team data equal to the copy's, both close (-9). The
// choices edit the copy: side by side over it.
/// Comparer for `tactics_team_instructions_dialog_handle_event`.
pub(crate) unsafe extern "C" fn compare_tactics_team_instructions_dialog_handle_event(item: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I1%%);
    let id = tel26_item_id(item as i16);
    let (copy, owner) = (get_screen_param(0), get_screen_param(1));
    match id {
        3..=0xf if copy != 0 => compare_tactics_method("team instructions event", &[tactics_screen_region(copy)], || o(item),
                                                       || tactics_team_instructions_dialog_handle_event(item)),
        1 | 2 => {
            let n = tactics_screen_state::TEAM_DATA_SIZE;
            let applies = id == 2 && copy != 0 && owner != 0 && read_u32(copy + tactics_screen::READ_ONLY) == 0;
            let kept = if applies { core::slice::from_raw_parts((copy + tactics_screen::FORMATION) as *const u8, n).to_vec() } else { Vec::new() };
            let r = o(item);
            let same = !applies || core::slice::from_raw_parts((owner + tactics_screen::FORMATION) as *const u8, n) == &kept[..];
            SCOREBOARD.record(r == TACTICS_DIALOG_CLOSED && same, || format!("team instructions event {}: {} applied {}", id, r, same));
            r
        }
        _ => o(item),
    }
}
