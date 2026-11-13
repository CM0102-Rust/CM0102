// ---------- batch %%LEVA%%: the Russian season turnover (rus_first.cpp, rus_prm.cpp) ----------

/// Rule: the Russian First Division turnover (called by the Premier League's): the season reopened (+0x4c = 0), the
/// table, the dates and the stage objects freed, the statistics archived, the next year, the rules, the table, the
/// fixtures, the order and the predicted positions (`rebuild_season`, lines 0x168, 0x16f, 0x175, 0x17c of
/// rus_first.cpp); then the board expectations and the officials (`set_expectations_and_renew_officials`).
/// Original: `RusFirst::start_new_season` (0x7ea8c0, rus_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn rus_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_23 russian first division turnover");
    if !rebuild_season(this, 0x7ea6c0, 0xa7ac8c, [0x168, 0x16f, 0x175, 0x17c], 1) {
        return 0;
    }
    set_expectations_and_renew_officials(this)
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
    let exchange: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7ebe20usize);
    exchange(this.addr()); // mwd23: the First and second division exchange
    covered!("mwd_23 russian premier league turnover");
    if !rebuild_croatian_league(this, 0x7eb610, 0xa7acc8, [0x1b4, 0x1bb, 0x1c1, 0x1c8], true) {
        return 0;
    }
    let first = comp_by_global(KEY_CLUB_COMP_RUSSIAN_FIRST_DIVISION);
    exe::league_limit_club_division_tier(first, 1, 3, -3, 1);
    bra_nat_first_call_virtual_no_args(first, comp_vtable::START_NEW_SEASON);
    finish_croatian_turnover(this)
}

/// The six Russian second division zones, as pairs whose best club goes up: west and central, south and Volga, east and
/// Ural (each pair checked in that order).
pub(crate) const RUSSIAN_ZONE_PAIRS: [[usize; 2]; 3] = [
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_WEST, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_CENTRAL],
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_SOUTH, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_VOLGA],
    [KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_EAST, KEY_CLUB_COMP_RUSSIAN_SECOND_DIVISION_URAL],
];

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
        let c = read_usize(list + 4 * k as usize);
        let d = read_usize(c + club::DIVISION);
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
    let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
    for (back, zone) in [(1i32, zones[0][1]), (2, zones[1][0]), (3, zones[2][1])] {
        let row = exe::league_get_table_row(first, League(first).club_count() as i32 - back);
        relegate(this, read_usize(row), zone, 1);
    }
    exe::crt_free(list);
}

/// Comparer for `rus_first_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_rus_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("rus_first_start_new_season", League(this), || o(this))
}

/// Comparer for `rus_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_rus_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let wrong: Vec<(usize, usize)> = plan.iter().copied().filter(|&(c, z)| read_usize(c + club::DIVISION) != z).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("rus_prm_exchange_first_and_second_division_clubs({:#x}): relegated clubs not in their zone {:x?}", this, wrong));
}
