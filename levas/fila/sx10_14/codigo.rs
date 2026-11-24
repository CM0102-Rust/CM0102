// ---------- batch %%LEVA%%: drawing a TrueType/T2K text on a 15 or 16-bit surface (FONT::draw_text, font code of fog_of_war.cpp) ----------

/// The surface `FONT::draw_text` draws on (filled by the caller): the 16-bit pixels, the pitch in pixels and the size.
#[allow(dead_code)]
mod font_surface {
    pub(crate) const PIXELS: usize = 0x0;
    pub(crate) const PITCH: usize = 0x4;
    /// i16 each.
    pub(crate) const WIDTH: usize = 0x8;
    pub(crate) const HEIGHT: usize = 0xa;
}

/// The green mask of the display's pixel format (0x7e0: 5-6-5, otherwise 5-5-5).
const FONT_PIXEL_GREEN_MASK: usize = 0xae316c;

/// The `FONT::draw_text` flags read by the drawing itself (the low five bits go to the glyph: bold, slant, blur 0x10).
const DRAW_EMBOSS: u32 = 0x20;
const DRAW_TINT: u32 = 0x40;
const DRAW_SHADOW: u32 = 0x80;
const DRAW_MIRROR: u32 = 0x100;
const DRAW_BAR_AS_SPACE: u32 = 0x200;

/// The callback a caller may give `FONT::draw_text` (cdecl): the box of each character (left, top, right, bottom, the
/// two vertical metrics) and a last empty box at the pen's end; a non-zero answer skips drawing that character.
type DrawTextCallback = unsafe extern "C" fn(i32, i32, i32, i32) -> i32;

/// The three 8-bit components (red, green, blue) of a 16-bit pixel in the display format, as the original unpacks
/// them: the low 3 bits of each component are zero (the green keeps 2 more in 5-6-5).
unsafe fn font_unpack(pixel: u16) -> (i32, i32, i32) {
    let p = pixel as u32;
    let blue = (p << 3) as u8 as i32;
    if read_u32(FONT_PIXEL_GREEN_MASK) == 0x7e0 {
        ((p >> 8) as u8 as i32 & 0xf8, (p >> 3) as u8 as i32 & 0xfc, blue)
    } else {
        ((p >> 7) as u8 as i32 & 0xf8, (p >> 2) as u8 as i32 & 0xf8, blue)
    }
}

/// Packs three components (their low bytes) into a 16-bit pixel in the display format.
unsafe fn font_pack(red: i32, green: i32, blue: i32) -> u16 {
    let (r, g, b) = (red as u8 as u16, green as u8 as u16, blue as u8 as u16);
    if read_u32(FONT_PIXEL_GREEN_MASK) == 0x7e0 {
        ((r & 0xf8) << 8) | ((g & 0xfc) << 3) | (b >> 3)
    } else {
        ((r & 0xf8) << 7) | ((g & 0xf8) << 2) | (b >> 3)
    }
}

/// The coverage scale of the glyph bitmaps (a byte of 126 is full ink).
const FULL_INK: i32 = 0x7e;

/// Mixes a component towards `target` by `alpha` / 126 (truncated).
fn font_mix(current: i32, target: i32, alpha: i32) -> i32 {
    (current * (FULL_INK - alpha) + target * alpha) / FULL_INK
}

/// The part of a glyph that falls on the surface: the clipped box on the surface and where the walk starts in the
/// glyph's bitmap.
struct GlyphClip {
    x0: i16,
    x1: i16,
    y0: i16,
    y1: i16,
    /// The bitmap byte of (x0, y0) for the walk (the mirrored walk starts at the right end).
    source: usize,
}

/// Places a glyph at the pen (16.16) on the baseline (16.16) and clips it to the surface; None when nothing shows.
///
/// The box: x from (pen + (left + 32) × 1024) >> 16 over `WIDTH` pixels, y from (baseline + (32 - top) × 1024) >> 16
/// over `HEIGHT` rows (16-bit arithmetic as in the original). Outside the surface: None; otherwise the box is clamped
/// to it and the bitmap walk starts at the clamped row and at the clamped left column, or the right one when mirrored.
///
/// Defect of the original: a mirrored glyph cut by the right edge starts its walk from the clipped right column, so
/// its visible part is shifted.
unsafe fn font_clip_glyph(glyph: usize, surface: usize, pen: i32, baseline: i32, mirrored: bool) -> Option<GlyphClip> {
    let x = (read_i32(glyph + font_glyph::LEFT).wrapping_add(0x20) << 10).wrapping_add(pen) >> 16;
    let y = ((0x20i32.wrapping_sub(read_i32(glyph + font_glyph::TOP)) << 10).wrapping_add(baseline)) >> 16;
    let (x, y) = (x as i16, y as i16);
    let right = (read_u16(glyph + font_glyph::WIDTH) as i16).wrapping_add(x).wrapping_sub(1);
    let bottom = (read_u16(glyph + font_glyph::HEIGHT) as i16).wrapping_add(y).wrapping_sub(1);
    let (width, height) = (read_i16(surface + font_surface::WIDTH), read_i16(surface + font_surface::HEIGHT));
    if x >= width || right < 0 || y >= height || bottom < 0 {
        return None;
    }
    let clip = GlyphClip { x0: x.max(0), x1: right.min(width - 1), y0: y.max(0), y1: bottom.min(height - 1), source: 0 };
    let start = (if mirrored { clip.x1 } else { clip.x0 }) as i32;
    let row = (clip.y0 as i32 - y as i32).wrapping_mul(read_i32(glyph + font_glyph::ROW_BYTES));
    let source = read_usize(glyph + font_glyph::BITMAP).wrapping_add(row.wrapping_sub(x as i32).wrapping_add(start) as usize);
    Some(GlyphClip { source, ..clip })
}

