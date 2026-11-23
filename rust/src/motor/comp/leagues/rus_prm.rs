use super::*;

/// Rule: the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks wins, head to head, goal
/// difference, goals for; points ties across a band go to tie-break matches; nobody goes up, 2 go down; rule bit 0x40
/// (`rus_prm_rules`).
/// Original: `RusPrm::set_up_rules` (0x7eb740, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwd_14 russian premier league rules");
    apply_league_rules_with_flags(League(this), 0x40, &rus_prm_rules())
}

/// Comparer for `rus_prm_set_up_rules` (the rehearsal of `eng_prm_compare_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_SET_UP_RULES);
    eng_prm_compare_league_rules("rus_prm_set_up_rules", League(this), rus_prm_rules(), || o(this), || rus_prm_set_up_rules(this))
}

/// Rule: the Russian Premier League has 16 clubs (`KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION`); when full, the prize fund
/// (50,000 a share) and TV pays 300,000 to each club (the box at line 0x13c).
/// Original: `RusPrm::create_table` (0x7eb610, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x10, 0xa7acc8, 0x13c, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || {
        covered!("mwd_15 russian premier league tv money");
        eng_prm_pay_tv_rights(this, 0x493e0, 0)
    })
}

/// Comparer for `rus_prm_create_table`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_CREATE_TABLE);
    eng_prm_compare_league_table("rus_prm_create_table", League(this), 0x10, KEY_CLUB_COMP_RUSSIAN_PREMIER_DIVISION, 0xc350, || o(this))
}

/// Rule: the Russian turnover, led by the Premier League: the season reopened; promotion and relegation along the chain
/// (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`: push 1); the exchange between the First Division and
/// the six second division zones (`rus_prm_exchange_first_and_second_division_clubs`, ecx = this); the Premier League
/// rebuilt for the next year (`rebuild_croatian_league`, reopening: lines 0x1b4, 0x1bb, 0x1c1, 0x1c8); the First
/// Division's division tier capped (League::limit_club_division_tier(1, 3, −3, 1)) and its turnover (virtual
/// `comp_vtable::START_NEW_SEASON`); then the predicted positions, the board expectations and the officials.
/// Original: `RusPrm::start_new_season` (0x7eb800, rus_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 1);
    exe::rus_prm_exchange_first_and_second_division_clubs(this); // mwd23: the First and second division exchange
    covered!("mwd_23 russian premier league turnover");
    if !rebuild_croatian_league(this, 0x7eb610, 0xa7acc8, [0x1b4, 0x1bb, 0x1c1, 0x1c8], true) {
        return 0;
    }
    let first = comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    exe::league_limit_club_division_tier(first, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(first, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// Rule: at the Russian turnover, the best club of each pair of second division zones (west/central, south/Volga,
/// east/Ural) goes up to the First Division, and the First Division's last three go down: the last to the central
/// zone, the second last to the south zone, the third last to the Ural zone; with fewer than 3 candidates nobody moves.
///
/// The zone records are `G_CLUB_COMP_TABLE` + 0x6b × id (`comp_record_of`). A list of `G_CLUB_COUNT` pointers
/// (`exe::crt_malloc`; out of memory: the box at line 0x2bd and return) with the clubs (the first [`G_CLUB_COUNT`] − 2 ×
/// [`G_NATION_COUNT`]) whose division pointer (`club::DIVISION`) is one of the six zone records, in database order
/// (16-bit index). Fewer than 3: the list freed and return. Otherwise `exe::comp_util_sort_and_shuffle_top_clubs`(list,
/// n, 3) and, along the list until all three pairs have a club, the first club of each pair is moved to the First
/// Division (League::move_club_to_division(this, club, the First Division's record, 1)); then the First Division's rows
/// `club_count` − 1, − 2 and − 3 (League::get_table_row) go down (League::relegate_club_to_division(this, club, zone, 1))
/// to the central, south and Ural zones, whatever their status; the list is freed.
/// Original: `rus_prm_exchange_first_and_second_division_clubs` (0x7ebe20, rus_prm.cpp, thiscall: ecx = the Premier League, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_prm_exchange_first_and_second_division_clubs(this: usize) {
    let first = comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xa7acc8, 0x2bd);
        return;
    }
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if zones.iter().any(|p| d == p[0] || d == p[1]) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) < 3 {
        exe::crt_free(list);
        return;
    }
    covered!("mwd_23 russian zones up");
    exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd23: the best nine shuffled
    let mut taken = [false; 3];
    let m = n as u16 as i16 as i32;
    let mut k = 0i32;
    while k < m {
        if taken.iter().all(|&t| t) {
            break;
        }
        let c = Club(read_usize(list + 4 * k as usize));
        let d = c.division().addr();
        if let Some(z) = zones.iter().position(|p| d == p[0] || d == p[1]) {
            if !taken[z] {
                taken[z] = true;
                exe::league_move_club_to_division(this, c, comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), 1); // mwd23: the best of the pair up
            }
        } else {
            exe::league_move_club_to_division(this, c, comp_record_of(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION), 1);
        }
        k += 1;
    }
    for (back, zone) in [(1i32, zones[0][1]), (2, zones[1][0]), (3, zones[2][1])] {
        let row = exe::league_get_table_row(first, League(first).club_count() as i32 - back);
        exe::league_relegate_club_to_division(this, read_usize(row), zone, 1);
    }
    exe::crt_free(list);
}

