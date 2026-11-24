// ---------- sbC_1: the graphics base: the line, the darkened box, the RGN pictures, the bitmap fonts, the text width and the fade (goldcup.cpp by the map; the graphics library) ----------

/// The 60 % shade of every 16-bit colour (u16 × 0x10000), built on the first `gui_darken_rect`; its last word stays 0
/// until then.
const DARKEN_TABLE: usize = 0xaebe30;
/// The "rb" mode string of fopen.
const FOPEN_READ_BINARY: usize = 0x98f928;
/// An image header (`gui_save_screen_image`, `gui_load_rgn_picture`): +0 the width, +4 the height, +8 the pixel bytes,
/// +0xc the pixels, +0x10 the pixel format (0x20 bytes, the DDPIXELFORMAT; its green mask at +0x24).
const IMAGE_HEADER_SIZE: usize = 0x30;
const IMAGE_GREEN_MASK: usize = 0x24;
/// The glyph records of a bitmap font slot (`FONT_TABLE`): character c at +`FONT_GLYPH_RECORDS` + c × `FONT_GLYPH_SIZE`,
/// four dwords read from the file (+0 the width in pixels, +4 the bytes of a bitmap row, +8 and +0xc the bearings that
/// `font_glyph_gap` reads [inference]) and +0x10 the bitmap (4 bits per pixel, rows of whole bytes). The slot's +0 is
/// the font's height. (`FONT_GLYPHS` of the shutdown points at +8 of the same records.)
const FONT_GLYPH_RECORDS: usize = 4;
const GLYPH_WIDTH: usize = 0;
const GLYPH_ROW_BYTES: usize = 4;
const GLYPH_BITMAP: usize = 0x10;
/// The T2K font object and the point size of each built-in font 0 to 7 (font mode not −1).
pub(crate) const T2K_FONT_SIZES: [(usize, u32); 8] =
    [(0xaebdf4, 9), (0xaebdf8, 9), (0xaebdf8, 0xa), (0xaebdfc, 0xe), (0xaebdfc, 0x10), (0xaebdfc, 0x12), (0xaebe00, 0x14), (0xaebe00, 0x16)];

/// The glyph record of character `c` in the bitmap font `font`.
pub(crate) fn bitmap_font_glyph(font: i16, c: usize) -> usize {
    FONT_TABLE.wrapping_add((font as isize as usize).wrapping_mul(FONT_SLOT_SIZE)).wrapping_add(FONT_GLYPH_RECORDS).wrapping_add(c.wrapping_mul(FONT_GLYPH_SIZE))
}

/// The back buffer locked over `rect` with `flags`: its first pixel and its pitch in pixels (the byte pitch halved,
/// rounded toward zero, as the original computes it); none when the lock fails.
pub(crate) unsafe fn lock_back_buffer_pixels(rect: &[i32; 4], flags: u32) -> Option<(usize, i32)> {
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    if !lock_back_buffer(rect, &mut desc, flags) {
        return None;
    }
    let d = desc.as_ptr() as usize;
    Some((read_usize(d + SURFACE_DESC_SURFACE), read_i32(d + SURFACE_DESC_PITCH) / 2))
}

/// The address of pixel (x, y) of a locked surface whose first pixel is `surface` and pitch `pitch` pixels.
pub(crate) fn pixel_at(surface: usize, pitch: i32, x: i32, y: i32) -> usize {
    surface.wrapping_add((pitch.wrapping_mul(y).wrapping_add(x) as isize as usize).wrapping_mul(2))
}

