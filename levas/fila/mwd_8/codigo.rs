// ---------- batch %%LEVA%%: the Croatian Cup (cro_cup.cpp) ----------

/// What changes from one national cup constructor to another (the CroCup template, shared by the Polish, Russian and
/// Turkish cups and the Greek and Polish super cups).
pub(crate) struct EastNationCup {
    pub(crate) vtable: usize,
    /// the discipline rule (+0x50)
    pub(crate) discipline: u8,
    /// `collect_entrants` (or `select_teams` in a super cup), direct call, thiscall, ret 0
    pub(crate) entrants: usize,
    /// the prize fund (+0x28) written before the round table; `None` in the super cups (not written)
    pub(crate) prize: Option<u32>,
    /// +0x49 = 7 and +0x4a = 3 written after the round table instead of before the load (the Polish League Cup)
    pub(crate) late_substitutes: bool,
    /// the stage vector (+0xc) allocated from +0x2c (the Russian Cup)
    pub(crate) stage_vector: bool,
    /// Cup::rank_teams at the end (not in the super cups)
    pub(crate) rank: bool,
    pub(crate) file: usize,
    /// the boxes: entrants, fixture tree, statistics
    pub(crate) rows: [u32; 3],
}

/// Calls the exe routine at `f` as thiscall(c) with no arguments (ret 0) and returns its eax.
pub(crate) unsafe fn call_thiscall_no_args(f: usize, c: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(f);
    f(c)
}

/// The body of the national cup constructors (thiscall(this, year, record), ret 8; the SEH frame only covers the
/// operator new of the statistics).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the competition record (+4); the vtable; the year (+0x40, word); the
/// current round (+0xab) = 0; the main stage (+0x44 = 0xff), current stage −1 (+0x30), no stages (+0x2c = 0); club
/// sides (+0x42 = 1); 7 substitutes (+0x49) and 3 substitutions (+0x4a) (here or later, `late_substitutes`); the
/// discipline rule (+0x50); +0xb1 = 0. The saved state (Cup::load_from_disk 0x51fc00: push 1 -> thiscall, ret 4):
/// loaded, returns this. Otherwise: the stage vector when `stage_vector` (+0xc = malloc(+0x2c × 4) when +0x2c > 0,
/// else 0); the entrants (`entrants`; 0: the box of line `rows[0]` and this; note that the −1 of a failed allocation
/// passes as success, as the original); the prize fund; the round table (virtual `comp_vtable::CREATE_SCHEDULE`: push 0;
/// push &+0x3a; push &+0x3c; push −1 -> thiscall, ret 0x10) into `cup::ROUND_TABLE`; the fixture tree
/// (Cup::map_fixture_tree 0x518790, thiscall, ret 0; 0: the box `rows[1]` and this); the statistics (operator new
/// `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats `exe::new_comp_stats`(this); 0: the box `rows[2]` and
/// this) into +8; last, when `rank`, the predicted positions (Cup::rank_teams 0x5223a0, thiscall, ret 0). Returns
/// this.
pub(crate) unsafe fn construct_east_nation_cup(this: Cup, year: u32, record: usize, p: &EastNationCup, branch: &'static str) -> usize {
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(p.entrants);
    exe::new_cup(this);
    write_usize(this.addr() + 4, record);
    write_usize(this.addr(), p.vtable);
    this.set_year((year as u16) as i16);
    write_u8(this.addr() + cup::CURRENT_ROUND, 0);
    write_u8(this.addr() + cup::STAGE_INDEX, 0xff);
    write_i32(this.addr() + cup::CURRENT_STAGE, -1);
    write_u32(this.addr() + cup::STAGE_COUNT, 0);
    write_u8(this.addr() + cup::CLUB_SIDES, 1);
    if !p.late_substitutes {
        write_u8(this.addr() + cup::SUBSTITUTES_NAMED, 7);
        write_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED, 3);
    }
    write_u8(this.addr() + cup::DISCIPLINE_INDEX, p.discipline); // mwd8: the discipline rule
    write_u8(this.addr() + 0xb1, 0);
    let load: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if load(this.addr(), 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    if p.stage_vector {
        let n = read_i32(this.addr() + cup::STAGE_COUNT);
        let v = if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) } else { 0 };
        write_usize(this.addr() + cup::STAGES, v);
    }
    if entrants(this.addr()) == 0 {
        show_error_at(p.file, p.rows[0]);
        return this.addr();
    }
    if let Some(v) = p.prize {
        write_u32(this.addr() + cup::PRIZE_FUND, v);
    }
    let schedule: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = schedule(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    write_usize(this.addr() + cup::ROUND_TABLE, t);
    if p.late_substitutes {
        write_u8(this.addr() + cup::SUBSTITUTES_NAMED, 7);
        write_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED, 3);
    }
    if call_thiscall_no_args(0x518790, this.addr()) == 0 {
        show_error_at(p.file, p.rows[1]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    write_usize(this.addr() + 8, s);
    if s == 0 {
        show_error_at(p.file, p.rows[2]);
        return this.addr();
    }
    if p.rank {
        call_thiscall_no_args(0x5223a0, this.addr());
    }
    this.addr()
}

/// Comparer for the national cup constructors: only the original runs; checks the return, vtable, year, record,
/// discipline rule, substitutes and, for a new cup (not loaded, with statistics), the team and round tables and the
/// prize fund.
pub(crate) unsafe fn compare_east_nation_cup_ctor(name: &str, this: Cup, year: u32, record: usize, p: &EastNationCup, original: impl FnOnce() -> usize) -> usize {
    let loaded = league_will_load_saved_state(record);
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && (this.year() as u16) == year as u16 && read_usize(this.addr() + 4) == record
        && this.discipline_index() == p.discipline && read_u8(this.addr() + cup::SUBSTITUTES_NAMED) == 7 && read_u8(this.addr() + cup::SUBSTITUTIONS_ALLOWED) == 3;
    if !loaded && read_usize(this.addr() + 8) != 0 {
        ok = ok && this.team_table() != 0 && this.round_table() != 0 && p.prize.map_or(true, |v| this.prize_fund() == v as i32);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}) [year {} arg {}, record {:#x} arg {:#x}]: returned {:#x}, vtable {:#x}, discipline {}, teams {:#x}, rounds {:#x}, prize {}", name, this.addr(), this.year(), year as u16,
        read_usize(this.addr() + 4), record, r, read_usize(this.addr()), this.discipline_index(), this.team_table(), this.round_table(), this.prize_fund()));
    r
}

