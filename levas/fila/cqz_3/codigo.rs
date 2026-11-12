// ---------- cqz_3: the constructors of the Asian Cup, the Olympic football and the Confederations Cup (asia_nations.cpp, olympics.cpp, fifa_confed.cpp) ----------

/// The edition year of a tournament held every four years: from `start`, goes up one year at a time while the remainder
/// of (year − `base`) by 4 is not 0 (signed, as the original's `and 0x80000003` with its sign adjustment).
fn four_year_edition(start: i16, base: i32) -> i16 {
    let mut year = start;
    while (year as i32 - base) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The Asian Cup at career creation: the edition is the game's year moved to the next year of the four-year cycle from
/// 1996 (1996, 2000, 2004...: in a 2001 game, 2004); vtable 0x967614, 3 stages, the rules AsiaNations::set_up_rules
/// (0x40f4c0) and the error boxes of lines 0x53 and 0x5a of asia_nations.cpp (`build_nations_tournament`: League::League,
/// the record, `club_sides` 4, no upper or lower division, discipline rule 0, importance 0xf; the saved state if it
/// loads; otherwise `league_flags` 0x28, inactive, the main stage, the stage vector, the rules and the statistics).
/// Original: `AsiaNations::AsiaNations` (0x40e860, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 asian cup");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Asian Cup of the four-year cycle from 1996
    build_nations_tournament(League(this), edition, record, 0x967614, 3, 0x40f4c0, 0x987ba0, (0x53, 0x5a))
}

/// Comparer for `construct_asia_nations`: only the original runs (it allocates the stages and the statistics), then the
/// plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_nations_tournament_ctor("construct_asia_nations", League(this), record, 0x967614, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// The Olympic football at career creation: the edition is the game's year moved to the next Olympic year (the
/// four-year cycle from 1996: in a 2001 game, 2004); vtable 0x96e1d0, 4 stages, the rules Olympics::set_up_rules
/// (0x7a0230) and the error boxes of lines 0x56 and 0x5d of olympics.cpp (`build_nations_tournament`, the same body as
/// the Asian Cup's).
/// Original: `Olympics::Olympics` (0x79f530, olympics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_olympics(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 olympics");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Olympic year
    build_nations_tournament(League(this), edition, record, 0x96e1d0, 4, 0x7a0230, 0xa6a360, (0x56, 0x5d))
}

/// Comparer for `construct_olympics` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_olympics(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_nations_tournament_ctor("construct_olympics", League(this), record, 0x96e1d0, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// The Confederations Cup edition: from `start`, one year more while the year is even or is 2001 (an odd year other
/// than 2001: in a 2001 game, 2003).
fn confed_cup_edition(start: i16) -> i16 {
    let mut year = start;
    while (year as i32) % 2 == 0 || year == 0x7d1 {
        year = year.wrapping_add(1);
    }
    year
}

/// The Confederations Cup at career creation.
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); vtable 0x96a270, the record (`league::COMPETITION`),
/// `club_sides` 4, no upper or lower division (−1), the edition (`confed_cup_edition` of the game's year), discipline
/// rule 0 and importance 0xf. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`: push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: inactive, the main stage (0xff), `current_stage` −1, 2 stages and the stage vector (`exe::crt_malloc(8)`:
/// push; add esp -> cdecl, unchecked); then, unlike its sisters, the statistics before the rules: operator new
/// (`exe::crt_operator_new(0x5ce)`) and Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx,
/// new -> thiscall, ret 4) into `comp_stats`; null: the error box of line 0x53 of fifa_confed.cpp and no rules;
/// otherwise the rules FifaConfed::set_up_rules (0x58bdb0, thiscall, ret 0; its return value is not read). The rules set
/// `league_flags` (0x28). Returns this.
/// Original: `FifaConfed::FifaConfed` (0x58b310, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    exe::new_league(league);
    write_u32(this, 0x96a270);
    write_u32(this + league::COMPETITION, record as u32);
    league.set_club_sides(4);
    write_u32(this + league::UPPER_DIVISION_ID, 0xffff_ffff);
    write_u32(this + league::LOWER_DIVISION_ID, 0xffff_ffff);
    let edition = confed_cup_edition(year as i16); // cqz3: the Confederations Cup in an odd year after 2001
    league.set_year(edition);
    league.set_discipline_rule(0);
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_3 confederations cup");
    write_u32(this + league::ACTIVE, 0);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 2);
    write_u32(this + league::STAGES, exe::crt_malloc(8) as u32);
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, league) } else { 0 };
    write_u32(this + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(0x9c7474, 0x53);
    } else {
        let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x58bdb0usize);
        set_up_rules(this);
    }
    this
}

/// Comparer for `construct_fifa_confed` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_fifa_confed(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_nations_tournament_ctor("construct_fifa_confed", League(this), record, 0x96a270, confed_cup_edition(year as i16), || o(this, year, record))
}
