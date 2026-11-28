use super::*;

/// rule: in the Oceania qualifier groups, a team going through gets the qualification (weight 20) and one going out the
/// failed-to-qualify record and ends its campaign; in the Oceania final, the winner gets the qualification (weight 30)
/// and the loser the knock-out; in the play-off (against South America [inference]), the winner gets the World Cup
/// place (weight 100 record, the entry in its list and status 4) and the loser the knock-out (weight 100) and the end
/// of the campaign
/// Original: `WcOceaniaLeague::update_staff_records_for_status` (0x91e840, wc_oceania_league.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, out: usize, game: usize, _a6: u32) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    let eliminar = |weight: u32| {
        // simplification: +0x30 and +0x32 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
        european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
        clear_national_team_competition(Club(club));
    };
    if f < 1 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x14);
            clear_national_team_competition(Club(club));
        }
    } else if f == 1 {
        if d == 1 {
            european_cup_add_qualified_record(club, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            covered!("cqj_10 oceania in the world cup");
            let id = read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP);
            european_cup_add_qualified_record(club, read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            exe::worldcup_add_qualified_team(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(id.wrapping_mul(4))), club);
            write_u8(out, 4);
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

/// rule: the Oceania qualifier groups are single round robin, 3/1 points, tiebreak 1, 2, 3, one qualifier per group (to
/// the Oceania final), category 10, 7 substitutes and 3 changes, starting on 15 of month 0 on weekday 5
/// Original: `WcOceaniaLeague::set_up_rules` (0x91ecf0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_up_rules(this: usize) -> u32 {
    covered!("cqk_10 oceania rules");
    goldcup_write_rule_fields(this, &OCEANIA_RULE_FIELDS, 3);
    let day = 0xf; // cqk10: the start day in Oceania
    let word = competition_date_word(League(this), day, 0, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// The season turnover of the Oceania World Cup qualifying: last campaign's table, round dates, national-team pool and
/// stages freed, the season archived, the next campaign's year, the pool and the predicted positions again, three new
/// stages, the rules and the group table.
///
/// `qualifying_turnover_free_table`; the national-team pool (`ENTRANT_LIST`, +0xef) freed when there (not zeroed); the
/// stages destroyed (`destroy_comp_stages`) and the stage vector (`.stages()`) freed (not zeroed); the current stage −1;
/// the season archived (`exe::comp_stats_archive_season_and_reset`(`.comp_stats()`, 1)); the year up by four until it
/// reaches the game's (`advance_qualifying_campaign`); `.set_active(0)` and the current stage −1 again; the pool
/// (WcOceaniaLeague::build_entrant_pool, thiscall, ret 0; its return not read), the predicted positions (virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`), 3 stages (`.set_stage_count(3)`) in a new vector (`exe::crt_malloc(0xc)`,
/// cdecl); the rules (virtual `comp_vtable::SET_UP_RULES`; 0: the box of line 0x305) and the group table
/// (WcOceaniaLeague::init_group_table, thiscall, ret 0; 0: the box of line 0x30c), whose eax it returns. After a box
/// the copy returns 0 (the original the box's eax).
/// Original: `WcOceaniaLeague::start_new_season` (0x91ef40, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_start_new_season(this: usize) -> u32 {
    use wc_turnover_consts as wt;
    let this = League(this);
    qualifying_turnover_free_table(this);
    let pool = read_usize(this.addr() + wt::ENTRANT_LIST);
    if pool != 0 {
        exe::crt_free(pool);
    }
    destroy_comp_stages(this);
    exe::crt_free(this.stages());
    this.set_current_stage(usize::MAX);
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    advance_qualifying_campaign(this.addr() + league::YEAR);
    covered!("mwg_12 oceania qualifying rollover");
    this.set_active(0);
    this.set_current_stage(usize::MAX);
    call_qualifier_routine(wt::FN_WC_OCEANIA_LEAGUE_BUILD_ENTRANT_POOL, this.addr());
    call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS), this.addr());
    this.set_stage_count(3);
    this.set_stages(exe::crt_malloc(0xc) as usize);
    if call_qualifier_routine(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES), this.addr()) == 0 {
        show_error_at(wt::WC_OCEANIA_LEAGUE_CPP, 0x305);
        return 0;
    }
    let r = call_qualifier_routine(wt::FN_WC_OCEANIA_LEAGUE_INIT_GROUP_TABLE, this.addr());
    if r == 0 {
        show_error_at(wt::WC_OCEANIA_LEAGUE_CPP, 0x30c);
        return 0;
    }
    r
}

/// Comparer for `wc_oceania_league_start_new_season` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_START_NEW_SEASON);
    compare_qualifying_turnover("wc_oceania_league_start_new_season", League(this), || o(this))
}

