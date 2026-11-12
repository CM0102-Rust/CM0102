use super::*;

/// Club k of the drawn list (+0xee pointers).
pub(crate) unsafe fn conmebol_liber_drawn_club(this: League, k: usize) -> usize {
    read_usize(read_usize(this.addr() + conmebol_liber::DRAWN_CLUBS) + k * 4) // cqr10: o sorteado k
}

/// rule: the Libertadores has 32 clubs (those the South American rules put in it, shuffled) in 8 groups of 4, home and
/// away (3 points a win, 1 a draw; two qualify from each group, to the round of 16); the draw is on the start day (15
/// January of the cup year): group 1 stays in the cup's own table (drawn clubs 0 to 3, in order) and groups 2 to 8 are
/// stages (drawn clubs 4g + 4 to 4g + 7); on the draw day: the clubs (`conmebol_liber_select_qualified_clubs` (the 32 clubs with the Libertadores as
/// their season competition, shuffled)), the group 1 table, the fixtures, the sorted table, the other groups' stages
/// and the predicted positions; then the league day
/// Original: `ConmebolLiber::update_for_date` (0x4c4410, conmebol_liber.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_update_for_date(this: usize, today: usize, full: u32) -> u32 {
    let this = League(this);
    if this.active() == 0 && full != 0 {
        let day = this.next_draw_day() as u16; // cqr10: the draw day
        let year = (this.next_draw_year_offset() as u16).wrapping_add(this.year() as u16);
        if year == read_u16(today + 2) && day == read_u16(today) {
            covered!("cqr_10 libertadores draw");
            exe::conmebolliber_select_qualified_clubs(this);
            if exe::conmebolliber_build_first_group_table(this) == 0 {
                show_error_at(0x9a7de4, 0x365);
            }
            if exe::league_create_fixtures(this) == 0 {
                show_error_at(0x9a7de4, 0x36a);
            }
            if exe::league_sort_table(this, 0) == 0 {
                show_error_at(0x9a7de4, 0x370);
            }
            for g in 0..7i32 {
                exe::conmebolliber_create_group_stage(this, g);
            }
            let predicted: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
            predicted(this.addr());
            exe::crt_free(read_usize(this.addr() + conmebol_liber::DRAWN_CLUBS));
            write_usize(this.addr() + conmebol_liber::DRAWN_CLUBS, 0);
            this.set_active(1);
        }
    }
    exe::league_update_for_date(this, today, full) as u32 
}

// simplification: the clubs (shuffled), the tables, the matches and the league day do not run twice. Original only; the plan
// on the draw day (the copy’s condition): +0x45 = 1, the list +0xee zeroed, the first group’s table and the 7 stages
// created and the current stage (+0x30) the last of them.

/// Builds the table of the first group of the Copa Libertadores. Rule: group 1 has 4 clubs, the first 4 drawn, each in
/// the row of its order; human managers get the group draw news.
///
/// Ret 0: 4 teams (`set_club_count`), table `set_table` (`exe::crt_malloc`(0xec): push 0xec; add esp, 4 -> cdecl; 4
/// rows of 0x3b bytes; out of memory: the box at line 0x16f and returns 0), each row i with drawn club i
/// (`conmebol_liber_drawn_club`) (`exe::league_init_table_row`: push 0; push 0; push club; push i; mov ecx, this ->
/// thiscall, ret 0x10), the group draw news (`exe::league_send_group_draw_news`: mov ecx, this -> thiscall, ret 0) and
/// returns 1. The same body as `init_group_a_table` of the African Nations Cup (only the file and the box line change).
/// Original: `ConmebolLiber::build_first_group_table` (0x4c37a0, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn build_first_group_table(this: usize) -> u32 {
    let this = League(this);
    this.set_club_count(4);
    let t = exe::crt_malloc(0xec);
    this.set_table(t);
    if t == 0 {
        show_error_at(0x9a7de4, 0x16f);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < this.club_count() {
        covered!("cqr_10 first group row");
        // simplification: the index goes in a dword whose 3 high bytes are stack leftovers; 0x687430 reads only the byte (char)
        exe::league_init_table_row(this, i as u8 as u32, conmebol_liber_drawn_club(this, i as usize), 0, 0);
        i = i.wrapping_add(1);
    }
    exe::league_send_group_draw_news(this);
    1
}

// simplification: the table is allocated (and each row allocates the history) and the news goes to the managers: does not run twice. Only the
// original; the plan: returns 1, 4 teams and in row i (0x3b bytes) the drawn team i with position i (+4).

