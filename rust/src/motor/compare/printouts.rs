use super::*;

// The comparers of the routines in ../printouts.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../printouts.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `pick_random_pundit` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: lado a lado nos cabeçalhos das duas listas (o cursor) e no sorteio, e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_pick_random_pundit(this: usize, clube: usize, tipo: u32, nacao: i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32>(IDX_PICK_RANDOM_PUNDIT);
    let mut regioes = Vec::new();
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, clube, tipo, nacao),
        || crate::aleatorio::sem_contar(|| pick_random_pundit(this, clube, tipo, nacao)),
    );
    PLACAR.registrar(igual && r == n, || format!("pick_random_pundit(clube {:#x}, tipo {}, nacao {}) original={} {:x?} rust={} {:x?}", clube, tipo as i8, nacao, r, mo, n, mr));
    r
}

/// Comparer for `get_pundit_name` (comparison mode: see the header of motor.rs).
///
/// [pt] O comparador: lado a lado no nome (até 0xd4 bytes: o nome do comentarista tem 0xd1 e o padrão menos), na marca e
/// nos cabeçalhos das listas (o cursor).
pub(crate) unsafe extern "thiscall" fn comparar_get_pundit_name(this: usize, tipo: u32, id: u32, nome: usize, marca: *mut u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8)>(IDX_GET_PUNDIT_NAME);
    let mut regioes = vec![(marca as usize, 1)];
    if nome != 0 {
        regioes.push((nome, 0xd4));
    }
    for l in [read_usize(this), read_usize(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tipo, id, nome, marca), || get_pundit_name(this, tipo, id, nome, marca));
    PLACAR.registrar(igual, || format!("get_pundit_name(tipo {}, id {}) original={:x?} rust={:x?}", tipo as i8, id as i16, mo, mr));
}
