use super::*;

/// AsiaClubChamp::advance_stage: moves the Asian Club Championship to its next stage (+0x30); at the last stage (+0x30
/// >= +0x2c - 1) it shows the error box instead. Reaching stage 1 (after the two zonal knockouts) it creates the two
/// groups (`exe::asiaclubchamp_create_group_stages`); reaching stage 3 (after the groups) it creates the semi-finals (`exe::asiaclubchamp_create_semi_final_stage`).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_advance_stage(this: usize) {
    let this = Cup(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        return show_error_at(0x987a40, 0x355);
    }
    let f = f + 1;
    this.set_current_stage(f as usize);
    if f == 1 {
        covered!("cqt_12 asia groups");
        exe::asiaclubchamp_create_group_stages(this);
    } else if f == 3 {
        exe::asiaclubchamp_create_semi_final_stage(this);
    }
}

// simplification: the groups and the semifinals allocate and draw: they do not run twice. Original only; the plan: +0x30 goes up by one
// (the groups write 2, the same value) outside the last stage, which gives the message box and changes nothing.

/// The 8 group-stage teams: the survivors (+5 = 1 in the 6-byte table) of the first zone's knockout (the cup's table,
/// +0xa7) up to 4, then those of the second zone's knockout (table +0xa7 of stage [+0xc][0]) up to 8.
///
/// Both searches go up to the cup's +0x3e (signed word). Returns the 8 (leftover slots stay 0: the original routine
/// leaves them with stack leftovers).
pub(crate) unsafe fn group_stage_qualifiers(this: Cup) -> [usize; 8] {
    let mut v = [0usize; 8];
    let mut n = 0usize;
    let teams = this.team_count();
    let mut i = 0i16;
    while i < teams && n < 4 {
        let e = this.team_table() + i as usize * 6;
        if read_u8(e + 5) == 1 {
            v[n] = read_usize(e); // cqt12: um vivo da primeira zona
            n += 1;
        }
        i += 1;
    }
    let mut i = 0i16;
    while i < teams && n < 8 {
        let e = read_usize(read_usize(this.stages()) + cup::TEAM_TABLE) + i as usize * 6;
        if read_u8(e + 5) == 1 {
            v[n] = read_usize(e);
            n += 1;
        }
        i += 1;
    }
    v
}

/// Creates one group (a League_Stage with 4 teams) in [+0xc][`g`], frees the round table and returns the stage.
///
/// The round table comes from virtual +0x3c (push 0; push &id; push &rodadas; push `g`; mov ecx, this -> thiscall, ret
/// 0x10). The stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; constructor League_Stage 0x68b100, thiscall, ret 0x58,
/// 22 arguments: the cup, 4, the 4 teams, 1, +4, the round table, the rounds (word), 3, 1, 8, &{1, 2, 0, 0}, &{2, 0, 0,
/// 0}, the year +0x40 (word), `g`, the id (word), +0x51, `a17`, 0, 0x28, −1, 0, 2; the original routine pushes the
/// words and bytes with garbage in the high bytes, which the constructor does not read) goes into [+0xc][`g`]; the
/// table is freed (`exe::crt_free`).
///
/// Returns the stage.
pub(crate) unsafe fn create_asia_group(this: Cup, g: u32, teams: *const usize, a17: u32) -> usize {
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let rules: [u8; 4] = [1, 2, 0, 0];
    let rules_b: [u8; 4] = [2, 0, 0, 0];
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), g, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this.addr(),
            4,
            teams,
            1,
            read_usize(this.addr() + 4),
            table,
            rodadas as u16 as u32,
            3,
            1,
            8,
            rules.as_ptr(),
            rules_b.as_ptr(),
            this.year() as u16 as u32,
            g,
            id as u16 as u32,
            this.importance_bias() as u32,
            a17,
            0,
            0x28,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(this.stages() + g as usize * 4, stage);
    exe::crt_free(table);
    read_usize(this.stages() + g as usize * 4)
}

/// AsiaClubChamp::create_group_stages: takes the 8 survivors of the two zones, creates group 1 (the first 4) and group
/// 2 (the next 4) of the Asian Club Championship, sends the group-draw news for each, and sets the current stage
/// (+0x30) to 2; shows an error box and stops if a stage cannot be created.
pub(crate) unsafe extern "thiscall" fn create_group_stages(this: usize) {
    let this = Cup(this);
    let v = group_stage_qualifiers(this);
    let group_1 = [v[0], v[1], v[2], v[3]]; // cqt12: o grupo 1
    let f = create_asia_group(this, 1, group_1.as_ptr(), 2);
    if f == 0 {
        return show_error_at(0x987a40, 0x39e);
    }
    exe::league_send_group_draw_news(f);
    covered!("cqt_12 two asia groups");
    let group_2 = [v[4], v[5], v[6], v[7]]; // cqt12: o grupo 2
    let f = create_asia_group(this, 2, group_2.as_ptr(), 1);
    if f == 0 {
        return show_error_at(0x987a40, 0x3b4);
    }
    exe::league_send_group_draw_news(f);
    this.set_current_stage(2);
}

// simplification: the stages are allocated and the news goes to the managers: does not run twice. Original only; the plan: those alive
// read before; then the stages [+0xc][1] and [+0xc][2] with 4 teams, the cup year and the 4 alive in each (in any order:
// the constructor sorts the table) and +0x30 = 2.

/// The 4 semi-finalists in list order: group 1 winner, group 2 runner-up, group 2 winner, group 1 runner-up.
pub(crate) unsafe fn asia_semi_finalists(this: Cup) -> [usize; 4] {
    let l = |g: usize, p: usize| read_usize(read_usize(read_usize(this.stages() + g * 4) + league::TABLE) + p * league_table_row::SIZE);
    [l(1, 0), l(2, 1), l(2, 0), l(1, 1)] // cqt12: 1st of 1 × 2nd of 2, 1st of 2 × 2nd of 1
}

