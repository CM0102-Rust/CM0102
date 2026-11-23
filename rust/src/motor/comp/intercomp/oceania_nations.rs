use super::*;

/// rule: the Oceania Nations Cup predictions: the six drawn nations by reputation get the places 1, 2, 3, 3, 5 and 5
///
/// The virtual `comp_vtable::SET_PREDICTED_POSITIONS`. Without statistics (`league::COMP_STATS`), the error box of line
/// 0x436 and return; a copy of the drawn list +0xee with the count +0xf2 (signed word) × 4 bytes (`exe::crt_malloc`:
/// push; add esp -> cdecl; out of memory, the box of line 0x43e and return), sorted by reputation (qsort
/// `exe::crt_qsort`(copy, count, 4, 0x4b07d0): 4 pushes -> cdecl), and the first six get their predicted place in the
/// statistics (`exe::comp_stats_set_club_final_position`: push place; push team; mov ecx, statistics -> thiscall, ret
/// 8), whatever the count; the copy freed (`exe::crt_free`).
/// Original: `OceaniaNations::set_predicted_positions` (0x79a320, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_predicted_positions(this: usize) {
    let this = League(this);
    if this.comp_stats() == 0 {
        return show_error_at(0xa6a0ec, 0x436);
    }
    let n = read_i16(this.addr() + 0xf2) as i32;
    let v = exe::crt_malloc((n << 2) as u32);
    if v == 0 {
        return show_error_at(0xa6a0ec, 0x43e);
    }
    core::ptr::copy_nonoverlapping(read_usize(this.addr() + 0xee) as *const u8, v as *mut u8, ((n << 2) as u32) as usize);
    exe::crt_qsort(v, n as u32, 4, 0x4b07d0);
    covered!("cqz_9 oceania nations predictions");
    for (k, &p) in OCEANIA_NATIONS_PREDICTED_PLACES.iter().enumerate() {
        exe::comp_stats_set_club_final_position(this.comp_stats(), read_usize(v + k * 4), p); // cqz9: the place by reputation
    }
    exe::crt_free(v);
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_predicted_positions(this: usize) {
    let this = League(this);
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_OCEANIA_NATIONS_SET_PREDICTED_POSITIONS);
    if this.comp_stats() == 0 || read_usize(this.addr() + 0xee) == 0 || read_i16(this.addr() + 0xf2) != 6 {
        return o(this.addr());
    }
    let mut v: Vec<usize> = (0..6).map(|i| read_usize(read_usize(this.addr() + 0xee) + i * 4)).collect();
    exe::crt_qsort((v.as_mut_ptr()) as usize, 6, 4, 0x4b07d0);
    o(this.addr());
    let wrong = (0..6).find(|&k| {
        let e = CompStatsClub(if v[k] != 0 { exe::comp_stats_find_club_entry(this.comp_stats(), v[k]) } else { 0 });
        e.is_null() || e.final_position() as u32 != OCEANIA_NATIONS_PREDICTED_PLACES[k]
    });
    SCOREBOARD.record(wrong.is_none(), || format!("oceania_nations_set_predicted_positions: the predicted place of slot {:?} differs", wrong));
}

/// rule: the Oceania Nations Cup groups follow the Asian Cup rules (single round robin, 3/1 points, tiebreak 1, 2, 3,
/// two qualifiers per group, category 10, 7 substitutes and 3 changes); the draw date is 5 of month 2 of the cup year on
/// weekday 5, `.next_draw_year_offset()` 0.
///
/// The same body as `asia_nations_set_up_rules` (the fields of `AFRICAN_NATIONS_RULE_FIELDS`; note `+0xbe` = 2, not the
/// 1 of the Oceania World Cup qualifying), with Date::Date(5, 2, year, 5). Returns 1.
/// Original: `OceaniaNations::set_up_rules` (0x799690, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqz_1 oceania nations cup rules");
    let win = 3; // cqz1: the win in the Oceania Nations Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let month = 2; // cqz1: the month of the Oceania Nations Cup draw date
    let word = competition_date_word(League(this), 5, month, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

/// Comparer for `oceania_nations_set_up_rules` (original only, then the plan).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_SET_UP_RULES);
    check_set_up_rules("oceania_nations_set_up_rules", this, &AFRICAN_NATIONS_RULE_FIELDS, (5, 2, 0, 5), 0, || o(this))
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
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_OCEANIA_NATIONS_CREATE_SCHEDULE);
    compare_phase_table("oceania_nations_create_schedule", stage, oceania_nations_phase(League(this), stage), rounds, extra, league, || o(this, stage, rounds, extra, league),
        |r, e, l| oceania_nations_create_schedule(this, stage, r, e, l))
}

