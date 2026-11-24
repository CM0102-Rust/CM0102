// ---------- cx05_9: the European Championship knockout, the stage advance and the season honours (euro_champ.cpp) ----------

/// The European Championship quarter-finals (stage 3): the ruling body of the competition's discipline rule clears its
/// bookings (`exe::rulingbody_reset_bookings_for_club`(ruling body, 0); the ruling bodies of [`G_DISCIPLINE`] by
/// `.discipline_rule()`, signed); a vector of 8 (`exe::crt_malloc`(0x20); out of memory: the box at line 0x2f2 and
/// nothing more): the winner of group 2, the runner-up of group 1, the winner of group 3, the runner-up of group 4, the
/// winner of group 1, the runner-up of group 2, the winner of group 4 and the runner-up of group 3 (group 1 is the
/// cup's own table, groups 2 to 4 the stages 0 to 2; `exe::league_get_table_row`); then `nations_knockout_stage` of
/// cx05_3 (stage 3, seeded). No knockout: the box at line 0x30e. Otherwise the first round is set up
/// (`exe::cup_set_up_round`(knockout, 0)) and `.set_current_stage(3)`.
/// Original: `EuroChamp::create_knockout_stage` (0x57bb40, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_knockout_stage(this: usize) {
    let league = League(this);
    let rule = league.discipline_rule() as i8 as isize as usize;
    let ruling_body = read_usize(read_usize(read_usize(G_DISCIPLINE)) + rule.wrapping_mul(4));
    exe::rulingbody_reset_bookings_for_club(ruling_body, 0);
    let teams = exe::crt_malloc(0x20);
    if teams == 0 {
        return show_error_at(0x9c4638, 0x2f2);
    }
    let group = |g: usize| if g == 0 { this } else { read_usize(league.stages() + (g - 1) * 4) };
    let club = |g: usize, row: i32| read_usize(exe::league_get_table_row(group(g), row) + league_table_row::CLUB);
    // the original reads the rows in this order (the row reads have no side effects)
    write_usize(teams, club(1, 0)); // cx05_9: group 2's winner opens the draw
    write_usize(teams + 4, club(0, 1));
    write_usize(teams + 8, club(2, 0));
    write_usize(teams + 0xc, club(3, 1));
    write_usize(teams + 0x10, club(0, 0));
    write_usize(teams + 0x14, club(1, 1));
    write_usize(teams + 0x18, club(3, 0));
    write_usize(teams + 0x1c, club(2, 1));
    let knockout = nations_knockout_stage(league, 3, teams, 1);
    if knockout == 0 {
        return show_error_at(0x9c4638, 0x30e);
    }
    covered!("cx05_9 euro quarter finals");
    exe::cup_set_up_round(knockout, 0);
    league.set_current_stage(3);
}

/// Comparer for `euro_champ_create_knockout_stage`: only the original (the stage allocates, the first round is
/// drawn); then the plan: the knockout at [+0xc][3] and +0x30 = 3.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    o(this);
    let league = League(this);
    let ok = league.stages() != 0 && read_usize(league.stages() + 0xc) != 0 && league.current_stage() as i32 == 3;
    SCOREBOARD.record(ok, || format!("euro_champ_create_knockout_stage({:#x}): stage {}", this, league.current_stage() as i32));
}

/// rule: the European Championship moves on one stage when the current one ends; on reaching stage 3 (after the four
/// groups) it builds the quarter-finals; going past the last stage is an error (the box at line 0x297)
/// Original: `EuroChamp::advance_stage` (0x57b880, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_advance_stage(this: usize) {
    let knockout_at = 3; // cx05_9: the quarter-finals after the four groups
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), 0x9c4638, 0x297, knockout_at, 0x57bb40, "cx05_9 euro next stage");
}

/// Comparer for `euro_champ_advance_stage`: only the original (the knockout allocates and draws), then +0x30 one up
/// (unchanged on the last stage).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    con_merc_cup_compare_advance_stage("euro_champ_advance_stage", League(this), || o(this))
}

/// `NationalTeams::clear_competition_for_comp_clubs(comp)` (thiscall, ret 4): clears the current competition of every
/// national team of the competition and reviews the FA confidence of each one.
const FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS: usize = 0x775420;

/// rule: the season honours of the European Championship (winner, runner-up) are those of its knockout, the fourth
/// stage; then the finalists' national teams leave the competition
///
/// The knockout `.stages()`[3] records its honours (its virtual `comp_vtable::RECORD_SEASON_HONOURS`, thiscall, ret 0),
/// then NationalTeams::clear_competition_for_comp_clubs on [`G_NATIONAL_TEAMS`] with the competition record.
/// Original: `EuroChamp::record_season_honours` (0x57b730, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_record_season_honours(this: usize) {
    let league = League(this);
    covered!("cx05_9 euro honours");
    let knockout = read_usize(league.stages() + 0xc); // cx05_9: the knockout is the fourth stage
    let honours: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(knockout) + comp_vtable::RECORD_SEASON_HONOURS));
    honours(knockout);
    let clear: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_CLEAR_COMPETITION_FOR_COMP_CLUBS);
    clear(read_usize(G_NATIONAL_TEAMS), league.competition().addr());
}

/// Comparer for `euro_champ_record_season_honours`: the history and the national teams' competitions do not run
/// twice; only the original.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
