use super::*;

/// rule: the South American qualifier is a single home and away league, 3/1 points, tiebreak 1, 2 and 3; the direct
/// places (`set_promoted_count`) are four minus the exempt nations (the world champion and South American hosts,
/// counted in `wc_south_american_league_fields::EXEMPT_COUNT` and
/// `wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT` [inference]); the fifth goes to the play-off (+0xbf = 1);
/// category 10, 7 substitutes and 3 changes; starts on 25 of month 0 on weekday 5
/// Original: `WcSouthAmericanLeague::set_up_rules` (0x921040, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_set_up_rules(this: usize) -> u32 {
    let this = League(this);
    let list = read_usize(G_COMP_OBJECT_TABLE);
    if list != 0 {
        let o = read_usize(list + read_usize(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION).wrapping_mul(4));
        if o != 0 {
            write_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT, read_u8(o + wc_oceania_league_fields::EXEMPT_COUNT));
        }
    }
    covered!("cqj_12 south america rules");
    wc_europe_league_write_rule_fields(this, &SOUTH_AMERICA_QUALIFIER_RULE_FIELDS);
    this.set_win_points(3); // cqj12: the win in the South American qualifiers
    this.set_promoted_count(4u8.wrapping_sub(read_u8(this.addr() + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT)).wrapping_sub(read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT))); // cqj12: the direct places of South America
    let day = 0x19; // cqj12: the start day of the South American qualifier
    let word = comp_date_word(this, day, 0, 5);
    finish_rules_setup(this, word, 0)
}

/// The season turnover of the South American World Cup qualifying: last campaign's table and round dates freed, the
/// season archived, the next campaign's year, the exempt nations counted again, the rules and the team slots.
///
/// `qualifying_turnover_free_table` (no stages: the South American section is one league); the season archived
/// (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it reaches the game's
/// (`advance_qualifying_campaign`); `.set_active(0)`; the exempt nations (WcSouthAmericanLeague::count_exempt_nations,
/// thiscall, ret 0; its return not read); the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x2e1) and
/// the team slots (WcSouthAmericanLeague::build_team_slots, thiscall, ret 0; 0: the box of line 0x2e8), whose eax it
/// returns. After a box the copy returns 0 (the original the box's eax).
/// Original: `WcSouthAmericanLeague::start_new_season` (0x921470, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 south american qualifying rollover");
    this.set_active(0);
    call_qualifier_routine(wt::FN_WC_SOUTH_AMERICAN_LEAGUE_COUNT_EXEMPT_NATIONS, this.addr());
    if call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2e1);
        return 0;
    }
    let r = call_qualifier_routine(wt::FN_WC_SOUTH_AMERICAN_LEAGUE_BUILD_TEAM_SLOTS, this.addr());
    if r == 0 {
        show_error_at(wt::WC_SOUTH_AMERICAN_LEAGUE_CPP, 0x2e8);
        return 0;
    }
    r
}

/// Comparer for `wc_south_american_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_START_NEW_SEASON);
    compare_qualifying_turnover("wc_south_american_league_start_new_season", League(this), || o(this))
}

