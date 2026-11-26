use super::*;

/// The season turnover of the CONCACAF World Cup qualifying: last campaign's tables, entrant list and stages freed (the
/// stage vector kept), seven stages, the season archived, the next campaign's year, the entrants again, the predicted
/// positions, the seeds, the round table, the tie tree and the two Central American groups.
///
/// `qualifying_cup_free_tables`; `cup::STAGE_COUNT` = 7 and `cup::CURRENT_STAGE` = −1; the season archived
/// (`exe::comp_stats_archive_season_and_reset`, +8); `cup::SEASON_SET_UP` = 1; the year up by four until it reaches the
/// game's (`advance_qualifying_campaign`); `cup::CURRENT_ROUND` = 0 and `cup::RUN_BY_STAGES` = 0. Then, each with its own
/// box (wc_concacaf_cup.cpp) when it returns 0: the entrants (WcConcacafCup 0x916250, thiscall, ret 0; line 0x41f); the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`); the seeds (WcConcacafCup 0x915d40; line
/// 0x429), the round table (virtual `comp_vtable::SET_UP_RULES`; line 0x430) and the tie tree (virtual
/// `cup_vtable::MAP_FIXTURE_TREE`; line 0x437); finally the Central American groups
/// (WcConcacafCup::create_central_american_groups, thiscall, ret 0), whose eax it returns. After a box the copy returns
/// 0 (the original the box's eax).
/// Original: `WcConcacafCup::start_new_season` (0x915ff0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_start_new_season(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    qualifying_cup_free_tables(cup);
    cup.set_stage_count(7);
    cup.set_current_stage(usize::MAX);
    let stats = read_usize(this + 8);
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    cup.set_season_set_up(1);
    advance_qualifying_campaign(this + cup::YEAR);
    covered!("mwg_13 concacaf qualifying rollover");
    cup.set_current_round(0);
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x41f);
        return 0;
    }
    call_cup_virtual(cup, comp_vtable::SET_PREDICTED_POSITIONS);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_SET_UP_SEEDS, this) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x429);
        return 0;
    }
    if call_cup_virtual(cup, comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x430);
        return 0;
    }
    if call_cup_virtual(cup, cup_vtable::MAP_FIXTURE_TREE) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x437);
        return 0;
    }
    call_section_routine(wt::FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS, this) as u32
}

/// Comparer for `wc_concacaf_cup_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_START_NEW_SEASON);
    compare_qualifying_turnover("wc_concacaf_cup_start_new_season", League(this), || o(this))
}

/// A no-argument thiscall routine (ret 0) of a qualifying section; its signed eax.
pub(crate) unsafe fn call_section_routine(routine: usize, this: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(routine);
    f(this)
}

