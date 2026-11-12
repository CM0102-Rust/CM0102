use super::*;

/// National team club of the nation with the given id (database_get_national_team).
pub(crate) unsafe fn national_team_of_nation_id(id: usize) -> usize {
    exe::database_get_national_team(read_usize(G_NATION_TABLE) + id * nation::SIZE)
}

/// The 12 nations of the 2001 Copa América, in list order (the groups interleaved: A in slots 0, 3, 6 and 9; B in 1, 4,
/// 7 and 10; C in 2, 5, 8 and 11): the globals of the nations' ids.
pub(crate) const COPA_AMERICA_2001_NATIONS: [usize; 12] = [
    KEY_NATION_COLOMBIA, KEY_NATION_BRAZIL, KEY_NATION_COSTA_RICA, KEY_NATION_CHILE, // cqs12: slots 0 to 3 (Colombia in 0)
    KEY_NATION_MEXICO, KEY_NATION_HONDURAS, KEY_NATION_ECUADOR, KEY_NATION_PERU, // cqs12: slots 4 to 7 (Mexico in 4, Peru in 7)
    KEY_NATION_URUGUAY, KEY_NATION_VENEZUELA, KEY_NATION_PARAGUAY, KEY_NATION_BOLIVIA, // cqs12: slots 8 to 11 (Bolivia in 11)
];

/// `GoldCup::set_up_team_list()` (`set_up_team_list`, thiscall, ret 0, goldcup.cpp): the 12 teams, sorted by reputation; returns
/// 1, or 0 on failure.
///
/// +0xf2 = 12 and the list at +0xee (`exe::crt_malloc`(0x30): push; add esp, 4 -> cdecl; out of memory, the error box for line
/// 0x37b and returns 0). In 2001 (+0x40 = 0x7d1): the 12 fixed ones (COPA_AMERICA_2001_NATIONS, each via `exe::database_get_national_team`) and
/// returns 1, unsorted.
///
/// In other years: the Copa America hosts ([0x9cf7a8]) of the year (`find_hosts` HostCountry::find_hosts on [0xb5e134]:
/// push 1; push &second; push &first; push year; push competition -> thiscall, ret 0x14; both start at −1; returns how
/// many, signed byte) go first; then, in nation order ([0xae23a8], [0xae2350] of them, i as a word) and while the list
/// is not full (+0xf2), the nations of continent [0x9cfa1c] (+0x71, its +0) and the guests: in 1999 (0x7cf) [0x9cf36c],
/// in other years [0x9cf280]; and always Mexico ([0x9cf3d0]); each one enters if it is not the nation of a host (+0x53
/// of the national team).
///
/// Without 12: the error box for line 0x3bc and returns 0; with 12, the list is sorted by reputation (qsort `exe::crt_qsort`
/// with 0x4b07d0: 4 pushes; add esp -> cdecl) and returns 1.
pub(crate) unsafe fn build_gold_cup_team_list(this: League) -> u32 {
    write_usize(this.addr() + goldcup::DRAWN_CLUB_COUNT, 0xc);
    let list = exe::crt_malloc(0x30);
    write_usize(this.addr() + goldcup::DRAWN_CLUBS, list);
    if list == 0 {
        show_error_at(0x9cd640, 0x37b);
        return 0;
    }
    let year = this.year() as u16;
    if year == 0x7d1 {
        covered!("cqs_12 2001 national teams");
        for (k, &g) in COPA_AMERICA_2001_NATIONS.iter().enumerate() {
            write_usize(read_usize(this.addr() + goldcup::DRAWN_CLUBS) + k * 4, national_team_of_nation_id(read_usize(g)));
        }
        return 1;
    }
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // simplification: the original pushes eax with the year in the low 16 bits and leftovers in the high ones; 0x5fa730 only reads the word
    let how_many = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_COPA_AMERICA), year as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut k = 0i16;
    while k < how_many as i16 {
        write_usize(read_usize(this.addr() + goldcup::DRAWN_CLUBS) + k as usize * 4, national_team_of_nation_id(ids[k as usize] as usize));
        k += 1;
    }
    let mut i = 0i16;
    let mut nation = read_usize(G_NATION_TABLE);
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 && (k as i32) < read_usize(this.addr() + goldcup::DRAWN_CLUB_COUNT) as i32 {
        let continent = read_usize(nation + nation::CONTINENT);
        let id = read_usize(nation);
        let enters = (continent != 0 && read_usize(continent) == read_usize(KEY_CONTINENT_SOUTH_AMERICA))
            || (year == 0x7cf && id == read_usize(KEY_NATION_JAPAN))
            || (year != 0x7cf && id == read_usize(KEY_NATION_CANADA))
            || id == read_usize(KEY_NATION_MEXICO);
        if enters {
            let l = read_usize(this.addr() + goldcup::DRAWN_CLUBS);
            let sede = (0..how_many.max(0) as usize).any(|j| read_usize(read_usize(l + j * 4) + club::NATION) == nation);
            if !sede {
                write_usize(read_usize(this.addr() + goldcup::DRAWN_CLUBS) + k as usize * 4, exe::database_get_national_team(nation));
                k += 1;
            }
        }
        i = i.wrapping_add(1);
        nation += 0x122;
    }
    if k as i32 != read_usize(this.addr() + goldcup::DRAWN_CLUB_COUNT) as i32 {
        show_error_at(0x9cd640, 0x3bc);
        return 0;
    }
    exe::crt_qsort(read_usize(this.addr() + goldcup::DRAWN_CLUBS), k as i32 as u32, 4, 0x4b07d0);
    1
}