/// rule: the Oceania World Cup qualifying moves on stage by stage: from the groups to the tie of the group winners (stage
/// 1), and from it to the inter-continental play-off (stage 2) only once the South American qualifying is complete
/// (the play-off opponent comes from it); past the last stage it is an error
///
/// On stage 1 (`league::CURRENT_STAGE` = 1): the South American section (the competition object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]) must be complete (its
/// `comp_vtable::CHECK_COMPLETE` with 1, 1: push 1; push 1; mov ecx -> thiscall, ret 8; al 0 returns). At or past the
/// last stage (`stage_count` − 1, signed): the error box of line 0x329. Otherwise the stage goes up one: to 1, the tie of
/// the group winners (WcOceaniaLeague::create_final_stage 0x91f640, thiscall, ret 0); to 2, the play-off
/// (WcOceaniaLeague::create_playoff_stage 0x91f800, thiscall, ret 0).
/// Original: `WcOceaniaLeague::advance_stage` (0x91f0d0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_advance_stage(this: usize) {
    let this = League(this);
    let league = this;
    if (this.current_stage() as u32) == 1 {
        let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
        let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(south) + comp_vtable::CHECK_COMPLETE));
        if complete(south, 1, 1) == 0 { // cqz9: the play-off waits for the South American qualifying
            return;
        }
    }
    let stage = (this.current_stage() as u32) as i32;
    if stage >= (league.stage_count() as i32).wrapping_sub(1) {
        show_error_at(0xad4afc, 0x329);
        return;
    }
    let stage = stage + 1;
    this.set_current_stage((stage as u32) as usize);
    covered!("cqz_9 oceania qualifying next stage");
    match stage {
        1 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_WCOCEANIALEAGUE_CREATE_FINAL_STAGE);
            f(this.addr());
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_WCOCEANIALEAGUE_CREATE_PLAYOFF_STAGE);
            f(this.addr());
        }
        _ => {}
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_advance_stage(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_ADVANCE_STAGE);
    let before = (this.current_stage() as u32) as i32;
    o(this.addr());
    let after = (this.current_stage() as u32) as i32;
    SCOREBOARD.record(after == before || after == before + 1, || format!("wc_oceania_league_advance_stage({:#x}): stage {} -> {}", this.addr(), before, after));
}

/// The Oceania World Cup qualifying at career creation.
///
/// `start_qualifying_section` (vtable 0x970d78; above, the Asian section; below, the South American section), the
/// campaign year (`qualifying_campaign_year`, no minimum), inactive and importance 0xa. If the saved state loads,
/// returns this.
///
/// Otherwise: `league_flags` 0x28, the entrant pool (WcOceaniaLeague::build_entrant_pool 0x91fa80, thiscall, ret 0;
/// return value not read), 3 stages, then the statistics (null: the error box of line 0x63 and returns this); the
/// predicted positions (WcOceaniaLeague::set_predicted_positions 0x91ff30, thiscall, ret 0), the main stage,
/// `current_stage` −1 and the stage vector (`exe::crt_malloc(4 × stage_count)` when the count is positive, else null);
/// the rules (WcOceaniaLeague::set_up_rules 0x91ecf0; 0: the box of line 0x75) and the group table
/// (WcOceaniaLeague::init_group_table 0x91ec00; 0: the box of line 0x7c). Returns this.
/// Original: `WcOceaniaLeague::WcOceaniaLeague` (0x91df90, wc_oceania_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let this = League(this);
    let league = this;
    let edition = qualifying_campaign_year(year as i16); // cqz4: the Oceania campaign
    start_qualifying_section(league, 0x970d78, record, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, edition);
    this.set_active((0) as usize);
    let importance = 0xa; // cqz4: the importance of the Oceania qualifying matches
    league.set_importance(importance);
    if exe::league_load_from_temp_file(league, 1) != 0 {
        return this.addr();
    }
    covered!("cqz_4 oceania qualifying");
    league.set_league_flags(0x28);
    call_qualifier_routine(0x91fa80, this.addr());
    this.set_stage_count((3) as usize);
    if !qualifying_section_stats(league, 0xad4afc, 0x63) {
        return this.addr();
    }
    call_qualifier_routine(0x91ff30, this.addr());
    league.set_stage_index(stage_index::MAIN);
    this.set_current_stage((0xffff_ffff) as usize);
    let n = (this.stage_count() as u32) as i32;
    this.set_stages((if n > 0 { exe::crt_malloc((n as u32) << 2) as u32 } else { 0 }) as usize);
    if call_qualifier_routine(0x91ecf0, this.addr()) == 0 {
        show_error_at(0xad4afc, 0x75);
        return this.addr();
    }
    if call_qualifier_routine(0x91ec00, this.addr()) == 0 {
        show_error_at(0xad4afc, 0x7c);
    }
    this.addr()
}