/// Walks the clipped glyph: for each surface pixel (x, y) of the box, row by row, the glyph's coverage byte (stepping
/// backwards in the bitmap when mirrored) goes to `paint` with the pixel's address when it is not 0.
unsafe fn font_walk_glyph(glyph: usize, surface: usize, clip: &GlyphClip, step: isize, mut paint: impl FnMut(usize, i32, i16, i16)) {
    let (pixels, pitch) = (read_usize(surface + font_surface::PIXELS), read_i32(surface + font_surface::PITCH));
    let row_bytes = read_i32(glyph + font_glyph::ROW_BYTES) as isize;
    let mut row_source = clip.source as isize;
    for y in clip.y0..=clip.y1 {
        let line = pixels.wrapping_add(2 * (y as i32).wrapping_mul(pitch) as usize);
        let mut source = row_source;
        for x in clip.x0..=clip.x1 {
            let alpha = read_u8(source as usize) as i32;
            if alpha != 0 {
                paint(line.wrapping_add(2 * x as usize), alpha, x, y);
            }
            source += step;
        }
        row_source += row_bytes;
    }
}

/// The plain drawing: each pixel mixed towards the text colour by the coverage.
unsafe fn font_paint_blend(pixel: usize, alpha: i32, colour: (i32, i32, i32)) {
    let (r, g, b) = font_unpack(read_u16(pixel));
    write_u16(pixel, font_pack(font_mix(r, colour.0, alpha), font_mix(g, colour.1, alpha), font_mix(b, colour.2, alpha))); // sx10_14: the text pixel
}

/// Flag 0x40 (the shadow pass): each pixel mixed towards the average of itself and the colour.
unsafe fn font_paint_tint(pixel: usize, alpha: i32, colour: (i32, i32, i32)) {
    let (r, g, b) = font_unpack(read_u16(pixel));
    write_u16(pixel, font_pack(font_mix(r, (colour.0 + r) >> 1, alpha), font_mix(g, (colour.1 + g) >> 1, alpha),
        font_mix(b, (colour.2 + b) >> 1, alpha)));
}

/// Flag 0x20 (embossed): the pixel is lightened towards 7/5 of itself (capped at 255) and, when it is not on the
/// surface's first row or column, the pixel up and to the left darkened towards 3/5 of itself, both by the coverage.
unsafe fn font_paint_emboss(pixel: usize, alpha: i32, x: i16, y: i16, pitch: i32) {
    let light = |c: i32| font_mix(c, (c * 7 / 5).min(0xff), alpha);
    let (r, g, b) = font_unpack(read_u16(pixel));
    write_u16(pixel, font_pack(light(r), light(g), light(b)));
    if x != 0 && y != 0 {
        let corner = pixel.wrapping_add(2 * (-1 - pitch) as usize);
        let dark = |c: i32| font_mix(c, c * 3 / 5, alpha);
        let (r, g, b) = font_unpack(read_u16(corner));
        write_u16(corner, font_pack(dark(r), dark(g), dark(b)));
    }
}

