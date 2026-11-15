// ---------- mwb_1: the Scottish Challenge Cup (sco_chal_cup.cpp): constructor, round table and entrants; the constructor shared by the Scottish and Welsh cups ----------

/// Describes one of the Scottish and Welsh knockout cups built on Cup: the vtable, the bench (substitutes named and
/// substitutions allowed; `None` when the constructor leaves the Cup defaults), the discipline rule, the prize fund, the
/// entrants routine (thiscall, ret 0), the source file and the error-box lines (entrants, fixture tree, statistics).
pub(crate) struct BritishCupSpec {
    pub(crate) vtable: usize,
    pub(crate) bench: Option<(u8, u8)>,
    pub(crate) discipline: u8,
    pub(crate) prize_fund: u32,
    pub(crate) entrants: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 3],
}

/// The constructor shared by the Scottish Challenge Cup, the Scottish Cup and the Welsh Cup (the same template,
/// checked instruction by instruction: only the vtable, the bench, the discipline rule, the prize fund, the entrants
/// routine, the source file and the lines change).
///
/// Cup::Cup (`exe::new_cup`, mov ecx -> thiscall, ret 0); the record (+4), the vtable, the year (`cup::YEAR`, word) =
/// `year`; `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = 0xff, `cup::CURRENT_STAGE` = −1, `cup::STAGE_COUNT` = 0,
/// `cup::CLUB_SIDES` = 1, the bench (`cup::SUBSTITUTES_NAMED`, `cup::SUBSTITUTIONS_ALLOWED`; the Welsh Cup leaves them),
/// the discipline rule (`cup::DISCIPLINE_INDEX`) and `cup::RUN_BY_STAGES` = 0. If the saved state loads
/// (Cup::load_from_disk 0x51fc00, push 1; mov ecx -> thiscall, ret 4; nonzero), returns this.
///
/// Otherwise (a new career): the entrants (0 = the error box of the first line and this; the original tests `test eax,
/// eax`, so the −1 of an allocation failure goes on); the prize fund (`cup::PRIZE_FUND`); the round table of stage −1
/// (virtual +0x3c: push 0; push &STAGE_LABEL; push &ROUND_COUNT; push −1; thiscall, ret 0x10) into `cup::ROUND_TABLE`;
/// the fixture tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; 0 = the box of the second line); the statistics
/// (operator new `exe::crt_operator_new(0x5ce)` and Comp_Stats::Comp_Stats `exe::new_comp_stats(new, this)`, under the
/// original's SEH frame) into +8 (null: the box of the third line); finally the predicted positions (Cup::rank_teams
/// 0x5223a0, thiscall, ret 0). Returns this.
pub(crate) unsafe fn construct_british_cup(this: Cup, year: u32, record: usize, e: &BritishCupSpec, ramo: &'static str) -> usize {
    let t = this.addr();
    exe::new_cup(this);
    write_usize(t + league::COMPETITION, record);
    write_usize(t, e.vtable);
    write_u16(t + cup::YEAR, year as u16);
    write_u8(t + cup::CURRENT_ROUND, 0);
    write_u8(t + cup::STAGE_INDEX, 0xff);
    write_u32(t + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(t + cup::STAGE_COUNT, 0);
    write_u8(t + cup::CLUB_SIDES, 1);
    if let Some((named, allowed)) = e.bench {
        write_u8(t + cup::SUBSTITUTES_NAMED, named);
        write_u8(t + cup::SUBSTITUTIONS_ALLOWED, allowed);
    }
    write_u8(t + cup::DISCIPLINE_INDEX, e.discipline);
    write_u8(t + cup::RUN_BY_STAGES, 0);
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load_from_disk(t, 1) != 0 {
        return t;
    }
    covered!(ramo);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(e.entrants);
    if entrants(t) == 0 {
        show_error_at(e.file, e.rows[0]);
        return t;
    }
    write_u32(t + cup::PRIZE_FUND, e.prize_fund);
    let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(t) + comp_vtable::CREATE_SCHEDULE));
    let table = rounds(t, 0xffff_ffff, t + cup::ROUND_COUNT, t + cup::STAGE_LABEL, 0);
    write_usize(t + cup::ROUND_TABLE, table);
    let tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    if tree(t) == 0 {
        show_error_at(e.file, e.rows[1]);
        return t;
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(t + league::COMP_STATS, s);
    if s == 0 {
        show_error_at(e.file, e.rows[2]);
        return t;
    }
    let rank: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5223a0usize);
    rank(t);
    t
}

/// Comparer for the cup constructors: only the original runs (it allocates the entrants, the rounds and the
/// statistics); then the plan: the returned this, the vtable, the year, the record and the discipline rule and, in a
/// new career with statistics, the prize fund, the bench and the round table.
pub(crate) unsafe fn compare_british_cup_ctor(name: &str, this: Cup, year: u32, record: usize, e: &BritishCupSpec, original: impl FnOnce() -> usize) -> usize {
    let t = this.addr();
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == t && read_usize(t) == e.vtable && read_u16(t + cup::YEAR) == year as u16 && read_usize(t + league::COMPETITION) == record
        && read_u8(t + cup::DISCIPLINE_INDEX) == e.discipline;
    if !loaded && read_usize(t + league::COMP_STATS) != 0 {
        ok = ok && read_u32(t + cup::PRIZE_FUND) == e.prize_fund && read_usize(t + cup::ROUND_TABLE) != 0;
        if let Some((named, allowed)) = e.bench {
            ok = ok && read_u8(t + cup::SUBSTITUTES_NAMED) == named && read_u8(t + cup::SUBSTITUTIONS_ALLOWED) == allowed;
        }
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, year {}, record {:#x}, discipline {}, prize fund {}, bench {}/{}, rounds {:#x}",
        name, t, year as u16, record, loaded, r, read_usize(t), read_u16(t + cup::YEAR), read_usize(t + league::COMPETITION), read_u8(t + cup::DISCIPLINE_INDEX),
        read_u32(t + cup::PRIZE_FUND), read_u8(t + cup::SUBSTITUTES_NAMED), read_u8(t + cup::SUBSTITUTIONS_ALLOWED), read_usize(t + cup::ROUND_TABLE)));
    r
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

/// The id of the club's division (`club::DIVISION` → its competition record → the id); `None` without one.
pub(crate) unsafe fn club_division_id(c: usize) -> Option<usize> {
    let d = read_usize(c + club::DIVISION);
    if d == 0 { None } else { Some(read_usize(d)) }
}

/// Writes club `c` with class and status 0 into entry `k` of the cup's team table (6-byte entries).
pub(crate) unsafe fn put_cup_entrant(this: Cup, k: i32, c: usize) {
    let e = this.team_table() + k as usize * 6;
    write_usize(e, c);
    write_u8(e + 4, 0);
    write_u8(e + 5, 0);
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

// ---------- mwb_2: the Scottish Cup (sco_fa_cup.cpp): constructor, round table and entrants ----------

/// The Scottish Cup: 5 named substitutes and 3 substitutions, discipline rule 0x19, a prize fund of 1,000,000.
pub(crate) const SCOTTISH_CUP: BritishCupSpec = BritishCupSpec {
    vtable: 0x96f414,
    bench: Some((5, 3)),
    discipline: 0x19,
    prize_fund: 0xf4240,
    entrants: 0x7edaf0,
    file: 0xa7adac,
    rows: [0x4d, 0x5c, 0x63],
};

/// The Scottish Cup at career creation: 50 entrants (eight non-league clubs, the Second and Third Divisions, the First
/// and the Premier), 7 rounds from November to May, a prize fund of 1,000,000, 5 substitutes named and 3 allowed, the
/// discipline rule of the Scottish cups (0x19); then the fixture tree, the statistics and the predicted positions
/// (`construct_british_cup`, the mould of mwb_1).
/// Original: `ScoFaCup::ScoFaCup` (0x7ed310, sco_fa_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_fa_cup(this: usize, year: u32, record: usize) -> usize {
    construct_british_cup(Cup(this), year, record, &SCOTTISH_CUP, "mwb_2 new scottish cup")
}

/// Comparer for `construct_sco_fa_cup` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_fa_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_FA_CUP);
    compare_british_cup_ctor("construct_sco_fa_cup", Cup(this), year, record, &SCOTTISH_CUP, || o(this, year, record))
}

