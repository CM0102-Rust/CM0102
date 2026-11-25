// ---------- rx10_11: the Club World Championship constructor, season turnover and next stage (world_club_champ.cpp) ----------

/// The Club World Championship's vtable.
pub(crate) const WORLD_CLUB_CHAMP_VTABLE: usize = 0x97119c;

/// The first edition year of the Club World Championship from `year` on: the first odd year that is 2003 or later
/// (signed words).
pub(crate) fn world_club_champ_first_year(mut year: i16) -> i16 {
    while year % 2 == 0 || year < 0x7d3 {
        year = year.wrapping_add(1);
    }
    year
}

/// Rule: the FIFA Club World Championship is played every two years, the first edition in the first odd year from 2003
/// on; it has three stages (two group rounds and the knockout), international club sides, no division above or below,
/// importance 15 and no discipline rule of its own (−1).
///
/// League::League (`exe::new_league`), the vtable 0x97119c, the year (word) and the competition record,
/// `league::CLUB_SIDES` = 2, `league::UPPER_DIVISION_ID` = `league::LOWER_DIVISION_ID` = −1; the year moved up to
/// `world_club_champ_first_year`; `league::DISCIPLINE_RULE` = 0xff and `league::IMPORTANCE` = 15; the saved state
/// (`exe::league_load_from_temp_file`(this, 1)) ends it. Otherwise the dword at `league::ACTIVE` = 0, the main stage
/// index, `league::CURRENT_STAGE` = −1, `league::STAGE_COUNT` = 3 and the stage vector (`exe::crt_malloc`(0xc),
/// unchecked); the statistics (`exe::crt_operator_new`(0x5ce), `exe::new_comp_stats`; none: the world_club_champ.cpp box
/// at line 0x54 and the constructor returns) and the rules (WorldClubChamp::set_up_rules 0x929bb0, called directly,
/// its result not read). The SEH frame only covers the new. Returns this.
/// Original: `WorldClubChamp::WorldClubChamp` (0x929140, world_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_world_club_champ(this: usize, year: u32, record: usize) -> usize {
    let l = League(this);
    exe::new_league(l);
    write_usize(this, WORLD_CLUB_CHAMP_VTABLE);
    l.set_year(year as u16 as i16);
    write_usize(this + league::COMPETITION, record);
    write_u8(this + league::CLUB_SIDES, 2);
    write_i32(this + league::UPPER_DIVISION_ID, -1);
    write_i32(this + league::LOWER_DIVISION_ID, -1);
    l.set_year(world_club_champ_first_year(l.year())); // rx10_11: the first odd year from 2003
    l.set_discipline_rule(0xff);
    write_u8(this + league::IMPORTANCE, 0xf);
    if exe::league_load_from_temp_file(l, 1) != 0 {
        return this;
    }
    covered!("rx10_11 new club world championship");
    write_u32(this + league::ACTIVE, 0); // the original writes a dword over ACTIVE and the next three bytes
    l.set_stage_index(stage_index::MAIN);
    l.set_current_stage(0xffff_ffff);
    l.set_stage_count(3);
    l.set_stages(exe::crt_malloc(0xc));
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, l) } else { 0 };
    l.set_comp_stats(s);
    if s == 0 {
        show_error_at(0xad5764, 0x54);
        return this;
    }
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x929bb0usize); // WorldClubChamp::set_up_rules
    set_up_rules(this);
    this
}

/// Comparer for `construct_world_club_champ`: only the original (it allocates and loads), then the plan: the vtable,
/// the record, the first odd year from 2003, the discipline rule 0xff and importance 15; when built, the 3 stages and
/// the statistics.
pub(crate) unsafe extern "thiscall" fn compare_construct_world_club_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let l = League(this);
    let loaded = league_will_load_saved_state(record);
    let r = o(this, year, record);
    let mut ok = r == this && read_usize(this) == WORLD_CLUB_CHAMP_VTABLE && l.competition().addr() == record && l.discipline_rule() == 0xff
        && read_u8(this + league::IMPORTANCE) == 0xf;
    if !loaded {
        ok = ok && l.year() == world_club_champ_first_year(year as u16 as i16) && (l.comp_stats() == 0 || (l.stage_count() == 3 && l.stages() != 0));
    }
    SCOREBOARD.record(ok, || format!("construct_world_club_champ({:#x}, {}): year {}, vtable {:#x}, stages {}", this, year as u16 as i16, l.year(), read_usize(this), l.stage_count()));
    r
}

