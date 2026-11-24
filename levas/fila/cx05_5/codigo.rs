// ---------- cx05_5: the new edition of the African Nations Cup, the Asian Cup and the European Championship ----------

/// The turnover body of the national-team tournaments (virtual `comp_vtable::START_NEW_SEASON`, thiscall, ret 0):
/// `.set_season_closed(0)`, the table freed (`exe::league_free_table`(this, 0)), the round table `.round_dates()` freed
/// and zeroed, the season archived in the statistics (`exe::comp_stats_archive_season_and_reset`(stats, 1)), stages
/// 0..=`.current_stage()` destroyed (virtual 0 with 1), the year `.year()` up `years`, `.current_stage()` −1,
/// `.set_active(0)` and the rules again (virtual `comp_vtable::SET_UP_RULES`; 0: the box at line `line` of `file`).
pub(crate) unsafe fn nations_tournament_new_season(this: League, years: i16, file: usize, line: u32) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    destroy_current_stages(this);
    this.set_year(this.year().wrapping_add(years));
    this.set_current_stage(usize::MAX);
    this.set_active(0);
    if set_up_rules_again(this) == 0 {
        show_error_at(file, line);
    }
}

/// Stages 0..=`.current_stage()` (signed) destroyed through their virtual 0 with 1 (the destructor that frees); the
/// slots are left as they are.
unsafe fn destroy_current_stages(this: League) {
    let mut i = 0i32;
    while i <= this.current_stage() as i32 {
        let stage = read_usize(this.stages() + i as usize * 4);
        if stage != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(stage, 1);
        }
        i += 1;
    }
}

/// The rules of the competition again (virtual `comp_vtable::SET_UP_RULES`, thiscall, ret 0).
unsafe fn set_up_rules_again(this: League) -> u32 {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    rules(this.addr())
}

/// rule: the African Nations Cup is every two years: when it ends the table and stages are undone, the season goes to
/// the statistics history, the year goes up 2 and the rules are set up again (`nations_tournament_new_season`; the box
/// at line 0x2a8)
/// Original: `AfricanNations::start_new_season` (0x401d20, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_start_new_season(this: usize) {
    covered!("cx05_5 african nations new edition");
    let years = 2i16; // cx05_5: the African Nations Cup every two years
    nations_tournament_new_season(League(this), years, 0x9870f8, 0x2a8);
}

/// rule: the Asian Cup is every four years: when it ends the table and stages are undone, the season goes to the
/// statistics history, the year goes up 4 and the rules are set up again (`nations_tournament_new_season`; the box at
/// line 0x29b)
/// Original: `AsiaNations::start_new_season` (0x40f720, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_start_new_season(this: usize) {
    covered!("cx05_5 asian cup new edition");
    let years = 4i16; // cx05_5: the Asian Cup every four years
    nations_tournament_new_season(League(this), years, 0x987ba0, 0x29b);
}

/// rule: the European Championship is every four years: when it ends its qualifying competition starts its own new
/// edition, the finals' table, round table and stages are undone, the season goes to the statistics history, the year
/// goes up 4, the entrant list is rebuilt (the hosts and the default field) and the rules are set up again
///
/// `.set_season_closed(0)`; the qualifying competition's turnover (the competition object of
/// [`KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING`] in [`G_COMP_OBJECT_TABLE`], virtual
/// `comp_vtable::START_NEW_SEASON`); the table freed (`exe::league_free_table`(this, 0)), `.club_count()` 0 and
/// `.set_active(0)`; the round table freed and zeroed; the season archived (`exe::comp_stats_archive_season_and_reset`(
/// stats, 1)); stages 0..=`.current_stage()` destroyed; the year up 4, `.stage_index()` `stage_index::MAIN` and
/// `.current_stage()` −1; the entrant book freed and zeroed and built again (`exe::eurochamp_set_up_team_list`); the
/// rules again (0: the box at line 0x283 and nothing more); then `.table()` null.
/// Original: `EuroChamp::start_new_season` (0x57b750, euro_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn euro_champ_start_new_season(this: usize) {
    let league = League(this);
    league.set_season_closed(0);
    let qualifying = read_usize(read_usize(G_COMP_OBJECT_TABLE) + read_usize(KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING) * 4);
    let turnover: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(qualifying) + comp_vtable::START_NEW_SEASON));
    turnover(qualifying);
    exe::league_free_table(league, 0);
    league.set_club_count(0);
    league.set_active(0);
    if league.round_dates() != 0 {
        exe::crt_free(league.round_dates());
        league.set_round_dates(0);
    }
    if league.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(league.comp_stats(), 1);
    }
    destroy_current_stages(league);
    covered!("cx05_5 european championship new edition");
    league.set_year(league.year().wrapping_add(4)); // cx05_5: the European Championship every four years
    league.set_stage_index(stage_index::MAIN);
    league.set_current_stage(usize::MAX);
    exe::crt_free(league.entrant_book());
    league.set_entrant_book(0);
    exe::eurochamp_set_up_team_list(league);
    if set_up_rules_again(league) == 0 {
        return show_error_at(0x9c4638, 0x283);
    }
    league.set_table(0);
}

/// The plan of the turnovers (they free and destroy: they cannot run twice, only the original runs): the year up
/// `years`, not active, current stage −1 and the season open.
unsafe fn compare_nations_new_season(name: &str, this: League, years: i16, original: impl FnOnce()) {
    let year = this.year();
    original();
    let ok = this.year() == year.wrapping_add(years) && this.active() == 0 && this.current_stage() as i32 == -1 && this.season_closed() == 0;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x45 {}, +0x30 {}", name, this.addr(), year, this.year(), this.active(), this.current_stage() as i32));
}

pub(crate) unsafe extern "thiscall" fn compare_african_nations_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_nations_new_season("african_nations_start_new_season", League(this), 2, || o(this));
}

pub(crate) unsafe extern "thiscall" fn compare_asia_nations_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_nations_new_season("asia_nations_start_new_season", League(this), 4, || o(this));
}

/// Comparer for `euro_champ_start_new_season`: only the original, then the plan of the turnover, the entrant book
/// built again with 16 places and, when the rules were set up, no table.
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let league = League(this);
    compare_nations_new_season("euro_champ_start_new_season", league, 4, || o(this));
    let ok = league.entrant_book() != 0 && league.entrant_capacity() == 0x10 && league.stage_index() == stage_index::MAIN && league.club_count() == 0;
    SCOREBOARD.record(ok, || format!("euro_champ_start_new_season({:#x}): entrant book {:#x}, {} places", this, league.entrant_book(), league.entrant_capacity()));
}
