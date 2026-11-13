// ---------- batch %%LEVA%%: the Irish FAI Senior Challenge Cup: the round table, the entrants and the season turnover (ire_chal_cup.cpp) ----------

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    compare_cup_new_season("ire_chal_cup_start_new_season", Cup(this), || o(this))
}