/// Rule: when the Club World Championship ends, the table and stages are undone, the season goes to the statistics
/// history, the year goes up 2, new hosts are drawn (for it and the Confederations Cup) and the rules are set up
/// again.
///
/// `WorldClubChamp::start_new_season()` (virtual +8), the Oceania Club Championship's mould with its own order:
/// `league::SEASON_CLOSED` = 0, the table freed (`exe::league_free_table`(this, 0)), `league::ROUND_DATES` freed and
/// zeroed, the season archived (`exe::comp_stats_archive_season_and_reset`(stats, 1)), the stages 0..=current (signed,
/// read again on each pass) destroyed (virtual 0 with 1), the year up 2, `league::CURRENT_STAGE` = −1, the hosts
/// redrawn (HostCountry::redraw_world_club_and_confed_hosts 0x5fab70 on [`G_HOST_COUNTRY`], thiscall, ret 0), the dword
/// at `league::ACTIVE` = 0 and the rules (virtual `comp_vtable::SET_UP_RULES`, no box), whose eax it returns.
/// Original: `WorldClubChamp::start_new_season` (0x929d40, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_start_new_season(this: usize) -> u32 {
    let l = League(this);
    l.set_season_closed(0);
    exe::league_free_table(l, 0);
    if l.round_dates() != 0 {
        exe::crt_free(l.round_dates());
        l.set_round_dates(0);
    }
    if l.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(l.comp_stats(), 1);
    }
    let mut i = 0i32;
    while (l.current_stage() as i32) >= 0 && i <= l.current_stage() as i32 {
        let f = read_usize(l.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(f, 1);
        }
        i += 1;
    }
    covered!("rx10_11 club world championship turnover");
    l.set_year(l.year().wrapping_add(2)); // rx10_11: every two years
    l.set_current_stage(usize::MAX);
    let redraw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5fab70usize); // HostCountry::redraw_world_club_and_confed_hosts
    redraw(read_usize(G_HOST_COUNTRY));
    write_u32(this + league::ACTIVE, 0);
    bra_nat_first_call_virtual_no_args(this, comp_vtable::SET_UP_RULES)
}

/// Comparer for `world_club_champ_start_new_season`: only the original (it frees, draws hosts and rebuilds); the year
/// must go up 2 and the season be open.
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let l = League(this);
    let year = l.year();
    let r = o(this);
    let ok = l.year() == year.wrapping_add(2) && l.season_closed() == 0 && l.current_stage() as i32 == -1;
    SCOREBOARD.record(ok, || format!("world_club_champ_start_new_season({:#x}): year {} -> {}", this, year, l.year()));
    r
}

/// Rule: the Club World Championship moves on one stage when the current one ends; on reaching stage 2 (after the two
/// group rounds) it builds the knockout; going past the last stage is an error.
///
/// `WorldClubChamp::advance_stage()`: `league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1: the world_club_champ.cpp
/// box at line 0x2df and nothing else; otherwise the stage up one and, at 2, WorldClubChamp::create_knockout_stage
/// (0x92a140, thiscall, ret 0, called directly).
/// Original: `WorldClubChamp::advance_stage` (0x929dd0, world_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_club_champ_advance_stage(this: usize) {
    let l = League(this);
    let f = l.current_stage() as i32;
    if f >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0xad5764, 0x2df);
    }
    let f = f + 1;
    l.set_current_stage(f as usize);
    if f == 2 {
        covered!("rx10_11 club world championship knockout");
        let knockout: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x92a140usize); // WorldClubChamp::create_knockout_stage
        knockout(this);
    }
}

/// Comparer for `world_club_champ_advance_stage`: only the original (the knockout allocates and draws), with the plan
/// (`compare_state_league_advance_stage`: the stage up one, or unchanged at the last).
pub(crate) unsafe extern "thiscall" fn compare_world_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_advance_stage("world_club_champ_advance_stage", League(this), || o(this))
}