/// The CONCACAF World Cup qualifying at career creation.
///
/// Cup::Cup (`exe::new_cup`), the vtable, the year `year` (word), `cup::CURRENT_ROUND` 0, the main stage, no current
/// stage (−1), 7 stages, the record (`league::COMPETITION`), `cup::CLUB_SIDES` 4, no section above (−1) and the Asian
/// section below (the competition id of [`KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION`]), 7 substitutes named and 3
/// allowed, discipline 0 and importance bias 0xa. If the saved state loads (Cup::load_from_disk `exe::cup_load_from_disk`
/// (this, 1): push 1; mov ecx -> thiscall, ret 4), returns this.
///
/// Otherwise the campaign year (`qualifying_campaign_year`: a year one past the four-year cycle goes back one, then up
/// to the cycle), the stage vector (`exe::crt_malloc` of 4 × 7, cdecl), `cup::RUN_BY_STAGES` 0; the entrants
/// (WcConcacafCup::select_entrants 0x916250; 0: the error box of line 0x5e and this); the statistics (operator new of
/// 0x5ce and Comp_Stats::Comp_Stats into `league::COMP_STATS`; null: the box of line 0x66 and this); the predicted
/// positions (WcConcacafCup::rank_teams 0x917db0) and the knockout team table (WcConcacafCup::build_team_table 0x915d40;
/// 0: the box of line 0x70 and this), all thiscall, ret 0, called directly; the round table of stage −1 (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0), thiscall, ret 0x10) into
/// `cup::ROUND_TABLE`. A campaign that started before the game's year (`cup::YEAR` below `year`) stays dormant
/// (`cup::SEASON_SET_UP` 0, a dword); otherwise the two Central American groups
/// (WcConcacafCup::create_central_american_groups 0x916cd0). Last the tie tree (Cup::map_fixture_tree
/// `exe::cup_map_fixture_tree`; 0: the box of line 0x8a). Returns this. The SEH frame covers only the statistics' new.
/// Original: `WcConcacafCup::WcConcacafCup` (0x9149f0, wc_concacaf_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this.addr());
    exe::new_cup(cup);
    write_usize(this.addr(), WC_CONCACAF_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(stage_index::MAIN);
    cup.set_current_stage(usize::MAX);
    cup.set_stage_count(7);
    write_usize(this.addr() + league::COMPETITION, record);
    cup.set_club_sides(4);
    this.set_upper_division_id((u32::MAX) as i32);
    this.set_lower_division_id((read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)) as usize);
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_discipline_index(0);
    cup.set_importance_bias(0xa);
    if exe::cup_load_from_disk(cup, 1) != 0 {
        return this.addr();
    }
    covered!("cx16_1 concacaf qualifying");
    cup.set_year(qualifying_campaign_year(cup.year())); // cx16_1: the campaign of the four-year cycle
    let n = cup.stage_count() as i32;
    cup.set_stages(if n > 0 { exe::crt_malloc((n as u32) << 2) } else { 0 });
    cup.set_run_by_stages(0);
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_COLLECT_ENTRANTS, this.addr()) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x5e);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let stats = if p != 0 { exe::new_comp_stats(p, cup) } else { 0 };
    this.set_comp_stats(stats);
    if stats == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x66);
        return this.addr();
    }
    call_section_routine(FN_WC_CONCACAF_CUP_RANK_TEAMS, this.addr());
    if call_section_routine(wt::FN_WC_CONCACAF_CUP_SET_UP_SEEDS, this.addr()) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x70);
        return this.addr();
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    cup.set_round_table(create_schedule(this.addr(), u32::MAX, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0));
    if cup.year() < year as i16 {
        cup.set_season_set_up(0); // cx16_1: a campaign begun before the game stays dormant
    } else {
        call_section_routine(wt::FN_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS, this.addr());
    }
    if exe::cup_map_fixture_tree(cup) == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x8a);
    }
    this.addr()
}

/// Comparer for `construct_wc_concacaf_cup`: it allocates, reads the save and builds the stages, so only the original
/// runs; then the plan: this returned, the vtable, the record, the campaign year, the Asian section below, 7 stages and,
/// for a new campaign begun before the game's year, a dormant cup (`cup::SEASON_SET_UP` 0).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_concacaf_cup(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_CONCACAF_CUP);
    let loaded = league_will_load_saved_state(record);
    let r = o(this.addr(), year, record);
    let cup = Cup(this.addr());
    let expected = qualifying_campaign_year(year as i16);
    let dormant = loaded || expected >= year as i16 || cup.season_set_up() == 0;
    let ok = r == this.addr() && read_usize(this.addr()) == WC_CONCACAF_CUP_VTABLE && this.competition().addr() == record
        && built_year_ok(cup.year() as u16, expected as u16) && (this.lower_division_id() as u32) == read_u32(KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION)
        && cup.stage_count() == 7 && dormant;
    SCOREBOARD.record(ok, || format!("construct_wc_concacaf_cup({:#x}, {}, {:#x}): returned {:#x}, year {} expected {}, vtable {:#x}, +0x45 {}", this.addr(), year as i16, record, r, cup.year(), expected, read_usize(this.addr()), cup.season_set_up()));
    r
}

