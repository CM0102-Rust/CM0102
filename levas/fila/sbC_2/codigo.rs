// ---------- sbC_2: the graphics base: a text line, the text box and the panel (goldcup.cpp by the map; the graphics library) ----------

/// `gui_draw_text_line` (cdecl) and `gui_darken_rect` (cdecl), called by the exe's address as the original does.
const FN_GUI_DRAW_TEXT_LINE: usize = 0x5e5160;
const FN_GUI_DARKEN_RECT: usize = 0x5e42e0;
/// The y offset and the flags FONT::draw_text gets for each built-in font 0 to 7 (0x200: the bar '|' drawn as a space).
const T2K_LINE_PLACEMENT: [(i32, u32); 8] = [(3, 0x200), (3, 0x200), (4, 0x200), (4, 0x200), (4, 0x200), (4, 0x200), (9, 0x280), (0xb, 0x280)];

/// The line height the text routines use inline: with the built-in fonts the fixed height of fonts 0 to 7
/// (`BUILT_IN_FONT_HEIGHTS`, 0 for another), otherwise +0 of the bitmap font's slot. (Not `font_get_height`: that one
/// is changed in the 1080 exe, these inline tables are not.)
pub(crate) unsafe fn inline_font_height(font: i16) -> i32 {
    if read_i32(G_FONT_MODE) != -1 {
        BUILT_IN_FONT_HEIGHTS.get(font as i32 as u32 as usize).copied().unwrap_or(0) as i32
    } else {
        read_i32(bitmap_font_glyph(font, 0) - FONT_GLYPH_RECORDS)
    }
}

/// The back buffer's pixel at (x, y): 0 without a back buffer, when the point is off the screen or when the lock
/// (0x11, for reading) fails.
pub(crate) unsafe fn read_back_buffer_pixel(x: i32, y: i32) -> u16 {
    if read_usize(BACK_BUFFER) == 0 {
        return 0;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x, y, x, y, &mut rect) == 0 {
        return 0;
    }
    let Some((surface, _)) = lock_back_buffer_pixels(&rect, 0x11) else { return 0 };
    let pixel = read_u16(surface);
    unlock_back_buffer();
    pixel
}

/// The target FONT::draw_text writes to: the first pixel, the pitch in pixels, the width and the height.
#[repr(C)]
struct T2kTarget {
    pixels: usize,
    pitch: i32,
    width: i16,
    height: i16,
}

/// A text line in the T2K font of built-in font `font` (font mode not −1); see `gui_draw_text_line`.
unsafe fn draw_t2k_text_line(x: i32, y: i32, font: i16, colour: u32, codes: usize, caret: i32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 || font == -1 {
        return;
    }
    let screen = [0, 0, read_i32(SCREEN_WIDTH) - 1, read_i32(SCREEN_HEIGHT) - 1];
    let Some((pixels, pitch)) = lock_back_buffer_pixels(&screen, 0x21) else { return };
    let target = T2kTarget { pixels, pitch: pitch as i16 as i32, width: read_i16(SCREEN_WIDTH), height: read_i16(SCREEN_HEIGHT) };
    let callback = if caret > -1 {
        write_i32(CARET_GLYPH_INDEX, caret);
        write_u32(CARET_GLYPH_COUNTER, 0);
        write_i32(CARET_X, -1);
        0x5e5aa0 // text_caret_glyph_callback
    } else {
        0
    };
    if let Some(&(object, size)) = T2K_FONT_SIZES.get(font as i32 as u32 as usize) {
        let (offset, flags) = T2K_LINE_PLACEMENT[font as usize];
        let draw: unsafe extern "thiscall" fn(usize, *const T2kTarget, i32, i32, u32, u32, u32, usize, usize) -> u32 = core::mem::transmute(0x5b0d80usize); // FONT::draw_text
        draw(object, &target, x, y + offset, flags, size, colour, codes, callback);
    }
    unlock_back_buffer();
    if caret > -1 && read_i32(CARET_X) > -1 {
        let cx = read_i32(CARET_X);
        exe::graphics_draw_line(cx, y, cx, inline_font_height(font) + y - 1, 2, colour);
    }
}

/// The background of the last blended pixel of a bitmap text line, and the colour that blend produced.
struct GlyphBlend {
    last: u32,
    background: [u8; 3],
}