/// Comparer for `construct_wc_oceania_league` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wc_oceania_league(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WC_OCEANIA_LEAGUE);
    compare_qualifying_ctor("construct_wc_oceania_league", League(this), record, 0x970d78, qualifying_campaign_year(year as i16),
        KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT, || o(this, year, record))
}

/// rule: the Oceania World Cup qualifying has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of five rounds on 7,
/// 9, 11, 14 and 16 April; the tie of the group winners (stage 1: 20 June) and the inter-continental play-off (stage
/// 2: 2 December); stages from 3 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcOceaniaLeague::create_schedule` (0x91e2e0, wc_oceania_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_oceania_league_phase(stage), rounds, extra, league, 0xad4afc)
}

/// Comparer for `wc_oceania_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WC_OCEANIA_LEAGUE_CREATE_SCHEDULE);
    compare_phase_table("wc_oceania_league_create_schedule", stage, wc_oceania_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_oceania_league_create_schedule(this, stage, r, e, l))
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_BUILD_ENTRANT_POOL);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_CREATE_FINAL_STAGE);
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

/// The predicted positions of the Oceania World Cup qualifier: its national teams (+0xef, +0xf3 dword) by strength (the
/// reputation `club::REPUTATION` of the national teams' clubs, `predicted_by_reputation_of_copy`): 1, 2, 3, 3, 5, 5, 7,
/// 7, 9 and, if there are 10, the tenth 10 (boxes at lines 0x52a and 0x532).
/// Original: `WcOceaniaLeague::set_predicted_positions` (0x91ff30, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_predicted_positions(this: usize) {
    let n = read_usize(this + 0xf3) as i32;
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_of_copy(this, read_usize(this + 0xef), n, &WC_OCEANIA_LEAGUE_POSITIONS[..k], 0xad4afc, (0x52a, 0x532))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_SET_PREDICTED_POSITIONS);
    let (list, n) = (read_usize(this + 0xef), read_usize(this + 0xf3) as i32);
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_check("wc_oceania_league_set_predicted_positions", this, &clubs, &WC_OCEANIA_LEAGUE_POSITIONS[..k]);
}

