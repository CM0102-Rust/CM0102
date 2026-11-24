// ---------- sbF_2: the Greek Cup schedule, first group table and group stages (gre_cup.cpp) ----------

/// The Greek Cup groups: six matchdays from 11 August to 8 September (11, 18, 22, 25 and 29 August and 8 September;
/// weekday 2 with code 2, or weekday 6 with code 1).
pub(crate) const GREEK_CUP_GROUP_ROUNDS: [ItaCupGroupRound; 6] = [(0xb, 7, 2, 2), (0x12, 7, 2, 2), (0x16, 7, 6, 1), (0x19, 7, 2, 2), (0x1d, 7, 6, 1), (8, 8, 2, 2)]; // sbF2: the Greek Cup group matchdays

/// The Greek Cup knockout: the 1st round drawn on 16 September and played on 15 December (12 clubs, 6 ties, all new),
/// the 2nd drawn 16 December and played 13 January (16 clubs, 8 ties, 10 new from entry 12), the quarter-finals drawn
/// 14 January and played over two legs 14 days apart from 24 February, the semi-finals drawn 8 March and played over
/// two legs from 5 April, the final drawn 20 April and played on 10 May at a neutral ground [inference: venue 4];
/// single-match tie rules 3; category 4 up to the quarter-finals and 6 after; no prize per round (−1).
pub(crate) const GREEK_CUP_KNOCKOUT_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0x10, 8, 0, 3), (0xf, 11, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF2: the Greek Cup 1st round
    cql10r((0x10, 11, 0, 3), (0xd, 0, 1, 2, 2, 0), 0x1e, (0, 3, 0), 4, (0x10, 8, 0xa, 0xc), (0, 1, 0), (0, 0xffff_ffff, 0)),
    cql10r((0xe, 0, 1, 3), (0x18, 1, 1, 2, 2, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((8, 2, 1, 3), (5, 3, 1, 2, 2, 0), 0x82, (0, 0, 3), 6, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((0x14, 3, 1, 3), (0xa, 4, 1, 2, 2, 4), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffff_ffff, 0)),
];

/// Rule: the Greek Cup's eleven groups of four (the cup's own group and stages 0 to 9) play six matchdays from 11
/// August to 8 September, each group a league stage (is_league = 1, label 0x3e9 + stage); the knockout (stage 10) has
/// 22 clubs: 12 in the 1st round (December), 10 more in the 2nd (January), then two-legged quarter- and semi-finals
/// and the final on 10 May.
///
/// `create_schedule` with the knockout at stage 10, the label 0x3e9, `GREEK_CUP_GROUP_ROUNDS`,
/// `GREEK_CUP_KNOCKOUT_ROUNDS` and the boxes at lines 0xc3 and 0xee of gre_cup.cpp (checked by emulation of the
/// original for every stage −128..127 in an odd and an even year: the same calls and the same 16 fields per round as
/// ItaCup::create_schedule). Above stage 10: 0.
/// Original: `GreCup::create_schedule` (0x5e8bf0, gre_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_schedule(League(this), stage, rounds, extra, league, 0xa, 0x3e9, &GREEK_CUP_GROUP_ROUNDS, &GREEK_CUP_KNOCKOUT_ROUNDS, 0x9cdab0, 0xc3, 0xee,
        "sbF_2 greek cup groups", "sbF_2 greek cup knockout rounds")
}

/// Comparer for `gre_cup_create_schedule` (side by side: `ita_cup_compare_schedule`).
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    ita_cup_compare_schedule("gre_cup_create_schedule", stage, rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| gre_cup_create_schedule(this, stage, r, e, l), 0xa)
}

/// The Greek Cup entrant pool: 44 clubs in eleven columns of four (entry `group + 11 × k` is club k of group `group`;
/// group 0 is the cup's own group, group g + 1 the group stage g). The pool is read, never cleared: each column is used
/// once.
pub(crate) const GREEK_CUP_GROUPS: usize = 0xb;

/// Pool entry `k` of Greek Cup group `group` (the entrant vector `ita_cup_fields::ENTRANT_CLUBS`, column `group`).
pub(crate) unsafe fn greek_cup_pool_club(this: League, group: usize, k: usize) -> usize {
    read_usize(cup_pool_entry(this, group + k * GREEK_CUP_GROUPS))
}

