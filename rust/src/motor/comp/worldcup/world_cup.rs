use super::*;

/// The keys of the 32 nations of the real 2002 draw (the g_key_nation_* globals, index into the nation table
/// [0xae23a8], 0x122 bytes each), in the order in which the original routine writes them to the list: groups A to H,
/// four per group.
pub(crate) const WORLD_CUP_2002_DRAW: [usize; 32] = [
    KEY_NATION_FRANCE, KEY_NATION_SENEGAL, KEY_NATION_URUGUAY, KEY_NATION_DENMARK, // A: France, Senegal, Uruguay, Denmark
    KEY_NATION_SPAIN, KEY_NATION_SLOVENIA, KEY_NATION_PARAGUAY, KEY_NATION_SOUTH_AFRICA, // B: Spain, Slovenia, Paraguay, South Africa
    KEY_NATION_BRAZIL, KEY_NATION_TURKEY, KEY_NATION_CHINA, KEY_NATION_COSTA_RICA, // C: Brasil, Turquia, China, Costa Rica
    KEY_NATION_SOUTH_KOREA, KEY_NATION_POLAND, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, // D: South Korea, Poland, United States, Portugal
    KEY_NATION_GERMANY, KEY_NATION_SAUDI_ARABIA, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_CAMEROON, // E: Germany, Saudi Arabia, Ireland, Cameroon
    KEY_NATION_ARGENTINA, KEY_NATION_NIGERIA, KEY_NATION_ENGLAND, KEY_NATION_SWEDEN, // F: Argentina, Nigeria, England, Sweden
    KEY_NATION_ITALY, KEY_NATION_ECUADOR, KEY_NATION_CROATIA, KEY_NATION_MEXICO, // G: Italy, Ecuador, Croatia, Mexico
    KEY_NATION_JAPAN, KEY_NATION_BELGIUM, KEY_NATION_RUSSIA, KEY_NATION_TUNISIA, // H: Japan, Belgium, Russia, Tunisia
];

