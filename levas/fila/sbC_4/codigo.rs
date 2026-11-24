// ---------- sbC_4: the graphics base: the display init (goldcup.cpp by the map; the graphics library) ----------

/// Nonzero until the first display init has created the window and DirectDraw.
const DISPLAY_FIRST_INIT: usize = 0x9cd808;
/// The application instance handle (WinMain's).
const APP_INSTANCE: usize = 0xb67a18;
/// The window frame height (GetSystemMetrics SM_CYDLGFRAME) and the desktop's size (its window rectangle).
const WINDOW_FRAME_Y: usize = 0xae318c;
const DESKTOP_WIDTH: usize = 0xaebdd8;
const DESKTOP_HEIGHT: usize = 0xae3198;
/// The flags of the last display init (bit 1: a separate back buffer).
const DISPLAY_INIT_FLAGS: usize = 0xae314c;
/// The game window's class name ("main"), its procedure (main_window_proc) and the "Error" title.
const MAIN_WINDOW_CLASS: usize = 0x9cd9b4;
const MAIN_WINDOW_PROC: usize = 0x68d630;
const ERROR_TITLE: usize = 0x9870e0;
/// "Unable to initialise the graphics in full screen mode - ..." and "... in windowed mode - ...", and "Unable to load
/// the fonts".
const FULL_SCREEN_FAILED: usize = 0x9cd8f0;
const WINDOWED_FAILED: usize = 0x9cd828;
const FONTS_FAILED: usize = 0x9cd80c;
/// The window styles: WS_POPUP | WS_SYSMENU (full screen), WS_SYSMENU | WS_MINIMIZEBOX (a new window) and WS_CAPTION |
/// WS_SYSMENU | WS_MINIMIZEBOX (a window set again).
const STYLE_FULL_SCREEN: u32 = 0x80080000;
const STYLE_NEW_WINDOW: u32 = 0xa0000;
const STYLE_WINDOW: u32 = 0xca0000;
/// The DirectDraw cooperative levels: DDSCL_NORMAL and DDSCL_EXCLUSIVE | DDSCL_FULLSCREEN | DDSCL_ALLOWREBOOT.
const DDSCL_NORMAL: u32 = 8;
const DDSCL_FULL_SCREEN: u32 = 0x13;

/// A user32 function through the exe's import table (so the DLL's own IAT hooks still see it).
unsafe fn user32_import(slot: usize) -> usize {
    read_usize(slot)
}

/// GetSystemMetrics (through the import table).
unsafe fn system_metric(index: i32) -> i32 {
    let f: unsafe extern "system" fn(i32) -> i32 = core::mem::transmute(user32_import(0x9671b0));
    f(index)
}

/// ShowWindow (through the import table) on the game window.
unsafe fn show_main_window(command: i32) {
    let f: unsafe extern "system" fn(usize, i32) -> i32 = core::mem::transmute(user32_import(0x967204));
    f(read_usize(MAIN_WINDOW), command);
}

/// The method at `slot` of the COM object `object`.
unsafe fn directdraw_method(object: usize, slot: usize) -> usize {
    read_usize(read_usize(object) + slot)
}

/// Shows the init's error box ("Error" and the full screen or the windowed message) and returns 0.
unsafe fn display_init_failed(full_screen: u32) -> u32 {
    exe::gui_show_message_box(ERROR_TITLE, if full_screen != 0 { FULL_SCREEN_FAILED } else { WINDOWED_FAILED }, 0);
    0
}

