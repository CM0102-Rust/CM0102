use super::*;

/// rule: the Asian groups are home and away, 3/1 points, tiebreak 1, 2 and 3, each group winner goes on, category 10,
/// starts on 25 of month 2 on weekday 5, +0xb8 = 1 [inference: the stage starts the following year]
/// Original: `WcAsiaLeague::set_up_rules` (0x912eb0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_up_rules(this: usize) -> u32 {
    covered!("cqj_12 asia rules");
    wc_europe_league_write_rule_fields(League(this), &ASIA_QUALIFIER_RULE_FIELDS);
    let day = 0x19; // cqj12: the start day of the Asian qualifiers
    let word = comp_date_word(League(this), day, 2, 5);
    finish_rules_setup(League(this), word, 1)
}

/// The season turnover of the Asian World Cup qualifying: last campaign's table, round dates, national-team list and
/// stages freed, the season archived, the next campaign's year, the national teams again and the rules.
///
/// `qualifying_turnover_free_table`; the national-team list (`ENTRANT_LIST`, +0xef) freed when there (not zeroed: the
/// entrant routine writes it again); the stages destroyed (`destroy_comp_stages`) and `.set_current_stage()` = −1; the
/// season archived in the statistics (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1), thiscall, ret 4);
/// the year up by four until it reaches the game's (`advance_qualifying_campaign`); `.set_active(0)` and the current
/// stage −1 again; the national teams (WcAsiaLeague::build_entrant_list, thiscall, ret 0; its return not read) and the
/// rules (virtual `comp_vtable::SET_UP_RULES`): 0 shows the box (wc_asia_league.cpp, line 0x45e) and the copy returns
/// 0 (the original the box's eax); otherwise its eax.
/// Original: `WcAsiaLeague::start_new_season` (0x913360, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let list = read_usize(this.addr() + wt::ENTRANT_LIST);
    if list != 0 {
        exe::crt_free(list);
    }
    destroy_comp_stages(this);
    this.set_current_stage(usize::MAX);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 asian qualifying rollover");
    this.set_active(0);
    this.set_current_stage(usize::MAX);
    call_qualifier_routine(wt::FN_WC_ASIA_LEAGUE_BUILD_ENTRANT_LIST, this.addr());
    let r = call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr());
    if r == 0 {
        show_error_at(wt::WC_ASIA_LEAGUE_CPP, 0x45e);
        return 0;
    }
    r
}

/// Comparer for `wc_asia_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_START_NEW_SEASON);
    compare_qualifying_turnover("wc_asia_league_start_new_season", League(this), || o(this))
}

/// rule: on the draw date of the Asian qualifying, with the full day update and the league not yet started, the first
/// round groups are drawn, the first group gets its table and fixtures and the nine other groups their stages; before
/// the draw nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the draw (WcAsiaLeague::draw_first_round_groups 0x912bd0, thiscall, ret 0), the first
/// group table (WcAsiaLeague::init_first_group_table 0x912ad0; 0: the error box of line 0x40c and 0), the fixtures
/// (`exe::league_create_fixtures`; 0: the box of line 0x412 and 0), the sorted table (`exe::league_sort_table(this,
/// 0)`; 0: the box of line 0x419 and 0), the groups 0 to 8 (WcAsiaLeague::create_first_round_group 0x913540: push k;
/// mov ecx -> thiscall, ret 4), the predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the drawn
/// list +0xf7 freed (`exe::crt_free`: push; add esp -> cdecl) and zeroed and `league::ACTIVE` = 1. Then the league day
/// with `full`, whose al it passes on.
/// Original: `WcAsiaLeague::update_for_date` (0x913140, wc_asia_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let league = League(this);
    covered!("cqz_12 asian day");
    let (before, draw_day) = qualifying_day_before_draw(league, today);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if before {
        return 0;
    }
    if draw_day && league.active() == 0 && full != 0 {
        covered!("cqz_12 asian draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WCASIALEAGUE_DRAW_FIRST_ROUND_GROUPS);
        draw(this);
        if exe::wcasialeague_init_first_group_table(this) == 0 {
            show_error_at(0xad4680, 0x40c);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4680, 0x412);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4680, 0x419);
            return 0;
        }
        for k in 0..9 {
            exe::wcasialeague_create_first_round_group(this, k);
        }
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xf7));
        write_u32(this + 0xf7, 0);
        league.set_active(1);
    }
    exe::league_update_for_date(this, today, full) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && qualifying_day_before_draw(league, today).1;
    let r = o(this, today, full);
    if draws {
        let ok = league.active() == 1 && read_usize(this + 0xf7) == 0 && league.table() != 0;
        SCOREBOARD.record(ok, || format!("wc_asia_league_update_for_date({:#x}): the draw did not build the groups", this));
    }
    r
}

