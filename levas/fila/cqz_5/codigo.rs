// ---------- cqz_5: the daily update of the Asian Cup, the Olympic football and the Confederations Cup (asia_nations.cpp, olympics.cpp, fifa_confed.cpp) ----------

/// The shape of a national-team tournament's day: the routines of its draw day and the original's quirks.
struct NationsDayPlan {
    /// Without `full`, the league day runs at once (the Asian Cup); otherwise the date test comes first.
    light_day_first: bool,
    /// The draw only runs on a day update without `full` (the Olympics and the Confederations Cup).
    draw_needs_light_day: bool,
    /// The draw of the finalists (thiscall, ret 0; its return value is not read).
    draw: usize,
    /// The first group table (thiscall, ret 0; 0 = error).
    group_table: usize,
    /// The source file of the error boxes and the lines of the group table, fixtures and sort errors.
    file: usize,
    rows: [u32; 3],
    /// The group stage routine and how many it builds: `Some(n)`, push k; thiscall, ret 4 for k = 0..n; `None`, one
    /// call without arguments (thiscall, ret 0).
    group_stage: usize,
    group_stages: Option<u32>,
    /// The entrant count +0xf2 is zeroed as a dword (not a word).
    count_as_dword: bool,
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
unsafe fn nations_tournament_day(this: League, today: usize, full: u32, plan: &NationsDayPlan) -> u32 {
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    let league_day: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    if plan.light_day_first && full == 0 {
        return league_day(this.addr(), today, 0) as u32;
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
    league_day(this.addr(), today, full) as u32
}

/// Shared comparer: the draw, the tables, the fixtures and the league day cannot run twice, so only the original runs;
/// on the draw day (the copy's condition) the plan: `league::ACTIVE` = 1, the list +0xee and its count zeroed, the
/// first group table and the first group stage created.
unsafe fn compare_nations_tournament_day(name: &str, this: League, today: usize, full: u32, plan: &NationsDayPlan, o: impl FnOnce() -> u32) -> u32 {
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

/// The Asian Cup day: the draw of the finalists (AsiaNations::select_and_draw_teams 0x40fdf0), group A's table
/// (AsiaNations::create_group_table 0x40f3d0) and the stages of the two other groups (AsiaNations::create_group_stage
/// 0x40f8a0 with 0 and 1); the boxes of lines 0x3dd, 0x3e3 and 0x3ea of asia_nations.cpp; the count +0xf2 zeroed as a
/// word; without `full`, the league day at once.
const ASIA_NATIONS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: true, draw_needs_light_day: false, draw: 0x40fdf0, group_table: 0x40f3d0,
    file: 0x987ba0, rows: [0x3dd, 0x3e3, 0x3ea], group_stage: 0x40f8a0, group_stages: Some(2), count_as_dword: false };

/// rule: on the draw day of the Asian Cup (5 June of the cup year: day 5 of month 5, months from 0) the finalists are drawn into three groups, group A
/// gets its table and fixtures and the other two groups their stages (`nations_tournament_day`)
/// Original: `AsiaNations::update_for_date` (0x410170, asia_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 asian cup day");
    nations_tournament_day(League(this), today, full, &ASIA_NATIONS_DAY)
}

/// Comparer for `asia_nations_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_asia_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    compare_nations_tournament_day("asia_nations_update_for_date", League(this), today, full, &ASIA_NATIONS_DAY, || o(this, today, full))
}

/// The Olympic day: the date test even without `full`, and the draw only on a day without `full` (a quirk of the
/// original, as in the Confederations Cup); the draw of the finalists (Olympics::draw_finalists 0x7a09e0), group A's
/// table (Olympics::init_group_a_table 0x7a0140) and the stages of groups B to D (Olympics::create_group_stage 0x7a05d0
/// with 0, 1 and 2); the boxes of lines 0x41b, 0x421 and 0x428 of olympics.cpp; the count +0xf2 zeroed as a dword.
const OLYMPICS_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x7a09e0, group_table: 0x7a0140,
    file: 0xa6a360, rows: [0x41b, 0x421, 0x428], group_stage: 0x7a05d0, group_stages: Some(3), count_as_dword: true };

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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    compare_nations_tournament_day("olympics_update_for_date", League(this), today, full, &OLYMPICS_DAY, || o(this, today, full))
}

/// The Confederations Cup day: like the Olympic one (the date test even without `full`, the draw only without it); the
/// selection of the nations (FifaConfed::select_teams 0x58c6b0), the first group's table (FifaConfed::create_group_table
/// 0x58bcb0) and the second group's stage (FifaConfed::create_group_stage 0x58c100, no argument); the boxes of lines
/// 0x32b, 0x332 and 0x339 of fifa_confed.cpp; the count +0xf2 zeroed as a dword.
const FIFA_CONFED_DAY: NationsDayPlan = NationsDayPlan { light_day_first: false, draw_needs_light_day: true, draw: 0x58c6b0, group_table: 0x58bcb0,
    file: 0x9c7474, rows: [0x32b, 0x332, 0x339], group_stage: 0x58c100, group_stages: None, count_as_dword: true };

/// rule: on the draw day of the Confederations Cup (20 February of the cup year: day 0x14 of month 1) the nations are selected and drawn
/// into two groups, the first gets its table and fixtures and the second its stage (`nations_tournament_day`)
/// Original: `FifaConfed::update_for_date` (0x58c4b0, fifa_confed.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    covered!("cqz_5 confederations cup day");
    nations_tournament_day(League(this), today, full, &FIFA_CONFED_DAY)
}

/// Comparer for `fifa_confed_update_for_date` (original only, then the plan on the draw day).
pub(crate) unsafe extern "thiscall" fn compare_fifa_confed_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    compare_nations_tournament_day("fifa_confed_update_for_date", League(this), today, full, &FIFA_CONFED_DAY, || o(this, today, full))
}