/// rule: the CONCACAF qualifying predicts the places by reputation: the three best (fewer by the exempt hosts and
/// holder) first, then 4, 5 and 6, the next three 7, the next three 10 and the rest 13; every entrant's national team
/// becomes busy with this competition
///
/// Without statistics (`league::COMP_STATS`), the error box of line 0x7cf; a copy of the entrant list (+0xb2, the count
/// +0xb6 dword; `exe::crt_malloc` of 4 × count, cdecl; out of memory, the box of line 0x7d7) sorted by reputation
/// (`predicted_by_reputation_sort`: qsort with 0x4b07d0). The North American national teams are released
/// (`exe::nationalteams_release_competitions_of_continent`([`G_NATIONAL_TEAMS`], the continent of
/// [`KEY_CONTINENT_NORTH_AMERICA`] in [`G_CONTINENT_TABLE`], 0xc6 bytes each, 0): push 0; push; mov ecx -> thiscall, ret
/// 8); the places of `concacaf_predicted_positions`(count, exempt +0xba, signed byte) go in order to the sorted copy
/// (`exe::comp_stats_set_club_final_position`, thiscall, ret 8; the original pushes the places of the second band with
/// stack garbage in the high bytes: the callee stores only the byte); then the first count − exempt of the copy that
/// are there and have a nation become busy with the record (`exe::nationalteams_set_current_competition`, thiscall,
/// ret 8); the copy freed (`exe::crt_free`).
/// Original: `WcConcacafCup::rank_teams` (0x917db0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_rank_teams(this: usize) {
    let this = League(this);
    use wc_cup_turnover_consts as wt;
    let stats = this.comp_stats();
    if stats == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7cf);
    }
    let n = read_u32(this.addr() + league::NEXT_DRAW_DAY); // the entrant count
    let v = exe::crt_malloc(n << 2);
    if v == 0 {
        return show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x7d7);
    }
    core::ptr::copy_nonoverlapping(read_usize(this.addr() + wt::CUP_ENTRANT_LIST) as *const u8, v as *mut u8, (n << 2) as usize);
    predicted_by_reputation_sort(v, n);
    let national_teams = read_usize(G_NATIONAL_TEAMS);
    exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_NORTH_AMERICA).wrapping_mul(0xc6)), 0);
    let exempt = (this.round_dates() as u8) as i8 as i32; // the exempt North Americans (hosts and holder)
    covered!("cx16_1 concacaf predicted positions");
    for (k, p) in concacaf_predicted_positions(n as i32, exempt).into_iter().enumerate() {
        exe::comp_stats_set_club_final_position(stats, read_usize(v + k * 4), p);
    }
    for k in 0..(n as i32 - exempt).max(0) as usize {
        let team = Club(read_usize(v + k * 4));
        if !team.is_null() && !team.nation().is_null() {
            exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), this.competition().addr());
        }
    }
    exe::crt_free(v);
}

/// Comparer for `wc_concacaf_cup_rank_teams`: the original runs, then the copy again on the state it left (the places
/// and the busy marks are written again the same way); each entrant's place and its national team's competition must
/// not change.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_rank_teams(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_CONCACAF_CUP_RANK_TEAMS)(this);
    let a = concacaf_entrant_marks(this);
    if a.is_empty() {
        return;
    }
    wc_concacaf_cup_rank_teams(this);
    let b = concacaf_entrant_marks(this);
    SCOREBOARD.record(a == b, || format!("wc_concacaf_cup_rank_teams({:#x}): original {:x?} rust {:x?}", this, a, b));
}

/// rule: the CONCACAF qualifying knockout holds the first 24 Caribbean entrants, fewer by the exempt Caribbean nations
///
/// `cup::TEAM_COUNT` = 0x18 − the exempt Caribbean count (+0xbb, signed byte; a word); `cup::TEAM_TABLE` = a table of
/// that many 6-byte rows (`exe::crt_malloc`, cdecl; out of memory, the error box of line 0x350 and −1); row i gets the
/// entrant of slot i of the list (+0xb2) and its seed and status bytes (+4, +5) 0. Returns 1.
/// Original: `WcConcacafCup::build_team_table` (0x915d40, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let cup = Cup(this);
    cup.set_team_count((0x18i16).wrapping_sub(read_u8(this + 0xbb) as i8 as i16)); // cx16_1: 24 Caribbean places
    let t = exe::crt_malloc((cup.team_count() as i32 as u32).wrapping_mul(6));
    cup.set_team_table(t);
    if t == 0 {
        show_error_at(wc_cup_turnover_consts::WC_CONCACAF_CUP_CPP, 0x350);
        return -1;
    }
    covered!("cx16_1 concacaf team table");
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    let mut i = 0i16;
    while i < cup.team_count() {
        let row = t + i as usize * 6;
        write_usize(row, read_usize(list + i as usize * 4));
        write_u8(row + 4, 0);
        write_u8(row + 5, 0);
        i += 1;
    }
    1
}

/// Comparer for `wc_concacaf_cup_build_team_table`: the original builds its table; the copy then builds its own, which
/// must have the same count and rows; the copy's table is freed and the original's put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WC_CONCACAF_CUP_BUILD_TEAM_TABLE)(this);
    let cup = Cup(this);
    let (t, n) = (cup.team_table(), cup.team_count());
    if o != 1 || t == 0 || !(0..=64).contains(&n) {
        return o;
    }
    let rows = |t: usize, n: i16| core::slice::from_raw_parts(t as *const u8, n as usize * 6).to_vec();
    let a = rows(t, n);
    let r = wc_concacaf_cup_build_team_table(this);
    let (t2, n2) = (cup.team_table(), cup.team_count());
    let b = if t2 != 0 && n2 == n { rows(t2, n2) } else { Vec::new() };
    if t2 != 0 && t2 != t {
        exe::crt_free(t2);
    }
    cup.set_team_table(t);
    cup.set_team_count(n);
    SCOREBOARD.record(r == o && a == b, || format!("wc_concacaf_cup_build_team_table({:#x}): {} teams, original {:x?} rust {:x?}", this, n, a, b));
    o
}