/// rule: the Copa América has 12 nations; in 2001 a fixed list (the groups interleaved in the places: A = Colombia,
/// Chile, Ecuador and Venezuela; B = Brazil, Mexico, Peru and Paraguay; C = Costa Rica, Honduras, Uruguay and Bolivia);
/// in other years the hosts first, then the South American nations and the guests (Japan in 1999 [inference: the nation
/// of `KEY_NATION_JAPAN`], in other years that of `KEY_NATION_CANADA`, and Mexico always), without repeating the hosts,
/// up to 12, sorted by reputation; without 12, an error
/// Original: `GoldCup::set_up_team_list` (0x5e1cd0, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn set_up_team_list(this: usize) -> u32 {
    build_gold_cup_team_list(League(this))
}

/// The predicted positions of the Copa América, in order of reputation of the 12 teams.
pub(crate) const COPA_AMERICA_PREDICTED_PLACES: [u32; 12] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 0xa, 0xa, 0xa];

/// Rule: the Copa América predicted positions by reputation: 1, 2, 3, 3, four 5ths, 9 and three 10ths.
///
/// `GoldCup::set_predicted_positions()` (virtual +0x5c): without statistics (+8), the error box at line 0x419 and
/// return; a copy of the list `goldcup::DRAWN_CLUBS` (`exe::crt_malloc`(`goldcup::DRAWN_CLUB_COUNT` × 4); out of
/// memory, the error box at line 0x421 and return), sorted by reputation (qsort `exe::crt_qsort` with 0x4b07d0,
/// `goldcup::DRAWN_CLUB_COUNT` items), and each one's predicted position in the statistics
/// (`exe::comp_stats_set_club_final_position` Comp_Stats::set_club_final_position: push position; push club; mov ecx,
/// [+8] -> thiscall, ret 8): 1, 2, 3, 3, four 5, 9 and three 10 (the first 12 of the copy); the copy freed
/// (`exe::crt_free`).
/// Original: `GoldCup::set_predicted_positions` (0x5e2340, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_set_predicted_positions(this: usize) {
    if read_usize(this + 8) == 0 {
        return show_error_at(0x9cd640, 0x419);
    }
    let v = exe::crt_malloc((read_usize(this + goldcup::DRAWN_CLUB_COUNT) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9cd640, 0x421);
    }
    let n = read_usize(this + goldcup::DRAWN_CLUB_COUNT) << 2;
    core::ptr::copy_nonoverlapping(read_usize(this + goldcup::DRAWN_CLUBS) as *const u8, v as *mut u8, n);
    exe::crt_qsort(v, read_usize(this + goldcup::DRAWN_CLUB_COUNT) as u32, 4, 0x4b07d0);
    covered!("cqs_12 copa america expected");
    for (k, &p) in COPA_AMERICA_PREDICTED_PLACES.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k * 4), p);
    }
    exe::crt_free(v);
}

// simplification: the statistic finds or creates each national team’s entry (allocates): does not run twice. Original only; the plan:
// the list sorted by reputation first, and then the position (+0x72 of the entry, 0x4a4850 Comp_Stats::find_club_entry) of
// each of the 12 equal to the band’s.

/// Rule: the Copa América is every 2 years: when it ends, the table and stages are undone, the season goes to the
/// statistics history, the year goes up 2 and the rules are set up again.
///
/// `GoldCup::start_new_season()` (virtual +8), the next Copa América: `.set_season_closed(0)`, the cup table freed
/// (`exe::league_free_table` League::free_table: push 0; mov ecx, this -> thiscall, ret 4), the round table
/// `.round_dates()` freed (`exe::crt_free`) and zeroed, the season archived in the statistics
/// (`exe::comp_stats_archive_season_and_reset` Comp_Stats::archive_season_and_reset: push 1; mov ecx, `.comp_stats()`
/// -> thiscall, ret 4), stages 0..`.current_stage()` of `.stages()` destroyed (virtual 0 of each with 1: the destructor
/// that frees), the year `.year()` goes up 2, `.set_active(0)`, `.set_current_stage()` = −1 and the rules again
/// (virtual `comp_vtable::SET_UP_RULES`, thiscall, ret 0; 0: the error box at line 0x28b).
/// Original: `GoldCup::start_new_season` (0x5e1530, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_start_new_season(this: usize) {
    let this = League(this);
    this.set_season_closed(0);
    exe::league_free_table(this, 0);
    if this.round_dates() != 0 {
        exe::crt_free(this.round_dates());
        this.set_round_dates(0);
    }
    if this.comp_stats() != 0 {
        exe::comp_stats_archive_season_and_reset(this.comp_stats(), 1);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + i as usize * 4);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f)));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
    covered!("cqs_12 copa america new season");
    this.set_year(this.year().wrapping_add(2)); // cqs12: the Copa América every 2 years
    this.set_active(0);
    this.set_current_stage(usize::MAX);
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    if rules(this.addr()) == 0 {
        show_error_at(0x9cd640, 0x28b);
    }
}

// simplification: frees the tables and destroys the stages: does not run twice. Original only; the plan: the year goes up 2, +0x45 = 0,
// +0x30 = −1, +0xba = 0 and +0x4c = 0 (the rules write the round table again at +0xba: only checked that it exists
// when there was one before).

