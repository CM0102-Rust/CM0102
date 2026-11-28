use super::*;

/// Writes byte/word rule fields (offset, size, value) of a qualifier league.
pub(crate) unsafe fn wc_europe_league_write_rule_fields(this: League, fields: &[(usize, u8, u32)]) {
    for &(o, n, v) in fields {
        if n == 1 {
            write_u8(this.addr() + o, v as u8);
        } else {
            write_u16(this.addr() + o, v as u16);
        }
    }
}

/// First word of Date(day, month, competition year +0x40, weekday).
pub(crate) unsafe fn comp_date_word(this: League, day: u32, month: u32, week: u32) -> u16 {
    let mut d = [0u8; 8];
    // simplification: the year goes in a register with garbage in the high 16 bits; 0x549f70 reads it as a word
    exe::date_date_549f70((d.as_mut_ptr()) as *mut [u16; 4], day, month, (this.year() as u16) as u32, week);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}

/// Shared end of rule setup: +0xb6 = date word, +0xb8 = b8, round table (virtual +0x3c) at +0xba; returns 1.
pub(crate) unsafe fn finish_rules_setup(this: League, word: u16, b8: u16) -> u32 {
    this.set_next_draw_day((word) as i16);
    this.set_next_draw_year_offset((b8) as i16);
    let vt = read_usize(this.addr());
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(vt + 0x3c));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    this.set_round_dates((t as u32) as usize);
    1
}

/// Comparer: runs the original and checks fields, +0xb6 date, +0xb8 and return 1.
pub(crate) unsafe fn check_qualifier_rules(name: &str, this: usize, fields: &[(usize, u8, u32)], data: (u32, u32, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let this = League(this);
    let word = comp_date_word(this, data.0, data.1, data.2);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in fields {
        let (lido, v) = if n == 1 { (read_u8(this.addr() + off) as u32, v & 0xff) } else { (read_u16(this.addr() + off) as u32, v & 0xffff) };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = this.next_draw_day() as u16;
    let lido8 = this.next_draw_year_offset() as u16;
    SCOREBOARD.record(r == 1 && k == 0 && b6 == word && lido8 == b8, || format!("{}({:#x}) returned {} fields errados {:x?} date {:#x} prevista {:#x} +0xb8 {}", name, this.addr(), r, &errados[..k], b6, word, lido8));
    r
}

pub(crate) const EUROPE_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
    (0xc6, 1, 2), (0xc7, 1, 4), (0xc8, 1, 5), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// rule: the European groups are home and away, 3 points a win and 1 a draw, tiebreak by codes 1, 2, 4 and 5
/// [inference: goal difference, goals for, head-to-head, away goals]; each group winner goes straight through; national
/// teams (+0x43 = 2); category 10 (national teams) on each match; 7 substitutes and 3 changes; starts on 25 of month 2
/// [inference: 25 March] moved to weekday 5
/// Original: `WcEuropeLeague::set_up_rules` (0x9191e0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    covered!("cqj_12 europe rules");
    wc_europe_league_write_rule_fields(this, &EUROPE_RULE_FIELDS);
    this.set_win_points(3); // cqj12: the win in the European qualifiers
    this.set_match_category(match_category::NATIONAL_TEAMS); // cqj12: the match category of the European qualifiers
    let day = 0x19; // cqj12: the start day of the European qualifiers
    let word = comp_date_word(this, day, 2, 5);
    finish_rules_setup(this, word, 0)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

pub(crate) const SOUTH_AMERICA_QUALIFIER_RULE_FIELDS: [(usize, u8, u32); 16] = [(0x3c, 2, 2), (0x43, 1, 2), (0xc6, 1, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 0xa), (0xab, 2, 0),
    (0xdb, 2, 0), (0xc5, 1, 1), (0xc7, 1, 3), (0xbf, 1, 1), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0x4a, 1, 3), (0xa7, 2, 0xffff)];

// simplification: virtual +0x3c allocates the round table: does not run twice. Original only; the plan checks the rules and
// the direct places (4 − +0xef − +0xee, with the +0xef the routine itself copies, computed before).

pub(crate) const ASIA_QUALIFIER_RULE_FIELDS: [(usize, u8, u32); 12] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x3c, 2, 2), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2), (0xc5, 1, 1),
    (0xc6, 1, 2), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0)];

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

/// The season turnover of the European World Cup qualifying: last campaign's table, round dates, list and stages freed,
/// the season archived, the next campaign's year, the entry pool again, as many stages as its format needs, the rules
/// and a new list.
///
/// The season reopened (`.set_season_closed(0)`), the table freed (`exe::league_free_table`(this, 0), thiscall, ret 4),
/// the round dates (`.round_dates()`) freed and zeroed; the list (`EUROPE_LIST`), when there, destroyed (LLIST::~LLIST
/// `exe::drop_llist`, thiscall) and deleted (`exe::crt_operator_delete`, cdecl); the season archived
/// (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it reaches the game's
/// (`advance_qualifying_campaign`); `.set_active(0)` and `EUROPE_FB` = 0; the entry pool
/// (WcEuropeLeague::build_entry_pool, thiscall, ret 0; its return not read); the stages destroyed
/// (`destroy_comp_stages`) and their vector freed (not zeroed); the stage count by the exempt count
/// (`WC_EUROPE_EXEMPT`): 0 → 11, 1 or 2 → 10, 3 → 9, any other kept; a new vector (`exe::crt_malloc`(4 × count),
/// cdecl), the current stage −1; the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x36a and the copy
/// returns 0, the original the box's eax); then a new list (`exe::crt_operator_new(0xe)` and LLIST::LLIST
/// `exe::new_llist`, or 0 when out of memory) in `EUROPE_LIST`, which it returns (the original keeps the list's
/// construction under an SEH frame; nothing here throws).
/// Original: `WcEuropeLeague::start_new_season` (0x919430, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let list = read_usize(this.addr() + wt::EUROPE_LIST);
    if list != 0 {
        exe::drop_llist(list);
        exe::crt_operator_delete(list);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_13 european qualifying rollover");
    this.set_active(0);
    write_u32(this.addr() + wt::EUROPE_FB, 0);
    call_section_routine(wt::FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL, this.addr());
    destroy_comp_stages(this);
    exe::crt_free(this.stages());
    match read_u8(this.addr() + WC_EUROPE_EXEMPT) {
        0 => this.set_stage_count(0xb),
        1 | 2 => this.set_stage_count(0xa),
        3 => this.set_stage_count(9),
        _ => {}
    }
    this.set_stages(exe::crt_malloc((this.stage_count() as u32) << 2));
    this.set_current_stage(usize::MAX);
    if call_section_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x36a);
        return 0;
    }
    let p = exe::crt_operator_new(0xe);
    let list = if p != 0 { exe::new_llist(p) } else { 0 };
    write_usize(this.addr() + wt::EUROPE_LIST, list);
    list as u32
}

/// Comparer for `wc_europe_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_START_NEW_SEASON);
    compare_qualifying_turnover("wc_europe_league_start_new_season", League(this), || o(this))
}

/// rule: the European World Cup qualifying entry pool: fifty places minus the European nations exempt (the hosts of the
/// next World Cup and the last winner when European), filled with the other European national teams in database order
/// and sorted by reputation; the exempt count sets the format: none, nine groups (stages up to 8) then the runners-up
/// (9) and the play-offs (10); one or two, eight groups (7, 8, 9); more, seven groups (6, 7, 8)
///
/// +0xf2 (the pool count, dword) = 0x32, `WC_EUROPE_EXEMPT` = 0, `league::GAMES_PER_CLUB` = −1. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x4c1 and no hosts; otherwise the hosts of the World Cup two years later
/// (HostCountry::find_hosts: 5 pushes; mov ecx -> thiscall, ret 0x14; the year with garbage in the high 16 bits, read as
/// a word); for each (k signed byte below the count) up to the first −1, a European one (`KEY_CONTINENT_EUROPE`) adds
/// one. The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup record, 0)): its nation, with a
/// continent, European and neither host, adds one. +0xf2 = 0x32 − exempt (signed byte); the old list +0xee freed
/// (`exe::crt_free`) when there is one, and a new one of fifty slots (`exe::crt_malloc(0xc8)`; out of memory, the box of
/// line 0x4f4 and 0). Over the nations (i word signed), each European one while the list is below fifty (not below the
/// count): its national team, when the nation is neither host and the team is not the winner, goes into the list. Below
/// the count (signed) it returns 0; otherwise the list sorted by reputation (qsort `exe::crt_qsort`(list, n, 4,
/// 0x4b07d0)), the format set and 1.
/// Original: `WcEuropeLeague::build_entry_pool` (0x919e30, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_build_entry_pool(this: usize) -> u32 {
    let league = League(this);
    let exempt = this + WC_EUROPE_EXEMPT;
    write_u32(this + 0xf2, 0x32);
    write_u8(exempt, 0);
    league.set_games_per_club(-1);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4a6c, 0x4c1);
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
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
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
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && id != hosts[0] && id != hosts[1] {
                write_u8(exempt, read_u8(exempt).wrapping_add(1));
            }
        }
    }
    write_u32(this + 0xf2, (0x32 - read_u8(exempt) as i8 as i32) as u32);
    if read_usize(this + 0xee) != 0 {
        exe::crt_free(read_usize(this + 0xee));
    }
    let list = exe::crt_malloc(0xc8);
    write_u32(this + 0xee, list as u32);
    if list == 0 {
        show_error_at(0xad4a6c, 0x4f4);
        return 0;
    }
    covered!("cqz_13 european pool");
    let mut count = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE) && count < 0x32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                count += 1;
                write_usize(read_usize(this + 0xee) + (count as usize - 1) * 4, team);
            }
        }
        i = i.wrapping_add(1);
    }
    if count < read_u32(this + 0xf2) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this + 0xee), count as u32, 4, 0x4b07d0);
    let format: [u8; 3] = match read_u8(exempt) {
        0 => [8, 9, 0xa], // cqz13: nine groups without exempt nations
        1 | 2 => [7, 8, 9],
        _ => [6, 7, 8],
    };
    for (k, v) in format.into_iter().enumerate() {
        write_u8(this + WC_EUROPE_LAST_GROUP + k, v);
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_entry_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL);
    let r = o(this);
    let n = read_u32(this + 0xf2) as i32;
    let list = read_usize(this + 0xee);
    let format = match read_u8(this + WC_EUROPE_EXEMPT) {
        0 => [8u8, 9, 0xa],
        1 | 2 => [7, 8, 9],
        _ => [6, 7, 8],
    };
    let ok = r == 0 || (list != 0 && (0..3).all(|k| read_u8(this + WC_EUROPE_LAST_GROUP + k) == format[k]) && (0..n.max(0) as usize).all(|k| {
        let nation = Club(read_usize(list + k * 4)).nation();
        let c = if nation.is_null() { 0 } else { nation.continent() };
        c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_EUROPE)
    }));
    SCOREBOARD.record(ok, || format!("wc_europe_league_build_entry_pool({:#x}) returned {} with {} places", this, r, n));
    r
}