/// `AsiaClubChamp::create_semi_final_stage()` (`exe::asiaclubchamp_create_semi_final_stage`, thiscall, ret 0, asia_club_champ.cpp; SEH frame only for the
/// stage's new: no destructor on the normal path): builds the 4-team semi-final stage of the Asian Club Championship.
///
/// The list of 4 (`exe::crt_malloc`(0x10): push; add esp, 4 -> cdecl; out of memory, the box for line 0x3d1 and return), with
/// the pairings of asia_semi_finalists; the round table via the virtual +0x3c (push 0; push &id; push &rounds; push 3;
/// mov ecx, this -> thiscall, ret 0x10).
///
/// The stage (`exe::crt_operator_new`(0xb2) push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage `construct_cup_stage`, thiscall, ret 0x3c: the cup, 4,
/// the list, the rounds, +4, the table, the year +0x40, 3, 2, the id, 0x14, 0, 0, 0 and no seed; the original pushes
/// the rounds, the id and the year with garbage in the high 16 bits, which the constructor does not read) goes in
/// [+0xc][3] (stage slot 3); the table is freed (`exe::crt_free`).
///
/// No stage: the box for line 0x3e6 and return (the list is not freed, as in the original). With it: the draw of round
/// 0 (`draw_round` Cup::draw_round: push 0; mov ecx, stage -> thiscall, ret 4) and the list freed.
pub(crate) unsafe extern "thiscall" fn asia_club_champ_create_semi_final_stage(this: usize) {
    let this = Cup(this);
    let list = exe::crt_malloc(0x10);
    if list == 0 {
        return show_error_at(0x987a40, 0x3d1);
    }
    covered!("cqt_12 asia pairings");
    for (k, &c) in asia_semi_finalists(this).iter().enumerate() {
        write_usize(list + k * 4, c);
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), 3, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construct(p, this.addr(), 4, list, rodadas as u16 as u32, read_usize(this.addr() + 4), table, this.year() as u16 as u32, 3, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_usize(this.stages() + 0xc, stage);
    exe::crt_free(table);
    let f = read_usize(this.stages() + 0xc);
    if f == 0 {
        return show_error_at(0x987a40, 0x3e6);
    }
    exe::cup_draw_round(f, 0);
    exe::crt_free(list);
}

// simplification: allocates the stage and draws the round: does not run twice. Original only; the plan: the stage at [+0xc][3] with 4
// teams, the cup’s +4 and the 4 crossed ones (read before) in its list (+0xa7, 6 bytes; as a set: the draw may
// reordenar).

/// One operation of building the round table, in the original routine's order.
///
/// `D` the date of a group round (`league_round_set_date`: round, day, month with 0 = January, years to add, weekday, code, value;
/// records of 0x41 bytes), `L` a match of the group round (`store_round_match`: round, match, a, b, c, value), `S` the draw of a
/// cup round (`cup_set_round_draw_date`: round, day, month, years, weekday; records of 0x68 bytes), `J` the match of the cup round
/// (`cup_set_round_match_date`: round, day, month, years, weekday, time, home side), `E` a day of the tie (`cup_set_round_match_day`: round, slot,
/// weekday, time, code, home side) and `B`/`W`/`X` writing a byte/word/dword at the given offset of the table.
#[derive(Clone, Copy)]
pub(crate) enum AsiaClubChampScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use AsiaClubChampScheduleOp::{B as Cqp11B, D as Cqp11D, E as Cqp11E, J as Cqp11J, L as Cqp11L, S as Cqp11S, W as Cqp11W, X as Cqp11X};

/// Describes one stage of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`).
///
/// It holds the é_liga written (if requested), the rounds, the extra (only if the pointer is not null, when
/// `extra_checa`), the table size, the error-box line when out of memory, the operations and the counter name. `None`:
/// the stage returns 0 without writing anything.
#[derive(Clone, Copy)]
pub(crate) struct ScheduleStage {
    pub(crate) league: i32,
    pub(crate) rounds: i16,
    pub(crate) extra: i16,
    pub(crate) extra_check: bool,
    pub(crate) size: u32,
    pub(crate) row: u32,
    pub(crate) ops: &'static [AsiaClubChampScheduleOp],
    pub(crate) name: &'static str,
}

/// Runs stage description `f` (or returns 0 if there is none): sets the outputs, allocates the round table and applies
/// its operations; returns the table or 0.
///
/// The table is allocated with `exe::crt_malloc` (out of memory, the error box `linha` of source `arquivo` and 0) and the
/// operations are applied in order. The cup's year (+0x40) is passed as a clean 16-bit value to `league_round_set_date` (the original
/// pushes the register with garbage in the high 16 bits; `league_round_set_date` only uses the word) and as signed to the cup
/// routines.
pub(crate) unsafe fn create_round_table(this: Cup, f: Option<ScheduleStage>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, file: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.league;
    }
    *rodadas = f.rounds;
    if !f.extra_check || !extra.is_null() {
        *extra = f.extra;
    }
    let t = exe::crt_malloc(f.size);
    if t == 0 {
        show_error_at(file, f.row);
        return 0;
    }
    covered!(f.name);
    let ano16 = (this.year() as u16) as u32;
    let ano = this.year() as i32 as u32;
    for op in f.ops {
        match *op {
            Cqp11D(r, dia, mes, anos, sem, cod, v) => {
                exe::league_round_set_date(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp11L(r, j, a, b, c, v) => exe::league_round_set_match_slot(t, r, j, a, b, c, v),
            Cqp11S(r, dia, mes, anos, sem) => {
                exe::cup_set_round_draw_date(t, r, dia, mes, anos, sem, ano);
            }
            Cqp11J(r, dia, mes, anos, sem, hora, mando) => {
                exe::cup_set_round_match_date(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp11E(r, vaga, sem, hora, cod, mando) => {
                exe::cup_set_round_match_day(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp11B(o, v) => *((t + o) as *mut u8) = v,
            Cqp11W(o, v) => write_u16(t + o, v),
            Cqp11X(o, v) => write_u32(t + o, v),
        }
    }
    t
}

/// The bytes the operations write in table t, in operation order, for comparison.
pub(crate) unsafe fn written_round_table_bytes(ops: &[AsiaClubChampScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops {
        match *op {
            Cqp11D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqp11L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp11S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp11J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqp11E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqp11B(o, _) => pega(o, 1),
            Cqp11W(o, _) => pega(o, 2),
            Cqp11X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Side-by-side comparer of create_round_table.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables must be equal; the copy's table
/// is freed (`exe::crt_free`).
///
/// In the stage without a table only the original runs (neither writes anything; the outputs may not even exist).
pub(crate) unsafe fn asia_club_champ_compare_round_table(name: &str, stage: u32, f: Option<ScheduleStage>, rounds: *mut i16, extra: *mut i16, league: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            SCOREBOARD.record(a == 0, || format!("{}(stage {}) returned {:#x}", name, stage as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rounds, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if league.is_null() { 0 } else { *league };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if league.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let outs = *rounds == r2 && (extra.is_null() || *extra == e2) && (league.is_null() || *league == l2);
    let table = if a != 0 && b != 0 { written_round_table_bytes(f.ops, a) == written_round_table_bytes(f.ops, b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} ({} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Liga dos Campeões asiática, eliminatórias de cada zona: 1ª rodada sorteada em 1/8 e jogada em 23/8, 2ª em 14/9 e 20/9, ida e volta.
pub(crate) const ASIAN_CHAMPIONS_LEAGUE_ZONE_KNOCKOUT_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0),
];
/// The Asian Champions League final groups: 3 rounds (21, 23 and 25 March of the following year, value 1) with the
/// matches of each round.
pub(crate) const ASIAN_CLUB_CHAMPIONSHIP_GROUP_OPS: [AsiaClubChampScheduleOp; 12] = [
    Cqp11D(0, 0x15, 2, 1, 2, 1, 1), Cqp11L(0, 0, 2, 2, 3, 0x10), Cqp11L(0, 1, 2, 1, 3, 0x10),
    Cqp11L(0, 2, -1, -1, -1, 0), Cqp11D(1, 0x17, 2, 1, 4, 1, 1), Cqp11L(1, 0, 4, 2, 3, 0x10),
    Cqp11L(1, 1, 4, 1, 3, 0x10), Cqp11L(1, 2, -1, -1, -1, 0), Cqp11D(2, 0x19, 2, 1, 6, 1, 1),
    Cqp11L(2, 0, 6, 2, 3, 0x10), Cqp11L(2, 1, 6, 1, 3, 0x10), Cqp11L(2, 2, -1, -1, -1, 0),
];
/// The Asian Champions League final stage in May of the following year: semi-final (with ties `cup_set_round_match_day`), third place
/// and final.
pub(crate) const ASIAN_CHAMPIONS_LEAGUE_FINALS: [AsiaClubChampScheduleOp; 57] = [
    Cqp11S(0, 0x1a, 2, 1, 0), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10),
    Cqp11E(0, 1, 3, 2, 3, 0x10), Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x383), Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2),
    Cqp11W(0xea, 1), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, eliminatórias: 1ª rodada em 23/8, 2ª em 20/9, quartas em 24 de março do ano seguinte, ida e volta.
pub(crate) const ASIAN_CUP_WINNERS_KNOCKOUT_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 1, 7, 0, 5), Cqp11J(0, 0x17, 7, 0, 2, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0), Cqp11B(0x17, 4), Cqp11W(0x18, 0x10), Cqp11W(0x1a, 8), Cqp11W(0x1c, 0x10), Cqp11W(0xf, 0x83),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 0x15), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xe, 8, 0, 3), Cqp11J(1, 0x14, 8, 0, 2, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11W(0x77, 0x83), Cqp11B(0x7f, 4), Cqp11W(0x80, 8), Cqp11W(0x82, 4),
    Cqp11W(0x84, 0), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0x1c, 8, 0, 3), Cqp11J(2, 0x18, 2, 1, 2, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11W(0xdf, 0x83), Cqp11B(0xe7, 4), Cqp11W(0xe8, 4),
    Cqp11W(0xea, 2), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// A Recopa asiática, fase final em maio do ano seguinte: semifinal, terceiro lugar e final (e a cauda comum +0x12c..+0x137).
pub(crate) const ASIAN_CUP_WINNERS_CUP_FINALS: [AsiaClubChampScheduleOp; 57] = [
    Cqp11S(0, 0xf, 3, 1, 6), Cqp11J(0, 0x18, 4, 1, 3, 2, 1), Cqp11E(0, 0, 3, 1, 3, 0x10), Cqp11E(0, 1, 3, 2, 3, 0x10),
    Cqp11E(0, 2, -1, -1, -1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0), Cqp11W(0xd, 0x383),
    Cqp11B(0x17, 6), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0), Cqp11W(0x1e, 0),
    Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0), Cqp11X(0x64, 0),
    Cqp11S(1, 0x19, 4, 1, 4), Cqp11J(1, 0x1a, 4, 1, 5, 2, 0x10), Cqp11W(0x6f, 0x96), Cqp11W(0x71, 1), Cqp11W(0x73, 0),
    Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 6), Cqp11W(0x80, 2), Cqp11W(0x82, 1), Cqp11W(0x84, 0),
    Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0), Cqp11X(0xc8, 0),
    Cqp11X(0xcc, 0), Cqp11S(2, 0x19, 4, 1, 4), Cqp11J(2, 0x1a, 4, 1, 5, 1, 0x10), Cqp11W(0xd7, 0x8c), Cqp11W(0xd9, 2),
    Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 6), Cqp11W(0xe8, 2), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0), Cqp11X(0x12c, 0),
    Cqp11X(0x130, 0), Cqp11X(0x134, 0),
];
/// O LDV Vans Trophy, cada chave: 5 rodadas de dezembro a março do ano seguinte, a última (a final da chave) em ida e volta.
pub(crate) const LDV_VANS_TROPHY_SECTION_OPS: [AsiaClubChampScheduleOp; 90] = [
    Cqp11S(0, 7, 0xa, 0, 5), Cqp11J(0, 0xa, 0xb, 0, 1, 2, 0), Cqp11W(0x7, 0xa), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 4), Cqp11W(0x18, 0x18), Cqp11W(0x1a, 0xc), Cqp11W(0x1c, 0x18), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0xb, 0xb, 0, 2), Cqp11J(1, 0xe, 0, 1, 1, 2, 0), Cqp11W(0x6f, 0x14), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x83), Cqp11W(0x77, 0), Cqp11B(0x7f, 4), Cqp11W(0x80, 0x10), Cqp11W(0x82, 8),
    Cqp11W(0x84, 4), Cqp11W(0x86, 0x18), Cqp11B(0x88, 0), Cqp11B(0x89, 1), Cqp11B(0x8a, 0), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 0xf, 0, 1, 2), Cqp11J(2, 4, 1, 1, 1, 2, 0), Cqp11W(0xd7, 0x78),
    Cqp11W(0xd9, 2), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0x83), Cqp11W(0xdf, 0), Cqp11B(0xe7, 4), Cqp11W(0xe8, 8),
    Cqp11W(0xea, 4), Cqp11W(0xec, 0), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 1), Cqp11B(0xf2, 0),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0), Cqp11X(0x134, 0), Cqp11S(3, 5, 1, 1, 2), Cqp11J(3, 0x12, 1, 1, 1, 2, 0),
    Cqp11W(0x13f, 0x82), Cqp11W(0x141, 3), Cqp11W(0x143, 0), Cqp11W(0x145, 0x83), Cqp11W(0x147, 0), Cqp11B(0x14f, 4),
    Cqp11W(0x150, 4), Cqp11W(0x152, 2), Cqp11W(0x154, 0), Cqp11W(0x156, 0), Cqp11B(0x158, 0), Cqp11B(0x159, 1),
    Cqp11B(0x15a, 0), Cqp11X(0x194, 0), Cqp11X(0x198, 0), Cqp11X(0x19c, 0), Cqp11S(4, 0x13, 1, 1, 2),
    Cqp11J(4, 4, 2, 1, 1, 2, 0), Cqp11W(0x1a7, 0x96), Cqp11W(0x1a9, 4), Cqp11W(0x1ab, 0), Cqp11W(0x1ad, 0),
    Cqp11W(0x1af, 0x83), Cqp11B(0x1b7, 6), Cqp11W(0x1b8, 2), Cqp11W(0x1ba, 1), Cqp11W(0x1bc, 0), Cqp11W(0x1be, 0),
    Cqp11B(0x1c0, 0), Cqp11B(0x1c1, 2), Cqp11B(0x1c2, 7), Cqp11X(0x1fc, 0), Cqp11X(0x200, 0), Cqp11X(0x204, 0),
];
/// O LDV Vans Trophy, a final: sorteio em 12/3 e jogo em 20/4 do ano seguinte, mando 4 (estádio neutro [inferência]).
pub(crate) const LDV_VANS_TROPHY_FINAL: [AsiaClubChampScheduleOp; 18] = [
    Cqp11S(0, 0xc, 2, 1, 3), Cqp11J(0, 0x14, 3, 1, 6, 1, 4), Cqp11W(0x7, 0x96), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x83), Cqp11B(0x17, 6), Cqp11W(0x18, 2), Cqp11W(0x1a, 1), Cqp11W(0x1c, 2), Cqp11W(0xf, 0),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 1), Cqp11B(0x22, 0), Cqp11X(0x5c, 0), Cqp11X(0x60, 0xffff_ffff),
    Cqp11X(0x64, 0),
];

