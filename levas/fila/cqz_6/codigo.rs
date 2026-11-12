// ---------- cqz_6: the round tables of the Asian Cup, the Oceania Nations Cup and the Olympic football (asia_nations.cpp, oceania_nations.cpp, olympics.cpp) ----------

/// cqz_6 asian cup round table, the operations of round table 1 (from the original, in order).
const ASIA_NATIONS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_6 asian cup round table, the operations of round table 2 (from the original, in order).
const ASIA_NATIONS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x40), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 3 (from the original, in order).
const ASIA_NATIONS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(0, 1, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 4 (from the original, in order).
const ASIA_NATIONS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 5, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// cqz_6 asian cup round table, the operations of round table 5 (from the original, in order).
const ASIA_NATIONS_OPS_5: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x15, 9, 0, 5), Cqp12J(0, 0x17, 9, 0, 0, 1, 1), Cqp12E(0, 0, 0, 2, 3, 0x40), Cqp12E(0, 1, 0, 1, 3, 4),
    Cqp12E(0, 2, 1, 2, 3, 0x40), Cqp12E(0, 3, 1, 1, 3, 4), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8), Cqp12W(0x1a, 0x4),
    Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0),
    Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x19, 9, 0, 2), Cqp12J(1, 0x1a, 9, 0, 3, 2, 1),
    Cqp12E(1, 0, 3, 2, 3, 0x40), Cqp12E(1, 1, 3, 1, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4), Cqp12W(0x82, 0x2),
    Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0),
    Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1b, 9, 0, 4), Cqp12J(2, 0x1d, 9, 0, 6, 2, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2),
    Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1),
    Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0), Cqp12S(3, 0x1b, 9, 0, 4),
    Cqp12J(3, 0x1d, 9, 0, 6, 1, 0x40), Cqp12W(0x150, 0x2), Cqp12W(0x141, 0x3), Cqp12W(0x145, 0x3),
    Cqp12W(0x13f, 0x8c), Cqp12W(0x143, 0x0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 0x1), Cqp12W(0x154, 0x0),
    Cqp12W(0x147, 0x0), Cqp12W(0x156, 0x0), Cqp12B(0x158, 0x0), Cqp12B(0x159, 0x1), Cqp12B(0x15a, 0x0),
    Cqp12X(0x194, 0x0), Cqp12X(0x198, 0x0), Cqp12X(0x19c, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 asian cup round table, or none.
fn asia_nations_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x420), extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_1, ops2: &[], name: "cqz_6 asian cup round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x41f, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_2, ops2: &[], name: "cqz_6 asian cup round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x420, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_3, ops2: &[], name: "cqz_6 asian cup round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x421, extra_check: false, size: 0xc3, row: 0x99,
            ops: &ASIA_NATIONS_OPS_4, ops2: &[], name: "cqz_6 asian cup round table" }),
        2 => Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xd3,
            ops: &ASIA_NATIONS_OPS_5, ops2: &[], name: "cqz_6 asian cup round table" }),
        3..=127 => None,
    }
}

/// cqz_6 oceania nations cup round table, the operations of round table 1 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];
/// cqz_6 oceania nations cup round table, the operations of round table 2 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(1, 0, 0, 1, 3, 0x40), Cqp12L(2, 0, 2, 1, 3, 4),
];
/// cqz_6 oceania nations cup round table, the operations of round table 3 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 2, 3, 4), Cqp12L(2, 0, 2, 2, 3, 0x40),
];
/// cqz_6 oceania nations cup round table, the operations of round table 4 (from the original, in order).
const OCEANIA_NATIONS_EVEN_OPS_4: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 0x10, 0xa, 0, 3), Cqp12J(0, 0x11, 0xa, 0, 4, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x40),
    Cqp12E(0, 1, 4, 1, 3, 4), Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0),
    Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x4), Cqp12W(0x1a, 0x2), Cqp12W(0x1c, 0x4), Cqp12W(0xf, 0x0),
    Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0),
    Cqp12X(0x64, 0x0), Cqp12S(1, 0x12, 0xa, 0, 5), Cqp12J(1, 0x13, 0xa, 0, 6, 2, 4), Cqp12W(0x6f, 0x96),
    Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x2), Cqp12W(0x82, 0x1),
    Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0),
    Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x12, 0xa, 0, 5),
    Cqp12J(2, 0x13, 0xa, 0, 6, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3),
    Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0),
    Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0),
    Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table, or none.
