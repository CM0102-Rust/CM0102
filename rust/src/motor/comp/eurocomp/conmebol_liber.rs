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
    let mut d = [0u8; 8];
    exe::date_date_549f70((d.as_mut_ptr()) as *mut u8 as *mut [u16; 4], g.day, g.month, (this.year() as u16) as u32, g.week);
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

/// The Libertadores predicted positions: by the rounds of stage 7 (the knockout) and in blocks of 8 for those who stay
/// in the groups (box at line 0x30e); see `conmebol_set_predicted_positions`.
/// Original: `ConmebolLiber::set_predicted_positions` (0x4c4210, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_set_predicted_positions(this: usize) {
    conmebol_set_predicted_positions(this, 7, 8, 0x9a7de4, 0x30e)
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_SET_PREDICTED_POSITIONS);
    conmebol_compare_predicted_positions("conmebol_liber_set_predicted_positions", this, 7, 8, || o(this))
}

/// The Libertadores season turnover: the first half (`conmebol_free_season`), the next year (`conmebol_next_year`),
/// the CONMEBOL seeding for the new season (ConmebolSeeding::update_for_new_season 0x4c61b0(`G_CONMEBOL_SEEDING`, 0):
/// push 0; mov ecx -> thiscall, ret 4: the nations' places), `league::ACTIVE` = 0 (the draw still to be made) and the
/// rules (`conmebol_set_up_rules_again`, the box for line 0x1e3).
/// Original: `ConmebolLiber::start_new_season` (0x4c3980, conmebol_liber.cpp, thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn conmebol_liber_start_new_season(this: usize) -> u32 {
    let this = League(this);
    let league = this;
    let seed: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONMEBOLSEEDING_UPDATE_FOR_NEW_SEASON); // return unused
    conmebol_free_season(league);
    covered!("cqv_12 libertadores turnover");
    conmebol_next_year(league); // cqv12: the Libertadores of the next year
    seed(read_usize(G_CONMEBOL_SEEDING), 0); // cqv12: the places of the new Libertadores
    this.set_active((0) as usize); // cqv12: the new Libertadores not drawn yet
    conmebol_set_up_rules_again(league, 0x9a7de4, 0x1e3)
}

pub(crate) unsafe extern "thiscall" fn compare_conmebol_liber_start_new_season(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_START_NEW_SEASON);
    conmebol_compare_new_season("conmebol_liber_start_new_season", this, true, || o(this))
}

/// The body of the predicted positions: without statistics (`league::COMP_STATS`), the box `row` of `file`; the list
/// +0xee of +0xf2 invited clubs sorted by reputation (`exe::crt_qsort`(list, n, 4, 0x4b07d0): 4 pushes; add esp ->
/// cdecl); the round table of stage `stage` by the virtual `comp_vtable::CREATE_SCHEDULE` (push 0; push &extra; push
/// &rounds; push stage; mov ecx, this -> thiscall, ret 0x10); each club of the plan (`conmebol_predicted_positions_plan`)
/// with its position (`exe::comp_stats_set_club_final_position`: push position; push club; mov ecx, stats -> thiscall,
/// ret 8; the original pushes the byte with leftovers in the high bytes, 0x4a2540 reads the byte); the table freed
/// (`exe::crt_free`).
pub(crate) unsafe fn conmebol_set_predicted_positions(this: usize, stage: u32, block: u32, file: usize, row: u32) {
    let this = League(this);
    if this.comp_stats() == 0 {
        return show_error_at(file, row);
    }
    exe::crt_qsort(read_usize(this.addr() + 0xee), read_usize(this.addr() + 0xf2) as u32, 4, 0x4b07d0);
    let round_table: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::CREATE_SCHEDULE));
    let (mut rounds, mut extra) = (0i16, 0i16);
    let table = round_table(this.addr(), stage, &mut rounds, &mut extra, 0);
    for (c, p) in conmebol_predicted_positions_plan(this.addr(), table, rounds, block) {
        exe::comp_stats_set_club_final_position(this.comp_stats(), c, p as u32);
    }
    exe::crt_free(table);
}

pub(crate) unsafe fn conmebol_compare_predicted_positions(name: &str, this: usize, _stage: u32, _block: u32, o: impl FnOnce()) {
    let this = League(this);
    o();
    let stats = this.comp_stats();
    let list = read_usize(this.addr() + 0xee);
    let n = read_usize(this.addr() + 0xf2) as i32;
    if stats == 0 || list == 0 || !(1..=256).contains(&n) {
        return;
    }
    let mut read: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = read_usize(list + k * 4);
        let e = if c != 0 { exe::comp_stats_find_club_entry(stats, c) } else { 0 };
        read.push(if e != 0 { read_u8(e + comp_stats_club::FINAL_POSITION) as i32 } else { -1 });
    }
    let ok = read[0] == 1 && read.iter().all(|&x| x >= 1) && read.windows(2).all(|w| w[0] <= w[1]);
    SCOREBOARD.record(ok, || format!("{}({:#x}): positions {:?}", name, this.addr(), read));
}