/// The stages of the Asian Champions League [inference: the Asian Club Championship].
///
/// −1 and 0 each zone's qualifiers (2 two-legged rounds in August and September), 1 and 2 the final groups (3 rounds in
/// March), 3 the final stage (semi-final, third place and final in May).
pub(crate) fn asian_club_champ_stage(stage: u32) -> Option<ScheduleStage> {
    let f = stage as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0x480 + (f != -1) as i16, extra_check: false, size: 0xd0, row: 0xc5,
            ops: &ASIAN_CHAMPIONS_LEAGUE_ZONE_KNOCKOUT_OPS, name: "cqp_11 eliminatorias da liga dos campeoes asiatica" }),
        1 | 2 => Some(ScheduleStage { league: 1, rounds: 3, extra: 0x482 + (f != 1) as i16, extra_check: false, size: 0xc3, row: 0x103,
            ops: &ASIAN_CLUB_CHAMPIONSHIP_GROUP_OPS, name: "cqp_11 grupos da liga dos campeoes asiatica" }),
        3 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: true, size: 0x138, row: 0x125,
            ops: &ASIAN_CHAMPIONS_LEAGUE_FINALS, name: "cqp_11 final da liga dos campeoes asiatica" }),
        _ => None,
    }
}

/// rule: the Asian Club Championship has, in each zone (stages −1 and 0, ids 0x480 and 0x481), two two-legged
/// qualifying rounds (1st drawn on 1/8 and played on 23/8, 2nd on 14/9 and 20/9; 21 days between legs in the 1st, 7 in
/// the 2nd); the final groups (stages 1 and 2, ids 0x482 and 0x483) of three rounds on 21, 23 and 25 March of the
/// following year with fixed pairings (matches 0 and 1 of each round); the final stage (stage 3) in May: semi-final on
/// 24/5, third place and final on 26/5, single matches (tie-break 0x383 and 0x83); category 4 in the qualifiers and 6
/// after
/// Original: `AsiaClubChamp::create_round_table` (0x40ae90, asia_club_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_create_round_table(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), asian_club_champ_stage(stage), rounds, extra, league, 0x987a40)
}

