// ---------- batch 310: the Danish and German lower divisions' season turnover (den_first.cpp, den_second.cpp, ger_second.cpp) ----------

/// Rule: the Danish First Division turnover (called by the Superliga's): the season reopened (`league::SEASON_CLOSED`
/// = 0), rebuilt for the next year with the stage fields kept (`rebuild_turkish_league`: the table, the round dates and
/// the statistics; the year + 1; the rules, DenFirst::create_table at 0x552290, the fixtures and the order, each with
/// the box of den_first.cpp at lines 0x1a3, 0x1aa, 0x1b0, 0x1b7), then the predicted positions, the board expectations
/// and the officials (`finish_croatian_turnover`), whose eax is returned. After a box the copy returns 0 (the original
/// returns the box's eax; the caller does not read it).
/// Original: `DenFirst::start_new_season` (0x552450, den_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwc_18 danish first division turnover");
    if !rebuild_turkish_league(this, 0x552290, 0x9b8e30, [0x1a3, 0x1aa, 0x1b0, 0x1b7]) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: the Danish Second Division turnover (called by the Superliga's): the season reopened, the stages of the last
/// season destroyed and the league rebuilt for the next year (`rebuild_season`: the table, the round dates, the
/// statistics, the stages 0..=current by their virtual destructor with 1, the year + 1, the current stage −1; the
/// rules, DenSecond::create_table at 0x554a40, the fixtures and the order, each with the box of den_second.cpp at lines
/// 0x1af, 0x1b6, 0x1bc, 0x1c3; the predicted positions), then the board expectations and the officials
/// (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `DenSecond::start_new_season` (0x554c00, den_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x554a40, 0x9b8ea4, [0x1af, 0x1b6, 0x1bc, 0x1c3], 1) {
        return 0;
    }
    covered!("mwc_18 danish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Rule: the German 2. Bundesliga turnover (called by the Bundesliga's): the same template as the Danish Second
/// Division (`rebuild_season` with GerSecond::create_table at 0x5df2b0 and the boxes of ger_second.cpp at lines 0x20a,
/// 0x211, 0x217, 0x21e; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `GerSecond::start_new_season` (0x5df490, ger_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5df2b0, 0x9cd4c4, [0x20a, 0x211, 0x217, 0x21e], 1) {
        return 0;
    }
    covered!("mwc_18 german second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `den_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_den_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("den_first_start_new_season", League(this), || o(this))
}

/// Comparer for `den_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_den_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("den_second_start_new_season", League(this), || o(this))
}

/// Comparer for `ger_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_ger_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("ger_second_start_new_season", League(this), || o(this))
}

// ---------- batch 310: the Dutch and Belgian lower divisions' season turnover (hol_first.cpp, bel_second.cpp, bel_third.cpp) ----------

/// Rule: the Dutch Eerste Divisie turnover (called by the Eredivisie's): the season reopened, last season's stages
/// destroyed and the league rebuilt for the next year (`rebuild_season` with HolFirst::create_table at 0x5f6240 and the
/// boxes of hol_first.cpp at lines 0x1ff, 0x206, 0x20c, 0x213; the predicted positions), then the board expectations and
/// the officials (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `HolFirst::start_new_season` (0x5f63f0, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x5f6240, 0x9cee08, [0x1ff, 0x206, 0x20c, 0x213], 1) {
        return 0;
    }
    covered!("mwc_19 dutch eerste divisie turnover");
    set_expectations_and_renew_officials(this)
}

/// Rule: the Belgian Second Division turnover (called by the First Division's): the same template as the Eerste
/// Divisie (`rebuild_season` with BelSecond::create_table at 0x41fd70 and the boxes of bel_second.cpp at lines 0x20d,
/// 0x214, 0x21a, 0x221; then `set_expectations_and_renew_officials`). After a box: 0.
/// Original: `BelSecond::start_new_season` (0x41ff50, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x41fd70, 0x989b54, [0x20d, 0x214, 0x21a, 0x221], 1) {
        return 0;
    }
    covered!("mwc_19 belgian second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Rule: the Belgian Third Division turnover (called by the First Division's, after the Third Division was regrouped):
/// the season reopened, last season's stages destroyed, the next year; the rules, the table, the order and the
/// fixtures rebuilt; the group B stage built again; the predicted positions and the officials (no board expectations).
///
/// `league::SEASON_CLOSED` = 0; League::free_table (`exe::league_free_table`(0)); the round dates freed and cleared;
/// with statistics, `exe::comp_stats_archive_season_and_reset`(1); with a current stage ≥ 0, the stages 0..=current
/// (re-read on each step) destroyed by their virtual 0 with 1; the year + 1 and the current stage −1. Then, each with
/// the box of bel_third.cpp (0x989d04) at its line and 0 returned when it returns 0, in this order (the order and the
/// fixtures swapped against the other leagues): the rules (virtual `comp_vtable::SET_UP_RULES`, 0x29e),
/// BelThird::fill_league_table (0x4224e0, thiscall, ret 0; 0x2a5), League::sort_table (`exe::league_sort_table`(0),
/// 0x2ac) and League::create_fixtures (0x2b2). Then BelThird::create_group_b_stage (0x422970, thiscall, ret 0), the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`) and OfficialsManager::renew_officials on
/// `G_OFFICIALS_MANAGER` with the record, whose eax is returned.
/// Original: `BelThird::start_new_season` (0x4227b0, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    this.set_year(this.year().wrapping_add(1));
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    let file = 0x989d04;
    if bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(file, 0x29e);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4224e0usize);
    if table(this.addr()) == 0 {
        show_error_at(file, 0x2a5);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(file, 0x2ac);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(file, 0x2b2);
        return 0;
    }
    covered!("mwc_19 belgian third division turnover");
    let group_b: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x422970usize);
    group_b(this.addr());
    bra_nat_first_call_virtual_no_args(this.addr(), comp_vtable::SET_PREDICTED_POSITIONS);
    exe::officialsmanager_renew_officials(read_usize(G_OFFICIALS_MANAGER), this.competition().addr()) as u32
}

/// Comparer for `hol_first_start_new_season`: only the original runs (it frees and rebuilds the league); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("hol_first_start_new_season", League(this), || o(this))
}

/// Comparer for `bel_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("bel_second_start_new_season", League(this), || o(this))
}

/// Comparer for `bel_third_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("bel_third_start_new_season", League(this), || o(this))
}