/// Draws a solid line (bit 1 of `style`) or a dashed one (4 pixels on, 2 off) of `colour` from (x0, y0) to (x1, y1) on
/// the back buffer.
///
/// Nothing when the display is off, without a back buffer, when the line's box does not lie fully on the screen
/// (`clip_rect_to_surface`) or when the lock (0x21) fails. A horizontal line (the box one row high, checked first) or a
/// vertical one locks only its box and walks it from its first pixel, the dash counted from the start. Any other is a
/// Bresenham walk over the locked screen (0, 0, width − 1, height − 1) along its longer axis, from its lower end: the
/// first point always drawn, the dash counted from it.
/// Original: `graphics_draw_line` (0x5e3730, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_draw_line(x0: i32, y0: i32, x1: i32, y1: i32, style: u32, colour: u32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 {
        return;
    }
    let mut bounds = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut bounds) == 0 {
        return;
    }
    let solid = style & 2 != 0;
    let colour = colour as u16;
    if bounds[1] == bounds[3] || bounds[0] == bounds[2] {
        let Some((surface, pitch)) = lock_back_buffer_pixels(&bounds, 0x21) else { return };
        let (first, last, step) = if bounds[1] == bounds[3] { (bounds[0], bounds[2], 2isize) } else { (bounds[1], bounds[3], pitch as isize * 2) };
        let mut pixel = surface;
        for k in first..=last {
            if solid || (k - first) % 6 < 4 {
                write_u16(pixel, colour);
            }
            pixel = pixel.wrapping_add(step as usize);
        }
        unlock_back_buffer();
        return;
    }
    covered!("sbC_1 sloped line");
    let screen = [0, 0, read_i32(SCREEN_WIDTH) - 1, read_i32(SCREEN_HEIGHT) - 1];
    let Some((surface, pitch)) = lock_back_buffer_pixels(&screen, 0x21) else { return };
    let inside = |x: i32, y: i32| x >= bounds[0] && x <= bounds[2] && y >= bounds[1] && y <= bounds[3];
    let mut drawn = 0i32;
    let mut plot = |x: i32, y: i32, first: bool| {
        if inside(x, y) {
            if first || solid || drawn % 6 < 4 {
                write_u16(pixel_at(surface, pitch, x, y), colour);
            }
            drawn = if first { 1 } else { drawn + 1 };
        }
    };
    // the walk along `major` (from the lower end), `minor` stepping by ±1 when the error is not negative
    let steep = x1.wrapping_sub(x0).wrapping_abs() < y1.wrapping_sub(y0).wrapping_abs();
    let (mut a0, mut b0, mut a1, mut b1) = if steep { (y0, x0, y1, x1) } else { (x0, y0, x1, y1) };
    if a0 > a1 {
        core::mem::swap(&mut a0, &mut a1);
        core::mem::swap(&mut b0, &mut b1);
    }
    let step = if b1 > b0 { 1 } else { -1 };
    let minor = b1.wrapping_sub(b0).wrapping_abs();
    let major = a1.wrapping_sub(a0);
    let (straight, diagonal) = (minor.wrapping_mul(2), minor.wrapping_sub(major).wrapping_mul(2));
    let mut error = minor.wrapping_mul(2).wrapping_sub(major);
    let mut b = b0;
    let point = |a: i32, b: i32| if steep { (b, a) } else { (a, b) };
    let (x, y) = point(a0, b);
    plot(x, y, true);
    for a in a0 + 1..=a1 {
        if error >= 0 {
            b += step;
            error = error.wrapping_add(diagonal);
        } else {
            error = error.wrapping_add(straight);
        }
        let (x, y) = point(a, b);
        plot(x, y, false);
    }
    unlock_back_buffer();
}

/// Darkens the box (x0, y0)-(x1, y1) of the back buffer to 60 %: each pixel replaced by its entry in `DARKEN_TABLE`,
/// which is built on the first use (when its last word is still 0) with `gui_scale_colour`(colour, 60) for every
/// colour. Nothing when the display is off, without a back buffer, when the box does not lie fully on the screen or when
/// the lock (0x21) fails.
/// Original: `gui_darken_rect` (0x5e42e0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_darken_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return;
    }
    if read_u16(DARKEN_TABLE + 0xffff * 2) == 0 {
        covered!("sbC_1 darken table built");
        for colour in 0..=0xffffu32 {
            write_u16(DARKEN_TABLE + colour as usize * 2, gui_scale_colour(colour, 60, 0)); // sbC_1: 60 % of each colour
        }
    }
    let Some((surface, pitch)) = lock_back_buffer_pixels(&rect, 0x21) else { return };
    for y in rect[1]..=rect[3] {
        for x in rect[0]..=rect[2] {
            let pixel = pixel_at(surface, pitch, x - rect[0], y - rect[1]);
            write_u16(pixel, read_u16(DARKEN_TABLE + read_u16(pixel) as usize * 2));
        }
    }
    unlock_back_buffer();
}