impl GlyphBlend {
    /// Mixes `foreground` over the pixel `pixel` with coverage `alpha` (1 to 14 of 15), channel by channel, and packs it
    /// (0 when the display is off). The background is unpacked again only when the pixel differs from the colour the
    /// previous mix produced (the original's cache keys on that result, not on the previous background).
    unsafe fn mix(&mut self, pixel: u16, foreground: [u8; 3], alpha: u8) -> u16 {
        let off = read_u32(DISPLAY_OFF) != 0;
        if pixel as u32 != self.last && !off {
            self.background = unpack_pixel(pixel as u32, SCREEN_PIXEL_FORMAT);
        }
        let (a, b) = (alpha as i32, 15 - alpha as i32);
        let channel = |k: usize| ((foreground[k] as i32 * a + self.background[k] as i32 * b) / 15) as u8;
        let result = if off { 0 } else { pack_pixel(channel(0), channel(1), channel(2), SCREEN_PIXEL_FORMAT) };
        self.last = result as u32;
        result
    }
}

/// Draws a 4-bit glyph bitmap of `width` × `height` with its top-left corner at (x, y): coverage 15 puts `colour`, 0
/// leaves the pixel, any other mixes (`GlyphBlend`). Nothing when the glyph does not lie fully on the screen. False when
/// the lock fails (the line stops there).
unsafe fn draw_bitmap_glyph(x: i32, y: i32, width: i32, height: i32, bitmap: usize, colour: u32, foreground: [u8; 3], blend: &mut GlyphBlend) -> bool {
    let (right, bottom) = (width + x - 1, height + y - 1);
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x, y, right, bottom, &mut rect) == 0 {
        return true;
    }
    let Some((surface, pitch)) = lock_back_buffer_pixels(&rect, 0x21) else { return false };
    let mut nibbles = bitmap;
    let mut pixel = surface;
    for _ in y..=bottom {
        for column in x..=right {
            let alpha = if (column - x) % 2 == 0 {
                read_u8(nibbles) >> 4
            } else {
                nibbles += 1;
                read_u8(nibbles - 1) & 0xf
            };
            match alpha {
                0xf => write_u16(pixel, colour as u16),
                0 => {}
                _ => write_u16(pixel, blend.mix(read_u16(pixel), foreground, alpha)),
            }
            pixel += 2;
        }
        if width.max(0) % 2 != 0 {
            nibbles += 1; // the row's last byte, half used
        }
        pixel = pixel.wrapping_add((pitch.wrapping_sub(width) as isize as usize).wrapping_mul(2));
    }
    unlock_back_buffer();
    true
}

/// A text line in the bitmap font `font` (font mode −1); see `gui_draw_text_line`.
unsafe fn draw_bitmap_text_line(x: i32, y: i32, font: i16, colour: u32, codes: &[u16], caret: i32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 || font == -1 {
        return;
    }
    let height = read_i32(bitmap_font_glyph(font, 0) - FONT_GLYPH_RECORDS);
    let foreground = unpack_pixel(colour, SCREEN_PIXEL_FORMAT);
    let mut blend = GlyphBlend { last: u32::MAX, background: [0; 3] };
    let (mut pen, mut caret_x, mut index) = (x, -1i32, 1i32);
    for &code in codes.iter().take_while(|&&c| c != 0) {
        let c = if code == b'|' as u16 { 0x20 } else { code };
        if c >= 0x20 {
            let glyph = bitmap_font_glyph(font, c as usize);
            let (width, bitmap) = (read_i32(glyph + GLYPH_WIDTH), read_usize(glyph + GLYPH_BITMAP));
            if bitmap != 0 && c > 0x20 && !draw_bitmap_glyph(pen, y, width, height, bitmap, colour, foreground, &mut blend) {
                return;
            }
            if index == caret.wrapping_add(1) {
                caret_x = x.max(pen - 1);
            }
            pen = pen.wrapping_add(width.wrapping_add(font_glyph_gap(font, codes.as_ptr() as usize, index)));
        }
        index += 1;
    }
    if index == caret.wrapping_add(1) {
        caret_x = pen;
    }
    if caret_x > -1 {
        covered!("sbC_2 bitmap caret");
        exe::graphics_draw_line(caret_x, y, caret_x, height + y - 1, 2, colour);
    }
}