// ---------- batch 310: the Danish and Dutch top-division season turnovers (den_prm.cpp, hol_prm.cpp) ----------

/// Rule: the Danish turnover, led by the Superliga: the season reopened; promotion and relegation along the chain; the
/// four best Danish clubs of the lowest division (the best twelve shuffled) up into the Second Division and its
/// relegated clubs down (`den_prm_refresh_second_division_clubs`); the division tier of the clubs capped (the
/// Superliga and the First Division with (1, 3, −3, 1), the Second Division with (2, −3, 3, 0)); the Superliga rebuilt
/// for the next year (rules, table, fixtures, order); the First and Second Divisions turned over; the predicted
/// positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1),
/// `den_prm_refresh_second_division_clubs` (direct, ecx = this) and League::limit_club_division_tier
/// (`exe::league_limit_club_division_tier`: 4 pushes -> thiscall, ret 0x10) on this, the First Division and the Second
/// Division (the objects of `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` and `KEY_CLUB_COMP_DANISH_SECOND_DIVISION`); the table
/// DenPrm::create_table (0x5538d0) and the boxes of den_prm.cpp (0x9b8e6c) at 0x21b, 0x222, 0x228, 0x22f; after, the
/// First and Second Divisions' start_new_season (virtual `comp_vtable::START_NEW_SEASON`, no null check). After a box: 0.
/// Original: `DenPrm::start_new_season` (0x553aa0, den_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x553cb0, this);
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DANISH_FIRST_DIVISION), 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), 2, -3i32 as u32, 3, 0);
    }, 0x5538d0, 0x9b8e6c, [0x21b, 0x222, 0x228, 0x22f], || {
        covered!("mwc_20 danish superliga turnover");
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DANISH_FIRST_DIVISION), comp_vtable::START_NEW_SEASON);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: at the Danish turnover, the four best Danish clubs of the lowest division (`KEY_CLUB_COMP_A_LOWER_DIVISION`,
/// sorted by strength with the best twelve shuffled) go up to the Second Division, and the Second Division's relegated
/// clubs (status 3) go down to the lowest division; with no candidate nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of den_prm.cpp at line 0x256 and
/// return) with the clubs in database order, stopping at the first national team (`exe::database_is_national_team`,
/// cdecl: the national teams come after the clubs), whose division (`club::DIVISION`) is the lowest one and whose nation
/// (`club::NATION`) is Denmark (`KEY_NATION_DENMARK`) (a 16-bit index). With at least one:
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4) (cdecl) and the first four moved to the Second Division
/// (League::move_club_to_division(this, club, the Second Division's record, 1)); with fewer than four the original
/// moves the list's unwritten slots as well (a defect of the original, kept); then each row i of the Second Division
/// (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice as the
/// original, and League::relegate_club_to_division(this, club, the lowest division's record, 1)). The list is freed.
/// Original: `DenPrm::refresh_second_division_clubs` (0x553cb0, den_prm.cpp, thiscall: ecx = the Superliga, ret 0).
pub(crate) unsafe extern "thiscall" fn den_prm_refresh_second_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9b8e6c, 0x256);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        if exe::database_is_national_team(club) != 0 {
            break;
        }
        let (d, nation) = (read_usize(club + club::DIVISION), read_usize(club + club::NATION));
        if d != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_DENMARK) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n as u16 != 0 {
        covered!("mwc_20 danish lowest division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc20: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_DANISH_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let record = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, record, 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Rule: the Dutch turnover, led by the Eredivisie: the season reopened; promotion and relegation along the chain; the
/// Eredivisie rebuilt for the next year (rules, table, fixtures, order); the division tier of the Eerste Divisie clubs
/// capped and the Eerste Divisie turned over; the predicted positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1); the table
/// HolPrm::create_table (0x5f8430) and the boxes of hol_prm.cpp (0x9cefa4) at 0x1eb, 0x1f2, 0x1f8, 0x1ff; after,
/// League::limit_club_division_tier(1, 3, −3, 1) and the start_new_season (virtual `comp_vtable::START_NEW_SEASON`, no
/// null check) of the object of `KEY_CLUB_COMP_DUTCH_TOTO_DIVISION` (the Eerste Divisie). After a box: 0.
/// Original: `HolPrm::start_new_season` (0x5f8610, hol_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
    }, 0x5f8430, 0x9cefa4, [0x1eb, 0x1f2, 0x1f8, 0x1ff], || {
        covered!("mwc_20 dutch eredivisie turnover");
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION), 1, 3, -3, 1);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_DUTCH_TOTO_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Comparer for `den_prm_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_den_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_DEN_PRM_START_NEW_SEASON);
    compare_state_league_new_season("den_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `den_prm_refresh_second_division_clubs`: only the original runs (it draws); with at least one
/// candidate, the Second Division's clubs of status 3 must end in the lowest division.
pub(crate) unsafe extern "thiscall" fn compare_den_prm_refresh_second_division_clubs(this: usize) {
    let mut candidates = 0;
    for i in 0..read_usize(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        if exe::database_is_national_team(c) != 0 {
            break;
        }
        let (d, nation) = (read_usize(c + club::DIVISION), read_usize(c + club::NATION));
        if d != 0 && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_DENMARK) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) {
            candidates += 1;
        }
    }
    let l = League(comp_by_global(KEY_CLUB_COMP_DANISH_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        for i in 0..l.club_count() as usize {
            let row = l.table() + i * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_DEN_PRM_REFRESH_SECOND_DIVISION_CLUBS)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("den_prm_refresh_second_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Comparer for `hol_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_HOL_PRM_START_NEW_SEASON);
    compare_state_league_new_season("hol_prm_start_new_season", League(this), || o(this))
}

// ---------- batch 310: the German season turnover (ger_first.cpp) ----------

