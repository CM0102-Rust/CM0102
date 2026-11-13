// ---------- batch %%LEVA%%: the Greek season turnover (gre_prm.cpp, gre_second.cpp) ----------

/// Rule: the Greek turnover, led by the First Division: the season reopened (+0x4c = 0); promotion and relegation along
/// the chain (virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN` with 0: push 0 -> thiscall, ret 4); the two
/// best Third Division clubs up and the Second Division's relegated down (`gre_prm_swap_second_and_third_division_clubs`,
/// ecx = this); the First Division rebuilt for the next year (`rebuild_croatian_league` without reopening the season:
/// lines 0x1dc, 0x1e3, 0x1e9, 0x1f0); the Second Division turned over (virtual `comp_vtable::START_NEW_SEASON`); then
/// the predicted positions, the board expectations and the officials (`finish_croatian_turnover`).
/// Original: `GrePrm::start_new_season` (0x5eaf50, gre_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    let chain: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN));
    chain(this.addr(), 0);
    let swap: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5eb120usize);
    swap(this.addr());
    covered!("mwd_22 greek first division turnover");
    if !rebuild_croatian_league(this, 0x5ead60, 0x9cdae4, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), comp_vtable::START_NEW_SEASON); // mwd22: the Second Division turnover
    finish_croatian_turnover(this)
}

/// Rule: the Greek Second Division turnover (called by the First Division's): reopened, rebuilt for the next year
/// (`rebuild_croatian_league` without reopening: lines 0x1dc, 0x1e3, 0x1e9, 0x1f0 of gre_second.cpp), the predicted
/// positions, the board expectations and the officials.
/// Original: `GreSecond::start_new_season` (0x5ebe90, gre_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    covered!("mwd_22 greek second division turnover");
    if !rebuild_croatian_league(this, 0x5ebcb0, 0x9cdb1c, [0x1dc, 0x1e3, 0x1e9, 0x1f0], false) {
        return 0;
    }
    finish_croatian_turnover(this)
}

/// Rule: at the Greek turnover, the two best clubs of the Third Division (`KEY_CLUB_COMP_GREEK_LOWER_DIVISION`, sorted
/// by strength with the best nine shuffled) go up to the Second Division, and the Second Division's relegated clubs
/// (status 3) go down to the Third; with fewer than 2 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d5 and return) with the clubs
/// of the Third Division in database order (a 16-bit index; no nation check). With at least 2:
/// `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 3) (cdecl), the first two moved to the Second Division
/// (League::move_club_to_division(this, club, the Second Division's record, 1)); then each row i of the Second Division
/// (its club count read again on each step) with status 3 goes down (League::get_table_row, called twice as the
/// original, and League::relegate_club_to_division(this, club, the Third Division's record, 1)). The list is freed.
/// Original: `gre_prm_swap_second_and_third_division_clubs` (0x5eb120, gre_prm.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9cdae4, 0x2d5);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 2 {
        covered!("mwd_22 greek third division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 3); // mwd22: the best nine shuffled
        for k in 0..2 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_GREEK_SECOND_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < League(second).club_count() as i32 {
            if read_u8(exe::league_get_table_row(second, i) + league_table_row::STATUS) == 3 {
                let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
                let club = read_usize(exe::league_get_table_row(second, i));
                relegate(this, club, comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION), 1);
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `gre_prm_start_new_season` (only the original; the year went up by one).
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("gre_prm_start_new_season", League(this), || o(this))
}

/// Comparer for `gre_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_gre_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_state_league_new_season("gre_second_start_new_season", League(this), || o(this))
}

/// Comparer for `gre_prm_swap_second_and_third_division_clubs`: only the original runs (it draws); with at least 2
/// Third Division clubs, the Second Division's clubs of status 3 must end in the Third Division.
pub(crate) unsafe extern "thiscall" fn compare_gre_prm_swap_second_and_third_division_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_GREEK_LOWER_DIVISION);
    let l = League(comp_by_global(KEY_CLUB_COMP_GREEK_SECOND_DIVISION));
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let d = read_usize(read_usize(G_CLUB_TABLE) + i * club::SIZE + club::DIVISION);
        d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_GREEK_LOWER_DIVISION)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    if l.table() != 0 && candidates >= 2 {
        for i in 0..l.club_count() as i32 {
            let row = l.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                planned.push(read_usize(row));
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("gre_prm_swap_second_and_third_division_clubs({:#x}): relegated clubs not in the Third Division {:x?}", this, wrong));
}
