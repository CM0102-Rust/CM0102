// ---------- cqz_7: the round tables of the European Championship and of the Asian and Oceania World Cup qualifying (euro_champ.cpp, wc_asia_league.cpp, wc_oceania_league.cpp) ----------

/// cqz_7 euro round table, the operations of round table 1 (from the original, in order).
const EURO_CHAMP_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_7 euro round table, the operations of round table 2 (from the original, in order).
const EURO_CHAMP_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 2, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x80), Cqp12L(1, 0, 3, 2, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// cqz_7 euro round table, the operations of round table 3 (from the original, in order).
const EURO_CHAMP_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x40), Cqp12L(0, 1, 0, 1, 3, 0x1000), Cqp12L(1, 0, 3, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 2, 3, 0x1000), Cqp12L(2, 0, 1, 2, 3, 0x1000), Cqp12L(2, 1, 1, 2, 3, 0x40),
];
/// cqz_7 euro round table, the operations of round table 4 (from the original, in order).
const EURO_CHAMP_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 2, 3, 0x100), Cqp12L(0, 1, 1, 1, 3, 0x400), Cqp12L(1, 0, 4, 1, 3, 0x100),
    Cqp12L(1, 1, 6, 1, 3, 0x400), Cqp12L(2, 0, 2, 2, 3, 0x400), Cqp12L(2, 1, 2, 2, 3, 0x100),
];
/// cqz_7 euro round table, the operations of round table 5 (from the original, in order).
const EURO_CHAMP_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 9, 5, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xf, 5, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 5, 0, 2, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 6, 1, 3, 0x200), Cqp12L(0, 1, 1, 2, 3, 0x800), Cqp12L(1, 0, 4, 2, 3, 0x200),
    Cqp12L(1, 1, 6, 2, 3, 0x800), Cqp12L(2, 0, 2, 1, 3, 0x200), Cqp12L(2, 1, 2, 1, 3, 0x800),
];
/// cqz_7 euro round table, the operations of round table 6 (from the original, in order).
const EURO_CHAMP_OPS_6: [GoldcupScheduleOp; 62] = [
    Cqp12S(0, 0x14, 5, 0, 3), Cqp12J(0, 0x16, 5, 0, 5, 1, 1), Cqp12E(0, 0, 5, 1, 3, 0x40), Cqp12E(0, 1, 5, 2, 3, 4),
    Cqp12E(0, 2, 6, 2, 3, 0x80), Cqp12E(0, 3, 6, 1, 3, 0x100), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x383), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8), Cqp12W(0x1a, 0x4),
    Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0),
    Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x18, 5, 0, 0), Cqp12J(1, 0x1a, 5, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 2, 3, 0x40), Cqp12E(1, 1, 2, 1, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x383), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4),
    Cqp12W(0x82, 0x2), Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1),
    Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1b, 5, 0, 3),
    Cqp12J(2, 0x1e, 5, 0, 6, 1, 4), Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x283),
    Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0),
    Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0),
    Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 euro round table, or none.
fn euro_champ_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_1, ops2: &[], name: "cqz_7 euro round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_2, ops2: &[], name: "cqz_7 euro round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_3, ops2: &[], name: "cqz_7 euro round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3ea, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_4, ops2: &[], name: "cqz_7 euro round table" }),
        2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3eb, extra_check: false, size: 0xc3, row: 0xb4,
            ops: &EURO_CHAMP_OPS_5, ops2: &[], name: "cqz_7 euro round table" }),
        3 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xf6,
            ops: &EURO_CHAMP_OPS_6, ops2: &[], name: "cqz_7 euro round table" }),
        4..=127 => None,
    }
}

