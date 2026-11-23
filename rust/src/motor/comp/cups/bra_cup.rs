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

/// The Copa do Brasil constructor `BraCup::BraCup(year, record)`, run once at career creation. On the Cup::Cup base
/// (`exe::new_cup`): the competition record at +4, the vtable (0x967cf8), the year (`cup::YEAR`), `current_round()` =
/// 0, `stage_index()` = 0xff, `current_stage()` = −1, `stage_count()` = 0, `club_sides()` = 1 (clubs of one nation),
/// discipline rule 6 and `run_by_stages()` = 0. If the saved state loads (Cup::load_from_disk 0x51fc00, thiscall(this,
/// 1), ret 4; returns ≠ 0), it returns this. On a new cup, in order: the 71 entrants (BraCup::select_entrants
/// 0x427f00, thiscall, ret 0, unchecked); the 71-team table (BraCup::build_team_table 0x427c20, thiscall, ret 0; the
/// error box of line 0x4b if it returns 0); 7 substitutes named, 3 substitutions allowed and the prize fund
/// (`cup::PRIZE_FUND`) = 500,000; the rounds (`comp_vtable::CREATE_SCHEDULE`, into `round_table()`); the tie tree
/// (Cup::map_fixture_tree 0x518790, thiscall, ret 0; the error box of line 0x59 if it returns 0); the statistics at +8
/// (the error box of line 0x60 without them); the final ranking (Cup::rank_teams 0x5223a0, thiscall, ret 0,
/// unchecked). Every failure returns this. [the bench, substitutions, discipline and type fields read from the
/// competitions reference, §2.1]
/// Original: `BraCup::BraCup` (0x427430, bra_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    construct_bra_cup_body(this, year, record) // cqv6: the Copa do Brasil
}

/// Runs only the original (it allocates, draws and reads the save file), checking the plan (`bra_cup_ctor_plan_error`).
pub(crate) unsafe extern "thiscall" fn compare_construct_bra_cup(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_BRA_CUP)(this, year, record);
    let error = bra_cup_ctor_plan_error(this, r, year, record, BRA_CUP_VTABLE, &BRA_CUP_CTOR_FIELDS, true);
    SCOREBOARD.record(error.is_none(), || format!("construct_bra_cup({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The 71 Copa do Brasil entrants, chosen in the cup constructor (0x427430) at career creation. The entrant book
/// (`cup::ENTRANT_BOOK`, `exe::crt_calloc`(6, 0x47), cdecl: 0x47 six-byte records; out of memory, the error box of line
/// 0x1e0), `cup::ENTRANT_COUNT` = `cup::ENTRANT_CAPACITY` = 0x47 and +0x4b = 0xff.
/// 1) In club table order, each Brazilian club (`club::NATION` = [`KEY_NATION_BRAZIL`]) whose name (`club::NAME`) is
///    one of the 71 fixed ones (`BRA_CUP_ENTRANT_NAMES`) goes to that name's slot. With all 71 slots filled, done.
/// 2) Otherwise (an edited database): up to 200 clubs of the twelve state leagues (`club::RESERVE_DIVISION`,
///    `BRA_CUP_STATE_LEAGUES`) in a list sorted by reputation (`exe::crt_qsort` with 0x4b07d0, cdecl); the best of each
///    state league goes to its group (slots 0x36.., 0x3d.. or 0x42..), and up to 5 from the
///    [`KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION`] division (`club::DIVISION`) to group 0x36; then the others,
///    up to 5 per state league (7 in the three biggest [inference]), from slot 0 on. Without 54 + 7 + 5 + 5, the error
///    box of line 0x3ce.
/// Original: `BraCup::select_entrants` (0x427f00, bra_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn bra_cup_select_entrants(this: usize) {
    bra_cup_select_entrants_body(this)
}

/// Side by side from the same state (`cup::ENTRANT_COUNT`/`cup::ENTRANT_CAPACITY`, +0x4b, [0xb67a34]; it draws
/// nothing): the original leaves its book in `cup::ENTRANT_BOOK`, the copy builds its own, compared over the 0x47
/// six-byte records and freed.
pub(crate) unsafe extern "thiscall" fn compare_bra_cup_select_entrants(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BRA_CUP_SELECT_ENTRANTS);
    let before = read_usize(this + cup::ENTRANT_BOOK);
    let book_original = core::cell::Cell::new(0usize);
    let book_rust = core::cell::Cell::new(0usize);
    let (_, _, same, _, _) = crate::comparison::side_by_side(&[(this + cup::ENTRANT_COUNT, 4usize), (this + 0x4b, 1), (0xb67a34, 4)], || {
        o(this);
        book_original.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, before as u32);
    }, || {
        bra_cup_select_entrants_body(this);
        book_rust.set(read_usize(this + cup::ENTRANT_BOOK));
        write_u32(this + cup::ENTRANT_BOOK, book_original.get() as u32);
    });
    let (lo, lr) = (book_original.get(), book_rust.get());
    let book = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x47 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x47 * 6);
    if lr != 0 && lr != lo {
        exe::crt_free(lr);
    }
    SCOREBOARD.record(same && book, || format!("bra_cup_select_entrants({:#x}) memory {} book {}", this, same, book));
}