/// The Oceania qualifying day (`comp_vtable::UPDATE_FOR_DATE`; the original's SEH frame is only for the local dates,
/// with no destructor): `WcOceaniaLeague::update_for_date(today, full)`. Before the start (today's year lower,
/// unsigned; or the same year and the day lower, signed) it returns 0 without the league day. On the start day or on
/// any day of 2001 (a game starting in 2001 sets up the qualifying already under way), with `league::ACTIVE` = 0 and
/// WITH `full`: the Oceania national teams are released from the continent's competitions (0x775070([`G_NATIONAL_TEAMS`],
/// the continent [`G_CONTINENT_TABLE`] + 0xc6 × [`KEY_CONTINENT_OCEANIA`], 0): push 0; push; mov ecx -> thiscall, ret 8);
/// each national team of the first group's table (`league::TABLE`, `league_table_row::SIZE`-byte rows;
/// `league::CLUB_COUNT` rows) and each of the +0xf3 of the list +0xef (the non-null ones) enters the qualifying
/// (`exe::nationalteams_set_current_competition`([`G_NATIONAL_TEAMS`], its `club::NATION`, `league::COMPETITION`),
/// thiscall, ret 8); the fixtures (`exe::league_create_fixtures`; 0: the error box of line 0x3a4 and 0), the sorted
/// table (`exe::league_sort_table`(this, 0); 0: the error box of line 0x3ab and 0), the second group stage
/// (WcOceaniaLeague::create_second_group_stage 0x91f1a0, thiscall, ret 0) and `league::ACTIVE` = 1. Then the league
/// day (League::update_for_date `FN_LEAGUE_UPDATE_FOR_DATE`, thiscall(this, today, full), ret 8), whose return value it
/// passes on.
/// Original: `WcOceaniaLeague::update_for_date` (0x91f3d0, wc_oceania_league.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    // the full eax is passed on, so the u8 wrapper `exe::league_update_for_date` is not used
    let (day, year) = wc_oceania_league_start_date(this.addr());
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // ponytail: on the early returns before the start and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    let set_up_day = (today_year == year && today_day == day) || today_year == 0x7d1; // cqu12: the 2001 game sets it up at once
    if set_up_day && this.active() == 0 && full != 0 {
        covered!("cqu_12 qualifying set up");
        let national_teams = read_usize(G_NATIONAL_TEAMS);
        exe::nationalteams_release_competitions_of_continent(national_teams, read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_OCEANIA).wrapping_mul(0xc6)), 0);
        let mut i = 0i16;
        while i < this.club_count() {
            let team = Club(read_usize(this.table() + i as usize * league_table_row::SIZE));
            exe::nationalteams_set_current_competition(national_teams, team.nation().addr(), this.competition().addr());
            i += 1;
        }
        let mut k = 0i16;
        while (k as i32) < read_usize(this.addr() + 0xf3) as i32 {
            let e = Club(read_usize(read_usize(this.addr() + 0xef) + k as usize * 4));
            if !e.is_null() {
                exe::nationalteams_set_current_competition(national_teams, e.nation().addr(), this.competition().addr());
            }
            k = k.wrapping_add(1);
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0xad4afc, 0x3a4);
            return 0;
        }
        if exe::league_sort_table(this, 0usize) == 0 {
            show_error_at(0xad4afc, 0x3ab);
            return 0;
        }
        exe::wcoceanialeague_create_second_group_stage(this);
        this.set_active((1) as usize);
    }
    exe::league_update_for_date(this, today, full) as u32 // the callers read only al
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_WC_OCEANIA_LEAGUE_UPDATE_FOR_DATE);
    let sets_up = this.active() == 0 && full != 0 && {
        let (day, year) = wc_oceania_league_start_date(this.addr());
        let (td, ty) = (read_u16(today), read_u16(today + 2));
        !(ty < year || (ty == year && (td as i16) < day as i16)) && ((ty == year && td == day) || ty == 0x7d1)
    };
    let r = o(this.addr(), today, full);
    if sets_up {
        let stages = this.stages();
        let ok = this.active() == 1 && stages != 0 && read_usize(stages) != 0 && this.current_stage() == 0;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_update_for_date({:#x}): the set-up did not build the stage (+0x45 {}, +0x30 {})", this.addr(), this.active(), this.current_stage() as i32));
    }
    r
}

/// The first Oceania qualifying group has 5 national teams (`league::CLUB_COUNT`), the table `league::TABLE`
/// (`exe::crt_malloc`(0x127), cdecl: 5 rows of `league_table_row::SIZE` bytes; out of memory, the error box of line
/// 0x24b and 0), each row i with the national team of the even places of the list +0xef (2i)
/// (`exe::league_init_table_row`(this, i, team, 0, 0), thiscall, ret 0x10), the group draw news
/// (`exe::league_send_group_draw_news`) and returns 1.
/// Original: `WcOceaniaLeague::init_group_table` (0x91ec00, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_init_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(5); // cqu12: 5 national teams in the first group
    let t = exe::crt_malloc(0x127);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xad4afc, 0x24b);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqu_12 first group row");
        // ponytail: the index goes in a dword whose 3 high bytes are stack garbage; the routine reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, read_usize(read_usize(this.addr() + 0xef) + i as isize as usize * 8), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_init_group_table(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_INIT_GROUP_TABLE);
    let list = read_usize(this.addr() + 0xef);
    if list == 0 {
        return o(this.addr());
    }
    let expected: [usize; 5] = core::array::from_fn(|i| read_usize(list + i * 8));
    let r = o(this.addr());
    let t = this.table();
    let mut read = [(0usize, 0u8); 5];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && this.club_count() == 5 && t != 0 && (0..5).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("wc_oceania_league_init_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this.addr(), r, read, expected));
    r
}