/// cqz_7 asian qualifying round table, the operations of round table 1 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0x1e, 3, 1, 0, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 4, 4, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 7, 4, 1, 0, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 0xb, 4, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0x12, 4, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x19, 4, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
];
/// cqz_7 asian qualifying round table, the operations of round table 2 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 0x11, 7, 1, 4, 2, 0), Cqp12L(0, 0, -1, -1, -1, 0), Cqp12D(1, 0x18, 7, 1, 4, 2, 0),
    Cqp12L(1, 0, -1, -1, -1, 0), Cqp12D(2, 0x1f, 7, 1, 4, 2, 0), Cqp12L(2, 0, -1, -1, -1, 0),
    Cqp12D(3, 7, 8, 1, 4, 2, 0), Cqp12L(3, 0, -1, -1, -1, 0), Cqp12D(4, 0xe, 8, 1, 4, 2, 0),
    Cqp12L(4, 0, -1, -1, -1, 0), Cqp12D(5, 0x15, 8, 1, 4, 2, 0), Cqp12L(5, 0, -1, -1, -1, 0),
    Cqp12D(6, 0x1c, 8, 1, 4, 2, 0), Cqp12L(6, 0, -1, -1, -1, 0), Cqp12D(7, 5, 9, 1, 4, 2, 0),
    Cqp12L(7, 0, -1, -1, -1, 0), Cqp12D(8, 0xc, 9, 1, 4, 2, 0), Cqp12L(8, 0, -1, -1, -1, 0),
    Cqp12D(9, 0x13, 9, 1, 4, 2, 0), Cqp12L(9, 0, -1, -1, -1, 0),
];
/// cqz_7 asian qualifying round table, the operations of round table 3 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x14, 9, 1, 5), Cqp12J(0, 0x1a, 9, 1, 4, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x3), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x0),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];
/// cqz_7 asian qualifying round table, the operations of round table 4 (from the original, in order).
const WC_ASIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1b, 9, 1, 5), Cqp12J(0, 0xb, 0xa, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2),
    Cqp12B(0x21, 0x2), Cqp12W(0x7, 0x0), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa),
    Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0),
    Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 asian qualifying round table, or none.
fn wc_asia_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=8 => Some(GoldcupStagePlan { league: 1, rounds: 6, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x186, row: 0xc0,
            ops: &WC_ASIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        9..=10 => Some(GoldcupStagePlan { league: 1, rounds: 10, extra: (f as i16).wrapping_add(0x3fe), extra_check: false, size: 0x28a, row: 0xea,
            ops: &WC_ASIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        11 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0x1e, extra_check: false, size: 0x68, row: 0x120,
            ops: &WC_ASIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        12 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x14b,
            ops: &WC_ASIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 asian qualifying round table" }),
        13..=127 => None,
    }
}

/// cqz_7 oceania qualifying round table, the operations of round table 1 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_1: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 6, 1, 3, 0x100), Cqp12L(0, 1, 6, 2, 3, 0x100),
    Cqp12L(1, 0, 2, 1, 3, 0x100), Cqp12L(1, 1, 2, 2, 3, 0x100), Cqp12L(2, 0, 5, 1, 3, 0x100),
    Cqp12L(2, 1, 5, 2, 3, 0x100), Cqp12L(3, 0, 0, 1, 3, 0x100), Cqp12L(3, 1, 0, 2, 3, 0x100),
    Cqp12L(4, 0, 2, 1, 3, 0x100), Cqp12L(4, 1, 2, 2, 3, 0x100),
];
/// cqz_7 oceania qualifying round table, the operations of round table 2 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_2: [GoldcupScheduleOp; 20] = [
    Cqp12D(0, 7, 3, 1, 5, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 9, 3, 1, 0, 2, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xb, 3, 1, 2, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12D(3, 0xe, 3, 1, 5, 1, 1), Cqp12L(3, 2, -1, -1, -1, 0), Cqp12D(4, 0x10, 3, 1, 0, 2, 1),
    Cqp12L(4, 2, -1, -1, -1, 0), Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 0x80),
    Cqp12L(1, 0, 1, 1, 3, 0x80), Cqp12L(1, 1, 1, 0, 3, 0x80), Cqp12L(2, 0, 4, 1, 3, 0x80),
    Cqp12L(2, 1, 4, 2, 3, 0x80), Cqp12L(3, 0, 6, 1, 3, 0x80), Cqp12L(3, 1, 6, 2, 3, 0x80),
    Cqp12L(4, 0, 1, 1, 3, 0x80), Cqp12L(4, 1, 1, 2, 3, 0x80),
];
/// cqz_7 oceania qualifying round table, the operations of round table 3 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_3: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x1c, 3, 1, 3), Cqp12J(0, 0x14, 5, 1, 2, 2, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x96),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x4), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];
/// cqz_7 oceania qualifying round table, the operations of round table 4 (from the original, in order).
const WC_OCEANIA_LEAGUE_OPS_4: [GoldcupScheduleOp; 18] = [
    Cqp12S(0, 0x17, 0xa, 1, 3), Cqp12J(0, 2, 0xb, 1, 5, 1, 0), Cqp12W(0x18, 0x2), Cqp12W(0x1c, 0x2), Cqp12W(0x7, 0x0),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x0), Cqp12B(0x17, 0xa), Cqp12W(0x1a, 0x1), Cqp12W(0xf, 0x3),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x2), Cqp12B(0x22, 0x7), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_7 oceania qualifying round table, or none.