/// The plan the comparer checks after the original: the return value (this), the vtable, the competition record (+4)
/// and the year (`cup::YEAR`); on a new cup (`stage_index()` = 0xff), each fixed field the constructor writes
/// (offset, width in bytes, value) and the round table and statistics it created. Returns the first difference.
pub(crate) unsafe fn bra_cup_ctor_plan_error(this: usize, r: usize, year: u32, record: usize, vtable: usize, fields: &[(usize, u8, u32)], rounds: bool) -> Option<String> {
    if r != this || read_usize(this) != vtable || read_usize(this + 4) != record || read_u16(this + cup::YEAR) != year as u16 {
        return Some(format!("returned {:#x}, vtable {:#x}, +4 {:#x}, year {}", r, read_usize(this), read_usize(this + 4), read_u16(this + cup::YEAR)));
    }
    if read_u8(this + cup::STAGE_INDEX) != 0xff || league_will_load_saved_state(record) {
        return None;
    }
    for &(o, t, v) in fields {
        let x = match t {
            1 => read_u8(this + o) as u32,
            2 => read_u16(this + o) as u32,
            _ => read_u32(this + o),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, the constructor writes {:#x}", o, x, v));
        }
    }
    if (rounds && read_usize(this + cup::ROUND_TABLE) == 0) || read_usize(this + 8) == 0 {
        return Some(format!("rounds {:#x}, statistics {:#x}", read_usize(this + cup::ROUND_TABLE), read_usize(this + 8)));
    }
    None
}

/// The body of the Copa do Brasil constructor (see `construct_bra_cup`).
pub(crate) unsafe fn construct_bra_cup_body(this: usize, year: u32, record: usize) -> usize {
    let cup = Cup(this);
    exe::new_cup(this);
    write_usize(this + 4, record);
    write_usize(this, BRA_CUP_VTABLE);
    cup.set_year(year as i16);
    cup.set_current_round(0);
    cup.set_stage_index(0xff);
    cup.set_current_stage(0xffff_ffff);
    cup.set_stage_count(0);
    cup.set_club_sides(1);
    cup.set_discipline_index(6);
    cup.set_run_by_stages(0);
    if exe::cup_load_from_disk(this, 1) != 0 {
        return this;
    }
    covered!("cqv_6 copa do brasil new");
    let select_entrants: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_BRACUP_SELECT_ENTRANTS); // return unused
    select_entrants(this);
    if exe::bracup_build_team_table(this) == 0 {
        show_error_at(0x98a3bc, 0x4b);
        return this;
    }
    cup.set_substitutes_named(7);
    cup.set_substitutions_allowed(3);
    cup.set_prize_fund(0x7a120);
    bra_cup_ctor_create_rounds(this);
    if exe::cup_map_fixture_tree(this) == 0 {
        show_error_at(0x98a3bc, 0x59);
        return this;
    }
    if !bra_cup_ctor_create_stats(this) {
        show_error_at(0x98a3bc, 0x60);
        return this;
    }
    exe::cup_rank_teams(this);
    this
}