/// Builds the World Cup draw list: the seeds into groups 1..8, the other 24 into drawn groups with a limit; in 2002,
/// the real draw and the nation table with the holder and the hosts.
///
/// `WorldCup::draw_groups()`, from the draw made on the day (`run_group_draw`); no SEH. The draw list: 32 entries of 5
/// bytes (+0 the id of the national team's club, +4 the group 1..8), zeroed (`exe::crt_calloc`(0x20, 5): push 5; push
/// 0x20; add esp, 8 -> cdecl; out of memory, the box for line 0x4b5 and it returns 0).
///
/// 1) The seeds (8 slots of 5 bytes at `world_cup_fields::DRAW_SLOT_CLUB`: +0 the id, `world_cup_fields::DRAW_SLOT_POT`
/// the pot; 0xff = already drawn): slot 0 goes to group 1 and slot 1 (+0xf3) to group 3; groups 2 and 4..8 draw one of
/// slots 2..7 (`exe::utils_random`(6) + 2), drawing again while the drawn slot has already gone. THIS IS THE LOOP OF
/// THE 27/12/2001 HANG (the game stops without an error; worked around by Alan_v2 by changing the jump at 0x92ea55): if
/// fewer than six of slots 2..7 are still standing (in a career started before 2001, `seed_teams` does not seed the
/// 2002 World Cup and the slots stay 0xff), the draw never finds a slot and the loop never ends. The port repeats the
/// loop like the original, with no limit.
///
/// 2) The other 24 (the slots at +0x116, in order) fall into drawn groups (`exe::utils_random`(8)), each group with at
/// most (k / 8) + 1 teams when the kth goes in (k from 8 to 31): it draws again while the group already has the limit.
///
/// 3) In the 2002 World Cup (`league::YEAR` = 2002), after the draw, the list becomes the real draw (the order of
/// `WORLD_CUP_2002_DRAW`: the id of the main national team's club, `exe::database_get_national_team` push nation; add
/// esp -> cdecl), groups i/4 + 1, and the table of the 32 World Cup nations ([`league::ENTRANT_BOOK`], 6 bytes: +0 the
/// club, +4 the status) receives the clubs with status 6, France (holder) 2, Korea and Japan (hosts) 1;
/// `league::ENTRANT_COUNT` (how many) = 32.
///
/// Returns the list.
/// Original: `WorldCup::draw_groups` (0x92e940, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn draw_groups(this: usize) -> usize {
    let mut count = [0i8; 8];
    let list = exe::crt_calloc(0x20, 5);
    if list == 0 {
        show_error_at(0xad5888, 0x4b5);
        return 0;
    }
    let mut s = 0i32;
    while s < 8 {
        count[s as usize] = count[s as usize].wrapping_add(1);
        if s == 0 {
            write_usize(list, read_usize(this + world_cup_fields::DRAW_SLOT_CLUB));
            write_u8(list + 4, 1);
            write_u8(this + world_cup_fields::DRAW_SLOT_POT, 0xff);
        } else if s == 2 {
            write_usize(list + 0xa, read_usize(this + 0xf3));
            write_u8(list + 0xe, 3);
            write_u8(this + 0xf7, 0xff);
        } else {
            // the loop of the 27/12/2001 crash (0x92ea55): with no standing slot among 2..7, it never ends
            let mut k = exe::utils_random(6).wrapping_add(2) as i16 as i32;
            while read_u8(this.wrapping_add((k * 5) as usize) + world_cup_fields::DRAW_SLOT_POT) == 0xff {
                k = exe::utils_random(6).wrapping_add(2) as i16 as i32;
            }
            let c = this.wrapping_add((k * 5) as usize);
            let e = list + s as usize * 5;
            write_usize(e, read_usize(c + world_cup_fields::DRAW_SLOT_CLUB));
            write_u8(e + 4, (s as u8).wrapping_add(1));
            write_u8(c + world_cup_fields::DRAW_SLOT_POT, 0xff);
        }
        s += 1;
    }
    let mut src = this + 0x116;
    let mut dst = list + 0x28;
    let mut k = 8i32;
    for _ in 0..0x18 {
        let lim = ((k / 8) as i8).wrapping_add(1); // cqf4: the limit of teams per group
        let mut r = exe::utils_random(8);
        while count[r as i16 as usize] >= lim {
            r = exe::utils_random(8);
        }
        covered!("cqf_4 place in the group");
        write_usize(dst, read_usize(src));
        src += 5;
        write_u8(dst + 4, (r as u8).wrapping_add(1));
        dst += 5;
        let g = r as i16 as usize;
        count[g] = count[g].wrapping_add(1);
        k += 1;
    }
    if read_i16(this + league::YEAR) != 0x7d2 {
        return list;
    }
    covered!("cqf_4 2002 draw");
    for (i, &chave) in WORLD_CUP_2002_DRAW.iter().enumerate() { // cqf4: the nations of the real random draw
        let nation = read_usize(G_NATION_TABLE).wrapping_add((read_usize(chave) as i32).wrapping_mul(0x122) as usize);
        write_usize(list + i * 5, read_usize(exe::database_get_national_team(nation)));
    }
    for i in 0..32usize {
        let e = list + i * 5;
        let club = read_usize(G_CLUB_TABLE).wrapping_add(read_usize(e).wrapping_mul(club::SIZE));
        write_usize(read_usize(this + league::ENTRANT_BOOK) + i * 6, club);
        write_u8(read_usize(this + league::ENTRANT_BOOK) + i * 6 + 4, 6);
        write_u8(e + 4, (i / 4) as u8 + 1);
    }
    write_u8(read_usize(this + league::ENTRANT_BOOK) + 4, 2);
    write_u8(read_usize(this + league::ENTRANT_BOOK) + 0x4c, 1);
    write_u8(read_usize(this + league::ENTRANT_BOOK) + 0xac, 1);
    write_u16(this + league::ENTRANT_COUNT, 0x20);
    list
}

