// ---------- sbF_4: the Korean League Cup schedule and staff records (kor_league_cup.cpp) ----------

/// The Korean League Cup groups: ten matchdays from 25 March to 28 April (twice a week: weekday 6 or 5 with code 1,
/// weekday 2 with code 2).
pub(crate) const KOREAN_LEAGUE_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 10] = [(0x19, 2, 6, 1), (0x1c, 2, 2, 2), (0x1f, 2, 5, 1), (4, 3, 2, 2), (7, 3, 5, 1),
    (0xb, 3, 2, 2), (0xe, 3, 5, 1), (0x12, 3, 2, 2), (0x15, 3, 5, 1), (0x1c, 3, 5, 1)]; // sbF4: the Korean League Cup group matchdays

/// The Korean League Cup knockout: the semi-finals drawn on 29 April and played on 5 May (4 clubs, 2 single-match
/// ties, tie rules 0x283), the final drawn on 6 May and played over two legs 4 days apart from 9 May (two-leg tie
/// rules 0x83); category 6, no prize per round (−1).
pub(crate) const KOREAN_LEAGUE_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 2] = [
    cql10r((0x1d, 3, 0, 6), (5, 4, 0, 5, 1, 0), 0x82, (0, 0x283, 0), 6, (4, 2, 4, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF4: the Korean League Cup semi-finals
    cql10r((6, 4, 0, 6), (9, 4, 0, 2, 2, 0), 0x96, (0, 0, 0x83), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
];

/// Rule: the Korean League Cup's two groups of five (the cup's own group A and stage 0, group B) play ten matchdays
/// from 25 March to 28 April, each group a league stage (is_league = 1, label 0x420 + stage); the knockout (stage 1):
/// single-match semi-finals on 5 May and a two-legged final from 9 May.
///
/// `create_schedule` with the knockout at stage 1, the label 0x420, `KOREAN_LEAGUE_CUP_GROUP_ROUNDS`,
/// `KOREAN_LEAGUE_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xbf and 0xfa of kor_league_cup.cpp (checked by
/// emulation of the original for every stage −128..127 in an odd and an even year: the same calls and the same 16
/// fields per round as ItaCup::create_schedule). Above stage 1: 0.
/// Original: `KorLeagueCup::create_schedule` (0x66c240, kor_league_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 1, 0x420, &KOREAN_LEAGUE_CUP_GROUP_ROUNDS, &KOREAN_LEAGUE_CUP_KNOCKOUT_ROUNDS, KOR_LEAGUE_CUP_FILE, 0xbf, 0xfa,
        "sbF_4 korean league cup groups", "sbF_4 korean league cup knockout rounds")
}

/// Comparer for `kor_league_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    ita_cup_compare_schedule("kor_league_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| kor_league_cup_create_schedule(this, stage, r, e, l), 1)
}

/// The staff records of a cup played as a league with groups and a knockout (`update_staff_records_for_status(club,
/// outcome, stage, _, fixture, final)`, the virtual +0x44); returns 0. The body of the Italian, Greek, Dutch and Korean
/// cups (`italian_cup_staff_records` with the round count of the weight as a parameter; the Korean one checked
/// instruction by instruction against GreCup's: only the knockout stage, the group qualification round and the round
/// count change).
///
/// Stage (signed byte) below `limit`, the groups: outcome 6, the qualification (`european_cup_add_qualified_record`:
/// stage 0, round `group_round`, weight 0x28); 0xfe, the failure (`european_cup_add_failed_to_qualify_record`: 0x41c,
/// 0x28). Stage `limit`, the knockout, weight = `scale_by_round`(0x32, 0x64, the fixture's `round_number()`,
/// `rounds`): 0xff, the elimination (`european_cup_add_knocked_out_record`: the fixture's `round_prefix()` and
/// `round_name()`); 0, the qualification with `round_prefix()` and the code of the next round in the knockout stage's
/// round table ([`cup::STAGES`][`limit`]; the high 16 bits the original pushes are the stage pointer's); 1, the
/// winner (Staff_Records 0x86a800); 2, the runner-up (0x86b0b0). Anything else: nothing.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn group_cup_staff_records(this: usize, club: usize, outcome: u32, stage: u32, game: usize, final_: u32, limit: i8, group_round: u32, rounds: u32, label: &'static str) -> u32 {
    let game = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < limit {
        if d == 6 {
            covered!(label);
            european_cup_add_qualified_record(club, comp, 0, group_round, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == limit {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), rounds);
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), rounds);
                let knockout = read_usize(read_usize(this + cup::STAGES) + limit as usize * 4);
                let r = game.round_number() as isize;
                let name = read_u16(read_usize(knockout + cup::ROUND_TABLE).wrapping_add((r * cup_round::SIZE as isize) as usize) + cup_round::ROUND_CODE + cup_round::SIZE) as u32;
                european_cup_add_qualified_record(club, comp, game.round_prefix() as u16 as u32, (knockout as u32 & 0xffff_0000) | name, weight);
            }
            1 => cup_record_final_result(0x86a800, club, game, final_),
            2 => cup_record_final_result(0x86b0b0, club, game, final_),
            _ => {}
        }
    }
    0
}

/// Rule: in the Korean League Cup a manager whose club goes through its group gets a qualification record (round 0x82,
/// the semi-finals) and one whose club goes out a failure record; in the knockout (stage 1), an elimination or a
/// qualification weighted by the round (of two), the cup win or the runner-up place.
///
/// `group_cup_staff_records` with the knockout at stage 1, the group qualification round 0x82 and two rounds.
/// Original: `KorLeagueCup::update_staff_records_for_status` (0x66d090, kor_league_cup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn kor_league_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    group_cup_staff_records(this, club, outcome, stage, game, final_, 1, 0x82, 2, "sbF_4 korean league cup group qualified")
}

/// Comparer for `kor_league_cup_update_staff_records_for_status`: only the original (it writes the staff histories);
/// returns 0.
pub(crate) unsafe extern "thiscall" fn compare_kor_league_cup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == 0, || format!("kor_league_cup_update_staff_records_for_status({:#x}) returned {}", this, r));
    r
}
