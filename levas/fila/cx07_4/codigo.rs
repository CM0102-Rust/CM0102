// ---------- cx07_4: the Oceania Nations Cup turnover, stage advance and knockout (oceania_nations.cpp) ----------

/// The file name of oceania_nations.cpp for the error boxes.
const OCEANIA_NATIONS_SOURCE: usize = 0xa6a0ec;

/// rule: the Oceania Nations Cup is every two years: when it ends the table and stages are undone, the season goes to
/// the statistics history, the year goes up 2 and the rules are set up again (`nations_tournament_new_season` of
/// cx05_5; the box at line 0x2fd)
/// Original: `OceaniaNations::start_new_season` (0x7998e0, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_start_new_season(this: usize) {
    covered!("cx07_4 oceania nations new edition");
    let years = 2i16; // cx07_4: the Oceania Nations Cup every two years
    nations_tournament_new_season(League(this), years, OCEANIA_NATIONS_SOURCE, 0x2fd);
}

/// Comparer for `oceania_nations_start_new_season`: only the original (it frees and destroys), then the plan: the year
/// up 2, `.current_stage()` −1, `.active()` 0 and `.season_closed()` 0.
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_start_new_season(this: usize) {
    let league = League(this);
    let year = league.year();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let ok = league.year() == year.wrapping_add(2) && league.current_stage() == usize::MAX && league.active() == 0 && league.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("oceania_nations_start_new_season({:#x}): year {} -> {}, stage {}", this, year, league.year(), league.current_stage() as i32));
}

/// rule: the Oceania Nations Cup moves on one stage when the current one ends; on reaching stage 1 (after the two
/// groups) it builds the knockout; going past the last stage is an error (the box at line 0x30f)
/// Original: `OceaniaNations::advance_stage` (0x7999d0, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_advance_stage(this: usize) {
    let knockout_at = 1; // cx07_4: the Oceania knockout after the groups
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), OCEANIA_NATIONS_SOURCE, 0x30f, knockout_at, 0x799c50, "cx07_4 oceania nations next stage");
}

/// Comparer for `oceania_nations_advance_stage`: only the original (the knockout allocates and draws), then +0x30 one
/// up (unchanged on the last stage).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    con_merc_cup_compare_advance_stage("oceania_nations_advance_stage", League(this), || o(this))
}

/// The Oceania Nations Cup semi-finals (stage 1): group A's winner against group B's runner-up and group A's runner-up
/// against group B's winner.
///
/// A vector of 4 (`exe::crt_malloc`(0x10); out of memory: the box at line 0x366 and nothing more): A's winner, B's
/// runner-up, A's runner-up, B's winner (group A is the cup's own table, group B the stage 0;
/// `exe::league_get_table_row`), then the knockout (`nations_knockout_from` of cx07_2, stage 1, 4 teams; no stage: the
/// box at line 0x37d).
/// Original: `OceaniaNations::create_knockout_stage` (0x799c50, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_knockout_stage(this: usize) {
    let league = League(this);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        return show_error_at(OCEANIA_NATIONS_SOURCE, 0x366);
    }
    covered!("cx07_4 oceania nations semi-finals");
    let group_b = read_usize(league.stages());
    let pairs = [(this, 0), (group_b, 1), (this, 1), (group_b, 0)]; // cx07_4: the semi-final pairs
    for (k, &(group, row)) in pairs.iter().enumerate() {
        write_usize(teams + k * 4, table_row_club(group, row));
    }
    nations_knockout_from(league, 1, 4, teams, OCEANIA_NATIONS_SOURCE, 0x37d);
}

/// Comparer for `oceania_nations_create_knockout_stage`: only the original (the stage allocates and the first round is
/// set up), then the plan (`check_nations_knockout`).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_knockout_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    check_nations_knockout("oceania_nations_create_knockout_stage", League(this), 1);
}
