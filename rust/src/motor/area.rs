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

/// Frees an area's saved images: the one behind the scroll bar and the one behind the area (`exe::gui_free_image`,
/// each pointer cleared), and clears the keyboard scroll bar ([`G_ACTIVE_AREA_SCROLLBAR`]) when it is this area's.
/// Original: `Area::release` (0x4031a0, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_release(area: usize) {
    covered!("tel29_2 area released");
    if read_usize(area + area_scroll::BAR_IMAGE) != 0 {
        exe::gui_free_image(read_usize(area + area_scroll::BAR_IMAGE));
        write_usize(area + area_scroll::BAR_IMAGE, 0);
    }
    if read_usize(area + area_fields::SAVED_IMAGE) != 0 {
        exe::gui_free_image(read_usize(area + area_fields::SAVED_IMAGE));
        write_usize(area + area_fields::SAVED_IMAGE, 0);
    }
    if read_usize(G_ACTIVE_AREA_SCROLLBAR) == area + area_fields::SCROLLBAR {
        write_usize(G_ACTIVE_AREA_SCROLLBAR, 0);
    }
}

/// Comparer for `area_release`.
pub(crate) unsafe extern "thiscall" fn compare_area_release(area: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_RELEASE)(area);
    let ok = read_usize(area + area_scroll::BAR_IMAGE) == 0 && read_usize(area + area_fields::SAVED_IMAGE) == 0
        && read_usize(G_ACTIVE_AREA_SCROLLBAR) != area + area_fields::SCROLLBAR;
    SCOREBOARD.record(ok, || format!("area_release {:#x}: images {:#x} {:#x}", area, read_usize(area + area_scroll::BAR_IMAGE), read_usize(area + area_fields::SAVED_IMAGE)));
}

/// Adds a widget to the area's list (`area_fields::ITEM_LIST`), kept sorted by the widget's row, and widens the area's
/// first and last rows.
///
/// Nothing for widget −1, with 1200 widgets already (`area_fields::ITEM_COUNT`, signed) or in a grid-only area
/// (`area_fields_flags::GRID_ONLY`). The count goes up; the first row (`area_scroll::FIRST_ROW`) drops to the widget's
/// row when lower, the last (`LAST_ROW`) rises to it when higher; the widget goes in after the last entry whose row is
/// not above its own (insertion from the end: equal rows keep their order).
/// Original: `Area::add_widget` (0x403200, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_add_widget(area: usize, widget: i16) {
    use area_scroll as s;
    let count = read_i16(area + area_fields::ITEM_COUNT);
    if widget == -1 || count >= 1200 || read_u32(area + area_fields::FLAGS) & area_fields_flags::GRID_ONLY != 0 {
        return;
    }
    covered!("tel29_2 widget added");
    let total = count.wrapping_add(1);
    let row = read_i32(area_widget(area, widget) + s::WIDGET_ROW);
    write_i16(area + area_fields::ITEM_COUNT, total);
    if row < read_i32(area + s::FIRST_ROW) {
        write_i32(area + s::FIRST_ROW, row);
    }
    if row > read_i32(area + s::LAST_ROW) {
        write_i32(area + s::LAST_ROW, row);
    }
    let list = |k: i16| (area as isize + area_fields::ITEM_LIST as isize + k as isize * 2) as usize;
    let mut k = total.wrapping_sub(2);
    while k >= 0 {
        let other = read_i16(list(k));
        if row >= read_i32(area_widget(area, other) + s::WIDGET_ROW) { // tel29_2: sorted by row, stable
            write_i16(list(k + 1), widget);
            return;
        }
        write_i16(list(k + 1), other);
        k -= 1;
    }
    write_i16(list(0), widget);
}

/// Comparer for `area_add_widget`: side by side over the area.
pub(crate) unsafe extern "thiscall" fn compare_area_add_widget(area: usize, widget: i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i16)>(IDX_AREA_ADD_WIDGET);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&[(area, area_scroll::AREA_SIZE)], || o(area, widget), || area_add_widget(area, widget));
    SCOREBOARD.record(equal, || format!("area_add_widget {:#x}({}): differences em {:?}", area, widget, first_differences(&mo, &mr)));
}

/// The row of the first visible widget (`area_fields::FIRST_VISIBLE_ITEM` in the list) when the area has a scroll bar
/// (`area_scroll::HAS_SCROLLBAR`), 0 when it has none.
/// Original: `Area::get_shown_row` (0x4032e0, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_get_shown_row(area: usize) -> u32 {
    if read_u32(area + area_scroll::HAS_SCROLLBAR) == 0 {
        return 0;
    }
    read_u32(area_list_item(area, read_i16(area + area_fields::FIRST_VISIBLE_ITEM)) + area_scroll::WIDGET_ROW)
}