/// Reads an RGN picture file into an image and returns it.
///
/// 0 when the display is off, without a path, when the file does not open (fopen "rb") or when the header (0x30 bytes,
/// the image header) cannot be read. The image `reuse` is kept when its width and height are the file's (its own header
/// left as it is); otherwise a new one (calloc 0x30 with the file's header, its pixels calloc'd by the header's byte
/// count; out of memory: 0, the header freed when only the pixels failed). The pixels are read whole (failing: 0, a new
/// image freed). When the file's green mask differs from the screen's, every pixel is converted, 555 to 565 when the
/// screen is 565 (green mask 0x7e0), else 565 to 555. The file is closed before returning.
/// Original: `gui_load_rgn_picture` (0x5e40d0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_load_rgn_picture(path: usize, reuse: usize) -> usize {
    if read_u32(DISPLAY_OFF) != 0 || path == 0 {
        return 0;
    }
    let file = exe::crt_fopen(path, FOPEN_READ_BINARY);
    if file == 0 {
        return 0;
    }
    let mut header = [0u8; IMAGE_HEADER_SIZE];
    let h = header.as_mut_ptr() as usize;
    if exe::crt_fread(h, IMAGE_HEADER_SIZE as u32, 1, file) != 1 {
        exe::crt_fclose(file);
        return 0;
    }
    let image = if reuse != 0 && read_i32(reuse) == read_i32(h) && read_i32(reuse + 4) == read_i32(h + 4) {
        reuse
    } else {
        let new = exe::crt_calloc(1, IMAGE_HEADER_SIZE as u32);
        if new == 0 {
            exe::crt_fclose(file);
            return 0;
        }
        core::ptr::copy_nonoverlapping(h as *const u8, new as *mut u8, IMAGE_HEADER_SIZE);
        let pixels = exe::crt_calloc(1, read_u32(new + 8));
        write_usize(new + 0xc, pixels);
        if pixels == 0 {
            exe::crt_free(new);
            exe::crt_fclose(file);
            return 0;
        }
        new
    };
    if exe::crt_fread(read_usize(image + 0xc), read_u32(image + 8), 1, file) != 1 {
        if image != reuse {
            gui_free_image(image);
        }
        exe::crt_fclose(file);
        return 0;
    }
    let screen_green = read_u32(SCREEN_PIXEL_FORMAT + 0x14);
    if read_u32(h + IMAGE_GREEN_MASK) != screen_green {
        covered!("sbC_1 picture converted");
        let pixels = read_usize(image + 0xc);
        for k in 0..read_i32(image + 4).wrapping_mul(read_i32(image)).max(0) as usize {
            let p = read_u16(pixels + k * 2);
            let converted = if screen_green == 0x7e0 { ((p & 0xffe0) << 1) | (p & 0x1f) } else { ((p >> 1) & 0x7fe0) | (p & 0x1f) };
            write_u16(pixels + k * 2, converted);
        }
    }
    exe::crt_fclose(file);
    image
}

/// The accented characters of the two Trade fonts (slots 6 and 7) and the glyph each is copied from.
const TRADE_FONT_COPIES: [(usize, usize); 14] = [
    (0x82, 0x2c), (0x84, 0x20), (0x85, 0x20), (0x86, 0x20), (0x87, 0x20), (0x8b, 0x20), (0x8c, 0xc6), (0x9b, 0x20),
    (0x9c, 0xe6), (0xa0, 0x9f), (0xd0, 0x44), (0xde, 0x54), (0xf0, 0x64), (0xfe, 0x74),
];

/// Reads the glyph records and bitmaps of characters 0x20 to 0xff of slot `font`; false at the first failed read or
/// allocation.
unsafe fn read_bitmap_font_glyphs(font: i16, file: usize) -> bool {
    let height = read_i32(bitmap_font_glyph(font, 0) - FONT_GLYPH_RECORDS);
    for c in 0x20..=0xffusize {
        let glyph = bitmap_font_glyph(font, c);
        if !(0..4).all(|k| exe::crt_fread(glyph + k * 4, 4, 1, file) == 1) {
            return false;
        }
        let row_bytes = read_i32(glyph + GLYPH_ROW_BYTES);
        if row_bytes != 0 {
            let size = row_bytes.wrapping_mul(height) as u32;
            let bitmap = exe::crt_malloc(size);
            write_usize(glyph + GLYPH_BITMAP, bitmap);
            if bitmap == 0 || exe::crt_fread(bitmap, size, 1, file) != 1 {
                return false;
            }
        }
    }
    true
}

