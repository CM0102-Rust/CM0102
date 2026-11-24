// ---------- cx07_12: the graphics base: the screen flip, the colour conversions, the font metrics, the text escapes, a glyph and the screen images (goldcup.cpp by the map; the graphics library) ----------

/// The pixel format of the screen (a copy of the primary surface's DDPIXELFORMAT, 0x20 bytes): +0x10 the red mask,
/// +0x14 the green mask (0x7e0 for 565), +0x18 the blue mask.
const SCREEN_PIXEL_FORMAT: usize = 0xae3158;
/// Nonzero when every screen update is turned to grey first (the paused or background look).
const GREYSCALE_UPDATES: usize = 0xaebe28;
/// Set to 1 by each screen flip.
const SCREEN_UPDATED: usize = 0xaebe10;
/// The window frame width and the caption plus frame height (GetSystemMetrics at the display init).
const WINDOW_FRAME_X: usize = 0xae3180;
const WINDOW_CAPTION_Y: usize = 0xae3144;
/// The glyph counter of a T2K line draw, the glyph index of the caret and the x of the caret's glyph.
const CARET_GLYPH_COUNTER: usize = 0xae3178;
const CARET_GLYPH_INDEX: usize = 0xaebde0;
const CARET_X: usize = 0xae3188;
/// The DirectDraw surface description's size and the offsets read from it (DDSURFACEDESC).
const SURFACE_DESC_SIZE: usize = 0x6c;
const SURFACE_DESC_PITCH: usize = 0x10;
const SURFACE_DESC_SURFACE: usize = 0x24;

/// The pixel format given, or the screen's.
fn pixel_format_or_screen(format: usize) -> usize {
    if format != 0 { format } else { SCREEN_PIXEL_FORMAT }
}

/// Locks the rectangle `rect` of the back buffer (IDirectDrawSurface::Lock, slot 0x64, with `flags`) into the
/// description `desc`; true when it worked.
unsafe fn lock_back_buffer(rect: &[i32; 4], desc: &mut [u8; SURFACE_DESC_SIZE], flags: u32) -> bool {
    let back = read_usize(BACK_BUFFER);
    write_u32(desc.as_mut_ptr() as usize, SURFACE_DESC_SIZE as u32);
    let lock: unsafe extern "system" fn(usize, *const [i32; 4], *mut u8, u32, usize) -> i32 = core::mem::transmute(read_usize(read_usize(back) + 0x64));
    lock(back, rect, desc.as_mut_ptr(), flags, 0) == 0
}

/// Unlocks the back buffer (IDirectDrawSurface::Unlock, slot 0x80, with no rectangle).
unsafe fn unlock_back_buffer() {
    let back = read_usize(BACK_BUFFER);
    let unlock: unsafe extern "system" fn(usize, usize) -> i32 = core::mem::transmute(read_usize(read_usize(back) + 0x80));
    unlock(back, 0);
}

/// The overlap of the mouse cursor's box (from 5 pixels before the mouse to 15 after) with the rectangle (x0, y0, x1,
/// y1) is not empty (`rect_overlap_area`).
unsafe fn cursor_over(x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
    let (mx, my) = (read_i32(DISPLAY_MOUSE_X), read_i32(DISPLAY_MOUSE_Y));
    rect_overlap_area(mx - 5, my - 5, mx + 0xf, my + 0xf, x0, y0, x1, y1) != 0
}