/// Stores a requested scroll row (`area_scroll::PENDING_ROW`): at most the last row less the visible rows plus one
/// (`LAST_ROW` − `area_fields::ROW_COUNT` + 1), and at least 0.
/// Original: `Area::set_scroll_row` (0x403320, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_set_scroll_row(area: usize, row: i32) {
    let top = read_i32(area + area_scroll::LAST_ROW).wrapping_sub(read_u8(area + area_fields::ROW_COUNT) as i32).wrapping_add(1);
    let row = if row < top { row } else { top };
    write_i32(area + area_scroll::PENDING_ROW, if row > 0 { row } else { 0 }); // tel29_2: never above the first row
}

/// Comparer for `area_set_scroll_row`: side by side over the pending row.
pub(crate) unsafe extern "thiscall" fn compare_area_set_scroll_row(area: usize, row: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_AREA_SET_SCROLL_ROW);
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(area + area_scroll::PENDING_ROW, 4)], || o(area, row), || area_set_scroll_row(area, row));
    SCOREBOARD.record(equal, || format!("area_set_scroll_row({}) original={:x?} rust={:x?}", row, mo, mr));
}

/// Sets up the area's scroll bar inside margins `mx`, `my`: the bar, its track and thumb, a pending scroll row applied,
/// and the bar made the keyboard's.
///
/// The bar: x2 = area x2 − `mx`, x1 = x2 − 0x13, y1 = area y1 + `my`, y2 = area y2 − `my`; the track 0x14 inside the
/// arrow buttons. A pending row (`area_scroll::PENDING_ROW` ≠ −1) is clamped to 0..last row, the first visible widget
/// becomes the first whose row is not below it, and it is the scroll row (otherwise 0). Hidden rows = last row − visible
/// rows + 1; the thumb height = track × visible / (hidden + visible), at least 10; its top = track top + (track −
/// thumb + 1) × scroll row / hidden rows, its bottom = top + height − 1 (divisions truncating). The bar's saved image
/// pointer and the pressed arrow are cleared (the image is not freed), the area has a bar, and the keyboard scroll bar
/// ([`G_ACTIVE_AREA_SCROLLBAR`]) is this one.
/// Original: `Area::setup_scrollbar` (0x403600, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_setup_scrollbar(area: usize, mx: i32, my: i32) {
    use area_scroll as s;
    covered!("tel29_2 scroll bar set up");
    let x2 = read_i32(area + 8).wrapping_sub(mx);
    write_i32(area + s::BAR_X2, x2);
    write_i32(area + s::BAR_X1, x2.wrapping_sub(0x13));
    let y1 = read_i32(area + 4).wrapping_add(my);
    let y2 = read_i32(area + 0xc).wrapping_sub(my);
    write_i32(area + s::BAR_Y2, y2);
    let top = y1.wrapping_add(0x14);
    let bottom = y2.wrapping_sub(0x14);
    write_i32(area + s::BAR_Y1, y1);
    write_i32(area + s::TRACK_TOP, top);
    write_i32(area + s::TRACK_BOTTOM, bottom);
    if read_i32(area + s::PENDING_ROW) != -1 {
        if read_i32(area + s::PENDING_ROW) < 0 {
            write_i32(area + s::PENDING_ROW, 0);
        }
        if read_i32(area + s::PENDING_ROW) > read_i32(area + s::LAST_ROW) {
            write_i32(area + s::PENDING_ROW, read_i32(area + s::LAST_ROW));
        }
        let mut i: i16 = 0;
        while i < read_i16(area + area_fields::ITEM_COUNT) && read_i32(area_list_item(area, i) + s::WIDGET_ROW) < read_i32(area + s::PENDING_ROW) {
            i += 1;
        }
        write_i16(area + area_fields::FIRST_VISIBLE_ITEM, i);
        write_i32(area + s::SCROLL_ROW, read_i32(area + s::PENDING_ROW));
    } else {
        write_i32(area + s::SCROLL_ROW, 0);
    }
    let rows = read_u8(area + area_fields::ROW_COUNT) as i32;
    let hidden = read_i32(area + s::LAST_ROW).wrapping_sub(rows).wrapping_add(1);
    write_i32(area + s::HIDDEN_ROWS, hidden);
    let thumb = bottom.wrapping_sub(top).wrapping_add(1).wrapping_mul(rows).wrapping_div(hidden.wrapping_add(rows));
    let thumb = if thumb > 10 { thumb } else { 10 }; // tel29_2: the thumb at least 10 pixels
    write_i32(area + s::THUMB_HEIGHT, thumb);
    write_u32(area + s::BAR_IMAGE, 0);
    write_u8(area + s::PRESSED_ARROW, 0);
    write_u32(area + s::HAS_SCROLLBAR, 1);
    let thumb_top = bottom.wrapping_sub(top).wrapping_sub(thumb).wrapping_add(1).wrapping_mul(read_i32(area + s::SCROLL_ROW)).wrapping_div(hidden).wrapping_add(top);
    write_i32(area + s::THUMB_TOP, thumb_top);
    write_i32(area + s::THUMB_BOTTOM, thumb_top.wrapping_add(thumb).wrapping_sub(1));
    write_usize(G_ACTIVE_AREA_SCROLLBAR, area + area_fields::SCROLLBAR);
}

