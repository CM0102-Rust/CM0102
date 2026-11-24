// ---------- cx07_6: the Olympic football turnover, stage advance and knockout (olympics.cpp) ----------

/// The file name of olympics.cpp for the error boxes.
const OLYMPICS_SOURCE: usize = 0xa6a360;

/// rule: the Olympic football is every four years: when it ends the table and stages are undone, the season goes to
/// the statistics history, the year goes up 4 and the rules are set up again (`nations_tournament_new_season` of
/// cx05_5; the box at line 0x291)
/// Original: `Olympics::start_new_season` (0x7a0450, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_start_new_season(this: usize) {
    covered!("cx07_6 olympics new edition");
    let years = 4i16; // cx07_6: the Olympic football every four years
    nations_tournament_new_season(League(this), years, OLYMPICS_SOURCE, 0x291);
}

/// Comparer for `olympics_start_new_season`: only the original (it frees and destroys), then the plan: the year up 4,
/// `.current_stage()` −1, `.active()` 0 and `.season_closed()` 0.
pub(crate) unsafe extern "thiscall" fn compare_olympics_start_new_season(this: usize) {
    let league = League(this);
    let year = league.year();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let ok = league.year() == year.wrapping_add(4) && league.current_stage() == usize::MAX && league.active() == 0 && league.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("olympics_start_new_season({:#x}): year {} -> {}, stage {}", this, year, league.year(), league.current_stage() as i32));
}

/// rule: the Olympic football moves on one stage when the current one ends; on reaching stage 3 (after the four
/// groups) it builds the knockout; going past the last stage is an error (the box at line 0x2a3)
/// Original: `Olympics::advance_stage` (0x7a0540, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_advance_stage(this: usize) {
    let knockout_at = 3; // cx07_6: the Olympic knockout after the groups
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), OLYMPICS_SOURCE, 0x2a3, knockout_at, 0x7a07c0, "cx07_6 olympics next stage");
}

/// Comparer for `olympics_advance_stage`: only the original (the knockout allocates and draws), then +0x30 one up
/// (unchanged on the last stage).
pub(crate) unsafe extern "thiscall" fn compare_olympics_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    con_merc_cup_compare_advance_stage("olympics_advance_stage", League(this), || o(this))
}

/// The Olympic football quarter-finals (stage 3): A1, B2, C1, D2, A2, B1, C2, D1 (group A is the cup's own table,
/// groups B to D the stages 0 to 2), in that order.
///
/// A vector of 8 (`exe::crt_malloc`(0x20); out of memory: the box at line 0x2fb and nothing more) with the clubs read
/// straight from the rows of each group's table, then the knockout (`nations_knockout_from` of cx07_2, stage 3, 8
/// teams; no stage: the box at line 0x316).
/// Original: `Olympics::create_knockout_stage` (0x7a07c0, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_create_knockout_stage(this: usize) {
    let league = League(this);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(OLYMPICS_SOURCE, 0x2fb);
    }
    covered!("cx07_6 olympics quarter-finals");
    let group_table = |g: usize| if g == 0 { league.table() } else { League(read_usize(league.stages() + (g - 1) * 4)).table() };
    let order = [(0, 0), (1, 1), (2, 0), (3, 1), (0, 1), (1, 0), (2, 1), (3, 0)]; // cx07_6: (group, row) of each quarter-finalist
    for (k, &(g, row)) in order.iter().enumerate() {
        write_usize(teams + k * 4, read_usize(group_table(g) + row * league_table_row::SIZE + league_table_row::CLUB));
    }
    nations_knockout_from(league, 3, 8, teams, OLYMPICS_SOURCE, 0x316);
}

/// Comparer for `olympics_create_knockout_stage`: only the original (the stage allocates and the first round is set
/// up), then the plan (`check_nations_knockout`).
pub(crate) unsafe extern "thiscall" fn compare_olympics_create_knockout_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    check_nations_knockout("olympics_create_knockout_stage", League(this), 3);
}