/// Copies glyph `source` of slot `font` over glyph `target`: its old bitmap freed, the four dwords copied and, with a
/// row size, a new bitmap (malloc, not checked) with the same bytes.
unsafe fn copy_bitmap_font_glyph(font: i16, target: usize, source: usize) {
    let (t, s) = (bitmap_font_glyph(font, target), bitmap_font_glyph(font, source));
    let old = read_usize(t + GLYPH_BITMAP);
    if old != 0 {
        exe::crt_free(old);
        write_usize(t + GLYPH_BITMAP, 0);
    }
    for k in 0..4 {
        write_u32(t + k * 4, read_u32(s + k * 4));
    }
    let row_bytes = read_i32(t + GLYPH_ROW_BYTES);
    if row_bytes != 0 {
        let size = read_i32(bitmap_font_glyph(font, 0) - FONT_GLYPH_RECORDS).wrapping_mul(row_bytes) as usize;
        let bitmap = exe::crt_malloc(size as u32);
        write_usize(t + GLYPH_BITMAP, bitmap);
        core::ptr::copy_nonoverlapping(read_usize(s + GLYPH_BITMAP) as *const u8, bitmap as *mut u8, size);
    }
}

/// Loads the bitmap font file `path` into slot `font` of the font table: 1 when it loaded, 0 otherwise.
///
/// 0 without a path or when the file does not open (the original formats "load_font() : unable to open file '%s' for
/// input" into a local buffer and drops it). The slot (0x1404 bytes) is cleared, the font height read into +0, then
/// for characters 0x20 to 0xff the four dwords of the glyph record and, with a row size, the bitmap (malloc, row size ×
/// height). A failed read (after the height) or allocation frees the bitmaps read so far (not with slot −1; the
/// pointers are left as they are) and returns 0. The file is closed. For the two Trade fonts (slots 6 and 7) the
/// accented characters without a design of their own are copied from others (`TRADE_FONT_COPIES`).
/// Defects of the original: the freed bitmaps' pointers stay in the slot (a shutdown frees them again); the copied
/// bitmap's allocation is not checked.
/// Original: `gui_load_bitmap_font` (0x5e4ba0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_load_bitmap_font(font: i16, path: usize) -> u32 {
    if path == 0 {
        return 0;
    }
    let file = exe::crt_fopen(path, FOPEN_READ_BINARY);
    if file == 0 {
        return 0;
    }
    let slot = bitmap_font_glyph(font, 0) - FONT_GLYPH_RECORDS;
    core::ptr::write_bytes(slot as *mut u8, 0, FONT_SLOT_SIZE);
    if exe::crt_fread(slot, 4, 1, file) != 1 {
        exe::crt_fclose(file);
        return 0;
    }
    if !read_bitmap_font_glyphs(font, file) {
        if font != -1 {
            for c in 0x20..=0xffusize {
                let bitmap = read_usize(bitmap_font_glyph(font, c) + GLYPH_BITMAP);
                if bitmap != 0 {
                    exe::crt_free(bitmap);
                }
            }
        }
        exe::crt_fclose(file);
        return 0;
    }
    exe::crt_fclose(file);
    if font == 6 || font == 7 {
        covered!("sbC_1 Trade font accents");
        for (target, source) in TRADE_FONT_COPIES {
            copy_bitmap_font_glyph(font, target, source);
        }
    }
    1
}

/// Returns the width in pixels of the byte string `text` (its "#xxxx#" escapes decoded, `text_decode_hash_escapes`) in
/// font `font`.
///
/// With the built-in fonts (font mode not −1) the T2K font object and size of fonts 0 to 7 (`T2K_FONT_SIZES`;
/// FONT::get_text_width 0x5b18a0, thiscall, flags 0; the low word, signed), 0 for another font. Otherwise 0 for font −1,
/// else the sum, over the characters from 0x20 (the bar '|' as a space; control characters add nothing), of each
/// glyph's width and the gap to the next character (`font_glyph_gap`).
/// Defect of the original: the codes go to a stack buffer of 0x1000 with no length check (the copy sizes it to the text).
/// Original: `font_text_width` (0x5e5ad0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn font_text_width(font: i16, text: usize) -> i32 {
    let mut codes = vec![0u16; c_string_len(text) + 1];
    let codes_at = codes.as_mut_ptr() as usize;
    text_decode_hash_escapes(codes_at, text);
    if read_i32(G_FONT_MODE) != -1 {
        let Some(&(object, size)) = T2K_FONT_SIZES.get(font as i32 as u32 as usize) else { return 0 };
        let width: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(0x5b18a0usize); // FONT::get_text_width
        return width(object, 0, size, codes_at) as i16 as i32;
    }
    if font == -1 {
        return 0;
    }
    let mut total = 0i32;
    for (k, &code) in codes.iter().take_while(|&&c| c != 0).enumerate() {
        let c = if code == b'|' as u16 { 0x20 } else { code };
        if c >= 0x20 {
            let width = read_i32(bitmap_font_glyph(font, c as usize) + GLYPH_WIDTH);
            total = total.wrapping_add(width.wrapping_add(font_glyph_gap(font, codes_at, k as i32 + 1)));
        }
    }
    total
}

