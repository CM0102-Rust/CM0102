// ---------- cx16_4: the European World Cup qualifying constructor (wc_europe_league.cpp) ----------

/// The European section's vtable (WcEuropeLeague).
pub(crate) const WC_EUROPE_LEAGUE_VTABLE: usize = 0x970cc4;

/// WcEuropeLeague routines the constructor calls directly (thiscall, ret 0): the rules and the 2002 campaign.
pub(crate) const FN_WC_EUROPE_LEAGUE_SET_UP_RULES: usize = 0x9191e0;
pub(crate) const FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN: usize = 0x91b760;

/// The European campaign year of a game year: the four-year cycle (`qualifying_campaign_year`), never before 2004.
pub(crate) fn wc_europe_campaign_year(year: i16) -> i16 {
    qualifying_campaign_year(year).max(0x7d4) // cx16_4: the European campaign, from 2004
}

/// The European World Cup qualifying at career creation.
///
/// League::League (`exe::new_league`), the vtable, the year `year` (word), the record (`league::COMPETITION`),
/// `league::CLUB_SIDES` 4, the African section above (the competition id of
/// [`KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION`]) and none below (−1), the dwords +0xf7, +0xee, +0xf2 and +0xfb
/// 0, one exempt nation (`WC_EUROPE_EXEMPT` 1), discipline 0; the campaign year (`wc_europe_campaign_year`); a new
/// list (`exe::crt_operator_new(0xe)` and LLIST::LLIST `exe::new_llist`, or 0) in `wc_cup_turnover_consts::EUROPE_LIST`
/// and importance 0xf. If the saved state loads (League::load_from_temp_file `exe::league_load_from_temp_file`(this, 1)),
/// returns this.
///
/// Otherwise `league::LEAGUE_FLAGS` 0x24; the entry pool (WcEuropeLeague::build_entry_pool 0x919e30; its result not
/// read); the stage count by the exempt count (0 → 11, 1 or 2 → 10, 3 → 9, any other kept); the main stage, no current
/// stage (−1), the stage vector (`exe::crt_malloc` of 4 × count when positive, else null), `league::ACTIVE` 0 (dword);
/// the rules (WcEuropeLeague::set_up_rules 0x9191e0; 0: the error box of line 0x7f and this); the statistics (operator
/// new of 0x5ce and Comp_Stats::Comp_Stats into `league::COMP_STATS`; null: the box of line 0x87 and this). A campaign
/// year not after the game's (never: the campaign is at least 2004 and the game starts in 2001) loads the 2002 campaign
/// (WcEuropeLeague::load_2002_campaign 0x91b760). Returns this. The SEH frame covers only the two operator new.
/// Original: `WcEuropeLeague::WcEuropeLeague` (0x918200, wc_europe_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_europe_league(this: usize, year: u32, record: usize) -> usize {
    use wc_cup_turnover_consts as wt;
    let league = League(this);
    exe::new_league(league);
    write_usize(this, WC_EUROPE_LEAGUE_VTABLE);
    league.set_year(year as i16);
    write_usize(this + league::COMPETITION, record);
    league.set_club_sides(4);
    league.set_upper_division_id(read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) as i32);
    league.set_lower_division_id(usize::MAX);
    write_u32(this + 0xf7, 0); // the drawn groups [inference]
    write_u32(this + 0xee, 0); // the entry pool list
    write_u32(this + 0xf2, 0); // its count
    write_u32(this + wt::EUROPE_FB, 0);
    write_u8(this + WC_EUROPE_EXEMPT, 1);
    league.set_discipline_rule(0);
    league.set_year(wc_europe_campaign_year(year as i16));
    let p = exe::crt_operator_new(0xe);
    write_usize(this + wt::EUROPE_LIST, if p != 0 { exe::new_llist(p) } else { 0 });
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this;
    }
    covered!("cx16_4 european qualifying");
    league.set_league_flags(0x24);
    call_section_routine(wt::FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL, this);
    match read_u8(this + WC_EUROPE_EXEMPT) {
        0 => league.set_stage_count(0xb),
        1 | 2 => league.set_stage_count(0xa),
        3 => league.set_stage_count(9),
        _ => {}
    }
    league.set_stage_index(stage_index::MAIN);
    league.set_current_stage(usize::MAX);
    let n = league.stage_count() as i32;
    league.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    league.set_active(0);
    if call_section_routine(FN_WC_EUROPE_LEAGUE_SET_UP_RULES, this) == 0 {
        show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x7f);
        return this;
    }
    if !qualifying_section_stats(league, wt::WC_EUROPE_LEAGUE_CPP, 0x87) {
        return this;
    }
    if year as i16 >= league.year() {
        call_section_routine(FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN, this);
    }
    this
}

/// Comparer for `construct_wc_europe_league`: it allocates, reads the save and builds the pool, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year (`wc_europe_campaign_year`), the
/// African section above and none below.
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_europe_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let r = o(this, year, record);
    let league = League(this);
    let expected = wc_europe_campaign_year(year as i16);
    let ok = r == this && read_usize(this) == WC_EUROPE_LEAGUE_VTABLE && league.competition().addr() == record && built_year_ok(league.year() as u16, expected as u16)
        && league.upper_division_id() as u32 == read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) && league.lower_division_id() as u32 == u32::MAX;
    SCOREBOARD.record(ok, || format!("construct_wc_europe_league({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}", this, year as i16, record, r, league.year(), expected, read_usize(this)));
    r
}
