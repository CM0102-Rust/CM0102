// ---------- cx01_12: the Brazilian Champions Cup: the groups, the semi-finals and the season turnover (bra_champ_cup.cpp) ----------

/// The places of a Champions Cup group (the first two go through), passed to League_Stage::League_Stage.
const BRA_CHAMP_CUP_GROUP_PLACES: [u8; 4] = [2, 0, 0, 0];

/// The four clubs of each Champions Cup group: the preliminary's winner with entrants 3, 4 and 5 (group A), the
/// preliminary's runner-up with entrants 6, 7 and 8 (group B).
unsafe fn bra_champ_cup_group_clubs(cup: League) -> [[usize; 4]; 2] {
    let (table, book) = (cup.table(), cup.entrant_book());
    let row = |k: usize| read_usize(table + k * league_table_row::SIZE);
    let entrant = |k: usize| read_usize(book + k * 6);
    [[row(0), entrant(3), entrant(4), entrant(5)], [row(1), entrant(6), entrant(7), entrant(8)]]
}

/// Builds group `g` of the Champions Cup as stage `g`: its round table (`comp_vtable::CREATE_SCHEDULE` for stage g),
/// the League_Stage (operator new 0xee; `FN_LEAGUE_STAGE_LEAGUE_STAGE` with the cup, 4 clubs, `clubs`, 1, the record,
/// the table, the rounds, the win and draw points, the match category, the tie-breaks, `BRA_CHAMP_CUP_GROUP_PLACES`,
/// the year, g, the stage label, the importance, 1, 0, the league flags, −1, 0, 2) into stage g and the round table
/// freed. Returns the stage (null when it was not built).
unsafe fn bra_champ_cup_build_group(cup: League, g: u32, clubs: &[usize; 4]) -> usize {
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(cup.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = schedule(cup.addr(), g, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let mut stage = 0usize;
    if p != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        // simplification: the original pushes the rounds, the label, the year and the flags as dwords with stale high
        // bytes; League_Stage::League_Stage reads the words and bytes
        stage = ctor(p, cup.addr(), 4, clubs.as_ptr() as usize, 1, cup.competition().addr(), table, rounds as u16 as u32, cup.win_points() as u32,
            cup.draw_points() as u32, cup.match_category() as u32, cup.addr() + league::TIEBREAKS, BRA_CHAMP_CUP_GROUP_PLACES.as_ptr() as usize,
            cup.year() as u16 as u32, g, label as u16 as u32, cup.importance() as u32, 1, 0, cup.league_flags() as u16 as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(cup.stages() + 4 * g as usize, stage);
    exe::crt_free(table);
    read_usize(cup.stages() + 4 * g as usize)
}

/// Builds the two four-club groups of the Brazilian Champions Cup (`bra_champ_cup_group_clubs`), each as a stage
/// (`bra_champ_cup_build_group`) followed by its group draw news (`exe::league_send_group_draw_news`); a group not
/// built shows the error box (group A line 0x252, group B line 0x268) and answers 0. Otherwise the current stage is 1
/// and it answers 1.
/// Original: `BraChampCup::create_group_stages` (0x426470, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let cup = League(this);
    let groups = bra_champ_cup_group_clubs(cup);
    for (g, row) in [(0u32, 0x252u32), (1, 0x268)] {
        let stage = bra_champ_cup_build_group(cup, g, &groups[g as usize]);
        if stage == 0 {
            show_error_at(BRA_CHAMP_CUP_SOURCE, row);
            return 0;
        }
        exe::league_send_group_draw_news(stage);
    }
    covered!("cx01_12 champions cup groups");
    write_i32(this + league::CURRENT_STAGE, 1);
    1
}

/// The four semi-finalists in draw order: group A's winner, group B's runner-up, group A's runner-up and group B's
/// winner (the first two rows of each group's table).
unsafe fn bra_champ_cup_semi_finalists(cup: League) -> [usize; 4] {
    let row = |g: usize, k: usize| read_usize(League(read_usize(cup.stages() + 4 * g)).table() + k * league_table_row::SIZE);
    [row(0, 0), row(1, 1), row(0, 1), row(1, 0)]
}

/// Builds the four-club knockout of the Champions Cup as stage 2: the semi-finalists (`bra_champ_cup_semi_finalists`)
/// in a list of four (`exe::crt_malloc`; none: the error box of line 0x286), then `create_knockout_stage` (the round
/// table of stage 2, the Cup_Stage with no seeds, the list and the table freed, the box of line 0x29d without a stage,
/// otherwise the first draw).
/// Original: `BraChampCup::create_semi_final_stage` (0x426720, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let clubs = exe::crt_malloc(0x10);
    if clubs == 0 {
        return show_error_at(BRA_CHAMP_CUP_SOURCE, 0x286);
    }
    covered!("cx01_12 champions cup semi-finals");
    for (k, c) in bra_champ_cup_semi_finalists(cup).into_iter().enumerate() {
        write_usize(clubs + 4 * k, c);
    }
    create_knockout_stage(cup, 4, clubs, 2, core::ptr::null(), BRA_CHAMP_CUP_SOURCE, 0x29d);
}

/// Turns the Champions Cup over to a new season: not closed, the table freed (`exe::league_free_table` with 0), the
/// round dates freed and cleared, the statistics archived (`exe::comp_stats_archive_season_and_reset` with 1), every
/// stage up to the current one destroyed (virtual slot 0 with 1), the year up by one and the current stage −1; then
/// the rules (`comp_vtable::SET_UP_RULES`; 0: the error box of line 0x1fc and 0); with them, no table, no clubs and not
/// active, and it answers the rules' value.
/// Original: `BraChampCup::start_new_season` (0x4262c0, bra_champ_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_champ_cup_start_new_season(this: usize) -> u32 {
    let cup = League(this);
    cup.set_season_closed(0);
    exe::league_free_table(this, 0);
    if cup.round_dates() != 0 {
        exe::crt_free(cup.round_dates());
        cup.set_round_dates(0);
    }
    if cup.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(cup.comp_stats(), 1);
    }
    let mut k = 0i32;
    while k <= cup.current_stage() as i32 {
        let stage = read_usize(cup.stages() + 4 * k as usize);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(stage)));
            destroy(stage, 1);
        }
        k += 1;
    }
    cup.set_year(cup.year().wrapping_add(1));
    write_i32(this + league::CURRENT_STAGE, -1);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_UP_RULES));
    let r = rules(this);
    if r == 0 {
        // simplification: the original returns the box's eax; the season turnover does not read it
        show_error_at(BRA_CHAMP_CUP_SOURCE, 0x1fc);
        return 0;
    }
    covered!("cx01_12 champions cup new season");
    cup.set_table(0);
    cup.set_club_count(0);
    cup.set_active(0);
    r
}