/// Creates a group stage of the Copa Libertadores. Rule: each of the other groups is a league stage with the next 4
/// drawn clubs (4g + 4 to 4g + 7), the cup's rules and year and the cup's round table for the group; human managers get
/// the news.
///
/// Ret 4; SEH frame only for the stage's `exe::crt_operator_new` (no destructor on the normal path): the stage of group
/// g + 1 (g from 0 to 6, from the draw day). The teams in a vector of `club_count()` pointers
/// (`exe::crt_malloc`(`club_count()` << 2): push; add esp, 4 -> cdecl; out of memory: the box at line 0x218 and 0):
/// drawn clubs 4g + 4 to 4g + 7 (`conmebol_liber_drawn_club`); the round table through virtual
/// `comp_vtable::CREATE_SCHEDULE` (push 0; push &id; push &rounds; push g; mov ecx, this -> thiscall, ret 0x10); the
/// stage (`exe::crt_operator_new`(0xee) push; add esp, 4 -> cdecl; the League_Stage constructor
/// `FN_LEAGUE_STAGE_LEAGUE_STAGE`, thiscall, ret 0x58, 22 arguments: the cup, the teams (`club_count()`, word), the
/// vector, `legs()` (word), `competition()`, the round table, the rounds (word), `win_points()`, `draw_points()`,
/// `match_category()`, &`league::TIEBREAKS`, &`league::PROMOTED_COUNT`, the year `year()` (word), g, the id (word),
/// `importance()`, 2, 0, `league_flags()` (word), −1, 0, 2; the original pushes the words and bytes with garbage in the
/// high bytes, which the constructor does not read: the copy passes clean values) in `stages()[g]`; the vector and the
/// round table are freed (`exe::crt_free`). No stage: the box at line 0x22e and 0. With it: the group news
/// (`exe::league_send_group_draw_news` on the stage), `set_current_stage(g)` and 1.
/// Original: `ConmebolLiber::create_group_stage` (0x4c3b10, conmebol_liber.cpp, thiscall, ret 4).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_group_stage(this: usize, g: i32) -> u32 {
    let this = League(this);
    #[allow(clippy::type_complexity)]
    let construct: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let teams = exe::crt_malloc(((this.club_count() as i32) << 2) as u32);
    if teams == 0 {
        show_error_at(0x9a7de4, 0x218);
        return 0;
    }
    let mut k = g * 4 + 4;
    while k < g * 4 + 8 {
        let n = (k - (g * 4 + 4)) as usize;
        covered!("cqr_10 group team");
        write_usize(teams + n * 4, conmebol_liber_drawn_club(this, k as usize)); // cqr10: the team in the group slot
        k += 1;
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
            this.legs() as u16 as u32,
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
            2,
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
        show_error_at(0x9a7de4, 0x22e);
        return 0;
    }
    exe::league_send_group_draw_news(f);
    this.set_current_stage(g as usize);
    1
}

// simplification: the stage is allocated and the news goes to the managers: does not run twice. Original only; the plan: returns 1 with the
// stage in [+0xc][g], +0x30 = g, the stage with the teams and the cup year and, in its table (+0xb1, rows of 0x3b bytes), the
// drawn 4g + 4 to 4g + 7 (in any order: the constructor sorts the table).

/// The rules of a continental group (virtual +0x8c, set_up_rules, on a group league): the byte and word fields written
/// (offset, size 1 or 2, value), the start date (`exe::date_date_549f70`: day, month, competition year +0x40, weekday; −1 = any) and
/// the prize fund (+0x28), when there is one.
pub(crate) struct ContinentalGroupRules {
    pub(crate) fields: &'static [(usize, u8, u32)],
    pub(crate) day: u32,
    pub(crate) month: u32,
    pub(crate) week: u32,
    pub(crate) prize: Option<u32>,
}