/// Copies the rectangle (x0, y0)-(x1, y1) of the back buffer to the screen and marks the screen as updated.
///
/// Nothing when the display is off, without both surfaces or when they are the same, or when the rectangle does not lie
/// fully on the screen (`clip_rect_to_surface`). With `GREYSCALE_UPDATES` the box is turned grey first
/// (gui_greyscale_rect 0x5e7f60, cdecl). With the software cursor (`SOFTWARE_CURSOR`) and the cursor's box over the
/// rectangle, the cursor is hidden (ShowCursor 0) around the copy and shown again after. Full screen: the primary's
/// BltFast (slot 0x1c, DDBLTFAST_WAIT) at the rectangle's corner from the back buffer's rectangle (right and bottom one
/// past). Windowed: the window's rectangle (GetWindowRect) moved by the frame and the caption, and the primary's Blt
/// (slot 0x14, DDBLT_WAIT) of the same size there. Then `SCREEN_UPDATED` = 1.
/// Original: `graphics_flip_rect` (0x5e2eb0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_flip_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    let (primary, back) = (read_usize(PRIMARY_SURFACE), read_usize(BACK_BUFFER));
    if read_u32(DISPLAY_OFF) != 0 || primary == 0 || back == 0 || primary == back {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return;
    }
    if read_u32(GREYSCALE_UPDATES) != 0 {
        let grey: unsafe extern "C" fn(i32, i32, i32, i32) = core::mem::transmute(0x5e7f60usize); // gui_greyscale_rect
        grey(x0, y0, x1, y1);
    }
    if read_u32(SOFTWARE_CURSOR) != 0 && cursor_over(x0, y0, x1, y1) {
        user32_show_cursor(0);
    }
    rect[2] += 1;
    rect[3] += 1;
    if read_u32(FULL_SCREEN) != 0 {
        let blt_fast: unsafe extern "system" fn(usize, i32, i32, usize, *const [i32; 4], u32) -> i32 = core::mem::transmute(read_usize(read_usize(primary) + 0x1c));
        blt_fast(primary, rect[0], rect[1], read_usize(BACK_BUFFER), &rect, 0x10);
    } else {
        let mut window = [0i32; 4];
        let get_window_rect: unsafe extern "system" fn(usize, *mut [i32; 4]) -> i32 = core::mem::transmute(read_usize(0x9671a8)); // GetWindowRect
        get_window_rect(read_usize(MAIN_WINDOW), &mut window);
        let left = window[0] + read_i32(WINDOW_FRAME_X) + rect[0];
        let top = window[1] + read_i32(WINDOW_CAPTION_Y) + rect[1];
        let dest = [left, top, left - rect[0] + rect[2], top - rect[1] + rect[3]];
        let blt: unsafe extern "system" fn(usize, *const [i32; 4], usize, *const [i32; 4], u32, usize) -> i32 = core::mem::transmute(read_usize(read_usize(primary) + 0x14));
        blt(read_usize(PRIMARY_SURFACE), &dest, read_usize(BACK_BUFFER), &rect, 0x1000000, 0);
    }
    if read_u32(SOFTWARE_CURSOR) != 0 && cursor_over(x0, y0, x1, y1) {
        user32_show_cursor(1);
    }
    write_u32(SCREEN_UPDATED, 1);
}

/// Packs red, green and blue (their low bytes) into a 16-bit pixel of the format `format` (the screen's when null):
/// 565 when the green mask is 0x7e0, otherwise 555. 0 when the display is off.
/// Original: `graphics_rgb_to_pixel` (0x5e4800, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_rgb_to_pixel(red: u32, green: u32, blue: u32, format: usize) -> u16 {
    if read_u32(DISPLAY_OFF) != 0 {
        return 0;
    }
    pack_pixel(red as u8, green as u8, blue as u8, pixel_format_or_screen(format))
}

/// The 16-bit pixel of 8-bit red, green and blue in the format at `format` (565 when its green mask is 0x7e0,
/// otherwise 555).
unsafe fn pack_pixel(red: u8, green: u8, blue: u8, format: usize) -> u16 {
    let (r, g, b) = (red as u16, green as u16, blue as u16);
    if read_u32(format + 0x14) == 0x7e0 {
        ((r & 0xf8) << 8) | ((g & 0xfc) << 3) | (b >> 3)
    } else {
        ((r & 0xf8) << 7) | ((g & 0xf8) << 2) | (b >> 3)
    }
}

/// Splits a 16-bit colour into 8-bit channels through the masks of the format at `format`: each channel is
/// (colour & mask) × 256 / (mask + 1), unsigned.
unsafe fn unpack_pixel(colour: u32, format: usize) -> [u8; 3] {
    let c = colour & 0xffff;
    let channel = |mask: u32| (((mask & c) << 8) / mask.wrapping_add(1)) as u8;
    [channel(read_u32(format + 0x10)), channel(read_u32(format + 0x14)), channel(read_u32(format + 0x18))]
}

