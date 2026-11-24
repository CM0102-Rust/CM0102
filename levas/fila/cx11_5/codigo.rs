// ---------- cx11_5: the Serie C2 groups A and B schedules (ita_ser_c2a.cpp, ita_ser_c2b.cpp) ----------

/// The Serie C2/A schedule, the same as Serie C1's (`create_c1_schedule`, checked instruction by instruction: only the
/// source file changes; the box lines are C1/A's). By stage: −1, the league (34 rounds from 5 September to 7 May, extra
/// 0); 0, the promotion play-off (2 two-legged rounds, the semi-finals on 21 and 28 May and the final on 4 and 11 June,
/// extra 0xa0); 1, the relegation play-out (1 two-legged round on the semi-final dates, extra 0); any other stage
/// returns 0 and writes nothing. Out of memory: the ita_ser_c2a.cpp box of the stage's line (0xab, 0x155, 0x18f) and 0.
/// Original: `ItaSerC2a::create_schedule` (0x6531a0, ita_ser_c2a.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    covered!("cx11_5 serie c2 schedule");
    create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea654, rows: [0xab, 0x155, 0x18f] }) // cx11_5: the C2/A schedule
}

/// The Serie C2/B schedule, the same as C2/A's (ita_ser_c2b.cpp, the same box lines).
/// Original: `ItaSerC2b::create_schedule` (0x657660, ita_ser_c2b.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ita_ser_c2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_c1_schedule(League(this), stage, rounds, extra, league, &C1BracketVariant { file: 0x9ea75c, rows: [0xab, 0x155, 0x18f] }) // cx11_5: the C2/B schedule
}

/// Comparer for `ita_ser_c2a_create_schedule`: side by side over the outputs and the written table (`compare_c1_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2a_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_c1_schedule("ita_ser_c2a_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_c2a_create_schedule(this, stage, r, e, l))
}

/// Comparer for `ita_ser_c2b_create_schedule`: the same.
pub(crate) unsafe extern "thiscall" fn compare_ita_ser_c2b_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_c1_schedule("ita_ser_c2b_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| ita_ser_c2b_create_schedule(this, stage, r, e, l))
}
