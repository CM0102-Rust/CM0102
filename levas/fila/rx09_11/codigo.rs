// ---------- rx09_11: the Swedish Second Division predicted positions and progress codes (swe_second.cpp) ----------

/// The predicted positions of the Swedish Second Division: each of the six sections (the North in the league's table,
/// the others in the section stages 0..4) sorted by strength, and the club in place k gets 6k + 1 whatever its section
/// (`grouped_league_predicted_positions`; a list of 0x30 bytes, boxes at lines 0x48b and 0x493).
/// Original: `SweSecond::set_predicted_positions` (0x893470, swe_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn swe_second_set_predicted_positions(this: usize) {
    grouped_league_predicted_positions(League(this), 5, 0x30, SWEDISH_SECOND_SOURCE, (0x48b, 0x493), |_, k| k.wrapping_mul(6).wrapping_add(1)); // rx09_11: 6k + 1
}

/// Comparer for `swe_second_set_predicted_positions` (`compare_grouped_predicted_positions`).
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    compare_grouped_predicted_positions("swe_second_set_predicted_positions", League(this), 5, || o(this), || swe_second_set_predicted_positions(this));
}

/// The three progress codes (current, best, worst) of a Swedish Second Division club at a stage (byte arithmetic):
/// - the sections (stage below 5): each code × 5 − 4;
/// - the relegation play-off (6): each code 1, 7 or 13 becomes 0x37, 0x3d or 0x43, any other stays;
/// - the promotion play-off (5) and later stages: the codes as they came.
pub(crate) fn swedish_second_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage < 5 {
        let scale = |v: u8| v.wrapping_mul(5).wrapping_sub(4);
        return (scale(current), scale(best), scale(worst));
    }
    if stage != 6 {
        return (current, best, worst);
    }
    let play_off = |v: u8| match v {
        1 => 0x37,
        7 => 0x3d, // rx09_11: the relegation play-off codes
        13 => 0x43,
        v => v,
    };
    (play_off(current), play_off(best), play_off(worst))
}

/// Stores a Swedish Second Division club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `swedish_second_progress`.
/// Original: `SweSecond::set_club_progress` (0x8935e0, swe_second.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_11 swedish second progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = swedish_second_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_current_position(c);
    entry.set_worst_position(w);
}

/// Comparer for `swe_second_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_swe_second_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || swe_second_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("swe_second_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}