/// The body of `bra_cup_select_entrants` (also run by its comparer).
pub(crate) unsafe fn bra_cup_select_entrants_body(this: usize) {
    write_u8(this + 0x4b, 0xff);
    write_u16(this + cup::ENTRANT_CAPACITY, 0x47);
    write_u16(this + cup::ENTRANT_COUNT, 0x47);
    let book = exe::crt_calloc(6, 0x47);
    write_u32(this + cup::ENTRANT_BOOK, book as u32);
    if book == 0 {
        show_error_at(0x98a3bc, 0x1e0);
        return;
    }
    let base = read_usize(G_CLUB_TABLE);
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let c = base + i as u16 as usize * club::SIZE;
        let nation = read_usize(c + club::NATION);
        if nation != 0 && read_usize(nation) == read_usize(KEY_NATION_BRAZIL) {
            for (k, &t) in BRA_CUP_ENTRANT_NAMES.iter().enumerate() { // cqv6: the 71 names
                if bra_cup_club_name_is(c, t) {
                    write_u32(read_usize(this + cup::ENTRANT_BOOK) + k * 6, c as u32); // cqv6: the slot of the name
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut k: i16 = 0;
    while k < 0x47 {
        if read_usize(read_usize(this + cup::ENTRANT_BOOK) + k as usize * 6) == 0 {
            break;
        }
        k += 1;
    }
    if k == 0x47 {
        covered!("cqv_6 the 71 names");
        return;
    }
    covered!("cqv_6 fill from the state leagues");
    // the original's counters (stack bytes, zeroed at the start): [0] the next slot for the others, [1..3] the count of
    // each group (0x3d, 0x42, 0x36), [4..0xf] the mark and count of each state league, [0x10] those of the
    // KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION division
    let mut b = [0i8; 0x11];
    let group_count = |g: usize| [3usize, 1, 2][g];
    let group_first_slot = |g: usize| [0x36i32, 0x3d, 0x42][g];
    let mut list = [0usize; 200];
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        if n >= 0xc8 {
            break;
        }
        let c = base + i as u16 as usize * club::SIZE;
        if read_usize(c + club::RESERVE_DIVISION) != 0 && bra_cup_state_league_of(c).is_some() {
            list[n as usize] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    exe::crt_qsort(list.as_mut_ptr() as usize, n as i32 as u32, 4, 0x4b07d0usize);
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, g, _) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] == 0 {
                let cg = group_count(g);
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + (group_first_slot(g) + b[cg] as i32) as usize * 6, c as u32);
                b[mark] = 1;
                b[cg] = b[cg].wrapping_add(1);
                list[j] = 0;
            }
        }
        let c = list[j];
        let d = if c != 0 { read_usize(c + club::DIVISION) } else { 0 };
        if d != 0 && read_usize(d) == read_usize(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION) && b[0x10] < 5 {
            write_u32(read_usize(this + cup::ENTRANT_BOOK) + (0x36 + b[3] as i32) as usize * 6, c as u32);
            b[3] = b[3].wrapping_add(1);
            b[0x10] = b[0x10].wrapping_add(1);
            list[j] = 0;
        }
    }
    for j in 0..n.max(0) as usize {
        let c = list[j];
        if c == 0 {
            continue;
        }
        if let Some(e) = bra_cup_state_league_of(c) {
            let (_, mark, _, ceiling) = BRA_CUP_STATE_LEAGUES[e];
            if b[mark] < ceiling {
                write_u32(read_usize(this + cup::ENTRANT_BOOK) + b[0] as i32 as usize * 6, c as u32);
                b[0] = b[0].wrapping_add(1);
                b[mark] = b[mark].wrapping_add(1);
            }
        }
    }
    if !(b[0] == 0x36 && b[3] == 7 && b[1] == 5 && b[2] == 5) {
        show_error_at(0x98a3bc, 0x3ce);
    }
}

