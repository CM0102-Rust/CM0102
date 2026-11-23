use super::*;

/// The Scottish Challenge Cup at career creation: the clubs of the First, Second and Third Divisions (30), 5 rounds from
/// July to October, a prize fund of 200,000, 5 substitutes named and 3 allowed, the discipline rule of the Scottish cups
/// (0x19); then the fixture tree, the statistics and the predicted positions (`construct_british_cup`).
/// Original: `ScoChalCup::ScoChalCup` (0x7ec980, sco_chal_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_chal_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &SCOTTISH_CHALLENGE_CUP, "mwb_1 new challenge cup")
}

/// Comparer for `construct_sco_chal_cup` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_chal_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_CHAL_CUP);
    compare_british_cup_ctor("construct_sco_chal_cup", Cup(this), year, record, &SCOTTISH_CHALLENGE_CUP, || o(this, year, record))
}

/// rule: the Scottish Challenge Cup has 5 single-match rounds from July to October: 28 clubs in the 1st round (drawn
/// 28/6, played 10/7), 16 in the 2nd (12/7 and 17/7: the 14 winners and 2 more), quarter-finals 19/7 and 24/7,
/// semi-finals 26/7 and 31/7, final drawn 2/9 and played on 14/10 at a neutral ground (venue 0x28); the dates are moved
/// to the weekday of each round; tie-break code 3; category 4 then 6; no prize per round. Outside stage −1 it writes
/// nothing and returns 0; out of memory, the box of line 0x97 and 0 (`eng_cc_cup_create_single_stage_round_table`).
/// Original: `ScoChalCup::create_round_table` (0x7ecc40, sco_chal_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_chal_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &CHALLENGE_CUP_ROUNDS, 0xa7ad70, 0x97, "mwb_1 challenge cup rounds")
}

/// Comparer for `sco_chal_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_chal_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_CHAL_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("sco_chal_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| sco_chal_cup_create_round_table(this, stage, r, e, s))
}

/// rule: the Scottish Challenge Cup is for the 30 clubs of the First, Second and Third Divisions (the Premier Division
/// does not enter), in database order; it returns 1 with the list full, 0 otherwise and −1 out of memory
/// Original: `ScoChalCup::collect_entrants` (0x7ed010, sco_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_chal_cup_collect_entrants(this: usize) -> i32 {
    challenge_cup_entrants(Cup(this)).0
}

/// Comparer for `sco_chal_cup_collect_entrants`: side by side, the return, the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_sco_chal_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SCO_CHAL_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("sco_chal_cup_collect_entrants", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || challenge_cup_entrants(this),
        |_, n| n.clamp(0, 0x1e) as usize * 6)
}

/// The Challenge Cup turnover: frees last season's teams, rounds and ties, archives the statistics, moves to the next
/// year (+1) and rebuilds the entrants (`ScoChalCup::collect_entrants`), the rounds (virtual +0x8c) and the fixture tree
/// (virtual +0x94), each with its error box (lines 0x180, 0x187, 0x18e of sco_chal_cup.cpp) when it returns negative;
/// then the predicted positions (virtual +0x5c), whose eax it returns (`start_new_cup_season`: the same template as the
/// Brazilian Cup's, checked instruction by instruction).
/// Original: `ScoChalCup::start_new_season` (0x7ed130, sco_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_chal_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7ed010, 0xa7ad70, [0x180, 0x187, 0x18e], 1, "mwb_4 challenge cup turnover") // mwb4: the next Challenge Cup
}

/// Comparer for `sco_chal_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_chal_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_CHAL_CUP_START_NEW_SEASON);
    compare_cup_new_season("sco_chal_cup_start_new_season", Cup(this), || o(this))
}

/// The Challenge Cup entrants: `cup::TEAM_COUNT` = 30, the table (`exe::crt_malloc(0xb4)`; out of memory, the box of
/// line 0x114 and −1); then the clubs whose division is the Scottish Third, Second or First, in database order (the
/// whole club table is walked; the list only takes them while it has room). Returns (1 with the list full, else 0; the
/// count).
pub(crate) unsafe fn challenge_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x1e);
    let l = exe::crt_malloc(0xb4);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7ad70, 0x114);
        return (-1, 0);
    }
    let mut k = 0i32;
    let n = read_usize(G_CLUB_COUNT) as i32;
    let order: Vec<i32> = (0..n).collect(); // mwb1: the clubs in database order
    for i in order {
        let c = eng_cc_cup_club_by_index(i as i16);
        let d = club_division_id(c);
        let lower = d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION)) || d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION))
            || d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION));
        if lower && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
        }
    }
    covered!("mwb_1 challenge cup entrants");
    ((k == this.team_count() as i32) as i32, k)
}

/// The Scottish Challenge Cup: 5 named substitutes and 3 substitutions, discipline rule 0x19, a prize fund of 200,000.
pub(crate) const SCOTTISH_CHALLENGE_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x96f374,
    bench: Some((5, 3)), // mwb1: the Challenge Cup bench
    discipline: 0x19,
    prize_fund: 0x30d40,
    entrants: 0x7ed010,
    file: 0xa7ad70,
    rows: [0x3e, 0x4c, 0x53],
};

/// The five rounds of the Scottish Challenge Cup (all single matches): the first drawn on 28 June and played on 10 July
/// (28 teams, 14 ties, 28 entering), the second on 12/7 and 17/7 (16 teams: the 14 winners and 2 entering from the 29th
/// place), the quarter-finals on 19/7 and 24/7, the semi-finals on 26/7 and 31/7 and the final drawn on 2 September and
/// played on 14 October at a neutral ground (venue 0x28); category 4 in the first two rounds, 6 after; no prize per
/// round (−1).
pub(crate) const CHALLENGE_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x1c, 5, 0, 6), (0xa, 7, 0, 1, 2, 0), 0xa, (0, 3, 0), 4, (0x1c, 0xe, 0x1c, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xc, 7, 0, 3), (0x11, 7, 0, 1, 2, 0), 0x14, (0, 3, 0), 4, (0x10, 8, 2, 0x1c), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 7, 0, 3), (0x18, 7, 0, 1, 1, 0), 0x78, (0, 3, 0), 6, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0x1a, 7, 0, 3), (0x1f, 7, 0, 1, 2, 0), 0x82, (0, 3, 0), 6, (4, 2, 0, 0), (0, 1, 4), (0, 0xffff_ffff, 0)),
    cql10r((2, 8, 0, 3), (0xe, 9, 0, 6, 1, 0x28), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mwb1: the Challenge Cup final
];
