// ---------- rx10_5: the German Regionalliga schedule, South stage and club array (ger_regional.cpp) ----------

/// Rule: the Regionalliga schedule: the North (stage −1) has 34 rounds from 30 July to 20 May, 38 in 2000 (four more
/// rounds, three of them midweek); the South (stage 0) has 34 rounds over the same season, with rounds 21 and 22 both
/// on 25 February (a defect of the original: two rounds on one day).
///
/// `GerRegional::create_schedule(stage, &rounds, &extra, &league)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage 1 or
/// above (signed byte): 0 with nothing written. Otherwise the round count is 38 for stage −1 in 2000 and 34 else;
/// league = 1 (if asked); the table (`exe::crt_malloc` of 0x41 × the count; none: the box at line 0xb1 and 0, with no
/// count nor extra written); stage −1: extra 0x453 and the dates of `REGIONAL_NORTH_ROUNDS` (the marked ones only in
/// the 38-round year), any other stage: extra 0x456 and the dates of `REGIONAL_SOUTH_ROUNDS`
/// (`exe::league_round_set_date`(t, i, day, month, years, weekday, code, year, 0), cdecl; the year a word); then the
/// count and, for every round, one empty match slot (`exe::league_round_set_match_slot`(t, i, 0, −1, −1, −1, 0)).
/// Returns the table. (One of the 2000 rounds, 10 June, sits between 4 and 11 February: [inference] a typo for 10
/// February; the year never comes back in a career.)
/// Original: `GerRegional::create_schedule` (0x5dcef0, ger_regional.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_regional_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    if stage as u8 as i8 >= 1 {
        return 0;
    }
    let main = stage as u8 == 0xff;
    let n: i16 = if l.year() == 0x7d0 && main { 0x26 } else { 0x22 };
    if !league.is_null() {
        *league = 1;
    }
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x41));
    if t == 0 {
        show_error_at(0x9cd488, 0xb1);
        return 0;
    }
    let year = l.year() as u16 as u32;
    let mut i = 0i32;
    let mut date = |r: (i8, i8, i8, i8, i8)| {
        exe::league_round_set_date(t, i, r.0 as i32, r.1 as i32, r.2 as i32, r.3 as i32, r.4 as i32, year, 0);
        i += 1;
    };
    if main {
        *extra = 0x453;
        covered!("rx10_5 regionalliga north calendar");
        for &(d, m, y, w, c, only_38) in REGIONAL_NORTH_ROUNDS.iter() {
            if !only_38 || n == 0x26 {
                date((d, m, y, w, c));
            }
        }
    } else {
        *extra = 0x456;
        covered!("rx10_5 regionalliga south calendar");
        for &r in REGIONAL_SOUTH_ROUNDS.iter() {
            date(r);
        }
    }
    *rounds = n;
    for k in 0..n as i32 {
        exe::league_round_set_match_slot(t, k, 0, -1, -1, -1, 0);
    }
    t
}

/// The schedule comparer of a league whose every stage is a league table: side by side (the original, then the copy
/// into its own outputs and table, freed after): the outputs, the error flag and the bytes each round writes must be
/// equal.
pub(crate) unsafe fn compare_section_schedule(name: &str, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32,
                       original: impl FnOnce() -> usize, copy: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let (mut r2, mut e2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let pl2 = if league.is_null() { core::ptr::null_mut() } else { &mut l2 as *mut i32 };
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, original, || copy(&mut r2, &mut e2, pl2));
    let outs = (a == 0 && b == 0) || (*rounds == r2 && *extra == e2 && (league.is_null() || *league == l2));
    let table = if a != 0 && b != 0 {
        let n = *rounds as usize;
        written_league_round_bytes(a, a, n) == written_league_round_bytes(b, a, n)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(same && outs && table, || format!("{}(stage {}): outs {} table {}", name, stage as u8 as i8, outs, table));
    a
}

/// Comparer for `ger_regional_create_schedule` (`compare_section_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_section_schedule("ger_regional_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ger_regional_create_schedule(this, stage, r, e, l))
}