/// Comparer for `area_setup_scrollbar`: side by side over the area and the keyboard scroll bar.
pub(crate) unsafe extern "thiscall" fn compare_area_setup_scrollbar(area: usize, mx: i32, my: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(IDX_AREA_SETUP_SCROLLBAR);
    let regions = [(area, area_scroll::AREA_SIZE), (G_ACTIVE_AREA_SCROLLBAR, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side(&regions, || o(area, mx, my), || area_setup_scrollbar(area, mx, my));
    SCOREBOARD.record(equal, || format!("area_setup_scrollbar {:#x}({}, {}): differences em {:?}", area, mx, my, first_differences(&mo, &mr)));
}

/// Where a popup area opens (its top left corner in `*out_x`, `*out_y`), beside or below the widget it belongs to,
/// kept inside the screen.
///
/// The owner widget (`area_fields::OWNER_ITEM`) is w. When w's own area (`area_scroll::WIDGET_AREA`) is itself a popup
/// that does not open to the side (flag `POPUP_BESIDE`): vertically at w's top when the area fits below it (its height
/// + w's y1 below the screen height 600), otherwise ending at w's bottom (at least 0); horizontally right of w (x2 + 3)
/// when the parent popup's widget (x2) is left of the parent area (x1), otherwise left of w (w x1 − width − 3). With
/// `POPUP_BESIDE` on this area: vertically under w (y2 + 3) when it fits, otherwise above it (w y1 − height − 3);
/// horizontally at w's x1 when w's x1 ≤ 400, otherwise ending at w's x2. Otherwise: vertically at w's y1 when (the
/// height fits below it or `POPUP_KEEP_DOWN` is set) and `POPUP_ABOVE` is not set, else ending at w's y2 (at least 0);
/// horizontally right of w (x2 + 3) when (w's x1 ≤ 400 or `POPUP_RIGHT_ANYWAY`) and not `POPUP_LEFT`, else left of w
/// (w x1 − width − 3). The 600 is read from the exe (the 1080
/// exe rewrites it); the 400 is fixed in both. Without an owner widget: the original routine runs (error box `Area.cpp
/// 718`).
/// Original: `Area::get_popup_position` (0x403760, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_get_popup_position(area: usize, out_x: *mut i32, out_y: *mut i32) {
    use area_scroll as s;
    let owner = read_i16(area + area_fields::OWNER_ITEM);
    if owner == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32)>(IDX_AREA_GET_POPUP_POSITION)(area, out_x, out_y);
    }
    let w = area_widget(area, owner);
    let (x1, y1, x2, y2) = (read_i32(area), read_i32(area + 4), read_i32(area + 8), read_i32(area + 0xc));
    let height = y2.wrapping_sub(y1);
    let clamp0 = |v: i32| if v > 0 { v } else { 0 };
    let parent_index = read_i16(w + s::WIDGET_AREA);
    let parent = if parent_index != -1 { read_usize(area + area_fields::AREAS).wrapping_add((parent_index as isize * s::AREA_SIZE as isize) as usize) } else { 0 };
    if parent != 0 && read_i16(parent + area_fields::OWNER_ITEM) != -1 && read_u32(parent + area_fields::FLAGS) & s::POPUP_BESIDE == 0 {
        covered!("tel29_2 popup from a popup");
        let wy1 = read_i32(w + s::WIDGET_Y1);
        *out_y = if height.wrapping_add(wy1) < popup_screen_height(0) { wy1 } else { clamp0(read_i32(w + s::WIDGET_Y2).wrapping_sub(y2).wrapping_add(y1)) };
        let grand = area_widget(area, read_i16(parent + area_fields::OWNER_ITEM));
        *out_x = if read_i32(grand + s::WIDGET_X2) < read_i32(parent) {
            read_i32(w + s::WIDGET_X2).wrapping_add(3)
        } else {
            read_i32(w + s::WIDGET_X1).wrapping_sub(x2).wrapping_add(x1).wrapping_sub(3)
        };
        return;
    }
    let flags = read_u32(area + area_fields::FLAGS);
    if flags & s::POPUP_BESIDE != 0 {
        covered!("tel29_2 popup beside");
        let wy2 = read_i32(w + s::WIDGET_Y2);
        *out_y = if wy2.wrapping_sub(y1).wrapping_add(y2).wrapping_add(3) < popup_screen_height(1) {
            wy2.wrapping_add(3)
        } else {
            read_i32(w + s::WIDGET_Y1).wrapping_sub(y2).wrapping_add(y1).wrapping_sub(3)
        };
        let wx1 = read_i32(w + s::WIDGET_X1);
        *out_x = if wx1 <= 400 { wx1 } else { read_i32(w + s::WIDGET_X2).wrapping_sub(x2).wrapping_add(x1) };
        return;
    }
    covered!("tel29_2 popup below");
    let wy1 = read_i32(w + s::WIDGET_Y1);
    let fits = height.wrapping_add(wy1) < popup_screen_height(2);
    *out_y = if (fits || flags & s::POPUP_KEEP_DOWN != 0) && flags & s::POPUP_ABOVE == 0 {
        wy1
    } else {
        clamp0(read_i32(w + s::WIDGET_Y2).wrapping_sub(y2).wrapping_add(y1))
    };
    let wx1 = read_i32(w + s::WIDGET_X1);
    let flags = read_u32(area + area_fields::FLAGS);
    *out_x = if (wx1 <= 400 || flags & s::POPUP_RIGHT_ANYWAY != 0) && flags & s::POPUP_LEFT == 0 { // tel29_2: right of the widget
        read_i32(w + s::WIDGET_X2).wrapping_add(3)
    } else {
        wx1.wrapping_sub(x2).wrapping_add(x1).wrapping_sub(3)
    };
}