/// The seven rounds of the Scottish Cup: the first drawn on 8 November and played on 5 December (16 teams, 8 ties: the
/// 16 lowest entrants), the second on 6/12 and 2/1 (20 teams: the 8 winners and 12 entering from the 16th place), the
/// third on 3/1 and 29/1 (32 teams: 10 winners and 22 entering from the 28th: the Premier and the First), the fourth on
/// 30/1 and 19/2, the quarter-finals on 20/2 and 11/3, the semi-finals on 12/3 and 8/4 at a neutral ground (venue 0x28)
/// and the final drawn on 9/4 and played on 27 May at the national stadium (venue 4); replays allowed (+0x20 = 1) up to
/// the semi-finals, tie-break code 3 on a replay, and in the final code 3 on the day; category 4 up to the fourth round,
/// 6 after; the prize to the winner of each round: 3,000, none, 6,000, 10,000, 20,000, 70,000 and 150,000.
pub(crate) const SCOTTISH_CUP_ROUNDS: [EngCcCupCupRound; 7] = [
    cql10r((8, 0xa, 0, 6), (5, 0xb, 0, 5, 1, 0), 0xa, (0, 0, 3), 4, (0x10, 8, 0x10, 0), (1, 1, 0), (0, 0xbb8, 0)),
    cql10r((6, 0xb, 0, 6), (2, 0, 1, 5, 1, 0), 0x14, (0, 0, 3), 4, (0x14, 0xa, 0xc, 0x10), (1, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((3, 0, 1, 6), (0x1d, 0, 1, 5, 1, 0), 0x1e, (0, 0, 3), 4, (0x20, 0x10, 0x16, 0x1c), (1, 1, 0), (0, 0x1770, 0)),
    cql10r((0x1e, 0, 1, 6), (0x13, 1, 1, 5, 1, 0), 0x28, (0, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0), (0, 0x2710, 0)),
    cql10r((0x14, 1, 1, 6), (0xb, 2, 1, 5, 1, 0), 0x32, (0, 0, 3), 6, (8, 4, 0, 0), (1, 1, 0), (0, 0x4e20, 0)), // mwb2: the Scottish Cup quarter-finals
    cql10r((0xc, 2, 1, 6), (8, 3, 1, 5, 1, 0x28), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (1, 1, 0), (0, 0x11170, 0)),
    cql10r((9, 3, 1, 6), (0x1b, 4, 1, 5, 1, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0x249f0, 0)),
];

/// rule: the Scottish Cup has 7 rounds from November to May: 1st round drawn 8/11, played 5/12 (16 clubs), 2nd 6/12 and
/// 2/1 (20: 8 winners and 12 entering), 3rd 3/1 and 29/1 (32: 10 winners and the 22 of the Premier and the First), 4th
/// 30/1 and 19/2, quarter-finals 20/2 and 11/3, semi-finals 12/3 and 8/4 at a neutral ground, final drawn 9/4 and played
/// on 27/5 at the national stadium; replays up to the semi-finals; category 4 then 6; the round winners get 3,000, none,
/// 6,000, 10,000, 20,000, 70,000 and 150,000. Outside stage −1 it writes nothing and returns 0; out of memory, the box
/// of line 0xa7 and 0 (`eng_cc_cup_create_single_stage_round_table`).
/// Original: `ScoFaCup::create_round_table` (0x7ed5d0, sco_fa_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &SCOTTISH_CUP_ROUNDS, 0xa7adac, 0xa7, "mwb_2 scottish cup rounds")
}

/// Comparer for `sco_fa_cup_create_round_table` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_FA_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("sco_fa_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| sco_fa_cup_create_round_table(this, stage, r, e, s))
}

/// The Scottish Cup entrants, in the original routine's order.
///
/// `cup::TEAM_COUNT` = 50 and the table (`exe::crt_malloc(0x12c)`; out of memory, the box of line 0x153 and −1).
///
/// A local list of up to 30 clubs: first the Scottish non-league clubs (nation Scotland, not a national side
/// `exe::database_is_national_team`, division the Scottish lower league `KEY_CLUB_COMP_A_LOWER_DIVISION`), in database
/// order while the list has fewer than 30 (the test comes before each club); sorted by reputation with 8 picks
/// shuffled to the top (`exe::comp_util_sort_and_shuffle_top_clubs(list, count, 8)`, cdecl, the count as a word). The
/// list goes back to 8 and the clubs of the Third and Second Divisions are added after them (while fewer than 30 and
/// `cup::TEAM_COUNT` > 0); the whole list is shuffled (`exe::comp_util_shuffle_pointer_array(list, count)`, cdecl) and
/// copied into the table (class and status 0).
///
/// Then the First Division clubs, in database order, while the table has room (stops when full), and 20 swaps among the
/// last ten entries: each swaps the clubs of entries (k − 1 − random(10)) and (k − 1 − random(10)), k the count so far
/// (bytes, `exe::utils_random`, two draws per swap, the first for the first entry). Finally the Premier Division clubs
/// while there is room. Returns (1 with the list full, else 0; the count).
pub(crate) unsafe fn scottish_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x32);
    let l = exe::crt_malloc(0x12c);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0xa7adac, 0x153);
        return (-1, 0);
    }
    let count = || read_usize(G_CLUB_COUNT) as i32;
    let mut list = [0usize; 30];
    let mut m: i8 = 0;
    let mut i: i16 = 0;
    while (i as i32) < count() && m < 0x1e {
        let c = eng_cc_cup_club_by_index(i);
        let nation = read_usize(c + club::NATION);
        if nation != 0 && exe::database_is_national_team(c) == 0 && read_usize(nation) == read_usize(KEY_NATION_SCOTLAND)
            && club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_A_LOWER_DIVISION)) {
            list[m as usize] = c;
            m += 1;
        }
        i = i.wrapping_add(1);
    }
    exe::comp_util_sort_and_shuffle_top_clubs(list.as_mut_ptr() as usize, m as i16 as u16 as u32, 8); // mwb2: the eight non-league clubs
    let mut m: i8 = 8;
    let mut i: i16 = 0;
    while (i as i32) < count() && m < 0x1e {
        let c = eng_cc_cup_club_by_index(i);
        let d = club_division_id(c);
        if (d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION)) || d == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION))) && this.team_count() > 0 {
            list[m as usize] = c;
            m += 1;
        }
        i = i.wrapping_add(1);
    }
    let shuffled = m as i16;
    exe::comp_util_shuffle_pointer_array(list.as_mut_ptr() as usize, shuffled as u16 as u32);
    let mut k: i32 = 0;
    for (j, &c) in list.iter().enumerate().take(shuffled.max(0) as usize) {
        put_cup_entrant(this, j as i32, c);
        k = j as i32 + 1;
    }
    let mut i: i16 = 0;
    while (i as i32) < count() {
        let c = eng_cc_cup_club_by_index(i);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION)) && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    for _ in 0..0x14 { // mwb2: the swaps among the last ten
        let a = (k as u8).wrapping_sub(exe::utils_random(10) as u8).wrapping_sub(1) as i8 as i32;
        let b = (k as u8).wrapping_sub(exe::utils_random(10) as u8).wrapping_sub(1) as i8 as i32;
        let t = this.team_table();
        let x = read_usize(t + a as usize * 6);
        write_usize(t + a as usize * 6, read_usize(t + b as usize * 6));
        write_usize(t + b as usize * 6, x);
    }
    let mut i: i16 = 0;
    while (i as i32) < count() {
        let c = eng_cc_cup_club_by_index(i);
        if club_division_id(c) == Some(read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION)) && k < this.team_count() as i32 {
            put_cup_entrant(this, k, c);
            k += 1;
            if k == this.team_count() as i32 {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("mwb_2 scottish cup entrants");
    ((k == this.team_count() as i32) as i32, k)
}

/// rule: the Scottish Cup has 50 entrants: eight non-league Scottish clubs (sorted by reputation, the eight picks
/// shuffled), the Second and Third Divisions (all shuffled together with them: they enter in the first two rounds), the
/// First Division (shuffled among themselves: they enter in the second and third rounds) and the Premier Division (the
/// third round); returns 1 with the list full, 0 when clubs are missing and −1 out of memory
/// Original: `ScoFaCup::collect_entrants` (0x7edaf0, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_collect_entrants(this: usize) -> i32 {
    scottish_cup_entrants(Cup(this)).0
}

/// Comparer for `sco_fa_cup_collect_entrants`: side by side with the random generator's state (the copy's draws not
/// counted), the return, the count and the list.
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_SCO_FA_CUP_COLLECT_ENTRANTS);
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend_from_slice(&[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    eng_cc_cup_compare_entrant_list("sco_fa_cup_collect_entrants", this, &reg, || o(this.addr()), || crate::rng::without_counting(|| scottish_cup_entrants(this)),
        |_, n| n.clamp(0, 0x32) as usize * 6)
}

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_CUP);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WEL_CUP_CREATE_ROUND_TABLE);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_WEL_CUP_COLLECT_ENTRANTS);
    eng_cc_cup_compare_entrant_list("wel_cup_collect_entrants", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || welsh_cup_entrants(this),
        |_, n| n.clamp(0, 0x20) as usize * 6)
}

// ---------- mwb_4: the season turnover of the Scottish Challenge Cup, the Scottish Cup and the Welsh Cup (sco_chal_cup.cpp, sco_fa_cup.cpp, wel_cup.cpp) ----------

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

/// The Scottish Cup turnover: the same template, with the Scottish Cup entrants (`ScoFaCup::collect_entrants`) and the
/// error-box lines 0x20f, 0x216 and 0x21d of sco_fa_cup.cpp.
/// Original: `ScoFaCup::start_new_season` (0x7ede10, sco_fa_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_fa_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x7edaf0, 0xa7adac, [0x20f, 0x216, 0x21d], 1, "mwb_4 scottish cup turnover") // mwb4: the next Scottish Cup
}

/// Comparer for `sco_fa_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_fa_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FA_CUP_START_NEW_SEASON);
    compare_cup_new_season("sco_fa_cup_start_new_season", Cup(this), || o(this))
}

/// The Welsh Cup turnover: the same template, with the Welsh Cup entrants (`WelCup::collect_entrants`) and the
/// error-box lines 0x17b, 0x182 and 0x189 of wel_cup.cpp.
/// Original: `WelCup::start_new_season` (0x924c50, wel_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x924ab0, 0xad5684, [0x17b, 0x182, 0x189], 1, "mwb_4 welsh cup turnover") // mwb4: the next Welsh Cup
}