/// rule: on the draw date of the European qualifying, with the full day update and the league not yet started, the
/// groups are drawn, this group gets its table and fixtures, the other groups and the runners-up stage are built; on a
/// day with results the runners-up table is refreshed; before the draw nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the draw (WcEuropeLeague::draw_groups 0x918ec0, thiscall, ret 0), this group's table
/// (WcEuropeLeague::init_own_group_table 0x9190d0; 0: the error box of line 0x5c7 and 0), the fixtures
/// (`exe::league_create_fixtures`; 0: the box of line 0x5cd and 0), the sorted table (`exe::league_sort_table(this,
/// 0)`; 0: the box of line 0x5d4 and 0); the groups 0 to n − 2 (WcEuropeLeague::create_group_stage 0x9196d0: push 6;
/// push 0; push g; mov ecx -> thiscall, ret 0xc), n by the exempt count (0: 10; 1 or 2: 9; more: 8); the drawn list
/// +0xf7 freed (`exe::crt_free`) and zeroed; the runners-up stage (WcEuropeLeague::create_runners_up_stage 0x91a140);
/// the predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the pool +0xee freed and zeroed, its
/// count +0xf2 (dword) zeroed and `league::ACTIVE` = 1. On any other day (the draw day too when active or without
/// `full`): when the game's date ([`G_CURRENT_DATE`], the two words) is `league::LAST_RESULT_DATE`, the runners-up table
/// (WcEuropeLeague::fill_runners_up_table 0x91a240). Then the league day with `full` (`League::update_for_date`), whose
/// al it passes on.
/// Original: `WcEuropeLeague::update_for_date` (0x91a500, wc_europe_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let league = this;
    covered!("cqz_13 european day");
    let (day, year) = oceania_nations_draw_date(league);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 && full != 0 {
        covered!("cqz_13 european draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WCEUROPELEAGUE_DRAW_GROUPS);
        draw(this.addr());
        if exe::wceuropeleague_init_own_group_table(this) == 0 {
            show_error_at(0xad4a6c, 0x5c7);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4a6c, 0x5cd);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4a6c, 0x5d4);
            return 0;
        }
        let groups: i32 = match read_u8(this.addr() + WC_EUROPE_EXEMPT) {
            0 => 0xa,
            1 | 2 => 9,
            _ => 8,
        };
        for g in 0..groups - 1 {
            exe::wceuropeleague_create_group_stage(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this.addr() + 0xf7));
        write_u32(this.addr() + 0xf7, 0);
        exe::wceuropeleague_create_runners_up_stage(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        exe::crt_free(read_usize(this.addr() + 0xee));
        write_u32(this.addr() + 0xee, 0);
        write_u32(this.addr() + 0xf2, 0);
        league.set_active(1);
    } else if read_u16(G_CURRENT_DATE + 2) == read_u16(this.addr() + league::LAST_RESULT_DATE + 2) && read_u16(G_CURRENT_DATE) == (this.last_result_date() as u16) {
        exe::wceuropeleague_fill_runners_up_table(this);
    }
    exe::league_update_for_date(this, today, full) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_u32(this + 0xf2) == 0 && read_usize(this + 0xf7) == 0;
        SCOREBOARD.record(ok, || format!("wc_europe_league_update_for_date({:#x}): the draw did not build the groups", this));
    }
    r
}

/// The European qualifying's exempt count (+0xf6, byte: the European hosts and holder) and its group format: the last
/// group-stage index, the runners-up stage and the play-off stage (+0xff, +0x100, +0x101), read by the round table.
pub(crate) const WC_EUROPE_EXEMPT: usize = 0xf6;

pub(crate) const WC_EUROPE_LAST_GROUP: usize = 0xff;

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
    let this = League(this);
    use wc_cup_turnover_consts as wt;
    let league = this;
    exe::new_league(league);
    write_usize(this.addr(), WC_EUROPE_LEAGUE_VTABLE);
    league.set_year(year as i16);
    write_usize(this.addr() + league::COMPETITION, record);
    league.set_club_sides(4);
    league.set_upper_division_id(read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) as i32);
    league.set_lower_division_id(usize::MAX);
    write_u32(this.addr() + 0xf7, 0); // the drawn groups [inference]
    write_u32(this.addr() + 0xee, 0); // the entry pool list
    write_u32(this.addr() + 0xf2, 0); // its count
    write_u32(this.addr() + wt::EUROPE_FB, 0);
    write_u8(this.addr() + WC_EUROPE_EXEMPT, 1);
    league.set_discipline_rule(0);
    league.set_year(wc_europe_campaign_year(year as i16));
    let p = exe::crt_operator_new(0xe);
    write_usize(this.addr() + wt::EUROPE_LIST, if p != 0 { exe::new_llist(p) } else { 0 });
    league.set_importance(0xf);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cx16_4 european qualifying");
    league.set_league_flags(0x24);
    call_section_routine(wt::FN_WC_EUROPE_LEAGUE_BUILD_ENTRY_POOL, this.addr());
    match read_u8(this.addr() + WC_EUROPE_EXEMPT) {
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
    if call_section_routine(FN_WC_EUROPE_LEAGUE_SET_UP_RULES, this.addr()) == 0 {
        show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x7f);
        return this.addr();
    }
    if !qualifying_section_stats(league, wt::WC_EUROPE_LEAGUE_CPP, 0x87) {
        return this.addr();
    }
    if year as i16 >= league.year() {
        call_section_routine(FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN, this.addr());
    }
    this.addr()
}

/// Comparer for `construct_wc_europe_league`: it allocates, reads the save and builds the pool, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year (`wc_europe_campaign_year`), the
/// African section above and none below.
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_europe_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_EUROPE_LEAGUE);
    let r = o(this, year, record);
    let league = League(this);
    let expected = wc_europe_campaign_year(year as i16);
    let ok = r == this && read_usize(this) == WC_EUROPE_LEAGUE_VTABLE && league.competition().addr() == record && built_year_ok(league.year() as u16, expected as u16)
        && league.upper_division_id() as u32 == read_u32(KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION) && league.lower_division_id() as u32 == u32::MAX;
    SCOREBOARD.record(ok, || format!("construct_wc_europe_league({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}", this, year as i16, record, r, league.year(), expected, read_usize(this)));
    r
}

/// The European campaign year of a game year: the four-year cycle (`qualifying_campaign_year`), never before 2004.
pub(crate) fn wc_europe_campaign_year(year: i16) -> i16 {
    qualifying_campaign_year(year).max(0x7d4) // cx16_4: the European campaign, from 2004
}

/// The European section's vtable (WcEuropeLeague).
pub(crate) const WC_EUROPE_LEAGUE_VTABLE: usize = 0x970cc4;

/// WcEuropeLeague routines the constructor calls directly (thiscall, ret 0): the rules and the 2002 campaign.
pub(crate) const FN_WC_EUROPE_LEAGUE_SET_UP_RULES: usize = 0x9191e0;

pub(crate) const FN_WC_EUROPE_LEAGUE_LOAD_2002_CAMPAIGN: usize = 0x91b760;

