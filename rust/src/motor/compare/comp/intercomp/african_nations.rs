use super::*;

// The comparers of the routines in ../comp/intercomp/african_nations.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/intercomp/african_nations.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `african_nations_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_FOR_DATE);
    let sorteia = full != 0
        && read_u32_as_usize(this + 0x45) == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((today + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(today as *const u16);
    let r = o(this, today, full);
    if sorteia {
        let stages = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_i16(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && stages != 0
            && (0..3).all(|g| read_u32_as_usize(stages + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 2;
        PLACAR.record(ok, || format!("african_nations_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
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
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + 0xf2, 2usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::rng::without_counting(|| draw_african_nations_finalists(this));
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.record(igual && a == b && list, || format!("draw_finalists({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `african_nations_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_AFRICAN_NATIONS_CREATE_GROUP_STAGE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plan: Vec<usize> = (0..4).map(|k| read_u32_as_usize(list + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plan;
        PLACAR.record(ok, || format!("african_nations_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plan, a));
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
    let after = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.record(after == f + 1, || format!("african_nations_advance_stage({:#x}) fase {} -> {}", this, f, after));
}

/// Comparer for `init_group_a_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_init_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_INIT_GROUP_A_TABLE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(list + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut read = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let same = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| read[i] == (previstos[i], i as u8));
    PLACAR.record(same, || format!("init_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `african_nations_record_season_honours` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_AFRICAN_NATIONS_RECORD_SEASON_HONOURS)(this)
}

/// Comparer for `african_nations_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_update_staff_records_for_status(this: usize, clube: usize, outcome: u32, stage: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_AFRICAN_NATIONS_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, clube, outcome, stage, a4, jogo, a6);
    PLACAR.record(r == 0, || format!("african_nations_update_staff_records_for_status({:#x}, {}, {}) devolveu {}", clube, outcome as u8, stage as u8, r));
    r
}


/// Comparer for `african_nations_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_AFRICAN_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("african_nations_create_schedule", stage, african_nations_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| african_nations_create_schedule(this, stage, r, e, l))
}

/// Comparer for `african_nations_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_african_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_AFRICAN_NATIONS_SET_UP_RULES);
    check_set_up_rules("african_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 9, -1, 5), 0xffff, || o(this))
}
