// ---------- rx11_4: the progress codes and the qualification status of a nation in the African World Cup qualifying (wc_african_cup.cpp) ----------

/// A progress code through a band table: code k (1-based) becomes `bands[k − 1]`; a code outside the table stays.
pub(crate) fn african_band(code: u8, bands: &[u8]) -> u8 {
    match code {
        1..=6 if (code as usize) <= bands.len() => bands[code as usize - 1],
        _ => code,
    }
}

/// The African progress codes (current, best, worst) of a nation at `stage` (signed byte) by the format byte
/// (`wc_cup_turnover_consts::AFRICAN_FORMAT`). The first round (stage −1; with format 2 its own bands): winner 1 at
/// best, the rest 25; the current and worst 21 (19 with format 2) for 1 and 25 for 2. A group stage (below 5 with
/// format 0, below 4 with 1 or 3 and others, below 5 with 2): the band tables below by position. The play-off with
/// format 2 (stage 5 and on) and with the other formats at stage 4: 1 stays 1 and 2 becomes 4 (format 2) or 3. Any
/// other stage, as they came.
pub(crate) fn african_progress_codes(format: u8, stage: i8, current: u8, best: u8, worst: u8) -> (u8, u8, u8) {
    let first_round = |one: u8| -> (u8, u8, u8) {
        let two = |v: u8, a: u8| match v {
            1 => a,
            2 => 0x19,
            _ => v,
        };
        (two(current, one), two(best, 1), two(worst, one))
    };
    let all = |c: &[u8], b: &[u8], w: &[u8]| (african_band(current, c), african_band(best, b), african_band(worst, w));
    let pair = |two: u8| {
        let f = |v: u8| match v {
            1 => 1,
            2 => two,
            _ => v,
        };
        (f(current), f(best), f(worst))
    };
    match format {
        0 => match stage {
            -1 => first_round(0x15),
            s if s < 5 => {
                let b = [1, 6, 0xb, 0x10, 0x15];
                all(&b, &b, &b) // rx11_4: five groups of five
            }
            _ => (current, best, worst),
        },
        1 => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let b = [1, 5, 9, 0xd, 0x11, 0x15];
                all(&b, &b, &b)
            }
            _ => (current, best, worst),
        },
        2 => match stage {
            -1 => first_round(0x13),
            s if s < 5 => all(&[1, 7, 0xd, 0x13], &[1, 7, 0xd, 0x13], &[4, 7, 0xd, 0x13]),
            _ => pair(4),
        },
        _ => match stage {
            -1 => first_round(0x15),
            s if s < 4 => {
                let others = [3, 5, 9, 0xd, 0x11, 0x15];
                all(&others, &[1, 5, 9, 0xd, 0x11, 0x15], &others)
            }
            4 => pair(3),
            _ => (current, best, worst),
        },
    }
}

/// rule: the African qualifying progress of a nation: in the first round the winner is 1 at best and 21 (19 with six
/// groups) now, the loser 25; in the groups the places map to bands of the final position (five groups: 1, 6, 11, 16,
/// 21; four groups: 1, 5, 9, 13, 17, 21, the winners 3 now with format 3; six groups: 1, 7, 13, 19, the winners 4 at
/// worst); in the play-off the winner 1 and the loser 3 (4 with six groups)
///
/// The codes of `african_progress_codes` into the nation's statistics entry (`exe::comp_stats_find_club_entry`, not
/// checked for null; `write_progress_codes`).
/// Original: `WcAfricanCup::set_club_progress` (0x9115a0, wc_african_cup.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    covered!("rx11_4 african progress");
    let e = exe::comp_stats_find_club_entry(read_usize(this + league::COMP_STATS), club);
    let format = read_u8(this + wc_cup_turnover_consts::AFRICAN_FORMAT);
    let (c, b, w) = african_progress_codes(format, stage as i8, current as u8, best as u8, worst as u8);
    write_progress_codes(e, c, b, w);
}

/// rule: a nation's status in the African, Asian, CONCACAF and Oceania qualifying is qualified (−2) while its progress
/// code (stat 0x12) is 1, and not (−3) otherwise
///
/// `qualifying_stat_byte`(club, 0x12) = 1: −2, otherwise −3 (a dword in eax). Shared by four vtables (+0x70).
/// Original: `WcAfricanCup::get_qualification_status` (0x911b80, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    covered!("rx11_4 qualification status");
    if qualifying_stat_byte(this, club, 0x12) == 1 { -2 } else { -3 } // rx11_4: qualified at code 1
}

/// rule: in the African qualifying a nation's status changes down (−1) whenever its progress code (stat 0x12) is 1, up
/// (1) when it left 1 (stat 0x13, the previous code, was 1), otherwise 0
///
/// `qualifying_stat_byte` of 0x13 (before) and 0x12 (now): now 1, −1 (whatever before was; unlike the Oceania one, so
/// a nation still at 1 reads as falling again [inference: a slip]); otherwise 1 when before was 1, else 0. The byte in
/// al.
/// Original: `WcAfricanCup::get_qualification_status_change` (0x911ba0, wc_african_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    covered!("rx11_4 african status change");
    let before = qualifying_stat_byte(this, club, 0x13);
    let now = qualifying_stat_byte(this, club, 0x12);
    if now == 1 {
        -1
    } else {
        (before == 1) as i8 // rx11_4: left code 1
    }
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I0%%);
    compare_progress_codes("wc_african_cup_set_club_progress", this, club, stage, (current, best, worst), || o(this, club, stage, current, best, worst),
        || wc_african_cup_set_club_progress(this, club, stage, current, best, worst))
}

/// Comparers of the two status readers: they only read the statistics; side by side on the result.
pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status(this: usize, club: usize) -> i32 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(%%I1%%)(this, club);
    let b = wc_african_cup_get_qualification_status(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}

pub(crate) unsafe extern "thiscall" fn compare_wc_african_cup_get_qualification_status_change(this: usize, club: usize) -> i8 {
    let a = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i8>(%%I2%%)(this, club);
    let b = wc_african_cup_get_qualification_status_change(this, club);
    SCOREBOARD.record(a == b, || format!("wc_african_cup_get_qualification_status_change({:#x}, {:#x}) original={} rust={}", this, club, a, b));
    a
}