/// The Croatian Cup: discipline rule 0x21, 100,000 prize fund.
pub(crate) const CROATIAN_CUP: EastNationCup = EastNationCup {
    vtable: 0x9692e0,
    discipline: 0x21,
    entrants: 0x517bb0,
    prize: Some(0x186a0),
    late_substitutes: false,
    stage_vector: false,
    rank: true,
    file: 0x9b5468,
    rows: [0x40, 0x4e, 0x55],
};

/// Rule: the Croatian Cup at career creation: discipline rule 0x21, 7 substitutes and 3 substitutions and, with no
/// saved state, the 48 entrants, a 100,000 prize fund, the six rounds, the fixture tree, the statistics and the
/// predicted positions (`construct_east_nation_cup`).
/// Original: `CroCup::CroCup` (0x517480, cro_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_cro_cup(this: usize, year: u32, record: usize) -> usize {
    construct_east_nation_cup(Cup(this), year, record, &CROATIAN_CUP, "mwd_8 croatian cup new")
}

/// The Croatian Cup: six single-leg rounds from June to April, no prize per round won (−1 = a share of the fund).
pub(crate) const CROATIAN_CUP_ROUNDS: [FraSuperCupRound; 6] = [
    cql12r((0x10, 6, 0, 4), (0xf, 7, 0, 6, 1, 0), 0xe6, (0, 1, 0), 4, (0x20, 0x10, 0x20, 0), (0, 1, 0), (0, 0xffffffff, 0)), // mwd8: the preliminary round
    cql12r((0x14, 7, 0, 4), (0x16, 8, 0, 2, 2, 0), 0xa, (0, 1, 0), 4, (0x20, 0x10, 0x10, 0x20), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x18, 8, 0, 4), (0x1b, 9, 0, 2, 2, 0), 0x14, (0, 1, 0), 4, (0x10, 8, 0, 0), (0, 1, 0), (0, 0xffffffff, 0)),
    cql12r((0x1d, 9, 0, 4), (7, 0xa, 0, 2, 2, 0), 0x78, (0, 0, 1), 4, (8, 4, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql12r((0x17, 0xa, 0, 4), (0x14, 2, 1, 2, 2, 0), 0x82, (0, 0, 3), 4, (4, 2, 0, 0), (0, 2, 0xe), (0, 0xffffffff, 0)),
    cql12r((4, 3, 1, 3), (0x18, 4, 1, 2, 2, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 7), (0, 0xffffffff, 0)),
];

/// Rule: the Croatian Cup has six rounds: a preliminary round (code 0xe6, 32 teams: the 32 second division clubs) drawn
/// 16/7, played 15/8 (weekday 6); the first round (16 new teams: the First Division) drawn 20/8, played 22/9 (weekday
/// 2); the round of 16 drawn 24/9, played 27/10; the quarter-finals, two legs 14 days apart, drawn 29/10, played 7/11;
/// the semi-finals, two legs 14 days apart (two-leg tie-break 3), drawn 23/11, played 20/3 of the next year; the
/// final, two legs 7 days apart, drawn 4/4, played 24/5 (category 6); no prize per round won (a share of the fund).
/// Original: `CroCup::create_round_table` (0x517740, cro_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn cro_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    fra_super_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &CROATIAN_CUP_ROUNDS, 0x9b5468, 0x99, "mwd_8 croatian cup rounds", |_| {})
}