/// rule: the European nation that plays the Asian section is the best-ranked group runner-up who is not in the
/// European play-off
///
/// The runners-up stage (`cup::STAGES`[+0x100], signed byte; its table `league::TABLE`, `league_table_row::SIZE`-byte
/// rows, `league::CLUB_COUNT`) in order; the first whose club is not in the play-off stage's team table
/// (`cup::STAGES`[+0x101]: `cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT`). None: 0.
/// Original: `WcEuropeLeague::get_asia_play_off_team` (0x91def0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_asia_play_off_team(this: usize) -> usize {
    let stages = League(this).stages();
    let runners = League(read_usize(stages + (read_u8(this + 0x100) as i8 as isize as usize).wrapping_mul(4)));
    let play_off = Cup(read_usize(stages + (read_u8(this + 0x101) as i8 as isize as usize).wrapping_mul(4)));
    covered!("cx16_10 european asian play-off team");
    for i in 0..runners.club_count().max(0) as usize {
        let club = read_usize(runners.table() + i * league_table_row::SIZE);
        let n = play_off.team_count();
        let mut k = 0i16;
        while k < n && read_usize(play_off.team_table() + k as usize * 6) != club {
            k += 1;
        }
        if k == n {
            return club; // cx16_10: the first runner-up not in the play-off
        }
    }
    0
}

/// Comparer for `wc_europe_league_get_asia_play_off_team`: it only reads; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_asia_play_off_team(this: usize) -> usize {
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WC_EUROPE_LEAGUE_GET_ASIA_PLAY_OFF_TEAM)(this);
    let b = wc_europe_league_get_asia_play_off_team(this);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_asia_play_off_team({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}

/// rule: the European qualifying moves on one stage at a time; reaching the runners-up stage or the play-off stage
/// builds the play-off; past the last stage it is an error
///
/// At or past the last stage (`league::STAGE_COUNT` − 1, signed), the error box of line 0x37f. Otherwise
/// `league::CURRENT_STAGE` + 1 and, when the new stage is the runners-up stage (+0x100) or the play-off stage (+0x101),
/// signed bytes, WcEuropeLeague::create_play_off_stage (0x919910, thiscall, ret 0, called directly).
/// Original: `WcEuropeLeague::advance_stage` (0x919620, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_advance_stage(this: usize) {
    let league = League(this);
    let current = league.current_stage() as i32;
    if current >= (league.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x37f);
    }
    let next = current + 1;
    league.set_current_stage(next as usize);
    covered!("cx16_11 european next stage");
    if next == read_u8(this + 0x100) as i8 as i32 || next == read_u8(this + 0x101) as i8 as i32 { // cx16_11: the play-off when the runners-up stage comes
        call_section_routine(FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE, this);
    }
}

/// Comparer for `wc_europe_league_advance_stage`: it builds stages, so only the original runs; the stage stays or moves
/// on by one.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_advance_stage(this: usize) {
    let before = League(this).current_stage() as i32;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_ADVANCE_STAGE)(this);
    let after = League(this).current_stage() as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_europe_league_advance_stage({:#x}): stage {} -> {}", this, before, after));
}

/// rule: the European runners-up are ranked in a league stage of their own, one runner-up per group (the last group
/// index + 2), with the section's points, category and tie-breaks 1, 2, 5, its best placed going through
///
/// `league::PROMOTED_COUNT` 0; the stage (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, the last group (+0xff, signed) + 2, no teams, 0, the
/// record, no round table, 0 rounds, `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the
/// tie-breaks {1, 2, 5, 0}, the places at `league::PROMOTED_COUNT` (the section's own four bytes), the year, the
/// runners-up stage index (+0x100), the label 0x3fc, importance 0, 4, 0, flags 2, −1, 0, 2; the original pushes the count,
/// the bytes and the year with garbage in the high bytes, which the constructor does not read: the copy passes clean
/// values) into `league::STAGES`[+0x100] (null not checked); `league::PROMOTED_COUNT` 1; `league::CURRENT_STAGE` the
/// runners-up stage index; then the table filled (WcEuropeLeague::fill_runners_up_table
/// `exe::wceuropeleague_fill_runners_up_table`, thiscall, ret 0). The SEH frame covers only the stage's new.
/// Original: `WcEuropeLeague::create_runners_up_stage` (0x91a140, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_runners_up_stage(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = this;
    let tiebreaks: [u8; 4] = [1, 2, 5, 0];
    let n = (read_u8(this.addr() + 0xff) as i8 as i16).wrapping_add(2); // cx16_11: one runner-up per group
    let index = read_u8(this.addr() + 0x100) as i8;
    league.set_promoted_count(0);
    covered!("cx16_11 european runners-up stage");
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(p, this.addr(), n as u16 as u32, 0, 0, league.competition().addr(), 0, 0, league.win_points() as u32, league.draw_points() as u32, league.match_category() as u32,
            tiebreaks.as_ptr() as usize, this.addr() + league::PROMOTED_COUNT, league.year() as u16 as u32, index as u8 as u32, 0x3fc, 0, 4, 0, 2, -1, 0, 2)
    } else {
        0
    };
    write_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4)), stage);
    league.set_promoted_count(1);
    league.set_current_stage(index as isize as usize);
    exe::wceuropeleague_fill_runners_up_table(this);
}

/// Comparer for `wc_europe_league_create_runners_up_stage`: it builds a stage, so only the original runs; then the
/// plan: the current stage is the runners-up stage, which exists with one row per group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_runners_up_stage(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_CREATE_RUNNERS_UP_STAGE)(this);
    let league = League(this);
    let index = read_u8(this + 0x100) as i8;
    let stage = League(read_usize(league.stages().wrapping_add((index as isize as usize).wrapping_mul(4))));
    let n = (read_u8(this + 0xff) as i8 as i16).wrapping_add(2);
    let ok = league.current_stage() as i32 == index as i32 && !stage.is_null() && stage.club_count() == n;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_runners_up_stage({:#x}): stage {} index {}, {} rows of {}", this, league.current_stage() as i32, index, if stage.is_null() { -1 } else { stage.club_count() as i32 }, n));
}

/// rule: the first European qualifying group is the section itself: its size by the exempt count (group −1), its
/// nations from the first drawn slots, and the group draw news
///
/// `league::CLUB_COUNT` = WcEuropeLeague::get_group_size(−1) (0x918dd0, thiscall, ret 4); `league::TABLE` = a table of
/// that many `league_table_row::SIZE`-byte rows (`exe::crt_malloc`; out of memory, the error box of line 0x2b2 and 0);
/// `league::GAMES_PER_CLUB` = (count − 1) × `league::LEGS` (words); row i with the drawn slot i (+0xf7, a pointer to the
/// slot table) (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10; the original pushes i with
/// garbage in the high bytes: the routine reads the byte); the group draw news (`exe::league_send_group_draw_news`); 1.
/// Original: `WcEuropeLeague::init_own_group_table` (0x9190d0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let league = League(this);
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    league.set_club_count(size(this, u32::MAX));
    let t = exe::crt_malloc((league.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    league.set_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x2b2);
        return 0;
    }
    league.set_games_per_club(league.club_count().wrapping_sub(1).wrapping_mul(league.legs()));
    covered!("cx16_11 european own group");
    let slots = read_usize(this + 0xf7); // the drawn group slots
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        exe::league_init_table_row(league, i as u8 as u32, read_usize(slots + i as isize as usize * 4), 0, 0); // cx16_11: slot i in row i
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

/// Comparer for `wc_europe_league_init_own_group_table`: it allocates the table and sends news, so only the original
/// runs; then the plan: 1, the group size and each row with its drawn slot in order.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_init_own_group_table(this: usize) -> u32 {
    let slots = read_usize(this + 0xf7);
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_EUROPE_LEAGUE_INIT_OWN_GROUP_TABLE)(this);
    let league = League(this);
    if r != 1 || slots == 0 || league.table() == 0 {
        return r;
    }
    let n = league.club_count().clamp(0, 16) as usize;
    let rows: Vec<usize> = (0..n).map(|k| read_usize(league.table() + k * league_table_row::SIZE)).collect();
    let plan: Vec<usize> = (0..n).map(|k| read_usize(slots + k * 4)).collect();
    SCOREBOARD.record(rows == plan, || format!("wc_europe_league_init_own_group_table({:#x}): rows {:x?} plan {:x?}", this, rows, plan));
    r
}

/// rule: the European qualifying table update is the league's, and once the whole qualifying is complete the play-off
/// winners are marked as qualified (once)
///
/// League::update_positions_and_status (`FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS`, through its detour); then, while the
/// done flag (`wc_cup_turnover_consts::EUROPE_FB`) is 0 and the section is complete (virtual `comp_vtable::CHECK_COMPLETE`
/// with 1, 0: push 0; push 1; mov ecx -> thiscall, ret 8; al), WcEuropeLeague::mark_play_off_qualifiers (0x91a7e0).
/// Original: `WcEuropeLeague::update_positions_and_status` (0x91a7b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_positions_and_status(this: usize) {
    call_section_routine(FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS, this);
    if read_u32(this + wc_cup_turnover_consts::EUROPE_FB) != 0 {
        return;
    }
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CHECK_COMPLETE));
    if complete(this, 1, 0) != 0 {
        covered!("cx16_12 european qualifying complete");
        call_section_routine(FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS, this);
    }
}

/// Comparer for `wc_europe_league_update_positions_and_status`: the table update cannot run twice; only the original.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_positions_and_status(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_UPDATE_POSITIONS_AND_STATUS)(this)
}

