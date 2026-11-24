// ---------- cx12_14: the Korean league schedule and table (kor_league.cpp) ----------

/// The K League schedule: only stage −1 (the byte; otherwise 0 with nothing written): `*out` (when given) = 1,
/// `*rounds` = 27, `*extra` = 0 and the table (`exe::crt_malloc` of 27 × 0x41; out of memory, the error box at line
/// 0xa1 of kor_league.cpp and 0) with `KOREAN_LEAGUE_2002` when the league's year is 2002 (the original passes the
/// literal 2002 as the year of round 0 there, the same value) and `KOREAN_LEAGUE_ROUNDS` otherwise
/// (`fra_third_write_league_rounds`). Returns the table.
/// Original: `KorLeague::create_schedule` (0x66b130, kor_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn kor_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    let this = League(this);
    if !out.is_null() {
        *out = 1;
    }
    *rounds = 0x1b;
    *extra = 0;
    let t = exe::crt_malloc(0x6db);
    if t == 0 {
        show_error_at(0x9eb120, 0xa1);
        return 0;
    }
    covered!("cx12_14 korean schedule");
    let table: &[FraThirdLeagueRound] = if this.year() == 0x7d2 { &KOREAN_LEAGUE_2002 } else { &KOREAN_LEAGUE_ROUNDS }; // cx12_14: the 2002 dates
    fra_third_write_league_rounds(t, this.year() as u16 as u32, 0, table);
    t
}

/// The K League table: 10 clubs (`league::CLUB_COUNT`), the table allocated (`exe::crt_malloc` of 10 × 0x3b; out of
/// memory, the error box at line 0x1a1 of kor_league.cpp and 0); every club of the database (`G_CLUB_COUNT`, re-read
/// each turn) whose `club::DIVISION` is this league's competition record (the pointer, not the id) takes the next row
/// (`exe::league_init_table_row`(row, club, 0, 0)) until the ten rows are full; with fewer, 0. Full: the prize fund for
/// a 10,000 share per place (`set_prize_fund`: `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1) and 1.
/// Original: `KorLeague::create_table` (0x66bba0, kor_league.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn kor_league_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(0xa);
    let t = exe::crt_malloc(0x24e);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9eb120, 0x1a1);
        return 0;
    }
    let mut rows = 0u8;
    let mut k = 0i32;
    while k < read_i32(G_CLUB_COUNT) {
        let club = club_at(k as isize);
        let division = club.division().addr();
        if division != 0 && division == this.competition().addr() {
            exe::league_init_table_row(this, rows as u32, club, 0, 0);
            rows = rows.wrapping_add(1);
            if rows as i8 as i16 == this.club_count() {
                break;
            }
        }
        k += 1;
    }
    if rows as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!("cx12_14 korean table");
    set_prize_fund(this, 0x2710); // cx12_14: the Korean quota
    1
}

/// Comparer for `kor_league_create_schedule`: side by side over the outputs and the written table (`compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("kor_league_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| kor_league_create_schedule(this, stage, r, e, l))
}

/// Comparer for `kor_league_create_table`: only the original runs (it allocates), against the plan
/// (`compare_japanese_table` with the Korean league's key, no clearing, the 10,000 share).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    compare_japanese_table("kor_league_create_table", League(this), 0xa, KEY_CLUB_COMP_KOREAN_LEAGUE, false, 0x2710, || o(this))
}