/// Comparer for `area_get_popup_position`: side by side over the two outputs.
pub(crate) unsafe extern "thiscall" fn compare_area_get_popup_position(area: usize, out_x: *mut i32, out_y: *mut i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut i32, *mut i32)>(IDX_AREA_GET_POPUP_POSITION);
    if read_i16(area + area_fields::OWNER_ITEM) == -1 {
        return o(area, out_x, out_y);
    }
    let regions = [(out_x as usize, 4), (out_y as usize, 4)];
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&regions, || o(area, out_x, out_y), || area_get_popup_position(area, out_x, out_y));
    SCOREBOARD.record(equal, || format!("area_get_popup_position {:#x}: original={:x?} rust={:x?}", area, mo, mr));
}

/// Moves the area's rectangle so its top left corner is at (`x`, `y`).
///
/// When the move would take it off the screen (x < 0, y < 0, or past the right or bottom edge, 0x31f and 0x257 in the
/// 800 × 600 exe, read from the exe since the 1080 exe rewrites them): the original routine runs (error boxes `Area.cpp
/// 823`, `828`, `833`, `838`, and the move clamped to the screen). Otherwise all four corners shift by the difference
/// (nothing written when it is zero).
/// Original: `Area::move_to` (0x403a70, Area.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn area_move_to(area: usize, x: i32, y: i32) {
    let dx = x.wrapping_sub(read_i32(area));
    let dy = y.wrapping_sub(read_i32(area + 4));
    if x < 0 || read_i32(area + 8).wrapping_add(dx) > (0x403b02 as *const i32).read_unaligned() || y < 0 || read_i32(area + 0xc).wrapping_add(dy) > (0x403bde as *const i32).read_unaligned() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(IDX_AREA_MOVE_TO)(area, x, y);
    }
    if dx == 0 && dy == 0 {
        return;
    }
    covered!("tel29_2 area moved");
    write_i32(area, read_i32(area).wrapping_add(dx));
    write_i32(area + 4, read_i32(area + 4).wrapping_add(dy));
    write_i32(area + 8, read_i32(area + 8).wrapping_add(dx));
    write_i32(area + 0xc, read_i32(area + 0xc).wrapping_add(dy)); // tel29_2: the bottom moves with the top
}

/// Comparer for `area_move_to`: side by side over the rectangle (off the screen, only the original: the boxes).
pub(crate) unsafe extern "thiscall" fn compare_area_move_to(area: usize, x: i32, y: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32)>(IDX_AREA_MOVE_TO);
    let dx = x.wrapping_sub(read_i32(area));
    let dy = y.wrapping_sub(read_i32(area + 4));
    if x < 0 || read_i32(area + 8).wrapping_add(dx) > (0x403b02 as *const i32).read_unaligned() || y < 0 || read_i32(area + 0xc).wrapping_add(dy) > (0x403bde as *const i32).read_unaligned() {
        return o(area, x, y);
    }
    let (_, _, equal, mo, mr) = crate::comparison::side_by_side_light(&[(area, 16)], || o(area, x, y), || area_move_to(area, x, y));
    SCOREBOARD.record(equal, || format!("area_move_to {:#x}({}, {}): original={:x?} rust={:x?}", area, x, y, mo, mr));
}

