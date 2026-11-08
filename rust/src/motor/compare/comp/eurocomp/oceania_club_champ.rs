use super::*;

// The comparers of the routines in ../comp/eurocomp/oceania_club_champ.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/oceania_club_champ.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `oceania_club_champ_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OCEANIA_CLUB_CHAMP_UPDATE_FOR_DATE);
    let draws = read_u32_as_usize(this + league::ACTIVE) == 0
        && full == 0
        && (read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as u16).wrapping_add(read_i16(this + league::YEAR) as u16) == core::ptr::read_unaligned((today + 2) as *const u16)
        && read_i16(this + league::NEXT_DRAW_DAY) as u16 == core::ptr::read_unaligned(today as *const u16);
    let r = o(this, today, full);
    if draws {
        let stages = read_u32_as_usize(this + league::STAGES);
        let ok = read_u32_as_usize(this + league::ACTIVE) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xf2) == 0
            && read_u32_as_usize(this + league::TABLE) != 0
            && stages != 0
            && read_u32_as_usize(stages) != 0
            && read_u32_as_usize(this + league::CURRENT_STAGE) == 0;
        SCOREBOARD.record(ok, || format!("oceania_club_champ_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + league::ACTIVE), read_u32_as_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// Comparer for `create_group_a_table`.
pub(crate) unsafe extern "thiscall" fn compare_create_group_a_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_A_TABLE);
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
    SCOREBOARD.record(same, || format!("create_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `create_group_b_stage`.
pub(crate) unsafe extern "thiscall" fn compare_create_group_b_stage(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CREATE_GROUP_B_STAGE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let mut plan: Vec<usize> = (4..8).map(|k| read_u32_as_usize(list + k * 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + league::STAGES));
        let t = if f != 0 { read_u32_as_usize(f + league::TABLE) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + league::CURRENT_STAGE) == 0 && read_i16(f + league::CLUB_COUNT) == 4 && read_i16(f + league::YEAR) == read_i16(this + league::YEAR) && a == plan;
        SCOREBOARD.record(ok, || format!("create_group_b_stage: a fase não bate com o plano ({:x?} lidos {:x?})", plan, a));
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
    let reg = [(this + oceania_club_champ::DRAWN_CLUB_COUNT, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + oceania_club_champ::DRAWN_CLUBS));
        r
    }, || {
        let r = crate::rng::without_counting(|| select_qualified_clubs(this));
        lr.set(read_u32_as_usize(this + oceania_club_champ::DRAWN_CLUBS));
        write_u32_from_usize(this + oceania_club_champ::DRAWN_CLUBS, lo.get());
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
    SCOREBOARD.record(igual && a == b && list, || format!("oceania_club_champ_select_qualified_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `oceania_club_champ_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    let last = f >= (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1);
    o(this);
    let after = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == if last { f } else { f + 1 }, || format!("oceania_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, after));
}

/// Comparer for `oceania_club_champ_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_oceania_club_champ_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_CLUB_CHAMP_CREATE_KNOCKOUT_STAGE);
    if read_u32_as_usize(this + league::TABLE) == 0 || read_u32_as_usize(this + league::STAGES) == 0 || read_u32_as_usize(read_u32_as_usize(this + league::STAGES)) == 0 {
        return o(this);
    }
    let mut plan = semifinalists(this).to_vec();
    o(this);
    let stage = read_u32_as_usize(read_u32_as_usize(this + league::STAGES) + 4);
    if stage == 0 {
        SCOREBOARD.record(false, || format!("oceania_club_champ_create_knockout_stage({:#x}) sem fase", this));
        return;
    }
    let list = read_u32_as_usize(stage + cup::TEAM_TABLE);
    let mut found: Vec<usize> = (0..4).map(|k| read_u32_as_usize(list + 6 * k)).collect();
    let (pl, ac) = (plan.clone(), found.clone());
    plan.sort_unstable();
    found.sort_unstable();
    let same = plan == found && read_i16(stage + cup::TEAM_COUNT) == 4 && read_u32_as_usize(stage + 4) == read_u32_as_usize(this + 4);
    SCOREBOARD.record(same, || format!("oceania_club_champ_create_knockout_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
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
    check_cup_rules("oceania_club_champ_set_up_rules", this, &OCEANIA_RULES, || o(this))
}