/// The first init: the system cursor kept, the game's cursors loaded (resources 0x69 busy, 0x65 normal, 0x67 pointing;
/// failing, the system's wait and arrow cursors), the busy cursor shown, the window class "main" registered
/// (main_window_proc, icon 0x68), the frame metrics and the desktop size kept, the window created (full screen: a popup
/// the size of the screen; windowed: the frame around `width` × `height`, centred on the desktop), updated and shown, and
/// DirectDraw created (DirectDrawCreate through the exe's thunk 0x944c1c). False when the window or DirectDraw fails.
unsafe fn create_main_window(width: i32, height: i32, full_screen: u32, title: usize) -> bool {
    write_u32(DISPLAY_FIRST_INIT, 0);
    if read_usize(SAVED_SYSTEM_CURSOR) == 0 {
        write_usize(SAVED_SYSTEM_CURSOR, user32_get_cursor());
    }
    let instance = read_usize(APP_INSTANCE);
    let load_cursor: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(user32_import(0x9671c4));
    for (cursor, resource) in [(CURSOR_BUSY, 0x69), (CURSOR_NORMAL, 0x65), (CURSOR_POINTING, 0x67)] {
        if read_usize(cursor) == 0 {
            write_usize(cursor, load_cursor(instance, resource));
        }
    }
    for (cursor, system) in [(CURSOR_BUSY, 0x7f02), (CURSOR_POINTING, 0x7f00), (CURSOR_NORMAL, 0x7f00)] {
        if read_usize(cursor) == 0 {
            write_usize(cursor, load_cursor(0, system)); // IDC_WAIT, IDC_ARROW
        }
    }
    if read_u32(CURSOR_MODE) != 1 {
        write_u32(CURSOR_MODE, 1);
        user32_show_cursor(0);
        user32_set_cursor(read_usize(CURSOR_BUSY));
        user32_show_cursor(1);
    }
    let load_icon: unsafe extern "system" fn(usize, usize) -> usize = core::mem::transmute(user32_import(0x9671b8));
    // WNDCLASSA: style, the procedure, the two extra sizes, the instance, the icon, the cursor, the brush, the menu, the name
    let class: [usize; 10] = [0, MAIN_WINDOW_PROC, 0, 0, instance, load_icon(instance, 0x68), 0, 0, 0, MAIN_WINDOW_CLASS];
    let register_class: unsafe extern "system" fn(*const [usize; 10]) -> u16 = core::mem::transmute(user32_import(0x9671b4));
    register_class(&class);
    write_i32(WINDOW_FRAME_X, system_metric(7));
    write_i32(WINDOW_FRAME_Y, system_metric(8));
    write_i32(WINDOW_CAPTION_Y, system_metric(4) + read_i32(WINDOW_FRAME_Y));
    let get_desktop_window: unsafe extern "system" fn() -> usize = core::mem::transmute(user32_import(0x9671ac));
    let get_window_rect: unsafe extern "system" fn(usize, *mut [i32; 4]) -> i32 = core::mem::transmute(user32_import(0x9671a8));
    let mut desktop = [0i32; 4];
    get_window_rect(get_desktop_window(), &mut desktop);
    write_i32(DESKTOP_WIDTH, desktop[2]);
    write_i32(DESKTOP_HEIGHT, desktop[3]);
    let create_window: unsafe extern "system" fn(u32, usize, usize, u32, i32, i32, i32, i32, usize, usize, usize, usize) -> usize =
        core::mem::transmute(user32_import(0x9671a4));
    let window = if full_screen != 0 {
        let h = system_metric(1);
        let w = system_metric(0);
        create_window(0, MAIN_WINDOW_CLASS, title, STYLE_FULL_SCREEN, 0, 0, w, h, 0, 0, instance, 0)
    } else {
        let (frame_x, caption, frame_y) = (read_i32(WINDOW_FRAME_X), read_i32(WINDOW_CAPTION_Y), read_i32(WINDOW_FRAME_Y));
        let (w, h) = (2 * frame_x + width, caption + frame_y + height);
        let (x, y) = ((desktop[2] - 2 * frame_x - width) / 2, (desktop[3] - caption - frame_y - height) / 2);
        create_window(0, MAIN_WINDOW_CLASS, title, STYLE_NEW_WINDOW, x, y, w, h, 0, 0, instance, 0)
    };
    write_usize(MAIN_WINDOW, window);
    if window == 0 {
        return false;
    }
    let update_window: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(user32_import(0x9671a0));
    update_window(window);
    show_main_window(5);
    let direct_draw_create: unsafe extern "system" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x944c1cusize);
    direct_draw_create(0, DIRECTDRAW, 0) == 0
}