/// Comparer for `wel_cup_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_CUP_START_NEW_SEASON);
    compare_cup_new_season("wel_cup_start_new_season", Cup(this), || o(this))
}

// ---------- mwb_5: the Scottish First Division (sco_first.cpp): constructor, table and rules; the moulds of the Scottish and Welsh divisions ----------

/// Describes one of the Scottish and Welsh league divisions: the vtable, the stadium requirement (capacity and seats,
/// `None` when the constructor leaves them), the discipline rule, whether the 1999 season had one stage beyond the league
/// (the First Division's promotion play-off), the custom table compare the saved state gets back (the Premier Division's
/// split, `None` elsewhere), the rules and table routines (thiscall, ret 0), the source file and the error-box lines
/// (rules, table, fixtures, order, statistics).
pub(crate) struct BritishDivisionSpec {
    pub(crate) vtable: usize,
    pub(crate) stadium: Option<(u16, u16)>,
    pub(crate) discipline: u8,
    pub(crate) playoff_in_1999: bool,
    pub(crate) split_compare: Option<usize>,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The number of stages beyond the league: 1 in the 1999 season of a division with a play-off then, 0 otherwise.
pub(crate) unsafe fn british_division_stage_count(this: League, e: &BritishDivisionSpec) -> u32 {
    if e.playoff_in_1999 && this.year() == 0x7cf { 1 } else { 0 }
}

/// The constructor shared by the Scottish Premier, First, Second and Third Divisions and the League of Wales (the same
/// template, checked instruction by instruction: only the vtable, the stadium requirement, the 1999 play-off, the
/// Premier's split compare, the two routines, the source file and the lines change).
///
/// League::League (`exe::new_league`, mov ecx -> thiscall, ret 0); the record (`league::COMPETITION`), the vtable, the
/// year (word) = `year`, the stadium requirement (`league::REQUIRED_CAPACITY`, `league::REQUIRED_SEATS`) and the
/// discipline rule. If the saved state loads (`exe::league_load_from_temp_file(this, 1)`, nonzero), the Premier gets
/// its split compare back (`league::CUSTOM_COMPARE`, when the season is played in four rounds and the year is not 1999)
/// and it returns this.
///
/// Otherwise (a new career): `league::STAGE_INDEX` = 0xff, `league::CURRENT_STAGE` = −1, the stages beyond the league
/// (`british_division_stage_count`) and their vector (`exe::crt_malloc` of 4 per stage, unchecked; 0 without stages);
/// then, each with its own error box (and returning this) when it returns 0: the rules, the table, the fixtures
/// (`exe::league_create_fixtures`) and the order (`exe::league_sort_table(this, 0)`); the statistics (operator new
/// `exe::crt_operator_new(0x5ce)` and `exe::new_comp_stats(new, this)`, under the original's SEH frame) into
/// `league::COMP_STATS` (null: the box); finally the predicted positions (`exe::league_set_predicted_positions`).
pub(crate) unsafe fn construct_british_division(this: League, year: u32, record: usize, e: &BritishDivisionSpec, ramo: &'static str) -> usize {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.rules);
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), e.vtable);
    this.set_year(year as u16 as i16);
    if let Some((capacity, seats)) = e.stadium {
        this.set_required_capacity(capacity);
        this.set_required_seats(seats);
    }
    this.set_discipline_rule(e.discipline);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        if let Some(f) = e.split_compare {
            if this.legs() == 4 && this.year() != 0x7cf {
                this.set_custom_compare(f);
            }
        }
        return this.addr();
    }
    covered!(ramo);
    this.set_stage_index(0xff);
    this.set_current_stage(0xffff_ffff);
    let n = british_division_stage_count(this, e);
    this.set_stage_count(n as usize);
    if n as i32 > 0 {
        let v = exe::crt_malloc(n * 4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if rules(this.addr()) == 0 {
        show_error_at(e.file, e.rows[0]);
        return this.addr();
    }
    if table(this.addr()) == 0 {
        show_error_at(e.file, e.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, e.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(e.file, e.rows[3]);
        return this.addr();
    }
    let p = exe::crt_operator_new(0x5ce);
    let s = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(e.file, e.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// Comparer for the division constructors: only the original runs (it allocates the table, the fixtures and the
/// statistics); then the plan: the returned this, the vtable, the year, the record, the discipline rule, the stadium
/// requirement and, in a new career with statistics, the stages and the table.
pub(crate) unsafe fn compare_british_division_ctor(name: &str, this: League, year: u32, record: usize, e: &BritishDivisionSpec, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == e.vtable && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == e.discipline;
    if let Some((c, s)) = e.stadium {
        ok = ok && this.required_capacity() == c && read_u16(this.addr() + league::REQUIRED_SEATS) == s;
    }
    if !loaded && this.comp_stats() != 0 {
        let n = british_division_stage_count(this, e) as usize;
        ok = ok && this.stage_index() == 0xff && this.stage_count() == n && (this.stages() != 0) == (n != 0) && this.table() != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year arg {}, record arg {:#x}, loaded {}]: returned {:#x}, vtable {:#x}, year {}, record {:#x}, discipline {}, stages {} at {:#x}, table {:#x}, stats {:#x}",
        name, this.addr(), year as u16, record, loaded, r, read_usize(this.addr()), this.year(), this.competition().addr(), this.discipline_rule(), this.stage_count(), this.stages(), this.table(), this.comp_stats()));
    r
}

/// The table of a Scottish or Welsh division: `league::CLUB_COUNT` = `n`, the table (`exe::crt_malloc` of n × 0x3b;
/// out of memory, the box of `row` and 0), the clubs whose division (`club::DIVISION`) is the competition of global `g`,
/// in database order, each into the next row (`exe::league_init_table_row(this, row, club, 0, 0)`), until full
/// (`rotate` is 0 in the original: row k gets the k-th club; it exists only for the mutations).
///
/// Not full: 0. Full: with a prize fund, `league::PRIZE_FUND` = League::calc_prize_fund(`prize`) and `league::PRIZE_MODE`
/// = 1, and with TV money, League::pay_tv_rights(`tv`, 0) to every club (the Premier); returns 1 (the Second and Third
/// Divisions, the Welsh League and the Premier from 2000 on have no prize fund or TV money of their own here: only
/// those given).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe fn fill_british_division_table(this: League, n: i16, file: usize, row: u32, g: usize, prize: Option<i32>, tv: Option<i32>, rotate: u8, ramo: &'static str) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut c = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(g) {
            // simplification: the original pushes the row in eax with the high bits of the division id; 0x687430 only
            // reads the byte
            let l = ((k as u32 + rotate as u32) % n as u32) as u8;
            exe::league_init_table_row(this, l as u32, c, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        c = c.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    covered!(ramo);
    if let Some(q) = prize {
        let f = exe::league_calc_prize_fund(this, q);
        this.set_prize_fund(f);
        this.set_prize_mode(1);
    }
    if let Some(v) = tv {
        exe::league_pay_tv_rights(this, v, 0usize);
    }
    1
}

/// Comparer for the division tables: the division's clubs planned beforehand (database order, up to `n`); only the
/// original runs (it allocates and pays); then the count, the return (1 when the plan filled up), the rows and, when full
/// and with a prize fund, the fund and the mode.
pub(crate) unsafe fn compare_british_division_table(name: &str, this: League, n: i16, g: usize, prize: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let c = read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE;
        let d = read_usize(c + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(g) {
            plan.push(c);
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if let (1, Some(q)) = (r, prize) {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, q);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// The rules of a Scottish division (the Premier has its own): 4 rounds (`league::LEGS`), 3 points for a win and 1 for a
/// draw, category 2, club sides, tie-breaks 1, 2 and 3 (goal difference, goals scored, position [inference from the
/// English codes]), `promoted` promoted directly; in 1999 `playoff_1999` to the promotion play-off and nobody relegated,
/// from 2000 `relegated` relegated directly; the divisions above and below (`upper` and `lower`: the globals of the ids,
/// 0 = none, −1), 3 substitutions and 5 substitutes named.
pub(crate) unsafe fn scottish_division_rules(this: League, win: u8, promoted: u8, playoff_1999: u8, relegated: u8, upper: usize, lower: usize) -> Vec<(usize, u8, u32)> {
    let mut v = vec![(league::LEGS, 2, 4), (league::WIN_POINTS, 1, win as u32), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
                     (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3), (league::PROMOTED_COUNT, 1, promoted as u32)];
    if this.year() == 0x7cf {
        v.extend_from_slice(&[(league::PROMOTION_PLAYOFF_COUNT, 1, playoff_1999 as u32), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0)]);
    } else {
        v.extend_from_slice(&[(league::RELEGATED_COUNT, 1, relegated as u32), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0)]);
    }
    let id = |g: usize| if g == 0 { 0xffff_ffff } else { read_usize(g) as u32 };
    v.extend_from_slice(&[(league::UPPER_DIVISION_ID, 4, id(upper)), (league::LOWER_DIVISION_ID, 4, id(lower)), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// The Scottish First Division: discipline rule 0x18, no stadium requirement, the promotion play-off in 1999.
pub(crate) const SCOTTISH_FIRST_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f4b4,
    stadium: None,
    discipline: 0x18, // mwb5: the Scottish discipline rule
    playoff_in_1999: true,
    split_compare: None,
    rules: 0x7eee50,
    table: 0x7eed30,
    file: 0xa7ade4,
    rows: [0x4c, 0x53, 0x5a, 0x63, 0x6a],
};

/// The Scottish First Division at career creation: discipline rule 0x18, the 1999 promotion play-off, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`).
/// Original: `ScoFirst::ScoFirst` (0x7edfc0, sco_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_FIRST_DIVISION, "mwb_5 new scottish first")
}

/// Comparer for `construct_sco_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_FIRST);
    compare_british_division_ctor("construct_sco_first", League(this), year, record, &SCOTTISH_FIRST_DIVISION, || o(this, year, record))
}

/// The First Division table: 10 clubs, those whose division is the Scottish First, in database order; the prize fund of
/// 25,000 per place (League::calc_prize_fund) and no TV money; out of memory, the box of line 0x183
/// Original: `ScoFirst::create_table` (0x7eed30, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7ade4, 0x183, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), None, 0, "mwb_5 scottish first table") // mwb5: the First Division rows
}

/// Comparer for `sco_first_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_CREATE_TABLE);
    compare_british_division_table("sco_first_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, Some(0x61a8), || o(this))
}

/// The First Division rule writes (`scottish_division_rules`): the champion goes up to the Premier; in 1999 2 to the
/// promotion play-off and nobody down, from 2000 2 down to the Second.
pub(crate) unsafe fn scottish_first_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 1, 2, 2, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION) // mwb5: the First Division rules
}

/// rule: the Scottish First Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the champion goes
/// up to the Premier and the bottom 2 go down to the Second (in 1999: 2 to the promotion play-off and nobody down);
/// tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5 substitutes named and 3
/// allowed; builds the round schedule (virtual +0x3c into `league::ROUND_DATES`); returns 1
/// Original: `ScoFirst::set_up_rules` (0x7eee50, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_5 scottish first rules");
    eng_prm_apply_league_rules(League(this), &scottish_first_rules(League(this)))
}

/// Comparer for `sco_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_first_set_up_rules", League(this), scottish_first_rules(League(this)), || o(this), || sco_first_set_up_rules(this))
}