/// The Oceania Nations Cup at career creation: the edition is the game's year, never before 2002, moved to the first
/// even year; vtable 0x96e0cc, 2 stages, the rules OceaniaNations::set_up_rules (0x799690) and the error boxes of lines
/// 0x57 and 0x5e (`build_nations_tournament`).
/// Original: `OceaniaNations::OceaniaNations` (0x7988b0, oceania_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    covered!("cqw_10 oceania nations cup");
    let edition = nations_cup_edition((year as i16).max(0x7d2), 0); // cqw10: the Oceania Nations Cup from 2002 on
    build_nations_tournament(League(this), edition, record, 0x96e0cc, 2, 0x799690, 0xa6a0ec, (0x57, 0x5e))
}

/// Comparer for `construct_oceania_nations`.
pub(crate) unsafe extern "thiscall" fn compare_construct_oceania_nations(this: usize, year: u32, record: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_OCEANIA_NATIONS);
    compare_nations_tournament_ctor("construct_oceania_nations", League(this), record, 0x96e0cc, nations_cup_edition((year as i16).max(0x7d2), 0), || o(this, year, record))
}

/// The second group of the Oceania Nations Cup: a league stage with the nations of places 3, 4 and 5 of the list.
///
/// `OceaniaNations::create_group_stage(stage)`. The nations 3, 4 and 5 of the list at +0xee in a vector
/// (`exe::crt_malloc` of 0xc; out of memory, the error box of line 0x331 and 0); the round table of `stage` (virtual
/// `comp_vtable::CREATE_SCHEDULE`); the stage (operator new `exe::crt_operator_new` of 0xee and
/// League_Stage::League_Stage `FN_LEAGUE_STAGE_LEAGUE_STAGE`, ret 0x58: the cup, 3, the vector, 1, the competition
/// record, the table, the rounds, `win_points`, `draw_points`, `match_category`, &`league::TIEBREAKS`,
/// &`league::PROMOTED_COUNT`, the year, the stage, the id, `importance`, 4, 0, `league_flags`, −1, 0, 2; words and bytes
/// zero-extended) in `stages()`[stage]; the vector and the table freed (`exe::crt_free`); without a stage, the error box
/// of line 0x347 and 0; with it, `current_stage` = stage, the group draw news (League::send_group_draw_news
/// `exe::league_send_group_draw_news` on the stage) and 1. The original's SEH frame covers only the stage's `new`.
/// Original: `OceaniaNations::create_group_stage` (0x799a60, oceania_nations.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let this = League(this);
    let teams = exe::crt_malloc(0xc);
    if teams == 0 {
        show_error_at(0xa6a0ec, 0x331);
        return 0;
    }
    for i in 0..3usize {
        write_u32(teams + i * 4, read_u32(read_usize(this.addr() + 0xee) + 0xc + i * 4));
    }
    let create_schedule: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut id) = (0i16, 0i16);
    let table = create_schedule(this.addr(), stage, &mut rounds, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let group = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
            core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
        construct(p, this.addr(), 3, teams, 1, this.competition().addr(), table, rounds as u16 as u32, this.win_points() as u32, this.draw_points() as u32, this.match_category() as u32,
                  this.addr() + league::TIEBREAKS, this.addr() + league::PROMOTED_COUNT, this.year() as u16 as u32, stage, id as u16 as u32, this.importance() as u32, 4, 0,
                  this.league_flags() as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    write_u32(this.stages() + stage as usize * 4, group as u32);
    exe::crt_free(teams);
    exe::crt_free(table);
    let group = read_usize(this.stages() + stage as usize * 4);
    if group == 0 {
        show_error_at(0xa6a0ec, 0x347);
        return 0;
    }
    this.set_current_stage((stage) as usize);
    exe::league_send_group_draw_news(group);
    1
}

/// Comparer for `oceania_nations_create_group_stage`. The stage is allocated and the news goes to the managers, so it
/// cannot run twice: only the original runs, then the plan is checked: it returns 1, the stage is in
/// `stages()`[stage], `current_stage` = stage and the nations 3, 4 and 5 of the list are in its table (in any order).
pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_group_stage(this: usize, stage: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_OCEANIA_NATIONS_CREATE_GROUP_STAGE);
    let league = League(this);
    let list = read_usize(this + 0xee);
    if list == 0 || stage > 8 {
        return o(this, stage);
    }
    let mut plan: Vec<usize> = (3..6).map(|k| read_usize(list + k * 4)).collect();
    let r = o(this, stage);
    if r == 1 {
        let group = read_usize(league.stages() + stage as usize * 4);
        let t = if group != 0 { League(group).table() } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..3).map(|k| read_usize(t + k * league_table_row::SIZE)).collect() } else { Vec::new() };
        a.sort_unstable();
        plan.sort_unstable();
        SCOREBOARD.record(group != 0 && league.current_stage() == stage as usize && a == plan, || format!("oceania_nations_create_group_stage: {:x?} plan {:x?}", a, plan));
    }
    r
}