/// Splits a packed 16-bit colour into 8-bit red, green and blue through the masks of the format (`unpack_pixel`; the
/// screen's when null). Nothing when the display is off. Returns the last quotient (the blue byte), as the original
/// leaves it in eax (the display flag when the display is off).
/// Original: `gui_colour_to_rgb` (0x5e49f0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_colour_to_rgb(colour: u32, red: *mut u8, green: *mut u8, blue: *mut u8, format: usize) -> u32 {
    if read_u32(DISPLAY_OFF) != 0 {
        return read_u32(DISPLAY_OFF);
    }
    let [r, g, b] = unpack_pixel(colour, pixel_format_or_screen(format));
    *red = r;
    *green = g;
    *blue = b;
    let blue_mask = read_u32(pixel_format_or_screen(format) + 0x18);
    ((blue_mask & colour & 0xffff) << 8) / blue_mask.wrapping_add(1)
}

/// Scales each channel of a packed colour (`unpack_pixel` through the format, the screen's when null) by `percent`
/// (its low byte; channel × percent / 100), caps each at 0xff and packs it again (`pack_pixel`). 0 when the display is
/// off.
/// Original: `gui_scale_colour` (0x5e4890, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_scale_colour(colour: u32, percent: u32, format: usize) -> u16 {
    if read_u32(DISPLAY_OFF) != 0 {
        return 0;
    }
    let [r, g, b] = unpack_pixel(colour, pixel_format_or_screen(format));
    let scale = |c: u8| ((c as i32 * (percent & 0xff) as i32) / 100).min(0xff) as u8;
    pack_pixel(scale(r), scale(g), scale(b), pixel_format_or_screen(format))
}

/// The line heights of the built-in fonts 0 to 7.
const BUILT_IN_FONT_HEIGHTS: [u32; 8] = [0xf, 0xf, 0x12, 0x15, 0x18, 0x1b, 0x27, 0x2d];

/// Returns the line height of font `font`: with the built-in fonts active (`G_FONT_MODE` not −1) the fixed height of
/// fonts 0 to 7 (`BUILT_IN_FONT_HEIGHTS`), 0 for any other; otherwise 0 for −1, else +0 of the font's slot in the
/// loaded font table.
/// Original: `font_get_height` (0x5e5cd0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_get_height(font: i16) -> u32 {
    if read_i32(G_FONT_MODE) != -1 {
        return BUILT_IN_FONT_HEIGHTS.get(font as i32 as u32 as usize).copied().unwrap_or(0);
    }
    if font == -1 {
        return 0;
    }
    read_u32(FONT_TABLE.wrapping_add((font as isize as usize).wrapping_mul(FONT_SLOT_SIZE)))
}

/// Returns the spacing between character `index` of the 16-bit string `text` and the character before it in bitmap
/// font `font`: three quarters of the font's +0x284 (signed, rounded toward zero) minus the character's +0 and the
/// previous character's +4 (the bar '|' counted as a space), never below 0. 0 for font −1, without a string or at its
/// end.
/// Original: `font_glyph_gap` (0x5e5d60, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_glyph_gap(font: i16, text: usize, index: i32) -> i32 {
    if font == -1 || text == 0 {
        return 0;
    }
    let base = FONT_TABLE.wrapping_add((font as isize as usize).wrapping_mul(FONT_SLOT_SIZE));
    let current = read_u16(text.wrapping_add((index as usize).wrapping_mul(2)));
    if current == 0 {
        return 0;
    }
    let previous = read_u16(text.wrapping_add((index as usize).wrapping_mul(2)).wrapping_sub(2));
    let space = |c: u16| if c == b'|' as u16 { b' ' as usize } else { c as usize };
    let glyph = |c: usize| base + FONT_GLYPHS + c * FONT_GLYPH_SIZE;
    let gap = read_i32(base + 0x284).wrapping_mul(3) / 4 - read_i32(glyph(space(current))) - read_i32(glyph(space(previous)) + 4);
    gap.max(0)
}

