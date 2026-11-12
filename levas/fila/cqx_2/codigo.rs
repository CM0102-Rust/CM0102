// ---------- batch %%LEVA%%: the FIFA Club World Championship: the round tables, the rules and the day (world_club_champ.cpp) ----------

/// The World Club Championship: the three rounds of each group (29 July, 2 and 5 August), code 1, value 1, with match
/// 2 empty.
const WORLD_CLUB_CHAMP_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::D(0, 0x1d, 6, 0, 6, 1, 1), GoldcupScheduleOp::L(0, 2, -1, -1, -1, 0), GoldcupScheduleOp::D(1, 2, 7, 0, 3, 1, 1),
    GoldcupScheduleOp::L(1, 2, -1, -1, -1, 0), GoldcupScheduleOp::D(2, 5, 7, 0, 6, 1, 1), GoldcupScheduleOp::L(2, 2, -1, -1, -1, 0),
];
/// The World Club Championship: the matches of the group of stage −1.
const WORLD_CLUB_CHAMP_GROUP_A_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 5, 1, 3, 4), GoldcupScheduleOp::L(0, 1, 6, 2, 3, 0x40), GoldcupScheduleOp::L(1, 0, 2, 1, 3, 4),
    GoldcupScheduleOp::L(1, 1, 2, 2, 3, 0x40), GoldcupScheduleOp::L(2, 0, 5, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 5, 2, 3, 0x40),
];
/// The World Club Championship: the matches of the group of stage 0.
const WORLD_CLUB_CHAMP_GROUP_B_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 6, 1, 3, 4), GoldcupScheduleOp::L(0, 1, 0, 2, 3, 4), GoldcupScheduleOp::L(1, 0, 3, 1, 3, 0x40),
    GoldcupScheduleOp::L(1, 1, 3, 2, 3, 4), GoldcupScheduleOp::L(2, 0, 6, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 6, 2, 3, 0x40),
];
/// The World Club Championship: the matches of the group of stage 1.
const WORLD_CLUB_CHAMP_GROUP_C_MATCHES: [GoldcupScheduleOp; 6] = [
    GoldcupScheduleOp::L(0, 0, 1, 1, 3, 0x40), GoldcupScheduleOp::L(0, 1, 1, 2, 3, 4), GoldcupScheduleOp::L(1, 0, 4, 1, 3, 0x40),
    GoldcupScheduleOp::L(1, 1, 4, 2, 3, 4), GoldcupScheduleOp::L(2, 0, 0, 1, 3, 4), GoldcupScheduleOp::L(2, 1, 0, 2, 3, 0x40),
];
/// The World Club Championship knockout (stage 2), single matches: the semi-finals drawn on 7 August and played on 9
/// August (tie-break 0x83, category 0x17), the final drawn on 10 August and played on 12 August (prizes 6,000,000 and
/// 5,000,000) and the third-place match on 12 August (prizes 4,000,000 and 3,000,000).
const WORLD_CLUB_CHAMP_KNOCKOUT: [GoldcupScheduleOp; 54] = [
    GoldcupScheduleOp::S(0, 7, 7, 0, 1), GoldcupScheduleOp::J(0, 9, 7, 0, 3, 1, 0x40), GoldcupScheduleOp::W(0x7, 0x82),
    GoldcupScheduleOp::W(0x9, 0), GoldcupScheduleOp::W(0xb, 0), GoldcupScheduleOp::W(0xd, 0x83), GoldcupScheduleOp::B(0x17, 0x17),
    GoldcupScheduleOp::W(0x18, 4), GoldcupScheduleOp::W(0x1a, 2), GoldcupScheduleOp::W(0x1c, 4), GoldcupScheduleOp::W(0xf, 0),
    GoldcupScheduleOp::W(0x1e, 0), GoldcupScheduleOp::B(0x20, 0), GoldcupScheduleOp::B(0x21, 1), GoldcupScheduleOp::B(0x22, 0),
    GoldcupScheduleOp::X(0x5c, 0), GoldcupScheduleOp::X(0x60, 0), GoldcupScheduleOp::X(0x64, 0),
    GoldcupScheduleOp::S(1, 0xa, 7, 0, 4), GoldcupScheduleOp::J(1, 0xc, 7, 0, 6, 2, 4), GoldcupScheduleOp::W(0x6f, 0x96),
    GoldcupScheduleOp::W(0x71, 1), GoldcupScheduleOp::W(0x73, 0), GoldcupScheduleOp::W(0x75, 0x83), GoldcupScheduleOp::B(0x7f, 0x17),
    GoldcupScheduleOp::W(0x80, 2), GoldcupScheduleOp::W(0x82, 1), GoldcupScheduleOp::W(0x84, 0), GoldcupScheduleOp::W(0x77, 0),
    GoldcupScheduleOp::W(0x86, 0), GoldcupScheduleOp::B(0x88, 0), GoldcupScheduleOp::B(0x89, 1), GoldcupScheduleOp::B(0x8a, 0),
    GoldcupScheduleOp::X(0xc4, 0), GoldcupScheduleOp::X(0xc8, 0x5b8d80), GoldcupScheduleOp::X(0xcc, 0x4c4b40), // cqx2: the final prizes
    GoldcupScheduleOp::S(2, 0xa, 7, 0, 4), GoldcupScheduleOp::J(2, 0xc, 7, 0, 6, 1, 0x40), GoldcupScheduleOp::W(0xd9, 2),
    GoldcupScheduleOp::W(0xe8, 2), GoldcupScheduleOp::W(0xd7, 0x8c), GoldcupScheduleOp::W(0xdb, 0), GoldcupScheduleOp::W(0xdd, 0x83),
    GoldcupScheduleOp::B(0xe7, 0x17), GoldcupScheduleOp::W(0xea, 1), GoldcupScheduleOp::W(0xec, 0), GoldcupScheduleOp::W(0xdf, 0),
    GoldcupScheduleOp::W(0xee, 0), GoldcupScheduleOp::B(0xf0, 0), GoldcupScheduleOp::B(0xf1, 1), GoldcupScheduleOp::B(0xf2, 0),
    GoldcupScheduleOp::X(0x12c, 0), GoldcupScheduleOp::X(0x130, 0x3d0900), GoldcupScheduleOp::X(0x134, 0x2dc6c0),
];

