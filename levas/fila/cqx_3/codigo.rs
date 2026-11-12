// ---------- batch %%LEVA%%: the predicted positions of the state leagues: Bahia state league, Centre-West league, Rio Grande do Sul state league (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp) ----------

/// One Brazilian state league's predicted positions: the source file of the error boxes, their two lines (no
/// statistics, no memory), the fixed places of the first clubs in the order (the knockout) and the counter.
struct StateLeaguePredicted {
    file: usize,
    lines: [u32; 2],
    places: &'static [u32],
    label: &'static str,
}

/// The predicted positions of a Brazilian state league (the same mould of 336 to 432 bytes in the twelve, checked
/// instruction by instruction: only the source file, the box lines and the fixed knockout places change; the
/// quarter-final ones (8 places) run the tail with an index and a position, the others with the position only: the
/// same clubs and places).
///
/// `set_predicted_positions()` (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, from the constructor and the season
/// turnover). Without statistics (`league::COMP_STATS`): the box of the first line and return. The clubs of the
/// `league::CLUB_COUNT` rows of the table (`league::TABLE`, `league_table_row::SIZE` bytes each) go to an array
/// (`exe::crt_malloc`(4 × n): push; add esp, 4 -> cdecl; out of memory: the box of the second line and return),
/// sorted by strength (`exe::comp_util_sort_clubs_by_strength`(array, n): push n; push array; add esp, 8 -> cdecl, the
/// strongest first); the first ones get the fixed places without looking at how many clubs there are, and the next
/// ones, while there is a club (k up to n), place k (Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, club, place): push place; push club; mov ecx, stats -> thiscall,
/// ret 8); the array is freed (`exe::crt_free`).
unsafe fn state_league_predicted_positions(this: usize, s: &StateLeaguePredicted) {
    let l = League(this);
    if l.comp_stats() == 0 {
        return show_error_at(s.file, s.lines[0]);
    }
    let v = exe::crt_malloc(((l.club_count() as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(s.file, s.lines[1]);
    }
    let mut i = 0i16;
    while i < l.club_count() {
        write_usize(v + 4 * i as usize, read_usize(l.table() + league_table_row::SIZE * i as usize));
        i += 1;
    }
    exe::comp_util_sort_clubs_by_strength(v, l.club_count() as i32); // cqx3: the order by strength
    for (k, &p) in s.places.iter().enumerate() {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(v + 4 * k), p);
    }
    covered!(s.label);
    let mut k = s.places.len() as i32 + 1;
    while k <= l.club_count() as i32 {
        exe::comp_stats_set_club_final_position(l.comp_stats(), read_usize(v + 4 * (k - 1) as usize), k as u32);
        k += 1;
    }
    exe::crt_free(v);
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each club in the statistics `stats`, 0 without
/// entry (Comp_Stats::find_club_entry `exe::comp_stats_find_club_entry`).
unsafe fn state_league_positions(stats: usize, clubs: &[usize]) -> Vec<u8> {
    clubs.iter().map(|&c| {
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The comparer of the state leagues' predicted positions, side by side over the same entries: the original first
/// (Comp_Stats::set_club_final_position adds the entry of a club that has none), then the copy, which finds the entries
/// already there and only rewrites `comp_stats_club::FINAL_POSITION` (the same table, the same sort by strength: the
/// same order, ties included); the position of each club of the table after the copy must equal the original's. With
/// fewer clubs than fixed places (the original reads past the array), without statistics or table: only the original.
unsafe fn compare_state_league_predicted(name: &str, this: usize, s: &StateLeaguePredicted, o: impl FnOnce()) {
    let l = League(this);
    let n = l.club_count() as i32;
    if l.comp_stats() == 0 || l.table() == 0 || n < s.places.len() as i32 {
        return o();
    }
    let clubs: Vec<usize> = (0..n as usize).map(|i| read_usize(l.table() + league_table_row::SIZE * i)).collect();
    o();
    let original = state_league_positions(l.comp_stats(), &clubs);
    state_league_predicted_positions(this, s);
    let copy = state_league_positions(l.comp_stats(), &clubs);
    SCOREBOARD.record(copy == original, || format!("{}({:#x}): copy {:?} original {:?}", name, this, copy, original));
}

/// The Bahia state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const BAHIA_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98acac, lines: [0x307, 0x30f], places: &[1, 2, 3, 3], label: "cqx_3 bahia predicted" }; // cqx3: the bahia places

/// The predicted positions of the Bahia state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `BAHIA_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x307 (no statistics) and 0x30f (no memory).
/// Original: `BraRegBahia::set_predicted_positions` (0x430340, bra_reg_bahia.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_bahia_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &BAHIA_PREDICTED)
}

/// Comparer for `bra_reg_bahia_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_bahia_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_state_league_predicted("bra_reg_bahia_set_predicted_positions", this, &BAHIA_PREDICTED, || o(this));
}

/// The Centre-West league: the fixed places 1, 2, 3, 3, 5, 5, 5, 5 (the quarter-finals).
const CENTRAL_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae10, lines: [0x30c, 0x314], places: &[1, 2, 3, 3, 5, 5, 5, 5], label: "cqx_3 centre-west predicted" }; // cqx3: the centre-west places

/// The predicted positions of the Centre-West league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, 5, 5, 5, 5, and the others their place in the order (`state_league_predicted_positions`
/// with `CENTRAL_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x30c (no statistics) and 0x314 (no memory).
/// Original: `BraRegCentral::set_predicted_positions` (0x431610, bra_reg_central.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_central_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &CENTRAL_PREDICTED)
}

/// Comparer for `bra_reg_central_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_central_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_state_league_predicted("bra_reg_central_set_predicted_positions", this, &CENTRAL_PREDICTED, || o(this));
}

/// The Rio Grande do Sul state league: the fixed places 1, 2, 3, 3 (the semi-finals).
const GAUCHO_PREDICTED: StateLeaguePredicted = StateLeaguePredicted { file: 0x98ae50, lines: [0x303, 0x30b], places: &[1, 2, 3, 3], label: "cqx_3 rio grande do sul predicted" }; // cqx3: the rio grande do sul places

/// The predicted positions of the Rio Grande do Sul state league (at creation and at the season turnover): the clubs sorted by strength; the
/// first ones get the knockout places, 1, 2, 3, 3, and the others their place in the order (`state_league_predicted_positions`
/// with `GAUCHO_PREDICTED`); it is the board and press expectation for each club [inference]. The error boxes at
/// lines 0x303 (no statistics) and 0x30b (no memory).
/// Original: `BraRegGaucho::set_predicted_positions` (0x432980, bra_reg_gaucho.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn bra_reg_gaucho_set_predicted_positions(this: usize) {
    state_league_predicted_positions(this, &GAUCHO_PREDICTED)
}

/// Comparer for `bra_reg_gaucho_set_predicted_positions` (`compare_state_league_predicted`, side by side).
unsafe extern "thiscall" fn compare_bra_reg_gaucho_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    compare_state_league_predicted("bra_reg_gaucho_set_predicted_positions", this, &GAUCHO_PREDICTED, || o(this));
}