/// The second Oceania qualifying group has 4 national teams, or 5 when `wc_oceania_league_fields::EXEMPT_COUNT` is
/// not 1; the teams in an array (`exe::crt_malloc`(4 × n), cdecl; out of memory, the error box of line 0x355 and 0)
/// with the national teams of the odd places of the list +0xef (2i + 1); the round table through
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee), cdecl; the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall,
/// ret 0x58, 22 arguments: the qualifying, n, the array, 1, `league::COMPETITION`, the round table, the rounds (word),
/// `league::WIN_POINTS`, `league::DRAW_POINTS`, `league::MATCH_CATEGORY`, &`league::TIEBREAKS`, a local 4-byte array
/// {1, 0, 0, 0} (the places [inference]), `league::YEAR` (word), 0, the id (word), `league::IMPORTANCE`, 4, 0,
/// `league::LEAGUE_FLAGS` (word), −1, 0, 2; the original pushes the words and bytes with garbage in the high bytes,
/// which the constructor does not read: the copy passes clean values) in [`league::STAGES`][0]; the array and the
/// round table freed (`exe::crt_free`). Without the stage: the error box of line 0x36b and 0. With it:
/// `league::CURRENT_STAGE` = 0, the group news (`exe::league_send_group_draw_news` on the stage) and 1. (The original's
/// SEH frame is only for the stage's new: no destructor on the normal path.)
/// Original: `WcOceaniaLeague::create_second_group_stage` (0x91f1a0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let places: [u8; 4] = [1, 0, 0, 0];
    let n = 4 + (read_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let teams = exe::crt_malloc((n * 4) as u32);
    if teams == 0 {
        show_error_at(0xad4afc, 0x355);
        return 0;
    }
    for i in 0..n {
        covered!("cqu_12 second group team");
        write_usize(teams + i * 4, read_usize(read_usize(this.addr() + 0xef) + i * 8 + 4)); // cqu12: the national team in place i of the second group
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this.addr(), 0, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        let b = |d: usize| read_u8(this.addr() + d) as u32;
        construct_stage(
            p,
            this.addr(),
            n as u32,
            teams,
            1,
            this.competition().addr(),
            table,
            rounds as u16 as u32,
            b(league::WIN_POINTS),
            b(league::DRAW_POINTS),
            b(league::MATCH_CATEGORY),
            this.addr() + league::TIEBREAKS,
            places.as_ptr() as usize,
            this.year() as u16 as u32,
            0,
            id as u16 as u32,
            b(league::IMPORTANCE),
            4,
            0,
            this.league_flags() as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages());
    if f == 0 {
        show_error_at(0xad4afc, 0x36b);
        return 0;
    }
    this.set_current_stage((0) as usize);
    exe::league_send_group_draw_news(f);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_second_group_stage(this: usize) -> u32 {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WC_OCEANIA_LEAGUE_CREATE_SECOND_GROUP_STAGE);
    let list = read_usize(this.addr() + 0xef);
    if list == 0 {
        return o(this.addr());
    }
    let n = 4 + (read_u8(this.addr() + wc_oceania_league_fields::EXEMPT_COUNT) != 1) as usize;
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + i * 8 + 4)).collect();
    let r = o(this.addr());
    if r == 1 {
        let f = League(read_usize(this.stages()));
        let t = if !f.is_null() { f.table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..n).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        let ok = !f.is_null() && this.current_stage() == 0 && f.club_count() as usize == n && f.year() == this.year() && a == plan;
        SCOREBOARD.record(ok, || format!("wc_oceania_league_create_second_group_stage: the stage does not match the plan ({:x?} read {:x?})", plan, a));
    }
    r
}

/// Calls a routine of the qualifying section by its address (thiscall, ret 0; through the hook when it is in Rust).
pub(crate) unsafe fn call_qualifier_routine(routine: usize, this: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(routine);
    f(this)
}

/// The round table of stage `stage` (signed byte) of cqz_7 oceania qualifying round table, or none.
pub(crate) fn wc_oceania_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e8, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10b,
            ops: &WC_OCEANIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        2 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x134,
            ops: &WC_OCEANIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        3..=127 => None,
    }
}

/// Whether the nation with id `id` belongs to Oceania: its record (`G_NATION_TABLE` + `nation::SIZE` × id) has a
/// continent (`nation::CONTINENT`) whose id is the Oceania key's (`KEY_CONTINENT_OCEANIA`).
pub(crate) unsafe fn nation_id_is_oceanian(id: usize) -> bool {
    let c = Nation(read_usize(G_NATION_TABLE).wrapping_add(id.wrapping_mul(nation::SIZE))).continent();
    c != 0 && read_usize(c) == read_usize(KEY_CONTINENT_OCEANIA)
}

/// The qualifying's start date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the
/// day `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
pub(crate) unsafe fn wc_oceania_league_start_date(this: usize) -> (u16, u16) {
    let this = League(this);
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which the routine also writes
    let mut date = [0u32; 2];
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    exe::new_date(date.as_mut_ptr() as usize, this.next_draw_day() as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// cqz_7 oceania qualifying round table, the operations of round table 1 (from the original, in order).
pub(crate) const WC_OCEANIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 6, 1, 3, 0x100), Cqp12L(0, 1, 6, 2, 3, 0x100),
    Cqp12L(1, 0, 2, 1, 3, 0x100), Cqp12L(1, 1, 2, 2, 3, 0x100), Cqp12L(2, 0, 5, 1, 3, 0x100),
    Cqp12L(2, 1, 5, 2, 3, 0x100), Cqp12L(3, 0, 0, 1, 3, 0x100), Cqp12L(3, 1, 0, 2, 3, 0x100),
    Cqp12L(4, 0, 2, 1, 3, 0x100), Cqp12L(4, 1, 2, 2, 3, 0x100),
];

