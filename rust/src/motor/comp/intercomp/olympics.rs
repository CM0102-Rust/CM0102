use super::*;

/// rule: the Olympic football groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3, two
/// qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 20 of month 2 of the Olympic year
/// on weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules`, with Date::Date(0x14, 2, year, 5). Returns 1.
/// Original: `Olympics::set_up_rules` (0x7a0230, olympics.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn olympics_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 olympics rules");
    let win = 3; // cqz1: the win in the Olympic groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let day = 0x14; // cqz1: the day of the Olympic draw date
    let word = competition_date_word(League(this), day, 2, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `olympics_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_olympics_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OLYMPICS_SET_UP_RULES);
    check_set_up_rules("olympics_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (0x14, 2, 0, 5), 0, || o(this))
}

/// The Olympic football at career creation: the edition is the game's year moved to the next Olympic year (the
/// four-year cycle from 1996: in a 2001 game, 2004); vtable 0x96e1d0, 4 stages, the rules Olympics::set_up_rules
/// (0x7a0230) and the error boxes of lines 0x56 and 0x5d of olympics.cpp (`build_nations_tournament`, the same body as
/// the Asian Cup's).
/// Original: `Olympics::Olympics` (0x79f530, olympics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_olympics(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 olympics");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Olympic year
    build_nations_tournament(League(this), edition, record, 0x96e1d0, 4, 0x7a0230, 0xa6a360, (0x56, 0x5d))
}

/// Comparer for `construct_olympics` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_construct_olympics(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_OLYMPICS);
    compare_nations_tournament_ctor("construct_olympics", League(this), record, 0x96e1d0, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// rule: on the draw day of the Olympic football (20 March of the Olympic year: day 0x14 of month 2, months from 0) the finalists are drawn into four
/// groups, group A gets its table and fixtures and groups B to D their stages; before the draw day nothing runs, not
/// even the league day (`nations_tournament_day`)
/// Original: `Olympics::update_for_date` (0x7a11c0, olympics.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn olympics_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 olympics day");
    nations_tournament_day(League(this), today, full, &OLYMPICS_DAY)
}

/// Comparer for `olympics_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_olympics_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OLYMPICS_UPDATE_FOR_DATE);
    compare_nations_tournament_day("olympics_update_for_date", League(this), today, full, &OLYMPICS_DAY, || o(this, today, full))
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OLYMPICS_CREATE_SCHEDULE);
    compare_phase_table("olympics_create_schedule", stage, olympics_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| olympics_create_schedule(this, stage, r, e, l))
}

/// The round table of stage `stage` (signed byte) of cqz_6 olympics round table, or none.
pub(crate) fn olympics_phase(stage: u32) -> Option<GoldcupStagePlan> {
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

/// The Olympic day: the date test even without `full`, and the draw only on a day without `full` (a quirk of the
/// original, as in the Confederations Cup); the draw of the finalists (Olympics::draw_finalists 0x7a09e0), group A's
/// table (Olympics::init_group_a_table 0x7a0140) and the stages of groups B to D (Olympics::create_group_stage 0x7a05d0
/// with 0, 1 and 2); the boxes of lines 0x41b, 0x421 and 0x428 of olympics.cpp; the count +0xf2 zeroed as a dword.
pub(crate) const OLYMPICS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x7a09e0, group_table: 0x7a0140,
    file: 0xa6a360, rows: [0x41b, 0x421, 0x428], group_stage: 0x7a05d0, group_stages: Some(3), count_as_dword: true };

/// cqz_6 olympics round table, the operations of round table 1 (from the original, in order).
pub(crate) const OLYMPICS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];

/// cqz_6 olympics round table, the operations of round table 2 (from the original, in order).
pub(crate) const OLYMPICS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 4), Cqp12L(0, 1, 2, 1, 3, 0x80), Cqp12L(1, 0, 5, 1, 3, 4), Cqp12L(1, 1, 5, 1, 3, 0x80),
    Cqp12L(2, 0, 1, 1, 3, 4), Cqp12L(2, 1, 1, 1, 3, 0x80),
];

/// cqz_6 olympics round table, the operations of round table 3 (from the original, in order).
pub(crate) const OLYMPICS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 2, 1, 3, 0x40), Cqp12L(0, 1, 2, 1, 3, 0x100), Cqp12L(1, 0, 5, 1, 3, 0x40),
    Cqp12L(1, 1, 5, 1, 3, 0x100), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x100),
];

/// cqz_6 olympics round table, the operations of round table 4 (from the original, in order).
pub(crate) const OLYMPICS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 0x40), Cqp12L(0, 1, 3, 1, 3, 0x100), Cqp12L(1, 0, 6, 1, 3, 0x40),
    Cqp12L(1, 1, 6, 1, 3, 0x100), Cqp12L(2, 0, 2, 1, 3, 0x40), Cqp12L(2, 1, 2, 1, 3, 0x100),
];

/// cqz_6 olympics round table, the operations of round table 5 (from the original, in order).
pub(crate) const OLYMPICS_OPS_5: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 8, 0, 2, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 8, 0, 5, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 8, 0, 1, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 1, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 1, 3, 0x80),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 1, 3, 0x80),
];

/// cqz_6 olympics round table, the operations of round table 6 (from the original, in order).
pub(crate) const OLYMPICS_OPS_6: [GoldcupScheduleOp; 80] = [
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
