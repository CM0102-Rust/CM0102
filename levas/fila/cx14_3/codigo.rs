// ---------- cx14_3: the Turkish category B club list, predicted positions and progress codes (tur_second_b.cpp) ----------

/// The source file of the Turkish second division category B (tur_second_b.cpp), for the error boxes.
pub(crate) const TURKISH_CATEGORY_B_SOURCE: usize = 0xaa18b8;

/// Where `get_club_array` of a league split into groups writes each club, in the original's order: the league's own
/// table at 0.., then each of the `groups` group stages (`league::STAGES`[0..groups]) at `club_count`.. again.
///
/// Defect of the original: the group stages are not offset by the groups before them, so every group overwrites the
/// same slots (`club_count` to 2 × `club_count` − 1) and only the last group's clubs stay there; the rest of the list
/// (allocated for every club of every group) is left as malloc returned it. With `bound_by_own`, each group is copied
/// for the league's own club count instead of the group's (the Spanish Second Division B; the groups have the same
/// size).
pub(crate) unsafe fn grouped_league_club_slots(this: League, groups: usize, bound_by_own: bool) -> Vec<(usize, usize)> {
    let mut slots = Vec::new();
    let mut i = 0i16;
    while i < this.club_count() {
        slots.push((i as usize, aus_nsl_table_row_club(this, i as usize)));
        i += 1;
    }
    for g in 0..groups {
        let stage = League(read_usize(this.stages() + 4 * g));
        let mut i = 0i16;
        while i < if bound_by_own { this.club_count() } else { stage.club_count() } {
            let k = this.club_count() as i32 + i as i32; // cx14_3: the slot of a group club (no group offset)
            slots.push((k as usize, aus_nsl_table_row_club(stage, i as usize)));
            i += 1;
        }
    }
    slots
}

/// The body of `get_club_array` of a league split into groups (thiscall(&list, &count), ret 8): the count = `total`
/// (written first), a list of `total` club pointers (`exe::crt_malloc`; out of memory: the box at `line` of `file`,
/// the count 0 and nothing else), filled as `grouped_league_club_slots` says.
pub(crate) unsafe fn grouped_league_club_array(this: League, list: *mut usize, n: *mut i16, total: i16, groups: usize, bound_by_own: bool, file: usize, line: u32) {
    *n = total;
    let v = exe::crt_malloc((total as i32 as u32).wrapping_mul(4));
    *list = v;
    if v == 0 {
        show_error_at(file, line);
        *n = 0;
        return;
    }
    covered!("cx14_3 grouped club list");
    for (k, club) in grouped_league_club_slots(this, groups, bound_by_own) {
        write_usize(v + 4 * k, club);
    }
}

/// The comparer of a grouped `get_club_array`: only the original runs (it allocates the list the caller frees); the
/// count and every slot the copy would write (the last write of each slot) must match.
pub(crate) unsafe fn compare_grouped_league_club_array(name: &str, this: League, list: *mut usize, n: *mut i16, total: i16, groups: usize, bound_by_own: bool, original: impl FnOnce()) {
    original();
    if *list == 0 {
        return;
    }
    let mut expected: Vec<Option<usize>> = vec![None; total.max(0) as usize];
    for (k, club) in grouped_league_club_slots(this, groups, bound_by_own) {
        if k < expected.len() {
            expected[k] = Some(club);
        }
    }
    let got: Vec<Option<usize>> = expected.iter().enumerate().map(|(k, e)| e.map(|_| read_usize(*list + 4 * k))).collect();
    SCOREBOARD.record(*n == total && got == expected, || format!("{}({:#x}) count {}: original {:x?} rust {:x?}", name, this.addr(), *n, got, expected));
}

/// The clubs of the Turkish second division category B: 50 slots (the five groups of ten), filled by
/// `grouped_league_club_array` with the group stages 0..3 (only the first group and the last group stage end up in
/// the list, the defect described there).
/// Original: `TurSecondB::get_club_array` (0x901800, tur_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn tur_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    grouped_league_club_array(League(this), list, n, 0x32, 4, false, TURKISH_CATEGORY_B_SOURCE, 0x4c5);
}

/// Comparer for `tur_second_b_get_club_array` (`compare_grouped_league_club_array`).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_get_club_array(this: usize, list: *mut usize, n: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I0%%);
    compare_grouped_league_club_array("tur_second_b_get_club_array", League(this), list, n, 0x32, 4, false, || o(this, list, n));
}