/// rule: the Asian World Cup qualifying entrants: 44 places minus the Asian nations exempt (the hosts of the next World
/// Cup and the last winner when Asian), filled with the other Asian national teams in database order, sorted by
/// reputation; at most 40 go on
///
/// +0xf3 (the entrant count, dword) = 0x2c, the exempt count +0xee = 0, `league::GAMES_PER_CLUB` = −1. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x5e1 and no hosts; otherwise the hosts of the World Cup two years later
/// (HostCountry::find_hosts: 5 pushes; mov ecx -> thiscall, ret 0x14; the year with garbage in the high 16 bits, read
/// as a word); for each (k signed byte below the count) up to the first −1, an Asian one (`KEY_CONTINENT_ASIA`) adds one
/// to +0xee. The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup record, 0)): its nation, with
/// a continent, Asian and neither host, adds one. +0xf3 = 0x2c − exempt (signed byte) and the list +0xef
/// (`exe::crt_malloc` of 4 × count: cdecl; out of memory, the box of line 0x610 and 0). Over the nations (i word
/// signed), each Asian one while the list (dword) is below the count: its national team (`exe::database_get_national_team`),
/// when the nation is neither host and the team is not the winner, goes into the list. Short of the count it returns 0
/// (unsorted); full, the list sorted by reputation (qsort `exe::crt_qsort`(list, n, 4, 0x4b07d0)), the count capped
/// at 40 (as a signed word) and 1.
/// Original: `WcAsiaLeague::build_entrant_list` (0x913f60, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_build_entrant_list(this: usize) -> u32 {
    let league = League(this);
    let exempt = this + 0xee;
    write_u32(this + 0xf3, 0x2c);
    write_u8(exempt, 0);
    league.set_games_per_club(-1);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4680, 0x5e1);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (league.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            let c = Nation(read_usize(G_NATION_TABLE).wrapping_add((id as usize).wrapping_mul(nation::SIZE))).continent();
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) {
                write_u8(exempt, read_u8(exempt).wrapping_add(1)); // cqz12: an Asian host is exempt
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let winner = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if winner != 0 {
        let nation = Club(winner).nation();
        if !nation.is_null() {
            let c = nation.continent();
            let id = read_usize(nation.addr()) as i32;
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) && id != hosts[0] && id != hosts[1] {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
        }
    }
    let n = 0x2c - read_u8(exempt) as i8 as i32; // cqz12: 44 places minus the exempt
    write_u32(this + 0xf3, n as u32);
    let list = exe::crt_malloc((n as u32).wrapping_mul(4));
    write_u32(this + 0xef, list as u32);
    if list == 0 {
        show_error_at(0xad4680, 0x610);
        return 0;
    }
    covered!("cqz_12 asian entrants");
    let mut count = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA) && count < read_u32(this + 0xf3) as i32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                count += 1;
                write_usize(read_usize(this + 0xef) + (count as usize - 1) * 4, team);
            }
        }
        i = i.wrapping_add(1);
    }
    if count != read_u32(this + 0xf3) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this + 0xef), count as u32, 4, 0x4b07d0);
    let capped = (read_u32(this + 0xf3) as i32).min(0x28) as i16; // cqz12: at most 40 go on
    write_u32(this + 0xf3, capped as i32 as u32);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_entrant_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_BUILD_ENTRANT_LIST);
    let r = o(this);
    let n = read_u32(this + 0xf3) as i32;
    let list = read_usize(this + 0xef);
    let ok = r == 0 || (n <= 0x28 && list != 0 && (0..n.max(0) as usize).all(|k| {
        let nation = Club(read_usize(list + k * 4)).nation();
        let c = if nation.is_null() { 0 } else { nation.continent() };
        c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_ASIA)
    }));
    SCOREBOARD.record(ok, || format!("wc_asia_league_build_entrant_list({:#x}) returned {} with {} entrants", this, r, n));
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
    let this = League(this);
    let league = this;
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the Asian campaign, from 2004
    start_qualifying_section(league, 0x970b70, record, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, edition);
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3);
    write_u32(this.addr() + 0xf7, 0);
    league.set_importance(0xa);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqz_4 asian qualifying");
    league.set_league_flags(0x28);
    this.set_active((0) as usize);
    call_qualifier_routine(0x913f60, this.addr());
    league.set_stage_index(stage_index::MAIN);
    this.set_stage_count((0xd) as usize);
    this.set_current_stage((0xffff_ffff) as usize);
    this.set_stages((exe::crt_malloc(0x34) as u32) as usize);
    if call_qualifier_routine(0x912eb0, this.addr()) == 0 {
        show_error_at(0xad4680, 0x6f);
        return this.addr();
    }
    qualifying_section_stats(league, 0xad4680, 0x77);
    this.addr()
}

/// Comparer for `construct_wc_asia_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_asia_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_ASIA_LEAGUE);
    compare_qualifying_ctor("construct_wc_asia_league", League(this), record, 0x970b70, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, || o(this, year, record))
}

/// rule: the Asian World Cup qualifying has first-round groups (stages up to 8, ids 0x3fe + stage) of six rounds
/// between 30 April and 25 May, second-round groups (stages 9 and 10) of ten rounds between 17 August and 19 October,
/// the two-legged tie of the group runners-up (stage 11: 26 October) and the inter-continental play-off (stage 12:
/// draw 27/10, 11 November); stages from 13 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcAsiaLeague::create_schedule` (0x911f00, wc_asia_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_asia_league_phase(stage), rounds, extra, league, 0xad4680)
}

/// Comparer for `wc_asia_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_ASIA_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_asia_league_create_schedule", stage, wc_asia_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_asia_league_create_schedule(this, stage, r, e, l))
}

/// Destroys the stages 0..=`current_stage` (signed, read again on each pass) of a competition (virtual 0 with 1: the
/// destructor that frees), as the turnovers do; the stage vector itself stays.
pub(crate) unsafe fn destroy_comp_stages(this: League) {
    let mut i = 0i32;
    while (this.current_stage() as i32) >= 0 && i <= this.current_stage() as i32 {
        let f = read_usize(this.stages() + i as usize * 4);
        if f != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
            destroy(f, 1);
        }
        i += 1;
    }
}

/// The start of a qualifying section's turnover: the season reopened (`.set_season_closed(0)`), the table freed (`exe::league_free_table` League::free_table: push 0; mov ecx, this -> thiscall, ret 4) and the
/// round dates (`.round_dates()`) freed (`exe::crt_free`) and zeroed.
pub(crate) unsafe fn qualifying_turnover_free_table(this: League) {
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
}

/// The day of a World Cup qualifying section, before its draw: (true, al 0) when today is before the draw date (today's
/// year lower, unsigned; or the same year and the day lower, signed), and whether today is the draw day.
pub(crate) unsafe fn qualifying_day_before_draw(this: League, today: usize) -> (bool, bool) {
    let (day, year) = oceania_nations_draw_date(this);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    let before = today_year < year || (today_year == year && (today_day as i16) < day as i16);
    (before, today_year == year && today_day == day)
}

/// The first year of a World Cup qualifying campaign: a year one past the four-year cycle from 2000 goes back one year
/// (2001 → 2000); then up one year at a time until the remainder of (year − 2000) by 4 is 0 (signed, as the
/// original's `and 0x80000003` with its sign adjustment).
pub(crate) fn qualifying_campaign_year(year: i16) -> i16 {
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
pub(crate) unsafe fn start_qualifying_section(this: League, vtable: usize, record: usize, upper: usize, lower: usize, year: i16) {
    exe::new_league(this);
    write_u32(this.addr(), vtable as u32);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    this.set_club_sides(4);
    this.set_upper_division_id((read_u32(upper)) as i32);
    this.set_lower_division_id((read_u32(lower)) as usize);
    this.set_discipline_rule(0);
    this.set_year(year);
}

/// The statistics of a qualifying section: operator new (`exe::crt_operator_new(0x5ce)`: push; add esp -> cdecl) and
/// Comp_Stats::Comp_Stats (`exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4) into
/// `comp_stats`; null: the error box of line `row` of source `file`. Returns whether they were created.
pub(crate) unsafe fn qualifying_section_stats(this: League, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    this.set_comp_stats((e as u32) as usize);
    if e == 0 {
        show_error_at(file, row);
    }
    e != 0
}

/// Shared comparer of the three constructors: they allocate the stages, the statistics and the entrant lists and read
/// the save, so only the original runs; then the plan: it returns this, the vtable, the record, the edition year and
/// the chained sections above and below.
pub(crate) unsafe fn compare_qualifying_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, upper: usize, lower: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let (u, l) = ((this.upper_division_id() as u32), (this.lower_division_id() as u32));
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && this.competition().addr() == record && built_year_ok(read as u16, year as u16) && u == read_u32(upper) && l == read_u32(lower);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}, sections {} {}", name, this.addr(), read, year, read_usize(this.addr()), u as i32, l as i32));
    r
}