/// The day of the Oceania Nations Cup (the virtual `comp_vtable::UPDATE_FOR_DATE`; SEH frame only for the local dates,
/// no destructor). Without `full`, only the league day (`League::update_for_date`(this, today, 0): 2 pushes; mov ecx ->
/// thiscall, ret 8). Before the draw (today's year lower, unsigned; or the same year and the day lower, signed) it
/// returns 0 without the league day. On the draw day (`oceania_nations_draw_date`), with `league::ACTIVE` = 0: the six
/// nations (`oceania_nations_select_and_draw_teams`, 0x799e30, thiscall, ret 0), the predicted positions (the virtual
/// `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group table (`oceania_nations_create_group_table`,
/// 0x7995a0, thiscall, ret 0; 0: the box for line 0x405 and 0), the fixtures (`exe::league_create_fixtures`; 0: the box
/// for line 0x40b and 0), the sorted table (`exe::league_sort_table`(this, 0): push 0; mov ecx -> thiscall, ret 4; 0:
/// the box for line 0x412 and 0), the second group stage (0x799a60(this, 0): push 0; mov ecx -> thiscall, ret 4), the
/// entrant list +0xee freed (`exe::crt_free`) and zeroed, its count +0xf2 (word) = 0 and `league::ACTIVE` = 1. Then the
/// league day, whose return value it passes on.
/// Original: `OceaniaNations::update_for_date` (0x79a100, oceania_nations.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let league = this;
    // the full eax is passed on (the exe:: wrapper reads only the byte)
    if full == 0 {
        return exe::league_update_for_date(this, today, 0) as u32; // the callers read only al
    }
    let (day, year) = oceania_nations_draw_date(league);
    let today_day = read_u16(today);
    let today_year = read_u16(today + 2);
    // simplification: on the returns before the draw and on the errors the original returns only al (0); the caller reads the byte
    if today_year < year || (today_year == year && (today_day as i16) < day as i16) {
        return 0;
    }
    if today_year == year && today_day == day && league.active() == 0 {
        covered!("cqv_10 draw day");
        exe::oceanianations_select_and_draw_teams(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if exe::oceanianations_create_group_table(this) == 0 {
            show_error_at(0xa6a0ec, 0x405);
            return 0;
        }
        if exe::league_create_fixtures(league) == 0 {
            show_error_at(0xa6a0ec, 0x40b);
            return 0;
        }
        if exe::league_sort_table(league, 0) == 0 {
            show_error_at(0xa6a0ec, 0x412);
            return 0;
        }
        exe::oceanianations_create_group_stage(this, 0);
        exe::crt_free(read_usize(this.addr() + 0xee));
        write_u32(this.addr() + 0xee, 0);
        write_i16(this.addr() + 0xf2, 0);
        this.set_active((1) as usize);
    }
    exe::league_update_for_date(this, today, full) as u32 // the callers read only al
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_OCEANIA_NATIONS_UPDATE_FOR_DATE);
    let league = League(this);
    let draws = full != 0 && league.active() == 0 && {
        let (day, year) = oceania_nations_draw_date(league);
        read_u16(today) == day && read_u16(today + 2) == year
    };
    let r = o(this, today, full);
    if draws {
        let stages = league.stages();
        let ok = league.active() == 1 && read_usize(this + 0xee) == 0 && read_i16(this + 0xf2) == 0 && league.table() != 0 && stages != 0 && read_usize(stages) != 0;
        SCOREBOARD.record(ok, || format!("oceania_nations_update_for_date({:#x}): the draw did not build the groups (+0x45 {})", this, league.active()));
    }
    r
}