/// rule: when the European qualifying ends, the play-off winners get the qualified status (7) in their group tables;
/// without a play-off stage the best group runner-up qualifies instead, with its staff record
///
/// Once (the done flag `wc_cup_turnover_consts::EUROPE_FB` set to 1 on the first call; set already, nothing). Without a
/// play-off stage (+0x101 = −1): the top club of the runners-up stage's table (`league::STAGES`[+0x100], row 0) is
/// looked for in the group tables −1 to the last group (+0xff), in order; found, its row's `league_table_row::STATUS`
/// is 7 and the section's staff records for it (virtual `comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS`(club, 7, group,
/// 0, 0, 0): thiscall, ret 0x18); not found, the error box of line 0x65f. With one: for each group, each play-off team
/// row (`cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT` of the play-off stage) whose status (+5) is 1, its row in the
/// group (League::get_table_row `exe::league_get_table_row`, thiscall, ret 4) gets the status 7.
/// Original: `WcEuropeLeague::mark_play_off_qualifiers` (0x91a7e0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_mark_play_off_qualifiers(this: usize) {
    use wc_cup_turnover_consts as wt;
    if read_u32(this + wt::EUROPE_FB) != 0 {
        return;
    }
    write_u32(this + wt::EUROPE_FB, 1);
    let last = read_u8(this + 0xff) as i8 as i32;
    let play_off = read_u8(this + 0x101) as i8;
    if play_off == -1 {
        let runners = wc_europe_group(this, read_u8(this + 0x100) as i8 as i32);
        let best = read_usize(runners.table());
        for g in -1..=last {
            let group = wc_europe_group(this, g);
            for k in 0..group.club_count().max(0) as usize {
                let row = group.table() + k * league_table_row::SIZE;
                if read_usize(row) == best {
                    covered!("cx16_12 european best runner-up qualifies");
                    write_u8(row + league_table_row::STATUS, 7);
                    let records: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32 =
                        core::mem::transmute(read_usize(read_usize(this) + comp_vtable::UPDATE_STAFF_RECORDS_FOR_STATUS));
                    records(this, best, 7, g as u32, 0, 0, 0);
                    return;
                }
            }
        }
        return show_error_at(wt::WC_EUROPE_LEAGUE_CPP, 0x65f);
    }
    covered!("cx16_12 european play-off qualifiers");
    for g in -1..=last {
        let group = wc_europe_group(this, g);
        let mut i = 0i32;
        loop {
            let stage = Cup(wc_europe_group(this, read_u8(this + 0x101) as i8 as i32).addr());
            if i >= stage.team_count() as i32 {
                break;
            }
            let team = stage.team_table() + i as usize * 6;
            if read_u8(team + 5) == 1 { // cx16_12: a play-off winner
                let mut k = 0i32;
                while k < group.club_count() as i32 {
                    let row = exe::league_get_table_row(group, k);
                    if read_usize(row) == read_usize(team) {
                        write_u8(exe::league_get_table_row(group, k) + league_table_row::STATUS, 7);
                        break;
                    }
                    k += 1;
                }
            }
            i += 1;
        }
    }
}

/// Comparer for `wc_europe_league_mark_play_off_qualifiers`: it runs once (the flag) and adds staff records, so only the
/// original runs; then the plan: the flag set and, with a play-off stage, every winner's row in its group with status 7.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_mark_play_off_qualifiers(this: usize) {
    let first = read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 0;
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS)(this);
    let play_off = read_u8(this + 0x101) as i8;
    if !first || play_off == -1 {
        return;
    }
    let stage = Cup(wc_europe_group(this, play_off as i32).addr());
    let last = read_u8(this + 0xff) as i8 as i32;
    let mut missing = Vec::new();
    for i in 0..stage.team_count().max(0) as usize {
        let team = stage.team_table() + i * 6;
        if read_u8(team + 5) != 1 {
            continue;
        }
        let marked = (-1..=last).any(|g| {
            let group = wc_europe_group(this, g);
            (0..group.club_count().max(0) as usize).any(|k| {
                let row = group.table() + k * league_table_row::SIZE;
                read_usize(row) == read_usize(team) && read_u8(row + league_table_row::STATUS) == 7
            })
        });
        if !marked {
            missing.push(read_usize(team));
        }
    }
    SCOREBOARD.record(read_u32(this + wc_cup_turnover_consts::EUROPE_FB) == 1 && missing.is_empty(), || format!("wc_europe_league_mark_play_off_qualifiers({:#x}): winners not marked {:x?}", this, missing));
}

/// rule: the European qualifying progress of a nation: in a group the winner is 1, the runner-up 16 (10 at best), then
/// 19, 28, 37 and 46; in the play-off the winner 11 and the loser 16
///
/// The codes of `wc_europe_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcEuropeLeague::set_club_progress` (0x91b510, wc_europe_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let this = League(this);
    covered!("cx16_7 european progress");
    let e = exe::comp_stats_find_club_entry(this.comp_stats(), club);
    let (c, b, w) = wc_europe_progress_codes(this.addr(), stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_EUROPE_LEAGUE_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_europe_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_europe_league_set_club_progress(this, club, stage, current, best, worst))
}

/// rule: a nation's status in the European qualifying is still open (−2) while its stat 0x12 (its current progress
/// code) is below 12, and decided (−3) from 12 on
///
/// `qualifying_stat_byte`(club, 0x12) below 12: −2, otherwise −3 (a word).
/// Original: `WcEuropeLeague::get_qualification_status` (0x91b6e0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    covered!("cx16_8 european qualification status");
    if qualifying_stat_byte(this, club, 0x12) < 0xc { -2 } else { -3 } // cx16_8: open below 12
}

/// rule: in the European qualifying a nation's status changes down (−1) when its progress code (stat 0x12) falls below
/// 12 from 12 or more (stat 0x13, the previous code), up (1) when it rises from below 12 to 12 or more, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now), signed: now below 12, (before below 12) − 1; otherwise
/// before below 12 (1 or 0). The original returns the byte in al.
/// Original: `WcEuropeLeague::get_qualification_status_change` (0x91b710, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 european status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now < 0xc {
        (before < 0xc) as i8 - 1
    } else {
        (before < 0xc) as i8
    }
}

/// Comparers: the three only read the statistics; side by side on the result (the byte or word).
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status(this: usize, club: usize) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(IDX_WC_EUROPE_LEAGUE_GET_QUALIFICATION_STATUS)(this, club);
    let b = wc_europe_league_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_EUROPE_LEAGUE_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_europe_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

/// rule: the European qualifying groups (−1 is the section's own group, 0 on its stages) have five nations, with six
/// in groups 2, 4, 6 and 7 when one nation is exempt, in 2, 4 and 7 when two are, and in all but group 2 when three are
///
/// By the exempt count (`WC_EUROPE_EXEMPT`, signed byte): 0, 5; 1, 6 for groups 2, 4, 6, 7; 2, 6 for groups 2, 4, 7;
/// 3, 5 for group 2 and 6 for the others; any other count, the error box of line 0x25f and 5. `group` is the low byte
/// of the argument. Returns a word.
/// Original: `WcEuropeLeague::get_group_size` (0x918dd0, wc_europe_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let g = group as u8;
    covered!("cx16_9 european group size");
    match read_u8(this + WC_EUROPE_EXEMPT) as i8 {
        0 => 5,
        1 => if matches!(g, 2 | 4 | 6 | 7) { 6 } else { 5 },
        2 => if matches!(g, 2 | 4 | 7) { 6 } else { 5 }, // cx16_9: two exempt, three groups of six
        3 => 5 + (g != 2) as i16,
        _ => {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x25f);
            5
        }
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_get_group_size(this: usize, group: u32) -> i16 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(IDX_WC_EUROPE_LEAGUE_GET_GROUP_SIZE)(this, group);
    if (read_u8(this + WC_EUROPE_EXEMPT) as i8) < 0 || read_u8(this + WC_EUROPE_EXEMPT) > 3 {
        return a;
    }
    let b = wc_europe_league_get_group_size(this, group);
    SCOREBOARD.record(a == b, || format!("wc_europe_league_get_group_size({:#x}, {}) exempt {} original={} rust={}", this, group as u8 as i8, read_u8(this + WC_EUROPE_EXEMPT), a, b));
    a
}

