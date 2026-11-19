// ---------- batch %%LEVA%%: the Polish season turnover (pol_first.cpp, pol_second.cpp) ----------

/// Rule: the Polish turnover, led by the First Division: the season reopened; promotion and relegation along the chain;
/// the four best clubs of the lowest Polish division (the best twelve shuffled) up into the Second Division and its
/// bottom clubs down (`pol_first_promote_lower_division_clubs`); the First Division rebuilt for the next year (rules,
/// table, fixtures, order); its second group stage built again; the Second Division turned over; the predicted
/// positions, the board expectations and the officials.
///
/// `top_division_turnover` with, before, virtual `league_vtable::PROCESS_PROMOTION_RELEGATION_CHAIN`(1) (push 1 ->
/// thiscall, ret 4) and `pol_first_promote_lower_division_clubs` (direct, ecx = this); the table PolFirst::create_table
/// (0x7c9450) and the boxes of pol_first.cpp (0xa78598) at 0x1e7, 0x1ee, 0x1f4, 0x1fb; after the order,
/// PolFirst::create_second_group_stage (0x7c9900, thiscall, ret 0, not ported) and the Second Division's start_new_season
/// (virtual `comp_vtable::START_NEW_SEASON` on the object of `KEY_CLUB_COMP_POLISH_SECOND_DIVISION`, called without a
/// null check). The stages are not destroyed and the current stage is not reset (as the original). After a box: 0.
/// Original: `PolFirst::start_new_season` (0x7c9740, pol_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    top_division_turnover(this, || {
        promotion_relegation_chain(this, 1);
        call_on_league(0x7ca030, this);
    }, 0x7c9450, 0xa78598, [0x1e7, 0x1ee, 0x1f4, 0x1fb], || {
        covered!("mwd_29 polish first division turnover");
        call_on_league(0x7c9900, this);
        bra_nat_first_call_virtual_no_args(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), comp_vtable::START_NEW_SEASON);
    })
}

/// Rule: at the Polish turnover, the four best Polish clubs of the lowest division (`KEY_CLUB_COMP_POLISH_LOWER_DIVISION`,
/// sorted by strength with the best twelve shuffled) go up to the Second Division, and the Second Division's bottom
/// `league::RELEGATED_COUNT` clubs go down to the lowest division; with fewer than 4 candidates nobody moves.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box of pol_first.cpp at line 0x2f8 and
/// return) with the clubs whose nation (`club::NATION`) is Poland (`KEY_NATION_POLAND`) and whose division
/// (`club::DIVISION`) is the lowest one, in database order (a 16-bit index); each of them also gets
/// `club::LAST_DIVISION` = its division. With at least 4: `exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4)
/// (cdecl), the first four moved to the Second Division (League::move_club_to_division(this, club, the Second
/// Division's record, 1)); then the rows from `club_count − relegated_count` (a word and a signed byte of the Second
/// Division) up to the last one go down (League::get_table_row, then League::relegate_club_to_division(this, club, the
/// lowest division's record, 1)). The list is freed.
/// Original: `pol_first_promote_lower_division_clubs` (0x7ca030, pol_first.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_first_promote_lower_division_clubs(this: usize) {
    let second = comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0xa78598, 0x2f8);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) {
            let d = read_usize(club + club::DIVISION);
            if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION) {
                write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
                n = n.wrapping_add(1);
                write_usize(club + club::LAST_DIVISION, read_usize(club + club::DIVISION));
            }
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_29 polish lowest division up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd29: the best twelve shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_POLISH_SECOND_DIVISION), 1);
        }
        let last = read_i16(second + league::CLUB_COUNT) as i32;
        let mut i = last - read_i8(second + league::RELEGATED_COUNT) as i32;
        while i < last {
            let record = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
            let club = read_usize(exe::league_get_table_row(second, i));
            exe::league_relegate_club_to_division(this, club, record, 1); // mwd29: the bottom clubs down
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Rule: the Polish Second Division turnover (called by the First Division's): the season reopened, last season's
/// stages destroyed and the league rebuilt for the next year (`rebuild_season` with PolSecond::create_table at 0x7cbdf0
/// and the boxes of pol_second.cpp at lines 0x204, 0x20b, 0x211, 0x218; the predicted positions), then the board
/// expectations and the officials (`set_expectations_and_renew_officials`), whose eax is returned. After a box: 0.
/// Original: `PolSecond::start_new_season` (0x7cc000, pol_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn pol_second_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x7cbdf0, 0xa7860c, [0x204, 0x20b, 0x211, 0x218], 1) {
        return 0;
    }
    covered!("mwd_29 polish second division turnover");
    set_expectations_and_renew_officials(this)
}

/// Comparer for `pol_first_start_new_season`: only the original runs (it draws and rebuilds); the year must go up by one.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    compare_state_league_new_season("pol_first_start_new_season", League(this), || o(this))
}

/// Comparer for `pol_first_promote_lower_division_clubs`: only the original runs (it draws); with at least 4
/// candidates, the Second Division's bottom `relegated_count` clubs must end in the lowest division.
pub(crate) unsafe extern "thiscall" fn compare_pol_first_promote_lower_division_clubs(this: usize) {
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let (nation, d) = (read_usize(c + club::NATION), read_usize(c + club::DIVISION));
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_POLAND) && d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_POLISH_LOWER_DIVISION)
    }).count();
    let l = League(comp_by_global(KEY_CLUB_COMP_POLISH_SECOND_DIVISION));
    let mut planned: Vec<usize> = Vec::new();
    if candidates >= 4 && l.table() != 0 {
        let last = l.club_count() as i32;
        for i in (last - read_i8(l.addr() + league::RELEGATED_COUNT) as i32).max(0)..last {
            planned.push(read_usize(l.table() + i as usize * league_table_row::SIZE));
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let lower = comp_record_of(KEY_CLUB_COMP_POLISH_LOWER_DIVISION);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("pol_first_promote_lower_division_clubs({:#x}): relegated clubs not in the lowest division {:x?}", this, wrong));
}

/// Comparer for `pol_second_start_new_season`.
pub(crate) unsafe extern "thiscall" fn compare_pol_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_state_league_new_season("pol_second_start_new_season", League(this), || o(this))
}