/// rule: the CONCACAF qualifying has 25 Caribbean places and 10 Central and North American ones, less the World Cup
/// hosts and holder from North America; the 2000 campaign has its fixed 35 national teams; later campaigns take the
/// North American national teams in database order (and those without a continent), each list sorted
///
/// The exempt counts +0xba, +0xbb and +0xbc start at 0. The hosts of the World Cup two years on (HostCountry::find_hosts
/// `exe::hostcountry_find_hosts`([`G_HOST_COUNTRY`], [`KEY_CLUB_COMP_FIFA_WORLD_CUP`], `cup::YEAR` + 2, &h0, &h1, 1): 5
/// pushes; mov ecx -> thiscall, ret 0x14; without [`G_HOST_COUNTRY`], the error box of line 0x467 and no hosts), up to
/// the first −1, and the last World Cup winner (`exe::comp_util_get_latest_placed_club`(its record, 0), cdecl) when its
/// nation is neither host: each North American one is counted (`concacaf_count_exempt`). The list: +0xb6 = 35 and +0xb2
/// = `exe::crt_calloc`(35, 4) (cdecl; null, the box of line 0x4aa and 0).
///
/// Campaign 2000 (`cup::YEAR`): the national teams (`exe::database_get_national_team`, cdecl) of
/// `WC_CONCACAF_2000_ENTRANTS` in order, and 1. Otherwise, over the nations (`G_NATION_COUNT`), those of North America
/// or without a continent whose id is not a host's and whose national team is not the holder: region 6 into the next of
/// the first 25 slots, region 7 into the next of the last 10, until the Caribbean count reaches 25 − (+0xbb) and the other
/// 10 − (+0xbc). Short of either, 0; full, each part sorted (qsort `exe::crt_qsort`(part, count, 4, 0x4b0810), cdecl)
/// and 1.
/// Original: `WcConcacafCup::select_entrants` (0x916250, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_select_entrants(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    write_u8(this + 0xba, 0); // the exempt North Americans
    write_u8(this + 0xbb, 0); // of them, the Caribbean ones
    write_u8(this + 0xbc, 0); // and the others
    let mut hosts = [-1i32; 2];
    let host_country = read_usize(G_HOST_COUNTRY);
    if host_country == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x467);
    } else {
        let (a, b) = hosts.split_at_mut(1);
        let n = exe::hostcountry_find_hosts(host_country, read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), (cup.year() as u16).wrapping_add(2) as u32, &mut a[0], &mut b[0], 1) as i8;
        for k in 0..n.max(0) as usize {
            let h = hosts.get(k).copied().unwrap_or(-1); // ponytail: find_hosts gives at most the two slots
            if h == -1 {
                break;
            }
            concacaf_count_exempt(this, nation_of_id(h));
        }
    }
    let world_cup = read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b));
    let champion = exe::comp_util_get_latest_placed_club(world_cup, 0);
    if champion != 0 && !Club(champion).nation().is_null() {
        let nation = Club(champion).nation();
        let id = read_u32(nation.addr()) as i32;
        if id != hosts[0] && id != hosts[1] {
            concacaf_count_exempt(this, nation);
        }
    }
    write_u32(this + 0xb6, 0x23); // the 35 places
    let list = exe::crt_calloc(0x23, 4);
    write_usize(this + wt::CUP_ENTRANT_LIST, list);
    if list == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x4aa);
        return 0;
    }
    if cup.year() == 0x7d0 {
        covered!("cx16_3 concacaf 2000 entrants");
        for (k, &key) in WC_CONCACAF_2000_ENTRANTS.iter().enumerate() {
            write_usize(list + k * 4, exe::database_get_national_team(nation_of_id(read_u32(key) as i32)));
        }
        return 1;
    }
    let caribbean_places = 0x19 - read_u8(this + 0xbb) as i8 as i32;
    let other_places = 0xa - read_u8(this + 0xbc) as i8 as i32;
    let (mut caribbean, mut others) = (0i32, 0i32);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let nation = nation_of_id(i as i32);
        let c = nation.continent();
        if c == 0 || read_usize(c) == read_usize(KEY_CONTINENT_NORTH_AMERICA) {
            let team = exe::database_get_national_team(nation);
            let id = read_u32(nation.addr()) as i32;
            if id != hosts[0] && id != hosts[1] && team != champion {
                let region = nation.region();
                if region == 6 && caribbean < 0x19 {
                    write_usize(list + caribbean as usize * 4, team);
                    caribbean += 1;
                } else if region == 7 && others < 0xa {
                    write_usize(list + 0x64 + others as usize * 4, team);
                    others += 1;
                }
                if caribbean == caribbean_places && others == other_places {
                    break;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if caribbean != caribbean_places || others != other_places {
        return 0;
    }
    covered!("cx16_3 concacaf entrants");
    exe::crt_qsort(list, caribbean as u32, 4, 0x4b0810);
    exe::crt_qsort(list + 0x64, others as u32, 4, 0x4b0810); // cx16_3: the Central and North American part sorted
    1
}

/// Comparer for `wc_concacaf_cup_select_entrants`: the original makes its list; the copy then makes its own, which must
/// match it with the same result and counts; the copy's list is freed and the original's state put back.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_select_entrants(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_SELECT_ENTRANTS)(this);
    let list = read_usize(this + 0xb2);
    let a = concacaf_entrants_state(this, r);
    let r2 = wc_concacaf_cup_select_entrants(this);
    let b = concacaf_entrants_state(this, r2);
    let list2 = read_usize(this + 0xb2);
    if list2 != 0 && list2 != list {
        exe::crt_free(list2);
    }
    write_usize(this + 0xb2, list);
    write_u8(this + 0xba, a.1[0]);
    write_u8(this + 0xbb, a.1[1]);
    write_u8(this + 0xbc, a.1[2]);
    write_u32(this + 0xb6, a.2);
    SCOREBOARD.record(a == b, || format!("wc_concacaf_cup_select_entrants({:#x}): original {:x?} rust {:x?}", this, a, b));
    r
}

/// rule: the two Central American groups of the CONCACAF qualifying take the seeded slots in turn: the first slots 25,
/// 27 and 29, the second 26, 28 and 30 (three nations each, or two when the exempt nations take places), 3 points a win
/// and 1 a draw
///
/// A team array (`exe::crt_malloc` of 0xc, cdecl; null, the error box of line 0x572). The first group (stage 0): its
/// slots (`concacaf_group_slots` by +0xbc, signed byte) moved into the array (`take_concacaf_slots`) and the group
/// built (`concacaf_group_stage` with index 0) into `cup::STAGES`[0] (null: the box of line 0x591, ending), its round
/// table freed (`exe::crt_free`) and its draw news sent (`exe::league_send_group_draw_news`). The second (stage 1) the
/// same with index 1 into `cup::STAGES`[1] (null: the box of line 0x5b3). Then the team array freed,
/// `cup::CURRENT_STAGE` = 1 and the second group's draw news sent again. Defect of the original: the second group's
/// draw news goes out twice. Returns 1 (the original the eax of the news call, unread by the constructor; the turnover
/// passes it on), 0 after a box.
/// Original: `WcConcacafCup::create_central_american_groups` (0x916cd0, wc_concacaf_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_create_central_american_groups(this: usize) -> u32 {
    use wc_cup_turnover_consts as wt;
    let cup = Cup(this);
    let teams = exe::crt_malloc(0xc);
    if teams == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x572);
        return 0;
    }
    covered!("cx16_3 central american groups");
    let (first, n1, second, n2) = concacaf_group_slots(read_u8(this + 0xbc) as i8);
    take_concacaf_slots(this, &first[..n1], teams);
    let (s, table) = concacaf_group_stage(this, 0, n1 as u32, teams, 0);
    write_usize(cup.stages(), s);
    if s == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x591);
        return 0;
    }
    exe::crt_free(table);
    exe::league_send_group_draw_news(s);
    take_concacaf_slots(this, &second[..n2], teams);
    let (s, table) = concacaf_group_stage(this, 1, n2 as u32, teams, 1);
    write_usize(cup.stages() + 4, s);
    if s == 0 {
        show_error_at(wt::WC_CONCACAF_CUP_CPP, 0x5b3);
        return 0;
    }
    exe::crt_free(table);
    exe::league_send_group_draw_news(s);
    exe::crt_free(teams);
    cup.set_current_stage(1);
    exe::league_send_group_draw_news(s);
    1
}