fn oceania_nations_even_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_1, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_2, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0xa0,
            ops: &OCEANIA_NATIONS_EVEN_OPS_3, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xcc,
            ops: &OCEANIA_NATIONS_EVEN_OPS_4, ops2: &[], name: "cqz_6 oceania nations cup round table" }),
        2..=127 => None,
    }
}

/// cqz_6 oceania nations cup round table (other years), the operations of round table 1 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 2 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(2, 0, 4, 1, 3, 4),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 3 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 2, 3, 0x40), Cqp12L(1, 0, 2, 2, 3, 4), Cqp12L(2, 0, 4, 2, 3, 0x40),
];
/// cqz_6 oceania nations cup round table (other years), the operations of round table 4 (from the original, in order).
const OCEANIA_NATIONS_OTHER_OPS_4: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 0x18, 5, 0, 5), Cqp12J(0, 0x19, 5, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x4), Cqp12W(0x1a, 0x2), Cqp12W(0x1c, 0x4), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0),
    Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1), Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0),
    Cqp12S(1, 0x1a, 5, 0, 0), Cqp12J(1, 0x1c, 5, 0, 2, 2, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 0x1),
    Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x3), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x2), Cqp12W(0x82, 0x1), Cqp12W(0x84, 0x0),
    Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0), Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0),
    Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0), Cqp12S(2, 0x1a, 5, 0, 0), Cqp12J(2, 0x1c, 5, 0, 2, 1, 0x40),
    Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 0x2), Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2),
    Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0), Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1),
    Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0), Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table (other years), or none.
fn oceania_nations_other_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_1, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_2, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0x11f,
            ops: &OCEANIA_NATIONS_OTHER_OPS_3, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        1 => Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0x14b,
            ops: &OCEANIA_NATIONS_OTHER_OPS_4, ops2: &[], name: "cqz_6 oceania nations cup round table (other years)" }),
        2..=127 => None,
    }
}