/// The six nations of the Oceania Nations Cup and their draw. +0xf2 = 6 and the list +0xee of 6 national teams
/// (`exe::crt_malloc`(0x18), cdecl; out of memory, the box for line 0x39e and 0). Of the Oceania nations (continent
/// `KEY_CONTINENT_OCEANIA`, `nation::CONTINENT`) other than Australia (`KEY_NATION_AUSTRALIA`) and New Zealand
/// (`KEY_NATION_NEW_ZEALAND`): up to 10 Melanesian (`exe::database_is_melanesian_nation`) and up to 10 Polynesian
/// (`exe::database_is_polynesian_nation`; a Melanesian one beyond the 10 falls through to the Polynesian test), each by
/// its main national team (`exe::database_get_national_team`). A draw of 2 (`exe::utils_random`(2), cdecl) puts
/// Australia in slot 0 and New Zealand in slot 3 (with 1) or the other way round; the Melanesian and the Polynesian
/// teams are sorted and shuffled (`exe::comp_util_sort_and_shuffle_top_clubs`(list, n, 2): 3 pushes -> cdecl); another
/// draw puts the first two Melanesian teams in slots 1 and 4 (or 4 and 1) and another the first two Polynesian ones in
/// slots 2 and 5 (or 5 and 2). Returns 1.
/// Original: `OceaniaNations::select_and_draw_teams` (0x799e30, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    write_i16(this + 0xf2, 6);
    let list = exe::crt_malloc(0x18);
    write_usize(this + 0xee, list);
    if list == 0 {
        show_error_at(0xa6a0ec, 0x39e);
        return 0;
    }
    let (mut mel, mut pol) = ([0usize; 10], [0usize; 10]);
    let (mut nm, mut np) = (0i8, 0i8);
    let oceania = read_usize(KEY_CONTINENT_OCEANIA);
    let mut i = 0i16;
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let rec = read_usize(G_NATION_TABLE).wrapping_add((i as isize as usize).wrapping_mul(nation::SIZE));
        let cont = Nation(rec).continent();
        let k = i as i32;
        if cont != 0 && read_usize(cont) == oceania && k != read_usize(KEY_NATION_AUSTRALIA) as i32 && k != read_usize(KEY_NATION_NEW_ZEALAND) as i32 {
            if exe::database_is_melanesian_nation(rec) != 0 && nm < 10 {
                mel[nm as usize] = exe::database_get_national_team(rec);
                nm += 1;
            } else if exe::database_is_polynesian_nation(rec) != 0 && np < 10 {
                // cqv10: up to 10 Polynesian teams
                pol[np as usize] = exe::database_get_national_team(rec);
                np += 1;
            }
        }
        i = i.wrapping_add(1);
    }
    covered!("cqv_10 oceania cup draw");
    let (australia, new_zealand) = (oceania_nations_nation_record(KEY_NATION_AUSTRALIA), oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND));
    if exe::utils_random(2) != 0 { // cqv10: the draw of Australia and New Zealand
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(new_zealand));
    } else {
        write_usize(read_usize(this + 0xee) + 0xc, exe::database_get_national_team(australia));
        write_usize(read_usize(this + 0xee), exe::database_get_national_team(new_zealand));
    }
    exe::comp_util_sort_and_shuffle_top_clubs(mel.as_mut_ptr() as usize, nm as i16 as u16 as u32, 2); // cqv10: the Melanesian teams with 2 picks
    exe::comp_util_sort_and_shuffle_top_clubs(pol.as_mut_ptr() as usize, np as i16 as u16 as u32, 2);
    let slot = |k: usize| read_usize(this + 0xee) + k * 4;
    if exe::utils_random(2) != 0 {
        write_usize(slot(1), mel[0]);
        write_usize(slot(4), mel[1]);
    } else {
        write_usize(slot(4), mel[0]);
        write_usize(slot(1), mel[1]);
    }
    if exe::utils_random(2) != 0 {
        write_usize(slot(2), pol[0]);
        write_usize(slot(5), pol[1]);
    } else {
        write_usize(slot(5), pol[0]);
        write_usize(slot(2), pol[1]);
    }
    1
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_select_and_draw_teams(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_SELECT_AND_DRAW_TEAMS);
    let r = o(this);
    if r == 1 {
        let list = read_usize(this + 0xee);
        let mut a = [read_usize(list), read_usize(list + 0xc)];
        let mut b = [exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_AUSTRALIA)), exe::database_get_national_team(oceania_nations_nation_record(KEY_NATION_NEW_ZEALAND))];
        a.sort_unstable();
        b.sort_unstable();
        SCOREBOARD.record(read_i16(this + 0xf2) == 6 && a == b, || format!("oceania_nations_select_and_draw_teams({:#x}): slots 0 and 3 {:x?} expected {:x?}", this, a, b));
    }
    r
}