/// Draws the zero-terminated 16-bit text `text` on a 15 or 16-bit surface with the FONT, alpha-blending each glyph from
/// the glyph cache; returns 1, or 0 when no font file is loaded.
///
/// `flags`: 0x80 first draws the text as a drop shadow (the same call with 0x80 off, 0x10 blur and 0x40 tint on, black,
/// no callback; every flag above bit 15 is dropped too), then the text itself with 0x80 off; otherwise the style and
/// size are set (`FONT::set_style_and_size`). 0x100 draws each glyph mirrored, 0x200 draws '|' as a space; 0x20 embosses
/// (`font_paint_emboss`), 0x40 tints (`font_paint_tint`), otherwise the colour blends (`font_paint_blend`). The colour is
/// a pixel of the display format (`font_unpack`).
///
/// The pen starts at (x, y) in 16.16 and the baseline is y plus the scaler's two vertical metrics. Each character below
/// 0x20 draws nothing and does not move the pen; any other gets its glyph (`FONT::get_cached_glyph` with the flags and
/// size); with a callback, the character's box (pen, y + metric B, pen + advance, y + metric A, each rounded) is offered
/// first and a non-zero answer skips the drawing. The glyph is clipped (`font_clip_glyph`), painted and the pen moves by
/// its advance. After the text, the callback (if any) gets the empty box at the pen.
/// Original: `FONT::draw_text` (0x5b0d80, fog_of_war.cpp, thiscall, ret 0x20).
pub(crate) unsafe extern "thiscall" fn font_draw_text(this: usize, surface: usize, x: i32, y: i32, flags: u32, size: u32, colour: u32, text: usize, callback: usize) -> u32 {
    let state = read_usize(this);
    if read_u32(state + font_state::LOADED) == 0 {
        return 0;
    }
    let flags = if flags & DRAW_SHADOW != 0 {
        let flags = flags & 0xff7f;
        covered!("sx10_14 text shadow");
        font_draw_text(this, surface, x, y, flags | 0x50, size, 0, text, 0);
        flags
    } else {
        font_set_style_via_exe(this, flags, size);
        flags
    };
    let scaler = read_usize(state + font_state::SCALER);
    let top = (y as i16 as i32) << 16;
    let baseline = read_i32(scaler + t2k_scaler::METRIC_B).wrapping_add(read_i32(scaler + t2k_scaler::METRIC_A)).wrapping_add(top);
    let round = |v: i32| v.wrapping_add(0x8000) >> 16;
    let metric = |field: usize| round(read_i32(read_usize(state + font_state::SCALER) + field).wrapping_add(top));
    let callback: Option<DrawTextCallback> = if callback != 0 { Some(core::mem::transmute(callback)) } else { None };
    let mirrored = flags & DRAW_MIRROR != 0;
    let paint_colour = font_unpack(colour as u16);
    let pitch = read_i32(surface + font_surface::PITCH);
    let mut pen = (x as i16 as i32) << 16;
    let mut k: i16 = 0;
    loop {
        let mut ch = read_u16(text + 2 * k as usize);
        if ch == 0 {
            break;
        }
        let mut advance = 0;
        if ch >= 0x20 {
            if flags & DRAW_BAR_AS_SPACE != 0 && ch == b'|' as u16 {
                ch = b' ' as u16;
            }
            let glyph = font_glyph_via_exe(this, flags, size, ch as u32);
            advance = read_i32(glyph + font_glyph::ADVANCE);
            let skipped = match callback {
                Some(f) => f(round(pen), metric(t2k_scaler::METRIC_B), round(advance.wrapping_add(pen)), metric(t2k_scaler::METRIC_A)) != 0,
                None => false,
            };
            if !skipped {
                if let Some(clip) = font_clip_glyph(glyph, surface, pen, baseline, mirrored) {
                    let step = if mirrored { -1 } else { 1 };
                    if flags & DRAW_EMBOSS != 0 {
                        font_walk_glyph(glyph, surface, &clip, step, |p, a, px, py| font_paint_emboss(p, a, px, py, pitch));
                    } else if flags & DRAW_TINT != 0 {
                        font_walk_glyph(glyph, surface, &clip, step, |p, a, _, _| font_paint_tint(p, a, paint_colour));
                    } else {
                        font_walk_glyph(glyph, surface, &clip, step, |p, a, _, _| font_paint_blend(p, a, paint_colour));
                    }
                }
            }
        }
        pen = pen.wrapping_add(advance);
        k = k.wrapping_add(1);
    }
    if let Some(f) = callback {
        f(round(pen), metric(t2k_scaler::METRIC_B), round(pen), metric(t2k_scaler::METRIC_A));
    }
    covered!("sx10_14 text drawn");
    1
}

/// Comparer for `font_draw_text`: side by side on the whole surface (the copy's glyphs all come from the cache the
/// original just filled, in the same final order). With a callback (it may record the boxes) or a surface without a
/// usable size, only the original runs.
// ponytail: copies the whole surface three times per text; fine for the comparison mode, a box from the metrics if it is too slow.
pub(crate) unsafe extern "thiscall" fn compare_font_draw_text(this: usize, surface: usize, x: i32, y: i32, flags: u32, size: u32, colour: u32, text: usize, callback: usize) -> u32 {
    let original = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32, i32, u32, u32, u32, usize, usize) -> u32>(%%I0%%);
    let pixels = read_usize(surface + font_surface::PIXELS);
    let bytes = 2 * read_i32(surface + font_surface::PITCH) as i64 * read_i16(surface + font_surface::HEIGHT) as i64;
    if callback != 0 || pixels == 0 || bytes <= 0 || bytes > 0x0400_0000 {
        return original(this, surface, x, y, flags, size, colour, text, callback);
    }
    let (o, r, equal, mo, mr) = crate::comparison::side_by_side(&[(pixels, bytes as usize)],
        || original(this, surface, x, y, flags, size, colour, text, callback),
        || font_draw_text(this, surface, x, y, flags, size, colour, text, callback));
    SCOREBOARD.record(equal && o == r, || format!("font_draw_text(at {},{} flags {:#x} size {} colour {:#x}): original {} copy {} {:?}",
        x as i16, y as i16, flags, size, colour as u16, o, r, first_differences(&mo, &mr)));
    o
}
