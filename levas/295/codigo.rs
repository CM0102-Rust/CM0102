// ---------- batch 295: the Irish FAI Senior Challenge Cup: the round table, the entrants and the season turnover (ire_chal_cup.cpp) ----------

/// The FAI Senior Challenge Cup: six single-leg rounds with replays (`(1, 1, 0)`: replay on, one match, no gap), from
/// November to a final in late April of the next year at a neutral ground [inference: venue 0x400]; two-leg tiebreak 3
/// [inference]; category 4, and 6 in the final; the tie winner's prize is a share of the prize fund (−1).
pub(crate) const IRISH_CHALLENGE_CUP_ROUNDS: [EngCcCupCupRound; 6] = [
    cql10r((1, 0xa, 0, 0), (0x1c, 0xa, 0, 6, 1, 0), 0xa, (0, 0, 3), 4, (0x14, 0xa, 0x14, 0), (1, 1, 0), (0, 0xffffffff, 0)), // mwa1: the first round of the Irish Challenge Cup
    cql10r((6, 0xb, 0, 0), (9, 0, 1, 6, 1, 0), 0x14, (0, 0, 3), 4, (0x20, 0x10, 0x16, 0x14), (1, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x11, 0, 1, 0), (6, 1, 1, 6, 1, 0), 0x1e, (0, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0xe, 1, 1, 0), (4, 2, 1, 5, 1, 0), 0x78, (0, 0, 3), 4, (8, 4, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0xd, 2, 1, 0), (2, 3, 1, 6, 1, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
    cql10r((3, 3, 1, 0), (0x1e, 3, 1, 6, 1, 0x400), 0x96, (0, 0, 3), 6, (2, 1, 0, 0), (1, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the FAI Senior Challenge Cup has six single-leg rounds with replays: the first drawn on 1/11 and played on 28/11
/// (20 clubs, 10 ties), the second drawn 6/12 and played 9/1 of the next year (32 clubs: the 10 winners and 22 entering
/// now), then 17/1 and 6/2 (16), 14/2 and 4/3 (quarter-finals), 13/3 and 2/4 (semi-finals) and the final drawn 3/4 and
/// played on 30/4 at a neutral ground [inference: venue 0x400], each moved to its weekday; two-leg tiebreak 3
/// [inference]; category 4, and 6 in the final; the tie winner's prize is a share of the prize fund (−1).
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1 (the byte); `out` (when given) = 0, `rounds` = 6, `extra` = 0; the table (`exe::crt_malloc` of 6 × 0x68; out of
/// memory, the error box at line 0x9c and 0) with the six rounds of `IRISH_CHALLENGE_CUP_ROUNDS`
/// (`eng_cc_cup_create_single_stage_round_table`, checked instruction by instruction: only the table, the file and the
/// line change). Returns the table.
/// Original: `IreChalCup::create_round_table` (0x634360, ire_chal_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_chal_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &IRISH_CHALLENGE_CUP_ROUNDS, 0x9e9e94, 0x9c, "mwa_1 irish challenge cup rounds")
}

unsafe extern "thiscall" fn compare_ire_chal_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_CHAL_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ire_chal_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_chal_cup_create_round_table(this, stage, r, e, s))
}

/// The id of the club's division competition (+0 of the `club::DIVISION` record), or None without a division.
pub(crate) unsafe fn irish_club_division_id(club: Club) -> Option<usize> {
    let d = club.division();
    if d.is_null() { None } else { Some(read_usize(d.addr())) }
}

/// The clubs the Irish cups go through: `G_CLUB_COUNT` − 2 × `G_NATION_COUNT` (the national sides at the end of the
/// table left out), re-read on every turn as the original does.
pub(crate) unsafe fn irish_cup_club_count() -> i32 {
    read_i32(G_CLUB_COUNT) - 2 * read_i32(G_NATION_COUNT)
}

/// The body of `ire_chal_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 42; the entrant table (`exe::crt_malloc` of 0xfc into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x12d and −1). One pass over the clubs (`irish_cup_club_count`, in table order): a club of one of
/// the four regional senior leagues (Munster, Leinster, Ulster, Connacht: the key globals
/// `KEY_CLUB_COMP_IRISH_*_SENIOR_LEAGUE`) goes into a local pool while it holds fewer than 200; otherwise a club of the
/// Premier or First Division goes into the next entry from entry 20 on (the club and status 0; no bound check: the 22
/// League of Ireland clubs fill entries 20 to 41). Then five draws per league, in the order Munster (entries 0 to 4),
/// Leinster (5 to 9), Ulster (10 to 14) and Connacht (15 to 19): a random slot of the pool (`exe::utils_random`(pool
/// size), cdecl) is drawn again until it holds a club of that league, which takes the entry (status 0) and leaves the
/// pool. The class byte (+4) of the entries is not written. Returns 1.
/// (The original loops forever if a league has fewer than five clubs; so does the copy.)
pub(crate) unsafe fn collect_irish_challenge_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x2a);
    let t = exe::crt_malloc(0xfc);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9e9e94, 0x12d);
        return (-1, 0);
    }
    let mut pool = [0usize; 200];
    let mut n = 0i16;
    let mut slot = 0usize;
    let regional = [KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE];
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 200 && regional.iter().any(|&k| id == read_usize(k)) {
                pool[n as usize] = c.addr();
                n += 1;
            } else if id == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_IRISH_FIRST_DIVISION) {
                slot += 6;
                write_usize(this.team_table() + slot + 0x72, c.addr()); // mwa1: the League of Ireland clubs from entry 20
                write_u8(this.team_table() + slot + 0x77, 0);
            }
        }
        i += 1;
    }
    covered!("mwa_1 irish challenge cup entrants");
    let order = [KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_ULSTER_SENIOR_LEAGUE, KEY_CLUB_COMP_IRISH_CONNACHT_SENIOR_LEAGUE];
    for (g, &key) in order.iter().enumerate() {
        for k in 0..5usize { // mwa1: five clubs of each regional league
            let p = loop {
                let r = exe::utils_random(n as i32) as usize;
                let c = pool[r];
                if c != 0 && read_usize(Club(c).division().addr()) == read_usize(key) {
                    break r;
                }
            };
            let e = this.team_table() + (g * 5 + k) * 6;
            write_usize(e, pool[p]);
            pool[p] = 0;
            write_u8(e + 5, 0);
        }
    }
    (1, 42)
}

/// Rule: the FAI Senior Challenge Cup takes 42 clubs: five drawn at random from each of the four regional senior
/// leagues (Munster, Leinster, Ulster and Connacht, in that order) and the 22 clubs of the Premier and First Divisions.
/// Original: `IreChalCup::collect_entrants` (0x6347d0, ire_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_chal_cup_collect_entrants(this: usize) -> i32 {
    collect_irish_challenge_cup_entrants(Cup(this)).0
}

/// Side-by-side comparer for the Irish cups' entrant routines (they allocate `cup::TEAM_TABLE` and may draw): compares
/// `cup::TEAM_COUNT`, [0xb67a34], the generator state and, of the entries the routine writes, the club and the status
/// (+5), and the class (+4) when `class` (some routines leave it as malloc left it); frees the copy's table.
pub(crate) unsafe fn compare_irish_cup_entrants(name: &str, this: Cup, class: bool, o: impl FnOnce() -> i32, c: impl FnOnce() -> (i32, i32)) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let nr = core::cell::Cell::new(0i32);
    let s = crate::rng::STATE_REGIONS;
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4), s[0], s[1]];
    let (a, b, equal, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let (r, n) = crate::rng::without_counting(c);
        lr.set(this.team_table());
        nr.set(n);
        this.set_team_table(lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let mut list = (lo == 0) == (lr == 0);
    if lo != 0 && lr != 0 {
        for k in 0..nr.get().max(0) as usize {
            let (x, y) = (lo + k * 6, lr + k * 6);
            if read_u32(x) != read_u32(y) || read_u8(x + 5) != read_u8(y + 5) || (class && read_u8(x + 4) != read_u8(y + 4)) {
                list = false;
            }
        }
    }
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(equal && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {} ({} entries)", name, this.addr(), a, b, equal, list, nr.get()));
    a
}

unsafe extern "thiscall" fn compare_ire_chal_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_CHAL_CUP_COLLECT_ENTRANTS);
    compare_irish_cup_entrants("ire_chal_cup_collect_entrants", Cup(this), false, || o(this), || collect_irish_challenge_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the FAI Senior Challenge Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made
/// again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, IreChalCup::collect_entrants 0x6347d0, the file and the error box lines 0x1c4, 0x1cb and 0x1d2
/// change).
/// Original: `IreChalCup::start_new_season` (0x634a60, ire_chal_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_chal_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x6347d0, 0x9e9e94, [0x1c4, 0x1cb, 0x1d2], 1, "mwa_1 irish challenge cup turnover") // mwa1: the Irish Challenge Cup turnover
}

unsafe extern "thiscall" fn compare_ire_chal_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_CHAL_CUP_START_NEW_SEASON);
    compare_cup_new_season("ire_chal_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the Leinster Senior Cup: the round table, the clubs and the season turnover (ire_leinster_cup.cpp) ----------

/// The Leinster Senior Cup: four knockout rounds of single matches (no replay) for 16 clubs, from August to a final in
/// March of the next year; tie rule 3 [inference]; category 4.
pub(crate) const LEINSTER_SENIOR_CUP_ROUNDS: [EngCcCupCupRound; 4] = [
    cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa2: the first round of the Leinster cup
    cql10r((0x19, 7, 0, 4), (0x12, 9, 0, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x14, 9, 0, 4), (0x11, 0, 1, 2, 2, 0), 0x82, (0, 3, 0), 4, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x13, 0, 1, 2), (0xe, 2, 1, 2, 2, 0), 0x96, (0, 3, 0), 4, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Leinster Senior Cup is a 16-club knockout of single matches: the first round drawn on 1/8 and played on
/// 23/8, the quarter-finals drawn 25/8 and played 18/10, the semi-finals drawn 20/10 and played 17/1 of the next year
/// and the final drawn 19/1 and played on 14/3, each moved to its weekday; tie rule 3 [inference: extra time and
/// penalties]; category 4; the tie winner's prize is a share of the prize fund (−1).
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 4, `extra` = 0; the table (`exe::crt_malloc` of 4 × 0x68; out of memory, the
/// error box at line 0x9d and 0) with `LEINSTER_SENIOR_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction). Returns the table.
/// Original: `IreLeinsterCup::create_round_table` (0x636760, ire_leinster_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &LEINSTER_SENIOR_CUP_ROUNDS, 0x9e9f0c, 0x9d, "mwa_2 leinster cup rounds")
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_LEINSTER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ire_leinster_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_leinster_cup_create_round_table(this, stage, r, e, s))
}

/// Whether the club's stadium town is in Leinster (the exe's ire_club_is_in_leinster 0x6399c0, push club; add esp, 4 ->
/// cdecl, a gap function of ire_pres_cup.cpp not yet ported: Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or
/// Kilkenny).
pub(crate) unsafe fn ire_club_is_in_leinster(club: Club) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6399c0usize);
    f(club.addr()) != 0
}