/// The World Club Championship stage description by the signed stage byte (the three groups below 2, the knockout at
/// 2, nothing above).
fn world_club_champ_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f < 2 {
        let games: &'static [GoldcupScheduleOp] = match f {
            -1 => &WORLD_CLUB_CHAMP_GROUP_A_MATCHES,
            0 => &WORLD_CLUB_CHAMP_GROUP_B_MATCHES,
            1 => &WORLD_CLUB_CHAMP_GROUP_C_MATCHES,
            _ => &[],
        };
        Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x99,
            ops: &WORLD_CLUB_CHAMP_GROUP_ROUNDS, ops2: games, name: "cqx_2 world club championship groups" })
    } else if f == 2 {
        Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xd3,
            ops: &WORLD_CLUB_CHAMP_KNOCKOUT, ops2: &[], name: "cqx_2 world club championship knockout" })
    } else {
        None
    }
}

/// Rule: the World Club Championship has three groups of three rounds, single round robin, on 29 July, 2 and 5 August
/// (stages −1, 0 and 1, ids 0x3e8 to 0x3ea), each round with the group's two fixed matches; then the knockout (stage 2)
/// in single matches: semi-finals on 9 August, final and third place on 12 August; category 0x17 (club world cups);
/// prizes 6 and 5 million in the final, 4 and 3 million for the third place.
///
/// `WorldClubChamp::create_schedule(stage, &rounds, &extra, &is_league)` (virtual `comp_vtable::CREATE_SCHEDULE`). Stage
/// below 2 (signed byte): is_league = 1 (if asked), 3 rounds, extra = stage + 0x3e9; the table (`exe::crt_malloc`(0xc3);
/// out of memory, the error box at line 0x99 and 0) with the three round dates (`exe::league_round_set_date`, 9
/// arguments, cdecl; the year +0x40 as a word) and match 2 of each round empty (`exe::league_round_set_match_slot`, 7
/// arguments, cdecl), then the group's six matches. Stage 2: is_league = 0, 3 rounds, extra = 0; the table (0x138; the
/// box at line 0xd3) with the draw dates (`exe::cup_set_round_draw_date`, 7 arguments) and match dates
/// (`exe::cup_set_round_match_date`, 9 arguments; the year signed), and the fields of each round record (0x68 bytes).
/// Above 2: 0 with nothing written. (`build_phase_table`, the engine of the national-team tournaments.)
/// Original: `WorldClubChamp::create_schedule` (0x929390, world_club_champ.cpp, thiscall, ret 0x10).
unsafe extern "thiscall" fn world_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), world_club_champ_phase(stage), rounds, extra, league, 0xad5764)
}

/// Comparer for `world_club_champ_create_schedule`: side by side (`compare_phase_table`).
unsafe extern "thiscall" fn compare_world_club_champ_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    compare_phase_table("world_club_champ_create_schedule", stage, world_club_champ_phase(stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| world_club_champ_create_schedule(this, stage, r, e, l))
}

