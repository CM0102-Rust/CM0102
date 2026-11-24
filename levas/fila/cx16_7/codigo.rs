// ---------- cx16_7: the club progress codes of the CONCACAF, European and Oceania World Cup qualifying (wc_concacaf_cup.cpp, wc_europe_league.cpp, wc_oceania_league.cpp) ----------

/// The CONCACAF progress codes (current, best, worst) of a nation at `stage` (signed byte), with `exempt` (+0xba) and
/// `others` (+0xbc) the exempt counts. The knockout (stage −1, or stage 0 with fewer than two exempt non-Caribbean
/// nations): best 1 for a winner (1) else 13, current and worst 13. A Central American or semi-final group (stages
/// below 3 other than the knockout, with fewer than two exempt others; stage 3 too, then; or below 3 with exactly two):
/// 1 and 2 become 1 for the best and 6 for the current and worst, 3 becomes 7 and 4 becomes 10. The final group (stage 4
/// with fewer than two exempt others, or 3 with exactly two): codes up to 3 − exempt become 1. Any other, as they came.
pub(crate) fn wc_concacaf_progress_codes(stage: i8, exempt: i8, others: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let knockout = (stage == -1 || stage == 0) && (stage == -1 || others < 2);
    if knockout {
        return (0xd, if best == 1 { 1 } else { 0xd }, 0xd);
    }
    let group = (stage != 0 && stage < 4 && others < 2) || (stage < 3 && others == 2);
    if group {
        let band = |v: u8, top: u8| match v {
            1 | 2 => top,
            3 => 7,
            4 => 0xa,
            _ => v,
        };
        return (band(current, 6), band(best, 1), band(worst, 6)); // cx16_7: the group bands
    }
    let last = (stage == 4 && others < 2) || (stage == 3 && others == 2);
    if last {
        let top = 3 - exempt as i32;
        let q = |v: u8| if v as i32 > top { v } else { 1 };
        return (q(current), q(best), q(worst));
    }
    (current, best, worst)
}

/// rule: the CONCACAF qualifying progress of a nation: a knockout winner is best 1, the rest 13; in a group the first
/// two are 1 at best and 6 now, third 7 and fourth 10; in the final group the qualifying places (three less the exempt)
/// are 1
///
/// The codes of `wc_concacaf_progress_codes` (by +0xba and +0xbc, signed bytes) into the nation's statistics entry
/// (`exe::comp_stats_find_club_entry`, not checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcConcacafCup::set_club_progress` (0x918040, wc_concacaf_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_concacaf_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 concacaf progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_concacaf_progress_codes(stage as i8, read_u8(this + 0xba) as i8, read_u8(this + 0xbc) as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The European progress codes (current, best, worst) at `stage` (signed byte). In a group (up to the last group stage,
/// +0xff): 1 → 1, 2 → 16 (the best 10), 3 → 19, 4 → 28, 5 → 37, 6 → 46. In the play-off stage (+0x101): 1 → 11, 2 → 16.
/// Any other stage, as they came. Defect of the original [inference]: a group runner-up's best becomes 10 where its
/// current and worst become 16.
pub(crate) unsafe fn wc_europe_progress_codes(this: usize, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage <= read_u8(this + 0xff) as i8 {
        let band = |v: u8, second: u8| match v {
            1 => 1,
            2 => second,
            3 => 0x13,
            4 => 0x1c,
            5 => 0x25,
            6 => 0x2e,
            _ => v,
        };
        return (band(current, 0x10), band(best, 0xa), band(worst, 0x10)); // cx16_7: the European group bands
    }
    if stage == read_u8(this + 0x101) as i8 {
        let play_off = |v: u8| match v {
            1 => 0xb,
            2 => 0x10,
            _ => v,
        };
        return (play_off(current), play_off(best), play_off(worst));
    }
    (current, best, worst)
}

/// rule: the European qualifying progress of a nation: in a group the winner is 1, the runner-up 16 (10 at best), then
/// 19, 28, 37 and 46; in the play-off the winner 11 and the loser 16
///
/// The codes of `wc_europe_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcEuropeLeague::set_club_progress` (0x91b510, wc_europe_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 european progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_europe_progress_codes(this, stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// The Oceania progress codes (current, best, worst) at `stage` (signed byte): from stage 1 on, as they came. In the
/// groups (below 1): the best 1, 2 → 1, 3 → 5, 4 → 7, 5 → 9, 6 → 11; the worst 1, 2 → 3, 3 → 5; the current 1 → 1, 2 → 3,
/// 3 → 8. Defects of the original [inference]: a worst of 4, 5 or 6 and a current of 4, 5 or 6 are not mapped; the
/// best becomes 7, 9 or 11 instead (the worst's case first, then the current's, the last one written winning).
pub(crate) fn wc_oceania_progress_codes(stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    if stage >= 1 {
        return (current, best, worst);
    }
    let mut b = match best {
        1 | 2 => 1,
        3 => 5,
        4 => 7,
        5 => 9,
        6 => 0xb,
        _ => best,
    };
    let mut w = worst;
    match worst {
        1 | 2 => w = 3,
        3 => w = 5,
        4 => b = 7, // the original writes the best
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    let mut c = current;
    match current {
        1 => c = 1,
        2 => c = 3, // cx16_7: the Oceania group runner-up
        3 => c = 8,
        4 => b = 7,
        5 => b = 9,
        6 => b = 0xb,
        _ => {}
    }
    (c, b, w)
}

/// rule: the Oceania qualifying group progress of a nation: the best 1 for the top two, then 5, 7, 9 and 11; the
/// current 1, 3 and 8 for the first three; later stages keep their codes
///
/// The codes of `wc_oceania_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes` of the Minas Gerais state league).
/// Original: `WcOceaniaLeague::set_club_progress` (0x9200e0, wc_oceania_league.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("cx16_7 oceania progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let (c, b, w) = wc_oceania_progress_codes(stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// Side-by-side comparer of a progress writer: the three codes of the nation's statistics entry (only the original when
/// there is no entry or no statistics).
pub(crate) unsafe fn compare_progress_codes(name: &str, this: usize, club: usize, stage: u32, codes: (u32, u32, u32), o: impl FnOnce(), c: impl FnOnce()) {
    let stats = read_usize(this + league::COMP_STATS);
    let e = if stats != 0 { exe::comp_stats_find_club_entry(stats, club) } else { 0 };
    if e == 0 {
        return o();
    }
    let (_, _, same, mo, mr) = crate::comparison::side_by_side_light(&[(e + comp_stats_club::CURRENT_POSITION, 3)], o, c);
    SCOREBOARD.record(same, || format!("{}({:#x}, {:#x}, stage {}, {:?}) original={:x?} rust={:x?}", name, this, club, stage as u8 as i8, codes, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn compare_wc_concacaf_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_progress_codes("wc_concacaf_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_concacaf_cup_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_europe_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I1%%);
    compare_progress_codes("wc_europe_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_europe_league_set_club_progress(this, club, stage, current, best, worst))
}

pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_progress_codes("wc_oceania_league_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_oceania_league_set_club_progress(this, club, stage, current, best, worst))
}
