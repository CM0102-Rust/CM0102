use super::*;

/// Calls a no-argument thiscall virtual method of the cup at vtable offset o.
pub(crate) unsafe fn call_cup_virtual(this: Cup, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(read_usize(read_usize(this.addr()) + o));
    f(this.addr())
}

/// Season rollover of a knockout cup (`start_new_season`, slot 3, +0x08): frees the entrant, round and tie tables,
/// archives the stats, advances the year and rebuilds entrants, round table, tie tree and predicted positions.
///
/// The same template is used in all the cups, checked instruction by instruction: only the source file of the error
/// boxes, the line numbers and the entrants routine change.
///
/// Steps: +0x4c = 0; the entrant list (+0xa7) and the round list (+0xa3) are freed (`exe::crt_free`, cdecl) and zeroed; the
/// ties per round (+0xad: one array per round, +0x3c rounds, signed word) are freed and zeroed one by one, and the
/// array itself is freed and zeroed; with stats (+8), Comp_Stats::archive_season_and_reset (`exe::comp_stats_archive_season_and_reset`, push 1; mov ecx
/// -> thiscall, ret 4); the year (+0x40, word) goes up by `ano` (1); +0xab = 0 and +0xb1 = 0 (bytes).
///
/// Then, each with its own error box if it returns negative: the entrants (`inscritos`, mov ecx, esi; direct call ->
/// thiscall, ret 0), the rounds (virtual +0x8c, create_round_table for stage −1) and the tie tree (virtual +0x94);
/// finally the predicted positions (virtual +0x5c), whose eax it returns.
pub(crate) unsafe fn start_new_cup_season(this: Cup, entrants: usize, file: usize, rows: [u32; 3], year: i16, ramo: &'static str) -> u32 {
    this.set_season_closed((0) as usize);
    for o in [0xa7usize, 0xa3] {
        let l = read_usize(this.addr() + o);
        if l != 0 {
            exe::crt_free(l);
            write_u32(this.addr() + o, 0);
        }
    }
    if this.tie_tables() != 0 {
        let mut i = 0i32;
        while i < this.round_count() as i32 {
            let p = read_usize(this.tie_tables() + i as usize * 4);
            if p != 0 {
                exe::crt_free(p);
                write_u32(this.tie_tables() + i as usize * 4, 0);
            }
            i += 1;
        }
        exe::crt_free(this.tie_tables());
        this.set_tie_tables((0) as usize);
    }
    let e = read_usize(this.addr() + 8);
    if e != 0 {
        exe::comp_stats_archive_season_and_reset(e, 1);
    }
    let a = this.year();
    this.set_year(a.wrapping_add(year));
    covered!(ramo);
    this.set_current_round(0);
    this.set_run_by_stages(0);
    let entrants: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(entrants);
    // simplification: in the boxes the original exits with the box’s eax (0x5e8290); the caller of the turnover does not read the return
    if entrants(this.addr()) < 0 {
        show_error_at(file, rows[0]);
        return 0;
    }
    if call_cup_virtual(this, comp_vtable::SET_UP_RULES) < 0 {
        show_error_at(file, rows[1]);
        return 0;
    }
    if call_cup_virtual(this, cup_vtable::MAP_FIXTURE_TREE) < 0 {
        show_error_at(file, rows[2]);
        return 0;
    }
    call_cup_virtual(this, comp_vtable::SET_PREDICTED_POSITIONS) as u32
}

/// Comparer for the cup season change: only the original runs; checks the year (+0x40) went up by one.
pub(crate) unsafe fn compare_cup_new_season(name: &str, this: Cup, original: impl FnOnce() -> u32) -> u32 {
    let before = this.year();
    let r = original();
    let after = this.year();
    SCOREBOARD.record(after == before.wrapping_add(1), || format!("{}({:#x}) year before={} after={}", name, this.addr(), before, after));
    r
}

/// rule: at the season turn the Brazilian Cup is rebuilt from scratch: last edition's tables are freed, the season
/// history is archived (Comp_Stats), the year goes up by one and the cup builds again its entrants (the team table),
/// the rounds and the fixture tree, and the predicted positions
/// Original: `BraCup::start_new_season` (0x427d10, bra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_cup_start_new_season(this: usize) -> u32 {
    start_new_cup_season(Cup(this), 0x427c20, 0x98a3bc, [0x189, 0x190, 0x197], 1, "cqr_5 virada da copa do brasil") // cqr5: the Copa do Brasil
}

