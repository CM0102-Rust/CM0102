// ---------- rx10_8: the Portuguese Segunda B schedule, zone stages and club array (por_second_b.cpp) ----------

/// Rule: the Segunda B schedule, the same for the three zones: 38 rounds from 29 August to 11 June, mostly on
/// weekday 6 [inference: Sunday], some on weekday 5, and two midweek rounds.
///
/// `PorSecondB::create_schedule(stage, &rounds, &extra, &league)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage 2 or
/// above (signed byte): 0 with nothing written. Otherwise league = 1 (if asked), 38 rounds, extra 0x453 for stage −1,
/// 0x457 for stage 0 and 0x456 for any other, all written before the table (`exe::crt_malloc`(0x9a6); none: the box at
/// line 0xb9 and 0); each round's date and its empty match slot (`eng_second_set_league_rounds` with
/// `SEGUNDA_B_ROUNDS` and the year, a word). Returns the table.
/// Original: `PorSecondB::create_schedule` (0x7d1c40, por_second_b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn por_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let l = League(this);
    let s = stage as u8 as i8;
    if s >= 2 {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x26;
    *extra = match s {
        -1 => 0x453,
        0 => 0x457,
        _ => 0x456,
    };
    let t = exe::crt_malloc(0x9a6);
    if t == 0 {
        show_error_at(0xa78798, 0xb9);
        return 0;
    }
    covered!("rx10_8 segunda b calendar");
    eng_second_set_league_rounds(t, l.year() as u16 as u32, 0, &SEGUNDA_B_ROUNDS);
    t
}

/// Comparer for `por_second_b_create_schedule` (`compare_section_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_section_schedule("por_second_b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| por_second_b_create_schedule(this, stage, r, e, l))
}

/// A Segunda B zone stage: 20 clubs of the Segunda B (`KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B`), the league's own
/// places, no B team marks, the boxes of por_second_b.cpp at lines 0x246, 0x257, 0x264 and 0x26f.
pub(crate) const SEGUNDA_B_ZONE: SectionStage = SectionStage { count: 0x14, division: Some(KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B), places: None, mark_b_teams: false,
    file: 0xa78798, lines: [0x246, 0x257, 0x264, 0x26f] };

/// The zone of Segunda B stage k: Central for 0, South for any other.
pub(crate) fn segunda_b_zone(k: u8) -> usize {
    if k == 0 { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_CENTRAL } else { KEY_CLUB_COMP_PORTUGUESE_SECOND_DIVISION_B_SOUTH }
}

/// Rule: the Segunda B Central (stage 0) and South (stage 1) zones are stages of their own, 20 clubs each, home and
/// away, with the league's places (one up, four down).
///
/// `create_section_stage` with `SEGUNDA_B_ZONE` and the zone of `segunda_b_zone(k)` (the index a byte; the round table
/// asked with the whole dword).
/// Original: `PorSecondB::create_zone_stage` (0x7d2870, por_second_b.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn por_second_b_create_zone_stage(this: usize, k: u32) {
    covered!("rx10_8 segunda b zone stage");
    create_section_stage(this, k as u8, segunda_b_zone(k as u8), &SEGUNDA_B_ZONE)
}

/// Comparer for `por_second_b_create_zone_stage`: only the original, with the plan (`compare_section_stage`).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_create_zone_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_section_stage("por_second_b_create_zone_stage", this, k as u8, segunda_b_zone(k as u8), &SEGUNDA_B_ZONE, || o(this, k))
}

/// Rule: the Segunda B's club array: 60 places, the North table's 20 clubs first; but both zone stages are copied from
/// index 20 (the South over the Central), so the Central zone's clubs are lost and the last 20 places are never
/// written (a defect of the original).
///
/// `divided_league_club_array` with 60, two sections bounded by this league's own count and the box at line 0x290.
/// Original: `PorSecondB::get_club_array` (0x7d2b80, por_second_b.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn por_second_b_get_club_array(this: usize, out: *mut usize, count: *mut i16) {
    covered!("rx10_8 segunda b clubs");
    divided_league_club_array(this, out, count, 0x3c, 2, false, 0xa78798, 0x290)
}

/// Comparer for `por_second_b_get_club_array`: side by side (`compare_divided_league_club_array`, the 40 entries the
/// routine writes).
pub(crate) unsafe extern "thiscall" fn compare_por_second_b_get_club_array(this: usize, out: *mut usize, count: *mut i16) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize, *mut i16)>(%%I2%%);
    compare_divided_league_club_array("por_second_b_get_club_array", this, out, count, 0x28, || o(this, out, count), |a, n| por_second_b_get_club_array(this, a, n))
}

