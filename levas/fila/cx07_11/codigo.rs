// ---------- cx07_11: the graphics base: cursors, the display flags, rectangle clipping and overlap, an image's release, the shutdown (goldcup.cpp by the map; the graphics library) ----------

/// Nonzero while there is no usable screen (set at the start of the display init, cleared at its end).
const DISPLAY_OFF: usize = 0xaebe0c;
/// The DirectDraw object (IDirectDraw).
const DIRECTDRAW: usize = 0xae3194;
/// The primary surface (IDirectDrawSurface).
const PRIMARY_SURFACE: usize = 0xae31a4;
/// The back buffer surface (the primary itself when there is no flipping chain).
const BACK_BUFFER: usize = 0xaebde4;
/// The clipper attached to the primary surface (windowed mode).
const CLIPPER: usize = 0xaebe2c;
/// Nonzero when the display runs full screen.
const FULL_SCREEN: usize = 0xae319c;
/// The screen width and height in pixels.
const SCREEN_WIDTH: usize = 0xaebe08;
const SCREEN_HEIGHT: usize = 0xaebdd0;
/// The main window handle.
const MAIN_WINDOW: usize = 0xb67a2c;
/// Nonzero when the screen flip hides and redraws the mouse cursor around each update.
const SOFTWARE_CURSOR: usize = 0xaebe14;
/// The current cursor mode (0 normal, 1 busy, 2 pointing [inference]) and the three cursors.
const CURSOR_MODE: usize = 0x9cd804;
const CURSOR_NORMAL: usize = 0xaebe24;
const CURSOR_BUSY: usize = 0xaebe1c;
const CURSOR_POINTING: usize = 0xaebe20;
/// The system cursor in use before the game's cursors were set (put back at shutdown).
const SAVED_SYSTEM_CURSOR: usize = 0xaebe18;
/// The bitmap font table: font k at `FONT_TABLE + k × FONT_SLOT_SIZE` (the seven loaded fonts in slots 1 to 7); the
/// glyph of character c at +`FONT_GLYPHS` + c × `FONT_GLYPH_SIZE` (+0 its width, +8 its bitmap pointer).
const FONT_TABLE: usize = 0xae1dac;
const FONT_SLOT_SIZE: usize = 0x1404;
const FONT_GLYPHS: usize = 0xc;
const FONT_GLYPH_SIZE: usize = 0x14;
const FONT_GLYPH_BITMAP: usize = 8;

/// The user32 functions, called through the exe's import table (so the DLL's own IAT hooks still see them).
unsafe fn user32_show_cursor(show: i32) -> i32 {
    let f: unsafe extern "system" fn(i32) -> i32 = core::mem::transmute(read_usize(0x9671c0)); // ShowCursor
    f(show)
}
unsafe fn user32_set_cursor(cursor: usize) -> usize {
    let f: unsafe extern "system" fn(usize) -> usize = core::mem::transmute(read_usize(0x9671bc)); // SetCursor
    f(cursor)
}
unsafe fn user32_get_cursor() -> usize {
    let f: unsafe extern "system" fn() -> usize = core::mem::transmute(read_usize(0x9671c8)); // GetCursor
    f()
}

/// Calls the COM method at `slot` of `object` with no other argument (stdcall).
unsafe fn com_call0(object: usize, slot: usize) -> i32 {
    let f: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(read_usize(object) + slot));
    f(object)
}

/// Builds the static array of four T2K font objects at 0xaebdf4 (the C runtime's vector constructor with
/// FONT::FONT 0x5b0ab0 and FONT::~FONT 0x5b0b40, 4 bytes each) and registers its destructor
/// (`t2k_fonts_static_destroy`) with atexit. Runs once, before the game's main.
/// Original: `t2k_fonts_static_init` (0x5e25d0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn t2k_fonts_static_init() -> i32 {
    exe::crt_vector_constructor_iterator(0xaebdf4, 4, 4, 0x5b0ab0, 0x5b0b40);
    exe::crt_atexit(0x5e2600)
}

/// Destroys the static array of four T2K font objects at 0xaebdf4 (the C runtime's vector destructor with
/// FONT::~FONT 0x5b0b40). Runs at program exit.
/// Original: `t2k_fonts_static_destroy` (0x5e2600, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn t2k_fonts_static_destroy() {
    exe::crt_vector_destructor_iterator(0xaebdf4, 4, 4, 0x5b0b40);
}

/// Restores the primary surface and then the back buffer when DirectDraw reports them lost (IsLost, slot 0x60; Restore,
/// slot 0x6c), unless the display is off.
/// Original: `graphics_restore_lost_surfaces` (0x5e2e50, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_restore_lost_surfaces() {
    if read_u32(DISPLAY_OFF) != 0 {
        return;
    }
    for surface in [PRIMARY_SURFACE, BACK_BUFFER] {
        let s = read_usize(surface);
        if s != 0 && com_call0(s, 0x60) != 0 {
            covered!("cx07_11 surface restored");
            com_call0(read_usize(surface), 0x6c);
        }
    }
}

