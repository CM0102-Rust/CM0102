use super::*;

// The comparers of the routines in ../comp/intercomp/african_nations.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/intercomp/african_nations.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `african_nations_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_FOR_DATE);
    let sorteia = completo != 0
        && read_u32_as_usize(this + 0x45) == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_i16(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..3).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 2;
        PLACAR.registrar(ok, || format!("african_nations_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// Comparer for `draw_finalists` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa os grupos dela em +0xee, a cópia faz os seus, comparados nas 16 vagas e liberados (sem os 16, as listas
/// de 50 comparadas nas 16 primeiras vagas).
pub(crate) unsafe extern "thiscall" fn comparar_draw_finalists(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DRAW_FINALISTS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| draw_african_nations_finalists(this));
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("draw_finalists({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// Comparer for `african_nations_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_AFRICAN_NATIONS_CREATE_GROUP_STAGE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("african_nations_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

/// Comparer for `african_nations_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("african_nations_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Comparer for `init_group_a_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_GROUP_A_TABLE);
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
    PLACAR.registrar(igual, || format!("init_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// Comparer for `african_nations_record_season_honours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `african_nations_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("african_nations_update_staff_records_for_status({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// Comparer for `african_nations_update_staff_records_for_status_91e840` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_staff_records_for_status_91e840(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_STAFF_RECORDS_FOR_STATUS_91E840);
    let vaga = fase as u8 == 2 && desfecho as u8 == 1 && saida != 0;
    let mundial = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    if vaga && mundial == 0 {
        return o(this, clube, desfecho, fase, saida, jogo, a6);
    }
    let n = if vaga { read_i16(mundial + 0x36) } else { 0 };
    let cabe = vaga && n < read_i16(mundial + 0x38);
    let r = o(this, clube, desfecho, fase, saida, jogo, a6);
    let igual = r == 0 && (!vaga || (*(saida as *const u8) == 4 && read_i16(mundial + 0x36) == if cabe { n + 1 } else { n }));
    PLACAR.registrar(igual, || format!("african_nations_update_staff_records_for_status_91e840({:#x}, {}, {}) devolveu {} vaga={} lista {}", clube, desfecho as u8, fase as u8, r, vaga, n));
    r
}

/// Comparer for `african_nations_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_AFRICAN_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("african_nations_create_schedule", fase, african_nations_phase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| african_nations_create_schedule(this, fase, r, e, l))
}

/// Comparer for `african_nations_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AFRICAN_NATIONS_SET_UP_RULES);
    check_set_up_rules("african_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 9, -1, 5), 0xffff, || o(this))
}