/// Stage description of the Asian Cup Winners' Cup: zone qualifiers (-1, 0) and the final stage (1).
pub(crate) fn asian_cup_winners_stage(stage: u32) -> Option<ScheduleStage> {
    let f = stage as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0x480 + (f != -1) as i16, extra_check: false, size: 0x138, row: 0xc6,
            ops: &ASIAN_CUP_WINNERS_KNOCKOUT_OPS, name: "cqp_11 eliminatorias da recopa asiatica" }),
        1 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: true, size: 0x138, row: 0x112,
            ops: &ASIAN_CUP_WINNERS_CUP_FINALS, name: "cqp_11 final da recopa asiatica" }),
        _ => None,
    }
}

/// Stage description of the LDV Vans Trophy: North/South sections (-1, 0) and the final (1).
pub(crate) fn ldv_trophy_stage(stage: u32) -> Option<ScheduleStage> {
    let f = stage as u8 as i8;
    match f {
        -1 | 0 => Some(ScheduleStage { league: 0, rounds: 5, extra: 0x41d + (f != -1) as i16, extra_check: false, size: 0x208, row: 0xca,
            ops: &LDV_VANS_TROPHY_SECTION_OPS, name: "cqp_11 chaves do trofeu" }),
        1 => Some(ScheduleStage { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x13d,
            ops: &LDV_VANS_TROPHY_FINAL, name: "cqp_11 final do trofeu" }),
        _ => None,
    }
}

/// rule: the Asian Club Championship has 16 teams, the qualified clubs (`asia_club_champ_fields::QUALIFIED_CLUBS`) in
/// the order they were picked, all with class and status 0
/// Original: `AsiaClubChamp::build_team_table` (0x40b620, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_build_team_table(this: usize) -> i32 {
    build_team_table(Cup(this), 0x987a40, 0x1cd, "cqq_12 times da liga dos campeoes asiatica", |i| i) // cqq12: the Asian Champions League teams
}