/// rule: the South American qualifying starts on its start date: with the full day update and the league not yet
/// started, the fixtures are made, the table sorted, the predictions set and every South American national team
/// leaves its continental competitions to play this one; before the start nothing runs, not even the league day
///
/// The draw date (`oceania_nations_draw_date`): before it returns 0 (al) without the league day. On it, with
/// `league::ACTIVE` 0 and `full`: the fixtures (`exe::league_create_fixtures`; 0: the error box of line 0x27b and 0),
/// the sorted table (`exe::league_sort_table(this, 0)`; 0: the box of line 0x282 and 0), `league::ACTIVE` = 1, the
/// predicted positions (the virtual `comp_vtable::SET_PREDICTED_POSITIONS`), the South American national teams released
/// from their continental competitions (NationalTeams::release_competitions_of_continent 0x775070 on
/// [`G_NATIONAL_TEAMS`] with the South American continent record — `G_CONTINENT_TABLE` + 0xc6 × the key — and 0: 2
/// pushes; mov ecx -> thiscall, ret 8) and each nation of the table rows (the club of row k, 0x3b bytes from
/// `league::TABLE`, its `club::NATION`; k word signed below `league::CLUB_COUNT`) entered in this competition
/// (`exe::nationalteams_set_current_competition`(national teams, nation, the record `league::COMPETITION`): 2 pushes;
/// mov ecx -> thiscall, ret 8). Then the league day with `full` (`League::update_for_date`), whose al it passes on.
/// Original: `WcSouthAmericanLeague::update_for_date` (0x921160, wc_south_american_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let league = this;
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    covered!("cqz_12 south american day");
    let (before, draw_day) = qualifying_day_before_draw(league, today);
    // simplification: on the returns before the start and on the errors the original returns only al (0); the caller reads the byte
    if before {
        return 0;
    }
    if draw_day && league.active() == 0 && full != 0 {
        covered!("cqz_12 south american start");
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xad4ce0, 0x27b);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xad4ce0, 0x282);
            return 0;
        }
        league.set_active(1);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_SOUTH_AMERICA).wrapping_mul(0xc6));
        exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), continent, 0); // cqz12: the South American teams leave their continental competitions
        let mut k = 0i16;
        while k < this.club_count() {
            let club = Club(read_usize(this.table() + (k as isize * 0x3b) as usize));
            exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation(), this.competition().addr());
            k = k.wrapping_add(1);
        }
    }
    exe::league_update_for_date(this, today, full) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_UPDATE_FOR_DATE);
    let league = League(this);
    let starts = full != 0 && league.active() == 0 && qualifying_day_before_draw(league, today).1;
    let r = o(this, today, full);
    if starts {
        SCOREBOARD.record(league.active() == 1, || format!("wc_south_american_league_update_for_date({:#x}): the start did not activate it", this));
    }
    r
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
    let this = League(this);
    let league = this;
    let edition = qualifying_campaign_year(year as i16).max(0x7d4); // cqz4: the South American campaign, from 2004
    start_qualifying_section(league, 0x970e2c, record, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, edition);
    this.set_active((0) as usize);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqz_4 south american qualifying");
    league.set_league_flags(0x24);
    league.set_stage_index(stage_index::MAIN);
    this.set_current_stage((0xffff_ffff) as usize);
    this.set_stage_count((0) as usize);
    this.set_stages((0) as usize);
    call_qualifier_routine(0x920d10, this.addr());
    if call_qualifier_routine(0x921040, this.addr()) == 0 {
        show_error_at(0xad4ce0, 0x65);
        return this.addr();
    }
    if call_qualifier_routine(0x920eb0, this.addr()) == 0 {
        show_error_at(0xad4ce0, 0x6c);
        return this.addr();
    }
    if !qualifying_section_stats(league, 0xad4ce0, 0x73) {
        return this.addr();
    }
    if year as i16 >= league.year() {
        covered!("cqz_4 south american 2002 campaign");
        call_qualifier_routine(0x9216d0, this.addr());
    }
    this.addr()
}

/// Comparer for `construct_wc_south_american_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_south_american_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_SOUTH_AMERICAN_LEAGUE);
    compare_qualifying_ctor("construct_wc_south_american_league", League(this), record, 0x970e2c, qualifying_campaign_year(year as i16).max(0x7d4),
        KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION, || o(this, year, record))
}

/// rule: the South American qualifying is one double round robin (stage −1, id 0, a league): with ten nations, 18
/// rounds from 29 March of the campaign year to 15 November of the next (two windows a year: March to November); with
/// two or more nations exempt (the World Cup hosts and holder), eight nations and 14 rounds; other stages have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`,
/// `build_phase_table`): each operation is a call of the original (league_round_set_date 0x68a160,
/// league_round_set_match_slot 0x68a1c0: pushes; add esp -> cdecl) in the original's order (months from 0); the outputs
/// (`rounds`, the stage id 0 always written, `league` 1 only when not null), the size (0x41 per round) and the error-box
/// line when out of memory (0xb5); every stage −128..127 and both exempt cases checked against the original by
/// emulation.
/// Original: `WcSouthAmericanLeague::create_schedule` (0x920590, wc_south_american_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_south_american_phase(League(this), stage), rounds, extra, league, 0xad4ce0)
}

/// Comparer for `wc_south_american_league_create_schedule`: side by side (`compare_phase_table`).
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_SOUTH_AMERICAN_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_south_american_league_create_schedule", stage, wc_south_american_phase(League(this), stage), rounds, extra, league,
        || o(this, stage, rounds, extra, league), |r, e, l| wc_south_american_league_create_schedule(this, stage, r, e, l))
}