// ---------- mwb_6: the Scottish First Division (sco_first.cpp): the schedule, the turnover and the club progress; the moulds of the Scottish schedules and turnovers ----------

/// The main-stage schedule of a Scottish division: the rounds before the turn of the year, the reference round, the
/// eight cases of the year-end round chosen by the weekday (and the default one), the rounds after it, the source file
/// and the error-box lines (out of memory, the case out of range).
pub(crate) struct ScottishSchedule {
    pub(crate) before: &'static [EngFirstScheduleRound],
    pub(crate) reference: usize,
    pub(crate) cases: [&'static [EngFirstScheduleRound]; 8],
    pub(crate) default: &'static [EngFirstScheduleRound],
    pub(crate) after: &'static [EngFirstScheduleRound],
    pub(crate) file: usize,
    pub(crate) rows: (u32, u32),
}

/// Writes the main-stage rounds of a Scottish division into table `t`, in the original routine's order.
///
/// The rounds before (`eng_first_set_league_rounds`: League round date and its TV slots, cdecl); then the date of 1
/// December of the year (Date::Date `exe::date_date_549f70`) and k = (the day of the reference round, word at +0 of its
/// 0x41-byte entry, − the day of that date + 1, in 16 bits) − 15: from 0 to 7 the year-end round of case k; outside, the
/// error box of the second line and the default round (moved to Saturday instead of the weekday the cases leave); then
/// the rounds after.
pub(crate) unsafe fn set_scottish_league_rounds(t: usize, year: u32, s: &ScottishSchedule) {
    eng_first_set_league_rounds(t, year, 0, s.before);
    let mut d = [0u16; 4];
    exe::date_date_549f70(&mut d, 1, 0xb, year, 0xffff_ffff);
    let day = read_u16(t + s.reference * 0x41);
    let k = (day.wrapping_sub(d[0]).wrapping_add(1) as i16 as i32).wrapping_sub(0xf);
    let middle = if k as u32 > 7 {
        show_error_at(s.file, s.rows.1);
        s.default
    } else {
        s.cases[k as usize]
    };
    eng_first_set_league_rounds(t, year, s.before.len() as i32, middle);
    eng_first_set_league_rounds(t, year, (s.before.len() + middle.len()) as i32, s.after);
}

/// The main stage (stage −1) of a Scottish division's `create_schedule`: `league` (when given) = 1, `rounds` = the
/// number of rounds, `extra` = 0; the table (`exe::crt_malloc` of 0x41 per round; out of memory, the box of the first
/// line and 0) filled by `set_scottish_league_rounds`; returns the table.
pub(crate) unsafe fn scottish_league_round_table(this: League, rounds: *mut i16, extra: *mut i16, league: *mut i32, s: &ScottishSchedule, ramo: &'static str) -> usize {
    if !league.is_null() {
        *league = 1;
    }
    let n = s.before.len() + s.cases[0].len() + s.after.len();
    *rounds = n as i16;
    *extra = 0;
    let t = exe::crt_malloc(n as u32 * 0x41);
    if t == 0 {
        show_error_at(s.file, s.rows.0);
        return 0;
    }
    covered!(ramo);
    set_scottish_league_rounds(t, (this.year() as u16) as u32, s);
    t
}

/// The turnover of a Scottish division (First, Second, Third) or of the League of Wales (`welsh`), in the original
/// routine's order.
///
/// `league::SEASON_CLOSED` = 0; the League of Wales first records the final positions (virtual +0xa4); the table freed
/// (`exe::league_free_table(this, 0)`), the round dates freed and zeroed, the season archived in the statistics
/// (`exe::comp_stats_archive_season_and_reset(stats, 1)`); the Scottish divisions destroy the stages 0..=current
/// (virtual 0 with 1) and set the current stage to −1; the First also frees the stage vector (always, even null) and,
/// after the year, rebuilds the stage count and vector of `british_division_stage_count`. The year goes up by `step` (1).
///
/// Then, each with its error box (and 0) when it returns 0: the rules (virtual +0x8c), the table (`e.table`), the
/// fixtures and the order; finally the predicted positions (virtual +0x5c), the clubs' board expectations
/// (`exe::league_set_club_season_expectations`) and the renewal of the officials, whose eax it returns
/// (`set_expectations_and_renew_officials`).
pub(crate) unsafe fn british_division_new_season(this: League, e: &BritishDivisionSpec, rows: [u32; 4], welsh: bool, step: i16, ramo: &'static str) -> u32 {
    let call = |slot: usize| {
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + slot));
        f(this.addr())
    };
    write_u32(this.addr() + league::SEASON_CLOSED, 0);
    if welsh {
        call(league_vtable::RECORD_FINAL_POSITIONS);
    }
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates(0);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    if !welsh {
        if (this.current_stage() as i32) >= 0 {
            let mut i = 0i32;
            loop {
                let f = read_usize(this.stages() + 4 * i as usize);
                if f != 0 {
                    let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                    destroy(f, 1);
                }
                i += 1;
                if i > this.current_stage() as i32 {
                    break;
                }
            }
        }
        this.set_current_stage(0xffff_ffff);
    }
    if e.playoff_in_1999 {
        exe::crt_free(this.stages());
    }
    this.set_year(this.year().wrapping_add(step));
    if e.playoff_in_1999 {
        let n = british_division_stage_count(this, e);
        this.set_stage_count(n as usize);
        if n as i32 > 0 {
            let v = exe::crt_malloc(n * 4);
            this.set_stages(v);
        } else {
            this.set_stages(0);
        }
    }
    covered!(ramo);
    if call(comp_vtable::SET_UP_RULES) == 0 {
        show_error_at(e.file, rows[0]);
        return 0;
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(e.table);
    if table(this.addr()) == 0 {
        show_error_at(e.file, rows[1]);
        return 0;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(e.file, rows[2]);
        return 0;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(e.file, rows[3]);
        return 0;
    }
    call(comp_vtable::SET_PREDICTED_POSITIONS);
    set_expectations_and_renew_officials(this)
}