/// The Regionalliga South stage: 18 clubs whose division is this league, fixed places (2 up, 0, 0, 4 down), the B
/// teams marked, the boxes of ger_regional.cpp at lines 0x1f2, 0x203, 0x210 and 0x21b.
pub(crate) const REGIONAL_SOUTH: SectionStage = SectionStage { count: 0x12, division: None, places: Some([2, 0, 0, 4]), mark_b_teams: true, file: 0x9cd488,
    lines: [0x1f2, 0x203, 0x210, 0x21b] };

/// Rule: the Regionalliga South is a stage of its own: the 18 Regionalliga clubs of the South section, home and away,
/// two going up and four down; a B team whose senior club is above cannot go up.
///
/// `create_section_stage` with `REGIONAL_SOUTH` and the South section (0x9cf560): the division compared with this
/// league's record as a pointer, the places from a local [2, 0, 0, 4] (not `league::PROMOTED_COUNT`).
/// Original: `GerRegional::create_south_stage` (0x5ddc50, ger_regional.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn ger_regional_create_south_stage(this: usize, k: u32) {
    covered!("rx10_5 regionalliga south stage");
    create_section_stage(this, k as u8, 0x9cf560, &REGIONAL_SOUTH)
}

/// Comparer for `ger_regional_create_south_stage`: only the original, with the plan (`compare_section_stage`).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_create_south_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_section_stage("ger_regional_create_south_stage", this, k as u8, 0x9cf560, &REGIONAL_SOUTH, || o(this, k))
}

/// Rule: the Regionalliga's clubs are the North table's followed by the South stage's: 36 (37 in 2000).
///
/// `divided_league_club_array` with 36 + (the year is 2000), one section bounded by the stage's own count and the box
/// at line 0x24e.
/// Original: `GerRegional::get_club_array` (0x5ddff0, ger_regional.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn ger_regional_get_club_array(this: usize, out: *mut usize, count: *mut i16) {
    covered!("rx10_5 regionalliga clubs");
    divided_league_club_array(this, out, count, 0x24 + (League(this).year() == 0x7d0) as i16, 1, true, 0x9cd488, 0x24e)
}

/// Comparer for `ger_regional_get_club_array`: side by side (`compare_divided_league_club_array`, all the entries).
pub(crate) unsafe extern "thiscall" fn compare_ger_regional_get_club_array(this: usize, out: *mut usize, count: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I2%%);
    compare_divided_league_club_array("ger_regional_get_club_array", this, out, count, 0x25, || o(this, out, count), |a, n| ger_regional_get_club_array(this, a, n))
}

/// The Regionalliga North (stage −1): the 38 rounds of 2000; those marked true are left out in the other years (34 rounds).
pub(crate) const REGIONAL_NORTH_ROUNDS: [(i8, i8, i8, i8, i8, bool); 38] = [
    (30, 6, 0, 6, 1, false), // 0: 30 Jul
    (6, 7, 0, 6, 1, false), // 1: 6 Aug
    (13, 7, 0, 6, 1, false), // 2: 13 Aug
    (20, 7, 0, 6, 1, false), // 3: 20 Aug
    (23, 7, 0, 2, 2, true), // 4: 23 Aug
    (3, 8, 0, 6, 1, false), // 5: 3 Sep
    (10, 8, 0, 6, 1, false), // 6: 10 Sep
    (17, 8, 0, 6, 1, false), // 7: 17 Sep
    (24, 8, 0, 6, 1, false), // 8: 24 Sep
    (1, 9, 0, 6, 1, false), // 9: 1 Oct
    (8, 9, 0, 6, 1, false), // 10: 8 Oct
    (15, 9, 0, 6, 1, false), // 11: 15 Oct
    (18, 9, 0, 2, 2, false), // 12: 18 Oct
    (22, 9, 0, 6, 1, false), // 13: 22 Oct
    (29, 9, 0, 6, 1, false), // 14: 29 Oct
    (5, 10, 0, 6, 1, false), // 15: 5 Nov
    (12, 10, 0, 6, 1, false), // 16: 12 Nov
    (19, 10, 0, 6, 1, false), // 17: 19 Nov
    (26, 10, 0, 6, 1, false), // 18: 26 Nov
    (3, 11, 0, 6, 1, false), // 19: 3 Dec
    (10, 11, 0, 6, 1, false), // 20: 10 Dec
    (28, 0, 1, 6, 1, false), // 21: 28 Jan (+1 year)
    (4, 1, 1, 6, 1, false), // 22: 4 Feb (+1 year)
    (10, 5, 1, 6, 1, true), // 23: 10 Jun (+1 year)
    (11, 1, 1, 6, 1, false), // 24: 11 Feb (+1 year)
    (14, 1, 1, 2, 2, false), // 25: 14 Feb (+1 year)
    (18, 1, 1, 6, 1, false), // 26: 18 Feb (+1 year)
    (25, 1, 1, 6, 1, false), // 27: 25 Feb (+1 year)
    (4, 2, 1, 6, 1, true), // 28: 4 Mar (+1 year)
    (11, 2, 1, 6, 1, false), // 29: 11 Mar (+1 year)
    (18, 2, 1, 6, 1, false), // 30: 18 Mar (+1 year)
    (1, 3, 1, 6, 1, false), // 31: 1 Apr (+1 year)
    (8, 3, 1, 6, 1, false), // 32: 8 Apr (+1 year)
    (16, 3, 1, 0, 2, true), // 33: 16 Apr (+1 year)
    (22, 3, 1, 6, 1, false), // 34: 22 Apr (+1 year)
    (6, 4, 1, 6, 1, false), // 35: 6 May (+1 year)
    (13, 4, 1, 6, 1, false), // 36: 13 May (+1 year)
    (20, 4, 1, 6, 1, false), // 37: 20 May (+1 year)
];