/// rule: the South American nations exempt from qualifying are the hosts of the next World Cup and the last World Cup
/// winner (when not a host); the same count is kept for Oceania (the Oceania section reads it)
///
/// `wc_south_american_league_fields::EXEMPT_COUNT` (+0xee) and `OCEANIA_EXEMPT_COUNT` (+0xef) = 0. Without
/// [`G_HOST_COUNTRY`]: the error box of line 0x1ab and no hosts. Otherwise the hosts of the World Cup two years later
/// (`south_american_world_cup_hosts`); for each of them (k signed byte, below the count) up to the first −1: a
/// South American nation (`KEY_CONTINENT_SOUTH_AMERICA`) adds one to +0xee, else an Oceania one
/// (`KEY_CONTINENT_OCEANIA`) to +0xef (a nation without continent, nothing). Then the last World Cup winner
/// (`last_world_cup_winner`), if any: its nation (`club::NATION`), with a continent, South American and neither host
/// adds one to +0xee; then (the nation read again) with a continent, from Oceania and neither host, one to +0xef. Both
/// host slots start at −1 and stay so without [`G_HOST_COUNTRY`].
/// Original: `WcSouthAmericanLeague::count_exempt_nations` (0x920d10, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_count_exempt_nations(this: usize) {
    let league = League(this);
    let south = this + wc_south_american_league_fields::EXEMPT_COUNT;
    let oceania = this + wc_south_american_league_fields::OCEANIA_EXEMPT_COUNT;
    covered!("cqz_8 exempt nations");
    write_u8(south, 0); // cqz8: no South American nation exempt yet
    write_u8(oceania, 0);
    let mut hosts = [-1i32; 2];
    if read_usize(G_HOST_COUNTRY) == 0 {
        show_error_at(0xad4ce0, 0x1ab);
    } else {
        let n = south_american_world_cup_hosts(league, &mut hosts);
        let mut k = 0i8;
        // simplification: the original reads the host slot k of its two locals for any k below the count; find_hosts gives at most two
        while k < n && (k as usize) < hosts.len() {
            let id = hosts[k as usize];
            if id == -1 {
                break;
            }
            if nation_id_on_continent(id as usize, KEY_CONTINENT_SOUTH_AMERICA) {
                covered!("cqz_8 south american host");
                write_u8(south, read_u8(south).wrapping_add(1)); // cqz8: a South American host is exempt
            } else if nation_id_on_continent(id as usize, KEY_CONTINENT_OCEANIA) {
                write_u8(oceania, read_u8(oceania).wrapping_add(1));
            }
            k += 1;
        }
    }
    let winner = last_world_cup_winner();
    if winner == 0 {
        return;
    }
    let (h0, h1) = (hosts[0], hosts[1]);
    for (continent, count) in [(KEY_CONTINENT_SOUTH_AMERICA, south), (KEY_CONTINENT_OCEANIA, oceania)] {
        let nation = Club(winner).nation();
        if nation.is_null() {
            return;
        }
        let c = nation.continent();
        if c == 0 {
            return;
        }
        let id = read_usize(nation.addr()) as i32;
        if read_usize(c) == read_usize(continent) && id != h0 && id != h1 {
            covered!("cqz_8 world cup holder exempt");
            write_u8(count, read_u8(count).wrapping_add(1));
        }
    }
}

/// Comparer for `wc_south_american_league_count_exempt_nations`: side by side (it only reads and writes the two counts;
/// the error box needs a missing host table), the two counts equal.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_count_exempt_nations(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_SOUTH_AMERICAN_LEAGUE_COUNT_EXEMPT_NATIONS);
    let south = this + wc_south_american_league_fields::EXEMPT_COUNT;
    let (a, b) = (read_u8(south), read_u8(south + 1));
    wc_south_american_league_count_exempt_nations(this);
    let mine = (read_u8(south), read_u8(south + 1));
    write_u8(south, a);
    write_u8(south + 1, b);
    o(this);
    let theirs = (read_u8(south), read_u8(south + 1));
    SCOREBOARD.record(mine == theirs, || format!("wc_south_american_league_count_exempt_nations({:#x}): {:?} expected {:?}", this, mine, theirs));
}