/// The 32 Asian Club Championship qualifiers. Rule: two zones of 16 (east: the Far East and South Asia; west: the
/// Middle East and Central Asia); the last winner takes place 0 or 16 by its nation's zone; the J1 champion (or the
/// runner-up, if the champion is already in) and the Korean league champion join the pool when those leagues are
/// selected; then the Asian clubs, sorted by reputation and shuffled, give one place per nation in its zone while
/// places remain.
///
/// `AsiaClubChamp::select_qualified_clubs()`: +0xb6 = 32 and the 32-place list +0xb2 (`exe::crt_calloc`(0x20, 4):
/// push 4; push 0x20; add esp, 8 -> cdecl; out of memory, the error box of line 0x289 and 0). The latest winner of the
/// cup itself (`exe::comp_util_get_latest_placed_club`([+4], 0), cdecl; its nation id is −1 without club or nation)
/// goes to place 0 if its nation is in the east zone (`ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS`), otherwise to 16. In a local
/// list of up to 1,200 clubs: the J1 club (`asian_club_champ_league_club` with [`KEY_NATION_JAPAN`] and
/// [`KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1`]), then the Korean league's ([`KEY_NATION_SOUTH_KOREA`],
/// [`KEY_CLUB_COMP_KOREAN_LEAGUE`]); and every club of the club table (`G_CLUB_COUNT` − 2 × `G_NATION_COUNT` of them)
/// whose nation's continent is [`KEY_CONTINENT_ASIA`], outside places 0 and 16, while the count (a signed word) is below
/// 0x4b0. The list is sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n − 1, 40), cdecl). In its
/// order, each club whose nation has not had a place yet takes the first free place of its zone (0..15 east
/// (`ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS`), 16..31 west (`ASIAN_CUP_WINNERS_WEST_ZONE_KEYS`, the same 19 nations)), while
/// the zone's last place (15 or 31) is free; the nation is marked even when the zone is full; a club of no zone is
/// skipped. Returns 1. (Stack frame 0x1594.)
/// Original: `AsiaClubChamp::select_qualified_clubs` (0x40b950, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + 0xb6, 0x20);
    let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS[i]) as u32);
    let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
    let list = exe::crt_calloc(0x20, 4);
    write_usize(this + 0xb2, list);
    if list == 0 {
        show_error_at(0x987a40, 0x289);
        return 0;
    }
    let holder = exe::comp_util_get_latest_placed_club(read_usize(this + 4), 0);
    if asian_club_champ_zone_slot(&east, asia_cup_winner_club_nation_id(holder)).is_some() {
        write_usize(list, holder);
    } else {
        write_usize(list + 0x40, holder);
    }
    let mut clubs = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = asian_club_champ_league_club(this, KEY_NATION_JAPAN, KEY_CLUB_COMP_JAPANESE_J_LEAGUE_1) {
        covered!("cqx_1 j1 club");
        clubs[0] = c;
        n = 1;
    }
    if let Some(c) = asian_club_champ_league_club(this, KEY_NATION_SOUTH_KOREA, KEY_CLUB_COMP_KOREAN_LEAGUE) {
        clubs[n] = c;
        n += 1;
    }
    let total = (read_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    let asia = read_usize(KEY_CONTINENT_ASIA);
    let mut c = read_usize(G_CLUB_TABLE);
    for _ in 0..total.max(0) {
        let nation = Nation(read_usize(c + club::NATION));
        if !nation.is_null() {
            let continent = nation.continent();
            if continent != 0 && read_usize(continent) == asia && (n as i16) < 0x4b0 && c != read_usize(list) && c != read_usize(list + 0x40) {
                clubs[n] = c;
                n += 1;
            }
        }
        c += club::SIZE;
    }
    covered!("cqx_1 club draw");
    exe::comp_util_sort_and_shuffle_top_clubs(clubs.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqx1: 40 picks in the shuffle
    let (mut east_marks, mut west_marks) = ([0u8; 23], [0u8; 19]);
    for &club in clubs.iter().take((n as i16).max(0) as usize) {
        let nation = asia_cup_winner_club_nation_id(club);
        let (mark, first) = match asian_club_champ_zone_slot(&east, nation) {
            Some(i) => (&mut east_marks[i], 0usize),
            None => match asian_club_champ_zone_slot(&west, nation) {
                Some(j) => (&mut west_marks[j], 16usize),
                None => continue,
            },
        };
        if *mark != 0 {
            continue;
        }
        *mark = 1;
        if read_usize(list + (first + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (first..first + 16).find(|&k| read_usize(list + k * 4) == 0) {
            write_usize(list + k * 4, club);
        }
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_CLUB_CHAMP_SELECT_QUALIFIED_CLUBS);
    let r = o(this);
    if r == 1 {
        let east: [u32; 23] = core::array::from_fn(|i| read_usize(ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS[i]) as u32);
        let west: [u32; 19] = core::array::from_fn(|i| read_usize(ASIAN_CUP_WINNERS_WEST_ZONE_KEYS[i]) as u32);
        let list = read_usize(this + 0xb2);
        let mut nations: Vec<i32> = Vec::new();
        let mut wrong = 0usize;
        for k in 0..32usize {
            let c = if list != 0 { read_usize(list + k * 4) } else { 0 };
            if c == 0 || k == 0 || k == 16 {
                continue;
            }
            let nation = asia_cup_winner_club_nation_id(c);
            let zone: &[u32] = if k < 16 { &east } else { &west };
            if asian_club_champ_zone_slot(zone, nation).is_none() || nations.contains(&nation) {
                wrong = k;
            }
            nations.push(nation);
        }
        let ok = list != 0 && read_usize(this + 0xb6) == 0x20 && wrong == 0;
        SCOREBOARD.record(ok, || format!("asia_club_champ_select_qualified_clubs({:#x}): list {:#x} +0xb6 {} wrong place {}", this, list, read_usize(this + 0xb6), wrong));
    }
    r
}

/// Stage 0 of the Asian Club Championship: a knockout of the 16 west zone qualifiers (places 16 to 31 of the list
/// +0xb2), on the cup's round table; the same body as the Cup Winners' Cup's (`asia_cup_winner_zone_knockout_stage`),
/// with the error boxes at lines 0x32a (no memory for the teams) and 0x341 (no stage). (The original's SEH frame is only
/// for the stage's new: no destructor on the normal path.)
/// Original: `AsiaClubChamp::create_second_zone_cup_stage` (0x40bec0, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_create_second_zone_cup_stage(this: usize) {
    asia_cup_winner_zone_knockout_stage(this, 0x987a40, (0x32a, 0x341), "cqx_1 west zone team"); // cqx1: the west zone knockout
}

/// Comparer for `asia_club_champ_create_second_zone_cup_stage`: only the original (it allocates the stage), with the
/// plan of the Cup Winners' Cup's (`asia_cup_winner_check_zone_stage`).
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_create_second_zone_cup_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_CREATE_SECOND_ZONE_CUP_STAGE);
    asia_cup_winner_check_zone_stage("asia_club_champ_create_second_zone_cup_stage", this, || o(this))
}

/// The predicted positions of the two Asian Club Championship zones: in each zone, by reputation, 1, 3, 5, 7, four 9ths
/// and eight 17ths; the same body as the Cup Winners' Cup's (`asia_cup_winner_predicted_positions`), with the error
/// boxes at lines 0x45a (no statistics) and 0x462 (no memory).
/// Original: `AsiaClubChamp::rank_teams` (0x40c720, asia_club_champ.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_rank_teams(this: usize) {
    covered!("cqx_1 predicted positions");
    asia_cup_winner_predicted_positions(this, 0x987a40, (0x45a, 0x462)); // cqx1: the predicted positions of the two zones
}

/// Comparer for `asia_club_champ_rank_teams`, side by side over the same entries: the original first (it adds each
/// club's entry to the statistics, Comp_Stats::set_club_final_position), with the plan of the Cup Winners' Cup's
/// (`asia_cup_winner_check_positions`: by reputation 1, 3, 5, 7, four 9ths and eight 17ths in each half); then the
/// copy, which finds the entries already there and only rewrites `comp_stats_club::FINAL_POSITION` (the sort of the
/// same array, the same qsort: the same order); the 32 positions after the copy must equal the original's.
pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_CLUB_CHAMP_RANK_TEAMS);
    let clubs = asian_club_champ_ranked_clubs(this);
    asia_cup_winner_check_positions("asia_club_champ_rank_teams", this, || o(this));
    if let Some(clubs) = clubs {
        let original = asian_club_champ_positions(this, &clubs);
        asia_club_champ_rank_teams(this);
        let copy = asian_club_champ_positions(this, &clubs);
        SCOREBOARD.record(copy == original, || format!("asia_club_champ_rank_teams({:#x}): copy {:?} original {:?}", this, copy, original));
    }
}