/// The Scottish First Division before the turn of the year: 19 Saturday rounds from 7 August to 18 December (none on 9
/// October).
pub(crate) const SCOTTISH_FIRST_BEFORE: [EngFirstScheduleRound; 19] = [
    (7, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 7/ago
    (14, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 14/ago
    (21, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 21/ago
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 28/ago
    (4, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 4/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 2/out, mwb6: a First Division round
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (13, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 13/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/dez
    (11, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 11/dez
    (18, 11, 0, 5, 1, &[(-1, -1, -1)]), // 18: 18/dez
];

/// The year-end round of the Scottish divisions by case (the day of the reference round minus 1 December, minus 15):
/// 26 December with code 2 in cases 0, 1, 2 and 7, with code 1 in 3 and 4, 27 December in 5, 28 December (code 2) in 6
/// (the weekday left as it is: −1); out of range, 26 December moved to Saturday.
pub(crate) const SCOTTISH_BOXING_DAY_A: [EngFirstScheduleRound; 1] = [(26, 11, 0, -1, 2, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_BOXING_DAY_B: [EngFirstScheduleRound; 1] = [(26, 11, 0, -1, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_DECEMBER_27: [EngFirstScheduleRound; 1] = [(27, 11, 0, -1, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_DECEMBER_28: [EngFirstScheduleRound; 1] = [(28, 11, 0, -1, 2, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_BOXING_DAY_SATURDAY: [EngFirstScheduleRound; 1] = [(26, 11, 0, 5, 1, &[(-1, -1, -1)])];
pub(crate) const SCOTTISH_YEAR_END_CASES: [&[EngFirstScheduleRound]; 8] = [&SCOTTISH_BOXING_DAY_A, &SCOTTISH_BOXING_DAY_A, &SCOTTISH_BOXING_DAY_A,
    &SCOTTISH_BOXING_DAY_B, &SCOTTISH_BOXING_DAY_B, &SCOTTISH_DECEMBER_27, &SCOTTISH_DECEMBER_28, &SCOTTISH_BOXING_DAY_A];

/// The Scottish First Division after the turn of the year: 3 January (on its own weekday, code 2), then Saturdays to 6
/// May of the next year.
pub(crate) const SCOTTISH_FIRST_AFTER: [EngFirstScheduleRound; 16] = [
    (3, 0, 1, 0, 2, &[(-1, -1, -1)]), // 20: 3/jan
    (8, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 8/jan
    (15, 0, 1, 5, 1, &[(-1, -1, -1)]), // 22: 15/jan
    (22, 0, 1, 5, 1, &[(-1, -1, -1)]), // 23: 22/jan
    (5, 1, 1, 5, 1, &[(-1, -1, -1)]), // 24: 5/fev
    (12, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 12/fev
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 26: 26/fev
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 4/mar
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 18/mar
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 25/mar
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 1/abr
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 31: 8/abr
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 15/abr
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 22/abr
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 29/abr
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 6/mai
];

pub(crate) const SCOTTISH_FIRST_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_FIRST_BEFORE,
    reference: 18,
    cases: SCOTTISH_YEAR_END_CASES,
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_FIRST_AFTER,
    file: 0xa7ade4,
    rows: (0xb1, 0x118),
};

/// The 1999 promotion play-off of the First Division (stage 0): 27 April, 31 May (code 2) and 4 June of the next year.
pub(crate) const SCOTTISH_FIRST_PLAYOFF: [(i32, i32, i32, i32, i32); 3] = [(27, 4, 1, 5, 1), (31, 4, 1, 2, 2), (4, 5, 1, 5, 1)];

/// rule: the Scottish First Division plays 36 rounds (10 clubs, 4 rounds), Saturdays from 7 August to 6 May of the next
/// year: 19 before the year end (none on 9 October), the year-end round on 26, 27 or 28 December by the weekday and 16
/// after (3 January on its own day); no TV slots. In 1999 the promotion play-off (stage 0): 3 rounds, 27 April, 31 May
/// and 4 June, with stage label 0xa0 and the date argument 0x28. Any other stage: nothing and 0
/// Original: `ScoFirst::create_schedule` (0x7ee2e0, sco_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => scottish_league_round_table(this, rounds, extra, league, &SCOTTISH_FIRST_SCHEDULE, "mwb_6 scottish first calendar"),
        0 => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 3;
            *extra = 0xa0;
            let t = exe::crt_malloc(0xc3);
            if t == 0 {
                show_error_at(0xa7ade4, 0x15f);
                return 0;
            }
            let year = (this.year() as u16) as u32;
            for (i, &(day, month, years, week, code)) in SCOTTISH_FIRST_PLAYOFF.iter().enumerate() {
                exe::league_round_set_date(t, i as i32, day, month, years, week, code, year, 0x28);
                exe::league_round_set_match_slot(t, i as i32, 0, -1, -1, -1, 0);
            }
            t
        }
        _ => 0,
    }
}

/// Comparer for `sco_first_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("sco_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_first_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish First Division is rebuilt: the season archived in the statistics, the old table,
/// rounds and stages freed, the next year (with the play-off stage only in 1999), the rules, the table, the fixtures and
/// the order; then the predicted positions, the board expectations and the referees renewed
/// (`british_division_new_season`; the error boxes of lines 0x1fe, 0x205, 0x20b and 0x212)
/// Original: `ScoFirst::start_new_season` (0x7eef00, sco_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_first_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_FIRST_DIVISION, [0x1fe, 0x205, 0x20b, 0x212], false, 1, "mwb_6 scottish first turnover") // mwb6: the next First Division season
}

/// Comparer for `sco_first_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("sco_first_start_new_season", League(this), || o(this))
}

/// Maps a progress code of the 1999 play-off: 1 → `first`, 2 and 3 → `rest`, the others as they are.
fn scottish_playoff_code(v: u8, first: u8, rest: u8) -> u8 {
    match v {
        1 => first,
        2 | 3 => rest,
        _ => v,
    }
}

/// rule: outside 1999 the club's progress codes go to the common routine (Comp::set_club_progress 0x48e380, the same
/// this and arguments: thiscall, ret 0x14). In 1999 they go into the club's Comp_Stats entry (current, best, worst):
/// in the league (stage 0xff) a best of 2 becomes 1; in the play-off (stage 0) the Premier club's codes go into the
/// Premier's statistics entry with 1 → 8 and 2, 3 → 10, and a First Division club's own with 1 → 1 and 2, 3 → 3; any
/// other stage as they came (the entry is not checked for null)
/// Original: `ScoFirst::set_club_progress` (0x7ef100, sco_first.cpp, thiscall, ret 0x14).
pub(crate) unsafe extern "thiscall" fn sco_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let league = League(this);
    if league.year() != 0x7cf {
        covered!("mwb_6 first division progress");
        let common: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x48e380usize);
        return common(this, club, stage, current, best, worst); // mwb6: the common progress codes
    }
    covered!("mwb_6 first division progress 1999");
    let mut e = exe::comp_stats_find_club_entry(league.comp_stats(), club);
    let (mut a, mut m, mut p) = (current as u8, best as u8, worst as u8);
    match stage as u8 {
        0xff => {
            if m == 2 {
                m = 1;
            }
        }
        0 => {
            let d = read_usize(club + club::DIVISION);
            if read_usize(d) == read_usize(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION) {
                let premier = comp_by_global(KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION);
                e = exe::comp_stats_find_club_entry(read_usize(premier + league::COMP_STATS), club);
                m = scottish_playoff_code(m, 8, 0xa);
                p = scottish_playoff_code(p, 8, 0xa);
                a = scottish_playoff_code(a, 8, 0xa);
            } else {
                m = scottish_playoff_code(m, 1, 3);
                p = scottish_playoff_code(p, 1, 3);
                a = scottish_playoff_code(a, 1, 3);
            }
        }
        _ => {}
    }
    let e = CompStatsClub(e);
    e.set_best_position(m);
    e.set_current_position(a);
    e.set_worst_position(p);
}

/// Comparer for `sco_first_set_club_progress`: side by side on the three codes of the club's statistics entry (only the
/// original without an entry, and in the 1999 play-off, where the Premier's entry is written).
pub(crate) unsafe extern "thiscall" fn compare_sco_first_set_club_progress(this: usize, club: usize, stage: u32, current: u32, best: u32, worst: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(IDX_SCO_FIRST_SET_CLUB_PROGRESS);
    if League(this).year() == 0x7cf && stage as u8 == 0 {
        return o(this, club, stage, current, best, worst);
    }
    compare_club_stage_status("sco_first_set_club_progress", this, club, || o(this, club, stage, current, best, worst), || sco_first_set_club_progress(this, club, stage, current, best, worst))
}

// ---------- mwb_7: the Scottish Second Division (sco_second.cpp): constructor, table and rules ----------

/// The Scottish Second Division: discipline rule 0x18, no stadium requirement, no 1999 play-off.
pub(crate) const SCOTTISH_SECOND_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f6bc,
    stadium: None,
    discipline: 0x18, // mwb7: the Second Division discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x7f3f80,
    table: 0x7f3e80,
    file: 0xa7afa8,
    rows: [0x42, 0x49, 0x50, 0x57, 0x5e],
};

/// The Scottish Second Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoSecond::ScoSecond` (0x7f3250, sco_second.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_SECOND_DIVISION, "mwb_7 new scottish second")
}

/// Comparer for `construct_sco_second` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_second(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_SECOND);
    compare_british_division_ctor("construct_sco_second", League(this), year, record, &SCOTTISH_SECOND_DIVISION, || o(this, year, record))
}

/// The Second Division table: 10 clubs, those whose division is the Scottish Second, in database order; no prize fund
/// and no TV money (returns 1 only when full); out of memory, the box of line 0x157
/// Original: `ScoSecond::create_table` (0x7f3e80, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7afa8, 0x157, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, None, None, 0, "mwb_7 scottish second table") // mwb7: the Second Division rows
}

/// Comparer for `sco_second_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_CREATE_TABLE);
    compare_british_division_table("sco_second_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, None, || o(this))
}

/// The Second Division rule writes (`scottish_division_rules`): the top 2 go up to the First, the bottom 2 down to the
/// Third (in 1999 nobody down and no play-off).
pub(crate) unsafe fn scottish_second_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 2, KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION) // mwb7: the Second Division rules
}

/// rule: the Scottish Second Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up
/// to the First and the bottom 2 go down to the Third (in 1999 nobody went down); tie-breaks goal difference, goals for
/// and position [inference from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `ScoSecond::set_up_rules` (0x7f3f80, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_set_up_rules(this: usize) -> u32 {
    covered!("mwb_7 scottish second rules");
    eng_prm_apply_league_rules(League(this), &scottish_second_rules(League(this)))
}

/// Comparer for `sco_second_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_second_set_up_rules", League(this), scottish_second_rules(League(this)), || o(this), || sco_second_set_up_rules(this))
}