/// Comparer for `wc_concacaf_cup_create_central_american_groups`: it builds the stages and sends news, so only the
/// original runs; then the plan: the current stage 1, the moved slots empty and each group's table with the entrants
/// of its slots (taken before).
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_create_central_american_groups(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_CONCACAF_CUP_CREATE_CENTRAL_AMERICAN_GROUPS);
    let list = read_usize(this + 0xb2);
    let stages = Cup(this).stages();
    if list == 0 || stages == 0 {
        return o(this);
    }
    let (first, n1, second, n2) = concacaf_group_slots(read_u8(this + 0xbc) as i8);
    let plan = |slots: &[usize]| {
        let mut v: Vec<usize> = slots.iter().map(|&s| read_usize(list + s * 4)).collect();
        v.sort_unstable();
        v
    };
    let (p1, p2) = (plan(&first[..n1]), plan(&second[..n2]));
    let r = o(this);
    let teams_of = |s: usize| {
        let g = League(s);
        let mut v: Vec<usize> = if !g.is_null() && g.table() != 0 { (0..g.club_count().max(0) as usize).map(|k| read_usize(g.table() + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        v.sort_unstable();
        v
    };
    let (g1, g2) = (teams_of(read_usize(stages)), teams_of(read_usize(stages + 4)));
    let empty = first[..n1].iter().chain(second[..n2].iter()).all(|&s| read_usize(list + s * 4) == 0);
    let ok = Cup(this).current_stage() == 1 && empty && g1 == p1 && g2 == p2;
    SCOREBOARD.record(ok, || format!("wc_concacaf_cup_create_central_american_groups({:#x}): groups {:x?} {:x?} plan {:x?} {:x?}, stage {}", this, g1, g2, p1, p2, Cup(this).current_stage() as i32));
    r
}

/// rule: on 10 January of the campaign year, with the full update, every CONCACAF entrant and every team already in a
/// stage becomes busy with the qualifying; then the cup's day
///
/// On that day (`concacaf_busy_day`) with `full`: the North American national teams released
/// (`exe::nationalteams_release_competitions_of_continent`, the continent of [`KEY_CONTINENT_NORTH_AMERICA`]); each
/// non-null entrant of the list (+0xb2, the count +0xb6) busy with the record
/// (`exe::nationalteams_set_current_competition`([`G_NATIONAL_TEAMS`], its nation, `league::COMPETITION`), thiscall,
/// ret 8; the nation not checked); then, when `cup::CURRENT_STAGE` is −1 or more, for each stage index from −1 to the
/// current one: −1 the cup's own team table (`cup::TEAM_TABLE`, 6-byte rows, `cup::TEAM_COUNT`), the others the table of
/// that stage (`league::TABLE`, `league_table_row::SIZE`-byte rows, `league::CLUB_COUNT`), each team's nation busy too.
/// Then Cup::update_for_date (`exe::cup_update_for_date`(this, today, full), thiscall, ret 8), whose eax it returns.
/// Original: `WcConcacafCup::update_for_date` (0x915e40, wc_concacaf_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let cup = Cup(this.addr());
    if concacaf_busy_day(this.addr(), today) && full != 0 { // cx16_3: the busy day
        covered!("cx16_3 concacaf busy day");
        let national_teams = read_usize(G_NATIONAL_TEAMS);
        let record = this.competition().addr();
        exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_NORTH_AMERICA).wrapping_mul(0xc6)), 0);
        let list = read_usize(this.addr() + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
        let mut k = 0i16;
        while (k as i32) < read_u32(this.addr() + league::NEXT_DRAW_DAY) as i32 {
            let team = Club(read_usize(list + k as usize * 4));
            if !team.is_null() {
                exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), record);
            }
            k = k.wrapping_add(1);
        }
        let current = cup.current_stage() as i32;
        let mut s = -1i8;
        while current >= -1 && (s as i32) <= current {
            if s == -1 {
                for i in 0..cup.team_count().max(0) as usize {
                    exe::nationalteams_set_current_competition(national_teams, Club(read_usize(cup.team_table() + i * 6)).nation().addr(), record);
                }
            } else {
                let g = League(read_usize(cup.stages() + s as usize * 4));
                for i in 0..g.club_count().max(0) as usize {
                    exe::nationalteams_set_current_competition(national_teams, Club(read_usize(g.table() + i * league_table_row::SIZE)).nation().addr(), record);
                }
            }
            s = s.wrapping_add(1);
        }
    }
    exe::cup_update_for_date(cup, today, full)
}