/// The World Cup draw day: the draw list, group 1 in the World Cup itself with its fixtures and sorted table, the group
/// 2..8 stages, the predicted positions and the bookings reset.
///
/// `WorldCup::run_group_draw()`, from the draw day (`world_cup_update_for_date`); no SEH. The draw list
/// (`exe::worldcup_draw_groups`, mov ecx, this -> thiscall); group 1 in the World Cup's own table
/// (`exe::worldcup_create_group_table` push list; mov ecx -> thiscall, ret 4; 0: the box for line 0x46d and end); its
/// fixtures (`exe::league_create_fixtures` mov ecx -> thiscall, ret 0; 0: line 0x473); the sorted table
/// (`exe::league_sort_table` push 0; mov ecx -> thiscall, ret 4; 0: line 0x47a). With everything done:
/// `.set_active(1)`, the stages of groups 2..8 (`exe::worldcup_create_group_stage` push list; push g; mov ecx ->
/// thiscall, ret 8, g from 0 to 6), the list freed (`exe::crt_free` push; add esp -> cdecl), the predicted positions
/// (method `comp_vtable::SET_PREDICTED_POSITIONS`, `world_cup_set_predicted_positions`) and the bookings reset in the
/// competition's body (`exe::rulingbody_reset_bookings_for_club` push 0; ecx = [[`G_DISCIPLINE`] + 4 ×
/// (char)`.discipline_rule()`] -> thiscall, ret 4). On an error the list is not freed (as in the original).
/// Original: `WorldCup::run_group_draw` (0x92e7f0, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn run_group_draw(this: usize) {
    let this = League(this);
    let list = exe::worldcup_draw_groups(this);
    if exe::worldcup_create_group_table(this, list) == 0 {
        show_error_at(0xad5888, 0x46d);
        return;
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(0xad5888, 0x473);
        return;
    }
    if exe::league_sort_table(this, 0) == 0 {
        show_error_at(0xad5888, 0x47a);
        return;
    }
    this.set_active(1);
    for g in 0..7 {
        exe::worldcup_create_group_stage(this, g, list);
    }
    exe::crt_free(list);
    let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
    predicted(this.addr());
    let body = read_usize(read_usize(read_usize(G_DISCIPLINE)).wrapping_add((this.discipline_rule() as i8 as i32 * 4) as usize));
    exe::rulingbody_reset_bookings_for_club(body, 0);
}

/// The World Cup day: the seeding on the seeding date, the draw on its date (no table yet) and the league day.
///
/// `WorldCup::update_for_date(today, full)`, method +0x14 of the World Cup, the World Cup's day from the competitions'
/// day. On the seeding date (`world_cup_fields::SEEDING_DAY` the day of the year,
/// `world_cup_fields::SEEDING_YEAR_OFFSET` the year relative to `.year()`: 15/12 of the previous year, written by
/// 0x92d200) and with `full`, it seeds (`exe::worldcup_seed_teams` mov ecx -> thiscall, ret 0); on the draw date
/// (`.next_draw_day()`, `.next_draw_year_offset()`: 27/12 of the previous year), with `full` and no table yet
/// (`.table()` = 0), the draw (`exe::worldcup_run_group_draw` mov ecx -> thiscall, ret 0). Then the league day
/// (`exe::league_update_for_date` push full; push today; mov ecx -> thiscall, ret 8), whose return value (eax) it
/// passes on. The date: +0 the day of the year, +2 the year (unsigned).
/// Original: `WorldCup::update_for_date` (0x92f1a0, world_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn world_cup_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    let year = read_u16(today + 2) as i32;
    let day = read_u16(today);
    if read_i16(this.addr() + world_cup_fields::SEEDING_DAY) as u16 == day && read_i16(this.addr() + world_cup_fields::SEEDING_YEAR_OFFSET) as i32 + this.year() as i32 == year && full != 0 {
        exe::worldcup_seed_teams(this);
    }
    let day_of_draw = this.next_draw_day() as u16; // cqf4: the random draw day
    if day_of_draw == day && this.next_draw_year_offset() as i32 + this.year() as i32 == year && full != 0 && this.table() == 0 {
        covered!("cqf_4 draw day");
        exe::worldcup_run_group_draw(this);
    }
    exe::league_update_for_date(this, today, full) as u32 
}

/// Club of entry i (5 bytes, id at +0) of the draw list: [G_CLUB_TABLE] + id × 0x245.
pub(crate) unsafe fn draw_list_club(list: usize, i: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(read_usize(list + i * 5).wrapping_mul(club::SIZE))
}