/// Writes the rule fields of `g` and the round table; returns 1.
///
/// The win points (`+0xc2`) come from `vitoria`. The start date goes into a local Date (`0x549f70`: push week; push
/// year; push month; push day; lea ecx, local -> thiscall, ret 0x10) and its first word into `+0xb6`, `+0xb8` = 0; then
/// the prize; then the round table from virtual `+0x3c` (push 0; push &`+0x3a`; push &`+0xa9`; push −1; mov ecx, this
/// -> thiscall, ret 0x10) into `+0xba`. Returns 1.
pub(crate) unsafe fn apply_cup_rules(this: League, g: &ContinentalGroupRules, day: u32, win: u8) -> u32 {
    for &(o, n, v) in g.fields {
        let v = if o == 0xc2 { win as u32 } else { v };
        if n == 1 {
            *((this.addr() + o) as *mut u8) = v as u8;
        } else if n == 2 {
            core::ptr::write_unaligned((this.addr() + o) as *mut u16, v as u16);
        } else {
            core::ptr::write_unaligned((this.addr() + o) as *mut u32, v);
        }
    }
    let mut d = [0u8; 8];
    // simplification: the year goes in a register that may have garbage in the high 16 bits; 0x549f70 reads it as a word
    exe::date_date_549f70((d.as_mut_ptr()) as *mut [u16; 4], day, g.month, (this.year() as u16) as u32, g.week);
    this.set_next_draw_day((core::ptr::read_unaligned(d.as_ptr() as *const u16)) as i16);
    this.set_next_draw_year_offset((0) as i16);
    if let Some(p) = g.prize {
        this.set_prize_fund((p) as i32);
    }
    let vt = core::ptr::read_unaligned(this.addr() as *const usize);
    let table: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = table(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
    this.set_round_dates((t as u32) as usize);
    1
}

/// Comparer: runs the original and checks rule fields, date word, +0xb8, prize and return 1.
pub(crate) unsafe fn check_cup_rules(name: &str, this: League, g: &ContinentalGroupRules, o: impl FnOnce() -> u32) -> u32 {
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u8; 8];
    data(d.as_mut_ptr(), g.day, g.month, (this.year() as u16) as u32, g.week);
    let word = core::ptr::read_unaligned(d.as_ptr() as *const u16);
    let r = o();
    let mut errados: [(usize, u32); 4] = [(0, 0); 4];
    let mut k = 0;
    for &(off, n, v) in g.fields {
        let lido = match n {
            1 => *((this.addr() + off) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this.addr() + off) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this.addr() + off) as *const u32),
        };
        let v = if n == 1 { v & 0xff } else if n == 2 { v & 0xffff } else { v };
        if lido != v && k < 4 {
            errados[k] = (off, lido);
            k += 1;
        }
    }
    let b6 = this.next_draw_day() as u16;
    let b8 = this.next_draw_year_offset() as u16;
    let premio = g.prize.map_or(true, |p| (this.prize_fund() as u32) == p);
    SCOREBOARD.record(r == 1 && k == 0 && b6 == word && b8 == 0 && premio, || format!("{}({:#x}) returned {} fields errados {:x?} date {:#x} prevista {:#x} +0xb8 {} prize ok {}", name, this.addr(), r, &errados[..k], b6, word, b8, premio));
    r
}

pub(crate) const LIBERTADORES_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0x3c, 2, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0x4a, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7),
        (0xc5, 1, 1), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff), (0x49, 1, 7)],
    day: 0xf, month: 0, week: 0xffff_ffff, prize: Some(0x1e8480),
};

pub(crate) const MERCONORTE_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0x3c, 2, 2), (0xc6, 1, 2), (0x42, 1, 2), (0xc2, 1, 3), (0xc7, 1, 3), (0xc3, 1, 1), (0xc4, 1, 7), (0xc5, 1, 1), (0xbe, 1, 1),
        (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0x1c, 4, 0xffff_ffff), (0x20, 4, 0xffff_ffff)],
    day: 1, month: 7, week: 0xffff_ffff, prize: Some(0x1e8480),
};

pub(crate) const OCEANIA_RULES: ContinentalGroupRules = ContinentalGroupRules {
    fields: &[(0x43, 1, 2), (0xc6, 1, 2), (0xbe, 1, 2), (0xc2, 1, 3), (0x49, 1, 3), (0x4a, 1, 3), (0x3c, 2, 1), (0xc3, 1, 1), (0xc4, 1, 8),
        (0xc5, 1, 1), (0xc7, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 0), (0xd9, 2, 0x28)],
    day: 0x14, month: 7, week: 5, prize: None,
};

/// rule: the Libertadores groups are home and away (double round robin), 3 points a win and 1 a draw, tiebreak by codes
/// 1, 2 and 3 [inference: goal difference, goals for, wins]; two qualifiers per group; clubs from several nations;
/// category 7 on each match; 7 substitutes on the bench and 3 changes; starts on day 15 of month 0 of the competition
/// year [inference: 15 January, months counted from 0]; prize fund of 2,000,000
/// Original: `ConmebolLiber::set_up_rules` (0x4c3890, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_set_up_rules(this: usize) -> u32 {
    covered!("cqj_11 libertadores rules");
    let day = 0xf; // cqj11: the Libertadores start day
    let win = 3; // cqj11: the win in the Libertadores groups
    apply_cup_rules(League(this), &LIBERTADORES_RULES, day, win)
}

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

// simplification: virtual +0x3c allocates the rounds table: it does not run twice. Original only; the plan checks the rules.

