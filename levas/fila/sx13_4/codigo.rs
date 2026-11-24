// ---------- batch %%LEVA%%: the game window's procedure and the restart (main.cpp) ----------

/// Globals and exe routines of the game window (main.cpp), with the window messages it handles.
#[allow(dead_code)]
pub(crate) mod main_window {
    /// u8: the character code of the last `WM_CHAR`.
    pub(crate) const G_LAST_CHAR_CODE: usize = 0xb67a24;
    /// u32: a left click arrived and the game has not handled it yet.
    pub(crate) const G_LEFT_CLICK_PENDING: usize = 0xb67a0c;
    /// u32: a right click arrived and the game has not handled it yet.
    pub(crate) const G_RIGHT_CLICK_PENDING: usize = 0xb67a1c;
    /// The single instance mutex (win_main).
    pub(crate) const G_SINGLE_INSTANCE_MUTEX: usize = 0xb67a28;
    /// The game window's handle.
    pub(crate) const G_MAIN_WINDOW: usize = 0xb67a2c;
    /// u32: 1 while the game runs.
    pub(crate) const G_GAME_RUNNING: usize = 0xb67a30;
    /// u32: 1 while the display is being created (the drawing routines skip their work).
    pub(crate) const G_DISPLAY_NOT_READY: usize = 0xaebe0c;
    /// The immediates of the full screen mode the activation restores (800 × 600 in cm0102.exe, 1920 × 1080 in the
    /// cm0102-1080.exe built by resolucao.py): read from the running exe's code.
    pub(crate) const RESTORE_WIDTH_IMMEDIATE: usize = 0x68d8d5;
    pub(crate) const RESTORE_HEIGHT_IMMEDIATE: usize = 0x68d8e0;
    /// `graphics_is_full_screen` (cdecl, returns the flag 0xae319c).
    pub(crate) const FN_GRAPHICS_IS_FULL_SCREEN: usize = 0x5e2ea0;
    /// `graphics_restore_lost_surfaces` (cdecl).
    pub(crate) const FN_GRAPHICS_RESTORE_LOST_SURFACES: usize = 0x5e2e50;
    /// `reapply_cursor_mode` (cdecl).
    pub(crate) const FN_REAPPLY_CURSOR_MODE: usize = 0x5e81e0;
    /// Import table slots (user32/kernel32), called through the table as the original does.
    pub(crate) const IAT_BEGIN_PAINT: usize = 0x9671f8;
    pub(crate) const IAT_END_PAINT: usize = 0x9671fc;
    pub(crate) const IAT_CHANGE_DISPLAY_SETTINGS: usize = 0x96719c;
    pub(crate) const IAT_CLOSE_WINDOW: usize = 0x967200;
    pub(crate) const IAT_SHOW_WINDOW: usize = 0x967204;
    pub(crate) const IAT_DEF_WINDOW_PROC: usize = 0x9671f4;
    pub(crate) const IAT_CLOSE_HANDLE: usize = 0x9670ec;
    pub(crate) const IAT_GET_COMMAND_LINE: usize = 0x9670cc;
    pub(crate) const IAT_WIN_EXEC: usize = 0x9670d4;
    pub(crate) const WM_ACTIVATE: u32 = 0x6;
    pub(crate) const WM_PAINT: u32 = 0xf;
    pub(crate) const WM_CLOSE: u32 = 0x10;
    pub(crate) const WM_ACTIVATEAPP: u32 = 0x1c;
    pub(crate) const WM_KEYDOWN: u32 = 0x100;
    pub(crate) const WM_CHAR: u32 = 0x102;
    pub(crate) const WM_MOUSEMOVE: u32 = 0x200;
    pub(crate) const WM_LBUTTONDOWN: u32 = 0x201;
    pub(crate) const WM_LBUTTONUP: u32 = 0x202;
    pub(crate) const WM_RBUTTONDOWN: u32 = 0x204;
    pub(crate) const WM_RBUTTONUP: u32 = 0x205;
    pub(crate) const WM_MOUSEWHEEL: u32 = 0x20a;
}

/// An exe routine with no argument and an int result (cdecl).
unsafe fn main_window_call(f: usize) -> u32 {
    let g: unsafe extern "C" fn() -> u32 = core::mem::transmute(f);
    g()
}