/// rule: the South American qualifying table has ten places minus the exempt nations, filled with the South American
/// national teams in database order, leaving out the hosts of the next World Cup and the last winner
///
/// The hosts (`south_american_world_cup_hosts`; [`G_HOST_COUNTRY`] is not checked here; its count is not read: both
/// slots start at −1) and the last World Cup winner (`last_world_cup_winner`). `league::CLUB_COUNT` = 10 − exempt
/// (`EXEMPT_COUNT`, signed byte) and the table (`exe::crt_malloc` of 0x3b × count: push; add esp -> cdecl) into
/// `league::TABLE`; out of memory, the error box of line 0x204 and 0. Over the nations (`G_NATION_COUNT`, i word
/// signed), each South American one (`KEY_CONTINENT_SOUTH_AMERICA`) while the rows (b, signed byte) are fewer than the
/// count: its national team (`exe::database_get_national_team`: push; add esp, 4 -> cdecl), when the nation is neither
/// host and the team is not the winner, goes into row b (`exe::league_init_table_row`(this, b, team, 0, 0): 4 pushes;
/// mov ecx -> thiscall, ret 0x10; the row index pushed with garbage in the high bytes, read as a byte). Returns 1 when
/// the rows filled the count, else 0.
/// Original: `WcSouthAmericanLeague::build_team_slots` (0x920eb0, wc_south_american_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_south_american_league_build_team_slots(this: usize) -> u32 {
    let league = League(this);
    let mut hosts = [-1i32; 2];
    south_american_world_cup_hosts(league, &mut hosts);
    let winner = last_world_cup_winner();
    let places = 10 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16; // cqz8: ten places minus the exempt
    league.set_club_count(places);
    let table = exe::crt_malloc((places as i32 as u32).wrapping_mul(0x3b));
    league.set_table(table);
    if table == 0 {
        show_error_at(0xad4ce0, 0x204);
        return 0;
    }
    let mut b = 0i8;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let c = nation.continent();
        if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_SOUTH_AMERICA) && (b as i16) < league.club_count() {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != winner {
                covered!("cqz_8 south american slot");
                exe::league_init_table_row(league, b as u8 as u32, team, 0, 0);
                b = b.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    ((b as i16) == league.club_count()) as u32
}

pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_build_team_slots(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_SOUTH_AMERICAN_LEAGUE_BUILD_TEAM_SLOTS);
    let r = o(this);
    let league = League(this);
    let expected = 10 - read_u8(this + wc_south_american_league_fields::EXEMPT_COUNT) as i8 as i16;
    let mut ok = league.club_count() == expected && (r == 0 || r == 1);
    if r == 1 && league.table() != 0 {
        for k in 0..expected.max(0) as usize {
            let team = read_usize(league.table() + k * 0x3b);
            let nation = if team != 0 { Club(team).nation() } else { Nation(0) };
            let c = if nation.is_null() { 0 } else { nation.continent() };
            ok &= c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_SOUTH_AMERICA);
        }
    }
    SCOREBOARD.record(ok, || format!("wc_south_american_league_build_team_slots({:#x}) returned {} with {} places", this, r, league.club_count()));
    r
}

/// The round table of stage `stage` of the South American qualifying: the double round robin of ten nations (18 rounds)
/// or, with two or more exempt (`wc_south_american_league_fields::EXEMPT_COUNT`, signed byte), of eight (14 rounds).
pub(crate) unsafe fn wc_south_american_phase(this: League, stage: u32) -> Option<GoldcupStagePlan> {
    if (read_u8(this.addr() + wc_south_american_league_fields::EXEMPT_COUNT) as i8) >= 2 { // cqz8: eight nations with two exempt
        wc_south_american_eight_phase(stage)
    } else {
        wc_south_american_ten_phase(stage)
    }
}

/// Whether the nation with id `id` (`G_NATION_TABLE` + `nation::SIZE` × id) has a continent whose id is the one of the
/// key `continent`.
pub(crate) unsafe fn nation_id_on_continent(id: usize, continent: usize) -> bool {
    let c = Nation(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE))).continent();
    c != 0 && read_usize(c) == read_usize(continent)
}