/// Rule: the German turnover, led by the Bundesliga: the season reopened; with the Regionalliga loaded, its own step
/// first; promotion and relegation along the chain; the clubs exchanged between the 2. Bundesliga and the Regionalliga
/// (with it loaded by GerFirst's 0x5db980, without it by `ger_first_exchange_regional_clubs_unplayed`); the
/// Regionalliga clubs split north and south; the Bundesliga rebuilt for the next year (rules, table, fixtures, order);
/// the division tier of the 2. Bundesliga clubs capped and the 2. Bundesliga (and the Regionalliga, when loaded) turned
/// over; the predicted positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before: 0x5dbf50 (thiscall, ret 0, not ported) when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists; virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1)
/// (`ger_first_process_promotion_relegation_chain`); 0x5db980 (not ported) or `ger_first_exchange_regional_clubs_unplayed`
/// by that object again; `ger_first_split_regional_north_south` (0x5dbbd0, not ported; called with ecx = this, it does
/// not read it); the table GerFirst::create_table (0x5db230) and the boxes of ger_first.cpp (0x9cd414) at 0x2ec, 0x2f3,
/// 0x2f9, 0x300; after: League::limit_club_division_tier(1, 3, −3, 1) and the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the 2. Bundesliga, then the Regionalliga's when it exists. After a
/// box: 0.
/// Original: `GerFirst::start_new_season` (0x5db4a0, ger_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5dbf50, this);
        }
        promotion_relegation_chain(this, 1);
        if comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) != 0 {
            call_on_league(0x5db980, this);
        } else {
            call_on_league(0x5db7b0, this);
        }
        call_on_league(0x5dbbd0, this);
    }, 0x5db230, 0x9cd414, [0x2ec, 0x2f3, 0x2f9, 0x300], || {
        covered!("mwc_21 german bundesliga turnover");
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 3, -3, 1);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        start_new_season_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    })
}

/// Rule: German promotion and relegation: the final positions of the Bundesliga and the 2. Bundesliga recorded and
/// their marked clubs swapped; with the Regionalliga loaded, its final positions and its first group's recorded and
/// the marked clubs swapped between the 2. Bundesliga and the Regionalliga, and between the 2. Bundesliga and that
/// group.
///
/// Virtual `league_vtable::RECORD_FINAL_POSITIONS` on the objects of `KEY_CLUB_COMP_GERMAN_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`; League::promote_and_relegate (`exe::league_promote_and_relegate`(this,
/// above, below, 1, 1, −1, −1): 6 pushes -> thiscall, ret 0x18) Bundesliga / 2. Bundesliga; when the object of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL` exists: RECORD_FINAL_POSITIONS on it and on its stage 0 (`league::STAGES`[0]), then
/// promote_and_relegate 2. Bundesliga / Regionalliga and 2. Bundesliga / stage 0. Returns the last call's eax (without
/// the Regionalliga, the value of `G_COMP_OBJECT_TABLE` the original leaves in eax); the argument is not read.
/// Original: `GerFirst::process_promotion_relegation_chain` (0x5db6c0, ger_first.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ger_first_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), league_vtable::RECORD_FINAL_POSITIONS);
    covered!("mwc_21 german promotion and relegation");
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1, 1, -1, -1); // mwc21: the Bundesliga and the 2. Bundesliga
    let regional = comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL);
    if regional == 0 {
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    bra_nat_first_call_virtual_no_args(regional, league_vtable::RECORD_FINAL_POSITIONS);
    bra_nat_first_call_virtual_no_args(read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES)), league_vtable::RECORD_FINAL_POSITIONS);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL), 1, 1, -1, -1);
    let stage = read_usize(read_usize(comp_by_global(KEY_CLUB_COMP_GERMAN_REGIONAL) + league::STAGES));
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), stage, 1, 1, -1, -1)
}