/// The first half of the three cups' season turnover (the same body): `league::SEASON_CLOSED` = 0 and the table freed
/// (`exe::league_free_table`(this, 0): push 0; mov ecx -> thiscall, ret 4); the calendar `league::ROUND_DATES` freed
/// (`exe::crt_free`: push; add esp, 4 -> cdecl) and zeroed; with statistics (`league::COMP_STATS`),
/// `exe::comp_stats_archive_season_and_reset`(stats, 1) (push 1; mov ecx -> thiscall, ret 4); the invited clubs list
/// +0xee freed (not zeroed); with the current stage (`league::CURRENT_STAGE`) >= 0, the existing stages 0..=current
/// destroyed (the virtual `comp_vtable::SCALAR_DELETING_DESTRUCTOR` with 1: push 1; mov ecx -> thiscall).
pub(crate) unsafe fn conmebol_free_season(this: League) {
    this.set_season_closed((0) as usize);
    exe::league_free_table(this, 0);
    let l = this.round_dates();
    if l != 0 {
        exe::crt_free(l);
        this.set_round_dates((0) as usize);
    }
    let stats = this.comp_stats();
    if stats != 0 {
        exe::comp_stats_archive_season_and_reset(stats, 1);
    }
    let c = read_usize(this.addr() + 0xee);
    if c != 0 {
        exe::crt_free(c);
    }
    if (this.current_stage() as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = read_usize(this.stages() + 4 * i as usize);
            if f != 0 {
                let destroy: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(f) + comp_vtable::SCALAR_DELETING_DESTRUCTOR));
                destroy(f, 1);
            }
            i += 1;
            if i > this.current_stage() as i32 {
                break;
            }
        }
    }
}

/// The year (`league::YEAR`, word) goes up by one and the current stage (`league::CURRENT_STAGE`) goes back to -1.
pub(crate) unsafe fn conmebol_next_year(this: League) {
    this.set_year(this.year().wrapping_add(1));
    this.set_current_stage((u32::MAX) as usize);
}

/// The rules again (the virtual `comp_vtable::SET_UP_RULES`, thiscall, ret 0); 0: the box for line `row` of `file`
/// and 0 (the original returns the box's eax; the caller does not read it). Returns what the rules returned.
pub(crate) unsafe fn conmebol_set_up_rules_again(this: League, file: usize, row: u32) -> u32 {
    let rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(this.addr()) + comp_vtable::SET_UP_RULES));
    let r = rules(this.addr());
    if r == 0 {
        show_error_at(file, row);
    }
    r
}

pub(crate) unsafe fn conmebol_compare_new_season(name: &str, this: usize, clears_active: bool, o: impl FnOnce() -> u32) -> u32 {
    let this = League(this);
    let league = this;
    let year = league.year();
    let r = o();
    let new = league.year();
    let ok = this.season_closed() == 0 && new == year.wrapping_add(1) && (!clears_active || league.active() == 0);
    SCOREBOARD.record(ok, || format!("{}({:#x}): year {} -> {}, +0x4c {}, +0xba {:#x}, +0x45 {}", name, this.addr(), year, new, this.season_closed(), league.round_dates(), league.active()));
    r
}