/// A later init: the window hidden, DirectDraw back to the normal cooperative level, the clipper, the back buffer (when
/// separate) and the primary surface released, the window style and place set again (full screen: a popup the size of
/// the screen; windowed: the frame around `width` × `height`) and the window shown.
/// Defect of the original: the windowed place is not the first init's centring: the frame height is added instead of
/// subtracted and twice the frame width added instead of subtracted.
unsafe fn reset_main_window(width: i32, height: i32, full_screen: u32) {
    show_main_window(0);
    let dd = read_usize(DIRECTDRAW);
    let cooperative: unsafe extern "system" fn(usize, usize, u32) -> i32 = core::mem::transmute(directdraw_method(dd, 0x50));
    cooperative(dd, read_usize(MAIN_WINDOW), DDSCL_NORMAL);
    let clipper = read_usize(CLIPPER);
    if clipper != 0 {
        com_call0(clipper, 8);
        write_usize(CLIPPER, 0);
    }
    let (primary, back) = (read_usize(PRIMARY_SURFACE), read_usize(BACK_BUFFER));
    if primary != back && back != 0 {
        com_call0(back, 8);
    }
    if read_usize(PRIMARY_SURFACE) != 0 {
        com_call0(read_usize(PRIMARY_SURFACE), 8);
    }
    write_usize(PRIMARY_SURFACE, 0);
    write_usize(BACK_BUFFER, 0);
    let set_window_long: unsafe extern "system" fn(usize, i32, u32) -> i32 = core::mem::transmute(user32_import(0x9671ec));
    let move_window: unsafe extern "system" fn(usize, i32, i32, i32, i32, i32) -> i32 = core::mem::transmute(user32_import(0x9671e8));
    let window = read_usize(MAIN_WINDOW);
    if full_screen != 0 {
        set_window_long(window, -16, STYLE_FULL_SCREEN);
        let h = system_metric(1);
        let w = system_metric(0);
        move_window(read_usize(MAIN_WINDOW), 0, 0, w, h, 0);
    } else {
        covered!("sbC_4 window set again");
        set_window_long(window, -16, STYLE_WINDOW);
        let (frame_x, caption, frame_y) = (read_i32(WINDOW_FRAME_X), read_i32(WINDOW_CAPTION_Y), read_i32(WINDOW_FRAME_Y));
        let (w, h) = (2 * frame_x + width, caption + frame_y + height);
        let x = (2 * frame_x - width + read_i32(DESKTOP_WIDTH)) / 2;
        let y = (read_i32(DESKTOP_HEIGHT) - caption - height + frame_y) / 2;
        move_window(read_usize(MAIN_WINDOW), x, y, w, h, 0);
    }
    show_main_window(5);
}

/// The DirectDraw part of the init: the cooperative level and the 16-bit display mode (full screen: exclusive, at
/// `width` × `height`; windowed: normal, at the desktop's size), the primary surface, with flag 2 a separate back buffer
/// in system memory (otherwise the primary itself), the screen size and flags kept, the primary's pixel format copied
/// to `SCREEN_PIXEL_FORMAT` and, windowed, a clipper on the window (once). False at the first failure.
unsafe fn create_display_surfaces(width: i32, height: i32, flags: u32, full_screen: u32) -> bool {
    let window = read_usize(MAIN_WINDOW);
    let dd = read_usize(DIRECTDRAW);
    let cooperative: unsafe extern "system" fn(usize, usize, u32) -> i32 = core::mem::transmute(directdraw_method(dd, 0x50));
    let display_mode = |w: i32, h: i32| {
        let dd = read_usize(DIRECTDRAW);
        let set: unsafe extern "system" fn(usize, i32, i32, u32) -> i32 = core::mem::transmute(directdraw_method(dd, 0x54));
        set(dd, w, h, 16)
    };
    let mode_set = if full_screen != 0 {
        cooperative(dd, window, DDSCL_FULL_SCREEN) == 0 && display_mode(width, height) == 0
    } else {
        cooperative(dd, window, DDSCL_NORMAL) == 0 && display_mode(read_i32(DESKTOP_WIDTH), read_i32(DESKTOP_HEIGHT)) == 0
    };
    if !mode_set {
        return false;
    }
    // DDSURFACEDESC: +0 the size, +4 the flags, +8 the height, +0xc the width, +0x48 the pixel format, +0x68 the caps
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    let d = desc.as_mut_ptr() as usize;
    let create_surface = |target: usize| {
        let dd = read_usize(DIRECTDRAW);
        let create: unsafe extern "system" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(directdraw_method(dd, 0x18));
        create(dd, d, target, 0)
    };
    write_u32(d, SURFACE_DESC_SIZE as u32);
    write_u32(d + 4, 1); // DDSD_CAPS
    write_u32(d + 0x68, 0x200); // DDSCAPS_PRIMARYSURFACE
    if create_surface(PRIMARY_SURFACE) != 0 {
        return false;
    }
    if flags & 2 != 0 {
        write_u32(d + 4, 7); // DDSD_CAPS | DDSD_HEIGHT | DDSD_WIDTH
        write_u32(d + 0x68, 0x840); // DDSCAPS_OFFSCREENPLAIN | DDSCAPS_SYSTEMMEMORY
        write_i32(d + 8, height);
        write_i32(d + 0xc, width);
        if create_surface(BACK_BUFFER) != 0 {
            return false;
        }
    } else {
        write_usize(BACK_BUFFER, read_usize(PRIMARY_SURFACE));
    }
    write_i32(SCREEN_WIDTH, width);
    write_u32(DISPLAY_INIT_FLAGS, flags);
    write_i32(SCREEN_HEIGHT, height);
    let primary = read_usize(PRIMARY_SURFACE);
    let get_desc: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(directdraw_method(primary, 0x58));
    if get_desc(primary, d) != 0 {
        return false;
    }
    core::ptr::copy_nonoverlapping((d + 0x48) as *const u8, SCREEN_PIXEL_FORMAT as *mut u8, 0x20);
    if full_screen == 0 && read_usize(CLIPPER) == 0 {
        covered!("sbC_4 clipper");
        let create_clipper: unsafe extern "system" fn(usize, u32, usize, usize) -> i32 = core::mem::transmute(directdraw_method(dd, 0x10));
        if create_clipper(read_usize(DIRECTDRAW), 0, CLIPPER, 0) != 0 {
            return false;
        }
        let clipper = read_usize(CLIPPER);
        let set_window: unsafe extern "system" fn(usize, u32, usize) -> i32 = core::mem::transmute(directdraw_method(clipper, 0x20));
        if set_window(clipper, 0, read_usize(MAIN_WINDOW)) != 0 {
            return false;
        }
        let primary = read_usize(PRIMARY_SURFACE);
        let set_clipper: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(directdraw_method(primary, 0x70));
        if set_clipper(primary, read_usize(CLIPPER)) != 0 {
            return false;
        }
    }
    true
}

