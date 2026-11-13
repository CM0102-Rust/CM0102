// ---------- batch %%LEVA%%: the Croatian leagues (cro_a1.cpp, cro_a2b.cpp): the next stage and the third tier ----------

/// Rule: the Croatian First Division's next stage: at the last stage (+0x30 ≥ +0x2c − 1, signed) the error box (line
/// 0x1fa) and nothing else; otherwise the current stage goes up by one and, on reaching 0 (the league is over), the
/// tie-break play-off is built (`cro_a1_create_playoff_stage`, direct call with ecx = this).
/// Original: `CroA1::advance_stage` (0x514bd0, cro_a1.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_advance_stage(this: usize) {
    covered!("mwd_6 croatian first division next stage");
    state_league_advance_stage(League(this), 0x9b53c0, 0x1fa, 0x514c60);
}

/// Rule: the Croatian Second Division south group's next stage, as the First Division's (line 0x1e9 of cro_a2b.cpp,
/// the play-off `cro_a2b_create_tiebreak_playoff_stage`).
/// Original: `CroA2b::advance_stage` (0x517180, cro_a2b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a2b_advance_stage(this: usize) {
    covered!("mwd_6 croatian second division south next stage");
    state_league_advance_stage(League(this), 0x9b5430, 0x1e9, 0x517210);
}

/// Is the club in one of the Croatian divisions below the second (the lower division `KEY_CLUB_COMP_A_LOWER_DIVISION`
/// or the four third division groups: north, east, south, west, compared in that order)?
pub(crate) unsafe fn is_croatian_third_tier_club(club: usize) -> bool {
    let d = read_usize(club + club::DIVISION);
    if d == 0 {
        return false;
    }
    let id = read_usize(d);
    [KEY_CLUB_COMP_A_LOWER_DIVISION, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_EAST,
        KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_SOUTH, KEY_CLUB_COMP_CROATIAN_THIRD_DIVISION_WEST].iter().any(|&g| id == read_usize(g))
}

/// Relegates the club of row `i` of a second division group to the lower division (`KEY_CLUB_COMP_A_LOWER_DIVISION`)
/// when its final status is 3 (relegated): League::get_table_row (`exe::league_get_table_row`: push i -> thiscall, ret
/// 4; called again for the club, as the original) and League::relegate_club_to_division (0x6831a0: push 1; push
/// division; push club; mov ecx, this -> thiscall, ret 0xc).
pub(crate) unsafe fn relegate_croatian_row(this: usize, group: usize, i: i32) {
    if read_u8(exe::league_get_table_row(group, i) + league_table_row::STATUS) == 3 {
        let relegate: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6831a0usize);
        let club = read_usize(exe::league_get_table_row(group, i));
        relegate(this, club, comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION), 1); // mwd6: down to the lower division
    }
}

/// Rule: at the Croatian turnover, the four best clubs of the divisions below the second (the lower division and the
/// four third division groups) go up into the Second Division (north group: the regrouping that follows splits them),
/// and the clubs relegated from both second division groups go down to the lower division.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory: the box at line 0x2d2 and return) with the
/// Croatian clubs (`club::NATION` id = [`KEY_NATION_CROATIA`]) of those divisions (`is_croatian_third_tier_club`), in
/// database order (a 16-bit index). With at least 4: the list sorted by strength and the best shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4): 3 pushes; add esp, 0xc -> cdecl) and its first four moved
/// to the north group (League::move_club_to_division `exe::league_move_club_to_division`(this, club, the north group's
/// record, 1): thiscall, ret 0xc); then, for each row i below the north group's club count, the relegated rows of the
/// north group and then of the south group (that loop also bounded by the north group's count, as the original) go down
/// (`relegate_croatian_row`). The list is freed in every case.
/// Original: `cro_a1_swap_second_and_third_tier_clubs` (0x514ed0, cro_a1.cpp, thiscall: ecx = the First Division, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let north = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH);
    let south = comp_by_global(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH);
    let list = exe::crt_malloc((read_usize(G_CLUB_COUNT) as u32).wrapping_mul(4));
    if list == 0 {
        show_error_at(0x9b53c0, 0x2d2);
        return;
    }
    let mut n: u32 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 {
        let nation = read_usize(club + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(club) {
            write_usize(list + (n as u16 as i16 as isize as usize).wrapping_mul(4), club);
            n = n.wrapping_add(1);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    if (n as u16 as i16) >= 4 {
        covered!("mwd_6 croatian third tier up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4); // mwd6: the best four, shuffled
        for k in 0..4 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH), 1); // mwd6: up to the north group
        }
        let count = League(north).club_count() as i32;
        for i in 0..count {
            relegate_croatian_row(this, north, i);
            relegate_croatian_row(this, south, i);
        }
    }
    exe::crt_free(list);
}

/// Comparer for `cro_a1_advance_stage` (side by side on the current stage, only the original when the play-off would
/// be built: `ita_ser_c1a_compare_advance_stage`).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    ita_ser_c1a_compare_advance_stage("cro_a1_advance_stage", League(this), || o(this), || cro_a1_advance_stage(this))
}

/// Comparer for `cro_a2b_advance_stage`.
pub(crate) unsafe extern "thiscall" fn compare_cro_a2b_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    ita_ser_c1a_compare_advance_stage("cro_a2b_advance_stage", League(this), || o(this), || cro_a2b_advance_stage(this))
}

/// Comparer for `cro_a1_swap_second_and_third_tier_clubs`: only the original runs (it moves clubs and draws); with at
/// least 4 candidates, the clubs of both second division groups whose final status is 3 must end in the lower
/// division (the south group's rows only below the north group's club count, as the original).
pub(crate) unsafe extern "thiscall" fn compare_cro_a1_swap_second_and_third_tier_clubs(this: usize) {
    let lower = comp_record_of(KEY_CLUB_COMP_A_LOWER_DIVISION);
    let candidates = (0..read_usize(G_CLUB_COUNT)).filter(|&i| {
        let c = read_usize(G_CLUB_TABLE) + i * club::SIZE;
        let nation = read_usize(c + club::NATION);
        nation != 0 && read_usize(nation) == read_usize(KEY_NATION_CROATIA) && is_croatian_third_tier_club(c)
    }).count();
    let mut planned: Vec<usize> = Vec::new();
    for g in [KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH, KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH] {
        let l = League(comp_by_global(g));
        if l.table() != 0 && candidates >= 4 {
            for i in 0..l.club_count() as i32 {
                let row = l.table() + i as usize * league_table_row::SIZE;
                if read_u8(row + league_table_row::STATUS) == 3 {
                    planned.push(read_usize(row));
                }
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let wrong: Vec<usize> = planned.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != lower).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("cro_a1_swap_second_and_third_tier_clubs({:#x}): relegated clubs not in the lower division {:x?}", this, wrong));
}