/// rule: in the European qualifying a group winner (status 7) gets the World Cup qualification record (weight 100),
/// enters the World Cup and the list of qualified nations, and its status becomes 4; in the play-off the winner (1) gets
/// the same with weight 50 and the loser (2) the knocked-out record (weight 50) and ends its campaign; the other group
/// fates leave no record
///
/// `this.update_staff_records_for_status(club, outcome, stage, out, game, a6)` (virtual +0x44). In the groups (the play-off
/// stage `WC_EUROPE_PLAY_OFF_STAGE` not set, or the stage byte below it, signed): outcome 7, the World Cup
/// (`wc_europe_league_qualify_nation` with 0x64) and `*out` = 4 when there is `out`, −1; outcomes 1 and 0xfe, −1 without
/// any record; any other, 0. In the play-off stage itself: outcome 1, the World Cup with 0x32 (`*out` untouched), −1;
/// outcome 2, `european_cup_add_knocked_out_record`(club, `league::COMPETITION`, the fixture's `fixture::ROUND_PREFIX`,
/// its `fixture::ROUND_NAME`, 0x32) and the national team's campaign ended (`clear_national_team_competition`), 0; any
/// other, 0. Past the play-off stage, 0.
/// Original: `WcEuropeLeague::update_staff_records_for_status` (0x918980, wc_europe_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let game = Fixture(game);
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let stage = stage as u8 as i8;
    if play_off == -1 || stage < play_off {
        return match outcome as u8 {
            7 => {
                covered!("rx08_4 european group winner to the world cup");
                wc_europe_league_qualify_nation(club, 0x64);
                if out != 0 {
                    write_u8(out, 4);
                }
                u32::MAX
            }
            1 | 0xfe => u32::MAX,
            _ => 0,
        };
    }
    if stage != play_off {
        return 0;
    }
    match outcome as u8 {
        1 => {
            covered!("rx08_4 european play-off winner");
            wc_europe_league_qualify_nation(club, 0x32);
            u32::MAX
        }
        2 => {
            covered!("rx08_4 european play-off loser");
            // simplification: the original pushes the two words with the high halves of the registers; the callee reads words
            european_cup_add_knocked_out_record(club, League(this).competition().addr(), (game.round_prefix() as u16) as u32, (game.round_name() as u16) as u32, 0x32);
            clear_national_team_competition(Club(club)); // rx08_4: the play-off loser ends its campaign
            0
        }
        _ => 0,
    }
}

/// Comparer for `wc_europe_league_update_staff_records_for_status`: the records, the World Cup place and the end of the
/// campaign do not run twice, so only the original runs; the plan: the return, `*out` = 4 for a group winner with
/// `out`, and the World Cup's entrant count one more (when it had room) for a nation through.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let s = stage as u8 as i8;
    let in_groups = play_off == -1 || s < play_off;
    let through = (in_groups && outcome as u8 == 7) || (!in_groups && s == play_off && outcome as u8 == 1);
    let expected = if through || (in_groups && matches!(outcome as u8, 1 | 0xfe)) { u32::MAX } else { 0 };
    let world_cup = League(world_cup_record_and_object().1);
    if through && world_cup.is_null() {
        return o(this, club, outcome, stage, out, game, a6);
    }
    let n = if through { world_cup.entrant_count() } else { 0 };
    let room = through && n < world_cup.entrant_capacity();
    let r = o(this, club, outcome, stage, out, game, a6);
    let out_ok = !(in_groups && outcome as u8 == 7 && out != 0) || read_u8(out) == 4;
    let list_ok = !through || world_cup.entrant_count() == if room { n + 1 } else { n };
    SCOREBOARD.record(r == expected && out_ok && list_ok, || format!("wc_europe_league_update_staff_records_for_status({:#x}, {}, stage {}) returned {:#x} (expected {:#x}), list {}", club, outcome as u8, s, r, expected, n));
    r
}

/// rule: the news text of a nation's fate in the European qualifying: in the groups the winner "qualify for" the World
/// Cup (the full sentence is the shared group one) and the eliminated get the shared group sentence; the best runner-up
/// "are the best second placed team" (or "one of the three best", with one exempt nation) of the qualifying, "qualify
/// for" the World Cup in the short sentence; in the play-off the shared cup tie sentence, or in short "qualify for" the
/// World Cup for the winner and "out of" the European Championship for the loser
///
/// The arguments as `world_cup_build_fate_news_text` (the club index `club_index` in [`G_CLUB_TABLE`], the fate byte
/// `fate`, `stage`, the stage id `a3` (a signed word), the fixture `a4`, `a5`, `a6`, `a7`, `full_sentence` and the Text
/// `text`).
/// - Stage ids 1000 to 1009 (the groups): fate 7 with the full sentence, the shared group sentence
///   (`exe::comp_util_write_group_qualify_news`(club_index, a3, the competition, 7, full_sentence, text)); fate 7
///   without it, 0x9c470c with the World Cup's short name; fate 0xfe, the group sentence with −2; any other, nothing.
/// - Stage id 1020 (the runners-up) and fate 7: with the full sentence, 0x9c4800 when the exempt count is 1 (one of the
///   three best runners-up) or 0x9c4764 otherwise (the best), with the competition's long name (`competition::NAME`), the
///   club linked and the competition's long name linked (0x7d0); without it, 0xad4ab0 with the World Cup.
/// - Otherwise only with a fixture `a4` whose `fixture::ROUND_NAME` is 0xa4 (the play-off): with the full sentence, the
///   shared cup tie sentence (`exe::comp_util_write_cup_tie_result_news`(club_index, a4, a7, fate, a5, text)); without
///   it, fate 1 0xad4ab0 with the World Cup, any other fate 0x9c46b8 "out of" with the European Championship's short
///   name (`KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP`), still linked to the World Cup.
/// The short sentences go through `wc_europe_league_fate_sentence` (the club's link first, then the World Cup's).
/// Original: `WcEuropeLeague::build_fate_news_text` (0x918b00, wc_europe_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let _ = (stage, a6);
    let league = League(this);
    let comp = league.competition().addr();
    let club = club_at(club_index as i32 as isize);
    let (world_cup, _) = world_cup_record_and_object();
    let to_world_cup = |template: usize| wc_europe_league_fate_sentence(text, template, club, world_cup, world_cup + competition::SHORT_NAME, world_cup);
    let stage_id = a3 as u16 as i16;
    // simplification: the original leaves al with the fate byte or a callee's eax; the callers do not read it
    if (0x3e8..=0x3f1).contains(&stage_id) {
        match fate as u8 {
            7 if full_sentence != 0 => {
                covered!("rx08_4 european group winner, full sentence");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, 7, full_sentence, text);
            }
            7 => {
                covered!("rx08_4 european group winner");
                to_world_cup(0x9c470c);
            }
            0xfe => {
                covered!("rx08_4 european group out");
                exe::comp_util_write_group_qualify_news(club_index, a3, comp, -2, full_sentence, text); // rx08_4: the group exit sentence
            }
            _ => {}
        }
        return;
    }
    if stage_id == 0x3fc && fate as u8 == 7 {
        if full_sentence == 0 {
            to_world_cup(0xad4ab0);
            return;
        }
        covered!("rx08_4 european best runner-up");
        let template = if read_u8(this + WC_EUROPE_EXEMPT) == 1 { 0x9c4800 } else { 0x9c4764 }; // rx08_4: one exempt nation, three best runners-up
        let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        let club_gender = club.short_gender_name() as i8 as i32;
        let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
        sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club.addr() + club::SHORT_NAME, comp + competition::NAME);
        exe::text_set_string(text, 0xde1f64 as *const u8);
        exe::text_add_link(text, (club.addr() + club::SHORT_NAME) as *const u8, 0x7d5, club);
        exe::text_add_link(text, (comp + competition::NAME) as *const u8, 0x7d0, comp);
        return;
    }
    if a4 == 0 || read_u16(a4 as usize + fixture::ROUND_NAME) != 0xa4 {
        return;
    }
    covered!("rx08_4 european play-off fate");
    if full_sentence != 0 {
        exe::comp_util_write_cup_tie_result_news(club_index, a4, a7, fate, a5, text);
    } else if fate as u8 == 1 {
        to_world_cup(0xad4ab0);
    } else {
        let euro = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP).wrapping_mul(0x6b));
        wc_europe_league_fate_sentence(text, 0x9c46b8, club, euro, euro + competition::SHORT_NAME, world_cup); // rx08_4: the loser "out of" the European Championship
    }
}

/// Comparer for `wc_europe_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, a3: u32, a4: u32, a5: u32, a6: u32, a7: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_EUROPE_LEAGUE_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, text),
        |t| wc_europe_league_build_fate_news_text(this, club_index, fate, stage, a3, a4, a5, a6, a7, full_sentence, t),
        || format!("wc_europe_league_build_fate_news_text(club {}, fate {}, stage id {}, full {})", club_index, fate as u8, a3 as u16 as i16, full_sentence));
}

/// rule: the European qualifying draw: the nations, strongest first, go into the groups pot by pot; in each pot the
/// groups are shuffled and each one with room takes the next nation, so every pot gives one nation to each group
///
/// The group count is the last group stage (`WC_EUROPE_LAST_GROUP`, signed byte) + 2 (the section's own group and the
/// group stages); a slot table of six per group (`exe::crt_malloc` of the signed byte groups × 6, times 4) in +0xf7
/// (without memory, the error box of line 0x27b and nothing else), dealt by `wc_europe_league_deal_groups`. The unused
/// slots keep what `malloc` left.
/// Original: `WcEuropeLeague::draw_groups` (0x918ec0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_draw_groups(this: usize) {
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let slots = exe::crt_malloc(((groups.wrapping_mul(6) as i32) << 2) as u32);
    write_usize(this + 0xf7, slots);
    if slots == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x27b);
        return;
    }
    covered!("rx08_4 european group draw");
    wc_europe_league_deal_groups(this, slots, groups);
}

