// ---------- cqz_12: the day of the South American and Asian World Cup qualifying and the Asian entrant list (wc_south_american_league.cpp, wc_asia_league.cpp) ----------

/// The day of a World Cup qualifying section, before its draw: (true, al 0) when today is before the draw date (today's
/// year lower, unsigned; or the same year and the day lower, signed), and whether today is the draw day.
unsafe fn qualifying_day_before_draw(this: League, today: usize) -> (bool, bool) {
    let (day, year) = oceania_nations_draw_date(this);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    let before = today_year < year || (today_year == year && (today_day as i16) < day as i16);
    (before, today_year == year && today_day == day)
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
    let league = League(this);
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
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
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        let release: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775070usize);
        let continent = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_SOUTH_AMERICA).wrapping_mul(0xc6));
        release(read_usize(G_NATIONAL_TEAMS), continent, 0); // cqz12: the South American teams leave their continental competitions
        let mut k = 0i16;
        while k < read_i16(this + league::CLUB_COUNT) {
            let club = Club(read_usize(read_usize(this + league::TABLE) + (k as isize * 0x3b) as usize));
            exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation(), read_usize(this + league::COMPETITION));
            k = k.wrapping_add(1);
        }
    }
    league_day(this, today, full) as u32
}

// simplification: the fixtures, the national teams' competitions and the league day cannot run twice. Only the original; the plan
// on the start day (the copy's condition): `league::ACTIVE` = 1.
pub(crate) unsafe extern "thiscall" fn compare_wc_south_american_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let league = League(this);
    let starts = full != 0 && league.active() == 0 && qualifying_day_before_draw(league, today).1;
    let r = o(this, today, full);
    if starts {
        SCOREBOARD.record(league.active() == 1, || format!("wc_south_american_league_update_for_date({:#x}): the start did not activate it", this));
    }
    r
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
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    covered!("cqz_12 asian day");
    let (before, draw_day) = qualifying_day_before_draw(league, today);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if before {
        return 0;
    }
    if draw_day && league.active() == 0 && full != 0 {
        covered!("cqz_12 asian draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x912bd0usize);
        let first_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x912ad0usize);
        let group: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x913540usize);
        draw(this);
        if first_group(this) == 0 {
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
            group(this, k);
        }
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xf7));
        write_u32(this + 0xf7, 0);
        league.set_active(1);
    }
    league_day(this, today, full) as u32
}

// simplification: the draw, the tables and the league day cannot run twice. Only the original; the plan on the draw day (the
// copy's condition): `league::ACTIVE` = 1, the list +0xf7 zeroed and the first group table built.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
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

// simplification: the list is allocated and kept: it cannot run twice. Only the original; the plan: return value 1 with the count
// at most 40 and every team of the list from Asia, or 0.
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_build_entrant_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
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