/// Rule: the Asian Club Championship records: in the zone knockouts (stage 0), elimination and qualification weigh by
/// round (1 to 80 over the 2 rounds) and the zone winner gets the record of reaching the finals (round 0x78, weight
/// 0x5a); in the stages 1 and 2 the qualified club gets the record of round 0x82 (weight 0x5a); in the finals (stage 3)
/// the winner, the runner-up and the third get the final records.
///
/// `AsiaClubChamp::update_staff_records_for_status(club, outcome, stage, _, fixture, final)` (the virtual +0x44).
/// Stage (signed byte) below 1, weight = `scale_by_round`(1, 0x50, the fixture's `round_number()`, 2): outcome 0xff or
/// 2, the elimination (`european_cup_add_knocked_out_record`: `round_prefix()`, `round_name()`, the weight); 0, the
/// qualification (`european_cup_add_qualified_record`: `round_prefix()`, the name word +0x6f of the fixture's round in
/// the cup's own round table `cup::ROUND_TABLE`, the weight); 1, the qualification (0, 0x78, 0x5a); returns 0. Stages 1
/// and 2: outcome 6, the qualification (0, 0x82, 0x5a), returns 0; otherwise −1. Stage 3: outcome 1 the winner
/// (Staff_Records 0x86a800), 2 the runner-up (0x86b0b0), 3 the third (0x86b710), returns 0; otherwise −1. Above 3: 0.
/// (The high 16 bits of the words the original pushes come from small products of the round: zero.)
/// Original: `AsiaClubChamp::update_staff_records_for_status` (0x40c5b0, asia_club_champ.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, final_: u32) -> u32 {
    let g = Fixture(game);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = read_usize(this + 4);
    if f < 1 {
        match d {
            0xff | 2 => {
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                european_cup_add_knocked_out_record(club, comp, g.round_prefix() as u16 as u32, g.round_name() as u16 as u32, weight);
            }
            0 => {
                covered!("cqx_9 asian club championship zone qualified");
                let weight = scale_by_round(1, 0x50, g.round_number(), 2);
                let name = read_u16(Cup(this).round_table().wrapping_add((g.round_number() as isize * cup_round::SIZE as isize) as usize) + 0x6f) as u32;
                european_cup_add_qualified_record(club, comp, g.round_prefix() as u16 as u32, name, weight);
            }
            1 => european_cup_add_qualified_record(club, comp, 0, 0x78, 0x5a),
            _ => {}
        }
        0
    } else if f < 3 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x82, 0x5a);
            0
        } else {
            u32::MAX
        }
    } else if f == 3 {
        match d {
            1 => cup_record_final_result(0x86a800, club, g, final_),
            2 => cup_record_final_result(0x86b0b0, club, g, final_),
            3 => cup_record_final_result(0x86b710, club, g, final_),
            _ => return u32::MAX,
        }
        0
    } else {
        0
    }
}

pub(crate) unsafe extern "thiscall" fn compare_asia_club_champ_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, a4: u32, game: usize, final_: u32) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let plan = if (1..3).contains(&f) && d != 6 || f == 3 && !(1..=3).contains(&d) { u32::MAX } else { 0 };
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_ASIA_CLUB_CHAMP_UPDATE_STAFF_RECORDS_FOR_STATUS)(this, club, outcome, stage, a4, game, final_);
    SCOREBOARD.record(r == plan, || format!("asia_club_champ_update_staff_records_for_status({:#x}, {}, {}) returned {:#x}, plan {:#x}", club, d, f, r, plan));
    r
}

/// The Asian Club Championship of the game's year, at career creation: 4 stages and a 4,000,000 prize fund; with nothing
/// saved, the 32 qualifiers, the team table, the rounds, the statistics, the second zone cup stage, the tie tree and the
/// predicted positions (`build_asian_club_cup` with `ASIAN_CLUB_CHAMPIONSHIP_PARTS`). The original's SEH frame only
/// matters for exceptions.
/// Original: `AsiaClubChamp::AsiaClubChamp` (0x40aae0, asia_club_champ.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_13 asian club championship");
    let year = year as i16; // cqw13: the Asian Club Championship of the game's year
    build_asian_club_cup(Cup(this), year, record, &ASIAN_CLUB_CHAMPIONSHIP_PARTS)
}

/// Comparer for `construct_asia_club_champ`.
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_club_champ(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_CLUB_CHAMP);
    compare_asian_club_cup_ctor("construct_asia_club_champ", this, year, record, 0x9674d4, || o(this, year, record))
}

/// The index of nation id `n` in a zone (the first equal), as the original's signed-byte loop over the local list.
pub(crate) fn asian_club_champ_zone_slot(zone: &[u32], n: i32) -> Option<usize> {
    zone.iter().position(|&g| g as i32 == n)
}

/// The champion of the league `league_key` (or its runner-up, if the champion is already in place 0 or 16), when the
/// nation `nation_key` has its `nation::LEAGUE_SELECTED` byte set in its [`G_NATION_TABLE`] record; the club is taken
/// as it comes (no null check).
pub(crate) unsafe fn asian_club_champ_league_club(this: usize, nation_key: usize, league_key: usize) -> Option<usize> {
    let n = read_usize(nation_key);
    if read_u8(read_usize(G_NATION_TABLE).wrapping_add(n.wrapping_mul(nation::SIZE)).wrapping_add(nation::LEAGUE_SELECTED)) == 0 {
        return None;
    }
    let league = comp_record_of_key(league_key);
    let list = read_usize(this + 0xb2);
    let c = exe::comp_util_get_latest_placed_club(league, 0); // cqx1: the league champion
    if c == read_usize(list) || c == read_usize(list + 0x40) {
        return Some(exe::comp_util_get_latest_placed_club(league, 1));
    }
    Some(c)
}

/// The clubs of the two halves (the cup's team table and stage 0's), when both tables are there with 16 clubs.
pub(crate) unsafe fn asian_club_champ_ranked_clubs(this: usize) -> Option<Vec<usize>> {
    let stages = read_usize(this + cup::STAGES);
    if read_i16(this + cup::TEAM_COUNT) != 16 || read_usize(this + 8) == 0 || stages == 0 || read_usize(stages) == 0 || read_usize(this + cup::TEAM_TABLE) == 0
        || read_usize(read_usize(stages) + cup::TEAM_TABLE) == 0 {
        return None;
    }
    let mut v: Vec<usize> = (0..16).map(|k| read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6)).collect();
    v.extend((0..16).map(|k| read_usize(read_usize(read_usize(stages) + cup::TEAM_TABLE) + k * 6)));
    if v.contains(&0) { None } else { Some(v) }
}

/// The predicted position (`comp_stats_club::FINAL_POSITION`) of each club in the cup's statistics, 0 without entry.
pub(crate) unsafe fn asian_club_champ_positions(this: usize, clubs: &[usize]) -> Vec<u8> {
    clubs.iter().map(|&c| {
        let e = exe::comp_stats_find_club_entry(read_usize(this + 8), c);
        if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 }
    }).collect()
}

