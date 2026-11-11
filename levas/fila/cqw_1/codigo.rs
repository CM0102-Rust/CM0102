// ---------- %%LEVA%%a leva: os calendários Baiano, Centro-Oeste, Gaúcho, Goiano (as datas e os jogos de cada rodada da fase de pontos corridos e do mata-mata) (bra_reg_bahia.cpp, bra_reg_central.cpp, bra_reg_gaucho.cpp, bra_reg_goias.cpp) ----------
/// The Bahia state league, the round-robin stage: 18 rounds (from 11 January to 20 April), 1170 bytes, 64 operations.
const BAHIA_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 64] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0xe, 2, 0, 2, 2, 0),
    Cqp11L(6, 0, -1, -1, -1, 0), Cqp11D(7, 0x12, 2, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0),
    Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 0x19, 2, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0),
    Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 1, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0),
    Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 8, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0),
    Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0),
    Cqp11D(0xc, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0),
    Cqp11L(0xc, 2, -1, -1, -1, 0), Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0),
    Cqp11L(0xd, 1, 5, 1, 1, 0), Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xe, 0, 5, 1, 1, 0), Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
    Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0), Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0),
    Cqp11L(0xf, 2, -1, -1, -1, 0), Cqp11D(0x10, 0x10, 4, 0, 2, 2, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
    Cqp11D(0x11, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x11, 0, -1, -1, -1, 0),
];

/// The Bahia state league, stage 0: 2 rounds (from 21 April to 10 May), 208 bytes, 36 operations.
const BAHIA_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x249f0), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the Bahia round table: the round-robin stage (-1) (a league): 18 rounds, from 11 January to 20 April; stage 0
/// (knockout or final stage): 2 rounds, from 21 April to 10 May.
fn bra_reg_bahia_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 18, extra: 0, extra_check: false, size: 0x492, row: 0xa8,
            ops: &BAHIA_LEAGUE_STAGE_OPS, name: "cqw_1 bahia league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x126,
            ops: &BAHIA_STAGE_0_OPS, name: "cqw_1 bahia stage 0" }),
        _ => None,
    }
}

/// The Bahia state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_bahia_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegBahia::create_schedule` (0x42f5d0, bra_reg_bahia.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_bahia_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_bahia_schedule_stage(stage), rounds, extra, league, 0x98acac)
}

unsafe extern "thiscall" fn compare_bra_reg_bahia_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    asia_club_champ_compare_round_table("bra_reg_bahia_create_schedule", stage, bra_reg_bahia_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_bahia_create_schedule(this, stage, r, e, l))
}

/// The Centre-West state league, the round-robin stage: 15 rounds (from 11 January to 6 April), 975 bytes, 56 operations.
const CENTRAL_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 56] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x19, 1, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 4, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0xb, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x12, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x19, 2, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 1, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 4, 3, 0, 2, 2, 0), Cqp11L(8, 0, -1, -1, -1, 0), Cqp11D(9, 8, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0),
    Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0),
    Cqp11D(0xb, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0),
    Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 0x19, 3, 0, 2, 2, 0), Cqp11L(0xc, 0, -1, -1, -1, 0),
    Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0),
    Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0), Cqp11L(0xe, 0, 5, 1, 1, 0),
    Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0),
];

/// The Centre-West state league, stage 0: 3 rounds (from 7 April to 10 May), 312 bytes, 54 operations.
const CENTRAL_STAGE_0_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 7, 4, 0, 0), Cqp11J(0, 0xd, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x78), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x15, 4, 0, 0), Cqp11J(1, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x6f, 0x82), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 8), Cqp11W(0x80, 4), Cqp11W(0x82, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 4, 5, 0, 0), Cqp11J(2, 0xa, 5, 0, 6, 1, 0), Cqp11W(0xd9, 2),
    Cqp11W(0xe8, 2), Cqp11W(0xd7, 0x96), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11B(0xe7, 8), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xdf, 0x804), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0x3d090), Cqp11X(0x134, 0x186a0),
];

/// The stages of the Centre-West round table: the round-robin stage (-1) (a league): 15 rounds, from 11 January to 6 April; stage 0
/// (knockout or final stage): 3 rounds, from 7 April to 10 May.
fn bra_reg_central_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 15, extra: 0, extra_check: false, size: 0x3cf, row: 0xa4,
            ops: &CENTRAL_LEAGUE_STAGE_OPS, name: "cqw_1 centre-west league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x114,
            ops: &CENTRAL_STAGE_0_OPS, name: "cqw_1 centre-west stage 0" }),
        _ => None,
    }
}