/// Rule: at the German turnover without the Regionalliga loaded, the four best clubs of the Regionalliga division
/// (sorted by strength with the best twelve shuffled; B teams left out) go up to the 2. Bundesliga, and its relegated
/// clubs (status 3) go down to the Regionalliga division; with no candidate nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of ger_first.cpp at line 0x354 and
/// return) with the clubs in database order whose division (`club::DIVISION`) is `KEY_CLUB_COMP_GERMAN_REGIONAL`'s,
/// with a nation (`club::NATION` not null) and not a B team (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a
/// partner with the flag 0 leaves the club out; the flag is one local shared by every call, as the original's stack
/// slot) (a 16-bit index). With at least one: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4) and the first four
/// moved to the 2. Bundesliga (League::move_club_to_division(this, club, its record, 1)); with fewer than four the
/// original moves the list's unwritten slots as well (a defect of the original, kept); then each row i of the 2.
/// Bundesliga (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice,
/// and League::relegate_club_to_division(this, club, the Regionalliga's record, 1)). The list is freed.
/// Original: `ger_first_exchange_regional_clubs_unplayed` (0x5db7b0, ger_first.cpp, thiscall: ecx = the Bundesliga, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cd414, 0x354);
        return;
    }
    let mut flag: u32 = 0;
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) {
            if exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0 {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n as u16 != 0 {
        covered!("mwc_21 german regional division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwc21: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let record = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
                let club = read_usize(exe::league_get_table_row(second, i));
                exe::league_relegate_club_to_division(this, club, record, 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `ger_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_GER_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("ger_first_start_new_season", League(this), || o(this))
}

/// Comparer for `ger_first_process_promotion_relegation_chain`: only the original runs (it moves clubs between the
/// divisions; as the Turkish chain's, `tur_first_process_promotion_relegation_chain`).
pub(crate) unsafe extern "thiscall" fn compare_ger_first_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_GER_FIRST_PROCESS_PROMOTION_RELEGATION_CHAIN)(this, pairs)
}

/// Comparer for `ger_first_exchange_regional_clubs_unplayed`: only the original runs (it draws); with at least 4
/// candidates, the 2. Bundesliga's clubs of status 3 must end in the Regionalliga division.
pub(crate) unsafe extern "thiscall" fn compare_ger_first_exchange_regional_clubs_unplayed(this: usize) {
    let mut flag: u32 = 0;
    let mut candidates = 0;
    for i in 0..read_usize(G_CLUB_COUNT) {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(c + club::NATION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL) && (exe::club_get_linked_b_team(c, &mut flag, 1) == 0 || flag != 0) {
            candidates += 1;
        }
    }
    let l = League(comp_by_global(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        for i in 0..l.club_count() as usize {
            let row = l.table() + i * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_GER_FIRST_EXCHANGE_REGIONAL_CLUBS_UNPLAYED)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_GERMAN_REGIONAL);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("ger_first_exchange_regional_clubs_unplayed({:#x}): relegated clubs not in the Regionalliga division {:x?}", this, wrong));
}

// ---------- batch 310: the Belgian, Danish and German cup entrants (bel_fa_cup.cpp, den_cup.cpp, ger_cup.cpp) ----------

/// Rule: the Belgian Cup takes 112 clubs: 44 Belgian clubs of the four Fourth Division groups (sorted by strength, the
/// best 132 shuffled) with class 3, then the Third Division clubs (the national one and groups A and B) with class 2,
/// the Second Division's with class 1 and the First Division's with class 0, each group in database order until the
/// list is full; returns 1 when it is full.
///
/// `team_count` = 112; the list (`exe::crt_malloc` of 0x2a0 into `cup::TEAM_TABLE`; out of memory: the box of
/// bel_fa_cup.cpp at line 0x161 and −1). Over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]) whose division
/// (`club::DIVISION`) has the id of `KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A` to `_D` and whose nation (`club::NATION`)
/// is Belgium's record (`KEY_NATION_BELGIUM`): a local list of at most 150. `exe::comp_util_sort_and_shuffle_top_clubs`
/// (list, n, 44) and its first 44 into the entries 0..43 (the original copies 44 whatever the count: with fewer it reads
/// stack garbage; the copy reads zeros). Then three passes over the clubs, each adding at the next entry and stopping
/// when the count reaches `team_count` (checked only after adding): the division id of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`,
/// `_A` or `_B` (class 2), `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION` (1) and `KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION` (0).
/// Returns count == `team_count`.
/// Original: `BelFaCup::collect_entrants` (0x41d8d0, bel_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut lower = [0usize; 150];
    let mut n: i16 = 0;
    this.set_team_count(0x70);
    let l = exe::crt_malloc(0x2a0);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x989ae0, 0x161);
        return -1;
    }
    let fourth = [KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_A, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_B, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_C, KEY_CLUB_COMP_BELGIAN_FOURTH_DIVISION_D];
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && fourth.iter().any(|&g| read_usize(d) == read_usize(g)) && read_usize(club + club::NATION) == oceania_nations_nation_record(KEY_NATION_BELGIUM) && n < 0x96 {
            lower[n as usize] = club;
            n += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 belgian cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as u32, 0x2c); // mwc22: the best 132 shuffled
    for k in 0..0x2c {
        set_cup_entry(this, k, lower[k], 3);
    }
    let mut k = 0x2ci32;
    let passes: [(&[usize], u8); 3] = [(&[KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A, KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B], 2),
        (&[KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION], 1), (&[KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION], 0)];
    for (keys, class) in passes {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && keys.iter().any(|&g| read_usize(d) == read_usize(g)) {
                set_cup_entry(this, k as usize, club, class);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the Danish Cup takes 92 clubs, all class 0: up to 48 Danish clubs of the lowest division, then the Danish
/// Second Division clubs, the First Division's and the Superliga's, each in database order until the list is full;
/// returns 1 when it is full.
///
/// `team_count` = 92; the list (`exe::crt_malloc` of 0x228; out of memory: the box of den_cup.cpp at line 0x15a and −1).
/// Four passes over the clubs (a 16-bit index up to [`G_CLUB_COUNT`]), each adding at the next entry with class 0: the
/// division id of `KEY_CLUB_COMP_A_LOWER_DIVISION` with Denmark's nation record (`KEY_NATION_DENMARK`) while fewer than
/// 48 are in; `KEY_CLUB_COMP_DANISH_SECOND_DIVISION` with Denmark's nation record while the count is below
/// `team_count` (the pass does not stop); `KEY_CLUB_COMP_DANISH_FIRST_DIVISION` and
/// `KEY_CLUB_COMP_DANISH_PREMIER_DIVISION` (no nation check) while below `team_count`, stopping when it is reached.
/// Returns count == `team_count`.
/// Original: `DenCup::collect_entrants` (0x551240, den_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn den_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x5c);
    let l = exe::crt_malloc(0x228);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9b8dfc, 0x15a);
        return -1;
    }
    let mut k = 0i32;
    let denmark = || oceania_nations_nation_record(KEY_NATION_DENMARK);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION) && read_usize(club + club::NATION) == denmark() && k * 6 < 0x120 { // mwc22: up to 48 from the lowest division
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 danish cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_DANISH_SECOND_DIVISION) && read_usize(club + club::NATION) == denmark() && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 0);
            k += 1;
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    for g in [KEY_CLUB_COMP_DANISH_FIRST_DIVISION, KEY_CLUB_COMP_DANISH_PREMIER_DIVISION] {
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i16;
        while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(g) && k < this.team_count() as i32 {
                set_cup_entry(this, k as usize, club, 0);
                k += 1;
                if k == this.team_count() as i32 {
                    break;
                }
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
        }
    }
    (k == this.team_count() as i32) as i32
}