/// The K League rounds of the 2002 season: 28 July to 27 October.
pub(crate) const KOREAN_LEAGUE_2002: [FraThirdLeagueRound; 27] = [
    (28, 6, 0, 5, 1, &[(-1, -1, -1)]), // 0: 28 jul
    (1, 7, 0, 2, 2, &[(-1, -1, -1)]), // 1: 1 aug
    (4, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 4 aug
    (8, 7, 0, 2, 2, &[(-1, -1, -1)]), // 3: 8 aug
    (11, 7, 0, 5, 1, &[(-1, -1, -1)]), // 4: 11 aug
    (15, 7, 0, 2, 2, &[(-1, -1, -1)]), // 5: 15 aug
    (18, 7, 0, 5, 1, &[(-1, -1, -1)]), // 6: 18 aug
    (22, 7, 0, 2, 2, &[(-1, -1, -1)]), // 7: 22 aug
    (25, 7, 0, 5, 1, &[(-1, -1, -1)]), // 8: 25 aug
    (29, 7, 0, 2, 2, &[(-1, -1, -1)]), // 9: 29 aug
    (1, 8, 0, 5, 1, &[(-1, -1, -1)]), // 10: 1 sep
    (4, 8, 0, 2, 2, &[(-1, -1, -1)]), // 11: 4 sep
    (8, 8, 0, 5, 1, &[(-1, -1, -1)]), // 12: 8 sep
    (12, 8, 0, 2, 2, &[(-1, -1, -1)]), // 13: 12 sep
    (15, 8, 0, 5, 1, &[(-1, -1, -1)]), // 14: 15 sep
    (19, 8, 0, 2, 2, &[(-1, -1, -1)]), // 15: 19 sep
    (22, 8, 0, 5, 1, &[(-1, -1, -1)]), // 16: 22 sep
    (26, 8, 0, 2, 2, &[(-1, -1, -1)]), // 17: 26 sep
    (29, 8, 0, 5, 2, &[(-1, -1, -1)]), // 18: 29 sep
    (3, 9, 0, 2, 1, &[(-1, -1, -1)]), // 19: 3 oct
    (6, 9, 0, 5, 2, &[(-1, -1, -1)]), // 20: 6 oct
    (10, 9, 0, 2, 1, &[(-1, -1, -1)]), // 21: 10 oct
    (13, 9, 0, 5, 2, &[(-1, -1, -1)]), // 22: 13 oct
    (17, 9, 0, 2, 1, &[(-1, -1, -1)]), // 23: 17 oct
    (20, 9, 0, 5, 2, &[(-1, -1, -1)]), // 24: 20 oct
    (24, 9, 0, 2, 1, &[(-1, -1, -1)]), // 25: 24 oct
    (27, 9, 0, 5, 2, &[(-1, -1, -1)]), // 26: 27 oct
];

/// The K League rounds of any other season: 16 June to 27 October.
pub(crate) const KOREAN_LEAGUE_ROUNDS: [FraThirdLeagueRound; 27] = [
    (16, 5, 0, 5, 1, &[(-1, -1, -1)]), // 0: 16 jun
    (20, 5, 0, 2, 2, &[(-1, -1, -1)]), // 1: 20 jun
    (23, 5, 0, 5, 1, &[(-1, -1, -1)]), // 2: 23 jun
    (27, 5, 0, 2, 2, &[(-1, -1, -1)]), // 3: 27 jun
    (30, 5, 0, 5, 1, &[(-1, -1, -1)]), // 4: 30 jun
    (7, 6, 0, 5, 1, &[(-1, -1, -1)]), // 5: 7 jul
    (11, 6, 0, 2, 2, &[(-1, -1, -1)]), // 6: 11 jul
    (14, 6, 0, 5, 2, &[(-1, -1, -1)]), // 7: 14 jul
    (21, 6, 0, 5, 1, &[(-1, -1, -1)]), // 8: 21 jul
    (25, 6, 0, 2, 2, &[(-1, -1, -1)]), // 9: 25 jul
    (28, 6, 0, 5, 1, &[(-1, -1, -1)]), // 10: 28 jul
    (1, 7, 0, 2, 2, &[(-1, -1, -1)]), // 11: 1 aug
    (18, 7, 0, 5, 1, &[(-1, -1, -1)]), // 12: 18 aug
    (22, 7, 0, 2, 2, &[(-1, -1, -1)]), // 13: 22 aug
    (25, 7, 0, 5, 1, &[(-1, -1, -1)]), // 14: 25 aug
    (29, 7, 0, 2, 2, &[(-1, -1, -1)]), // 15: 29 aug
    (1, 8, 0, 5, 1, &[(-1, -1, -1)]), // 16: 1 sep
    (5, 8, 0, 2, 2, &[(-1, -1, -1)]), // 17: 5 sep
    (8, 8, 0, 5, 2, &[(-1, -1, -1)]), // 18: 8 sep
    (19, 8, 0, 2, 1, &[(-1, -1, -1)]), // 19: 19 sep
    (22, 8, 0, 5, 2, &[(-1, -1, -1)]), // 20: 22 sep
    (26, 8, 0, 2, 1, &[(-1, -1, -1)]), // 21: 26 sep
    (13, 9, 0, 5, 2, &[(-1, -1, -1)]), // 22: 13 oct
    (17, 9, 0, 2, 1, &[(-1, -1, -1)]), // 23: 17 oct
    (20, 9, 0, 5, 2, &[(-1, -1, -1)]), // 24: 20 oct
    (24, 9, 0, 2, 1, &[(-1, -1, -1)]), // 25: 24 oct
    (27, 9, 0, 5, 2, &[(-1, -1, -1)]), // 26: 27 oct
];