/// Comparer for `wc_europe_league_draw_groups`: the original draws into its own slot table; the generator goes back to
/// where it was and the copy deals the same draws into a table of its own (its draws not counted); the dealt slots of
/// every group must match. The generator is left as the original left it.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_draw_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_DRAW_GROUPS);
    let rng_before = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    o(this);
    let slots = read_usize(this + 0xf7);
    if slots == 0 {
        return;
    }
    let rng_after = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    rng_before.write_back();
    let groups = (read_u8(this + WC_EUROPE_LAST_GROUP) as i8).wrapping_add(2);
    let mut mine = [0usize; 60];
    let filled = crate::rng::pause_trace(|| crate::rng::without_counting(|| wc_europe_league_deal_groups(this, mine.as_mut_ptr() as usize, groups)));
    rng_after.write_back();
    let wrong = (0..groups.clamp(0, 10) as usize).flat_map(|g| (0..filled[g].clamp(0, 6) as usize).map(move |k| g * 6 + k))
        .find(|&s| read_usize(slots + s * 4) != mine[s]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_draw_groups({:#x}): slot {:?} original {:#x} rust {:#x}", this, wrong, wrong.map_or(0, |s| read_usize(slots + s * 4)), wrong.map_or(0, |s| mine[s])));
}

/// rule: each European qualifying group after the section's own is a league stage of its drawn nations (or of a given
/// list), home and away with the section's points, category and tie-breaks, and its draw news goes out
///
/// `this.create_group_stage(group, teams, count)`: the group's schedule (`wc_europe_league_schedule`). Without `teams`:
/// the group's size (WcEuropeLeague::get_group_size(group), a signed word), a list of that many (`exe::crt_malloc`;
/// none: the error box of line 0x3a7 and 0) copied from the drawn slots of group + 1 (+0xf7, six per group), and the
/// count is the size; with `teams`, the schedule is freed (`exe::crt_free`) and none is passed. The stage
/// (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58:
/// the section, the count, the list, 2, the record, the schedule, the rounds, `league::WIN_POINTS`,
/// `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, the tie-breaks `league::TIEBREAKS`, the places
/// `league::PROMOTED_COUNT`, the year, the group, the stage id, `league::IMPORTANCE`, 4, 0, `league::LEAGUE_FLAGS`, −1,
/// 0, 2) into `league::STAGES`[group]; none: the box of line 0x3c3 and 0. Then the drawn list and the schedule freed,
/// `league::CURRENT_STAGE` = group, the group draw news (`exe::league_send_group_draw_news` of the stage) and 1.
/// Original: `WcEuropeLeague::create_group_stage` (0x9196d0, wc_europe_league.cpp, thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let league = this;
    let cpp = wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP;
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let mut schedule = wc_europe_league_schedule(this.addr(), group, &mut rounds, &mut stage_id);
    let (list, count) = if teams == 0 {
        let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
        let size = size_of(this.addr(), group);
        let list = exe::crt_malloc((size as i32 as u32).wrapping_mul(4));
        if list == 0 {
            show_error_at(cpp, 0x3a7);
            return 0;
        }
        let drawn = read_usize(this.addr() + 0xf7) + (group as usize + 1) * 6 * 4; // rx08_5: the group after the section's own
        for k in 0..(size as i32).max(0) as usize {
            write_usize(list + k * 4, read_usize(drawn + k * 4)); // rx08_5: the drawn nations in draw order
        }
        (list, size as u16 as u32)
    } else {
        if schedule != 0 {
            exe::crt_free(schedule);
        }
        schedule = 0;
        (teams, count)
    };
    covered!("rx08_5 european group stage");
    let p = exe::crt_operator_new(0xee);
    // simplification: the original pushes the rounds, the stage id, the bytes and the words with garbage in the high
    // halves of the registers, which the constructor does not read: the copy passes clean values
    let stage = if p != 0 {
        construct(p, this.addr(), count, list, 2, league.competition().addr(), schedule, rounds as u16 as u32, league.win_points() as u32, league.draw_points() as u32,
            league.match_category() as u32, this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, league.year() as u16 as u32, group, stage_id as u16 as u32,
            this.importance() as u32, 4, 0, (this.league_flags() as u16) as u32, -1, 0, 2)
    } else {
        0
    };
    let slot = league.stages() + group as usize * 4;
    write_usize(slot, stage);
    if read_usize(slot) == 0 {
        show_error_at(cpp, 0x3c3);
        return 0;
    }
    if teams == 0 {
        exe::crt_free(list);
    }
    if schedule != 0 {
        exe::crt_free(schedule);
    }
    league.set_current_stage(group as usize);
    exe::league_send_group_draw_news(read_usize(slot));
    1
}

/// Comparer for `wc_europe_league_create_group_stage`: it builds the stage and sends news, so only the original runs;
/// the plan: 1, the stage in its slot with the count's rows (the group's size when drawn) and the current stage the group.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_group_stage(this: usize, group: u32, teams: usize, count: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u32>(IDX_WC_EUROPE_LEAGUE_CREATE_GROUP_STAGE);
    let r = o(this, group, teams, count);
    let league = League(this);
    let stage = League(read_usize(league.stages() + group as usize * 4));
    if r != 1 || stage.is_null() || (teams == 0 && read_u8(this + WC_EUROPE_EXEMPT) > 3) {
        return r;
    }
    let size_of: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let n = if teams == 0 { size_of(this, group) } else { count as i16 };
    let ok = stage.club_count() == n && league.current_stage() == group as usize;
    SCOREBOARD.record(ok, || format!("wc_europe_league_create_group_stage({:#x}, {}): {} rows of {}, current stage {}", this, group, stage.club_count(), n, league.current_stage() as i32));
    r
}

/// rule: when the European groups end, the best runners-up go to the play-off: one of them, drawn, plays the Asian
/// section, the others play each other home and away; every other nation not through (status 7) is out
///
/// Without a runners-up stage (+0x100 = −1) or a play-off stage (`WC_EUROPE_PLAY_OFF_STAGE` = −1): every row of every
/// group (`wc_europe_league_groups`) not at status 7 (`league_table_row::STATUS`) is out (`wc_europe_league_eliminate`).
/// Otherwise the play-off size N (8 with up to one exempt nation, 6 with more) and a list of N + 1 (`exe::crt_malloc`;
/// none: the error box of line 0x444 and nothing else) with the clubs of the runners-up stage's rows 0..N
/// (`exe::league_get_table_row`); one drawn (`exe::utils_random`(N + 1)) goes to Asia and its place takes the last one
/// (which stays at the end too). Each group row: a club among the first N of the list (the first match), or the Asian
/// one, goes on (status 1) with the qualified record for round 0xa0 (`european_cup_add_qualified_record`(club, the
/// competition, 0, 0xa0, 0x1e)); the Asian section's record (`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`) only
/// when the list entry found is the Asian one; any other row not at status 7 is out. Then, on the last group processed
/// (the runners-up stage when there is no group), its first rows down to its row count less N get the section's
/// virtual `SET_CLUB_PROGRESS`(club, −1, 1, 1, 1). The play-off schedule (`wc_europe_league_schedule` of the play-off
/// stage), the stage (`exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the section, N, the list, the rounds, the record, the schedule, the year, the play-off stage, 4, the stage id,
/// 0x14, 0, 0, 0, 0) into `league::STAGES`[play-off]; the list and the schedule freed; no stage: the box of line 0x49f;
/// otherwise its first round (`exe::cup_set_up_round`(stage, 0)) and `league::CURRENT_STAGE` the play-off stage.
/// Original: `WcEuropeLeague::create_play_off_stage` (0x919910, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_create_play_off_stage(this: usize) {
    let league = League(this);
    let runners_up = read_u8(this + 0x100) as i8;
    let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8;
    let not_through = |l: League, k: usize| read_u8(l.table() + k * league_table_row::SIZE + league_table_row::STATUS) != 7;
    if runners_up == -1 || play_off == -1 {
        covered!("rx08_5 european groups end without a play-off");
        for group in wc_europe_league_groups(this) {
            for k in 0..group.club_count().max(0) as usize {
                if not_through(group, k) {
                    wc_europe_league_eliminate(this, table_club(group, k));
                }
            }
        }
        return;
    }
    let n: i16 = if read_u8(this + WC_EUROPE_EXEMPT) <= 1 { 8 } else { 6 }; // rx08_5: eight play-off places with up to one exempt nation
    let list = exe::crt_malloc((n as u32 + 1) * 4);
    if list == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x444);
        return;
    }
    covered!("rx08_5 european play-off");
    let runners_up_stage = League(read_usize(league.stages() + runners_up as isize as usize * 4));
    for k in 0..=n as usize {
        write_usize(list + k * 4, read_usize(exe::league_get_table_row(runners_up_stage, k as i32)));
    }
    let drawn = exe::utils_random(n as i32 + 1) as usize;
    let asia = read_usize(list + drawn * 4);
    write_usize(list + drawn * 4, read_usize(list + n as usize * 4));
    let mut last = runners_up_stage;
    for group in wc_europe_league_groups(this) {
        last = group;
        for k in 0..group.club_count().max(0) as usize {
            let club = table_club(group, k);
            let found = (0..n as usize).find(|&j| read_usize(list + j * 4) == club).unwrap_or(n as usize);
            if found < n as usize || club == asia {
                write_u8(group.table() + k * league_table_row::SIZE + league_table_row::STATUS, 1);
                let comp = if read_usize(list + found * 4) == asia { club_comp_record(read_usize(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)) } else { league.competition().addr() };
                european_cup_add_qualified_record(club, comp, 0, 0xa0, 0x1e);
            } else if not_through(group, k) {
                wc_europe_league_eliminate(this, club);
            }
        }
    }
    let progress: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_CLUB_PROGRESS));
    for k in 0..(last.club_count() as i32 - n as i32).max(0) as usize {
        progress(this, table_club(last, k), -1, 1, 1, 1);
    }
    let (mut rounds, mut stage_id) = (0i16, 0i16);
    let schedule = wc_europe_league_schedule(this, play_off as u8 as u32, &mut rounds, &mut stage_id);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, n as u32, list, rounds as u16 as u32, league.competition().addr(), schedule, league.year() as u16 as u32, play_off as u8 as u32, 4, stage_id as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = league.stages() + play_off as isize as usize * 4;
    write_usize(slot, stage);
    exe::crt_free(list);
    exe::crt_free(schedule);
    if read_usize(slot) == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x49f);
        return;
    }
    exe::cup_set_up_round(read_usize(slot), 0);
    league.set_current_stage(play_off as isize as usize);
}

