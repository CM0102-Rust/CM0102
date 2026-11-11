use super::*;

// The area (0xbf1 bytes, 249 per Display starting at Display+4; created by list 0x55fba0): +0 x1,
// +4 y1, +8 x2, +0xc y2; +0x10 the first and +0x14 the last row of the items; +0x18 the marks; the
// grids of at most 30 columns and 30 rows: +0x1c start and +0x10c end of the columns, +0x94 start and
// +0x184 end of the rows (i32, relative to x1/y1); +0x1fc the row requested for scrolling; +0x200 the
// saved image; +0x204 the areas; +0x208 the items (0x18c bytes each); +0x20c the colour; +0x20e the item
// owner (areas that open on top, -1 in the others); +0x212 the item list (1200 i16); +0xb72 the
// area index; +0xb74 item count; +0xb76 the first visible item; +0xb7a..0xbab the scroll
// scrolling; +0xbab columns and +0xbac rows (u8); +0xbad the column weights and +0xbcb the row weights
// (30 u8 each); +0xbe9 open; +0xbed has a scroll bar.

/// Address of item k (0x18c bytes each) in the item table base.
pub(crate) fn area_item_at(base: usize, k: i16) -> usize {
    (base as isize).wrapping_add(k as isize * 0x18c) as usize
}

/// Item at position i of the area's item list (+0x212).
pub(crate) unsafe fn area_list_item(area: usize, i: i16) -> usize {
    let k = core::ptr::read_unaligned((area as isize + i as isize * 2 + (area_fields::ITEM_LIST as isize)) as *const i16);
    area_item_at(*((area + area_fields::ITEMS) as *const usize), k)
}

/// Reads an unaligned i32 at area + offset.
pub(crate) unsafe fn read_i32_at(area: usize, desl: isize) -> i32 {
    core::ptr::read_unaligned((area as isize).wrapping_add(desl) as *const i32)
}

/// Writes an unaligned i32 at area + offset.
pub(crate) unsafe fn write_i32_at(area: usize, desl: isize, v: i32) {
    core::ptr::write_unaligned((area as isize).wrapping_add(desl) as *mut i32, v)
}

