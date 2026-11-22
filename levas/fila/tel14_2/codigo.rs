// ---------- batch %%LEVA%%: the tactics screen (tactics_screens.cpp, 2): building it ----------

/// Colours of the screen items not named in tel26_1 (words, zero-extended when pushed).
const TACTICS_COLOUR_BDD6: usize = 0xaebdd6;
const TACTICS_COLOUR_3154: usize = 0xae3154;
const TACTICS_COLOUR_31A8: usize = 0xae31a8;

/// The static colours of `tactics_screen_build`, set on the first build (guard bits in `GUARD` and `GUARD2`).
#[allow(dead_code)]
mod tactics_build_statics {
    pub(crate) const GUARD: usize = 0xde1f30;
    pub(crate) const GUARD2: usize = 0xde1e1a;
    /// The dark blue of the bench numbers (rgb 0, 0, 0x40).
    pub(crate) const BENCH_NUMBER: usize = 0xde1f36;
    /// The pitch greens (rgb 0, 0x90 / 0x60 / 0x80, 0) and the dark green of a pending player (rgb 0, 0x40, 0).
    pub(crate) const GREEN_90: usize = 0xde1e18;
    pub(crate) const GREEN_60: usize = 0xde1e1e;
    pub(crate) const GREEN_80: usize = 0xde1f32;
    pub(crate) const GREEN_40: usize = 0xde1d0e;
    /// Copies of the colours 0xae3154, 0xaebdd4, 0xaebdec and 0xae317e.
    pub(crate) const RED: usize = 0xde1d0c;
    pub(crate) const RUN: usize = 0xde1f38;
    pub(crate) const ZONE: usize = 0xde1e1c;
    pub(crate) const HOVER_ZONE: usize = 0xde1e22;
}

/// The flag byte that offers the preset formations ("Load Preset") [inference].
const TACTICS_PRESETS_ON: usize = 0xb7434a;
/// The empty string.
const TACTICS_EMPTY: usize = 0xad9c64;

/// `Display::add_item` on the current human's Display (the 18 fields); returns eax.
unsafe fn tactics_item(f: [u32; 18]) -> u32 {
    tel26_add_item(current_human_display(), f)
}

/// `Display::add_area` on the current human's Display (the 11 fields); returns eax.
unsafe fn tactics_area(f: [u32; 11]) -> u32 {
    tel26_add_area(current_human_display(), f)
}

/// A colour word, zero-extended.
unsafe fn tactics_colour(a: usize) -> u32 {
    job_colour(a)
}

/// `langlib_sprintf(JOB_TEXT, fmt)` (the translated text into the screen text buffer).
unsafe fn tactics_text(fmt: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt);
}

/// `langlib_sprintf(JOB_TEXT, fmt, a)`.
unsafe fn tactics_text1(fmt: usize, a: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt, a);
}

/// `langlib_sprintf(JOB_TEXT, fmt, a, b)`.
unsafe fn tactics_text2(fmt: usize, a: usize, b: usize) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    f(JOB_TEXT, fmt, a, b);
}

/// `sprintf(out, fmt, a)` (the CRT's).
unsafe fn tactics_sprintf1(out: usize, fmt: usize, a: i32) {
    let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    f(out, fmt, a);
}

/// `count_human_managers(connected only)` (0x838d10) by address (hooked when ported).
unsafe fn exe_count_human_managers(connected: u32) -> u32 {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x838d10usize);
    f(connected)
}

/// A menu row of a popup area: style `kind` (2 a row, 0x2002 a ticked row), `flags` (1 enabled, 0x21 greyed), the
/// screen text, the id and value the event handler gets.
unsafe fn tactics_menu_row(kind: u32, flags: u32, id: u32, value: u32, area: u32) -> u32 {
    tactics_item([kind, 0, 0, 0, 0, 0, 0, 0x10, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), flags, 1,
        tactics_colour(JOB_COLOUR_BDEA), JOB_TEXT as u32, 0, id, value, area])
}

/// A separator line of a popup area.
unsafe fn tactics_menu_separator(area: u32) -> u32 {
    tactics_item([1, 0, 0, 0, 0, 0, 0, 0x1000010, tactics_colour(JOB_COLOUR_31AA), tactics_colour(JOB_COLOUR_BDEC), 1, 1,
        tactics_colour(JOB_COLOUR_BDEA), 0, 0, 0, 0, area])
}

/// A popup area owned by item `owner` (flags 0x130 for a menu opened by a button, 0x30 for a submenu).
unsafe fn tactics_popup(flags: u32, owner: u32) -> u32 {
    tactics_area([0, 0, 0, 0, 0, 0, 0, 0, flags, tactics_colour(JOB_COLOUR_31AA), owner])
}