/// The body of `ire_leinster_cup_select_clubs`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 16; the entrant table (`exe::crt_malloc` of 0x60 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x108 and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Leinster Senior
/// League (`KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE`) goes into a local pool while it holds fewer than 40;
/// otherwise a Premier or First Division club based in Leinster (`ire_club_is_in_leinster`) takes the next entry from
/// entry 4 on (club and status 0), while the entry offset counter is below 0x4e (so up to 13 clubs: the 13th would
/// write entry 16, past the 16-entry table, a defect of the original that the career's 12 or fewer Leinster clubs never
/// reach). Then the pool is sorted and its best shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 4))
/// and its first four take entries 0 to 3 (status 0). The class byte (+4) is not written. Returns 1.
pub(crate) unsafe fn select_leinster_cup_clubs(this: Cup) -> (i32, i32) {
    this.set_team_count(0x10);
    let t = exe::crt_malloc(0x60);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9e9f0c, 0x108);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than four Leinster Senior League clubs
    // the copy would take nulls where the original took leftovers (the career's league has more)
    let mut pool = [0usize; 40];
    let mut n = 0i16;
    let mut slot = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x28 && id == read_usize(KEY_CLUB_COMP_IRISH_LEINSTER_SENIOR_LEAGUE) {
                pool[n as usize] = c.addr();
                n += 1;
            } else if (id == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_IRISH_FIRST_DIVISION)) && ire_club_is_in_leinster(c) && slot < 0x4e {
                slot += 6;
                write_usize(this.team_table() + slot + 0x12, c.addr()); // mwa2: the League of Ireland clubs from Leinster
                write_u8(this.team_table() + slot + 0x17, 0);
            }
        }
        i += 1;
    }
    covered!("mwa_2 leinster cup clubs");
    exe::comp_util_sort_and_shuffle_top_clubs(pool.as_mut_ptr() as usize, n as i32 as u32, 4);
    for k in 0..4usize { // mwa2: the four best of the Leinster Senior League
        write_usize(this.team_table() + k * 6, pool[k]);
        write_u8(this.team_table() + k * 6 + 5, 0);
    }
    (1, 4 + (slot / 6) as i32)
}

/// Rule: the Leinster Senior Cup takes 16 clubs: four of the Leinster Senior League (the best, shuffled among the top
/// twelve) and the Premier and First Division clubs based in Leinster.
/// Original: `IreLeinsterCup::select_clubs` (0x636a80, ire_leinster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_select_clubs(this: usize) -> i32 {
    select_leinster_cup_clubs(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_select_clubs(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_LEINSTER_CUP_SELECT_CLUBS);
    compare_irish_cup_entrants("ire_leinster_cup_select_clubs", Cup(this), false, || o(this), || select_leinster_cup_clubs(Cup(this)))
}

/// Rule: at the season turn the Leinster Senior Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the clubs, the rules, the tie tree and the predicted positions are made again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, IreLeinsterCup::select_clubs 0x636a80, the file and the error box lines 0x167, 0x16e and 0x175
/// change).
/// Original: `IreLeinsterCup::start_new_season` (0x636c20, ire_leinster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_leinster_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x636a80, 0x9e9f0c, [0x167, 0x16e, 0x175], 1, "mwa_2 leinster cup turnover")
}

unsafe extern "thiscall" fn compare_ire_leinster_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_LEINSTER_CUP_START_NEW_SEASON);
    compare_cup_new_season("ire_leinster_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the Munster Senior Cup: the round table, the entrants and the season turnover (ire_munster_cup.cpp) ----------

/// The Munster Senior Cup: five knockout rounds of single matches (no replay) for 20 clubs, from August to a final in
/// May of the next year; tie rule 3 [inference]; category 4, and 6 in the final.
pub(crate) const MUNSTER_SENIOR_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((1, 7, 0, 1), (0x17, 7, 0, 2, 2, 0), 0xa, (0, 3, 0), 4, (0x10, 8, 0x10, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwa3: the preliminary round of the Munster cup
    cql10r((0x19, 7, 0, 4), (0x12, 9, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (8, 4, 0, 0x14), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x14, 9, 0, 4), (0x11, 0, 1, 2, 2, 0), 0x78, (0, 3, 0), 4, (8, 4, 4, 0x10), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0x13, 0, 1, 4), (0xe, 2, 1, 2, 2, 0), 0x82, (0, 3, 0), 4, (4, 2, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql10r((0xf, 2, 1, 3), (9, 4, 1, 2, 1, 0x200), 0x96, (0, 3, 0), 6, (2, 1, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
];

/// Rule: the Munster Senior Cup has five knockout rounds of single matches: the preliminary round drawn on 1/8 and
/// played on 23/8 (16 clubs, 8 ties), the next drawn 25/8 and played 18/10 (the 8 winners), the quarter-finals drawn
/// 20/10 and played 17/1 of the next year (the 4 winners and the 4 entrants still out), the semi-finals drawn 19/1 and
/// played 14/3 and the final drawn 15/3 and played on 9/5 [inference: home side code 0x200], each moved to its weekday;
/// tie rule 3 [inference]; category 4, and 6 in the final; the tie winner's prize is a share of the prize fund (−1).
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 5, `extra` = 0; the table (`exe::crt_malloc` of 5 × 0x68; out of memory, the
/// error box at line 0x9c and 0) with `MUNSTER_SENIOR_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction). Returns the table.
/// Original: `IreMunsterCup::create_round_table` (0x638a60, ire_munster_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &MUNSTER_SENIOR_CUP_ROUNDS, 0x9e9f84, 0x9c, "mwa_3 munster cup rounds")
}

unsafe extern "thiscall" fn compare_ire_munster_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_MUNSTER_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ire_munster_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_munster_cup_create_round_table(this, stage, r, e, s))
}

/// The four League of Ireland clubs that play the Munster Senior Cup, by English name (the exe's strings at 0x9e9ff4,
/// 0x9e9fe4, 0x9e9fd0 and 0x9e9fc0, compared with an inline strcmp).
pub(crate) const MUNSTER_LEAGUE_OF_IRELAND_CLUBS: [&[u8]; 4] = [b"Cork City", b"Cobh Ramblers", b"Waterford United", b"Limerick F.C."];

/// Whether the C string at `p` is `name` (byte for byte, up to its terminating zero).
pub(crate) unsafe fn c_string_is(p: usize, name: &[u8]) -> bool {
    for (k, &b) in name.iter().enumerate() {
        if read_u8(p + k) != b {
            return false;
        }
    }
    read_u8(p + name.len()) == 0
}

/// The body of `ire_munster_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 20; the entrant table (`exe::crt_malloc` of 0x78 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x11a and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Munster Senior
/// League (`KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE`) goes into a local pool while it holds fewer than 40; otherwise
/// a Premier or First Division club whose English name (`exe::club_get_english_name`, cdecl) is Cork City, Cobh
/// Ramblers, Waterford United or Limerick F.C. takes the next entry from entry 16 on (club and status 0), up to four.
/// Then the pool is sorted and its best shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 16)) and its
/// first sixteen take entries 0 to 15 (status 0). The class byte (+4) is not written. Returns 1.
pub(crate) unsafe fn collect_munster_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x14);
    let t = exe::crt_malloc(0x78);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9e9f84, 0x11a);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than sixteen Munster Senior League
    // clubs the copy would take nulls where the original took leftovers (the career's league has more)
    let mut munster = [0usize; 40];
    let mut n = 0i16;
    let mut taken = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x28 && id == read_usize(KEY_CLUB_COMP_IRISH_MUNSTER_SENIOR_LEAGUE) {
                munster[n as usize] = c.addr();
                n += 1;
            } else if id == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_IRISH_FIRST_DIVISION) {
                let name = exe::club_get_english_name(c);
                if MUNSTER_LEAGUE_OF_IRELAND_CLUBS.iter().any(|x| c_string_is(name, x)) && taken < 0x18 { // mwa3: the four Munster clubs of the League of Ireland
                    write_usize(this.team_table() + taken + 0x60, c.addr());
                    write_u8(this.team_table() + taken + 0x65, 0);
                    taken += 6;
                }
            }
        }
        i += 1;
    }
    covered!("mwa_3 munster cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(munster.as_mut_ptr() as usize, n as i32 as u32, 0x10);
    for (k, &club) in munster.iter().take(16).enumerate() { // mwa3: the sixteen best of the Munster Senior League
        write_usize(this.team_table() + k * 6, club);
        write_u8(this.team_table() + k * 6 + 5, 0);
    }
    (1, 16 + (taken / 6) as i32)
}

/// Rule: the Munster Senior Cup takes 20 clubs: sixteen of the Munster Senior League (the best, shuffled among the top
/// forty-eight), entries 0 to 15, and the four Munster clubs of the League of Ireland (Cork City, Cobh Ramblers,
/// Waterford United and Limerick), entries 16 to 19.
/// Original: `IreMunsterCup::collect_entrants` (0x638e30, ire_munster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_collect_entrants(this: usize) -> i32 {
    collect_munster_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_munster_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_MUNSTER_CUP_COLLECT_ENTRANTS);
    compare_irish_cup_entrants("ire_munster_cup_collect_entrants", Cup(this), false, || o(this), || collect_munster_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the Munster Senior Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made
/// again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, IreMunsterCup::collect_entrants 0x638e30, the file and the error box lines 0x179, 0x180 and 0x187
/// change).
/// Original: `IreMunsterCup::start_new_season` (0x6390d0, ire_munster_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_munster_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x638e30, 0x9e9f84, [0x179, 0x180, 0x187], 1, "mwa_3 munster cup turnover")
}

unsafe extern "thiscall" fn compare_ire_munster_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_MUNSTER_CUP_START_NEW_SEASON);
    compare_cup_new_season("ire_munster_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the Irish President's Cup: the round table, the two clubs and the season turnover (ire_pres_cup.cpp) ----------

/// The President's Cup: one match between two clubs, drawn on 5/7 and played on 4/8 [inference: home side code 0x400];
/// tie rule 3 [inference: penalties]; category 6; no prize.
pub(crate) const IRISH_PRESIDENTS_CUP_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (4, 7, 0, 4, 2, 0x400), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa4: the President's Cup final
];

/// Rule: the President's Cup is a single match between two clubs, drawn on 5/7 and played on 4/8 (moved to weekday 4)
/// [inference: home side code 0x400]; tie rule 3 [inference: penalties]; category 6; no prize.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 1, `extra` = 0; the table (`exe::crt_malloc` of 0x68; out of memory, the
/// error box at line 0x95 and 0) with `IRISH_PRESIDENTS_CUP_ROUNDS` (`eng_cc_cup_create_single_stage_round_table`,
/// checked instruction by instruction: the original writes the same fields in another order). Returns the table.
/// Original: `IrePresCup::create_round_table` (0x639530, ire_pres_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &IRISH_PRESIDENTS_CUP_ROUNDS, 0x9ea000, 0x95, "mwa_4 presidents cup rounds")
}

unsafe extern "thiscall" fn compare_ire_pres_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_IRE_PRES_CUP_CREATE_ROUND_TABLE);
    compare_cup_round_table("ire_pres_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| ire_pres_cup_create_round_table(this, stage, r, e, s))
}