/// The first group of the Oceania Nations Cup has 3 national teams (`league::CLUB_COUNT`); the table
/// `league::TABLE` (`exe::crt_malloc`(0xb1): 3 rows of `league_table_row::SIZE` bytes; out of memory, the box for line
/// 0x246 and 0), each row i with national team i of the list +0xee (`exe::league_init_table_row`: push 0; push 0; push
/// team; push i; mov ecx -> thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`) and returns 1.
/// The body of the Oceania qualifier's 0x91ec00 with 3 rows and the list in 4-byte steps.
/// Original: `OceaniaNations::create_group_table` (0x7995a0, oceania_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn oceania_nations_create_group_table(this: usize) -> u32 {
    let league = League(this);
    league.set_club_count(3);
    let t = exe::crt_malloc(0xb1);
    league.set_table(t);
    if t == 0 {
        show_error_at(0xa6a0ec, 0x246);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < league.club_count() {
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(league, i as u8 as u32, read_usize(read_usize(this + 0xee) + i as isize as usize * 4), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(league);
    1
}

pub(crate) unsafe extern "thiscall" fn compare_oceania_nations_create_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_OCEANIA_NATIONS_CREATE_GROUP_TABLE);
    let list = read_usize(this + 0xee);
    if list == 0 {
        return o(this);
    }
    let expected: [usize; 3] = core::array::from_fn(|i| read_usize(list + i * 4));
    let r = o(this);
    let league = League(this);
    let t = league.table();
    let mut read = [(0usize, 0u8); 3];
    if t != 0 {
        for (i, l) in read.iter_mut().enumerate() {
            *l = (read_usize(t + i * league_table_row::SIZE), read_u8(t + i * league_table_row::SIZE + league_table_row::POSITION));
        }
    }
    let same = r == 1 && league.club_count() == 3 && t != 0 && (0..3).all(|i| read[i] == (expected[i], i as u8));
    SCOREBOARD.record(same, || format!("oceania_nations_create_group_table({:#x}) returned {} rows {:x?} expected {:x?}", this, r, read, expected));
    r
}

