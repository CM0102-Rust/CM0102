// ---------- cx14_9: the Allsvenskan's promotion chain, the regional clubs going up and its progress codes (swe_prm.cpp) ----------

/// The Swedish promotion and relegation chain at the turnover: the final positions (`finalize_league_table`, virtual
/// `league_vtable::RECORD_FINAL_POSITIONS`) of the Allsvenskan ([`KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION`]) and the
/// Superettan ([`KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION`]), both from `G_COMP_OBJECT_TABLE` and not checked; League::promote_and_relegate (`exe::league_promote_and_relegate`(this,
/// upper, lower, 1, 1, −1, −1)) between them. Then, when the Second Division ([`KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION`])
/// exists (only with a larger database), its final positions and the exchanges of the Superettan with it and with each
/// of its five section stages (`league::STAGES`[0..4]). The argument is not read; returns the last call's eax (the
/// competition table pointer when there is no Second Division, as the original).
/// Original: `SwePrm::process_promotion_relegation_chain` (0x8907f0, swe_prm.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn swe_prm_process_promotion_relegation_chain(this: usize, _pairs: u32) -> u32 {
    let premier = comp_by_global(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION);
    finalize_league_table(League(premier));
    finalize_league_table(League(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)));
    let first = comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION);
    exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_SWEDISH_PREMIER_DIVISION), first, 1, 1, -1, -1); // cx14_9: the Allsvenskan and the Superettan
    let second = comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION);
    if second == 0 {
        covered!("cx14_9 swedish chain without the second division");
        return read_usize(G_COMP_OBJECT_TABLE) as u32;
    }
    covered!("cx14_9 swedish chain with the second division");
    finalize_league_table(League(second));
    let mut r = exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION), 1, 1, -1, -1);
    for k in 0..5 {
        let section = read_usize(League(comp_by_global(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION)).stages() + 4 * k);
        r = exe::league_promote_and_relegate(this, comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), section, 1, 1, -1, -1);
    }
    r
}

/// Comparer for `swe_prm_process_promotion_relegation_chain`: only the original runs (it moves clubs between the
/// divisions); nothing is compared.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_process_promotion_relegation_chain(this: usize, pairs: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%)(this, pairs)
}

/// The clubs of the Swedish regional sections: in database order (up to [`G_CLUB_COUNT`]), those with a division and a
/// nation (`club::NATION`) whose division's id is one of `SWEDISH_SECOND_DIVISION_SECTIONS` (the original tests North, East
/// Gotaland, West Gotaland, East Svealand, West Svealand, South Gotaland: the order does not matter); the scan stops at 72.
pub(crate) unsafe fn swedish_regional_clubs() -> Vec<usize> {
    let mut v = Vec::new();
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < read_usize(G_CLUB_COUNT) as i32 && v.len() < 0x48 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(club + club::NATION) != 0 && SWEDISH_SECOND_DIVISION_SECTIONS.iter().any(|&k| read_usize(d) == read_usize(k)) {
            v.push(club);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    v
}

/// When the Swedish Second Division is not played (the smaller databases), three clubs of its regional sections go up
/// into the Superettan and the Superettan's relegated clubs go down to the North section.
///
/// A list of 72 (`exe::crt_malloc` of 0x120; none: the box of swe_prm.cpp (0xa8b578) line 0x224 and nothing else) with
/// `swedish_regional_clubs`. With at least one: sorted with the best ones shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`
/// (list, n, 3)) and the first three moved to the Superettan (League::move_club_to_division
/// `exe::league_move_club_to_division`(this, club, its record, 1); with fewer than three clubs the original moves the
/// list's leftovers too); then each Superettan row of status 3 (in table order, the Superettan from `G_COMP_OBJECT_TABLE`)
/// goes down (League::relegate_club_to_division `exe::league_relegate_club_to_division`(this, club, the North
/// section's record, 1)). The list is freed.
/// Original: `swe_prm_promote_regional_clubs_to_first_division` (0x890980, swe_prm.cpp, thiscall: ecx = the Allsvenskan, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_prm_promote_regional_clubs_to_first_division(this: usize) {
    let first = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION));
    let list = exe::crt_malloc(0x120);
    if list == 0 {
        return show_error_at(0xa8b578, 0x224);
    }
    let clubs = swedish_regional_clubs();
    for (k, &c) in clubs.iter().enumerate() {
        write_usize(list + 4 * k, c);
    }
    if !clubs.is_empty() {
        covered!("cx14_9 swedish regional clubs up");
        exe::comp_util_sort_and_shuffle_top_clubs(list, clubs.len() as u32, 3);
        for k in 0..3 {
            exe::league_move_club_to_division(this, read_usize(list + 4 * k), comp_record_of(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION), 1);
        }
        let mut i = 0i32;
        while i < first.club_count() as i32 {
            let row = first.table() + i as usize * league_table_row::SIZE;
            if read_u8(row + league_table_row::STATUS) == 3 {
                exe::league_relegate_club_to_division(this, read_usize(row), comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH), 1); // cx14_9: down to the North section
            }
            i += 1;
        }
    }
    exe::crt_free(list);
}

/// Comparer for `swe_prm_promote_regional_clubs_to_first_division`: only the original runs (it draws); with regional
/// clubs, three of them must now be in the Superettan and the Superettan's clubs of status 3 (planned beforehand) in the
/// North section.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_promote_regional_clubs_to_first_division(this: usize) {
    let candidates = swedish_regional_clubs();
    let first = League(comp_by_global(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION));
    let down: Vec<usize> = (0..first.club_count().max(0) as usize).map(|i| first.table() + i * league_table_row::SIZE)
        .filter(|&r| read_u8(r + league_table_row::STATUS) == 3).map(|r| read_usize(r)).collect();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    if candidates.len() < 3 {
        return;
    }
    let up = candidates.iter().filter(|&&c| read_usize(c + club::DIVISION) == comp_record_of(KEY_CLUB_COMP_SWEDISH_FIRST_DIVISION)).count();
    let north = comp_record_of(KEY_CLUB_COMP_SWEDISH_SECOND_DIVISION_NORTH);
    let wrong: Vec<usize> = down.iter().copied().filter(|&c| read_usize(c + club::DIVISION) != north).collect();
    SCOREBOARD.record(up == 3 && wrong.is_empty(), || format!("swe_prm_promote_regional_clubs_to_first_division({:#x}): {} up, relegated not in the North section {:x?}", this, up, wrong));
}

/// Stores an Allsvenskan club's progress codes in the competition statistics (`exe::comp_stats_find_club_entry`, not
/// checked): the current code as it came; the best code 11 or 12 becomes 10 and the worst code 11 or 12 becomes 13
/// (the play-off codes). The stage is not read.
/// Original: `SwePrm::set_club_progress` (0x8916b0, swe_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx14_9 allsvenskan progress");
    let entry = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let play_off = |v: u8, to: u8| if v == 0xb || v == 0xc { to } else { v };
    write_progress_codes(entry, current as u8, play_off(best as u8, 0xa), play_off(worst as u8, 0xd)); // cx14_9: 10 and 13
}

/// Comparer for `swe_prm_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_swe_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || swe_prm_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("swe_prm_set_club_progress({:#x}, club {:#x}, {}, {}, {}): original {:x?} rust {:x?}", this, club, current as u8, best as u8, worst as u8, mo, mr));
}