/// The glyph callback of a T2K line draw: when the glyph counter reaches the caret's glyph index, the glyph's x (the low
/// word of the first argument, sign-extended) is stored as the caret's x; the counter goes up and 0 is returned (draw
/// the glyph).
/// Original: `text_caret_glyph_callback` (0x5e5aa0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn text_caret_glyph_callback(x: i32) -> i32 {
    if read_u32(CARET_GLYPH_COUNTER) == read_u32(CARET_GLYPH_INDEX) {
        write_i32(CARET_X, x as i16 as i32);
    }
    write_u32(CARET_GLYPH_COUNTER, read_u32(CARET_GLYPH_COUNTER).wrapping_add(1));
    0
}

/// Converts the byte string `text` into 16-bit character codes at `out` (ending in 0): each byte becomes its code,
/// except a "#xxxx#" escape (a '#', four hexadecimal digits by isxdigit and a '#', starting before the last five
/// bytes), which becomes the code the digits give (sscanf "#%4x#"). Returns the number of codes, or `out` itself for an
/// empty string (as the original leaves eax).
/// Original: `text_decode_hash_escapes` (0x5e5060, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn text_decode_hash_escapes(out: usize, text: usize) -> u32 {
    let is_hex_digit: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x947063usize); // isxdigit
    let scan: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_946FD8); // sscanf
    let len = c_string_len(text) as i32;
    if len <= 0 {
        write_u16(out, 0);
        return out as u32;
    }
    let (mut i, mut count) = (0i32, 0u32);
    while i < len {
        let p = text + i as usize;
        let mut code = read_u8(p) as u32;
        if code == b'#' as u32 && i < len - 5 && read_u8(p + 5) == b'#' && (1..5).all(|k| is_hex_digit(read_u8(p + k) as i8 as i32) != 0)
            && scan(p, 0x9cda78, &mut code as *mut u32) == 1 { // "#%4x#"
            covered!("cx07_12 hash escape");
            i += 5;
        }
        write_u16(out + count as usize * 2, code as u16);
        count += 1;
        i += 1;
    }
    write_u16(out + count as usize * 2, 0);
    count
}

/// Draws a one-colour glyph on the back buffer at (x, y): the glyph is its width byte, its height byte and a mask of
/// width × height bytes; each non-zero mask byte puts `colour` in its pixel. Nothing when the display is off, without
/// a back buffer or a glyph, when the glyph does not lie fully on the screen (`clip_rect_to_surface`) or when the lock
/// fails (Lock with 0x21, DDLOCK_WAIT | DDLOCK_WRITEONLY [inference]).
/// Original: `gui_draw_glyph` (0x5e3ce0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_draw_glyph(x: i32, y: i32, colour: u32, glyph: usize) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 || glyph == 0 {
        return;
    }
    let (w, h) = (read_u8(glyph) as i32, read_u8(glyph + 1) as i32);
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x, y, x + w - 1, y + h - 1, &mut rect) == 0 {
        return;
    }
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    if !lock_back_buffer(&rect, &mut desc, 0x21) {
        return;
    }
    let pitch = read_i32(desc.as_ptr() as usize + SURFACE_DESC_PITCH) / 2;
    let mut pixel = read_usize(desc.as_ptr() as usize + SURFACE_DESC_SURFACE);
    let mut mask = glyph + 2;
    for _ in 0..h {
        for col in 0..w.max(0) as usize {
            if read_u8(mask + col) != 0 {
                write_u16(pixel + col * 2, colour as u16);
            }
        }
        mask += w as usize;
        pixel = pixel.wrapping_add(((pitch as isize) * 2) as usize);
    }
    unlock_back_buffer();
}