/// The body of `ire_pres_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 2. The clubs whose last season's division (`club::LAST_DIVISION`) was the Premier Division
/// (`KEY_CLUB_COMP_IRISH_PREMIER_DIVISION`) go into a 12-place table by last season's position (`club::LAST_POSITION`,
/// a signed byte, 1 to 12; the last club of a position wins it; all `G_CLUB_COUNT` clubs are scanned). The entrant table (`exe::crt_calloc`(2, 6), cdecl; out of memory, the error box at line 0xd9
/// and −1). From the top, the first two clubs based in Leinster (`ire_club_is_in_leinster`) take entries 0 and 1 (each
/// leaves the table); if the second entry is still empty, the remaining clubs from the top fill the empty entry 0 (it
/// stays in the table) and then entry 1. The class and status bytes of both entries are zeroed. Returns 1.
pub(crate) unsafe fn collect_presidents_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(2);
    let mut by_position = [0usize; 12];
    let mut i = 0i32;
    while i < read_i32(G_CLUB_COUNT) {
        let c = club_at(i as isize);
        let p = c.last_position() as i8;
        let last = c.last_division();
        if !last.is_null() && read_usize(last.addr()) == read_usize(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) && (1..=12).contains(&p) {
            by_position[p as usize - 1] = c.addr();
        }
        i += 1;
    }
    let t = exe::crt_calloc(2, 6);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0x9ea000, 0xd9);
        return (-1, 0);
    }
    covered!("mwa_4 presidents cup clubs");
    for k in 0..12usize { // mwa4: the two best placed Leinster clubs
        let c = by_position[k];
        if c != 0 && ire_club_is_in_leinster(Club(c)) {
            if read_usize(t) != 0 {
                write_usize(t + 6, c);
                by_position[k] = 0;
                break;
            }
            write_usize(t, c);
            by_position[k] = 0;
        }
    }
    if read_usize(t + 6) == 0 {
        covered!("mwa_4 presidents cup without two leinster clubs");
        for &c in by_position.iter() {
            if c != 0 {
                if read_usize(t) != 0 {
                    write_usize(t + 6, c);
                    break;
                }
                write_usize(t, c);
            }
        }
    }
    write_u8(t + 4, 0);
    write_u8(t + 5, 0);
    write_u8(t + 0xa, 0);
    write_u8(t + 0xb, 0);
    (1, 2)
}

/// Rule: the President's Cup is between the two best placed clubs of last season's Premier Division based in Leinster
/// (Dublin, Bray, Longford, Dundalk, Drogheda, Athlone or Kilkenny); with fewer than two, the best placed of the others
/// complete it.
/// Original: `IrePresCup::collect_entrants` (0x639670, ire_pres_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_collect_entrants(this: usize) -> i32 {
    collect_presidents_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_ire_pres_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_IRE_PRES_CUP_COLLECT_ENTRANTS);
    compare_irish_cup_entrants("ire_pres_cup_collect_entrants", Cup(this), true, || o(this), || collect_presidents_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the President's Cup is rebuilt: last edition's tables are freed, the statistics archived,
/// the year goes up by one and the two clubs, the rules and the tie tree are made again (no predicted positions).
///
/// The knockout cups' turnover without the predicted positions (`usa_mls_all_stars_cup_start_new_season` with
/// `predicted` false, checked instruction by instruction against UsaMlsAllStars's: only the entrant routine,
/// IrePresCup::collect_entrants 0x639670, the file and the error box lines 0x14d, 0x154 and 0x15b change).
/// Original: `IrePresCup::start_new_season` (0x639820, ire_pres_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_pres_cup_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x639670, 0x9ea000, [0x14d, 0x154, 0x15b], 1, false, "mwa_4 presidents cup turnover")
}

unsafe extern "thiscall" fn compare_ire_pres_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRES_CUP_START_NEW_SEASON);
    compare_cup_new_season("ire_pres_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the Northern Irish Charity Shield: the round table, the two clubs and the season turnover (nir_charity.cpp) ----------

/// The Northern Irish Charity Shield: one match between two clubs, drawn on 5/7 and played on 5/8, without a draw
/// (tie rule bits 0x200: the built order [inference]); category 6; no prize.
pub(crate) const NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS: [EngCcCupCupRound; 1] = [
    cql10r((5, 6, 0, 5), (5, 7, 0, 5, 1, 0), 0, (0, 0x200, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // mwa5: the Charity Shield match
];

/// Rule: the Northern Irish Charity Shield is a single match between two clubs, drawn on 5/7 and played on 5/8 (moved
/// to weekday 5) at the first club's ground [inference: home side code 0], with tie rule bits 0x200 (no draw news
/// [inference]); category 6; no prize.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 1, `extra` = 0; the table (`exe::crt_malloc` of 0x68; out of memory, the
/// error box at line 0x99 and 0) with `NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS`
/// (`eng_cc_cup_create_single_stage_round_table`, checked instruction by instruction). Returns the table.
/// Original: `NirCharity::create_round_table` (0x78a0a0, nir_charity.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &NORTHERN_IRISH_CHARITY_SHIELD_ROUNDS, 0xa695f0, 0x99, "mwa_5 northern irish charity shield rounds")
}

unsafe extern "thiscall" fn compare_nir_charity_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_CHARITY_CREATE_ROUND_TABLE);
    compare_cup_round_table("nir_charity_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out),
        |r, e, s| nir_charity_create_round_table(this, stage, r, e, s))
}

/// The body of `nir_charity_select_teams` (SpaSuper::select_teams's template, checked instruction by instruction: only
/// the two competitions and the file change; the error box lines are the same).
///
/// The two-entry list (`alloc_two_team_list`: `cup::TEAM_COUNT` = 2, `exe::crt_malloc` of 0xc; out of memory, the
/// error box at line 0xc2 and −1). The Irish Cup winner (`latest_placed_club`(`KEY_CLUB_COMP_NORTHERN_IRISH_CUP`, 0);
/// without it, line 0xcb and 0) in entry 0 and the Premier Division champion
/// (`KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`, 0; without it, line 0xd4 and 0) in entry 1; if both are the
/// same club, entry 0 becomes the Irish Cup runner-up (without it, line 0xe0 and 0). Class and status 0; returns 1.
pub(crate) unsafe fn select_northern_irish_charity_teams(this: usize) -> (i32, i32) {
    const F: usize = 0xa695f0;
    let l = match alloc_two_team_list(Cup(this), F, 0xc2) {
        Some(l) => l,
        None => return (-1, 0),
    };
    let a = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRISH_CUP, 0); // mwa5: the Irish Cup winner
    if !set_team_slot(l, 0, a, F, 0xcb) {
        return (0, 0);
    }
    let b = latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0);
    if !set_team_slot(l, 1, b, F, 0xd4) {
        return (0, 0);
    }
    if a == b {
        covered!("mwa_5 northern irish double");
        if !set_team_slot(l, 0, latest_placed_club(KEY_CLUB_COMP_NORTHERN_IRISH_CUP, 1), F, 0xe0) {
            return (0, 0);
        }
    }
    covered!("mwa_5 northern irish charity shield clubs");
    finish_two_team_list(l)
}

/// Rule: the Northern Irish Charity Shield is between the Irish Cup winner and the Premier Division champion; if the
/// same club won both, the Irish Cup runner-up takes its place; if one is missing, the error box.
/// Original: `NirCharity::select_teams` (0x78a1e0, nir_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_charity_select_teams(this: usize) -> i32 {
    select_northern_irish_charity_teams(this).0
}

unsafe extern "thiscall" fn compare_nir_charity_select_teams(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NIR_CHARITY_SELECT_TEAMS);
    compare_team_list("nir_charity_select_teams", Cup(this), || o(this), || select_northern_irish_charity_teams(this))
}

/// Rule: at the season turn the Northern Irish Charity Shield is rebuilt: last edition's tables are freed, the
/// statistics archived, the year goes up by one and the two clubs, the rules and the tie tree are made again (no
/// predicted positions).
///
/// The knockout cups' turnover without the predicted positions (`usa_mls_all_stars_cup_start_new_season` with
/// `predicted` false, checked instruction by instruction: only the entrant routine, NirCharity::select_teams 0x78a1e0,
/// the file and the error box lines 0x125, 0x12c and 0x133 change).
/// Original: `NirCharity::start_new_season` (0x78a410, nir_charity.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_charity_start_new_season(this: usize) -> u32 {
    usa_mls_all_stars_cup_start_new_season(Cup(this), 0x78a1e0, 0xa695f0, [0x125, 0x12c, 0x133], 1, false, "mwa_5 northern irish charity shield turnover")
}

unsafe extern "thiscall" fn compare_nir_charity_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_CHARITY_START_NEW_SEASON);
    compare_cup_new_season("nir_charity_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the Northern Irish Cup: the round table, the entrants and the season turnover (nir_cup.cpp) ----------

/// The Northern Irish Cup (Irish Cup): five single-leg rounds with replays for 32 clubs, from January to a final in
/// May; draw bits 8 in the first three rounds [inference]; category 4; loser prizes of 250, 1,500, 3,000 and 5,500 and
/// 10,000 to the winner of the final.
pub(crate) const NORTHERN_IRISH_CUP_ROUNDS: [EngCcCupCupRound; 5] = [
    cql10r((0xa, 0xb, 0, 6), (0x14, 0, 1, 5, 1, 0), 0x32, (8, 0, 3), 4, (0x20, 0x10, 0x20, 0), (1, 1, 4), (0, 0, 0)), // mwa6: the fifth round of the Irish Cup
    cql10r((0x15, 0, 1, 6), (0x11, 1, 1, 5, 1, 0), 0x3c, (8, 0, 3), 4, (0x10, 8, 0, 0), (1, 1, 0), (0, 0, 0xfa)),
    cql10r((0x12, 1, 1, 6), (0xa, 2, 1, 5, 1, 0), 0x78, (8, 0, 3), 4, (8, 4, 0, 0), (1, 1, 0), (0, 0, 0x5dc)),
    cql10r((0xb, 2, 1, 6), (7, 3, 1, 5, 1, 1), 0x82, (0, 0x100, 3), 4, (4, 2, 0, 0), (1, 1, 0), (0, 0, 0xbb8)),
    cql10r((8, 3, 1, 6), (5, 4, 1, 5, 1, 4), 0x96, (0, 2, 3), 4, (2, 1, 0, 0), (1, 1, 0), (0, 0x2710, 0x157c)),
];

/// The body of `nir_cup_create_round_table`: the single-stage table and, in the semi-finals (round 3), three match-day
/// slots (`exe::cup_set_round_match_day`, cdecl): slot 0 on weekday 4, time 2, code 1, home side 0x40; slot 1 on
/// weekday 5, time 1, code 1, home side 0x80; slot 2 empty (−1, −1, −1, 0). The original sets the slots between the
/// dates and the fields of round 3; they do not overlap (+0x23 on).
pub(crate) unsafe fn build_northern_irish_cup_round_table(this: Cup, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let t = eng_cc_cup_create_single_stage_round_table(this, stage, rounds, extra, out, &NORTHERN_IRISH_CUP_ROUNDS, 0xa69628, 0x9c, "mwa_6 irish cup rounds");
    if t != 0 {
        exe::cup_set_round_match_day(t, 3, 0, 4, 2, 1, 0x40); // mwa6: the semi-finals on two days
        exe::cup_set_round_match_day(t, 3, 1, 5, 1, 1, 0x80);
        exe::cup_set_round_match_day(t, 3, 2, -1, -1, -1, 0);
    }
    t
}

/// Rule: the (Northern) Irish Cup has five single-leg rounds with replays for 32 clubs: the fifth round drawn on 10/12
/// and played on 20/1 of the next year (32 clubs, 16 ties; replay 4 days later), the sixth drawn 21/1 and played 17/2,
/// the quarter-finals drawn 18/2 and played 10/3, the semi-finals drawn 11/3 and played 7/4 over two days (Thursday
/// and Friday slots [inference: weekdays 4 and 5]) at a neutral ground [inference: home side code 1] and the final drawn
/// 8/4 and played on 5/5 [inference: home side code 4], each moved to its weekday; draw bits 8 in the first three
/// rounds, tie rules 0x100 (no fixture news [inference]) in the semi-finals and 2 in the final; category 4; the losers
/// get 250, 1,500, 3,000 and 5,500 from the sixth round on, the winner of the final 10,000.
///
/// The cup's virtual `comp_vtable::CREATE_SCHEDULE` (create_round_table(stage, &rounds, &extra, &out)): only for stage
/// −1; `out` (when given) = 0, `rounds` = 5, `extra` = 0; the table (`exe::crt_malloc` of 5 × 0x68; out of memory, the
/// error box at line 0x9c and 0) with `NORTHERN_IRISH_CUP_ROUNDS` and the semi-final match days
/// (`build_northern_irish_cup_round_table`). Returns the table.
/// Original: `NirCup::create_round_table` (0x78a870, nir_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn nir_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    build_northern_irish_cup_round_table(Cup(this), stage, rounds, extra, out)
}

