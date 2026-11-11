// ---------- batch %%LEVA%%: the Oceania World Cup qualifying: the ten nations and the final between the group winners; the second group of the Oceania Nations Cup (wc_oceania_league.cpp, oceania_nations.cpp) ----------

/// Whether the nation with id `id` belongs to Oceania: its record (`G_NATION_TABLE` + `nation::SIZE` × id) has a
/// continent (`nation::CONTINENT`) whose id is the Oceania key's (`KEY_CONTINENT_OCEANIA`).
unsafe fn nation_id_is_oceanian(id: usize) -> bool {
    let c = Nation(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE))).continent();
    c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_OCEANIA)
}

/// The Oceania World Cup qualifying entrants: ten places minus the exempt nations, filled with the Oceania national
/// teams in database order and sorted by reputation.
///
/// `WcOceaniaLeague::build_entrant_pool()`. +0xf3 (the pool count, dword) = 10, `wc_oceania_league_fields::EXEMPT_COUNT`
/// = 0, `games_per_club` = −1. The hosts of the World Cup two years later (HostCountry::find_hosts
/// `exe::hostcountry_find_hosts`([`G_HOST_COUNTRY`], [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], `year()` + 2, &host0, &host1, 1):
/// 5 pushes; mov ecx -> thiscall, ret 0x14; without [`G_HOST_COUNTRY`], the error box of line 0x452 and no hosts): each
/// host from Oceania (`nation_id_is_oceanian`) is exempt (EXEMPT_COUNT + 1), up to the first −1. The last World Cup winner
/// (`exe::comp_util_get_latest_placed_club`(World Cup competition record, 0): 2 pushes; add esp, 8 -> cdecl), when its
/// nation is from Oceania and is not one of the hosts, is exempt too.
///
/// +0xf3 = 10 − exempt, and the pool list +0xef (`exe::crt_malloc` of 4 × n: push; add esp, 4 -> cdecl; out of memory, the
/// error box of line 0x481 and 0). Over the nations (`G_NATION_COUNT`), the Oceania ones except New Caledonia
/// (`KEY_NATION_NEW_CALEDONIA`) and Papua New Guinea (`KEY_NATION_PAPUA_NEW_GUINEA`), while there are places: the main
/// national team (`exe::database_get_national_team`: push; add esp, 4 -> cdecl) that is neither a host nor the winner.
/// Short of filling the places it returns 0 (unsorted); full, the list sorted by reputation (qsort `exe::crt_qsort`(list,
/// n, 4, 0x4b07d0): 4 pushes -> cdecl) and 1.
/// Original: `WcOceaniaLeague::build_entrant_pool` (0x91fa80, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_build_entrant_pool(this: usize) -> u32 {
    let this = League(this);
    let mut hosts = [-1i32; 2];
    write_u32(this.addr() + 0xf3, 10);
    write_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT, 0);
    this.set_games_per_club(-1);
    let host_country = read_usize(G_HOST_COUNTRY);
    if host_country == 0 {
        show_error_at(0xad4afc, 0x452);
    } else {
        let year = (this.year() as u16).wrapping_add(2);
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(host_country, read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), year as u32, &mut a[0], &mut b[0], 1) as i8;
        let mut k = 0i8;
        while k < n {
            let s = hosts[k as usize];
            if s == -1 {
                break;
            }
            if nation_id_is_oceanian(s as usize) {
                let p = this.addr() + wc_oceania_league_fields::EXEMPT_COUNT;
                write_u8(p, read_u8(p).wrapping_add(1));
            }
            k += 1;
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let champion = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if champion != 0 {
        let nation = Club(champion).nation();
        if !nation.is_null() {
            let c = nation.continent();
            let id = read_usize(nation.addr()) as i32;
            if c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_OCEANIA) && id != hosts[0] && id != hosts[1] {
                let p = this.addr() + wc_oceania_league_fields::EXEMPT_COUNT;
                write_u8(p, read_u8(p).wrapping_add(1));
            }
        }
    }
    let n = 10 - read_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT) as i8 as i32;
    write_u32(this.addr() + 0xf3, n as u32);
    let list = exe::crt_malloc((n as u32).wrapping_mul(4));
    write_u32(this.addr() + 0xef, list as u32);
    if list == 0 {
        show_error_at(0xad4afc, 0x481);
        return 0;
    }
    let mut k = 0i32;
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = Nation(read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE)));
        let continent = nation.continent();
        let id = read_usize(nation.addr()) as i32;
        let excluded = id == read_usize(KEY_NATION_NEW_CALEDONIA) as i32 || id == read_usize(KEY_NATION_PAPUA_NEW_GUINEA) as i32; // cqw11: without New Caledonia and Papua New Guinea
        if continent != 0 && read_usize(continent) == read_usize(KEY_CONTINENT_OCEANIA) && !excluded && k < read_usize(this.addr() + 0xf3) as i32 {
            let team = exe::database_get_national_team(nation);
            let id = read_usize(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != champion {
                covered!("cqw_11 qualifying nation");
                write_u32(read_usize(this.addr() + 0xef) + k as usize * 4, team as u32);
                k += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    if k != read_usize(this.addr() + 0xf3) as i32 {
        return 0;
    }
    exe::crt_qsort(read_usize(this.addr() + 0xef), k as u32, 4, 0x4b07d0); // cqw11: the ten by reputation
    1
}

/// Comparer for `wc_oceania_league_build_entrant_pool`. The list is allocated, so it cannot run twice: only the
/// original runs, then the plan is checked: with 1, +0xf3 = 10 − EXEMPT_COUNT and the list holds Oceania national
/// teams (without New Caledonia and Papua New Guinea), with no repeats, by decreasing reputation.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_build_entrant_pool(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let n = read_usize(this + 0xf3) as i32;
        let list = read_usize(this + 0xef);
        let v: Vec<usize> = (0..n.max(0) as usize).map(|k| read_usize(list + k * 4)).collect();
        let mut u = v.clone();
        u.sort_unstable();
        u.dedup();
        let ok = n == 10 - read_u8(this + wc_oceania_league_fields::EXEMPT_COUNT) as i8 as i32
            && u.len() == v.len()
            && v.iter().all(|&c| {
                let id = read_usize(Club(c).nation().addr());
                nation_id_is_oceanian(id) && id != read_usize(KEY_NATION_NEW_CALEDONIA) && id != read_usize(KEY_NATION_PAPUA_NEW_GUINEA)
            })
            && v.windows(2).all(|w| Club(w[0]).reputation() >= Club(w[1]).reputation());
        SCOREBOARD.record(ok, || format!("wc_oceania_league_build_entrant_pool({:#x}): {} nations {:x?}", this, n, v));
    }
    r
}