/// Rule: the Croatian Cup takes 48 clubs: first the 32 clubs of both second division groups
/// (`KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH`/`_SOUTH`), each with a random seed 0..49 (`exe::utils_random`(50)),
/// then the 16 First Division clubs (`KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4`), in database order, status 0.
///
/// `team_count` = 48; the list (`exe::crt_malloc` of 48 × 6 into `cup::TEAM_TABLE`; out of memory: the box at line
/// 0x12b and −1); the clubs looked at are the first [`G_CLUB_COUNT`] − 2 × [`G_NATION_COUNT`] (without the national
/// teams [inference]); there is no bound on the list (the database fills it exactly). The First Division clubs' seed
/// byte (+4) is not written: it keeps what the allocation had (as the original). Returns 1.
/// Original: `CroCup::collect_entrants` (0x517bb0, cro_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cro_cup_collect_entrants(this: usize) -> i32 {
    let this = Cup(this);
    this.set_team_count(0x30);
    let l = exe::crt_malloc(0x120);
    this.set_team_table(l);
    if l == 0 {
        show_error_at(0x9b5468, 0x12b);
        return -1;
    }
    let limit = || (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let mut k = 0usize;
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && (read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_NORTH) || read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_SECOND_DIVISION_SOUTH)) {
            let e = this.team_table() + k;
            write_usize(e, club);
            write_u8(e + 5, 0);
            write_u8(this.team_table() + k + 4, exe::utils_random(0x32) as u8); // mwd8: the second division seed
            k += 6;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    covered!("mwd_8 croatian cup entrants");
    let mut club = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < limit() {
        let d = read_usize(club + club::DIVISION);
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4) {
            write_usize(this.team_table() + k, club);
            write_u8(this.team_table() + k + 5, 0);
            k += 6;
        }
        i += 1;
        club = club.wrapping_add(club::SIZE);
    }
    1
}

/// Comparer for `construct_cro_cup`.
pub(crate) unsafe extern "thiscall" fn compare_construct_cro_cup(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    compare_east_nation_cup_ctor("construct_cro_cup", Cup(this), year, record, &CROATIAN_CUP, || o(this, year, record))
}

/// Comparer for `cro_cup_create_round_table` (side by side, the written rounds compared).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_cup_round_table("cro_cup_create_round_table", stage, rounds, extra, out, || o(this, stage, rounds, extra, out), |r, e, s| cro_cup_create_round_table(this, stage, r, e, s))
}

/// Side by side for an entrant list whose seeds are written only for some entries: the original and the copy (the
/// random generator restored between them), then each entry's club and status compared, and the seed only where
/// `seeded(entry index)`; the copy's list is freed.
pub(crate) unsafe fn compare_entrants_with_seeds(name: &str, this: Cup, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32, seeded: impl Fn(usize) -> bool) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this.addr() + cup::TEAM_COUNT, 2usize), (G_MOUSE_LEFT_BUTTON_DOWN, 4)];
    let (a, b, same, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o();
        lo.set(this.team_table());
        r
    }, || {
        let r = c();
        lr.set(this.team_table());
        this.set_team_table(lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let n = if a == 1 { this.team_count().max(0) as usize } else { 0 };
    let list = if lo != 0 && lr != 0 {
        (0..n).all(|k| read_usize(lo + 6 * k) == read_usize(lr + 6 * k) && read_u8(lo + 6 * k + 5) == read_u8(lr + 6 * k + 5) && (!seeded(k) || read_u8(lo + 6 * k + 4) == read_u8(lr + 6 * k + 4)))
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && a == b && list, || format!("{}({:#x}) returned {} x {} memory {} list {} ({} entries)", name, this.addr(), a, b, same, list, n));
    a
}

/// Comparer for `cro_cup_collect_entrants` (the seeds compared only for the second division clubs).
pub(crate) unsafe extern "thiscall" fn compare_cro_cup_collect_entrants(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let c = Cup(this);
    let first = KEY_CLUB_COMP_CROATIAN_FIRST_DIVISION_9CF8A4;
    compare_entrants_with_seeds("cro_cup_collect_entrants", c, || o(this), || cro_cup_collect_entrants(this), |k| {
        let club = read_usize(c.team_table() + 6 * k);
        let d = if club != 0 { read_usize(club + club::DIVISION) } else { 0 };
        d != 0 && read_usize(d) != read_usize(first)
    })
}