/// Creates (the first time) or sets again the game window and the DirectDraw display, then loads the interface colours
/// and the fonts and clears the screen to black: 1 when it worked, 0 after an error box.
///
/// The display is off (`DISPLAY_OFF` = 1) meanwhile. The first time `create_main_window`, later `reset_main_window`;
/// then `create_display_surfaces`. Any failure shows "Error" with the full screen or the windowed message (MessageBoxA,
/// the display being off) and returns 0, leaving the display off. Then the display is on, `gui_init_palette_colours`
/// and `gui_load_fonts`(`font_mode`) run (failing: "Unable to load the fonts", 0), the screen is filled with black
/// (`graphics_draw_rect`, style 4) and flipped, and the full screen flag is kept.
/// Original: `graphics_init_display` (0x5e2620, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_init_display(width: i32, height: i32, flags: u32, font_mode: i32, full_screen: u32, title: usize) -> u32 {
    write_u32(DISPLAY_OFF, 1);
    if read_u32(DISPLAY_FIRST_INIT) != 0 {
        if !create_main_window(width, height, full_screen, title) {
            return display_init_failed(full_screen);
        }
    } else {
        reset_main_window(width, height, full_screen);
    }
    if !create_display_surfaces(width, height, flags, full_screen) {
        return display_init_failed(full_screen);
    }
    write_u32(DISPLAY_OFF, 0);
    let init_colours: unsafe extern "C" fn() = core::mem::transmute(0x5e4560usize); // gui_init_palette_colours (changed in the 1080 exe)
    init_colours();
    let load_fonts: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x5e4a60usize); // gui_load_fonts
    if load_fonts(font_mode) == 0 {
        exe::gui_show_message_box(ERROR_TITLE, FONTS_FAILED, 0);
        return 0;
    }
    exe::graphics_draw_rect(0, 0, width - 1, height - 1, 4, read_u16(G_GUI_COLOUR_BLACK) as u32);
    exe::graphics_flip_rect(0, 0, (width - 1) as u32, (height - 1) as u32);
    write_u32(FULL_SCREEN, full_screen);
    1
}

/// Comparer: only the original (it creates the window and the DirectDraw objects).
pub(crate) unsafe extern "C" fn compare_graphics_init_display(width: i32, height: i32, flags: u32, font_mode: i32, full_screen: u32, title: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(i32, i32, u32, i32, u32, usize) -> u32>(%%I0%%)(width, height, flags, font_mode, full_screen, title)
}