/// Team k of the drawn list (+0xee).
pub(crate) unsafe fn drawn_team(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + goldcup::DRAWN_CLUBS) + k * 4) // cqr13: o sorteado k
}

/// Rule: the Copa América has 12 nations in 3 groups of 4 (single round, 3/1 points, two qualify from each group plus
/// the two best thirds to the quarter-finals); the draw is on the start day (20 March of the cup year, moved to weekday
/// 5) and only on the call without `full`: the list of 12, the predicted positions, group A in the cup's own table, the
/// fixtures and groups B and C as stages; before that day the cup does not even run the league day.
///
/// `GoldCup::update_for_date(today, full)` (virtual +0x14; SEH frame only for the local dates, no destructor), the Copa
/// América's day. The draw date is that of 0x549ef0(`.next_draw_day()`, `.next_draw_year_offset()` + `.year()`) (lea
/// ecx, local; push year; push day -> thiscall, ret 8, pure: the day of the year and the year). Before it (today's year
/// lower, unsigned; or the same year and today's day lower, signed) the routine returns 0 without the league day. On
/// the draw day, if the draw is not done (`.active()` = 0) and WITHOUT `full` (the original draws only on the call with
/// full = 0): the list of the 12 national teams (`exe::goldcup_set_up_team_list`: mov ecx -> thiscall, ret 0), the
/// predicted positions (virtual `comp_vtable::SET_PREDICTED_POSITIONS`, thiscall, ret 0), the first group's table
/// (`exe::goldcup_create_group_table`, thiscall, ret 0; 0: the error box at line 0x3e7), the fixtures
/// (`exe::league_create_fixtures` League::create_fixtures, thiscall, ret 0; 0: the error box at line 0x3ed), the sorted
/// table (`exe::league_sort_table` push 0; mov ecx -> thiscall, ret 4; 0: the error box at line 0x3f4) (an error
/// returns 0 without the league day), the stages of groups 0 and 1 (`exe::goldcup_create_group_stage`: push g; mov ecx
/// -> thiscall, ret 4), the list `goldcup::DRAWN_CLUBS` freed (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed,
/// `goldcup::DRAWN_CLUB_COUNT` = 0 and `.set_active(1)`. Then the league day (`exe::league_update_for_date`: push full;
/// push today; mov ecx, this -> thiscall, ret 8), whose return value it passes on.
/// Original: `GoldCup::update_for_date` (0x5e2120, goldcup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn goldcup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let day = this.next_draw_day() as u16; // cqr13: the draw day
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    let day_of_today = read_u16(today);
    let year_of_today = read_u16(today + 2);
    // simplification: in the two passes before the draw and on errors the original returns only al = 0 (the rest of eax is leftover from the
    // earlier calls); the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        return 0;
    }
    if year_of_today == year && day_of_today == day && this.active() == 0 && full == 0 {
        covered!("cqr_13 copa america draw");
        exe::goldcup_set_up_team_list(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if exe::goldcup_create_group_table(this) == 0 {
            show_error_at(0x9cd640, 0x3e7);
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9cd640, 0x3ed);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0x9cd640, 0x3f4);
            return 0;
        }
        for g in 0..2i32 {
            exe::goldcup_create_group_stage(this, g);
        }
        exe::crt_free(read_usize(this.addr() + goldcup::DRAWN_CLUBS));
        write_usize(this.addr() + goldcup::DRAWN_CLUBS, 0);
        write_usize(this.addr() + goldcup::DRAWN_CLUB_COUNT, 0);
        this.set_active(1);
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the list, the predictions, the tables, the matches and the league day do not run twice. Original only; the plan in the
// draw day (the copy’s condition): +0x45 = 1, the list +0xee and +0xf2 zeroed, the first group’s table and the two
// stages created, and the current stage (+0x30) is the last of them (1).

/// Rule: group A of the Copa América has 4 nations, drawn entries 0, 3, 6 and 9 (the groups are interleaved in the
/// list), each in the row of its order; human managers get the group draw news.
///
/// `GoldCup::create_group_table()`, group A of the Copa América: 4 teams (`.set_club_count()`), the table
/// `.set_table()` (`exe::crt_malloc`(0xec): push 0xec; add esp, 4 -> cdecl; 4 rows of 0x3b bytes; out of memory, the
/// error box at line 0x1d4 and returns 0), each row i with drawn entry 3i (`goldcup::DRAWN_CLUBS`: the groups
/// interleaved) (`exe::league_init_table_row` League::init_table_row: push 0; push 0; push national team; push i; mov
/// ecx, this -> thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`: mov ecx, this ->
/// thiscall, ret 0) and returns 1.
/// Original: `GoldCup::create_group_table` (0x5e11e0, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_create_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9cd640, 0x1d4);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqr_13 first group row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, drawn_team(this, i as usize * 3), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn 3i with position i (+4).