/// Side by side like `compare_cup_round_table`, also comparing the three semi-final match-day slots (round 3, 21 bytes
/// from `cup_round::SLOT_VENUE`).
unsafe extern "thiscall" fn compare_nir_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_NIR_CUP_CREATE_ROUND_TABLE);
    if stage as u8 != 0xff {
        let a = o(this, stage, rounds, extra, out);
        SCOREBOARD.record(a == 0, || format!("nir_cup_create_round_table(stage {}) returned {:#x}", stage as u8 as i8, a));
        return a;
    }
    let (mut r2, mut e2) = (*rounds, *extra);
    let mut s2 = if out.is_null() { 0 } else { *out };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, equal, _, _) = crate::comparison::side_by_side(&reg, || o(this, stage, rounds, extra, out),
        || build_northern_irish_cup_round_table(Cup(this), stage, &mut r2, &mut e2, if out.is_null() { core::ptr::null_mut() } else { &mut s2 }));
    let outs = *rounds == r2 && *extra == e2 && (out.is_null() || *out == s2);
    let slots = |t: usize| core::slice::from_raw_parts((t + 3 * cup_round::SIZE + cup_round::SLOT_VENUE) as *const u8, 21).to_vec();
    let table = if a != 0 && b != 0 { eng_cc_cup_written_round_bytes(a, *rounds) == eng_cc_cup_written_round_bytes(b, r2) && slots(a) == slots(b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(equal && outs && table, || format!("nir_cup_create_round_table(stage {}): outs {} ({} x {}) table {}", stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// The body of `nir_cup_collect_entrants`; returns (the routine's return, the entries it writes).
///
/// `cup::TEAM_COUNT` = 32; the entrant table (`exe::crt_malloc` of 0xc0 into `cup::TEAM_TABLE`; out of memory, the
/// error box at line 0x11d and −1). One pass over the clubs (`irish_cup_club_count`): a club of the Northern Irish
/// lower division (`KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION`) goes into a local pool while it holds fewer than
/// 30; otherwise a Premier or First Division club takes the next entry from entry 12 on (club, status 0 and class 1;
/// no bound check: the 20 senior clubs fill entries 12 to 31). Then the pool is sorted and its best shuffled
/// (`exe::comp_util_sort_and_shuffle_top_clubs`(pool, size, 12)) and its first twelve take entries 0 to 11 (status 0,
/// class 2). Returns 1.
pub(crate) unsafe fn collect_northern_irish_cup_entrants(this: Cup) -> (i32, i32) {
    this.set_team_count(0x20);
    let t = exe::crt_malloc(0xc0);
    this.set_team_table(t);
    if t == 0 {
        show_error_at(0xa69628, 0x11d);
        return (-1, 0);
    }
    // simplification: the original's pool is the uninitialised stack; with fewer than twelve lower division clubs the
    // copy would take nulls where the original took leftovers (the career's division has more)
    let mut lower = [0usize; 30];
    let mut n = 0i16;
    let mut senior = 0usize;
    let mut i = 0i32;
    while i < irish_cup_club_count() {
        let c = club_at(i as isize);
        if let Some(id) = irish_club_division_id(c) {
            if n < 0x1e && id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_LOWER_DIVISION) {
                lower[n as usize] = c.addr();
                n += 1;
            } else if id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION) || id == read_usize(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION) {
                senior += 6;
                let e = this.team_table() + senior + 0x42;
                write_usize(e, c.addr());
                write_u8(e + 5, 0);
                write_u8(e + 4, 1); // mwa6: the senior clubs' class
            }
        }
        i += 1;
    }
    covered!("mwa_6 irish cup entrants");
    exe::comp_util_sort_and_shuffle_top_clubs(lower.as_mut_ptr() as usize, n as i32 as u32, 0xc);
    for (k, &club) in lower.iter().take(12).enumerate() { // mwa6: the twelve best of the lower division
        let e = this.team_table() + k * 6;
        write_usize(e, club);
        write_u8(e + 5, 0);
        write_u8(e + 4, 2);
    }
    (1, 12 + (senior / 6) as i32)
}

/// Rule: the (Northern) Irish Cup takes 32 clubs: twelve of the lower division (the best, shuffled among the top 30),
/// entries 0 to 11 with class 2, and the 20 clubs of the Premier and First Divisions, entries 12 to 31 with class 1.
/// Original: `NirCup::collect_entrants` (0x78ac90, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_cup_collect_entrants(this: usize) -> i32 {
    collect_northern_irish_cup_entrants(Cup(this)).0
}

unsafe extern "thiscall" fn compare_nir_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_NIR_CUP_COLLECT_ENTRANTS);
    compare_irish_cup_entrants("nir_cup_collect_entrants", Cup(this), true, || o(this), || collect_northern_irish_cup_entrants(Cup(this)))
}

/// Rule: at the season turn the (Northern) Irish Cup is rebuilt: last edition's tables are freed, the statistics
/// archived, the year goes up by one and the entrants, the rules, the tie tree and the predicted positions are made
/// again.
///
/// The knockout cups' turnover (`start_new_cup_season`, checked instruction by instruction against BraCup's: only the
/// entrant routine, NirCup::collect_entrants 0x78ac90, the file and the error box lines 0x17e, 0x185 and 0x18c change).
/// Original: `NirCup::start_new_season` (0x78ae30, nir_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x78ac90, 0xa69628, [0x17e, 0x185, 0x18c], 1, "mwa_6 irish cup turnover")
}

unsafe extern "thiscall" fn compare_nir_cup_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_CUP_START_NEW_SEASON);
    compare_cup_new_season("nir_cup_start_new_season", Cup(this), || o(this))
}

// ---------- batch 295: the constructors of the Irish Challenge, Leinster and Munster cups (ire_chal_cup.cpp, ire_leinster_cup.cpp, ire_munster_cup.cpp) ----------

/// What changes between the Irish cups' constructors (the Cup template of JapEmpCup and SpaSuper, read instruction by
/// instruction): the vtable, the bench (`cup::SUBSTITUTES_NAMED`, `cup::SUBSTITUTIONS_ALLOWED`; None keeps the base's),
/// the discipline rule, the entrant routine (thiscall, ret 0), whether a step fails on a negative return (otherwise on
/// zero), the prize fund written before the round table (None keeps the base's), whether the final ranking runs, the
/// source file and the three error box lines.
pub(crate) struct IrishCupConstructor {
    pub(crate) vtable: usize,
    pub(crate) bench: Option<(u8, u8)>,
    pub(crate) discipline: u8,
    pub(crate) entrants: usize,
    pub(crate) fails_when_negative: bool,
    pub(crate) prize: Option<i32>,
    pub(crate) rank: bool,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 3],
}

/// The body of the Irish cups' constructors (thiscall, ret 8: this, year, the competition record).
///
/// Over the base Cup::Cup (`exe::new_cup`, thiscall, ret 0): Comp +4 = the record, the vtable, `cup::YEAR` = the year
/// (word), `cup::CURRENT_ROUND` = 0, `cup::STAGE_INDEX` = `stage_index::MAIN` (0xff), `cup::CURRENT_STAGE` = −1,
/// `cup::STAGE_COUNT` = 0, `cup::CLUB_SIDES` = 1, the bench, the discipline rule and `cup::RUN_BY_STAGES` = 0. If the
/// saved state loads (Cup::load_from_disk 0x51fc00: push 1; mov ecx -> thiscall, ret 4; ≠ 0) it returns this. Otherwise,
/// in order: the entrants (the cup's routine; when it fails, the error box at the first line); the prize fund; the
/// round table (`jap_emp_cup_create_rounds`: the cup's own virtual `comp_vtable::CREATE_SCHEDULE` into
/// `cup::ROUND_TABLE`); the tie tree (Cup::map_fixture_tree 0x518790, thiscall, ret 0; failing, the second line); the
/// statistics at Comp +8 (`jap_emp_cup_create_comp_stats`; without them, the third line); the final ranking
/// (Cup::rank_teams 0x5223a0, thiscall, ret 0, unchecked) when `rank`. Each box returns this. Returns this.
pub(crate) unsafe fn construct_irish_cup(this: Cup, year: u32, record: usize, k: &IrishCupConstructor, branch: &'static str) -> usize {
    exe::new_cup(this);
    write_usize(this.addr() + 4, record); // the competition record (Comp +4)
    write_usize(this.addr(), k.vtable);
    write_u16(this.addr() + cup::YEAR, year as u16);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    this.set_stage_count(0);
    this.set_club_sides(1);
    if let Some((named, allowed)) = k.bench {
        this.set_substitutes_named(named);
        this.set_substitutions_allowed(allowed);
    }
    this.set_discipline_index(k.discipline);
    this.set_run_by_stages(0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize); // Cup::load_from_disk
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    let failed = |r: i32| if k.fails_when_negative { r < 0 } else { r == 0 };
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(k.entrants);
    if failed(entrants(this.addr())) {
        show_error_at(k.file, k.rows[0]);
        return this.addr();
    }
    if let Some(p) = k.prize {
        this.set_prize_fund(p);
    }
    jap_emp_cup_create_rounds(this);
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize); // Cup::map_fixture_tree
    if failed(map_fixture_tree(this.addr())) {
        show_error_at(k.file, k.rows[1]);
        return this.addr();
    }
    if !jap_emp_cup_create_comp_stats(this) {
        show_error_at(k.file, k.rows[2]);
        return this.addr();
    }
    if k.rank {
        let rank_teams: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize); // Cup::rank_teams
        rank_teams(this.addr());
    }
    this.addr()
}