/// The body of `set_predicted_positions` of a league split into groups (thiscall, ret 0): without statistics, the box
/// at `lines.0` and nothing else; otherwise a list of `list_size` bytes (`exe::crt_malloc`; out of memory: the box at
/// `lines.1`). For each group g = −1 (the league's own table) to `groups` − 1 (`league::STAGES`[g]): its clubs copied
/// into the list, sorted by strength (qsort `exe::crt_qsort` with 0x4b07d0, cdecl) and the club in place k gets the
/// predicted place `place(g, k)` in the statistics (`exe::comp_stats_set_club_final_position`, which keeps a byte).
/// The list is freed at the end.
pub(crate) unsafe fn grouped_league_predicted_positions(this: League, groups: i8, list_size: u32, file: usize, lines: (u32, u32), place: impl Fn(i8, u8) -> u8) {
    if this.comp_stats() == 0 {
        show_error_at(file, lines.0);
        return;
    }
    let list = exe::crt_malloc(list_size);
    if list == 0 {
        show_error_at(file, lines.1);
        return;
    }
    covered!("cx14_3 grouped predicted positions");
    for g in -1..groups {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        let mut i = 0i16;
        while i < t.club_count() {
            write_usize(list + 4 * i as usize, aus_nsl_table_row_club(t, i as usize));
            i += 1;
        }
        exe::crt_qsort(list, t.club_count() as i32 as u32, 4, 0x4b07d0);
        let mut k = 0i16;
        while k < t.club_count() {
            exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(list + 4 * k as usize), place(g, k as u8) as u32);
            k += 1;
        }
    }
    exe::crt_free(list);
}

/// The comparer of a grouped `set_predicted_positions`: side by side over the predicted place byte
/// (`comp_stats_club::FINAL_POSITION`) of every club of every group (only the original without statistics or when a
/// club has no entry yet).
pub(crate) unsafe fn compare_grouped_predicted_positions(name: &str, this: League, groups: i8, original: impl FnOnce(), copy: impl FnOnce()) {
    let stats = this.comp_stats();
    if stats == 0 {
        return original();
    }
    let mut regions = Vec::new();
    for g in -1..groups {
        let t = if g == -1 { this } else { League(read_usize(this.stages() + 4 * g as usize)) };
        for i in 0..t.club_count().max(0) as usize {
            let e = exe::comp_stats_find_club_entry(stats, aus_nsl_table_row_club(t, i));
            if e == 0 {
                return original();
            }
            regions.push((e + comp_stats_club::FINAL_POSITION, 1));
        }
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&regions, original, copy);
    SCOREBOARD.record(same, || format!("{}({:#x}) original={:x?} rust={:x?}", name, this.addr(), mo, mr));
}

/// The predicted positions of the Turkish second division category B: each of the five groups (the first group in
/// the league's table, the other four in the group stages 0..3) sorted by strength, and the club in place k (0 to 9)
/// gets 5k + 1 whatever its group (`grouped_league_predicted_positions`; a list of 0xc8 bytes, boxes at lines 0x4ec and
/// 0x4f4).
/// Original: `TurSecondB::set_predicted_positions` (0x901950, tur_second_b.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_predicted_positions(this: usize) {
    grouped_league_predicted_positions(League(this), 4, 0xc8, TURKISH_CATEGORY_B_SOURCE, (0x4ec, 0x4f4), |_, k| k.wrapping_mul(5).wrapping_add(1)); // cx14_3: 5k + 1
}

/// Comparer for `tur_second_b_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    compare_grouped_predicted_positions("tur_second_b_set_predicted_positions", League(this), 4, || o(this), || tur_second_b_set_predicted_positions(this));
}

/// The three progress codes (current, best, worst) of a category B club at a stage, as the original turns them into
/// the season codes (byte arithmetic, "×5 + 1" written `scale`):
/// - first phase (stage below 4, signed: the league's own group −1 and the group stages 0..3): current
///   `scale(5 × current − 4)`, best 1, worst `scale(7)` when the worst code is below 3, else `scale(0x2e)`;
/// - promotion group (4): current `scale(current)` from 3 up, else `scale(1)`; best 1; worst `scale(1)` for 1 and 2,
///   `scale(5)` for 3 to 5, else `scale(worst)`;
/// - classification groups (5 to 9): `scale(current)`, 1, `scale(worst)`;
/// - play-off (10): each code 1 stays 1, any other becomes 0xb;
/// - later stages: the codes as they came.
pub(crate) fn turkish_category_b_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let scale = |v: u8| v.wrapping_mul(5).wrapping_add(1);
    if stage < 4 {
        (scale(current.wrapping_mul(5).wrapping_sub(4)), 1, scale(if worst < 3 { 7 } else { 0x2e }))
    } else if stage == 4 {
        let worst = match worst {
            1 | 2 => 1,
            3..=5 => 5,
            w => w,
        };
        (scale(if current >= 3 { current } else { 1 }), 1, scale(worst))
    } else if stage < 10 {
        (scale(current), 1, scale(worst))
    } else if stage == 10 {
        let play_off = |v: u8| if v == 1 { 1 } else { 0xb }; // cx14_3: the play-off codes
        (play_off(current), play_off(best), play_off(worst))
    } else {
        (current, best, worst)
    }
}

/// Stores a category B club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `turkish_category_b_progress`.
/// Original: `TurSecondB::set_club_progress` (0x901ac0, tur_second_b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn tur_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx14_3 turkish category b progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = turkish_category_b_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_worst_position(w);
    entry.set_current_position(c);
}

/// Comparer for `tur_second_b_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_tur_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || tur_second_b_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("tur_second_b_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}