/// Comparer for `wc_concacaf_cup_update_for_date`: the cup's day cannot run twice; only the original, and on the busy
/// day with the full update every entrant's national team must be busy with the qualifying.
pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let busy = full != 0 && concacaf_busy_day(this.addr(), today);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_CONCACAF_CUP_UPDATE_FOR_DATE)(this.addr(), today, full);
    if busy {
        let record = this.competition().addr();
        let list = read_usize(this.addr() + 0xb2);
        let n = read_u32(this.addr() + league::NEXT_DRAW_DAY) as i32;
        let free: Vec<usize> = (0..n.clamp(0, 64) as usize)
            .map(|k| Club(read_usize(list + k * 4)))
            .filter(|t| !t.is_null() && !t.nation().is_null() && exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), t.nation().addr()) != record)
            .map(|t| t.addr())
            .collect();
        SCOREBOARD.record(free.is_empty(), || format!("wc_concacaf_cup_update_for_date({:#x}): entrants not busy {:x?}", this.addr(), free));
    }
    r
}

/// The predicted positions of the CONCACAF entrants in reputation order, for `n` entrants of which `exempt` are exempt
/// (+0xba): the first 3 − exempt get 1; then, from that place up to the sixth, the place itself plus one (a byte); the
/// seventh to ninth get 7, the tenth to twelfth 10, and from the thirteenth up to n − exempt, 13.
pub(crate) fn concacaf_predicted_positions(n: i32, exempt: i32) -> Vec<u32> {
    let seeded = 3 - exempt;
    let mut v: Vec<u32> = (0..seeded.max(0)).map(|_| 1).collect();
    let mut j = seeded as i16;
    while j < 6 {
        v.push((j as u8).wrapping_add(1) as u32);
        j = j.wrapping_add(1);
    }
    v.extend((6..12).map(|s| if s < 9 { 7 } else { 10 })); // cx16_1: the middle bands
    v.extend((12..n - exempt).map(|_| 13));
    v
}

