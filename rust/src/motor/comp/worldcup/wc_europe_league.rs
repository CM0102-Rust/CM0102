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