/// The Oceania qualifying final between the winners of the two groups, the first group's winner first.
///
/// `WcOceaniaLeague::create_final_stage()`. A list of 2 (`exe::crt_malloc` of 8; out of memory, the error box of line
/// 0x3ce and return) with the top club of the qualifying table (the first group; League::get_table_row
/// `exe::league_get_table_row(this, 0)`: push; mov ecx -> thiscall, ret 4) and that of stage 0 (the second group); the
/// round table of stage 1 (virtual `comp_vtable::CREATE_SCHEDULE`: push 0; push &extra; push &rounds; push 1; mov ecx
/// -> thiscall, ret 0x10); the stage (operator new `exe::crt_operator_new` of 0xb2 and Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, ret 0x3c: the qualifying, 2, the list, the rounds, the competition record, the table, the
/// year, 1, 4, the extra, 0x14, 0, 0, 0, 0; words zero-extended) in `stages()`[1]; the list and the table freed
/// (`exe::crt_free`); without a stage, the error box of line 0x3e3; with it, its first round set up (Cup::set_up_round
/// `exe::cup_set_up_round(stage, 0)`: push 0; mov ecx -> thiscall, ret 4). The original's SEH frame covers only the
/// stage's `new`.
/// Original: `WcOceaniaLeague::create_final_stage` (0x91f640, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_final_stage(this: usize) {
    let this = League(this);
    let v = exe::crt_malloc(8);
    if v == 0 {
        return show_error_at(0xad4afc, 0x3ce);
    }
    covered!("cqw_11 oceania final");
    write_u32(v, read_u32(exe::league_get_table_row(this, 0))); // cqw11: the first group winner is listed first
    write_u32(v + 4, read_u32(exe::league_get_table_row(read_usize(this.stages()), 0)));
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this.addr(), 1, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, v, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, 1, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_u32(this.stages() + 4, stage as u32);
    exe::crt_free(v);
    exe::crt_free(table);
    let stage = read_usize(this.stages() + 4);
    if stage == 0 {
        return show_error_at(0xad4afc, 0x3e3);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_oceania_league_create_final_stage`. The stage is allocated and its round set up, so it cannot run
/// twice: only the original runs, then the plan is checked: the stage in `stages()`[1] has the two group winners (the
/// first group's and stage 0's) in its team table (`cup::TEAM_TABLE`, 6-byte rows), in that order.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let league = League(this);
    let stages = league.stages();
    if stages == 0 || read_usize(stages) == 0 || league.table() == 0 || League(read_usize(stages)).table() == 0 {
        return o(this);
    }
    let plan = [read_usize(league.table()), read_usize(League(read_usize(stages)).table())];
    o(this);
    let stage = read_usize(league.stages() + 4);
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read = if t != 0 { [read_usize(t), read_usize(t + 6)] } else { [0, 0] };
    SCOREBOARD.record(read == plan, || format!("wc_oceania_league_create_final_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

/// The second group of the Oceania Nations Cup: a league stage with the nations of places 3, 4 and 5 of the list.
///
/// `OceaniaNations::create_group_stage(stage)`. The nations 3, 4 and 5 of the list at +0xee in a vector
/// (`exe::crt_malloc` of 0xc; out of memory, the error box of line 0x331 and 0); the round table of `stage` (virtual
/// `comp_vtable::CREATE_SCHEDULE`); the stage (operator new `exe::crt_operator_new` of 0xee and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, ret 0x58: the cup, 3, the vector, 1, the competition
/// record, the table, the rounds, `win_points`, `draw_points`, `match_category`, &`league::TIEBREAKS`,
/// &`league::PROMOTED_COUNT`, the year, the stage, the id, `importance`, 4, 0, `league_flags`, −1, 0, 2; words and bytes
/// zero-extended) in `stages()`[stage]; the vector and the table freed (`exe::crt_free`); without a stage, the error box
/// of line 0x347 and 0; with it, `current_stage` = stage, the group draw news (League::send_group_draw_news
/// `exe::league_send_group_draw_news` on the stage) and 1. The original's SEH frame covers only the stage's `new`.
/// Original: `OceaniaNations::create_group_stage` (0x799a60, oceania_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let this = League(this);
    let teams = exe::crt_malloc(0xc);
    if teams == 0 {
        show_error_at(0xa6a0ec, 0x331);
        return 0;
    }
    for i in 0..3usize {
        write_u32(teams + i * 4, read_u32(read_usize(this.addr() + 0xee) + 0xc + i * 4));
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let group = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construct(p, this.addr(), 3, teams, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32, this.match_category() as u32,
                  this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, stage, id as u16 as u32, this.importance() as u32, 4, 0,
                  this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_u32(this.stages() + stage as usize * 4, group as u32);
    exe::crt_free(teams);
    exe::crt_free(table);
    let group = read_usize(this.stages() + stage as usize * 4);
    if group == 0 {
        show_error_at(0xa6a0ec, 0x347);
        return 0;
    }
    write_u32(this.addr() + league::CURRENT_STAGE, stage);
    exe::league_send_group_draw_news(group);
    1
}

/// Comparer for `oceania_nations_create_group_stage`. The stage is allocated and the news goes to the managers, so it
/// cannot run twice: only the original runs, then the plan is checked: it returns 1, the stage is in
/// `stages()`[stage], `current_stage` = stage and the nations 3, 4 and 5 of the list are in its table (in any order).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    let league = League(this);
    let list = read_usize(this + 0xee);
    if list == 0 || stage > 8 {
        return o(this, stage);
    }
    let mut plan: Vec<usize> = (3..6).map(|k| read_usize(list + k * 4)).collect();
    let r = o(this, stage);
    if r == 1 {
        let group = read_usize(league.stages() + stage as usize * 4);
        let t = if group != 0 { League(group).table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..3).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        SCOREBOARD.record(group != 0 && league.current_stage() == stage as usize && a == plan, || format!("oceania_nations_create_group_stage: {:x?} plan {:x?}", a, plan));
    }
    r
}
