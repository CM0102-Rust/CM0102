// ---------- sbF_1: the German League Cup round table and the Dutch Cup schedule (ger_lge_cup.cpp, hol_cup.cpp) ----------

/// The German League Cup rounds in an odd year: the quarter-finals drawn on 1 July and played on 9 July (4 clubs, 2
/// ties, tie rules 0x101), the semi-finals drawn on 11 July and played on 12 July (4 clubs, 2 ties, two of them new:
/// entries 4 and 5, tie rules 0x501), the final drawn on 14 July and played on 16 July at a neutral ground
/// [inference: venue 0x40]; all single matches, category 6, no prize per round (−1).
pub(crate) const GERMAN_LEAGUE_CUP_ODD_YEAR_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((1, 6, 0, 5), (9, 6, 0, 4, 2, 8), 0x78, (0, 0x101, 0), 6, (4, 2, 4, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // sbF1: the odd-year quarter-finals
    cql10r((0xb, 6, 0, 6), (0xc, 6, 0, 0, 2, 8), 0x82, (0, 0x501, 0), 6, (4, 2, 2, 4), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xe, 6, 0, 2), (0x10, 6, 0, 4, 2, 0x40), 0x96, (0, 1, 0), 6, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// The German League Cup rounds in an even year (the World Cup summer [inference]): the same rounds about two weeks
/// later: the quarter-finals drawn on 1 July and played on 24 July, the semi-finals drawn on 28 July and played on
/// 29 July, the final drawn on 31 July and played on 1 August.
pub(crate) const GERMAN_LEAGUE_CUP_EVEN_YEAR_ROUNDS: [EngCcCupCupRound; 3] = [
    cql10r((1, 6, 0, 5), (0x18, 6, 0, 0, 2, 8), 0x78, (0, 0x101, 0), 6, (4, 2, 4, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1c, 6, 0, 4), (0x1d, 6, 0, 5, 2, 8), 0x82, (0, 0x501, 0), 6, (4, 2, 2, 4), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1f, 6, 0, 0), (1, 7, 0, 1, 2, 0x40), 0x96, (0, 1, 0), 6, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// The match-day slots of the German League Cup quarter- and semi-finals (round, weekday, time, code; slot 0 holds the
/// day, slot 1 is left empty): (odd year, even year).
pub(crate) const GERMAN_LEAGUE_CUP_SLOTS: [[(i32, i32, i32, i32); 2]; 2] = [[(0, 5, 2, 3), (1, 1, 2, 3)], [(0, 3, 2, 3), (1, 6, 1, 3)]];

/// The German League Cup round table: `eng_cc_cup_create_single_stage_round_table` (only stage −1; `out` = 0 when
/// given, 3 rounds, extra 0; out of memory, the box at line 0x98 of ger_lge_cup.cpp and 0) with the rounds of the
/// year's parity, then the match-day slots of rounds 0 and 1 (`exe::cup_set_round_match_day`: slot 0 the day, slot 1
/// empty, home side 0). The original writes each round's slots between its dates and its fields; they do not overlap
/// (+0x23 on).
pub(crate) unsafe fn build_german_league_cup_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let odd = (this.year() as i32) % 2 != 0; // the original's signed remainder of the movsx year
    let table = if odd { &GERMAN_LEAGUE_CUP_ODD_YEAR_ROUNDS } else { &GERMAN_LEAGUE_CUP_EVEN_YEAR_ROUNDS };
    let t = eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, table, 0x9cd450, 0x98, "sbF_1 german league cup rounds");
    if t != 0 {
        for &(round, week, hour, code) in &GERMAN_LEAGUE_CUP_SLOTS[if odd { 0 } else { 1 }] {
            exe::cup_set_round_match_day(t, round, 0, week, hour, code, 0);
            exe::cup_set_round_match_day(t, round, 1, -1, -1, -1, 0);
        }
    }
    t
}

/// Rule: the German League Cup (Ligapokal) has three single-match rounds in July: the quarter-finals with four clubs,
/// the semi-finals where two clubs enter and meet the two winners, and the final at a neutral ground
/// [inference]; in an even year (the World Cup summer [inference]) the matches go about two weeks later, the final on 1
/// August; category 6, no prize per round.
///
/// `build_german_league_cup_round_table` (checked by emulation of the original for every stage −128..127 in an odd and
/// an even year: the outputs, the table size and every byte written).
/// Original: `GerLgeCup::create_round_table` (0x5dc3a0, ger_lge_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ger_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_german_league_cup_round_table(Cup(this), stage, rounds, extra, out)
}

