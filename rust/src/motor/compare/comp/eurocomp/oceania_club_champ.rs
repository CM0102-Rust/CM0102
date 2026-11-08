use super::*;

// The comparers of the routines in ../comp/eurocomp/oceania_club_champ.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/oceania_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `oceania_club_champ_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OCEANIA_CLUB_CHAMP_UPDATE_FOR_DATE);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && read_u32_as_usize(fases) != 0
            && read_u32_as_usize(this + 0x30) == 0;
        PLACAR.registrar(ok, || format!("oceania_club_champ_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// Comparer for `create_group_a_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_A_TABLE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(lista + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("create_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// Comparer for `create_group_b_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_group_b_stage(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_B_STAGE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let mut plano: Vec<usize> = (4..8).map(|k| read_u32_as_usize(lista + k * 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc));
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) == 0 && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("create_group_b_stage: a fase não bate com o plano ({:x?} lidos {:x?})", plano, a));
    }
    r
}

/// Comparer for `oceania_club_champ_select_qualified_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 8 vagas e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_SELECT_QUALIFIED_CLUBS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| select_qualified_clubs(this));
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x20) == core::slice::from_raw_parts(lr as *const u8, 0x20)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("oceania_club_champ_select_qualified_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// Comparer for `oceania_club_champ_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("oceania_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Comparer for `oceania_club_champ_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_CREATE_KNOCKOUT_STAGE);
    if read_u32_as_usize(this + 0xb1) == 0 || read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = semifinalists(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("oceania_club_champ_create_knockout_stage({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("oceania_club_champ_create_knockout_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

/// Comparer for `oceania_club_champ_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OCEANIA_CLUB_CHAMP_CREATE_SCHEDULE);
    compare_stage_table("oceania_club_champ_create_schedule", fase, oceania_stage_spec(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| oceania_club_champ_create_schedule(this, fase, r, e, l))
}

/// Comparer for `oceania_club_champ_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_oceania_club_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_SET_UP_RULES);
    check_cup_rules("oceania_club_champ_set_up_rules", this, &OCEANIA_RULES, || o(this))
}
