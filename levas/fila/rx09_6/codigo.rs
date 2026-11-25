// ---------- rx09_6: the Spanish Second Division B progress codes (spa_second_b.cpp) ----------

/// The three progress codes (current, best, worst) of a Second Division B club at a stage, as the original turns them
/// into the season codes (byte arithmetic):
/// - the groups (stage below 3): best 1 to 4 becomes 1, worst 1 to 4 becomes 4, then each of the three × 4 − 3;
/// - the promotion play-off groups (3 to 6): each code 1 to 4 becomes 4v − 3 (1, 5, 9, 13), any other stays;
/// - the relegation play-off (7) and its final (8): the play-off code (1 the winner, 3 at 7 or 2 at 8 the loser) becomes
///   0x3b or 0x3f, but only in the best code: a best code is mapped, then a worst code or a current code of 1 or 3 (2)
///   overwrites the best code with its mapping, and the current and worst codes are kept as they came (defect of the
///   original: the register of the best code is reused for the other two);
/// - later stages: the codes as they came.
pub(crate) fn segunda_b_progress(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let scale = |v: u8| v.wrapping_mul(4).wrapping_sub(3);
    match stage {
        i8::MIN..=2 => {
            let b = if (1..=4).contains(&best) { 1 } else { best };
            let w = if (1..=4).contains(&worst) { 4 } else { worst };
            (scale(current), scale(b), scale(w))
        }
        3..=6 => {
            let group = |v: u8| if (1..=4).contains(&v) { scale(v) } else { v };
            (group(current), group(best), group(worst))
        }
        7 | 8 => {
            let lost = if stage == 7 { 3 } else { 2 };
            let code = |v: u8| if v == 1 { Some(0x3b) } else if v == lost { Some(0x3f) } else { None }; // rx09_6: the relegation play-off codes
            let mut b = code(best).unwrap_or(best);
            if let Some(c) = code(worst) {
                b = c;
            }
            if let Some(c) = code(current) {
                b = c;
            }
            (current, b, worst)
        }
        _ => (current, best, worst),
    }
}

/// Stores a Second Division B club's current, best and worst position codes in its statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked), turned into the season codes by `segunda_b_progress`.
/// Original: `SpaSecondB::set_club_progress` (0x858460, spa_second_b.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn spa_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx09_6 segunda b progress");
    let entry = CompStatsClub(exe::comp_stats_find_club_entry(League(this).comp_stats(), club));
    let (c, b, w) = segunda_b_progress(stage as u8 as i8, current as u8, best as u8, worst as u8);
    entry.set_best_position(b);
    entry.set_worst_position(w);
    entry.set_current_position(c);
}

/// Comparer for `spa_second_b_set_club_progress`: the three position bytes of the club's entry side by side.
pub(crate) unsafe extern "thiscall" fn compare_spa_second_b_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    let stats = League(this).comp_stats();
    let entry = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if entry == 0 {
        return o(this, club, stage, current, best, worst);
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side(&[(entry + comp_stats_club::CURRENT_POSITION, 3)], || o(this, club, stage, current, best, worst),
        || spa_second_b_set_club_progress(this, club, stage, current, best, worst));
    SCOREBOARD.record(same, || format!("spa_second_b_set_club_progress({:#x}, club {:#x}, stage {}, {}, {}, {}): original {:x?} rust {:x?}", this, club, stage as u8 as i8,
        current as u8, best as u8, worst as u8, mo, mr));
}