/// cqz_7 oceania qualifying round table, the operations of round table 2 (from the original, in order).
pub(crate) const WC_OCEANIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 0x80),
    Cqp12L(1, 0, 1, 1, 3, 0x80), Cqp12L(1, 1, 1, 0, 3, 0x80), Cqp12L(2, 0, 4, 1, 3, 0x80),
    Cqp12L(2, 1, 4, 2, 3, 0x80), Cqp12L(3, 0, 6, 1, 3, 0x80), Cqp12L(3, 1, 6, 2, 3, 0x80),
    Cqp12L(4, 0, 1, 1, 3, 0x80), Cqp12L(4, 1, 1, 2, 3, 0x80),
];

/// cqz_7 oceania qualifying round table, the operations of round table 3 (from the original, in order).
pub(crate) const WC_OCEANIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1c, 3, 1, 3), Cqp12J(0, 0x14, 5, 1, 2, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];

/// cqz_7 oceania qualifying round table, the operations of round table 4 (from the original, in order).
pub(crate) const WC_OCEANIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x17, 0xa, 1, 3), Cqp12J(0, 2, 0xb, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x0),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x7), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];

/// The predicted positions of the Oceania World Cup qualifier, by strength (9 or 10 nations).
pub(crate) const WC_OCEANIA_LEAGUE_POSITIONS: [u32; 10] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 10];

/// rule: the Oceania qualifying group progress of a nation: the best 1 for the top two, then 5, 7, 9 and 11; the
/// current 1, 3 and 8 for the first three; later stages keep their codes
///
/// The codes of `wc_oceania_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcOceaniaLeague::set_club_progress` (0x9200e0, wc_oceania_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let this = League(this);
    covered!("cx16_7 oceania progress");
    let e = exe::comp_stats_find_club_entry(this.comp_stats(), club);
    let (c, b, w) = wc_oceania_progress_codes(stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_WC_OCEANIA_LEAGUE_SET_CLUB_PROGRESS);
    compare_progress_codes("wc_oceania_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_oceania_league_set_club_progress(this, club, stage, current, best, worst))
}

/// rule: in the Oceania and CONCACAF qualifying a nation's status changes up (1) when its progress code (stat 0x12)
/// leaves 1 (stat 0x13, the previous code, was 1), down (−1) when it becomes 1 from another code, otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 when before was not 1 (else 0); otherwise 1 when
/// before was 1 (else 0). The byte in al. Shared by the CONCACAF and Oceania vtables (+0x74).
/// Original: `WcOceaniaLeague::get_qualification_status_change` (0x920260, wc_oceania_league.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("cx16_8 oceania status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -((before != 1) as i8) // cx16_8: became 1
    } else {
        (before == 1) as i8
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(IDX_WC_OCEANIA_LEAGUE_GET_QUALIFICATION_STATUS_CHANGE)(this, club);
    let b = wc_oceania_league_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

/// rule: the competition the World Cup qualifying sections feed is the FIFA World Cup
///
/// The record of [`KEY_CLUB_COMP_FIFA_WORLD_CUP`] in [`G_CLUB_COMP_TABLE`] (0x6b bytes each). Shared by the CONCACAF,
/// European and Oceania vtables (+0x78); `this` is not read.
/// Original: `WcOceaniaLeague::get_finals_comp` (0x91e1c0, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_get_finals_comp(_this: usize) -> usize {
    covered!("cx16_9 finals comp");
    read_usize(G_CLUB_COMP_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP).wrapping_mul(0x6b)) // cx16_9: the World Cup record
}

/// Comparers: side by side on the result (and, for the reset, the byte it writes); the group size only off the error
/// path (a second box).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_get_finals_comp(this: usize) -> usize {
    let a = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_WC_OCEANIA_LEAGUE_GET_FINALS_COMP)(this);
    let b = wc_oceania_league_get_finals_comp(this);
    SCOREBOARD.record(a == b, || format!("wc_oceania_league_get_finals_comp({:#x}) original={:#x} rust={:#x}", this, a, b));
    a
}