/// cqz_6 olympics round table, the operations of round table 1 (from the original, in order).
const OLYMPICS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// cqz_6 olympics round table, the operations of round table 2 (from the original, in order).
const OLYMPICS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 4), Cqp12L(0, 1, 2, 1, 3, 0x80), Cqp12L(1, 0, 5, 1, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// cqz_6 olympics round table, the operations of round table 3 (from the original, in order).
const OLYMPICS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 0x40), Cqp12L(0, 1, 2, 1, 3, 0x100), Cqp12L(1, 0, 5, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 1, 3, 0x100), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x100),
];
/// cqz_6 olympics round table, the operations of round table 4 (from the original, in order).
const OLYMPICS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 0x40), Cqp12L(0, 1, 3, 1, 3, 0x100), Cqp12L(1, 0, 6, 1, 3, 0x40),
    Cqp12L(1, 1, 6, 1, 3, 0x100), Cqp12L(2, 0, 2, 1, 3, 0x40), Cqp12L(2, 1, 2, 1, 3, 0x100),
];
/// cqz_6 olympics round table, the operations of round table 5 (from the original, in order).
const OLYMPICS_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 1, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 1, 3, 0x80),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 1, 3, 0x80),
];
/// cqz_6 olympics round table, the operations of round table 6 (from the original, in order).
const OLYMPICS_OPS_6: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x15, 8, 0, 3), Cqp12J(0, 0x17, 8, 0, 5, 1, 0), Cqp12E(0, 0, 5, 2, 3, 0x100),
    Cqp12E(0, 1, 5, 2, 3, 0x80), Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 5, 2, 3, 4), Cqp12E(0, 4, -1, -1, -1, 0),
    Cqp12W(0x7, 0x78), Cqp12W(0x9, 0x0), Cqp12W(0xb, 0x0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 0x8),
    Cqp12W(0x1a, 0x4), Cqp12W(0x1c, 0x8), Cqp12W(0xf, 0x0), Cqp12W(0x1e, 0x0), Cqp12B(0x20, 0x0), Cqp12B(0x21, 0x1),
    Cqp12B(0x22, 0x0), Cqp12X(0x5c, 0x0), Cqp12X(0x60, 0x0), Cqp12X(0x64, 0x0), Cqp12S(1, 0x18, 8, 0, 6),
    Cqp12J(1, 0x1a, 8, 0, 1, 1, 0), Cqp12E(1, 0, 2, 2, 3, 0x40), Cqp12E(1, 1, 2, 1, 3, 4),
    Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82), Cqp12W(0x71, 0x1), Cqp12W(0x73, 0x0), Cqp12W(0x75, 0x303),
    Cqp12B(0x7f, 0xa), Cqp12W(0x80, 0x4), Cqp12W(0x82, 0x2), Cqp12W(0x84, 0x0), Cqp12W(0x77, 0x0), Cqp12W(0x86, 0x0),
    Cqp12B(0x88, 0x0), Cqp12B(0x89, 0x1), Cqp12B(0x8a, 0x0), Cqp12X(0xc4, 0x0), Cqp12X(0xc8, 0x0), Cqp12X(0xcc, 0x0),
    Cqp12S(2, 0x1c, 8, 0, 3), Cqp12J(2, 0x1e, 8, 0, 5, 1, 4), Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 0x2),
    Cqp12W(0xdb, 0x0), Cqp12W(0xdd, 0x3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 0x2), Cqp12W(0xea, 0x1), Cqp12W(0xec, 0x0),
    Cqp12W(0xdf, 0x0), Cqp12W(0xee, 0x0), Cqp12B(0xf0, 0x0), Cqp12B(0xf1, 0x1), Cqp12B(0xf2, 0x0), Cqp12X(0x12c, 0x0),
    Cqp12X(0x130, 0x0), Cqp12X(0x134, 0x0), Cqp12S(3, 0x1c, 8, 0, 3), Cqp12J(3, 0x1d, 8, 0, 4, 1, 0x40),
    Cqp12W(0x141, 0x3), Cqp12W(0x145, 0x3), Cqp12W(0x13f, 0x8c), Cqp12W(0x143, 0x0), Cqp12B(0x14f, 0xa),
    Cqp12W(0x150, 0x2), Cqp12W(0x152, 0x1), Cqp12W(0x154, 0x0), Cqp12W(0x147, 0x0), Cqp12W(0x156, 0x0),
    Cqp12B(0x158, 0x0), Cqp12B(0x159, 0x1), Cqp12B(0x15a, 0x0), Cqp12X(0x194, 0x0), Cqp12X(0x198, 0x0),
    Cqp12X(0x19c, 0x0),
];

/// The round table of stage `stage` (signed byte) of cqz_6 olympics round table, or none.
fn olympics_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    match f {
        -128..=-2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_1, ops2: &[], name: "cqz_6 olympics round table" }),
        -1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e8, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_2, ops2: &[], name: "cqz_6 olympics round table" }),
        0 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3e9, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_3, ops2: &[], name: "cqz_6 olympics round table" }),
        1 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3ea, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_4, ops2: &[], name: "cqz_6 olympics round table" }),
        2 => Some(GoldcupStagePlan { league: 1, rounds: 3, extra: 0x3eb, extra_check: false, size: 0xc3, row: 0x9c,
            ops: &OLYMPICS_OPS_5, ops2: &[], name: "cqz_6 olympics round table" }),
        3 => Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xde,
            ops: &OLYMPICS_OPS_6, ops2: &[], name: "cqz_6 olympics round table" }),
        4..=127 => None,
    }
}

