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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    eng_cc_cup_compare_entrant_list("sco_chal_cup_collect_entrants", this, &[(this.addr() + cup::TEAM_COUNT, 2), (G_MOUSE_LEFT_BUTTON_DOWN, 4)], || o(this.addr()), || challenge_cup_entrants(this),
        |_, n| n.clamp(0, 0x1e) as usize * 6)
}
