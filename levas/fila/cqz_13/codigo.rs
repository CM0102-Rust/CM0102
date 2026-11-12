// ---------- cqz_13: the European World Cup qualifying: the entry pool and the day (wc_europe_league.cpp) ----------

/// The European qualifying's exempt count (+0xf6, byte: the European hosts and holder) and its group format: the last
/// group-stage index, the runners-up stage and the play-off stage (+0xff, +0x100, +0x101), read by the round table.
const WC_EUROPE_EXEMPT: usize = 0xf6;
const WC_EUROPE_LAST_GROUP: usize = 0xff;

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

// simplification: the pool is allocated and kept: it cannot run twice. Only the original; the plan: return value 0, or 1 with
// the count at least the places, every team of the list from Europe and the format of the exempt count.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_build_entry_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
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
    let league = League(this);
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    covered!("cqz_13 european day");
    let (day, year) = oceania_nations_draw_date(league);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 && full != 0 {
        covered!("cqz_13 european draw");
        let draw: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x918ec0usize);
        let own_group: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x9190d0usize);
        let group: unsafe extern "thiscall" fn(usize, i32, usize, u32) -> u32 = core::mem::transmute(0x9196d0usize);
        let runners_up: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x91a140usize);
        draw(this);
        if own_group(this) == 0 {
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
        let groups: i32 = match read_u8(this + WC_EUROPE_EXEMPT) {
            0 => 0xa,
            1 | 2 => 9,
            _ => 8,
        };
        for g in 0..groups - 1 {
            group(this, g, 0, 6);
        }
        exe::crt_free(read_usize(this + 0xf7));
        write_u32(this + 0xf7, 0);
        runners_up(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        exe::crt_free(read_usize(this + 0xee));
        write_u32(this + 0xee, 0);
        write_u32(this + 0xf2, 0);
        league.set_active(1);
    } else if read_u16(G_CURRENT_DATE + 2) == read_u16(this + league::LAST_RESULT_DATE + 2) && read_u16(G_CURRENT_DATE) == read_u16(this + league::LAST_RESULT_DATE) {
        let fill: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x91a240usize);
        fill(this);
    }
    league_day(this, today, full) as u32
}

// simplification: the draw, the stages, the runners-up table and the league day cannot run twice. Only the original; the plan on
// the draw day (the copy's condition): `league::ACTIVE` = 1 and the pool, its count and the list +0xf7 zeroed.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
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
