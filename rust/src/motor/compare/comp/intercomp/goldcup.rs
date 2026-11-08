use super::*;

// The comparers of the routines in ../comp/intercomp/goldcup.rs.
// [pt] Os comparadores das rotinas de ../comp/intercomp/goldcup.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Side-by-side comparer: +0xf2 and [0xb67a34] from the same state (nothing is drawn at random); the original routine
/// leaves its list in +0xee, the copy builds its own, compared over the 12 slots and freed.
///
/// [pt] Lado a lado: o +0xf2 e o [0xb67a34] a partir do mesmo estado (nada sorteia); a original deixa a lista dela em +0xee, a
/// cópia faz a sua, comparada nas 12 vagas e liberada.
pub(crate) unsafe extern "thiscall" fn compare_set_up_team_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SET_UP_TEAM_LIST);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + goldcup::DRAWN_CLUB_COUNT, 4usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(read_usize(this + goldcup::DRAWN_CLUBS));
        r
    }, || {
        let r = build_gold_cup_team_list(this);
        lr.set(read_usize(this + goldcup::DRAWN_CLUBS));
        write_usize(this + goldcup::DRAWN_CLUBS, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let list = if lo != 0 && lr != 0 {
        a == 0 || core::slice::from_raw_parts(lo as *const u8, 0x30) == core::slice::from_raw_parts(lr as *const u8, 0x30)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("set_up_team_list({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, list));
    a
}

/// Comparer for `goldcup_set_predicted_positions`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_SET_PREDICTED_POSITIONS);
    if read_usize(this + 8) == 0 || read_usize(this + goldcup::DRAWN_CLUBS) == 0 || read_usize(this + goldcup::DRAWN_CLUB_COUNT) != 0xc {
        return o(this);
    }
    let mut v: Vec<usize> = (0..12).map(|i| read_usize(read_usize(this + goldcup::DRAWN_CLUBS) + i * 4)).collect();
    exe::crt_qsort((v.as_mut_ptr()) as usize, 12, 4, 0x4b07d0);
    o(this);
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { exe::comp_stats_find_club_entry(read_usize(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + comp_stats_club::DANGER_POSITION) as *const u8) as u32 != COPA_AMERICA_PREDICTED_PLACES[k]
    });
    SCOREBOARD.record(errado.is_none(), || format!("goldcup_set_predicted_positions: a posição prevista da vaga {:?} não bate", errado));
}

/// Comparer for `goldcup_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_START_NEW_SEASON);
    let year = read_i16(this + league::YEAR);
    o(this);
    let ok = read_i16(this + league::YEAR) == year.wrapping_add(2) && read_usize(this + league::ACTIVE) == 0 && read_usize(this + league::CURRENT_STAGE) as i32 == -1 && read_usize(this + league::SEASON_CLOSED) == 0;
    SCOREBOARD.record(ok, || format!("goldcup_start_new_season({:#x}): ano {} -> {}, +0x45 {}, +0x30 {}", this, year, read_i16(this + league::YEAR), read_usize(this + league::ACTIVE), read_usize(this + league::CURRENT_STAGE) as i32));
}

/// Comparer for `goldcup_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_GOLDCUP_UPDATE_FOR_DATE);
    let draws = read_usize(this + league::ACTIVE) == 0
        && full == 0
        && (read_i16(this + league::NEXT_DRAW_YEAR_OFFSET) as u16).wrapping_add(read_i16(this + league::YEAR) as u16) == core::ptr::read_unaligned((today + 2) as *const u16)
        && read_i16(this + league::NEXT_DRAW_DAY) as u16 == core::ptr::read_unaligned(today as *const u16);
    let r = o(this, today, full);
    if draws {
        let stages = read_usize(this + league::STAGES);
        let ok = read_usize(this + league::ACTIVE) == 1
            && read_usize(this + 0xee) == 0
            && read_usize(this + 0xf2) == 0
            && read_usize(this + league::TABLE) != 0
            && stages != 0
            && (0..2).all(|g| read_usize(stages + g * 4) != 0)
            && read_usize(this + league::CURRENT_STAGE) == 1;
        SCOREBOARD.record(ok, || format!("goldcup_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_usize(this + league::ACTIVE), read_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// Comparer for `goldcup_create_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GOLDCUP_CREATE_GROUP_TABLE);
    let list = read_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_usize(list + i * 12));
    let r = o(this);
    let t = read_usize(this + league::TABLE);
    let mut read = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let same = r == 1 && read_i16(this + league::CLUB_COUNT) == 4 && t != 0 && (0..4).all(|i| read[i] == (previstos[i], i as u8));
    SCOREBOARD.record(same, || format!("goldcup_create_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `goldcup_create_group_stage`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_GOLDCUP_CREATE_GROUP_STAGE);
    let list = read_usize(this + 0xee);
    if list == 0 || !(0..2).contains(&g) {
        return o(this, g);
    }
    let mut plan: Vec<usize> = (0..4).map(|n| read_usize(list + ((g + 1) as usize + n * 3) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_usize(read_usize(this + league::STAGES) + g as usize * 4);
        let t = if f != 0 { read_usize(f + league::TABLE) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = f != 0 && read_usize(this + league::CURRENT_STAGE) as i32 == g && read_i16(f + league::CLUB_COUNT) == 4 && read_i16(f + league::YEAR) == read_i16(this + league::YEAR) && a == plan;
        SCOREBOARD.record(ok, || format!("goldcup_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plan, a));
    }
    r
}

/// Comparer for `goldcup_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_GOLDCUP_CREATE_SCHEDULE);
    compare_phase_table("goldcup_create_schedule", stage, copa_america_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| goldcup_create_schedule(this, stage, r, e, l))
}

/// Comparer for `goldcup_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GOLDCUP_SET_UP_RULES);
    check_set_up_rules("goldcup_set_up_rules", this, &COPA_AMERICA_RULE_FIELDS, (0x14, 2, 0, 5), 0, || o(this))
}

/// Comparer for `goldcup_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GOLDCUP_ADVANCE_STAGE);
    con_merc_cup_compare_advance_stage("goldcup_advance_stage", this, || o(this))
}

/// Comparer for `goldcup_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_goldcup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_GOLDCUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, a6);
    SCOREBOARD.record(r == 0, || format!("goldcup_update_staff_records_for_status({:#x}, {}, {}) devolveu {}", club, outcome as u8, stage as u8, r));
    r
}
