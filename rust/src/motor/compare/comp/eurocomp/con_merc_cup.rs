use super::*;

// The comparers of the routines in ../comp/eurocomp/con_merc_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/con_merc_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `con_merc_cup_create_semi_final_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_MERC_CUP_CREATE_SEMI_FINAL_STAGE);
    con_merc_cup_compare_knockout_stage("con_merc_cup_create_semi_final_stage", this, 4, 3, 3, || o(this))
}

/// Comparer for `con_merc_cup_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_MERC_CUP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    let last = f >= (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1);
    o(this);
    let after = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("con_merc_cup_advance_stage({:#x}) fase {} -> {}", this, f, after));
}

/// Comparer for `con_merc_cup_select_invited_clubs`.
///
/// Side by side: +0xf2, [0xb67a34] and the random generator's state, starting from the same state (the copy draws
/// without counting); the original routine leaves its list at +0xee, the copy builds its own, compared over the 16
/// slots and freed.
///
/// The draws of the nations' lists stay on each one's stack: only the final list and the random generator count.
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 16 vagas e liberada. Os sorteios das listas
/// das nações ficam na pilha de cada uma: só a lista final e o sorteador contam.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_select_invited_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_MERC_CUP_SELECT_INVITED_CLUBS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + con_merc_cup::DRAWN_CLUB_COUNT, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + con_merc_cup::DRAWN_CLUBS));
        r
    }, || {
        let r = crate::rng::without_counting(|| select_invited_clubs(this));
        lr.set(read_u32_as_usize(this + con_merc_cup::DRAWN_CLUBS));
        write_u32_from_usize(this + con_merc_cup::DRAWN_CLUBS, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("con_merc_cup_select_invited_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `con_merc_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CON_MERC_CUP_UPDATE_FOR_DATE);
    let draws = read_u32_as_usize(this + league::ACTIVE) == 0
        && full != 0
        && (read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as u16).wrapping_add(read_i16(this + league::YEAR) as u16) == core::ptr::read_unaligned((today + 2) as *const u16)
        && read_i16(this + league::NEXT_DRAW_DAY) as u16 == core::ptr::read_unaligned(today as *const u16);
    let r = o(this, today, full);
    if draws {
        let stages = read_u32_as_usize(this + league::STAGES);
        let ok = read_u32_as_usize(this + league::ACTIVE) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + league::TABLE) != 0
            && stages != 0
            && (0..3).all(|g| read_u32_as_usize(stages + g * 4) != 0)
            && read_u32_as_usize(this + league::CURRENT_STAGE) == 2;
        SCOREBOARD.record(ok, || format!("con_merc_cup_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + league::ACTIVE), read_u32_as_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// Comparer for `con_merc_cup_create_first_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_MERC_CUP_CREATE_FIRST_GROUP_TABLE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(list + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + league::TABLE);
    let mut read = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let same = r == 1 && read_i16(this + league::CLUB_COUNT) == 4 && t != 0 && (0..4).all(|i| read[i] == (previstos[i], i as u8));
    SCOREBOARD.record(same, || format!("con_merc_cup_create_first_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `con_merc_cup_create_group_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CON_MERC_CUP_CREATE_GROUP_STAGE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 || !(0..3).contains(&g) {
        return o(this, g);
    }
    let mut plan: Vec<usize> = (0..4).map(|k| read_u32_as_usize(list + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + league::TABLE) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + league::CURRENT_STAGE) as i32 == g && read_i16(f + league::CLUB_COUNT) == 4 && read_i16(f + league::YEAR) == read_i16(this + league::YEAR) && a == plan;
        SCOREBOARD.record(ok, || format!("con_merc_cup_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plan, a));
    }
    r
}

/// Comparer for `con_merc_cup_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CON_MERC_CUP_CREATE_SCHEDULE);
    compare_stage_schedule("con_merc_cup_create_schedule", stage, merconorte_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| con_merc_cup_create_schedule(this, stage, r, e, l))
}

/// Comparer for `con_merc_cup_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_MERC_CUP_SET_UP_RULES);
    check_cup_rules("con_merc_cup_set_up_rules", this, &MERCONORTE_RULES, || o(this))
}
