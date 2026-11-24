// ---------- cx03_23: the Swedish Cup's constructor, season turnover and predicted positions (swe_cup.cpp) ----------

/// SweCup::build_club_pool (thiscall, ret 0).
const FN_SWECUP_BUILD_CLUB_POOL: usize = 0x88da20;
/// SweCup::collect_entrants (thiscall, ret 0).
const FN_SWECUP_COLLECT_ENTRANTS: usize = 0x88d5a0;
/// SweCup::create_preliminary_stage (thiscall, ret 4).
const FN_SWECUP_CREATE_PRELIMINARY_STAGE: usize = 0x88dda0;
/// SweCup::rank_teams (thiscall, ret 0).
const FN_SWECUP_RANK_TEAMS: usize = 0x88e420;

/// Calls one of the Swedish Cup's own routines with no argument (thiscall, ret 0), through its detour.
unsafe fn swe_cup_call(this: Cup, f: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(f);
    f(this.addr())
}

/// The seven preliminary stages (`create_preliminary_stage` 0..6; the stage pushed as a dword whose high bytes the
/// original leaves as they were: the routine reads the low byte), then a fresh pool for the predicted positions:
/// the pool freed and cleared, built again (`build_club_pool`), `ranking` run, the pool freed and cleared.
unsafe fn swe_cup_stages_and_ranking(this: Cup, ranking: impl FnOnce()) {
    let stage: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_SWECUP_CREATE_PRELIMINARY_STAGE);
    for g in 0..7u32 {
        stage(this.addr(), g);
    }
    let pool = this.addr() + SWE_CUP_CLUB_POOL;
    exe::crt_free(read_usize(pool));
    write_usize(pool, 0);
    swe_cup_call(this, FN_SWECUP_BUILD_CLUB_POOL);
    ranking();
    exe::crt_free(read_usize(pool));
    write_usize(pool, 0);
}

/// The fixed fields the Swedish Cup's constructor leaves on a new cup, for the comparer.
pub(crate) const SWEDISH_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0xc3500), (cup::STAGE_COUNT, 4, 8), (cup::CURRENT_STAGE, 4, 6),
    (cup::CLUB_SIDES, 1, 1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 5), (cup::SUBSTITUTIONS_ALLOWED, 1, 3), (cup::DISCIPLINE_INDEX, 1, 0x1d),
    (cup::CURRENT_ROUND, 1, 0), (SWE_CUP_CLUB_POOL, 4, 0)];

/// The Swedish Cup at career creation (`SweCup(year, record)`): 5 substitutes named and 3 substitutions, discipline rule
/// 0x1d, eight stages (seven preliminary sections and the main draw), a prize fund of 800,000; a new cup builds its
/// pool, its own 30 entrants, its rounds and tie tree, its statistics, the seven preliminary stages and the predicted
/// positions.
///
/// On the Cup base (`exe::new_cup`): the competition record (+4), the vtable 0x970044, the year, `cup::CURRENT_ROUND` =
/// 0, `cup::STAGE_INDEX` 0xff, `cup::CURRENT_STAGE` −1, `cup::STAGE_COUNT` 8, `cup::CLUB_SIDES` 1, 5 substitutes, 3
/// substitutions, `cup::DISCIPLINE_INDEX` 0x1d, `cup::RUN_BY_STAGES` 0 and the pool (`SWE_CUP_CLUB_POOL`) null. If the
/// saved state loads (`exe::cup_load_from_disk`(this, 1) ≠ 0), returns this. Otherwise: the stage vector
/// (`exe::crt_malloc` of 4 per stage; none with no stages); the pool (`build_club_pool`); the entrants
/// (`collect_entrants`; only 0 shows the box of line 0x55 and returns: its −1 for no memory goes on); the prize fund
/// 800,000 and the round table (`build_cup_round_table_by_vtable`); the tie tree (`exe::cup_map_fixture_tree`; 0: the
/// box of line 0x63 and this); the statistics (`create_comp_stats_at_8`; none: the box of line 0x6a and this); the
/// stages and the predicted positions (`swe_cup_stages_and_ranking` with `rank_teams`). Returns this.
/// Original: `SweCup::SweCup` (0x88cbb0, swe_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_swe_cup(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, 0x970044); // the Swedish Cup vtable
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(8);
    cup.set_club_sides(1);
    cup.set_substitutes_named(5);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(0x1d);
    cup.set_run_by_stages(0);
    write_usize(this + SWE_CUP_CLUB_POOL, 0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cx03_23 swedish cup new");
    let stages = cup.stage_count() as i32;
    cup.set_stages(if stages > 0 { exe::crt_malloc(stages as u32 * 4) } else { 0 });
    swe_cup_call(cup, FN_SWECUP_BUILD_CLUB_POOL);
    if swe_cup_call(cup, FN_SWECUP_COLLECT_ENTRANTS) == 0 {
        show_error_at(0xa8b4a0, 0x55);
        return this;
    }
    cup.set_prize_fund(800_000);
    build_cup_round_table_by_vtable(cup);
    if exe::cup_map_fixture_tree(this) == 0 {
        show_error_at(0xa8b4a0, 0x63);
        return this;
    }
    if !create_comp_stats_at_8(this) {
        show_error_at(0xa8b4a0, 0x6a);
        return this;
    }
    swe_cup_stages_and_ranking(cup, || {
        swe_cup_call(cup, FN_SWECUP_RANK_TEAMS);
    });
    this
}