/// Rule: the German Cup takes 64 clubs: up to 4 amateur clubs relegated from the 2. Bundesliga last season (class 2),
/// then the other amateur clubs (sorted by strength, the best 72 shuffled) until 28 are in (class 2), then the 2.
/// Bundesliga clubs (class 2 for those promoted from below, 1 for the others) and the Bundesliga's (class 1) until the
/// list is full; returns 1 when it is full. The amateur clubs: those of the Regionalliga division with a reserve
/// division, and the German clubs of the lowest division; B teams left out.
///
/// `team_count` = 64; the list (`exe::crt_malloc` of 0x180; out of memory: the box of ger_cup.cpp at line 0x13c and
/// −1); a local list of 700 pointers (`exe::crt_malloc` of 0xaf0; out of memory: the box at line 0x144 and 0, the team
/// list left allocated). Over the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] clubs (a 16-bit index, the limit read
/// again each step) with a division: an amateur club is one with `club::RESERVE_DIVISION` set and the division id of
/// `KEY_CLUB_COMP_GERMAN_REGIONAL`, or else with a nation whose id is Germany's (`KEY_NATION_GERMANY`) and the division
/// id of `KEY_CLUB_COMP_A_LOWER_DIVISION`; B teams (`exe::club_get_linked_b_team`(club, &flag, 1), cdecl: a partner
/// with the flag 0; one flag shared by every call) are left out. An amateur club whose `club::LAST_DIVISION` has the id
/// of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION` goes straight in (class 2) while fewer than 4 are in; the others go to the
/// local list (at most 700). `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 24); then, while the count is below
/// 28, the list's clubs in order (class 2), each slot zeroed after it is copied; and a second pass over the list from
/// its start while still below 28 (it copies the zeroed slots: with too few amateur clubs the original fills the rest
/// with null clubs; a defect of the original, kept). The local list is freed. Then a pass over the clubs (16-bit index
/// up to `G_CLUB_COUNT`) for the 2. Bundesliga (division id of `KEY_CLUB_COMP_GERMAN_SECOND_DIVISION`) while below
/// `team_count`: class 2 when the last division is set and is neither the 2. Bundesliga nor the Bundesliga, else 1;
/// then the Bundesliga (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`, class 1); both stop when the count reaches
/// `team_count`. Returns count == `team_count`.
/// Original: `GerCup::collect_entrants` (0x5d9370, ger_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x40);
    let l = exe::crt_malloc(0x180);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd3e0, 0x13c);
        return -1;
    }
    let list = exe::crt_malloc(0xaf0);
    if list == 0 {
        show_error_at(0x9cd3e0, 0x144);
        return 0;
    }
    let mut k = 0i32;
    let mut n: u32 = 0;
    let mut flag: u32 = 0;
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 {
            let amateur = (read_usize(club + club::RESERVE_DIVISION) != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_REGIONAL)) || {
                let nation = read_usize(club + club::NATION);
                nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(d) == read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)
            };
            if amateur && (exe::club_get_linked_b_team(club, &mut flag, 1) == 0 || flag != 0) {
                let last = read_usize(club + club::LAST_DIVISION);
                if last != 0 && read_usize(last) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k * 6 < 0x18 { // mwc22: up to 4 relegated amateur clubs
                    set_cup_entry(this, k as usize, club, 2);
                    k += 1;
                } else if (n as u16 as i16) < 0x2bc {
                    write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                    n = n.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwc_22 german cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 0x18); // mwc22: the best 72 amateur clubs shuffled
    let count = n as u16 as i16;
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            let slot = list + 4 * j as usize;
            set_cup_entry(this, k as usize, read_usize(slot), 2);
            write_usize(slot, 0);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    if count > 0 {
        let mut j = 0i16;
        while k * 6 < 0xa8 {
            set_cup_entry(this, k as usize, read_usize(list + 4 * j as usize), 2);
            k += 1;
            j += 1;
            if j >= count {
                break;
            }
        }
    }
    exe::crt_free(list);
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && k < this.team_count() as i32 {
            let last = read_usize(club + club::LAST_DIVISION);
            let up = last != 0 && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_SECOND_DIVISION) && read_usize(last) != read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
            set_cup_entry(this, k as usize, club, if up { 2 } else { 1 });
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION) && k < this.team_count() as i32 {
            set_cup_entry(this, k as usize, club, 1);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    (k == this.team_count() as i32) as i32
}

/// Comparer for `bel_fa_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_bel_fa_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BEL_FA_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("bel_fa_cup_collect_entrants", Cup(this), || o(this), || bel_fa_cup_collect_entrants(this), |_| true)
}

/// Comparer for `den_cup_collect_entrants` (side by side, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_den_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_DEN_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("den_cup_collect_entrants", Cup(this), || o(this), || den_cup_collect_entrants(this), |_| true)
}

/// Comparer for `ger_cup_collect_entrants` (side by side with the random generator restored, every entry compared).
pub(crate) unsafe extern "thiscall" fn compare_ger_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GER_CUP_COLLECT_ENTRANTS);
    compare_entrants_with_seeds("ger_cup_collect_entrants", Cup(this), || o(this), || ger_cup_collect_entrants(this), |_| true)
}

// ---------- batch 310: the German League Cup entrants and the Belgian and Dutch super cup clubs (ger_lge_cup.cpp, bel_super.cpp, hol_super.cpp) ----------

/// Rule: the German League Cup (Ligapokal) takes 6 clubs: the Bundesliga's top six of last season minus the German Cup
/// holder, who enters as the sixth; entries 4 and 5 (the best-placed one after the holder, and the holder) are class 1.
///
/// `team_count` = 6. A local list of 6 slots, zeroed; over the clubs (up to [`G_CLUB_COUNT`] read once) while fewer
/// than 6 are counted: a club whose `club::LAST_DIVISION` is the Bundesliga's record (`KEY_CLUB_COMP_GERMAN_FIRST_DIVISION`)
/// and whose `club::LAST_POSITION` (a signed byte) is 1..6 goes to slot position − 1 (the count goes up even when a
/// slot is overwritten). Short of 6: the box of ger_lge_cup.cpp at line 0x11d, then over the clubs (the count read
/// again) while short of 6: a German club (nation id `KEY_NATION_GERMANY`) of the same last division with a last
/// position above 6 goes to the first empty slot (the count goes up even when none is empty). The list
/// (`exe::crt_calloc`(team_count, 6) into `cup::TEAM_TABLE`; out of memory: the box at line 0x13d and −1). Entry 5 =
/// the latest winner of the German Cup (`exe::comp_util_get_latest_placed_club`(record of `KEY_CLUB_COMP_GERMAN_CUP`,
/// 0)); none: the box at line 0x146 and 0. Then entries 4, 3, 2, 1, 0 each take the first slot that is set and is not
/// the holder (the slot is cleared); none: the entry stays empty. Class and status 0 for all, class 1 for entries 4
/// and 5. Returns 1.
/// Original: `GerLgeCup::collect_entrants` (0x5dc700, ger_lge_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let mut slots = [0usize; 6];
    this.set_team_count(6);
    let first = comp_record_of(KEY_CLUB_COMP_GERMAN_FIRST_DIVISION);
    let mut n: i16 = 0;
    let count = read_usize(G_CLUB_COUNT) as i32;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < count && n < 6 {
        let pos = read_i8(club + club::LAST_POSITION);
        if read_usize(club + club::LAST_DIVISION) == first && pos < 7 && pos > 0 {
            slots[(pos - 1) as usize] = club;
            n += 1;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if n != 6 {
        show_error_at(0x9cd450, 0x11d);
        let mut club = read_usize(G_CLUB_TABLE);
        let mut i = 0i32;
        while i < read_usize(G_CLUB_COUNT) as i32 && n < 6 {
            let nation = read_usize(club + club::NATION);
            if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_GERMANY) && read_usize(club + club::LAST_DIVISION) == first && read_i8(club + club::LAST_POSITION) > 6 {
                if let Some(s) = slots.iter_mut().find(|s| **s == 0) {
                    *s = club;
                }
                n += 1;
            }
            i += 1;
            club = club.wrapping_add(club::SIZE);
        }
    }
    let l = exe::crt_calloc(this.team_count() as i32 as u32, 6);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9cd450, 0x13d);
        return -1;
    }
    write_usize(l + 0x1e, exe::comp_util_get_latest_placed_club(comp_record_of(KEY_CLUB_COMP_GERMAN_CUP), 0));
    if read_usize(this.team_table() + 0x1e) == 0 {
        show_error_at(0x9cd450, 0x146);
        return 0;
    }
    covered!("mwc_23 german league cup entrants");
    let mut e = 4i32;
    while e >= 0 {
        if let Some(s) = slots.iter_mut().find(|s| **s != 0 && **s != read_usize(this.team_table() + 0x1e)) {
            write_usize(this.team_table() + 6 * e as usize, *s);
            *s = 0;
        }
        e -= 1;
    }
    for k in 0..6usize {
        write_u8(this.team_table() + 6 * k + 4, 0);
        write_u8(this.team_table() + 6 * k + 5, 0);
        if k >= 4 { // mwc23: the holder and the best placed after him seeded
            write_u8(this.team_table() + 6 * k + 4, 1);
        }
    }
    1
}

