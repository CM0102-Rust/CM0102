use super::*;

// The comparers of the routines in ../comp/eurocomp/conmebol_liber.rs.
// [pt] Os comparadores das rotinas de ../comp/eurocomp/conmebol_liber.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conmebol_liber_update_for_date`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CONMEBOL_LIBER_UPDATE_FOR_DATE);
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
            && (0..7).all(|g| read_u32_as_usize(stages + g * 4) != 0)
            && read_u32_as_usize(this + league::CURRENT_STAGE) == 6;
        SCOREBOARD.record(ok, || format!("conmebol_liber_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + league::ACTIVE), read_u32_as_usize(this + league::CURRENT_STAGE) as i32));
    }
    r
}

/// Comparer for `build_first_group_table`.
pub(crate) unsafe extern "thiscall" fn compare_build_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BUILD_FIRST_GROUP_TABLE);
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
    SCOREBOARD.record(same, || format!("build_first_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, read, previstos));
    r
}

/// Comparer for `conmebol_liber_create_group_stage`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CONMEBOL_LIBER_CREATE_GROUP_STAGE);
    let list = read_u32_as_usize(this + 0xee);
    if list == 0 || !(0..7).contains(&g) {
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
        SCOREBOARD.record(ok, || format!("conmebol_liber_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plan, a));
    }
    r
}

/// Comparer for `conmebol_liber_set_up_rules`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_SET_UP_RULES);
    check_cup_rules("conmebol_liber_set_up_rules", this, &LIBERTADORES_RULES, || o(this))
}

/// Comparer for `conmebol_liber_select_qualified_clubs` (comparison mode: see the header of motor.rs): the comparer of
/// `0x4c3f40`, side by side with the two lists (the original routine allocates its own, the copy its own).
///
/// The original runs and its list is kept; the random-draw state is restored, the copy runs without counting and its
/// list is compared with the original's and freed (`0x9452ca`: push; add esp, 4 -> cdecl). With fewer than 32 entrants
/// (the boxes): only the original routine.
///
/// [pt] Comparador da 0x4c3f40: lado a lado com as duas listas (a original aloca a dela, a cópia a sua): a original roda e a
/// lista dela fica; o estado do sorteador volta, a cópia roda sem contar e a lista dela é comparada com a da original e
/// liberada (0x9452ca: push; add esp, 4 -> cdecl). Com menos de 32 inscritos (as caixas): só a original.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_SELECT_QUALIFIED_CLUBS);
    let mut v = [0usize; 32];
    if libertadores_entrants(this, 0x20, &mut v) != 0x20 {
        return o(this); // simplification: the error boxes: original only
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let (ee, f2) = (read_u32_as_usize(this + conmebol_liber::DRAWN_CLUBS), read_u32_as_usize(this + conmebol_liber::DRAWN_CLUB_COUNT));
    let r = o(this);
    let (la, depois) = (read_u32_as_usize(this + conmebol_liber::DRAWN_CLUBS), crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS));
    let a: [u32; 32] = if la != 0 { core::ptr::read_unaligned(la as *const [u32; 32]) } else { [0; 32] };
    state.write_back();
    core::ptr::write_unaligned((this + conmebol_liber::DRAWN_CLUBS) as *mut u32, ee as u32);
    core::ptr::write_unaligned((this + conmebol_liber::DRAWN_CLUB_COUNT) as *mut u32, f2 as u32);
    let rr = crate::rng::pause_trace(|| crate::rng::without_counting(|| conmebol_liber_select_qualified_clubs(this)));
    let lb = read_u32_as_usize(this + conmebol_liber::DRAWN_CLUBS);
    let b: [u32; 32] = if lb != 0 { core::ptr::read_unaligned(lb as *const [u32; 32]) } else { [0; 32] };
    let same_state = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == depois;
    if lb != 0 && lb != la {
        soltar(lb);
    }
    core::ptr::write_unaligned((this + conmebol_liber::DRAWN_CLUBS) as *mut u32, la as u32);
    core::ptr::write_unaligned((this + conmebol_liber::DRAWN_CLUB_COUNT) as *mut u32, 0x20);
    depois.write_back();
    SCOREBOARD.record(r == rr && a == b && same_state, || format!("conmebol_liber_select_qualified_clubs({:#x}) devolveu {} x {} estado igual {} original {:x?} rust {:x?}", this, r, rr, same_state, a, b));
    r
}

/// Comparer for `conmebol_liber_update_staff_records_for_status`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CONMEBOL_LIBER_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let title = stage as u8 == 7 && outcome as u8 == 1;
    let mut slot = 0usize;
    if title {
        let mundial = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP).wrapping_mul(4)));
        let t = read_u32_as_usize(mundial + cup::TEAM_TABLE);
        let nation = read_u32_as_usize(read_u32_as_usize(t) + 0x53);
        slot = if nation != 0 && read_u32_as_usize(read_u32_as_usize(nation + nation::CONTINENT)) == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA) { t } else { t + 6 };
    }
    let r = o(this, club, outcome, stage, a4, game, a6);
    let same = r == 0 && (!title || read_u32_as_usize(slot) == club);
    SCOREBOARD.record(same, || format!("conmebol_liber_update_staff_records_for_status({:#x}, {}, {}) devolveu {} titulo={} vaga {:#x}", club, outcome as u8, stage as u8, r, title, slot));
    r
}

/// Comparer for `conmebol_liber_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    if f >= (read_u32_as_usize(this + league::STAGE_COUNT) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let after = read_u32_as_usize(this + league::CURRENT_STAGE) as i32;
    SCOREBOARD.record(after == f + 1, || format!("conmebol_liber_advance_stage({:#x}) fase {} -> {}", this, f, after));
}

/// Comparer for `conmebol_liber_create_schedule`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CONMEBOL_LIBER_CREATE_SCHEDULE);
    compare_south_american_cup_schedule("conmebol_liber_create_schedule", stage, rounds, extra, league, 7, &[], || o(this, stage, rounds, extra, league),
        |r, e, l| conmebol_liber_create_schedule(this, stage, r, e, l))
}

/// Comparer for `conmebol_liber_create_knockout_stage`.
pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_CREATE_KNOCKOUT_STAGE);
    con_merc_cup_compare_knockout_stage("conmebol_liber_create_knockout_stage", this, 0x10, 7, 7, || o(this))
}