/// The World Cup the qualifying leads to: the hosts of the World Cup two years after the campaign year (HostCountry::
/// find_hosts on [`G_HOST_COUNTRY`]: 5 pushes; mov ecx -> thiscall, ret 0x14; the year pushed with garbage in the high
/// 16 bits, read as a word), into `hosts` (−1 when none), and the number found (signed byte).
pub(crate) unsafe fn south_american_world_cup_hosts(this: League, hosts: &mut [i32; 2]) -> i8 {
    let (a, b) = hosts.split_at_mut(1);
    exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (this.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8
}

/// The last World Cup winner (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add
/// esp, 8 -> cdecl), or 0.
pub(crate) unsafe fn last_world_cup_winner() -> usize {
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    exe::comp_util_get_latest_placed_club(world_cup, 0)
}

/// The round table of stage `stage` (signed byte) of cqz_8 south american qualifying round table, or none.
pub(crate) fn wc_south_american_ten_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => None,
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xb5,
            ops: &WC_SOUTH_AMERICAN_TEN_OPS_1, ops2: &[], name: "cqz_8 south american qualifying round table" }),
        0..=127 => None,
    }
}

/// The round table of stage `stage` (signed byte) of cqz_8 south american qualifying round table (eight nations), or none.
pub(crate) fn wc_south_american_eight_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => None,
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 14, extra: 0, extra_check: false, size: 0x38e, row: 0xb5,
            ops: &WC_SOUTH_AMERICAN_EIGHT_OPS_1, ops2: &[], name: "cqz_8 south american qualifying round table (eight nations)" }),
        0..=127 => None,
    }
}

/// cqz_8 south american qualifying round table, the operations of round table 1 (from the original, in order).
pub(crate) const WC_SOUTH_AMERICAN_TEN_OPS_1: [GoldcupScheduleOp; 36] = [
    Cqp12D(0, 0x1d, 2, 0, 2, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x1a, 3, 0, 2, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 4, 5, 0, 6, 1, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0x1c, 5, 0, 2, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x13, 6, 0, 5, 1, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x1a, 6, 0, 5, 1, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 0x10, 7, 0, 2, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 6, 8, 0, 2, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 7, 9, 0, 5, 1, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0xf, 0xa, 0, 2, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0), Cqp12D(0xa, 0x1d, 2, 1, 2, 2, 0),
    Cqp12L(0xa, 0, -1, -1, -1, 0), Cqp12D(0xb, 0x1a, 3, 1, 2, 2, 0), Cqp12L(0xb, 0, -1, -1, -1, 0),
    Cqp12D(0xc, 4, 5, 1, 5, 1, 0), Cqp12L(0xc, 0, -1, -1, -1, 0), Cqp12D(0xd, 0x10, 7, 1, 2, 2, 0),
    Cqp12L(0xd, 0, -1, -1, -1, 0), Cqp12D(0xe, 6, 8, 1, 2, 2, 0), Cqp12L(0xe, 0, -1, -1, -1, 0),
    Cqp12D(0xf, 7, 9, 1, 5, 1, 0), Cqp12L(0xf, 0, -1, -1, -1, 0), Cqp12D(0x10, 8, 0xa, 1, 2, 2, 0),
    Cqp12L(0x10, 0, -1, -1, -1, 0), Cqp12D(0x11, 0xf, 0xa, 1, 2, 2, 0), Cqp12L(0x11, 0, -1, -1, -1, 0),
];

/// cqz_8 south american qualifying round table (eight nations), the operations of round table 1 (from the original, in order).
pub(crate) const WC_SOUTH_AMERICAN_EIGHT_OPS_1: [GoldcupScheduleOp; 28] = [
    Cqp12D(0, 0x1d, 2, 0, 2, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 4, 5, 0, 6, 1, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 0x1c, 5, 0, 2, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0x13, 6, 0, 5, 1, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x10, 7, 0, 2, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 6, 8, 0, 2, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 7, 9, 0, 5, 1, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 0x1d, 2, 1, 2, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 0x1a, 3, 1, 2, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0x10, 7, 1, 2, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0), Cqp12D(0xa, 6, 8, 1, 2, 2, 0),
    Cqp12L(0xa, 0, -1, -1, -1, 0), Cqp12D(0xb, 7, 9, 1, 5, 1, 0), Cqp12L(0xb, 0, -1, -1, -1, 0),
    Cqp12D(0xc, 8, 0xa, 1, 2, 2, 0), Cqp12L(0xc, 0, -1, -1, -1, 0), Cqp12D(0xd, 0xf, 0xa, 1, 2, 2, 0),
    Cqp12L(0xd, 0, -1, -1, -1, 0),
];
