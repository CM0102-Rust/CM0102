// ---------- cx07_13: the graphics base: the interface colours, rectangles, the grey box and the fonts (goldcup.cpp by the map; the graphics library) ----------

/// The interface colours with fixed 565/555 values (`INTERFACE_COLOURS_FIXED`: the address, the 565 value, the 555
/// value) and those packed from red, green and blue (`INTERFACE_COLOURS_RGB`), in the order the original sets them.
const INTERFACE_COLOURS_FIXED: [(usize, u16, u16); 9] = [
    (G_GUI_COLOUR_BLACK, 0, 0), (0xae31a2, 0xffff, 0x7fff), (G_GUI_COLOUR_PALE_GREY, 0xe71c, 0x739c), (G_GUI_COLOUR_RED, 0xf800, 0x7c00),
    (0xaebdcc, 0x8000, 0x4000), (0xaebdd4, 0xfc00, 0x7e00), (0xae3190, 0x8200, 0x4100), (G_GUI_COLOUR_YELLOW, 0xffe0, 0x7fe0),
    (0xaebdee, 0xfff0, 0x7ff0),
];
const INTERFACE_COLOURS_RGB: [(usize, u8, u8, u8); 16] = [
    (0xae31aa, 0, 0x80, 0), (0xae3150, 0, 0x40, 0), (0xae3152, 0, 0xff, 0xff), (0xaebde8, 0x80, 0xff, 0xff),
    (G_GUI_COLOUR_BLUE, 0, 0, 0xff), (0xaebdde, 0, 0x80, 0xff), (G_GUI_COLOUR_DARK_BLUE, 0, 0, 0x80), (0xaebdd6, 0, 0, 0x60),
    (0xae3148, 0x80, 0, 0x80), (0xae3192, 0x40, 0, 0x40), (0xae31a0, 0x80, 0x40, 0x40), (G_GUI_COLOUR_GREY, 0x80, 0x80, 0x80),
    (0xaebddc, 0xe0, 0xe0, 0xe0), (0xae3154, 0xc0, 0xc0, 0xc0), (0xae317c, 0x40, 0x40, 0x40), (0xae3140, 0, 0x80, 0x80),
]; // cx07_13: the packed interface colours

/// Sets the global interface colour words for the screen's pixel format once the screen exists: the fixed ones by the
/// format (565 when the screen's green mask is 0x7e0, otherwise 555), the others packed from red, green and blue
/// (`graphics_rgb_to_pixel` with the screen's format). Nothing when the display is off.
/// Original: `gui_init_palette_colours` (0x5e4560, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_init_palette_colours() {
    if read_u32(DISPLAY_OFF) != 0 {
        return;
    }
    let is_565 = read_u32(SCREEN_PIXEL_FORMAT + 0x14) == 0x7e0;
    for &(colour, v565, v555) in INTERFACE_COLOURS_FIXED.iter() {
        write_u16(colour, if is_565 { v565 } else { v555 });
    }
    for &(colour, r, g, b) in INTERFACE_COLOURS_RGB.iter() {
        write_u16(colour, graphics_rgb_to_pixel(r as u32, g as u32, b as u32, 0));
    }
}

/// Draws the rectangle (x0, y0)-(x1, y1) on the back buffer: with bit 0 of `style` its outline as four dashed lines
/// (graphics_draw_line 0x5e3730, cdecl: top, right, bottom and left, style 1), else with bit 1 as four lines of style 2;
/// otherwise the box filled with `colour` (when there is a back buffer and the box lies fully on the screen,
/// `clip_rect_to_surface`): the back buffer's Blt (slot 0x14) with DDBLT_COLORFILL | DDBLT_WAIT, the box one past on the
/// right and bottom, and a DDBLTFX with only its size (0x64) and fill colour (+0x50, the colour's low word) set (the
/// original leaves the rest of it as stack leftovers; the copy clears it). Nothing when the display is off.
/// Original: `graphics_draw_rect` (0x5e3b50, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn graphics_draw_rect(x0: i32, y0: i32, x1: i32, y1: i32, style: u32, colour: u32) {
    if read_u32(DISPLAY_OFF) != 0 {
        return;
    }
    let line: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32) = core::mem::transmute(FN_GRAPHICS_DRAW_LINE);
    for (bit, line_style) in [(1u8, 1u32), (2, 2)] {
        if style as u8 & bit != 0 {
            line(x0, y0, x1, y0, line_style, colour);
            line(x1, y0, x1, y1, line_style, colour);
            line(x1, y1, x0, y1, line_style, colour);
            line(x0, y1, x0, y0, line_style, colour);
            return;
        }
    }
    let back = read_usize(BACK_BUFFER);
    if back == 0 {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return;
    }
    covered!("cx07_13 filled rectangle");
    let mut fx = [0u8; 0x64];
    write_u32(fx.as_mut_ptr() as usize, 0x64);
    write_u32(fx.as_mut_ptr() as usize + 0x50, colour & 0xffff); // the fill colour
    rect[2] += 1;
    rect[3] += 1;
    let blt: unsafe extern "system" fn(usize, *const [i32; 4], usize, usize, u32, *const u8) -> i32 = core::mem::transmute(read_usize(read_usize(back) + 0x14));
    blt(back, &rect, 0, 0, 0x1000400, fx.as_ptr());
}

