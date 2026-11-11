// ---------- %%LEVA%%a leva: as posições previstas e o sorteio dos grupos do Campeonato de Clubes da Oceania; as posições previstas da Copa Africana de Nações e da eliminatória da Oceania (oceania_club_champ.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

/// The positions in the order of the array sorted by reputation (`exe::comp_stats_set_club_final_position`: push
/// position; push club; mov ecx, stats (`league::COMP_STATS`) -> thiscall, ret 8).
unsafe fn predicted_by_reputation_write(this: usize, v: usize, positions: &[u32]) {
    for (k, &p) in positions.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + league::COMP_STATS), read_usize(v + k * 4), p);
    }
}

/// `exe::crt_qsort`(v, n, 4, 0x4b07d0) (4 pushes; add esp -> cdecl): the reputation (`club::REPUTATION`) descending,
/// nulls at the end.
unsafe fn predicted_by_reputation_sort(v: usize, n: u32) {
    exe::crt_qsort(v, n, 4, 0x4b07d0);
}

/// The predicted positions of the 8 Oceania Club Championship entrants, by reputation.
const OCEANIA_CLUB_CHAMP_POSITIONS: [u32; 8] = [1, 2, 3, 3, 5, 5, 7, 7];

/// The predicted positions of the Oceania Club Championship and its group draw: without statistics
/// (`league::COMP_STATS`), the box for line 0x3a0; otherwise the 8 chosen clubs (+0xee, +0xf2 clubs) sorted by
/// reputation (`predicted_by_reputation_sort`), the positions 1, 2, 3, 3, 5, 5, 7, 7 and the list shuffled
/// (`exe::comp_util_shuffle_pointer_array`(list, n): push n (word); push list; add esp, 8 -> cdecl: 3n drawn swaps).
/// That shuffle is what makes the groups: the first 4 go to group A, the others to B.
/// Original: `OceaniaClubChamp::set_predicted_positions` (0x798640, oceania_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn oceania_club_champ_set_predicted_positions(this: usize) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(0xa6a068, 0x3a0);
    }
    let list = read_usize(this + 0xee);
    predicted_by_reputation_sort(list, read_usize(this + 0xf2) as u32); // cqv13: the 8 by reputation
    predicted_by_reputation_write(this, read_usize(this + 0xee), &OCEANIA_CLUB_CHAMP_POSITIONS);
    covered!("cqv_13 oceania group draw");
    let n = read_i16(this + 0xf2) as u16 as u32;
    exe::comp_util_shuffle_pointer_array(read_usize(this + 0xee), n); // cqv13: the 8 shuffled
}

/// The plan of the predicted positions: the clubs `clubs` (taken before the original), each with the position written
/// (`comp_stats_club::FINAL_POSITION` of its entry, `exe::comp_stats_find_club_entry`) after it; by reputation
/// (`club::REPUTATION`) descending the positions make `positions` (reputation ties may swap with each other: the qsort
/// is not stable). Only with as many clubs as positions and none null.
unsafe fn predicted_by_reputation_check(name: &str, this: usize, clubs: &[usize], positions: &[u32]) {
    let stats = read_usize(this + league::COMP_STATS);
    if stats == 0 || clubs.len() != positions.len() || clubs.iter().any(|&c| c == 0) {
        return;
    }
    let mut v: Vec<(i16, u8)> = clubs
        .iter()
        .map(|&c| {
            let e = exe::comp_stats_find_club_entry(stats, c);
            (Club(c).reputation(), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
        })
        .collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let read: Vec<u32> = v.iter().map(|x| x.1 as u32).collect();
    SCOREBOARD.record(read == positions, || format!("{}({:#x}): positions by reputation {:?}", name, this, read));
}

// simplification: Comp_Stats::set_club_final_position adds entries and the shuffle draws: it cannot run twice. Only the
// original; the plan: the positions by reputation and the list after it a permutation of the one before.
unsafe extern "thiscall" fn compare_oceania_club_champ_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let list = read_usize(this + 0xee);
    let n = read_i16(this + 0xf2);
    if list == 0 || !(0..=64).contains(&n) {
        return o(this);
    }
    let mut before: Vec<usize> = (0..n as usize).map(|k| read_usize(list + k * 4)).collect();
    o(this);
    let mut after: Vec<usize> = (0..n as usize).map(|k| read_usize(read_usize(this + 0xee) + k * 4)).collect();
    predicted_by_reputation_check("oceania_club_champ_set_predicted_positions", this, &before, &OCEANIA_CLUB_CHAMP_POSITIONS);
    before.sort_unstable();
    after.sort_unstable();
    SCOREBOARD.record(before == after, || format!("oceania_club_champ_set_predicted_positions({:#x}): the shuffled list is not the same", this));
}