/// What the predicted positions leave in the game for each entrant of the list: its place in the statistics and its
/// national team's current competition.
pub(crate) unsafe fn concacaf_entrant_marks(this: usize) -> Vec<(usize, u8, usize)> {
    let this = League(this);
    let (stats, list, n) = (this.comp_stats(), read_usize(this.addr() + 0xb2), read_u32(this.addr() + league::NEXT_DRAW_DAY) as i32);
    if stats == 0 || list == 0 || !(0..=64).contains(&n) {
        return Vec::new();
    }
    (0..n as usize)
        .map(|k| {
            let team = Club(read_usize(list + k * 4));
            if team.is_null() {
                return (0, 0, 0);
            }
            let e = exe::comp_stats_find_club_entry(stats, team);
            let place = if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 };
            let busy = if !team.nation().is_null() { exe::nationalteams_get_current_competition(read_usize(G_NATIONAL_TEAMS), team.nation().addr()) } else { 0 };
            (team.addr(), place, busy)
        })
        .collect()
}

/// The nation record of id `id` (`G_NATION_TABLE`, `nation::SIZE` bytes each).
pub(crate) unsafe fn nation_of_id(id: i32) -> Nation {
    Nation(read_usize(G_NATION_TABLE).wrapping_add((id as isize as usize).wrapping_mul(nation::SIZE)))
}

/// Counts an exempt nation into the CONCACAF section when it is North American: +0xba, and +0xbb when its region byte
/// is 6 (the Caribbean [inference]) or +0xbc otherwise.
pub(crate) unsafe fn concacaf_count_exempt(this: usize, nation: Nation) {
    let c = nation.continent();
    if c == 0 || read_usize(c) != read_usize(KEY_CONTINENT_NORTH_AMERICA) {
        return;
    }
    write_u8(this + 0xba, read_u8(this + 0xba).wrapping_add(1));
    let k = if nation.region() == 6 { 0xbb } else { 0xbc };
    write_u8(this + k, read_u8(this + k).wrapping_add(1));
}