/// Lays out the area grid: splits the area width and height by the column and row weights and sets up the scrollbar.
///
/// It writes the area index (`area_fields::AREA_INDEX`) into the owner widget (`guio::POPUP_AREA_INDEX`, if there is
/// one; and clears `area_fields::IS_OPEN`) and clears the first visible widget (`area_fields::FIRST_VISIBLE_ITEM`).
/// Outside grid-only areas (`area_fields_flags::GRID_ONLY`), when there are columns and rows, it computes the margins
/// (`area_fields_flags::NO_MARGINS`: 0; otherwise 2 at the sides and, at top and bottom, 2 with
/// `area_fields_flags::THIN_VERTICAL_MARGINS` or a single row, 8 for the others), turns on the scrollbar
/// (`exe::area_setup_scrollbar`) when the widgets exceed the rows, and splits the width and the height by the weights.
///
/// More than 30 columns or rows: the original routine runs (error box, `Area.cpp 497` and `507`).
/// Original: `Area::layout` (0x403350, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_layout(area: usize) {
    let marks = *((area + area_fields::FLAGS) as *const u32);
    let columns = *((area + area_fields::COLUMN_COUNT) as *const u8);
    let rows = *((area + area_fields::ROW_COUNT) as *const u8);
    if marks & area_fields_flags::GRID_ONLY == 0 && columns != 0 && (columns > 30 || (rows != 0 && rows > 30)) {
        // simplification: the original writes the owner and +0xb76 before the error; repeating is harmless (same values)
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT)(area);
    }
    covered!("screens arrange the area");
    let owner = core::ptr::read_unaligned((area + area_fields::OWNER_ITEM) as *const i16);
    if owner != -1 {
        *((area + area_fields::IS_OPEN) as *mut u32) = 0;
        let item = area_item_at(*((area + area_fields::ITEMS) as *const usize), owner);
        core::ptr::write_unaligned((item + guio::POPUP_AREA_INDEX) as *mut u16, core::ptr::read_unaligned((area + area_fields::AREA_INDEX) as *const u16));
    }
    core::ptr::write_unaligned((area + area_fields::FIRST_VISIBLE_ITEM) as *mut i16, 0);
    if marks & area_fields_flags::GRID_ONLY != 0 || columns == 0 || rows == 0 {
        return;
    }
    let (lado, cima): (i32, i32) = if marks & area_fields_flags::NO_MARGINS != 0 {
        (0, 0)
    } else if marks & area_fields_flags::THIN_VERTICAL_MARGINS != 0 || rows == 1 {
        (2, 2)
    } else {
        (2, 8)
    };
    let mut width = read_i32_at(area, 8).wrapping_sub(2 * lado).wrapping_sub(read_i32_at(area, 0)).wrapping_add(1);
    let height = (-2 * cima).wrapping_sub(read_i32_at(area, 4)).wrapping_add(read_i32_at(area, 0xc)).wrapping_add(1);
    if read_i32_at(area, 0x14).wrapping_add(1) > rows as i32 {
        exe::area_setup_scrollbar(area, lado, cima);
        width = width.wrapping_sub(0x16);
    }
    // columns: +0x1c start and +0x10c end, by weights +0xbad
    let weights = |desl: usize, n: usize| (0..n).map(|k| *((area + desl + k) as *const u8) as i32).fold(0i32, |a, b| a.wrapping_add(b));
    let n = *((area + area_fields::COLUMN_COUNT) as *const u8) as i32 - 1;
    let sum = weights(area_fields::COLUMN_WEIGHTS, (n + 1) as usize);
    let mut accumulated = 0i32;
    let mut c = 0i32;
    while c <= n {
        accumulated = accumulated.wrapping_add(*((area as isize + c as isize + (area_fields::COLUMN_WEIGHTS as isize)) as *const u8) as i32);
        if c == 0 {
            write_i32_at(area, 0x1c, lado);
        } else {
            write_i32_at(area, c as isize * 4 + (area_fields::COLUMN_STARTS as isize), read_i32_at(area, c as isize * 4 + 0x108).wrapping_add(2));
        }
        if c < *((area + area_fields::COLUMN_COUNT) as *const u8) as i32 - 1 {
            write_i32_at(area, c as isize * 4 + (area_fields::COLUMN_ENDS as isize), accumulated.wrapping_mul(width).wrapping_div(sum).wrapping_add(lado - 1));
        } else {
            write_i32_at(area, c as isize * 4 + (area_fields::COLUMN_ENDS as isize), width.wrapping_add(lado - 1));
        }
        c += 1;
        if c > *((area + area_fields::COLUMN_COUNT) as *const u8) as i32 - 1 {
            break;
        }
    }
    // rows: +0x94 start and +0x184 end, by weights +0xbcb
    let n = *((area + area_fields::ROW_COUNT) as *const u8) as i32 - 1;
    let sum = weights(area_fields::ROW_WEIGHTS, (n + 1) as usize);
    let mut accumulated = 0i32;
    let mut l = 0i32;
    while l <= n {
        accumulated = accumulated.wrapping_add(*((area as isize + l as isize + (area_fields::ROW_WEIGHTS as isize)) as *const u8) as i32);
        if l == 0 {
            write_i32_at(area, 0x94, cima);
        } else {
            write_i32_at(area, l as isize * 4 + (area_fields::ROW_STARTS as isize), read_i32_at(area, l as isize * 4 + 0x180).wrapping_add(2));
        }
        let end = if l < *((area + area_fields::ROW_COUNT) as *const u8) as i32 - 1 {
            accumulated.wrapping_mul(height).wrapping_div(sum).wrapping_add(cima - 1)
        } else {
            height.wrapping_add(cima - 1)
        };
        write_i32_at(area, l as isize * 4 + (area_fields::ROW_ENDS as isize), end);
        l += 1;
        if l > *((area + area_fields::ROW_COUNT) as *const u8) as i32 - 1 {
            break;
        }
    }
}

/// Initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights.
///
/// An area that opens on top of a widget (`owner` ≠ -1): weight 1 in all 30 columns and rows, zero rectangle, 1 column
/// and 0 rows. For the others: it orders x1/x2 and y1/y2 (swap `exe::utils_swap_bytes`), x1/y1 at least 0 and x2/y2 at
/// most the screen (`0x31f`/`0x257` in the 800x600 exe; the 1920x1080 exe replaces these immediates, which are read
/// here from the exe itself). It writes the fields, clears the scrollbar (`area_fields::SCROLLBAR`) and, outside
/// grid-only areas (`area_fields_flags::GRID_ONLY`), copies the weights (with 1..29 columns/rows and a pointer) or
/// fills them with 1 (also with 30 or more: in that case the original overruns the 30-entry array, and so does this
/// code). The visible rows are clamped at 30 (the grid size); the rest goes to the scrollbar.
/// Original: `Area::init` (0x402ac0, Area.cpp, thiscall).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn init(
    area: usize, areas: usize, items: usize, index: u32, x1: i32, y1: i32, x2: i32, y2: i32, columns: u32,
    weights_columns: usize, rows: u32, weights_rows: usize, marks: u32, colour: u32, owner: u32,
) {
    covered!("screens start the area");
    let rows = cap_area_rows(rows);
    let (mut x1, mut y1, mut x2, mut y2) = (x1, y1, x2, y2);
    let (mut columns, mut weights_columns, mut rows, mut weights_rows) = (columns as u8, weights_columns, rows as u8, weights_rows);
    if owner as u16 as i16 != -1 {
        core::ptr::write_bytes((area + area_fields::COLUMN_WEIGHTS) as *mut u8, 1, 30);
        core::ptr::write_bytes((area + area_fields::ROW_WEIGHTS) as *mut u8, 1, 30);
        x1 = 0;
        y1 = 0;
        x2 = 0;
        y2 = 0;
        columns = 1;
        rows = 0;
        weights_columns = 0;
        weights_rows = 0;
    } else {
        if x1 > x2 {
            exe::utils_swap_bytes(&mut x1, &mut x2, 4);
        }
        if y1 > y2 {
            exe::utils_swap_bytes(&mut y1, &mut y2, 4);
        }
    }
    let x1 = if x1 > 0 { x1 } else { 0 };
    let y1 = if y1 > 0 { y1 } else { 0 };
    // the screen limits: immediates of `cmp edx, imm` and `mov edx, imm` (0x402b7c) and `mov esi, imm` (0x402b8d)
    let x2 = if x2 >= read_i32(0x402b7e) { read_i32(0x402b85) } else { x2 };
    let ceiling_y = read_i32(0x402b8e);
    let y2 = if y2 >= ceiling_y { ceiling_y } else { y2 };
    write_i32_at(area, 0, x1);
    write_i32_at(area, 4, y1);
    write_i32_at(area, 8, x2);
    write_i32_at(area, 0xc, y2);
    write_u8(area + area_fields::ROW_COUNT, rows);
    write_u8(area + area_fields::COLUMN_COUNT, columns);
    write_i32_at(area, 0x10, 0x7fffffff);
    write_i32_at(area, 0x14, 0);
    core::ptr::write_bytes((area + area_fields::SCROLLBAR) as *mut u8, 0, 0x31);
    write_u16(area + area_fields::OWNER_ITEM, owner as u16);
    write_usize(area + area_fields::AREAS, areas);
    write_usize(area + area_fields::ITEMS, items);
    write_u16(area + area_fields::AREA_INDEX, index as u16);
    write_u16(area + area_fields::COLOUR, colour as u16);
    write_i32_at(area, 0xbed, 0);
    write_i32_at(area, 0x200, 0);
    write_i32_at(area, 0xba6, 0);
    write_i16(area + 0x210, -1);
    write_i32_at(area, 0xbe9, 1);
    write_i16(area + area_fields::FIRST_VISIBLE_ITEM, -1);
    write_u32(area + area_fields::FLAGS, marks);
    write_i32_at(area, 0x1fc, -1);
    if marks & area_fields_flags::GRID_ONLY != 0 {
        return;
    }
    for (n, pesos, desl) in [(columns, weights_columns, 0xbad), (rows, weights_rows, 0xbcb)] {
        if pesos != 0 && n != 0 && n < 30 {
            core::ptr::copy(pesos as *const u8, (area + desl) as *mut u8, n as usize);
        } else {
            core::ptr::write_bytes((area + desl) as *mut u8, 1, n as usize);
        }
    }
}