/// Draws the byte string `text` (its "#xxxx#" escapes decoded, `text_decode_hash_escapes`) in font `font` and `colour`
/// with its top-left corner at (x, y), with a caret before character `caret` (−1: none).
///
/// Nothing when the display is off, without a back buffer or for font −1. With the built-in fonts (font mode not −1)
/// the whole screen is locked (0x21) and the T2K font object and size of fonts 0 to 7 draw it (FONT::draw_text
/// 0x5b0d80, thiscall: the y moved down and the flags of `T2K_LINE_PLACEMENT`; with a caret, `text_caret_glyph_callback`
/// records the caret's x); another font draws nothing. With the bitmap fonts each character from 0x20 (the bar '|' as a
/// space; control characters neither draw nor move) is drawn when it has a bitmap and is not a space
/// (`draw_bitmap_glyph`; a failed lock ends the line, without a caret) and the pen moves by its width and the gap to the
/// next (`font_glyph_gap`); the caret's x is the larger of x and the pen − 1 at the caret's character, or the pen after
/// the last character. A caret with an x is a solid line of the font's height (`graphics_draw_line`).
/// Defects of the original: the codes go to a stack buffer of 0x1000 with no length check (the copy sizes it to the
/// text); the bitmap blend reuses the previous background when a pixel equals the previous blend's result.
/// Original: `gui_draw_text_line` (0x5e5160, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_draw_text_line(x: i32, y: i32, font: i16, colour: u32, text: usize, caret: i32) {
    let mut codes = vec![0u16; c_string_len(text) + 1];
    text_decode_hash_escapes(codes.as_mut_ptr() as usize, text);
    if read_i32(G_FONT_MODE) != -1 {
        draw_t2k_text_line(x, y, font, colour, codes.as_ptr() as usize, caret);
    } else {
        draw_bitmap_text_line(x, y, font, colour, &codes, caret);
    }
}

/// The byte at `k` of a zero-filled line buffer (0 past its end).
fn byte_at(buffer: &[u8], k: i32) -> u8 {
    if k < 0 { 0 } else { buffer.get(k as usize).copied().unwrap_or(0) }
}

/// The C string at `from` in a line buffer, without its terminator.
fn c_string_at(buffer: &[u8], from: usize) -> Vec<u8> {
    buffer[from.min(buffer.len())..].iter().copied().take_while(|&c| c != 0).collect()
}

/// Copies `text` and its terminator into the line buffer at `at`.
fn put_c_string(buffer: &mut [u8], at: usize, text: &[u8]) {
    buffer[at..at + text.len()].copy_from_slice(text);
    buffer[at + text.len()] = 0;
}

/// The word wrap of `gui_draw_text_box`: `text` cut into the lines that fit `width` pixels in font `font`, each
/// appended to `list` (LLIST::add_tail with its terminator).
///
/// Position i runs over the text from 1. A character from 0x20, while the line has not wrapped, is appended (a "#xxxx#"
/// escape, starting before the last four bytes, whole, i jumping past it). When the line then measures more than
/// `width` (`font_text_width`), it is cut: at an escape ending at buffer index i − 1 (i > 6) the escape moves to the
/// carry; otherwise from buffer index i − 1 down to the last space before it (stopping at index 0), the part after it
/// moves to the carry and the line ends there; the line is marked wrapped (later characters are dropped until it is
/// flushed). A newline, or a wrapped line with flag 0x100, flushes the line (the carry starts the next). At the text's
/// last position a non-empty line is flushed.
/// Defect of the original: the cut is searched at the text position, not the line's; from the second line on it reads
/// past the line, over stack bytes the original never set (the copy's buffer is zero there, so the search falls back to
/// the line's last space).
unsafe fn wrap_text_box_lines(list: usize, text: usize, font: i16, width: i32, flags: u32) {
    let bytes = c_string_bytes(text);
    let length = bytes.len() as i32;
    let size = 0x100.max(bytes.len() * 2 + 16);
    let (mut line, mut carry) = (vec![0u8; size], vec![0u8; size]);
    let mut wrapped = false;
    let flush = |line: &[u8]| {
        let content = c_string_at(line, 0);
        exe::llist_add_tail(list, line.as_ptr() as usize, content.len() as u32 + 1);
    };
    let mut i = 1i32;
    while i <= length {
        let c = bytes[i as usize - 1];
        if c >= 0x20 && !wrapped {
            let piece = if c == b'#' && i < length - 4 && bytes[i as usize + 4] == b'#' {
                let p = bytes[i as usize..i as usize + 4].to_vec();
                i += 5;
                [&[b'#'][..], &p, &[b'#']].concat()
            } else {
                vec![c]
            };
            let end = c_string_at(&line, 0).len();
            put_c_string(&mut line, end, &piece);
            if exe::font_text_width(font as i32, line.as_ptr()) > width {
                covered!("sbC_2 text wrapped");
                // (where the carry starts, where the line ends)
                let (moved_from, line_end) = if i > 6 && byte_at(&line, i - 1) == b'#' {
                    (i - 6, i - 6) // the escape that went over
                } else {
                    let mut k = i - 1;
                    if i > 1 {
                        while byte_at(&line, k) != b' ' {
                            k -= 1;
                            if k <= 0 {
                                break;
                            }
                        }
                    }
                    (k + 1, k)
                };
                let moved = c_string_at(&line, moved_from as usize);
                put_c_string(&mut carry, 0, &moved);
                line[line_end as usize] = 0;
                wrapped = true;
            }
        }
        if bytes[i as usize - 1] == b'\n' || (flags & 0x100 != 0 && wrapped) {
            flush(&line);
            let next = c_string_at(&carry, 0);
            put_c_string(&mut line, 0, &next);
            wrapped = false;
            carry[0] = 0;
        }
        if i == length && line[0] != 0 {
            flush(&line);
        }
        i += 1;
    }
}