/// The World Club Championship rule fields: 3 points a win and 1 a draw, tie-breaks 1 and 2 [inference: goal
/// difference, goals for], two qualifiers per group, category 0x17, 7 substitutes and 3 changes, one meeting
/// (`league::LEGS` = 1), one promoted (the group winner) and the rule bits 0x28.
const WORLD_CLUB_CHAMP_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 0x17),
    (0xc5, 1, 1), (0xc7, 1, 4), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7), (0xd9, 2, 0x28)];

/// Rule: the World Club Championship groups are single round robin, 3 points a win and 1 a draw, category 0x17; the
/// draw is on 5 February of the competition year (moved to weekday 5), the competition year being the first even year
/// from 2003 on (the constructor 0x9291a2), so it never comes before June 2002.
///
/// `WorldClubChamp::set_up_rules()` (virtual `comp_vtable::SET_UP_RULES`): the rule fields
/// (`WORLD_CLUB_CHAMP_RULE_FIELDS`, `goldcup_write_rule_fields`), the round table of the first group (virtual
/// `comp_vtable::CREATE_SCHEDULE`(−1, &`league::ROUND_COUNT`, &`league::STAGE_ID`, 0): push 0; push; push; push −1; mov
/// ecx -> thiscall, ret 0x10) in `league::ROUND_DATES`, then the draw date (`competition_date_word`(5, 1, 0, 5):
/// Date::Date 0x549f70, thiscall, ret 0x10) in `league::NEXT_DRAW_DAY` and `league::NEXT_DRAW_YEAR_OFFSET` = 0
/// (unlike the Copa América's, the table comes before the date). Returns 1.
/// Original: `WorldClubChamp::set_up_rules` (0x929bb0, world_club_champ.cpp, thiscall, ret 0).
unsafe extern "thiscall" fn world_club_champ_set_up_rules(this: usize) -> u32 {
    covered!("cqx_2 world club championship rules");
    goldcup_write_rule_fields(this, &WORLD_CLUB_CHAMP_RULE_FIELDS, 3);
    let l = League(this);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::CREATE_SCHEDULE));
    l.set_round_dates(table(this, 0xffff_ffff, this + league::ROUND_COUNT, this + league::STAGE_ID, 0));
    let day = 5; // cqx2: the draw day
    l.set_next_draw_day(competition_date_word(l, day, 1, 0, 5) as i16);
    l.set_next_draw_year_offset(0);
    1
}

/// Comparer for `world_club_champ_set_up_rules`, side by side over the same object: the original first, with the plan
/// (`check_set_up_rules`: the fields, the date and return 1); then the copy, whose rule fields, date and round table
/// (the bytes the first group's operations write) must equal the original's; the copy's table is freed and the
/// original's put back in `league::ROUND_DATES`.
unsafe extern "thiscall" fn compare_world_club_champ_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let r = check_set_up_rules("world_club_champ_set_up_rules", this, &WORLD_CLUB_CHAMP_RULE_FIELDS, (5, 1, 0, 5), 0, || o(this));
    let l = League(this);
    let t = l.round_dates();
    if r != 1 || t == 0 {
        return r;
    }
    let before: Vec<u8> = (0x3au32..0xdb).filter(|&k| !(0xba..0xbe).contains(&k)).map(|k| read_u8(this + k as usize)).collect();
    let r2 = world_club_champ_set_up_rules(this);
    let t2 = l.round_dates();
    let after: Vec<u8> = (0x3au32..0xdb).filter(|&k| !(0xba..0xbe).contains(&k)).map(|k| read_u8(this + k as usize)).collect();
    let tables = t2 != 0 && phase_table_written_bytes(&WORLD_CLUB_CHAMP_GROUP_ROUNDS, &WORLD_CLUB_CHAMP_GROUP_A_MATCHES, t) == phase_table_written_bytes(&WORLD_CLUB_CHAMP_GROUP_ROUNDS, &WORLD_CLUB_CHAMP_GROUP_A_MATCHES, t2);
    if t2 != 0 && t2 != t {
        exe::crt_free(t2);
    }
    l.set_round_dates(t);
    SCOREBOARD.record(r2 == r && before == after && tables, || format!("world_club_champ_set_up_rules({:#x}): copy returned {}, fields equal {}, table equal {}", this, r2, before == after, tables));
    r
}