/// Comparer for `rus_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_RUS_PRM_START_NEW_SEASON);
    compare_state_league_new_season("rus_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `rus_prm_exchange_first_and_second_division_clubs`: only the original runs (it draws); with at least 3
/// candidates, the First Division's last three clubs must end in the central, south and Ural zones.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_exchange_first_and_second_division_clubs(this: usize) {
    let zones: [[usize; 2]; 3] = RUSSIAN_ZONE_PAIRS.map(|p| p.map(|g| unsafe { comp_record_of(g) }));
    let limit = (read_usize(G_CLUB_COUNT) as i32 - 2 * read_usize(G_NATION_COUNT) as i32).max(0) as usize;
    let candidates = (0..limit).filter(|&i| {
        let d = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION);
        zones.iter().any(|p| d == p[0] || d == p[1])
    }).count();
    let first = League(comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION));
    let n = first.club_count() as usize;
    let plan: Vec<(usize, usize)> = if candidates >= 3 && first.table() != 0 && n >= 3 {
        vec![(read_usize(first.table() + (n - 1) * 0x3b), zones[0][1]), (read_usize(first.table() + (n - 2) * 0x3b), zones[1][0]), (read_usize(first.table() + (n - 3) * 0x3b), zones[2][1])]
    } else {
        Vec::new()
    };
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RUS_PRM_EXCHANGE_FIRST_AND_SECOND_DIVISION_CLUBS)(this);
    let wrong: Vec<(usize, usize)> = plan.iter().copied().filter(|&(c, z)| read_usize(c + club::DIVISION) != z).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("rus_prm_exchange_first_and_second_division_clubs({:#x}): relegated clubs not in their zone {:x?}", this, wrong));
}

/// Rule: the Russian Premier League at career creation: discipline rule 0x27 and, with no saved state, no tie-break
/// pairs, the season open and the rest of the division template (`construct_tiebreak_league`, the Croatian template).
/// Original: `RusPrm::RusPrm` (0x7eaa80, rus_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_rus_prm(this: usize, year: u32, record: usize) -> usize {
    construct_tiebreak_league(League(this), year, record, &RUSSIAN_PREMIER_LEAGUE, "mwd_12 russian premier league new")
}

/// Comparer for `construct_rus_prm` (`compare_tiebreak_league_ctor`).
pub(crate) unsafe extern "thiscall" fn compare_construct_rus_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_RUS_PRM);
    compare_tiebreak_league_ctor("construct_rus_prm", League(this), year, record, &RUSSIAN_PREMIER_LEAGUE, || o(this, year, record))
}

/// The rules of the Russian Premier League: 2 legs, 3 points a win and 1 a draw; tie-breaks 3, 4, 1, 2 (wins, head to
/// head, goal difference, goals for); nobody goes up, 2 go down (+0xc1) to the First Division
/// (`KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION`); +0x52 = 2; 7 substitutes, 3 substitutions; points ties settled by
/// tie-break matches (+0xe6 = 1); the rule bit 0x40 (+0xd9, listed for the comparer).
pub(crate) unsafe fn rus_prm_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 4), (0xc7, 1, 1),
        (0xc8, 1, 2), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 2), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x4a, 1, 3),
        (0x20, 4, read_usize(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION) as u32), (0x49, 1, 7), (0xe6, 4, 1), (league::LEAGUE_FLAGS, 1, 0)]
}

/// Comparer for the tie-break league constructors: only the original runs; checks the return, vtable, year, record and
/// discipline rule and, for a new league (not loaded), the stage fields, `TIEBREAK_PAIRS` = 0 and `season_closed` = 1.
pub(crate) unsafe fn compare_tiebreak_league_ctor(name: &str, this: League, year: u32, record: usize, p: &LeagueDivision, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtabela && built_year_ok(this.year() as u16, year as u16)
        && this.competition().addr() == record && this.discipline_rule() == p.discipline;
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 && !loaded {
        ok = ok && this.stage_count() == 0 && this.stages() == 0 && read_u8(this.addr() + TIEBREAK_PAIRS) == 0 && this.season_closed() == 1;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}, discipline {}]: returned {:#x}, vtable {:#x}, +0x2c = {}, +0xee = {}, +0x4c = {}, +8 = {:#x}", name, this.addr(), this.year(), year as u16, this.competition().addr(), record, this.discipline_rule(), r, read_usize(this.addr()), this.stage_count(), read_u8(this.addr() + TIEBREAK_PAIRS), this.season_closed(), this.comp_stats()));
    r
}

/// The Russian Premier League (Top Division): discipline rule 0x27; points ties settled by tie-break matches.
pub(crate) const RUSSIAN_PREMIER_LEAGUE: LeagueDivision = LeagueDivision {
    vtabela: 0x96f278,
    discipline: 0x27,
    rules: 0x7eb740,
    table: 0x7eb610,
    file: 0xa7acc8,
    rows: [0x50, 0x57, 0x5e, 0x67, 0x6e],
    low: false,
};