/// The round table of stage `stage` of the Oceania Nations Cup: the editions of years with remainder 2 by 4 (2002,
/// 2006...) have their own dates (November), the others the June ones.
pub(crate) unsafe fn oceania_nations_phase(this: League, stage: u32) -> Option<GoldcupStagePlan> {
    if (this.year() as i32) % 4 == 2 { // cqz6: the Oceania Nations Cup of 2002, 2006...
        oceania_nations_even_phase(stage)
    } else {
        oceania_nations_other_phase(stage)
    }
}

/// The draw date (`exe::new_date` on a local date: push year; push day; lea ecx -> thiscall, ret 8): the day
/// `league::NEXT_DRAW_DAY` and the year `league::NEXT_DRAW_YEAR_OFFSET` + `league::YEAR` (words), returned as (day,
/// year).
pub(crate) unsafe fn oceania_nations_draw_date(this: League) -> (u16, u16) {
    // the date is 8 bytes: the day, the year and the leap-year dword at +4, which 0x549ef0 also writes
    let mut date = [0u32; 2];
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    exe::new_date(date.as_mut_ptr() as usize, this.next_draw_day() as u16 as u32, year as u32);
    (date[0] as u16, (date[0] >> 16) as u16)
}

/// The record of the nation of key `key` (global): `G_NATION_TABLE` + `nation::SIZE` x [key].
pub(crate) unsafe fn oceania_nations_nation_record(key: usize) -> usize {
    read_usize(G_NATION_TABLE).wrapping_add(read_usize(key).wrapping_mul(nation::SIZE))
}

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table, or none.
pub(crate) fn oceania_nations_even_phase(stage: u32) -> Option<GoldcupStagePlan> {
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

/// The round table of stage `stage` (signed byte) of cqz_6 oceania nations cup round table (other years), or none.
pub(crate) fn oceania_nations_other_phase(stage: u32) -> Option<GoldcupStagePlan> {
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

/// The predicted final places of the six Oceania Nations Cup nations, in reputation order.
pub(crate) const OCEANIA_NATIONS_PREDICTED_PLACES: [u32; 6] = [1, 2, 3, 3, 5, 5];

/// cqz_6 oceania nations cup round table, the operations of round table 1 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_EVEN_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];

/// cqz_6 oceania nations cup round table, the operations of round table 2 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_EVEN_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 1, 3, 4), Cqp12L(1, 0, 0, 1, 3, 0x40), Cqp12L(2, 0, 2, 1, 3, 4),
];

/// cqz_6 oceania nations cup round table, the operations of round table 3 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_EVEN_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0xa, 0xa, 0, 4, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0xd, 0xa, 0, 0, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0xf, 0xa, 0, 2, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 4, 2, 3, 0x40), Cqp12L(1, 0, 0, 2, 3, 4), Cqp12L(2, 0, 2, 2, 3, 0x40),
];

/// cqz_6 oceania nations cup round table, the operations of round table 4 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_EVEN_OPS_4: [GoldcupScheduleOp; 57] = [
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

/// cqz_6 oceania nations cup round table (other years), the operations of round table 1 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_OTHER_OPS_1: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
];

/// cqz_6 oceania nations cup round table (other years), the operations of round table 2 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_OTHER_OPS_2: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(2, 0, 4, 1, 3, 4),
];

/// cqz_6 oceania nations cup round table (other years), the operations of round table 3 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_OTHER_OPS_3: [GoldcupScheduleOp; 9] = [
    Cqp12D(0, 0x13, 5, 0, 0, 1, 1), Cqp12L(0, 1, -1, -1, -1, 0), Cqp12D(1, 0x15, 5, 0, 2, 1, 1),
    Cqp12L(1, 1, -1, -1, -1, 0), Cqp12D(2, 0x17, 5, 0, 4, 1, 1), Cqp12L(2, 1, -1, -1, -1, 0),
    Cqp12L(0, 0, 0, 2, 3, 0x40), Cqp12L(1, 0, 2, 2, 3, 4), Cqp12L(2, 0, 4, 2, 3, 0x40),
];

/// cqz_6 oceania nations cup round table (other years), the operations of round table 4 (from the original, in order).
pub(crate) const OCEANIA_NATIONS_OTHER_OPS_4: [GoldcupScheduleOp; 57] = [
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
