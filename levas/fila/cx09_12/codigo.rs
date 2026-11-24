// ---------- cx09_12: the Belgian Third Division table, rules and predicted positions (bel_third.cpp) ----------

/// The Belgian Third Division group size.
const BEL_THIRD_GROUP_SIZE: i16 = 0x10;

/// Whether a club belongs in the Belgian Third Division group A table: its `club::DIVISION` is the Third Division
/// ([`KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION`]) and its `club::RESERVE_DIVISION` is Third Division A.
unsafe fn bel_third_in_group_a(club: Club) -> bool {
    let (division, reserve) = (club.division().addr(), club.reserve_division().addr());
    division != 0 && reserve != 0 && read_u32(reserve) == read_u32(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A)
        && read_u32(division) == read_u32(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION)
}

/// The number of club records scanned by the Belgian Third Division: `G_CLUB_COUNT` − 2 × `G_NATION_COUNT`, read
/// again at each step as the original does.
unsafe fn bel_third_club_scan_count() -> i32 {
    read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT)
}

/// Fills the Belgian Third Division group A table: `league::CLUB_COUNT` = 16. In the 2001 season (`league::YEAR` =
/// 2001) Third Divisions A and B become one: every club whose `club::DIVISION` is A or B keeps it in
/// `club::RESERVE_DIVISION` and gets this competition as its division. The table (`exe::crt_malloc` of 16 × 0x3b into
/// `league::TABLE`; none: the bel_third.cpp box of line 0x22e and 0) takes, in club order, the clubs of group A
/// (`bel_third_in_group_a`) through League::init_table_row (`exe::league_init_table_row`(row, club, 0, 0)); a 17th
/// club shows the box of line 0x23f and loses its division (`club::DIVISION` = 0, a defect of the original). Returns 1
/// when the table is full.
/// Original: `BelThird::fill_league_table` (0x4224e0, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_fill_league_table(this: usize) -> u32 {
    let l = League(this);
    l.set_club_count(BEL_THIRD_GROUP_SIZE);
    if l.year() as u16 == 0x7d1 {
        let mut i = 0i32;
        while i < bel_third_club_scan_count() {
            let club = club_at(i as isize);
            let division = club.division().addr();
            if division != 0 && (read_u32(division) == read_u32(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_A) || read_u32(division) == read_u32(KEY_CLUB_COMP_BELGIAN_THIRD_DIVISION_B)) {
                covered!("cx09_12 belgian third division merged");
                club.set_reserve_division(club.division());
                club.set_division(l.competition());
            }
            i += 1;
        }
    }
    let table = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(league_table_row::SIZE as u32));
    l.set_table(table);
    if table == 0 {
        show_error_at(BEL_THIRD_SOURCE, 0x22e);
        return 0;
    }
    let mut row: u8 = 0;
    let mut i = 0i32;
    while i < bel_third_club_scan_count() {
        let club = club_at(i as isize);
        if bel_third_in_group_a(club) {
            if row as i8 as i16 >= l.club_count() {
                show_error_at(BEL_THIRD_SOURCE, 0x23f);
                write_usize(club.addr() + club::DIVISION, 0);
                break;
            }
            exe::league_init_table_row(this, row as u32, club.addr(), 0, 0);
            row = row.wrapping_add(1);
        }
        i += 1;
    }
    (row as i8 as i16 == l.club_count()) as u32
}

/// The rule writes of the Belgian Third Division (offset, size, value): two legs; 3 points for a win, 1 for a draw,
/// match category 2; club sides; tie-breakers 3, 1, 2, 0; places 1, 0, 1, 2; the six period flags cleared; the Second
/// Division above and no division below; 4 substitutes named and 3 substitutions; rule bits 0x43.
unsafe fn bel_third_rules() -> Vec<(usize, u8, u32)> {
    vec![(0x3c, 2, 2), (0xc2, 1, 3), (0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 3), (0xc6, 1, 1), (0xc7, 1, 2), (0xc8, 1, 0),
         (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 1), (0xc1, 1, 2), (0x102, 1, 0), (0x100, 1, 0), (0xfe, 1, 0), (0x103, 1, 0), (0x101, 1, 0),
         (0xff, 1, 0), (0x1c, 4, read_u32(KEY_CLUB_COMP_BELGIAN_SECOND_DIVISION)), (0x4a, 1, 3), (0x20, 4, 0xffff_ffff), (0x49, 1, 4),
         (0xd9, 2, 0x43)]
}

/// Sets up the Belgian Third Division rules (`bel_third_rules`), then builds the main-stage round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (−1, &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0) into `league::ROUND_DATES`
/// (`fra_first_apply_league_rules`). Returns 1.
/// Original: `BelThird::set_up_rules` (0x4226f0, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_set_up_rules(this: usize) -> u32 {
    covered!("cx09_12 belgian third division rules");
    fra_first_apply_league_rules(League(this), &bel_third_rules())
}