/// Draws a text line of the box in the engraved look (flag 0x20): the box centre's background pixel
/// (`read_back_buffer_pixel`) lightened to 166 % at (x + 1, y + 1), then darkened to 66 % at (x, y); the colour given
/// is not used.
unsafe fn draw_engraved_text_line(x: i32, y: i32, font: i16, line: usize, caret: i32, centre: (i32, i32)) {
    let draw: unsafe extern "C" fn(i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_GUI_DRAW_TEXT_LINE);
    let background = if read_u32(DISPLAY_OFF) != 0 { 0 } else { read_back_buffer_pixel(centre.0, centre.1) } as u32;
    draw(x + 1, y + 1, font as i32, gui_scale_colour(background, 166, 0) as u32, line, caret);
    draw(x, y, font as i32, gui_scale_colour(background, 66, 0) as u32, line, caret);
}

/// Draws the byte string `text` word-wrapped into the box (x1, y1)-(x2, y2) in font `font` and `colour`, with a caret
/// before character `caret` (−1: none) on every line.
///
/// Nothing when the display is off, for font −1 or without a text. The lines (`wrap_text_box_lines`) go to a new LLIST
/// (operator new 0xe, LLIST::LLIST); with a caret and no line, one empty line. Flag 2 starts at y1, otherwise the
/// lines are centred vertically (the box height minus the lines × the line height, halved). Each line: flag 0x80 turns
/// every character into '*'; flag 1 aligns it left at x1, 0x40 right at x2, otherwise centred on the box width
/// (`font_text_width`); flag 0x20 draws it engraved (`draw_engraved_text_line`), otherwise `gui_draw_text_line`; the next
/// line is one line height lower (`inline_font_height`). The list is destroyed and freed.
/// Original: `gui_draw_text_box` (0x5e6d80, goldcup.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn gui_draw_text_box(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, font: i16, colour: u32, text: usize, caret: i32) {
    if read_u32(DISPLAY_OFF) != 0 || font == -1 || text == 0 {
        return;
    }
    let (width, height) = (x2 - x1 + 1, y2 - y1 + 1);
    let list = exe::crt_operator_new(0xe);
    let list = if list != 0 { exe::new_llist(list) } else { 0 };
    wrap_text_box_lines(list, text, font, width, flags);
    if caret > -1 && read_u16(list + llist::COUNT) == 0 {
        exe::llist_add_tail(list, EMPTY_STRING, 1);
    }
    let line_height = inline_font_height(font);
    let mut y = if flags & 2 != 0 { y1 } else { (height - read_i16(list + llist::COUNT) as i32 * line_height) / 2 + y1 };
    write_u32(list + llist::CURSOR, read_u32(list + llist::HEAD));
    let draw: unsafe extern "C" fn(i32, i32, i32, u32, usize, i32) = core::mem::transmute(FN_GUI_DRAW_TEXT_LINE);
    loop {
        let line = exe::llist_get_next(list);
        if line == 0 {
            break;
        }
        if flags & 0x80 != 0 {
            core::ptr::write_bytes(line as *mut u8, b'*', c_string_len(line));
        }
        let x = if flags & 1 != 0 {
            x1
        } else if flags & 0x40 != 0 {
            x2 - exe::font_text_width(font as i32, line as *const u8) + 1
        } else {
            (width - exe::font_text_width(font as i32, line as *const u8)) / 2 + x1
        };
        if flags & 0x20 != 0 {
            draw_engraved_text_line(x, y, font, line, caret, (width / 2 + x1, height / 2 + y1));
        } else {
            draw(x, y, font as i32, colour, line, caret);
        }
        y += line_height;
    }
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
}