/// The round table of stage `stage` (signed byte) of cqz_7 asian qualifying round table, or none.
pub(crate) fn wc_asia_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=8 => Some(GoldcupStagePlan { league: 1, rounds: 6, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x186, row: 0xc0,
            ops: &WC_ASIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        9..=10 => Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x28a, row: 0xea,
            ops: &WC_ASIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        11 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0x1e, extra_check: false, size: 0x68, row: 0x120,
            ops: &WC_ASIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        12 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x14b,
            ops: &WC_ASIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        13..=127 => None,
    }
}

/// cqz_7 asian qualifying round table, the operations of round table 1 (from the original, in order).
pub(crate) const WC_ASIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0x1e, 3, 1, 0, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 4, 4, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 7, 4, 1, 0, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0xb, 4, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x12, 4, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x19, 4, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];

/// cqz_7 asian qualifying round table, the operations of round table 2 (from the original, in order).
pub(crate) const WC_ASIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 0x11, 7, 1, 4, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x18, 7, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 0x1f, 7, 1, 4, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 7, 8, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0xe, 8, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x15, 8, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 0x1c, 8, 1, 4, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 5, 9, 1, 4, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 0xc, 9, 1, 4, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0x13, 9, 1, 4, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];

/// cqz_7 asian qualifying round table, the operations of round table 3 (from the original, in order).
pub(crate) const WC_ASIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x14, 9, 1, 5), Cqp12J(0, 0x1a, 9, 1, 4, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x3), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x0),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];

/// cqz_7 asian qualifying round table, the operations of round table 4 (from the original, in order).
pub(crate) const WC_ASIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1b, 9, 1, 5), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2),
    Cqp12B(0x21, 0x2), Cqp12W(0x7, 0x0), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa),
    Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0),
    Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0),
];

/// The team count of an Asian first round group (the group −1 to 8; −1 is the league's own table), by the exempt count
/// (`WC_ASIA_EXEMPT`): none exempt, four in every group; one, three in group 1; two, three in groups 3 and 6; three,
/// three in groups 1, 3 and 5; otherwise four. Any count above 3 or negative shows the box at line 0x37f first.
/// Original: `WcAsiaLeague::get_first_round_group_size` (0x912db0, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    let three = match read_u8(this + WC_ASIA_EXEMPT) as i8 {
        0 => false,
        1 => g == 1,
        2 => g == 3 || g == 6, // rx09_16: two exempt, groups 3 and 6 of three
        3 => g == 1 || g == 3 || g == 5,
        _ => {
            show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x37f);
            false
        }
    };
    if three { 3 } else { 4 }
}

/// Comparer for `wc_asia_league_get_first_round_group_size`: the answers side by side (the box only on the original's
/// side for a bad exempt count).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_get_first_round_group_size(this: usize, group: u32) -> i16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_WC_ASIA_LEAGUE_GET_FIRST_ROUND_GROUP_SIZE)(this, group);
    let e = read_u8(this + WC_ASIA_EXEMPT) as i8;
    if (0..=3).contains(&e) {
        let c = wc_asia_league_get_first_round_group_size(this, group);
        SCOREBOARD.record(r == c, || format!("wc_asia_league_get_first_round_group_size(exempt {}, group {}) original={} rust={}", e, group as u8 as i8, r, c));
    }
    r
}

/// Draws the ten Asian first round groups: the group order 0..9 (a local array) shuffled by 20 swaps of two randomly
/// drawn places (`exe::utils_random`(10) twice: the first place, then the second); the draw (`exe::crt_malloc` of 0xa0,
/// ten groups of four dwords, into `WC_ASIA_DRAW`; none: the box at line 0x340 and nothing more). Then up to four pots
/// while fewer than the entrant count (`WC_ASIA_ENTRANT_COUNT`, signed byte against the dword) are placed: for each
/// group in the shuffled order, while some remain, group g takes the next entrant (`wc_turnover_consts::ENTRANT_LIST`)
/// into its next slot if it has fewer than its size (`wc_asia_group_size`(g − 1)).
/// Original: `WcAsiaLeague::draw_first_round_groups` (0x912bd0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_draw_first_round_groups(this: usize) {
    let mut taken = [0u8; 10];
    let mut order: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    for _ in 0..0x14 {
        let a = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        let b = exe::utils_random(0xa) as u8 as i8 as isize as usize;
        order.swap(a, b); // rx09_16: the group order shuffled
    }
    let draw = exe::crt_malloc(0xa0);
    write_usize(this + WC_ASIA_DRAW, draw);
    if draw == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x340);
    }
    covered!("rx09_16 asian first round draw");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let mut placed: i8 = 0;
    for _pot in 0..4 {
        if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
            break;
        }
        for &g in &order {
            if placed as i32 >= read_i32(this + WC_ASIA_ENTRANT_COUNT) {
                break;
            }
            if g >= 0xa {
                continue;
            }
            let c = taken[g as usize];
            if (c as i8 as i16) < wc_asia_group_size(this, (g as i8).wrapping_sub(1)) {
                write_usize(draw + 4 * (c as i8 as usize + 4 * g as usize), read_usize(list + 4 * placed as usize));
                placed = placed.wrapping_add(1);
                taken[g as usize] = c.wrapping_add(1);
            }
        }
    }
}

/// Comparer for `wc_asia_league_draw_first_round_groups`: only the original runs (it draws and allocates); every
/// entrant drawn once into a slot within its group's size (`compare_wc_asia_draw`).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_draw_first_round_groups(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_DRAW_FIRST_ROUND_GROUPS)(this);
    compare_wc_asia_draw("wc_asia_league_draw_first_round_groups", this);
}

/// Builds the table of the first Asian first round group (the league's own, group −1): `club_count` = its size
/// (`wc_asia_group_size`(−1)), the table (`exe::crt_malloc` of 0x3b a team; none: the box at line 0x30d and 0) and a row
/// for each of the group's drawn national teams (League::init_table_row `exe::league_init_table_row`(this, k, slot k of
/// the draw, 0, 0)); then the group's draw news (League::send_group_draw_news `exe::league_send_group_draw_news`).
/// Returns 1.
/// Original: `WcAsiaLeague::init_first_group_table` (0x912ad0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(wc_asia_group_size(this, -1));
    let t = exe::crt_malloc((l.club_count() as i32 * league_table_row::SIZE as i32) as u32);
    l.set_table(t);
    if t == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x30d);
        return 0;
    }
    covered!("rx09_16 asian first group table");
    let draw = read_usize(this + WC_ASIA_DRAW);
    let mut k: u8 = 0;
    while (k as i8 as i16) < l.club_count() {
        exe::league_init_table_row(l, k as u32, read_usize(draw + 4 * k as i8 as usize), 0, 0); // rx09_16: the k-th team of the first group
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(l);
    1
}