fn wc_oceania_league_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e8, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_2, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 5, extra: 0x3e9, extra_check: false, size: 0x145, row: 0xc1,
            ops: &WC_OCEANIA_LEAGUE_OPS_1, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0, extra_check: false, size: 0x68, row: 0x10b,
            ops: &WC_OCEANIA_LEAGUE_OPS_3, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        2 => Some(GoldcupStagePlan { league: 0, rounds: 1, extra: 0xa0, extra_check: false, size: 0x68, row: 0x134,
            ops: &WC_OCEANIA_LEAGUE_OPS_4, ops2: &[], name: "cqz_7 oceania qualifying round table" }),
        3..=127 => None,
    }
}

/// rule: the European Championship has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 9, 15 and
/// 19 June; the knockout (stage 3) in single matches: quarter-finals on 22/6, semi-finals on 26/6 and final on 30/6
/// (no third place); stages from 4 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `EuroChamp::create_schedule` (0x57ab90, euro_champ.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), euro_champ_phase(stage), rounds, extra, league, 0x9c4638)
}

/// Comparer for `euro_champ_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_euro_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_phase_table("euro_champ_create_schedule", stage, euro_champ_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| euro_champ_create_schedule(this, stage, r, e, l))
}

/// rule: the Asian World Cup qualifying has first-round groups (stages up to 8, ids 0x3fe + stage) of six rounds
/// between 30 April and 25 May, second-round groups (stages 9 and 10) of ten rounds between 17 August and 19 October,
/// the two-legged tie of the group runners-up (stage 11: 26 October) and the inter-continental play-off (stage 12:
/// draw 27/10, 11 November); stages from 13 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcAsiaLeague::create_schedule` (0x911f00, wc_asia_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_asia_league_phase(stage), rounds, extra, league, 0xad4680)
}

/// Comparer for `wc_asia_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_asia_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_phase_table("wc_asia_league_create_schedule", stage, wc_asia_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_asia_league_create_schedule(this, stage, r, e, l))
}

/// rule: the Oceania World Cup qualifying has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of five rounds on 7,
/// 9, 11, 14 and 16 April; the tie of the group winners (stage 1: 20 June) and the inter-continental play-off (stage
/// 2: 2 December); stages from 3 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `WcOceaniaLeague::create_schedule` (0x91e2e0, wc_oceania_league.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), wc_oceania_league_phase(stage), rounds, extra, league, 0xad4afc)
}

/// Comparer for `wc_oceania_league_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_wc_oceania_league_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_phase_table("wc_oceania_league_create_schedule", stage, wc_oceania_league_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| wc_oceania_league_create_schedule(this, stage, r, e, l))
}