// ---------- mwb_8: the Scottish Third Division (sco_third.cpp): constructor, table and rules ----------

/// The Scottish Third Division: discipline rule 0x18, no stadium requirement, no 1999 play-off.
pub(crate) const SCOTTISH_THIRD_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f770,
    stadium: None,
    discipline: 0x18, // mwb8: the Third Division discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x7f4f20,
    table: 0x7f4e20,
    file: 0xa7afe4,
    rows: [0x44, 0x4b, 0x52, 0x59, 0x60],
};

/// The Scottish Third Division at career creation: discipline rule 0x18, no stages beyond the league, the rules, the
/// table with its 10 clubs, the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5).
/// Original: `ScoThird::ScoThird` (0x7f41f0, sco_third.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_THIRD_DIVISION, "mwb_8 new scottish third")
}

/// Comparer for `construct_sco_third` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_third(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_THIRD);
    compare_british_division_ctor("construct_sco_third", League(this), year, record, &SCOTTISH_THIRD_DIVISION, || o(this, year, record))
}

/// The Third Division table: 10 clubs, those whose division is the Scottish Third, in database order; no prize fund
/// and no TV money (returns 1 only when full); out of memory, the box of line 0x159
/// Original: `ScoThird::create_table` (0x7f4e20, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_create_table(this: usize) -> u32 {
    fill_british_division_table(League(this), 0xa, 0xa7afe4, 0x159, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, None, None, 0, "mwb_8 scottish third table") // mwb8: the Third Division rows
}

/// Comparer for `sco_third_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_CREATE_TABLE);
    compare_british_division_table("sco_third_create_table", League(this), 0xa, KEY_CLUB_COMP_SCOTTISH_THIRD_DIVISION, None, || o(this))
}

/// The Third Division rule writes (`scottish_division_rules`): the top 2 go up to the Second, nobody goes down (no
/// division below: −1). The original compares the year with 1999 without using it: the bands are the same both ways.
pub(crate) unsafe fn scottish_third_rules(this: League) -> Vec<(usize, u8, u32)> {
    scottish_division_rules(this, 3, 2, 0, 0, KEY_CLUB_COMP_SCOTTISH_SECOND_DIVISION, 0) // mwb8: the Third Division rules
}

/// rule: the Scottish Third Division (10 clubs) plays 4 rounds, 3 points for a win and 1 for a draw; the top 2 go up to
/// the Second and nobody goes down (it is the last division); tie-breaks goal difference, goals for and position
/// [inference from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `ScoThird::set_up_rules` (0x7f4f20, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_set_up_rules(this: usize) -> u32 {
    covered!("mwb_8 scottish third rules");
    eng_prm_apply_league_rules(League(this), &scottish_third_rules(League(this)))
}

/// Comparer for `sco_third_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_SET_UP_RULES);
    eng_prm_compare_league_rules("sco_third_set_up_rules", League(this), scottish_third_rules(League(this)), || o(this), || sco_third_set_up_rules(this))
}

// ---------- mwb_9: the Scottish Second Division (sco_second.cpp): the schedule and the turnover; the schedule shared with the Third ----------