/// What the entrants routine leaves: its result, the three exempt counts, the count and the 35 slots.
pub(crate) unsafe fn concacaf_entrants_state(this: usize, r: u32) -> (u32, [u8; 3], u32, Vec<usize>) {
    let list = read_usize(this + 0xb2);
    let slots = if list != 0 { (0..0x23).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    (r, [read_u8(this + 0xba), read_u8(this + 0xbb), read_u8(this + 0xbc)], read_u32(this + 0xb6), slots)
}

/// One Central American group: its round table (virtual `comp_vtable::CREATE_SCHEDULE`(stage, &rounds, &label, 0):
/// thiscall, ret 0x10) and the League_Stage (`exe::crt_operator_new` of 0xee and League_Stage::League_Stage
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: the section, `n`, the teams, 2, the record, the round table, the
/// rounds, 3 and 1 points, category 0xa, the tie-breaks {1, 2, 3, 0}, the places {1, 1, 0, 0}, the year, `index`, the
/// label, importance 0xf, 4, 0, flags 0x28, −1, 0, 2; the original pushes the rounds and the label with garbage in the
/// high bytes, which the constructor does not read: the copy passes clean words). Returns (stage or 0, round table).
pub(crate) unsafe fn concacaf_group_stage(this: usize, stage: u32, n: u32, teams: usize, index: u32) -> (usize, usize) {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let tiebreaks: [u8; 4] = [1, 2, 3, 0];
    let places: [u8; 4] = [1, 1, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage, &mut rounds, &mut label, 0);
    let p = exe::crt_operator_new(0xee);
    let s = if p != 0 {
        construct(p, this.addr(), n, teams, 2, this.competition().addr(), table, rounds as u16 as u32, 3, 1, 0xa, tiebreaks.as_ptr() as usize, places.as_ptr() as usize,
            this.year() as u16 as u32, index, label as u16 as u32, 0xf, 4, 0, 0x28, -1, 0, 2)
    } else {
        0
    };
    (s, table)
}

/// Moves the entrants of the given slots of the list (+0xb2) into `teams`, leaving the slots empty.
pub(crate) unsafe fn take_concacaf_slots(this: usize, slots: &[usize], teams: usize) {
    let list = read_usize(this + wc_cup_turnover_consts::CUP_ENTRANT_LIST);
    for (k, &s) in slots.iter().enumerate() {
        write_usize(teams + k * 4, read_usize(list + s * 4));
        write_usize(list + s * 4, 0);
    }
}

/// The list slots of the two Central American groups: the first takes slots 25 and 27 (and 29 with fewer than two
/// exempt non-Caribbean nations), the second 26 and 28 (and 30 with none).
pub(crate) fn concacaf_group_slots(others: i8) -> ([usize; 3], usize, [usize; 3], usize) {
    let n1 = 2 + (others < 2) as usize;
    let n2 = 2 + (others < 1) as usize;
    ([25, 27, 29], n1, [26, 28, 30], n2) // cx16_3: the alternate slots of the two groups
}

/// Whether `today` (the date's day and year words) is 10 January of the CONCACAF campaign year (`cup::YEAR`; the
/// date built by Date::Date `exe::date_date_549f70`(&date, 10, 0, year, −1): 4 pushes; lea ecx -> thiscall, ret 0x10).
pub(crate) unsafe fn concacaf_busy_day(this: usize, today: usize) -> bool {
    let mut date = [0u16; 4];
    exe::date_date_549f70(&mut date, 0xa, 0, Cup(this).year() as u16 as u32, u32::MAX);
    read_u16(today + 2) == date[1] && read_u16(today) == date[0]
}

/// The CONCACAF section's vtable (WcConcacafCup).
pub(crate) const WC_CONCACAF_CUP_VTABLE: usize = 0x970c24;

/// WcConcacafCup::rank_teams (thiscall, ret 0), called by the constructor.
pub(crate) const FN_WC_CONCACAF_CUP_RANK_TEAMS: usize = 0x917db0;

/// The 35 national teams of the 2000 CONCACAF campaign (the 2002 World Cup qualifying), by nation key, in the list's
/// order: the 25 Caribbean slots, then the 10 Central and North American ones.
pub(crate) const WC_CONCACAF_2000_ENTRANTS: [usize; 35] = [
    KEY_NATION_CUBA, KEY_NATION_CAYMAN_ISLANDS, KEY_NATION_SAINT_LUCIA, KEY_NATION_SURINAM, KEY_NATION_ARUBA, KEY_NATION_PUERTO_RICO,
    KEY_NATION_BARBADOS, KEY_NATION_GRENADA, KEY_NATION_ST_VINCENT_THE_GRENADINES, KEY_NATION_US_VIRGIN_ISLANDS, KEY_NATION_ST_KITTS_NEVIS,
    KEY_NATION_TURKS_AND_CAICOS_ISLANDS, KEY_NATION_GUYANA, KEY_NATION_ANTIGUA_BARBUDA, KEY_NATION_BRITISH_VIRGIN_ISLANDS, KEY_NATION_BERMUDA,
    KEY_NATION_TRINIDAD_TOBAGO, KEY_NATION_NETHERLANDS_ANTILLES, KEY_NATION_DOMINICAN_REPUBLIC, KEY_NATION_MONTSERRAT, KEY_NATION_DOMINICA,
    KEY_NATION_HAITI, KEY_NATION_ANGUILLA, KEY_NATION_THE_BAHAMAS, KEY_NATION_JAMAICA, KEY_NATION_HONDURAS, KEY_NATION_EL_SALVADOR,
    KEY_NATION_GUATEMALA, KEY_NATION_PANAMA, KEY_NATION_BELIZE, KEY_NATION_NICARAGUA, KEY_NATION_CANADA, KEY_NATION_COSTA_RICA,
    KEY_NATION_MEXICO, KEY_NATION_UNITED_STATES,
];