/// Draws the area's scroll bar: the up button with its arrow, the darkened track, the thumb and the down button with
/// its arrow, after saving the screen behind the bar.
///
/// Without a bar (`area_scroll::HAS_SCROLLBAR`): the original routine runs (error box `Area.cpp 870`). The image behind
/// the bar is saved once (`exe::gui_save_screen_image`, `BAR_IMAGE`). The up button: a panel from the bar's top to the
/// track top − 1 (`exe::gui_draw_panel`, style 0x70 and the arrow 2 pixels in when the up arrow is pressed,
/// `PRESSED_ARROW` = 1, otherwise 0x30), colour [`G_GUI_COLOUR_GREY`]; its arrow (`GLYPH_UP`, gui_draw_glyph 0x5e3ce0,
/// colour [`G_GUI_COLOUR_BLACK`]) centred in the 20-pixel button. The track (style 2, black), the thumb (`THUMB_TOP`..
/// `THUMB_BOTTOM`, style 0x30, grey); the down button from the track bottom + 1 to the bar's bottom (pressed = 2), its
/// arrow (`GLYPH_DOWN`, centred with the up arrow's width and height, as the original does).
/// Original: `Area::draw_scrollbar` (0x403c80, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_draw_scrollbar(area: usize) {
    use area_scroll as s;
    if read_u32(area + s::HAS_SCROLLBAR) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_DRAW_SCROLLBAR)(area);
    }
    covered!("tel29_2 scroll bar drawn");
    let r = |f: usize| read_i32(area + f);
    if read_usize(area + s::BAR_IMAGE) == 0 {
        write_usize(area + s::BAR_IMAGE, exe::gui_save_screen_image(r(s::BAR_X1), r(s::BAR_Y1), r(s::BAR_X2), r(s::BAR_Y2), 0usize));
    }
    // simplification: the original pushes the colours from 16-bit loads into registers with leftover high bits; here
    // zero-extended
    let grey = read_u16(G_GUI_COLOUR_GREY) as u32;
    let black = read_u16(G_GUI_COLOUR_BLACK) as u32;
    let centre = |size_byte: usize| (0x14 - read_u8(size_byte) as i32) / 2;
    let (off, style) = if read_u8(area + s::PRESSED_ARROW) == 1 { (2, 0x70) } else { (0, 0x30) };
    exe::gui_draw_panel(r(s::BAR_X1), r(s::BAR_Y1), r(s::BAR_X2), r(s::TRACK_TOP) - 1, style, grey, 0);
    exe::gui_draw_glyph(centre(s::GLYPH_UP) + r(s::BAR_X1) + off, centre(s::GLYPH_UP + 1) + r(s::BAR_Y1) + off, black, s::GLYPH_UP);
    exe::gui_draw_panel(r(s::BAR_X1), r(s::TRACK_TOP), r(s::BAR_X2), r(s::TRACK_BOTTOM), 2, black, 0);
    exe::gui_draw_panel(r(s::BAR_X1), r(s::THUMB_TOP), r(s::BAR_X2), r(s::THUMB_BOTTOM), 0x30, grey, 0);
    let (off, style) = if read_u8(area + s::PRESSED_ARROW) == 2 { (2, 0x70) } else { (0, 0x30) };
    exe::gui_draw_panel(r(s::BAR_X1), r(s::TRACK_BOTTOM) + 1, r(s::BAR_X2), r(s::BAR_Y2), style, grey, 0);
    exe::gui_draw_glyph(centre(s::GLYPH_UP) + r(s::BAR_X1) + off, centre(s::GLYPH_UP + 1) + r(s::TRACK_BOTTOM) + off + 1, black, s::GLYPH_DOWN);
}

/// Comparer for `area_draw_scrollbar`.
pub(crate) unsafe extern "thiscall" fn compare_area_draw_scrollbar(area: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_DRAW_SCROLLBAR)(area)
}

/// Opens a popup area: moves it to its popup position, marks it open, draws it and flips its rectangle.
///
/// The position comes from `Area::get_popup_position` (0x403760); when it differs from the area's x1/y1 the area is
/// moved there (`Area::move_to` 0x403a70). Then `area_fields::IS_OPEN` = 1, `Area::draw(area, 0)` and the flip of
/// (x1, y1, x2, y2).
/// Original: `Area::open_popup` (0x402d10, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_open_popup(area: usize) {
    covered!("tel37_2 open a popup");
    let (mut x, mut y) = (0i32, 0i32);
    exe::area_get_popup_position(area, &mut x, &mut y);
    if read_i32(area) != x || read_i32(area + 4) != y {
        exe::area_move_to(area, x, y);
    }
    write_u32(area + area_fields::IS_OPEN, 1);
    display_method1(0x402e70, area, 0);
    area_flip(area);
}

