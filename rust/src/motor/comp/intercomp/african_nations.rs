use super::*;

/// `hoje.update_for_date(completo)` (0x4025a0, thiscall, ret 8, african_nations.cpp; virtual +0x14;
/// the SEH frame is only for the local dates, with no destructor): the daily update of the African Nations Cup.
///
/// Without `completo`, only the league day runs. With it: the draw date is that of 0x549ef0(+0xb6, +0xb8 + +0x40) (lea
/// ecx, local; push year; push day -> thiscall, ret 8, pure); before it (today's year lower, unsigned; or the same year
/// and today's day lower, signed) the routine returns 0 without the league day.
///
/// On the draw date, if the draw has not been done (+0x45 = 0): the draw of the 16 (`exe::africannations_draw_finalists`: mov ecx -> thiscall, ret
/// 0; the return value is not read), the predicted positions (virtual +0x5c, thiscall, ret 0), group A's table
/// (`init_group_a_table`, thiscall, ret 0; 0: the error box for line 0x3c2), the fixtures (`create_fixtures` League::create_fixtures,
/// thiscall, ret 0; 0: the error box for line 0x3c8), the sorted table (`sort_table` push 0; mov ecx -> thiscall, ret 4; 0:
/// the error box for line 0x3cf) (an error returns 0 without the league day), the stages of groups B to D (`exe::africannations_create_group_stage`:
/// push g; mov ecx -> thiscall, ret 4, g = 0..2), the list at +0xee freed (`exe::crt_free`: push; add esp, 4 -> cdecl) and
/// zeroed, +0xf2 (word) = 0 and +0x45 = 1.
///
/// Then the league day (`league_update_for_date`: push completo; push hoje; mov ecx, this -> thiscall, ret 8), whose return value it
/// passes on.
pub(crate) unsafe extern "thiscall" fn african_nations_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    if full == 0 {
        return exe::league_update_for_date(this, today, 0) as u32;
    }
    let day = this.next_draw_day() as u16;
    let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
    let day_of_today = read_u16(today);
    let year_of_today = read_u16(today + 2);
    // simplification: in the passes before the draw and on errors the original returns only al = 0 (the rest of eax is leftover from the
    // earlier calls); the caller reads the byte
    if year_of_today < year || (year_of_today == year && (day_of_today as i16) < day as i16) {
        return 0;
    }
    if year_of_today == year && day_of_today == day && this.active() == 0 {
        covered!("cqs_13 african cup draw day");
        exe::africannations_draw_finalists(this);
        let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
        predicted(this.addr());
        if exe::africannations_init_group_a_table(this) == 0 {
            show_error_at(0x9870f8, 0x3c2);
            return 0;
        }
        if exe::league_create_fixtures(this) == 0 {
            show_error_at(0x9870f8, 0x3c8);
            return 0;
        }
        if exe::league_sort_table(this, 0) == 0 {
            show_error_at(0x9870f8, 0x3cf);
            return 0;
        }
        for g in 0..3i32 {
            exe::africannations_create_group_stage(this, g);
        }
        exe::crt_free(read_usize(this.addr() + african_nations::DRAWN_CLUBS));
        write_usize(this.addr() + african_nations::DRAWN_CLUBS, 0);
        write_u16(this.addr() + african_nations::DRAWN_CLUB_COUNT, 0);
        this.set_active(1);
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the draw, the predictions, the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee and +0xf2 zeroed, the group A table and the three stages
// created, and the current stage (+0x30) is the last of them (2).

/// `AfricanNations::draw_finalists()` (`exe::africannations_draw_finalists`, thiscall, ret 0, african_nations.cpp): hosts first, then African
/// nations by reputation, drawn from 4 pots into 4 groups of 4 stored in +0xee.
///
/// +0xf2 (word) = 16 and the list +0xee of 50 (`exe::crt_malloc`(0xc8): push; add esp, 4 -> cdecl; out of memory, the box for
/// line 0x349 and returns 0).
///
/// First the hosts of the African Cup of Nations ([0x9cf7ac]) in year +0x40 (`find_hosts` HostCountry::find_hosts on
/// [0xb5e134]: push 1; push &second; push &first; push year; push competition -> thiscall, ret 0x14; returns how many,
/// signed byte; the outputs are only read up to the count), each one via its national team (`exe::database_get_national_team`
/// database_get_national_team: push nation; add esp -> cdecl; the nations in [0xae23a8], 0x122 bytes).
///
/// Then, in nation order ([0xae2350], i as a word), those of the African continent ([0x9cfa08]; the nation's +0x71, its
/// +0) while there are fewer than 50, except the hosts (+0x53 of the national team). Fewer than 16: returns 0 (no box;
/// the list stays in +0xee).
///
/// Those after the hosts are sorted by reputation (qsort `exe::crt_qsort` with 0x4b07d0: 4 pushes; add esp -> cdecl).
///
/// The groups (`exe::crt_malloc`(0x40); out of memory, the box for line 0x380 and returns 0): for each group j (0..3) and pot p
/// (0..3), one randomly drawn (`exe::utils_random`(4): push 4; add esp -> cdecl; the signed byte; again while the pot's slot was
/// already taken) among the 4 of the pot (the list in slots 4p to 4p + 3) goes to slot 4j + p of the groups and leaves
/// the pot (zeroed).
///
/// The list is freed (`exe::crt_free`) and +0xee = the groups. Returns 1.
pub(crate) unsafe fn draw_african_nations_finalists(this: League) -> u32 {
    write_u16(this.addr() + african_nations::DRAWN_CLUB_COUNT, 0x10);
    let list = exe::crt_malloc(0xc8);
    write_usize(this.addr() + african_nations::DRAWN_CLUBS, list);
    if list == 0 {
        show_error_at(0x9870f8, 0x349);
        return 0;
    }
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // simplification: the original pushes eax with the year in the low 16 bits and leftovers in the high ones; 0x5fa730 only reads the word
    let how_many = exe::hostcountry_find_hosts(read_usize(G_HOST_COUNTRY), read_usize(KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS), this.year() as u16 as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as i8;
    let mut n = 0usize;
    if how_many > 0 {
        for &id in ids.iter().take(how_many as usize) {
            write_usize(read_usize(this.addr() + african_nations::DRAWN_CLUBS) + n * 4, exe::database_get_national_team(read_usize(G_NATION_TABLE) + id as usize * nation::SIZE));
            n += 1;
        }
    }
    let mut i = 0i16;
    let mut nation = read_usize(G_NATION_TABLE);
    while (i as i32) < read_usize(G_NATION_COUNT) as i32 {
        let continent = read_usize(nation + nation::CONTINENT);
        if continent != 0 && read_usize(continent) == read_usize(KEY_CONTINENT_AFRICA) && n < 0x32 { // cqs13: up to 50 African ones
            let l = read_usize(this.addr() + african_nations::DRAWN_CLUBS);
            let sede = (0..how_many.max(0) as usize).any(|j| read_usize(read_usize(l + j * 4) + club::NATION) == nation);
            if !sede {
                write_usize(read_usize(this.addr() + african_nations::DRAWN_CLUBS) + n * 4, exe::database_get_national_team(nation));
                n += 1;
            }
        }
        i = i.wrapping_add(1);
        nation += 0x122;
    }
    if n < 0x10 {
        return 0;
    }
    let h = how_many as i32;
    exe::crt_qsort(read_usize(this.addr() + african_nations::DRAWN_CLUBS) + (h as isize * 4) as usize, (n as i32 - h) as u32, 4, 0x4b07d0); // cqs13: the non-hosts by reputation
    let groups = exe::crt_malloc(0x40);
    if groups == 0 {
        show_error_at(0x9870f8, 0x380);
        return 0;
    }
    covered!("cqs_13 african cup draw");
    for j in 0..4usize {
        for p in 0..4usize {
            let mut r = exe::utils_random(4) as u8 as i8 as usize;
            while read_usize(read_usize(this.addr() + african_nations::DRAWN_CLUBS) + (r + 4 * p) * 4) == 0 {
                r = exe::utils_random(4) as u8 as i8 as usize;
            }
            let e = read_usize(this.addr() + african_nations::DRAWN_CLUBS) + (r + 4 * p) * 4;
            write_usize(groups + (4 * j + p) * 4, read_usize(e)); // cqs13: the one from pot p in slot 4j + p
            write_usize(e, 0);
        }
    }
    exe::crt_free(read_usize(this.addr() + african_nations::DRAWN_CLUBS));
    write_usize(this.addr() + african_nations::DRAWN_CLUBS, groups);
    1
}

/// AfricanNations::draw_finalists(): hook entry that delegates to draw_african_nations_finalists, which takes the hosts
/// plus the best-reputed African nations and draws the 16 finalists into 4 groups of 4 by pots; returns 1 on success, 0
/// with fewer than 16 nations or out of memory.
pub(crate) unsafe extern "thiscall" fn draw_finalists(this: usize) -> u32 {
    draw_african_nations_finalists(League(this))
}

/// `g.create_group_stage()` (`0x401ea0`, thiscall, ret 4, `african_nations.cpp`): creates the stage of
/// group g + 1 (B to D, g = 0..2) of the African Nations Cup on draw day.
///
/// SEH frame only for the stage's `new`, with no destructor on the normal path.
///
/// The teams go into a vector of 4 pointers (`0x944e46(0x10)`: push; add esp, 4 -> cdecl; out of memory, the box of
/// line `0x2dc` and 0): the drawn teams 4g + 4 to 4g + 7 (`+0xee`); the round table via virtual `+0x3c` (push 0; push
/// &id; push &rodadas; push g; mov ecx, this -> thiscall, ret 0x10); the stage (`0x944cf1(0xee)` push; add esp, 4 ->
/// cdecl; the League_Stage constructor `0x68b100`, thiscall, ret 0x58, 22 arguments: the cup, 4, the vector, 1, `+4`,
/// the round table, the rounds (word), `+0xc2`, `+0xc3`, `+0xc4`, &`+0xc5`, &`+0xbe`, the year `+0x40` (word), g, the
/// id (word), `+0x51`, 4, 0, `+0xd9` (word), −1, 0, 2; the original routine pushes the words and bytes with garbage in
/// the high bytes, which the constructor does not read: the copy passes the clean values) into [`+0xc`][g]; the vector
/// and the round table are freed (`0x9452ca`).
///
/// Without the stage: the box of line `0x2f2` and 0. With it: `+0x30` = g, the group news (`0x684230` on the stage) and
/// 1.
pub(crate) unsafe extern "thiscall" fn african_nations_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(0x10);
    if teams == 0 {
        show_error_at(0x9870f8, 0x2dc);
        return 0;
    }
    for n in 0..4usize {
        covered!("cqs_13 group team");
        write_usize(teams + n * 4, read_usize(read_usize(this.addr() + african_nations::DRAWN_CLUBS) + ((g as usize + 1) * 4 + n) * 4)); // cqs13: the team in the group slot
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
        show_error_at(0x9870f8, 0x2f2);
        return 0;
    }
    this.set_current_stage(g as usize);
    exe::league_send_group_draw_news(f);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage at [+0xc][g], +0x30 = g, the stage with 4 teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn 4g + 4 to 4g + 7 (in any order: the constructor sorts the table).

/// rule: the African Nations Cup has three group stages (stages 0 to 2, one per group round [inference]) and, at stage
/// 3, the knockout; going past the last stage is an error (the box)
/// Original: `AfricanNations::advance_stage` (0x401e10, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_advance_stage(this: usize) {
    let this = League(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        show_error_at(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    this.set_current_stage((f) as usize);
    if f == 3 { // cqk12: the African Cup knockout after the groups
        covered!("cqk_12 african cup knockout");
        exe::africannations_create_knockout_stage(this);
    }
}

// simplification: the knockout (0x402090) allocates the stage and draws: does not run twice. Original only; the plan: +0x30 goes up one
// (outside the last stage, which gives the message box).

/// Builds the group A table of the African Nations Cup. Rule: group A has four national teams, the first four of the
/// draw list, in order; the managers get the draw news.
///
/// Ret 0: `AfricanNations::init_group_a_table()`: group A with 4 teams (+0x3e), table +0xb1 (`exe::crt_malloc`(0xec): push 0xec;
/// add esp, 4 -> cdecl; 4 rows of 0x3b bytes; out of memory: the box at line 0x1f1 and returns 0), each row i with
/// national side i of the draw list (+0xee) (`init_table_row`: push 0; push 0; push national side; push i; mov ecx, this ->
/// thiscall, ret 0x10), the group draw news (`send_group_draw_news`: mov ecx, this -> thiscall, ret 0) and returns 1.
/// Original: `AfricanNations::init_group_a_table` (0x4019c0, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn init_group_a_table(this: usize) -> u32 {
    let this = League(this);
    let teams = 4i16; // cqk12: the four of group A
    this.set_club_count(teams);
    let t = exe::crt_malloc(0xec);
    this.set_table((t as u32) as usize);
    if t == 0 {
        show_error_at(0x9870f8, 0x1f1);
        return 0;
    }
    covered!("cqk_12 african cup group a");
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        let national_team = read_usize(read_usize(this.addr() + african_nations::DRAWN_CLUBS).wrapping_add((i as isize * 4) as usize)); // cqk12: the national team in the row
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, national_team, 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the position history) and the news goes to the managers: does not run
// times. Original only; the plan: 4 teams, the table, and in each row i the national team i of those drawn (+0xee) with position i
// (+4 of the row, 0x3b bytes).

/// rule: the season honours (winner, runner-up, third) are the knockout ones
/// Original: `AfricanNations::record_season_honours` (0x401b90, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_record_season_honours(this: usize) {
    let this = League(this);
    let stage = read_usize(this.stages() + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(stage) + comp_vtable::RECORD_SEASON_HONOURS));
    h(stage);
}

