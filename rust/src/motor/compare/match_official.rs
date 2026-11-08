use super::*;

// The comparers of the routines in ../match_official.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../match_official.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `construct_referee` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_construct_referee(a: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_CONSTRUCT_REFEREE);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(a, 8)], || o(a), || construct_referee(a));
    PLACAR.registrar(igual && r == v, || format!("construct_referee({:#x}) original={:#x} {:x?} rust={:#x} {:x?}", a, r, mo, v, mr));
    r
}

/// Comparer for `init_strictness` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_strictness(this: usize, motor: u32, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_INIT_STRICTNESS);
    let mut reg = vec![(this, 10)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, motor, pessoa), || crate::aleatorio::sem_contar(|| init_strictness(this, motor, pessoa)));
    PLACAR.registrar(igual, || format!("init_strictness original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `move_counts` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_move_counts(this: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u8>(IDX_MOVE_COUNTS);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x475a, 16)], || o(this), || move_counts(this));
    PLACAR.registrar(igual && r == n, || format!("move_counts original={} {:x?} rust={} {:x?}", r, mo, n, mr));
    r
}

/// Comparer for `decide_foul` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_decide_foul(this: usize, j: usize, codigo: u16, tipo: *mut u8, valor: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(IDX_DECIDE_FOUL);
    let motor = *(this as *const usize);
    let mut reg = vec![(this + 8, 2), (tipo as usize, 1), (valor as usize, 1), (motor + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, codigo, tipo, valor), || crate::aleatorio::sem_contar(|| decide_foul(this, j, codigo, tipo, valor)));
    PLACAR.registrar(igual && r == n, || format!("decide_foul({:#x}) original={} {:x?} rust={} {:x?}", codigo, r, mo, n, mr));
    r
}

/// Comparer for `punish_foul` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_punish_foul(this: usize, j: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(IDX_PUNISH_FOUL);
    let reg = regioes_do_cartao(*(this as *const usize), j, tipo, peso);
    let t0 = *tipo;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, tipo, x, sev, flag, peso), || crate::aleatorio::sem_contar(|| punish_foul(this, j, tipo, x, sev, flag, peso)));
    PLACAR.registrar(igual, || format!("punish_foul(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `punish_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_punish_move(this: usize, j: usize, outro: usize, tipo: *mut u8, x: i8, sev: i8, flag: u8, peso: *mut f32, expulsar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(IDX_PUNISH_MOVE);
    let mut reg = regioes_do_cartao(*(this as *const usize), j, tipo, peso);
    reg.push((outro + 0x15, 1));
    let t0 = *tipo;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, outro, tipo, x, sev, flag, peso, expulsar), || crate::aleatorio::sem_contar(|| punish_move(this, j, outro, tipo, x, sev, flag, peso, expulsar)));
    PLACAR.registrar(igual, || format!("punish_move(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `foul_decision` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_foul_decision(
    this: usize, j: usize, outro: usize, tipo: i8, quem: i8, cartao: *mut u8, grav: *mut i8, vantagem: *mut u8,
) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(IDX_FOUL_DECISION);
    let mut regioes = vec![(this + 8, 2), (cartao as usize, 1), (grav as usize, 1), (vantagem as usize, 1)];
    for p in [j, outro] {
        if p != 0 {
            regioes.push((p, 0x1be));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, j, outro, tipo, quem, cartao, grav, vantagem),
        || crate::aleatorio::sem_contar(|| foul_decision(this, j, outro, tipo, quem, cartao, grav, vantagem)),
    );
    PLACAR.registrar(igual && r == v, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("foul_decision(tipo {:#x}) original={} rust={} diferencas em {:x?}", tipo, r, v, dif)
    });
    r
}
