use super::*;

// The comparers of the routines in ../area.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../area.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `area_layout` (comparison mode: see the header of motor.rs).
///
/// [pt] A área inteira, o +0x7c do item dono (se houver) e o ponteiro da barra de rolagem do teclado
/// (0xad9c58, que a 0x403600 grava) têm de sair iguais.
pub(crate) unsafe extern "thiscall" fn comparar_area_layout(area: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT);
    let marcas = *((area + 0x18) as *const u32);
    let colunas = *((area + 0xbab) as *const u8);
    let linhas = *((area + 0xbac) as *const u8);
    if marcas & 0x80000 == 0 && colunas != 0 && (colunas > 30 || (linhas != 0 && linhas > 30)) {
        return o(area);
    }
    let mut reg = vec![(area, 0xbf1), (G_ACTIVE_AREA_SCROLLBAR, 4)];
    let dono = core::ptr::read_unaligned((area + 0x20e) as *const i16);
    if dono != -1 {
        reg.push((area_item_at(*((area + 0x208) as *const usize), dono) + 0x7c, 2));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(area), || area_layout(area));
    PLACAR.registrar(igual, || format!("area_layout {:#x}: diferencas em {:?}", area, first_differences(&mo, &mr)));
}

/// Comparer for `init` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init(
    area: usize, areas: usize, itens: usize, indice: u32, x1: i32, y1: i32, x2: i32, y2: i32, colunas: u32,
    pesos_colunas: usize, linhas: u32, pesos_linhas: usize, marcas: u32, cor: u32, dono: u32,
) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, i32, i32, i32, u32, usize, u32, usize, u32, u32, u32)>(IDX_INIT);
    let reg = [(area, 0xbcb + 0xff)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || o(area, areas, itens, indice, x1, y1, x2, y2, colunas, pesos_colunas, cap_area_rows(linhas), pesos_linhas, marcas, cor, dono),
        || init(area, areas, itens, indice, x1, y1, x2, y2, colunas, pesos_colunas, linhas, pesos_linhas, marcas, cor, dono),
    );
    PLACAR.registrar(igual, || format!("init {:#x}: diferencas em {:?}", area, first_differences(&mo, &mr)));
}

/// Comparer for `draw` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_draw(area: usize, redesenhar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redesenhar)
}
