// ---------- cqz_4: the constructors of the Asian, Oceania and South American World Cup qualifying (wc_asia_league.cpp, wc_oceania_league.cpp, wc_south_american_league.cpp) ----------

/// Calls a routine of the qualifying section by its address (thiscall, ret 0; through the hook when it is in Rust).
unsafe fn call_qualifier_routine(routine: usize, this: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(routine);
    f(this)
}

/// The first year of a World Cup qualifying campaign: a year one past the four-year cycle from 2000 goes back one year
/// (2001 → 2000); then up one year at a time until the remainder of (year − 2000) by 4 is 0 (signed, as the
/// original's `and 0x80000003` with its sign adjustment).
fn qualifying_campaign_year(year: i16) -> i16 {
    let mut y = if (year as i32 - 0x7d0) % 4 == 1 { year.wrapping_sub(1) } else { year };
    while (y as i32 - 0x7d0) % 4 != 0 {
        y = y.wrapping_add(1);
    }
    y
}

/// The fields every qualifying section writes before its saved state loads: League::League (`exe::new_league`, mov ecx
/// -> thiscall, ret 0), the vtable, the record (`league::COMPETITION`), `club_sides` 4, the sections above and below
/// (the competition ids read from the keys `upper` and `lower`: the qualifying sections are chained), discipline rule 0
/// and the campaign year.
unsafe fn start_qualifying_section(this: League, vtable: usize, record: usize, upper: usize, lower: usize, year: i16) {
    exe::new_league(this);
    write_u32(this.addr(), vtable as u32);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    this.set_club_sides(4);
    write_u32(this.addr() + league::UPPER_DIVISION_ID, read_u32(upper));
    write_u32(this.addr() + league::LOWER_DIVISION_ID, read_u32(lower));
    this.set_discipline_rule(0);
    this.set_year(year);
}

/// The statistics of a qualifying section: operator new (`exe::crt_operator_new(0x5ce)`: push; add esp -> cdecl) and
/// Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4) into
/// `comp_stats`; null: the error box of line `row` of source `file`. Returns whether they were created.
unsafe fn qualifying_section_stats(this: League, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + league::COMP_STATS, e as u32);
    if e == 0 {
        show_error_at(file, row);
    }
    e != 0
}

/// Shared comparer of the three constructors: they allocate the stages, the statistics and the entrant lists and read
/// the save, so only the original runs; then the plan: it returns this, the vtable, the record, the edition year and
/// the chained sections above and below.
unsafe fn compare_qualifying_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, upper: usize, lower: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let (u, l) = (read_u32(this.addr() + league::UPPER_DIVISION_ID), read_u32(this.addr() + league::LOWER_DIVISION_ID));
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && this.competition().addr() == record && read == year && u == read_u32(upper) && l == read_u32(lower);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}, sections {} {}", name, this.addr(), read, year, read_usize(this.addr()), u as i32, l as i32));
    r
}

/// The Asian World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970b70; above, the CONCACAF section; below, the Oceania section), 7 substitutes
/// (`+0x49`), 3 substitutions (`+0x4a`), the dword `+0xf7` = 0, the campaign year (`qualifying_campaign_year`, never
/// before 2004) and importance 0xa. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`), returns
/// this.
///
/// Otherwise: `league_flags` 0x28, inactive, the entrant list (WcAsiaLeague::build_entrant_list 0x913f60, thiscall, ret
/// 0; its return value is not read), the main stage, 13 stages (`current_stage` −1) and the stage vector
/// (`exe::crt_malloc(0x34)`, unchecked); the rules (WcAsiaLeague::set_up_rules 0x912eb0; 0: the error box of line 0x6f
/// and returns this) and the statistics (null: the box of line 0x77). Returns this.
/// Original: `WcAsiaLeague::WcAsiaLeague` (0x911bf0, wc_asia_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_asia_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the Asian campaign, from 2004
    start_qualifying_section(league, 0x970b70, record, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, edition);
    write_u8(this + league::SUBSTITUTES_NAMED, 7);
    write_u8(this + league::SUBSTITUTIONS_ALLOWED, 3);
    write_u32(this + 0xf7, 0);
    league.set_importance(0xa);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 asian qualifying");
    league.set_league_flags(0x28);
    write_u32(this + league::ACTIVE, 0);
    call_qualifier_routine(0x913f60, this);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::STAGE_COUNT, 0xd);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGES, exe::crt_malloc(0x34) as u32);
    if call_qualifier_routine(0x912eb0, this) == 0 {
        show_error_at(0xad4680, 0x6f);
        return this;
    }
    qualifying_section_stats(league, 0xad4680, 0x77);
    this
}

/// Comparer for `construct_wc_asia_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_asia_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_qualifying_ctor("construct_wc_asia_league", League(this), record, 0x970b70, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, || o(this, year, record))
}