/// Side by side like `compare_cup_round_table`, also comparing the match-day slots 0 and 1 of rounds 0 and 1 (14
/// bytes each from `cup_round::SLOT_VENUE`).
pub(crate) unsafe extern "thiscall" fn compare_ger_lge_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    if stage as u8 != 0xff {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("ger_lge_cup_create_round_table(stage {}) returned {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, equal, _, _) = crate::comparison::side_by_side(&reg, || o(this, stage, rounds, extra, out),
        || build_german_league_cup_round_table(Cup(this), stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let slots = |t: usize| (0..2usize).flat_map(|r| core::slice::from_raw_parts((t + r * cup_round::SIZE + cup_round::SLOT_VENUE) as *const u8, 14).to_vec()).collect::<Vec<u8>>();
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && slots(a) == slots(b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(equal && outs && table, || format!("ger_lge_cup_create_round_table(stage {}): outs {} ({} x {}) table {}", stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// The Dutch Cup groups: three matchdays in August (4, 11 and 18; weekday 2, code 2).
pub(crate) const DUTCH_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 3] = [(4, 7, 2, 2), (0xb, 7, 2, 2), (0x12, 7, 2, 2)]; // sbF1: the Dutch Cup group matchdays

/// The Dutch Cup knockout: six single-match rounds from September to May of the next year: the 1st round drawn on
/// 26 August and played on 25 September (40 clubs, 20 ties, tie rules 0x283), the 2nd drawn 30 September and played 31
/// October (20 clubs), the 3rd drawn 5 November and played 29 January (16 clubs, 6 new from entry 40), the
/// quarter-finals drawn 30 January and played 12 February, the semi-finals drawn 13 February and played 12 April and
/// the final drawn 13 April and played 21 May at a neutral ground [inference: venue 4]; tie rules 0x83 after the 1st
/// round; category 4 up to the 3rd round and 6 after; no prize per round (−1).
pub(crate) const DUTCH_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 6] = [
    cql10r((0x1a, 7, 0, 3), (0x19, 8, 0, 5, 2, 0), 0xa, (0, 0x283, 0), 4, (0x28, 0x14, 0x28, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF1: the Dutch Cup 1st round
    cql10r((0x1e, 8, 0, 3), (0x1f, 9, 0, 6, 2, 0), 0x14, (0, 0x83, 0), 4, (0x14, 0xa, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((5, 10, 0, 4), (0x1d, 0, 1, 5, 1, 0), 0x1e, (0, 0x83, 0), 4, (0x10, 8, 6, 0x28), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1e, 0, 1, 6), (0xc, 1, 1, 5, 2, 0), 0x78, (0, 0x83, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xd, 1, 1, 6), (0xc, 3, 1, 2, 2, 0), 0x82, (0, 0x83, 0), 6, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xd, 3, 1, 3), (0x15, 4, 1, 6, 1, 4), 0x96, (0, 0x83, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Rule: the Dutch Cup groups (stages up to 18) play three matchdays in August, each group a league stage (is_league =
/// 1, label 0x3e9 + stage); the knockout (stage 19) has six single-match rounds: 40 clubs in the 1st round (late
/// September), 20 in the 2nd (October), 16 in the 3rd (January, six new) and then quarter-finals, semi-finals and the
/// final in May.
///
/// `create_schedule` with the knockout at stage 19, the label 0x3e9, `DUTCH_CUP_GROUP_ROUNDS`,
/// `DUTCH_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xa7 and 0xc6 of hol_cup.cpp (checked by emulation of the
/// original for every stage −128..127 in an odd and an even year: the same calls and the same 16 fields per round as
/// ItaCup::create_schedule). Above stage 19: 0.
/// Original: `HolCup::create_schedule` (0x5f36e0, hol_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn hol_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0x13, 0x3e9, &DUTCH_CUP_GROUP_ROUNDS, &DUTCH_CUP_KNOCKOUT_ROUNDS, 0x9cedd4, 0xa7, 0xc6,
        "sbF_1 dutch cup groups", "sbF_1 dutch cup knockout rounds")
}

/// Comparer for `hol_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_hol_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    ita_cup_compare_schedule("hol_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| hol_cup_create_schedule(this, stage, r, e, l), 0x13)
}