/// Rule: groups B and C of the Copa América are league stages with drawn entries g + 1, g + 4, g + 7 and g + 10, the
/// cup's rules and year and the cup's round table for the group (6, 10 and 13 July); human managers get the news.
///
/// `GoldCup::create_group_stage(g)` (SEH frame only for the stage's new: no destructor on the normal path), the stage
/// of group g + 1 (g = 0 and 1, on the draw day). The teams in a vector of 4 pointers (`exe::crt_malloc`(0x10): push;
/// add esp, 4 -> cdecl; out of memory, the error box at line 0x2bf and 0): drawn entries g + 1, g + 4, g + 7 and g + 10
/// (`goldcup::DRAWN_CLUBS`); the round table through virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push
/// &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the stage (`exe::crt_operator_new`(0xee) push; add esp, 4 ->
/// cdecl; the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, 4,
/// the vector, 1, `.competition()`, the round table, the rounds (word), `.win_points()`, `.draw_points()`,
/// `.match_category()`, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year `.year()` (word), g, the id (word),
/// `.importance()`, 4, 0, `.league_flags()` (word), −1, 0, 2; the original pushes the words and bytes with garbage in
/// the high bytes, which the constructor does not read: the copy passes clean values) into `.stages()`[g]; the vector
/// and the round table freed (`exe::crt_free`). No stage: the error box at line 0x2d6 and 0. With it:
/// `.set_current_stage(g)`, the group news (`exe::league_send_group_draw_news` on the stage) and 1.
/// Original: `GoldCup::create_group_stage` (0x5e16b0, goldcup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn goldcup_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0x9cd640, 0x2bf);
        return 0;
    }
    for n in 0..4usize {
        covered!("cqr_13 group team");
        write_usize(teams + n * 4, drawn_team(this, (g + 1) as usize + n * 3)); // cqr13: the team in the group slot
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
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
            this.competition().addr(),
            table,
            rodadas as u16 as u32,
            this.win_points() as u32 ,
            this.draw_points() as u32 ,
            this.match_category() as u32 ,
            this.addr() + league::TIEBREAKS,
            this.addr() + league::PROMOTED_COUNT,
            this.year() as u16 as u32,
            g as u32,
            id as u16 as u32,
            this.importance() as u32 ,
            4,
            0,
            this.league_flags() as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_usize(this.stages() + g as usize * 4, stage);
    exe::crt_free(teams);
    exe::crt_free(table);
    let f = read_usize(this.stages() + g as usize * 4);
    if f == 0 {
        show_error_at(0x9cd640, 0x2d6);
        return 0;
    }
    this.set_current_stage(g as usize);
    exe::league_send_group_draw_news(f);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage at [+0xc][g], +0x30 = g, the stage with 4 teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn g + 1, g + 4, g + 7 and g + 10 (in any order: the constructor sorts the table).

/// One operation of building the round table, in the original routine's order.
///
/// `D` the date of a group round (`league_round_set_date`: round, day, month with 0 = January, years to add, weekday, code, value;
/// records of 0x41 bytes), `L` a match of the group round (`store_round_match`: round, match, a, b, c, value), `S` the draw of a
/// cup round (`cup_set_round_draw_date`: round, day, month, years, weekday; records of 0x68 bytes), `J` the match of the cup round
/// (`cup_set_round_match_date`: round, day, month, years, weekday, time, home side), `E` a day of the tie (`cup_set_round_match_day`: round, slot,
/// weekday, time, code, home side) and `B`/`W`/`X` writing a byte/word/dword at the given offset of the table.
#[derive(Clone, Copy)]
pub(crate) enum GoldcupScheduleOp {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
pub(crate) use GoldcupScheduleOp::{B as Cqp12B, D as Cqp12D, E as Cqp12E, J as Cqp12J, L as Cqp12L, S as Cqp12S, W as Cqp12W, X as Cqp12X};

/// Describes one stage of virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`).
///
/// It holds the é_liga written (if requested), the rounds, the extra (only if the pointer is not null, when
/// `extra_checa`), the table size, the error-box line when out of memory, the common operations and the stage's own
/// operations (the matches of each group) and the counter name. `None`: the stage returns 0 without writing anything.
#[derive(Clone, Copy)]
pub(crate) struct GoldcupStagePlan {
    pub(crate) league: i32,
    pub(crate) rounds: i16,
    pub(crate) extra: i16,
    pub(crate) extra_check: bool,
    pub(crate) size: u32,
    pub(crate) row: u32,
    pub(crate) ops: &'static [GoldcupScheduleOp],
    pub(crate) ops2: &'static [GoldcupScheduleOp],
    pub(crate) name: &'static str,
}

/// Builds the round table of tournament phase `f`, or returns 0 without it.
///
/// It writes the outputs, the table (`exe::crt_malloc`; when out of memory, the error box `linha` of source file `arquivo` and
/// 0) and the operations in order. The cup year (+0x40) goes as clean 16 bits to `league_round_set_date` (the original routine pushes
/// the register with garbage in the high 16 bits; `league_round_set_date` only uses the word) and signed to the cup routines.
pub(crate) unsafe fn build_phase_table(this: League, f: Option<GoldcupStagePlan>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, file: usize) -> usize {
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
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqp12D(r, dia, mes, anos, sem, cod, v) => {
                exe::league_round_set_date(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqp12L(r, j, a, b, c, v) => exe::league_round_set_match_slot(t, r, j, a, b, c, v),
            Cqp12S(r, dia, mes, anos, sem) => {
                exe::cup_set_round_draw_date(t, r, dia, mes, anos, sem, ano);
            }
            Cqp12J(r, dia, mes, anos, sem, hora, mando) => {
                exe::cup_set_round_match_date(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqp12E(r, vaga, sem, hora, cod, mando) => {
                exe::cup_set_round_match_day(t, r, vaga, sem, hora, cod, mando);
            }
            Cqp12B(o, v) => *((t + o) as *mut u8) = v,
            Cqp12W(o, v) => write_u16(t + o, v),
            Cqp12X(o, v) => write_u32(t + o, v),
        }
    }
    t
}

/// The bytes the phase operations write into table t, in operation order.
pub(crate) unsafe fn phase_table_written_bytes(ops: &[GoldcupScheduleOp], ops2: &[GoldcupScheduleOp], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqp12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * league_round_date::SIZE + league_round_date::VENUE_VALUE, 4);
            }
            Cqp12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqp12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqp12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * cup_round::SIZE + cup_round::VENUE_VALUE, 4);
            }
            Cqp12E(r, vaga, ..) => pega(r as usize * cup_round::SIZE + cup_round::SLOT_VENUE + vaga as usize * 7, 7),
            Cqp12B(o, _) => pega(o, 1),
            Cqp12W(o, _) => pega(o, 2),
            Cqp12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Side-by-side comparer for a phase table.
///
/// The original routine runs with the caller's outputs, the copy with its own outputs that start with the same values,
/// both from the same [0xb67a34]; the outputs and the bytes written to the two tables must be equal; the copy's table
/// is freed (`exe::crt_free`).
///
/// In the stage without a table only the original runs (neither writes anything; the outputs may not even exist).
pub(crate) unsafe fn compare_phase_table(name: &str, stage: u32, f: Option<GoldcupStagePlan>, rounds: *mut i16, extra: *mut i16, league: *mut i32,
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
    let table = if a != 0 && b != 0 { phase_table_written_bytes(f.ops, f.ops2, a) == phase_table_written_bytes(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        exe::crt_free(b);
    }
    SCOREBOARD.record(igual && outs && table, || format!("{}(stage {}): outs {} ({} x {}) table {}", name, stage as u8 as i8, outs, *rounds, r2, table));
    a
}

/// A Copa América, as três rodadas de cada grupo (6, 10 e 13 de julho, código 1, valor 1) com o jogo 2 vazio.
pub(crate) const COPA_AMERICA_GROUP_DATE_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 6, 6, 0, 3, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xa, 6, 0, 0, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0xd, 6, 0, 3, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa América, os jogos do grupo da fase −1.
pub(crate) const COPA_AMERICA_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 2, 2, 3, 4), Cqp12L(0, 1, 3, 2, 3, 0x80), Cqp12L(1, 0, 6, 1, 3, 0x100), Cqp12L(1, 1, 6, 2, 3, 0x40),
    Cqp12L(2, 0, 2, 1, 3, 4), Cqp12L(2, 1, 2, 2, 3, 0x80),
];
/// A Copa América, os jogos do grupo da fase 0.
pub(crate) const COPA_AMERICA_GROUP_B_MATCHES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 4, 1, 3, 0x40), Cqp12L(0, 1, 4, 2, 3, 0x100), Cqp12L(1, 0, 0, 1, 3, 4), Cqp12L(1, 1, 0, 2, 3, 0x80),
    Cqp12L(2, 0, 3, 1, 3, 0x40), Cqp12L(2, 1, 3, 2, 3, 0x100),
];
/// A Copa América, os jogos do grupo da fase 1.
pub(crate) const COPA_AMERICA_GROUP_C_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x80), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 1, 1, 3, 0x100), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x80),
];
/// A Copa América, o mata-mata em jogo único: quartas em 16/7 (com os confrontos fixos), semifinal em 19/7, final em 23/7 e terceiro lugar em 22/7.
pub(crate) const COPA_AMERICA_KNOCKOUT_OPS: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0xf, 6, 0, 5), Cqp12J(0, 0x10, 6, 0, 6, 1, 1), Cqp12E(0, 0, 6, 2, 3, 4), Cqp12E(0, 1, 6, 1, 3, 0x40),
    Cqp12E(0, 2, 0, 1, 3, 0x100), Cqp12E(0, 3, 0, 2, 3, 0x80), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x103), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x12, 6, 0, 1), Cqp12J(1, 0x13, 6, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 0x40), Cqp12E(1, 1, 2, 2, 3, 4), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x103), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x14, 6, 0, 3), Cqp12J(2, 0x17, 6, 0, 6, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 3), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x14, 6, 0, 3),
    Cqp12J(3, 0x16, 6, 0, 5, 1, 0x80), Cqp12W(0x150, 2), Cqp12W(0x141, 3), Cqp12W(0x145, 3), Cqp12W(0x13f, 0x8c),
    Cqp12W(0x143, 0), Cqp12B(0x14f, 0xa), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// A Copa Africana, as três rodadas de cada grupo (8, 12 e 16 de fevereiro) com o jogo 2 vazio.