/// The Tactics_Screen methods and the other routines the screen calls by address (tactics_screens.cpp,
/// formation.cpp, the human manager and player stats), all left in the original.
mod tactics_calls {
    use super::*;
    pub(crate) unsafe fn thiscall2(f: usize, this: usize, a: u32, b: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
        g(this, a, b)
    }
    pub(crate) unsafe fn thiscall3(f: usize, this: usize, a: u32, b: u32, c: u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(f);
        g(this, a, b, c)
    }
    /// `Tactics_Screen::get_slot_player(row, opponent)` (0x8ab480): the player (Staff) in a row, or 0.
    pub(crate) unsafe fn slot_player(ts: usize, row: i8) -> usize {
        thiscall2(0x8ab480, ts, row as u8 as u32, 0) as usize
    }
    /// `Tactics_Screen::get_slot_number_text(row, out, size, no fallback, opponent)` (0x8aa200).
    pub(crate) unsafe fn slot_number_text(ts: usize, row: i8) {
        let g: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x8aa200usize);
        g(ts, row as u8 as u32, JOB_TEXT, 0x7d0, 0, 0);
    }
    /// `Tactics_Screen::get_slot_player_name_text(row, out, size, no fallback, name format, opponent)` (0x8aa550).
    pub(crate) unsafe fn slot_name_text(ts: usize, row: i8, format: u32) {
        let g: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8aa550usize);
        g(ts, row as u8 as u32, JOB_TEXT, 0x7d0, 1, format, 0);
    }
    /// `Tactics_Screen::get_slot_pitch_position(row, out position, opponent)` (0x8aa9d0): the row is on the pitch.
    pub(crate) unsafe fn slot_pitch_position(ts: usize, row: i8, out: *mut u8) -> u32 {
        thiscall3(0x8aa9d0, ts, row as u8 as u32, out as u32, 0)
    }
    /// `Tactics_Screen::is_slot_on_bench(row, opponent)` (0x8aaaa0).
    pub(crate) unsafe fn slot_on_bench(ts: usize, row: i8) -> u32 {
        thiscall2(0x8aaaa0, ts, row as u8 as u32, 0)
    }
    /// The yellow and red cards (0x8aabb0, 0x8aacb0), the condition (0x8aafd0), the match rating (0x8ab150) and the
    /// goals (0x8ab260) of a row (al).
    pub(crate) unsafe fn slot_byte(f: usize, ts: usize, row: i8) -> i8 {
        thiscall2(f, ts, row as u8 as u32, 0) as u8 as i8
    }
    /// `Tactics_Screen::get_slot_match_status_text(row, out, size, out colour, opponent)` (0x8aadb0).
    pub(crate) unsafe fn slot_status_text(ts: usize, row: i8, colour: *mut u32) {
        let g: unsafe extern "thiscall" fn(usize, u32, usize, u32, *mut u32, u32) -> u32 = core::mem::transmute(0x8aadb0usize);
        g(ts, row as u8 as u32, JOB_TEXT, 0x7d0, colour, 0);
    }
    /// `Tactics_Screen::is_slot_man_of_the_match(row, opponent)` (0x8ab360).
    pub(crate) unsafe fn slot_motm(ts: usize, row: i8) -> u32 {
        thiscall2(0x8ab360, ts, row as u8 as u32, 0)
    }
    /// `Tactics_Screen::find_slot_at_position(position, out pending, opponent)` (0x8ab670): the row, or -1.
    pub(crate) unsafe fn find_slot_at_position(ts: usize, slot: i8, pending: &mut u32) -> i8 {
        thiscall3(0x8ab670, ts, slot as u8 as u32, pending as *mut u32 as u32, 0) as u8 as i8
    }
    /// `Tactics_Screen::get_slot_marked_player` (0x8abb50) and `get_slot_marker` (0x8abd00) of a row.
    pub(crate) unsafe fn slot_staff(f: usize, ts: usize, row: i8) -> usize {
        thiscall2(f, ts, row as u8 as u32, 0) as usize
    }
    /// `Tactics_Screen::get_slot_coordinate(slot, out coordinate)` (0x8a94d0).
    pub(crate) unsafe fn slot_coordinate(ts: usize, slot: i8, out: &mut [i16; 2]) {
        thiscall2(0x8a94d0, ts, slot as u8 as u32, out.as_mut_ptr() as u32);
    }
    /// `Tactics_Screen::is_formation_slot_customised(slot)` (0x8a9fa0).
    pub(crate) unsafe fn slot_customised(ts: usize, slot: i8) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8a9fa0usize);
        g(ts, slot as u8 as u32)
    }
    /// `Tactics_Screen::format_title(out, size, colour a, colour b)` (0x8a8330).
    pub(crate) unsafe fn format_title(ts: usize, a: &mut u32, b: &mut u32) {
        let g: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32, *mut u32) -> u32 = core::mem::transmute(0x8a8330usize);
        g(ts, JOB_TEXT, 0x7d0, a, b);
    }
    /// `Formation::check_duplicate_position(slot, out later)` (0x5b3460).
    pub(crate) unsafe fn duplicate_position(fm: usize, slot: i8, later: &mut u32) -> u32 {
        thiscall2(0x5b3460, fm, slot as u8 as u32, later as *mut u32 as u32)
    }
    /// `Formation::check_duplicate_coordinate(slot, phase, zone, out later)` (0x5b4ee0).
    pub(crate) unsafe fn duplicate_coordinate(fm: usize, slot: i8, phase: u8, zone: u32, later: &mut u32) -> u32 {
        let g: unsafe extern "thiscall" fn(usize, u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(0x5b4ee0usize);
        g(fm, slot as u8 as u32, phase as u32, zone, later)
    }
    /// `Formation::has_central_slot_in_line(position, match players or 0)` (0x5b2e80).
    pub(crate) unsafe fn central_in_line(fm: usize, position: u16, players: usize) -> u32 {
        thiscall2(0x5b2e80, fm, position as u32, players as u32)
    }
    /// `Formation::find_slot_by_position(position)` (0x5b2f10): the slot, or 0xff.
    pub(crate) unsafe fn slot_by_position(fm: usize, position: u16) -> i8 {
        let g: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5b2f10usize);
        g(fm, position as u32) as u8 as i8
    }
    /// `get_pitch_point_for_position_flags(flags, left, top, right, bottom, out x, out y, wide)` (0x8ad780).
    pub(crate) unsafe fn pitch_point(position: u16, r: &[i32; 4], x: &mut i32, y: &mut i32, wide: u32) {
        let g: unsafe extern "C" fn(u32, i32, i32, i32, i32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x8ad780usize);
        g(position as u32, r[0], r[1], r[2], r[3], x, y, wide);
    }
    /// `mask64_to_bit_index(low, high)` (0x8adbb0, al).
    pub(crate) unsafe fn mask_index(lo: u32, hi: u32) -> u32 {
        let g: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x8adbb0usize);
        g(lo, hi)
    }
    /// `HUMAN_MANAGER::get_password(manager)` (0x5fde40): the password string (manager 0: the current one).
    pub(crate) unsafe fn password(manager: usize) -> usize {
        let g: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5fde40usize);
        g(read_usize(G_HUMAN_MANAGER), manager)
    }
}

/// Sets the static colours on the first build, each once (guard bits of `GUARD`, then bit 1 of `GUARD2`).
unsafe fn tactics_build_statics_init() {
    use tactics_build_statics as s;
    let rgb = |r: u32, g: u32, b: u32| exe::graphics_rgb_to_pixel(r, g, b, 0) as u16;
    let mut guard = read_u8(s::GUARD);
    if guard & 1 == 0 {
        write_u8(s::GUARD, guard | 1);
        write_u16(s::BENCH_NUMBER, rgb(0, 0, 0x40));
        guard = read_u8(s::GUARD);
    }
    if guard & 2 == 0 {
        write_u8(s::GUARD, guard | 2);
        write_u16(s::GREEN_90, rgb(0, 0x90, 0));
        guard = read_u8(s::GUARD);
    }
    if guard & 4 == 0 {
        guard |= 4;
        write_u8(s::GUARD, guard);
        write_u16(s::RED, read_u16(TACTICS_COLOUR_3154));
    }
    if guard & 8 == 0 {
        guard |= 8;
        write_u8(s::GUARD, guard);
        write_u16(s::RUN, read_u16(JOB_COLOUR_BDD4));
    }
    if guard & 0x10 == 0 {
        write_u8(s::GUARD, guard | 0x10);
        write_u16(s::GREEN_60, rgb(0, 0x60, 0));
        guard = read_u8(s::GUARD);
    }
    if guard & 0x20 == 0 {
        guard |= 0x20;
        write_u8(s::GUARD, guard);
        write_u16(s::ZONE, read_u16(JOB_COLOUR_BDEC));
    }
    if guard & 0x40 == 0 {
        guard |= 0x40;
        write_u8(s::GUARD, guard);
        write_u16(s::HOVER_ZONE, read_u16(JOB_COLOUR_317E));
    }
    if guard & 0x80 == 0 {
        write_u8(s::GUARD, guard | 0x80);
        write_u16(s::GREEN_80, rgb(0, 0x80, 0));
    }
    let guard2 = read_u8(s::GUARD2);
    if guard2 & 1 == 0 {
        write_u8(s::GUARD2, guard2 | 1);
        write_u16(s::GREEN_40, rgb(0, 0x40, 0));
    }
}

/// The preset formation names that have a translated text (the name of the tactic block, then the text).
const TACTICS_PRESET_NAMES: [usize; 8] = [0xa8b758, 0xa8b748, 0xa8b720, 0xa8b6e4, 0xa1593c, 0xa8b658, 0xa8b62c, 0x9b75dc];

/// The column views of the View menu: (view, text). Views 1..6 only tick; 7.. also copy their text as the column
/// heading.
const TACTICS_VIEW_TEXTS: [(u8, usize); 13] = [(7, 0xa8bf4c), (8, 0xa8bf40), (9, 0xa8bf34), (0xa, 0xa8bf28), (0xb, 0xa8bf18),
    (0xc, 0xa8bf08), (0xd, 0xa8befc), (0xe, 0xa8beec), (0xf, 0xa8bedc), (0x10, 0xa8becc), (0x11, 0xa8bec0), (0x12, 0xa8beac),
    (0x13, 0xa8bea0)];

/// One View menu row: ticked (0x2002, id 0) when it is the current view `view`, else id 0xd with the view as value.
/// A ticked row of view 7 or more copies its text into `heading`.
unsafe fn tactics_view_row(view: u8, k: u8, area: u32, heading: Option<&mut [u8; 0xc8]>) {
    if view == k {
        tactics_menu_row(0x2002, 1, 0, 0, area);
        if let Some(h) = heading {
            tel26_copy_text(h.as_mut_ptr(), JOB_TEXT as *const u8);
        }
    } else {
        tactics_menu_row(2, 1, 0xd, k as u32, area);
    }
}

