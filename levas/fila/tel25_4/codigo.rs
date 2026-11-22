// ---------- batch %%LEVA%%: the training screens (3): the schedule editor's builder (training_screens.cpp) ----------

/// The schedule editor's "choosable" colour [0xde1f4c] (a static word, green: `graphics_rgb_to_pixel(0, 0xc0, 0, 0)`)
/// and the flag byte [0xde1f49] whose bit 0 says it is made.
const SCHEDULE_EDITOR_GREEN: usize = 0xde1f4c;
const SCHEDULE_EDITOR_GREEN_MADE: usize = 0xde1f49;
/// The immediates the 1920x1080 exe patches in the schedule editor: the right (push 0x31f at 0x8b9a7b) and bottom (push
/// 0x257 at 0x8b9a73) edges of its full-screen area, read from the running exe.
const SCHEDULE_EDITOR_RIGHT_EDGE: *const u32 = 0x8b9a7c as *const u32;
const SCHEDULE_EDITOR_BOTTOM_EDGE: *const u32 = 0x8b9a74 as *const u32;
/// A schedule's target flags (word +0x16): the position bits (0x207f; 0x2000 alone means no specific position named)
/// and the side bits (0x4a80; 0x4000 the same for the side) [inference: from the editor's two blocks].
const SCHEDULE_TARGETS: usize = 0x16;
/// The position bits the "New Position" popup offers, and the side bits of "New Side".
const SCHEDULE_POSITION_BITS: [u16; 7] = [1, 2, 4, 8, 0x10, 0x20, 0x40];
const SCHEDULE_SIDE_BITS: [u16; 3] = [0x80, 0x200, 0x800];
/// "" (0xad9c64) and "  " (0xa78880).
const SCHEDULE_EMPTY_TEXT: usize = 0xad9c64;
const SCHEDULE_INDENT: usize = 0xa78880;

/// A choice cell of the schedule editor, its text already in the buffer: the chosen one is yellow text on the 0x221
/// style; the others are green buttons with the link and the value.
unsafe fn schedule_editor_choice(area: u32, col: u32, row: u32, chosen: bool, link: u32, value: u32) {
    if chosen {
        let yellow = read_u16(G_GUI_COLOUR_YELLOW) as u32;
        tel26_area_text([area, col, row, 0x221, yellow, 0xc, 2, yellow, TRAINING_TEXT_BUFFER as u32]);
    } else {
        tel26_area_button([area, col, row, 1, 0, 0xc, 2, read_u16(SCHEDULE_EDITOR_GREEN) as u32, TRAINING_TEXT_BUFFER as u32, link, value]);
    }
}

/// The "New Position" / "New Side" row of the schedule editor (row `row` of `area`): "None" (column 1) chosen when the
/// schedule has none of the `mask` bits, "Specific" (column 2, value `specific`) when it has some; with some, a popup
/// button in column 4 lists `bits` (items of kind 2 with the link and the bit as value) and, unless the `exact` bit is
/// set, column 3 shows the bits named (two spaces, then `formation_get_position_flag_name` 0x5b7ab0).
unsafe fn schedule_editor_target_row(area: u32, row: u32, schedule: usize, none_text: usize, specific_text: usize, mask: u16, exact: u16,
    link: u32, specific: u32, bits: &[u16]) {
    let flag_name: unsafe extern "C" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x5b7ab0usize);
    let targets = || read_u16(schedule + SCHEDULE_TARGETS);
    comp_lang_format()(TRAINING_TEXT_BUFFER, none_text, SCHEDULE_EMPTY_TEXT);
    schedule_editor_choice(area, 1, row, targets() & mask == 0, link, 0);
    comp_lang_format()(TRAINING_TEXT_BUFFER, specific_text, SCHEDULE_EMPTY_TEXT);
    schedule_editor_choice(area, 2, row, targets() & mask != 0, link, specific);
    if targets() & mask == 0 {
        return;
    }
    covered!("tel25_4 a specific training target");
    let black = read_u16(G_GUI_COLOUR_BLACK) as u32;
    let owner = tel26_area_button([area, 4, row, 0x30, black, 0xc, 1, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, 0, 0, 0]);
    let popup_colour = read_u16(0xae31aa) as u32;
    let popup = tel26_add_area(current_human_display(), [0, 0, 0, 0, 0, 0, 0, 0, 0x500030, popup_colour, owner]);
    for &bit in bits {
        flag_name(bit as u32, TRAINING_TEXT_BUFFER, 0x7d0);
        tel26_add_item(current_human_display(), [2, 0, 0, 0, 0, 0, 0, 0x10, popup_colour, read_u16(G_GUI_COLOUR_YELLOW) as u32, 1, 1,
            read_u16(G_GUI_COLOUR_BLACK) as u32, TRAINING_TEXT_BUFFER as u32, 0, link, bit as i16 as i32 as u32, popup]);
    }
    if targets() & exact == 0 {
        write_u16(TRAINING_TEXT_BUFFER, read_u16(SCHEDULE_INDENT));
        write_u8(TRAINING_TEXT_BUFFER + 2, 0);
        flag_name((targets() & mask) as u32, TRAINING_TEXT_BUFFER + 2, 0x7d0);
        tel26_area_text([area, 3, row, 1, 0, 1, 2, read_u16(G_GUI_COLOUR_YELLOW) as u32, TRAINING_TEXT_BUFFER as u32]);
    }
}