/// The Segunda B rounds (every stage): 38, from 29 August to 11 June, each with one empty match slot.
pub(crate) const SEGUNDA_B_ROUNDS: [EngSecondScheduleRound; 38] = [
    (29, 7, 0, 6, 1, &[(-1, -1, -1)]), // 0: 29 Aug
    (12, 8, 0, 6, 1, &[(-1, -1, -1)]), // 1: 12 Sep
    (26, 8, 0, 6, 1, &[(-1, -1, -1)]), // 2: 26 Sep
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 3: 2 Oct
    (17, 9, 0, 6, 1, &[(-1, -1, -1)]), // 4: 17 Oct
    (24, 9, 0, 6, 1, &[(-1, -1, -1)]), // 5: 24 Oct
    (31, 9, 0, 6, 1, &[(-1, -1, -1)]), // 6: 31 Oct
    (7, 10, 0, 6, 1, &[(-1, -1, -1)]), // 7: 7 Nov
    (21, 10, 0, 6, 1, &[(-1, -1, -1)]), // 8: 21 Nov
    (28, 10, 0, 6, 1, &[(-1, -1, -1)]), // 9: 28 Nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 10: 4 Dec
    (8, 11, 0, 2, 2, &[(-1, -1, -1)]), // 11: 8 Dec
    (12, 11, 0, 6, 1, &[(-1, -1, -1)]), // 12: 12 Dec
    (19, 11, 0, 6, 1, &[(-1, -1, -1)]), // 13: 19 Dec
    (2, 0, 1, 6, 1, &[(-1, -1, -1)]), // 14: 2 Jan (+1 year)
    (16, 0, 1, 6, 1, &[(-1, -1, -1)]), // 15: 16 Jan (+1 year)
    (23, 0, 1, 6, 1, &[(-1, -1, -1)]), // 16: 23 Jan (+1 year)
    (30, 0, 1, 6, 1, &[(-1, -1, -1)]), // 17: 30 Jan (+1 year)
    (6, 1, 1, 6, 1, &[(-1, -1, -1)]), // 18: 6 Feb (+1 year)
    (13, 1, 1, 6, 1, &[(-1, -1, -1)]), // 19: 13 Feb (+1 year)
    (20, 1, 1, 6, 1, &[(-1, -1, -1)]), // 20: 20 Feb (+1 year)
    (27, 1, 1, 6, 1, &[(-1, -1, -1)]), // 21: 27 Feb (+1 year)
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 22: 4 Mar (+1 year)
    (12, 2, 1, 6, 1, &[(-1, -1, -1)]), // 23: 12 Mar (+1 year)
    (19, 2, 1, 6, 1, &[(-1, -1, -1)]), // 24: 19 Mar (+1 year)
    (26, 2, 1, 6, 1, &[(-1, -1, -1)]), // 25: 26 Mar (+1 year)
    (2, 3, 1, 6, 1, &[(-1, -1, -1)]), // 26: 2 Apr (+1 year)
    (9, 3, 1, 6, 1, &[(-1, -1, -1)]), // 27: 9 Apr (+1 year)
    (16, 3, 1, 6, 1, &[(-1, -1, -1)]), // 28: 16 Apr (+1 year)
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 29: 22 Apr (+1 year)
    (30, 3, 1, 6, 1, &[(-1, -1, -1)]), // 30: 30 Apr (+1 year)
    (7, 4, 1, 6, 1, &[(-1, -1, -1)]), // 31: 7 May (+1 year)
    (10, 4, 1, 2, 2, &[(-1, -1, -1)]), // 32: 10 May (+1 year)
    (14, 4, 1, 6, 1, &[(-1, -1, -1)]), // 33: 14 May (+1 year)
    (20, 4, 1, 5, 1, &[(-1, -1, -1)]), // 34: 20 May (+1 year)
    (28, 4, 1, 6, 1, &[(-1, -1, -1)]), // 35: 28 May (+1 year)
    (4, 5, 1, 6, 1, &[(-1, -1, -1)]), // 36: 4 Jun (+1 year)
    (11, 5, 1, 6, 1, &[(-1, -1, -1)]), // 37: 11 Jun (+1 year)
];