/// Clubs whose season continental competition (+0x1db) is this competition, in database order, up to max.
pub(crate) unsafe fn libertadores_entrants(this: usize, max: i32, out: &mut [usize; 32]) -> i32 {
    let mut n = 0i32;
    let mut i = 0i16;
    let mut club = read_usize(G_CLUB_TABLE);
    if (read_usize(G_CLUB_COUNT) as i32) > 0 {
        loop {
            if read_usize(club + club::CONTINENTAL_COMP_ID) == read_usize(read_usize(this + 4)) && n < max {
                if (n as usize) < out.len() {
                    out[n as usize] = club;
                }
                n += 1;
            }
            i = i.wrapping_add(1);
            club = club.wrapping_add(club::SIZE);
            if i as i32 >= read_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    n
}

/// Selects the clubs qualified for the Libertadores. Rule: takes the 32 clubs marked with it as the season's
/// continental competition (the mark comes from league qualification), in database order, then shuffled (96 random
/// swaps) for the groups; any other count is an error (the box).
///
/// Ret 0: `ConmebolLiber::select_qualified_clubs()`: `conmebol_liber::DRAWN_CLUB_COUNT` = 32 and the list
/// `conmebol_liber::DRAWN_CLUBS` (`exe::crt_malloc`(0x80): push 0x80; add esp, 4 -> cdecl; out of memory: the box at
/// line 0x285 and returns 0) with the clubs, in database order, whose season competition (`club::CONTINENTAL_COMP_ID`,
/// `libertadores_entrants`) is the Libertadores, up to 32; fewer than 32: the box at line 0x297 and returns 0;
/// otherwise the list is shuffled (`exe::comp_util_shuffle_pointer_array`(list, 32): push n; push list; add esp, 8 ->
/// cdecl; 3 × 32 random swaps) and returns 1.
/// Original: `ConmebolLiber::select_qualified_clubs` (0x4c3f40, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_select_qualified_clubs(this: usize) -> u32 {
    write_u32(this + conmebol_liber::DRAWN_CLUB_COUNT, 0x20);
    let list = exe::crt_malloc(0x80);
    write_u32(this + conmebol_liber::DRAWN_CLUBS, list as u32);
    if list == 0 {
        show_error_at(0x9a7de4, 0x285);
        return 0;
    }
    let mut v = [0usize; 32];
    let n = libertadores_entrants(this, read_usize(this + conmebol_liber::DRAWN_CLUB_COUNT) as i32, &mut v);
    for (k, &c) in v.iter().enumerate().take(n.min(32) as usize) {
        write_u32(list + k * 4, c as u32); // cqk11: the club in database order
    }
    if n != read_usize(this + conmebol_liber::DRAWN_CLUB_COUNT) as i32 {
        show_error_at(0x9a7de4, 0x297);
        return 0;
    }
    covered!("cqk_11 libertadores qualified");
    exe::comp_util_shuffle_pointer_array(list, read_i16(this + conmebol_liber::DRAWN_CLUB_COUNT) as u16 as u32); // cqk11: the 32 of the Libertadores shuffled
    1
}

/// rule: in the seven groups (stages 0 to 6), a club going through gets the qualification record (weight 40) and one
/// going out the failed-to-qualify record (weight 40); in the knockout (stage 7), a club going through gets the
/// qualification by round (weight 50 to 100), one going out the knocked-out record; the winner gets the title record
/// and goes to the Intercontinental Cup, in the South American place (place 0 if a club of continent
/// `KEY_CONTINENT_SOUTH_AMERICA` is there, otherwise place 1), with the Intercontinental season archive and predicted
/// positions; the runner-up gets the runner-up record
/// Original: `ConmebolLiber::update_staff_records_for_status` (0x4c4080, conmebol_liber.cpp, thiscall, ret 0x18).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_update_staff_records_for_status(this: usize, club: usize, outcome: u32, stage: u32, _a4: u32, game: usize, a6: u32) -> u32 {
    let game = Fixture(game);
    let this = League(this);
    let (d, f) = (outcome as u8, stage as u8 as i8);
    let comp = this.competition().addr();
    let final_ = |g: usize| {
        let h: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(g);
        h(eng_second_staff_records(), club, game.addr(), a6);
    };
    if f < 7 {
        if d == 6 {
            european_cup_add_qualified_record(club, comp, 0, 0x14, 0x28);
        } else if d == 0xfe {
            european_cup_add_failed_to_qualify_record(club, comp, 0x41c, 0x28);
        }
    } else if f == 7 {
        match d {
            0xff => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 4);
                // simplification: +0x30 goes in a register with garbage in the high 16 bits; 0x86c000 only uses the word
                european_cup_add_knocked_out_record(club, comp, game.round_prefix() as u16 as u32, game.round_name() as u16 as u32, weight);
            }
            0 => {
                let weight = scale_by_round(0x32, 0x64, game.round_number(), 4);
                let knockout = Cup(read_usize(this.stages() + 0x1c));
                let r = game.round_number() as isize;
                let round = (knockout.addr() as u32 & 0xffff_0000) | read_u16(knockout.round_table().wrapping_add((r * (cup_round::SIZE as isize)) as usize) + cup_round::ROUND_CODE + 0x68) as u32;
                european_cup_add_qualified_record(club, comp, (game.round_prefix() as u16) as u32, round, weight);
            }
            1 => {
                final_(0x86a800);
                let mundial = Cup(read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP).wrapping_mul(4))));
                let t = mundial.team_table();
                let nation = Nation(read_usize(read_usize(t) + club::NATION));
                covered!("cqk_11 libertadores champion");
                if !nation.is_null() && read_usize(nation.continent()) == read_usize(KEY_CONTINENT_SOUTH_AMERICA) {
                    write_u32(t, club as u32); // cqk11: the champion in the South American slot
                } else {
                    write_u32(t + 6, club as u32);
                }
                exe::comp_stats_archive_season_and_reset(read_usize(mundial.addr() + 8), 1);
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_usize(read_usize(mundial.addr()) + comp_vtable::SET_PREDICTED_POSITIONS));
                prever(mundial.addr());
            }
            2 => final_(0x86b0b0),
            _ => {}
        }
    }
    0
}