/// rule: the Oceania qualifying fate news: in the groups (stage below 1) a nation that goes through (fate 6) "have
/// qualified for the semi finals of" the competition (or, in short, "qualify for semi finals") and one that goes out
/// (−2) the shared group sentence; in the final (stage 1) the winner "qualify for playoff" and the loser "knocked out of
/// World Cup qualifying"; in the inter-continental play-off (code 0xa0) "qualify for World Cup" or "knocked out in World
/// Cup playoff"
///
/// `this.build_fate_news_text(club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text)` (virtual +0x48; the
/// club in [`G_CLUB_TABLE`] by index, the fate byte, the stage byte (signed), the stage code (a word), four values
/// passed on, `full_sentence` and the Text).
///
/// Stage below 1: fate 6, with `full_sentence`, "have qualified for the semi finals of the" competition (its long name
/// `competition::NAME` with the short gender byte), the club's link and the competition's (its short name); without,
/// "qualify for semi finals" with the club's link. Fate 0xfe: the shared group sentence
/// (`exe::comp_util_write_group_qualify_news`(club_index, code, the competition record, −2, full_sentence, text)). Any
/// other fate, nothing.
///
/// Stage 1, or any other stage with code 0xa0: with `full_sentence`, the shared tie sentence
/// (`exe::comp_util_write_cup_tie_result_news`(club_index, a5, a8, the fate dword, a6, text)); otherwise fate 1 "qualify
/// for playoff" (stage 1) or "qualify for World Cup" (play-off), any other fate "knocked out of World Cup qualifying" or
/// "knocked out in World Cup playoff", with the club's link. Another stage with another code: nothing.
/// Original: `WcOceaniaLeague::build_fate_news_text` (0x91e990, wc_oceania_league.cpp, thiscall, ret 0x28).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    use wc_oceania_fate_texts as t;
    let _ = a7;
    let comp = League(this).competition().addr();
    let club = club_of_index(club_index);
    let stage = stage as u8 as i8;
    // simplification: on the paths without text the original leaves al with a byte of the arguments; the callers do not read it
    if stage < 1 {
        match fate as u8 {
            6 => {
                covered!("rx12_17 oceania group qualifier");
                if full_sentence != 0 {
                    qualifying_fate_line(text, t::QUALIFIED_FOR_SEMI_FINALS_OF, club, Some((comp + competition::SHORT_GENDER_NAME, comp + competition::NAME)));
                    link_fate_club(text, club);
                    link_fate_competition(text, comp); // rx12_17: the competition's link after the club's
                } else {
                    qualifying_fate_line(text, t::QUALIFY_FOR_SEMI_FINALS, club, None);
                    link_fate_club(text, club);
                }
            }
            0xfe => exe::comp_util_write_group_qualify_news(club_index, code, comp, -2, full_sentence, text),
            _ => {}
        }
        return;
    }
    let final_tie = stage == 1;
    if !final_tie && code as u16 != 0xa0 {
        return;
    }
    covered!("rx12_17 oceania tie fate");
    if full_sentence != 0 {
        return exe::comp_util_write_cup_tie_result_news(club_index, a5, a8, fate, a6, text);
    }
    let through = fate as u8 == 1;
    let template = match (final_tie, through) {
        (true, true) => t::QUALIFY_FOR_PLAYOFF,
        (true, false) => t::KNOCKED_OUT_OF_QUALIFYING,
        (false, true) => t::QUALIFY_FOR_WORLD_CUP, // rx12_17: the play-off winner
        (false, false) => t::KNOCKED_OUT_IN_PLAYOFF,
    };
    qualifying_fate_line(text, template, club, None);
    link_fate_club(text, club);
}

/// Comparer for `wc_oceania_league_build_fate_news_text`: side by side into two Texts (`compare_text_writer`).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_build_fate_news_text(this: usize, club_index: u32, fate: u32, stage: u32, code: u32, a5: u32, a6: u32, a7: u32, a8: u32, full_sentence: u32, text: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, usize)>(IDX_WC_OCEANIA_LEAGUE_BUILD_FATE_NEWS_TEXT);
    compare_text_writer(text, || o(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, text),
        |t| wc_oceania_league_build_fate_news_text(this, club_index, fate, stage, code, a5, a6, a7, a8, full_sentence, t),
        || format!("wc_oceania_league_build_fate_news_text(club {}, fate {}, stage {}, code {:#x}, full {})", club_index, fate as u8, stage as u8 as i8, code as u16, full_sentence));
}