/// Rule: the schedule editor: the schedule's name box, the level of each of the five training categories (0, 10, 25 or
/// 50: the current one yellow, the others green buttons) and the position and side it targets; Cancel, and Ok only
/// when the schedule has a name.
///
/// Parameter 0 is the session, 1 the schedule being edited. The green is made once (`SCHEDULE_EDITOR_GREEN`). A
/// schedule whose byte +0x15 is 7 has its name (+9) emptied first. Then the background picture
/// (`exe::background_set_picture(0)`), a full-screen area (its right and bottom edges read from the exe, patched in the
/// 1080 exe), the dark green frame (0xa0, 0x7d)–(0x2d9, 0x1da), the title (the session's own copy of the schedule's
/// name, +0xfa + 0x9b × (i8)[session+0x7d9] + 9), a two-column area (weights 3, 1) with Cancel (id 2) and Ok (id 1;
/// grey text with style 0x2c when the name is empty), the pitch (`draw_pitch` (0xb4, 0xc8)–(0x2c5, 0x19e), its outputs
/// unused) and three areas over the same rectangle (0xb9, 0xcd)–(0x2c0, 0x199), 8 rows each: 4 columns (weights 10, 2,
/// 10, 12) for the name, 5 (10, 6, 6, 6, 6) for the categories and 5 (10, 6, 6, 11, 1) for the targets. Row 0: " Name"
/// and an edit box (an item 0x80018 on the name, 11 characters, id 9). Rows 1 to 5: a space and the category name
/// (`training_schedule_get_category_name` 0x8b5520) and the four levels (`training_schedule_get_intensity_name`
/// 0x8b5690 of 0, 10, 0x19, 0x32; the level from `exe::training_schedule_get_category_level`; links 3 to 6 with the
/// category as value). Row 6: " New Position" (`schedule_editor_target_row`, mask 0x207f, link 7, Specific = 1, the 7
/// position bits); row 7: " New Side" (mask 0x4a80, link 8, Specific = 0x80, the 3 side bits).
/// Original: `training_screens_draw_schedule_editor` (0x8b9940, training_screens.cpp, cdecl)
pub(crate) unsafe extern "C" fn training_screens_draw_schedule_editor() {
    let made = read_u8(SCHEDULE_EDITOR_GREEN_MADE);
    if made & 1 == 0 {
        write_u8(SCHEDULE_EDITOR_GREEN_MADE, made | 1);
        write_u16(SCHEDULE_EDITOR_GREEN, exe::graphics_rgb_to_pixel(0, 0xc0, 0, 0) as u16);
    }
    let buttons_weights: [u8; 2] = [3, 1];
    let name_weights: [u8; 4] = [0xa, 2, 0xa, 0xc];
    let level_weights: [u8; 5] = [0xa, 6, 6, 6, 6];
    let target_weights: [u8; 5] = [0xa, 6, 6, 0xb, 1];
    let session = get_screen_param(0);
    let schedule = get_screen_param(1);
    if read_u8(schedule + 0x15) == 7 {
        write_u8(schedule + 9, 0);
    }
    exe::background_set_picture(0usize);
    tel26_add_area(current_human_display(), [0, 0, SCHEDULE_EDITOR_RIGHT_EDGE.read_unaligned(), SCHEDULE_EDITOR_BOTTOM_EDGE.read_unaligned(), 1, 0, 1, 0, 2, 0, u32::MAX]);
    let frame = exe::graphics_rgb_to_pixel(0, 0x38, 0, 0);
    tel26_add_area(current_human_display(), [0xa0, 0x7d, 0x2d9, 0x1da, 1, 0, 1, 0, 0x30, frame, u32::MAX]);
    let title = session + 0xfa + read_i8(session + 0x7d9) as isize as usize * TRAINING_SCHEDULE_SIZE + 9;
    comp_text_item([0xaa, 0x87, 0x2cf, 0xb4, 0x30, read_u16(G_GUI_COLOUR_BLACK) as u32, 0xc, 4, read_u16(G_GUI_COLOUR_PALE_GREY) as u32, title as u32, 1]);
    let buttons = tel26_add_area(current_human_display(), [0xaa, 0x1b2, 0x2cf, 0x1d0, 2, buttons_weights.as_ptr() as u32, 1, 0, 1, 0, u32::MAX]);
    let (grey, pale_grey) = (read_u16(G_GUI_COLOUR_GREY) as u32, read_u16(G_GUI_COLOUR_PALE_GREY) as u32);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98a1a0);
    tel26_area_button([buttons, 0, 0, 0x30, grey, 0xc, 3, pale_grey, TRAINING_TEXT_BUFFER as u32, 2, 0]);
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x990f7c);
    let name = schedule + 9;
    if read_u8(name) == 0 {
        tel26_area_text([buttons, 1, 0, 0x30, grey, 0x2c, 3, pale_grey, TRAINING_TEXT_BUFFER as u32]);
    } else {
        tel26_area_button([buttons, 1, 0, 0x30, grey, 0xc, 3, pale_grey, TRAINING_TEXT_BUFFER as u32, 1, 0]);
    }
    let (mut x1, mut y1, mut x2, mut y2) = (0i32, 0i32, 0i32, 0i32);
    type DrawPitch = unsafe extern "C" fn(u32, u32, u32, u32, *mut i32, *mut i32, *mut i32, *mut i32, u32, u32, u32);
    let draw_pitch: DrawPitch = core::mem::transmute(FN_DRAW_PITCH);
    draw_pitch(0xb4, 0xc8, 0x2c5, 0x19e, &mut x1, &mut y1, &mut x2, &mut y2, 1, buttons, 0x4040_0000);
    let name_area = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 4, name_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let levels = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 5, level_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let targets = tel26_add_area(current_human_display(), [0xb9, 0xcd, 0x2c0, 0x199, 5, target_weights.as_ptr() as u32, 8, 0, 1, 0, u32::MAX]);
    let heading = read_u16(0xae3154) as u32;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0x98ee04);
    tel26_area_text([name_area, 0, 0, 1, 0, 1, 3, heading, TRAINING_TEXT_BUFFER as u32]);
    tel26_add_item(current_human_display(), [0x80018, 0, 0, 0, 0, 2, 0, 1, 0, 0, 1, 2, read_u16(SCHEDULE_EDITOR_GREEN) as u32, name as u32, 0xb, 9, 0, name_area]);
    let category_name: unsafe extern "C" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x8b5520usize);
    let intensity_name: unsafe extern "C" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x8b5690usize);
    let mut row = 1u32;
    for category in 0..5u32 {
        write_u16(TRAINING_TEXT_BUFFER, 0x0020); // " "
        category_name(category, TRAINING_TEXT_BUFFER + 1, 0x7d0);
        tel26_area_text([levels, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
        let level = exe::training_schedule_get_category_level(schedule, category);
        for (col, value) in [(1u32, 0i8), (2, 0xa), (3, 0x19), (4, 0x32)] {
            intensity_name(value as u32, TRAINING_TEXT_BUFFER, 0x7d0);
            schedule_editor_choice(levels, col, row, level == value, col + 2, category);
        }
        row += 1;
    }
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d430);
    tel26_area_text([targets, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
    schedule_editor_target_row(targets, row, schedule, 0xa8d404, 0xa8d3d4, 0x207f, 0x2000, 7, 1, &SCHEDULE_POSITION_BITS);
    row += 1;
    comp_lang_format()(TRAINING_TEXT_BUFFER, 0xa8d3c8);
    tel26_area_text([targets, 0, row, 1, 0, 1, 3, read_u16(0xae3154) as u32, TRAINING_TEXT_BUFFER as u32]);
    schedule_editor_target_row(targets, row, schedule, 0xa8d3a0, 0xa8d374, 0x4a80, 0x4000, 8, 0x80, &SCHEDULE_SIDE_BITS);
}

/// Comparer for `training_screens_draw_schedule_editor`: side by side over the display list
/// (`compare_match_screen_build`) plus the schedule being edited (its name may be emptied) and the static colours.
pub(crate) unsafe extern "C" fn compare_training_screens_draw_schedule_editor() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I0%%);
    let schedule = get_screen_param(1);
    if schedule == 0 || get_screen_param(0) == 0 {
        return o();
    }
    compare_match_screen_build("schedule editor", &[(schedule, TRAINING_SCHEDULE_SIZE), (0xde1f48, 8)], || o(),
        || training_screens_draw_schedule_editor());
}