/// The World Cup itself becomes group 1: four teams, the allocated table, the group 1 teams of the draw list in the
/// rows and the group draw news.
///
/// `WorldCup::create_group_table(list)`, from the draw (`run_group_draw`); no SEH. The World Cup itself becomes group
/// 1: `.set_club_count()` (teams) = 4, the table `.set_table()` allocated with 0xec bytes (`exe::crt_malloc` push; add
/// esp -> cdecl; out of memory, the box for line 0x2b1 and it returns 0); the list entries of group 1 (+4 = 1), in
/// order, go into rows 0..3 (`exe::league_init_table_row` push 0; push 0; push club; push row; mov ecx, this ->
/// thiscall, ret 0x10; the row goes in cl and the original leaves garbage in the high bytes of ecx, which
/// `exe::league_init_table_row` does not read: the port passes the clean byte), up to 4 rows or the end of the 32
/// entries; then the group news (`exe::league_send_group_draw_news` mov ecx, this -> thiscall). Returns 1.
/// Original: `WorldCup::create_group_table` (0x92d0f0, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn world_cup_create_group_table(this: usize, list: usize) -> u32 {
    let this = League(this);
    this.set_club_count((4) as i16);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0xad5888, 0x2b1);
        return 0;
    }
    let mut row = 0i8;
    let mut i = 0i8;
    while row < 4 {
        let e = i as usize; // cqf5: the list entry
        if read_u8(list + e * 5 + 4) == 1 {
            covered!("cqf_5 group one team");
            let club = draw_list_club(list, e);
            exe::league_init_table_row(this, row as u8 as u32, club, 0, (0) as u32);
            row += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    exe::league_send_group_draw_news(this);
    1
}

/// Clubs of group g (+4 = g) in the draw list, in order, up to 4.
pub(crate) unsafe fn group_clubs(list: usize, g: i32) -> Vec<usize> {
    (0..32usize).filter(|&i| read_u8(list + i * 5 + 4) as i8 as i32 == g).map(|i| draw_list_club(list, i)).take(4).collect()
}

/// Creates the stage of one of groups 2 to 8: its teams, the round schedule, the stage constructor, the current stage
/// and the group news.
///
/// `WorldCup::create_group_stage(g, list)`, from the draw (`run_group_draw`, g from 0 to 6: groups 2 to 8); SEH frame
/// only for the new: no destructor on the normal path. The teams of group g+2 (up to 4, in list order) in a vector of
/// `.club_count()` pointers (`exe::crt_malloc`; out of memory, the box for line 0x366 and 0); the round table through
/// the World Cup's method `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push g; mov ecx -> thiscall,
/// ret 0x10: `world_cup_create_schedule` writes both outputs for stages 0..6); the stage (`exe::crt_operator_new`(0xee)
/// push; add esp -> cdecl; the League_Stage constructor `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58: 22
/// arguments, checked against its reading: the World Cup, the teams (word), the vector, 1, `.competition()`, the round
/// table, the rounds (word: +0xa9 of the stage, the date count), `.win_points()`, `.draw_points()`,
/// `.match_category()`, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year `.year()` (word), g (byte), the id
/// (word: +0x3a of the stage), `.importance()`, 4, 0, `.league_flags()` (word), −1, 0, 2; the original pushes the words
/// and bytes with garbage in the high bytes, which the constructor does not read: the port passes the clean values)
/// stored in `.stages()`[g]; the vector and the round table freed (`exe::crt_free`). Without a stage: the box for line
/// 0x380 and 0. With it: `.set_current_stage(g)` and the group news (`exe::league_send_group_draw_news` on the stage).
/// Returns 1.
/// Original: `WorldCup::create_group_stage` (0x92d540, world_cup.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn world_cup_create_group_stage(this: usize, g: i32, list: usize) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0xad5888, 0x366);
        return 0;
    }
    let mut n = 0i16;
    let mut i = 0i16;
    while n < 4 {
        let e = list + i as usize * 5;
        if read_u8(e + 4) as i8 as i32 == g + 2 {
            covered!("cqf_5 group team");
            write_usize(teams + n as usize * 4, draw_list_club(list, i as usize)); // cqf5: the team in the group slot
            n += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    let rounds_of_cup: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let table = rounds_of_cup(this.addr(), g, &mut rodadas, &mut id, 0);
    let p = exe::crt_operator_new(0xee);
    let stage = if p != 0 {
        construct(
            p,
            this.addr(),
            this.club_count() as u16 as u32,
            teams,
            1,
            this.competition().addr(),
            table,
            rodadas as u16 as u32, // cqf5: the rounds (the dates, +0xa9 of the stage)
            this.win_points() as u32,
            this.draw_points() as u32,
            this.match_category() as u32,
            this.addr() + league::TIEBREAKS,
            this.addr() + league::PROMOTED_COUNT,
            this.year() as u16 as u32,
            g as u8 as u32,
            id as u16 as u32,
            this.importance() as u32,
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
    if read_usize(this.stages() + g as usize * 4) == 0 {
        show_error_at(0xad5888, 0x380);
        return 0;
    }
    this.set_current_stage(g as usize);
    exe::league_send_group_draw_news(read_usize(this.stages() + g as usize * 4));
    1
}

/// Sorts the 32 nations by reputation, sets each one's predicted position (1, 2, 3, 4, 5, 9, 17, 25 by the stage bands)
/// and makes the World Cup the nation's current competition.
///
/// `WorldCup::set_predicted_positions()`, method +0x5c of the World Cup, from the draw (`run_group_draw`). Without
/// statistics (+8): the box for line 0x671 and it returns. Fewer than 32 nations (`league::ENTRANT_COUNT`): the box for
/// line 0x678 and `league::ENTRANT_COUNT` = 32. The clubs of the nation table ([`league::ENTRANT_BOOK`], 6 bytes) in a
/// vector (`exe::crt_malloc`; out of memory, the box for line 0x680 and it returns), sorted by reputation (qsort
/// `exe::crt_qsort` with 0x4b07d0: push; add esp, 0x10 -> cdecl), and each one's predicted position in the statistics
/// (`exe::comp_stats_set_club_final_position` push position; push club; mov ecx, [+8] -> thiscall, ret 8): 1, 2, 3, 4,
/// four 5s, eight 9s, eight 17s and eight 25s (the bands of the final stages); then the current competition of each
/// club's nation (`club::NATION`) becomes the World Cup (`exe::nationalteams_set_current_competition` push [+4]; push
/// nation; mov ecx, [`G_NATIONAL_TEAMS`] -> thiscall, ret 8) and the vector is freed (`exe::crt_free`).
/// Original: `WorldCup::set_predicted_positions` (0x92f680, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn world_cup_set_predicted_positions(this: usize) {
    if read_usize(this + 8) == 0 {
        show_error_at(0xad5888, 0x671);
        return;
    }
    if read_i16(this + league::ENTRANT_COUNT) < 0x20 {
        show_error_at(0xad5888, 0x678);
        write_u16(this + league::ENTRANT_COUNT, 0x20);
    }
    let v = exe::crt_malloc(((read_i16(this + league::ENTRANT_COUNT) as i32) << 2) as u32);
    if v == 0 {
        show_error_at(0xad5888, 0x680);
        return;
    }
    let mut i = 0i16;
    while i < read_i16(this + league::ENTRANT_COUNT) {
        write_usize(v + i as usize * 4, read_usize(read_usize(this + league::ENTRANT_BOOK) + i as usize * 6));
        i += 1;
    }
    exe::crt_qsort(v, read_i16(this + league::ENTRANT_COUNT) as i32 as u32, 4, 0x4b07d0);
    const FAIXAS: [u32; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    for (k, &p) in FAIXAS.iter().enumerate() {
        exe::comp_stats_set_club_final_position(read_usize(this + 8), read_usize(v + k * 4), p);
    }
    let mut i = 0i16;
    while i < read_i16(this + league::ENTRANT_COUNT) {
        covered!("cqf_5 nation competition");
        exe::nationalteams_set_current_competition(read_usize(G_NATIONAL_TEAMS), read_usize(read_usize(v + i as usize * 4) + club::NATION), read_usize(this + 4));
        i += 1; // cqf5: the next nation
    }
    exe::crt_free(v);
}

/// The matches of each group stage in the round table (`store_round_match`: round, match, a, b, c, value), in the original's
/// order, by stage + 1 (jump table 0x92ccd0: −1 is the Cup itself, group 1; 0..6 groups 2..8).
pub(crate) const WORLD_CUP_GROUP_GAMES: [[(i32, i32, i32, i32, i32, u32); 6]; 8] = [
    [(0, 0, 2, 1, 3, 4), (0, 1, 2, 2, 3, 0x4000), (1, 0, 1, 1, 3, 0x1000), (1, 1, 1, 2, 3, 0x400), (2, 0, 1, 2, 3, 0x40), (2, 1, 1, 2, 3, 0x2000)],
    [(0, 0, 3, 1, 3, 0x1000), (0, 1, 3, 2, 3, 0x800), (1, 0, 2, 1, 3, 0x2000), (1, 1, 2, 2, 3, 0x4000), (2, 0, 1, 1, 3, 4), (2, 1, 1, 1, 3, 0x400)],
    [(0, 0, 4, 1, 3, 0x200), (0, 1, 4, 2, 3, 0x40), (1, 0, 3, 2, 3, 0x800), (1, 1, 3, 1, 3, 4), (2, 0, 2, 1, 3, 0x100), (2, 1, 2, 1, 3, 0x1000)],
    [(0, 0, 4, 1, 3, 0x4000), (0, 1, 5, 1, 3, 0x400), (1, 0, 5, 1, 3, 0x2000), (1, 1, 5, 2, 3, 0x80), (2, 0, 2, 2, 3, 0x200), (2, 1, 2, 2, 3, 0x800)],
    [(0, 0, 5, 1, 3, 0x100), (0, 1, 5, 2, 3, 4), (1, 0, 5, 1, 3, 0x1000), (1, 1, 5, 2, 3, 0x40), (2, 0, 3, 1, 3, 0x2000), (2, 1, 3, 1, 3, 0x80)],
    [(0, 0, 6, 1, 3, 0x2000), (0, 1, 0, 2, 3, 0x80), (1, 0, 6, 1, 3, 0x200), (1, 1, 6, 2, 3, 0x100), (2, 0, 3, 2, 3, 0x4000), (2, 1, 3, 2, 3, 0x400)],
    [(0, 0, 0, 1, 3, 0x40), (0, 1, 0, 2, 3, 0x100), (1, 0, 0, 1, 3, 0x4000), (1, 1, 0, 2, 3, 0x800), (2, 0, 4, 2, 3, 4), (2, 1, 4, 2, 3, 0x200)],
    [(0, 0, 6, 1, 3, 0x800), (0, 1, 6, 2, 3, 0x200), (1, 0, 5, 1, 3, 0x400), (1, 1, 6, 1, 3, 0x80), (2, 0, 4, 1, 3, 0x1000), (2, 1, 4, 1, 3, 0x100)],
];

/// Creates the three-round schedule of a World Cup group stage (12, 19 and 24 June, moved to the weekday) with the two
/// fixtures of each round; the knockout stays with the original.
///
/// `WorldCup::create_schedule(stage, &rounds, &id, &league)`, method +0x3c of the World Cup, from the creation of the
/// rules (0x92d200, stage −1) and from the group stages (`world_cup_create_group_stage`, 0..6). In the group stages
/// (char < 7): league (if requested) = 1, rounds = 3, id = 0x420 + stage; the table of 3 rounds of 0x41 bytes
/// (`exe::crt_malloc`(0xc3); out of memory, the box for line 0xcc and it returns 0), the dates
/// (`exe::league_round_set_date`: 12, 19 and 24 June of the World Cup year (`.year()`), moved to weekdays 4, 4 and 2,
/// code 1, value 1), game 2 of each round empty (−1, `exe::league_round_set_match_slot`) and the two fixtures of each
/// round of the stage (`WORLD_CUP_GROUP_GAMES`). The knockout (stage 7, with the cup routines) and the stages above:
/// the original. Returns the table.
/// Original: `WorldCup::create_schedule` (0x92c290, world_cup.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn world_cup_create_schedule(this: usize, stage: u32, rounds: *mut i16, id: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    let f = stage as u8 as i8;
    if f >= 7 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_WORLD_CUP_CREATE_SCHEDULE)(this.addr(), stage, rounds, id, league);
    }
    if !league.is_null() {
        *league = 1;
    }
    *rounds = 3; // cqf6: três rodadas
    *id = (f as i16).wrapping_add(0x420);
    let t = exe::crt_malloc(0xc3);
    if t == 0 {
        show_error_at(0xad5888, 0xcc);
        return 0;
    }
    covered!("cqf_6 round fixtures");
    // the year carries garbage in the high bytes in the original (the year’s cx over the previous ecx); 0x68a160 only uses the 16 bits
    // low (0x549f70 reads the year as a word and the word at +2 holds year − base): the copy passes the clean year
    let year = this.year() as u16 as u32;
    exe::league_round_set_date(t, 0, 0xc, 5, 0, 4, 1, year, 1); // cqf6: the first round
    exe::league_round_set_match_slot(t, 0, 2, -1, -1, -1, 0);
    exe::league_round_set_date(t, 1, 0x13, 5, 0, 4, 1, year, 1); // cqf6: a segunda rodada
    exe::league_round_set_match_slot(t, 1, 2, -1, -1, -1, 0);
    exe::league_round_set_date(t, 2, 0x18, 5, 0, 2, 1, year, 1); // cqf6: the third round
    exe::league_round_set_match_slot(t, 2, 2, -1, -1, -1, 0);
    let k = (f as i32 + 1) as u32;
    if k <= 7 {
        for &(r, j, a, b, c, v) in WORLD_CUP_GROUP_GAMES[k as usize].iter() {
            exe::league_round_set_match_slot(t, r, j, a, b, c, v);
        }
    }
    t
}

