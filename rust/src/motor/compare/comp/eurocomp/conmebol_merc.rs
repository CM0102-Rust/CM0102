use super::*;

// The comparers of the routines in ../comp/eurocomp/conmebol_merc.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/conmebol_merc.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conmebol_merc_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CONMEBOL_MERC_UPDATE_FOR_DATE);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && full != 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((today + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(today as *const u16) && !is_rollover_day(this, today);
    let r = o(this, today, full);
    if sorteia {
        let stages = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && stages != 0
            && (0..4).all(|g| read_u32_as_usize(stages + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 3;
        SCOREBOARD.record(ok, || format!("conmebol_merc_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// Comparer for `conmebol_merc_create_first_group_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_CREATE_FIRST_GROUP_TABLE);
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
    SCOREBOARD.record(same, || format!("conmebol_merc_create_first_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `conmebol_merc_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CONMEBOL_MERC_CREATE_GROUP_STAGE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 || !(0..4).contains(&g) {
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
        SCOREBOARD.record(ok, || format!("conmebol_merc_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plan, a));
    }
    r
}

/// Comparer for `conmebol_merc_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CONMEBOL_MERC_CREATE_SCHEDULE);
    compare_south_american_cup_schedule("conmebol_merc_create_schedule", stage, rounds, extra, league, 4, &MERCOSUR_FINAL_EXTRA_WRITES, || o(this, stage, rounds, extra, league),
        |r, e, l| conmebol_merc_create_schedule(this, stage, r, e, l))
}

/// Comparer for `conmebol_merc_select_invited_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nos convidados que ela gravou e liberada.
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_select_invited_clubs(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CONMEBOL_MERC_SELECT_INVITED_CLUBS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let (r, n) = crate::rng::without_counting(|| mercosur_select_invited_clubs(this));
        lr.set(read_u32_as_usize(this + 0xee));
        nr.set(n);
        core::ptr::write_unaligned((this + 0xee) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = nr.get().max(0) as usize * 4;
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("conmebol_merc_select_invited_clubs({:#x}) devolveu {} x {} memória {} lista {} ({} bytes)", this, a, b, igual, list, k));
    a
}

/// Comparer for `conmebol_merc_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_merc_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_ADVANCE_STAGE);
    con_merc_cup_compare_advance_stage("conmebol_merc_advance_stage", this, || o(this))
}