/// rule: the Asian Cup has three groups (stages −1 to 1, ids 0x41f to 0x421) of three rounds on 13, 16 and 19 October,
/// each with the group's fixed matches; the knockout (stage 2) in single matches: quarter-finals on 23/10 (draw 21/10),
/// semi-finals on 26/10, third place and final on 29/10; stages from 3 on have no table (0).
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `AsiaNations::create_schedule` (0x40eb00, asia_nations.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn asia_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), asia_nations_phase(stage), rounds, extra, league, 0x987ba0)
}

/// Comparer for `asia_nations_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_phase_table("asia_nations_create_schedule", stage, asia_nations_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_nations_create_schedule(this, stage, r, e, l))
}

/// The round table of stage `stage` of the Oceania Nations Cup: the editions of years with remainder 2 by 4 (2002,
/// 2006...) have their own dates (November), the others the June ones.
unsafe fn oceania_nations_phase(this: League, stage: u32) -> Option<GoldcupStagePlan> {
    if (this.year() as i32) % 4 == 2 { // cqz6: the Oceania Nations Cup of 2002, 2006...
        oceania_nations_even_phase(stage)
    } else {
        oceania_nations_other_phase(stage)
    }
}

/// rule: the Oceania Nations Cup has two groups (stages −1 and 0, ids 0x3e8 and 0x3e9) of three rounds and the
/// knockout (stage 1: semi-finals, third place and final). In the years with remainder 2 by 4 (2002...) the groups play
/// on 10, 13 and 15 November and the knockout on 17/11 and 19/11; in the other years, on 19, 21 and 23 June and 25/6
/// and 28/6; the error-box lines differ between the two halves of the original routine. Stages from 2 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `OceaniaNations::create_schedule` (0x798b60, oceania_nations.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), oceania_nations_phase(League(this), stage), rounds, extra, league, 0xa6a0ec)
}

/// Comparer for `oceania_nations_create_schedule`: side by side (`compare_phase_table`).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    compare_phase_table("oceania_nations_create_schedule", stage, oceania_nations_phase(League(this), stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| oceania_nations_create_schedule(this, stage, r, e, l))
}

/// rule: the Olympic football has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 13, 16 and 19
/// September; the knockout (stage 3) in single matches: quarter-finals on 23/9, semi-finals on 26/9, third place on
/// 29/9 and final on 30/9; stages from 4 on have no table.
///
/// Built from the original with the round-table machine of the Copa América (`GoldcupScheduleOp`, `build_phase_table`):
/// each operation is a call of the original (league_round_set_date 0x68a160, league_round_set_match_slot 0x68a1c0,
/// cup_set_round_draw_date 0x521e60, cup_set_round_match_date 0x521eb0, cup_set_round_match_day 0x521f10: pushes; add esp
/// -> cdecl) or a byte/word/dword written into the table, in the original's order (months from 0 = January); the outputs
/// (`rounds`, `extra` always written, `league` only when not null), the size of the table and the error-box line when
/// out of memory are per stage (signed byte), every stage −128..127 checked against the original by emulation.
/// Original: `Olympics::create_schedule` (0x79f7d0, olympics.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn olympics_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), olympics_phase(stage), rounds, extra, league, 0xa6a360)
}

/// Comparer for `olympics_create_schedule`: side by side (`compare_phase_table`: the outputs and the bytes written to the two tables).
pub(crate) unsafe extern "thiscall" fn compare_olympics_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    compare_phase_table("olympics_create_schedule", stage, olympics_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| olympics_create_schedule(this, stage, r, e, l))
}