/// Closes a popup area that is open: unhighlights its widgets (closing the popups they own), puts back the screen
/// under it and flips its rectangle.
///
/// With `area_fields::IS_OPEN` set: from `area_fields::FIRST_VISIBLE_ITEM` (when not -1) to the end of the list each
/// widget gets `display_item::HIGHLIGHTED` = 0 and the popup area it owns (`guio::POPUP_AREA_INDEX`, when not -1) is
/// closed the same way. Then `IS_OPEN` = 0; with a saved image (`area_fields::SAVED_IMAGE`) it is put back at (x1, y1),
/// freed and cleared, and the rectangle is flipped.
/// Original: `Area::close_popup` (0x402d80, Area.cpp, thiscall).
pub(crate) unsafe extern "thiscall" fn area_close_popup(area: usize) {
    if read_u32(area + area_fields::IS_OPEN) == 0 {
        return;
    }
    covered!("tel37_2 close a popup");
    let mut k = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    if k != -1 && k < read_i16(area + area_fields::ITEM_COUNT) {
        loop {
            write_u32(area_list_item(area, k) + display_item::HIGHLIGHTED, 0);
            let popup = read_i16(area_list_item(area, k) + guio::POPUP_AREA_INDEX);
            if popup != -1 {
                area_close_popup(area_item_at_size(read_usize(area + area_fields::AREAS), popup, display_list::AREA_SIZE));
            }
            k = k.wrapping_add(1);
            if k >= read_i16(area + area_fields::ITEM_COUNT) {
                break;
            }
        }
    }
    let image = read_usize(area + area_fields::SAVED_IMAGE);
    write_u32(area + area_fields::IS_OPEN, 0);
    if image != 0 {
        display_restore_image(read_i32(area), read_i32(area + 4), image);
        exe::gui_free_image(read_usize(area + area_fields::SAVED_IMAGE));
        write_usize(area + area_fields::SAVED_IMAGE, 0);
        area_flip(area);
    }
}

/// Scrolls an area by `delta` rows: the new first visible widget, the visible widgets placed and drawn, the flip and
/// the screen manager told the new first row.
///
/// Nothing without rows (`area_fields::ROW_COUNT`) or widgets. The saved image (`area_fields::SAVED_IMAGE`) is put
/// back first. The new first position comes from `area_scroll_target`; when it is a position of the list, each widget
/// from it on whose row is within `ROW_COUNT` - 1 rows of the first gets the rectangle of `area_visible_item_rect` and
/// is drawn (`Guio::draw(item, 0)`). Then the flip of (x1, y1, x2 - 22, y2) and `ScreenManager` 0x800330 (the area
/// index and the first visible widget's row).
/// Original: `Area::scroll_rows` (0x403ee0, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_scroll_rows(area: usize, delta: i32) {
    if read_u8(area + area_fields::ROW_COUNT) == 0 || read_i16(area + area_fields::ITEM_COUNT) <= 0 {
        return;
    }
    covered!("tel37_2 scroll an area");
    let image = read_usize(area + area_fields::SAVED_IMAGE);
    if image != 0 {
        display_restore_image(read_i32(area), read_i32(area + 4), image);
    }
    let mut k = area_scroll_target(area, delta);
    write_i16(area + area_fields::FIRST_VISIBLE_ITEM, k);
    if k != -1 {
        while k < read_i16(area + area_fields::ITEM_COUNT) {
            let item = area_list_item(area, k);
            let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
            let distance = read_i32(item + display_item::ROW).wrapping_sub(area_row_at(area, first));
            if distance > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
                break;
            }
            let [x1, y1, x2, y2] = area_visible_item_rect(area, item, distance);
            write_i32(item + display_item::X1, x1);
            write_i32(item + display_item::X2, x2);
            write_i32(item + display_item::Y1, y1);
            write_i32(item + display_item::Y2, y2);
            exe::guio_draw(area_list_item(area, k), 0);
            k = k.wrapping_add(1);
            if k == -1 {
                break;
            }
        }
    }
    display_rect_call(0x5e2eb0, read_i32(area), read_i32(area + 4), read_i32(area + 8).wrapping_sub(0x16), read_i32(area + 0xc));
    let first_row = area_row_at(area, read_i16(area + area_fields::FIRST_VISIBLE_ITEM));
    exe::screenmanager_send_area_scroll_row(G_SCREEN_MANAGER, read_u16(area + area_fields::AREA_INDEX) as u32, first_row);
}

/// Converts the area colour (`area_fields::COLOUR`) to the screen's pixel format: the colour is split into red,
/// green and blue by the masks of the other format (`graphics` 0x5e49f0: 5-5-5 when `to_565` is 0, 5-6-5 otherwise)
/// and joined again by `graphics_rgb_to_pixel` (the current format).
/// Original: `Area::convert_colour_to_pixel_format` (0x4041d0, Area.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn area_convert_colour_to_pixel_format(area: usize, to_565: u32) {
    // the pixel format record: only the red, green and blue masks (+0x10, +0x14, +0x18) are read
    let mut format = [0u32; 7];
    (format[4], format[5]) = if to_565 != 0 { (0xf800, 0x7e0) } else { (0x7c00, 0x3e0) };
    format[6] = 0x1f;
    // the blue output is the argument's own slot in the original; red and green are locals
    let (mut red, mut green, mut blue) = (0u32, 0u32, to_565);
    exe::gui_colour_to_rgb(read_u16(area + area_fields::COLOUR) as u32, &mut red, &mut green, &mut blue, format.as_ptr());
    let pixel = exe::graphics_rgb_to_pixel(red, green, blue, 0);
    write_u16(area + area_fields::COLOUR, pixel as u16);
}