/// The Oceania World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970d78; above, the Asian section; below, the South American section), the
/// campaign year (`qualifying_campaign_year`, no minimum), inactive and importance 0xa. If the saved state loads,
/// returns this.
///
/// Otherwise: `league_flags` 0x28, the entrant pool (WcOceaniaLeague::build_entrant_pool 0x91fa80, thiscall, ret 0;
/// return value not read), 3 stages, then the statistics (null: the error box of line 0x63 and returns this); the
/// predicted positions (WcOceaniaLeague::set_predicted_positions 0x91ff30, thiscall, ret 0), the main stage,
/// `current_stage` −1 and the stage vector (`exe::crt_malloc(4 × stage_count)` when the count is positive, else null);
/// the rules (WcOceaniaLeague::set_up_rules 0x91ecf0; 0: the box of line 0x75) and the group table
/// (WcOceaniaLeague::init_group_table 0x91ec00; 0: the box of line 0x7c). Returns this.
/// Original: `WcOceaniaLeague::WcOceaniaLeague` (0x91df90, wc_oceania_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16); // cqz4: the Oceania campaign
    start_qualifying_section(league, 0x970d78, record, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, edition);
    write_u32(this + league::ACTIVE, 0);
    let importance = 0xa; // cqz4: the importance of the Oceania qualifying matches
    league.set_importance(importance);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 oceania qualifying");
    league.set_league_flags(0x28);
    call_qualifier_routine(0x91fa80, this);
    write_u32(this + league::STAGE_COUNT, 3);
    if !qualifying_section_stats(league, 0xad4afc, 0x63) {
        return this;
    }
    call_qualifier_routine(0x91ff30, this);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    let n = read_u32(this + league::STAGE_COUNT) as i32;
    write_u32(this + league::STAGES, if n > 0 { exe::crt_malloc((n as u32) << 2) as u32 } else { 0 });
    if call_qualifier_routine(0x91ecf0, this) == 0 {
        show_error_at(0xad4afc, 0x75);
        return this;
    }
    if call_qualifier_routine(0x91ec00, this) == 0 {
        show_error_at(0xad4afc, 0x7c);
    }
    this
}

/// Comparer for `construct_wc_oceania_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%);
    compare_qualifying_ctor("construct_wc_oceania_league", League(this), record, 0x970d78, qualifying_campaign_year(year as i16),
        KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, || o(this, year, record))
}

/// The South American World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970e2c; above, the Oceania section; below, the African section), inactive and
/// the campaign year (`qualifying_campaign_year`, never before 2004); the importance is not written (it stays as
/// League::League left it). If the saved state loads, returns this.
///
/// Otherwise: `league_flags` 0x24, the main stage, `current_stage` −1, no stages (count 0, null vector); the exempt
/// nations (WcSouthAmericanLeague::count_exempt_nations 0x920d10, thiscall, ret 0); the rules
/// (WcSouthAmericanLeague::set_up_rules 0x921040; 0: the error box of line 0x65 and returns this); the team slots
/// (WcSouthAmericanLeague::build_team_slots 0x920eb0; 0: the box of line 0x6c and returns this); the statistics (null:
/// the box of line 0x73 and returns this). Then, when the game's year (the argument, signed word) is not before the
/// edition, the real 2000-2001 campaign (WcSouthAmericanLeague::load_2002_campaign 0x9216d0, thiscall, ret 0). Returns
/// this.
/// Original: `WcSouthAmericanLeague::WcSouthAmericanLeague` (0x9202b0, wc_south_american_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_south_american_league(this: usize, year: u32, record: usize) -> usize {
    let league = League(this);
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the South American campaign, from 2004
    start_qualifying_section(league, 0x970e2c, record, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, edition);
    write_u32(this + league::ACTIVE, 0);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cqz_4 south american qualifying");
    league.set_league_flags(0x24);
    league.set_stage_index(stage_index::MAIN);
    write_u32(this + league::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this + league::STAGE_COUNT, 0);
    write_u32(this + league::STAGES, 0);
    call_qualifier_routine(0x920d10, this);
    if call_qualifier_routine(0x921040, this) == 0 {
        show_error_at(0xad4ce0, 0x65);
        return this;
    }
    if call_qualifier_routine(0x920eb0, this) == 0 {
        show_error_at(0xad4ce0, 0x6c);
        return this;
    }
    if !qualifying_section_stats(league, 0xad4ce0, 0x73) {
        return this;
    }
    if year as i16 >= league.year() {
        covered!("cqz_4 south american 2002 campaign");
        call_qualifier_routine(0x9216d0, this);
    }
    this
}

/// Comparer for `construct_wc_south_american_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_south_american_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%);
    compare_qualifying_ctor("construct_wc_south_american_league", League(this), record, 0x970e2c, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, || o(this, year, record))
}