/// Comparer for `wc_asia_league_init_first_group_table`: only the original runs (it allocates the table and sends the
/// news); the club count, the result and the rows against the first group of the draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_init_first_group_table(this: usize) -> u32 {
    let n = wc_asia_group_size(this, -1).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|k| read_usize(draw + 4 * k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_INIT_FIRST_GROUP_TABLE)(this);
    let l = League(this);
    let found: Vec<usize> = if l.table() != 0 { (0..n).map(|k| aus_nsl_table_row_club(l, k)).collect() } else { Vec::new() };
    SCOREBOARD.record(r == 1 && l.club_count() as usize == n && found == plan, || format!("wc_asia_league_init_first_group_table({:#x}) original={} plan={:x?} table={:x?}", this, r, plan, found));
    r
}

/// Builds Asian first round group k (0 to 8: groups 1 to 9, after the first in the league's own table) as stage k: its
/// size (`wc_asia_group_size`(k)), a list of that many (`exe::crt_malloc`; none: the box at line 0x49a and 0) with the
/// group's drawn teams (slots of group k + 1 in `WC_ASIA_DRAW`), its round table (`stage_round_table`(k)) and the
/// League_Stage (`qualifying_league_stage`: the league's own places from `league::PROMOTED_COUNT`, importance and rule
/// bits) into slot k; the list and the round table freed. No stage: the box at line 0x4b0 and 0. Otherwise the current
/// stage = k, the group's draw news (League::send_group_draw_news on the stage) and 1.
/// Original: `WcAsiaLeague::create_first_round_group` (0x913540, wc_asia_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let this = League(this);
    let l = this;
    let size = wc_asia_group_size(this.addr(), k as u8 as i8);
    let n = size as i32;
    let list = exe::crt_malloc((n * 4) as u32);
    if list == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x49a);
        return 0;
    }
    let draw = read_usize(this.addr() + WC_ASIA_DRAW);
    for i in 0..n.max(0) as usize {
        write_usize(list + 4 * i, read_usize(draw + 16 * (k as usize + 1) + 4 * i)); // rx09_17: the group's drawn teams
    }
    let (table, rounds, extra) = stage_round_table(l, k);
    let stage = qualifying_league_stage(l, size as u16 as u32, list, table, rounds, extra, this.addr() + league::PROMOTED_COUNT, k, l.importance() as u32, l.league_flags() as u16 as u32);
    let slot = l.stages() + 4 * k as usize;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(slot) == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4b0);
        return 0;
    }
    covered!("rx09_17 asian first round group");
    l.set_current_stage(k as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_asia_league_create_first_round_group`: only the original runs (it allocates the stage and sends the
/// news); with a stage built, its teams (as a set) and the current stage against the group's draw.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_first_round_group(this: usize, k: u32) -> u32 {
    let l = League(this);
    let n = wc_asia_group_size(this, k as u8 as i8).max(0) as usize;
    let draw = read_usize(this + WC_ASIA_DRAW);
    let plan: Vec<usize> = (0..n).map(|i| read_usize(draw + 16 * (k as usize + 1) + 4 * i)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_CREATE_FIRST_ROUND_GROUP)(this, k);
    if r == 1 {
        compare_category_b_stage("wc_asia_league_create_first_round_group", l, k as usize, &plan);
    }
    r
}

/// Rule: the next stage of the Asian qualifying: past the last stage, the error box; otherwise the current stage goes
/// up by one; at 9 (the first round is over) the two second round groups are built, at 11 the third round tie, at 12
/// the play-off.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box at line 0x471 and nothing
/// else. Otherwise the current stage + 1; at 9, WcAsiaLeague::create_second_round_groups (0x913740); at 11,
/// WcAsiaLeague::create_third_round_tie (0x913b60); at 12, WcAsiaLeague::create_playoff_tie (0x913d50) (thiscall,
/// ret 0).
/// Original: `WcAsiaLeague::advance_stage` (0x913490, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x471);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("rx09_17 asian next stage");
    match stage {
        9 => call_on_league(0x913740, l),
        11 => call_on_league(0x913b60, l),
        12 => call_on_league(0x913d50, l),
        _ => {}
    }
}

/// Comparer for `wc_asia_league_advance_stage` (`compare_advance_stage_building`: stages 9, 11 and 12 build).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_ADVANCE_STAGE);
    compare_advance_stage_building("wc_asia_league_advance_stage", League(this), |s| s == 9 || s == 11 || s == 12, || o(this), || wc_asia_league_advance_stage(this))
}

/// Builds the two Asian second round groups (stages 9 and 10, five teams each). A list of five (`exe::crt_malloc` of
/// 0x14; none: the box at line 0x4d9 and 0) takes five first round winners drawn at random (`exe::utils_random`(10):
/// the group r − 1, drawn again when taken; `wc_asia_first_round_winner`); its round table (`stage_round_table`(9)) and
/// the League_Stage (`qualifying_league_stage`: five teams, `wc_asia_second_round_places`, stage 9, importance 0xf,
/// rule bits 4) into slot 9; the list and the round table freed; none: the box at line 0x502 and 0. Otherwise the
/// group's draw news, the current stage = 9, and the other five winners in first round order into a second list (none:
/// the box at line 0x510 and 0) for stage 10 the same way (the box at line 0x534); then the current stage = 10, its draw
/// news and 1.
/// Original: `WcAsiaLeague::create_second_round_groups` (0x913740, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let places = wc_asia_second_round_places(this);
    let mut taken = [0u8; 10];
    let first = exe::crt_malloc(0x14);
    if first == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x4d9);
        return 0;
    }
    let mut placed = 0usize;
    while placed < 5 {
        let r = exe::utils_random(0xa) as usize;
        if taken[r] == 0 {
            write_usize(first + 4 * placed, wc_asia_first_round_winner(l, r as i32 - 1));
            placed += 1;
            taken[r] = 1;
        }
    }
    let build = |list: usize, stage: u32| {
        let (table, rounds, extra) = stage_round_table(l, stage);
        let p = qualifying_league_stage(l, 5, list, table, rounds, extra, places.as_ptr() as usize, stage, 0xf, 4);
        write_usize(l.stages() + 4 * stage as usize, p);
        exe::crt_free(list);
        exe::crt_free(table);
        read_usize(l.stages() + 4 * stage as usize)
    };
    let stage = build(first, 9);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x502);
        return 0;
    }
    covered!("rx09_17 asian second round groups");
    exe::league_send_group_draw_news(stage);
    l.set_current_stage(9);
    let second = exe::crt_malloc(0x14);
    if second == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x510);
        return 0;
    }
    let mut placed = 0usize;
    for k in -1i32..9 {
        if taken[(k + 1) as usize] == 0 {
            write_usize(second + 4 * placed, wc_asia_first_round_winner(l, k)); // rx09_17: the winners not drawn into the first group
            placed += 1;
        }
    }
    let stage = build(second, 10);
    if stage == 0 {
        show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x534);
        return 0;
    }
    l.set_current_stage(10);
    exe::league_send_group_draw_news(stage);
    1
}

/// Comparer for `wc_asia_league_create_second_round_groups`: only the original runs (it draws and allocates); the two
/// groups together hold the ten first round winners, five each, and the current stage is 10.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_second_round_groups(this: usize) -> u32 {
    let l = League(this);
    let mut winners: Vec<usize> = (-1i32..9).map(|k| wc_asia_first_round_winner(l, k)).collect();
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_ASIA_LEAGUE_CREATE_SECOND_ROUND_GROUPS)(this);
    if r == 1 {
        let (a, b) = (League(read_usize(l.stages() + 4 * 9)), League(read_usize(l.stages() + 4 * 10)));
        let (ca, cb) = (category_b_stage_clubs(a), category_b_stage_clubs(b));
        let mut found: Vec<usize> = ca.iter().chain(cb.iter()).copied().collect();
        found.sort_unstable();
        winners.sort_unstable();
        SCOREBOARD.record(ca.len() == 5 && cb.len() == 5 && found == winners && l.current_stage() == 10, || format!("wc_asia_league_create_second_round_groups({:#x}) winners {:x?} found {:x?} current {}", this,
            winners, found, l.current_stage() as i32));
    }
    r
}

/// Builds the Asian third round tie as stage 11: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x552)
/// with `wc_asia_third_round_sides`, the round table of stage 11 (`stage_round_table`), the Cup_Stage (`league_cup_stage`:
/// two teams, stage 11, 4) into slot 11; the list and the round table freed; none: the box at line 0x572. Otherwise its
/// round set up (Cup::set_up_round `exe::cup_set_up_round`(stage, 0)) and the current stage = 11.
/// Original: `WcAsiaLeague::create_third_round_tie` (0x913b60, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_third_round_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x552);
    }
    let sides = wc_asia_third_round_sides(this);
    write_usize(list, sides[0]);
    write_usize(list + 4, sides[1]);
    let (table, rounds, extra) = stage_round_table(l, 0xb);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xb, 4, 0);
    write_usize(l.stages() + 4 * 0xb, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xb) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x572);
    }
    covered!("rx09_18 asian third round tie");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xb), 0);
    l.set_current_stage(0xb);
}

/// Comparer for `wc_asia_league_create_third_round_tie`: only the original runs (it allocates); the tie's teams and the
/// current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_third_round_tie(this: usize) {
    let plan = wc_asia_third_round_sides(this);
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_CREATE_THIRD_ROUND_TIE)(this);
    compare_league_cup_stage("wc_asia_league_create_third_round_tie", League(this), 0xb, &plan);
}

/// Builds the Asian play-off as stage 12: a list of two (`exe::crt_malloc` of 8; none: the box at line 0x591) with the
/// Asian side (`wc_asia_play_off_side`; when none, the slot is left as malloc returned it) and the European one
/// (`wc_asia_play_off_european_side`); the round table of stage 12, the Cup_Stage (`league_cup_stage`: two teams, stage
/// 12, 4) into slot 12; the list and the round table freed; none: the box at line 0x5be. Otherwise its round set up
/// (`exe::cup_set_up_round`(stage, 0)) and the current stage = 12.
/// Original: `WcAsiaLeague::create_playoff_tie` (0x913d50, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_playoff_tie(this: usize) {
    let l = League(this);
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x591);
    }
    if let Some(side) = wc_asia_play_off_side(this) {
        write_usize(list, side);
    }
    write_usize(list + 4, wc_asia_play_off_european_side());
    let (table, rounds, extra) = stage_round_table(l, 0xc);
    let p = league_cup_stage(l, 2, list, table, rounds, extra, 0xc, 4, 0);
    write_usize(l.stages() + 4 * 0xc, p);
    exe::crt_free(list);
    exe::crt_free(table);
    if read_usize(l.stages() + 4 * 0xc) == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x5be);
    }
    covered!("rx09_18 asian play-off");
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 0xc), 0);
    l.set_current_stage(0xc);
}

/// Comparer for `wc_asia_league_create_playoff_tie`: only the original runs (it allocates); with an Asian side, the
/// tie's teams and the current stage against the plan.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_playoff_tie(this: usize) {
    let side = wc_asia_play_off_side(this);
    let europe = wc_asia_play_off_european_side();
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_CREATE_PLAYOFF_TIE)(this);
    if let Some(side) = side {
        compare_league_cup_stage("wc_asia_league_create_playoff_tie", League(this), 0xc, &[side, europe]);
    }
}

/// The predicted positions of the Asian qualifying: without statistics, the box at line 0x6bb and nothing else. The
/// entrants (`wc_turnover_consts::ENTRANT_LIST`, `WC_ASIA_ENTRANT_COUNT`) sorted by reputation (qsort `exe::crt_qsort`
/// with 0x4b07d0); the Asian national teams released from the continent's competitions
/// (`exe::nationalteams_release_competitions_of_continent`(the Asian continent record, 0) on [`G_NATIONAL_TEAMS`]);
/// then each entrant k gets its predicted place (`wc_asia_predicted_place` with the exempt count read again each time;
/// `exe::comp_stats_set_club_final_position`) and this competition as its nation's current one
/// (`exe::nationalteams_set_current_competition`(the entrant's nation, the record)).
/// Original: `WcAsiaLeague::set_predicted_positions` (0x9144d0, wc_asia_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(wc_turnover_consts::WC_ASIA_LEAGUE_CPP, 0x6bb);
    }
    covered!("rx09_18 asian predicted positions");
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    exe::crt_qsort(list, read_u32(this + WC_ASIA_ENTRANT_COUNT), 4, 0x4b07d0);
    let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_ASIA).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), continent, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + WC_ASIA_ENTRANT_COUNT) {
        let team = Club(read_usize(list + 4 * k as usize));
        if let Some(p) = wc_asia_predicted_place(read_u8(this + WC_ASIA_EXEMPT) as i8, k) {
            exe::comp_stats_set_club_final_position(l.comp_stats(), team, p as u32);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr(), l.competition().addr());
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_asia_league_set_predicted_positions`: only the original runs (it sorts the list and moves the
/// national teams); then each entrant's predicted place byte (`comp_stats_club::FINAL_POSITION`) against the plan, over
/// the sorted list.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_ASIA_LEAGUE_SET_PREDICTED_POSITIONS)(this);
    let l = League(this);
    let stats = l.comp_stats();
    if stats == 0 {
        return;
    }
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let exempt = read_u8(this + WC_ASIA_EXEMPT) as i8;
    let mut bad = Vec::new();
    for k in 0..read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as i16 {
        let e = exe::comp_stats_find_club_entry(stats, read_usize(list + 4 * k as usize));
        if let (Some(p), true) = (wc_asia_predicted_place(exempt, k), e != 0) {
            if read_u8(e + comp_stats_club::FINAL_POSITION) != p {
                bad.push((k, p, read_u8(e + comp_stats_club::FINAL_POSITION)));
            }
        }
    }
    SCOREBOARD.record(bad.is_empty(), || format!("wc_asia_league_set_predicted_positions({:#x}) exempt {}: (place, plan, original) {:?}", this, exempt, bad));
}

/// Updates the staff records by a nation's fate in the Asian qualifying (`wc_asia_fate`; the records with this
/// league's competition). A nation through to the World Cup returns −1; any other fate returns the exempt count
/// (`WC_ASIA_EXEMPT`, sign-extended). The last two arguments are not read.
/// Original: `WcAsiaLeague::update_staff_records_for_status` (0x912500, wc_asia_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, _game: usize, _a6: u32) -> u32 {
    match wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT)) {
        AsianFate::Qualified(s, r, w) => european_cup_add_qualified_record(club, League(this).competition().addr(), s, r, w),
        AsianFate::Out(s, w) => wc_asia_eliminate(this, club, s, w),
        AsianFate::WorldCup => {
            covered!("rx09_19 asian nation to the world cup");
            wc_asia_qualify_nation(club, out);
            return u32::MAX;
        }
        AsianFate::Nothing => {}
    }
    read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32
}