pub(crate) const AFRICAN_NATIONS_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 8, 1, 0, 6, 1, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 0xc, 1, 0, 3, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 0x10, 1, 0, 0, 1, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// A Copa Africana, os jogos do grupo da fase −1.
pub(crate) const AFRICAN_NATIONS_GROUP_A_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 4), Cqp12L(0, 1, 6, 1, 3, 0x40), Cqp12L(1, 0, 2, 1, 3, 4), Cqp12L(1, 1, 2, 2, 3, 0x40),
    Cqp12L(2, 0, 6, 1, 3, 4), Cqp12L(2, 1, 6, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 0.
pub(crate) const AFRICAN_NATIONS_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 0, 1, 3, 4), Cqp12L(0, 1, 0, 2, 3, 0x40), Cqp12L(1, 0, 3, 1, 3, 4), Cqp12L(1, 1, 3, 2, 3, 0x40),
    Cqp12L(2, 0, 0, 1, 3, 4), Cqp12L(2, 1, 0, 1, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 1.
pub(crate) const AFRICAN_NATIONS_GROUP_C_MATCHES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 0x80), Cqp12L(0, 1, 6, 2, 3, 0x80), Cqp12L(1, 0, 2, 2, 3, 0x80),
    Cqp12L(1, 1, 3, 2, 3, 0x80), Cqp12L(2, 0, 0, 2, 3, 0x80), Cqp12L(2, 1, 0, 2, 3, 0x40),
];
/// A Copa Africana, os jogos do grupo da fase 2.
pub(crate) const AFRICAN_NATIONS_GROUP_D_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 0, 2, 3, 0x80), Cqp12L(0, 1, 1, 2, 3, 0x80), Cqp12L(1, 0, 4, 1, 3, 0x80),
    Cqp12L(1, 1, 4, 2, 3, 0x80), Cqp12L(2, 0, 1, 1, 3, 0x40), Cqp12L(2, 1, 1, 1, 3, 0x80),
];
/// A Copa Africana, o mata-mata em jogo único: quartas em 21/2, semifinal em 25/2, final em 28/2 e terceiro lugar em 27/2.
pub(crate) const AFRICAN_NATIONS_KNOCKOUT_OPS: [GoldcupScheduleOp; 80] = [
    Cqp12S(0, 0x12, 1, 0, 2), Cqp12J(0, 0x15, 1, 0, 5, 1, 1), Cqp12E(0, 0, 4, 2, 3, 0x80), Cqp12E(0, 1, 5, 1, 3, 4),
    Cqp12E(0, 2, 5, 2, 3, 0x40), Cqp12E(0, 3, 6, 1, 3, 0x40), Cqp12E(0, 4, -1, -1, -1, 0), Cqp12W(0x7, 0x78),
    Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x303), Cqp12B(0x17, 0xa), Cqp12W(0x18, 8), Cqp12W(0x1a, 4),
    Cqp12W(0x1c, 8), Cqp12W(0xf, 0), Cqp12W(0x1e, 0), Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0),
    Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0), Cqp12S(1, 0x17, 1, 0, 0), Cqp12J(1, 0x19, 1, 0, 2, 1, 1),
    Cqp12E(1, 0, 2, 1, 3, 4), Cqp12E(1, 1, 2, 2, 3, 0x80), Cqp12E(1, 2, -1, -1, -1, 0), Cqp12W(0x6f, 0x82),
    Cqp12W(0x71, 1), Cqp12W(0x73, 0), Cqp12W(0x75, 0x303), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 4), Cqp12W(0x82, 2),
    Cqp12W(0x84, 0), Cqp12W(0x77, 0), Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0),
    Cqp12X(0xc4, 0), Cqp12X(0xc8, 0), Cqp12X(0xcc, 0), Cqp12S(2, 0x1a, 1, 0, 3), Cqp12J(2, 0x1c, 1, 0, 5, 1, 4),
    Cqp12W(0xd7, 0x96), Cqp12W(0xd9, 2), Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x203), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2),
    Cqp12W(0xea, 1), Cqp12W(0xec, 0), Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1),
    Cqp12B(0xf2, 0), Cqp12X(0x12c, 0), Cqp12X(0x130, 0), Cqp12X(0x134, 0), Cqp12S(3, 0x1a, 1, 0, 3),
    Cqp12J(3, 0x1b, 1, 0, 4, 2, 0x40), Cqp12W(0x13f, 0x8c), Cqp12W(0x141, 3), Cqp12W(0x143, 0), Cqp12W(0x145, 0x203),
    Cqp12B(0x14f, 0xa), Cqp12W(0x150, 2), Cqp12W(0x152, 1), Cqp12W(0x154, 0), Cqp12W(0x147, 0), Cqp12W(0x156, 0),
    Cqp12B(0x158, 0), Cqp12B(0x159, 1), Cqp12B(0x15a, 0), Cqp12X(0x194, 0), Cqp12X(0x198, 0), Cqp12X(0x19c, 0),
];
/// The Confederations Cup: the three rounds of each group (29 July, 1 and 4 August) with match 2 empty.
pub(crate) const CONFED_CUP_GROUP_ROUNDS: [GoldcupScheduleOp; 6] = [
    Cqp12D(0, 0x1d, 6, 0, 5, 2, 1), Cqp12L(0, 2, -1, -1, -1, 0), Cqp12D(1, 1, 7, 0, 1, 1, 1),
    Cqp12L(1, 2, -1, -1, -1, 0), Cqp12D(2, 4, 7, 0, 4, 2, 1), Cqp12L(2, 2, -1, -1, -1, 0),
];
/// The Confederations Cup: the group matches of stage −1.
pub(crate) const CONFED_CUP_GROUP_A_OPS: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 6, 1, 3, 4), Cqp12L(0, 1, 6, 2, 3, 0x40), Cqp12L(1, 0, 1, 1, 3, 4), Cqp12L(1, 1, 1, 2, 3, 0x40),
    Cqp12L(2, 0, 3, 1, 3, 4), Cqp12L(2, 1, 3, 2, 3, 0x40),
];
/// The Confederations Cup: the group matches of stage 0.
pub(crate) const CONFED_CUP_GROUP_GAMES: [GoldcupScheduleOp; 6] = [
    Cqp12L(0, 0, 5, 1, 3, 0x40), Cqp12L(0, 1, 5, 2, 3, 4), Cqp12L(1, 0, 2, 1, 3, 0x40), Cqp12L(1, 1, 2, 2, 3, 4),
    Cqp12L(2, 0, 4, 1, 3, 4), Cqp12L(2, 1, 4, 2, 3, 0x40),
];
/// The Confederations Cup: the single-match knockout, with the semi-final on 6 August, the final on 9 August and the
/// third-place match on 8 August.
pub(crate) const CONFED_CUP_KNOCKOUT: [GoldcupScheduleOp; 57] = [
    Cqp12S(0, 5, 7, 0, 5), Cqp12J(0, 6, 7, 0, 6, 1, 1), Cqp12E(0, 0, 6, 1, 3, 0x40), Cqp12E(0, 1, 6, 1, 3, 4),
    Cqp12E(0, 2, -1, -1, -1, 0), Cqp12W(0x7, 0x82), Cqp12W(0x9, 0), Cqp12W(0xb, 0), Cqp12W(0xd, 0x383),
    Cqp12B(0x17, 0xa), Cqp12W(0x18, 4), Cqp12W(0x1a, 2), Cqp12W(0x1c, 4), Cqp12W(0xf, 0), Cqp12W(0x1e, 0),
    Cqp12B(0x20, 0), Cqp12B(0x21, 1), Cqp12B(0x22, 0), Cqp12X(0x5c, 0), Cqp12X(0x60, 0), Cqp12X(0x64, 0),
    Cqp12S(1, 7, 7, 0, 0), Cqp12J(1, 9, 7, 0, 2, 1, 4), Cqp12W(0x6f, 0x96), Cqp12W(0x71, 1), Cqp12W(0x73, 0),
    Cqp12W(0x75, 0x83), Cqp12B(0x7f, 0xa), Cqp12W(0x80, 2), Cqp12W(0x82, 1), Cqp12W(0x84, 0), Cqp12W(0x77, 0),
    Cqp12W(0x86, 0), Cqp12B(0x88, 0), Cqp12B(0x89, 1), Cqp12B(0x8a, 0), Cqp12X(0xc4, 0), Cqp12X(0xc8, 0),
    Cqp12X(0xcc, 0), Cqp12S(2, 7, 7, 0, 0), Cqp12J(2, 8, 7, 0, 1, 1, 0x40), Cqp12W(0xd7, 0x8c), Cqp12W(0xd9, 2),
    Cqp12W(0xdb, 0), Cqp12W(0xdd, 0x83), Cqp12B(0xe7, 0xa), Cqp12W(0xe8, 2), Cqp12W(0xea, 1), Cqp12W(0xec, 0),
    Cqp12W(0xdf, 0), Cqp12W(0xee, 0), Cqp12B(0xf0, 0), Cqp12B(0xf1, 1), Cqp12B(0xf2, 0), Cqp12X(0x12c, 0),
    Cqp12X(0x130, 0), Cqp12X(0x134, 0),
];

