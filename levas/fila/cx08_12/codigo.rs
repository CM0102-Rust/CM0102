// ---------- cx08_12: the Argentine Second Division predicted positions and club progress (arg_second.cpp) ----------

/// The predicted place of each of the 25 Argentine Second Division clubs by strength: the strongest 1, then shared
/// places in threes (and the fifth alone at 4).
const ARG_SECOND_PREDICTED: [u32; 25] = [1, 2, 2, 2, 4, 6, 6, 8, 8, 8, 0xb, 0xb, 0xb, 0xe, 0xe, 0xe, 0x11, 0x11, 0x11, 0x14, 0x14, 0x14, 0x17, 0x17, 0x17];

/// Rule: the Argentine Second Division's predicted positions: the 25 clubs sorted by strength get the places 1, 2, 2,
/// 2, 4, 6, 6, 8, 8, 8 and then threes at 11, 14, 17, 20 and 23.
///
/// `ArgSecond::set_predicted_positions()`: without statistics (`league::COMP_STATS`) the arg_second.cpp box of line
/// 0x79e and nothing else; the list (`exe::crt_malloc` of 0x64; none: the box of line 0x7a6) with the table's clubs in
/// row order (`league::CLUB_COUNT`), sorted (`exe::comp_util_sort_clubs_by_strength`(list, 25)); each of the 25 gets its
/// place of `ARG_SECOND_PREDICTED` (Comp_Stats::set_club_final_position `exe::comp_stats_set_club_final_position`);
/// the list freed.
/// Original: `ArgSecond::set_predicted_positions` (0x409fe0, arg_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn arg_second_set_predicted_positions(this: usize) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x79e);
    }
    let list = exe::crt_malloc(0x64);
    if list == 0 {
        return show_error_at(ARG_SECOND_FILE, 0x7a6);
    }
    let mut k: i16 = 0;
    while k < l.club_count() {
        write_usize(list + 4 * k as usize, read_usize(l.table() + k as usize * league_table_row::SIZE));
        k += 1;
    }
    exe::comp_util_sort_clubs_by_strength(list, 0x19);
    covered!("cx08_12 argentine second predicted positions");
    for (k, &place) in ARG_SECOND_PREDICTED.iter().enumerate() {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(list + 4 * k), place);
    }
    exe::crt_free(list);
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each club of the table in the statistics, 0 without
/// an entry.
unsafe fn table_predicted_positions(l: League) -> Vec<u8> {
    (0..l.club_count().max(0) as usize).map(|k| {
        let e = exe::comp_stats_find_club_entry(l.comp_stats(), read_usize(l.table() + k * league_table_row::SIZE));
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// Comparer for `arg_second_set_predicted_positions`: the original runs, then the copy writes again over it; each
/// club's predicted position must come out the same.
pub(crate) unsafe extern "thiscall" fn compare_arg_second_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let l = League(this);
    o(this);
    if l.comp_stats() == 0 || l.table() == 0 {
        return;
    }
    let original = table_predicted_positions(l);
    arg_second_set_predicted_positions(this);
    let copy = table_predicted_positions(l);
    SCOREBOARD.record(copy == original, || format!("arg_second_set_predicted_positions({:#x}): copy {:?} original {:?}", this, copy, original));
}

/// Rule: the Argentine Second Division progress codes: in the first phase and the zones (stages 0 to 2) each code is
/// multiplied by three (a byte); in the promotion cup (stage 3) the best code is 1 and the current and worst codes 1,
/// 2, 3, 4 become 1, 2, 5, 8; elsewhere as they come.
///
/// `ArgSecond::set_club_progress(club, stage, current, best, worst)`: the club's entry (`exe::comp_stats_find_club_entry`,
/// unchecked); the stage byte (signed) 0 to 2: × 3; 3: best = 1 and the jump tables 0x40a334 (current) and 0x40a324
/// (worst) for codes 1 to 4; below 0 or above 3, as they come. The bytes go to `comp_stats_club::CURRENT_POSITION`,
/// `BEST_POSITION` and `WORST_POSITION`.
/// Original: `ArgSecond::set_club_progress` (0x40a240, arg_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn arg_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let e = exe::comp_stats_find_club_entry(League(this).comp_stats(), club);
    let (mut c, mut b, mut w) = (current as u8, best as u8, worst as u8);
    let cup = |v: u8| if (1..=4).contains(&v) { [1, 2, 5, 8][v as usize - 1] } else { v };
    match stage as u8 as i8 {
        0..=2 => {
            covered!("cx08_12 argentine second phase progress");
            (c, b, w) = (c.wrapping_mul(3), b.wrapping_mul(3), w.wrapping_mul(3));
        }
        3 => {
            covered!("cx08_12 argentine second cup progress");
            (c, b, w) = (cup(c), 1, cup(w));
        }
        _ => {}
    }
    write_u8(e + comp_stats_club::CURRENT_POSITION, c);
    write_u8(e + comp_stats_club::BEST_POSITION, b);
    write_u8(e + comp_stats_club::WORST_POSITION, w);
}

/// Comparer for `arg_second_set_club_progress`: side by side on the 3 bytes (`compare_group_championship_progress`).
pub(crate) unsafe extern "thiscall" fn compare_arg_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_group_championship_progress("arg_second_set_club_progress", this, club, || o(this, club, stage, current, best, worst),
        || arg_second_set_club_progress(this, club, stage, current, best, worst));
}