/// Copies the box (x0, y0)-(x1, y1) of the back buffer into an image and returns it.
///
/// Nothing (0) when the display is off, without a back buffer or when the box does not lie fully on the screen. The
/// image `image` is reused when its width (+0) and height (+4) are the box's; otherwise a new one (calloc 0x30: +0 the
/// width, +4 the height, +8 the bytes, +0xc the pixels, calloc'd; out of memory: 0, the header freed when only the
/// pixels failed). The back buffer is locked for reading (Lock with 0x11; failing: a new image is freed and 0); each
/// row copied (width × 2 bytes, the surface pitch between rows); the screen's pixel format (0x20 bytes) into +0x10; the
/// back buffer unlocked.
/// Original: `gui_save_screen_image` (0x5e3dd0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_save_screen_image(x0: i32, y0: i32, x1: i32, y1: i32, image: usize) -> usize {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 {
        return 0;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return 0;
    }
    let (w, h) = (rect[2] - rect[0] + 1, rect[3] - rect[1] + 1);
    let target = if image != 0 && read_i32(image) == w && read_i32(image + 4) == h {
        image
    } else {
        let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize); // calloc
        let new = calloc(1, 0x30);
        if new == 0 {
            return 0;
        }
        write_i32(new, w);
        write_i32(new + 4, h);
        write_i32(new + 8, w.wrapping_mul(h).wrapping_mul(2));
        let pixels = calloc(1, read_u32(new + 8));
        write_usize(new + 0xc, pixels);
        if pixels == 0 {
            exe::crt_free(new);
            return 0;
        }
        new
    };
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    if !lock_back_buffer(&rect, &mut desc, 0x11) {
        if target != image {
            gui_free_image(target);
        }
        return 0;
    }
    covered!("cx07_12 screen image saved");
    let pitch = read_i32(desc.as_ptr() as usize + SURFACE_DESC_PITCH) / 2;
    let mut src = read_usize(desc.as_ptr() as usize + SURFACE_DESC_SURFACE);
    let mut dst = read_usize(target + 0xc);
    for _ in rect[1]..=rect[3] {
        let row = (read_i32(target) * 2) as usize;
        core::ptr::copy(src as *const u8, dst as *mut u8, row);
        src = src.wrapping_add(((pitch as isize) * 2) as usize);
        dst += row;
    }
    core::ptr::copy_nonoverlapping(SCREEN_PIXEL_FORMAT as *const u8, (target + 0x10) as *mut u8, 0x20);
    unlock_back_buffer();
    target
}

/// Copies a saved image (`gui_save_screen_image`) back onto the back buffer with its top-left corner at (x, y). Nothing
/// when the display is off, without a back buffer or an image, when the image does not lie fully on the screen or when
/// the lock (0x21) fails.
/// Original: `gui_restore_screen_image` (0x5e3fd0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_restore_screen_image(x: i32, y: i32, image: usize) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 || image == 0 {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x, y, x + read_i32(image) - 1, y + read_i32(image + 4) - 1, &mut rect) == 0 {
        return;
    }
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    if !lock_back_buffer(&rect, &mut desc, 0x21) {
        return;
    }
    let pitch = read_i32(desc.as_ptr() as usize + SURFACE_DESC_PITCH) / 2;
    let mut dst = read_usize(desc.as_ptr() as usize + SURFACE_DESC_SURFACE);
    let mut src = read_usize(image + 0xc);
    for _ in rect[1]..=rect[3] {
        let row = (read_i32(image) * 2) as usize;
        core::ptr::copy(src as *const u8, dst as *mut u8, row);
        dst = dst.wrapping_add(((pitch as isize) * 2) as usize);
        src += row;
    }
    unlock_back_buffer();
}

/// Comparers: what draws or allocates runs only the original; the pure conversions and metrics side by side.
pub(crate) unsafe extern "C" fn compare_graphics_flip_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32)>(%%I0%%)(x0, y0, x1, y1)
}

