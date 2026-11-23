use super::*;

/// Common start: +0x3e = 2 and allocates the two-entry list (0xc bytes) at +0xa7; None with the error box on out of
/// memory.
pub(crate) unsafe fn alloc_two_team_list(this: Cup, file: usize, row: u32) -> Option<usize> {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(file, row);
        return None;
    }
    Some(l)
}

/// Common end: class and status 0 for both entries; returns (1, 2).
pub(crate) unsafe fn finish_two_team_list(l: usize) -> (i32, i32) {
    write_u8(l + 4, 0);
    write_u8(l + 5, 0);
    write_u8(l + cup_team_entry::SEED_LEVEL + 6, 0);
    write_u8(l + cup_team_entry::STATUS + 6, 0);
    (1, 2)
}

/// Writes club c into slot v of the list; with no club shows the error box and returns false.
pub(crate) unsafe fn set_team_slot(l: usize, v: usize, c: usize, file: usize, row: u32) -> bool {
    write_u32(l + 6 * v, c as u32);
    if c == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// `SpaSuper::select_teams()` (`0x858aa0`, thiscall, ret 0, `spa_super.cpp`): picks the Copa del Rey winner and the
/// league champion for the Spanish Super Cup.
///
/// Allocates the list (out of memory, the box of line `0xc2` and −1). The Copa del Rey winner ([`0x9cf740`]; without
/// it, box `0xcb` and 0) and the league champion ([`0x9cf624`]; without it, `0xd4` and 0); if both are the same club,
/// the first becomes the Copa del Rey runner-up (without it, `0xe0` and 0). Returns 1.
pub(crate) unsafe fn select_spanish_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0xa82a58;
    let l = match alloc_two_team_list(Cup(this), F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 0); // cqm12: the Copa del Rey champion
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_SPANISH_FIRST_DIVISION, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        covered!("cqm_12 spanish double");
        if !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_SPANISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    covered!("cqm_12 spanish super cup clubs");
    finish_two_team_list(l)
}

/// rule: the Spanish Super Cup is between the Copa del Rey winner (home in the first leg [inference]) and the League
/// champion; if the same club won both, the Copa del Rey runner-up takes its place; if one is missing, the error box
/// Original: `SpaSuper::select_teams` (0x858aa0, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_super_select_teams(this: usize) -> i32 {
    select_spanish_super_cup_teams(this).0
}

/// Side-by-side comparer for routines that allocate the list +0xa7: compares +0x3e, [0xb67a34] and the 12 list bytes
/// (when returning 1); frees the port's list.
pub(crate) unsafe fn compare_team_list(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), comp_history_region()];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let (r, _) = c();
        lr.set(this.team_table());
        this.set_team_table((lo.get() as u32) as usize);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 12 } else { 0 };
    let list = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(igual && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {}", name, this.addr(), a, b, igual, list));
    a
}

/// `JapSuper::select_teams()` (`jap_super_select_teams`, thiscall, ret 0, jap_super.cpp): the J1 champion and the Emperor's Cup winner.
/// The list (out of memory, the box for line 0xc1 and −1).
///
/// The J1 champion ([0x9cf65c]; without it, 0xca and 0) — with no nation (+0x53 = 0), the runner-up (without it, 0xd6
/// and 0); the Emperor's Cup winner ([0x9cf660]; without it, 0xe0 and 0) — with no nation, the runner-up (without it,
/// 0xec and 0).
///
/// If they are the same club, the second becomes the Emperor's Cup runner-up (without it, 0xf9 and 0) and, if still the
/// same, the first becomes the J1 runner-up (without it, 0x105 and 0). Returns 1.
pub(crate) unsafe fn select_japanese_super_cup_teams(this: usize) -> (i32, i32) {
    const F: usize = 0x9eaf4c;
    let l = match alloc_two_team_list(Cup(this), F, 0xc1) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = Club(latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 0)); // cqm12: the J1 champion
    if !set_team_slot(l, 0, a.addr(), F, 0xca) {
        return (0, 0);
    }
    if a.nation().addr() == 0 && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0xd6) {
        return (0, 0);
    }
    let b = Club(latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 0));
    if !set_team_slot(l, 1, b.addr(), F, 0xe0) {
        return (0, 0);
    }
    if b.nation().addr() == 0 && !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xec) {
        return (0, 0);
    }
    if read_usize(l) == read_usize(l + 6) {
        covered!("cqm_12 japanese double");
        if !set_team_slot(l, 1, latest_placed_club(KEY_CLUB_COMP_JAPANESE_EMPEROR_S_CUP, 1), F, 0xf9) {
            return (0, 0);
        }
        if read_usize(l) == read_usize(l + 6) && !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1, 1), F, 0x105) {
            return (0, 0);
        }
    }
    covered!("cqm_12 japanese super cup clubs");
    finish_two_team_list(l)
}

/// Bytes the original writes in a round: +0..+0x10, +0x13..+0x22 and +0x5c..+0x67.
pub(crate) unsafe fn spa_super_written_round_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for &(a, b) in &[(0usize, 0x11usize), (0x13, 0x23), (0x5c, 0x68)] {
        v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, b - a));
    }
    v
}