/// Copa America phase description by signed phase byte (groups below 2, knockout at 2).
pub(crate) fn copa_america_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f < 2 {
        let games: &'static [GoldcupScheduleOp] = match f {
            -1 => &COPA_AMERICA_GROUP_GAMES,
            0 => &COPA_AMERICA_GROUP_B_MATCHES,
            1 => &COPA_AMERICA_GROUP_C_OPS,
            _ => &[],
        };
        Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x420), extra_check: false, size: 0xc3, row: 0x9c,
            ops: &COPA_AMERICA_GROUP_DATE_OPS, ops2: games, name: "cqp_12 grupos da copa america" })
    } else if f == 2 {
        Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xd6,
            ops: &COPA_AMERICA_KNOCKOUT_OPS, ops2: &[], name: "cqp_12 mata-mata da copa america" })
    } else {
        None
    }
}

/// rule: the Copa América has three groups (stages −1, 0 and 1, ids 0x41f to 0x421) of three rounds on 6, 10 and 13
/// July, each round with the group's two fixed matches (pairings and venue); the knockout (stage 2) in single matches:
/// quarter-finals on 16/7 with fixed pairings (the two best third-placed teams [inference]), semi-final on 19/7, third
/// place on 22/7 and final on 23/7 (venue 4, neutral ground [inference]); category 10; tie-break 0x103 up to the
/// semi-final and 3 in the final; no prizes
/// Original: `GoldCup::create_schedule` (0x5e0900, goldcup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn goldcup_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), copa_america_phase(stage), rounds, extra, league, 0x9cd640)
}