/// The predicted positions the CONMEBOL cups give (0x4c2a90, 0x4c4210 and 0x4c5a90, the same body), in order: the list
/// of invited clubs (+0xee, +0xf2 clubs) already sorted; the rounds of the table `table` (`cup_round::SIZE`-byte
/// records; the word `cup_round::TIES` of each, those who go out in it), from the last to the first: the last gives
/// position 1 to those who go out in it and, like all of them, the current position (which goes up with each club) to
/// as many more; those left over go in blocks of `block` with the position of the start of the block. Returns (club,
/// position).
pub(crate) unsafe fn conmebol_predicted_positions_plan(this: usize, table: usize, rounds: i16, block: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let list = read_usize(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut current: u8 = 1;
    let mut r = rounds as i32 - 1;
    while r >= 0 {
        let p = table + r as usize * cup_round::SIZE + cup_round::TIES;
        if r == rounds as i32 - 1 {
            let mut j: i16 = 0;
            while j < read_i16(p) {
                v.push((read_usize(list + i as isize as usize * 4), current));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        current = bl;
        let mut j: i16 = 0;
        while j < read_i16(p) {
            v.push((read_usize(list + i as isize as usize * 4), current));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < read_usize(this + 0xf2) as i32 {
        v.push((read_usize(list + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + block {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// The Copa Libertadores at career creation: discipline rule 2 and importance 15; a new cup has eight stages, the
/// Conmebol seeding of the new season, the rules and the statistics (`build_south_american_cup`).
/// Original: `ConmebolLiber::ConmebolLiber` (0x4c2f50, conmebol_liber.cpp, thiscall, ret 8).
pub(crate) unsafe extern "thiscall" fn construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    build_south_american_cup(League(this), year, record, &COPA_LIBERTADORES, "cx03_19 libertadores new")
}

/// Comparer for `construct_conmebol_liber`: only the original runs (it allocates and reads the save file), then the
/// plan is checked (`cup_constructor_plan_mismatch`, without the round table).
pub(crate) unsafe extern "thiscall" fn compare_construct_conmebol_liber(this: usize, year: u32, record: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(IDX_CONSTRUCT_CONMEBOL_LIBER)(this, year, record);
    let error = cup_constructor_plan_mismatch(this, r, year, record, COPA_LIBERTADORES.vtable, &south_american_cup_ctor_fields(8), false);
    SCOREBOARD.record(error.is_none(), || format!("construct_conmebol_liber({:#x}, {}, {:#x}): {}", this, year as u16, record, error.clone().unwrap_or_default()));
    r
}

/// The constructor of the two South American cups (`Libertadores(year, record)`, `Mercosur(year, record)`), run once, at
/// career creation; the same body in both (checked instruction by instruction), `SouthAmericanCup` sets them apart.
///
/// On the League base (`exe::new_league`, thiscall, ret 0): the competition record (`league::COMPETITION`), the vtable,
/// the year (`set_year`), `discipline_rule` 2 (the South American rules), the drawn club list (+0xee) = 0 and
/// `importance` 0xf. If the saved state loads (`exe::league_load_from_temp_file`: push 1 -> thiscall, ret 4; nonzero),
/// returns this. On a new cup, in order: `league_flags` 0x28, `stage_index` 0xff, `current_stage` −1, the stage count
/// and the stage vector (`exe::crt_malloc` of 4 per stage, unchecked); in the Libertadores the Conmebol seeding's new
/// season (ConmebolSeeding::update_for_new_season on `G_CONMEBOL_SEEDING` with 0, thiscall, ret 4); the rules (thiscall,
/// ret 0; 0: the box of `rows[0]` and this); `club_count` 0, `table` 0 and `active` 0; the statistics
/// (`create_comp_stats_at_8`; without them the box of `rows[1]`). Returns this.
pub(crate) unsafe fn build_south_american_cup(this: League, year: u32, record: usize, cup: &SouthAmericanCup, branch: &'static str) -> usize {
    exe::new_league(this);
    write_u32(this.addr() + league::COMPETITION, record as u32);
    write_u32(this.addr(), cup.vtable as u32);
    this.set_year(year as u16 as i16);
    this.set_discipline_rule(2);
    write_u32(this.addr() + conmebol_liber::DRAWN_CLUBS, 0);
    this.set_importance(0xf);
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    covered!(branch);
    this.set_league_flags(0x28);
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(cup.stages);
    this.set_stages(exe::crt_malloc(cup.stages as u32 * 4));
    if cup.seeding {
        let seed: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CONMEBOLSEEDING_UPDATE_FOR_NEW_SEASON); // return unused
        seed(read_usize(G_CONMEBOL_SEEDING), 0);
    }
    let rules: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cup.rules);
    if rules(this.addr()) == 0 {
        show_error_at(cup.file, cup.rows[0]);
        return this.addr();
    }
    this.set_club_count(0);
    this.set_table(0);
    this.set_active(0);
    if !create_comp_stats_at_8(this.addr()) {
        show_error_at(cup.file, cup.rows[1]);
    }
    this.addr()
}

/// The Copa Libertadores: 8 stages, the seeding moved on, the boxes of lines 0x52 and 0x60.
pub(crate) const COPA_LIBERTADORES: SouthAmericanCup = SouthAmericanCup {
    vtable: 0x968bf4, stages: 8, seeding: true, rules: FN_CONMEBOLLIBER_SET_UP_RULES, file: 0x9a7de4, rows: [0x52, 0x60],
};

/// ConmebolLiber::set_up_rules (thiscall, ret 0; 0 = failure).
pub(crate) const FN_CONMEBOLLIBER_SET_UP_RULES: usize = 0x4c3890;