/// Comparer for `area_open_popup`: only the original (it draws); afterwards the area must be open at its popup
/// position (asked before, `Area::get_popup_position`).
pub(crate) unsafe extern "thiscall" fn compare_area_open_popup(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_OPEN_POPUP);
    let position: unsafe extern "thiscall" fn(usize, *mut i32, *mut i32) = core::mem::transmute(FN_AREA_GET_POPUP_POSITION);
    let (mut x, mut y) = (0i32, 0i32);
    position(area, &mut x, &mut y);
    o(area);
    let (ax, ay, open) = (read_i32(area), read_i32(area + 4), read_u32(area + area_fields::IS_OPEN));
    SCOREBOARD.record(ax == x && ay == y && open == 1, || format!("Area::open_popup (area {}): at ({}, {}) open {}, expected ({}, {}) open 1",
        read_i16(area + area_fields::AREA_INDEX), ax, ay, open, x, y));
}

/// Comparer for `area_close_popup`: only the original (it puts the screen back); afterwards the area must be closed,
/// its saved image gone and its widgets from the first visible one unhighlighted.
pub(crate) unsafe extern "thiscall" fn compare_area_close_popup(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_CLOSE_POPUP);
    let was_open = read_u32(area + area_fields::IS_OPEN) != 0;
    o(area);
    if !was_open {
        return;
    }
    let mut lit = Vec::new();
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    if first != -1 {
        let mut k = first;
        while k >= 0 && k < read_i16(area + area_fields::ITEM_COUNT) {
            if read_u32(area_list_item(area, k) + display_item::HIGHLIGHTED) != 0 {
                lit.push(k);
            }
            k += 1;
        }
    }
    let (open, image) = (read_u32(area + area_fields::IS_OPEN), read_usize(area + area_fields::SAVED_IMAGE));
    SCOREBOARD.record(open == 0 && image == 0 && lit.is_empty(), || format!("Area::close_popup (area {}): open {} image {:#x} still highlighted {:?}",
        read_i16(area + area_fields::AREA_INDEX), open, image, lit));
}

/// Comparer for `area_scroll_rows`: only the original (it draws); the plan (the new first visible position and the
/// rectangles of the visible widgets) is computed before and checked afterwards.
pub(crate) unsafe extern "thiscall" fn compare_area_scroll_rows(area: usize, delta: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32)>(IDX_AREA_SCROLL_ROWS);
    if read_u8(area + area_fields::ROW_COUNT) == 0 || read_i16(area + area_fields::ITEM_COUNT) <= 0 {
        return o(area, delta);
    }
    let target = area_scroll_target(area, delta);
    let mut plan = Vec::new();
    let count = read_i16(area + area_fields::ITEM_COUNT);
    if target >= 0 && target < count {
        let base_row = area_row_at(area, target);
        let mut k = target;
        while k < count {
            let item = area_list_item(area, k);
            let distance = read_i32(item + display_item::ROW).wrapping_sub(base_row);
            if distance > read_u8(area + area_fields::ROW_COUNT) as i32 - 1 {
                break;
            }
            plan.push((k, area_visible_item_rect(area, item, distance)));
            k += 1;
        }
    }
    o(area, delta);
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    let wrong: Vec<_> = plan.iter().filter_map(|&(k, r)| {
        let item = area_list_item(area, k);
        let got = [read_i32(item + display_item::X1), read_i32(item + display_item::Y1), read_i32(item + display_item::X2), read_i32(item + display_item::Y2)];
        (got != r).then_some((k, got, r))
    }).collect();
    SCOREBOARD.record(first == target && wrong.is_empty(), || format!("Area::scroll_rows (area {}, {} rows): first {} expected {}; rectangles (position, original, plan) {:?}",
        read_i16(area + area_fields::AREA_INDEX), delta, first, target, wrong));
}

/// Comparer for `area_convert_colour_to_pixel_format`: the area colour side by side.
pub(crate) unsafe extern "thiscall" fn compare_area_convert_colour_to_pixel_format(area: usize, to_565: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_AREA_CONVERT_COLOUR_TO_PIXEL_FORMAT);
    let reg = [(area + area_fields::COLOUR, 2)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&reg, || o(area, to_565), || area_convert_colour_to_pixel_format(area, to_565));
    SCOREBOARD.record(same, || format!("Area::convert_colour_to_pixel_format({}): {:?}", to_565, first_differences(&mo, &mr)));
}