/// The body of `BelSuper::select_teams` and `HolSuper::select_teams` (thiscall, ret 0): the league champion and the cup
/// winner, with no fallback.
///
/// `team_count` = 2; the list (`exe::crt_malloc` of 12 into `cup::TEAM_TABLE`; out of memory: the box at `rows[0]` and
/// −1). Entry 0 = the latest champion of `league` (`exe::comp_util_get_latest_placed_club`(record, 0), cdecl); none:
/// the box at `rows[1]` and 0. Entry 1 = the latest winner of `cup`; none: the box at `rows[2]` and 0. The same club in
/// both: entry 1 = the latest runner-up (place 1) of `league`; none: the box at `rows[3]` and 0. Class `classes.0` and
/// status 0 for entry 0, class `classes.1` and status 0 for entry 1; with `swap_2001` and the cup's year (+0x40) 2001,
/// the two clubs swap entries (the classes stay). Returns 1.
pub(crate) unsafe fn select_champion_and_cup_winner(this: Cup, league: usize, cup: usize, file: usize, rows: [u32; 4], classes: (u8, u8), swap_2001: bool) -> (i32, i32) {
    this.set_team_count(2);
    let l = exe::crt_malloc(0xc);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(file, rows[0]);
        return (-1, 0);
    }
    write_usize(l, exe::comp_util_get_latest_placed_club(comp_record_of(league), 0)); // mwc23: the league champion
    if read_usize(this.team_table()) == 0 {
        show_error_at(file, rows[1]);
        return (0, 0);
    }
    write_usize(this.team_table() + 6, exe::comp_util_get_latest_placed_club(comp_record_of(cup), 0));
    let t = this.team_table();
    if read_usize(t + 6) == 0 {
        show_error_at(file, rows[2]);
        return (0, 0);
    }
    if read_usize(t) == read_usize(t + 6) {
        covered!("mwc_23 super cup double");
        write_usize(this.team_table() + 6, exe::comp_util_get_latest_placed_club(comp_record_of(league), 1));
        if read_usize(this.team_table() + 6) == 0 {
            show_error_at(file, rows[3]);
            return (0, 0);
        }
    }
    covered!("mwc_23 super cup clubs");
    let t = this.team_table();
    write_u8(t + 4, classes.0);
    write_u8(t + 5, 0);
    write_u8(t + 0xa, classes.1);
    write_u8(t + 0xb, 0);
    if swap_2001 && read_i16(this.addr() + 0x40) == 0x7d1 {
        let (a, b) = (read_usize(t), read_usize(t + 6));
        write_usize(t, b);
        write_usize(t + 6, a);
    }
    (1, 0)
}

/// Rule: the Belgian Super Cup is between the Belgian First Division champion (`KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION`,
/// class 2) and the Belgian Cup winner (`KEY_CLUB_COMP_BELGIAN_CUP`, class 1); if the same club won both, the First
/// Division runner-up comes in; a missing club is an error box (lines 0xc6, 0xcf, 0xd8, 0xe4 of bel_super.cpp) and 0.
/// In 2001 the two clubs swap places (the champion second).
/// Original: `BelSuper::select_teams` (0x4212f0, bel_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP, 0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true).0
}

/// Rule: the Dutch Super Cup (Johan Cruijff Schaal) is between the Eredivisie champion
/// (`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) and the Dutch Cup winner (`KEY_CLUB_COMP_DUTCH_AMSTEL_CUP`), both class
/// 0; if the same club won both, the Eredivisie runner-up comes in; a missing club is an error box (lines 0xbe, 0xc7,
/// 0xd0, 0xdc of hol_super.cpp) and 0.
/// Original: `HolSuper::select_teams` (0x5f8c10, hol_super.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_super_select_teams(this: usize) -> i32 {
    select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP, 0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false).0
}

/// Comparer for `ger_lge_cup_collect_entrants` (side by side, every entry compared).
///
/// The holder comes from `comp_util_get_latest_placed_club` (0x4afd80), which re-sorts the global competition history
/// table (`[0xae23d4]`, `[0xae237c]` records of 0x1a bytes) with the CRT's unstable qsort on every call: run twice, the
/// second call can see records of equal key in another order. So the copy runs on the history as the original saw it,
/// and the original's history is put back after it (310th batch's real game: `list false` on the six entries).
pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_GER_LGE_CUP_COLLECT_ENTRANTS);
    let hist = || (read_usize(0xae23d4), read_usize(0xae237c) * 0x1a);
    let snap = |(p, n): (usize, usize)| if p == 0 { Vec::new() } else { core::slice::from_raw_parts(p as *const u8, n).to_vec() };
    let put = |p: usize, v: &[u8]| if p != 0 && !v.is_empty() { core::ptr::copy_nonoverlapping(v.as_ptr(), p as *mut u8, v.len()) };
    let before = core::cell::RefCell::new(Vec::new());
    let after = core::cell::RefCell::new(Vec::new());
    let lists = core::cell::RefCell::new((Vec::new(), Vec::new()));
    let entries = |t: usize| (0..6).map(|k| if t == 0 { 0 } else { read_usize(t + 6 * k) }).collect::<Vec<_>>();
    let r = compare_entrants_with_seeds("ger_lge_cup_collect_entrants", Cup(this), || {
        *before.borrow_mut() = snap(hist());
        let r = o(this);
        *after.borrow_mut() = snap(hist());
        lists.borrow_mut().0 = entries(Cup(this).team_table());
        r
    }, || {
        put(hist().0, &before.borrow());
        let r = ger_lge_cup_collect_entrants(this);
        lists.borrow_mut().1 = entries(Cup(this).team_table());
        put(hist().0, &after.borrow());
        r
    }, |_| true);
    let l = lists.borrow();
    if l.0 != l.1 {
        linha(&format!("engine: ger_lge_cup_collect_entrants lists original {:x?} rust {:x?}", l.0, l.1));
    }
    r
}

