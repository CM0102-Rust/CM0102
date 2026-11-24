// ---------- sbC_5: the game's message box (goldcup.cpp by the map; the graphics library) ----------

/// Nonzero while the game runs (WinMain's loop).
const GAME_RUNNING: usize = 0xb67a30;
/// Nonzero while a game message box is open (a box asked meanwhile returns at once).
const MESSAGE_BOX_OPEN: usize = 0xb0be30;
/// The box's fixed rows: the box from y 242 to 357, the title band to 264, the text from 266 to 315, the buttons from
/// 318 to 342; a button is 74 pixels wide.
const BOX_TOP: i32 = 0xf2;
const BOX_BOTTOM: i32 = 0x165;
const BUTTON_TOP: i32 = 0x13e;
const BUTTON_BOTTOM: i32 = 0x156;
const BUTTON_WIDTH: i32 = 0x4a;
/// The immediates of the box's width cap and the screen width it is centred on (600 and 800 in cm0102.exe, 1080 and
/// 1920 in the 1080 exe built by resolucao.py): read from the running exe's code.
const BOX_WIDTH_CAP_COMPARED: usize = 0x5e82f5;
const BOX_WIDTH_CAP: usize = 0x5e8305;
const BOX_SCREEN_WIDTH: usize = 0x5e830a;
/// "Ok" and "Quit <%s - COMMENT - Option to quit the game>", formatted by the language library (the %s with
/// `EMPTY_STRING`).
const OK_TEXT: usize = 0x990f7c;
const QUIT_TEXT: usize = 0x9cda80;
/// The key code of Enter (`DISPLAY_KEY_CODE`).
const KEY_ENTER: u32 = 0xd;

/// Where the box goes: its left and right edges, the OK button's left edge and the Quit button's box (all −1 without
/// it).
struct MessageBoxLayout {
    left: i32,
    right: i32,
    ok_left: i32,
    quit: [i32; 4],
}

impl MessageBoxLayout {
    /// The box is the text's width in font 1 plus 24, between 200 and the exe's cap, centred on the exe's screen width;
    /// the OK button centred, or OK and Quit side by side (85 pixels apart) centred with `allow_exit`.
    unsafe fn new(text: usize, allow_exit: bool) -> Self {
        let width = exe::font_text_width(1i32, text as *const u8) + 0x18;
        let width = if width <= 0xc8 {
            0xc8
        } else if width >= exe_immediate(BOX_WIDTH_CAP_COMPARED as *const i32) {
            exe_immediate(BOX_WIDTH_CAP as *const i32)
        } else {
            width
        };
        let left = (exe_immediate(BOX_SCREEN_WIDTH as *const i32) - width) / 2;
        let right = left + width - 1;
        if allow_exit {
            let ok_left = (width - 0x8c) / 2 + left;
            MessageBoxLayout { left, right, ok_left, quit: [ok_left + 0x55, BUTTON_TOP, ok_left + 0x55 + BUTTON_WIDTH, BUTTON_BOTTOM] }
        } else {
            MessageBoxLayout { left, right, ok_left: (width - 0x4b) / 2 + left, quit: [-1; 4] }
        }
    }

    fn ok(&self) -> [i32; 4] {
        [self.ok_left, BUTTON_TOP, self.ok_left + BUTTON_WIDTH, BUTTON_BOTTOM]
    }
}

/// (x, y) lies in the box `b` (edges included).
fn point_in(b: [i32; 4], x: i32, y: i32) -> bool {
    x >= b[0] && y >= b[1] && x <= b[2] && y <= b[3]
}

/// Draws a button of the box: pressed, a sunken panel (flags 0x70) with its label two pixels down and right; otherwise
/// a raised panel (0x30) with the label centred on it (font 1, pale grey).
unsafe fn draw_message_box_button(b: [i32; 4], pressed: bool, label: usize) {
    let grey = read_u16(G_GUI_COLOUR_GREY) as u32;
    let pale_grey = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    let (flags, shift) = if pressed { (0x70, 2) } else { (0x30, 0) };
    exe::gui_draw_panel(b[0], b[1], b[2], b[3], flags, grey, 0);
    exe::gui_draw_text_box(b[0] + shift, b[1] + shift, b[2] + shift, b[3] + shift, 0xc, 1, pale_grey, label, -1);
}

/// Draws the whole box: the grey panel, the dark blue title band with the title, the text (word-wrapped), the OK button
/// and, with `allow_exit`, the Quit button (their labels through the language library), then flips it to the screen and
/// clears the screen-updated mark.
unsafe fn draw_message_box(layout: &MessageBoxLayout, title: usize, text: usize, allow_exit: bool, ok_pressed: bool, quit_pressed: bool) {
    let (left, right) = (layout.left, layout.right);
    let pale_grey = read_u16(G_GUI_COLOUR_PALE_GREY) as u32;
    exe::gui_draw_panel(left, BOX_TOP, right, BOX_BOTTOM, 0x30, read_u16(G_GUI_COLOUR_GREY) as u32, 0);
    exe::gui_draw_panel(left + 2, 0xf4, right - 2, 0x108, 0x210, read_u16(G_GUI_COLOUR_DARK_BLUE) as u32, 0);
    exe::gui_draw_text_box(left + 10, 0xf4, right - 10, 0x108, 9, 1, pale_grey, title, -1);
    exe::gui_draw_text_box(left + 12, 0x10a, right - 12, 0x13b, 0x10c, 1, pale_grey, text, -1);
    let format: unsafe extern "C" fn(usize, usize, ...) -> usize = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut label = [0u8; 0x100];
    let label_at = label.as_mut_ptr() as usize;
    format(label_at, OK_TEXT);
    draw_message_box_button(layout.ok(), ok_pressed, label_at);
    if allow_exit {
        format(label_at, QUIT_TEXT, EMPTY_STRING);
        draw_message_box_button(layout.quit, quit_pressed, label_at);
    }
    exe::graphics_flip_rect(left as u32, BOX_TOP as u32, right as u32, BOX_BOTTOM as u32);
    write_u32(SCREEN_UPDATED, 0);
}