/// African Nations Cup phase description (groups below 3, knockout at 3).
pub(crate) fn african_nations_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f < 3 {
        let games: &'static [GoldcupScheduleOp] = match f {
            -1 => &AFRICAN_NATIONS_GROUP_A_OPS,
            0 => &AFRICAN_NATIONS_GROUP_GAMES,
            1 => &AFRICAN_NATIONS_GROUP_C_MATCHES,
            2 => &AFRICAN_NATIONS_GROUP_D_OPS,
            _ => &[],
        };
        Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x9e,
            ops: &AFRICAN_NATIONS_GROUP_ROUNDS, ops2: games, name: "cqp_12 grupos da copa africana" })
    } else if f == 3 {
        Some(GoldcupStagePlan { league: 0, rounds: 4, extra: 0, extra_check: false, size: 0x1a0, row: 0xe0,
            ops: &AFRICAN_NATIONS_KNOCKOUT_OPS, ops2: &[], name: "cqp_12 mata-mata da copa africana" })
    } else {
        None
    }
}

/// Confederations Cup phase description (groups below 1, knockout at 1).
pub(crate) fn confed_cup_phase(stage: u32) -> Option<GoldcupStagePlan> {
    let f = stage as u8 as i8;
    if f < 1 {
        let games: &'static [GoldcupScheduleOp] = match f {
            -1 => &CONFED_CUP_GROUP_A_OPS,
            0 => &CONFED_CUP_GROUP_GAMES,
            _ => &[],
        };
        Some(GoldcupStagePlan { league: 1, rounds: 3, extra: (f as i16).wrapping_add(0x3e9), extra_check: false, size: 0xc3, row: 0x95,
            ops: &CONFED_CUP_GROUP_ROUNDS, ops2: games, name: "cqp_12 grupos da copa das confederacoes" })
    } else if f == 1 {
        Some(GoldcupStagePlan { league: 0, rounds: 3, extra: 0, extra_check: false, size: 0x138, row: 0xc7,
            ops: &CONFED_CUP_KNOCKOUT, ops2: &[], name: "cqp_12 mata-mata da copa das confederacoes" })
    } else {
        None
    }
}