/// The body of the predicted positions with a copy (0x4027d0 and 0x91ff30): without statistics (`league::COMP_STATS`),
/// the box `rows.0`; a copy of the `n` pointers of the list (`exe::crt_malloc`(4n): push; add esp, 4 -> cdecl; out of
/// memory, the box `rows.1`), sorted by reputation (`predicted_by_reputation_sort`); the positions in that order; the
/// copy freed (`exe::crt_free`).
unsafe fn predicted_by_reputation_of_copy(this: usize, list: usize, n: i32, positions: &[u32], file: usize, rows: (u32, u32)) {
    if read_usize(this + league::COMP_STATS) == 0 {
        return show_error_at(file, rows.0);
    }
    let v = exe::crt_malloc((n as u32).wrapping_mul(4));
    if v == 0 {
        return show_error_at(file, rows.1);
    }
    core::ptr::copy_nonoverlapping(list as *const u8, v as *mut u8, (n as u32).wrapping_mul(4) as usize);
    predicted_by_reputation_sort(v, n as u32);
    predicted_by_reputation_write(this, v, positions);
    exe::crt_free(v);
}

/// The predicted positions of the 16 African Nations Cup finalists, by reputation.
const AFRICAN_NATIONS_POSITIONS: [u32; 16] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13];

/// The predicted positions of the African Nations Cup: the 16 finalists (+0xee, +0xf2 word) by reputation
/// (`predicted_by_reputation_of_copy`): 1, 2, 3, 4, four 5s, four 9s and four 13s (boxes at lines 0x3f4 and 0x3fc).
/// Original: `AfricanNations::set_predicted_positions` (0x4027d0, african_nations.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn african_nations_set_predicted_positions(this: usize) {
    predicted_by_reputation_of_copy(this, read_usize(this + 0xee), read_i16(this + 0xf2) as i32, &AFRICAN_NATIONS_POSITIONS, 0x9870f8, (0x3f4, 0x3fc))
}

// simplification: Comp_Stats::set_club_final_position adds entries: it cannot run twice. Only the original; the plan:
// the positions by reputation.
unsafe extern "thiscall" fn compare_african_nations_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (list, n) = (read_usize(this + 0xee), read_i16(this + 0xf2));
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    predicted_by_reputation_check("african_nations_set_predicted_positions", this, &clubs, &AFRICAN_NATIONS_POSITIONS);
}

/// The predicted positions of the Oceania World Cup qualifier, by strength (9 or 10 nations).
const WC_OCEANIA_LEAGUE_POSITIONS: [u32; 10] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 10];

/// The predicted positions of the Oceania World Cup qualifier: its national teams (+0xef, +0xf3 dword) by strength (the
/// reputation `club::REPUTATION` of the national teams' clubs, `predicted_by_reputation_of_copy`): 1, 2, 3, 3, 5, 5, 7,
/// 7, 9 and, if there are 10, the tenth 10 (boxes at lines 0x52a and 0x532).
/// Original: `WcOceaniaLeague::set_predicted_positions` (0x91ff30, wc_oceania_league.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn wc_oceania_league_set_predicted_positions(this: usize) {
    let n = read_usize(this + 0xf3) as i32;
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_of_copy(this, read_usize(this + 0xef), n, &WC_OCEANIA_LEAGUE_POSITIONS[..k], 0xad4afc, (0x52a, 0x532))
}

unsafe extern "thiscall" fn compare_wc_oceania_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (list, n) = (read_usize(this + 0xef), read_usize(this + 0xf3) as i32);
    let clubs: Vec<usize> = if list != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| read_usize(list + k * 4)).collect() } else { Vec::new() };
    o(this);
    let k = if n == 10 { 10 } else { 9 };
    predicted_by_reputation_check("wc_oceania_league_set_predicted_positions", this, &clubs, &WC_OCEANIA_LEAGUE_POSITIONS[..k]);
}