/// Returns the full screen flag that the display init stored.
/// Original: `graphics_is_full_screen` (0x5e2ea0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_is_full_screen() -> u32 {
    read_u32(FULL_SCREEN)
}

/// Orders the rectangle (x0, y0)-(x1, y1) into `out` (left, top, right, bottom) and clamps it to the screen (0 to the
/// width or height − 1). Returns 1 when it lay fully inside, 0 when it was clamped, when the display is off or without
/// `out` (then nothing is written).
/// Original: `clip_rect_to_surface` (0x5e3680, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn clip_rect_to_surface(x0: i32, y0: i32, x1: i32, y1: i32, out: *mut [i32; 4]) -> u32 {
    if read_u32(DISPLAY_OFF) != 0 || out.is_null() {
        return 0;
    }
    let o = &mut *out;
    *o = [x0.min(x1), y0.min(y1), x0.max(x1), y0.max(y1)];
    let mut inside = 1;
    if o[0] < 0 {
        o[0] = 0;
        inside = 0;
    }
    if o[1] < 0 {
        o[1] = 0;
        inside = 0;
    }
    let right = read_i32(SCREEN_WIDTH) - 1;
    if o[2] > right {
        o[2] = right;
        inside = 0;
    }
    let bottom = read_i32(SCREEN_HEIGHT) - 1;
    if o[3] > bottom {
        o[3] = bottom;
        inside = 0;
    }
    inside
}

/// Returns the area of the overlap of the rectangles `a` and `b` (left, top, right, bottom, passed by value), or 0 when
/// they do not overlap.
/// Original: `rect_overlap_area` (0x5e7f00, goldcup.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn rect_overlap_area(a0: i32, a1: i32, a2: i32, a3: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 {
    let w = a2.min(b2).wrapping_sub(a0.max(b0)).max(0);
    let h = a3.min(b3).wrapping_sub(a1.max(b1)).max(0);
    w.wrapping_mul(h)
}

/// Frees an image: its pixel buffer (+0xc, cleared) and then the image header itself; nothing without an image.
/// Original: `gui_free_image` (0x5e42b0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_free_image(image: usize) {
    if image == 0 {
        return;
    }
    let pixels = read_usize(image + 0xc); // the image's pixel buffer
    if pixels != 0 {
        exe::crt_free(pixels);
        write_usize(image + 0xc, 0);
    }
    exe::crt_free(image);
}

/// Returns the current mouse cursor mode.
/// Original: `get_cursor_mode` (0x5e8160, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn get_cursor_mode() -> u32 {
    read_u32(CURSOR_MODE)
}

/// The cursor of mode `mode` (0 normal, 1 busy, 2 pointing), or none.
unsafe fn cursor_of_mode(mode: u32) -> Option<usize> {
    match mode {
        0 => Some(read_usize(CURSOR_NORMAL)),
        1 => Some(read_usize(CURSOR_BUSY)),
        2 => Some(read_usize(CURSOR_POINTING)),
        _ => None,
    }
}

/// Sets the mouse cursor for mode 0, 1 or 2 when the mode changes: the cursor hidden (ShowCursor 0), the mode stored,
/// the mode's cursor set (another mode sets none) and the cursor shown again.
/// Original: `set_cursor_mode` (0x5e8170, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn set_cursor_mode(mode: u32) {
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

/// Sets the cursor of the current mode again (hidden and shown around it) when the system's cursor (GetCursor) is a
/// different one; another mode does nothing.
/// Original: `reapply_cursor_mode` (0x5e81e0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn reapply_cursor_mode() {
    let Some(cursor) = cursor_of_mode(read_u32(CURSOR_MODE)) else { return };
    if user32_get_cursor() == cursor {
        return;
    }
    user32_show_cursor(0);
    user32_set_cursor(cursor);
    user32_show_cursor(1);
}

/// Stores the flag that makes the screen flip hide and redraw the cursor around each update.
/// Original: `set_software_cursor_flag` (0x5e8280, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn set_software_cursor_flag(on: u32) {
    write_u32(SOFTWARE_CURSOR, on);
}