/// Collects the written bytes of a 3-round table: per round +0..+4 and +0x3d..+0x40, and each of the 3 matches
/// +5..+0xb.
pub(crate) unsafe fn written_group_round_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..3usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + league_round_date::VENUE_VALUE) as *const u8, 4));
        for j in 0..3usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + j * 7 + 5) as *const u8, 7));
        }
    }
    v
}

/// Adds a qualified nation to the World Cup nation table (status 6), or shows the box when the list is full.
///
/// `WorldCup::add_qualified_team(club)`, from the qualifiers (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200,
/// 0x920500); no SEH. With the nation list (`league::ENTRANT_COUNT`) below the maximum (`league::ENTRANT_CAPACITY`),
/// slot `league::ENTRANT_COUNT` of table [`league::ENTRANT_BOOK`] (6 bytes) receives status 6 and the club, and
/// `league::ENTRANT_COUNT` goes up; when full, the box for line 0x456.
/// Original: `WorldCup::add_qualified_team` (0x92e740, world_cup.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn add_qualified_team(this: usize, club: usize) {
    let n = read_i16(this + league::ENTRANT_COUNT);
    if n >= read_i16(this + league::ENTRANT_CAPACITY) {
        show_error_at(0xad5888, 0x456);
        return;
    }
    covered!("cqf_6 qualified nation");
    let e = read_usize(this + league::ENTRANT_BOOK).wrapping_add((n as i32 * 6) as usize);
    write_u8(e + 4, 6);
    write_usize(read_usize(this + league::ENTRANT_BOOK).wrapping_add((read_i16(this + league::ENTRANT_COUNT) as i32 * 6) as usize), club);
    write_u16(this + league::ENTRANT_COUNT, (read_i16(this + league::ENTRANT_COUNT) as u16).wrapping_add(1));
}