/// The Centre-West state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_central_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegCentral::create_schedule` (0x4307b0, bra_reg_central.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_central_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_central_schedule_stage(stage), rounds, extra, league, 0x98ae10)
}

unsafe extern "thiscall" fn compare_bra_reg_central_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    asia_club_champ_compare_round_table("bra_reg_central_create_schedule", stage, bra_reg_central_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_central_create_schedule(this, stage, r, e, l))
}

/// The Rio Grande do Sul (Gaúcho) state league, the round-robin stage: 17 rounds (from 11 January to 20 April), 1105 bytes, 62 operations.
const GAUCHO_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 62] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x15, 1, 0, 2, 2, 0), Cqp11L(2, 0, -1, -1, -1, 0), Cqp11D(3, 0x19, 1, 0, 6, 1, 0),
    Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0), Cqp11D(4, 4, 2, 0, 6, 1, 0),
    Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0), Cqp11D(5, 0xb, 2, 0, 6, 1, 0),
    Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0), Cqp11D(6, 0x12, 2, 0, 6, 1, 0),
    Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0), Cqp11D(7, 0x19, 2, 0, 6, 1, 0),
    Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0), Cqp11D(8, 1, 3, 0, 6, 1, 0),
    Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0), Cqp11D(9, 8, 3, 0, 6, 1, 0),
    Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0), Cqp11D(0xa, 0xf, 3, 0, 6, 1, 0),
    Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0), Cqp11L(0xa, 2, -1, -1, -1, 0),
    Cqp11D(0xb, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0), Cqp11L(0xb, 1, 5, 1, 1, 0),
    Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 0x19, 3, 0, 6, 1, 0), Cqp11L(0xc, 0, -1, -1, -1, 0),
    Cqp11D(0xd, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xd, 0, 5, 1, 1, 0), Cqp11L(0xd, 1, 5, 1, 1, 0),
    Cqp11L(0xd, 2, -1, -1, -1, 0), Cqp11D(0xe, 6, 4, 0, 6, 1, 0), Cqp11L(0xe, 0, 5, 1, 1, 0),
    Cqp11L(0xe, 1, 5, 1, 1, 0), Cqp11L(0xe, 2, -1, -1, -1, 0), Cqp11D(0xf, 0xd, 4, 0, 6, 1, 0),
    Cqp11L(0xf, 0, 5, 1, 1, 0), Cqp11L(0xf, 1, 5, 1, 1, 0), Cqp11L(0xf, 2, -1, -1, -1, 0),
    Cqp11D(0x10, 0x14, 4, 0, 6, 1, 0), Cqp11L(0x10, 0, -1, -1, -1, 0),
];

/// The Rio Grande do Sul (Gaúcho) state league, stage 0: 2 rounds (from 21 April to 10 May), 208 bytes, 36 operations.
const GAUCHO_STAGE_0_OPS: [AsiaClubChampScheduleOp; 36] = [
    Cqp11S(0, 0x15, 4, 0, 6), Cqp11J(0, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x82), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 4), Cqp11W(0x1a, 2), Cqp11W(0x1c, 4), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 4, 5, 0, 0), Cqp11J(1, 0xa, 5, 0, 6, 1, 0), Cqp11W(0x71, 1), Cqp11W(0x82, 1),
    Cqp11W(0x6f, 0x96), Cqp11W(0x73, 0), Cqp11W(0x75, 0), Cqp11B(0x7f, 8), Cqp11W(0x80, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0x3d090), Cqp11X(0xcc, 0x186a0),
];

/// The stages of the Rio Grande do Sul (Gaúcho) round table: the round-robin stage (-1) (a league): 17 rounds, from 11 January to 20 April; stage 0
/// (knockout or final stage): 2 rounds, from 21 April to 10 May.
fn bra_reg_gaucho_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 17, extra: 0, extra_check: false, size: 0x451, row: 0xa8,
            ops: &GAUCHO_LEAGUE_STAGE_OPS, name: "cqw_1 gaucho league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 2, extra: 0, extra_check: false, size: 0xd0, row: 0x122,
            ops: &GAUCHO_STAGE_0_OPS, name: "cqw_1 gaucho stage 0" }),
        _ => None,
    }
}

