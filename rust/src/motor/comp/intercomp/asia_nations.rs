use super::*;

/// rule: the season honours of the Asian Cup (winner, runner-up, third) are those of its knockout, the third stage
///
/// A tail call: the stage `.stages()`[2] (the knockout) does its own `comp_vtable::RECORD_SEASON_HONOURS` (thiscall,
/// ret 0).
/// Original: `AsiaNations::record_season_honours` (0x40f590, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_record_season_honours(this: usize) {
    let this = League(this);
    covered!("cqz_9 asian cup honours");
    let stage = read_usize(this.stages() + 8); // cqz9: the knockout is the third stage
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    h(stage);
}

pub(crate) unsafe extern "thiscall" fn compare_asia_nations_record_season_honours(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_ASIA_NATIONS_RECORD_SEASON_HONOURS)(this)
}

/// rule: the Asian Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group, national
/// teams, category 10, 7 substitutes and 3 changes; the draw date is 5 of month 5 of the cup year on weekday 5,
/// `.next_draw_year_offset()` 0; then the stage −1 round table (virtual +0x3c).
///
/// Writes the rule bytes and words of `AFRICAN_NATIONS_RULE_FIELDS` (the same values as the African Nations Cup:
/// `+0x43` 2, `+0xc6` 2, `+0xbe` 2, win 3, `+0xc7` 3, `+0x4a` 3, `+0x3c` 1, `+0xc3` 1, category 0xa, `+0xc5` 1,
/// `+0xbf`..`+0xc1` 0, `+0x49` 7), Date::Date(5, 5, year, 5) (0x549f70: lea ecx, local; 4 pushes -> thiscall, ret 0x10;
/// the year pushed with garbage in the high 16 bits, read as a word) into `.next_draw_day()`, `.next_draw_year_offset()`
/// 0 and the round table of stage −1 (virtual +0x3c: push 0; push &+0x3a; push &+0xa9; push −1 -> thiscall, ret 0x10)
/// into `.round_dates()` (`finish_rules_setup`). Returns 1. The SEH frame only covers the local date (no destructor).
/// Original: `AsiaNations::set_up_rules` (0x40f4c0, asia_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn asia_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 asian cup rules");
    let win = 3; // cqz1: the win in the Asian Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 5; // cqz1: the month of the Asian Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `asia_nations_set_up_rules`: original only (virtual +0x3c allocates the round table), then the fields,
/// the date and the return value are checked (`check_set_up_rules`).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_ASIA_NATIONS_SET_UP_RULES);
    check_set_up_rules("asia_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 5, 0, 5), 0, || o(this))
}

/// The Asian Cup at career creation: the edition is the game's year moved to the next year of the four-year cycle from
/// 1996 (1996, 2000, 2004...: in a 2001 game, 2004); vtable 0x967614, 3 stages, the rules AsiaNations::set_up_rules
/// (0x40f4c0) and the error boxes of lines 0x53 and 0x5a of asia_nations.cpp (`build_nations_tournament`: League::League,
/// the record, `club_sides` 4, no upper or lower division, discipline rule 0, importance 0xf; the saved state if it
/// loads; otherwise `league_flags` 0x28, inactive, the main stage, the stage vector, the rules and the statistics).
/// Original: `AsiaNations::AsiaNations` (0x40e860, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqz_3 asian cup");
    let edition = four_year_edition(year as i16, 0x7cc); // cqz3: the Asian Cup of the four-year cycle from 1996
    build_nations_tournament(League(this), edition, record, 0x967614, 3, 0x40f4c0, 0x987ba0, (0x53, 0x5a))
}

/// Comparer for `construct_asia_nations`: only the original runs (it allocates the stages and the statistics), then the
/// plan (`compare_nations_tournament_ctor`: this, vtable, record, edition year and importance).
pub(crate) unsafe extern "thiscall" fn compare_construct_asia_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_ASIA_NATIONS);
    compare_nations_tournament_ctor("construct_asia_nations", League(this), record, 0x967614, four_year_edition(year as i16, 0x7cc), || o(this, year, record))
}

