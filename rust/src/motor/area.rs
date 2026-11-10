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
///
/// [pt] O item `k` (i16) da tabela de itens `base` (0x18c bytes cada).
pub(crate) fn area_item_at(base: usize, k: i16) -> usize {
    (base as isize).wrapping_add(k as isize * 0x18c) as usize
}

/// Item at position i of the area's item list (+0x212).
///
/// [pt] O item da posição `i` (i16) da lista da área (+0x212).
pub(crate) unsafe fn area_list_item(area: usize, i: i16) -> usize {
    let k = core::ptr::read_unaligned((area as isize + i as isize * 2 + 0x212) as *const i16);
    area_item_at(*((area + 0x208) as *const usize), k)
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
/// It writes the area index into the owner widget (`+0x7c`, if there is one; and clears `+0xbe9`) and clears the first
/// visible widget. Outside grid-only areas (flag `0x80000`), when there are columns and rows, it computes the margins
/// (flag 1: 0; otherwise 2 at the sides and, at top and bottom, 2 with flag `0x10` or a single row, 8 for the others),
/// turns on the scrollbar (`0x403600`) when the widgets exceed the rows, and splits the width and the height by the
/// weights.
///
/// More than 30 columns or rows: the original routine runs (error box, `Area.cpp 497` and `507`).
/// Original: `Area::layout` (0x403350, Area.cpp, thiscall).
///
/// [pt] { 0x403350 } (thiscall, Area.cpp): organiza a grade da área. Grava o índice da área no item dono
/// (+0x7c, se houver; e zera +0xbe9) e zera o primeiro item à mostra; fora das áreas só de grade
/// (marca 0x80000), com colunas e linhas, calcula as margens (marca 1: 0; senão 2 nas laterais e, em
/// cima e embaixo, 2 com a marca 0x10 ou uma linha só, 8 nas outras), liga a barra de rolagem (0x403600)
/// quando os itens passam das linhas e reparte a largura e a altura pelos pesos. Mais de 30 colunas
/// ou linhas: a original (caixa de erro, Area.cpp 497 e 507).
pub(crate) unsafe extern "thiscall" fn area_layout(area: usize) {
    let marks = *((area + 0x18) as *const u32);
    let columns = *((area + 0xbab) as *const u8);
    let rows = *((area + 0xbac) as *const u8);
    if marks & 0x80000 == 0 && columns != 0 && (columns > 30 || (rows != 0 && rows > 30)) {
        // simplification: the original writes the owner and +0xb76 before the error; repeating is harmless (same values)
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT)(area);
    }
    crate::comparison::branch("screens arrange the area");
    let owner = core::ptr::read_unaligned((area + 0x20e) as *const i16);
    if owner != -1 {
        *((area + 0xbe9) as *mut u32) = 0;
        let item = area_item_at(*((area + 0x208) as *const usize), owner);
        core::ptr::write_unaligned((item + 0x7c) as *mut u16, core::ptr::read_unaligned((area + 0xb72) as *const u16));
    }
    core::ptr::write_unaligned((area + 0xb76) as *mut i16, 0);
    if marks & 0x80000 != 0 || columns == 0 || rows == 0 {
        return;
    }
    let (lado, cima): (i32, i32) = if marks & 1 != 0 {
        (0, 0)
    } else if marks & 0x10 != 0 || rows == 1 {
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
    let n = *((area + 0xbab) as *const u8) as i32 - 1;
    let sum = weights(0xbad, (n + 1) as usize);
    let mut accumulated = 0i32;
    let mut c = 0i32;
    while c <= n {
        accumulated = accumulated.wrapping_add(*((area as isize + c as isize + 0xbad) as *const u8) as i32);
        if c == 0 {
            write_i32_at(area, 0x1c, lado);
        } else {
            write_i32_at(area, c as isize * 4 + 0x1c, read_i32_at(area, c as isize * 4 + 0x108).wrapping_add(2));
        }
        if c < *((area + 0xbab) as *const u8) as i32 - 1 {
            write_i32_at(area, c as isize * 4 + 0x10c, accumulated.wrapping_mul(width).wrapping_div(sum).wrapping_add(lado - 1));
        } else {
            write_i32_at(area, c as isize * 4 + 0x10c, width.wrapping_add(lado - 1));
        }
        c += 1;
        if c > *((area + 0xbab) as *const u8) as i32 - 1 {
            break;
        }
    }
    // rows: +0x94 start and +0x184 end, by weights +0xbcb
    let n = *((area + 0xbac) as *const u8) as i32 - 1;
    let sum = weights(0xbcb, (n + 1) as usize);
    let mut accumulated = 0i32;
    let mut l = 0i32;
    while l <= n {
        accumulated = accumulated.wrapping_add(*((area as isize + l as isize + 0xbcb) as *const u8) as i32);
        if l == 0 {
            write_i32_at(area, 0x94, cima);
        } else {
            write_i32_at(area, l as isize * 4 + 0x94, read_i32_at(area, l as isize * 4 + 0x180).wrapping_add(2));
        }
        let end = if l < *((area + 0xbac) as *const u8) as i32 - 1 {
            accumulated.wrapping_mul(height).wrapping_div(sum).wrapping_add(cima - 1)
        } else {
            height.wrapping_add(cima - 1)
        };
        write_i32_at(area, l as isize * 4 + 0x184, end);
        l += 1;
        if l > *((area + 0xbac) as *const u8) as i32 - 1 {
            break;
        }
    }
}

/// Initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights.
///
/// An area that opens on top of a widget (`dono` ≠ -1): weight 1 in all 30 columns and rows, zero rectangle, 1 column
/// and 0 rows. For the others: it orders x1/x2 and y1/y2 (swap `0x90ba90`), x1/y1 at least 0 and x2/y2 at most the
/// screen (`0x31f`/`0x257` in the 800x600 exe; the 1920x1080 exe replaces these immediates, which are read here from
/// the exe itself). It writes the fields, clears the scrollbar and, outside grid-only areas (`0x80000`), copies the
/// weights (with 1..29 columns/rows and a pointer) or fills them with 1 (also with 30 or more: in that case the
/// original overruns the 30-entry array, and so does this code). The visible rows are clamped at 30 (the grid size);
/// the rest goes to the scrollbar.
/// Original: `Area::init` (0x402ac0, Area.cpp, thiscall).
///
/// [pt] { 0x402ac0 } (thiscall, Area.cpp): inicia a área. Área que abre por cima de um item (`dono` ≠ -1):
/// pesos 1 nas 30 colunas e linhas, retângulo zero, 1 coluna e 0 linhas. Nas outras: ordena x1/x2 e
/// y1/y2 (troca 0x90ba90), x1/y1 no mínimo 0 e x2/y2 no máximo a tela (0x31f/0x257 no exe de 800x600; o
/// exe de 1920x1080 troca esses imediatos, lidos aqui do próprio exe). Grava os campos, zera a barra
/// de rolagem e, fora das áreas só de grade (0x80000), copia os pesos (com 1..29 colunas/linhas e
/// ponteiro) ou enche de 1 (inclusive com 30 ou mais: aí a original passa do vetor de 30, como aqui).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn init(
    area: usize, areas: usize, items: usize, index: u32, x1: i32, y1: i32, x2: i32, y2: i32, columns: u32,
    weights_columns: usize, rows: u32, weights_rows: usize, marks: u32, colour: u32, owner: u32,
) {
    crate::comparison::branch("screens start the area");
    let rows = cap_area_rows(rows);
    let (mut x1, mut y1, mut x2, mut y2) = (x1, y1, x2, y2);
    let (mut columns, mut weights_columns, mut rows, mut weights_rows) = (columns as u8, weights_columns, rows as u8, weights_rows);
    if owner as u16 as i16 != -1 {
        core::ptr::write_bytes((area + 0xbad) as *mut u8, 1, 30);
        core::ptr::write_bytes((area + 0xbcb) as *mut u8, 1, 30);
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
    let x2 = if x2 >= core::ptr::read_unaligned(0x402b7e as *const i32) { core::ptr::read_unaligned(0x402b85 as *const i32) } else { x2 };
    let ceiling_y = core::ptr::read_unaligned(0x402b8e as *const i32);
    let y2 = if y2 >= ceiling_y { ceiling_y } else { y2 };
    write_i32_at(area, 0, x1);
    write_i32_at(area, 4, y1);
    write_i32_at(area, 8, x2);
    write_i32_at(area, 0xc, y2);
    *((area + 0xbac) as *mut u8) = rows;
    *((area + 0xbab) as *mut u8) = columns;
    write_i32_at(area, 0x10, 0x7fffffff);
    write_i32_at(area, 0x14, 0);
    core::ptr::write_bytes((area + 0xb7a) as *mut u8, 0, 0x31);
    core::ptr::write_unaligned((area + 0x20e) as *mut u16, owner as u16);
    core::ptr::write_unaligned((area + 0x204) as *mut usize, areas);
    core::ptr::write_unaligned((area + 0x208) as *mut usize, items);
    core::ptr::write_unaligned((area + 0xb72) as *mut u16, index as u16);
    core::ptr::write_unaligned((area + 0x20c) as *mut u16, colour as u16);
    write_i32_at(area, 0xbed, 0);
    write_i32_at(area, 0x200, 0);
    write_i32_at(area, 0xba6, 0);
    core::ptr::write_unaligned((area + 0x210) as *mut i16, -1);
    write_i32_at(area, 0xbe9, 1);
    core::ptr::write_unaligned((area + 0xb76) as *mut i16, -1);
    core::ptr::write_unaligned((area + 0x18) as *mut u32, marks);
    write_i32_at(area, 0x1fc, -1);
    if marks & 0x80000 != 0 {
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
///
/// [pt] A trava das 30 linhas: a grade da área tem no máximo 30 linhas (vetor fixo em +0xbcb; o Area::layout
/// mostra a caixa "Area.cpp 507" acima disso, e o Area::init já escreve os pesos fora do vetor). Em
/// 1920 × 1080 o relatório de uma partida antiga pede mais; travadas em 30, o resto vai para a barra de
/// rolagem, como em 800 × 600 (onde nunca passa de 30 e a trava não muda nada). Fica no Area::init porque
/// o começo do Display::add_area (0x55fba0) já é desvio do exe de 1080 (escala das coordenadas).
// simplification: the 30 rows spread over the height of the area (taller rows in 1080); layout by the
// real resolution is left for the modernization (PLANO_LIMITE.md); the original and the copy get the lock
// equal in the comparer, so the comparison remains valid
/// Caps an area grid's row count (low byte) at 30, the fixed array size; extra rows go to the scroll bar.
pub(crate) fn cap_area_rows(rows: u32) -> u32 {
    if rows & 0xff > 30 {
        crate::comparison::branch("screens area lines locked at 30");
        (rows & !0xff) | 30
    } else {
        rows
    }
}

/// Draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar.
///
/// With `redesenhar`, it frees the saved image (`0x5e42b0`); an area that opens on top of a widget saves the background
/// (`0x5e3dd0`); it draws the panel (`0x5e5df0`). Grid-only area (`0x80000`): the grid lines (`0x5e3730`, colour
/// `[0xae317e]`) and that is all. For the others: it saves the scrollbar background, places each visible widget in its
/// cell (the grids `+0x1c`/`+0x10c` and `+0x94`/`+0x184`, when the widget has no rectangle of its own at
/// `+0x28`/`+0x30`) and draws it (`0x5ee590`) until it passes the last row; then the scrollbar (`0x403c80`).
///
/// Without a first visible widget (`+0xb76` = -1): the original routine runs (error box `Area.cpp 245` and the layout
/// `0x403350`).
/// Original: `Area::draw` (0x402e70, Area.cpp, thiscall).
///
/// [pt] { 0x402e70 } (thiscall, Area.cpp): desenha a área. Com `redesenhar`, libera a imagem guardada
/// (0x5e42b0); área que abre por cima guarda o fundo (0x5e3dd0); desenha o painel (0x5e5df0). Área só
/// de grade (0x80000): as linhas da grade (0x5e3730, cor [0xae317e]) e fim. Nas outras: guarda o fundo
/// da barra de rolagem, põe cada item à mostra na sua célula (as grades +0x1c/+0x10c e +0x94/+0x184,
/// quando o item não tem retângulo próprio em +0x28/+0x30) e o desenha (0x5ee590) até passar da última
/// linha; depois a barra (0x403c80). Sem o primeiro item à mostra (+0xb76 = -1): a original (caixa de
/// erro Area.cpp 245 e a organização 0x403350).
pub(crate) unsafe extern "thiscall" fn draw(area: usize, redraw: u32) {
    if core::ptr::read_unaligned((area + 0xb76) as *const i16) == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redraw);
    }
    crate::comparison::branch("screens draw the area");
    let image = (area + 0x200) as *mut usize;
    if redraw != 0 && *image != 0 {
        exe::gui_free_image(*image);
        *image = 0;
    }
    if core::ptr::read_unaligned((area + 0x20e) as *const i16) != -1 && *image == 0 {
        *image = exe::gui_save_screen_image(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8), read_i32_at(area, 0xc), 0);
    }
    // simplification: the original pushes edx with the colour in dx and, on top, leftovers from earlier calls
    // (unpredictable); the color is u16 for the panel, here zero-extended
    let colour = core::ptr::read_unaligned((area + 0x20c) as *const u16) as u32;
    exe::gui_draw_panel(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8), read_i32_at(area, 0xc), *((area + 0x18) as *const u32), colour, 0);
    if *((area + 0x18) as *const u32) & 0x80000 != 0 {
        let mut k: i16 = 1;
        while *((area + 0xbac) as *const u8) > 1 && k < *((area + 0xbac) as *const u8) as i16 {
            let y1 = read_i32_at(area, 4);
            let y = read_i32_at(area, 0xc).wrapping_sub(y1).wrapping_add(1).wrapping_mul(k as i32) / *((area + 0xbac) as *const u8) as i32;
            let y = y.wrapping_add(y1);
            exe::graphics_draw_line(read_i32_at(area, 0), y, read_i32_at(area, 8), y, 2, *(G_GUI_COLOUR_GREY as *const u16) as u32);
            k = k.wrapping_add(1);
        }
        let mut k: i16 = 1;
        while *((area + 0xbab) as *const u8) > 1 && k < *((area + 0xbab) as *const u8) as i16 {
            let x1 = read_i32_at(area, 0);
            let x = read_i32_at(area, 8).wrapping_sub(x1).wrapping_add(1).wrapping_mul(k as i32) / *((area + 0xbab) as *const u8) as i32;
            let x = x.wrapping_add(x1);
            exe::graphics_draw_line(x, read_i32_at(area, 4), x, read_i32_at(area, 0xc), 2, *(G_GUI_COLOUR_GREY as *const u16) as u32);
            k = k.wrapping_add(1);
        }
        return;
    }
    if read_i32_at(area, 0xbed) != 0 && *image == 0 {
        *image = exe::gui_save_screen_image(read_i32_at(area, 0), read_i32_at(area, 4), read_i32_at(area, 8).wrapping_sub(0x16), read_i32_at(area, 0xc), 0);
    }
    let total = || core::ptr::read_unaligned((area + 0xb74) as *const i16);
    let mut i = core::ptr::read_unaligned((area + 0xb76) as *const i16);
    let mut parar = false;
    while i < total() && !parar {
        let item = area_list_item(area, i);
        let first = area_list_item(area, core::ptr::read_unaligned((area + 0xb76) as *const i16));
        let d = read_i32_at(item, 0x24).wrapping_sub(read_i32_at(first, 0x24));
        if d > *((area + 0xbac) as *const u8) as i32 - 1 {
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
///
/// [pt] O retângulo do item na tela (+0x10 x1, +0x14 y1, +0x18 x2, +0x1c y2): o próprio (+0x28..+0x34)
/// ou, onde for 0, a célula da grade (coluna +0x20; linha = primeira linha da área + `d`).
pub(crate) unsafe fn position_area_item(area: usize, item: usize, d: i32) {
    let (mut x1, mut x2, mut y1, mut y2) = (read_i32_at(item, 0x28), read_i32_at(item, 0x2c), read_i32_at(item, 0x30), read_i32_at(item, 0x34));
    if x1 == 0 {
        let c = read_i32_at(item, 0x20) as isize;
        x1 = read_i32_at(area, c * 4 + 0x1c).wrapping_add(read_i32_at(area, 0));
        x2 = read_i32_at(area, c * 4 + 0x10c).wrapping_add(read_i32_at(area, 0));
    }
    if y1 == 0 {
        let l = read_i32_at(area, 0x10).wrapping_add(d) as isize;
        y1 = read_i32_at(area, l * 4 + 0x94).wrapping_add(read_i32_at(area, 4));
        y2 = read_i32_at(area, l * 4 + 0x184).wrapping_add(read_i32_at(area, 4));
    }
    write_i32_at(item, 0x10, x1);
    write_i32_at(item, 0x18, x2);
    write_i32_at(item, 0x14, y1);
    write_i32_at(item, 0x1c, y2);
}

// simplification: desenhar guarda e libera imagens (aloca) e escurece o fundo (desenhar duas vezes escurece
// twice): in comparison mode only the original runs; the copy is proven in Lucas’s game in Rust mode
// (counter ”telas desenhar a area” in ddraw.log)

