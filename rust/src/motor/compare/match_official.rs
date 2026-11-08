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
pub(crate) unsafe extern "thiscall" fn comparar_init_strictness(this: usize, engine: u32, person: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(IDX_INIT_STRICTNESS);
    let mut reg = vec![(this, 10)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, engine, person), || crate::aleatorio::sem_contar(|| init_strictness(this, engine, person)));
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
pub(crate) unsafe extern "thiscall" fn comparar_decide_foul(this: usize, j: usize, code: u16, kind: *mut u8, value: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u16, *mut u8, *mut u8) -> u8>(IDX_DECIDE_FOUL);
    let engine = *(this as *const usize);
    let mut reg = vec![(this + 8, 2), (kind as usize, 1), (value as usize, 1), (engine + 0x475a, 16)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, code, kind, value), || crate::aleatorio::sem_contar(|| decide_foul(this, j, code, kind, value)));
    PLACAR.registrar(igual && r == n, || format!("decide_foul({:#x}) original={} {:x?} rust={} {:x?}", code, r, mo, n, mr));
    r
}

/// Comparer for `punish_foul` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_punish_foul(this: usize, j: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, i8, i8, u8, *mut f32)>(IDX_PUNISH_FOUL);
    let reg = regioes_do_cartao(*(this as *const usize), j, kind, weight);
    let t0 = *kind;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, kind, x, sev, flag, weight), || crate::aleatorio::sem_contar(|| punish_foul(this, j, kind, x, sev, flag, weight)));
    PLACAR.registrar(igual, || format!("punish_foul(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `punish_move` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_punish_move(this: usize, j: usize, other: usize, kind: *mut u8, x: i8, sev: i8, flag: u8, weight: *mut f32, expulsar: u8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, *mut u8, i8, i8, u8, *mut f32, u8)>(IDX_PUNISH_MOVE);
    let mut reg = regioes_do_cartao(*(this as *const usize), j, kind, weight);
    reg.push((other + 0x15, 1));
    let t0 = *kind;
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, j, other, kind, x, sev, flag, weight, expulsar), || crate::aleatorio::sem_contar(|| punish_move(this, j, other, kind, x, sev, flag, weight, expulsar)));
    PLACAR.registrar(igual, || format!("punish_move(tipo {}, sev {}, flag {}) original={:x?} rust={:x?}", t0, sev, flag, mo, mr));
}

/// Comparer for `foul_decision` (comparison mode: see the header of motor.rs).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_foul_decision(
    this: usize, j: usize, other: usize, kind: i8, who: i8, card: *mut u8, grav: *mut i8, advantage: *mut u8,
) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i8, i8, *mut u8, *mut i8, *mut u8) -> u8>(IDX_FOUL_DECISION);
    let mut regions = vec![(this + 8, 2), (card as usize, 1), (grav as usize, 1), (advantage as usize, 1)];
    for p in [j, other] {
        if p != 0 {
            regions.push((p, 0x1be));
        }
    }
    regions.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regions,
        || o(this, j, other, kind, who, card, grav, advantage),
        || crate::aleatorio::sem_contar(|| foul_decision(this, j, other, kind, who, card, grav, advantage)),
    );
    PLACAR.registrar(igual && r == v, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("foul_decision(tipo {:#x}) original={} rust={} diferencas em {:x?}", kind, r, v, dif)
    });
    r
}