/// The body of the two 32-club Asian cups (0x40aae0 and 0x40cbe0).
///
/// Cup::Cup (`exe::new_cup`, thiscall, ret 0); the competition record at +4, the vtable, the year (`set_year`),
/// `current_round` 0, `stage_index` 0xff, `current_stage` −1, `stage_count` = stages, `comp_type` 1 (cup), `club_sides`
/// 2, +0x1c = +0x20 = −1 (the upper and lower division ids of the Comp base), the qualified club list
/// (`asia_club_champ_fields::QUALIFIED_CLUBS`) 0, `discipline_index` 0x29, 5 `substitutes_named` and 3
/// `substitutions_allowed`. If the saved state loads (Cup::load_from_disk 0x51fc00(this, 1): push 1; mov ecx ->
/// thiscall, ret 4; nonzero), returns this.
///
/// Otherwise: the stage vector (`exe::crt_malloc` of 4 × stages; null when stages ≤ 0), `importance_bias` 0xf,
/// `run_by_stages` 0, the 32 qualified clubs (`select_qualified`; 0: the box `rows[0]`), the team table
/// (`build_team_table`; 0: the box `rows[1]`), the prize fund 4,000,000, the round table (`asian_club_cup_round_table`),
/// the statistics (the box `rows[2]`), the zone cup stage (`create_stage`), the qualified list freed (`exe::crt_free`)
/// and zeroed, the tie tree (Cup::map_fixture_tree 0x518790; 0: the box `rows[3]`) and the predicted positions
/// (`predicted`). Each failing step returns this; all the routines are thiscall, ret 0. Returns this.
pub(crate) unsafe fn build_asian_club_cup(this: Cup, year: i16, record: usize, c: &AsianClubCupParts) -> usize {
    let load_from_disk: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_LOAD_FROM_DISK); // Cup::load_from_disk
    let map_fixture_tree: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_CUP_MAP_FIXTURE_TREE); // Cup::map_fixture_tree
    let routine = |a: usize| -> unsafe extern "thiscall" fn(usize) -> u32 { core::mem::transmute(a) };
    exe::new_cup(this);
    write_u32(this.addr() + 4, record as u32);
    write_u32(this.addr(), c.vtable as u32);
    this.set_year(year);
    this.set_current_round(0);
    this.set_stage_index(stage_index::MAIN);
    write_u32(this.addr() + cup::CURRENT_STAGE, 0xffff_ffff);
    write_u32(this.addr() + cup::STAGE_COUNT, c.stages);
    this.set_comp_type(1);
    this.set_club_sides(2);
    write_u32(this.addr() + 0x1c, 0xffff_ffff);
    write_u32(this.addr() + 0x20, 0xffff_ffff);
    write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    this.set_discipline_index(0x29);
    this.set_substitutes_named(5);
    this.set_substitutions_allowed(3);
    if load_from_disk(this.addr(), 1) != 0 {
        return this.addr();
    }
    let n = read_usize(this.addr() + cup::STAGE_COUNT) as i32;
    write_u32(this.addr() + cup::STAGES, if n > 0 { exe::crt_malloc((n as u32).wrapping_mul(4)) as u32 } else { 0 });
    this.set_importance_bias(0xf);
    this.set_run_by_stages(0);
    if routine(c.select_qualified)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[0]);
        return this.addr();
    }
    if routine(c.build_team_table)(this.addr()) == 0 {
        show_error_at(c.file, c.rows[1]);
        return this.addr();
    }
    this.set_prize_fund(0x3d0900);
    asian_club_cup_round_table(this);
    if !asian_club_cup_comp_stats(this, c.file, c.rows[2]) {
        return this.addr();
    }
    routine(c.create_stage)(this.addr());
    let l = read_usize(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS);
    if l != 0 {
        exe::crt_free(l);
        write_u32(this.addr() + asia_club_champ_fields::QUALIFIED_CLUBS, 0);
    }
    if map_fixture_tree(this.addr()) == 0 {
        show_error_at(c.file, c.rows[3]);
        return this.addr();
    }
    routine(c.predicted)(this.addr());
    this.addr()
}

/// Shared comparer of the Asian club cup constructors. They allocate, draw and create stages, so they cannot run
/// twice: only the original runs, then the plan is checked: it returns this, the vtable, the record at +4 and the year
/// (`cup::YEAR`).
pub(crate) unsafe fn compare_asian_club_cup_ctor(name: &str, this: usize, year: u32, record: usize, vtable: usize, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = Cup(this).year();
    let ok = r == this && read_usize(this) == vtable && read_usize(this + 4) == record && built_year_ok(read as u16, year as u16);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} requested {}, vtable {:#x}", name, this, read, year as i16, read_usize(this)));
    r
}

/// The nation id of the club ([[`club::NATION`]], dword); without a club or a nation, −1.
pub(crate) unsafe fn asia_cup_winner_club_nation_id(c: usize) -> i32 {
    let c = Club(c);
    if c.is_null() || c.nation().addr() == 0 {
        return -1;
    }
    read_usize(c.nation().addr()) as i32
}

/// The knockout stage of one zone of the Asian cups (0x40bec0 and 0x40ddf0, the same body; only the source file and
/// the box lines change): the `cup::TEAM_COUNT` teams (`exe::crt_malloc`(4 × n), cdecl; out of memory, the box
/// `lines.0`) are qualifiers 16.. of the list +0xb2; the round table through `comp_vtable::CREATE_SCHEDULE` (push 0;
/// push &extra; push &rounds; push 0; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xb2)
/// and the Cup_Stage constructor `FN_CUP_STAGE_CUP_STAGE`, thiscall, ret 0x3c, 15 arguments: the cup, n, the teams,
/// the rounds, +4, the table, the year `cup::YEAR`, 0, 2, the extra, `cup::IMPORTANCE_BIAS`, 0, 0, 0, 0; the original
/// pushes the words and the byte with garbage in the high bytes, which the constructor does not read: the copy passes
/// clean values) in [`cup::STAGES`][0]; the teams and the table freed (`exe::crt_free`). Without the stage, the box
/// `lines.1`; with it, `cup::CURRENT_STAGE` = 0.
pub(crate) unsafe fn asia_cup_winner_zone_knockout_stage(this: usize, file: usize, lines: (u32, u32), label: &'static str) {
    #[allow(clippy::type_complexity)]
    let construct_stage: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
    let n = read_i16(this + cup::TEAM_COUNT);
    let teams = exe::crt_malloc((n as i32 as u32).wrapping_mul(4));
    if teams == 0 {
        return show_error_at(file, lines.0);
    }
    for i in 0..n.max(0) as usize {
        covered!(label);
        write_usize(teams + i * 4, read_usize(read_usize(this + 0xb2) + 0x40 + i * 4)); // cqu11: team i of the west zone
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = create_schedule(this, 0, &mut rounds, &mut extra, 0);
    let p = exe::crt_operator_new(0xb2);
    let stage = if p != 0 {
        construct_stage(p, this, n as u16 as u32, teams, rounds as u16 as u32, read_usize(this + 4), table, read_i16(this + cup::YEAR) as u16 as u32, 0, 2,
                        extra as u16 as u32, read_u8(this + cup::IMPORTANCE_BIAS) as u32, 0, 0, 0, 0)
    } else {
        0
    };
    write_usize(read_usize(this + cup::STAGES), stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    if read_usize(read_usize(this + cup::STAGES)) == 0 {
        return show_error_at(file, lines.1);
    }
    write_u32(this + cup::CURRENT_STAGE, 0);
}

pub(crate) unsafe fn asia_cup_winner_check_zone_stage(name: &str, this: usize, o: impl FnOnce()) {
    let list = read_usize(this + 0xb2);
    let n = read_i16(this + cup::TEAM_COUNT).max(0) as usize;
    if list == 0 || n > 16 {
        return o();
    }
    let mut plan: Vec<usize> = (0..n).map(|i| read_usize(list + 0x40 + i * 4)).collect();
    o();
    let f = read_usize(read_usize(this + cup::STAGES));
    if f == 0 {
        return;
    }
    let t = read_usize(f + cup::TEAM_TABLE);
    let mut read: Vec<usize> = if t != 0 { (0..n).map(|i| read_usize(t + i * 6)).collect() } else { Vec::new() };
    read.sort_unstable();
    plan.sort_unstable();
    SCOREBOARD.record(read_usize(this + cup::CURRENT_STAGE) == 0 && read == plan, || format!("{}({:#x}): teams {:x?} plan {:x?} +0x30 {}", name, this, read, plan, read_usize(this + cup::CURRENT_STAGE) as i32));
}

/// The predicted positions of the Asian cups (0x40c720 and 0x40e3b0, the same body): without statistics (+8), the box
/// `lines.0`; an array of 2n clubs (`exe::crt_malloc`(8 × `cup::TEAM_COUNT`); out of memory, the box `lines.1`): the n
/// of the cup's team table (`cup::TEAM_TABLE`, 6-byte rows) and the n of stage 0's team table; each half sorted by
/// reputation (`exe::crt_qsort`(v, n, 4, 0x4b07d0), cdecl); the 32 positions in order
/// (`exe::comp_stats_set_club_final_position`(stats, club, position), thiscall, ret 8): in each half 1, 3, 5, 7, 9, 9,
/// 9, 9 and eight 17s; the array freed.
pub(crate) unsafe fn asia_cup_winner_predicted_positions(this: usize, file: usize, lines: (u32, u32)) {
    if read_usize(this + 8) == 0 {
        return show_error_at(file, lines.0);
    }
    let v = exe::crt_malloc((read_i16(this + cup::TEAM_COUNT) as i32 as u32).wrapping_mul(8));
    if v == 0 {
        return show_error_at(file, lines.1);
    }
    let mut i = 0i16;
    while i < read_i16(this + cup::TEAM_COUNT) {
        let k = i as usize;
        write_usize(v + k * 4, read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6));
        let n = read_i16(this + cup::TEAM_COUNT) as isize as usize;
        write_usize(v + (n + k) * 4, read_usize(read_usize(read_usize(read_usize(this + cup::STAGES)) + cup::TEAM_TABLE) + k * 6));
        i += 1;
    }
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0usize);
    let n = read_i16(this + cup::TEAM_COUNT) as i32;
    exe::crt_qsort(v + (n as usize).wrapping_mul(4), n as u32, 4, 0x4b07d0usize);
    for (k, &p) in ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k * 4), p);
    }
    exe::crt_free(v);
}