/// The cup's rounds through its `comp_vtable::CREATE_SCHEDULE` method (push 0; push &`cup::STAGE_LABEL`; push
/// &`cup::ROUND_COUNT`; push −1; mov ecx, this -> thiscall, ret 0x10): the round table, stored in `round_table()`.
pub(crate) unsafe fn bra_cup_ctor_create_rounds(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let t = f(this, 0xffff_ffff, this + cup::ROUND_COUNT, this + cup::STAGE_LABEL, 0);
    Cup(this).set_round_table(t);
}

/// The statistics (`exe::crt_operator_new`(0x5ce), cdecl; `exe::new_comp_stats`(new, this), thiscall, ret 4) at +8; the
/// original's SEH frame covers only that new. Returns whether they were created.
pub(crate) unsafe fn bra_cup_ctor_create_stats(this: usize) -> bool {
    let n = exe::crt_operator_new(0x5ce);
    let s = if n != 0 { exe::new_comp_stats(n, this) } else { 0 };
    write_usize(this + 8, s);
    s != 0
}

/// Is the club's name (`club::NAME`) equal to the text `t` (the original's inline comparison, byte by byte up to the
/// terminating zero)?
pub(crate) unsafe fn bra_cup_club_name_is(c: usize, t: usize) -> bool {
    let c = Club(c);
    let mut i = 0usize;
    loop {
        let a = read_u8(c.addr() + club::NAME + i);
        if a != read_u8(t + i) {
            return false;
        }
        if a == 0 {
            return true;
        }
        i += 1;
    }
}

/// The club's state league (`club::RESERVE_DIVISION`; the record's +0 is its id), in the original's test order.
pub(crate) unsafe fn bra_cup_state_league_of(c: usize) -> Option<usize> {
    let c = Club(c);
    let id = read_usize(c.reserve_division().addr());
    BRA_CUP_STATE_LEAGUES.iter().position(|e| id == read_usize(e.0))
}

pub(crate) const BRA_CUP_VTABLE: usize = 0x967cf8;

/// The fixed fields the Copa do Brasil constructor writes on a new cup (offset, width, value).
pub(crate) const BRA_CUP_CTOR_FIELDS: [(usize, u8, u32); 10] = [(cup::PRIZE_FUND, 4, 0x7a120), (cup::STAGE_COUNT, 4, 0x0), (cup::CURRENT_STAGE, 4, 0xffffffff), (cup::CLUB_SIDES, 1, 0x1), (cup::STAGE_INDEX, 1, 0xff), (cup::SUBSTITUTES_NAMED, 1, 0x7), (cup::SUBSTITUTIONS_ALLOWED, 1, 0x3), (cup::DISCIPLINE_INDEX, 1, 0x6), (cup::CURRENT_ROUND, 1, 0x0), (cup::RUN_BY_STAGES, 1, 0x0)];