/// Comparer for `bel_super_select_teams` (side by side: `compare_team_list`).
pub(crate) unsafe extern "thiscall" fn compare_bel_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_BEL_SUPER_SELECT_TEAMS);
    compare_team_list("bel_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_BELGIAN_FIRST_DIVISION, KEY_CLUB_COMP_BELGIAN_CUP,
        0x989ccc, [0xc6, 0xcf, 0xd8, 0xe4], (2, 1), true))
}

/// Comparer for `hol_super_select_teams`.
pub(crate) unsafe extern "thiscall" fn compare_hol_super_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_HOL_SUPER_SELECT_TEAMS);
    compare_team_list("hol_super_select_teams", Cup(this), || o(this), || select_champion_and_cup_winner(Cup(this), KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION, KEY_CLUB_COMP_DUTCH_AMSTEL_CUP,
        0x9cefdc, [0xbe, 0xc7, 0xd0, 0xdc], (0, 0), false))
}

// ---------- batch 310: the Belgian Second Division and Eerste Divisie period results and stages (bel_second.cpp, hol_first.cpp) ----------

/// A league result in a season split into periods (the Belgian Second Division and the Eerste Divisie): the result goes
/// to the league (League::add_result `exe::league_add_result`(this, fixture, arg): thiscall, ret 8); a fixture with no
/// stage yet (`fixture::STAGE_INDEX` = 0xff) gets the period of its round (`fixture::ROUND_NUMBER`, a signed word: the
/// number of `bounds` it has reached); while the league's current stage (`league::CURRENT_STAGE`) is below that period,
/// the next period stage is created (`create_period`(this, current stage byte + 1): push -> thiscall, ret 4); then the
/// result also goes to the period stage (its virtual +4 with (fixture, arg): thiscall) with the league's statistics
/// (`league::COMP_STATS`) set aside meanwhile, and the fixture's stage goes back to 0xff. Returns 1.
pub(crate) unsafe fn add_result_by_period(this: usize, fixture: usize, arg: u32, bounds: &[i16], create_period: usize) -> u32 {
    exe::league_add_result(this, fixture, arg);
    if read_u8(fixture + fixture::STAGE_INDEX) != 0xff {
        return 1;
    }
    let round = read_i16(fixture + fixture::ROUND_NUMBER);
    write_u8(fixture + fixture::STAGE_INDEX, bounds.iter().take_while(|&&b| round >= b).count() as u8);
    let create: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(create_period);
    while read_i32(this + league::CURRENT_STAGE) < read_i8(fixture + fixture::STAGE_INDEX) as i32 {
        create(this, read_u8(this + league::CURRENT_STAGE).wrapping_add(1) as u32);
    }
    let stats = read_usize(this + league::COMP_STATS);
    write_usize(this + league::COMP_STATS, 0);
    let stage = read_usize(read_usize(this + league::STAGES) + 4 * read_i8(fixture + fixture::STAGE_INDEX) as isize as usize);
    let add: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(stage) + 4));
    add(stage, fixture, arg);
    write_usize(this + league::COMP_STATS, stats);
    write_u8(fixture + fixture::STAGE_INDEX, 0xff);
    1
}

/// Rule: a Belgian Second Division result also counts in its period: rounds 1..9 the first period, 10..21 the second,
/// 22 on the third (the period stages created as the season reaches them); `add_result_by_period` with the bounds 10
/// and 22 and BelSecond::create_period_stage (0x4204d0).
/// Original: `BelSecond::add_result` (0x420110, bel_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn bel_second_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    covered!("mwc_24 belgian second division result");
    add_result_by_period(this, fixture, arg, &[10, 22], 0x4204d0) // mwc24: the Belgian periods
}

/// Rule: the next stage of the Belgian Second Division: past the last stage, the error box; otherwise the current
/// stage goes up by one; on reaching stage 3 (after the three periods) the play-off group is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of bel_second.cpp at line
/// 0x26e and nothing else. Otherwise the current stage + 1; at 3, BelSecond::create_playoff_group (0x420230, thiscall,
/// ret 0, not ported).
/// Original: `BelSecond::advance_stage` (0x4201a0, bel_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_second_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x989b54, 0x26e);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwc_24 belgian second division next stage");
    if stage == 3 {
        call_on_league(0x420230, l);
    }
}

/// Rule: an Eerste Divisie result also counts in its period: rounds 1..7 the first period, 8..15 the second, 16..25 the
/// third, 26 on the fourth; `add_result_by_period` with the bounds 8, 16 and 26 and HolFirst::create_period_stage
/// (0x5f6ad0).
/// Original: `HolFirst::add_result` (0x5f65b0, hol_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    covered!("mwc_24 eerste divisie result");
    add_result_by_period(this, fixture, arg, &[8, 16, 26], 0x5f6ad0) // mwc24: the Dutch periods
}

/// The common comparer of the period results: only the original runs (it creates stages and adds results); for a
/// fixture with no stage, the league's current stage must reach the period of its round and the fixture's stage must
/// be 0xff again.
pub(crate) unsafe fn compare_add_result_by_period(name: &str, this: usize, fixture: usize, bounds: &[i16], original: impl FnOnce() -> u32) -> u32 {
    let open = read_u8(fixture + fixture::STAGE_INDEX) == 0xff;
    let round = read_i16(fixture + fixture::ROUND_NUMBER);
    let r = original();
    if open {
        let period = bounds.iter().take_while(|&&b| round >= b).count() as i32;
        let (stage, mark) = (read_i32(this + league::CURRENT_STAGE), read_u8(fixture + fixture::STAGE_INDEX));
        SCOREBOARD.record(r == 1 && stage >= period && mark == 0xff, || format!("{}({:#x}) round {} period {} stage {} mark {:#x} returned {}", name, this, round, period, stage, mark, r));
    }
    r
}

/// Comparer for `bel_second_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_bel_second_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_BEL_SECOND_ADD_RESULT);
    compare_add_result_by_period("bel_second_add_result", this, fixture, &[10, 22], || o(this, fixture, arg))
}