/// Rule: the Greek Cup's own group (the cup is the league of its first group) takes the four clubs of column 0 of the
/// entrant pool; the group draw news follows.
///
/// `league::CLUB_COUNT` = 4; the table (`exe::crt_malloc` of 0xec; out of memory, the box at line 0x1ab of gre_cup.cpp
/// and 0); row k gets pool entry 11 × k (`exe::league_init_table_row`(this, k, club, 0, 0); the original pushes the
/// dword of the local byte with stack garbage above it, the routine reads only the byte; the index is a signed byte
/// compared with the word `league::CLUB_COUNT`); unlike the Welsh and Irish cups the pool entry is not cleared. Then
/// the group draw news (League::send_group_draw_news, thiscall, ret 0). Returns 1.
/// Original: `GreCup::create_table` (0x5e9260, gre_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9cdab0, 0x1ab);
        return 0;
    }
    covered!("sbF_2 greek cup first group");
    let mut k: i8 = 0;
    while (k as i16) < this.club_count() {
        exe::league_init_table_row(this, k as u8 as u32, greek_cup_pool_club(this, 0, k as usize), 0, 0);
        k = k.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Comparer for `gre_cup_create_table`: the plan (column 0 of the pool), then only the original (`init_table_row`
/// allocates and the news goes to the managers); returns 1, four clubs, the table rows in pool order and the pool
/// untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let t = League(this);
    let plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, 0, k)).collect();
    let r = o(this);
    let found: Vec<usize> = if t.table() != 0 { (0..4).map(|k| read_usize(t.table() + k * league_table_row::SIZE)).collect() } else { Vec::new() };
    let ok = r == 1 && t.club_count() == 4 && found == plan && (0..4).all(|k| greek_cup_pool_club(t, 0, k) == plan[k]);
    SCOREBOARD.record(ok, || format!("gre_cup_create_table({:#x}) returned {}; plan {:x?} table {:x?}", this, r, plan, found));
    r
}

/// How a cup played as a league builds one of its group stages (`build_cup_group_stage`): the club count, the first
/// local tie-break, the 4th and the 18th arguments of League_Stage::League_Stage (1 or 2, and 0 or 0x82 [inference:
/// the places through and the stage's round name]), the source file and the two error-box lines.
pub(crate) struct CupGroupStage {
    pub(crate) clubs: u32,
    pub(crate) tiebreak: u8,
    pub(crate) arg4: u32,
    pub(crate) arg18: u32,
    pub(crate) file: usize,
    pub(crate) rows: (u32, u32),
}

/// Builds group stage `g` of a cup played as a league, in the original routine's order (the body of
/// WelPrmCup::create_group_stage, GreCup::create_group_stage, HolCup::create_group_stage and
/// KorLeagueCup::create_group_b_stage, compared instruction by instruction: only the values of `CupGroupStage` and how
/// the clubs leave the entrant pool change).
///
/// The round table of the stage (virtual `comp_vtable::CREATE_SCHEDULE`(g, &rounds, &label, 0)); the club list
/// (`exe::crt_malloc` of 4 × clubs; null: the box of the first line and 0) with `take`(k) for k = 0.. clubs − 1 (the
/// club of slot k, taken from the pool by the caller's rule); a League_Stage (`exe::crt_operator_new`(0xee) and
/// League_Stage::League_Stage 0x68b100, thiscall, ret 0x58: this cup as the parent, the club count, the list, `arg4`,
/// `league::COMPETITION`, the round table and its count, the win, draw and category bytes, the tie-breaks
/// (`league::TIEBREAKS`), a local tie-break list (`tiebreak`, 0, 0, 0), the year, g, the label, the importance, 1,
/// `arg18`, the league flags, −1, 0, 2) into stage slot g (0 without memory); the list and the round table freed; no
/// stage: the box of the second line and 0; otherwise the group draw news of the stage (League::send_group_draw_news),
/// the current stage = g and 1. The original pushes the round count, the label, the year and the flags as dwords with
/// stale high bytes; League_Stage::League_Stage reads the words and bytes.
pub(crate) unsafe fn build_cup_group_stage(this: League, g: u32, p: &CupGroupStage, mut take: impl FnMut(usize) -> usize, ramo: &'static str) -> u32 {
    let tiebreaks: [u8; 4] = [p.tiebreak, 0, 0, 0];
    let (mut rounds, mut label) = (0i16, 0i16);
    let schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let table = schedule(this.addr(), g, &mut rounds, &mut label, 0);
    let clubs = exe::crt_malloc(4 * p.clubs);
    if clubs == 0 {
        show_error_at(p.file, p.rows.0);
        return 0;
    }
    covered!(ramo);
    for k in 0..p.clubs as usize {
        write_usize(clubs + 4 * k, take(k));
    }
    let n = exe::crt_operator_new(0xee);
    let mut s = 0usize;
    if n != 0 {
        type LeagueStageCtor = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize;
        let ctor: LeagueStageCtor = core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        s = ctor(n, this.addr(), p.clubs, clubs, p.arg4, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32,
            this.match_category() as u32, this.addr() + league::TIEBREAKS, tiebreaks.as_ptr() as usize, this.year() as u16 as u32, g, label as u16 as u32,
            this.importance() as u32, 1, p.arg18, (this.league_flags() as u16) as u32, 0xffff_ffff, 0, 2);
    }
    write_usize(this.stages() + 4 * g as usize, s);
    exe::crt_free(clubs);
    exe::crt_free(table);
    let s = read_usize(this.stages() + 4 * g as usize);
    if s == 0 {
        show_error_at(p.file, p.rows.1);
        return 0;
    }
    exe::league_send_group_draw_news(s);
    this.set_current_stage(g as usize);
    1
}