/// Writes the input globals of a keyboard or mouse message as the window procedure does, before any of its calls:
/// `WM_KEYDOWN` the key code, `WM_CHAR` the character; while no scroll key holds the mouse
/// (`DISPLAY_SCROLL_KEY_HELD` = 0), `WM_MOUSEMOVE` the position (x the low word, y the high word) and the buttons their
/// state (a press also marks the click pending). Returns false for the other messages (nothing written).
pub(crate) unsafe fn store_window_input(msg: u32, wparam: u32, lparam: u32) -> bool {
    use main_window::*;
    let free = read_u32(DISPLAY_SCROLL_KEY_HELD) == 0;
    match msg {
        WM_KEYDOWN => write_u32(DISPLAY_KEY_CODE, wparam),
        WM_CHAR => write_u8(G_LAST_CHAR_CODE, wparam as u8), // sx13_4: the character code
        WM_MOUSEMOVE if free => {
            write_u32(DISPLAY_MOUSE_X, lparam & 0xffff); // sx13_4: x from the low word
            write_u32(DISPLAY_MOUSE_Y, lparam >> 16);
        }
        WM_LBUTTONDOWN if free => {
            write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 1);
            write_u32(G_LEFT_CLICK_PENDING, 1);
        }
        WM_LBUTTONUP if free => write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0),
        WM_RBUTTONDOWN if free => {
            write_u32(DISPLAY_RIGHT_BUTTON, 1);
            write_u32(G_RIGHT_CLICK_PENDING, 1);
        }
        WM_RBUTTONUP if free => write_u32(DISPLAY_RIGHT_BUTTON, 0),
        WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP => {}
        _ => return false,
    }
    true
}

/// Redraws the whole current screen: the screen manager's slot word cleared around `Display::draw_screen(-1, -1, -1,
/// -1)` of the records display, then both mouse buttons released.
unsafe fn main_window_redraw() {
    let slot = read_u16(G_SCREEN_MANAGER_CURRENT_SLOT);
    write_u16(G_SCREEN_MANAGER_CURRENT_SLOT, 0);
    exe::display_draw_screen(read_usize(G_SCREEN_RECORDS_DISPLAY), u32::MAX, u32::MAX, u32::MAX, u32::MAX);
    write_u16(G_SCREEN_MANAGER_CURRENT_SLOT, slot);
    write_u32(G_MOUSE_LEFT_BUTTON_DOWN, 0);
    write_u32(DISPLAY_RIGHT_BUTTON, 0);
}

/// `WM_PAINT`: between `BeginPaint` and `EndPaint` of the game window (the global handle, not the message's), the
/// whole screen is redrawn.
unsafe fn main_window_paint() {
    use main_window::*;
    let mut paint = [0u8; 0x40]; // PAINTSTRUCT
    let begin: unsafe extern "system" fn(usize, *mut u8) -> usize = core::mem::transmute(read_usize(IAT_BEGIN_PAINT));
    let end: unsafe extern "system" fn(usize, *mut u8) -> u32 = core::mem::transmute(read_usize(IAT_END_PAINT));
    begin(read_usize(G_MAIN_WINDOW), paint.as_mut_ptr());
    main_window_redraw();
    end(read_usize(G_MAIN_WINDOW), paint.as_mut_ptr());
}