/// The Rio Grande do Sul (Gaúcho) state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_gaucho_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegGaucho::create_schedule` (0x431ac0, bra_reg_gaucho.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, 0x98ae50)
}

unsafe extern "thiscall" fn compare_bra_reg_gaucho_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    asia_club_champ_compare_round_table("bra_reg_gaucho_create_schedule", stage, bra_reg_gaucho_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_gaucho_create_schedule(this, stage, r, e, l))
}

/// The Goiás state league, the round-robin stage: 13 rounds (from 11 January to 6 April), 845 bytes, 52 operations.
const GOIAS_LEAGUE_STAGE_OPS: [AsiaClubChampScheduleOp; 52] = [
    Cqp11D(0, 0xb, 1, 0, 6, 1, 0), Cqp11L(0, 0, 5, 1, 1, 0), Cqp11L(0, 1, 5, 1, 1, 0), Cqp11L(0, 2, -1, -1, -1, 0),
    Cqp11D(1, 0x12, 1, 0, 6, 2, 0), Cqp11L(1, 0, 5, 1, 1, 0), Cqp11L(1, 1, 5, 1, 1, 0), Cqp11L(1, 2, -1, -1, -1, 0),
    Cqp11D(2, 0x19, 1, 0, 6, 1, 0), Cqp11L(2, 0, 5, 1, 1, 0), Cqp11L(2, 1, 5, 1, 1, 0), Cqp11L(2, 2, -1, -1, -1, 0),
    Cqp11D(3, 4, 2, 0, 6, 1, 0), Cqp11L(3, 0, 5, 1, 1, 0), Cqp11L(3, 1, 5, 1, 1, 0), Cqp11L(3, 2, -1, -1, -1, 0),
    Cqp11D(4, 0xb, 2, 0, 6, 1, 0), Cqp11L(4, 0, 5, 1, 1, 0), Cqp11L(4, 1, 5, 1, 1, 0), Cqp11L(4, 2, -1, -1, -1, 0),
    Cqp11D(5, 0x12, 2, 0, 6, 1, 0), Cqp11L(5, 0, 5, 1, 1, 0), Cqp11L(5, 1, 5, 1, 1, 0), Cqp11L(5, 2, -1, -1, -1, 0),
    Cqp11D(6, 0x19, 2, 0, 6, 1, 0), Cqp11L(6, 0, 5, 1, 1, 0), Cqp11L(6, 1, 5, 1, 1, 0), Cqp11L(6, 2, -1, -1, -1, 0),
    Cqp11D(7, 1, 3, 0, 6, 1, 0), Cqp11L(7, 0, 5, 1, 1, 0), Cqp11L(7, 1, 5, 1, 1, 0), Cqp11L(7, 2, -1, -1, -1, 0),
    Cqp11D(8, 8, 3, 0, 6, 1, 0), Cqp11L(8, 0, 5, 1, 1, 0), Cqp11L(8, 1, 5, 1, 1, 0), Cqp11L(8, 2, -1, -1, -1, 0),
    Cqp11D(9, 0xf, 3, 0, 6, 1, 0), Cqp11L(9, 0, 5, 1, 1, 0), Cqp11L(9, 1, 5, 1, 1, 0), Cqp11L(9, 2, -1, -1, -1, 0),
    Cqp11D(0xa, 0x16, 3, 0, 6, 1, 0), Cqp11L(0xa, 0, 5, 1, 1, 0), Cqp11L(0xa, 1, 5, 1, 1, 0),
    Cqp11L(0xa, 2, -1, -1, -1, 0), Cqp11D(0xb, 0x1d, 3, 0, 6, 1, 0), Cqp11L(0xb, 0, 5, 1, 1, 0),
    Cqp11L(0xb, 1, 5, 1, 1, 0), Cqp11L(0xb, 2, -1, -1, -1, 0), Cqp11D(0xc, 6, 4, 0, 6, 1, 0),
    Cqp11L(0xc, 0, 5, 1, 1, 0), Cqp11L(0xc, 1, 5, 1, 1, 0), Cqp11L(0xc, 2, -1, -1, -1, 0),
];

