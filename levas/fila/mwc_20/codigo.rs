// ---------- batch %%LEVA%%: the Danish and Dutch top-division season turnovers (den_prm.cpp, hol_prm.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
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
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("den_prm_refresh_second_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Comparer for `hol_prm_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_hol_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("hol_prm_start_new_season", League(this), || o(this))
}