/// The comparer of the Irish cups' constructors: only the original runs (it allocates, draws and reads the save file),
/// and the plan is checked (`jap_emp_cup_check_constructor_plan`): this, the vtable, Comp +4, the year and, on a new
/// cup, the fixed fields the constructor writes, the round table and the statistics.
pub(crate) unsafe fn compare_irish_cup_constructor(name: &str, this: usize, year: u32, record: usize, k: &IrishCupConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut fields = vec![(cup::STAGE_COUNT, 4u8, 0u32), (cup::CURRENT_STAGE, 4, 0xffff_ffff), (cup::CLUB_SIDES, 1, 1), (cup::STAGE_INDEX, 1, 0xff),
        (cup::DISCIPLINE_INDEX, 1, k.discipline as u32), (cup::CURRENT_ROUND, 1, 0), (cup::RUN_BY_STAGES, 1, 0)];
    if let Some((named, allowed)) = k.bench {
        fields.push((cup::SUBSTITUTES_NAMED, 1, named as u32));
        fields.push((cup::SUBSTITUTIONS_ALLOWED, 1, allowed as u32));
    }
    if let Some(p) = k.prize {
        fields.push((cup::PRIZE_FUND, 4, p as u32));
    }
    let error = jap_emp_cup_check_constructor_plan(Cup(this), r, year, record, k.vtable, &fields, true);
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}, {}, {:#x}): {}", name, this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The FAI Senior Challenge Cup's constructor: the vtable 0x96bc2c, 5 substitutes on the bench and 3 substitutions,
/// discipline rule `discipline_rule::IRELAND`, the entrants IreChalCup::collect_entrants (0x6347d0; fails on 0), the
/// 100,000 prize fund, the final ranking; the error box lines 0x43, 0x51 and 0x58 of ire_chal_cup.cpp.
pub(crate) const IRISH_CHALLENGE_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bc2c, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x6347d0, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9e94, rows: [0x43, 0x51, 0x58] };

/// Rule: the FAI Senior Challenge Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 42 entrants,
/// the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `IRISH_CHALLENGE_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup 0x661d30: only the vtable, the discipline rule, the entrant routine, the prize, the file and the
/// lines change).
/// Original: `IreChalCup::IreChalCup` (0x6340a0, ire_chal_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_chal_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, "mwa_7 irish challenge cup new") // mwa7: the Irish Challenge Cup
}

unsafe extern "thiscall" fn compare_construct_ire_chal_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_CHAL_CUP);
    compare_irish_cup_constructor("construct_ire_chal_cup", this, year, record, &IRISH_CHALLENGE_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Leinster Senior Cup's constructor: the vtable 0x96bd80, the same as the Challenge Cup's otherwise (5 and 3,
/// Irish rules, 100,000, the final ranking), with IreLeinsterCup::select_clubs (0x636a80) and the lines 0x44, 0x52 and
/// 0x59 of ire_leinster_cup.cpp.
pub(crate) const LEINSTER_SENIOR_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bd80, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x636a80, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9f0c, rows: [0x44, 0x52, 0x59] };

/// Rule: the Leinster Senior Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 16 clubs, the
/// rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `LEINSTER_SENIOR_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup).
/// Original: `IreLeinsterCup::IreLeinsterCup` (0x6364a0, ire_leinster_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_leinster_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 leinster cup new") // mwa7: the Leinster cup
}

unsafe extern "thiscall" fn compare_construct_ire_leinster_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_LEINSTER_CUP);
    compare_irish_cup_constructor("construct_ire_leinster_cup", this, year, record, &LEINSTER_SENIOR_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Munster Senior Cup's constructor: the vtable 0x96bed4, the same as the Challenge Cup's otherwise, with
/// IreMunsterCup::collect_entrants (0x638e30) and the lines 0x43, 0x51 and 0x58 of ire_munster_cup.cpp.
pub(crate) const MUNSTER_SENIOR_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bed4, bench: Some((5, 3)), discipline: discipline_rule::IRELAND,
    entrants: 0x638e30, fails_when_negative: false, prize: Some(0x186a0), rank: true, file: 0x9e9f84, rows: [0x43, 0x51, 0x58] };

/// Rule: the Munster Senior Cup is a cup of one nation's clubs of the game's year, with a 100,000 prize fund, 5
/// substitutes on the bench and 3 substitutions, under the Irish discipline rules; with nothing saved, its 20 entrants,
/// the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `MUNSTER_SENIOR_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// JapEmpCup::JapEmpCup).
/// Original: `IreMunsterCup::IreMunsterCup` (0x6387a0, ire_munster_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_munster_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, "mwa_7 munster cup new") // mwa7: the Munster cup
}

unsafe extern "thiscall" fn compare_construct_ire_munster_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_MUNSTER_CUP);
    compare_irish_cup_constructor("construct_ire_munster_cup", this, year, record, &MUNSTER_SENIOR_CUP_CONSTRUCTOR, || o(this, year, record))
}

// ---------- batch 295: the constructors of the Irish President's Cup, the Northern Irish Charity Shield and the Northern Irish Cup (ire_pres_cup.cpp, nir_charity.cpp, nir_cup.cpp) ----------

/// The President's Cup's constructor: the vtable 0x96bf74, the bench of the base, discipline rule
/// `discipline_rule::IRELAND`, the entrants IrePresCup::collect_entrants (0x639670; fails when negative), no prize
/// fund, no final ranking; the error box lines 0x3f, 0x4d and 0x54 of ire_pres_cup.cpp.
pub(crate) const IRISH_PRESIDENTS_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96bf74, bench: None, discipline: discipline_rule::IRELAND,
    entrants: 0x639670, fails_when_negative: true, prize: None, rank: false, file: 0x9ea000, rows: [0x3f, 0x4d, 0x54] };

/// Rule: the President's Cup is a cup of one nation's clubs of the game's year, with no prize fund of its own and the
/// base's bench, under the Irish discipline rules; with nothing saved, its two clubs, the round, the tie tree and the
/// statistics are made (no final ranking).
///
/// `construct_irish_cup` with `IRISH_PRESIDENTS_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// FraSuper::FraSuper 0x5bf320: only the vtable, the discipline rule, the entrant routine and the file change).
/// Original: `IrePresCup::IrePresCup` (0x639280, ire_pres_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_pres_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, "mwa_8 presidents cup new") // mwa8: the President's Cup
}

unsafe extern "thiscall" fn compare_construct_ire_pres_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_PRES_CUP);
    compare_irish_cup_constructor("construct_ire_pres_cup", this, year, record, &IRISH_PRESIDENTS_CUP_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Charity Shield's constructor: the vtable 0x96da24, 3 substitutes on the bench and 3
/// substitutions, discipline rule `discipline_rule::NORTHERN_IRELAND`, the clubs NirCharity::select_teams (0x78a1e0;
/// fails when negative), no prize fund, no final ranking; the lines 0x43, 0x51 and 0x58 of nir_charity.cpp.
pub(crate) const NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96da24, bench: Some((3, 3)), discipline: discipline_rule::NORTHERN_IRELAND,
    entrants: 0x78a1e0, fails_when_negative: true, prize: None, rank: false, file: 0xa695f0, rows: [0x43, 0x51, 0x58] };

/// Rule: the Northern Irish Charity Shield is a cup of one nation's clubs of the game's year, with no prize fund of its
/// own, 3 substitutes on the bench and 3 substitutions, under the Northern Irish discipline rules; with nothing saved,
/// its two clubs, the round, the tie tree and the statistics are made (no final ranking).
///
/// `construct_irish_cup` with `NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR` (checked instruction by instruction against
/// SpaSuper::SpaSuper 0x8586b0: the same steps with the year and the record written in another order).
/// Original: `NirCharity::NirCharity` (0x789df0, nir_charity.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_charity(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, "mwa_8 northern irish charity shield new") // mwa8: the Charity Shield
}

unsafe extern "thiscall" fn compare_construct_nir_charity(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_CHARITY);
    compare_irish_cup_constructor("construct_nir_charity", this, year, record, &NORTHERN_IRISH_CHARITY_SHIELD_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Cup's constructor: the vtable 0x96dac4, 3 substitutes on the bench and 3 substitutions,
/// discipline rule `discipline_rule::NORTHERN_IRELAND`, the entrants NirCup::collect_entrants (0x78ac90; fails on 0),
/// a prize fund of 0, the final ranking; the lines 0x43, 0x51 and 0x58 of nir_cup.cpp.
pub(crate) const NORTHERN_IRISH_CUP_CONSTRUCTOR: IrishCupConstructor = IrishCupConstructor { vtable: 0x96dac4, bench: Some((3, 3)), discipline: discipline_rule::NORTHERN_IRELAND,
    entrants: 0x78ac90, fails_when_negative: false, prize: Some(0), rank: true, file: 0xa69628, rows: [0x43, 0x51, 0x58] };

/// Rule: the Northern Irish Cup is a cup of one nation's clubs of the game's year, with a prize fund of 0 (the rounds
/// pay fixed prizes), 3 substitutes on the bench and 3 substitutions, under the Northern Irish discipline rules; with
/// nothing saved, its 32 entrants, the rounds, the tie tree, the statistics and the final ranking are made.
///
/// `construct_irish_cup` with `NORTHERN_IRISH_CUP_CONSTRUCTOR` (checked instruction by instruction against
/// FraLgeCup::FraLgeCup 0x5bc980 and JapEmpCup::JapEmpCup).
/// Original: `NirCup::NirCup` (0x78a5b0, nir_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_cup(this: usize, year: u32, record: usize) -> usize {
    construct_irish_cup(Cup(this), year, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, "mwa_8 irish cup new") // mwa8: the Irish Cup
}

unsafe extern "thiscall" fn compare_construct_nir_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_CUP);
    compare_irish_cup_constructor("construct_nir_cup", this, year, record, &NORTHERN_IRISH_CUP_CONSTRUCTOR, || o(this, year, record))
}

// ---------- batch 295: the Northern Irish Premier Division: the constructor, the rules and the table (nir_prm.cpp) ----------

/// What changes between the Irish leagues' constructors (the League template of EngFirst and JapJ2, read instruction by
/// instruction): the vtable, the discipline rule, whether there is one extra stage (the promotion play-off: the stage
/// count 1 and a 4-byte stage array, unchecked; otherwise 0 and none), whether the IrePrm block from +0xee is reset,
/// the rules and table routines (thiscall, ret 0; 0 = failure), the source file and the five error box lines (rules,
/// table, fixtures, order, statistics).
pub(crate) struct IrishLeagueConstructor {
    pub(crate) vtable: usize,
    pub(crate) discipline: u8,
    pub(crate) play_off_stage: bool,
    pub(crate) reset_ten_slots: bool,
    pub(crate) rules: usize,
    pub(crate) table: usize,
    pub(crate) file: usize,
    pub(crate) rows: [u32; 5],
}