/// Sets the World Cup seeds (holder, hosts, reputation) and the pots by continent; does nothing for the 2002 World Cup.
///
/// `WorldCup::seed_teams()`, from the World Cup day (`world_cup_update_for_date`, on the seeding date); no SEH. In the
/// 2002 World Cup (`.year()` = 2002) it returns without doing anything: its draw slots come from the database, which is
/// why a 2002 World Cup created by another path hangs the draw (the `draw_groups` loop, cqf_4). In the others:
/// `.set_entrants_are_qualifiers(0)`; with fewer than 32 nations (`.entrant_count()`), the box for line 0x59e and it
/// completes the list with the main national team (`exe::database_get_national_team`) of each nation (`G_NATION_COUNT`
/// nations of 0x122 bytes in `G_NATION_TABLE`) not yet in it, up to 32; it sorts table `.entrant_book()` (6 bytes) by
/// reputation (qsort `exe::crt_qsort` with 0x4b07b0); the hosts (`exe::hostcountry_find_hosts` on `G_HOST_COUNTRY`:
/// thiscall, ret 0x14, with the World Cup id `KEY_CLUB_COMP_FIFA_WORLD_CUP`, the year as a word, the two outputs and 1;
/// output −1 = no host) and the holder (the most recent champion, `exe::comp_util_get_latest_placed_club` of the
/// competition `G_CLUB_COMP_TABLE` + id × 0x6b) go to slots 1, 2 and 0 (swapping places), and the others are sorted
/// again from the first free one; the first 8 become seeds (pot 0, status 3) and the other 24 go to their continent's
/// pot (`.nation()`, `.continent()`: `KEY_CONTINENT_EUROPE` pot 1,
/// `KEY_CONTINENT_OCEANIA`/`KEY_CONTINENT_ASIA`/`KEY_CONTINENT_SOUTH_AMERICA` pot 2,
/// `KEY_CONTINENT_AFRICA`/`KEY_CONTINENT_NORTH_AMERICA` pot 3; without a nation or continent, the box for line 0x61e)
/// with the club id in the slots at `world_cup_fields::DRAW_SLOT_POT` − 4 (5 bytes); finally the 24 of the table (from
/// the 9th on) and the slots at +0x116 are sorted by pot (0x92f660).
/// Original: `WorldCup::seed_teams` (0x92f230, world_cup.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn seed_teams(this: usize) {
    let this = League(this);
    if this.year() == 0x7d2 {
        covered!("cqf_6 no seeds in 2002");
        return;
    }
    this.set_entrants_are_qualifiers(0);
    if this.entrant_count() < 0x20 {
        show_error_at(0xad5888, 0x59e);
        let mut k = 0i32;
        let mut desloc = 0usize;
        if read_usize(G_NATION_COUNT) as i32 > 0 {
            while this.entrant_count() < 0x20 {
                let mut j = 0i16;
                let mut ja = false;
                while j < this.entrant_count() {
                    let c = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(desloc));
                    if read_usize(this.entrant_book() + j as usize * 6) == c {
                        ja = true;
                        break;
                    }
                    j += 1;
                }
                if !ja {
                    let c = exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(desloc));
                    write_usize(this.entrant_book() + this.entrant_count() as usize * 6, c);
                    this.set_entrant_count(((this.entrant_count() as u16).wrapping_add(1)) as i16);
                }
                k += 1;
                desloc += 0x122;
                if k >= read_usize(G_NATION_COUNT) as i32 {
                    break;
                }
            }
        }
    }
    exe::crt_qsort(this.entrant_book(), this.entrant_count() as i32 as u32, 6, 0x4b07b0);
    // simplification: both outputs start at −1 (no venue); the original leaves them with the stack leftover when 0x5fa730 does not
    // finds the Cup (it only writes them when it finds it)
    let (mut a, mut b) = (-1i32, -1i32);
    exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), this.year() as u16 as u32, &mut a, &mut b, 1);
    let sede1 = if a != -1 { exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(a.wrapping_mul(0x122) as usize)) } else { 0 };
    let sede2 = if b != -1 { exe::database_get_national_team(read_usize(G_NATION_TABLE).wrapping_add(b.wrapping_mul(0x122) as usize)) } else { 0 };
    let detentor = exe::comp_util_get_latest_placed_club(read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) as i32).wrapping_mul(0x6b) as usize), 0);
    let mut fixed = 0i8;
    for i in 0..32usize {
        for (vaga, alvo) in [(0usize, detentor), (1, sede1), (2, sede2)] {
            let t = this.entrant_book();
            let e = t + i * 6;
            if read_usize(e) == alvo {
                let (c, w) = (read_usize(e), read_i16(e + 4) as u16);
                let f = t + vaga * 6;
                write_usize(e, read_usize(f));
                write_u16(e + 4, read_i16(f + 4) as u16);
                let t = this.entrant_book() + vaga * 6;
                write_usize(t, c);
                write_u16(t + 4, w);
                if fixed < vaga as i8 + 1 {
                    fixed = vaga as i8 + 1;
                }
            }
        }
    }
    let mut n = fixed;
    if sede2 == sede1 || sede2 == detentor {
        n = 2;
    }
    if sede1 == detentor {
        n = 1;
    }
    exe::crt_qsort(this.entrant_book() + n as i32 as usize * 6, (0x20 - n as i32) as u32, 6, 0x4b07b0);
    for i in 0..0x20usize {
        let club = Club(read_usize(this.entrant_book() + i * 6));
        let pot = this.addr() + world_cup_fields::DRAW_SLOT_POT + i * 5;
        write_usize(pot - 4, read_usize(club.addr()));
        if i < 8 {
            write_u8(pot, 0);
            write_u8(this.entrant_book() + i * 6 + 4, 3);
            continue;
        }
        let nation = club.nation();
        let count = if !nation.is_null() { nation.continent() } else { 0 };
        if count == 0 {
            show_error_at(0xad5888, 0x61e);
            continue;
        }
        let c = read_usize(count);
        let (p, s) = if c == read_usize(KEY_CONTINENT_EUROPE) {
            (1, 0xa)
        } else if c == read_usize(KEY_CONTINENT_OCEANIA) || c == read_usize(KEY_CONTINENT_ASIA) || c == read_usize(KEY_CONTINENT_SOUTH_AMERICA) {
            (2, 0xb)
        } else if c == read_usize(KEY_CONTINENT_AFRICA) || c == read_usize(KEY_CONTINENT_NORTH_AMERICA) {
            (3, 0xc)
        } else {
            continue;
        };
        write_u8(pot, p);
        write_u8(this.entrant_book() + i * 6 + 4, s);
    }
    exe::crt_qsort(this.entrant_book() + 0x30, 0x18, 6, 0x92f660);
    exe::crt_qsort(this.addr() + 0x116, 0x18, 5, 0x92f660);
}