/// Builds the fade shades of colour `colour`: `steps` + 1 words (malloc), shade k at `gui_scale_colour`(colour,
/// 100 × k ÷ steps %), in reverse order when fading out; 0 when out of memory.
unsafe fn fade_shades(colour: u32, steps: i32, fade_out: bool) -> usize {
    let shades = exe::crt_malloc((steps as u32).wrapping_mul(2).wrapping_add(2));
    if shades == 0 {
        return 0;
    }
    for k in 0..=steps {
        let percent = (k.wrapping_mul(100) / steps) as u32;
        let index = if fade_out { steps - k } else { k };
        write_u16(shades + index as usize * 2, gui_scale_colour(colour, percent, 0));
    }
    shades
}

/// Fades the box (x0, y0)-(x1, y1) of the screen in from black (`steps` > 0) or out to black (`steps` < 0) in |steps|
/// + 1 frames.
///
/// Nothing when the display is off, without a back buffer, with 0 steps, when the box does not lie fully on the screen
/// or when it cannot be saved (`gui_save_screen_image`). Every colour of the saved box gets its shades (`fade_shades`);
/// then for each frame i the box of the back buffer is locked (0x21; failing: the routine stops), each pixel set to
/// shade i of its saved colour, unlocked and flipped to the screen (`graphics_flip_rect`). The shades and the saved box
/// are freed at the end.
/// Defect of the original: when an allocation or a lock fails it returns at once, leaking the shades and the saved box.
/// Original: `graphics_fade_rect` (0x5e30e0, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_fade_rect(x0: i32, y0: i32, x1: i32, y1: i32, steps: i32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 || steps == 0 {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return;
    }
    let image = exe::gui_save_screen_image(rect[0], rect[1], rect[2], rect[3], 0usize);
    if image == 0 {
        return;
    }
    let frames = steps.wrapping_abs();
    let (pixels, count) = (read_usize(image + 0xc), (read_i32(image) * read_i32(image + 4)).max(0) as usize);
    let mut shades = vec![0usize; 0x10000];
    for k in 0..count {
        let colour = read_u16(pixels + k * 2) as usize;
        if shades[colour] == 0 {
            shades[colour] = fade_shades(colour as u32, frames, steps < 0);
            if shades[colour] == 0 {
                return;
            }
        }
    }
    covered!("sbC_1 fade");
    for frame in 0..=frames as usize {
        let Some((surface, pitch)) = lock_back_buffer_pixels(&rect, 0x21) else { return };
        let mut source = pixels;
        for y in 0..=rect[3] - rect[1] {
            for x in 0..=rect[2] - rect[0] {
                write_u16(pixel_at(surface, pitch, x, y), read_u16(shades[read_u16(source) as usize] + frame * 2));
                source += 2;
            }
        }
        unlock_back_buffer();
        exe::graphics_flip_rect(rect[0] as u32, rect[1] as u32, rect[2] as u32, rect[3] as u32);
    }
    for &s in shades.iter().filter(|&&s| s != 0) {
        exe::crt_free(s);
    }
    gui_free_image(image);
}