/// The fill of the panel (x1, y1)-(x2, y2) in `colour`: flag 2 darkens it (`gui_darken_rect`); 0x10 fills it, as an
/// ellipse with 0x10000 (`gui_draw_circle`, style 4) or a rectangle (`graphics_draw_rect`, style 4); 4 is a horizontal
/// gradient (column i of n = x2 − x1 at 100 + (−100 × i) ÷ n % of the colour, `gui_scale_colour`, a vertical line each)
/// and 8 the same downward (rows).
unsafe fn draw_panel_fill(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, colour: u32) {
    if flags & 2 != 0 {
        let darken: unsafe extern "C" fn(i32, i32, i32, i32) = core::mem::transmute(FN_GUI_DARKEN_RECT);
        darken(x1, y1, x2, y2);
    } else if flags & 0x10 != 0 {
        if flags & 0x10000 != 0 {
            exe::gui_draw_circle(x1, y1, x2, y2, 4, colour);
        } else {
            exe::graphics_draw_rect(x1, y1, x2, y2, 4, colour);
        }
    } else if flags & 0xc != 0 {
        covered!("sbC_2 panel gradient");
        let horizontal = flags & 4 != 0;
        let (first, last) = if horizontal { (x1, x2) } else { (y1, y2) };
        for k in first..=last {
            let percent = ((k - first).wrapping_mul(-100) / (last - first)).wrapping_add(100) as u32; // sbC_2: the gradient step
            let shade = gui_scale_colour(colour, percent, 0) as u32;
            if horizontal {
                exe::graphics_draw_line(k, y1, k, y2, 2, shade);
            } else {
                exe::graphics_draw_line(x1, k, x2, k, 2, shade);
            }
        }
    }
}

/// The colour a frame line lightens: the panel's, or the dark blue interface colour when its low word is 0.
unsafe fn panel_frame_colour(colour: u32) -> u32 {
    if colour as u16 != 0 { colour } else { read_u16(G_GUI_COLOUR_DARK_BLUE) as u32 }
}

/// The bevel of the panel (x1, y1)-(x2, y2): `thickness` nested rectangles from the inside out, ring i with its top
/// and left lines in the light shade (`panel_frame_colour` at 100 + q %) and its bottom and right lines in the dark one
/// (the panel's own colour at 100 − q %), q = 66 × (i + 1) ÷ thickness; flag 0x40 swaps the two (pressed).
unsafe fn draw_panel_bevel(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, colour: u32, thickness: i32) {
    for i in 0..thickness {
        let q = 66 * (i + 1) / thickness;
        let mut light = gui_scale_colour(panel_frame_colour(colour), (q + 100) as u32, 0) as u32;
        let mut dark = gui_scale_colour(colour, (100 - q) as u32, 0) as u32;
        if flags & 0x40 != 0 {
            core::mem::swap(&mut light, &mut dark);
        }
        let (top, left) = (y1 + thickness - 1 - i, x1 + thickness - 1 - i);
        let (bottom, right) = (y2 - thickness + 1 + i, x2 - thickness + 1 + i);
        exe::graphics_draw_line(left, top, right, top, 2, light);
        exe::graphics_draw_line(left, top, left, bottom, 2, light);
        exe::graphics_draw_line(right, bottom, left + 1, bottom, 2, dark);
        exe::graphics_draw_line(right, bottom, right, top + 1, 2, dark);
    }
}