/// Comparer for `wc_asia_league_update_staff_records_for_status`: the records and the World Cup place do not run twice,
/// so only the original runs; the plan: the result and, for a nation through, `*out` = 4.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let fate = wc_asia_fate(stage as u8 as i8, outcome as u8, read_u8(this + WC_ASIA_EXEMPT));
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_ASIA_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, out, game, a6);
    let expected = if fate == AsianFate::WorldCup { u32::MAX } else { read_u8(this + WC_ASIA_EXEMPT) as i8 as i32 as u32 };
    let out_ok = fate != AsianFate::WorldCup || read_u8(out) == 4;
    SCOREBOARD.record(r == expected && out_ok, || format!("wc_asia_league_update_staff_records_for_status({:#x}, outcome {:#x}, stage {}) original={:#x} plan {:?}", club, outcome as u8,
        stage as u8 as i8, r, fate));
    r
}

/// The fate news text of an Asian qualifying nation (thiscall, ret 0x28; the arguments of Comp::build_fate_news_text):
/// `wc_asia_fate_text` with the outcome byte, the stage (signed) and the a7 byte. The group news keeps the full outcome
/// argument when it passes it on (the −2 is a constant).
/// Original: `WcAsiaLeague::build_fate_news_text` (0x912780, wc_asia_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let comp = League(this).competition().addr();
    match wc_asia_fate_text(stage as u8 as i8, outcome as u8, a7 as u8, news) {
        AsianFateText::Group(-2, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, -2, 0, text),
        AsianFateText::Group(_, _) => exe::comp_util_write_group_qualify_news(club_id, a4, comp, outcome as i32, news, text),
        AsianFateText::CupTie => exe::comp_util_write_cup_tie_result_news(club_id, a5, a8, outcome, a6, text),
        AsianFateText::Club(format) => {
            covered!("rx09_19 asian round text");
            play_off_club_text(club_id, format, text);
        }
        AsianFateText::WorldCup(format) => {
            covered!("rx09_19 asian world cup text");
            club_and_competition_short_text(club_id, world_cup_record(), format, true, text);
        }
        AsianFateText::Nothing => {}
    }
}