/// rule: on the draw day of the Asian Cup (5 June of the cup year: day 5 of month 5, months from 0) the finalists are drawn into three groups, group A
/// gets its table and fixtures and the other two groups their stages (`nations_tournament_day`)
/// Original: `AsiaNations::update_for_date` (0x410170, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 asian cup day");
    nations_tournament_day(League(this), today, full, &ASIA_NATIONS_DAY)
}

/// Comparer for `asia_nations_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_ASIA_NATIONS_UPDATE_FOR_DATE);
    compare_nations_tournament_day("asia_nations_update_for_date", League(this), today, full, &ASIA_NATIONS_DAY, || o(this, today, full))
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_ASIA_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("asia_nations_create_schedule", stage, asia_nations_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| asia_nations_create_schedule(this, stage, r, e, l))
}

/// The edition year of a tournament held every four years: from `start`, goes up one year at a time while the remainder
/// of (year − `base`) by 4 is not 0 (signed, as the original's `and 0x80000003` with its sign adjustment).
pub(crate) fn four_year_edition(start: i16, base: i32) -> i16 {
    let mut year = start;
    while (year as i32 - base) % 4 != 0 {
        year = year.wrapping_add(1);
    }
    year
}

/// The day of a national-team tournament with a draw, groups and knockout (the virtual `comp_vtable::UPDATE_FOR_DATE`;
/// the SEH frame only covers the local dates).
///
/// With `light_day_first` and no `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov
/// ecx -> thiscall, ret 8). Otherwise the draw date (`oceania_nations_draw_date`: 0x549ef0 on `.next_draw_day()` and
/// `.next_draw_year_offset()` + `.year()`): before it (today's year lower, unsigned; or the same year and today's day
/// lower, signed) returns 0 without the league day. On the draw day, inactive (`league::ACTIVE` 0) and, with
/// `draw_needs_light_day`, only without `full`: the draw, the predicted positions (the virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group table (0: the box of `rows[0]` and 0), the
/// fixtures (`exe::league_create_fixtures`; 0: the box of `rows[1]` and 0), the sorted table (`exe::league_sort_table`
/// (this, 0): push 0 -> thiscall, ret 4; 0: the box of `rows[2]` and 0), the other group stages, the entrant list
/// +0xee freed (`exe::crt_free`: push; add esp -> cdecl) and zeroed, its count +0xf2 zeroed and `league::ACTIVE` = 1.
/// Then the league day with `full`, whose return value (al; the callers read the byte) it passes on.
pub(crate) unsafe fn nations_tournament_day(this: League, today: usize, full: u32, plan: &NationsDayPlan) -> u32 {
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    if plan.light_day_first && full == 0 {
        return exe::league_update_for_date(this, today, 0) as u32;
    }
    let (day, year) = oceania_nations_draw_date(this);
    let (today_day, today_year) = (read_u16(today), read_u16(today + 2));
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) {
        let draw: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.draw);
        let group_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.group_table);
        draw(this.addr());
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if group_table(this.addr()) == 0 {
            show_error_at(plan.file, plan.rows[0]);
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(plan.file, plan.rows[1]);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(plan.file, plan.rows[2]);
            return 0;
        }
        match plan.group_stages {
            Some(n) => {
                let stage: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(plan.group_stage);
                for k in 0..n {
                    stage(this.addr(), k);
                }
            }
            None => {
                let stage: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(plan.group_stage);
                stage(this.addr());
            }
        }
        exe::crt_free(read_usize(this.addr() + 0xee));
        write_u32(this.addr() + 0xee, 0);
        if plan.count_as_dword {
            write_u32(this.addr() + 0xf2, 0);
        } else {
            write_u16(this.addr() + 0xf2, 0);
        }
        this.set_active(1);
    }
    exe::league_update_for_date(this, today, full) as u32
}