/// A área e os 0xd9 bytes depois dela (os pesos de 30 ou mais linhas passam do fim) têm de sair iguais.
#[allow(clippy::too_many_arguments)]
/// The 30-row cap: an area's grid has at most 30 rows (fixed array at +0xbcb; Area::layout shows the "Area.cpp 507" box
/// above that, and Area::init already writes the weights outside the array).
///
/// At 1920 × 1080 the report of an old match asks for more; capped at 30, the rest goes to the scroll bar, as at 800 ×
/// 600 (where it never exceeds 30 and the cap changes nothing).
///
/// It lives in Area::init because the start of Display::add_area (0x55fba0) is already a detour of the 1080 exe
/// (coordinate scaling).
// simplification: the 30 rows spread over the height of the area (taller rows in 1080); layout by the
// real resolution is left for the modernization (PLANO_LIMITE.md); the original and the copy get the lock
// equal in the comparer, so the comparison remains valid
/// Caps an area grid's row count (low byte) at 30, the fixed array size; extra rows go to the scroll bar.
pub(crate) fn cap_area_rows(rows: u32) -> u32 {
    if rows & 0xff > 30 {
        covered!("screens area lines locked at 30");
        (rows & !0xff) | 30
    } else {
        rows
    }
}

/// Draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar.
///
/// With `redraw`, it frees the saved image (`exe::gui_free_image`); an area that opens on top of a widget saves the
/// background (`exe::gui_save_screen_image`); it draws the panel (`exe::gui_draw_panel`). Grid-only area
/// (`area_fields_flags::GRID_ONLY`): the grid lines (`exe::graphics_draw_line`, colour `G_GUI_COLOUR_GREY`) and that is
/// all. For the others: it saves the scrollbar background, places each visible widget in its cell (the grids
/// `area_fields::COLUMN_STARTS`/`area_fields::COLUMN_ENDS` and `area_fields::ROW_STARTS`/`area_fields::ROW_ENDS`, when
/// the widget has no rectangle of its own at `+0x28`/`+0x30`) and draws it (`exe::guio_draw`) until it passes the last
/// row; then the scrollbar (`exe::area_draw_scrollbar`).
///
/// Without a first visible widget (`area_fields::FIRST_VISIBLE_ITEM` = -1): the original routine runs (error box
/// `Area.cpp 245` and the layout `0x403350`).
/// Original: `Area::draw` (0x402e70, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn draw(area: usize, redraw: u32) {
    if read_i16(area + area_fields::FIRST_VISIBLE_ITEM) == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redraw);
    }
    covered!("screens draw the area");
    let image = (area + area_fields::SAVED_IMAGE) as *mut usize;
    if redraw != 0 && *image != 0 {
        exe::gui_free_image(*image);
        *image = 0;
    }
    if read_i16(area + area_fields::OWNER_ITEM) != -1 && *image == 0 {
        *image = exe::gui_save_screen_image(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8), read_i32_at(area, 0xc), 0);
    }
    // simplification: the original pushes edx with the colour in dx and, on top, leftovers from earlier calls
    // (unpredictable); the color is u16 for the panel, here zero-extended
    let colour = read_u16(area + area_fields::COLOUR) as u32;
    exe::gui_draw_panel(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8), read_i32_at(area, 0xc), read_u32(area + area_fields::FLAGS), colour, 0);
    if read_u32(area + area_fields::FLAGS) & area_fields_flags::GRID_ONLY != 0 {
        let mut k: i16 = 1;
        while read_u8(area + area_fields::ROW_COUNT) > 1 && k < read_u8(area + area_fields::ROW_COUNT) as i16 {
            let y1 = read_i32_at(area, 4);
            let y = read_i32_at(area, 0xc).wrapping_sub(y1).wrapping_add(1).wrapping_mul(k as i32) / read_u8(area + area_fields::ROW_COUNT) as i32;
            let y = y.wrapping_add(y1);
            exe::graphics_draw_line(read_i32_at(area, 0), y, read_i32_at(area, 8), y, 2, read_u16(G_GUI_COLOUR_GREY) as u32);
            k = k.wrapping_add(1);
        }
        let mut k: i16 = 1;
        while read_u8(area + area_fields::COLUMN_COUNT) > 1 && k < read_u8(area + area_fields::COLUMN_COUNT) as i16 {
            let x1 = read_i32_at(area, 0);
            let x = read_i32_at(area, 8).wrapping_sub(x1).wrapping_add(1).wrapping_mul(k as i32) / read_u8(area + area_fields::COLUMN_COUNT) as i32;
            let x = x.wrapping_add(x1);
            exe::graphics_draw_line(x, read_i32_at(area, 4), x, read_i32_at(area, 0xc), 2, read_u16(G_GUI_COLOUR_GREY) as u32);
            k = k.wrapping_add(1);
        }
        return;
    }
    if read_i32_at(area, 0xbed) != 0 && *image == 0 {
        *image = exe::gui_save_screen_image(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8).wrapping_sub(0x16), read_i32_at(area, 0xc), 0);
    }
    let total = || read_i16(area + area_fields::ITEM_COUNT);
    let mut i = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    let mut parar = false;
    while i < total() && !parar {
        let item = area_list_item(area, i);
        let first = area_list_item(area, read_i16(area + area_fields::FIRST_VISIBLE_ITEM));
        let d = read_i32_at(item, 0x24).wrapping_sub(read_i32_at(first, 0x24));
        if d > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
            parar = true;
        } else {
            position_area_item(area, item, d);
            exe::guio_draw(area_list_item(area, i), redraw);
        }
        i = i.wrapping_add(1);
    }
    if read_i32_at(area, 0xbed) != 0 {
        exe::area_draw_scrollbar(area);
    }
}