/// Comparer for `wc_asia_league_build_fate_news_text`: the texts the routine writes itself side by side with
/// `compare_text_writer`; the shared group and cup news only the original (it writes the Text once).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_fate_news_text(this: usize, club_id: u32, outcome: u32, stage: u32, a4: u32, a5: u32, a6: u32, a7: u32, a8: u32, news: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_ASIA_LEAGUE_BUILD_FATE_NEWS_TEXT);
    let s = stage as u8 as i8;
    let own = matches!(wc_asia_fate_text(s, outcome as u8, a7 as u8, news), AsianFateText::Club(_) | AsianFateText::WorldCup(_));
    if !own || text == 0 {
        return o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text);
    }
    compare_text_writer(text, || o(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, text),
        |t| wc_asia_league_build_fate_news_text(this, club_id, outcome, stage, a4, a5, a6, a7, a8, news, t),
        || format!("wc_asia_league_build_fate_news_text(club {}, outcome {:#x}, stage {}, a7 {}, news {})", club_id, outcome as u8, s, a7 as u8, news));
}

/// Stores an Asian qualifying nation's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the overall codes by `wc_asia_progress`.
/// Original: `WcAsiaLeague::set_club_progress` (0x914730, wc_asia_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_19 asian progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = wc_asia_progress(stage as u8 as i8, read_u8(this + WC_ASIA_EXEMPT), current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `wc_asia_league_set_club_progress`: the three position bytes of the nation's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_ASIA_LEAGUE_SET_CLUB_PROGRESS);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || wc_asia_league_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("wc_asia_league_set_club_progress({:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", club, stage as u8 as i8, current as u8, best as u8,
        worst as u8, mo, mr));
}

/// The team count of Asian first round group g (−1 to 8), through the hooked routine.
pub(crate) unsafe fn wc_asia_group_size(this: usize, g: i8) -> i16 {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE);
    size(this, g as u8 as u32)
}

/// The national teams of the Asian draw that the plan can check: each of the first entrants (up to the slots of the
/// ten groups) must be in exactly one used slot (group g, slot c below the group's size).
pub(crate) unsafe fn compare_wc_asia_draw(name: &str, this: usize) {
    let draw = read_usize(this + WC_ASIA_DRAW);
    if draw == 0 {
        return;
    }
    let sizes: Vec<usize> = (0..10).map(|g| wc_asia_group_size(this, g as i8 - 1).max(0) as usize).collect();
    let n = (read_i32(this + WC_ASIA_ENTRANT_COUNT).max(0) as usize).min(sizes.iter().sum());
    let list = read_usize(this + wc_turnover_consts::ENTRANT_LIST);
    let slots: Vec<usize> = (0..10).flat_map(|g| (0..sizes[g]).map(move |c| (g, c))).map(|(g, c)| read_usize(draw + 4 * (c + 4 * g))).collect();
    let missing: Vec<usize> = (0..n).map(|k| read_usize(list + 4 * k)).filter(|t| slots.iter().filter(|s| *s == t).count() != 1).collect();
    SCOREBOARD.record(missing.is_empty(), || format!("{}({:#x}): {} entrants, not drawn once {:x?}", name, this, n, missing));
}