/// The outline of the panel: with flag 0x20 an ellipse (0x10000: `gui_draw_circle` in `extra`'s colour), a rectangle
/// (0x200: `graphics_draw_rect`), each dashed with 0x2000, or the bevel (`draw_panel_bevel`, 4 thick when the panel is
/// at least 50 × 50, its bottom above y 100 and flag 0x10 is set, otherwise 2); without 0x20, flag 0x80 underlines it
/// (a dashed line two pixels in from each side on y2, at 90 % of `panel_frame_colour`) and 0x400 draws a line across
/// it, at y1 with 0x4000 or halfway down, dashed with 0x2000.
unsafe fn draw_panel_outline(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, colour: u32, extra: u32) {
    let style = if flags & 0x2000 != 0 { 1 } else { 2 };
    if flags & 0x20 != 0 {
        if flags & 0x10000 != 0 {
            exe::gui_draw_circle(x1, y1, x2, y2, style, extra);
        } else if flags & 0x200 != 0 {
            exe::graphics_draw_rect(x1, y1, x2, y2, style, colour);
        } else {
            let thick = x2 - x1 >= 0x32 && y2 - y1 >= 0x32 && y2 < 0x64 && flags & 0x10 != 0;
            draw_panel_bevel(x1, y1, x2, y2, flags, colour, if thick { 4 } else { 2 });
        }
    } else if flags & 0x80 != 0 {
        exe::graphics_draw_line(x1 + 2, y2, x2 - 2, y2, 1, gui_scale_colour(panel_frame_colour(colour), 90, 0) as u32);
    } else if flags & 0x400 != 0 {
        let y = if flags & 0x4000 != 0 { y1 } else { (y2 - y1) / 2 + y1 };
        exe::graphics_draw_line(x1, y, x2, y, style, colour);
    }
}

/// Draws a screen panel in the box (x1, y1)-(x2, y2) by its flags.
///
/// Nothing when the display is off. Flag 0x20000 pulls the right and bottom edges in by 2, 0x40000 the left and top
/// edges; 0x1000 takes the colour from the back buffer's pixel at the (moved) top-left corner
/// (`read_back_buffer_pixel`). Then the fill (`draw_panel_fill`) and the outline (`draw_panel_outline`); flag 0x1000000
/// adds a groove halfway down (from two pixels in on each side: a line at 66 % of the colour, then one below at 133 % of
/// `panel_frame_colour`); flag 0x800 a yellow solid rectangle one pixel outside the box (the focus).
/// Defect of the original: a gradient panel one pixel wide (or high) divides by zero.
/// Original: `gui_draw_panel` (0x5e5df0, goldcup.cpp, cdecl).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn gui_draw_panel(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, colour: u32, extra: u32) {
    if read_u32(DISPLAY_OFF) != 0 {
        return;
    }
    let (mut x1, mut y1, mut x2, mut y2, mut colour) = (x1, y1, x2, y2, colour);
    if flags & 0x20000 != 0 {
        x2 -= 2;
        y2 -= 2;
    }
    if flags & 0x40000 != 0 {
        x1 += 2;
        y1 += 2;
    }
    if flags & 0x1000 != 0 {
        colour = read_back_buffer_pixel(x1, y1) as u32;
    }
    draw_panel_fill(x1, y1, x2, y2, flags, colour);
    draw_panel_outline(x1, y1, x2, y2, flags, colour, extra);
    if flags & 0x1000000 != 0 {
        covered!("sbC_2 panel groove");
        let middle = (y2 - y1) / 2 + y1;
        exe::graphics_draw_line(x1 + 2, middle, x2 - 2, middle, 2, gui_scale_colour(colour, 66, 0) as u32);
        exe::graphics_draw_line(x1 + 2, middle + 1, x2 - 2, middle + 1, 2, gui_scale_colour(panel_frame_colour(colour), 133, 0) as u32);
    }
    if flags & 0x800 != 0 {
        exe::graphics_draw_rect(x1 - 1, y1 - 1, x2 + 1, y2 + 1, 2, read_u16(G_GUI_COLOUR_YELLOW) as u32);
    }
}

/// Comparers: the text line and the text box only the original (the text has no fixed box, and drawing a blended text
/// twice over itself would change it); the panel side by side on the back buffer (`compare_drawing_in_box`, one pixel
/// around the box for the focus rectangle).
pub(crate) unsafe extern "C" fn compare_gui_draw_text_line(x: i32, y: i32, font: i16, colour: u32, text: usize, caret: i32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i16, u32, usize, i32)>(%%I0%%)(x, y, font, colour, text, caret)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_gui_draw_text_box(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, font: i16, colour: u32, text: usize, caret: i32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, u32, i16, u32, usize, i32)>(%%I1%%)(x1, y1, x2, y2, flags, font, colour, text, caret)
}

#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "C" fn compare_gui_draw_panel(x1: i32, y1: i32, x2: i32, y2: i32, flags: u32, colour: u32, extra: u32) {
    compare_drawing_in_box("gui_draw_panel", [x1 - 1, y1 - 1, x2 + 1, y2 + 1],
        || original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32)>(%%I2%%)(x1, y1, x2, y2, flags, colour, extra),
        || gui_draw_panel(x1, y1, x2, y2, flags, colour, extra));
}