/// `WM_ACTIVATE` (active when the low word of wparam is not `WA_INACTIVE`) and `WM_ACTIVATEAPP` (wparam).
///
/// Deactivated in full screen with the display ready: the display mode goes back to the registry's
/// (`ChangeDisplaySettingsA(0, 0)`), a shared empty routine runs and the game window is minimised (`CloseWindow`).
/// Activated: while the game runs, the lost surfaces are restored and the whole screen is redrawn; then in full screen
/// with the display ready, the full screen mode (16 bits, the width and height the exe's code holds) is set again and
/// the window shown (`SW_SHOW`).
/// Defect of the original: only `dmSize`, `dmFields`, `dmBitsPerPel`, `dmPelsWidth` and `dmPelsHeight` of the
/// DEVMODE are set; `dmDriverExtra` and the rest are stack garbage (here zero).
unsafe fn main_window_activate(hwnd: usize, msg: u32, wparam: u32) {
    use main_window::*;
    let active = if msg == WM_ACTIVATE { (wparam as u16 != 0) as u32 } else { wparam };
    let change: unsafe extern "system" fn(*mut u8, u32) -> i32 = core::mem::transmute(read_usize(IAT_CHANGE_DISPLAY_SETTINGS));
    let full_screen_ready = || main_window_call(FN_GRAPHICS_IS_FULL_SCREEN) != 0 && read_u32(G_DISPLAY_NOT_READY) == 0;
    if active == 0 {
        if full_screen_ready() {
            change(core::ptr::null_mut(), 0);
            exe::shared_noop_ret0_5a8f60();
            let close: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(IAT_CLOSE_WINDOW));
            close(read_usize(G_MAIN_WINDOW));
        }
        return;
    }
    if read_u32(G_GAME_RUNNING) != 0 {
        main_window_call(FN_GRAPHICS_RESTORE_LOST_SURFACES);
        main_window_redraw();
    }
    if full_screen_ready() {
        let mut mode = [0u8; 0x9c]; // DEVMODEA
        mode[0x24..0x26].copy_from_slice(&0x9cu16.to_le_bytes()); // dmSize
        mode[0x28..0x2c].copy_from_slice(&0x1c_0000u32.to_le_bytes()); // dmFields: bits per pixel, width, height
        mode[0x68..0x6c].copy_from_slice(&16u32.to_le_bytes()); // dmBitsPerPel
        mode[0x6c..0x70].copy_from_slice(&read_u32(RESTORE_WIDTH_IMMEDIATE).to_le_bytes());
        mode[0x70..0x74].copy_from_slice(&read_u32(RESTORE_HEIGHT_IMMEDIATE).to_le_bytes());
        change(mode.as_mut_ptr(), 0);
        let show: unsafe extern "system" fn(usize, i32) -> i32 = core::mem::transmute(read_usize(IAT_SHOW_WINDOW));
        show(hwnd, 5);
    }
}

/// `WM_CLOSE`: the window is not closed; a box titled "Exit" says "Please use the 'Exit Game' option on the menubar"
/// (both texts through the language table).
unsafe fn main_window_close_box() {
    let fmt = news_text_format();
    let mut text = [0u8; 0xc8];
    let mut title = [0u8; 0x110];
    fmt(text.as_mut_ptr() as usize, 0x9f186c); // "Please use the 'Exit Game' option on the menubar"
    fmt(title.as_mut_ptr() as usize, 0x9f1864); // "Exit"
    exe::gui_show_message_box(title.as_ptr() as usize, text.as_ptr() as usize, 0);
}

/// The game window's procedure.
///
/// `WM_ACTIVATE` and `WM_ACTIVATEAPP`: `main_window_activate`. `WM_PAINT`: `main_window_paint`. `WM_CLOSE`: the "Exit
/// Game" box. `WM_KEYDOWN`: the key code is kept and Page Up, Page Down, End, Home, Up and Down press the current
/// screen's scrollbar key 3, 2, 4, 5, 1 and 0 (held 1); Left and Right nothing. `WM_CHAR`: the character is kept.
/// `WM_MOUSEMOVE`: while no scroll key holds the mouse, the position is kept and the cursor of the current mode is set
/// again. The left and right buttons: their state and the pending click (`store_window_input`). `WM_MOUSEWHEEL`: while
/// no scroll key holds the mouse, the wheel delta in notches (÷ 120, truncated) presses scrollbar key 1 (up) for a
/// positive count or 0 with the count's size; returns 1. The handled messages return 0; the others go to
/// `DefWindowProcA`.
/// Original: `main_window_proc` (0x68d630, main.cpp, stdcall, ret 0x10).
pub(crate) unsafe extern "stdcall" fn main_window_proc(hwnd: usize, msg: u32, wparam: u32, lparam: u32) -> i32 {
    use main_window::*;
    match msg {
        WM_ACTIVATE | WM_ACTIVATEAPP => main_window_activate(hwnd, msg, wparam),
        WM_PAINT => main_window_paint(),
        WM_CLOSE => main_window_close_box(),
        WM_KEYDOWN => {
            store_window_input(msg, wparam, lparam);
            let key = match wparam {
                0x21 => Some(3), // Page Up
                0x22 => Some(2), // Page Down
                0x23 => Some(4), // End
                0x24 => Some(5), // Home
                0x26 => Some(1), // Up
                0x28 => Some(0), // Down
                _ => None,
            };
            if let Some(k) = key {
                covered!("sx13_4 scroll key");
                screen_manager_press_scrollbar_key(G_SCREEN_MANAGER, k, 1);
            }
        }
        WM_MOUSEWHEEL => {
            if read_u32(DISPLAY_SCROLL_KEY_HELD) == 0 {
                let notches = ((wparam >> 16) as i16 as i32 / 120) as i16;
                let up = notches > 0;
                let count = if up { notches as i32 } else { -(notches as i32) };
                screen_manager_press_scrollbar_key(G_SCREEN_MANAGER, up as u32, count as u32); // sx13_4: up for a positive delta
            }
            return 1;
        }
        _ => {
            if !store_window_input(msg, wparam, lparam) {
                let def: unsafe extern "system" fn(usize, u32, u32, u32) -> i32 = core::mem::transmute(read_usize(IAT_DEF_WINDOW_PROC));
                return def(hwnd, msg, wparam, lparam);
            }
            if msg == WM_MOUSEMOVE && read_u32(DISPLAY_SCROLL_KEY_HELD) == 0 {
                main_window_call(FN_REAPPLY_CURSOR_MODE);
            }
        }
    }
    0
}