/// The 71 fixed Copa do Brasil names, in slot order (texts in the exe, read from there like the original): Vasco (0),
/// Remo, Paysandu, River, Sampaio Corrêa, Nacional (AM), ABC, América (RN), Ceará, Confiança, CRB, Sergipe, Comercial
/// Bandeirantes, Comercial, Juventus, Serra, Tocantinópolis, União, Camaçari, Fluminense de Feira, Juazeiro, Vitória,
/// Caxias, Internacional, Juventude, Pelotas, Anapolina, Anápolis, Atlético Goianiense, Goiânia, Atlético Mineiro,
/// Caldense, Uberlândia, Villa Nova, Paraná, Londrina, Malutrom, Coritiba, Central, Ferroviário, Porto, Santa Cruz,
/// América (RJ), Americano, Bangu, Botafogo, Avaí, Criciúma, Figueirense, Tubarão, Botafogo (SP), Palmeiras, Rio
/// Branco, Santos, Sport, Ponte Preta, São Paulo, Gama, São Raimundo, Fluminense, Goiás, Vila Nova, Náutico, Bahia,
/// Fortaleza, Joinville, América (MG), Grêmio, Flamengo, Atlético Paranaense and Corinthians (70).
pub(crate) const BRA_CUP_ENTRANT_NAMES: [usize; 71] = [0x98a398, 0x98aa20, 0x98aa0c, 0x98a9f4, 0x98a9d4, 0x98a9b8, 0x98a9a4, 0x98a988, 0x98a974, 0x98a954, 0x98a93c, 0x98a924, 0x98a8f8, 0x98a8e0, 0x98a8b4, 0x98a8a0, 0x98a880, 0x98a86c, 0x98a854, 0x98a830, 0x98a818, 0x98a800, 0x98a7dc, 0x98a7c0, 0x98a7a8, 0x98a790, 0x98a770, 0x98a758, 0x98a73c, 0x98a724, 0x98a70c, 0x98a6ec, 0x98a6d0, 0x98a6b4, 0x98a6a4, 0x98a68c, 0x98a67c, 0x98a664, 0x98a650, 0x98a634, 0x98a61c, 0x98a600, 0x98a5e4, 0x98a5c4, 0x98a5ac, 0x98a58c, 0x98a578, 0x98a560, 0x98a544, 0x98a52c, 0x98a510, 0x98a4f0, 0x98a4d4, 0x98a4bc, 0x98a4a4, 0x98a484, 0x98a46c, 0x98a450, 0x98a344, 0x98a434, 0x98a420, 0x98a380, 0x98a364, 0x98a294, 0x98a408, 0x98a3f0, 0x98a328, 0x98a304, 0x98a2e4, 0x98a2c8, 0x98a2a8];

/// The twelve Copa do Brasil state leagues: the key, the control byte (the best-club mark in step 2, later the count
/// of the others), the slot group (0: slots 0x36..; 1: 0x3d..; 2: 0x42..) and the ceiling for the others (5 or 7). The
/// order is the original's test order; the indices are the original's stack bytes minus 0x13. [inference: the three
/// with ceiling 7 are the biggest state leagues]
pub(crate) const BRA_CUP_STATE_LEAGUES: [(usize, usize, usize, i8); 12] = [
    (KEY_CLUB_COMP_BRAZILIAN_BAHIA_STATE_CHAMPIONSHIP, 0xc, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_CENTRAL_STATE_CHAMPIONSHIP, 0xb, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_GAUCHO_STATE_CHAMPIONSHIP, 0x7, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_GOIAS_STATE_CHAMPIONSHIP, 0xd, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_MINAS_GERAIS_STATE_CHAMPIONSHI, 0x9, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_NORTH_STATE_CHAMPIONSHIP, 0xf, 0, 7),
    (KEY_CLUB_COMP_BRAZILIAN_NORTHEAST_STATE_CHAMPIONSHIP, 0xe, 1, 7),
    (KEY_CLUB_COMP_BRAZILIAN_PARANA_STATE_CHAMPIONSHIP, 0x4, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_PERNAMBUCO_STATE_CHAMPIONSHIP, 0xa, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_RIO_DE_JANEIRO_STATE_CHAMPIONS, 0x5, 2, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SANTA_CATARINA_STATE_CHAMPIONS, 0x6, 1, 5),
    (KEY_CLUB_COMP_BRAZILIAN_SAO_PAULO_STATE_CHAMP, 0x8, 2, 5),
];