/// Sets the cursor of `mode` when the cursor mode is another one (hidden and shown around it).
unsafe fn switch_cursor_mode(mode: u32) {
    if read_u32(CURSOR_MODE) == mode {
        return;
    }
    user32_show_cursor(0);
    write_u32(CURSOR_MODE, mode);
    if let Some(cursor) = cursor_of_mode(mode) {
        user32_set_cursor(cursor);
    }
    user32_show_cursor(1);
}

/// Shows the game's own message box with `title` and `text` and waits for OK (a click released on it, or Enter); with
/// `allow_exit` a Quit button ends the game.
///
/// Before the game runs, or with the display off, it is a Windows box instead (MessageBoxA on the game window, its
/// result returned). While another game box is open it returns 1 at once. Otherwise (`MessageBoxLayout`) the screen
/// under the box is saved and the cursor mode kept; then, until it closes: the box drawn when needed
/// (`draw_message_box`), the messages pumped, the pointing cursor over a button and the normal one elsewhere, a button
/// pressed by the left button over it and released (outside: back up; on Quit: game_fatal_cleanup and exit(−1)), Enter
/// closes it too (except in the pass where the Quit button went down or back up), and the box is drawn again when
/// something else updated the screen.
/// On closing the cursor mode is put back, the screen under the box restored and flipped, and the saved image freed.
/// Returns 0 then (the original leaves an undefined value; nobody reads it).
/// Original: `gui_show_message_box` (0x5e8290, goldcup.cpp, cdecl; its width cap and screen width read from the exe's
/// immediates).
pub(crate) unsafe extern "C" fn gui_show_message_box(title: usize, text: usize, allow_exit: u32) -> u32 {
    if read_u32(GAME_RUNNING) == 0 || read_u32(DISPLAY_OFF) != 0 {
        let message_box: unsafe extern "system" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(read_usize(0x9671d4)); // MessageBoxA
        return message_box(read_usize(MAIN_WINDOW), text, title, 0) as u32;
    }
    if read_u32(MESSAGE_BOX_OPEN) != 0 {
        return 1;
    }
    write_u32(MESSAGE_BOX_OPEN, 1);
    let allow_exit = allow_exit != 0;
    let layout = MessageBoxLayout::new(text, allow_exit);
    let saved = exe::gui_save_screen_image(layout.left, BOX_TOP, layout.right, BOX_BOTTOM, 0usize);
    let saved_cursor_mode = read_u32(CURSOR_MODE);
    let (mut redraw, mut ok_pressed, mut quit_pressed) = (true, false, false);
    loop {
        if redraw {
            draw_message_box(&layout, title, text, allow_exit, ok_pressed, quit_pressed);
            redraw = false;
        }
        exe::pump_messages();
        let over = |b: [i32; 4]| point_in(b, read_i32(DISPLAY_MOUSE_X), read_i32(DISPLAY_MOUSE_Y));
        let over_a_button = over(layout.ok()) || (allow_exit && over(layout.quit));
        switch_cursor_mode(if over_a_button { 2 } else { 0 });
        let down = read_u32(G_MOUSE_LEFT_BUTTON_DOWN) != 0;
        if !ok_pressed {
            if down && over(layout.ok()) {
                ok_pressed = true;
                redraw = true;
            }
        } else if !down {
            if over(layout.ok()) {
                break;
            }
            ok_pressed = false;
            redraw = true;
        }
        let mut quit_changed = false;
        if allow_exit {
            if !quit_pressed {
                if down && over(layout.quit) {
                    quit_pressed = true;
                    redraw = true;
                    quit_changed = true;
                }
            } else if !down {
                if over(layout.quit) {
                    covered!("sbC_5 quit");
                    exe::game_fatal_cleanup();
                    exe::shared_noop_ret0_5a8f60();
                    exe::crt_exit(-1);
                }
                quit_pressed = false;
                redraw = true;
                quit_changed = true;
            }
        }
        if !quit_changed && read_u32(DISPLAY_KEY_CODE) == KEY_ENTER {
            write_u32(DISPLAY_KEY_CODE, 0);
            break;
        }
        if read_u32(SCREEN_UPDATED) != 0 {
            redraw = true;
        }
    }
    covered!("sbC_5 box closed");
    switch_cursor_mode(saved_cursor_mode);
    exe::gui_restore_screen_image(layout.left, BOX_TOP, saved);
    exe::graphics_flip_rect(layout.left as u32, BOX_TOP as u32, layout.right as u32, BOX_BOTTOM as u32);
    gui_free_image(saved);
    write_u32(MESSAGE_BOX_OPEN, 0);
    0
}

/// Comparer: only the original (it waits for the player's click).
pub(crate) unsafe extern "C" fn compare_gui_show_message_box(title: usize, text: usize, allow_exit: u32) -> u32 {
    original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u32>(%%I0%%)(title, text, allow_exit)
}
