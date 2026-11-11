use super::*;

// The comparers of the routines in ../comp/eurocomp/con_merc_cup.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/con_merc_cup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `con_merc_cup_create_semi_final_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_MERC_CUP_CREATE_SEMI_FINAL_STAGE);
    con_merc_cup_compare_knockout_stage("con_merc_cup_create_semi_final_stage", League(this), 4, 3, 3, || o(this))
}

/// Comparer for `con_merc_cup_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_advance_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CON_MERC_CUP_ADVANCE_STAGE);
    let f = this.current_stage() as i32;
    let last = f >= (this.stage_count() as i32).wrapping_sub(1);
    o(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("con_merc_cup_advance_stage({:#x}) stage {} -> {}", this.addr(), f, after));
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
    let reg = [(this + con_merc_cup::DRAWN_CLUB_COUNT, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + con_merc_cup::DRAWN_CLUBS));
        r
    }, || {
        let r = crate::rng::without_counting(|| select_invited_clubs(this));
        lr.set(read_usize(this + con_merc_cup::DRAWN_CLUBS));
        write_usize(this + con_merc_cup::DRAWN_CLUBS, lo.get());
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
    SCOREBOARD.record(igual && a == b && list, || format!("con_merc_cup_select_invited_clubs({:#x}) returned {} x {} memory {} list {}", this, a, b, igual, list));
    a
}

/// Comparer for `con_merc_cup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CON_MERC_CUP_UPDATE_FOR_DATE);
    let draws = this.active() == 0
        && full != 0
        && (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16) == read_u16(today + 2)
        && this.next_draw_day() as u16 == read_u16(today);
    let r = o(this.addr(), today, full);
    if draws {
        let stages = this.stages();
        let ok = this.active() == 1
            && read_usize(this.addr() + con_merc_cup::DRAWN_CLUBS) == 0
            && this.table() != 0
            && stages != 0
            && (0..3).all(|g| read_usize(stages + g * 4) != 0)
            && this.current_stage() == 2;
        SCOREBOARD.record(ok, || format!("con_merc_cup_update_for_date({:#x}): o draw not montou the groups (+0x45 {}, +0x30 {})", this.addr(), this.active(), this.current_stage() as i32));
    }
    r
}

/// Comparer for `con_merc_cup_create_first_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_first_group_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CON_MERC_CUP_CREATE_FIRST_GROUP_TABLE);
    let list = read_usize(this.addr() + con_merc_cup::DRAWN_CLUBS);
    if list == 0 {
        return o(this.addr());
    }
    let predicted: [usize; 4] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this.addr());
    let t = this.table();
    let mut read = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * 0x3b), read_u8(t + i * 0x3b + 4));
        }
    }
    let same = r == 1 && this.club_count() == 4 && t != 0 && (0..4).all(|i| read[i] == (predicted[i], i as u8));
    SCOREBOARD.record(same, || format!("con_merc_cup_create_first_group_table({:#x}) returned {} rows {:x?} predicted {:x?}", this.addr(), r, read, predicted));
    r
}

/// Comparer for `con_merc_cup_create_group_stage`.
pub(crate) unsafe extern "thiscall" fn compare_con_merc_cup_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CON_MERC_CUP_CREATE_GROUP_STAGE);
    let list = read_usize(this.addr() + con_merc_cup::DRAWN_CLUBS);
    if list == 0 || !(0..3).contains(&g) {
        return o(this.addr(), g);
    }
    let mut plan: Vec<usize> = (0..4).map(|k| read_usize(list + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this.addr(), g);
    if r == 1 {
        let f = League(read_usize(this.stages() + g as usize * 4));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && this.current_stage() as i32 == g && f.club_count() == 4 && f.year() == this.year() && a == plan;
        SCOREBOARD.record(ok, || format!("con_merc_cup_create_group_stage({}): a stage not bate with o plan ({:x?} read {:x?})", g, plan, a));
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
    check_cup_rules("con_merc_cup_set_up_rules", League(this), &MERCONORTE_RULES, || o(this))
}