/// The Regionalliga South (stage 0 and any other below 1): 34 rounds.
pub(crate) const REGIONAL_SOUTH_ROUNDS: [(i8, i8, i8, i8, i8); 34] = [
    (30, 6, 0, 6, 1), // 0: 30 Jul
    (6, 7, 0, 6, 1), // 1: 6 Aug
    (13, 7, 0, 6, 1), // 2: 13 Aug
    (20, 7, 0, 6, 1), // 3: 20 Aug
    (27, 7, 0, 6, 1), // 4: 27 Aug
    (3, 8, 0, 6, 1), // 5: 3 Sep
    (10, 8, 0, 6, 1), // 6: 10 Sep
    (17, 8, 0, 6, 1), // 7: 17 Sep
    (24, 8, 0, 6, 1), // 8: 24 Sep
    (1, 9, 0, 6, 1), // 9: 1 Oct
    (8, 9, 0, 6, 1), // 10: 8 Oct
    (15, 9, 0, 6, 1), // 11: 15 Oct
    (22, 9, 0, 6, 1), // 12: 22 Oct
    (25, 9, 0, 6, 1), // 13: 25 Oct
    (29, 9, 0, 6, 1), // 14: 29 Oct
    (5, 10, 0, 6, 1), // 15: 5 Nov
    (12, 10, 0, 6, 1), // 16: 12 Nov
    (19, 10, 0, 6, 1), // 17: 19 Nov
    (26, 10, 0, 6, 1), // 18: 26 Nov
    (3, 11, 0, 6, 1), // 19: 3 Dec
    (10, 11, 0, 6, 1), // 20: 10 Dec
    (25, 1, 1, 6, 1), // 21: 25 Feb (+1 year)
    (25, 1, 1, 6, 1), // 22: 25 Feb (+1 year)
    (4, 2, 1, 6, 1), // 23: 4 Mar (+1 year)
    (11, 2, 1, 6, 1), // 24: 11 Mar (+1 year)
    (18, 2, 1, 6, 1), // 25: 18 Mar (+1 year)
    (1, 3, 1, 6, 1), // 26: 1 Apr (+1 year)
    (8, 3, 1, 6, 1), // 27: 8 Apr (+1 year)
    (15, 3, 1, 6, 1), // 28: 15 Apr (+1 year)
    (22, 3, 1, 6, 1), // 29: 22 Apr (+1 year)
    (29, 3, 1, 6, 1), // 30: 29 Apr (+1 year)
    (6, 4, 1, 6, 1), // 31: 6 May (+1 year)
    (13, 4, 1, 6, 1), // 32: 13 May (+1 year)
    (20, 4, 1, 6, 1), // 33: 20 May (+1 year)
];