/// The body of the Irish leagues' constructors (thiscall, ret 8: this, year, the competition record).
///
/// League::League (`exe::new_league`, thiscall, ret 0); `league::COMPETITION` = the record; the vtable; `league::YEAR`
/// (word); `league::DISCIPLINE_RULE`; in the Premier Division, the ten 5-byte entries from +0xee get −1 in their first
/// dword (its own state [inference: the last seasons' places]). If the saved state loads (League::load_from_temp_file
/// `exe::league_load_from_temp_file`(this, 1), thiscall, ret 4; ≠ 0) it returns this. Otherwise `league::STAGE_INDEX` =
/// `stage_index::MAIN`, `league::CURRENT_STAGE` = −1, the stage count and array; then, each with its own error box
/// (returning this) when it returns 0: the rules, the table, the fixtures (League::create_fixtures
/// `exe::league_create_fixtures`), the order (League::sort_table `exe::league_sort_table`(this, 0)); the statistics
/// (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`) at
/// `league::COMP_STATS` (without them, the box); finally the predicted positions (League::set_predicted_positions
/// `exe::league_set_predicted_positions`). Returns this.
pub(crate) unsafe fn construct_irish_league(this: League, year: u32, record: usize, k: &IrishLeagueConstructor, branch: &'static str) -> usize {
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, record);
    write_usize(this.addr(), k.vtable);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(k.discipline);
    if k.reset_ten_slots {
        for s in 0..10usize {
            write_i32(this.addr() + 0xee + s * 5, -1);
        }
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_stage_index(stage_index::MAIN);
    write_i32(this.addr() + league::CURRENT_STAGE, -1);
    if k.play_off_stage {
        this.set_stage_count(1);
        this.set_stages(exe::crt_malloc(4));
    } else {
        this.set_stage_count(0);
        this.set_stages(0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.rules);
    if rules(this.addr()) == 0 {
        show_error_at(k.file, k.rows[0]);
        return this.addr();
    }
    let table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(k.table);
    if table(this.addr()) == 0 {
        show_error_at(k.file, k.rows[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(k.file, k.rows[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, 0usize) == 0 {
        show_error_at(k.file, k.rows[3]);
        return this.addr();
    }
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(k.file, k.rows[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

/// The comparer of the Irish leagues' constructors: only the original runs (it allocates, draws the fixtures and reads
/// the save file); the plan: the return (this), the vtable, the year, the record, the discipline rule and, on a new
/// league (`stage_index::MAIN`), the current stage −1, the stage count, the stage array (none without stages) and the
/// statistics.
pub(crate) unsafe fn compare_irish_league_constructor(name: &str, this: League, year: u32, record: usize, k: &IrishLeagueConstructor, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == k.vtable && this.year() as u16 == year as u16
        && read_usize(this.addr() + league::COMPETITION) == record && this.discipline_rule() == k.discipline;
    if ok && this.stage_index() == stage_index::MAIN {
        ok = read_i32(this.addr() + league::CURRENT_STAGE) == -1 && read_u32(this.addr() + league::STAGE_COUNT) == k.play_off_stage as u32
            && (this.stages() != 0) == k.play_off_stage && read_usize(this.addr() + league::COMP_STATS) != 0;
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}, {}, {:#x}) returned {:#x}, vtable {:#x}, year {}, stages {} {:#x}, statistics {:#x}", name, this.addr(), year as u16, record, r,
        read_usize(this.addr()), this.year(), read_u32(this.addr() + league::STAGE_COUNT), this.stages(), read_usize(this.addr() + league::COMP_STATS)));
    r
}

/// The rule fields of an Irish league's set_up_rules, written in the original's order (offset, width, value), the
/// division above and below (the key global of its id; None = −1).
pub(crate) struct IrishLeagueRules {
    pub(crate) writes: &'static [(usize, u8, u32)],
    pub(crate) upper: Option<usize>,
    pub(crate) lower: Option<usize>,
}

/// The fields `set_up_rules` writes before the round table: `writes`, then `league::UPPER_DIVISION_ID` and
/// `league::LOWER_DIVISION_ID` (the id of the key global, or −1).
pub(crate) unsafe fn write_irish_league_rules(this: League, k: &IrishLeagueRules) {
    for &(o, n, x) in k.writes {
        match n {
            1 => write_u8(this.addr() + o, x as u8),
            2 => write_u16(this.addr() + o, x as u16),
            _ => write_u32(this.addr() + o, x),
        }
    }
    write_u32(this.addr() + league::UPPER_DIVISION_ID, k.upper.map_or(0xffff_ffff, |g| read_u32(g)));
    write_u32(this.addr() + league::LOWER_DIVISION_ID, k.lower.map_or(0xffff_ffff, |g| read_u32(g)));
}

/// The body of the Irish leagues' set_up_rules (thiscall, ret 0): the fields (`write_irish_league_rules`) and the round
/// table of the main stage (the league's own virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`,
/// &`league::STAGE_ID`, null), thiscall, ret 0x10) into `league::ROUND_DATES`. Returns 1.
pub(crate) unsafe fn set_up_irish_league_rules(this: League, k: &IrishLeagueRules) -> u32 {
    write_irish_league_rules(this, k);
    let schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = schedule(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    write_usize(this.addr() + league::ROUND_DATES, t);
    1
}

/// The comparer of the Irish leagues' set_up_rules: side by side over the league's fields outside the round table
/// (`league_regions_without_schedule`); the copy writes only the fields (the original's round table stays).
pub(crate) unsafe fn compare_irish_league_rules(name: &str, this: usize, k: &IrishLeagueRules, original: impl FnOnce() -> u32) -> u32 {
    let reg = league_regions_without_schedule(this);
    let (r, n, equal, mo, mr) = crate::comparison::side_by_side(&reg, original, || {
        write_irish_league_rules(League(this), k);
        1
    });
    SCOREBOARD.record(equal && r == n, || format!("{}({:#x}) original={} rust={} {:?} {:?}", name, this, r, n, mo, mr));
    r
}

/// The Northern Irish Premier Division's constructor: the vtable 0x96dccc, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, no extra stage, NirPrm::set_up_rules (0x78ed80) and NirPrm::create_table
/// (0x78ec60), the lines 0x45, 0x4c, 0x53, 0x5c and 0x63 of nir_prm.cpp.
pub(crate) const NORTHERN_IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96dccc, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: false, reset_ten_slots: false, rules: 0x78ed80, table: 0x78ec60, file: 0xa696d0, rows: [0x45, 0x4c, 0x53, 0x5c, 0x63] };

/// Rule: the Northern Irish Premier Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, a plain league (no extra stage): the rules, the table, the fixtures,
/// the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against
/// JapJ2::JapJ2 0x6644c0: only the vtable, the discipline rule, the two routines, the file and the lines change).
/// Original: `NirPrm::NirPrm` (0x78e070, nir_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, "mwa_9 northern irish premier new") // mwa9: the Northern Irish Premier
}

unsafe extern "thiscall" fn compare_construct_nir_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_PRM);
    compare_irish_league_constructor("construct_nir_prm", League(this), year, record, &NORTHERN_IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish Premier Division's rules: four meetings (`league::LEGS` = 4), 3 points for a win and 1 for a
/// draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides, the tie-breaks 1, 2
/// (goal difference, goals for) and 4, no promotion, one club to the relegation play-off and one relegated, the squad
/// flags 0, 3 substitutes on the bench and 3 substitutions, `league::LEAGUE_FLAGS` 3 (champion and promotion status
/// bits [inference]); no division above, the First Division below.
pub(crate) const NORTHERN_IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 4),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa9: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::LEAGUE_FLAGS, 2, 3)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION),
};

/// Rule: the Northern Irish Premier Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties
/// split by goal difference and goals for; the last goes down to the First Division and the second last plays the
/// promotion/relegation play-off; 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_PREMIER_RULES` (the fields, then the round table of the main stage
/// into `league::ROUND_DATES`). Returns 1.
/// Original: `NirPrm::set_up_rules` (0x78ed80, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_9 northern irish premier rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_PREMIER_RULES)
}

unsafe extern "thiscall" fn compare_nir_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_SET_UP_RULES);
    compare_irish_league_rules("nir_prm_set_up_rules", this, &NORTHERN_IRISH_PREMIER_RULES, || o(this))
}

/// Rule: the Northern Irish Premier Division table has ten clubs, the Premier Division clubs in database order; when
/// full, the prize fund for a 500 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`; out of memory, the error
/// box at line 0x15a of nir_prm.cpp) (ItaSerA::create_table's template, checked instruction by instruction: the end
/// without the TV money); not full, 0; full, the prize fund (League::calc_prize_fund `exe::league_calc_prize_fund`(this,
/// 500), thiscall, ret 4) in `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirPrm::create_table` (0x78ec60, nir_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_prm_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa696d0, 0x15a, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_9 northern irish premier table");
    set_prize_fund(League(this), 0x1f4); // mwa9: the Northern Irish Premier quota
    1
}

unsafe extern "thiscall" fn compare_nir_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_PRM_CREATE_TABLE);
    ita_ser_a_compare_league_table("nir_prm_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, Some(0x1f4), || o(this))
}

// ---------- batch 295: the Northern Irish First Division: the constructor, the rules and the table (nir_first.cpp) ----------

/// The Northern Irish First Division's constructor: the vtable 0x96db64, discipline rule
/// `discipline_rule::NORTHERN_IRELAND`, one extra stage (the promotion play-off), NirFirst::set_up_rules (0x78bd70) and
/// NirFirst::create_table (0x78bc50), the lines 0x49, 0x50, 0x57, 0x60 and 0x67 of nir_first.cpp.
pub(crate) const NORTHERN_IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96db64, discipline: discipline_rule::NORTHERN_IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x78bd70, table: 0x78bc50, file: 0xa6965c, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// Rule: the Northern Irish First Division at career creation: the year and the database competition, the Northern
/// Irish discipline rules and, with nothing saved, the league and one extra stage (the promotion play-off): the rules,
/// the table, the fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `NORTHERN_IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `NirFirst::NirFirst` (0x78b010, nir_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, "mwa_10 northern irish first new") // mwa10: the Northern Irish First
}

unsafe extern "thiscall" fn compare_construct_nir_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_NIR_FIRST);
    compare_irish_league_constructor("construct_nir_first", League(this), year, record, &NORTHERN_IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// The Northern Irish First Division's rules: four meetings, the tie-breaks 1, 2 and 4, 3 points for a win and 1 for a
/// draw, match category 2, club sides, one promoted directly and one to the promotion play-off, no relegation, 3
/// substitutes on the bench and 3 substitutions, the squad flags 2; the Premier Division above, none below.
pub(crate) const NORTHERN_IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::TIEBREAKS + 2, 1, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2),
        (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa10: one up, one to the play-off
        (league::SUBSTITUTES_NAMED, 1, 3), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SQUAD_FLAGS, 1, 2)],
    upper: Some(KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION),
    lower: None,
};

/// Rule: the Northern Irish First Division: ten clubs meeting four times, 3 points for a win and 1 for a draw, ties split
/// by goal difference and goals for; the champion goes up to the Premier Division and the runner-up plays the promotion
/// play-off; no relegation (no division below); 3 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `NORTHERN_IRISH_FIRST_RULES`. Returns 1.
/// Original: `NirFirst::set_up_rules` (0x78bd70, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_10 northern irish first rules");
    set_up_irish_league_rules(League(this), &NORTHERN_IRISH_FIRST_RULES)
}

unsafe extern "thiscall" fn compare_nir_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_SET_UP_RULES);
    compare_irish_league_rules("nir_first_set_up_rules", this, &NORTHERN_IRISH_FIRST_RULES, || o(this))
}

/// Rule: the Northern Irish First Division table has ten clubs, the division's in database order; when full, the prize
/// fund for a 300 share per place, paid by table position at the season's end.
///
/// `ita_ser_a_fill_league_table`(10 clubs, `KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION`; out of memory, the error box
/// at line 0x190 of nir_first.cpp); not full, 0; full, the prize fund (`exe::league_calc_prize_fund`(this, 300)) in
/// `league::PRIZE_FUND`, `league::PRIZE_MODE` = 1 and 1.
/// Original: `NirFirst::create_table` (0x78bc50, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_table(this: usize) -> u32 {
    if ita_ser_a_fill_league_table(League(this), 0xa, 0xa6965c, 0x190, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION) != Some(true) {
        return 0;
    }
    covered!("mwa_10 northern irish first table");
    set_prize_fund(League(this), 0x12c); // mwa10: the Northern Irish First quota
    1
}