/// Starts the game again: the game is cleaned up, the single instance mutex closed, the program run again with its
/// own command line (`WinExec`, `SW_SHOWDEFAULT`) and this copy ends (`exit(0)`).
/// Original: `restart_game` (0x68dbc0, main.cpp, cdecl).
pub(crate) unsafe extern "C" fn restart_game() {
    use main_window::*;
    exe::game_fatal_cleanup();
    let close: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(IAT_CLOSE_HANDLE));
    close(read_usize(G_SINGLE_INSTANCE_MUTEX));
    let command_line: unsafe extern "system" fn() -> usize = core::mem::transmute(read_usize(IAT_GET_COMMAND_LINE));
    let win_exec: unsafe extern "system" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(IAT_WIN_EXEC));
    win_exec(command_line(), 10);
    exe::crt_exit(0);
}

/// Comparer for `main_window_proc`: the keyboard and mouse messages side by side over the input globals the copy
/// writes (`store_window_input`; for `WM_KEYDOWN` only the key code, the scrollbar key moves the mouse globals);
/// painting, activation, the close box, the wheel and `DefWindowProcA` only the original (they draw, change the display
/// or open a box).
pub(crate) unsafe extern "stdcall" fn compare_main_window_proc(hwnd: usize, msg: u32, wparam: u32, lparam: u32) -> i32 {
    use main_window::*;
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32, u32, u32) -> i32>(%%I0%%);
    let regions: &[(usize, usize)] = match msg {
        WM_KEYDOWN => &[(DISPLAY_KEY_CODE, 4)],
        WM_CHAR | WM_MOUSEMOVE | WM_LBUTTONDOWN | WM_LBUTTONUP | WM_RBUTTONDOWN | WM_RBUTTONUP => &[(G_LEFT_CLICK_PENDING, 4),
            (DISPLAY_MOUSE_Y, 4), (G_RIGHT_CLICK_PENDING, 4), (DISPLAY_MOUSE_X, 4), (G_LAST_CHAR_CODE, 1),
            (G_MOUSE_LEFT_BUTTON_DOWN, 4), (DISPLAY_RIGHT_BUTTON, 4)],
        _ => return o(hwnd, msg, wparam, lparam),
    };
    let (r, _, equal, mo, mr) = crate::comparison::side_by_side_light(regions, || o(hwnd, msg, wparam, lparam),
        || store_window_input(msg, wparam, lparam) as i32);
    SCOREBOARD.record(equal && r == 0, || format!("main_window_proc(msg {:#x}, wparam {:#x}, lparam {:#x}) returned {}: differences in {:?}",
        msg, wparam, lparam, r, first_differences(&mo, &mr)));
    r
}

/// Comparer for `restart_game`: only the original (the process ends).
pub(crate) unsafe extern "C" fn compare_restart_game() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}