/// The screen limits the 1920 × 1080 exe rewrites in these routines, read from the exe itself: the 600 height of the
/// popup tests (`cmp reg, imm` at 0x40384b, 0x403922, 0x4039b5) and the 0x31f / 0x257 right and bottom edges of
/// Area::move_to (0x403b00 and 0x403bdc). The 400 middle of the popup tests is not rewritten.
pub(crate) unsafe fn popup_screen_height(k: usize) -> i32 {
    let imm = match k { 0 => 0x40384d as *const i32, 1 => 0x403924 as *const i32, _ => 0x4039b7 as *const i32 };
    imm.read_unaligned()
}

/// A widget of the area's widget table (`area_fields::ITEMS`, 0x18c bytes each).
pub(crate) unsafe fn area_widget(area: usize, k: i16) -> usize {
    area_item_at(read_usize(area + area_fields::ITEMS), k)
}

/// The screen-flip call (`graphics_flip_rect` 0x5e2eb0) over an area's rectangle.
pub(crate) unsafe fn area_flip(area: usize) {
    display_rect_call(0x5e2eb0, read_i32(area), read_i32(area + 4), read_i32(area + 8), read_i32(area + 0xc));
}

/// Element `k` of a table of `size`-byte records at `base` (signed index).
pub(crate) fn area_item_at_size(base: usize, k: i16, size: usize) -> usize {
    (base as isize).wrapping_add(k as isize * size as isize) as usize
}

/// The row (`display_item::ROW`) of the widget at list position `k`.
pub(crate) unsafe fn area_row_at(area: usize, k: i16) -> i32 {
    read_i32(area_list_item(area, k) + display_item::ROW)
}

/// The list position that becomes the first visible one after scrolling `delta` rows from `area_fields::FIRST_VISIBLE_ITEM`.
///
/// Down (`delta` > 0): the first position whose row is at least `delta` rows past the current first one (or the last
/// position). Up (otherwise): the last position going back whose row is at most `delta` rows from the current first one
/// (or position 0), then back to the first position of that row (equal rows go together).
pub(crate) unsafe fn area_scroll_target(area: usize, delta: i32) -> i16 {
    let count = read_i16(area + area_fields::ITEM_COUNT);
    let first = read_i16(area + area_fields::FIRST_VISIBLE_ITEM);
    let mut k = first;
    let distance = |k: i16| area_row_at(area, k).wrapping_sub(area_row_at(area, first));
    if delta > 0 {
        if distance(k) < delta {
            loop {
                k = k.wrapping_add(1);
                if k >= count {
                    k = k.wrapping_sub(1);
                    break;
                }
                if distance(k) >= delta {
                    break;
                }
            }
        }
        return k;
    }
    if distance(k) > delta {
        loop {
            k = k.wrapping_sub(1);
            if k < 0 {
                k = k.wrapping_add(1);
                break;
            }
            if distance(k) <= delta {
                break;
            }
        }
    }
    while k as i32 - 1 != -1 && area_row_at(area, k.wrapping_sub(1)) == area_row_at(area, k) {
        k = k.wrapping_sub(1);
    }
    k
}

/// The rectangle (x1, y1, x2, y2) of a widget shown `distance` rows below the first visible one: its fixed x and y
/// when set (`guio_rect::FIXED_X1`, `FIXED_Y1`), else its column's bounds and the row `AREA_FIRST_ROW` + `distance`
/// of the area's grid, from the area's x1/y1.
pub(crate) unsafe fn area_visible_item_rect(area: usize, item: usize, distance: i32) -> [i32; 4] {
    let (x1, x2) = if read_i32(item + guio_rect::FIXED_X1) != 0 {
        (read_i32(item + guio_rect::FIXED_X1), read_i32(item + guio_rect::FIXED_X2))
    } else {
        let column = read_i32(item + guio_rect::COLUMN) as isize;
        let left = read_i32(area);
        (read_i32((area as isize + area_fields::COLUMN_STARTS as isize + column * 4) as usize).wrapping_add(left),
         read_i32((area as isize + area_fields::COLUMN_ENDS as isize + column * 4) as usize).wrapping_add(left))
    };
    let (y1, y2) = if read_i32(item + guio_rect::FIXED_Y1) != 0 {
        (read_i32(item + guio_rect::FIXED_Y1), read_i32(item + guio_rect::FIXED_Y2))
    } else {
        let row = read_i32(area + AREA_FIRST_ROW).wrapping_add(distance) as isize;
        let top = read_i32(area + 4);
        (read_i32((area as isize + area_fields::ROW_STARTS as isize + row * 4) as usize).wrapping_add(top),
         read_i32((area as isize + area_fields::ROW_ENDS as isize + row * 4) as usize).wrapping_add(top))
    };
    [x1, y1, x2, y2]
}

/// i32: the first row of the area's grid that the widgets use (`Area::init` sets 0x7fffffff, `add_widget` lowers it).
pub(crate) const AREA_FIRST_ROW: usize = 0x10;
