// ---------- cx12_13: the Norwegian Premier and First Division schedules (nor_prm.cpp, nor_first.cpp) ----------

/// A round of a Norwegian league table: day, month (0 = January), years to add, weekday (−1 none), code, and the match
/// slots written (slot index, a, b, c), in the original's order (a slot may be written twice).
pub(crate) type NorwayRound = (i8, i8, i8, i8, i8, &'static [(i8, i8, i8, i8)]);

/// Writes `rounds` from round `*k` on (`exe::league_round_set_date` with value 0, then each slot with
/// `exe::league_round_set_match_slot`, value 0), moving `*k` on.
unsafe fn write_norway_rounds(t: usize, year: u32, k: &mut i32, rounds: &[NorwayRound]) {
    for &(day, month, years, weekday, code, slots) in rounds {
        exe::league_round_set_date(t, *k, day as i32, month as i32, years as i32, weekday as i32, code as i32, year, 0);
        for &(j, a, b, c) in slots {
            exe::league_round_set_match_slot(t, *k, j as i32, a as i32, b as i32, c as i32, 0);
        }
        *k += 1;
    }
}

/// Picks between two pairs of rounds by the calendar: the dates `first` and `second` of the league's year (day, month,
/// weekday; Date::Date `exe::date_date_549f70`, which moves a date to the weekday when one is given) and, when `second`
/// falls less than two days after `first` (Date::days_since `exe::date_days_since`), `near`, otherwise `far`.
unsafe fn write_norway_choice(t: usize, year: u32, k: &mut i32, first: (u32, u32, i32), second: (u32, u32, i32), near: &[NorwayRound], far: &[NorwayRound]) {
    let (mut a, mut b) = ([0u16; 4], [0u16; 4]);
    exe::date_date_549f70(&mut a, first.0, first.1, year, first.2 as u32);
    exe::date_date_549f70(&mut b, second.0, second.1, year, second.2 as u32);
    let days = exe::date_days_since(b.as_ptr() as usize, a[0] as u32 | (a[1] as u32) << 16, a[2] as u32 | (a[3] as u32) << 16);
    write_norway_rounds(t, year, k, if days < 2 { near } else { far });
}

/// The rounds of a Norwegian league season (`start`, the two spring choices (late April, mid-May), the rounds of
/// early May between them, and `end`) into a fresh table of `n` rounds. `*out` (when given) = 1, `*rounds` = n,
/// `*extra` = 0; out of memory, the error box at `row` of `file` and 0. Returns the table and the rounds written.
unsafe fn norway_league_schedule(this: League, n: i16, rounds: *mut i16, extra: *mut i16, out: *mut i32, start: &[NorwayRound], end: &[NorwayRound], file: usize, row: u32) -> (usize, i32) {
    if !out.is_null() {
        *out = 1;
    }
    *rounds = n;
    *extra = 0;
    let t = exe::crt_malloc(n as u32 * 0x41);
    if t == 0 {
        show_error_at(file, row);
        return (0, 0);
    }
    let year = this.year() as u16 as u32;
    let mut k = 0;
    write_norway_rounds(t, year, &mut k, start);
    write_norway_choice(t, year, &mut k, (30, 3, -1), (3, 4, 6), &NORWAY_LATE_APRIL_NEAR, &NORWAY_LATE_APRIL_FAR); // cx12_13: 30 April or 3 May
    write_norway_rounds(t, year, &mut k, &NORWAY_EARLY_MAY);
    write_norway_choice(t, year, &mut k, (13, 4, 2), (16, 4, -1), &NORWAY_MID_MAY_NEAR, &NORWAY_MID_MAY_FAR);
    write_norway_rounds(t, year, &mut k, end);
    (t, k)
}

/// The Norwegian Premier Division schedule: only stage −1 (the byte; otherwise 0 with nothing written), 26 rounds from
/// April to October, with one set of dates in even years and another in odd ones (the signed year's remainder by 2).
///
/// Defect of the original: in odd years round 0's first match slot is written twice, the fixed match (5, 1, 1) and then
/// the terminator over it, so that round keeps no fixed match.
/// Original: `NorPrm::create_schedule` (0x7917c0, nor_prm.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nor_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    let this = League(this);
    let odd = this.year() as i32 % 2 != 0;
    let (start, end): (&[NorwayRound], &[NorwayRound]) = if odd {
        (&NORWEGIAN_PREMIER_ODD_START, &NORWEGIAN_PREMIER_ODD_END)
    } else {
        (&NORWEGIAN_PREMIER_EVEN_START, &NORWEGIAN_PREMIER_EVEN_END) // cx12_13: the even years
    };
    covered!("cx12_13 norwegian premier schedule");
    norway_league_schedule(this, 0x1a, rounds, extra, out, start, end, 0xa6977c, 0xa0).0
}

/// The Norwegian First Division play-off against a Premier Division club: drawn on 19 October, one
/// two-legged tie from 25 October (weekday 6), seven days apart.
pub(crate) const NORWEGIAN_PLAY_OFF: EngCcCupCupRound = cql10r((0x13, 9, 0, 0), (0x19, 9, 0, 6, 1, 0), 0xa0, (0, 0, 3), 5, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0));