/// Shared comparer: the draw, the tables, the fixtures and the league day cannot run twice, so only the original runs;
/// on the draw day (the copy's condition) the plan: `league::ACTIVE` = 1, the list +0xee and its count zeroed, the
/// first group table and the first group stage created.
pub(crate) unsafe fn compare_nations_tournament_day(name: &str, this: League, today: usize, full: u32, plan: &NationsDayPlan, o: impl FnOnce() -> u32) -> u32 {
    let draws = !(plan.light_day_first && full == 0) && this.active() == 0 && (!plan.draw_needs_light_day || full == 0) && {
        let (day, year) = oceania_nations_draw_date(this);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o();
    if draws {
        let stages = this.stages();
        let count = if plan.count_as_dword { read_u32(this.addr() + 0xf2) } else { read_u16(this.addr() + 0xf2) as u32 };
        let ok = this.active() == 1 && read_usize(this.addr() + 0xee) == 0 && count == 0 && this.table() != 0 && stages != 0 && read_usize(stages) != 0;
        SCOREBOARD.record(ok, || format!("{}({:#x}): the draw did not build the groups (+0x45 {})", name, this.addr(), this.active()));
    }
    r
}

/// The round table of stage `stage` (signed byte) of cqz_6 asian cup round table, or none.
pub(crate) fn asia_nations_phase(stage: u32) -> Option<GoldcupStagePlan> {
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

/// Shared comparer of the three constructors. They allocate the stages and the statistics, so they cannot run twice:
/// only the original runs, then the plan is checked: it returns this, the vtable, the record at +4
/// (`league::COMPETITION`), the edition year and `importance` 0xf.
pub(crate) unsafe fn compare_nations_tournament_ctor(name: &str, this: League, record: usize, vtable: usize, year: i16, o: impl FnOnce() -> usize) -> usize {
    let r = o();
    let read = this.year();
    let ok = r == this.addr() && read_usize(this.addr()) == vtable && this.competition().addr() == record && built_year_ok(read as u16, year as u16) && this.importance() == 0xf;
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} expected {}, vtable {:#x}", name, this.addr(), read, year, read_usize(this.addr())));
    r
}

/// The Asian Cup day: the draw of the finalists (AsiaNations::select_and_draw_teams 0x40fdf0), group A's table
/// (AsiaNations::create_group_table 0x40f3d0) and the stages of the two other groups (AsiaNations::create_group_stage
/// 0x40f8a0 with 0 and 1); the boxes of lines 0x3dd, 0x3e3 and 0x3ea of asia_nations.cpp; the count +0xf2 zeroed as a
/// word; without `full`, the league day at once.
pub(crate) const ASIA_NATIONS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: true, draw_needs_light_day: false, draw: 0x40fdf0, group_table: 0x40f3d0,
    file: 0x987ba0, rows: [0x3dd, 0x3e3, 0x3ea], group_stage: 0x40f8a0, group_stages: Some(2), count_as_dword: false };

/// cqz_6 asian cup round table, the operations of round table 1 (from the original, in order).
pub(crate) const ASIA_NATIONS_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];

/// cqz_6 asian cup round table, the operations of round table 2 (from the original, in order).
pub(crate) const ASIA_NATIONS_OPS_2: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 3, 1, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x40), Cqp12L(1, 0, 6, 1, 3, 4), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x40),
];

/// cqz_6 asian cup round table, the operations of round table 3 (from the original, in order).
pub(crate) const ASIA_NATIONS_OPS_3: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(0, 1, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];

/// cqz_6 asian cup round table, the operations of round table 4 (from the original, in order).
pub(crate) const ASIA_NATIONS_OPS_4: [GoldcupScheduleOp; 12] = [
    Cqp12D(0, 0xd, 9, 0, 4, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0x10, 9, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x13, 9, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 5, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];

/// cqz_6 asian cup round table, the operations of round table 5 (from the original, in order).
pub(crate) const ASIA_NATIONS_OPS_5: [GoldcupScheduleOp; 80] = [
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
