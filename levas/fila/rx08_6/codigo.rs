// ---------- rx08_6: the European World Cup qualifying's predicted positions (wc_europe_league.cpp) ----------

/// The predicted final position of the nation at `k` in the strength-sorted European pool, by the exempt count: the
/// seeding pots (the first nine or ten are pot 1, the next ones pot 2 and so on), each pot's first place as the
/// prediction. `None` for an exempt count outside 0..=3 (no prediction).
fn wc_europe_league_predicted_position(exempt: u8, k: i16) -> Option<u32> {
    let pots: &[(i16, u32)] = match exempt {
        0 => &[(10, 1), (15, 11), (19, 16), (20, 20), (30, 21), (40, 31), (i16::MAX, 41)],
        1 => &[(9, 1), (14, 10), (18, 15), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        2 => &[(9, 1), (13, 10), (16, 14), (18, 17), (27, 19), (36, 28), (45, 37), (i16::MAX, 46)],
        3 => &[(8, 1), (12, 9), (15, 13), (16, 16), (24, 17), (32, 25), (40, 33), (i16::MAX, 41)],
        _ => return None,
    };
    pots.iter().find(|&&(below, _)| k < below).map(|&(_, position)| position)
}

/// rule: before the European qualifying starts every nation of the pool gets its predicted final position from its
/// seeding pot (the strongest pot first), and all the European national teams leave their continental competitions to
/// play this one
///
/// Without statistics (`league::COMP_STATS`): the error box of line 0x7a0 and nothing else. The pool (+0xee, count
/// +0xf2) sorted by strength (`exe::crt_qsort`(pool, count, 4, 0x4b07d0)); the European national teams released
/// (`exe::nationalteams_release_competitions_of_continent`(national teams, the European continent record —
/// `G_CONTINENT_TABLE` + 0xc6 × `KEY_CONTINENT_EUROPE` — and 0)); then each nation k of the pool (a signed word below
/// the count): its predicted position (`wc_europe_league_predicted_position`; Comp_Stats::set_club_final_position
/// `exe::comp_stats_set_club_final_position`(stats, nation, position)) and its current competition this one
/// (`exe::nationalteams_set_current_competition`(national teams, `club::NATION`, `league::COMPETITION`)).
/// Original: `WcEuropeLeague::set_predicted_positions` (0x91b1b0, wc_europe_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_predicted_positions(this: usize) {
    let league = League(this);
    if league.comp_stats() == 0 {
        show_error_at(wc_cup_turnover_consts::WC_EUROPE_LEAGUE_CPP, 0x7a0);
        return;
    }
    covered!("rx08_6 european predicted positions");
    let pool = read_usize(this + 0xee);
    exe::crt_qsort(pool, read_u32(this + 0xf2), 4, 0x4b07d0);
    let europe = read_usize(G_CONTINENT_TABLE).wrapping_add(read_usize(KEY_CONTINENT_EUROPE).wrapping_mul(0xc6));
    exe::nationalteams_release_competitions_of_continent(read_usize(G_NATIONAL_TEAMS), europe, 0);
    let mut k = 0i16;
    while (k as i32) < read_i32(this + 0xf2) {
        let nation = Club(read_usize(read_usize(this + 0xee) + k as isize as usize * 4));
        if let Some(position) = wc_europe_league_predicted_position(read_u8(this + WC_EUROPE_EXEMPT), k) { // rx08_6: the seeding pot's first place
            exe::comp_stats_set_club_final_position(league.comp_stats(), nation, position);
        }
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), nation.nation(), league.competition().addr()); // rx08_6: the nation plays this one
        k = k.wrapping_add(1);
    }
}

/// Comparer for `wc_europe_league_set_predicted_positions`: it moves the national teams between competitions, so only
/// the original runs; the plan: the pool sorted as a copy sorted the same way before.
pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let (pool, n) = (read_usize(this + 0xee), read_i32(this + 0xf2).clamp(0, 64) as usize);
    if League(this).comp_stats() == 0 || pool == 0 {
        return o(this);
    }
    let mut sorted: Vec<usize> = (0..n).map(|k| read_usize(pool + k * 4)).collect();
    exe::crt_qsort(sorted.as_mut_ptr() as usize, n as u32, 4, 0x4b07d0);
    o(this);
    let wrong = (0..n).find(|&k| read_usize(pool + k * 4) != sorted[k]);
    SCOREBOARD.record(wrong.is_none(), || format!("wc_europe_league_set_predicted_positions({:#x}): pool place {:?} differs from the sorted copy", this, wrong));
}