/// Comparer for `wc_europe_league_create_play_off_stage`: it draws, writes records and builds the stage, so only the
/// original runs; the plan: with both stages set, the current stage is the play-off stage, which exists.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_create_play_off_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE);
    let both = read_u8(this + 0x100) != 0xff && read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) != 0xff;
    o(this);
    if both {
        let league = League(this);
        let play_off = read_u8(this + WC_EUROPE_PLAY_OFF_STAGE) as i8 as isize as usize;
        let stage = read_usize(league.stages() + play_off * 4);
        SCOREBOARD.record(stage != 0 && league.current_stage() == play_off, || format!("wc_europe_league_create_play_off_stage({:#x}): stage {:#x}, current {}", this, stage, league.current_stage() as i32));
    }
}

/// rule: the European runners-up are ranked by their record against the top four of their own group: each group's
/// runner-up row in the runners-up table is the one of a four-team sub-league of the group's games among its top four
///
/// The period: from the first round date to the last one (`league::ROUND_DATES`, day and year offset + the year;
/// Date::Date `exe::new_date`). For each group (`wc_europe_league_groups`): its table sorted (`exe::league_sort_table`),
/// its runner-up (row 1, `exe::league_get_table_row`) and the ids of its top four in a new list (`exe::crt_malloc`(0x10);
/// none: the error box of line 0x57c and return, without the final sort); the sub-league of their games
/// (`exe::crt_operator_new`(0xee) and SubLeague::SubLeague `FN_SUBLEAGUE_SUBLEAGUE`, thiscall, ret 0x30: 4, the ids, −1,
/// −1, the two dates, the competition id, `league::WIN_POINTS`, `league::DRAW_POINTS`, the tie-breaks, 2 × n × (n − 1)
/// with n the group's row count, and the section's stored results +0x10); when the runner-up is one of its first 4
/// rows, that row is copied into the next row of the runners-up table (+0x100; the row advances only on a copy) with its
/// head-to-head and position-history pointers cleared (the sub-league owns them); the sub-league destroyed (its virtual
/// 0 with 1) and the list freed. At the end the runners-up table sorted.
/// Original: `WcEuropeLeague::fill_runners_up_table` (0x91a240, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_fill_runners_up_table(this: usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let league = this;
    let (mut from, mut to) = ([0u8; 8], [0u8; 8]);
    let dates = league.round_dates();
    exe::new_date(from.as_mut_ptr() as usize, read_u16(dates) as u32, read_u16(dates + 2).wrapping_add(league.year() as u16) as u32);
    let last = dates.wrapping_add((league.round_count() as i32 as usize).wrapping_mul(league_round_date::SIZE)).wrapping_sub(league_round_date::SIZE);
    exe::new_date(to.as_mut_ptr() as usize, read_u16(last) as u32, read_u16(last + 2).wrapping_add(league.year() as u16) as u32);
    let runners_up = || League(read_usize(league.stages() + read_u8(this.addr() + 0x100) as i8 as isize as usize * 4));
    let mut row = 0usize;
    for group in wc_europe_league_groups(this.addr()) {
        exe::league_sort_table(group, 0);
        let runner = read_usize(exe::league_get_table_row(group, 1));
        let ids = exe::crt_malloc(0x10);
        if ids == 0 {
            show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x57c);
            return;
        }
        for i in 0..4 {
            write_u32(ids + 4 * i as usize, read_u32(read_usize(exe::league_get_table_row(group, i))));
        }
        let p = exe::crt_operator_new(0xee);
        let n = group.club_count() as i32;
        let sub = if p != 0 {
            exe::new_subleague(p, 4, ids, -1, -1, from.as_ptr() as usize, to.as_ptr() as usize, read_i32(league.competition().addr()), league.win_points() as u32, league.draw_points() as u32, this.addr() + league::TIEBREAKS, n.wrapping_sub(1).wrapping_mul(n).wrapping_mul(2) as u32, read_usize(this.addr() + 0x10)) // rx08_5: both legs between the four
        } else {
            0
        };
        if let Some(j) = (0..4).find(|&j| read_usize(exe::league_get_table_row(sub, j)) == runner) {
            covered!("rx08_5 european runner-up row copied");
            let target = runners_up().table() + row;
            core::ptr::copy_nonoverlapping((League(sub).table() + j as usize * league_table_row::SIZE) as *const u8, target as *mut u8, league_table_row::SIZE);
            write_u32(target + league_table_row::POSITION_HISTORY, 0);
            write_u32(target + league_table_row::H2H_ENTRIES, 0);
            row += league_table_row::SIZE;
        }
        if sub != 0 {
            let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(sub) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
            destroy(sub, 1);
        }
        exe::crt_free(ids);
    }
    exe::league_sort_table(runners_up(), 0);
}

/// Comparer for `wc_europe_league_fill_runners_up_table`: side by side on the runners-up table's rows; the groups'
/// sorts and the sub-leagues are the same on both sides.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_fill_runners_up_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_FILL_RUNNERS_UP_TABLE);
    let stage = League(read_usize(League(this).stages() + read_u8(this + 0x100) as i8 as isize as usize * 4));
    if stage.is_null() || stage.table() == 0 {
        return o(this);
    }
    let regions = [(stage.table(), stage.club_count().clamp(0, 10) as usize * league_table_row::SIZE)];
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || wc_europe_league_fill_runners_up_table(this));
    SCOREBOARD.record(same, || format!("wc_europe_league_fill_runners_up_table({:#x}): {}", this, first_differences(&mo, &mr).join(", ")));
}