/// The Scottish Second and Third Divisions before the turn of the year: 18 Saturday rounds from 7 August to 18 December
/// (none on 9 October and 11 December).
pub(crate) const SCOTTISH_LOWER_BEFORE: [EngFirstScheduleRound; 18] = [
    (7, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 7/ago
    (14, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 14/ago
    (21, 7, 0, 5, 1, &[(-1, -1, -1)]), // 2: 21/ago
    (28, 7, 0, 5, 1, &[(-1, -1, -1)]), // 3: 28/ago
    (4, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 4/set
    (11, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 11/set
    (18, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 18/set
    (25, 8, 0, 5, 1, &[(-1, -1, -1)]), // 7: 25/set
    (2, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 2/out
    (16, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 16/out, mwb9: a Second and Third Division round
    (23, 9, 0, 5, 1, &[(-1, -1, -1)]), // 10: 23/out
    (30, 9, 0, 5, 1, &[(-1, -1, -1)]), // 11: 30/out
    (6, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 6/nov
    (13, 10, 0, 5, 1, &[(-1, -1, -1)]), // 13: 13/nov
    (20, 10, 0, 5, 1, &[(-1, -1, -1)]), // 14: 20/nov
    (27, 10, 0, 5, 1, &[(-1, -1, -1)]), // 15: 27/nov
    (4, 11, 0, 5, 1, &[(-1, -1, -1)]), // 16: 4/dez
    (18, 11, 0, 5, 1, &[(-1, -1, -1)]), // 17: 18/dez
];

/// The Scottish Second and Third Divisions after the turn of the year: 3 January (on its own weekday, code 2), then
/// Saturdays from 15 January to 6 May of the next year (none on 8 January).
pub(crate) const SCOTTISH_LOWER_AFTER: [EngFirstScheduleRound; 17] = [
    (3, 0, 1, 0, 2, &[(-1, -1, -1)]), // 19: 3/jan
    (15, 0, 1, 5, 1, &[(-1, -1, -1)]), // 20: 15/jan
    (22, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 22/jan
    (5, 1, 1, 5, 1, &[(-1, -1, -1)]), // 22: 5/fev
    (12, 1, 1, 5, 1, &[(-1, -1, -1)]), // 23: 12/fev
    (19, 1, 1, 5, 1, &[(-1, -1, -1)]), // 24: 19/fev
    (26, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 26/fev
    (4, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 4/mar
    (11, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 11/mar
    (18, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 18/mar
    (25, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 25/mar
    (1, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 1/abr
    (8, 3, 1, 5, 1, &[(-1, -1, -1)]), // 31: 8/abr
    (15, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 15/abr
    (22, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 22/abr
    (29, 3, 1, 5, 1, &[(-1, -1, -1)]), // 34: 29/abr
    (6, 4, 1, 5, 1, &[(-1, -1, -1)]), // 35: 6/mai
];

/// The main-stage schedule of the Scottish Second Division: the reference round is the 18th (index 17); the year-end
/// cases of the First Division (`SCOTTISH_YEAR_END_CASES`, mwb_6); out of memory, the box of line 0xa5; the case out of
/// range, 0x109.
pub(crate) const SCOTTISH_SECOND_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_LOWER_BEFORE,
    reference: 17,
    cases: SCOTTISH_YEAR_END_CASES,
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_LOWER_AFTER,
    file: 0xa7afa8,
    rows: (0xa5, 0x109),
};

/// rule: the Scottish Second Division plays 36 rounds (10 clubs, 4 rounds), Saturdays from 7 August to 6 May of the next
/// year: 18 before the year end (none on 9 October and 11 December), the year-end round on 26, 27 or 28 December by the
/// weekday and 17 after (3 January on its own day, none on 8 January); no TV slots; only the main stage (any other
/// stage: nothing and 0)
/// Original: `ScoSecond::create_schedule` (0x7f3540, sco_second.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    scottish_league_round_table(League(this), rounds, extra, league, &SCOTTISH_SECOND_SCHEDULE, "mwb_9 scottish second calendar")
}

/// Comparer for `sco_second_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_second_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_SECOND_CREATE_SCHEDULE);
    compare_league_round_table("sco_second_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_second_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish Second Division is rebuilt: the season archived in the statistics, the old table,
/// rounds and stages freed, the next year, the rules, the table, the fixtures and the order; then the predicted
/// positions, the board expectations and the referees renewed (`british_division_new_season`, mwb_6; the error boxes of
/// lines 0x1c3, 0x1ca, 0x1d0 and 0x1d7)
/// Original: `ScoSecond::start_new_season` (0x7f4030, sco_second.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_second_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_SECOND_DIVISION, [0x1c3, 0x1ca, 0x1d0, 0x1d7], false, 1, "mwb_9 scottish second turnover") // mwb9: the next Second Division season
}

/// Comparer for `sco_second_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_second_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_SECOND_START_NEW_SEASON);
    compare_state_league_new_season("sco_second_start_new_season", League(this), || o(this))
}

// ---------- mwb_10: the Scottish Third Division (sco_third.cpp): the schedule and the turnover ----------

/// The main-stage schedule of the Scottish Third Division: the rounds of the Second (`SCOTTISH_LOWER_BEFORE` and
/// `SCOTTISH_LOWER_AFTER`, mwb_9) and the same year-end cases; out of memory, the box of line 0xa7; the case out of
/// range, 0x10b.
pub(crate) const SCOTTISH_THIRD_SCHEDULE: ScottishSchedule = ScottishSchedule {
    before: &SCOTTISH_LOWER_BEFORE,
    reference: 17,
    cases: SCOTTISH_YEAR_END_CASES, // mwb10: the Third Division year-end cases
    default: &SCOTTISH_BOXING_DAY_SATURDAY,
    after: &SCOTTISH_LOWER_AFTER,
    file: 0xa7afe4,
    rows: (0xa7, 0x10b),
};

/// rule: the Scottish Third Division plays the calendar of the Second: 36 rounds, Saturdays from 7 August to 6 May of
/// the next year, the year-end round on 26, 27 or 28 December by the weekday; only the main stage (any other stage:
/// nothing and 0)
/// Original: `ScoThird::create_schedule` (0x7f44e0, sco_third.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn sco_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    if stage as u8 != 0xff {
        return 0;
    }
    scottish_league_round_table(League(this), rounds, extra, league, &SCOTTISH_THIRD_SCHEDULE, "mwb_10 scottish third calendar")
}

/// Comparer for `sco_third_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_sco_third_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_SCO_THIRD_CREATE_SCHEDULE);
    compare_league_round_table("sco_third_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| sco_third_create_schedule(this, stage, r, e, l))
}

/// rule: at the turnover the Scottish Third Division is rebuilt like the Second (`british_division_new_season`, mwb_6;
/// the error boxes of lines 0x1c5, 0x1cc, 0x1d2 and 0x1d9)
/// Original: `ScoThird::start_new_season` (0x7f4fb0, sco_third.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_third_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &SCOTTISH_THIRD_DIVISION, [0x1c5, 0x1cc, 0x1d2, 0x1d9], false, 1, "mwb_10 scottish third turnover") // mwb10: the next Third Division season
}

/// Comparer for `sco_third_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_sco_third_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_THIRD_START_NEW_SEASON);
    compare_state_league_new_season("sco_third_start_new_season", League(this), || o(this))
}

// ---------- mwb_11: the League of Wales (wel_first.cpp): constructor, rules and turnover ----------

/// The League of Wales: discipline rule 0x28, no stadium requirement, no stages beyond the league; the table is
/// WelFirst::create_table (0x925b30, still in the original).
pub(crate) const LEAGUE_OF_WALES: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x970f80,
    stadium: None,
    discipline: 0x28, // mwb11: the League of Wales discipline rule
    playoff_in_1999: false,
    split_compare: None,
    rules: 0x925c50,
    table: 0x925b30,
    file: 0xad56b8,
    rows: [0x47, 0x4e, 0x55, 0x5e, 0x65],
};

/// The League of Wales at career creation: discipline rule 0x28, no stages beyond the league, the rules, the table with
/// its 18 clubs, the fixtures, the order, the statistics and the predicted positions (`construct_british_division`, the
/// mould of mwb_5).
/// Original: `WelFirst::WelFirst` (0x924e30, wel_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &LEAGUE_OF_WALES, "mwb_11 new league of wales")
}

/// Comparer for `construct_wel_first` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_wel_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_WEL_FIRST);
    compare_british_division_ctor("construct_wel_first", League(this), year, record, &LEAGUE_OF_WALES, || o(this, year, record))
}

/// The League of Wales rule writes: match rules and round rules 0 (`league::MATCH_RULES`, `league::ROUND_DRAW_RULES`),
/// nobody promoted or relegated (no division above or below: −1), squad flags 0, 3 points for a win and 1 for a draw,
/// tie-breaks 1, 2 and 3, 2 rounds, category 2, club sides, 3 substitutions and 5 substitutes named.
pub(crate) fn league_of_wales_rules() -> Vec<(usize, u8, u32)> {
    vec![(league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0),
         (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), (league::SQUAD_FLAGS, 1, 0),
         (league::WIN_POINTS, 1, 3), // mwb11: the League of Wales points for a win
         (league::TIEBREAKS + 2, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEGS, 2, 2), (league::MATCH_CATEGORY, 1, 2), (league::TIEBREAKS + 1, 1, 2),
         (league::DRAW_POINTS, 1, 1), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::UPPER_DIVISION_ID, 4, 0xffff_ffff),
         (league::LOWER_DIVISION_ID, 4, 0xffff_ffff), (league::SUBSTITUTES_NAMED, 1, 5)]
}

/// rule: the League of Wales (18 clubs) plays 2 rounds, 3 points for a win and 1 for a draw, nobody goes up or down
/// (there is no division above or below in the game); tie-breaks goal difference, goals for and position [inference
/// from codes 1, 2, 3]; 5 substitutes named and 3 allowed; builds the round schedule; returns 1
/// Original: `WelFirst::set_up_rules` (0x925c50, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_set_up_rules(this: usize) -> u32 {
    covered!("mwb_11 league of wales rules");
    eng_prm_apply_league_rules(League(this), &league_of_wales_rules())
}

/// Comparer for `wel_first_set_up_rules` (side by side on the rule bytes, the copy without the round table).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_FIRST_SET_UP_RULES);
    eng_prm_compare_league_rules("wel_first_set_up_rules", League(this), league_of_wales_rules(), || o(this), || wel_first_set_up_rules(this))
}

/// rule: at the turnover the League of Wales records the final positions (virtual +0xa4), archives the season, frees the
/// table and the rounds, moves to the next year and rebuilds the rules, the table, the fixtures and the order; then the
/// predicted positions, the board expectations and the referees renewed (`british_division_new_season` with the Welsh
/// steps, mwb_6; the error boxes of lines 0x1f9, 0x200, 0x206 and 0x20d)
/// Original: `WelFirst::start_new_season` (0x925cf0, wel_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn wel_first_start_new_season(this: usize) -> u32 {
    british_division_new_season(League(this), &LEAGUE_OF_WALES, [0x1f9, 0x200, 0x206, 0x20d], true, 1, "mwb_11 league of wales turnover") // mwb11: the next League of Wales season
}

/// Comparer for `wel_first_start_new_season`: only the original runs; the year went up by one.
pub(crate) unsafe extern "thiscall" fn compare_wel_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_WEL_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("wel_first_start_new_season", League(this), || o(this))
}

// ---------- mwb_12: the League of Wales (wel_first.cpp): the schedule ----------

/// The League of Wales before the year end: 16 rounds from 19 August to 16 December (4 October on a Wednesday, code 2;
/// none on 7 and 14 October and 18 November).
pub(crate) const LEAGUE_OF_WALES_BEFORE: [EngFirstScheduleRound; 16] = [
    (19, 7, 0, 5, 1, &[(-1, -1, -1)]), // 0: 19/ago
    (26, 7, 0, 5, 1, &[(-1, -1, -1)]), // 1: 26/ago
    (2, 8, 0, 5, 1, &[(-1, -1, -1)]), // 2: 2/set
    (9, 8, 0, 5, 1, &[(-1, -1, -1)]), // 3: 9/set
    (16, 8, 0, 5, 1, &[(-1, -1, -1)]), // 4: 16/set
    (23, 8, 0, 5, 1, &[(-1, -1, -1)]), // 5: 23/set
    (30, 8, 0, 5, 1, &[(-1, -1, -1)]), // 6: 30/set
    (4, 9, 0, 2, 2, &[(-1, -1, -1)]), // 7: 4/out
    (21, 9, 0, 5, 1, &[(-1, -1, -1)]), // 8: 21/out
    (28, 9, 0, 5, 1, &[(-1, -1, -1)]), // 9: 28/out
    (4, 10, 0, 5, 1, &[(-1, -1, -1)]), // 10: 4/nov
    (11, 10, 0, 5, 1, &[(-1, -1, -1)]), // 11: 11/nov
    (25, 10, 0, 5, 1, &[(-1, -1, -1)]), // 12: 25/nov, mwb12: a League of Wales round
    (2, 11, 0, 5, 1, &[(-1, -1, -1)]), // 13: 2/dez
    (9, 11, 0, 5, 1, &[(-1, -1, -1)]), // 14: 9/dez
    (16, 11, 0, 5, 1, &[(-1, -1, -1)]), // 15: 16/dez
];

/// The League of Wales year-end rounds when the reference round falls on 13 December (case 0).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_0: [EngFirstScheduleRound; 2] = [
    (27, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 27/dez
    (30, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 14 December (case 1).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_1: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 15 December (case 2).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_2: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (29, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 29/dez
];

/// The League of Wales year-end rounds when the reference round falls on 16 December (case 3).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_3: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (30, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 30/dez
];

/// The League of Wales year-end rounds when the reference round falls on 17 December (case 4).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_4: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (31, 11, 0, -1, 1, &[(-1, -1, -1)]), // 17: 31/dez
];

/// The League of Wales year-end rounds when the reference round falls on 18 December (case 5).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_5: [EngFirstScheduleRound; 2] = [
    (26, 11, 0, -1, 1, &[(-1, -1, -1)]), // 16: 26/dez
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 17: 28/dez
];

/// The League of Wales year-end rounds when the reference round falls on 19 December (case 6).
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_6: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, -1, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales year-end rounds out of range (after the error box): 28 December on Sunday and 2 January.
pub(crate) const LEAGUE_OF_WALES_CHRISTMAS_DEFAULT: [EngFirstScheduleRound; 2] = [
    (28, 11, 0, 0, 2, &[(-1, -1, -1)]), // 16: 28/dez
    (2, 0, 1, -1, 1, &[(-1, -1, -1)]), // 17: 2/jan
];

/// The League of Wales after the year end: 16 rounds from 6 January to 28 April of the next year (21 February and 17
/// April midweek, code 2).
pub(crate) const LEAGUE_OF_WALES_AFTER: [EngFirstScheduleRound; 16] = [
    (6, 0, 1, 5, 1, &[(-1, -1, -1)]), // 18: 6/jan
    (13, 0, 1, 5, 1, &[(-1, -1, -1)]), // 19: 13/jan
    (20, 0, 1, 5, 1, &[(-1, -1, -1)]), // 20: 20/jan
    (27, 0, 1, 5, 1, &[(-1, -1, -1)]), // 21: 27/jan
    (3, 1, 1, 5, 1, &[(-1, -1, -1)]), // 22: 3/fev
    (17, 1, 1, 5, 1, &[(-1, -1, -1)]), // 23: 17/fev
    (21, 1, 1, 2, 2, &[(-1, -1, -1)]), // 24: 21/fev
    (24, 1, 1, 5, 1, &[(-1, -1, -1)]), // 25: 24/fev
    (3, 2, 1, 5, 1, &[(-1, -1, -1)]), // 26: 3/mar
    (17, 2, 1, 5, 1, &[(-1, -1, -1)]), // 27: 17/mar
    (24, 2, 1, 5, 1, &[(-1, -1, -1)]), // 28: 24/mar
    (31, 2, 1, 5, 1, &[(-1, -1, -1)]), // 29: 31/mar
    (14, 3, 1, 5, 1, &[(-1, -1, -1)]), // 30: 14/abr
    (17, 3, 1, 1, 2, &[(-1, -1, -1)]), // 31: 17/abr
    (21, 3, 1, 5, 1, &[(-1, -1, -1)]), // 32: 21/abr
    (28, 3, 1, 5, 1, &[(-1, -1, -1)]), // 33: 28/abr
];

/// The League of Wales calendar with its year-end block chosen by the day the 16th round (index 15) falls on (k = day
/// − 1 December + 1 − 13: 0 to 6; the default after the box of line 0x12e).
pub(crate) const LEAGUE_OF_WALES_SCHEDULE: EngFirstChristmasSchedule = EngFirstChristmasSchedule {
    comeco: &LEAGUE_OF_WALES_BEFORE,
    reference: 15,
    adjust: 0xd,
    cases: [&LEAGUE_OF_WALES_CHRISTMAS_0, &LEAGUE_OF_WALES_CHRISTMAS_1, &LEAGUE_OF_WALES_CHRISTMAS_2, &LEAGUE_OF_WALES_CHRISTMAS_3,
            &LEAGUE_OF_WALES_CHRISTMAS_4, &LEAGUE_OF_WALES_CHRISTMAS_5, &LEAGUE_OF_WALES_CHRISTMAS_6, &LEAGUE_OF_WALES_CHRISTMAS_DEFAULT],
    end: &LEAGUE_OF_WALES_AFTER,
    row: 0x12e,
};

/// rule: the League of Wales plays 34 rounds (18 clubs, 2 rounds) from 19 August to 28 April of the next year, mostly on
/// Saturdays; around the year end two rounds chosen by the day the 16th round falls on (26 to 31 December and 2
/// January); no TV slots; only the main stage (`league` = 1, `extra` = 0; out of memory, the box of line 0xa1 and 0;
/// any other stage: nothing and 0; `eng_first_set_christmas_rounds`)
/// Original: `WelFirst::create_schedule` (0x9250f0, wel_first.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    if stage as u8 != 0xff {
        return 0;
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 0x22;
    *extra = 0;
    let t = exe::crt_malloc(0x8a2);
    if t == 0 {
        show_error_at(0xad56b8, 0xa1);
        return 0;
    }
    covered!("mwb_12 league of wales calendar");
    eng_first_set_christmas_rounds(t, (this.year() as u16) as u32, &LEAGUE_OF_WALES_SCHEDULE, 0xad56b8);
    t
}