/// rule: at the turnover the Swedish Cup frees last season's teams, rounds and ties, archives its statistics and
/// moves to the next year; then a new pool, the cup's own 30 entrants, the rounds, the tie tree, the seven preliminary
/// stages and the predicted positions. Last season's stage objects are not destroyed (the new stages take their slots:
/// the old ones are lost; `DEFEITOS`).
///
/// `cup::SEASON_CLOSED` = 0; the team table and the round table freed and cleared; each round's tie table freed and
/// cleared, then the array freed and cleared; the season archived (`exe::comp_stats_archive_season_and_reset`(stats, 1),
/// with statistics); the year up by one, `cup::CURRENT_ROUND` and `cup::RUN_BY_STAGES` 0; the pool (`build_club_pool`);
/// the entrants (`collect_entrants`; negative: the box of line 0x1f1 and out), the rules (virtual
/// `comp_vtable::SET_UP_RULES`; negative: 0x1f8), the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`; negative:
/// 0x1ff); then `swe_cup_stages_and_ranking` with the virtual `comp_vtable::SET_PREDICTED_POSITIONS`. The original
/// returns what the last call left in eax; the copy returns 0.
/// Original: `SweCup::start_new_season` (0x88d690, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_start_new_season(this: usize) -> u32 {
    let cup = Cup(this);
    cup.set_season_closed(0);
    if cup.team_table() != 0 {
        exe::crt_free(cup.team_table());
        cup.set_team_table(0);
    }
    if cup.round_table() != 0 {
        exe::crt_free(cup.round_table());
        cup.set_round_table(0);
    }
    if cup.tie_tables() != 0 {
        let mut r = 0i32;
        while r < cup.round_count() as i32 {
            let slot = cup.tie_tables() + r as usize * 4;
            if read_usize(slot) != 0 {
                exe::crt_free(read_usize(slot));
                write_usize(slot, 0);
            }
            r += 1;
        }
        exe::crt_free(cup.tie_tables());
        cup.set_tie_tables(0);
    }
    let stats = League(this).comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_year(cup.year().wrapping_add(1)); // the next edition
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
    swe_cup_call(cup, FN_SWECUP_BUILD_CLUB_POOL);
    if swe_cup_call(cup, FN_SWECUP_COLLECT_ENTRANTS) < 0 {
        show_error_at(0xa8b4a0, 0x1f1);
        return 0;
    }
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(0xa8b4a0, 0x1f8);
        return 0;
    }
    if call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(0xa8b4a0, 0x1ff);
        return 0;
    }
    covered!("cx03_23 swedish cup turnover");
    swe_cup_stages_and_ranking(cup, || {
        call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);
    });
    0
}