/// Comparer for `bel_second_advance_stage`: side by side on the current stage when nothing is built; at the last stage
/// or on reaching stage 3, only the original.
pub(crate) unsafe extern "thiscall" fn compare_bel_second_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BEL_SECOND_ADVANCE_STAGE);
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 3 {
        return o(this);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + league::CURRENT_STAGE, 4)], || o(this), || bel_second_advance_stage(this));
    SCOREBOARD.record(same, || format!("bel_second_advance_stage({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// Comparer for `hol_first_add_result` (`compare_add_result_by_period`).
pub(crate) unsafe extern "thiscall" fn compare_hol_first_add_result(this: usize, fixture: usize, arg: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_HOL_FIRST_ADD_RESULT);
    compare_add_result_by_period("hol_first_add_result", this, fixture, &[8, 16, 26], || o(this, fixture, arg))
}

// ---------- batch 310: the Belgian season turnover and the Eerste Divisie and Dutch Cup stages (bel_first.cpp, hol_first.cpp, hol_cup.cpp) ----------

/// Rule: the Belgian turnover, led by the First Division: the season reopened; promotion and relegation along the
/// chain; the Third Division's movements and its split into groups A and B; the division tier of the clubs capped (the
/// First Division with (1, 3, −3, 1), the Second with (2, −3, 3, 0)); the First Division rebuilt for the next year
/// (rules, table, fixtures, order); the Second and Third Divisions turned over; the predicted positions, the board
/// expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1),
/// bel_first_move_third_division_clubs (0x41ec90) and bel_first_split_third_division_groups (0x41ef70) (called with ecx
/// = this, not ported) and League::limit_club_division_tier (`exe::league_limit_club_division_tier`) on this and on the
/// object of `KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION`; the table BelFirst::create_table (0x41e8b0) and the boxes of
/// bel_first.cpp (0x989b18) at 0x1af, 0x1b6, 0x1bc, 0x1c3; after, the start_new_season (virtual
/// `comp_vtable::START_NEW_SEASON`, no null check) of the Second Division and of `KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`.
/// After a box: 0.
/// Original: `BelFirst::start_new_season` (0x41ea90, bel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x41ec90, this);
        call_on_league(0x41ef70, this);
        exe::league_limit_club_division_tier(this, 1, 3, -3, 1);
        exe::league_limit_club_division_tier(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), 2, -3i32 as u32, 3, 0);
    }, 0x41e8b0, 0x989b18, [0x1af, 0x1b6, 0x1bc, 0x1c3], || {
        covered!("mwc_25 belgian first division turnover");
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: the next stage of the Eerste Divisie: past the last stage, the error box; otherwise only when the Eredivisie is
/// complete: its positions and statuses are updated and the current stage goes up by one; on reaching stage 4 (after
/// the four periods) the play-off groups are built. With the Eredivisie still playing, nothing (it asks again later).
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_first.cpp at line
/// 0x263. Otherwise `premier_division_complete`(`KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION`) (virtual
/// `comp_vtable::CHECK_COMPLETE` with (0, 1)); when complete, `update_premier_division_status` (virtual
/// `league_vtable::UPDATE_POSITIONS_AND_STATUS`), the current stage (read again) + 1 and, at 4,
/// HolFirst::create_play_off_groups (0x5f6720, thiscall, ret 0, not ported).
/// Original: `HolFirst::advance_stage` (0x5f6650, hol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_first_advance_stage(this: usize) {
    let l = League(this);
    if (l.current_stage() as i32) >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cee08, 0x263);
    }
    if !premier_division_complete(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION) {
        return;
    }
    update_premier_division_status(KEY_CLUB_COMP_DUTCH_KPN_TELECOMPETITION);
    let stage = (l.current_stage() as i32).wrapping_add(1);
    l.set_current_stage(stage as usize);
    covered!("mwc_25 eerste divisie next stage");
    if stage == 4 { // mwc25: the play-off groups after the four periods
        call_on_league(0x5f6720, l);
    }
}

/// Rule: the next stage of the Dutch Cup: past the last stage, the error box; otherwise the current stage goes up by
/// one; on reaching stage 19 (after the group stages) the knockout stage is built.
///
/// At the last stage (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the box of hol_cup.cpp at line
/// 0x24f. Otherwise the current stage + 1; at 0x13, HolCup::create_knockout_stage (0x5f4480, thiscall, ret 0, not
/// ported).
/// Original: `HolCup::advance_stage` (0x5f4190, hol_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn hol_cup_advance_stage(this: usize) {
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x9cedd4, 0x24f);
    }
    let stage = stage + 1;
    l.set_current_stage(stage as usize);
    covered!("mwc_25 dutch cup next stage");
    if stage == 0x13 {
        call_on_league(0x5f4480, l);
    }
}

/// Comparer for `bel_first_start_new_season`: only the original runs (it moves clubs and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_bel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BEL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("bel_first_start_new_season", League(this), || o(this))
}

/// Comparer for `hol_first_advance_stage`: only the original runs (it asks the Eredivisie and updates it). Without the
/// play-off it is `compare_first_division_advance`; on reaching stage 4, `HolFirst::create_play_off_groups` (0x5f6720)
/// adds stages 4 and 5 (the virtual `+0x3c` with 4 and with 5) and the original ends on 5: in the 310th batch's real
/// game `before=3 after=5`, a false alarm of the one-step rule.
pub(crate) unsafe extern "thiscall" fn compare_hol_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_HOL_FIRST_ADVANCE_STAGE);
    let l = League(this);
    let before = l.current_stage() as i32;
    if before + 1 != 4 || before >= (l.stage_count() as i32).wrapping_sub(1) {
        return compare_first_division_advance("hol_first_advance_stage", l, || o(this));
    }
    o(this);
    let after = l.current_stage() as i32;
    SCOREBOARD.record(after == before || after == 5, || format!("hol_first_advance_stage({:#x}) before={} after={} (play-off: 5 expected)", this, before, after));
}

/// Comparer for `hol_cup_advance_stage`: side by side on the current stage when nothing is built; at the last stage or
/// on reaching stage 0x13, only the original.
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_HOL_CUP_ADVANCE_STAGE);
    let l = League(this);
    let stage = l.current_stage() as i32;
    if stage >= (l.stage_count() as i32).wrapping_sub(1) || stage + 1 == 0x13 {
        return o(this);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(this + league::CURRENT_STAGE, 4)], || o(this), || hol_cup_advance_stage(this));
    SCOREBOARD.record(same, || format!("hol_cup_advance_stage({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}