/// The Brazilian Cup (Copa do Brasil): seven two-legged rounds from February to July of the cup's year.
///
/// The second leg is 14 days after the first up to the round of 16, 21 in the quarter-finals and 7 in the semi-final; 4
/// days in the last three, midweek [inference].
pub(crate) const COPA_DO_BRASIL_ROUNDS: [EngCcCupCupRound; 7] = [
    cql10r((0x12, 0, 0, 6), (0x17, 1, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo12: the 1st round of the Copa do Brasil
    cql10r((9, 2, 0, 3), (0x16, 2, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x22, 0x11, 7, 0x36), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((6, 3, 0, 3), (0x1a, 3, 0, 2, 2, 0), 0x1e, (0, 0, 1), 6, (0x16, 0xb, 5, 0x3d), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 4, 0, 3), (7, 5, 0, 2, 2, 0), 0x28, (0, 0, 1), 6, (0x10, 8, 5, 0x42), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xf, 5, 0, 3), (0x11, 5, 0, 5, 1, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 5, 0, 3), (0x18, 5, 0, 5, 1, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 5, 0, 3), (1, 6, 0, 5, 1, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xf4240, 0xc3500)),
];

/// rule: the Brazilian Cup has 7 two-legged rounds, all in the cup year: 1st drawn on 18/1 and played on 23/2 (54
/// teams, 27 ties), 2nd draw 9/3, match 22/3 (34 teams: the 27 through and 7 entering now), 3rd 6/4 and 26/4 (22 teams:
/// 17 and 5 new; return leg 21 days later), 4th 18/5 and 7/6 (16 teams: 11 and 5 new; return 7 days later); the late
/// entrants are those exempt from the early rounds [inference: the clubs in the continental cups], quarter-finals 15/6
/// and 17/6, semi-final 22/6 and 24/6, final draw 29/6 and match on 1/7 (return 4 days later in the last three);
/// two-leg tie-break 1 (away goals [inference]); category 4 in the first two and 6 after; final prize 1 million to the
/// winner and 800 thousand to the runner-up
/// Original: `BraCup::create_round_table` (0x427700, bra_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn bra_cup_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, out: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(Cup(this), stage, rounds, extra, out, &COPA_DO_BRASIL_ROUNDS, 0x98a3bc, 0xa4, "cqo_12 tabela da copa do brasil")
}

/// `BraCup::build_team_table()` (`bra_cup_build_team_table`, thiscall, ret 0, bra_cup.cpp): sets +0x3e = 71 and builds the team table
/// from the entrant list; returns 1.
///
/// The team table is allocated with `exe::crt_malloc` (0x1aa bytes) at +0xa7; when out of memory, the error box for line 0x143
/// and −1. Each slot i (6 bytes) receives the club of entry i of the list at +0x14 (6 bytes), status +5 = 0xff and
/// class +4 = 0.
///
/// Returns 1.
pub(crate) unsafe fn build_brazil_cup_teams(this: Cup) -> i32 {
    this.set_team_count(0x47);
    let l = exe::crt_malloc(0x1aa);
    this.set_team_table((l as u32) as usize);
    if l == 0 {
        show_error_at(0x98a3bc, 0x143);
        return -1;
    }
    covered!("cqo_12 copa do brasil teams");
    let mut i = 0i16;
    while i < this.team_count() {
        let o = i as usize * 6;
        let font = o; // cqo12: the registration of each slot
        let c = read_usize(this.entrant_book() + font);
        let e = this.team_table() + o;
        write_u32(e, c as u32);
        write_u8(e + 5, 0xff);
        write_u8(e + 4, 0);
        i = i.wrapping_add(1);
    }
    1
}

/// rule: the Brazilian Cup has 71 places, one per entrant, in the entrant list order, all with class 0 and status 0xff
/// (no match yet [inference])
/// Original: `BraCup::build_team_table` (0x427c20, bra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_cup_build_team_table(this: usize) -> i32 {
    build_brazil_cup_teams(Cup(this))
}

/// `BraCup::set_up_rounds()` (`0x427ec0`, thiscall, ret 0, `bra_cup.cpp`; a virtual of the cup): sets up the cup's
/// basic rules and round table.
///
/// 7 substitutes on the bench (`+0x49`), 3 substitutions (`+0x4a`), the 500,000 prize fund (`+0x28`) and the round
/// table in `+0xa3` (virtual `+0x3c` with stage −1, the rounds in `+0x3c` and the extra in `+0x3a`, no output). Returns
/// 1.
pub(crate) unsafe fn set_up_brazil_cup_rounds(this: Cup) -> i32 {
    covered!("cqo_12 copa do brasil rules");
    this.set_substitutes_named(7);
    this.set_substitutions_allowed(3); // cqo12: the Copa do Brasil substitutions
    this.set_prize_fund((0x7a120) as i32);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0);
    this.set_round_table((t as u32) as usize);
    1
}

/// rule: Brazilian Cup matches have 7 substitutes on the bench and 3 substitutions; the prize fund is 500 thousand;
/// builds the round table
/// Original: `BraCup::set_up_rounds` (0x427ec0, bra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_cup_set_up_rounds(this: usize) -> i32 {
    set_up_brazil_cup_rounds(Cup(this))
}