/// rule: before the European qualifying starts every nation of the pool gets its predicted final position from its
/// seeding pot (the strongest pot first), and all the European national teams leave their continental competitions to
/// play this one
///
/// Without statistics (`league::COMP_STATS`): the error box of line 0x7a0 and nothing else. The pool (+0xee, count
/// +0xf2) sorted by strength (`exe::crt_qsort`(pool, count, 4, 0x4b07d0)); the European national teams released
/// (`exe::nationalteams_release_competitions_of_continent`(national teams, the European continent record —
/// `G_CONTINENT_TABLE` + 0xc6 × `KEY_CONTINENT_EUROPE` — and 0)); then each nation k of the pool (a signed word below
/// the count): its predicted position (`wc_europe_league_predicted_position`; Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, nation, position)) and its current competition this one
/// (`exe::nationalteams_set_current_competition`(national teams, `club::NATION`, `league::COMPETITION`)).
/// Original: `WcEuropeLeague::set_predicted_positions` (0x91b1b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_predicted_positions(this: usize) {
    let league = League(this);
    if league.comp_stats() == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x7a0);
        return;
    }
    covered!("rx08_6 european predicted positions");
    let pool = read_usize(this + 0xee);
    exe::crt_qsort(pool, read_u32(this + 0xf2), 4, 0x4b07d0);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + 0xf2) {
        let nation = Club(read_usize(read_usize(this + 0xee) + k as isize as usize * 4));
        if let Some(position) = wc_europe_league_predicted_position(read_u8(this + WC_EUROPE_EXEMPT), k) { // rx08_6: the seeding pot's first place
            exe::comp_stats_set_club_final_position(league.comp_stats(), nation, position);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), nation.nation(), league.competition().addr()); // rx08_6: the nation plays this one
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_europe_league_set_predicted_positions`: it moves the national teams between competitions, so only
/// the original runs; the plan: the pool sorted as a copy sorted the same way before.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_EUROPE_LEAGUE_SET_PREDICTED_POSITIONS);
    let (pool, n) = (read_usize(this + 0xee), read_i32(this + 0xf2).clamp(0, 64) as usize);
    if League(this).comp_stats() == 0 || pool == 0 {
        return o(this);
    }
    let mut sorted: Vec<usize> = (0..n).map(|k| read_usize(pool + k * 4)).collect();
    exe::crt_qsort(sorted.as_mut_ptr() as usize, n as u32, 4, 0x4b07d0);
    o(this);
    let wrong = (0..n).find(|&k| read_usize(pool + k * 4) != sorted[k]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_set_predicted_positions({:#x}): pool place {:?} differs from the sorted copy", this, wrong));
}

/// The European group table of group `g` (−1 the section itself, else `league::STAGES`[g]).
pub(crate) unsafe fn wc_europe_group(this: usize, g: i32) -> League {
    if g == -1 { League(this) } else { League(read_usize(League(this).stages().wrapping_add((g as isize as usize).wrapping_mul(4)))) }
}

/// The European progress codes (current, best, worst) at `stage` (signed byte). In a group (up to the last group stage,
/// +0xff): 1 → 1, 2 → 16 (the best 10), 3 → 19, 4 → 28, 5 → 37, 6 → 46. In the play-off stage (+0x101): 1 → 11, 2 → 16.
/// Any other stage, as they came. Defect of the original [inference]: a group runner-up's best becomes 10 where its
/// current and worst become 16.
pub(crate) unsafe fn wc_europe_progress_codes(this: usize, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage <= read_u8(this + 0xff) as i8 {
        let band = |v: u8, second: u8| match v {
            1 => 1,
            2 => second,
            3 => 0x13,
            4 => 0x1c,
            5 => 0x25,
            6 => 0x2e,
            _ => v,
        };
        return (band(current, 0x10), band(best, 0xa), band(worst, 0x10)); // cx16_7: the European group bands
    }
    if stage == read_u8(this + 0x101) as i8 {
        let play_off = |v: u8| match v {
            1 => 0xb,
            2 => 0x10,
            _ => v,
        };
        return (play_off(current), play_off(best), play_off(worst));
    }
    (current, best, worst)
}

/// The World Cup's record (`KEY_CLUB_COMP_FIFA_WORLD_CUP` in [`G_CLUB_COMP_TABLE`], 0x6b bytes each) and its
/// competition object ([`G_COMP_OBJECT_TABLE`]).
pub(crate) unsafe fn world_cup_record_and_object() -> (usize, usize) {
    let key = read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
    (read_usize(G_CLUB_COMP_TABLE).wrapping_add(key.wrapping_mul(0x6b)), read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(key.wrapping_mul(4))))
}

/// A nation through to the World Cup from the European qualifying: the World Cup qualification record (weight
/// `weight`), its place in the World Cup (`exe::worldcup_add_qualified_team`) and in the national teams' list of
/// qualified nations (NationalTeams::add_qualified_nation on [`G_NATIONAL_TEAMS`]).
pub(crate) unsafe fn wc_europe_league_qualify_nation(club: usize, weight: u32) {
    let (record, world_cup) = world_cup_record_and_object();
    european_cup_add_qualified_record(club, record, 0, 0, weight);
    exe::worldcup_add_qualified_team(world_cup, club); // rx08_4: the nation enters the World Cup
    exe::nationalteams_add_qualified_nation(read_usize(G_NATIONAL_TEAMS), club, record);
}

/// Prints one news sentence of a nation and a competition into the global buffer 0xde1f64 (`FN_LANGLIB_SPRINTF`: the
/// template, the club's gender twice, the competition's gender twice, the club's short name and the competition's
/// `name`), sets it as the text and links the club (0x7d5) and the competition `linked` (its short name, 0x7d0).
pub(crate) unsafe fn wc_europe_league_fate_sentence(text: usize, template: usize, club: Club, comp: usize, name: usize, linked: usize) {
    let sprintf: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let club_gender = club.short_gender_name() as i8 as i32;
    let comp_gender = read_u8(comp + competition::SHORT_GENDER_NAME) as i8 as i32;
    let club_name = club.addr() + club::SHORT_NAME;
    sprintf(0xde1f64, template, club_gender, club_gender, comp_gender, comp_gender, club_name, name);
    exe::text_set_string(text, 0xde1f64 as *const u8);
    exe::text_add_link(text, club_name as *const u8, 0x7d5, club);
    exe::text_add_link(text, (linked + competition::SHORT_NAME) as *const u8, 0x7d0, linked);
}

/// Deals the European qualifying's entry pool (+0xee, already sorted by strength; its count +0xf2, a dword) into the
/// group slots `slots` (six per group): pot by pot (at most six), the group order shuffled (20 swaps of two drawn
/// places, `exe::utils_random`(groups) twice each), then each group in that order takes the next nation while it has
/// room (its size from WcEuropeLeague::get_group_size of the group index less one, a signed word). Returns how many
/// nations each group got.
pub(crate) unsafe fn wc_europe_league_deal_groups(this: usize, slots: usize, groups: i8) -> [i8; 10] {
    let size: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE);
    let mut filled = [0i8; 10];
    let mut order: [i8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut next = 0i8;
    let pool_left = |next: i8| (next as i32) < read_i32(this + 0xf2);
    for _pot in 0..6 {
        if !pool_left(next) {
            break;
        }
        for _ in 0..20 {
            let a = exe::utils_random(groups as i32) as i8 as usize;
            let b = exe::utils_random(groups as i32) as i8 as usize;
            order.swap(a, b);
        }
        let mut k = 0i16;
        while k < groups as i16 && pool_left(next) {
            let g = order[k as usize];
            if g < groups {
                let had = filled[g as usize];
                if (had as i16) < size(this, g.wrapping_sub(1) as u8 as u32) { // rx08_4: the group's size by its index less one
                    let nation = read_usize(read_usize(this + 0xee) + next as usize * 4);
                    write_usize(slots + (g as usize * 6 + had as usize) * 4, nation);
                    next += 1;
                    filled[g as usize] = had + 1;
                }
            }
            k += 1;
        }
    }
    filled
}

/// The section's virtual `CREATE_SCHEDULE` (+0x3c) for `stage`: the round table, with the round count and the stage id
/// written into the two words.
pub(crate) unsafe fn wc_europe_league_schedule(this: usize, stage: u32, rounds: &mut i16, stage_id: &mut i16) -> usize {
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    create_schedule(this, stage, rounds, stage_id, 0)
}

/// The European qualifying sections, from the section itself (−1) to the last group stage (`WC_EUROPE_LAST_GROUP`,
/// signed), in order; empty when the last group is below −1.
pub(crate) unsafe fn wc_europe_league_groups(this: usize) -> impl Iterator<Item = League> {
    let last = read_u8(this + WC_EUROPE_LAST_GROUP) as i8 as i32;
    (-1..=last).map(move |s| if s == -1 { League(this) } else { League(read_usize(League(this).stages() + s as usize * 4)) })
}

/// The club of row `k` of a league table (`league::TABLE`, `league_table_row::SIZE` bytes each).
pub(crate) unsafe fn table_club(league: League, k: usize) -> usize {
    read_usize(league.table() + k * league_table_row::SIZE)
}

/// A nation out of the European qualifying: the failed-to-qualify record (`european_cup_add_failed_to_qualify_record`
/// with the competition, 0x41c and 0x64) and the end of its campaign (`clear_national_team_competition`).
pub(crate) unsafe fn wc_europe_league_eliminate(this: usize, club: usize) {
    european_cup_add_failed_to_qualify_record(club, League(this).competition().addr(), 0x41c, 0x64);
    clear_national_team_competition(Club(club)); // rx08_5: out of the qualifying
}

/// The predicted final position of the nation at `k` in the strength-sorted European pool, by the exempt count: the
/// seeding pots (the first nine or ten are pot 1, the next ones pot 2 and so on), each pot's first place as the
/// prediction. `None` for an exempt count outside 0..=3 (no prediction).
pub(crate) fn wc_europe_league_predicted_position(exempt: u8, k: i16) -> Option<u32> {
    let pots: &[(i16, u32)] = match exempt {
        0 => &[(10, 1), (15, 11), (19, 16), (20, 20), (30, 21), (40, 31), (i16::MAX, 41)],
        1 => &[(9, 1), (14, 10), (18, 15), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        2 => &[(9, 1), (13, 10), (16, 14), (18, 17), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        3 => &[(8, 1), (12, 9), (15, 13), (16, 16), (24, 17), (32, 25), (40, 33), (i16::MAX, 41)],
        _ => return None,
    };
    pots.iter().find(|&&(below, _)| k < below).map(|&(_, position)| position)
}

/// WcEuropeLeague routines called directly (thiscall): the play-off stage (ret 0) and the group size (ret 4).
pub(crate) const FN_WC_EUROPE_LEAGUE_CREATE_PLAY_OFF_STAGE: usize = 0x919910;

pub(crate) const FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE: usize = 0x918dd0;

/// WcEuropeLeague::mark_play_off_qualifiers and League::update_positions_and_status (thiscall, ret 0), called directly
/// by the table update (the second through its detour).
pub(crate) const FN_WC_EUROPE_LEAGUE_MARK_PLAY_OFF_QUALIFIERS: usize = 0x91a7e0;

pub(crate) const FN_LEAGUE_UPDATE_POSITIONS_AND_STATUS: usize = 0x684cd0;

/// The European qualifying's play-off stage index (+0x101, signed byte; −1 before the format is set), after the last
/// group stage (`WC_EUROPE_LAST_GROUP`, +0xff) and the runners-up stage (+0x100).
pub(crate) const WC_EUROPE_PLAY_OFF_STAGE: usize = 0x101;
