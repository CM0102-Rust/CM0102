use super::*;

// The comparers of the routines in ../comp/eurocomp/conmebol_merc.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/conmebol_merc.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conmebol_merc_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CONMEBOL_MERC_UPDATE_FOR_DATE);
    let draws = this.active() == 0
        && full != 0
        && (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16) == read_u16(today + 2)
        && this.next_draw_day() as u16 == read_u16(today) && !is_rollover_day(this, today);
    let r = o(this.addr(), today, full);
    if draws {
        let stages = this.stages();
        let ok = this.active() == 1
            && read_usize(this.addr() + 0xee) == 0
            && this.table() != 0
            && stages != 0
            && (0..4).all(|g| read_usize(stages + g * 4) != 0)
            && this.current_stage() == 3;
        SCOREBOARD.record(ok, || format!("conmebol_merc_update_for_date({:#x}): o draw não montou the groups (+0x45 {}, +0x30 {})", this.addr(), this.active(), this.current_stage() as i32));
    }
    r
}

/// Comparer for `conmebol_merc_create_first_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_create_first_group_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_MERC_CREATE_FIRST_GROUP_TABLE);
    let list = read_usize(this.addr() + 0xee);
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
    SCOREBOARD.record(same, || format!("conmebol_merc_create_first_group_table({:#x}) returned {} rows {:x?} predicted {:x?}", this.addr(), r, read, predicted));
    r
}

/// Comparer for `conmebol_merc_create_group_stage`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CONMEBOL_MERC_CREATE_GROUP_STAGE);
    let list = read_usize(this.addr() + 0xee);
    if list == 0 || !(0..4).contains(&g) {
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
        SCOREBOARD.record(ok, || format!("conmebol_merc_create_group_stage({}): a stage não bate with o plan ({:x?} read {:x?})", g, plan, a));
    }
    r
}

/// Comparer for `conmebol_merc_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CONMEBOL_MERC_CREATE_SCHEDULE);
    compare_south_american_cup_schedule("conmebol_merc_create_schedule", stage, rounds, extra, league, 4, &MERCOSUR_FINAL_EXTRA_WRITES, || o(this, stage, rounds, extra, league),
        |r, e, l| conmebol_merc_create_schedule(this, stage, r, e, l))
}

/// Comparer for `conmebol_merc_select_invited_clubs`.
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nos convidados que ela gravou e liberada.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_select_invited_clubs(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_CONMEBOL_MERC_SELECT_INVITED_CLUBS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + conmebol_merc::DRAWN_CLUB_COUNT, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + conmebol_merc::DRAWN_CLUBS));
        r
    }, || {
        let (r, n) = crate::rng::without_counting(|| mercosur_select_invited_clubs(this));
        lr.set(read_usize(this + conmebol_merc::DRAWN_CLUBS));
        nr.set(n);
        write_u32(this + conmebol_merc::DRAWN_CLUBS, lo.get() as u32);
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
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("conmebol_merc_select_invited_clubs({:#x}) returned {} x {} memória {} list {} ({} bytes)", this, a, b, igual, list, k));
    a
}

/// Comparer for `conmebol_merc_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_merc_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_MERC_ADVANCE_STAGE);
    con_merc_cup_compare_advance_stage("conmebol_merc_advance_stage", League(this), || o(this))
}