// simplification: the records (history, call-ups), the statistics file and the predicted World Cup positions do not
// run twice. Original only; the plan: return 0 and, in the title (stage 7, outcome 1), the champion in the slot of the
// Club World Championship chosen by the rule (0 if a club from continent [0x9cfa1c] is there, otherwise 1).

/// rule: the Libertadores has seven group stages (0 to 6) and the knockout at stage 7 (the top two of each group, 16
/// clubs); going past the last stage is an error (the box)
/// Original: `ConmebolLiber::advance_stage` (0x4c3a80, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_advance_stage(this: usize) {
    let this = League(this);
    let f = this.current_stage() as i32;
    if f >= (this.stage_count() as i32).wrapping_sub(1) {
        show_error_at(0x9a7de4, 0x1f6);
        return;
    }
    let f = f + 1;
    this.set_current_stage((f) as usize);
    if f == 7 { // cqk11: the knockout after the seven groups
        covered!("cqk_11 libertadores knockout");
        exe::conmebolliber_create_knockout_stage(this);
    }
}

// simplification: the knockout (0x4c3d10) allocates the stage and draws: does not run twice. Original only; the plan: +0x30 goes up one
// (outside the last stage, which gives the message box).

/// rule: the Copa Libertadores starts with six-round groups on Tuesdays [inference: weekday 2], from 25/2 to 1/4 (each
/// group a league stage, id 0x3e9 + stage, up to stage 6: 8 groups [inference]); then the knockout (stage 7) over two
/// legs: round of 16 drawn on 2/4 and played on 15/4 (16 teams, return 14 days later), quarter-finals 30/4 and 20/5,
/// semi-final 28/5 and 3/6, final draw 11/6 and match on 17/6 (return 7 days later); two-leg tie-break 1 and draw with
/// bit 4; category 7 in the round of 16 and 8 after; final prize 2 million to the winner and 800 thousand to the
/// runner-up
/// Original: `ConmebolLiber::create_schedule` (0x4c31c0, conmebol_liber.cpp, thiscall, ret 0x10).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    south_american_cup_schedule(League(this), stage, rounds, extra, league, 7, &LIBERTADORES_GROUP_ROUNDS, &LIBERTADORES_KNOCKOUT_ROUNDS, &[], 0x9a7de4, 0x9f, 0xc5,
        "cqo_13 grupos da libertadores", "cqo_13 mata-mata da libertadores")
}

/// rule: the Libertadores knockout has 16 clubs: the top 2 (`promoted_count()`) of each of the 8 groups (group 1 in
/// table order, the others by position in each stage's table), in a cup stage (stage 7) with the cup's round table and
/// year; the first round is drawn at once
/// Original: `ConmebolLiber::create_knockout_stage` (0x4c3d10, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_create_knockout_stage(this: usize) {
    build_knockout_stage(League(this), 0x10, 7, 7, 0x9a7de4, 0x24c, 0x26a, "cqs_10 classificados da libertadores");
}