/// Shuts the display down: the saved system cursor set back (when there is one) and forgotten; the three game cursors
/// destroyed (DestroyCursor) and cleared; the clipper released (slot 8) and cleared; DirectDraw's display mode restored
/// (RestoreDisplayMode, slot 0x4c), DirectDraw released and cleared; the main window hidden (ShowWindow 0), destroyed
/// and cleared; the glyph bitmaps of characters 0x20 to 0xff of the seven bitmap fonts (slots 1 to 7) freed (the
/// pointers are left as they are).
/// Original: `graphics_shutdown_display` (0x5e2cc0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_shutdown_display() {
    let saved = read_usize(SAVED_SYSTEM_CURSOR);
    if saved != 0 {
        user32_set_cursor(saved);
    }
    write_usize(SAVED_SYSTEM_CURSOR, 0);
    let destroy_cursor: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(0x9671d0)); // DestroyCursor
    for cursor in [CURSOR_BUSY, CURSOR_NORMAL, CURSOR_POINTING] {
        let c = read_usize(cursor);
        if c != 0 {
            destroy_cursor(c);
        }
        write_usize(cursor, 0);
    }
    let clipper = read_usize(CLIPPER);
    if clipper != 0 {
        com_call0(clipper, 8);
        write_usize(CLIPPER, 0);
    }
    let dd = read_usize(DIRECTDRAW);
    if dd != 0 {
        com_call0(dd, 0x4c);
        com_call0(read_usize(DIRECTDRAW), 8);
        write_usize(DIRECTDRAW, 0);
    }
    let show_window: unsafe extern "system" fn(usize, i32) -> i32 = core::mem::transmute(read_usize(0x967204)); // ShowWindow
    show_window(read_usize(MAIN_WINDOW), 0);
    let window = read_usize(MAIN_WINDOW);
    if window != 0 {
        let destroy_window: unsafe extern "system" fn(usize) -> i32 = core::mem::transmute(read_usize(0x9671cc)); // DestroyWindow
        destroy_window(window);
        write_usize(MAIN_WINDOW, 0);
    }
    covered!("cx07_11 display shut down");
    for font in 1..=7 {
        for c in 0x20..=0xffusize {
            let bitmap = read_usize(FONT_TABLE + font * FONT_SLOT_SIZE + FONT_GLYPHS + c * FONT_GLYPH_SIZE + FONT_GLYPH_BITMAP);
            if bitmap != 0 {
                exe::crt_free(bitmap);
            }
        }
    }
}

/// Comparers. What draws, sets cursors, releases DirectDraw or frees memory runs only the original; the pure helpers
/// (`clip_rect_to_surface`, `rect_overlap_area`, the getters) side by side.
pub(crate) unsafe extern "C" fn compare_t2k_fonts_static_init() -> i32 {
    original_fn::<unsafe extern "C" fn() -> i32>(%%I0%%)()
}

pub(crate) unsafe extern "C" fn compare_t2k_fonts_static_destroy() {
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

pub(crate) unsafe extern "C" fn compare_graphics_restore_lost_surfaces() {
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}

pub(crate) unsafe extern "C" fn compare_graphics_is_full_screen() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I3%%)();
    let c = graphics_is_full_screen();
    SCOREBOARD.record(r == c, || format!("graphics_is_full_screen: original {} rust {}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_clip_rect_to_surface(x0: i32, y0: i32, x1: i32, y1: i32, out: *mut [i32; 4]) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, *mut [i32; 4]) -> u32>(%%I4%%)(x0, y0, x1, y1, out);
    let original = if out.is_null() { [0; 4] } else { *out };
    let mut copy = original;
    let c = clip_rect_to_surface(x0, y0, x1, y1, if out.is_null() { core::ptr::null_mut() } else { &mut copy });
    SCOREBOARD.record(r == c && copy == original, || format!("clip_rect_to_surface({}, {}, {}, {}): original {} {:?} rust {} {:?}", x0, y0, x1, y1, r, original, c, copy));
    r
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_rect_overlap_area(a0: i32, a1: i32, a2: i32, a3: i32, b0: i32, b1: i32, b2: i32, b3: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32, i32) -> i32>(%%I5%%)(a0, a1, a2, a3, b0, b1, b2, b3);
    let c = rect_overlap_area(a0, a1, a2, a3, b0, b1, b2, b3);
    SCOREBOARD.record(r == c, || format!("rect_overlap_area({:?}, {:?}): original {} rust {}", [a0, a1, a2, a3], [b0, b1, b2, b3], r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_gui_free_image(image: usize) {
    original_fn::<unsafe extern "C" fn(usize)>(%%I6%%)(image)
}

pub(crate) unsafe extern "C" fn compare_get_cursor_mode() -> u32 {
    let r = original_fn::<unsafe extern "C" fn() -> u32>(%%I7%%)();
    let c = get_cursor_mode();
    SCOREBOARD.record(r == c, || format!("get_cursor_mode: original {} rust {}", r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_set_cursor_mode(mode: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I8%%)(mode)
}

pub(crate) unsafe extern "C" fn compare_reapply_cursor_mode() {
    original_fn::<unsafe extern "C" fn()>(%%I9%%)()
}

pub(crate) unsafe extern "C" fn compare_set_software_cursor_flag(on: u32) {
    original_fn::<unsafe extern "C" fn(u32)>(%%I10%%)(on);
    SCOREBOARD.record(read_u32(SOFTWARE_CURSOR) == on, || format!("set_software_cursor_flag({}): the flag reads {}", on, read_u32(SOFTWARE_CURSOR)));
}

pub(crate) unsafe extern "C" fn compare_graphics_shutdown_display() {
    original_fn::<unsafe extern "C" fn()>(%%I11%%)()
}