/// The address of entrant pool entry `i` (`ita_cup_fields::ENTRANT_CLUBS`, a vector of club pointers).
pub(crate) unsafe fn cup_pool_entry(this: League, i: usize) -> usize {
    read_usize(this.addr() + ita_cup_fields::ENTRANT_CLUBS) + 4 * i
}

/// The Greek Cup group stages: four clubs, the local tie-break 2, 1 as the 4th argument, the boxes at lines 0x271 and
/// 0x286 of gre_cup.cpp.
pub(crate) const GREEK_CUP_GROUP_STAGE: CupGroupStage = CupGroupStage { clubs: 4, tiebreak: 2, arg4: 1, arg18: 0, file: 0x9cdab0, rows: (0x271, 0x286) };

/// Rule: Greek Cup group stage g (0 to 9) is a league stage of the four clubs of pool column g + 1 (they stay in the
/// pool), played over the cup's group schedule; the group draw news follows and the current stage becomes g.
///
/// `build_cup_group_stage` with `GREEK_CUP_GROUP_STAGE`, club k from pool entry g + 1 + 11 × k, not cleared (against
/// WelPrmCup::create_group_stage the pool stride is 11, the entries are not cleared and the 4th argument of
/// League_Stage::League_Stage is 1).
/// Original: `GreCup::create_group_stage` (0x5e9650, gre_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let this = League(this);
    build_cup_group_stage(this, g, &GREEK_CUP_GROUP_STAGE, |k| read_usize(cup_pool_entry(this, g as usize + 1 + k * GREEK_CUP_GROUPS)), // sbF2: the Greek group columns
        "sbF_2 greek cup group stage")
}

/// Comparer for `gre_cup_create_group_stage`: only the original (it builds a stage with its fixtures and sends the
/// news); then the return 1, the stage in slot g with four clubs and the cup's year, the current stage g, the stage
/// table holding pool column g + 1 (in any order: the constructor sorts it) and the pool untouched.
pub(crate) unsafe extern "thiscall" fn compare_gre_cup_create_group_stage(this: usize, g: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%);
    let t = League(this);
    let mut plan: Vec<usize> = (0..4).map(|k| greek_cup_pool_club(t, g as usize + 1, k)).collect();
    let r = o(this, g);
    let f = League(read_usize(t.stages() + 4 * g as usize));
    let mut ok = r == 1 && !f.is_null() && t.current_stage() as u32 == g && f.club_count() == 4 && f.year() == t.year()
        && (0..4).all(|k| greek_cup_pool_club(t, g as usize + 1, k) == plan[k]);
    if ok && f.table() != 0 {
        let mut found: Vec<usize> = (0..4).map(|k| read_usize(f.table() + k * league_table_row::SIZE)).collect();
        found.sort_unstable();
        plan.sort_unstable();
        ok = found == plan;
    }
    SCOREBOARD.record(ok, || format!("gre_cup_create_group_stage({:#x}, {}) returned {}, stage {:#x}, plan {:x?}", this, g, r, f.addr(), plan));
    r
}
