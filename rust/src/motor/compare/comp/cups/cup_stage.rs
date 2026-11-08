use super::*;

// The comparers of the routines in ../comp/cups/cup_stage.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/cups/cup_stage.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_cup_stage` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (a base, as alocações e a árvore, que sorteia: não roda duas vezes), conferindo o plano feito antes: devolve
/// this, +0x3e = n, +0x3c = as rodadas, +0x40 = o ano, as duas tabelas montadas e, na lista, os mesmos clubes.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_construct_cup_stage(this: usize, parent: usize, n: u32, teams: usize, rounds: u32, a5: u32, modelo: usize, year: u32, stage: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, por_ordem: u32, a14: u32, sementes: usize) -> usize {
    #[allow(clippy::type_complexity)]
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize>(IDX_CONSTRUCT_CUP_STAGE);
    let k = (n as u16 as i16).max(0) as usize;
    let plan: Vec<(usize, u8)> = (0..k).map(|i| {
        let s = if sementes != 0 { *((sementes + i) as *const u8) } else if por_ordem != 0 { i as u8 } else { 0 };
        (read_u32_as_usize(teams + i * 4), s)
    }).collect();
    let size = (rounds as u16 as i16 as i32 * 0x68).max(0) as usize;
    let modelo_antes: Vec<u8> = core::slice::from_raw_parts(modelo as *const u8, size).to_vec();
    let r = o(this, parent, n, teams, rounds, a5, modelo, year, stage, b2, id, b3, arvore, por_ordem, a14, sementes);
    let l = read_u32_as_usize(this + 0xa7);
    let t = read_u32_as_usize(this + 0xa3);
    // a árvore (chamada no fim) reescreve partes da tabela das rodadas e da lista (na comparação de 06/11 as duas
    // diferiam do que o construtor grava): confere os campos fixos, a tabela montada e o conjunto dos clubes da lista
    let _ = &modelo_antes;
    let ok = r == this && read_i16(this + 0x3e) as u16 == n as u16 && read_i16(this + 0x3c) as u16 == rounds as u16 && read_i16(this + 0x40) as u16 == year as u16
        && t != 0 && l != 0 && {
            let mut a: Vec<usize> = (0..plan.len()).map(|i| read_u32_as_usize(l + i * 6)).collect();
            let mut b: Vec<usize> = plan.iter().map(|x| x.0).collect();
            a.sort_unstable();
            b.sort_unstable();
            a == b
        };
    PLACAR.registrar(ok, || format!("construct_cup_stage({:#x}) n={} rodadas={} ano={}", this, n as u16 as i16, rounds as u16 as i16, year as u16));
    r
}

/// Comparer for `destroy_cup_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_destroy_cup_stage(this: usize) -> usize {
    original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_DESTROY_CUP_STAGE)(this)
}