/// A League_Stage of the Asian qualifying (`exe::crt_operator_new` of 0xee; League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58; 0 when out of memory): this league, `n` teams from `list`, two
/// legs, the competition, the round table with its rounds, the win, draw and category bytes and the tie-breaks of this
/// league, the place counts at `places`, the year, the stage, the extra as its label, `importance`, 4, 0, the rule bits
/// `flags`, −1, 0, 2 (as `category_b_league_stage`, with 4 where the leagues pass 1).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn qualifying_league_stage(l: League, n: u32, list: usize, table: usize, rounds: i16, extra: i16, places: usize, stage: u32, importance: u32, flags: u32) -> usize {
    let p = exe::crt_operator_new(0xee);
    if p == 0 {
        return 0;
    }
    let ctor: LeagueStageCtorFn = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    ctor(p, l.addr(), n, list, 2, l.competition().addr(), table, rounds as u16 as u32, l.win_points() as u32, l.draw_points() as u32, l.match_category() as u32,
        l.addr() + league::TIEBREAKS, places, l.year() as u16 as u32, stage, extra as u16 as u32, importance, 4, 0, flags, -1, 0, 2)
}

/// The winner (table row 0, League::get_table_row) of Asian first round group k (−1 the league's own table, 0 to 8 the
/// stages).
pub(crate) unsafe fn wc_asia_first_round_winner(l: League, k: i32) -> usize {
    let group = if k > -1 { read_usize(l.stages() + 4 * k as usize) } else { l.addr() };
    read_usize(exe::league_get_table_row(group, 0))
}

/// The places of the Asian second round groups (a local array of the original): with no exempt nation, 2 qualify and 1
/// goes to the third round; otherwise 1 and 1.
pub(crate) unsafe fn wc_asia_second_round_places(this: usize) -> [u8; 4] {
    let first = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_17: two qualify with no exempt nation
    [first, 1, 0, 0]
}

/// The two sides of the Asian third round tie: from each second round group (stages 9 and 10), the third-placed team
/// (table row 2) with no exempt nation, else the runner-up (row 1).
pub(crate) unsafe fn wc_asia_third_round_sides(this: usize) -> [usize; 2] {
    let l = League(this);
    let row = if read_u8(this + WC_ASIA_EXEMPT) == 0 { 2 } else { 1 }; // rx09_18: the third-placed sides with no exempt nation
    [9usize, 10].map(|s| read_usize(exe::league_get_table_row(read_usize(l.stages() + 4 * s), row)))
}

/// The Asian side of the play-off: the first entry of the third round tie's team table (`cup::TEAM_TABLE`, the count
/// re-read) whose status is 2 (lost) with one exempt nation, else 1 (won); none: None (the original leaves the slot as
/// malloc returned it).
pub(crate) unsafe fn wc_asia_play_off_side(this: usize) -> Option<usize> {
    let tie = Cup(read_usize(League(this).stages() + 4 * 0xb));
    let mut i = 0i16;
    while i < tie.team_count() {
        let want = if read_u8(this + WC_ASIA_EXEMPT) == 1 { 2 } else { 1 }; // rx09_18: the third round winner goes on
        let e = tie.team_table() + 6 * i as usize;
        if read_u8(e + cup_team_entry::STATUS) == want {
            return Some(read_usize(e + cup_team_entry::CLUB));
        }
        i += 1;
    }
    None
}

/// The European nation of the play-off (WcEuropeLeague 0x91def0 on the European section's object:
/// `KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION` in [`G_COMP_OBJECT_TABLE`]).
pub(crate) unsafe fn wc_asia_play_off_european_side() -> usize {
    let europe = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION).wrapping_mul(4)));
    let pick: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION);
    pick(europe)
}

/// The predicted place of the Asian entrant in place k (by reputation), by the exempt count: none exempt, 1 for the top
/// four, then 5, 6, 7 (to the eighth), 9 (to the tenth), 11 (to the twentieth), 21 (to the thirtieth), 31; one or two,
/// 1 for the top two, then 3, 4, 5 (to the sixth), 7 (to the eighth), 9, 11, 21, 31; three, 1, 2, 3 (to the fourth), 5,
/// 7, 9, 11, 21, 31; any other count, no place.
pub(crate) fn wc_asia_predicted_place(exempt: i8, k: i16) -> Option<u8> {
    let bands: &[(i16, u8)] = match exempt {
        0 => &[(4, 1), (5, 5), (6, 6), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        1 | 2 => &[(2, 1), (3, 3), (4, 4), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        3 => &[(1, 1), (2, 2), (4, 3), (6, 5), (8, 7), (10, 9), (20, 0xb), (30, 0x15)],
        _ => return None,
    };
    Some(bands.iter().find(|&&(limit, _)| k < limit).map_or(0x1f, |&(_, p)| p)) // rx09_18: the reputation bands
}

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each).
pub(crate) unsafe fn world_cup_record() -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b))
}

/// A nation through to the World Cup from the Asian qualifying: the World Cup qualification record (weight 100), its
/// place in the World Cup (`exe::worldcup_add_qualified_team` on the World Cup object, [`G_COMP_OBJECT_TABLE`]), `*out`
/// = 4 (not checked for null) and NationalTeams::add_qualified_nation(club, record) on [`G_NATIONAL_TEAMS`].
pub(crate) unsafe fn wc_asia_qualify_nation(club: usize, out: usize) {
    european_cup_add_qualified_record(club, world_cup_record(), 0, 0, 0x64);
    let world_cup = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(4)));
    exe::worldcup_add_qualified_team(world_cup, club); // rx09_19: the Asian nation enters the World Cup
    write_u8(out, 4);
    exe::nationalteams_add_qualified_nation(read_usize(G_NATIONAL_TEAMS), club, world_cup_record());
}

/// A nation out of the Asian qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record` with
/// `stage` and `weight`) and the national team's campaign ended (`clear_national_team_competition`).
pub(crate) unsafe fn wc_asia_eliminate(this: usize, club: usize, stage: u32, weight: u32) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), stage, weight);
    clear_national_team_competition(Club(club));
}

/// The fate of a nation with `outcome` (its byte) at `stage` (signed) of the Asian qualifying (`exempt`, the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (below 9): 6 through (0x14, 0, 0x1e), 0xfe out (0xa, 0x1e);
/// - the second round (9, 10): 7 to the World Cup, 1 to the third round (0x1e, 0x96, 0x3c), 0xfe out (0x14, 0x3c);
/// - the third round (11): with 1 or 3 exempt nations, 2 to the play-off (0xa0, 0, 0x46) and any other to the World
///   Cup; with none or 2, 2 out (0x1e, 0x3c) and any other to the play-off;
/// - the play-off (12): 1 to the World Cup, any other out (0xa0, 0x3c);
/// - later: nothing.
pub(crate) fn wc_asia_fate(stage: i8, outcome: u8, exempt: u8) -> AsianFate {
    match stage {
        i8::MIN..=8 => match outcome {
            6 => AsianFate::Qualified(0x14, 0, 0x1e),
            0xfe => AsianFate::Out(0xa, 0x1e),
            _ => AsianFate::Nothing,
        },
        9 | 10 => match outcome {
            7 => AsianFate::WorldCup,
            1 => AsianFate::Qualified(0x1e, 0x96, 0x3c),
            0xfe => AsianFate::Out(0x14, 0x3c),
            _ => AsianFate::Nothing,
        },
        11 if exempt != 0 && exempt != 2 => if outcome == 2 { AsianFate::Qualified(0xa0, 0, 0x46) } else { AsianFate::WorldCup },
        11 => if outcome == 2 { AsianFate::Out(0x1e, 0x3c) } else { AsianFate::Qualified(0xa0, 0, 0x46) },
        12 => if outcome == 1 { AsianFate::WorldCup } else { AsianFate::Out(0xa0, 0x3c) }, // rx09_19: the play-off winner goes to the World Cup
        _ => AsianFate::Nothing,
    }
}