/// How many clubs share each predicted place, from the first group of the pool on: 32, 16, 8, 4, 2 and 1 (read from the
/// last byte of the original's table backwards).
pub(crate) const SWEDISH_CUP_PLACE_GROUPS: [u8; 6] = [0x20, 0x10, 8, 4, 2, 1];

/// rule: the Swedish Cup's predicted positions: the pool sorted by reputation (`exe::crt_qsort` with 0x4b07d0); from
/// the pool's second club on, 32 clubs share place 1, the next 16 place 33, then 8 at 49, 4 at 57, 2 at 61 and 1 at 63;
/// every club after them place 63 as well. The pool's first club gets no place (`DEFEITOS`: the count starts at 1).
///
/// Without statistics (+8): the box of line 0x408 and nothing. Each place through Comp_Stats::set_club_final_position
/// (`exe::comp_stats_set_club_final_position`(stats, club, place), thiscall, ret 8; the original pushes the place byte
/// in a dword whose high bytes are stack garbage: the routine reads the byte).
/// Original: `SweCup::rank_teams` (0x88e420, swe_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_cup_rank_teams(this: usize) {
    let cup = Cup(this);
    let stats = League(this).comp_stats();
    if stats == 0 {
        return show_error_at(0xa8b4a0, 0x408);
    }
    covered!("cx03_23 swedish cup predicted positions");
    let (pool, size) = swe_cup_pool(cup);
    exe::crt_qsort(pool, size as i32 as u32, 4, 0x4b07d0usize);
    let mut k: i16 = 1;
    let mut next: u8 = 1;
    let mut place: u8 = 0;
    for &count in SWEDISH_CUP_PLACE_GROUPS.iter() {
        place = next;
        next = next.wrapping_add(count);
        for _ in 0..count {
            exe::comp_stats_set_club_final_position(stats, read_usize(pool + k as usize * 4), place as u32);
            k += 1;
        }
    }
    while k < size {
        exe::comp_stats_set_club_final_position(stats, read_usize(pool + k as usize * 4), place as u32);
        k += 1;
    }
}

/// Comparer for `construct_swe_cup`: only the original runs (it allocates, draws and reads the save file), then the
/// plan is checked (`cup_constructor_plan_mismatch`).
pub(crate) unsafe extern "thiscall" fn compare_construct_swe_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, 0x970044, &SWEDISH_CUP_CONSTRUCTOR_FIELDS, false);
    SCOREBOARD.record(error.is_none(), || format!("construct_swe_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// Comparer for `swe_cup_start_new_season`: only the original runs; the plan: the year up by one.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_cup_new_season("swe_cup_start_new_season", Cup(this), || o(this))
}

/// The predicted place of each club of the pool (`comp_stats_club::FINAL_POSITION`; 0 without an entry).
unsafe fn swe_cup_pool_places(this: Cup) -> Vec<u8> {
    let stats = League(this.addr()).comp_stats();
    let (pool, size) = swe_cup_pool(this);
    (0..size.max(0) as usize).map(|i| {
        let club = read_usize(pool + i * 4);
        let e = if club != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// Comparer for `swe_cup_rank_teams`: the original first; the pool's order put back, the copy run, and the pool and
/// every pool club's place compared.
pub(crate) unsafe extern "thiscall" fn compare_swe_cup_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let cup = Cup(this);
    let (pool, size) = swe_cup_pool(cup);
    if League(this).comp_stats() == 0 || pool == 0 || size <= 0 {
        return o(this);
    }
    let n = 4 * size as usize;
    let before = core::slice::from_raw_parts(pool as *const u8, n).to_vec();
    o(this);
    let order = core::slice::from_raw_parts(pool as *const u8, n).to_vec();
    let places = swe_cup_pool_places(cup);
    core::ptr::copy_nonoverlapping(before.as_ptr(), pool as *mut u8, n);
    swe_cup_rank_teams(this);
    let same_order = core::slice::from_raw_parts(pool as *const u8, n) == &order[..];
    let after = swe_cup_pool_places(cup);
    SCOREBOARD.record(same_order && after == places, || format!("swe_cup_rank_teams({:#x}): order {} places {:?} x {:?}", this, same_order, places, after));
}
