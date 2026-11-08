use super::*;

// The comparers of the routines in ../area.rs.
// [pt] Os comparadores das rotinas de ../area.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `area_layout`.
///
/// [pt] A área inteira, o +0x7c do item dono (se houver) e o ponteiro da barra de rolagem do teclado
/// (0xad9c58, que a 0x403600 grava) têm de sair iguais.
pub(crate) unsafe extern "thiscall" fn compare_area_layout(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT);
    let marks = *((area + 0x18) as *const u32);
    let columns = *((area + 0xbab) as *const u8);
    let rows = *((area + 0xbac) as *const u8);
    if marks & 0x80000 == 0 && columns != 0 && (columns > 30 || (rows != 0 && rows > 30)) {
        return o(area);
    }
    let mut reg = vec![(area, 0xbf1), (G_ACTIVE_AREA_SCROLLBAR, 4)];
    let owner = core::ptr::read_unaligned((area + 0x20e) as *const i16);
    if owner != -1 {
        reg.push((area_item_at(*((area + 0x208) as *const usize), owner) + 0x7c, 2));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(area), || area_layout(area));
    SCOREBOARD.record(igual, || format!("area_layout {:#x}: diferencas em {:?}", area, first_differences(&mo, &mr)));
}

/// Comparer for `init`.
pub(crate) unsafe extern "thiscall" fn compare_init(
    area: usize, areas: usize, items: usize, index: u32, x1: i32, y1: i32, x2: i32, y2: i32, columns: u32,
    pesos_colunas: usize, rows: u32, pesos_linhas: usize, marks: u32, colour: u32, owner: u32,
) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, i32, i32, i32, u32, usize, u32, usize, u32, u32, u32)>(IDX_INIT);
    let reg = [(area, 0xbcb + 0xff)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(
        &reg,
        || o(area, areas, items, index, x1, y1, x2, y2, columns, pesos_colunas, cap_area_rows(rows), pesos_linhas, marks, colour, owner),
        || init(area, areas, items, index, x1, y1, x2, y2, columns, pesos_colunas, rows, pesos_linhas, marks, colour, owner),
    );
    SCOREBOARD.record(igual, || format!("init {:#x}: diferencas em {:?}", area, first_differences(&mo, &mr)));
}

/// Comparer for `draw`.
pub(crate) unsafe extern "thiscall" fn compare_draw(area: usize, redraw: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redraw)
}