/// The inter-continental play-off of the Oceania qualifying: the Oceania final's winner against the South American
/// section's play-off nation (`oceania_playoff_nations`, the section the object of
/// `KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT` in [`G_COMP_OBJECT_TABLE`]), over two legs.
///
/// The list of 8 bytes (`exe::crt_malloc`; out of memory, the box of line 0x403); without both nations, the box of line
/// 0x41f and the list freed. Otherwise: the round table of the current stage (virtual `comp_vtable::CREATE_SCHEDULE`(the
/// current stage byte, &rounds, &extra, 0); the original pushes the byte with stack address bytes above it, which the
/// schedule does not read), the stage (`exe::crt_operator_new`(0xb2) and Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`(
/// the qualifying, 2, the list, the rounds, the competition record, the table, the year, the current stage byte, 4, the
/// extra, 0x14, 0, 0, 0, 0), ret 0x3c; words zero-extended) in `stages()`[current stage]; the list and the table freed
/// (`exe::crt_free`); without a stage, the box of line 0x431; with it, its first round set up (`exe::cup_set_up_round`(
/// stage, 0)).
/// Original: `WcOceaniaLeague::create_playoff_stage` (0x91f800, wc_oceania_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_playoff_stage(this: usize) {
    let this = League(this);
    let south = League(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4))));
    let v = exe::crt_malloc(8);
    if v == 0 {
        return show_error_at(0xad4afc, 0x403);
    }
    let nations = oceania_playoff_nations(this, south);
    for (k, &n) in nations.iter().enumerate() {
        write_usize(v + 4 * k, n); // rx12_17: the play-off list
    }
    if nations.len() != 2 { // rx12_17: both nations are needed
        show_error_at(0xad4afc, 0x41f);
        exe::crt_free(v);
        return;
    }
    covered!("rx12_17 oceania play-off");
    let stage_byte = this.current_stage() as u8 as u32;
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage_byte, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, v, rounds as u16 as u32, this.competition().addr(), table, this.year() as u16 as u32, stage_byte, 4, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    let slot = this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4);
    write_usize(slot, stage);
    exe::crt_free(v);
    exe::crt_free(table);
    let stage = read_usize(this.stages() + (this.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return show_error_at(0xad4afc, 0x431);
    }
    exe::cup_set_up_round(stage, 0);
}

/// Comparer for `wc_oceania_league_create_playoff_stage`: the stage is allocated and its round set up, so only the
/// original runs; then the stage in `stages()`[current stage] must hold the planned two nations
/// (`oceania_playoff_nations`, read before) in its team table, in that order.
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_WC_OCEANIA_LEAGUE_CREATE_PLAYOFF_STAGE);
    let league = League(this);
    let south = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT).wrapping_mul(4)));
    if league.stages() == 0 || read_usize(league.stages() + 4) == 0 || south == 0 || League(south).table() == 0 {
        return o(this);
    }
    let plan = oceania_playoff_nations(league, League(south));
    o(this);
    if plan.len() != 2 {
        return;
    }
    let stage = read_usize(league.stages() + (league.current_stage() as u32 as usize).wrapping_mul(4));
    if stage == 0 {
        return;
    }
    let t = Cup(stage).team_table();
    let read = if t != 0 { vec![read_usize(t), read_usize(t + 6)] } else { vec![0, 0] };
    SCOREBOARD.record(read == plan, || format!("wc_oceania_league_create_playoff_stage({:#x}): {:x?} plan {:x?}", this, read, plan));
}

/// The Oceania progress codes (current, best, worst) at `stage` (signed byte): from stage 1 on, as they came. In the
/// groups (below 1): the best 1, 2 → 1, 3 → 5, 4 → 7, 5 → 9, 6 → 11; the worst 1, 2 → 3, 3 → 5; the current 1 → 1, 2 → 3,
/// 3 → 8. Defects of the original [inference]: a worst of 4, 5 or 6 and a current of 4, 5 or 6 are not mapped; the
/// best becomes 7, 9 or 11 instead (the worst's case first, then the current's, the last one written winning).
pub(crate) fn wc_oceania_progress_codes(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage >= 1 {
        return (current, best, worst);
    }
    let mut b = match best {
        1 | 2 => 1,
        3 => 5,
        4 => 7,
        5 => 9,
        6 => 0xb,
        _ => best,
    };
    let mut w = worst;
    match worst {
        1 | 2 => w = 3,
        3 => w = 5,
        4 => b = 7, // the original writes the best
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    let mut c = current;
    match current {
        1 => c = 1,
        2 => c = 3, // cx16_7: the Oceania group runner-up
        3 => c = 8,
        4 => b = 7,
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    (c, b, w)
}

/// The two nations of the Oceania play-off: the Oceania final's winner (the first entry of the final stage's team table,
/// `stages()`[1], with outcome 1 at +5 of its 6-byte entry) and the South American section's play-off nation (the
/// first row of its table with `league_table_row::STATUS` 1), in that order; either may be missing.
pub(crate) unsafe fn oceania_playoff_nations(this: League, south: League) -> Vec<usize> {
    let mut v = Vec::new();
    let final_stage = Cup(read_usize(this.stages() + 4));
    let teams = final_stage.team_table();
    if let Some(k) = (0..final_stage.team_count().max(0) as usize).find(|&k| read_u8(teams + 6 * k + 5) == 1) {
        v.push(read_usize(teams + 6 * k));
    }
    let rows = south.table();
    if let Some(k) = (0..south.club_count().max(0) as usize).find(|&k| read_u8(rows + k * league_table_row::SIZE + league_table_row::STATUS) == 1) {
        v.push(read_usize(rows + k * league_table_row::SIZE));
    }
    v
}
