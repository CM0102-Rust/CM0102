use super::*;

// The comparers of the routines in ../ruling_body/ruling_body.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../ruling_body/ruling_body.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `make_points_ban` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado nos 0x70 bytes do registro (a rotina só grava nele).
pub(crate) unsafe extern "thiscall" fn comparar_make_points_ban(this: usize, r: usize, j: usize, by_games: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_MAKE_POINTS_BAN);
    let regions = [(r, 0x70)];
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&regions, || o(this, r, j, by_games), || make_points_ban(this, r, j, by_games));
    PLACAR.registrar(igual && a == b, || format!("make_points_ban(jogador {:x}, por_jogos {}) original={} rust={} {:x?} x {:x?}", read_u32_as_usize(j), by_games, a, b, mo, mr));
    a
}

/// Comparer for `ruling_body_start_new_season` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_ruling_body_start_new_season(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RULING_BODY_START_NEW_SEASON)(this);
}

/// Comparer for `compact_entries` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original realoca (não pode rodar duas vezes): as entradas ocupadas copiadas antes; roda só a
/// original e confere a conta, a capacidade e as entradas na nova ordem.
pub(crate) unsafe extern "thiscall" fn comparar_compact_entries(this: usize) {
    let mut expected: Vec<u8> = Vec::new();
    let base = read_u32_as_usize(this + 0xb);
    for k in 0..read_i32(this + 0x13).max(0) as usize {
        let e = base + 0xe * k;
        if read_i32(e) != -1 {
            expected.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 0xe));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_COMPACT_ENTRIES)(this);
    let n = (expected.len() / 0xe) as u32;
    let p = read_u32_as_usize(this + 0xb);
    let iguais = p != 0 && core::slice::from_raw_parts(p as *const u8, expected.len()) == &expected[..];
    PLACAR.registrar(read_u32_as_usize(this + 0x13) as u32 == n && read_u32_as_usize(this + 0xf) as u32 == n + 100 && iguais, || {
        format!("compact_entries: conta {} (esperava {}), capacidade {}, entradas iguais {}", read_u32_as_usize(this + 0x13), n, read_u32_as_usize(this + 0xf), iguais)
    });
}