/// Turns every pixel of the box (x0, y0)-(x1, y1) of the back buffer grey: each pixel's red, green and blue
/// (`unpack_pixel` with the screen's format) averaged ((r + g + b) / 3) and packed again into all three channels
/// (`pack_pixel`); a pixel met while the display is off becomes 0. Nothing when the display is off, without a back
/// buffer, when the box does not lie fully on the screen or when the lock (0x21) fails.
/// Original: `gui_greyscale_rect` (0x5e7f60, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_greyscale_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    if read_u32(DISPLAY_OFF) != 0 || read_usize(BACK_BUFFER) == 0 {
        return;
    }
    let mut rect = [0i32; 4];
    if clip_rect_to_surface(x0, y0, x1, y1, &mut rect) == 0 {
        return;
    }
    let mut desc = [0u8; SURFACE_DESC_SIZE];
    if !lock_back_buffer(&rect, &mut desc, 0x21) {
        return;
    }
    let pitch = read_i32(desc.as_ptr() as usize + SURFACE_DESC_PITCH) / 2;
    let mut row = read_usize(desc.as_ptr() as usize + SURFACE_DESC_SURFACE);
    for _ in rect[1]..=rect[3] {
        let mut pixel = row;
        for _ in rect[0]..=rect[2] {
            let grey = if read_u32(DISPLAY_OFF) != 0 {
                0
            } else {
                let [r, g, b] = unpack_pixel(read_u16(pixel) as u32, SCREEN_PIXEL_FORMAT);
                let average = ((r as i32 + g as i32 + b as i32) / 3) as u8; // cx07_13: the grey of a pixel
                pack_pixel(average, average, average, SCREEN_PIXEL_FORMAT)
            };
            write_u16(pixel, grey);
            pixel += 2;
        }
        row = row.wrapping_add(((pitch as isize) * 2) as usize);
    }
    unlock_back_buffer();
}

/// Loads the game's fonts: the font mode `font_mode` set (font_set_mode 0x5b1f30, cdecl), then the four T2K font sizes
/// (FONT::load_font_file 0x5b0b60, thiscall, ret 4: the objects at 0xaebdf4, 0xaebdf8, 0xaebdfc and 0xaebe00 with the
/// sizes 3, 0, 1 and 2), then the seven bitmap font files of SI_DATA (`exe::utils_build_file_path`("SI_DATA", name) and
/// gui_load_bitmap_font 0x5e4ba0, cdecl(slot, path), slots 1 to 7). Returns 0 at the first failure (and 0 when the display
/// is off), 1 when all loaded.
/// Original: `gui_load_fonts` (0x5e4a60, goldcup.cpp, cdecl).
pub(crate) unsafe extern "C" fn gui_load_fonts(font_mode: i32) -> u32 {
    if read_u32(DISPLAY_OFF) != 0 {
        return 0;
    }
    let set_mode: unsafe extern "C" fn(i32) = core::mem::transmute(0x5b1f30usize); // font_set_mode
    set_mode(font_mode);
    let load_t2k: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5b0b60usize); // FONT::load_font_file
    for (font, size) in [(0xaebdf4usize, 3u32), (0xaebdf8, 0), (0xaebdfc, 1), (0xaebe00, 2)] {
        let r = load_t2k(font, size);
        if r == 0 {
            return r;
        }
    }
    let load_bitmap: unsafe extern "C" fn(u32, usize) -> u32 = core::mem::transmute(0x5e4ba0usize); // gui_load_bitmap_font
    let files = [0x9cda30usize, 0x9cda1c, 0x9cda0c, 0x9cd9fc, 0x9cd9ec, 0x9cd9d4, 0x9cd9bc]; // arial_narrow_10, arial_narrow_11, arial_14, arial_16, arial_18, trade_cond_24_bold, trade_cond_28_bold
    for (k, &file) in files.iter().enumerate() {
        let r = load_bitmap(k as u32 + 1, exe::utils_build_file_path(0x9c3718, file)); // "SI_DATA"
        if r == 0 {
            return r;
        }
    }
    covered!("cx07_13 fonts loaded");
    1
}

/// Comparers: they write the screen's colours (the palette, compared side by side over the 25 words), draw or load
/// (only the original).
pub(crate) unsafe extern "C" fn compare_gui_init_palette_colours() {
    let words = || -> Vec<u16> {
        INTERFACE_COLOURS_FIXED.iter().map(|c| c.0).chain(INTERFACE_COLOURS_RGB.iter().map(|c| c.0)).map(|a| read_u16(a)).collect()
    };
    original_fn::<unsafe extern "C" fn()>(%%I0%%)();
    let original = words();
    gui_init_palette_colours();
    let copy = words();
    SCOREBOARD.record(original == copy, || format!("gui_init_palette_colours: original {:x?} rust {:x?}", original, copy));
}

pub(crate) unsafe extern "C" fn compare_graphics_draw_rect(x0: i32, y0: i32, x1: i32, y1: i32, style: u32, colour: u32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32, u32, u32)>(%%I1%%)(x0, y0, x1, y1, style, colour)
}

pub(crate) unsafe extern "C" fn compare_gui_greyscale_rect(x0: i32, y0: i32, x1: i32, y1: i32) {
    original_fn::<unsafe extern "C" fn(i32, i32, i32, i32)>(%%I2%%)(x0, y0, x1, y1)
}

pub(crate) unsafe extern "C" fn compare_gui_load_fonts(font_mode: i32) -> u32 {
    original_fn::<unsafe extern "C" fn(i32) -> u32>(%%I3%%)(font_mode)
}
