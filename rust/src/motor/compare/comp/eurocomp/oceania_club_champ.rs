use super::*;

// The comparers of the routines in ../comp/eurocomp/oceania_club_champ.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/oceania_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `oceania_club_champ_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OCEANIA_CLUB_CHAMP_UPDATE_FOR_DATE);
    let draws = this.active() == 0
        && full == 0
        && (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16) == read_u16(today + 2)
        && this.next_draw_day() as u16 == read_u16(today);
    let r = o(this.addr(), today, full);
    if draws {
        let stages = this.stages();
        let ok = this.active() == 1
            && read_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS) == 0
            && read_usize(this.addr() + oceania_club_champ::DRAWN_CLUB_COUNT) == 0
            && this.table() != 0
            && stages != 0
            && read_usize(stages) != 0
            && this.current_stage() == 0;
        SCOREBOARD.record(ok, || format!("oceania_club_champ_update_for_date({:#x}): o draw not montou the groups (+0x45 {}, +0x30 {})", this.addr(), this.active(), this.current_stage() as i32));
    }
    r
}

/// Comparer for `create_group_a_table`.
pub(crate) unsafe extern "thiscall" fn compare_create_group_a_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_A_TABLE);
    let list = read_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS);
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
    SCOREBOARD.record(same, || format!("create_group_a_table({:#x}) returned {} rows {:x?} predicted {:x?}", this.addr(), r, read, predicted));
    r
}

/// Comparer for `create_group_b_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_group_b_stage(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_B_STAGE);
    let list = read_usize(this.addr() + oceania_club_champ::DRAWN_CLUBS);
    if list == 0 {
        return o(this.addr());
    }
    let mut plan: Vec<usize> = (4..8).map(|k| read_usize(list + k * 4)).collect();
    let r = o(this.addr());
    if r == 1 {
        let f = League(read_usize(this.stages()));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && this.current_stage() == 0 && f.club_count() == 4 && f.year() == this.year() && a == plan;
        SCOREBOARD.record(ok, || format!("create_group_b_stage: a stage not bate with o plan ({:x?} read {:x?})", plan, a));
    }
    r
}

/// Comparer for `oceania_club_champ_select_qualified_clubs`.
///
/// [pt] Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 8 vagas e liberada.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_SELECT_QUALIFIED_CLUBS);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + oceania_club_champ::DRAWN_CLUB_COUNT, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + oceania_club_champ::DRAWN_CLUBS));
        r
    }, || {
        let r = crate::rng::without_counting(|| select_qualified_clubs(this));
        lr.set(read_usize(this + oceania_club_champ::DRAWN_CLUBS));
        write_usize(this + oceania_club_champ::DRAWN_CLUBS, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x20) == core::slice::from_raw_parts(lr as *const u8, 0x20)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("oceania_club_champ_select_qualified_clubs({:#x}) returned {} x {} memory {} list {}", this, a, b, igual, list));
    a
}

/// Comparer for `oceania_club_champ_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_advance_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = this.current_stage() as i32;
    let last = f >= (this.stage_count() as i32).wrapping_sub(1);
    o(this.addr());
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("oceania_club_champ_advance_stage({:#x}) stage {} -> {}", this.addr(), f, after));
}

/// Comparer for `oceania_club_champ_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_create_knockout_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_CREATE_KNOCKOUT_STAGE);
    if this.table() == 0 || this.stages() == 0 || read_usize(this.stages()) == 0 {
        return o(this.addr());
    }
    let mut plan = semifinalists(this).to_vec();
    o(this.addr());
    let stage = Cup(read_usize(this.stages() + 4));
    if stage.is_null() {
        SCOREBOARD.record(false, || format!("oceania_club_champ_create_knockout_stage({:#x}) without stage", this.addr()));
        return;
    }
    let list = stage.team_table();
    let mut found: Vec<usize> = (0..4).map(|k| read_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && stage.team_count() == 4 && read_usize(stage.addr() + 4) == this.competition().addr();
    SCOREBOARD.record(same, || format!("oceania_club_champ_create_knockout_stage({:#x}) plan={:x?} list={:x?}", this.addr(), pl, ac));
}

/// Comparer for `oceania_club_champ_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OCEANIA_CLUB_CHAMP_CREATE_SCHEDULE);
    compare_stage_table("oceania_club_champ_create_schedule", stage, oceania_stage_spec(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| oceania_club_champ_create_schedule(this, stage, r, e, l))
}

/// Comparer for `oceania_club_champ_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_CLUB_CHAMP_SET_UP_RULES);
    check_cup_rules("oceania_club_champ_set_up_rules", League(this), &OCEANIA_RULES, || o(this))
}