/// rule: the Spanish Super Cup is played over two legs (7 days apart) between 2 clubs, without a draw (+0xd = 0x200:
/// the built order), drawn on 5/7 with the first leg on 20/8 (moved to weekday 6) at the home side [inference: venue
/// 0]; two-leg tiebreak 1 [inference: away goals and penalties]; no prize; category 6
/// Original: `SpaSuper::create_round_table` (0x858960, spa_super.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn spa_super_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &SPANISH_SUPER_CUP, 0xa82a58, 0x99, "cql_12 tabela da supercopa da espanha", |_| {})
}

/// The season turnover of the Spanish Super Cup: last season's entrant, round and tie tables freed, the season archived in the
/// statistics, the year moved on by one and the entrants, the round table and the tie tree rebuilt; no predicted positions: it returns the eax of the tie tree.
///
/// The cup turnover template (`usa_mls_all_stars_cup_start_new_season`, checked instruction by instruction against the sisters): `cup::SEASON_CLOSED` = 0;
/// the team table (`cup::TEAM_TABLE`) and the round table (`cup::ROUND_TABLE`) freed (`exe::crt_free`) and zeroed; the
/// ties of each round (`cup::TIE_TABLES`, `cup::ROUND_COUNT` rounds) freed and zeroed, then the array; with statistics,
/// Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, thiscall(stats, 1), ret 4); the
/// year (`cup::YEAR`) up by one; `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own box
/// (spa_super.cpp, lines 0x125, 0x12c and 0x133) when it returns a negative value: the entrants (SpaSuper::select_teams at 0x858aa0, thiscall, ret 0), the round
/// table (virtual `comp_vtable::SET_UP_RULES`) and the tie tree (virtual `cup_vtable::MAP_FIXTURE_TREE`).
/// After a box the copy returns 0 (the original returns the box's eax; the turnover's caller does not read it).
/// Original: `SpaSuper::start_new_season` (0x858cd0, spa_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn spa_super_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x858aa0, 0xa82a58, [0x125, 0x12c, 0x133], 1, false, "mwf_4 Spanish Super Cup rollover") // mwf4: the turnover of the Spanish Super Cup
}

/// Comparer for `spa_super_start_new_season`: only the original runs (it frees and rebuilds the cup); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_spa_super_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SPA_SUPER_START_NEW_SEASON);
    compare_cup_new_season("spa_super_start_new_season", Cup(this), || o(this))
}

/// The constructor of the Spanish Super Cup: with no entrant table of its own; type 1 (clubs of one nation), no prize
/// fund (`cup::PRIZE_FUND` stays as the base left it), 7 substitutes on the bench and 3 substitutions, discipline rule
/// 26; on a new cup, in order: the two clubs (SpaSuper::select_teams, `spa_super_select_teams`, 0x858aa0, thiscall, ret
/// 0; the error box of line 0x43 if it returns a negative value); the rounds (the cup's own virtual
/// `comp_vtable::CREATE_SCHEDULE`, into `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree, `map_fixture_tree`,
/// 0x518790, thiscall, ret 0; the error box of line 0x51 if it returns a negative value); the statistics at Comp +8
/// (the error box of line 0x58 without them). Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0) it writes the
/// fixed fields in the original's order (the vtable 0x96fd38, the competition record at Comp +4, `cup::YEAR` and those
/// of `SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS`); if the saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx,
/// this -> thiscall, ret 4; returns ≠ 0) it returns this. Each step that fails shows its own error box
/// (`show_error_at`) and returns this. Returns this.
/// Original: `SpaSuper::SpaSuper` (0x8586b0, spa_super.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    construct_spa_super_body(Cup(this), year, record) // cqu6: the Spanish Super Cup
}

/// Comparer for `construct_spa_super`: only the original runs (it allocates, draws and reads the save file), and the plan
/// is checked (`spa_cup_check_constructor_plan`).
pub(crate) unsafe extern "thiscall" fn compare_construct_spa_super(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SPA_SUPER)(this, year, record);
    let error = spa_cup_check_constructor_plan(Cup(this), r, year, record, 0x96fd38, &SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_spa_super({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The body of `construct_spa_super` (its arguments are what the mutation tests change).
pub(crate) unsafe fn construct_spa_super_body(this: Cup, year: u32, record: usize) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), 0x96fd38); // the vtable
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    this.set_stage_count(0);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_club_sides(1);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    this.set_discipline_index(discipline_rule::SPAIN_LEAGUE);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!("cqu_6 spanish super cup new");
    let select_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_SPASUPER_SELECT_TEAMS); // SpaSuper::select_teams
    let r = select_teams(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x43);
        return this.addr();
    }
    spa_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let r = map_fixture_tree(this.addr());
    if r < 0 {
        show_error_at(0xa82a58, 0x51);
        return this.addr();
    }
    if !spa_cup_create_comp_stats(this) {
        show_error_at(0xa82a58, 0x58);
        return this.addr();
    }
    this.addr()
}

/// The fixed fields `construct_spa_super` writes on a new cup, for the comparer: (offset, width, value).
pub(crate) const SPANISH_SUPER_CUP_CONSTRUCTOR_FIELDS: [(usize, u8, u32); 9] = [(cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x1a), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];