/// The clubs of a stage's table (league rows) or team table (cup), as a sorted list.
unsafe fn stage_clubs(stage: usize) -> Vec<usize> {
    let s = League(stage);
    let mut v: Vec<usize> = if s.comp_type() == 1 {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(Cup(stage).team_table() + 6 * k)).collect()
    } else {
        (0..s.club_count().max(0) as usize).map(|k| read_usize(s.table() + k * league_table_row::SIZE)).collect()
    };
    v.sort_unstable();
    v
}

/// Comparer for `bra_champ_cup_create_group_stages`: only the original runs (the stages allocate and the news goes
/// out); the groups the copy plans before must be the clubs of the two stages built, and the stage 1.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_group_stages(this: usize) -> u32 {
    let cup = League(this);
    let plan = bra_champ_cup_group_clubs(cup).map(|mut g| { g.sort_unstable(); g.to_vec() });
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let built: Vec<Vec<usize>> = (0..2).map(|g| { let s = read_usize(cup.stages() + 4 * g); if s != 0 { stage_clubs(s) } else { Vec::new() } }).collect();
    let ok = r != 1 || (cup.current_stage() as i32 == 1 && built[0] == plan[0] && built[1] == plan[1]);
    SCOREBOARD.record(ok, || format!("bra_champ_cup_create_group_stages({:#x}): returned {}, plan {:x?}, built {:x?}", this, r, plan, built));
    r
}

/// Comparer for `bra_champ_cup_create_semi_final_stage`: only the original runs (the stage allocates and draws); the
/// semi-finalists the copy plans before must be the clubs of the knockout stage built.
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_create_semi_final_stage(this: usize) {
    let cup = League(this);
    let mut plan = bra_champ_cup_semi_finalists(cup).to_vec();
    plan.sort_unstable();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let stage = read_usize(cup.stages() + 8);
    let built = if stage != 0 { stage_clubs(stage) } else { Vec::new() };
    SCOREBOARD.record(stage == 0 || built == plan, || format!("bra_champ_cup_create_semi_final_stage({:#x}): plan {:x?}, built {:x?}", this, plan, built));
}

/// Comparer for `bra_champ_cup_start_new_season`: only the original runs (`compare_new_season`: the year up by one,
/// three stages and the current one −1).
pub(crate) unsafe extern "thiscall" fn compare_bra_champ_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_new_season("bra_champ_cup_start_new_season", League(this), || o(this))
}