/// The comparer of a routine that only draws inside the box (x0, y0)-(x1, y1) of the back buffer: the box saved, the
/// original drawn, its drawing saved and the box put back, the copy drawn, and the two drawings compared pixel by pixel
/// (on a difference the original's drawing is put back). Without a display, or with the box not fully on the screen,
/// only the original runs.
pub(crate) unsafe fn compare_drawing_in_box(name: &str, b: [i32; 4], original: impl FnOnce(), copy: impl FnOnce()) {
    let mut rect = [0i32; 4];
    let on_screen = read_u32(DISPLAY_OFF) == 0 && read_usize(BACK_BUFFER) != 0 && clip_rect_to_surface(b[0], b[1], b[2], b[3], &mut rect) != 0;
    let save = || gui_save_screen_image(rect[0], rect[1], rect[2], rect[3], 0);
    let before = if on_screen { save() } else { 0 };
    original();
    if before == 0 {
        return;
    }
    let by_original = save();
    gui_restore_screen_image(rect[0], rect[1], before);
    copy();
    let by_copy = save();
    let pixels = |image: usize| -> Vec<u16> {
        if image == 0 {
            return Vec::new();
        }
        core::slice::from_raw_parts(read_usize(image + 0xc) as *const u16, read_u32(image + 8) as usize / 2).to_vec()
    };
    let (o, c) = (pixels(by_original), pixels(by_copy));
    SCOREBOARD.record(o == c, || {
        let k = o.iter().zip(&c).position(|(a, b)| a != b).unwrap_or(0);
        let w = (rect[2] - rect[0] + 1).max(1) as usize;
        format!("{}: the drawings differ in the box {:?}, first at ({}, {}): original {:#06x?} rust {:#06x?}", name, b,
            rect[0] + (k % w) as i32, rect[1] + (k / w) as i32, o.get(k), c.get(k))
    });
    if o != c && by_original != 0 {
        gui_restore_screen_image(rect[0], rect[1], by_original);
    }
    for image in [before, by_original, by_copy] {
        gui_free_image(image);
    }
}

/// Comparers: the line and the darkened box side by side on the back buffer (`compare_drawing_in_box`); the RGN picture
/// side by side (the copy reads the file again into its own image, or into the reused one); the text width side by
/// side; the font load and the fade only the original (they fill the font table or flip frames to the screen).
pub(crate) unsafe extern "C" fn compare_graphics_draw_line(x0: i32, y0: i32, x1: i32, y1: i32, style: u32, colour: u32) {
    compare_drawing_in_box("graphics_draw_line", [x0, y0, x1, y1],
        || original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, u32, u32)>(%%I0%%)(x0, y0, x1, y1, style, colour),
        || graphics_draw_line(x0, y0, x1, y1, style, colour));
}

pub(crate) unsafe extern "C" fn compare_gui_darken_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    compare_drawing_in_box("gui_darken_rect", [x0, y0, x1, y1],
        || original_fn::<unsafe extern "C" fn(i32, i32, i32, i32)>(%%I1%%)(x0, y0, x1, y1),
        || gui_darken_rect(x0, y0, x1, y1));
}

/// The bytes of an image: its width, height and byte count and its pixels.
unsafe fn image_contents(image: usize) -> Vec<u8> {
    if image == 0 {
        return Vec::new();
    }
    let mut v = core::slice::from_raw_parts(image as *const u8, 0xc).to_vec();
    v.extend_from_slice(core::slice::from_raw_parts(read_usize(image + 0xc) as *const u8, read_u32(image + 8) as usize));
    v
}

pub(crate) unsafe extern "C" fn compare_gui_load_rgn_picture(path: usize, reuse: usize) -> usize {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> usize>(%%I2%%)(path, reuse);
    let original = image_contents(r);
    let c = gui_load_rgn_picture(path, reuse);
    let copy = image_contents(c);
    SCOREBOARD.record((r == 0) == (c == 0) && (r == reuse) == (c == reuse) && original == copy, || {
        format!("gui_load_rgn_picture({:?}): original {:#x} ({} bytes) rust {:#x} ({} bytes), reused {}", String::from_utf8_lossy(&c_string_bytes(path)),
            r, original.len(), c, copy.len(), reuse)
    });
    if c != 0 && c != reuse && c != r {
        gui_free_image(c);
    }
    r
}

pub(crate) unsafe extern "C" fn compare_gui_load_bitmap_font(font: i16, path: usize) -> u32 {
    original_fn::<unsafe extern "C" fn(i16, usize) -> u32>(%%I3%%)(font, path)
}

pub(crate) unsafe extern "C" fn compare_font_text_width(font: i16, text: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(i16, usize) -> i32>(%%I4%%)(font, text);
    let c = font_text_width(font, text);
    SCOREBOARD.record(r == c, || format!("font_text_width({}, {:?}): original {} rust {}", font, String::from_utf8_lossy(&c_string_bytes(text)), r, c));
    r
}

pub(crate) unsafe extern "C" fn compare_graphics_fade_rect(x0: i32, y0: i32, x1: i32, y1: i32, steps: i32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, i32)>(%%I5%%)(x0, y0, x1, y1, steps)
}
