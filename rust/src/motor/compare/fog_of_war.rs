use super::*;

// The comparers of the routines in ../fog_of_war.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../fog_of_war.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `chance_to_know` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_chance_to_know(person: usize, level: i8) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, i8) -> i32>(IDX_CHANCE_TO_KNOW)(person, level);
    let v = chance_to_know(person, level);
    PLACAR.registrar(r == v, || format!("chance_to_know({}, {}) original={} rust={}", *((person + 4) as *const i16), level, r, v));
    r
}

/// Comparer for `know_person` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_know_person(this: usize, person: usize, font: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_KNOW_PERSON);
    let mut reg: Vec<(usize, usize)> = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if person != 0 && *(person as *const i32) < *(G_STAFF_COUNT as *const i32) {
        reg.push(((*(this as *const usize)).wrapping_add((*(person as *const i32) as usize).wrapping_mul(9)), 9));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, person, font), || crate::aleatorio::sem_contar(|| know_person(this, person, font)));
    PLACAR.registrar(igual, || format!("know_person original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `fog_after_match` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "stdcall" fn comparar_fog_after_match(partida: usize, dados: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, usize)>(IDX_FOG_AFTER_MATCH);
    // the registers the routine may touch (same tests, without the 0x32 of the reserves: leftover does not hurt)
    let human: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let n = *(G_STAFF_COUNT as *const i32);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    for lado in 0..2usize {
        let clube = *((partida + 0x20 - 4 * lado) as *const usize);
        if human(clube, 0, 1, 0, 0) == 0 || *((clube + club::MANAGER) as *const usize) == 0 {
            continue;
        }
        let q = *((clube + club::MANAGER) as *const usize);
        let obj = *((*(G_FOG_OF_WAR as *const usize)).wrapping_add((n.wrapping_sub(*(q as *const i32)) as usize).wrapping_mul(4)).wrapping_sub(4) as *const usize);
        if obj == 0 {
            continue;
        }
        for vaga in 0..20usize {
            let i = *((dados + lado * 0x625 + 0x190 + vaga * 0x49) as *const i32);
            let p = (*(G_STAFF_TABLE as *const usize)).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
            if i != -1 && p != 0 && *(p as *const i32) < n {
                reg.push(((*(obj as *const usize)).wrapping_add((*(p as *const i32) as usize).wrapping_mul(9)), 9));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(partida, dados), || fog_after_match(partida, dados));
    PLACAR.registrar(igual, || format!("fog_after_match original={:x?} rust={:x?}", mo, mr));
}
