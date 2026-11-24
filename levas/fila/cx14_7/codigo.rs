// ---------- cx14_7: the Turkish category B play-off and the regional groups of the turnover (tur_second_b.cpp, tur_first.cpp) ----------

/// The clubs of the category B play-off: the clubs of the promotion group's rows of status 1 (stage 4, in row order),
/// then the first club of each classification group (stages 5 to 9); the first eight and how many more.
pub(crate) unsafe fn category_b_play_off_clubs(l: League) -> (Vec<usize>, usize) {
    let promotion = League(read_usize(l.stages() + 4 * 4));
    let mut v: Vec<usize> = (0..promotion.club_count().max(0) as usize)
        .map(|i| promotion.table() + i * league_table_row::SIZE)
        .filter(|&r| read_u8(r + league_table_row::STATUS) == 1)
        .map(|r| read_usize(r))
        .collect();
    for g in 5..10usize {
        v.push(aus_nsl_table_row_club(League(read_usize(l.stages() + 4 * g)), 0)); // cx14_7: each classification group winner
    }
    let more = v.len().saturating_sub(8);
    v.truncate(8);
    (v, more)
}

/// Builds the category B play-off as stage 10: the clubs of `category_b_play_off_clubs` in a local list of eight (one
/// past the eighth from the promotion group: the box at line 0x37e; from a classification group: line 0x389); the
/// round table of stage 10 (`stage_round_table`); a Cup_Stage (operator new of 0xb2; Cup_Stage::Cup_Stage
/// `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c: this league, 8, the list, the rounds, the competition, the round
/// table, the year, 10, 1, the extra, 0x14, 0, 0, 0, 0) into slot 10; none: the box at line 0x395 (the round table not
/// freed). Otherwise the round table freed, the first round drawn (Cup::set_up_round `exe::cup_set_up_round` with 0)
/// and the current stage = 10. With fewer than eight clubs the original passes stack leftovers in the empty slots and
/// the copy zeros (never seen: three from the promotion group and five group winners).
/// Original: `TurSecondB::create_playoff_stage` (0x901040, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_create_playoff_stage(this: usize) {
    let l = League(this);
    let (clubs, more) = category_b_play_off_clubs(l);
    let promotion_through = {
        let promotion = League(read_usize(l.stages() + 4 * 4));
        (0..promotion.club_count().max(0) as usize).filter(|&i| read_u8(promotion.table() + i * league_table_row::SIZE + league_table_row::STATUS) == 1).count()
    };
    for k in 0..more {
        // the extra clubs come from the promotion group first, then from the classification groups
        show_error_at(TURKISH_CATEGORY_B_SOURCE, if 8 + k < promotion_through { 0x37e } else { 0x389 });
    }
    let mut list = [0usize; 8];
    list[..clubs.len()].copy_from_slice(&clubs);
    let (table, rounds, extra) = stage_round_table(l, 10);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year as dwords with stack leftovers above the
    // word; the constructor reads the words.
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this, 8, list.as_ptr() as usize, rounds as u16 as u32, l.competition().addr(), table, l.year() as u16 as u32, 10, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(l.stages() + 4 * 10, stage);
    if read_usize(l.stages() + 4 * 10) == 0 {
        return show_error_at(TURKISH_CATEGORY_B_SOURCE, 0x395);
    }
    covered!("cx14_7 category b play-off");
    exe::crt_free(table);
    exe::cup_set_up_round(read_usize(l.stages() + 4 * 10), 0);
    l.set_current_stage(10);
}

/// Comparer for `tur_second_b_create_playoff_stage`: only the original runs (it allocates and draws); the eight clubs
/// of the stage's team table (6-byte entries) against the plan, as a set, and the current stage.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_create_playoff_stage(this: usize) {
    let l = League(this);
    let (plan, _) = category_b_play_off_clubs(l);
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let stage = Cup(read_usize(l.stages() + 4 * 10));
    let mut found: Vec<usize> = if stage.is_null() { Vec::new() } else { (0..stage.team_count().max(0) as usize).map(|k| read_usize(stage.team_table() + 6 * k)).collect() };
    let mut want = plan.clone();
    found.sort_unstable();
    want.sort_unstable();
    SCOREBOARD.record(found == want && l.current_stage() == 10, || format!("tur_second_b_create_playoff_stage({:#x}) current {}: plan {:x?} found {:x?}", this, l.current_stage() as i32, want, found));
}

/// The clubs of category B (`club::DIVISION` with the id of [`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B`]) in
/// database order (index a 16-bit counter, up to [`G_CLUB_COUNT`]), at most 50: the scan stops at the fiftieth.
pub(crate) unsafe fn category_b_clubs_for_regions() -> Vec<usize> {
    let mut v = Vec::new();
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while (i as i16 as i32) < read_usize(G_CLUB_COUNT) as i32 && v.len() < 0x32 {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B) {
            v.push(club);
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    v
}

/// Deals the clubs of category B into its five regional groups at the turnover: the clubs of
/// `category_b_clubs_for_regions` in a local list, sorted (qsort `exe::crt_qsort` with 0x53fe50, cdecl; [inference: by
/// the city's longitude, the comparer reads a double]) and given, ten at a time in that order, the reserve division
/// (`club::RESERVE_DIVISION`) of group 1 to group 5 (the records of [`KEY_CLUB_COMP_TURKISH_2_DIVISION_CATEGORY_B_G1`] to
/// `_G5`, `comp_record_of`, taken at the start); the 41st on all go to group 5.
/// Original: `tur_first_assign_third_tier_groups` (0x8fe580, tur_first.cpp, thiscall: ecx = the First Division, not read, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_first_assign_third_tier_groups(_this: usize) {
    covered!("cx14_7 turkish regional groups");
    let groups: Vec<usize> = TURKISH_CATEGORY_B_GROUPS.iter().map(|&g| comp_record_of(g)).collect();
    let mut list = category_b_clubs_for_regions();
    exe::crt_qsort(list.as_mut_ptr() as usize, list.len() as u32, 4, 0x53fe50);
    for (k, &club) in list.iter().enumerate() {
        let g = (k / 10).min(4); // cx14_7: ten clubs a group
        write_usize(club + club::RESERVE_DIVISION, groups[g]);
    }
}

/// Comparer for `tur_first_assign_third_tier_groups`: side by side over the reserve division of the category B clubs
/// (it only writes them, from the division and the cities: it can run twice).
pub(crate) unsafe extern "thiscall" fn compare_tur_first_assign_third_tier_groups(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let regions: Vec<(usize, usize)> = category_b_clubs_for_regions().iter().map(|&c| (c + club::RESERVE_DIVISION, 4)).collect();
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&regions, || o(this), || tur_first_assign_third_tier_groups(this));
    SCOREBOARD.record(same, || format!("tur_first_assign_third_tier_groups original={:x?} rust={:x?}", mo, mr));
}