// simplification: the history (the knockout’s virtual +0x30, 0x519a90 already in Rust with its own comparer) does not run twice;
// the routine writes nothing beyond the jump. Original only.

/// Calls a staff-records final routine f (winner/runner-up/third) on the staff records with club, match and a6.
pub(crate) unsafe fn african_nations_record_final_result(f: usize, club: usize, game: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(eng_second_staff_records(), club, game, a6);
}

/// Virtual +0x44 of a national-team tournament with groups and knockout (the African Nations Cup and the Copa América):
/// writes the qualification, elimination and final-placing records by outcome and stage; returns 0.
///
/// `fase` (signed byte) below `grupos` is the group stage: 6, qualification (`add_qualified_for_round_record`: stage 0, round 0x78, weight
/// 0x19); 0xfe, non-qualification (`add_failed_to_qualify_record`: stage 0x41c, weight 0x19), which ends the call-ups.
///
/// At stage `grupos` (the knockout, stage [+0xc][grupos]): 0, qualification with the match's stage +0x32, the +0x6f of
/// the match's round (+0x34) in the stage's round table (+0xa3, 0x68 bytes) and the weight `comp_util_scale_by_round`(0x28, 0x64, round,
/// 3); 0xff, elimination (`add_knocked_out_record`) with +0x32, +0x30 and the same weight; 1, 2 and 3, the champion, runner-up and
/// third-place records.
///
/// Returns 0.
pub(crate) unsafe fn national_tournament_staff_records(name: u8, this: League, club: usize, outcome: u32, stage: u32, game: usize, a6: u32, groups: i8) -> u32 {
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    if f < groups {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if name == 0 {
                covered!("cqj_10 african out in groups");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19); // cqj10: those knocked out in the African Cup groups
            } else {
                covered!("cqj_10 copa america out in groups");
                european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x19); // cqj10: those knocked out in the Copa América groups
            }
        }
    } else if f == groups {
        match d {
            0 => {
                let r = read_i16(game + fixture::ROUND_NUMBER);
                let weight = scale_by_round(0x28, 0x64, r, 3);
                let knockout = Cup(read_usize(this.stages() + groups as usize * 4));
                let round = read_i16(knockout.round_table().wrapping_add((r as isize * (cup_round::SIZE as isize)) as usize) + cup_round::ROUND_CODE + 0x68) as u16 as u32;
                european_cup_add_qualified_record(club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, round, weight);
            }
            0xff => {
                let weight = scale_by_round(0x28, 0x64, read_i16(game + fixture::ROUND_NUMBER), 3);
                // simplification: +0x32 and +0x30 go in registers with garbage in the high 16 bits; 0x86c000 only uses the words
                if name == 0 {
                    covered!("cqj_10 african out in knockout");
                    european_cup_add_knocked_out_record(club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, read_i16(game + fixture::ROUND_NAME) as u16 as u32, weight); // cqj10: those knocked out in the African Cup knockout
                } else {
                    european_cup_add_knocked_out_record(club, comp, read_i16(game + fixture::ROUND_PREFIX) as u16 as u32, read_i16(game + fixture::ROUND_NAME) as u16 as u32, weight);
                }
            }
            1 => african_nations_record_final_result(0x86a800, club, game, a6),
            2 => african_nations_record_final_result(0x86b0b0, club, game, a6),
            3 => african_nations_record_final_result(0x86b710, club, game, a6),
            _ => {}
        }
    }
    0
}