/// The text of a nation's fate in the Asian qualifying:
/// - the first round (below 9): with a news item the group news (outcome, news); without, 6 gives 0xad47bc "{nation}
///   qualify for second round", 0xfe the group news with −2 and 0;
/// - the second round (9, 10): with a news item the group news; without, 7 gives 0xad4738 "{nation} qualify for {World
///   Cup}", 1 0xad4780 "{nation} qualify for third round", 0xfe the group news with −2 and 0;
/// - the third round (11): with a news item the cup tie news; without, when `a7` is 1, outcome 1 "qualify for {World
///   Cup}" and any other 0xad46c0 "{nation} out of {World Cup}"; otherwise outcome 1 0xad4700 "{nation} qualify for
///   playoff" and any other "out of {World Cup}";
/// - the play-off (12): with a news item the cup tie news; without, 1 "qualify for {World Cup}", any other "out of
///   {World Cup}";
/// - later: nothing.
pub(crate) fn wc_asia_fate_text(stage: i8, outcome: u8, a7: u8, news: u32) -> AsianFateText {
    let group_out = |o: u8| if o == 0xfe { AsianFateText::Group(-2, 0) } else { AsianFateText::Nothing };
    match stage {
        i8::MIN..=8 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        i8::MIN..=8 => if outcome == 6 { AsianFateText::Club(0xad47bc) } else { group_out(outcome) },
        9 | 10 if news != 0 => AsianFateText::Group(outcome as i8 as i32, news),
        9 | 10 => match outcome {
            7 => AsianFateText::WorldCup(0xad4738),
            1 => AsianFateText::Club(0xad4780),
            o => group_out(o),
        },
        11 | 12 if news != 0 => AsianFateText::CupTie,
        11 if a7 != 1 => if outcome == 1 { AsianFateText::Club(0xad4700) } else { AsianFateText::WorldCup(0xad46c0) },
        11 | 12 => if outcome == 1 { AsianFateText::WorldCup(0xad4738) } else { AsianFateText::WorldCup(0xad46c0) }, // rx09_19: qualify for / out of the World Cup
        _ => AsianFateText::Nothing,
    }
}

/// The three progress codes (current, best, worst) of an Asian qualifying nation at a stage (bytes; `exempt` the byte at
/// `WC_ASIA_EXEMPT`):
/// - the first round (−1 to 8): best 1, 2, 3, 4 become 1, 11, 21, 31; worst and current 1, 2, 3, 4 become 9, 11, 21, 31
///   (the group winner's current code is 9, as its worst);
/// - the second round (9, 10): best 1 and 2 become 1, then 3, 4, 5 become 5, 7, 9; worst and current 1 and 2 become 4,
///   then 5, 7, 9;
/// - the third round (11): with no exempt nation best 1, worst and current below 4 become 1; with one, best below 4 and
///   worst and current below 3 become 1; with two, best and worst and current below 3, 2 and 2 become 1; otherwise best
///   below 2 becomes 1;
/// - any other code, and the other stages (below −1, or 12 on): as it came.
pub(crate) fn wc_asia_progress(stage: i8, exempt: u8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let map = |v: u8, table: &[u8]| if (1..=table.len()).contains(&(v as usize)) { table[v as usize - 1] } else { v };
    let below = |v: u8, n: u8| if v < n { 1 } else { v };
    match stage {
        -1..=8 => (map(current, &[9, 0xb, 0x15, 0x1f]), map(best, &[1, 0xb, 0x15, 0x1f]), map(worst, &[9, 0xb, 0x15, 0x1f])), // rx09_19: the first round codes
        9 | 10 => (map(current, &[4, 4, 5, 7, 9]), map(best, &[1, 1, 5, 7, 9]), map(worst, &[4, 4, 5, 7, 9])),
        11 => match exempt {
            0 => (below(current, 4), 1, below(worst, 4)),
            1 => (below(current, 3), below(best, 4), below(worst, 3)),
            2 => (below(current, 2), below(best, 3), below(worst, 2)),
            _ => (current, below(best, 2), worst),
        },
        _ => (current, best, worst),
    }
}

/// The Asian qualifying's exempt nation count (+0xee, signed byte; the hosts and the last winner when Asian).
pub(crate) const WC_ASIA_EXEMPT: usize = 0xee;

/// The Asian qualifying's entrant count (+0xf3, dword) of the list at `wc_turnover_consts::ENTRANT_LIST`.
pub(crate) const WC_ASIA_ENTRANT_COUNT: usize = 0xf3;

/// The Asian qualifying's first round draw (+0xf7, a pointer: ten groups of four national team slots).
pub(crate) const WC_ASIA_DRAW: usize = 0xf7;

/// `WcAsiaLeague::get_first_round_group_size` (0x912db0, thiscall, ret 4), through its hook.
pub(crate) const FN_WCASIALEAGUE_GET_FIRST_ROUND_GROUP_SIZE: usize = 0x912db0;

/// `WcEuropeLeague` 0x91def0 (thiscall, ret 0): the European nation drawn for the Asian play-off [inference: the first
/// team of the European runners-up stage, +0x100, that is not in the play-off stage, +0x101].
pub(crate) const FN_WC_EUROPE_LEAGUE_ASIAN_PLAY_OFF_NATION: usize = 0x91def0;

/// What happens to a nation by its Asian qualifying fate.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFate {
    /// The qualified-for-round record with (stage, round, weight).
    Qualified(u32, u32, u32),
    /// The failed-to-qualify record with (stage, weight) and the end of the campaign.
    Out(u32, u32),
    /// Through to the World Cup (`wc_asia_qualify_nation`); the result −1.
    WorldCup,
    Nothing,
}

/// What `wc_asia_league_build_fate_news_text` writes for a nation (the cup and group news are the shared routines').
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum AsianFateText {
    /// `exe::comp_util_write_group_qualify_news`(club, a4, the competition, the outcome or −2, the news item or 0, text).
    Group(i32, u32),
    /// `exe::comp_util_write_cup_tie_result_news`(club, a5, a8, outcome, a6, text).
    CupTie,
    /// A club text (`play_off_club_text`) with this format.
    Club(usize),
    /// A club and World Cup text (`club_and_competition_short_text` with the World Cup's record) with this format.
    WorldCup(usize),
    Nothing,
}