/// Writes the byte/word rule fields of a national-team tournament league; the win points (+0xc2) come separately.
pub(crate) unsafe fn goldcup_write_rule_fields(this: usize, fields: &[(usize, u8, u32)], win: u8) {
    for &(o, n, v) in fields {
        if n == 1 {
            write_u8(this + o, if o == 0xc2 { win } else { v as u8 });
        } else {
            write_u16(this + o, v as u16);
        }
    }
}

/// First word of Date(day, month, competition year + offset, weekday).
pub(crate) unsafe fn competition_date_word(this: League, day: u32, month: u32, year: i16, week: u32) -> u16 {
    let mut d = [0u8; 8];
    // simplification: the year goes in a register with garbage in the high 16 bits; 0x549f70 reads it as a word
    let a = this.year().wrapping_add(year) as u16 as u32;
    exe::date_date_549f70((d.as_mut_ptr()) as *mut [u16; 4], day, month, a, week);
    core::ptr::read_unaligned(d.as_ptr() as *const u16)
}

/// Runs the original set_up_rules and checks the fields, +0xb6 date, +0xb8 and return value 1.
pub(crate) unsafe fn check_set_up_rules(name: &str, this: usize, fields: &[(usize, u8, u32)], data: (u32, u32, i16, u32), b8: u16, o: impl FnOnce() -> u32) -> u32 {
    let this = League(this);
    let word = competition_date_word(this, data.0, data.1, data.2, data.3);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in fields {
        let (lido, v) = if n == 1 { (read_u8(this.addr() + off) as u32, v & 0xff) } else { (read_u16(this.addr() + off) as u32, v & 0xffff) };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = this.next_draw_day() as u16;
    let lido8 = this.next_draw_year_offset() as u16;
    SCOREBOARD.record(r == 1 && k == 0 && b6 == word && lido8 == b8, || format!("{}({:#x}) returned {} fields errados {:x?} date {:#x} prevista {:#x} +0xb8 {:#x}", name, this.addr(), r, &errados[..k], b6, word, lido8));
    r
}

pub(crate) const COPA_AMERICA_RULE_FIELDS: [(usize, u8, u32); 15] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 0xa), (0x43, 1, 2),
    (0xc5, 1, 1), (0xc6, 1, 2), (0xc8, 1, 0), (0xbe, 1, 2), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

/// rule: the Copa America groups are single round robin, 3 points a win and 1 a draw, tiebreak by codes 1, 2 and 3
/// [inference: goal difference, goals for, wins]; two qualifiers per group; national teams; category 10 on each match;
/// 7 substitutes and 3 changes; the date of 20 of month 2 of the competition year on weekday 5 [inference: the draw
/// date]
/// Original: `GoldCup::set_up_rules` (0x5e12d0, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_set_up_rules(this: usize) -> u32 {
    covered!("cqk_10 copa america rules");
    let win = 3; // cqk10: the win in the Copa América groups
    goldcup_write_rule_fields(this, &COPA_AMERICA_RULE_FIELDS, win);
    let day = 0x14; // cqk10: the day of the Copa América date
    let word = competition_date_word(League(this), day, 2, 0, 5);
    finish_rules_setup(League(this), word, 0)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

pub(crate) const AFRICAN_NATIONS_RULE_FIELDS: [(usize, u8, u32); 14] = [(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1),
    (0xc3, 1, 1), (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

pub(crate) const OCEANIA_RULE_FIELDS: [(usize, u8, u32); 14] = [(0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0x43, 1, 2), (0xc6, 1, 2), (0x3c, 2, 1), (0xc3, 1, 1),
    (0xc4, 1, 0xa), (0xc5, 1, 1), (0xbe, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x49, 1, 7)];

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

/// rule: the Copa América moves on one stage when the current one ends; after the 2 group stages (on reaching 2) it
/// builds the knockout; going past the last stage is an error
/// Original: `GoldCup::advance_stage` (0x5e1620, goldcup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn goldcup_advance_stage(this: usize) {
    con_merc_cup_advance_stage_cqs11_next_stage(League(this), 0x9cd640, 0x29d, 2, 0x5e18a0, "cqs_11 mata-mata da copa america");
}

/// rule: the Copa America follows the African Nations Cup rule with two group stages (the knockout at stage 2)
/// Original: `GoldCup::update_staff_records_for_status` (0x5e13b0, goldcup.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn goldcup_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    national_tournament_staff_records(1, League(this), club, outcome, stage, game, a6, 2)
}