/// The Norwegian First Division schedule. Stage −1: 30 rounds when 16 clubs have the First Division as their
/// `club::DIVISION` (all the clubs of the database, `G_CLUB_COUNT`, counted), otherwise 26, from 19 April to 4 October;
/// past the rounds, a count different from the rounds written shows the box at line 0x211 (never with these tables).
/// Stage 0: the play-off (`*out` (when given) = 0, `*rounds` = 1, `*extra` = 0, a cup round table of 0x68 bytes with
/// `NORWEGIAN_PLAY_OFF`; out of memory the box at line 0x224 and 0). Other stages: 0.
///
/// Defect of the original: with 16 clubs, 17 June is written for two rounds in a row.
/// Original: `NorFirst::create_schedule` (0x790040, nor_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nor_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            let first = read_u32(KEY_CLUB_COMP_NORWEGIAN_FIRST_DIVISION);
            let mut clubs = 0i16;
            for k in 0..read_i32(G_CLUB_COUNT).max(0) as isize {
                let division = club_at(k).division().addr();
                if division != 0 && read_u32(division) == first {
                    clubs = clubs.wrapping_add(1);
                }
            }
            let (n, end): (i16, &[NorwayRound]) = if clubs == 0x10 { (0x1e, &NORWEGIAN_FIRST_END_16) } else { (0x1a, &NORWEGIAN_FIRST_END_14) }; // cx12_13: 16 clubs, 30 rounds
            let (t, written) = norway_league_schedule(this, n, rounds, extra, out, &NORWEGIAN_FIRST_START, end, 0xa69740, 0xc1);
            if t != 0 && written as i16 != n {
                show_error_at(0xa69740, 0x211);
            }
            covered!("cx12_13 norwegian first schedule");
            t
        }
        0 => {
            if !out.is_null() {
                *out = 0;
            }
            *rounds = 1;
            *extra = 0;
            let t = exe::crt_malloc(0x68);
            if t == 0 {
                show_error_at(0xa69740, 0x224);
                return 0;
            }
            covered!("cx12_13 norwegian play-off round");
            eng_cc_cup_write_cup_round(t, 0, &NORWEGIAN_PLAY_OFF, this.year() as i32 as u32);
            t
        }
        _ => 0,
    }
}

/// Comparer for `nor_prm_create_schedule`: side by side over the outputs and the written table (`compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_nor_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("nor_prm_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nor_prm_create_schedule(this, stage, r, e, l))
}

/// Comparer for `nor_first_create_schedule`: the same.
pub(crate) unsafe extern "thiscall" fn compare_nor_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_c1_schedule("nor_first_create_schedule", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, l| nor_first_create_schedule(this, stage, r, e, l))
}

/// The two rounds when 3 May (moved to weekday 6) falls less than two days after 30 April: 26 April and 30 April (midweek).
pub(crate) const NORWAY_LATE_APRIL_NEAR: [NorwayRound; 2] = [
    (26, 3, 0, 6, 1, &[(0, -1, -1, -1)]), // 26 apr
    (30, 3, 0, -1, 2, &[(0, -1, -1, -1)]), // 30 apr
];

