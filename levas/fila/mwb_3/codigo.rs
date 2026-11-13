// ---------- mwb_3: the Welsh Cup (wel_cup.cpp): constructor, round table and entrants ----------

/// The Welsh Cup: the Cup bench defaults, discipline rule 0x28, a prize fund of 50,000.
pub(crate) const WELSH_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x970ee0,
    bench: None,
    discipline: 0x28, // mwb3: the Welsh Cup discipline rule
    prize_fund: 0xc350,
    entrants: 0x924ab0,
    file: 0xad5684,
    rows: [0x46, 0x54, 0x5b],
};

/// The Welsh Cup at career creation: 32 entrants (up to 14 Welsh lower-division clubs and the League of Wales), 5
/// rounds from October to May, a prize fund of 50,000, the Cup's own bench and discipline rule 0x28; then the fixture
/// tree, the statistics and the predicted positions (`construct_british_cup`, the mould of mwb_1; unlike the Scottish
/// cups it does not write the bench).
/// Original: `WelCup::WelCup` (0x924420, wel_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &WELSH_CUP, "mwb_3 new welsh cup")
}

/// Comparer for `construct_wel_cup` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_british_cup_ctor("construct_wel_cup", Cup(this), year, record, &WELSH_CUP, || o(this, year, record))
}

/// The five rounds of the Welsh Cup: the first drawn on 19 October and played on 18 November (32 teams, 16 ties, all
/// entering; a replay 3 days later), the second on 19/11 and 10/2 (a replay 8 days later), the quarter-finals on 11/2
/// and 10/3, the semi-finals on 11/3 and 7/4 (tie-break code 3 on the day in these two) and the final drawn on 8/4 and
/// played on 6 May (category 6, a replay 7 days later); single matches; no prize per round (−1).
pub(crate) const WELSH_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x13, 9, 0, 6), (0x12, 0xa, 0, 5, 1, 0), 0x1e, (0, 3, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 3), (0, 0xffff_ffff, 0)),
    cql10r((0x13, 0xa, 0, 6), (0xa, 1, 1, 5, 1, 0), 0x28, (0, 3, 0), 4, (0x10, 8, 0, 0), (0, 1, 8), (0, 0xffff_ffff, 0)),
    cql10r((0xb, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (0, 3, 3), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mwb3: the Welsh Cup quarter-finals
    cql10r((0xb, 2, 1, 6), (7, 3, 1, 5, 1, 0), 0x82, (0, 3, 3), 4, (4, 2, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((8, 3, 1, 6), (6, 4, 1, 6, 1, 0), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 7), (0, 0xffff_ffff, 0)),
];

/// rule: the Welsh Cup has 5 rounds from October to May: the 32 clubs in the 1st round (drawn 19/10, played 18/11), 2nd
/// 19/11 and 10/2, quarter-finals 11/2 and 10/3, semi-finals 11/3 and 7/4, final drawn 8/4 and played on 6/5; category 4
/// then 6 in the final; no prize per round. Outside stage −1 it writes nothing and returns 0; out of memory, the box of
/// line 0x9f and 0 (`eng_cc_cup_create_single_stage_round_table`).
/// Original: `WelCup::create_round_table` (0x9246e0, wel_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &WELSH_CUP_ROUNDS, 0xad5684, 0x9f, "mwb_3 welsh cup rounds")
}

/// Comparer for `wel_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_wel_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("wel_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| wel_cup_create_round_table(this, stage, r, e, s))
}

/// The Welsh Cup entrants: `cup::TEAM_COUNT` = 32 and the table (`exe::crt_malloc(0xc0)`; out of memory, the box of
/// line 0x11b and −1).
///
/// First the Welsh lower-division clubs (division `KEY_CLUB_COMP_WELSH_LOWER_DIVISION` and nation Wales, both records
/// present), in database order, while fewer than 14 are in (this pass does not look at `cup::TEAM_COUNT`); then the
/// League of Wales clubs (`KEY_CLUB_COMP_WELSH_PREMIER_DIVISION`) while the table has room (the whole club table is
/// walked). Class and status 0. Returns (1 with the list full, else 0; the count).
pub(crate) unsafe fn welsh_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x20);
    let l = exe::crt_malloc(0xc0);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xad5684, 0x11b);
        return (-1, 0);
    }
    let n = read_usize(G_CLUB_COUNT) as i32;
    let mut k = 0i32;
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = eng_cc_cup_club_by_index(i);
        let nation = read_usize(c + club::NATION);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_WELSH_LOWER_DIVISION)) && nation != 0 && read_usize(nation) == read_usize(KEY_NATION_WALES)
            && k < 0xe {
            put_cup_entrant(this, k, c);
            k += 1;
        }
        i = i.wrapping_add(1);
    }
    let order: Vec<i32> = (0..read_usize(G_CLUB_COUNT) as i32).collect(); // mwb3: the League of Wales in database order
    for i in order {
        let c = eng_cc_cup_club_by_index(i as i16);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_WELSH_PREMIER_DIVISION)) && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
        }
    }
    covered!("mwb_3 welsh cup entrants");
    ((k == this.team_count() as i32) as i32, k)
}

/// rule: the Welsh Cup has 32 entrants: up to 14 Welsh lower-division clubs, then the League of Wales clubs to fill the
/// list, in database order; returns 1 with the list full, 0 when clubs are missing and −1 out of memory
/// Original: `WelCup::collect_entrants` (0x924ab0, wel_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_cup_collect_entrants(this: usize) -> i32 {
    welsh_cup_entrants(Cup(this)).0
}

/// Comparer for `wel_cup_collect_entrants`: side by side, the return, the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_wel_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    eng_cc_cup_compare_entrant_list("wel_cup_collect_entrants", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || welsh_cup_entrants(this),
        |_, n| n.clamp(0, 0x20) as usize * 6)
}