/// Sets the item's screen rectangle (+0x10..+0x1c): its own (+0x28..+0x34) or, where 0, the grid cell (column +0x20,
/// row = area's first row + d).
pub(crate) unsafe fn position_area_item(area: usize, item: usize, d: i32) {
    let (mut x1, mut x2, mut y1, mut y2) = (read_i32_at(item, 0x28), read_i32_at(item, 0x2c), read_i32_at(item, 0x30), read_i32_at(item, 0x34));
    if x1 == 0 {
        let c = read_i32_at(item, 0x20) as isize;
        x1 = read_i32_at(area, c * 4 + (area_fields::COLUMN_STARTS as isize)).wrapping_add(read_i32_at(area, 0));
        x2 = read_i32_at(area, c * 4 + (area_fields::COLUMN_ENDS as isize)).wrapping_add(read_i32_at(area, 0));
    }
    if y1 == 0 {
        let l = read_i32_at(area, 0x10).wrapping_add(d) as isize;
        y1 = read_i32_at(area, l * 4 + (area_fields::ROW_STARTS as isize)).wrapping_add(read_i32_at(area, 4));
        y2 = read_i32_at(area, l * 4 + (area_fields::ROW_ENDS as isize)).wrapping_add(read_i32_at(area, 4));
    }
    write_i32_at(item, 0x10, x1);
    write_i32_at(item, 0x18, x2);
    write_i32_at(item, 0x14, y1);
    write_i32_at(item, 0x1c, y2);
}

// simplification: desenhar guarda e libera imagens (aloca) e escurece o fundo (desenhar duas vezes escurece
// twice): in comparison mode only the original runs; the copy is proven in Lucas’s game in Rust mode
// (counter ”telas desenhar a area” in ddraw.log)