/// Otherwise: 30 April (midweek) and 3 May.
pub(crate) const NORWAY_LATE_APRIL_FAR: [NorwayRound; 2] = [
    (30, 3, 0, -1, 2, &[(0, -1, -1, -1)]), // 30 apr
    (3, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 3 may
];

/// The two rounds between the choices: 6 May (midweek) and 10 May.
pub(crate) const NORWAY_EARLY_MAY: [NorwayRound; 2] = [
    (6, 4, 0, 2, 2, &[(0, -1, -1, -1)]), // 6 may
    (10, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 10 may
];

/// The two rounds when 16 May falls less than two days after 13 May (moved to weekday 2): 16 May and 20 May (midweek).
pub(crate) const NORWAY_MID_MAY_NEAR: [NorwayRound; 2] = [
    (16, 4, 0, -1, 1, &[(0, -1, -1, -1)]), // 16 may
    (20, 4, 0, 2, 2, &[(0, -1, -1, -1)]), // 20 may
];

/// Otherwise: 13 May (midweek) and 16 May.
pub(crate) const NORWAY_MID_MAY_FAR: [NorwayRound; 2] = [
    (13, 4, 0, 2, 2, &[(0, -1, -1, -1)]), // 13 may
    (16, 4, 0, -1, 1, &[(0, -1, -1, -1)]), // 16 may
];

/// The Norwegian Premier Division's first rounds in odd years.
pub(crate) const NORWEGIAN_PREMIER_ODD_START: [NorwayRound; 2] = [
    (12, 3, 0, 6, 1, &[(0, 5, 1, 1), (0, -1, -1, -1)]), // 12 apr
    (19, 3, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 19 apr
];

/// The Norwegian Premier Division's rounds after mid-May in odd years.
pub(crate) const NORWEGIAN_PREMIER_ODD_END: [NorwayRound; 18] = [
    (24, 4, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 24 may
    (31, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 31 may
    (14, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 14 jun
    (17, 5, 0, 2, 2, &[(0, -1, -1, -1)]), // 17 jun
    (21, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 21 jun
    (28, 5, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 28 jun
    (5, 6, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 5 jul
    (12, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 12 jul
    (26, 6, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 26 jul
    (2, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 2 aug
    (9, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 9 aug
    (23, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 23 aug
    (30, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 30 aug
    (13, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 13 sep
    (20, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 20 sep
    (27, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 27 sep
    (4, 9, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 4 oct
    (18, 9, 0, 6, 1, &[(0, -1, -1, -1)]), // 18 oct
];

/// The Norwegian Premier Division's first rounds in even years.
pub(crate) const NORWEGIAN_PREMIER_EVEN_START: [NorwayRound; 5] = [
    (5, 3, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 5 apr
    (8, 3, 0, 2, 2, &[(0, -1, -1, -1)]), // 8 apr
    (12, 3, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 12 apr
    (15, 3, 0, 2, 2, &[(0, -1, -1, -1)]), // 15 apr
    (19, 3, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 19 apr
];

/// The Norwegian Premier Division's rounds after mid-May in even years.
pub(crate) const NORWEGIAN_PREMIER_EVEN_END: [NorwayRound; 15] = [
    (24, 4, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 24 may
    (31, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 31 may
    (26, 6, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 26 jul
    (29, 6, 0, 2, 2, &[(0, -1, -1, -1)]), // 29 jul
    (2, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 2 aug
    (5, 7, 0, 2, 2, &[(0, -1, -1, -1)]), // 5 aug
    (9, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 9 aug
    (23, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 23 aug
    (30, 7, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 30 aug
    (13, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 13 sep
    (20, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 20 sep
    (27, 8, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 27 sep
    (30, 8, 0, 2, 2, &[(0, -1, -1, -1)]), // 30 sep
    (4, 9, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 4 oct
    (18, 9, 0, 6, 1, &[(0, 5, 1, 1), (1, -1, -1, -1)]), // 18 oct
];

/// The Norwegian First Division's first round, 19 April.
pub(crate) const NORWEGIAN_FIRST_START: [NorwayRound; 1] = [
    (19, 3, 0, 6, 1, &[(0, -1, -1, -1)]), // 19 apr
];

/// The Norwegian First Division's rounds after mid-May with 14 clubs (26 rounds in all).
pub(crate) const NORWEGIAN_FIRST_END_14: [NorwayRound; 19] = [
    (24, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 24 may
    (27, 4, 0, 2, 2, &[(0, -1, -1, -1)]), // 27 may
    (31, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 31 may
    (14, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 14 jun
    (17, 5, 0, 2, 2, &[(0, -1, -1, -1)]), // 17 jun
    (21, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 21 jun
    (28, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 28 jun
    (5, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 5 jul
    (12, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 12 jul
    (26, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 26 jul
    (2, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 2 aug
    (5, 7, 0, 2, 2, &[(0, -1, -1, -1)]), // 5 aug
    (9, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 9 aug
    (23, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 23 aug
    (30, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 30 aug
    (13, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 13 sep
    (20, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 20 sep
    (27, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 27 sep
    (4, 9, 0, 6, 1, &[(0, -1, -1, -1)]), // 4 oct
];

/// The Norwegian First Division's rounds after mid-May with 16 clubs (30 rounds in all; 17 June twice).
pub(crate) const NORWEGIAN_FIRST_END_16: [NorwayRound; 23] = [
    (24, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 24 may
    (27, 4, 0, 2, 2, &[(0, -1, -1, -1)]), // 27 may
    (31, 4, 0, 6, 1, &[(0, -1, -1, -1)]), // 31 may
    (7, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 7 jun
    (14, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 14 jun
    (17, 5, 0, 2, 2, &[(0, -1, -1, -1)]), // 17 jun
    (17, 5, 0, 2, 2, &[(0, -1, -1, -1)]), // 17 jun
    (21, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 21 jun
    (28, 5, 0, 6, 1, &[(0, -1, -1, -1)]), // 28 jun
    (5, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 5 jul
    (12, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 12 jul
    (26, 6, 0, 6, 1, &[(0, -1, -1, -1)]), // 26 jul
    (29, 6, 0, 2, 2, &[(0, -1, -1, -1)]), // 29 jul
    (2, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 2 aug
    (5, 7, 0, 2, 2, &[(0, -1, -1, -1)]), // 5 aug
    (9, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 9 aug
    (16, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 16 aug
    (23, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 23 aug
    (30, 7, 0, 6, 1, &[(0, -1, -1, -1)]), // 30 aug
    (13, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 13 sep
    (20, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 20 sep
    (27, 8, 0, 6, 1, &[(0, -1, -1, -1)]), // 27 sep
    (4, 9, 0, 6, 1, &[(0, -1, -1, -1)]), // 4 oct
];
