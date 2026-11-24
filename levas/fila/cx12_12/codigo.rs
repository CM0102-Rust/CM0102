// ---------- cx12_12: the Norwegian Premier Division club progress and the Second Division turnover steps (nor_prm.cpp) ----------

/// The club's progress codes in the Norwegian Premier Division (`premier_play_off_progress`): code 12 (the play-off
/// place) becomes 11 as the best and 14 as the worst.
/// Original: `NorPrm::set_club_progress` (0x792d40, nor_prm.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn nor_prm_set_club_progress(this: usize, club: usize, _stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx12_12 norwegian premier progress");
    premier_play_off_progress(this, club, current, best, worst, 0xc, 0xb, 0xe) // cx12_12: the play-off place
}

/// The eight groups of the Norwegian Second Division, by key.
const NORWEGIAN_SECOND_DIVISION_GROUPS: [usize; 8] = [0x9cf6c4, 0x9cf6c8, 0x9cf6cc, 0x9cf6d0, 0x9cf6d4, 0x9cf6d8, 0x9cf6dc, 0x9cf6e0];

/// Whether a division id is one of the eight Second Division groups.
unsafe fn is_norwegian_second_division(id: u32) -> bool {
    NORWEGIAN_SECOND_DIVISION_GROUPS.iter().any(|&k| read_u32(k) == id)
}

/// Promotes four Norwegian Second Division clubs and relegates the First Division's bottom clubs, at the turnover.
///
/// A list of `G_CLUB_COUNT` pointers (`exe::crt_malloc`; out of memory, the error box at line 0x24b of nor_prm.cpp) gets,
/// in club table order, every club with a division and a nation whose division is one of the eight Second Division
/// groups and whose nation is Norway (`KEY_NATION_NORWAY`). With at least one, the list is sorted and shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 4)) and its first four go to the First Division
/// (`exe::league_move_club_to_division`(this, club, the First Division's record, 1)); then every row of the First
/// Division's table with status 3 (relegated) goes to Group 1 (`exe::league_relegate_club_to_division`(this, club,
/// Group 1's record, 1)). The list is freed.
///
/// Defect of the original: with fewer than four candidates, the move reads list entries never written (the rest of the
/// allocation).
/// Original: `nor_prm_swap_first_and_second_division_clubs` (0x792960, nor_prm.cpp, called with ecx = the Premier
/// Division object and no arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_swap_first_and_second_division_clubs(this: usize) {
    let first = League(comp_object_of_key(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION));
    let list = exe::crt_malloc(read_u32(G_CLUB_COUNT).wrapping_mul(4));
    if list == 0 {
        return show_error_at(0xa6977c, 0x24b);
    }
    let mut n = 0u32;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let club = club_at(k as isize);
        let (division, nation) = (club.division().addr(), club.nation().addr());
        if division != 0 && nation != 0 && is_norwegian_second_division(read_u32(division)) && read_u32(nation) == read_u32(KEY_NATION_NORWAY) {
            write_usize(list + n as usize * 4, club.addr());
            n += 1;
        }
        k += 1;
    }
    if n as u16 != 0 {
        covered!("cx12_12 norwegian second division promotion");
        exe::comp_util_sort_and_shuffle_top_clubs(list, n, 4);
        for j in 0..4 { // cx12_12: four clubs go up
            exe::league_move_club_to_division(this, read_usize(list + j * 4), comp_record_of_key(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION), 1);
        }
        let mut row = 0i32;
        while row < first.club_count() as i32 {
            let r = LeagueTableRow(exe::league_get_table_row(first, row));
            if r.status() == 3 {
                exe::league_relegate_club_to_division(this, r.club() as usize, comp_record_of_key(NORWEGIAN_SECOND_DIVISION_GROUPS[0]), 1);
            }
            row += 1;
        }
    }
    exe::crt_free(list);
}

/// The Second Division group of the k-th club from the north: blocks of twelve, Group 1 for the first twelve and so on,
/// Group 8 for the 85th on.
fn norwegian_second_division_group(k: i16) -> usize {
    NORWEGIAN_SECOND_DIVISION_GROUPS[(k / 12).clamp(0, 7) as usize]
}

/// The Norwegian Second Division clubs, in club table order (the index a word compared with `G_CLUB_COUNT`, re-read
/// each turn; nothing when the count is not positive), whose `club::DIVISION` is one of the eight group records, up to
/// 96 (the count a signed byte), sorted by their city's latitude (`exe::crt_qsort` with the comparer 0x53fd40).
unsafe fn norwegian_second_division_clubs() -> ([usize; 0x60], i8) {
    let records = NORWEGIAN_SECOND_DIVISION_GROUPS.map(|k| comp_record_of_key(k));
    let mut list = [0usize; 0x60];
    let mut n = 0i8;
    if read_i32(G_CLUB_COUNT) > 0 {
        let mut k = 0i16;
        while n < 0x60 {
            let club = club_at(k as isize);
            if records.contains(&club.division().addr()) {
                list[n as usize] = club.addr();
                n += 1;
            }
            k = k.wrapping_add(1);
            if k as i32 >= read_i32(G_CLUB_COUNT) {
                break;
            }
        }
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x53fd40usize);
    (list, n)
}

/// Regroups the Norwegian Second Division at the turnover: its clubs (`norwegian_second_division_clubs`, up to 96,
/// sorted by their city's latitude) go to the eight groups in blocks of twelve (`norwegian_second_division_group`), each
/// club's `club::DIVISION` set to its group's competition record.
/// Original: `nor_prm_regroup_second_division_clubs` (0x792b40, nor_prm.cpp, called with ecx = the Premier Division
/// object and no arguments, ret 0).
pub(crate) unsafe extern "thiscall" fn nor_prm_regroup_second_division_clubs(_this: usize) {
    let (list, n) = norwegian_second_division_clubs();
    covered!("cx12_12 norwegian second division regrouping");
    for k in 0..n as i16 {
        Club(list[k as usize]).set_division(Competition(comp_record_of_key(norwegian_second_division_group(k)))); // cx12_12: by latitude
    }
}

/// Comparer for `nor_prm_set_club_progress`: side by side on the 3 bytes of the club's entry (`compare_c1_club_progress`).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_c1_club_progress("nor_prm_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || nor_prm_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparer for `nor_prm_swap_first_and_second_division_clubs`: only the original (it moves clubs after a shuffle).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_swap_first_and_second_division_clubs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparer for `nor_prm_regroup_second_division_clubs`: only the original (it moves the clubs); the plan, made before
/// (the same sorted list), must match every club's new division.
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_regroup_second_division_clubs(this: usize) {
    let (list, n) = norwegian_second_division_clubs();
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let wrong: Vec<(usize, usize)> = (0..n.max(0) as i16)
        .filter(|&k| Club(list[k as usize]).division().addr() != comp_record_of_key(norwegian_second_division_group(k)))
        .map(|k| (list[k as usize], Club(list[k as usize]).division().addr())).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("nor_prm_regroup_second_division_clubs: {} clubs, (club, division) out of plan {:x?}", n, wrong));
}