unsafe extern "thiscall" fn compare_nir_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_CREATE_TABLE);
    ita_ser_a_compare_league_table("nir_first_create_table", League(this), 0xa, KEY_CLUB_COMP_NORTHERN_IRELAND_FIRST_DIVISION, Some(0x12c), || o(this))
}

// ---------- batch 295: the Irish Premier Division: the constructor, the rules and the table (ire_prm.cpp) ----------

/// The Irish Premier Division's constructor: the vtable 0x96c014, discipline rule `discipline_rule::IRELAND`, no extra
/// stage, the ten slots from +0xee reset, IrePrm::set_up_rules (0x63a900) and IrePrm::create_table (0x63a740), the
/// lines 0x4b, 0x52, 0x59, 0x62 and 0x69 of ire_prm.cpp.
pub(crate) const IRISH_PREMIER_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96c014, discipline: discipline_rule::IRELAND,
    play_off_stage: false, reset_ten_slots: true, rules: 0x63a900, table: 0x63a740, file: 0x9ea080, rows: [0x4b, 0x52, 0x59, 0x62, 0x69] };

/// Rule: the Irish Premier Division at career creation: the year and the database competition, the Irish discipline
/// rules, its ten own slots cleared and, with nothing saved, a plain league: the rules, the table, the fixtures, the
/// order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_PREMIER_CONSTRUCTOR` (checked instruction by instruction against JapJ2::JapJ2:
/// the same steps with the ten 5-byte slots from +0xee set to −1 before loading).
/// Original: `IrePrm::IrePrm` (0x639bb0, ire_prm.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, "mwa_11 irish premier new") // mwa11: the Irish Premier
}

unsafe extern "thiscall" fn compare_construct_ire_prm(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_PRM);
    compare_irish_league_constructor("construct_ire_prm", League(this), year, record, &IRISH_PREMIER_CONSTRUCTOR, || o(this, year, record))
}

/// The Irish Premier Division's rules: three meetings (`league::LEGS` = 3; the table sets 4 with ten clubs), 3 points
/// for a win and 1 for a draw, match category 2, `league::MATCH_RULES` and `league::ROUND_DRAW_RULES` 0, club sides,
/// the tie-breaks 1 and 2 (goal difference, goals for), no promotion, one club to the relegation play-off and one
/// relegated (the table sets 1 and 3 with twelve clubs), the squad flags 0, 5 substitutes on the bench and 3
/// substitutions, `league::LEAGUE_FLAGS` 0x103; no division above, the First Division below.
pub(crate) const IRISH_PREMIER_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 3), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::MATCH_RULES, 2, 0),
        (league::ROUND_DRAW_RULES, 2, 0), (league::CLUB_SIDES, 1, 1), (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 0), (league::PROMOTION_PLAYOFF_COUNT, 1, 0), (league::RELEGATION_PLAYOFF_COUNT, 1, 1), (league::RELEGATED_COUNT, 1, 1), // mwa11: one relegated, one in the play-off
        (league::SQUAD_FLAGS, 1, 0), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5), (league::LEAGUE_FLAGS, 2, 0x103)],
    upper: None,
    lower: Some(KEY_CLUB_COMP_IRISH_FIRST_DIVISION),
};

/// Rule: the Irish Premier Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for;
/// the last goes down to the First Division and the second last plays the play-off (with twelve clubs, three go down
/// and one plays the play-off); 5 substitutes on the bench and 3 substitutions; no division above.
///
/// `set_up_irish_league_rules` with `IRISH_PREMIER_RULES`. Returns 1.
/// Original: `IrePrm::set_up_rules` (0x63a900, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_set_up_rules(this: usize) -> u32 {
    covered!("mwa_11 irish premier rules");
    set_up_irish_league_rules(League(this), &IRISH_PREMIER_RULES)
}

unsafe extern "thiscall" fn compare_ire_prm_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_SET_UP_RULES);
    compare_irish_league_rules("ire_prm_set_up_rules", this, &IRISH_PREMIER_RULES, || o(this))
}

/// How many clubs of the division whose id is in the key global `key` (all `G_CLUB_COUNT` clubs, a 16-bit counter as
/// in the original).
pub(crate) unsafe fn irish_division_club_count(key: usize) -> i16 {
    let mut n = 0i16;
    let mut i = 0i16;
    while (i as i32) < read_i32(G_CLUB_COUNT) {
        let d = club_at(i as isize).division();
        if !d.is_null() && read_usize(d.addr()) == read_usize(key) {
            n = n.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    n
}

/// The body of `ire_prm_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only ten or twelve are accepted (otherwise 0): ten meet four times (`league::LEGS` = 4), twelve
/// three times. The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x186 of
/// ire_prm.cpp and 0) with the clubs in database order; with twelve rows filled, one club to the relegation play-off and
/// three relegated; full, the prize fund for a 30,000 share per place (`set_prize_fund`) and 1; otherwise 0.
pub(crate) unsafe fn create_irish_premier_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION);
    this.set_club_count(n);
    match n {
        0xa => this.set_legs(4),
        0xc => this.set_legs(3),
        _ => return 0,
    }
    let full = match ita_ser_a_fill_league_table(this, n, 0x9ea080, 0x186, KEY_CLUB_COMP_IRISH_PREMIER_DIVISION) {
        Some(f) => f,
        None => return 0,
    };
    if full && n == 0xc {
        covered!("mwa_11 irish premier of twelve");
        this.set_relegation_playoff_count(1);
        this.set_relegated_count(3); // mwa11: three go down from a twelve-club Premier
    }
    if !full {
        return 0;
    }
    covered!("mwa_11 irish premier table");
    set_prize_fund(this, 0x7530); // mwa11: the Irish Premier quota
    1
}

/// Rule: the Irish Premier Division table takes the division's clubs in database order; only ten (meeting four times)
/// or twelve (three times, with three relegated and one in the play-off) are accepted; when full, the prize fund for a
/// 30,000 share per place, paid by table position at the season's end.
/// Original: `IrePrm::create_table` (0x63a740, ire_prm.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_prm_create_table(this: usize) -> u32 {
    create_irish_premier_table(League(this))
}

/// The comparer of the Irish tables (they allocate the table: only the original runs): the plan is the count of the
/// division's clubs; out of the accepted counts the routine must return 0; otherwise the rows, the return and the
/// prize (`ita_ser_a_compare_league_table`) and the meetings `legs`.
pub(crate) unsafe fn compare_irish_counted_table(name: &str, this: League, key: usize, legs: impl Fn(i16) -> Option<i16>, quota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let n = irish_division_club_count(key);
    match legs(n) {
        None => {
            let r = original();
            SCOREBOARD.record(r == 0, || format!("{}({:#x}) with {} clubs returned {}", name, this.addr(), n, r));
            r
        }
        Some(l) => {
            let r = ita_ser_a_compare_league_table(name, this, n, key, quota, original);
            SCOREBOARD.record(this.legs() == l, || format!("{}({:#x}) with {} clubs: legs {} (plan {})", name, this.addr(), n, this.legs(), l));
            r
        }
    }
}

unsafe extern "thiscall" fn compare_ire_prm_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_PRM_CREATE_TABLE);
    compare_irish_counted_table("ire_prm_create_table", League(this), KEY_CLUB_COMP_IRISH_PREMIER_DIVISION, |n| match n { 0xa => Some(4), 0xc => Some(3), _ => None }, Some(0x7530), || o(this))
}

// ---------- batch 295: the Irish First Division: the constructor, the rules and the table (ire_first.cpp) ----------

/// The Irish First Division's constructor: the vtable 0x96bccc, discipline rule `discipline_rule::IRELAND`, one extra
/// stage (the promotion play-off), IreFirst::set_up_rules (0x635aa0) and IreFirst::create_table (0x635910), the lines
/// 0x49, 0x50, 0x57, 0x60 and 0x67 of ire_first.cpp.
pub(crate) const IRISH_FIRST_CONSTRUCTOR: IrishLeagueConstructor = IrishLeagueConstructor { vtable: 0x96bccc, discipline: discipline_rule::IRELAND,
    play_off_stage: true, reset_ten_slots: false, rules: 0x635aa0, table: 0x635910, file: 0x9e9ed0, rows: [0x49, 0x50, 0x57, 0x60, 0x67] };

/// Rule: the Irish First Division at career creation: the year and the database competition, the Irish discipline rules
/// and, with nothing saved, the league and one extra stage (the promotion play-off): the rules, the table, the
/// fixtures, the order, the statistics and the predicted positions.
///
/// `construct_irish_league` with `IRISH_FIRST_CONSTRUCTOR` (checked instruction by instruction against
/// EngFirst::EngFirst 0x571680: only the vtable, the discipline rule, the two routines, the file and the last line).
/// Original: `IreFirst::IreFirst` (0x634c40, ire_first.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    construct_irish_league(League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, "mwa_12 irish first new") // mwa12: the Irish First
}

unsafe extern "thiscall" fn compare_construct_ire_first(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_IRE_FIRST);
    compare_irish_league_constructor("construct_ire_first", League(this), year, record, &IRISH_FIRST_CONSTRUCTOR, || o(this, year, record))
}

/// The Irish First Division's rules: four meetings (the table sets 3 with eleven or twelve clubs), 3 points for a win
/// and 1 for a draw, match category 2, club sides, the tie-breaks 1 and 2, one promoted directly and one to the
/// promotion play-off, no relegation, the squad flags 2, 5 substitutes on the bench and 3 substitutions; the Premier
/// Division above, none below.
pub(crate) const IRISH_FIRST_RULES: IrishLeagueRules = IrishLeagueRules {
    writes: &[(league::LEGS, 2, 4), (league::WIN_POINTS, 1, 3), (league::DRAW_POINTS, 1, 1), (league::MATCH_CATEGORY, 1, 2), (league::CLUB_SIDES, 1, 1),
        (league::TIEBREAKS, 1, 1), (league::TIEBREAKS + 1, 1, 2), (league::TIEBREAKS + 2, 1, 0),
        (league::PROMOTED_COUNT, 1, 1), (league::PROMOTION_PLAYOFF_COUNT, 1, 1), (league::RELEGATION_PLAYOFF_COUNT, 1, 0), (league::RELEGATED_COUNT, 1, 0), // mwa12: one up, one to the play-off
        (league::SQUAD_FLAGS, 1, 2), (league::SUBSTITUTIONS_ALLOWED, 1, 3), (league::SUBSTITUTES_NAMED, 1, 5)],
    upper: Some(KEY_CLUB_COMP_IRISH_PREMIER_DIVISION),
    lower: None,
};

/// Rule: the Irish First Division: 3 points for a win and 1 for a draw, ties split by goal difference and goals for; the
/// champion goes up to the Premier Division and the runner-up plays the promotion play-off; no relegation (no division
/// below); 5 substitutes on the bench and 3 substitutions.
///
/// `set_up_irish_league_rules` with `IRISH_FIRST_RULES`. Returns 1.
/// Original: `IreFirst::set_up_rules` (0x635aa0, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_set_up_rules(this: usize) -> u32 {
    covered!("mwa_12 irish first rules");
    set_up_irish_league_rules(League(this), &IRISH_FIRST_RULES)
}

unsafe extern "thiscall" fn compare_ire_first_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_FIRST_SET_UP_RULES);
    compare_irish_league_rules("ire_first_set_up_rules", this, &IRISH_FIRST_RULES, || o(this))
}