/// The Goiás state league, stage 0: 3 rounds (from 7 April to 10 May), 312 bytes, 54 operations.
const GOIAS_STAGE_0_OPS: [AsiaClubChampScheduleOp; 54] = [
    Cqp11S(0, 7, 4, 0, 0), Cqp11J(0, 0xd, 4, 0, 6, 1, 0), Cqp11W(0x7, 0x78), Cqp11W(0x9, 0), Cqp11W(0xb, 0),
    Cqp11W(0xd, 0x200), Cqp11B(0x17, 8), Cqp11W(0x18, 8), Cqp11W(0x1a, 4), Cqp11W(0x1c, 8), Cqp11W(0xf, 0x804),
    Cqp11W(0x1e, 0), Cqp11B(0x20, 0), Cqp11B(0x21, 2), Cqp11B(0x22, 7), Cqp11X(0x5c, 0), Cqp11X(0x60, 0),
    Cqp11X(0x64, 0), Cqp11S(1, 0x15, 4, 0, 0), Cqp11J(1, 0x1b, 4, 0, 6, 1, 0), Cqp11W(0x6f, 0x82), Cqp11W(0x71, 1),
    Cqp11W(0x73, 0), Cqp11W(0x75, 0x200), Cqp11B(0x7f, 8), Cqp11W(0x80, 4), Cqp11W(0x82, 2), Cqp11W(0x84, 0),
    Cqp11W(0x77, 0x804), Cqp11W(0x86, 0), Cqp11B(0x88, 0), Cqp11B(0x89, 2), Cqp11B(0x8a, 7), Cqp11X(0xc4, 0),
    Cqp11X(0xc8, 0), Cqp11X(0xcc, 0), Cqp11S(2, 4, 5, 0, 0), Cqp11J(2, 0xa, 5, 0, 6, 1, 0), Cqp11W(0xd9, 2),
    Cqp11W(0xe8, 2), Cqp11W(0xd7, 0x96), Cqp11W(0xdb, 0), Cqp11W(0xdd, 0), Cqp11B(0xe7, 8), Cqp11W(0xea, 1),
    Cqp11W(0xec, 0), Cqp11W(0xdf, 0x804), Cqp11W(0xee, 0), Cqp11B(0xf0, 0), Cqp11B(0xf1, 2), Cqp11B(0xf2, 7),
    Cqp11X(0x12c, 0), Cqp11X(0x130, 0x3d090), Cqp11X(0x134, 0x186a0),
];

/// The stages of the Goiás round table: the round-robin stage (-1) (a league): 13 rounds, from 11 January to 6 April; stage 0
/// (knockout or final stage): 3 rounds, from 7 April to 10 May.
fn bra_reg_goias_schedule_stage(stage: u32) -> Option<ScheduleStage> {
    match stage as u8 as i8 {
        -1 => Some(ScheduleStage { league: 1, rounds: 13, extra: 0, extra_check: false, size: 0x34d, row: 0xa4,
            ops: &GOIAS_LEAGUE_STAGE_OPS, name: "cqw_1 goias league" }),
        0 => Some(ScheduleStage { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x10c,
            ops: &GOIAS_STAGE_0_OPS, name: "cqw_1 goias stage 0" }),
        _ => None,
    }
}

/// The Goiás state league schedule (the virtual `comp_vtable::CREATE_SCHEDULE`): the stage from
/// `bra_reg_goias_schedule_stage`, run by the schedule machine of cqp_11 (`create_round_table`: the outputs, the table of
/// 0x41 bytes per round and, for each round, its date (day, month, years after the season's, weekday and code) and its
/// fixtures; the rest of the table is left as allocated); a stage without a table returns 0 and writes nothing.
/// Original: `BraRegGoias::create_schedule` (0x432e50, bra_reg_goias.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn bra_reg_goias_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    create_round_table(Cup(this), bra_reg_goias_schedule_stage(stage), rounds, extra, league, 0x98ae90)
}

unsafe extern "thiscall" fn compare_bra_reg_goias_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I3%%);
    asia_club_champ_compare_round_table("bra_reg_goias_create_schedule", stage, bra_reg_goias_schedule_stage(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| bra_reg_goias_create_schedule(this, stage, r, e, l))
}