pub(crate) unsafe fn asia_cup_winner_check_positions(name: &str, this: usize, o: impl FnOnce()) {
    let n = read_i16(this + cup::TEAM_COUNT);
    let stats = read_usize(this + 8);
    let stages = read_usize(this + cup::STAGES);
    if n != 16 || stats == 0 || stages == 0 || read_usize(stages) == 0 || read_usize(this + cup::TEAM_TABLE) == 0 || read_usize(read_usize(stages) + cup::TEAM_TABLE) == 0 {
        return o();
    }
    let halves: [Vec<usize>; 2] = [
        (0..16).map(|k| read_usize(read_usize(this + cup::TEAM_TABLE) + k * 6)).collect(),
        (0..16).map(|k| read_usize(read_usize(read_usize(stages) + cup::TEAM_TABLE) + k * 6)).collect(),
    ];
    if halves.iter().flatten().any(|&c| c == 0) {
        return o();
    }
    o();
    let mut error = None;
    for (m, half) in halves.iter().enumerate() {
        let mut v: Vec<(i16, u8)> = half
            .iter()
            .map(|&c| {
                let e = exe::comp_stats_find_club_entry(stats, c);
                (read_i16(c + club::REPUTATION), if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) } else { 0 })
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let positions: Vec<u8> = v.iter().map(|x| x.1).collect();
        if positions.iter().zip(ASIAN_CUP_WINNERS_PREDICTED_POSITIONS.iter()).any(|(&a, &b)| a as u32 != b) {
            error = Some((m, positions));
        }
    }
    SCOREBOARD.record(error.is_none(), || format!("{}({:#x}): positions by reputation {:?}", name, this, error));
}

/// Creates the cup's statistics at +8 (operator new `exe::crt_operator_new` of 0x5ce and Comp_Stats::Comp_Stats
/// `exe::new_comp_stats(new, this)`: push this; mov ecx, new -> thiscall, ret 4); when null, the error box `row` of
/// source `file` and false.
pub(crate) unsafe fn asian_club_cup_comp_stats(this: Cup, file: usize, row: u32) -> bool {
    let p = exe::crt_operator_new(0x5ce);
    let e = if p != 0 { exe::new_comp_stats(p, this) } else { 0 };
    write_u32(this.addr() + 8, e as u32);
    if e == 0 {
        show_error_at(file, row);
        return false;
    }
    true
}

/// The cup's round table (virtual `comp_vtable::CREATE_SCHEDULE`(−1, &`cup::ROUND_COUNT`, &`cup::STAGE_LABEL`, 0):
/// push 0; push; push; push −1; mov ecx -> thiscall, ret 0x10) in `round_table()`.
pub(crate) unsafe fn asian_club_cup_round_table(this: Cup) {
    let create_schedule: unsafe extern "thiscall" fn(usize, i32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    write_u32(this.addr() + cup::ROUND_TABLE, create_schedule(this.addr(), -1, this.addr() + cup::ROUND_COUNT, this.addr() + cup::STAGE_LABEL, 0) as u32);
}

/// The Asian Club Championship east zone (23 nations, in the original's order; Pakistan comes last, unlike the Cup
/// Winners' Cup list).
pub(crate) const ASIAN_CLUB_CHAMP_EAST_ZONE_KEYS: [usize; 23] = [KEY_NATION_BANGLADESH, KEY_NATION_BRUNEI_DARUSSALAM, KEY_NATION_CHINA, KEY_NATION_HONG_KONG, KEY_NATION_INDIA,
    KEY_NATION_INDONESIA, KEY_NATION_JAPAN, KEY_NATION_KAMPUCHEA, KEY_NATION_LAOS, KEY_NATION_MACAU, KEY_NATION_MALAYSIA, KEY_NATION_MALDIVES, KEY_NATION_MYANMAR,
    KEY_NATION_NEPAL, KEY_NATION_NORTH_KOREA, KEY_NATION_SINGAPORE, KEY_NATION_SOUTH_KOREA, KEY_NATION_SRI_LANKA, KEY_NATION_TAIWAN, KEY_NATION_THAILAND,
    KEY_NATION_THE_PHILIPPINES, KEY_NATION_VIETNAM, KEY_NATION_PAKISTAN];

/// The Asian Club Championship: vtable 0x9674d4, 4 stages; AsiaClubChamp::select_qualified_clubs (0x40b950),
/// build_team_table (0x40b620), create_second_zone_cup_stage (0x40bec0) and rank_teams (0x40c720, the predicted
/// positions); error boxes of source 0x987a40 at lines 0x4d, 0x54, 0x63 and 0x73.
pub(crate) const ASIAN_CLUB_CHAMPIONSHIP_PARTS: AsianClubCupParts = AsianClubCupParts { vtable: 0x9674d4, stages: 4, file: 0x987a40, select_qualified: 0x40b950, build_team_table: 0x40b620, create_stage: 0x40bec0, predicted: 0x40c720, rows: [0x4d, 0x54, 0x63, 0x73] };

/// The 32 predicted positions, in array order (the two halves).
pub(crate) const ASIAN_CUP_WINNERS_PREDICTED_POSITIONS: [u32; 32] = [1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17, 1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17];