/// The predicted positions of a league played in two groups, given to `emit` as (club, position): for group A (this
/// league's table) and then group B (stage 0's), the clubs in row order (the league's `league::CLUB_COUNT` of them, or
/// each group's own with `own_count`), sorted by qsort with the strength order 0x4b07d0, the k-th getting
/// `step` x k + 1 (a byte), each group emitted right after its sort. `list` is the scratch list.
unsafe fn two_group_predicted_positions(l: League, list: usize, own_count: bool, step: u8, mut emit: impl FnMut(usize, u8)) {
    for g in [-1i8, 0] {
        let group = if g == -1 { l } else { League(read_usize(l.stages() + 4 * g as usize)) };
        let n = if own_count { group.club_count() } else { l.club_count() };
        let mut k: i16 = 0;
        while k < n {
            write_usize(list + 4 * k as usize, LeagueTableRow(group.table() + k as usize * league_table_row::SIZE).club() as usize);
            k += 1;
        }
        exe::crt_qsort(list, n as i32 as u32, 4, 0x4b07d0usize);
        let mut k: i16 = 0;
        while k < n {
            emit(read_usize(list + 4 * k as usize), (k as u8).wrapping_mul(step).wrapping_add(1));
            k += 1;
        }
    }
}

/// Gives the Belgian Third Division clubs their predicted positions (`two_group_predicted_positions`: the league's
/// club count for both groups, 3k + 1), stored through
/// Comp_Stats::set_club_final_position (`exe::comp_stats_set_club_final_position`). Without statistics
/// (`league::COMP_STATS`): the bel_third.cpp box of line 0x6bf; without memory for the list (malloc of the club count ×
/// 4): the box of line 0x6c7. The list is freed.
/// Original: `BelThird::set_predicted_positions` (0x424030, bel_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bel_third_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x6bf);
    }
    let list = exe::crt_malloc((l.club_count() as i32 as u32).wrapping_mul(4));
    if list == 0 {
        return show_error_at(BEL_THIRD_SOURCE, 0x6c7);
    }
    covered!("cx09_12 belgian third division predicted positions");
    two_group_predicted_positions(l, list, false, 3, |club, position| {
        exe::comp_stats_set_club_final_position(l.comp_stats(), club, position as u32);
    });
    exe::crt_free(list);
}

/// Comparer for `bel_third_fill_league_table`: only the original (it may merge the divisions); afterwards the table
/// must hold, in order, the clubs of group A as the copy picks them, and the return must match.
pub(crate) unsafe extern "thiscall" fn compare_bel_third_fill_league_table(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%)(this);
    let l = League(this);
    if l.table() == 0 {
        return r;
    }
    let picked: Vec<usize> = (0..bel_third_club_scan_count()).map(|i| club_at(i as isize)).filter(|&c| bel_third_in_group_a(c)).map(|c| c.addr()).take(l.club_count() as usize).collect();
    let rows: Vec<usize> = (0..picked.len()).map(|k| LeagueTableRow(l.table() + k * league_table_row::SIZE).club() as usize).collect();
    let full = (picked.len() == l.club_count() as usize) as u32;
    SCOREBOARD.record(rows == picked && r == full, || format!("bel_third_fill_league_table({:#x}): returned {} copy {}, table {:x?} copy {:x?}", this, r, full, rows, picked));
    r
}

/// Comparer for `bel_third_set_up_rules`: a dry run (`compare_nation_league_rules`).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_nation_league_rules("bel_third_set_up_rules", League(this), bel_third_rules(), || o(this), || bel_third_set_up_rules(this))
}

/// Comparer for `bel_third_set_predicted_positions`: only the original (it writes the statistics); afterwards each club's
/// `comp_stats_club::FINAL_POSITION` must be the copy's (its sort on a local list).
pub(crate) unsafe extern "thiscall" fn compare_bel_third_set_predicted_positions(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let l = League(this);
    if l.comp_stats() == 0 || l.club_count() <= 0 {
        return;
    }
    let mut list = vec![0usize; l.club_count() as usize];
    let mut plan = Vec::new();
    two_group_predicted_positions(l, list.as_mut_ptr() as usize, false, 3, |c, p| plan.push((c, p)));
    let wrong: Vec<(usize, u8, u8)> = plan.iter().filter_map(|&(c, p)| {
        let e = exe::comp_stats_find_club_entry(l.comp_stats(), c);
        let got = if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 };
        (got != p).then_some((c, p, got))
    }).collect();
    SCOREBOARD.record(wrong.is_empty(), || format!("bel_third_set_predicted_positions({:#x}): (club, copy, original) {:x?}", this, wrong));
}