pub(crate) unsafe extern "C" fn compare_graphics_rgb_to_pixel(red: u32, green: u32, blue: u32, format: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32, u32, usize) -> u16>(%%I1%%)(red, green, blue, format);
    let c = graphics_rgb_to_pixel(red, green, blue, format);
    SCOREBOARD.record(r == c, || format!("graphics_rgb_to_pixel({}, {}, {}): original {:#x} rust {:#x}", red, green, blue, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_gui_colour_to_rgb(colour: u32, red: *mut u8, green: *mut u8, blue: *mut u8, format: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(u32, *mut u8, *mut u8, *mut u8, usize) -> u32>(%%I2%%)(colour, red, green, blue, format);
    if read_u32(DISPLAY_OFF) != 0 {
        return r;
    }
    let original = [*red, *green, *blue];
    let mut copy = [0u8; 3];
    let (cr, cg, cb) = (&mut copy[0] as *mut u8, &mut copy[1] as *mut u8, &mut copy[2] as *mut u8);
    let c = gui_colour_to_rgb(colour, cr, cg, cb, format);
    SCOREBOARD.record(r == c && copy == original, || format!("gui_colour_to_rgb({:#x}): original {} {:?} rust {} {:?}", colour, r, original, c, copy));
    r
}

pub(crate) unsafe extern "C" fn compare_gui_scale_colour(colour: u32, percent: u32, format: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(u32, u32, usize) -> u16>(%%I3%%)(colour, percent, format);
    let c = gui_scale_colour(colour, percent, format);
    SCOREBOARD.record(r == c, || format!("gui_scale_colour({:#x}, {}): original {:#x} rust {:#x}", colour, percent, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_font_get_height(font: i16) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(i16) -> u32>(%%I4%%)(font);
    let c = font_get_height(font);
    SCOREBOARD.record(r == c, || format!("font_get_height({}): original {} rust {}", font, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_font_glyph_gap(font: i16, text: usize, index: i32) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i16, usize, i32) -> i32>(%%I5%%)(font, text, index);
    let c = font_glyph_gap(font, text, index);
    SCOREBOARD.record(r == c, || format!("font_glyph_gap({}, index {}): original {} rust {}", font, index, r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_text_caret_glyph_callback(x: i32) -> i32 {
    let before = (read_u32(CARET_GLYPH_COUNTER), read_i32(CARET_X));
    let r = original_fn::<unsafe extern "C" fn(i32) -> i32>(%%I6%%)(x);
    let after = (read_u32(CARET_GLYPH_COUNTER), read_i32(CARET_X));
    write_u32(CARET_GLYPH_COUNTER, before.0);
    write_i32(CARET_X, before.1);
    let c = text_caret_glyph_callback(x);
    let copy = (read_u32(CARET_GLYPH_COUNTER), read_i32(CARET_X));
    SCOREBOARD.record(r == c && after == copy, || format!("text_caret_glyph_callback({}): original {:?} rust {:?}", x, after, copy));
    r
}

pub(crate) unsafe extern "C" fn compare_text_decode_hash_escapes(out: usize, text: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> u32>(%%I7%%)(out, text);
    if r as usize == out {
        return r;
    }
    let original: Vec<u16> = (0..=r as usize).map(|k| read_u16(out + k * 2)).collect();
    let mut copy = vec![0u16; c_string_len(text) + 1];
    let c = text_decode_hash_escapes(copy.as_mut_ptr() as usize, text);
    copy.truncate(c as usize + 1);
    SCOREBOARD.record(r == c && copy == original, || format!("text_decode_hash_escapes({:?}): original {} codes, rust {}", String::from_utf8_lossy(&c_string_bytes(text)), r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_gui_draw_glyph(x: i32, y: i32, colour: u32, glyph: usize) {
    original_fn::<unsafe extern "C" fn(i32, i32, u32, usize)>(%%I8%%)(x, y, colour, glyph)
}

pub(crate) unsafe extern "C" fn compare_gui_save_screen_image(x0: i32, y0: i32, x1: i32, y1: i32, image: usize) -> usize {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, usize) -> usize>(%%I9%%)(x0, y0, x1, y1, image)
}

pub(crate) unsafe extern "C" fn compare_gui_restore_screen_image(x: i32, y: i32, image: usize) {
    original_fn::<unsafe extern "C" fn(i32, i32, usize)>(%%I10%%)(x, y, image)
}