/// The menus of an editable tactics screen: Tactics (Team Instructions, Penalty Takers, Set To Default with the
/// presets, Load, Load Preset, Save, Save As, Save For, Delete, Lock or Unlock and the recent files), View (the
/// column views), Last Match and Edit (Undo, Copy, Copy All, Paste, Reflect, Show or Hide Runs).
unsafe fn tactics_build_menus(ts: usize, view: u8, heading: &mut [u8; 0xc8]) {
    use tactics_screen as t;
    let c = tactics_colour;
    // the Tactics menu
    let bar = tactics_area([0x6e, 0x50, 0x168, 0x64, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98e840);
    let button = tel26_area_button([bar, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    tactics_text(0xa8bfa4);
    tactics_menu_row(2, 1, 1, 0, menu);
    tactics_text(0xa8bf94);
    tactics_menu_row(2, 1, 0x22, 0, menu);
    tactics_text(0xa8bf84);
    let default_row = tactics_menu_row(2, 1, 0, 0, menu);
    let presets = tactics_popup(0x30, default_row);
    let mut k: u8 = 0;
    while (k as i8 as i32) < read_i32(read_usize(G_TACTICS) + 0x5fd) {
        let idx = k as i8 as i16 as u16 as u32;
        let found = TACTICS_PRESET_NAMES.iter().find(|&&name| exe::crt_stricmp(exe::tactics_get_tactic_block(read_usize(G_TACTICS), idx, 1), name) == 0);
        match found {
            Some(&name) => tactics_text(name),
            None => tel26_copy_text(JOB_TEXT as *mut u8, exe::tactics_get_tactic_block(read_usize(G_TACTICS), idx, 1) as *const u8),
        }
        tactics_menu_row(2, 1, 2, k as i8 as i32 as u32, presets);
        k = k.wrapping_add(1);
    }
    tactics_menu_separator(menu);
    tactics_text(0xa7e7f4);
    tactics_menu_row(2, 1, 3, 0, menu);
    if read_u8(TACTICS_PRESETS_ON) != 0 {
        tactics_text(0xa8bf78);
        tactics_menu_row(2, 1, 4, 0, menu);
    }
    tactics_text(0xa7e7dc);
    if read_u32(ts + t::SAVED) == 0 && read_u32(ts + t::HAS_FILE) != 0 {
        tactics_menu_row(2, 1, 5, 0, menu);
    } else {
        tactics_menu_row(2, 0x21, 0, 0, menu);
    }
    tactics_text(0xa7e7c4);
    tactics_menu_row(2, 1, 6, 0, menu);
    if read_u8(TACTICS_PRESETS_ON) != 0 {
        tactics_text(0xa8bf78);
        tactics_menu_row(2, 1, 4, 0, menu);
    }
    // Save For: one row per human manager (the others), greyed without a password
    if exe_count_human_managers(0) as i32 > 1 {
        tactics_text(0xa7e7a4);
        let save_for = tactics_menu_row(2, 1, 0, 0, menu);
        let sub = tactics_popup(0x30, save_for);
        for n in 0..0x10 {
            let staff = staff_at(read_i32(G_STAFF_COUNT) as isize - n - 1);
            if staff.classification() != 1 || staff.addr() == tel26_slot_manager().addr() {
                continue;
            }
            exe::database_format_staff_name(staff, JOB_TEXT, 0x7d0, 1);
            if read_u8(tactics_calls::password(staff.addr())) != 0 {
                tactics_menu_row(2, 1, 8, staff.addr() as u32, sub);
            } else {
                tactics_menu_row(2, 0x21, 0, 0, sub);
            }
        }
    }
    tactics_text(0xa69fb4);
    tactics_menu_row(2, 1, 9, 0, menu);
    if read_u8(ts + t::LOCKED) != 0 {
        tactics_text(0xa7e778);
        tactics_menu_row(2, 1, 0xb, 0, menu);
    } else if read_u8(tactics_calls::password(0)) != 0 {
        tactics_text(0xa7e770);
        tactics_menu_row(2, 1, 0xb, 0, menu);
    }
    // the recent files (4), after a separator
    let mut separated = false;
    let mut file = [0u8; 0x104];
    for n in 0..4u32 {
        let recent: unsafe extern "thiscall" fn(usize, u32, *mut u8, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x5fc760usize);
        recent(read_usize(G_HUMAN_MANAGER), n, file.as_mut_ptr(), 0x104, 1, 0, 1);
        if file[0] == 0 {
            continue;
        }
        if !separated {
            tactics_menu_separator(menu);
            separated = true;
        }
        let f: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
        f(JOB_TEXT, 0xa7e728, n + 1, file.as_ptr());
        tactics_menu_row(2, 1, 0xc, n, menu);
    }
    // the View menu
    tactics_text(0x988004);
    let button = tel26_area_button([bar, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    tactics_text(0x98edf4);
    tactics_view_row(view, 1, menu, None);
    tactics_text(0x98edc8);
    tactics_view_row(view, 2, menu, None);
    if read_u32(ts + t::MATCH) == 0 {
        tactics_text(0x98ed4c);
        tactics_view_row(view, 3, menu, None);
        tactics_text(0x98eddc);
        tactics_view_row(view, 4, menu, None);
        tactics_text(0xa8bf68);
        tactics_view_row(view, 5, menu, None);
    } else {
        tactics_text(0xa8bf58);
        tactics_view_row(view, 6, menu, None);
    }
    tactics_menu_separator(menu);
    for &(k, text) in TACTICS_VIEW_TEXTS.iter() {
        tactics_text(text);
        tactics_view_row(view, k, menu, Some(&mut *heading));
    }
    tactics_text1(0xa8be78, TACTICS_EMPTY);
    tactics_view_row(view, 0x14, menu, Some(&mut *heading));
    if read_u32(ts + t::MATCH) != 0 {
        tactics_menu_separator(menu);
        tactics_text(0xa8be70);
        tactics_view_row(view, 0x15, menu, Some(&mut *heading));
    }
    // Last Match and the Edit menu
    let bar = tactics_area([0x212, 0x50, 0x30c, 0x64, 2, 0, 1, 0, 1, 0, u32::MAX]);
    tactics_text(0x98e828);
    tel26_area_button([bar, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0xe, 0]);
    tactics_text(0xa8be68);
    let button = tel26_area_button([bar, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0, 0]);
    let menu = tactics_popup(0x130, button);
    let edit_row = |enabled: bool, id: u32| if enabled { tactics_menu_row(2, 1, id, 0, menu) } else { tactics_menu_row(2, 0x21, 0, 0, menu) };
    tactics_text(0xa8be60);
    edit_row(read_u32(ts + t::CAN_UNDO) != 0, 0xf);
    tactics_text(0xa8be58);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x10);
    tactics_text(0xa8be4c);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x11);
    tactics_text(0xa8be44);
    edit_row(read_u8(ts + t::VIEW) != 1 && read_u8(ts + t::CAN_PASTE) != 0, 0x12);
    tactics_text(0xa8be3c);
    edit_row(read_u8(ts + t::VIEW) != 1, 0x13);
    tactics_text(if read_u32(ts + t::SHOW_RUNS) != 0 { 0xa8be24 } else { 0xa8be30 });
    edit_row(read_u8(ts + t::VIEW) != 1, 0x14);
}

/// The formation name (with the unsaved mark), LOCKED and the Overview / With Ball / Without Ball tabs; then the
/// column widths of the player list for the view.
unsafe fn tactics_build_header(ts: usize, view: u8, widths: &mut [u8; 9]) {
    use tactics_screen as t;
    let c = tactics_colour;
    tel26_copy_text(JOB_TEXT as *mut u8, (ts + t::FORMATION) as *const u8);
    if read_u32(ts + t::SAVED) == 0 {
        let mut n = 0;
        while read_u8(JOB_TEXT + n) != 0 {
            n += 1;
        }
        tel26_copy_text((JOB_TEXT + n) as *mut u8, 0x989ad8 as *const u8);
    }
    let text_item: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(FN_SCREENMANAGER_ADD_TEXT_ITEM);
    text_item(G_SCREEN_MANAGER, 0x6e, 0x69, 0x1b8, 0x8c, 2, 0, 0xc, 3, c(JOB_COLOUR_BDEC), JOB_TEXT, 1);
    if read_u8(ts + t::LOCKED) != 0 {
        tactics_text(0xa8be18);
        text_item(G_SCREEN_MANAGER, 0x154, 0x69, 0x1b8, 0x7d, 1, 0, 0x4c, 1, c(JOB_COLOUR_BDEC), JOB_TEXT, 1);
    }
    let tabs = tactics_area([0x1c2, 0x69, 0x30c, 0x8c, 3, 0, 1, 0, 1, 0, u32::MAX]);
    // the font of the current tab: an immediate the 1080 exe patches (1 -> 3), read from the running exe
    tactics_text(0xa8be0c);
    if read_u8(ts + t::VIEW) == 1 {
        tel26_area_text([tabs, 0, 0, 0x830, c(TACTICS_COLOUR_BDD6), 0xc, core::ptr::read(0x89c626 as *const u8) as u32, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
    } else {
        tel26_area_button([tabs, 0, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x15, 1]);
    }
    let tab = |col: u32, view_k: u8, text: usize, current_font: u32| {
        tactics_text(text);
        if read_u8(ts + t::VIEW) == view_k {
            tel26_area_text([tabs, col, 0, 0x830, c(TACTICS_COLOUR_BDD6), 0xc, current_font, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        } else if read_u32(ts + t::READ_ONLY) == 0 {
            tel26_area_button([tabs, col, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x15, view_k as u32]);
        } else {
            tel26_area_text([tabs, col, 0, 0x30, c(TACTICS_COLOUR_BDD6), 0x2c, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    };
    tab(1, 2, 0xa8be00, core::ptr::read(0x89c69e as *const u8) as u32);
    tab(2, 3, 0xa8bdf0, core::ptr::read(0x89c73c as *const u8) as u32);
    // the column widths: the extra columns merge into the one the view shows
    match view {
        1 | 3 | 4 | 5 => {
            widths[6] = widths[6].wrapping_add(widths[7]).wrapping_add(widths[8]);
            widths[7] = 0;
            widths[8] = 0;
        }
        2 => {
            widths[7] = widths[7].wrapping_add(widths[8]);
            widths[8] = 0;
        }
        6 => {}
        _ => {
            widths[5] = widths[5].wrapping_add(widths[6]).wrapping_add(widths[7]).wrapping_add(widths[8]);
            widths[6] = 0;
            widths[7] = 0;
            widths[8] = 0;
        }
    }
}

/// The player instruction options of the column views 7 to 0x13: (low mask, high mask, text, the text takes the
/// empty-string argument, colour, the next option's low and high masks). Each option the player has shows a button
/// whose value is the next option's bit index with the formation slot (6 bits each).
unsafe fn tactics_instruction_options(view: u8) -> &'static [(u32, u32, usize, bool, usize, u32, u32)] {
    const T: usize = JOB_COLOUR_317E;
    const N: usize = JOB_COLOUR_BDEC;
    const NO: usize = 0x98e8cc;
    const YES: usize = 0x98e8d0;
    match view {
        7 => &[(1, 0, 0xa8bda8, true, T, 2, 0), (2, 0, 0xa8bd7c, true, N, 4, 0), (4, 0, 0xa8bd50, true, N, 8, 0),
            (8, 0, 0xa8bd24, true, N, 0x10, 0), (0x10, 0, 0xa8bcf8, true, N, 1, 0)],
        8 => &[(0x200, 0, 0xa8bcd8, true, T, 0x400, 0), (0x400, 0, 0xa8bcb8, true, N, 0x800, 0), (0x800, 0, 0xa8bc98, true, N, 0x1000, 0),
            (0x1000, 0, 0xa8bc78, true, N, 0x200, 0)],
        9 => &[(0x2000, 0, 0xa8bc58, true, T, 0x4000, 0), (0x4000, 0, 0xa8bc3c, true, N, 0x8000, 0), (0x8000, 0, 0xa8bc1c, true, N, 0x2000, 0)],
        0xa => &[(0x20, 0, 0xa8bbfc, true, T, 0x40, 0), (0x40, 0, 0xa8bbdc, true, N, 0x80, 0), (0x80, 0, 0xa8bbbc, true, N, 0x100, 0),
            (0x100, 0, 0xa8bb9c, true, N, 0x20, 0)],
        0xb => &[(0x10000, 0, 0xa8bb5c, true, T, 0x40000000, 0), (0x40000000, 0, 0xa8bb24, true, N, 0x20000, 0), (0x20000, 0, 0xa8bae4, true, N, 0x10000, 0)],
        0xc => &[(0x40000, 0, 0xa8baa4, true, T, 0x80000, 0), (0x80000, 0, 0xa8ba6c, true, N, 0x80000000, 0), (0x80000000, 0, 0xa8ba2c, true, N, 0x40000, 0)],
        0xd => &[(0x100000, 0, NO, false, T, 0x200000, 0), (0x200000, 0, YES, false, N, 0x100000, 0)],
        0xe => &[(0x400000, 0, NO, false, T, 0x800000, 0), (0x800000, 0, YES, false, N, 0x400000, 0)],
        0xf => &[(0x1000000, 0, NO, false, T, 0x2000000, 0), (0x2000000, 0, YES, false, N, 0x1000000, 0)],
        0x10 => &[(0x4000000, 0, NO, false, T, 0x8000000, 0), (0x8000000, 0, YES, false, N, 0x4000000, 0)],
        0x11 => &[(0x10000000, 0, NO, false, T, 0x20000000, 0), (0x20000000, 0, YES, false, N, 0x10000000, 0)],
        0x12 => &[(0, 2, NO, false, T, 0, 1), (0, 1, YES, false, N, 0, 2)],
        0x13 => &[(0, 8, NO, false, T, 0, 4), (0, 4, YES, false, N, 0, 8)],
        _ => &[],
    }
}

/// The value of an option button: the next option's bit index (6 bits) and the formation slot above it.
unsafe fn tactics_option_value(next_lo: u32, next_hi: u32, slot: u8) -> u32 {
    (tactics_calls::mask_index(next_lo, next_hi) & 0x3f) | ((slot as u32 & 0x3f) << 6)
}

/// An option button in column 5 of row `row`: id 0x1a (instructions), 0x1b (marking) or a person's id.
unsafe fn tactics_option_button(area: u32, row: u32, colour: usize, id: u32, value: u32) {
    tel26_area_button([area, 5, row, 1, 0, 0xc, 1, tactics_colour(colour), JOB_TEXT as u32, id, value]);
}

/// Row `k` of the player list (number, name, marks and the view's column), in the areas `list` (the 9 columns) and
/// `handles` (the drag column); `line` is the display row.
unsafe fn tactics_build_row(ts: usize, view: u8, k: i8, list: u32, handles: u32, line: u32) {
    use tactics_screen as t;
    use tactics_calls as x;
    let c = tactics_colour;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let player = x::slot_player(ts, k);
    if !read_only {
        tactics_item([0x102, 0, 0, 0, 0, 1, line, 1, 0, 0, 0xc, 1, 0, 0, 0, 0x16, k as i32 as u32, handles]);
    }
    // the penalty order or the captain mark (condition and match rating views)
    if view == 2 || view == 6 {
        if read_u32(ts + t::PENALTY_MODE) != 0 {
            let order = (0..0xbi8).find(|&n| read_i8(ts + t::PENALTY_ORDER + n as usize) == k);
            if let Some(n) = order {
                tactics_text1(0xa8bddc, (n as i32 + 1) as usize);
                let colour = if n < 5 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };
                let flags = if read_only { 1 } else { 0x42 };
                tactics_item([flags, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(colour), JOB_TEXT as u32, 0, 0x18, n as i32 as u32, list]);
            }
        } else {
            let captain_row = || (0..0x14i8).find(|&r| {
                let p = x::slot_player(ts, r);
                p != 0 && read_u32(p) == read_u32(ts + t::CAPTAIN)
            });
            let mark = match captain_row() {
                Some(r) if r == k => true,
                _ => captain_row().is_none() && k == 0,
            };
            if mark {
                covered!("tel14_2 captain mark");
                tactics_text(0xa8bdd4);
                if read_only {
                    tactics_item([1, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32, 0, 0x17, 0, list]);
                } else {
                    let colour = if captain_row().is_some() { JOB_COLOUR_BDEC } else { TACTICS_COLOUR_3154 };
                    tactics_item([0x42, 0, 0, 0, 0, 5, line, 1, 0, 0, 0xc, 1, c(colour), JOB_TEXT as u32, 0, 0x17, 0, list]);
                }
            }
        }
    }
    // the number: clickable on the pitch or for a substitute who can still come on
    let number_colour = if k < 0xb { c(JOB_COLOUR_BE04) } else { c(tactics_build_statics::BENCH_NUMBER) };
    x::slot_number_text(ts, k);
    let outside_squad = read_u32(ts + t::HAS_MATCH_RECORD) != 0 && k >= ((read_u8(ts + t::BENCH_SIZE) & 0xf) as i8).wrapping_add(0xb);
    if outside_squad {
        tel26_area_text([list, 0, line, 0x30, number_colour, 0x2c, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
    } else {
        let mut clickable = x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0;
        if !clickable && x::slot_on_bench(ts, k) != 0 {
            let m = read_usize(ts + t::MATCH);
            if m != 0 && read_u32(ts + t::FIXTURE) != 0 {
                let side = read_i8(ts + t::SIDE) as isize;
                let left = read_u8((m as isize + side * 0xd0 + 0x9049) as usize).wrapping_sub(read_u8(ts + t::SUBS_MADE)) as i8;
                clickable = left > 0;
            }
        }
        if clickable {
            let flags = if read_only { 1 } else { 0x42 };
            tactics_item([flags, 0, 0, 0, 0, 0, line, 0x30, number_colour, number_colour, 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0,
                0x19, k as i32 as u32, list]);
        } else {
            tel26_area_text([list, 0, line, 0x30, number_colour, 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32]);
        }
    }
    if !read_only {
        tactics_item([0x302, 0, 0, 0, 0, 0, line, 1, 0, 0, 0xc, 1, 0, 0, 0, 0x16, k as i32 as u32, list]);
    }
    // the cards
    let card = |colour: usize, style: u32| tel26_area_text([list, 2, line, style, c(colour), 0xc, 1, c(JOB_COLOUR_BDEA), 0]);
    if x::slot_byte(0x8aabb0, ts, k) != 0 {
        if x::slot_byte(0x8aacb0, ts, k) != 0 {
            card(JOB_COLOUR_BDEC, 0x20030);
            card(TACTICS_COLOUR_31A8, 0x40030);
        } else {
            card(JOB_COLOUR_BDEC, 0x20030);
        }
    } else if x::slot_byte(0x8aacb0, ts, k) != 0 {
        card(TACTICS_COLOUR_31A8, 0x20030);
    }
    tactics_build_row_column(ts, view, k, player, list, line, number_colour);
    // the name
    x::slot_name_text(ts, k, 1);
    if read_u8(JOB_TEXT) == 0 {
        return;
    }
    if player != 0 {
        let colour = if x::slot_motm(ts, k) != 0 {
            JOB_COLOUR_BDE8
        } else if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 {
            JOB_COLOUR_3184
        } else {
            JOB_COLOUR_317E
        };
        let id = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
        tactics_item([0x202, 0, 0, 0, 0, 4, line, 1, 0, 0, 1, 2, c(colour), JOB_TEXT as u32, 0, id, player as u32, list]);
    } else {
        let colour = if x::slot_motm(ts, k) != 0 {
            JOB_COLOUR_BDE8
        } else if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 {
            JOB_COLOUR_3184
        } else {
            JOB_COLOUR_317E
        };
        tactics_item([1, 0, 0, 0, 0, 4, line, 1, 0, 0, 1, 2, c(colour), JOB_TEXT as u32, 0, 0, 0, list]);
    }
}

/// The view's column of a row: positions (1), goals (3), form (4) and average rating (5) of the player; the match
/// status with the condition (2) or the match rating and goals (6); a player instruction (7 to 0x13), the marking
/// (0x14) or the marker (0x15).
unsafe fn tactics_build_row_column(ts: usize, view: u8, k: i8, player: usize, list: u32, line: u32, number_colour: u32) {
    use tactics_screen as t;
    use tactics_calls as x;
    let c = tactics_colour;
    let on_pitch_colour = || if x::slot_pitch_position(ts, k, core::ptr::null_mut()) != 0 { JOB_COLOUR_BDEC } else { JOB_COLOUR_317E };
    let player_data = || if player != 0 { read_usize(player + staff::PLAYER) } else { 0 };
    match view {
        1 => {
            if player == 0 || player_data() == 0 {
                return;
            }
            let positions: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53ee50usize);
            positions(player_data(), JOB_TEXT, 0x7d0, 1);
            let colour = on_pitch_colour();
            tel26_area_text([list, 6, line, 1, 0, 1, 1, c(colour), JOB_TEXT as u32]);
        }
        3 | 4 | 5 => {
            if player == 0 || player_data() == 0 {
                return;
            }
            if view == 4 {
                let form: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x7c1660usize);
                form(read_usize(G_PLAYER_STATS), player_data(), JOB_TEXT, 0x7d0);
            } else {
                let category = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 5 } else { 6 };
                let field = if view == 3 { 2 } else { 0x11 };
                let stat: unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32, u32, u32) = core::mem::transmute(0x7c25b0usize);
                stat(read_usize(G_PLAYER_STATS), player_data(), category, field, JOB_TEXT, 0x7d0, 1, 0, 1);
            }
            let colour = on_pitch_colour();
            tel26_area_text([list, 6, line, 1, 0, 0xc, 1, c(colour), JOB_TEXT as u32]);
        }
        2 | 6 => {
            // the status colour goes over the number colour's local (a word): without status text it is not used
            let mut status_colour: u32 = number_colour;
            x::slot_status_text(ts, k, &mut status_colour);
            if read_u8(JOB_TEXT) != 0 {
                tel26_area_text([list, 6, line, 1, 0, 1, 1, status_colour & 0xffff, JOB_TEXT as u32]);
            }
            if view == 2 {
                let condition = x::slot_byte(0x8aafd0, ts, k);
                if condition == 0 {
                    return;
                }
                if condition > 0 {
                    tactics_sprintf1(JOB_TEXT, 0x98e8d4, condition as i32);
                } else {
                    tactics_text(0x98e98c);
                }
                let colour = on_pitch_colour();
                tel26_area_text([list, 7, line, 1, 0, 0xc, 1, c(colour), JOB_TEXT as u32]);
            } else {
                let rating = x::slot_byte(0x8ab150, ts, k);
                if rating <= 0 {
                    return;
                }
                tactics_sprintf1(JOB_TEXT, 0x98806c, rating as i32);
                tel26_area_text([list, 7, line, 1, 0, 0xc, 2, c(JOB_COLOUR_BDE8), JOB_TEXT as u32]);
                let goals = x::slot_byte(0x8ab260, ts, k);
                if goals <= 0 {
                    return;
                }
                tactics_sprintf1(JOB_TEXT, 0x98806c, goals as i32);
                tel26_area_text([list, 8, line, 1, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
            }
        }
        _ => {
            let mut slot: u8 = 0;
            if x::slot_pitch_position(ts, k, &mut slot) == 0 {
                return;
            }
            let fm = ts + t::FORMATION;
            let instructions = exe::formation_get_player_instructions(fm, slot as i8);
            let (lo, hi) = (instructions as u32, (instructions >> 32) as u32);
            if view == 0x14 {
                let marking = exe::formation_get_marking_type(fm, slot as i8);
                if marking & 1 != 0 {
                    tactics_text1(0xa8ba0c, TACTICS_EMPTY);
                    let v = tactics_option_value(2, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_317E, 0x1b, v);
                }
                if marking & 2 != 0 {
                    tactics_text1(0xa8b9e8, TACTICS_EMPTY);
                    let v = tactics_option_value(4, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, 0x1b, v);
                }
                if marking & 4 != 0 {
                    let marked = x::slot_staff(0x8abb50, ts, k);
                    if marked != 0 {
                        let mut initials = [0u8; 0xc8];
                        exe::database_format_staff_name(marked, initials.as_mut_ptr() as usize, 0xc8, 6);
                        tactics_text2(0xa8b9ac, initials.as_ptr() as usize, TACTICS_EMPTY);
                    } else {
                        tactics_text1(0xa8b984, TACTICS_EMPTY);
                    }
                    let v = tactics_option_value(1, 0, slot);
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, 0x1b, v);
                }
            } else if view == 0x15 {
                let marker = x::slot_staff(0x8abd00, ts, k);
                if marker != 0 {
                    exe::database_format_staff_name(marker, JOB_TEXT, 0x7d0, 6);
                    let id = if exe::database_is_national_team(read_usize(ts + t::CLUB)) != 0 { 0x7dc } else { 0x7d7 };
                    tactics_option_button(list, line, JOB_COLOUR_BDEC, id, marker as u32);
                } else {
                    tel26_copy_text(JOB_TEXT as *mut u8, 0xa8b980 as *const u8);
                    tel26_area_text([list, 5, line, 1, 0, 0xc, 1, c(JOB_COLOUR_BDEC), JOB_TEXT as u32]);
                }
            } else {
                for &(mlo, mhi, text, with_empty, colour, nlo, nhi) in tactics_instruction_options(view).iter() {
                    if (lo & mlo) | (hi & mhi) == 0 {
                        continue;
                    }
                    if with_empty {
                        tactics_text1(text, TACTICS_EMPTY);
                    } else {
                        tactics_text(text);
                    }
                    let v = tactics_option_value(nlo, nhi, slot);
                    tactics_option_button(list, line, colour, 0x1a, v);
                }
            }
        }
    }
}

/// The two arrowheads of a run from (x1, y1) to (x2, y2), computed on the x87 like the original (fpatan, fsin,
/// fcos, the game's `_ftol` 0x945640): the angle a = atan2(y1 - y2, x2 - x1); the first head at
/// (x2 - 8 sin t, y2 + 8 cos t) with t = pi - a - pi/2 - c, the second at (x2 - 8 cos u, y2 + 8 sin u) with u = a - c
/// (c = 0.4488, the constants read from the exe). Returns (x, y) of each head.
unsafe fn tactics_arrow_heads(x1: i32, y1: i32, x2: i32, y2: i32) -> [i32; 4] {
    // [0] dy, [1] dx, [2] x2, [3] y2, [4..8] the results, [8..10] u (f64), then the constants
    let mut m = [0u32; 19];
    m[0] = y1.wrapping_sub(y2) as u32;
    m[1] = x2.wrapping_sub(x1) as u32;
    m[2] = x2 as u32;
    m[3] = y2 as u32;
    for (k, a) in [0x9703e8usize, 0x9703e0, 0x9703d8, 0x968f10].iter().enumerate() {
        m[10 + 2 * k] = read_u32(*a);
        m[11 + 2 * k] = read_u32(*a + 4);
    }
    m[18] = 0x945640;
    core::arch::asm!(
        "fild dword ptr [edi]",
        "fild dword ptr [edi + 4]",
        "fpatan",
        "fld qword ptr [edi + 40]",
        // fsub st(0), st(1)
        ".byte 0xd8, 0xe1",
        "fsub qword ptr [edi + 48]",
        "fsub qword ptr [edi + 56]",
        "fild dword ptr [edi + 8]",
        "fld st(1)",
        "fsin",
        "fmul qword ptr [edi + 64]",
        // fsubp st(1), st(0)
        ".byte 0xde, 0xe9",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 16], eax",
        "fcos",
        "fmul qword ptr [edi + 64]",
        "fiadd dword ptr [edi + 12]",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 20], eax",
        "fsub qword ptr [edi + 56]",
        "fstp qword ptr [edi + 32]",
        "fld qword ptr [edi + 32]",
        "fsin",
        "fmul qword ptr [edi + 64]",
        "fiadd dword ptr [edi + 12]",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 24], eax",
        "fild dword ptr [edi + 8]",
        "fld qword ptr [edi + 32]",
        "fcos",
        "fmul qword ptr [edi + 64]",
        ".byte 0xde, 0xe9",
        "call dword ptr [edi + 72]",
        "mov dword ptr [edi + 28], eax",
        in("edi") m.as_mut_ptr(),
        clobber_abi("C"),
    );
    [m[4] as i32, m[5] as i32, m[7] as i32, m[6] as i32]
}

/// The pitch rectangle (left, top, right, bottom) and the colours the pitch views use.
struct TacticsPitch {
    r: [i32; 4],
    /// The goalkeeper's colours (shirt, outline): dark on a white-ish kit, else the screen's light green.
    keeper_back: u32,
    keeper_line: u32,
    /// The outfield colours: the screen title's colours.
    field_back: u32,
    field_line: u32,
}

/// The run arrows of the overview: for each slot whose attacking position differs from its defending one, a line
/// from one to the other, nudged off the circles, and its two heads.
unsafe fn tactics_build_overview_runs(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let fm = ts + t::FORMATION;
    for slot in 0..0xbi8 {
        let mut pending: u32 = 0;
        if x::find_slot_at_position(ts, slot, &mut pending) <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_position(fm, slot, &mut later) == 0 {
            continue;
        }
        let from = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let to = exe::formation_get_slot_position(fm, slot as u8 as u32, 1);
        if from == to {
            continue;
        }
        let players = || if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = x::central_in_line(fm, from, players());
        let (mut x1, mut y1) = (0i32, 0i32);
        x::pitch_point(from, &pitch.r, &mut x1, &mut y1, wide);
        let (mut line_from, mut side_from, mut line_to, mut side_to) = (0u16, 0u16, 0u16, 0u16);
        exe::formation_split_position_code(from as u32, &mut line_from, &mut side_from);
        exe::formation_split_position_code(to as u32, &mut line_to, &mut side_to);
        let wide = if side_from != side_to { x::central_in_line(fm, to, players()) } else { x::central_in_line(fm, from, players()) };
        let (mut x2, mut y2) = (0i32, 0i32);
        x::pitch_point(to, &pitch.r, &mut x2, &mut y2, wide);
        if x::slot_by_position(fm, to) > -1 {
            let (dx, dy) = ((x2 - x1).wrapping_abs(), (y2 - y1).wrapping_abs());
            if y2 < y1 && dy > dx {
                y2 += 0x10;
            } else if y2 > y1 && dy > dx {
                y2 -= 0x12;
            } else if x2 < x1 {
                x2 += 0xd;
            } else if x2 > x1 {
                x2 -= 0xd;
            }
        }
        let colour = if pending == 0 { read_u32(tactics_build_statics::RED) } else { tactics_colour(tactics_build_statics::GREEN_80) };
        let colour = colour & 0xffff;
        tactics_item([1, x1 as u32, y1 as u32, x2 as u32, y2 as u32, 0, 0, 0x2400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        let h = tactics_arrow_heads(x1, y1, x2, y2);
        tactics_item([1, h[0] as u32, h[1] as u32, x2 as u32, y2 as u32, 0, 0, 0x400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        tactics_item([1, h[2] as u32, h[3] as u32, x2 as u32, y2 as u32, 0, 0, 0x400, colour, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
    }
}

/// The circles of the players: shadow (a later duplicate), the number (clickable and draggable, id 0x1d, plus the
/// drop target 0x1e when editable), the customised mark and, in the overview, the name under the circle.
unsafe fn tactics_slot_circle(ts: usize, cx: i32, cy: i32, ord: u32, back: u32, line: u32, later: u32, overview: bool) {
    use tactics_screen as t;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let (top, bottom) = if overview { (cy - 0x12, cy + 8) } else { (cy - 0xd, cy + 0xd) };
    if later != 0 {
        if overview {
            tactics_item([1, (cx - 0x11) as u32, (cy - 0x16) as u32, (cx + 9) as u32, (cy + 4) as u32, 0, 0, 0x10030, line, line, 0, 1, back,
                TACTICS_EMPTY as u32, 0, 0, 0, u32::MAX]);
        } else {
            tactics_item([1, (cx - 0x11) as u32, (cy - 0x11) as u32, (cx + 9) as u32, (cy + 9) as u32, 0, 0, 0x10030, line, line, 0, 1, back,
                TACTICS_EMPTY as u32, 0, 0, 0, u32::MAX]);
        }
    }
    let flags = if read_only { 1 } else { 0x246 };
    tactics_item([flags, (cx - 0xd) as u32, top as u32, (cx + 0xd) as u32, bottom as u32, 0, 0, 0x10030, line, line, 0, 1, back,
        JOB_TEXT as u32, 0, 0x1d, ord, u32::MAX]);
    if overview && !read_only {
        tactics_item([0x346, (cx - 0xd) as u32, top as u32, (cx + 0xd) as u32, bottom as u32, 0, 0, 0x10020, line, line, 0, 1, back,
            0, 0, 0x1e, ord, u32::MAX]);
    }
}

/// The customised mark around a circle.
unsafe fn tactics_slot_customised_mark(cx: i32, top: i32, bottom: i32, back: u32) {
    tactics_item([1, (cx - 0xe) as u32, top as u32, (cx + 0xe) as u32, bottom as u32, 0, 0, 0x10020, 0, 0, 0, 1, back, 0, 0, 0, 0, u32::MAX]);
}

/// The overview: the run arrows, then each slot's circle at its defending position, with the name under it.
unsafe fn tactics_build_overview(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let flags = if read_u32(ts + t::READ_ONLY) != 0 { 1 } else { 6 };
    let r = pitch.r;
    tactics_item([flags, r[0] as u32, r[1] as u32, r[2] as u32, r[3] as u32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0, u32::MAX]);
    tactics_build_overview_runs(ts, pitch);
    let fm = ts + t::FORMATION;
    for slot in 0..0xbi8 {
        let ord = slot as u32;
        let mut pending: u32 = 0;
        let row = x::find_slot_at_position(ts, slot, &mut pending);
        if row <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_position(fm, slot, &mut later) == 0 {
            continue;
        }
        let position = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let players = if read_u32(ts + t::MATCH) != 0 { ts + t::MATCH_PLAYERS } else { 0 };
        let wide = x::central_in_line(fm, position, players);
        let (mut cx, mut cy) = (0i32, 0i32);
        x::pitch_point(position, &pitch.r, &mut cx, &mut cy, wide);
        let (back, line, name_colour) = if pending == 0 {
            let keeper = position & 1 != 0;
            (if keeper { pitch.keeper_back } else { pitch.field_back }, if keeper { pitch.keeper_line } else { pitch.field_line },
                tactics_colour(JOB_COLOUR_BDEC))
        } else {
            let g = tactics_colour(tactics_build_statics::GREEN_80);
            (g, tactics_colour(tactics_build_statics::GREEN_40), g)
        };
        x::slot_number_text(ts, row);
        tactics_slot_circle(ts, cx, cy, ord, back & 0xffff, line & 0xffff, later, true);
        if x::slot_customised(ts, slot) != 0 {
            tactics_slot_customised_mark(cx, cy - 0x13, cy + 9, back & 0xffff);
        }
        x::slot_name_text(ts, row, 5);
        tactics_item([1, (cx - 0x2d) as u32, (cy + 6) as u32, (cx + 0x2d) as u32, (cy + 0x10) as u32, 0, 0, 1, 0, 0, 0xc, 1, name_colour,
            JOB_TEXT as u32, 0, 0, 0, u32::MAX]);
    }
}

/// A ball zone square of the 3 x 4 grid (x 0..2 across, y 0..3 up from the bottom), grown by `grow` pixels.
fn tactics_zone_rect(r: &[i32; 4], zx: i32, zy: i32, grow: i32) -> [u32; 4] {
    let (w, h) = (r[2] - r[0], r[3] - r[1]);
    [((zx * w) / 3 + r[0] - grow) as u32, (r[3] - ((zy + 1) * h) / 4 - grow) as u32, (((zx + 1) * w) / 3 + r[0] + grow) as u32,
        (r[3] - (zy * h) / 4 + grow) as u32]
}

/// A formation coordinate (27 x 36 grid) as the pitch square around it: (left, bottom, right, top) like the original's
/// registers: x = c.x * w / 27 + left, y = bottom - c.y * h / 36, the square w / 27 wide.
fn tactics_coordinate_square(r: &[i32; 4], c: [i16; 2]) -> (i32, i32, i32, i32) {
    let (w, h) = (r[2] - r[0], r[3] - r[1]);
    let x = (c[0] as i32).wrapping_mul(w) / 27 + r[0];
    let y = r[3] - (c[1] as i32).wrapping_mul(h) / 36;
    let d = w / 27;
    (x, y, x + d, y - d)
}

/// The with / without the ball views: the 12 zones (clickable when editable, id 0x1f, value y + 100 x), the runs
/// to the zone shown, the zone under the mouse and the zone shown, then the circles at the coordinates of the zone.
unsafe fn tactics_build_zones(ts: usize, pitch: &TacticsPitch) {
    use tactics_screen as t;
    use tactics_calls as x;
    let r = pitch.r;
    let read_only = read_u32(ts + t::READ_ONLY) != 0;
    let green = tactics_colour(tactics_build_statics::GREEN_60);
    for zx in 0..3 {
        for zy in 0..4 {
            let q = tactics_zone_rect(&r, zx, zy, 0);
            let flags = if read_only { 1 } else { 0x306 };
            tactics_item([flags, q[0], q[1], q[2], q[3], 0, 0, 0x2220, green, green, 0, 0, 0, 0, 0, 0x1f, (zy + 100 * zx) as u32, u32::MAX]);
        }
    }
    let zone = read_u32(ts + t::ZONE);
    let fm = ts + t::FORMATION;
    if read_u32(ts + t::SHOW_RUNS) != 0 {
        for slot in 0..0xbi8 {
            let mut pending: u32 = 0;
            if x::find_slot_at_position(ts, slot, &mut pending) <= -1 {
                continue;
            }
            let mut later: u32 = 0;
            if x::duplicate_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as u8, read_u32(ts + t::ZONE), &mut later) == 0 {
                continue;
            }
            let mut c = [0i16; 2];
            x::slot_coordinate(ts, slot, &mut c);
            let (ax1, ay1, ax2, ay2) = tactics_coordinate_square(&r, c);
            exe::formation_get_slot_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as i8, read_u32(ts + t::ZONE), c.as_mut_ptr() as *mut u32);
            let (bx1, by1, bx2, by2) = tactics_coordinate_square(&r, c);
            let colour = if pending == 0 { read_u32(tactics_build_statics::RUN) } else { tactics_colour(tactics_build_statics::GREEN_80) };
            tactics_item([1, ((ax2 - ax1) / 2 + ax1) as u32, ((ay1 - ay2) / 2 + ay2) as u32, ((bx2 - bx1) / 2 + bx1) as u32,
                ((by1 - by2) / 2 + by2) as u32, 0, 0, 0x2400, colour & 0xffff, 0, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        }
        let hover = read_u32(ts + t::HOVER_ZONE);
        if hover as u16 != zone as u16 || (hover >> 16) as u16 != (zone >> 16) as u16 {
            let q = tactics_zone_rect(&r, hover as u16 as i16 as i32, (hover >> 16) as u16 as i16 as i32, 1);
            let colour = tactics_colour(tactics_build_statics::HOVER_ZONE);
            tactics_item([1, q[0], q[1], q[2], q[3], 0, 0, 0x2220, colour, colour, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
        }
    }
    let q = tactics_zone_rect(&r, zone as u16 as i16 as i32, (zone >> 16) as u16 as i16 as i32, 1);
    let colour = tactics_colour(tactics_build_statics::ZONE);
    tactics_item([1, q[0], q[1], q[2], q[3], 0, 0, 0x2220, colour, colour, 0, 0, 0, 0, 0, 0, 0, u32::MAX]);
    let flags = if read_only { 1 } else { 6 };
    tactics_item([flags, r[0] as u32, r[1] as u32, r[2] as u32, r[3] as u32, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0x1c, 0, u32::MAX]);
    for slot in 0..0xbi8 {
        let ord = slot as u32;
        let mut pending: u32 = 0;
        let row = x::find_slot_at_position(ts, slot, &mut pending);
        if row <= -1 {
            continue;
        }
        let mut later: u32 = 0;
        if x::duplicate_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as u8, read_u32(ts + t::ZONE), &mut later) == 0 {
            continue;
        }
        let mut c = [0i16; 2];
        exe::formation_get_slot_coordinate(fm, slot, (read_u8(ts + t::VIEW) == 2) as i8, read_u32(ts + t::ZONE), c.as_mut_ptr() as *mut u32);
        let (x1, y1, x2, y2) = tactics_coordinate_square(&r, c);
        let position = exe::formation_get_slot_position(fm, slot as u8 as u32, 0);
        let (back, line) = if pending == 0 {
            let keeper = position & 1 != 0;
            (if keeper { pitch.keeper_back } else { pitch.field_back }, if keeper { pitch.keeper_line } else { pitch.field_line })
        } else {
            (tactics_colour(tactics_build_statics::GREEN_80), tactics_colour(tactics_build_statics::GREEN_40))
        };
        x::slot_number_text(ts, row);
        let (cx, cy) = ((x2 - x1) / 2 + x1, (y1 - y2) / 2 + y2);
        tactics_slot_circle(ts, cx, cy, ord, back & 0xffff, line & 0xffff, later, false);
        if x::slot_customised(ts, slot) != 0 {
            tactics_slot_customised_mark(cx, cy - 0xe, cy + 0xe, back & 0xffff);
        }
    }
}

/// Builds the tactics screen.
///
/// The first build sets the static colours (`tactics_build_statics`). The Tactics_Screen of parameter 0 is built
/// when missing (from parameters 1, 2, 3, 4, 7 and 8) and stored back; the view column is parameter 0xa. Background:
/// the stadium picture of the match record (else its competition's), or the club's. The title (`format_title`, its
/// two colours) and the menu bar (mode 4). A read-only screen has only the Team Instructions button; an editable one,
/// the Tactics, View, Last Match and Edit menus (`tactics_build_menus`). Then the formation name (with the unsaved
/// mark when not saved), LOCKED, the view tabs and the list's column widths (`tactics_build_header`); the list with
/// the heading of the instruction column (views 7 and up: one row lower) and its 20 rows (`tactics_build_row`); the
/// pitch (`draw_pitch` 0x8ac600, with the keeper's colours dark on a white kit); the overview (`tactics_build_overview`)
/// or the zone views (`tactics_build_zones`); and Cancel / Ok (editable) or the Back and Next buttons (read-only).
/// Original: `tactics_screen_build` (0x899ea0, tactics_screens.cpp, cdecl).
pub(crate) unsafe extern "C" fn tactics_screen_build() {
    use tactics_screen as t;
    covered!("tel14_2 build the tactics screen");
    let c = tactics_colour;
    tactics_build_statics_init();
    let mut widths: [u8; 9] = [0xe, 1, 5, 1, 0x3c, 0xf, 0x10, 7, 7];
    let mut list_widths: [u8; 2] = [0xe, 0x70];
    let mut button_widths: [u8; 2] = [3, 1];
    let mut ts = get_screen_param(0);
    let view = get_screen_param(0xa) as u8;
    if ts == 0 {
        let p = exe::crt_operator_new(t::SIZE);
        ts = 0;
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
    if read_u32(ts + t::HAS_MATCH_RECORD) != 0 {
        let stadium = read_usize(ts + t::STADIUM);
        if stadium == 0 || exe::background_set_stadium_picture(stadium, 0, 1) == 0 {
            exe::background_set_comp_picture(read_usize(ts + t::COMP), 0, 0);
        }
    } else {
        exe::background_set_club_picture(read_usize(ts + t::CLUB), 0, 0);
    }
    let (mut title_text, mut title_back) = (0u32, 0u32);
    tactics_calls::format_title(ts, &mut title_text, &mut title_back);
    exe::screenmanager_add_screen_title(G_SCREEN_MANAGER, title_back, title_text, JOB_TEXT);
    exe::menubar_build(4, 0);
    let mut heading = [0u8; 0xc8];
    if read_u32(ts + t::READ_ONLY) != 0 {
        tactics_text(0xa8bfa4);
        tel26_add_button([0x6e, 0x50, 0xeb, 0x64, 0x30, c(JOB_COLOUR_317E), 0xc, 1, c(JOB_COLOUR_3184), JOB_TEXT as u32, 1, 0]);
    } else {
        tactics_build_menus(ts, view, &mut heading);
    }
    tactics_build_header(ts, view, &mut widths);
    // the player list
    tactics_area([0x6e, 0x91, 0x1b8, 0x221, 1, 0, 1, 0, 2, 0, u32::MAX]);
    let (list, handles, mut line) = if heading[0] != 0 {
        let list = tactics_area([0x6e, 0x91, 0x1b8, 0x221, 9, widths.as_mut_ptr() as u32, 0x15, 0, 1, 0, u32::MAX]);
        let handles = tactics_area([0x6e, 0x91, 0x1b8, 0x221, 2, list_widths.as_mut_ptr() as u32, 0x15, 0, 1, 0, u32::MAX]);
        tel26_area_text([list, 5, 0, 0x10, c(JOB_COLOUR_BDEA), 0xc, 1, c(JOB_COLOUR_3184), heading.as_ptr() as u32]);
        (list, handles, 1u32)
    } else {
        let list = tactics_area([0x6e, 0x99, 0x1b8, 0x219, 9, widths.as_mut_ptr() as u32, 0x14, 0, 1, 0, u32::MAX]);
        let handles = tactics_area([0x6e, 0x99, 0x1b8, 0x219, 2, list_widths.as_mut_ptr() as u32, 0x14, 0, 1, 0, u32::MAX]);
        (list, handles, 0u32)
    };
    for k in 0..0x14i8 {
        tactics_build_row(ts, view, k, list, handles, line);
        line += 1;
    }
    // the pitch
    let mut r = [0i32; 4];
    let draw: unsafe extern "C" fn(i32, i32, i32, i32, *mut i32, *mut i32, *mut i32, *mut i32, u32, i32, u32) -> u32 = core::mem::transmute(FN_DRAW_PITCH);
    let (left, top, right, bottom) = (&mut r[0] as *mut i32, &mut r[1] as *mut i32, &mut r[2] as *mut i32, &mut r[3] as *mut i32);
    draw(0x1c2, 0x91, 0x30c, 0x221, left, top, right, bottom, 0, -1, 0x40800000);
    let club = read_usize(ts + t::CLUB);
    let white = |kit: u8, colour_at: usize| read_u8(ts + t::KIT) == kit && read_usize(club + colour_at) != 0
        && exe::database_get_colour_family(read_usize(club + colour_at)) == 5;
    let keeper_line = if white(1, 0x87) || white(2, 0x8f) || white(3, 0x97) { c(JOB_COLOUR_BDEC) } else { read_u32(tactics_build_statics::GREEN_90) };
    let pitch = TacticsPitch { r, keeper_back: c(JOB_COLOUR_BDEA), keeper_line, field_back: title_text, field_line: title_back };
    if read_u8(ts + t::VIEW) == 1 {
        tactics_build_overview(ts, &pitch);
    } else {
        tactics_build_zones(ts, &pitch);
    }
    if read_u32(ts + t::READ_ONLY) == 0 {
        let buttons = tactics_area([0x64, 0x22b, 0x316, 0x24e, 2, button_widths.as_mut_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
        tactics_text(0x98a1a0);
        tel26_area_button([buttons, 0, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x20, 0]);
        tactics_text(0x990f7c);
        tel26_area_button([buttons, 1, 0, 0x30, c(JOB_COLOUR_317E), 0xc, 3, c(JOB_COLOUR_3184), JOB_TEXT as u32, 0x21, 0]);
    } else {
        tel26_back_next();
    }
}

/// Comparer for `tactics_screen_build`.
///
/// Side by side over the built display list (`compare_match_screen_build`: the screen manager, the current
/// Display, the parameter block, the text buffer and the generator), with the Tactics_Screen object and the static
/// colours. Only the original when the object does not exist yet (it is allocated here).
pub(crate) unsafe extern "C" fn compare_tactics_screen_build() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let ts = get_screen_param(0);
    if ts == 0 {
        // simplification: the allocation of the object does not run twice; proven in Rust mode
        return o();
    }
    let extra = [(ts, tactics_screen::SIZE as usize), (tactics_build_statics::GUARD, 0xa), (tactics_build_statics::GUARD2, 1),
        (tactics_build_statics::GREEN_90, 2), (tactics_build_statics::ZONE, 4), (tactics_build_statics::HOVER_ZONE, 2),
        (tactics_build_statics::RED, 4)];
    compare_match_screen_build("tactics_screen_build", &extra, || o(), || tactics_screen_build());
}