/// The meetings of an Irish First Division of `n` clubs: nine or ten meet four times, eleven or twelve three times;
/// other counts are refused.
pub(crate) fn irish_first_meetings(n: i16) -> Option<i16> {
    match n {
        9 | 10 => Some(4), // mwa12: nine or ten clubs meet four times
        11 | 12 => Some(3),
        _ => None,
    }
}

/// The body of `ire_first_create_table`.
///
/// `league::CLUB_COUNT` = 0 and `league::GAMES_PER_CLUB` = −1; the division's clubs are counted into
/// `league::CLUB_COUNT`. Only nine to twelve are accepted (otherwise 0), with the meetings of `irish_first_meetings`.
/// The table (`ita_ser_a_fill_league_table`: n × 0x3b bytes; out of memory, the error box at line 0x1b6 of
/// ire_first.cpp and 0) with the clubs in database order; returns whether it filled up (no prize fund).
pub(crate) unsafe fn create_irish_first_table(this: League) -> u32 {
    this.set_club_count(0);
    this.set_games_per_club(-1);
    let n = irish_division_club_count(KEY_CLUB_COMP_IRISH_FIRST_DIVISION);
    this.set_club_count(n);
    let legs = match irish_first_meetings(n) {
        Some(l) => l,
        None => return 0,
    };
    this.set_legs(legs);
    covered!("mwa_12 irish first table");
    (ita_ser_a_fill_league_table(this, n, 0x9e9ed0, 0x1b6, KEY_CLUB_COMP_IRISH_FIRST_DIVISION) == Some(true)) as u32
}

/// Rule: the Irish First Division table takes the division's clubs in database order; only nine to twelve are
/// accepted: nine or ten meet four times, eleven or twelve three times; no prize fund.
/// Original: `IreFirst::create_table` (0x635910, ire_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn ire_first_create_table(this: usize) -> u32 {
    create_irish_first_table(League(this))
}

unsafe extern "thiscall" fn compare_ire_first_create_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_IRE_FIRST_CREATE_TABLE);
    compare_irish_counted_table("ire_first_create_table", League(this), KEY_CLUB_COMP_IRISH_FIRST_DIVISION, irish_first_meetings, None, || o(this))
}

// ---------- batch 295: the Northern Irish First Division: the season turnover, the next stage and the promotion play-off (nir_first.cpp) ----------

/// Rule: the Northern Irish First Division turnover: the statistics archived, the table and the round table cleared,
/// the year moved on and the rules, the table, the fixtures and the order rebuilt; then the predicted positions, the
/// board expectations and the referee renewal.
///
/// BraNatSecond::start_new_season's template (checked instruction by instruction: only the table routine,
/// NirFirst::create_table 0x78bc50, the file and the error box lines 0x1f5, 0x1fc, 0x202 and 0x209 change):
/// `league::SEASON_CLOSED` = 0, `rebuild_season` and, without a box, `set_expectations_and_renew_officials` (whose
/// return it passes on; 0 after a box).
/// Original: `NirFirst::start_new_season` (0x78be10, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_start_new_season(this: usize) -> u32 {
    let this = League(this);
    this.set_season_closed(0);
    if !rebuild_season(this, 0x78bc50, 0xa6965c, [0x1f5, 0x1fc, 0x202, 0x209], 1) { // mwa13: the next season of the Northern Irish First
        return 0;
    }
    covered!("mwa_13 northern irish first turnover");
    set_expectations_and_renew_officials(this)
}

unsafe extern "thiscall" fn compare_nir_first_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_NIR_FIRST_START_NEW_SEASON);
    compare_state_league_new_season("nir_first_start_new_season", League(this), || o(this))
}

/// The next stage of an Irish First Division (`advance_stage`, the comp virtual): at the last stage
/// (`league::CURRENT_STAGE` ≥ `league::STAGE_COUNT` − 1, signed) the error box and nothing else. Otherwise it waits for
/// the Premier Division above (the competition object of the key global `premier`, `comp_by_global`): when its
/// `comp_vtable::CHECK_COMPLETE`(0, 1) (thiscall, ret 8, the low byte) says it is over, the Premier's positions and
/// status are updated (`league_vtable::UPDATE_POSITIONS_AND_STATUS`, thiscall, ret 0), the current stage goes up by one
/// and, on reaching 0, the promotion play-off is built (`play_off`, thiscall, ret 0).
pub(crate) unsafe fn advance_irish_first_stage(this: League, premier: usize, file: usize, row: u32, play_off: usize) {
    let stage = this.current_stage() as i32;
    if stage >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(file, row);
    }
    let p = comp_by_global(premier);
    let complete: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(read_usize(read_usize(p) + comp_vtable::CHECK_COMPLETE));
    if complete(p, 0, 1) as u8 == 0 {
        return;
    }
    covered!("mwa_13 irish first division waits for the premier");
    bra_nat_first_call_virtual_no_args(p, league_vtable::UPDATE_POSITIONS_AND_STATUS);
    let stage = this.current_stage() as i32 + 1;
    this.set_current_stage(stage as usize);
    if stage == 0 {
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(play_off);
        f(this.addr());
    }
}

/// Rule: the Northern Irish First Division's league stage ends only when the Premier Division above has finished: then
/// the Premier's positions are updated and the promotion play-off is built; past the last stage, the error box.
///
/// `advance_irish_first_stage` with the Northern Irish Premier Division (`KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION`),
/// the error box at line 0x226 of nir_first.cpp and NirFirst::create_playoff_stage (0x78c0a0).
/// Original: `NirFirst::advance_stage` (0x78bfd0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_advance_stage(this: usize) {
    advance_irish_first_stage(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, 0x226, 0x78c0a0) // mwa13: the Northern Irish play-off waits for the Premier
}

/// The comparer of the Irish First Divisions' next stage: only the original runs (it may build the play-off); the
/// stage either stays or goes up by one, and never past the last stage.
pub(crate) unsafe fn compare_irish_first_advance_stage(name: &str, this: League, original: impl FnOnce()) {
    let before = this.current_stage() as i32;
    let last = before >= (this.stage_count() as i32).wrapping_sub(1);
    original();
    let after = this.current_stage() as i32;
    SCOREBOARD.record(after == before || (!last && after == before + 1), || format!("{}({:#x}) before={} after={} last={}", name, this.addr(), before, after, last));
}

unsafe extern "thiscall" fn compare_nir_first_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_FIRST_ADVANCE_STAGE);
    compare_irish_first_advance_stage("nir_first_advance_stage", League(this), || o(this))
}

/// The two clubs of an Irish promotion play-off: the First Division club whose table row has status 1 (the play-off
/// place; the last such row) and the club of the Premier Division row with status 2 (League::get_table_row
/// `exe::league_get_table_row`, thiscall, ret 4; the last such row), or None for each one missing.
pub(crate) unsafe fn irish_play_off_clubs(this: League, premier: usize) -> (Option<usize>, Option<usize>) {
    let mut first = None;
    for k in 0..this.club_count().max(0) as usize {
        let row = this.table() + k * 0x3b;
        if read_u8(row + 0x37) == 1 {
            first = Some(read_usize(row));
        }
    }
    let p = League(comp_by_global(premier));
    let mut second = None;
    let mut k = 0i32;
    while k < p.club_count() as i32 {
        if read_u8(exe::league_get_table_row(p, k) + 0x37) == 2 {
            second = Some(read_usize(exe::league_get_table_row(p, k)));
        }
        k += 1;
    }
    (first, second)
}

/// The promotion play-off of an Irish First Division: the First Division's play-off club against the Premier Division's
/// play-off club, in a two-club Cup_Stage in the first stage slot.
///
/// The list of 2 (`exe::crt_malloc` of 8; out of memory, the box `rows[0]` and nothing else); entry 0 = the First
/// Division club (`irish_play_off_clubs`; without one, the box `rows[1]` and return: the original leaves the list
/// allocated), entry 1 = the Premier Division club (without one, return with nothing else, the list left allocated).
/// The round table of stage 0 (the league's virtual `comp_vtable::CREATE_SCHEDULE`(0, &rounds, &extra, null)); the
/// stage (operator new `exe::crt_operator_new` of 0xb2; Cup_Stage::Cup_Stage `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret
/// 0x3c: the league, 2, the list, the rounds, the competition record, the table, the year, 0, 1, the extra, 0x14, 0, 0,
/// 0, 0) into the first stage slot; the list and the table freed; without the stage, the box `rows[2]`; otherwise
/// Cup::set_up_round(stage, 0) (`exe::cup_set_up_round`, thiscall, ret 4).
pub(crate) unsafe fn create_irish_promotion_play_off(this: League, premier: usize, file: usize, rows: [u32; 3], branch: &'static str) {
    let list = exe::crt_malloc(8);
    if list == 0 {
        return show_error_at(file, rows[0]);
    }
    let (first, second) = irish_play_off_clubs(this, premier);
    match first {
        Some(c) => write_usize(list, c),
        None => return show_error_at(file, rows[1]),
    }
    match second {
        Some(c) => write_usize(list + 4, c),
        None => return,
    }
    covered!(branch);
    let table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let t = table(this.addr(), 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    // simplification: the original pushes the rounds, the extra and the year with whatever is in the registers' high
    // halves; the constructor only reads the words
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 2, list, rounds as u16 as u32, this.competition().addr(), t, this.year() as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(this.stages(), stage);
    exe::crt_free(list);
    exe::crt_free(t);
    let stage = read_usize(this.stages());
    if stage == 0 {
        return show_error_at(file, rows[2]);
    }
    exe::cup_set_up_round(stage, 0);
}

/// The comparer of the Irish promotion play-offs: only the original runs (it allocates and draws); the plan, made
/// beforehand: the two clubs (`irish_play_off_clubs`). With both, the first stage slot must hold a stage of two clubs
/// of the league's competition with those two (as a set).
pub(crate) unsafe fn compare_irish_promotion_play_off(name: &str, this: League, premier: usize, original: impl FnOnce()) {
    let plan = irish_play_off_clubs(this, premier);
    original();
    if let (Some(a), Some(b)) = plan {
        let stage = Cup(read_usize(this.stages()));
        let ok = !stage.is_null() && stage.team_count() == 2 && read_usize(stage.addr() + 4) == this.competition().addr() && {
            let (x, y) = (read_usize(stage.team_table()), read_usize(stage.team_table() + 6));
            (x == a && y == b) || (x == b && y == a)
        };
        SCOREBOARD.record(ok, || format!("{}({:#x}) plan {:#x} v {:#x}, stage {:#x}", name, this.addr(), a, b, stage.addr()));
    }
}

/// Rule: the Northern Irish promotion play-off: the First Division runner-up against the Premier Division's second last
/// (the clubs with the play-off status in each table), in a two-club stage.
///
/// `create_irish_promotion_play_off` with the Northern Irish Premier Division and the error box lines 0x252, 0x262 and
/// 0x283 of nir_first.cpp.
/// Original: `NirFirst::create_playoff_stage` (0x78c0a0, nir_first.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn nir_first_create_playoff_stage(this: usize) {
    create_irish_promotion_play_off(League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, 0xa6965c, [0x252, 0x262, 0x283], "mwa_13 northern irish promotion play-off")
}

unsafe extern "thiscall" fn compare_nir_first_create_playoff_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_NIR_FIRST_CREATE_PLAYOFF_STAGE);
    compare_irish_promotion_play_off("nir_first_create_playoff_stage", League(this), KEY_CLUB_COMP_NORTHERN_IRELAND_PREMIER_DIVISION, || o(this))
}