/// Comparer for `wel_first_create_schedule` (side by side, the written rounds).
pub(crate) unsafe extern "thiscall" fn compare_wel_first_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WEL_FIRST_CREATE_SCHEDULE);
    compare_league_round_table("wel_first_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league), |r, e, l| wel_first_create_schedule(this, stage, r, e, l))
}

// ---------- mwb_13: the Scottish Premier Division (sco_prm.cpp): constructor, table and rules ----------

/// The Scottish Premier Division: a stadium of 10,000 places, all seated, discipline rule 0x18, no stages beyond the
/// league, and the split's table compare (0x7f3220) given back to a saved season played in four rounds.
pub(crate) const SCOTTISH_PREMIER_DIVISION: BritishDivisionSpec = BritishDivisionSpec {
    vtable: 0x96f608,
    stadium: Some((0x2710, 0x2710)),
    discipline: 0x18, // mwb13: the Premier discipline rule
    playoff_in_1999: false,
    split_compare: Some(0x7f3220),
    rules: 0x7f2970,
    table: 0x7f2820,
    file: 0xa7af70,
    rows: [0x53, 0x5a, 0x61, 0x68, 0x6f],
};

/// The Scottish Premier Division at career creation: a stadium of 10,000 places (all seated), discipline rule 0x18, the
/// rules, the table with its 12 clubs (10 in 1999), the fixtures, the order, the statistics and the predicted positions
/// (`construct_british_division`, the mould of mwb_5); a saved season played in four rounds (the split) outside 1999
/// gets back the split's table compare (`league::CUSTOM_COMPARE` = 0x7f3220).
/// Original: `ScoPrm::ScoPrm` (0x7f0b80, sco_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    construct_british_division(League(this), year, record, &SCOTTISH_PREMIER_DIVISION, "mwb_13 new scottish premier")
}

/// Comparer for `construct_sco_prm` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_sco_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_SCO_PRM);
    compare_british_division_ctor("construct_sco_prm", League(this), year, record, &SCOTTISH_PREMIER_DIVISION, || o(this, year, record))
}

/// The number of Premier clubs: 10 in 1999, 12 from 2000.
pub(crate) unsafe fn scottish_premier_clubs(this: League) -> i16 {
    if this.year() == 0x7cf { 0xa } else { 0xc }
}

/// The Premier table: 12 clubs (10 in 1999), those whose division is the Scottish Premier, in database order; the prize
/// fund of 50,000 per place (League::calc_prize_fund) and 500,000 of TV money to every club (League::pay_tv_rights with
/// no division filter); out of memory, the box of line 0x2a7
/// Original: `ScoPrm::create_table` (0x7f2820, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_create_table(this: usize) -> u32 {
    let n = scottish_premier_clubs(League(this));
    fill_british_division_table(League(this), n, 0xa7af70, 0x2a7, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), Some(0x7a120), 0, "mwb_13 scottish premier table") // mwb13: the Premier rows
}

/// Comparer for `sco_prm_create_table` (the plan, original only).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_CREATE_TABLE);
    let n = scottish_premier_clubs(League(this));
    compare_british_division_table("sco_prm_create_table", League(this), n, KEY_CLUB_COMP_SCOTTISH_PREMIER_DIVISION, Some(0xc350), || o(this))
}

/// The Premier rule writes before the round table: in 1999 4 rounds, nobody promoted, 1 to the relegation play-off and
/// nobody relegated directly, the games per club not computed (−1); from 2000 3 rounds, the bottom club relegated and 38
/// games per club; then no custom table compare (`league::CUSTOM_COMPARE` = 0), 3 points for a win and 1 for a draw,
/// category 2, match and round rules 0, club sides, tie-breaks 1, 2 and 3, no division above (−1) and the First below,
/// 3 substitutions, squad flags 2 and 5 substitutes named.
pub(crate) unsafe fn scottish_premier_rules(this: League) -> Vec<(usize, u8, u32)> {
    let mut v = if this.year() == 0x7cf {
        vec![(league::LEGS, 2, 4), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1),
             (league::RELEGATED_COUNT, 1, 0), (league::GAMES_PER_CLUB, 2, 0xffff)]
    } else {
        vec![(league::LEGS, 2, 3), (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 0),
             (league::RELEGATED_COUNT, 1, 1), (league::GAMES_PER_CLUB, 2, 0x26)]
    };
    v.extend_from_slice(&[(league::CUSTOM_COMPARE, 4, 0), (league::WIN_POINTS, 1, 3), // mwb13: the Premier points for a win
                          (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0), (league::ROUND_DRAW_RULES, 2, 0),
                          (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 3),
                          (league::UPPER_DIVISION_ID, 4, 0xffff_ffff), (league::LOWER_DIVISION_ID, 4, read_usize(KEY_CLUB_COMP_SCOTTISH_FIRST_DIVISION) as u32),
                          (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTES_NAMED, 1, 5)]);
    v
}

/// rule: the Scottish Premier Division (12 clubs) plays 3 rounds (33 matches each) and then the split: 3 points for a
/// win and 1 for a draw, the bottom club goes down to the First (in 1999: 10 clubs, 4 rounds, the bottom club to a
/// relegation play-off); tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; 5
/// substitutes named and 3 allowed, squad flags 2; builds the round schedule (virtual +0x3c, 38 rounds) and, outside
/// 1999, sets the round count to 33 (the rounds before the split); returns 1
/// Original: `ScoPrm::set_up_rules` (0x7f2970, sco_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn sco_prm_set_up_rules(this: usize) -> u32 {
    let league = League(this);
    covered!("mwb_13 scottish premier rules");
    eng_prm_apply_league_rules(league, &scottish_premier_rules(league));
    if league.year() != 0x7cf {
        write_u16(this + league::ROUND_COUNT, 0x21);
    }
    1
}

/// Comparer for `sco_prm_set_up_rules` (side by side on the rule bytes and, outside 1999, the round count; the copy
/// without the round table).
pub(crate) unsafe extern "thiscall" fn compare_sco_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_SCO_PRM_SET_UP_RULES);
    let mut written = scottish_premier_rules(League(this));
    if League(this).year() != 0x7cf {
        written.push((league::ROUND_COUNT, 2, 0x21));
    }
    eng_prm_compare_league_rules("sco_prm_set_up_rules", League(this), written, || o(this), || sco_prm_set_up_rules(this))
}