/// Rule: the World Club Championship's day: before the draw date nothing (not even the league day); on the draw day (5
/// February of the competition year, from 2003 on) the entrants, the predicted positions, the first group's table, the
/// fixtures, the order of the table and the other groups' stages are made once; then the league day.
///
/// `WorldClubChamp::update_for_date(today, full)` (virtual +0x14; the SEH frame only covers the local dates, no
/// destructor). The draw date is that of Date::Date 0x549ef0(`league::NEXT_DRAW_DAY`, `league::NEXT_DRAW_YEAR_OFFSET` +
/// `league::YEAR`) (lea ecx, local; push year; push day -> thiscall, ret 8: the day of the year and the year, 8 bytes).
/// Before it (today's year lower, unsigned; or the same year and today's day lower, signed) it returns 0. On the draw
/// day, if the draw is not done (`league::ACTIVE` = 0) and WITHOUT `full`: the entrants (WorldClubChamp 0x92a8b0, mov
/// ecx -> thiscall, ret 0), the predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0),
/// the first group's table (WorldClubChamp 0x929aa0, thiscall, ret 0; 0: the error box at line 0x3c7), the fixtures
/// (`exe::league_create_fixtures`; 0: the box at line 0x3ce), the sorted table (`exe::league_sort_table`(this, 0); 0:
/// the box at line 0x3d5) (an error returns 0 without the league day), the other groups' stages (WorldClubChamp
/// 0x929e60, thiscall, ret 0), the drawn list (+0xee) freed (`exe::crt_free`) and zeroed, +0xf2 = 0 and
/// `league::ACTIVE` = 1. Then the league day (`exe::league_update_for_date`(this, today, full): push full; push today;
/// mov ecx -> thiscall, ret 8), whose return value it passes on. (In the test career the draw date is in 2003: only
/// the early return runs.)
/// Original: `WorldClubChamp::update_for_date` (0x92a6b0, world_club_champ.cpp, thiscall, ret 8).
unsafe extern "thiscall" fn world_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let l = League(this);
    let day = l.next_draw_day() as u16;
    let year = (l.next_draw_year_offset() as u16).wrapping_add(l.year() as u16);
    let day_of_today = read_u16(today);
    let year_of_today = read_u16(today + 2);
    // simplification: on the early return and on errors the original returns only al = 0 (the rest of eax is leftover);
    // the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        covered!("cqx_2 before the draw");
        return 0; // cqx2: nothing before the draw date
    }
    if year_of_today == year && day_of_today == day && l.active() == 0 && full == 0 {
        covered!("cqx_2 world club championship draw");
        let routine = |a: usize| -> unsafe extern "thiscall" fn(usize) -> i32 { core::mem::transmute(a) };
        routine(0x92a8b0)(this); // WorldClubChamp: the entrants
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this);
        if routine(0x929aa0)(this) == 0 {
            show_error_at(0xad5764, 0x3c7);
            return 0;
        }
        if exe::league_create_fixtures(l) == 0 {
            show_error_at(0xad5764, 0x3ce);
            return 0;
        }
        if exe::league_sort_table(l, 0) == 0 {
            show_error_at(0xad5764, 0x3d5);
            return 0;
        }
        routine(0x929e60)(this); // WorldClubChamp: the other groups' stages
        exe::crt_free(read_usize(this + 0xee));
        write_usize(this + 0xee, 0);
        write_usize(this + 0xf2, 0);
        l.set_active(1);
    }
    exe::league_update_for_date(l, today, full) as u32
}

/// Comparer for `world_club_champ_update_for_date`. Before the draw date (the plan's condition) the routine only reads:
/// side by side, the original and then the copy, and the returned bytes must be equal. From the date on, only the
/// original (the draw allocates and draws; the league day plays), with the plan on the draw day: `league::ACTIVE` = 1,
/// the list +0xee and +0xf2 zeroed and the table made.
unsafe extern "thiscall" fn compare_world_club_champ_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let l = League(this);
    let day = l.next_draw_day() as u16;
    let year = (l.next_draw_year_offset() as u16).wrapping_add(l.year() as u16);
    let (dt, yt) = (read_u16(today), read_u16(today + 2));
    if yt < year || (yt == year && (dt as i16) < day as i16) {
        let r = o(this, today, full);
        let c = world_club_champ_update_for_date(this, today, full);
        SCOREBOARD.record(r as u8 == c as u8, || format!("world_club_champ_update_for_date({:#x}): before the draw original {} copy {}", this, r as u8, c as u8));
        return r;
    }
    let draws = yt == year && dt == day && l.active() == 0 && full == 0;
    let r = o(this, today, full);
    if draws {
        let ok = l.active() == 1 && read_usize(this + 0xee) == 0 && read_usize(this + 0xf2) == 0 && l.table() != 0;
        SCOREBOARD.record(ok, || format!("world_club_champ_update_for_date({:#x}): the draw (+0x45 {}, +0xee {:#x})", this, l.active(), read_usize(this + 0xee)));
    }
    r
}
