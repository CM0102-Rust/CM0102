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
