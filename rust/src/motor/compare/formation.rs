use super::*;

// The comparers of the routines in ../formation.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../formation.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `position_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_position_mask(p: usize) -> u16 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u16>(IDX_POSITION_MASK)(p);
    let n = position_mask(p);
    PLACAR.registrar(r == n, || format!("position_mask({:#x}) original={:#x} rust={:#x}", p, r, n));
    r
}

/// Comparer for `pitch_cell` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_pitch_cell(mascara: u32, ataque: i8, band: u32, saida: *mut i16) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32>(IDX_PITCH_CELL);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(mascara, ataque, band, saida), || pitch_cell(mascara, ataque, band, saida));
    PLACAR.registrar(igual && r == v, || format!("pitch_cell({:#x}, {}, {:#x}) original={} {:x?} rust={} {:x?}", mascara, ataque, band, r, mo, v, mr));
    r
}

/// Comparer for `set_slot_coordinate` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
pub(crate) unsafe extern "thiscall" fn comparar_set_slot_coordinate(this: usize, casa: u32, fase: u32, band: u32, coord: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(IDX_SET_SLOT_COORDINATE);
    let reg = formation_regions(this);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, fase, band, coord), || set_slot_coordinate(this, casa, fase, band, coord));
    PLACAR.registrar(igual && r == n, || format!("set_slot_coordinate({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, fase as u8 as i8, band, coord, r, n, mo, mr));
    r
}

/// Comparer for `check_duplicate_position` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura (grava só a saída): as duas sobre saídas próprias.
pub(crate) unsafe extern "thiscall" fn comparar_check_duplicate_position(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(IDX_CHECK_DUPLICATE_POSITION);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, saida); // simplification: a caixa, só a original
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, casa, &mut so);
    let n = check_duplicate_position(this, casa, &mut sr);
    PLACAR.registrar(r == n && so == sr, || format!("check_duplicate_position({:x}, {}) {}/{} saída {}/{}", this, casa as u8 as i8, r, n, so, sr));
    *saida = so;
    r
}

/// Comparador: grava um byte e +0x187f: lado a lado.
pub(crate) unsafe extern "thiscall" fn comparar_set_marking_type(this: usize, casa: u32, marca: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(IDX_SET_MARKING_TYPE);
    if !matches!(marca as u8, 1 | 2 | 4 | 8 | 0x10) {
        return o(this, casa, marca); // simplification: a caixa, só a original
    }
    let reg = [(this as isize + casa as u8 as i8 as isize + 0x5b4) as usize, 1usize];
    let reg = [(reg[0], reg[1]), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, marca), || set_marking_type(this, casa, marca));
    PLACAR.registrar(igual && r == n, || format!("set_marking_type({:x}, {}, {}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, marca as u8, r, n, mo, mr));
    r
}

/// Comparer for `build_position_lookup` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_position_lookup(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BUILD_POSITION_LOOKUP);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5eb, 0x187b - 0x5eb)], || o(this), || build_position_lookup(this));
    PLACAR.registrar(igual && r == n, || format!("build_position_lookup {:x?} x {:x?}", mo, mr));
    r
}

/// Comparer for `stored_cell` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_stored_cell(t: usize, vaga: i8, ataque: i8, band: u32, saida: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(IDX_STORED_CELL);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, vaga, ataque, band, saida), || stored_cell(t, vaga, ataque, band, saida));
    PLACAR.registrar(igual && r == v, || format!("stored_cell({:#x}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, vaga, ataque, band, r, mo, v, mr));
    r
}

/// Comparer for `set_formation_flag` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_formation_flag(t: usize, m: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_FORMATION_FLAG);
    let reg = [(t + 0x584, 4), (t + 0x187f, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, m), || set_formation_flag(t, m));
    PLACAR.registrar(igual && r == v, || format!("set_formation_flag({:#x}, {:#x}) original={} {:x?} rust={} {:x?}", t, m, r, mo, v, mr));
    r
}