/// rule: in the three African Nations Cup group stages, a team going through gets the qualification record (weight 25)
/// and one going out the failed-to-qualify record (weight 25), which ends the team's call-ups; in the knockout (stage
/// 3), a team going through gets the qualification by round (weight 40 to 100), one going out the knocked-out record
/// (which also ends the call-ups), and the winner, runner-up and third place the final records
/// Original: `AfricanNations::update_staff_records_for_status` (0x401ba0, african_nations.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn african_nations_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    national_tournament_staff_records(0, League(this), club, outcome, stage, game, a6, 3)
}

// simplification: the records go to the managers’ history and end the call-ups: they do not run twice, and the routine
// writes nothing besides them. Original only; the plan checks the return of 0.

// simplification: like the African Cup one: original only, return value 0.

/// Ends the national team's current competition (NationalTeams::clear_current_competition for the club's nation).
pub(crate) unsafe fn clear_national_team_competition(club: Club) {
    exe::nationalteams_clear_current_competition(read_usize(G_NATIONAL_TEAMS), club.nation().addr());
}

// simplification: the records, the end of the national team’s competition and the World Cup place do not run twice. Original only;
// the plan: return value 0 and, on a won play-off, *saida = 4 and the World Cup list (+0x36) one more (if it fit).

/// rule: the African Nations Cup has four groups (stages −1 to 2, ids 0x3e8 to 0x3eb) of three rounds on 8, 12 and 16
/// February, with each group's fixed matches; the knockout (stage 3) in single matches: quarter-finals on 21/2 with
/// fixed pairings, semi-final on 25/2, third place on 27/2 and final on 28/2 (venue 4); category 10; tie-break 0x303 up
/// to the semi-final and 0x203 in the final; no prizes
/// Original: `AfricanNations::create_schedule` (0x4012e0, african_nations.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn african_nations_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    build_phase_table(League(this), african_nations_phase(stage), rounds, extra, league, 0x9870f8)
}

/// rule: the African Nations Cup groups are single round robin, 3/1 points, tiebreak 1, 2, 3, two qualifiers per group,
/// category 10, 7 substitutes and 3 changes; the date of 5 of month 9 of the year before the tournament on weekday 5
/// [inference: the draw in the previous year] and +0xb8 = −1
/// Original: `AfricanNations::set_up_rules` (0x401ab0, african_nations.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn african_nations_set_up_rules(this: usize) -> u32 {
    covered!("cqk_10 african cup rules");
    let win = 3; // cqk10: the win in the African Cup groups
    goldcup_write_rule_fields(this, &AFRICAN_NATIONS_RULE_FIELDS, win);
    let day = 5; // cqk10: the day of the African Cup date
    let word = competition_date_word(League(this), day, 9, -1, 5);
    finish_rules_setup(League(this), word, 0xffff)
}