/// Comparer for `load_from_disk` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado na formação (0x18e3 bytes: até as instruções dos jogadores em +0x188b + 0x58) e na
/// devolução; o arquivo é aberto e lido pelas duas (só leitura).
pub(crate) unsafe extern "thiscall" fn comparar_load_from_disk(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(IDX_LOAD_FROM_DISK);
    if this == 0 {
        return o(this, caminho, quieto, ofuscado);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x18e3)], || o(this, caminho, quieto, ofuscado),
        || load_from_disk(this, caminho, quieto, ofuscado));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).take(8).collect();
        format!("load_from_disk(this {:x}) original={} rust={}, bytes diferentes {:x?}", this, r, n, dif)
    });
    r
}

/// Comparer for `set_slot_position` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
pub(crate) unsafe extern "thiscall" fn comparar_set_slot_position(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_SET_SLOT_POSITION);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, posicao, extra);
    }
    let reg = [(this + 0x12d, 0x584 - 0x12d), (this + 0x187b, 8), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, posicao, extra), || set_slot_position(this, casa, posicao, extra));
    PLACAR.registrar(igual && r == n, || format!("set_slot_position({}, {:#x}, {}) {:x?} x {:x?}", casa as u8 as i8, posicao as u16, extra, mo, mr));
    r
}

/// Comparer for `forget_person` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_forget_person(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FORGET_PERSON);
    let mut reg: Vec<(usize, usize)> = Vec::new();
    if pessoa != 0 && *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) {
        for k in 0..16usize {
            let q = *((this + 4 * k) as *const usize);
            if q != 0 {
                reg.push(((*(q as *const usize)).wrapping_add((*(pessoa as *const i32) as usize).wrapping_mul(9)), 1));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || forget_person(this, pessoa));
    PLACAR.registrar(igual, || format!("forget_person original={:x?} rust={:x?}", mo, mr));
}

/// Comparer for `clear_assigned` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_clear_assigned(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CLEAR_ASSIGNED);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x5b4, 11), (this + 0x5bf, 44)], || o(this), || clear_assigned(this));
    PLACAR.registrar(igual && r == v, || format!("clear_assigned original={} {:x?} rust={} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `competition_criterion` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "C" fn comparar_competition_criterion(registro: usize, a: u32, b: u32, tipo: u8) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, u32, u32, u8) -> u8>(IDX_COMPETITION_CRITERION)(registro, a, b, tipo);
    let v = competition_criterion(registro, a, b, tipo);
    PLACAR.registrar(r == v, || format!("competition_criterion({:#x}, {:#x}, {:#x}, {}) original={} rust={}", registro, a, b, tipo, r, v));
    r
}

/// Comparer for `formation_mask` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_formation_mask(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_FORMATION_MASK);
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this + 0x187b, 4)], || o(this), || formation_mask(this));
    PLACAR.registrar(igual && r == v, || format!("formation_mask original={:#x} {:x?} rust={:#x} {:x?}", r, mo, v, mr));
    r
}

/// Comparer for `set_square` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_square(this: usize, posicao: i8, mascara: u64) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, u64) -> u32>(IDX_SET_SQUARE);
    let regioes = [(this + 0x187f, 0x64), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, posicao, mascara), || set_square(this, posicao, mascara));
    PLACAR.registrar(igual && r == n, || format!("set_square({}, {:x}) original={} rust={} {:x?} x {:x?}", posicao, mascara, r, n, mo, mr));
    r
}

/// Comparer for `mark_stars` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_mark_stars(this: usize, lado: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8)>(IDX_MARK_STARS);
    let regioes = [(this, MOTOR), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, lado), || mark_stars(this, lado));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("mark_stars({}) diferencas em {:x?}", lado, dif)
    });
}

/// Comparer for `designated_for_position` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_designated_for_position(this: usize, posicao: i8) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> usize>(IDX_DESIGNATED_FOR_POSITION)(this, posicao);
    let n = designated_for_position(this, posicao);
    PLACAR.registrar(r == n, || format!("designated_for_position({}) original={:x} rust={:x}", posicao, r, n));
    r
}
